# breakout
Classic Breakout Game using Rust and Macroquad

## assets from kenney.nl

## Compile to wasm
1.  if wasm target not added then $ rustup target add wasm32-unknown-unknown
    run $ cargo build --release --target wasm32-unknown-unknown

2. cp breakout.wasm from target/wasm.../release/breakout.wasm to project root dir.

3. change index.html width and height if diffrent.

3. install basic-http-server and run
    $ cargo install basic-http-server
    $ basic-http-server .