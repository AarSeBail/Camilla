use crate::structures::sequence::read::ReadSeq;
use std::io::{BufRead, BufReader, Read};
use std::mem;
use std::ops::Deref;

use super::{SeqIter, SeqParser};

pub struct FastqParser<R: Read + Send> {
    reader: BufReader<R>,
    line: String,
}

#[allow(unused)]
impl<R: Read + Send> FastqParser<R> {
    pub fn new(read: R) -> Self {
        FastqParser {
            reader: BufReader::new(read),
            line: String::new(),
        }
    }
}

impl<R: Read + Send> SeqParser<R, FastqParser<R>> for FastqParser<R> {
    fn iter(self) -> SeqIter<R, FastqParser<R>> {
        SeqIter::new(self)
    }

    fn next_seq(&mut self) -> Option<Result<ReadSeq, u64>> {
        let mut read;
        if self.line.is_empty() {
            read = self.reader.read_line(&mut self.line);

            // Remove trailing whitespace
            let trunc = self.line.trim_end().len();
            self.line.truncate(trunc);

            if let Ok(0) = read {
                return None;
            }
        };

        let mut seq = String::new();
        let mut name = String::new();
        let mut sep = String::new();
        let mut qual = String::new();
        let mut after_sep = false;

        loop {
            if after_sep {
                if seq.len() <= qual.len() {
                    break;
                }
                qual += self.line.deref();
            } else {
                if self.line.starts_with("@") {
                    // Last seq was incomplete.
                    // TODO: Optionally error here
                    if !seq.is_empty() {
                        seq.clear();
                    }
                    if !name.is_empty() {
                        name.clear();
                    }

                    mem::swap(&mut self.line, &mut name);
                    name = name.split_off(1);
                } else if self.line.starts_with("+") {
                    after_sep = true;
                    mem::swap(&mut self.line, &mut sep);
                    sep = sep.split_off(1);
                    qual.reserve(seq.len());
                } else {
                    seq += self.line.deref();
                }
            }
            self.line.clear();

            read = self.reader.read_line(&mut self.line);

            let trunc = self.line.trim_end().len();
            self.line.truncate(trunc);

            if let Ok(0) = read {
                break;
            }
        }

        if seq.is_empty() || name.is_empty() {
            None
        } else if seq.len() != qual.len() {
            Some(Err(1))
        } else {
            Some(Ok(ReadSeq {
                name: name,
                sequence: seq,
                separator: Some(sep),
                quality: Some(qual),
            }))
        }
    }
}
