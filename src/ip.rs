use std::fmt;
use std::io::SeekFrom;
use std::str::FromStr;


const PARSE_ERROR: &str = "parse string error";


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


pub struct IpTrie<T> {
    pub root: Option<Cidr<T>>
}

impl  IpTrie<u32> {

        pub fn new(root: Option<Cidr<u32>>) -> Self {
            Self{root}
        }
    
}
