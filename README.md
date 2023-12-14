#
<div align="center">
  <h1>Light-Id</h1>

  [![Crates.io](https://img.shields.io/crates/v/light-id)](https://crates.io/crates/light-id)
  [![GitHub](https://img.shields.io/github/stars/ntillier/Light-id)](https://github.com/ntillier/Light-id)
  [![Documentation](https://docs.rs/light-id/badge.svg)](https://docs.rs/light-id/latest)
</div>

`light-id` is a Rust crate designed for effortless generation and manipulation of lightweight, incremental IDs. Built with safety and speed in mind, it offers a versatile and customizable approach to create and switch between different bases for IDs. The crate includes various utilities to tailor your IDs to specific requirements.

## Features

- **Incremental Generation:** Easily generate IDs in an incremental fashion.
- **Base Switching:** Seamlessly switch between different bases for your IDs.
- **Customization:** Fine-tune your IDs with a range of utilities for flexibility.

## Links
* [Crates.io](https://crates.io/crates/light-id): Find the crate on Crates.io.
* [GitHub Repository](https://github.com/ntillier/Light-id): Explore the source code and star it :star:
* [Documentation](https://docs.rs/light-id/latest): Refer to the documentation for detailed usage.

## Getting Started

#### Installation
Add the following lines to your `Cargo.toml` file:
```toml
[dependencies]
light-id = "0.1.0"
```

#### Usage
```rust
use light_id::LightId;

let mut generator = LightId::new();
println!("Current ID: {}", generator.next());
```

#### Custom base and skipping
```rust
use light_id::LightId;

let mut generator = LightId::from("abc");
generator.skip(2);
println!("Skipped ID: {}", generator.current());
```
#### Base switching
```rust
use light_id::IdSwitcher;

let switcher = IdSwitcher::new("0123456789", "abcdef");
let switched_id = switcher.switch("2");
println!("Switched ID: {}", switched_id);
```

## Support
If you encounter any issues, please [report them on GitHub](https://github.com/ntillier/Light-id/issues). I'll be pleased to help you!

## License
This crate is licensed under the [MIT License](https://opensource.org/license/mit/).