[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.12543036.svg)](https://doi.org/10.5281/zenodo.12543036)

# librna-sys

This crate provides low-level `Rust` bindings to [`libRNA/RNAlib/ViennaRNA`](https://www.tbi.univie.ac.at/RNA/).

## Current State

`librna-sys` is **highly experimental** and provides **unsafe low-level bindings**.
It's possible that building or linking does not work reliably. 
This crate was only tested on Linux but macOS should work as well.

### Compatibility

| `librna-sys` | ViennaRNA | Notes |
| ------------ | --------- | ----- |
| `<=0.1.4` | `<= 2.4.18` | Bindings to `ViennaRNA/pk_plex.h` introduced in `2.4.18` were not included |
| `0.1.5` | `<=2.5.*` | Bindings to new headers `ViennaRNA/pf_multifold.h`, `ViennaRNA/subopt_zuker.h` and `ViennaRNA/wrap_dlib.h` not included |
| `0.1.6-0.1.7` | `<=2.6.0` | see above |
| `0.2.0` | `2.6.0 - 2.6.2` | Added new header files introduced since 2.4.18 through 2.6.0 |
| `0.2.1` | `>=2.6.3` | Removed workaround for issue fixed in [#189](https://github.com/ViennaRNA/ViennaRNA/pull/189). |
| `>=0.2.2` | `>=2.6.4` | Re-added `vrna_config.h` which was removed in `0.2.1` as a workaround for an issue fixed in [#199](https://github.com/ViennaRNA/ViennaRNA/pull/199) |

This chart might be inconsinstent and incomplete. Please report any inaccuracies.
In general, only the latest version of ViennaRNA is supported but please reach out if you try to make an older version work.
Starting with `librna-sys@0.2.0`, I aim to release new minor versions in lockstep with new backwards-incompatible versions of ViennaRNA.

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

Afterwards the crate can be used as a dependency in `Cargo.toml`:

```toml
[dependencies]
librna-sys = "0.2"
```

### Using `pkg-config`

If `pkg-config` is available on your system and `ViennaRNA` was installed properly

```toml
[dependencies]
librna-sys = { version = "0.2" , features = ["auto"] }
```

may be used instead of setting environment variables.

## Usage

Please refer to the [original documentation](https://www.tbi.univie.ac.at/RNA/ViennaRNA/doc/html/index.html) of the `C` API.
In most cases, you probably want to use the official `Python` bindings.

Use this crate only if some features of the `C` API are not exposed as `Python` bindings and you prefer writing *unsafe* `Rust` over `C` for some reason.

### Example: Extending `ViennaRNA`

[*Note: As of version `2.5.0`, `ViennaRNA` contains support for the base pair distance with pseudoknots.*](https://github.com/ViennaRNA/ViennaRNA/pull/129)

This example is intended to illustrate how `librna-sys` could be used to build upon `ViennaRNA`.

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
## Contributions

I'm open to any ideas or advice.
At this point, it's not yet clear where this is going but here are a few thoughts:

- Providing complete *safe* bindings to `ViennaRNA` is probably as complex as a complete rewrite in `Rust`.
- Perhaps a separate crate could serve as a central collection of *safe* APIs extending `ViennaRNA`.

## Known Issues

If you encounter an error including `generated with LTO version X.0 instead of the expected Y.0`,
you could either recompile `ViennaRNA` yourself or [downgrade your `Rust` toolchain](https://doc.rust-lang.org/rustc/linker-plugin-lto.html#toolchain-compatibility).
Adjusting some [linker-related codegen options](https://doc.rust-lang.org/rustc/codegen-options/index.html#linker) might also help but was not thoroughly tested.
