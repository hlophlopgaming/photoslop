#!/usr/bin/env bash
# Build Photoslop for Linux and publish a GitHub Release, in one command.
#
#   scripts/release.sh [VERSION] [options]
#
#   VERSION          new version, e.g. 0.4.0 or 0.4.0-rc.1. Sets Cargo.toml + Cargo.lock and
#                    commits "Release vVERSION". Omit it to release the version already there.
#   --build-only     only build the packages into dist/release/ (no commit, tag, push or release)
#   --draft          create the GitHub Release as a draft
#   --formats "..."  package formats (default: "appimage tar", plus "deb rpm" if nfpm is installed)
#   --yes, -y        don't ask for confirmation before pushing and publishing
#   -h, --help       show this help
#
# Needs: cargo, git, and the system build dependencies from README.md. Publishing also needs the
# GitHub CLI (gh), logged in once with `gh auth login`. Versions with a suffix (-rc.1) are
# published as pre-releases.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

die() { echo "error: $*" >&2; exit 1; }
step() { printf '\n\033[1m==> %s\033[0m\n' "$*"; }

NEW_VERSION=""
BUILD_ONLY=0
DRAFT=0
YES=0
FORMATS=""
while [ $# -gt 0 ]; do
  case "$1" in
    --build-only) BUILD_ONLY=1; shift ;;
    --draft) DRAFT=1; shift ;;
    --formats) [ $# -ge 2 ] || die "--formats needs a value"; FORMATS="$2"; shift 2 ;;
    -y | --yes) YES=1; shift ;;
    -h | --help) sed -n '2,16p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
    -*) die "unknown option: $1 (see --help)" ;;
    *) [ -z "$NEW_VERSION" ] || die "only one VERSION allowed"; NEW_VERSION="${1#v}"; shift ;;
  esac
done

command -v cargo >/dev/null || die "cargo not found (install Rust: https://rustup.rs)"
command -v git >/dev/null || die "git not found"

if [ -z "$FORMATS" ]; then
  FORMATS="appimage tar"
  if command -v nfpm >/dev/null; then FORMATS="$FORMATS deb rpm"; fi
fi

# owner/repo of the fork, from `origin` (used for the AppImage update info and for gh).
REPO="${GITHUB_REPOSITORY:-$(git remote get-url origin 2>/dev/null | sed -E 's#^.*github\.com[:/]##; s#\.git$##')}"
[ -n "$REPO" ] || die "could not work out the GitHub repository from 'git remote get-url origin'"

current_version() { awk '/^\[/{p=($0=="[workspace.package]");next} p&&$1=="version"{gsub(/[" ]/,"",$3);print $3;exit}' Cargo.toml; }

# ---- checks before touching anything ------------------------------------------------------------
if [ "$BUILD_ONLY" = 0 ]; then
  command -v gh >/dev/null || die "gh (GitHub CLI) not found: https://cli.github.com, then run 'gh auth login'"
  gh auth status >/dev/null 2>&1 || die "gh is not logged in; run 'gh auth login' once"
  if ! git diff --quiet || ! git diff --cached --quiet; then die "the working tree has uncommitted changes; commit or stash them first"; fi
  BRANCH="$(git rev-parse --abbrev-ref HEAD)"
  [ "$BRANCH" != HEAD ] || die "not on a branch (detached HEAD)"
  git fetch -q origin "$BRANCH" 2>/dev/null || true
  if git rev-parse -q --verify "origin/$BRANCH" >/dev/null &&
    ! git merge-base --is-ancestor "origin/$BRANCH" HEAD; then
    die "'$BRANCH' is behind origin/$BRANCH; run 'git pull' first"
  fi
fi

VERSION="${NEW_VERSION:-$(current_version)}"
[ -n "$VERSION" ] || die "could not read the version from Cargo.toml"
printf '%s' "$VERSION" | grep -Eq '^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$' ||
  die "'$VERSION' is not a version like 1.2.3 or 1.2.3-rc.1"
TAG="v$VERSION"

if [ "$BUILD_ONLY" = 0 ]; then
  git rev-parse -q --verify "refs/tags/$TAG" >/dev/null && die "tag $TAG already exists; pick a new version"
  gh release view "$TAG" --repo "$REPO" >/dev/null 2>&1 && die "release $TAG already exists on $REPO; pick a new version"
fi

echo "Photoslop $VERSION -> $REPO"
echo "  formats: $FORMATS"
if [ "$BUILD_ONLY" = 1 ]; then
  echo "  mode:    build only (nothing is committed, tagged or published)"
else
  echo "  mode:    build, tag $TAG, push '$BRANCH', publish$([ "$DRAFT" = 1 ] && echo ' (draft)')"
  if [ "$YES" = 0 ]; then
    printf 'Continue? [y/N] '
    read -r answer
    case "$answer" in y | Y | yes | YES | д | Д | да | Да) ;; *) echo "cancelled"; exit 1 ;; esac
  fi
fi

# ---- version ------------------------------------------------------------------------------------
if [ -n "$NEW_VERSION" ] && [ "$NEW_VERSION" != "$(current_version)" ]; then
  if [ "$BUILD_ONLY" = 1 ]; then
    # Don't edit Cargo.toml in a dry build: only the package file names get the new version.
    export PHOTOCRAFT_VERSION="$VERSION"
  else
    step "Setting version $VERSION"
    cargo xtask version set "$VERSION"
    git add Cargo.toml Cargo.lock
    git commit -q -m "Release $TAG"
    echo "committed 'Release $TAG'"
  fi
fi

# ---- build --------------------------------------------------------------------------------------
step "Building Linux packages ($FORMATS)"
DIST="$ROOT/dist/release"
mkdir -p "$DIST"
rm -f "$DIST"/photocraft-"$VERSION"-linux-* "$DIST/SHA256SUMS.txt"
# GITHUB_REPOSITORY makes the AppImage's self-update info point at this fork's releases.
GITHUB_REPOSITORY="$REPO" DIST="$DIST" "$ROOT/packaging/linux/package.sh" --formats "$FORMATS"

shopt -s nullglob
ASSETS=("$DIST"/photocraft-"$VERSION"-linux-*)
shopt -u nullglob
[ "${#ASSETS[@]}" -gt 0 ] || die "the build produced no packages in $DIST"
(cd "$DIST" && for f in "${ASSETS[@]}"; do sha256sum "$(basename "$f")"; done >SHA256SUMS.txt)
ASSETS+=("$DIST/SHA256SUMS.txt")

if [ "$BUILD_ONLY" = 1 ]; then
  step "Done: packages are in $DIST"
  ls -lh "${ASSETS[@]}"
  exit 0
fi

# ---- tag, push, publish -------------------------------------------------------------------------
step "Tagging $TAG and pushing"
git tag -a "$TAG" -m "Photoslop $TAG"
git push origin "$BRANCH"
git push origin "$TAG"

step "Publishing the GitHub Release"
args=(--repo "$REPO" --title "Photoslop $TAG" --verify-tag --generate-notes)
case "$VERSION" in *-*) args+=(--prerelease) ;; esac
[ "$DRAFT" = 1 ] && args+=(--draft)
gh release create "$TAG" "${args[@]}" -- "${ASSETS[@]}"

step "Released Photoslop $TAG"
echo "https://github.com/$REPO/releases/tag/$TAG"
