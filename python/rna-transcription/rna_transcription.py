def to_rna(dna_strand):
    DNA = ['G', 'C', 'T', 'A']
    RNA = ['C', 'G', 'A', 'U']
    result = ""
    for letter in dna_strand:
       result += RNA[DNA.index(letter)]
    return result

