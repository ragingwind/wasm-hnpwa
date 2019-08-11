# wasm-hnpwa

> HNPWA with WASM, Warning you, it is still om experimental stage, PoC version for WebAssembly, Rust, wasm-pack, and wasm-bindgen. Application logic fully coded by rust in WASM module not HTML and JS also fetching. This is highly inspired by [wasm-bindgen todomvc example](https://github.com/rustwasm/wasm-bindgen/tree/master/examples/todomvc). also brought some of code and design from the example.

<img width="2160" alt="HNPWA with WASM" src="https://user-images.githubusercontent.com/124117/62828430-2dcf1700-bc21-11e9-993e-86e096ec351a.png">

# How to build and run

```sh
$ yarn build
$ npx http-server ./dist
```

# Deployment

This application is running powered by [now](https://zeit.co/now) with static build. Therefore, it might be needed to test the app on live.

```sh
now
```

# License

@ [Jimmy Moon](jimmymoon.dev)
