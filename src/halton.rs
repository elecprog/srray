pub struct HaltonSequence {
    index: u32,
    length: u32,
    base: u32,
}

// On [-1, 1]
impl HaltonSequence {
    pub fn new(length: u32, base: u32) -> HaltonSequence {
        HaltonSequence {
            index: 0,
            length,
            base,
        }
    }
}

impl Iterator for HaltonSequence {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.length {
            None
        } else {
            let mut i = self.index;
            self.index += 1;

            let mut f = 1 as f64;
            let mut r = 0 as f64;
            while i > 0 {
                f = f / self.base as f64;
                r = r + f * (i % self.base) as f64;
                i = i / self.base; // Integer division
            }
            Some(2.*r - 1.)
        }
    }
}

// On [-1, 1] × [-1, 1]
pub struct Halton2Sequence {
    seq1: HaltonSequence,
    seq2: HaltonSequence,
}

impl Halton2Sequence {
    pub fn new(length: u32, base1: u32, base2: u32) -> Halton2Sequence {
        Halton2Sequence {
            seq1: HaltonSequence::new(length, base1),
            seq2: HaltonSequence::new(length, base2),
        }
    }
}

impl Iterator for Halton2Sequence {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.seq1.next()?;
        let b = self.seq2.next()?;
        Some((a, b))
    }
}