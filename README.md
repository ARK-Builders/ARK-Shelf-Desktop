# ARK Shelf Desktop

A dekstop version of ARK Shelf.

## Preparation

-   Node.js 16+
-   Rust (MSRV 1.60 or latest)
-   Yarn 1

## Install

Fork or pull the repo first.

Then setup the node dependencies

```bash
pnpm i
```

Install the Rust requirements:

-   If you are on Debian

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

    Then install Rust

    ```sh
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ```

-   If you are on windows:

    Install Microsoft Visual Studio C++ build tools [Build tools](https://visualstudio.microsoft.com/fr/visual-cpp-build-tools/)
    Then install Rust via rustup [Rustup](https://www.rust-lang.org/tools/install)

-   If you are on MacOS:

    ```bash
    xcode-select --install
    ```

    Then install Rust

    ```sh
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ```

Also please create a directory for development build.

```sh
mkdir app/build
```

Once it finished, you can run this command to get in development.

```bash
pnpm dev
```

> It may takes more time for the first-run since it need to fetch dependencies for tauri and build. Please be patient

And to get a production build, use this command.

```bash
pnpm build
```

## Note For AppImage Build

Due to the limitation of AppImage, if you want to use another directory to store you link, you have to provide the absolute path (or full path) to use it.
