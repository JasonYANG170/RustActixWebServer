[简体中文](README.md) | [English](README_en.md)

# RustActixWebServer

A lightweight static file server built with Rust, Actix Web, and actix-files.

## Run

Install Rust and Cargo, then run from the repository root:

```sh
cargo run
```

Open `http://127.0.0.1:8080/` in a browser. The server listens on local address `127.0.0.1:8080`, serves `./apk` at `/`, uses `index.html` as the default index, and sets the default response header `Content-Type: text/html`.

## Project structure

- `src/main.rs`: Server entry point and route configuration.
- `apk/`: Static files served to the browser.
- `Cargo.toml`: Rust 2021 project depending on `actix-web` 4.5.1 and `actix-files` 0.6.5.
