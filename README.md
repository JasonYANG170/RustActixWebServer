[简体中文](README.md) | [English](README_en.md)

# RustActixWebServer

使用 Rust、Actix Web 与 actix-files 实现的轻量级静态文件服务器。

## 运行

安装 Rust 与 Cargo 后，在仓库根目录运行：

```sh
cargo run
```

浏览器访问 `http://127.0.0.1:8080/`。服务器监听本机的 `127.0.0.1:8080`，将 `./apk` 目录挂载到 `/`，默认首页为 `index.html`，并设置默认响应头 `Content-Type: text/html`。

## 项目结构

- `src/main.rs`：服务器入口与路由配置。
- `apk/`：提供给浏览器访问的静态文件。
- `Cargo.toml`：Rust 2021 项目，依赖 `actix-web` 4.5.1 与 `actix-files` 0.6.5。
