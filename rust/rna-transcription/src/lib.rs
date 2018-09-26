#[derive(Debug, PartialEq)]
pub struct DNA{
    strand: String
}


#[derive(Debug, PartialEq)]
pub struct RNA{
    strand: String
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        // check if all characters are valid and return
        // Ok or Err on said check
        match dna.chars().all(|s| DNA::valid_char(s)) {
            true => Ok(DNA {
                strand: dna.to_string(),
            }),
            false => Err(0),
        }
    }

    // Return a valid rna strand for a given dna strand
    pub fn to_rna(self) -> RNA {
        RNA::new(&self.strand
                .chars()
                .map(|s| DNA::convert(s))
                .collect::<String>()
            ).unwrap()
        
    }

    // Returns a bool after checking if a character is valid
    fn valid_char(nucleotide: char) -> bool {
        match nucleotide {
            'C' | 'G' | 'A' | 'T' => true,
            _ => false,
        }
    }

    // Return matching rna nucleotide for a given dna nucleotide
    fn convert(nucleotide: char) -> char {
        match nucleotide {
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            'A' => 'U',
            _ => nucleotide,
        }
    }
}

impl RNA {
    // Same principle as DNA::new()
    pub fn new(rna: &str) -> Result<RNA, usize> {
        match rna.chars().all(|s| RNA::valid_char(s)) {
            true => Ok(RNA {
                strand: rna.to_string(),
            }),
            false => Err(0),
        }
    }

    // Same as DNA::valid_char but swap T for U
    fn valid_char(nucleotide: char) -> bool {
        match nucleotide {
            'C' | 'G' | 'A' | 'U' => true,
            _ => false,
        }
    }
}
