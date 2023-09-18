# ARK Shelf Desktop

A dekstop version of ARK Shelf.

## Preparation

-   [Node.js 16+](https://nodejs.org/en/download)
-   [Rust (MSRV 1.60 or latest)](https://www.rust-lang.org/tools/install)
-   [pnpm](https://pnpm.io/installation): `curl -fsSL https://get.pnpm.io/install.sh | sh -`
-   [vite](https://vitejs.dev/): `pnpm install vite`
-   [tauri cli](https://tauri.app/): `pnpm add -D @tauri-apps/cli`

### Tauri requirements:

- Debian:

    ```bash
    sudo apt update
    sudo apt install libwebkit2gtk-4.0-dev \
        build-essential \
        curl \
        wget \
        libssl-dev \
        libgtk-3-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev
    ```

-  Windows:

    Install Microsoft Visual Studio C++ build tools [Build tools](https://visualstudio.microsoft.com/fr/visual-cpp-build-tools/)

-  MacOS:

    ```bash
    xcode-select --install
    ```


Create a build directory for tauri<sup>[*](https://github.com/tauri-apps/tauri/issues/3142)</sup>.

```sh
mkdir dist
```

## Install

Clone the repo

```
git clone https://github.com/ARK-Builders/ARK-Shelf-Desktop.git
```

Then setup the node dependencies

```bash
pnpm i
```

To run:

```bash
pnpm dev
```

To build a production release:

```bash
pnpm tauri build
```

This will output binaries in `./src-tauri/target/release`


## Note For AppImage Build

Due to the limitation of AppImage, if you want to use another directory, you have to provide the absolute path (or full path) to use it.
