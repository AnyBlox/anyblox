pub mod keys {
    pub const TYPE: &str = "ANYBLOX:TYPE";
    pub const VERSION: &str = "ANYBLOX:VERSION";

    pub mod decoder {
        pub const URI: &str = "ANYBLOX:DECODER:URI";
        pub const DESCRIPTION: &str = "ANYBLOX:DECODER:DESCRIPTION";
        pub const LICENSE: &str = "ANYBLOX:DECODER:LICENSE";
        pub const CHECKSUM_BLAKE3: &str = "ANYBLOX:DECODER:CHECKSUM_BLAKE3";
        pub const MIN_BATCH_SIZE: &str = "ANYBLOX:DECODER:MIN_BATCH_SIZE";
    }

    pub mod data {
        pub const NAME: &str = "ANYBLOX:DATA:NAME";
        pub const DESCRIPTION: &str = "ANYBLOX:DATA:DESCRIPTION";
        pub const COUNT: &str = "ANYBLOX:DATA:COUNT";
        pub const SIZE_IN_BYTES: &str = "ANYBLOX:DATA:SIZE_IN_BYTES";
    }
}