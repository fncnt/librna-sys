# librna-sys

This crate provides low-level `Rust` bindings to [`libRNA/RNAlib/viennaRNA`](https://www.tbi.univie.ac.at/RNA/).

## Current State

`librna-sys` is **highly experimental** and provides **unsafe low-level bindings**.
It's possible that building or linking does not work reliably. 
This crate was only tested on Linux but macOS might work as well.

## Prerequisites

- Install [`Rust`](https://rustup.rs/).
- Install [`ViennaRNA`](https://www.tbi.univie.ac.at/RNA/#download).

This crate requires the static library (`libRNA.a` on Linux and macOS) as well as the `C` header files.

## Configuration

### Using Environment Variables

`librna-sys` exposes two environment variables in case `ViennaRNA` is installed in a custom directory.
Use them like this:

```sh
export LIBRNA_INCLUDE_DIR=/path/to/headerdirectory # default: /usr/include
export LIBRNA_LIB_DIR=/path/to/librarydirectory # default: /usr/lib
```

Afterwards the crate can be used as a dependency in `cargo.toml`:

```toml
[dependencies]
librna-sys = { git = "https://github.com/fncnt/librna-sys", version = "0.1" }
```

### Using `pkg-config`

If `pkg-config` is available on your system and `ViennaRNA` was installed properly

```toml
[dependencies]
librna-sys = { git = "https://github.com/fncnt/librna-sys", version = "0.1" , features = ["auto"] }
```

may be used instead of setting environment variables.

## Usage

Please refer to the [original documentation](https://www.tbi.univie.ac.at/RNA/ViennaRNA/doc/html/index.html) of the `C` API.
In most cases, you probably want to use the official `Python` bindings.

Use this crate only if some features of the `C` API are not exposed as `Python` bindings and you prefer writing *unsafe* `Rust` over `C` for some reason.

### Example: Extending `ViennaRNA`

*Note: This will probably at some point land in `ViennaRNA`.*

[`examples/bpdist.rs`](examples/bpdist.rs) extends the base pair distance of `ViennaRNA` to secondary structures with pseudoknots.
Building this example by running

```sh
cargo build --release --example bpdist #--features auto
```

produces a dynamic library `target/release/examples/libbpdist.so` exposing `Python` bindings.
Copy it whereever you want and import it like this:

```python
from libbpdist import bp_distance_pk

structures = [".((..[[[..))..]]].", ".((.[.[[..))..]]]."]
print(bp_distance_pk(structures[0], structures[1]))
```

