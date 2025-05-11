# slog-rs

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