# deploytest

<p align="center">
  <img src="https://jamiemason.github.io/deploytest/logo.svg" width="200" height="179" alt="">
  <br>Consistent dependency versions in large JavaScript Monorepos.
  <br><a href="https://jamiemason.github.io/deploytest">https://jamiemason.github.io/deploytest</a>
</p>

## Rust

A work in progress implementation of Syncpack in Rust. It is not ready for public use.

## Develop

```shell
git clone https://github.com/JamieMason/deploytest.git -b main deploytest-rust
cd deploytest-rust
```

## Run (Development)

There are 2 commands, `lint` and `fix`.

```shell
cargo run -- lint
cargo run -- fix
```

Both will check formatting and version/range mismatches by default, but can be filtered with `--format` and `--versions`.

## Build and Run (Production)

```shell
cargo build --release
target/release/deploytest lint
target/release/deploytest fix
```
