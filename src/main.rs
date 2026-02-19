
use std::fmt;
use std::str::FromStr;


const PARSE_ERROR: &str = "parse string error";


#[derive(Debug)] 
struct IPV4Node {
    prefix: u32,
    length: u8
}

struct IPV4Trie {
    root: Option<IPV4Node>
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


fn main() {
   let root = IPV4Node::from_str("101.102.103.114/32").unwrap();
   println!("integer value {}, string value {}", root.prefix, root);
   let trie = IPV4Trie::new(Some(root));
}
