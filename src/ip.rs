use std::fmt;
use std::str::FromStr;


const PARSE_ERROR: &str = "parse string error";


#[derive(Debug)] 
pub struct Cidr<T> {
   pub address: T,
   pub length: u8
}


pub struct  CidrIter<T> {
    pub start: T,
    pub end: T, 
}

pub trait CidrTrait  {

    type AddrType;

    fn mask(&self) -> Self::AddrType;

    fn network(&self) -> Self::AddrType;

    fn broadcast(&self) -> Self::AddrType;

    fn prefix_len(&self) -> u8;
    
    fn iter(&self) -> CidrIter<Self::AddrType>;


}



impl fmt::Display for Cidr<u32> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [o1, o2, o3, o4] = self.address.to_be_bytes();
        write!(f, "{}.{}.{}.{}/{}", o1, o2, o3, o4, self.length)
    }
}


impl FromStr for Cidr<u32> {

    type Err = String;  

    fn from_str(string: &str) -> Result<Self, Self::Err>  {
        let (address_str, length) = string.split_once("/").expect(PARSE_ERROR);
        let address = address_str.split(".")
            .filter_map(|x| x.parse::<u8>().ok())
            .enumerate()
            .map(|(i, val)| (val as u32) << ((3 - i) * 8))
            .sum();
        Ok(Self{address, length:length.parse::<u8>().expect(PARSE_ERROR)})
     }
} 


impl CidrTrait for Cidr<u32>{

    type AddrType = u32;

    #[inline(always)]
    fn mask(&self) -> Self::AddrType {
        !0u32 << (32 - self.length)
    }

    fn network(&self) -> Self::AddrType {
        self.address & self.mask()
    }

    fn broadcast(&self) -> Self::AddrType {
        self.network() | (!self.mask())
    }

    fn prefix_len(&self) -> u8 {
        self.length
    }

    fn iter(&self) -> CidrIter<u32> {
        CidrIter { start: self.network(), end: self.broadcast() }
    }
}

impl Iterator for CidrIter<u32> {
    
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.start += 1;
        if self.start >= self.end {
            return None;
        }
        Some(self.start)
    }

}


pub struct CidrTrie<T: CidrTrait> {
    pub root: Option<T>,
}

impl<T: CidrTrait> CidrTrie<T> {
    pub fn new(root: Option<T>) -> Self {
        Self { root }
    }
}