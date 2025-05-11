<p align="center">
    <h1 align="center">slog-rs</h1>
    <p align="center">Simple just log library for my projects (tracing-subscriber wrapper) 🦀</p>
</p>

---
[![License - MIT](https://img.shields.io/github/license/containerscrew/slog-rs)](/LICENSE)
![Crates.io Version](https://img.shields.io/crates/v/slog-rs)
![Code Size](https://img.shields.io/github/languages/code-size/containerscrew/slog-rs)
[![Test Pipeline](https://github.com/containerscrew/slog-rs/actions/workflows/test.yml/badge.svg)](https://github.com/containerscrew/slog-rs/actions/workflows/test.yml)
---

# About

I needed to learn how to create a library in Rust and be able to share it with all my projects.

# Usage

```shell
cargo add slog_rs
```

```shell
use std::error::Error;

use slog_rs::{info, warn, Logger};


fn main() -> Result<(), Box<dyn Error>> {
    Logger::new()
        .with_level("warn")
        .with_format("json")
        .init()?;

    info!("Hello info message!");
    warn!("Hello warning message!");
    Ok(())
}
```

# License

`slog-rs` is distributed under the terms of the [MIT](./LICENSE) license.
