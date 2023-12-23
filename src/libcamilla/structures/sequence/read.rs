/*
Sequence as read by fastx parser.
 */

use std::marker::PhantomData;
use crate::structures::sequence::nucleotide::Nucleotide;
use crate::structures::sequence::packed::PackedSeq;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use bitvec::view::BitView;

pub struct ReadSeq {
    pub name: String,
    pub sequence: String,
    pub separator: Option<String>,
    pub quality: Option<String>,
}

impl ReadSeq {
    /*
    Conversion from a read to a packed sequence
     */
    pub fn pack(&self) -> PackedSeq {
        let seq = &self.sequence;

        let mut res = PackedSeq {
            ones: BitVec::with_capacity((seq.len() + 63) / 64),
            twos: BitVec::with_capacity((seq.len() + 63) / 64),
            len: seq.len()
        };

        let mut chunks = seq.as_bytes().chunks_exact(64);

        for chunk in chunks.by_ref() {
            let d = chunk.into_iter().map(Nucleotide::from_ascii).fold(
                (0_usize, 0_usize),
                |mut acc, b| {
                    acc.0 = 2 * acc.0 | (b.one() as usize);
                    acc.1 = 2 * acc.1 | (b.two() as usize);
                    acc
                },
            );
            res.ones.extend_from_bitslice(&d.0.view_bits::<Msb0>());
            res.twos.extend_from_bitslice(&d.1.view_bits::<Msb0>());
        } // TODO: Try SimdInt::reduce_or

        let rem = chunks.remainder();

        let d = rem.into_iter().map(Nucleotide::from_ascii).fold(
            (0_usize, 0_usize),
            |mut acc, b| {
                acc.0 = 2 * acc.0 | (b.one() as usize);
                acc.1 = 2 * acc.1 | (b.two() as usize);
                acc
            },
        );
        res.ones
            .extend_from_bitslice(&d.0.view_bits::<Msb0>()[64 - rem.len()..64]);
        res.twos
            .extend_from_bitslice(&d.1.view_bits::<Msb0>()[64 - rem.len()..64]);

        res
    }
}
