fn main() {
    let dna = DNA {
        sequence: String::from("AGGCTTAGATCGAGATCCGAT"),
        nucleotide_count: (0, 0, 0, 0)
    }
}

struct DNA {
    sequence: String,
    nucleotide_count: (u64, u64, u64, u64)
}

