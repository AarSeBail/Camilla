//

use crate::structures::sequence::{
    complement::{Complementation, Reversal},
    packed::{PackedSeq, PackedSeqSlice},
    storage::Storage,
};

pub type BucketHashes = (usize, usize);

pub trait BucketHashExt<'a, T: Storage, R: Reversal, C: Complementation> {
    fn from_kmer(kmer: &PackedSeqSlice<'a, T, R, C>) -> BucketHashes;
}

impl<'a, T: Storage, R: Reversal, C: Complementation> BucketHashExt<'a, T, R, C> for BucketHashes {
    fn from_kmer(kmer: &PackedSeqSlice<'a, T, R, C>) -> BucketHashes {
        let acc = (0..kmer.len).into_iter().fold(0, |acc: usize, i| {
            acc.wrapping_mul(4).wrapping_add(kmer.get(i) as usize)
        });
        let mask = (1 << 2 * kmer.len) - 1;
        (acc & mask, acc & mask)
    }
}

// Iterator over fingerprints of all k-mers in a sequence slice
// Hashes should correspond to the 8-mer maximizer of the k-mer
#[derive(Debug)]
pub struct BucketHashIter<'a, T: Storage, R: Reversal, C: Complementation> {
    data: PackedSeqSlice<'a, T, R, C>,
    pos: usize,
    window_size: usize,
    buffer: usize,
    mask: usize,
}

impl<'a, T: Storage, R: Reversal, C: Complementation> BucketHashIter<'a, T, R, C> {
    pub fn new(data: PackedSeqSlice<'a, T, R, C>, window_size: usize) -> Self {
        let acc = (0..window_size - 1).into_iter().fold(0, |acc: usize, i| {
            acc.wrapping_mul(4).wrapping_add(data.get(i) as usize)
        });

        Self {
            data,
            pos: window_size - 1,
            window_size,
            buffer: acc,
            mask: (1 << 2 * window_size) - 1,
        }
    }
}

impl<'a, T: Storage, R: Reversal, C: Complementation> Iterator for BucketHashIter<'a, T, R, C> {
    type Item = BucketHashes;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("Iter: {}", self.pos);
        if self.pos >= self.data.len {
            println!("A {}", self.pos);
            return None;
        }
        self.buffer = self
            .buffer
            .wrapping_mul(4)
            .wrapping_add(self.data.get(self.pos) as usize);
        self.buffer &= self.mask;
        self.pos += 1;

        println!(
            "Bucket Hash {:?} {:b}",
            self.data.get(self.pos - 1),
            self.buffer
        );

        Some((self.buffer, self.buffer))
    }
}

impl<T, R, C> PackedSeq<T, R, C>
where
    T: Storage,
    R: Reversal,
    C: Complementation,
{
    pub fn bucket_hash_iter(&self, window_size: usize) -> BucketHashIter<T, R, C> {
        BucketHashIter::new(self.as_slice(), window_size)
    }
}
