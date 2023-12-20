use libcamilla::structures::sequence::ReadSeq;

fn main() {
    let seq = ReadSeq {
        sequence: "TAGC".to_string(),
        name: "".to_string(),
        separator: None,
        quality: None,
    };

    let res = seq.pack_naive();
    for i in 0..res.len {
        println!(
            "{:b}",
            (if res.ones[i] { 1 } else { 0 }) + (if res.twos[i] { 2 } else { 0 })
        );
    }
}
