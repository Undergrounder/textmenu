# Textmenu (beta)
**LCD menu library**

[<img alt="github" src="https://img.shields.io/badge/github-Undergrounder/textmenu-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/Undergrounder/textmenu)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/Undergrounder/textmenu/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/dtolnay/syn/actions?query=branch%3Amaster)


## Building and testing

```
cargo build --release
cargo test --release
```

## Pendings improvements

* Features to disable item types
* Features to disable interface impls
** Debug
** Eq, PartialEq
* Item types
** Input item
** Charset input item
** More range items (u8, signed, float, ....)
* Horizontal scrolling if overflow
* Stabilize API and release first stable version