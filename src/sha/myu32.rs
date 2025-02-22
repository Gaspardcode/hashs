const overflow:u64 = (1 << 31) - 1;
pub struct U32 {
    value:u32
}
impl U32 {
    fn new U32(val:u32)
    {
         U32 {val}
    }
    fn plus(self, other:u32)
    {
       let a = self.value as u64; 
       let b = other.value as u64; 
       if (a + b) & overflow > 0 {
            
       }
    }
}
