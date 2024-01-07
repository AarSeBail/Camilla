use std::sync::RwLock;

use crate::filters::{BLOCK_SIZE, HASH_COUNT, NUM_INTS};

use super::BBFBlock;

// Generic implementation of a BBFBlock
pub struct BlanketBBFBlock {
    buffer: RwLock<[u32; NUM_INTS + 1]>,
}

impl Clone for BlanketBBFBlock {
    fn clone(&self) -> Self {
        BlanketBBFBlock {
            buffer: RwLock::new(self.buffer.read().unwrap().clone()),
        }
    }
}

impl Default for BlanketBBFBlock {
    fn default() -> Self {
        BlanketBBFBlock {
            buffer: RwLock::new([0; NUM_INTS + 1]),
        }
    }
}

impl BBFBlock for BlanketBBFBlock {
    #[inline]
    fn insert(&self, hash: usize) -> bool {
        let mut buf = self.buffer.write().unwrap();
        let res = buf[(hash % BLOCK_SIZE) / 32] >> (31 - hash % 32) & 1;
        buf[(hash % BLOCK_SIZE) / 32] |= 1 << (31 - hash % 32);
        buf[NUM_INTS] += (res == 0) as u32;
        res != 0
    }

    #[inline]
    fn insert_all(&self, hashes: [usize; HASH_COUNT]) -> bool {
        let mut buf = self.buffer.write().unwrap();
        let mut res = true;
        for hash in hashes.iter() {
            res &= buf[(hash % BLOCK_SIZE) / 32] >> (31 - hash % 32) & 1 != 0;
            buf[(hash % BLOCK_SIZE) / 32] |= 1 << (31 - hash % 32);
        }
        buf[NUM_INTS] += if res { 0 } else { 1 };
        res
    }

    #[inline]
    fn insert_all_unchecked(&self, hashes: [usize; HASH_COUNT]) {
        let mut buf = self.buffer.write().unwrap();
        for hash in hashes.iter() {
            buf[(hash % BLOCK_SIZE) / 32] |= 1 << (31 - hash % 32);
        }
        buf[NUM_INTS] += 1;
    }

    #[inline]
    fn read(&self, hash: usize) -> bool {
        let buf = self.buffer.read().unwrap();
        buf[(hash % BLOCK_SIZE) / 32] >> (31 - hash % 32) & 1 != 0
    }

    #[inline]
    fn get_density(&self) -> u32 {
        let buf = self.buffer.read().unwrap();
        buf[NUM_INTS]
    }

    #[inline]
    fn read_all(&self, hashes: [usize; HASH_COUNT]) -> bool {
        let buf = self.buffer.read().unwrap();
        let mut res = true;
        for hash in hashes.iter() {
            res &= buf[(hash % BLOCK_SIZE) / 32] >> (31 - hash % 32) & 1 != 0;
        }
        res
    }
}
