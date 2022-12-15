# libboxer
FFI bindings to `value-box-rs` crate

## Generating `boxer.h` header

Make sure to install latest [cbindgen](https://github.com/eqrion/cbindgen):
```bash
cargo install cbindgen
```

Generate the header running the following from the root of `libboxer` repository:
```bash
cbindgen --config cbindgen.toml --crate libboxer --output boxer.h
```

## Released `boxer.h` header
`boxer.h` is released together with shared libraries.
The latest header is available on GitHub: https://github.com/feenkcom/libboxer/releases/latest