# CLONED_TRACKER_DNA MODULE

Small zome to create and retrieve dna clones, in holochain RSM.

This module is designed to be included in other DNAs, assuming as little as possible from those. It is packaged as a holochain zome,

## Documentation

See our [`specification`](https://hackmd.io/BKUneWqYSOOgit6ZgKNYcQ?view).

## Installation and usage

### Including the zome in your DNA

1. Create a new folder in the `zomes` of the consuming DNA, with the name you want to give to this zome in your DNA.
2. Add a new `Cargo.toml` in that folder. In its content, paste the `Cargo.toml` content from any zome.
3. Change the `name` properties of the `Cargo.toml` file to the name you want to give to this zome in your DNA.
4. Add this zome as a dependency in the `Cargo.toml` file:
```toml
[dependencies]
cloned_dna_tracker = {git = "https://github.com/holochain-open-dev/cloned-dnas-tracker", package = "cloned_dna_tracker"}
```
5. Create a `src` folder besides the `Cargo.toml` with this content:
```rust
extern crate cloned_dna_tracker;
```
6. Add the zome into your `*.dna.workdir/dna.json` file.
7. Compile the DNA with the usual `CARGO_TARGET=target cargo build --release --target wasm32-unknown-unknown`.

## Developer setup

This respository is structured in the following way:

- `zome/`: example DNA with the `cloned_dna_tracker` code.
- Top level `Cargo.toml` is a virtual package necessary for other DNAs to include this zome by pointing to this git repository.

Read the [Zome developer setup](/zome/README.md).
