use super::{nucleotide::Nucleotide, read::ReadSeq, storage::Storage};
use crate::structures::sequence::complement::{Complementation, Forward, Reversal, Reverse};
use std::{fmt::Display, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct PackedSeq<T: Storage, R: Reversal, C: Complementation> {
    storage: Vec<T>,
    len: usize,
    _r: PhantomData<R>,
    _c: PhantomData<C>,
}

impl<T, R, C> PackedSeq<T, R, C>
where
    T: Storage,
    R: Reversal,
    C: Complementation,
{
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
            len: 0,
            _r: PhantomData,
            _c: PhantomData,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn with_capacity(n: usize) -> Self {
        Self {
            // Better than calculating directly,
            // if in the future we would prefer a Storage
            // implementation having infinite/variable capacity
            storage: Vec::with_capacity(T::addr(n).0 + 1),
            len: 0,
            _r: PhantomData,
            _c: PhantomData,
        }
    }

    pub fn from_read(read: &ReadSeq) -> Self {
        let s = &read.sequence;
        let mut res = Self::with_capacity(s.len());
        let (slots, _) = T::addr(s.len() - 1);
        res.storage.resize_with(slots + 1, T::default);
        let mut chunks = s.as_bytes().chunks_exact(T::CAPACITY);
        for (i, chunk) in chunks.by_ref().enumerate() {
            res.storage[i].write_chunk(chunk.iter().map(Nucleotide::from_ascii));
        }
        res.storage[slots].write_chunk(chunks.remainder().iter().map(Nucleotide::from_ascii));
        res.len = s.len();
        res
    }

    pub fn read(&self, n: usize) -> Option<Nucleotide> {
        if n < self.len {
            let (slot, pos) = T::addr(R::reindex(self.len, n));

            Some(C::translate(self.storage[slot].read(pos)))
        } else {
            None
        }
    }

    pub fn write(&mut self, n: usize, value: Nucleotide) {
        if n < self.len {
            let (slot, pos) = T::addr(n);
            self.storage[slot].write(pos, C::translate(value));
        } else {
            println!("Failed check! {}", n);
        }
    }

    pub fn write_positional(&mut self, n: usize, value: Nucleotide) {
        if n < self.len {
            let (slot, pos) = T::addr(n);
            self.storage[slot].write(pos, C::translate(value));
        }
    }

    pub fn iter(&self) -> PackedSeqIter<'_, T, R, C> {
        PackedSeqIter {
            seq: self,
            index: 0,
        }
    }

    pub fn extend<I: Iterator<Item = Nucleotide>>(&mut self, x: I) {
        for y in x {
            self.push(y);
        }
    }

    #[inline]
    pub fn push(&mut self, value: Nucleotide) {
        let (s, pos) = T::addr(self.len);
        println!("{} {}", s, pos);
        self.len += 1;
        if pos == 0 {
            self.storage.push(T::default());
        }
        self.write(self.len - 1, value);
    }

    #[inline]
    pub fn reverse_complement(self) -> PackedSeq<T, R::Inverse, C::Inverse> {
        PackedSeq::<T, R::Inverse, C::Inverse> {
            // Noop
            storage: self.storage.into_iter().map(|x| x.into()).collect(),
            len: self.len,
            _r: PhantomData,
            _c: PhantomData,
        }
    }

    #[inline]
    pub fn as_slice(&self) -> PackedSeqSlice<'_, T, R, C> {
        PackedSeqSlice {
            seq: self,
            start: 0,
            len: self.len,
        }
    }

    #[inline]
    pub fn slice(&self, start: usize, len: usize) -> PackedSeqSlice<'_, T, R, C> {
        PackedSeqSlice {
            seq: self,
            start,
            len,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PackedSeqIter<'a, T, R, C>
where
    T: Storage,
    R: Reversal,
    C: Complementation,
{
    seq: &'a PackedSeq<T, R, C>,
    index: usize,
}

impl<'a, T, R, C> Iterator for PackedSeqIter<'a, T, R, C>
where
    T: Storage,
    R: Reversal,
    C: Complementation,
{
    type Item = Nucleotide;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.seq.read(self.index - 1)
    }
}

impl<'a, T, R, C> DoubleEndedIterator for PackedSeqIter<'a, T, R, C>
where
    T: Storage,
    R: Reversal,
    C: Complementation,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        // println!("Iter: {}", self.index);
        if self.index == self.seq.len {
            return None;
        }
        self.index += 1;
        self.seq.read(self.index)
    }
}

impl<'a, T, R, C> IntoIterator for &'a PackedSeq<T, R, C>
where
    T: Storage,
    R: Reversal,
    C: Complementation,
{
    type Item = Nucleotide;

    type IntoIter = PackedSeqIter<'a, T, R, C>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PackedSeqSlice<'a, T, R, C>
where
    T: Storage,
    R: Reversal,
    C: Complementation,
{
    pub seq: &'a PackedSeq<T, R, C>,
    pub start: usize,
    pub len: usize,
}

impl<'a, T, R, C> PackedSeqSlice<'a, T, R, C>
where
    T: Storage,
    R: Reversal,
    C: Complementation,
{
    pub fn get(&self, n: usize) -> Nucleotide {
        self.seq.read(self.start + n).unwrap()
    }
}

impl<'a, T, R, C> Display for PackedSeqSlice<'a, T, R, C>
where
    T: Storage,
    R: Reversal,
    C: Complementation,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[start: {}, len: {}, {:?}]",
            self.start,
            self.len,
            (0..self.len)
                .map(|i| self.get(i))
                .collect::<Vec<Nucleotide>>()
        )
    }
}

// For integers it should be preferred to do this directly using pointer arithmetic
// If repacking becomes common, this should be optimized
macro_rules! extension_repack {
    ($a:ty, $b:ty) => {
        impl<R1, C1, C2> From<PackedSeq<$a, R1, C1>> for PackedSeq<$b, Forward, C2>
        where
            R1: Reversal,
            C1: Complementation,
            C2: Complementation,
        {
            #[inline]
            fn from(seq: PackedSeq<$a, R1, C1>) -> PackedSeq<$b, Forward, C2> {
                let mut res = Self::with_capacity(seq.len);
                res.extend(seq.iter());
                res
            }
        }

        impl<R1, C1, C2> From<PackedSeq<$a, R1, C1>> for PackedSeq<$b, Reverse, C2>
        where
            R1: Reversal,
            C1: Complementation,
            C2: Complementation,
        {
            #[inline]
            fn from(seq: PackedSeq<$a, R1, C1>) -> PackedSeq<$b, Reverse, C2> {
                let mut res = Self::with_capacity(seq.len);
                res.extend(seq.iter().rev());
                res
            }
        }
    };
}

macro_rules! repack_nonbranching {
    ($a:ty, $b:ty) => {
        extension_repack!($a, $b);
        extension_repack!($b, $a);
    };
    ($a:ty, $b:ty, $($rest:ty),+) => {
        repack_nonbranching!($a, $b);
        repack_nonbranching!($a, $($rest),*);
    }
}

macro_rules! repack_by_extension {
    ($a:ty, $b:ty) => {
        repack_nonbranching!($a, $b);
    };
    ($a:ty, $b:ty, $($rest:ty),+) => {
        repack_by_extension!($a, $b);
        repack_nonbranching!($a, $($rest),*);
        repack_by_extension!($b, $($rest),*);
    }
}

repack_by_extension!(u8, u16, u32, u64, u128, usize, Nucleotide);
