# wasm-tutorial

## rust

There are two main use cases for Rust and WebAssembly:

 * Build an entire application — an entire web app based in Rust.
 * Build a part of an application — using Rust in an existing JavaScript frontend.

Read more on [Rust_to_wasm](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm).


## wasm-pack


The wasm-bindgen tool generates javascript glue to handle types other than numbers


wasm-pack allows to pack up your rust wasm application as npm package and publish it!

    // install wasm-pack
    $ rustup target add wasm32-unknown-unknown
    // optional cargo install wasm-gc
    // optional cargo install wasm-bindgen-cli

    $ wasm-pack build -t web
    $ ls -la pkg
    total 64
    drwxr-xr-x  10 ivanitskiy  staff  320 Jul 15 13:13 .
    drwxr-xr-x  11 ivanitskiy  staff  352 Jul 15 13:10 ..
    -rw-r--r--   1 ivanitskiy  staff    1 Jul 15 13:13 .gitignore
    -rw-r--r--   1 ivanitskiy  staff   15 Jul 15 12:39 README.md
    -rw-r--r--   1 ivanitskiy  staff  282 Jul 15 13:13 package.json
    -rw-r--r--   1 ivanitskiy  staff  249 Jul 15 13:13 wasm_tutorial.d.ts
    -rw-r--r--   1 ivanitskiy  staff  579 Jul 15 13:13 wasm_tutorial.js
    -rw-r--r--   1 ivanitskiy  staff  291 Jul 15 13:10 wasm_tutorial_bg.js
    -rw-r--r--   1 ivanitskiy  staff  180 Jul 15 13:13 wasm_tutorial_bg.wasm
    -rw-r--r--   1 ivanitskiy  staff  134 Jul 15 13:13 wasm_tutorial_bg.wasm.d.ts


let's use the package on the Web

index.html
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>hello-wasm example</title>
  </head>
  <body>
    <script type="module">
      import init, {greet} from "./pkg/hello_wasm.js";
      init()
        .then(() => {
          greet("WebAssembly")
        });
    </script>
  </body>
</html>

```
##

start a web server to serve an index.html

    python3 -m http.server

more of wasm-pack: https://rustwasm.github.io/docs/wasm-bindgen/print.html

