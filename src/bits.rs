const _H_NUMBER:usize = 8;

// considering array is ordered in big endian order
#[derive(Debug, Clone)]
pub struct Bits {
    pub value: Vec<u32>,
    // size in bits
    bitsize: usize,
    // padding in bits
    padding: usize,
}

pub fn v2u32(data:&[u8]) -> u32
{
        (data[0].to_be_bytes()[0] << 24 +
        data[1].to_be_bytes()[0] << 16 +
        data[2].to_be_bytes()[0] << 8 +
        data[3].to_be_bytes()[0]).into()
}

impl Bits {
    pub fn new(bits:&Vec<u32>) -> Bits {
        let value:Vec<u32> = bits.clone();

        let bitsize = value.len() << 5;

        // nb of bits is a multiple of 32 hence no padding
        let padding = 0;

        Bits { 
            value,
            bitsize,
            padding 
        }
    }
    fn from_u8vec_BE(bits:&Vec<u8>) -> Bits {
        let mut bits = bits.clone();

        // padding in bits
        let padding = (bits.len() & 3 ) << 3;

        bits.extend(vec![0; padding]);

        let value = bits.chunks(4)
            .map(|four_bytes| v2u32(four_bytes))
            .collect::<Vec<u32>>();

        let bitsize = value.len() << 5;

        Bits { value,
               bitsize,
               padding,
        }
    }
    // rotates circularly to the right of n bits 
    // rotates no more than l, l being the size of the bits
    pub fn circular_rotR(&mut self, n:usize) {
           let n = if n < self.bitsize
           { n }
           else
           { n % self.bitsize };

           // the number of rotations to do on the array itself
           let rot_nb = n >> 5;
           println!("init: {:?}", self.value);
           self.value.rotate_right(rot_nb); 

           // TODO shift some bits
           // the number of rotations on bits
           let rot_remainder = n & 31;

           if rot_remainder > 0 {

               let mask = (1 << (rot_remainder + 1)) - 1;
               println!("mask : {}", mask);
               let mut rem_shifts:Vec<u32> = self.value.iter()
                   .map(|four_byte| 
                        {
                            let to_be_shifted = four_byte & mask;
                            to_be_shifted.rotate_right(rot_remainder as u32)
                        }
                   )
                   .collect();

               rem_shifts.rotate_right(1); 
               println!("post_rot: {:?}", self.value);
               println!("shitfs: {:?}", rem_shifts);

               self.value = self.value.iter()
                   .zip(rem_shifts.iter())
                   .map(|(four_byte, nb)| nb + (four_byte >> rot_remainder))
                   .collect();
           }
    }
    // rotates the value of a Bit struct in the binary sense
    pub fn rotR(&mut self, n:usize) {
           if n >= self.bitsize {
                self.value = vec![0; self.value.len()];
           }
           else
           {
               let rot_nb = n >> 5;

               let mut i = 0;
               while i + rot_nb < self.value.len() {
                   self.value[i + rot_nb] = self.value[i];
                   i += 1;
               }

               // TODO actually remove bits later
               for j in 0..rot_nb {
                   self.value[j] = 0;
               }

               let rot_remainder = n & 31;
               let tmp = self.value[rot_nb] & ((2 << rot_remainder) - 1);
               self.value[rot_nb] >>= rot_remainder;
               if rot_nb + 1 < self.value.len()
               {
                   self.value[rot_nb + 1] >>= rot_remainder;
                   self.value[rot_nb + 1] += tmp << (32 - rot_remainder);
               }
           }
    }
}
