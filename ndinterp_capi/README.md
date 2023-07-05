# `ndinterp` C API


## Benchmarks

To run the benchmarks first you need to compile and install the C library.

The suggested way is to use [`cargo-c`](https://github.com/lu-zero/cargo-c):
```sh
# inside this folder
cargo cinstall --prefix <your-prefix>
```
were `<your-prefix>` is the location were to install the library. A suitable
example might be `$HOME/.local`, for a user-wide installation.
