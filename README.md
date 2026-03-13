# echo_music

### Build

```bash
cargo build --release
```

### Flash

[minichlink](https://github.com/cnlohr/ch32fun/tree/master/minichlink)needs to be installed

Please install it and add it to your PATH.

You need `rust-objcopy`.

```bash
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

To flash the firmware to the device, run:
```bash
cargo run --release
```
