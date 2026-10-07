<p align="center">
  <img src="new_redesign/bigicon512.png" width="160" alt="Логотип Photoslop">
</p>

<h1 align="center">Photoslop</h1>

<p align="center">
  Нативный редактор изображений на Rust для художников на Linux и Wayland.<br>
  <a href="README.md">English</a> · <b>Русский</b>
</p>

---

Photoslop — независимый форк [PhotoCraft](https://github.com/storytold/photocraft), сосредоточенный
на Linux и Wayland: цифровой живописи, графических планшетах и привычной пользователям Photoshop
работе со слоями, масками и PSD.

Проект находится в **ранней альфе**. Цель — свободная альтернатива Photoshop для рисования на
Linux; до полноценной замены в профессиональной работе ещё далеко.

> **Про название.** Форк называется Photoslop и получил собственный логотип (см.
> [`new_redesign/`](new_redesign)). Внутри приложения, в бинарниках, именах пакетов и
> идентификаторах пока остаётся `photocraft` / **PhotoCraft**, чтобы существующие файлы,
> настройки и пакеты продолжали работать.

## Направление форка

- **Linux и Wayland:** работа в нативной сессии, масштабирование интерфейса, ввод и интеграция
  с рабочим столом.
- **Инструменты художника:** кисти, отзывчивый холст, нажим и наклон пера, удобные слои и
  выделения.
- **Совместимость:** редактируемые документы, обмен PSD, разная глубина цвета.
- **Надёжность:** сохранность работы и предсказуемое поведение важнее количества пунктов меню.

Это приоритеты, а не список готовых возможностей. В частности, поддержка пера через Wayland
`tablet-v2` **пока не реализована**: планшеты на Linux работают через X11/XInput2, поэтому в
Wayland-сессии перо работает через XWayland. Реальное состояние движка — в
[дорожной карте](docs/roadmap.md) и [таблице проверок](docs/scorecard.md).

## Что есть в основе

- Слои, маски, режимы наложения, корректирующие слои и эффекты.
- Кисти, выделения, трансформации, текст и векторные фигуры.
- Чтение и запись PSD, собственный формат `.pcraft` и экспорт изображений.
- Нативный интерфейс egui/eframe с рендерингом через wgpu (без Electron и webview).
- Общий Rust-движок для приложения, CLI и автоматизации через MCP.

Совместимость с Photoshop неполная. Наличие команды в меню не гарантирует совпадения поведения
или побайтно одинаковых файлов.

## Скачать

Готовые сборки для Linux (AppImage, `.tar.gz`, а также `.deb`/`.rpm`, если они собраны) лежат на
странице [Releases](https://github.com/hlophlopgaming/photoslop/releases).

```sh
chmod +x photocraft-*-linux-x86_64.AppImage
./photocraft-*-linux-x86_64.AppImage
```

## Сборка из исходников

Нужен Rust stable **1.95 или новее** ([rustup](https://rustup.rs)) и несколько системных библиотек.

```sh
# Debian / Ubuntu
sudo apt install build-essential pkg-config libxkbcommon-dev libwayland-dev \
  libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev libgtk-3-dev

# Fedora
sudo dnf install gcc pkgconf-pkg-config libxkbcommon-devel wayland-devel \
  libX11-devel libXrandr-devel libXi-devel mesa-libGL-devel gtk3-devel

# Arch
sudo pacman -S --needed base-devel pkgconf libxkbcommon wayland \
  libx11 libxrandr libxi mesa gtk3
```

Затем из корня репозитория:

```sh
cargo run --release -p photocraft                       # запустить приложение
cargo run --release -p photocraft -- path/to/image.psd  # открыть файл при запуске
```

Запускайте из своей Wayland-сессии, чтобы проверять работу именно там. Другие платформы и
веб-сборка унаследованы от исходного проекта; основной фокус форка — Linux. Шрифты, диагностика
и прочее — в [руководстве разработчика](docs/development.md).

## Сборка и релиз одной командой

[`scripts/release.sh`](scripts/release.sh) собирает пакеты для Linux и публикует GitHub Release:

```sh
scripts/release.sh 0.4.0        # поднять версию до 0.4.0, собрать, тег v0.4.0, push, релиз
scripts/release.sh              # выпустить версию, которая уже указана в Cargo.toml
scripts/release.sh --build-only # только собрать пакеты в dist/release/
```

Скрипт поднимает версию (`Cargo.toml` + `Cargo.lock`) и коммитит её, собирает AppImage и
`.tar.gz` (и `.deb`/`.rpm`, если установлен [nfpm](https://nfpm.goreleaser.com)), пишет
`SHA256SUMS.txt`, создаёт тег `v<версия>`, пушит и загружает всё через
[GitHub CLI](https://cli.github.com). Версии вида `0.4.0-rc.1` публикуются как pre-release.

Что нужно: чистое рабочее дерево (всё закоммичено), один раз выполненный `gh auth login` и
зависимости для сборки из списка выше. Все опции (`--draft`, `--formats`, `--yes`) —
в `scripts/release.sh --help`.

## Разработка

```sh
cargo test -p photocraft-ui-egui
cargo xtask layers
cargo xtask wasm
```

- [Архитектура и устройство workspace](docs/architecture.md)
- [Правила внесения изменений](docs/contributing.md)
- [Управление приложением и снимки экрана](docs/control-protocol.md)
- [Дизайн интерфейса](docs/ui-design.md)
- [Дорожная карта исходного проекта](docs/roadmap.md)

Сообщая о проблеме, укажите дистрибутив, композитор Wayland или X11, видеокарту, модель планшета
(если проблема с пером), шаги воспроизведения и размер документа.

## Происхождение и лицензия

Photoslop — форк [исходного PhotoCraft](https://github.com/storytold/photocraft). Проприетарные
логотипы и рекламные ссылки исходного бренда удалены; новый логотип Photoslop лежит в
[`new_redesign/`](new_redesign).

Код распространяется на условиях [MIT](LICENSE-MIT) или [Apache-2.0](LICENSE-APACHE), на ваш
выбор. Исходные уведомления об авторских правах сохранены в [NOTICE](NOTICE) и файлах лицензий.
Сведения об остальных ресурсах — в [ATTRIBUTION.md](ATTRIBUTION.md).
