The success of linking C-libraries during `cargo build` seems to depend on the existence of `lib.rs` even if only the binary is built.
Building packages which offer both a library and binary could suffer from this potential bug. This bug has been only been reproduced so far for linking against a library via
```rust
println!("cargo:rustc-link-search=../libs");
println!("cargo:rustc-link-lib=static=mylib");
```
inside `build.rs`.

**Exptected behavior**: The existence of a file called `lib.rs` should have no effect on the linking during `cargo build`:

**Observed behavior**:
1. Without the existence of `lib.rs`: Linking to custom C-library does work (see `./linking-without-lib-works` for working example)
2. Withthe existence of `lib.rs`: Linking to custom C-library does not work (see `./linking-with-lib-doesnt` for working example)
