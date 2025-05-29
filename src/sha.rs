mod nb_utils;
pub mod utils;
use crate::sha::nb_utils::*;
use crate::sha::utils::*;

// TODO later on merge the SHAS into an enum

#[derive(Debug, Clone)]
pub enum Hash {
    SHA256(Sha256),
    SHA1(Sha1),
}

#[derive(Debug, Clone)]
pub enum Blocks {
    SHA256(Block),
    SHA1(BlockSHA1),
}

#[derive(Debug, Clone)]
pub struct BlockSHA1 {
    // 64 * 32 bits = 2048 bits
    sub_blocks: [u32; 80],
}

impl BlockSHA1 {
    fn new(data: &[u32; 16]) -> BlockSHA1 {
        let mut sub_blocks: [u32; 80] = [0; 80];
        for i in 0..16 {
            sub_blocks[i] = data[i];
            println!("{:032b}", sub_blocks[i]);
        }
        // 16 <= t < 80
        // W(t) = S^1(W(t-3) XOR W(t-8) XOR W(t-14) XOR W(t-16)).
        for i in 16..80 {
            let tmp =
                sub_blocks[i - 3] ^ sub_blocks[i - 8] ^ sub_blocks[i - 14] ^ sub_blocks[i - 16];
            sub_blocks[i] = rotL(tmp, 1);
            println!("{:032b}", sub_blocks[i]);
        }

        BlockSHA1 { sub_blocks }
    }
    fn process(&self, sha: &mut Sha1) {
        let mut vu = sha.state;
        for i in 0..80 {
            // TEMP = S^5(A) + f(t;B,C,D) + E + W(t) + K(t);
            let temp = (rotL(vu[0], 5) as u64
                + sha1f(i, vu[1], vu[2], vu[3]) as u64
                + vu[4] as u64
                + self.sub_blocks[i] as u64
                + constants_k_sha1(i) as u64) as u32;

            for j in (1..5).rev() {
                vu[j] = vu[j - 1];
            }

            vu[2] = rotL(vu[2], 30);
            vu[0] = temp;
        }

        for i in 0..5 {
            sha.state[i] = ((sha.state[i] as u64) + (vu[i] as u64)) as u32;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    // 64 * 32 bits = 2048 bits
    sub_blocks: [u32; 64],
}

impl Block {
    fn new(data: &[u32; 16]) -> Block {
        let mut sub_blocks: [u32; 64] = [0; 64];
        for i in 0..16 {
            sub_blocks[i] = data[i];
            println!("{:032b}", sub_blocks[i]);
        }
        // 16 <= i <= 63
        // Wi = σ1(Wi−2) + Wi−7 + σ0(Wi−15) + Wi−16
        for i in 16..64 {
            sub_blocks[i] = (small_sigma1(sub_blocks[i - 2]) as u64
                + sub_blocks[i - 7] as u64
                + small_sigma0(sub_blocks[i - 15]) as u64
                + sub_blocks[i - 16] as u64) as u32;
            println!("{:032b}", sub_blocks[i]);
        }

        Block { sub_blocks }
    }
    fn process(&self, sha: &mut Sha256) {
        let mut shuffled_state = sha.state;
        for i in 0..64 {
            let t1 = shuffled_state[7] as u64
                + (sigma1(shuffled_state[4]) as u64)
                + (ch(shuffled_state[4], shuffled_state[5], shuffled_state[6]) as u64)
                + (constants_k[i] as u64)
                + (self.sub_blocks[i] as u64);

            let t2 = (sigma0(shuffled_state[0]) as u64)
                + (maj(shuffled_state[0], shuffled_state[1], shuffled_state[2]) as u64);

            for j in (1..8).rev() {
                shuffled_state[j] = shuffled_state[j - 1];
            }

            shuffled_state[4] = ((shuffled_state[4] as u64) + t1) as u32;
            shuffled_state[0] = (t1 + t2) as u32;
        }

        for i in 0..8 {
            sha.state[i] = ((sha.state[i] as u64) + (shuffled_state[i] as u64)) as u32;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sha256 {
    state: [u32; 8], // [a,b,c,d,e,f,g,h] in the paper
    size: usize,     // the number of blocks processed so far
    // scenario: the user hashs a message whom the size is not a multiple of 512,
    // in the regular case the leftovers are padded
    // However here, we let the user the choice to add more data to the hash
    // hence we store them here until :
    // 1. the user digests the hash
    // 2. the user adds data to the hash
    leftover: Vec<u8>,
}

impl Sha256 {
    pub fn new() -> Sha256 {
        let size = 0;
        let state = sha256_init.clone();
        let leftover = vec![];

        Sha256 {
            state,
            size,
            leftover,
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.size += data.len();
        self.leftover.append(&mut Vec::from(data));

        let bind = self.leftover.clone();

        let mut iter = bind.chunks(64);
        self.leftover = iter.next_back().unwrap_or_else(|| &[]).to_vec();

        iter.for_each(|slice| Block::new(&chunky(slice)).process(self));
    }

    /// #RETURNS
    ///
    /// A 32 bytes array
    pub fn digest_arr(&self) -> [u8; 32] {
        arru32_to_u8(&self.state)
    }

    pub fn digest(&mut self) {
        padding(&self.leftover, self.size)
            .iter()
            .for_each(|raw_block| Block::new(raw_block).process(self));
    }

    pub fn digest_string(&self) -> String {
        self.state
            .iter()
            .map(|x| format!("{:08x}", x))
            .collect::<String>()
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.state = sha256_init.clone();
        self.leftover = vec![];
    }
}

#[derive(Debug, Clone)]
pub struct Sha1 {
    state: [u32; 5], // fixed size of 8 : [a,b,c,d,e] in the paper
    size: usize,     // the number of blocks processed
    leftover: Vec<u8>,
}

impl Sha1 {
    pub fn new() -> Sha1 {
        let size = 0;
        let state = sha1_init.clone();
        let leftover = vec![];

        Sha1 {
            state,
            size,
            leftover,
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.size += data.len();
        self.leftover.append(&mut Vec::from(data));

        let bind = self.leftover.clone();

        let mut iter = bind.chunks(64);
        self.leftover = iter.next_back().unwrap_or_else(|| &[]).to_vec();

        iter.for_each(|slice| BlockSHA1::new(&chunky(slice)).process(self));
    }

    /// #RETURNS
    ///
    /// A 32 bytes array
    pub fn digest(&mut self) {
        padding(&self.leftover, self.size)
            .iter()
            .for_each(|raw_block| BlockSHA1::new(raw_block).process(self));
    }

    pub fn digest_string(&self) -> String {
        self.state
            .iter()
            .map(|x| format!("{:08x}", x))
            .collect::<String>()
    }

    pub fn digest_arr(&self) -> [u8; 20] {
        sha1_arr(&self.state)
    }
}
