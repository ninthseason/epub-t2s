# epub-t2s: convert traditional chinese to simplified chinese in epub

将 epub 格式电子书中的繁体中文转化为简体中文以愉悦阅读轻小说的命令行工具。

## Usage

```bash
./epub_t2s somebook.epub

# Output file at the same directory: somebook-简中.epub
```

## Acknowledge

- [OpenCC](https://github.com/BYVoid/OpenCC) - 中文繁简转换开源项目
- [opencc-rust](https://github.com/magiclen/opencc-rust) - OpenCC 的 Rust 绑定

## Build

### On Windows

需要安装有 MinGW(GCC>=8) 环境，Rust 工具链设置为 x86_64-pc-windows-gnu

```bash
cargo build_mingw

# cargo build_mingw_release  # for release
```

*事实上，也可采用 VS 环境与 x86_64-pc-windows-msvc 工具链。然而本仓库并未准备对应的 OpenCC 静态链接库。*

### On Linux

需要 GCC 版本 >=11

```bash
cargo build_glibc

# cargo build_glibc_release  # for release
```

### Cross-compile

目前只知道从 Linux 向 Windows 编译的方法。

需于 Linux 上安装 MinGW(GCC>=8) 环境

```bash
cargo build_mingw

# cargo build_mingw_release  # for release
```
