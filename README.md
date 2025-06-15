![lqf banner](lqf-banner.png)

# lqf 🪶

**lqf** is a **lightweight configuration format** featuring a clean, sectioned syntax centered around the use of the `>` symbol — designed to be easy to read, easy to write, and dead simple to parse. The official lqf specification can be found in [github.com/smit4k/lqf-spec](https://github.com/smit4k/lqf-spec)  

This Rust crate provides a parser for `.lqf` files using [pest.rs](https://pest.rs/), ready for use in config-heavy projects or DSL exploration.

What does **lqf** stand for? It stands for **Lightweight Quick Format**

You can find a full example of an lqf file in [example.lqf](example.lqf)

## ✨ Features & Syntax

- Simple syntax for sectioned key-value configuration
  - Section header: `>`
  - Assignment: `>>`
- Supports:
  - Strings: `"text"`
  - Numbers: `123`, `3.14`
  - Booleans: `true`, `false`
  - Arrays: `[1, 2, 3]`, `["a", "b"]`
  - Null: `null`
  - Comments: `#`
- Built with [pest](https://pest.rs) — robust and expressive PEG-based parser

## 🧪 Example

Given this `.lqf` file:

```lqf
> database
host >> "localhost"
port >> 5432

> features
enabled >> ["search", "logging", "metrics"]
```

You can parse it like this in Rust:

```rust
use lqf::parse_lqf;
fn main() {
    let input = r#"
        > database
        host >> "localhost"
        port >> 5432

        > features
        enabled >> ["search", "logging", "metrics"]
    "#;

    let parsed = parse_lqf(input).expect("failed to parse LQF");
    println!("{:#?}", parsed);
}
```

Output:

```rust
{
    "database": {
        "host": String("localhost"),
        "port": Number(5432.0),
    },
    "features": {
        "enabled": Array([
            String("search"),
            String("logging"),
            String("metrics"),
        ])
    }
}
```

## 📦 Installation

Add this to your `Cargo.toml`

```toml
[dependencies]
lqf = "0.1.2"
```

## 🤝 Contributing

Contributions are welcome! Feel free to open issues, submit pull requests, or discuss ideas.
