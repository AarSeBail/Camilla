use super::{nucleotide::Nucleotide, read::ReadSeq, storage::{Storage, ReverseComplement}};

pub struct PackedSeq<T: Storage> {
    storage: Vec<T>,
    len: usize
}

impl<T> PackedSeq<T>
where
    T: Storage,
{
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
            len: 0,
        }
    }

    pub fn with_capacity(n: usize) -> Self {
        Self {
            // Better than calculating directly,
            // if in the future we would prefer a Storage
            // implementation having infinite/variable capacity
            storage: Vec::with_capacity(T::addr(n).0 + 1),
            len: 0,
        }
    }

    pub fn from_read(read: &ReadSeq) -> Self {
        let s = &read.sequence;
        let mut res = Self::with_capacity(s.len());
        let (slots, _) = T::addr(s.len() - 1);
        res.storage.resize_with(slots+1, T::default);
        let mut chunks = s.as_bytes().chunks_exact(T::CAPACITY);
        for (i, chunk) in chunks.by_ref().enumerate() {
            res.storage[i].write_chunk(chunk.iter().map(Nucleotide::from_ascii));
        }
        res.storage[slots].write_chunk(chunks.remainder().iter().map(Nucleotide::from_ascii));
        res.len = s.len();
        res
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn read(&self, n: usize) -> Option<Nucleotide> {
        if n < self.len {
            let (slot, pos) = T::addr(T::reindex(self.len, n));

            Some(self.storage[slot].read(pos))
        } else {
            None
        }
    }

    pub fn write(&mut self, n: usize, value: Nucleotide) {
        if n < self.len {
            let (slot, pos) = T::addr(n);
            self.storage[slot].write(pos, value);
        }
    }

    pub fn iter(&self) -> PackedSeqIter<'_, T> {
        PackedSeqIter {
            seq: self,
            index: 0,
        }
    }

    pub fn extend<I: Iterator<Item=Nucleotide>>(&mut self, x: I){
        for y in x {
            self.push(y);
        }
    }

    pub fn push(&mut self, value: Nucleotide){
        let (_, pos) = T::addr(self.len);
        self.len += 1;
        if pos == 0 {
            self.storage.push(T::default());
        }
        self.write(self.len - 1, value);
    }

    #[inline]
    pub fn reverse_complement(self) -> PackedSeq<ReverseComplement<T>> {
        PackedSeq::<ReverseComplement<T>> {
            // Noop
            storage: self.storage.into_iter().map(|x| x.into()).collect(),
            len: self.len
        }
    }
}

pub struct PackedSeqIter<'a, T>
where
    T: Storage,
{
    seq: &'a PackedSeq<T>,
    index: usize,
}

impl<'a, T> Iterator for PackedSeqIter<'a, T>
where
    T: Storage,
{
    type Item = Nucleotide;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.seq.read(self.index - 1)
    }
}

impl<'a, T> IntoIterator for &'a PackedSeq<T>
where
    T: Storage
{
    type Item = Nucleotide;

    type IntoIter = PackedSeqIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone)]
pub struct PackedSeqSlice<'a, T>
where
    T: Storage
{
    pub seq: &'a PackedSeq<T>,
    pub start: usize,
    pub len: usize
}

// For integers it should be preferred to do this directly using pointer arithmetic
// If repacking becomes common, this should be optimized
macro_rules! extension_repack {
    ($a:ty, $b:ty) => {
        impl From<PackedSeq<$a>> for PackedSeq<$b> {
            #[inline]
            fn from(seq: PackedSeq<$a>) -> PackedSeq<$b> {
                let mut storage = Vec::with_capacity(<$b>::addr(seq.len()).0 + 1);
                storage.extend(seq.iter().map(|x| <$b>::from(x)));

                Self {
                    storage,
                    len: seq.len(),
                }
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