use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use bitvec::order::Msb0;
use bitvec::prelude::BitRef;
use wyz::{Mut, Mutability};

#[derive(Debug)]
pub enum Nucleotide {
    T = 0,
    A = 3,
    G = 1,
    C = 2,
}

impl Nucleotide {
    #[inline]
    pub fn from_ascii(c: &u8) -> Self {
        match &c {
            b'T' => Self::T,
            b'A' => Self::A,
            b'G' => Self::G,
            _ => Self::C,
        }
    }

    #[inline]
    pub(crate) fn from_bools(one: bool, two: bool) -> Self {
        match (one, two) {
            (false, false) => Self::T,
            (true, true) => Self::A,
            (true, false) => Self::G,
            (false, true) => Self::C,
        }
    }

    #[inline]
    pub fn one(&self) -> bool {
        match &self {
            Self::A => true,
            Self::G => true,
            _ => false,
        }
    }

    #[inline]
    pub fn two(&self) -> bool {
        match &self {
            Self::A => true,
            Self::C => true,
            _ => false,
        }
    }
}

pub struct NucleotideRef<'a, M>
    where
        M: Mutability,
{
    pub(crate) one: ManuallyDrop<BitRef<'a, M, usize, Msb0>>,
    pub(crate) two: ManuallyDrop<BitRef<'a, M, usize, Msb0>>,
    pub(crate) data: Nucleotide,
}

impl<'a, M> Deref for NucleotideRef<'a, M>
    where
        M: Mutability,
{
    type Target = Nucleotide;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a> DerefMut for NucleotideRef<'a, Mut> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a, M> Drop for NucleotideRef<'a, M>
    where
        M: Mutability,
{
    #[inline]
    fn drop(&mut self) {
        if M::CONTAINS_MUTABILITY {
            unsafe {
                ManuallyDrop::<BitRef<'a, M, usize, Msb0>>::take(&mut self.one)
                    .into_bitptr()
                    .to_mut()
                    .write(self.data.one());
                ManuallyDrop::<BitRef<'a, M, usize, Msb0>>::take(&mut self.two)
                    .into_bitptr()
                    .to_mut()
                    .write(self.data.two());
            }
        }
    }
}

