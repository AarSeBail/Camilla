use std::marker::PhantomData;
use crate::structures::sequence::nucleotide::{Nucleotide, NucleotideRef};
use bitvec::order::Msb0;
use bitvec::ptr::BitRef;
use bitvec::slice::{BitSlice, BitSliceIndex};
use bitvec::vec::BitVec;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use wyz::{Const, Mut, Mutability};

pub struct PackedSeq {
    pub(crate) ones: BitVec<usize, Msb0>,
    pub(crate) twos: BitVec<usize, Msb0>,
    pub len: usize,
}

pub trait PackedSeqIndex<'a> {
    type Immut;
    type Mut;

    fn get(self, seq: &'a PackedSeq) -> Option<Self::Immut>;
    fn get_mut(self, seq: &'a mut PackedSeq) -> Option<Self::Mut>;

    unsafe fn get_unchecked(self, seq: &'a PackedSeq) -> Self::Immut;
    unsafe fn get_unchecked_mut(self, seq: &'a mut PackedSeq) -> Self::Mut;

    fn index(self, seq: &'a PackedSeq) -> Self::Immut;
    fn index_mut(self, seq: &'a mut PackedSeq) -> Self::Mut;
}

impl<'a> PackedSeqIndex<'a> for usize {
    type Immut = NucleotideRef<'a, Const>;

    type Mut = NucleotideRef<'a, Mut>;

    #[inline]
    fn get(self, seq: &'a PackedSeq) -> Option<Self::Immut> {
        if self < seq.len {
            Some(unsafe { PackedSeqIndex::get_unchecked(self, seq) })
        } else {
            None
        }
    }

    #[inline]
    fn get_mut(self, seq: &'a mut PackedSeq) -> Option<Self::Mut> {
        if self < seq.len {
            Some(unsafe { PackedSeqIndex::get_unchecked_mut(self, seq) })
        } else {
            None
        }
    }

    #[inline]
    unsafe fn get_unchecked(self, seq: &'a PackedSeq) -> Self::Immut {
        let one = BitSliceIndex::<'a, usize, Msb0>::get_unchecked(self, &seq.ones);
        let two = BitSliceIndex::<'a, usize, Msb0>::get_unchecked(self, &seq.twos);
        let val = Nucleotide::from_bools(*one, *two);
        NucleotideRef::<'a, Const> {
            one: ManuallyDrop::new(one),
            two: ManuallyDrop::new(two),
            data: val,
        }
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, seq: &'a mut PackedSeq) -> Self::Mut {
        let one = BitSliceIndex::<'a, usize, Msb0>::get_unchecked_mut(self, &mut seq.ones);
        let two = BitSliceIndex::<'a, usize, Msb0>::get_unchecked_mut(self, &mut seq.twos);
        let val = Nucleotide::from_bools(*one, *two);
        NucleotideRef::<'a, Mut> {
            one: ManuallyDrop::new(one),
            two: ManuallyDrop::new(two),
            data: val,
        }
    }

    #[inline]
    fn index(self, seq: &'a PackedSeq) -> Self::Immut {
        PackedSeqIndex::get(self, seq)
            .unwrap_or_else(|| panic!("index {} out of bounds: {}", self, seq.len))
    }

    #[inline]
    fn index_mut(self, seq: &'a mut PackedSeq) -> Self::Mut {
        let len = seq.len;
        PackedSeqIndex::get_mut(self, seq)
            .unwrap_or_else(|| panic!("index {} out of bounds: {}", self, len))
    }
}