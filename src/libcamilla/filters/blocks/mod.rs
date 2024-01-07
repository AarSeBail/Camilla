use super::HASH_COUNT;

pub mod blanket;

// Trait definint Blocked Bloom Filter blocks in an architecture dependent paradigm.
// Threadsafe by either atomic operations or spinlocks.
pub trait BBFBlock: Clone + Send + Sync + Default {
    // Insert utilizing interior mutability
    fn insert(&self, hash: usize) -> bool;
    fn insert_all(&self, hashes: [usize; HASH_COUNT]) -> bool;
    fn insert_all_unchecked(&self, hashes: [usize; HASH_COUNT]);
    fn read(&self, hash: usize) -> bool;
    fn read_all(&self, hashes: [usize; HASH_COUNT]) -> bool;
    fn get_density(&self) -> u32;
}
