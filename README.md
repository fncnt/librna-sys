# librna-sys

This crate provides low-level C bindings to [`libRNA/RNAlib/viennaRNA`](https://www.tbi.univie.ac.at/RNA/).

## Current State

`librna-sys` is **highly experimental**.
It's possible building or linking does not work reliably. 

~~~See [`librna-rs`](https://github.com/fncnt/librna-rs) for equally experimental safe Rust bindings.~~~

## Configuration

Use

```
cargo build --features auto
```

to let `pkg-config` search for the library and header files.
By default, this feature is disabled and the build script uses the default system `lib` and `include` directories.

If you've built `libRNA.a` manually in a local directory, 
set `LIBRNA_INCLUDE_DIR` and `LIBRNA_LIB_DIR` to override these directories.

## Contributing

I'm still new to Rust. 
If you know better and have any kind of good advice, feel free to let me know!

### TODO
* [ ] Tests (manually and/or using `ctest`)
* [ ] Fallback mode when building (using `git2-rs` to download ViennaRNA **or** using a submodule)
* [ ] pre-generate bindings using bindgen CLI
* [ ] Publish crate
* [ ] Publish `librna-rs` on Github and link to it.
