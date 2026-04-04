const DNA_NUCLEOTIDES: [u8; 4] = [b'A', b'C', b'G', b'T'];
const RNA_NUCLEOTIDES: [u8; 4] = [b'A', b'C', b'G', b'U'];

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    nucleotides: Vec<u8>
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    nucleotides: Vec<u8>
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let bytes = dna.as_bytes();

        for i in 0..bytes.len() {
            if !DNA_NUCLEOTIDES.contains(&bytes[i]) {
                return Err(i);
            }
        }

        Ok(Self { nucleotides: bytes.to_vec() })
    }

    pub fn into_rna(self) -> Rna {
        let new_bytes: Vec<u8> = self.nucleotides.iter().map(|n| {
            return match n {
                b'G' => b'C',
                b'C' => b'G',
                b'T' => b'A',
                _ => b'U'
            };
        }).collect();

        Rna {
            nucleotides: new_bytes
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let bytes = rna.as_bytes();

        for i in 0..bytes.len() {
            if !RNA_NUCLEOTIDES.contains(&bytes[i]) {
                return Err(i);
            }
        }

        Ok(Self { nucleotides: bytes.to_vec() })
    }
}
