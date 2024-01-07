pub mod blocks;
pub mod bloom;
pub mod bucket_hashes;
pub mod rolling_hash;

pub const NUM_INTS: usize = 127;
pub const BLOCK_SIZE: usize = 32 * NUM_INTS;
pub const HASH_COUNT: usize = 1;
pub const MAXIMIZER_LENGTH: usize = 8;

const_assert!(HASH_COUNT <= 8);
