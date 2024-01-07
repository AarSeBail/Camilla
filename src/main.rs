use libcamilla::filters::blocks::blanket::BlanketBBFBlock;
use libcamilla::filters::bloom::BBFilter;
use libcamilla::structures::sequence::complement::{Complement, Forward, Identity, Reverse};
use libcamilla::structures::sequence::{nucleotide::Nucleotide, packed::PackedSeq, read::ReadSeq};
use std::{hint::black_box, mem::size_of};

fn main() {
    let mut seq = ReadSeq {
        sequence: "GTA".to_string(),
        name: "".to_string(),
        separator: None,
        quality: None,
    };

    seq.sequence.extend(['G'; 4096].iter());

    seq.sequence.extend(['T', 'A', 'T', 'G', 'A']);

    // println!("{}", seq.sequence);

    let bb = black_box(seq);
    let mut res = bb.pack::<u8>();

    let filter = BBFilter::<BlanketBBFBlock>::new(100, 24);

    filter.insert_kmers(&res, 3);

    println!("{}", filter.contains_kmer(res.slice(1, 5)));
    println!(
        "{} {}",
        res.slice(res.len() - 3, 3),
        filter.contains_kmer(res.slice(res.len() - 3, 3))
    );
}
