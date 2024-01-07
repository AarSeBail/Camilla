use crate::structures::sequence::nucleotide::Nucleotide;

pub trait Reversal {
    type Inverse: Reversal;
    const REVERSED: bool;
    fn reindex(len: usize, pos: usize) -> usize;
}

pub struct Forward;

impl Reversal for Forward {
    type Inverse = Reverse;
    const REVERSED: bool = false;

    #[inline(always)]
    fn reindex(_len: usize, pos: usize) -> usize {
        pos
    }
}

pub struct Reverse {}

impl Reversal for Reverse {
    type Inverse = Forward;
    const REVERSED: bool = true;

    #[inline(always)]
    fn reindex(len: usize, pos: usize) -> usize {
        len - pos - 1
    }
}

pub trait Complementation {
    type Inverse: Complementation;
    const COMPLEMENT: bool;
    fn translate(n: Nucleotide) -> Nucleotide;
}

pub struct Identity;

impl Complementation for Identity {
    type Inverse = Complement;
    const COMPLEMENT: bool = false;

    #[inline(always)]
    fn translate(n: Nucleotide) -> Nucleotide {
        n
    }
}

pub struct Complement {}

impl Complementation for Complement {
    type Inverse = Identity;
    const COMPLEMENT: bool = true;

    #[inline(always)]
    fn translate(n: Nucleotide) -> Nucleotide {
        n.complement()
    }
}
