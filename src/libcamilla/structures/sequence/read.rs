/*
Sequence as read by fastx parser.
 */
use crate::structures::sequence::complement::{Forward, Identity};
use crate::structures::sequence::packed::PackedSeq;
use crate::structures::sequence::storage::Storage;

pub struct ReadSeq {
    pub name: String,
    pub sequence: String,
    pub separator: Option<String>,
    pub quality: Option<String>,
}

impl ReadSeq {
    pub fn pack<T: Storage>(&self) -> PackedSeq<T, Forward, Identity> {
        PackedSeq::<T, Forward, Identity>::from_read(&self)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.sequence.len()
    }
}
