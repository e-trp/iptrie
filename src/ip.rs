use std::fmt;
use std::str::FromStr;


const PARSE_ERROR: &str = "parse string error";


#[derive(Debug)] 
pub struct IPV4Node {
   pub prefix: u32,
   pub length: u8
}

pub struct IPV4Trie {
    pub root: Option<IPV4Node>
}

impl fmt::Display for IPV4Node {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}.{}/{}",    
            self.prefix >> 24,
            self.prefix >> 16 & 0xff,
            self.prefix >> 8 & 0xff,
            self.prefix & 0xff,
            self.length
        )
    }
}


impl FromStr for IPV4Node {

    type Err = String;  

    fn from_str(string: &str) -> Result<Self, Self::Err>  {
        let (string_ip, length) = string.split_once("/").expect(PARSE_ERROR);
        let prefix = string_ip.split(".")
            .filter_map(|x| x.parse::<u8>().ok())
            .enumerate()
            .map(|(i, val)| (val as u32) << ((3 - i) * 8))
            .sum();
        Ok(Self{prefix, length:length.parse::<u8>().expect(PARSE_ERROR)})
     }
} 


impl IPV4Trie {

    pub fn new(root: Option<IPV4Node>) -> Self {
        Self { root }
    }

    pub fn insert(&self, node: IPV4Node) {

    }
}

impl IPV4Node {
    pub fn prefix(&self) -> u32 {
        self.prefix
    }
}
