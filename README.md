# ARK Shelf Desktop

A dekstop version of ARK Shelf.

## Preparation

- Node.js 16+
- Rust (MSRV 1.60 or latest)
- Yarn 1

## Install

Fork or pull the repo first.

Then setup the dependencies

```bash
yarn
```

Also please create a directory for development build.

```sh
mkdir app/build
```

Once it finished, you can run this command to get in development.

```bash
yarn tauri dev
```

> It may takes more time for the first-run since it need to fetch dependencies for tauri and build. Please be patient

And to get a production build, use this command.

```bash
yarn tauri build
```

## Options

```bash
-h, --help           Print help information
-p, --path <PATH>    Path to store .link file [default: $HOME/ark-shelf]
```

## Note For AppImage Build

Due to the limitation of AppImage, if you want to use another directory to store you link, you have to provide the absolute path (or full path) to use it.
