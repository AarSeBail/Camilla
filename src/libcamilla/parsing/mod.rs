use crate::structures::sequence::read::ReadSeq;
use std::io::Read;
use std::marker::PhantomData;

pub mod fastq;

pub trait SeqParser<R: Read + Send, P: SeqParser<R, P>> {
    fn iter(self) -> SeqIter<R, P>;
    fn next_seq(&mut self) -> Option<Result<ReadSeq, u64>>;
}

pub struct SeqIter<R: Read + Send, P: SeqParser<R, P>> {
    parser: P,
    placeholder: PhantomData<R>,
}

impl<R: Read + Send, P: SeqParser<R, P> + Send> SeqIter<R, P> {
    fn new(parser: P) -> Self {
        SeqIter {
            parser,
            placeholder: PhantomData,
        }
    }
}

impl<R: Read + Send, P: SeqParser<R, P>> Iterator for SeqIter<R, P> {
    type Item = ReadSeq;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let x = self.parser.next_seq();
            match x {
                None => return None,
                Some(Err(_)) => {}
                Some(Ok(s)) => return Some(s),
            }
        }
    }
}
