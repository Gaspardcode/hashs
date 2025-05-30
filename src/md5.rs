pub mod nb_utils;
pub mod utils;
use crate::md5::nb_utils::*;
use crate::md5::utils::*;
pub use crate::sha::utils::*;

#[derive(Debug, Clone)]
pub struct Block {
    data: [u32; 16],
}

impl Block {
    fn new(data: &[u32; 16]) -> Block {
        let data = data.clone();
        Block { data }
    }
    fn process(&self, md5: &mut Md5) {
        let mut new_state = md5.state;
        let mut F: u32;
        let mut g: u32;

        for i in 0..64 {
            if i < 16 {
                F = op1(new_state[1], new_state[2], new_state[3] );
                g = i;
            } else if i < 32 {
                F = op1(new_state[3], new_state[1], new_state[2]);
                g = (5 * i + 1) % 16;
            } else if i < 48 {
                F = op2(new_state[1], new_state[2], new_state[3]);
                g = (3 * i + 5) % 16;
            } else {
                F = op3(new_state[1], new_state[2], new_state[3]);
                g = (7 * i) % 16;
            }

            F = (F as u64
                + new_state[0] as u64
                + k_md5[i as usize] as u64
                + self.data[g as usize] as u64) as u32;

            new_state[0] = new_state[3];
            new_state[3] = new_state[2];
            new_state[2] = new_state[1];
            new_state[1] = (new_state[1] as u64 + rotL(F, s_md5[i as usize])as u64) as u32;
        }

        for i in 0..4 {
            md5.state[i] = (md5.state[i] as u64 + new_state[i] as u64) as u32
        }
    }
}

#[derive(Debug, Clone)]
pub struct Md5 {
    state: [u32; 4], // [a,b,c,d] in the paper
    size: usize,     // the number of blocks processed so far
    // scenario: the user hashs a message whom the size is not a multiple of 512,
    // in the regular case the leftovers are padded
    // However here, we let the user the choice to add more data to the hash
    // hence we store them here until :
    // 1. the user digests the hash
    // 2. the user adds data to the hash
    leftover: Vec<u8>,
}

impl Md5 {
    pub fn new() -> Md5 {
        let size = 0;
        let state = md5_init.clone();
        let leftover = vec![];

        Md5 {
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

    pub fn digest(&mut self) {
        padding(&self.leftover, self.size)
            .iter()
            .for_each(|raw_block| Block::new(raw_block).process(self));
    }

    pub fn digest_string(&self) -> String {
        self.state
            .iter()
            .map(|x| format!("{:04x}", x))
            .collect::<String>()
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.state = md5_init.clone();
        self.leftover = vec![];
    }
}
