use bitvec::{order::{Msb0, Lsb0, LocalBits}, vec::BitVec, view::BitView};

pub struct ReadSeq {
    pub name: String,
    pub sequence: String,
    pub separator: Option<String>,
    pub quality: Option<String>,
}

impl ReadSeq {
    pub fn pack_naive(&self) -> PackedSeq {
        let seq = &self.sequence;

        let mut res = PackedSeq {
            ones: BitVec::with_capacity((seq.len() + 63) / 64),
            twos: BitVec::with_capacity((seq.len() + 63) / 64),
            len: seq.len(),
        };

        let mut chunks = seq.as_bytes().chunks_exact(64);

        for chunk in chunks.by_ref() {
            let d = chunk
                .into_iter()
                .map(ascii_to_base_rep)
                .fold((0, 0), |(a1, a2), b| {
                    ((2 * a1) | (b & 1), (2 * a2) | (b & 2))
                });
            res.ones.extend_from_bitslice(d.0.view_bits::<Lsb0>());
            res.twos.extend_from_bitslice(d.1.view_bits::<Lsb0>());
        }

        let rem = chunks.remainder();

        println!(
            "{:b}",
            rem.into_iter()
                .map(ascii_to_base_rep)
                .reduce(|acc, b| (acc << 1) + (b & 1))
                .unwrap()
        );

        let mut d = rem
            .into_iter()
            .map(ascii_to_base_rep)
            .fold((0, 0), |(a1, a2), b| {
                ((2 * a1) | (b & 1), (2 * a2) | (b & 2))
            });
        res.ones
            .extend_from_bitslice(&d.0.view_bits::<Msb0>()[64-rem.len()..64]);
        println!("{:b}",d.0);
        res.twos
            .extend_from_bitslice(&d.1.view_bits::<Msb0>()[64-rem.len()..64]);
        println!("{:?}",d.1.view_bits::<Msb0>());
        res
    }
}

pub struct PackedSeq {
    pub ones: BitVec,
    pub twos: BitVec,
    pub len: usize,
}

fn ascii_to_base_rep(c: &u8) -> u64 {
    match c {
        b'T' | b't' => 0b00,
        b'A' | b'a' => 0b11,
        b'G' | b'g' => 0b01,
        b'C' | b'c' => 0b10,
        _ => 0,
    }
}

/*
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
    type Immut = BaseRef<'a, Immutable>;

    type Mut = BaseRef<'a, Mutable>;

    fn get(self, seq: &'a PackedSeq) -> Option<Self::Immut> {
        todo!()
    }

    fn get_mut(self, seq: &'a mut PackedSeq) -> Option<Self::Mut> {
        todo!()
    }

    unsafe fn get_unchecked(self, seq: &'a PackedSeq) -> Self::Immut {
        todo!()
    }

    unsafe fn get_unchecked_mut(self, seq: &'a mut PackedSeq) -> Self::Mut {
        todo!()
    }

    fn index(self, seq: &'a PackedSeq) -> Self::Immut {
        todo!()
    }

    fn index_mut(self, seq: &'a mut PackedSeq) -> Self::Mut {
        todo!()
    }
}

*/
