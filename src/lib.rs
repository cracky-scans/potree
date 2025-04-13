#![doc = include_str!("../README.md")]

use rust_embed::Embed;

/// A provider for the built `potree` assets so that they can be embedded into a
/// Rust application binary.
///
/// [`potree`]: https://github.com/potree/potree
#[derive(Embed)]
#[folder = "built_assets"]
pub struct PotreeAssets;

#[cfg(test)]
mod potree_assets_tests {
    use super::*;

    #[test]
    fn should_return_the_correct_asset() {
        // Act
        let asset = PotreeAssets::get("build/potree/potree.js");

        // Assert
        assert!(asset.is_some());
        assert_eq!(
            asset.unwrap().metadata.sha256_hash(),
            [
                233, 119, 224, 43, 52, 46, 99, 100, 218, 253, 246, 139, 172, 68, 39, 198, 242, 64,
                255, 127, 185, 52, 69, 21, 38, 118, 221, 238, 4, 116, 172, 95
            ]
        );
    }

    #[test]
    fn should_return_none_for_non_existent_asset() {
        // Act
        let asset = PotreeAssets::get("build/no/asset.txt");

        // Assert
        assert!(asset.is_none());
    }
}
