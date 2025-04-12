use rust_embed::Embed;

/// A provider for the built `potree` assets so that they can be embedded into a
/// Rust application binary.
///
/// [`potree`]: https://github.com/potree/potree
#[derive(Embed)]
#[folder = "built_assets"]
pub struct PotreeAssets;
