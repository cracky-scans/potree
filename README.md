# Potree Assets

Builds and bundles the built assets from the [`potree`](https://github.com/potree/potree) project with [`rust_embed`](https://crates.io/crates/rust-embed) so that they can be embedded into a Rust application binary.

## Architecture

The `potree` JS code is built and copied in to the [`built_assets`](./built_assets/) directory by the [`build.rs`](./build.rs).

## Usage

### Prerequisites

Because `potree` needs to be built [NodeJS](https://nodejs.org/en) needs to be installed on the build system.

### Examples

```rs
use potree_embed::PotreeAssets;

let asset = PotreeAssets::get("build/potree/potree.js").unwrap();
let asset_bytes = asset.data;
```