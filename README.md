# wasm-pack-workspace
> a minimal repro of `wasm-pack build` fails with `missing field 'package'`

Running `wasm-pack build` at the root fails with this error:

```sh
$ wasm-pack build
Error: failed to parse manifest: ./Cargo.toml
Caused by: missing field `package`
```

Running `cargo build` appears to work as expected (`cargo check` and `cargo test` also run and report success):

```sh
$ cargo build
   Compiling proc-macro2 v0.4.29
   Compiling unicode-xid v0.1.0
   Compiling syn v0.15.33
   Compiling autocfg v0.1.2
   Compiling version_check v0.1.5
   Compiling libc v0.2.54
   Compiling memchr v2.2.0
   Compiling wasm-bindgen-shared v0.2.43
   Compiling cfg-if v0.1.7
   Compiling failure_derive v0.1.5
   Compiling rustc-demangle v0.1.14
   Compiling bumpalo v2.4.1
   Compiling lazy_static v1.3.0
   Compiling ucd-util v0.1.3
   Compiling regex v1.1.6
   Compiling utf8-ranges v1.0.2
   Compiling unicode-segmentation v1.2.1
   Compiling quick-error v1.2.2
   Compiling termcolor v1.0.4
   Compiling wasm-bindgen v0.2.43
   Compiling sourcefile v0.1.4
   Compiling log v0.4.6
   Compiling thread_local v0.3.6
   Compiling regex-syntax v0.6.6
   Compiling nom v4.2.3
   Compiling backtrace v0.3.15
   Compiling humantime v1.2.0
   Compiling heck v0.3.1
   Compiling aho-corasick v0.7.3
   Compiling atty v0.2.11
   Compiling quote v0.6.12
   Compiling weedle v0.8.0
   Compiling env_logger v0.6.1
   Compiling synstructure v0.10.1
   Compiling wasm-bindgen-backend v0.2.43
   Compiling failure v0.1.5
   Compiling wasm-bindgen-macro-support v0.2.43
   Compiling wasm-bindgen-webidl v0.2.43
   Compiling wasm-bindgen-macro v0.2.43
   Compiling web-sys v0.3.20
   Compiling js-sys v0.3.20
   Compiling example-01 v0.1.0 (/Users/matt/Sites/mysterycommand/wasm-pack-workspace/src/examples/example-01)
   Compiling example-02 v0.1.0 (/Users/matt/Sites/mysterycommand/wasm-pack-workspace/src/examples/example-02)
    Finished dev [unoptimized + debuginfo] target(s) in 35.72s
```

Some env. info:

```sh
$ rustup --version
rustup 1.18.2 (a0bf3c9cb 2019-05-02)

$ rustc --version
rustc 1.34.1 (fc50f328b 2019-04-24)

$ cargo --version
cargo 1.34.0 (6789d8a0a 2019-04-01)

$ wasm-pack --version
wasm-pack 0.8.1

$ wasm-bindgen --version
wasm-bindgen 0.2.43
```
