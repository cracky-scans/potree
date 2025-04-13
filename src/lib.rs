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
                206, 129, 9, 224, 88, 142, 244, 214, 10, 62, 119, 242, 156, 117, 160, 183, 125, 88,
                114, 11, 46, 172, 148, 58, 30, 13, 14, 129, 60, 135, 150, 242
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
