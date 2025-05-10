# run

From: https://bevyengine.org/learn/quick-start/getting-started/setup/

1. install `mold clang`
1. Install rustup as usual.
1. `rustup toolchain install nightly`
1. `rustup toolchain list` should contain `nightly`
1. `rustup override set nightly`
1. `cargo run`
   
For windows:  

1. install `mingw-w64`
1. `rustup target add x86_64-pc-windows-gnu` 
1. `cargo build --target=x86_64-pc-windows-gnu --release`

Develop:                                           

`cargo run --target=x86_64-unknown-linux-gnu`
