EMCC_CFLAGS="-sUSE_GLFW=3 -sGL_ENABLE_GET_PROC_ADDRESS -sASYNCIFY" cargo build --release --target=wasm32-unknown-emscripten --features=wasm && mv target/wasm32-unknown-emscripten/release/*.* web
