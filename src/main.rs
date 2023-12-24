use libcamilla::structures::sequence::{read::ReadSeq, storage::ReverseComplement, nucleotide::Nucleotide, packed::PackedSeq};
use std::{hint::black_box, mem::size_of};

fn main() {
    let mut seq = ReadSeq {
        sequence: "G".to_string(),
        name: "".to_string(),
        separator: None,
        quality: None,
    };

    // seq.sequence.extend(['G'; 64].iter());

    // seq.sequence.extend(['T', 'A', 'T', 'G', 'A']);

    // println!("{}", seq.sequence);

    let bb = black_box(seq);
    let mut res = PackedSeq::<usize>::from_read(&bb);

    for i in 0..res.len() {
        print!("{:?}", res.read(i).unwrap());
    }
    println!("");
    // res.push(Nucleotide::A);

    let mut res2 = res.reverse_complement();
    for i in 0..res2.len() {
        print!("{:?}", res2.read(i).unwrap());
    }
    println!("");
    res2.push(Nucleotide::T);

    for i in 0..res2.len() {
        print!("{:?}", res2.read(i).unwrap());
    }
    println!("");

    let res3 = res2.reverse_complement();
    for i in 0..res3.len() {
        print!("{:?}", res3.read(i).unwrap());
    }
    println!("");
}
