use libcamilla::structures::sequence::read::ReadSeq;
use std::hint::black_box;
use libcamilla::structures::sequence::packed::PackedSeqIndex;

fn main() {
    let mut seq = ReadSeq {
        sequence: "TAGC".to_string(),
        name: "".to_string(),
        separator: None,
        quality: None,
    };

    seq.sequence.extend(['G'; 64].iter());

    seq.sequence.extend(['T', 'A', 'T', 'G', 'A']);

    println!("{}", seq.sequence);

    let bb = black_box(seq);
    let res = bb.pack();

    for i in 0..res.len {
        print!("{:?}", *i.get(&res).unwrap());
    }
    println!("");
}
