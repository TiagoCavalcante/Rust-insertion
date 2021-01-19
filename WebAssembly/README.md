# WebAssembly
An example of how compile Rust to WebAssembly and inlcude it inside JavaScript

## how to run
To run you need to:
* go to directory `docs`: `cd docs`
* serve the files: `php -S localhost:80`

(it's already compiled)

## how to compile
To compile you need to:
* delete `webassembly.js` and `webassembly_bg.wasm` from `docs`: `rm docs/webassembly*`
* compile the source code: `wasm-pack build --target web`
* move the file `webassembly.js` from `pkg` to `docs`: `mv pkg/webassembly.js docs/webassembly.js`
* move the file `webassembly_bg.wasm` from `pkg` to `docs`: `mv pkg/webassembly_bg.wasm docs/webassembly.wasm`