# /bin/bash
# Custom build-script using `xargo` to reduce the binary size, while also compiling for code optimization.
RUSTFLAGS="-C opt-level=3 -Zdylib-lto -C debug-assertions=off -C overflow-checks=off -C link-arg=-Wl,-O3" xargo build --release
