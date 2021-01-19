# Rust-in-Python
An example of how to call a Rust function inside Python

## what does it do?
This program receives 2 inputs: `x` and `n` and returns `x^(2*n)`

## how to run
To run you need to:
* execute the following command: `python src/main.py`

*(it's already compiled)*

## how to compile
To compile you need to:
* remve `libsquare.so` from `src`: `rm src/libsquare.so`
* remove `target` folder: `rm -r target`
* compile the lib: `cargo build --release`
* move the file `libsquare.so` from `target/release` to `src`: `mv target/release/libsquare.so src`