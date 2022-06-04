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
