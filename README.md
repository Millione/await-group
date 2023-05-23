# await-group

[![Crates.io][crates-badge]][crates-url]
[![License][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/await-group.svg
[crates-url]: https://crates.io/crates/await-group
[license-badge]: https://img.shields.io/crates/l/await-group.svg
[license-url]: #license
[actions-badge]: https://github.com/Millione/await-group/actions/workflows/ci.yaml/badge.svg
[actions-url]: https://github.com/Millione/await-group/actions

Golang like [WaitGroup](https://pkg.go.dev/sync#WaitGroup) implementation.

## Usage

Add this to your `Cargo.toml`:

```toml
[build-dependencies]
await-group = "0.1"
```

## Example
```rust
use await_group::AwaitGroup;

#[tokio::main]
async fn main() {
    let wg = AwaitGroup::new();
    for _ in 0..10 {
        let w = wg.clone();
        tokio::spawn(async move {
            _ = w;
        });
    }
    wg.await;
}

```

## License

Dual-licensed under the MIT license and the Apache License (Version 2.0).

See [LICENSE-MIT](https://github.com/Millione/await-group/blob/main/LICENSE-MIT) and [LICENSE-APACHE](https://github.com/Millione/await-group/blob/main/LICENSE-APACHE) for details.
