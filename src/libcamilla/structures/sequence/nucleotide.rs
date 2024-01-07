#[derive(Debug, Copy, Clone)]
pub enum Nucleotide {
    T = 0,
    A = 3,
    G = 1,
    C = 2,
}

impl Nucleotide {
    #[inline]
    pub fn from_ascii(c: &u8) -> Self {
        match &c {
            b'T' => Self::T,
            b'A' => Self::A,
            b'G' => Self::G,
            _ => Self::C,
        }
    }

    #[inline]
    pub fn complement(&self) -> Self {
        match &self {
            Self::T => Self::A,
            Self::A => Self::T,
            Self::G => Self::C,
            Self::C => Self::G,
        }
    }
}

macro_rules! nucleotide_bits {
    ($($t:ty),+ $(,)?) => { $(
        impl From<$t> for Nucleotide {
            #[inline]
            fn from(n: $t) -> Nucleotide {
                match n {
                    0b00 => Nucleotide::T,
                    0b11 => Nucleotide::A,
                    0b01 => Nucleotide::G,
                    _ => Nucleotide::C
                }
            }
        }

        impl From<Nucleotide> for $t {
            #[inline]
            fn from(n: Nucleotide) -> $t {
                n as $t
            }
        }
    )+ };
}

nucleotide_bits!(u8, u16, u32, u64, u128, usize);
