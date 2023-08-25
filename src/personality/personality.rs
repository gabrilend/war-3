use rand::Rng;

pub struct Personality {
    pub alignment: (u8, u8),
}

impl Personality {
    pub fn new() -> Self {
        Self {
            alignment: (0, 0),
        }
    }

    fn set_alignment(&mut self, alignment: (u8, u8)) {
        self.alignment = alignment;
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        let alignment = (rng.gen_range(0..100), rng.gen_range(0..100));
        self.set_alignment(alignment);
    }

    pub fn normalize(&mut self) -> (u8, u8) {
        (self.alignment.0 / 100, self.alignment.1 / 100)
    }   
}

