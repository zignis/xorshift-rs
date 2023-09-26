struct IdGenerator {
    state: [u32; 4],
}

impl IdGenerator {
    // Ctor with random seed
    fn new(seed: u32) -> IdGenerator {
        let mut generator = IdGenerator {
            state: [0; 4],
        };
        generator.seed(seed);
        generator
    }

    // Init the internal state array based on the seed value
    fn seed(&mut self, seed: u32) {
        self.state[0] = seed;
        self.state[1] = seed.wrapping_add(0x9E3779B9);
        self.state[2] = seed.wrapping_add(0x9E3779B9 ^ 0x243F6A88);
        self.state[3] = seed.wrapping_add(0x9E3779B9 ^ 0x243F6A88 ^ 0xB7E15162);
    }

    // Generate a new ID using XORShift algorithm
    fn generate(&mut self) -> u32 {
        let x = self.state[0] ^ (self.state[0] << 11);
        self.state[0] = self.state[1];
        self.state[1] = self.state[2];
        self.state[2] = self.state[3];
        self.state[3] = (self.state[3] ^ (self.state[3] >> 19)) ^ (x ^ (x >> 8));
        self.state[3]
    }
}

fn main() {
    let mut id_gen = IdGenerator::new(5);
    for _ in 0..5 {
        let id = id_gen.generate();
        println!("Sample ID: {}", id);
    }
}
