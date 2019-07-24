# xlib-sys

bindgen testing with Xlib. Warning: Do not use code from
this repository in production! Currently bindgen doesn't generate
Xlib bindings which will work on both 64-bit and 32-bit systems.

## Instructions

1. Download git submodules.

```
git submodule update --init
```

2. Remove Xlib header packages form your computer.

3. Generate bindings and binding documentation.

```
cd binding-generator
cargo run
cd ../xlib-sys
cargo doc --open
```


## License

MIT License
