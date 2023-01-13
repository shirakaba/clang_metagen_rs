This is a tool intended to help call C/Obj-C APIs from other languages such as JS.

Aims:

* Find all the C declarations in header files (e.g. for `CGMakeRect` and `NSTitledWindowMask`)
* Write out some metadata that expresses the same thing as the C declaration but in a serialisable format
* Learn some Rust (I could do this in JS via Deno instead)

Through this, we'll be able to bind any symbols that Obj-C helper APIs can't already bind for us.


Resources:

https://doc.rust-lang.org/cargo/getting-started/first-steps.html
https://doc.rust-lang.org/book/ch01-02-hello-world.html

How to run:

```sh
# To compile (if necessary) and run:
cargo run

# To just compile:
cargo build
```

# Environment variables

https://github.com/KyleMayes/clang-sys/blob/master/README.md says that the following environment variables are observed:

```
# Compile-time: The path to `libclang.dylib` (or its containing folder)
LIBCLANG_PATH=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib

# Runtime: the path to a `clang` executable
CLANG_PATH=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang
```

... However, I found that the environment variables were completely ignored when attempting `cargo test`, so I ended up linking Apple/Xcode clang (which is v14.0.0) into one of the directories on its search path, `/usr/local/lib`.

```sh
sudo ln -sfn /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/libclang.dylib /usr/local/lib/libclang.dylib
```

# Licence

I'm including some of the [clang-sys](https://github.com/KyleMayes/clang-sys) tests as a starting point (which come under Apache 2.0 - see `LICENSE-clang-sys`), but my source code is MIT (see `LICENSE`).
