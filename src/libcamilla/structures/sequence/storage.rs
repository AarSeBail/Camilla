use std::mem::size_of;

use super::nucleotide::Nucleotide;

pub trait Storage: Default {
    const WIDTH: usize;
    // In the case of infinite storage blocks, set to max usize
    // Current implementation does not support variable sized storage
    const CAPACITY: usize;
    fn addr(n: usize) -> (usize, usize);
    fn read(&self, pos: usize) -> Nucleotide;
    fn clear(&mut self, pos: usize);
    fn write(&mut self, pos: usize, value: Nucleotide);
    fn write_complemented(&mut self, pos: usize, value: Nucleotide);
    fn write_chunk<I: Iterator<Item = Nucleotide>>(&mut self, data: I);
}

macro_rules! storage_impl {
    ($($t:ty),+ $(,)?) => { $(
        impl Storage for $t {
            const WIDTH: usize = 2;
            const CAPACITY: usize = size_of::<$t>()*8 / 2;

            #[inline]
            fn addr(n: usize) -> (usize, usize) {
                (n / Self::CAPACITY, n % Self::CAPACITY)
            }

            #[inline]
            fn read(&self, pos: usize) -> Nucleotide {
                ((self >> Self::CAPACITY*Self::WIDTH - Self::WIDTH - Self::WIDTH * pos) & 0b11).into()
            }

            #[inline]
            fn clear(&mut self, pos: usize) {
                *self &= !(0b11 << Self::CAPACITY*Self::WIDTH - Self::WIDTH - Self::WIDTH * pos)
            }

            #[inline]
            fn write(&mut self, pos: usize, value: Nucleotide) {
                self.clear(pos);
                let x: $t = value.into();
                *self |= x << Self::CAPACITY*Self::WIDTH - Self::WIDTH - Self::WIDTH * pos;
            }

            #[inline]
            fn write_complemented(&mut self, pos: usize, value: Nucleotide) {
                Self::write(self, pos, value.complement());
            }

            #[inline]
            fn write_chunk<I: Iterator<Item=Nucleotide>>(&mut self, data: I) {
                for (i, x) in data.enumerate().take(Self::CAPACITY) {
                    self.write(i, x);
                }
            }
        }
    )+ };
}

storage_impl!(u8, u16, u32, u64, u128, usize);

impl Default for Nucleotide {
    fn default() -> Self {
        Nucleotide::T
    }
}

// For PackedSeq's with one byte per base
impl Storage for Nucleotide {
    const WIDTH: usize = 2;
    const CAPACITY: usize = 1;

    #[inline(always)]
    fn addr(n: usize) -> (usize, usize) {
        (n, 0)
    }

    #[inline(always)]
    fn read(&self, _pos: usize) -> Nucleotide {
        *self
    }

    #[inline(always)]
    fn clear(&mut self, _pos: usize) {
        *self = Nucleotide::T;
    }

    #[inline(always)]
    fn write(&mut self, _pos: usize, value: Nucleotide) {
        *self = value;
    }

    fn write_complemented(&mut self, _pos: usize, value: Nucleotide) {
        *self = value.complement();
    }

    #[inline]
    fn write_chunk<I: Iterator<Item = Nucleotide>>(&mut self, data: I) {
        if let Some(x) = data.take(1).next() {
            *self = x;
        }
    }
}
