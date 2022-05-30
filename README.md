# librna-rs

This crate provides a higher-level API wrapper around [`librna-sys`](https://github.com/fncnt/librna-rs/librna-sys/).

## Current State

`librna` is **highly experimental** and **incomplete**.
It's possible that building or linking does not work reliably. 
This crate was only tested on Linux but macOS should work as well.

## Prerequisites

Please make sure to install [`ViennaRNA`](https://www.tbi.univie.ac.at/RNA/#download) as this crate requires the static library (`libRNA.a` on Linux and macOS) as well as the `C` header files.

## Configuration

### Using Environment Variables

`librna` exposes two environment variables in case `ViennaRNA` is installed in a custom directory.
Use them like this:

```sh
export LIBRNA_INCLUDE_DIR=/path/to/headerdirectory # default: /usr/include
export LIBRNA_LIB_DIR=/path/to/librarydirectory # default: /usr/lib
```

Afterwards the crate can be used as a dependency in `Cargo.toml`:

```toml
[dependencies]
librna = "0.1"
```

### Using `pkg-config`

If `pkg-config` is available on your system and `ViennaRNA` was installed properly

```toml
[dependencies]
librna = { version = "0.1" , features = ["auto"] }
```

may be used instead of setting environment variables.

## Usage

Please refer to the [original documentation](https://www.tbi.univie.ac.at/RNA/ViennaRNA/doc/html/index.html) of the `C` API
where this crate lacks in [documentation](https://docs.rs/crate/librna/latest) or high-level bindings.

This crate re-exports the low-level FFI bindings `librna-sys`.
