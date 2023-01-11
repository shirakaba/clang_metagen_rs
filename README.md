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