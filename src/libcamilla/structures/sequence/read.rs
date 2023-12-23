/*
Sequence as read by fastx parser.
 */
pub struct ReadSeq {
    pub name: String,
    pub sequence: String,
    pub separator: Option<String>,
    pub quality: Option<String>,
}

impl ReadSeq {}
