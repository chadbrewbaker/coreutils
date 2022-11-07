# Testing with KLEE

[Rust guide to compiling to llvm-ir](https://rustc-dev-guide.rust-lang.org/backend/debugging.html)

[Graalvm guide to compiling Rust to llvm-ir](https://www.graalvm.org/22.2/reference-manual/llvm/Compiling/)

[Klee guide to testing coreutils](https://klee.github.io/tutorials/testing-coreutils/)

Last known klee complatable nightly is 1 January 2022?

```bash
RUSTFLAGS='--emit=llvm-ir' cargo build -Z build-std 
```
