use std::str::FromStr;
use primitive_types::U256;
use hex;

#[derive(Debug, Clone)]

pub struct EU256 {
    pub v: U256
}
#[derive(Debug, PartialEq, Eq)]
pub struct EU256ParseError;

impl FromStr for EU256 {
    type Err = EU256ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match U256::from_str_radix(txt: &str, radix: u32){
            Ok(n: U256) => return Ok(Self {v: n}),
            Err(_) => return Err(EU256ParseError);
        }
    }
    
}

impl ToString for EU256 {
    fn to_string(&self) -> String {
        let mut bytes: [u8; 32] = [0;32];
        self.v.to_big_endian(&mut bytes);
        return hex::encode(data:bytes);
    }
    
}