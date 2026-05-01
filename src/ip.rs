use std::fmt;
use std::str::FromStr;


const PARSE_ERROR: &str = "parse string error";


pub trait CidrTrait  {
    type AddrType;

    fn network(&self) -> Self::AddrType;
    fn prefix_len(&self) -> u8;
}


#[derive(Debug)] 
pub struct Cidr<T> {
   pub net: T,
   pub length: u8
}

impl fmt::Display for Cidr<u32> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [o1, o2, o3, o4] = self.net.to_be_bytes();
        write!(f, "{}.{}.{}.{}/{}", o1, o2, o3, o4, self.length)
    }
}


impl FromStr for Cidr<u32> {

    type Err = String;  

    fn from_str(string: &str) -> Result<Self, Self::Err>  {
        let (net_string, length) = string.split_once("/").expect(PARSE_ERROR);
        let net = net_string.split(".")
            .filter_map(|x| x.parse::<u8>().ok())
            .enumerate()
            .map(|(i, val)| (val as u32) << ((3 - i) * 8))
            .sum();
        Ok(Self{net, length:length.parse::<u8>().expect(PARSE_ERROR)})
     }
} 


impl CidrTrait for Cidr<u32>{

    type AddrType = u32;

    fn network(&self) -> Self::AddrType {
        self.net
    }

    fn prefix_len(&self) -> u8 {
        self.length
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