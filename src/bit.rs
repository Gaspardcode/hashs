const _H_NUMBER:usize = 8;

// considering array is ordered in big endian order
pub struct Bits {
    value: Rc<Vec<u32>>
    // size in bits
    bitsize: usize
    // padding in bits
    padding: usize
}
impl Bits {
    fn new(bits:&Vec<u32>) -> Bits {
        let value = bits.iter().map(|byte| byte.to_be_bytes()).collect();

        let bit_size = value.len() << 5;

        // nb of bits is a multiple of 32 hence no padding
        let padding = 0;

        Bits { 
            value,
            bitsize,
            padding 
        }
    }
    fn from_u8vec_BE(bits:&Vec<u8>) -> Bits {
        let mut bits = bits;

        // padding in bits
        let padding = (bits.len() & 3 ) << 3;

        bits = bits.extend(vec![0; padding]);

        let value = bits.chunks(4)
            .map(|slice|
                    slice[0].to_be_bytes() << 24 +
                    slice[1].to_be_bytes() << 16 +
                    slice[2].to_be_bytes() << 8 +
                    slice[3].to_be_bytes()
                )
            .collect::<Vec<u32>>();

        let bitsize = value.len() << 5;

        Bits { value,
               bitsize,
               padding,
        }
    }
    // rotates circularly to the right of n bits 
    // rotates no more than l, l being the size of the bits
    fn circular_rotR(&mut self, n:usize) {
           let n = if n < self.bitsize
           { n }
           else
           { n % self.bitsize };

           let rotation_nb = n >> 5;
           let rotation_remainder = n & 31;

           self.value.rotate_right(rotation_nb); 
           // TODO remainder
    }
    // rotates the value of a Bit struct in the binary sense
    fn rotR(&mut self, n:usize) {
           if n >= self.bitsize {
                self.value = vec![];
           }

           let rotation_nb = n >> 5;

           self.value = self.value.enumerate()
                     .retain(|i, _| i > rotation_nb)
                     .collect();

           let rotation_remainder = n & 31;
           // TODO remainder
           
    }
}

pub struct Block {
    first_blocks: Vec<u32>, // 16 first 32bits blocks
    remaining_blocks: Vec<u32>, // 48 remaining 32bits blocks
}

pub struct Sha256 {
    hs: Vec<u32> // fixed size of 8 : [a,b,c,d,e,f,g,h] in the paper
    size: usize // the number of blocks processed
}

impl Sha256 {
    fn new(bytes:&Vec<u8>) -> Sha256
    {
        // same as (len * 8) modulo 512
        // in big endian
        //(bytes.len() << 3) & 511;
        let size = 0;
        let hs = vec![
            0x6a09e667,
            0xbb67ae85,
            0x3c6ef372,
            0xa54ff53a,
            0x510e527f,
            0x9b05688c,
            0x1f83d9ab,
            0x5be0cd19
            ];

        Sha256 {
            hs,
            size
        }
    }
}
