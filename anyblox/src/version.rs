pub use anyblox_format::model::AnyBloxVersion;

pub const ANYBLOX_API_VERSION: AnyBloxVersion = AnyBloxVersion::V0_10;

pub trait AnyBloxVersionCompatibility {
    fn can_run_bundle(&self, bundle_version: AnyBloxVersion) -> bool;
}

impl AnyBloxVersionCompatibility for AnyBloxVersion {
    fn can_run_bundle(&self, bundle_version: AnyBloxVersion) -> bool {
        // Currently all changes are breaking.
        bundle_version.eq(self)
    }
}
