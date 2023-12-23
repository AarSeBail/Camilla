use std::sync::Arc;

use wyz::Const;
use crate::structures::sequence::nucleotide::NucleotideRef;
use crate::structures::sequence::packed::{PackedSeq, PackedSeqIndex};

impl PackedSeq {
    fn windows(&self, width: usize) -> Windows {
        Windows::from_seq(self, width)
    }
}

struct Windows<'a> {
    width: usize,
    seq: &'a PackedSeq,
    index: usize
}

impl<'a> Windows<'a> {
    fn from_seq(seq: &'a PackedSeq, width: usize) -> Self {
        assert_ne!(width, 0, "Window width must be non-zero.");

        let vec = if width <= seq.len {
            (0..width).map(|i| i.get(seq).unwrap()).collect::<Vec<NucleotideRef<'a, Const>>>()
        }else {
            vec![]
        };
        Self {
            width,
            seq,
            index: width
        }
    }
}

impl<'a> Iterator for Windows<'a> {
    type Item = [NucleotideRef<'a, Const>];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index <= self.seq.len {
            let vec = Vec::new();
            while vec.len() < self.width {
                vec.push(vec.len().get(self.seq).unwrap());
            }
            
            self.index += 1;
            Some(vec)
        }else{
            None
        }
    }
}