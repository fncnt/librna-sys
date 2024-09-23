[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.12543036.svg)](https://doi.org/10.5281/zenodo.12543036)

# librna-sys

This crate provides low-level Rust bindings to [`libRNA/RNAlib/ViennaRNA`](https://www.tbi.univie.ac.at/RNA/).

## Current State

`librna-sys` is mostly **experimental** and provides **unsafe low-level bindings**.
You might encounter issues with building or linking this crate but common installation setups of ViennaRNA should work reliably.
This crate was only tested on Linux. MacOS might work, possibly with some adjustments.

### Compatibility

| `librna-sys` | ViennaRNA | Notes |
| ------------ | --------- | ----- |
| `<=0.1.4` | `<= 2.4.18` | Bindings to `ViennaRNA/pk_plex.h` introduced in `2.4.18` were not included |
| `0.1.5` | `<=2.5.*` | Bindings to new headers `ViennaRNA/pf_multifold.h`, `ViennaRNA/subopt_zuker.h` and `ViennaRNA/wrap_dlib.h` not included |
| `0.1.6-0.1.7` | `<=2.6.0` | see above |
| `0.2.0` | `2.6.0 - 2.6.2` | Added new header files introduced since 2.4.18 through 2.6.0 |
| `0.2.1` | `>=2.6.3` | Removed workaround for issue fixed in [#189](https://github.com/ViennaRNA/ViennaRNA/pull/189). |
| `0.2.2-0.2.3` | `>=2.6.4` | Re-added `vrna_config.h` which was removed in `0.2.1` as a workaround for an issue fixed in [#199](https://github.com/ViennaRNA/ViennaRNA/pull/199) |
| `>=0.3.0` | **`>=2.5.0`** | Issue [#3](https://github.com/fncnt/librna-sys/issues/3) should be resolved. Added new header files for `2.7.0` |

This chart might be inconsinstent and incomplete. Please report any inaccuracies.
Since `librna-sys@0.2.0`, new minor versions have been released in lockstep with new minor versions of ViennaRNA.
In general, only the latest version of ViennaRNA is supported but please reach out if you try to make an older version work.

**`librna-sys>=0.3.0` restores backwards-compatibility with ViennaRNA `>=2.5.0`** (cf. below for known issues).

## Prerequisites

- Install [Rust](https://rustup.rs/).
- Install [ViennaRNA](https://www.tbi.univie.ac.at/RNA/#download).

This crate requires the static library (`libRNA.a` on Linux and macOS) as well as the C header files.

## Configuration

### Using `pkg-config` (default method)

If `pkg-config` is available on your system and ViennaRNA was installed properly

```toml
[dependencies]
librna-sys = "0.3"
```

suffices to automatically set the correct linking options.
If ViennaRNA is installed into a custom prefix that `pkg-config` is *not* aware of,
you can use environment variables to tell `pkg-config` where to look:

```sh
export LIBRNA_PREFIX=/not/usr # for convenience

export PKG_CONFIG_PATH="$PKG_CONFIG_PATH:$LIBRNA_PREFIX/lib/pkgconfig"
```

### Using Environment Variables

`librna-sys` exposes two environment variables in case `pkg-config` is not available or 
the default `auto` feature is [explicitly disabled](https://doc.rust-lang.org/cargo/reference/features.html#the-default-feature).
Use them like this:

```sh
export LIBRNA_INCLUDE_DIR=$LIBRNA_PREFIX/include # default: /usr/include
export LIBRNA_LIB_DIR=$LIBRNA_PREFIX/lib # default: /usr/lib
```

However, note that this approach is limited and assumes a certain set of linker arguments.
This might cause problems if your ViennaRNA configuration is unusual.
If possible, prefer using `pkg-config` via the default cargo features.

## Usage

Please refer to the [original documentation](https://www.tbi.univie.ac.at/RNA/ViennaRNA/doc/html/index.html) of the C API.
In most cases, you probably want to use the official Python bindings.

Use this crate only if some features of the C API are not exposed as Python bindings and you prefer writing *unsafe* Rust over C for some reason.

### Example: Extending ViennaRNA

[*Note: As of version `2.5.0`, ViennaRNA contains support for the base pair distance with pseudoknots.*](https://github.com/ViennaRNA/ViennaRNA/pull/129)

This example is intended to illustrate how `librna-sys` could be used to build upon ViennaRNA.

[`examples/bpdist.rs`](examples/bpdist.rs) extends the base pair distance of ViennaRNA to secondary structures with pseudoknots.
Building this example by running

```sh
cargo build --release --example bpdist #--features auto
```

produces a dynamic library `target/release/examples/libbpdist.so` exposing `Python` bindings.
Copy it wherever you want and import it like this:

```python
from libbpdist import bp_distance_pk

structures = [".((..[[[..))..]]].", ".((.[.[[..))..]]]."]
print(bp_distance_pk(structures[0], structures[1]))
```
## Contributions

I'm open to any ideas or advice.

It is not necessarily planned to provide *complete, safe* bindings to ViennaRNA.
However, I would like to restructure this project slightly and add a crate `librna-rs`
providing a safely wrapped subset of ViennaRNA functionality that can be extended when the need arises.

## Known Issues

If you encounter an error including `generated with LTO version X.0 instead of the expected Y.0`,
you could either recompile ViennaRNA yourself or [downgrade your `Rust` toolchain](https://doc.rust-lang.org/rustc/linker-plugin-lto.html#toolchain-compatibility).
Adjusting some [linker-related codegen options](https://doc.rust-lang.org/rustc/codegen-options/index.html#linker) might also help but was not thoroughly tested.

### Linking OpenMP

ViennaRNA is linked against OpenMP both internally and in its bundled `libsvm` dependency.
**Prior to version `2.7.0`**, this could cause linking issues with the Rust compiler.

This can be solved by adding [`openmp-sys`](https://crates.io/crates/openmp-sys) in downstream Rust code.
