use crate::{
    filters::{bucket_hashes::BucketHashes},
    structures::sequence::{
        complement::{Complementation, Reversal},
        packed::{PackedSeq, PackedSeqSlice},
        storage::Storage,
    },
};

use super::{blocks::*, rolling_hash::RollingHashExt, rolling_hash::RollingHashes};

use super::bucket_hashes::BucketHashExt;

use super::BLOCK_SIZE;

// Blocked Bloom Filter
pub struct BBFilter<B: BBFBlock> {
    blocks: Vec<B>,
    block_count: usize,
}

impl<B: BBFBlock> BBFilter<B> {
    pub fn new(num_keys: usize, bits_per_key: usize) -> Self {
        let size = num_keys * bits_per_key;
        let block_count = (size + BLOCK_SIZE - 1) / BLOCK_SIZE;
        Self {
            blocks: (0..size).map(|_| B::default()).collect(),
            block_count,
        }
    }

    pub fn insert_kmers<T, R, C>(&self, seq: &PackedSeq<T, R, C>, window_size: usize)
    where
        T: Storage,
        R: Reversal,
        C: Complementation,
    {
        // TODO: better hashing
        let bhashes = seq.bucket_hash_iter(window_size);

        let hashes = seq.rolling_hash_iter(window_size);

        for ((mut b1, mut b2), hashes) in bhashes.zip(hashes) {
            if b2 < b1 {
                std::mem::swap(&mut b1, &mut b2);
            }
            let block1 = &self.blocks[(b1 / BLOCK_SIZE) % self.block_count];
            let block2 = &self.blocks[(b2 / BLOCK_SIZE) % self.block_count];
            if !block1.read_all(hashes) {
                if !block2.read_all(hashes) {
                    if block1.get_density() <= block2.get_density() {
                        println!("Write to block {}", (b1 / BLOCK_SIZE) % self.block_count);
                        block1.insert_all_unchecked(hashes);
                    } else {
                        println!("Write to block {}", (b2 / BLOCK_SIZE) % self.block_count);
                        block2.insert_all_unchecked(hashes);
                    }
                }
            }
        }
    }

    pub fn contains_kmer<'a, T, R, C>(&self, kmer: PackedSeqSlice<'a, T, R, C>) -> bool
    where
        T: Storage,
        R: Reversal,
        C: Complementation,
    {
        let bhash = BucketHashes::from_kmer(&kmer);

        let block1 = &self.blocks[(bhash.0 / BLOCK_SIZE) % self.block_count];
        let block2 = &self.blocks[(bhash.1 / BLOCK_SIZE) % self.block_count];

        let hashes = RollingHashes::from_kmer(&kmer);

        println!("Attempted Read: {:b} {:b} {}", bhash.0, bhash.1, hashes[0]);

        block1.read_all(hashes) || block2.read_all(hashes)
    }
}
