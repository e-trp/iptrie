
use std::fmt;
use std::str::FromStr;


const PARSE_ERROR: &str = "parse string error";


#[derive(Debug)] 
struct IPV4Node {
    prefix: u32,
    lenght: u8
}

struct IPV4Trie {
    root: IPV4Node,
    size: u64,
}

impl fmt::Display for IPV4Node {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}.{}/{}",    
            self.prefix >> 24 & 0xff, self.prefix >> 16 & 0xff, self.prefix >> 8 & 0xff, self.prefix & 0xff, self.lenght
        )
    }
}


impl FromStr for IPV4Node {

    type Err = String;  

     fn from_str(string: &str) -> Result<Self, Self::Err>  {
        let (string_ip, lenght) = string.split_once("/").expect(PARSE_ERROR);
        let prefix = string_ip.split(".")
                        .enumerate()
                        .fold(0u32, |s, (i, v)| {
                            s + v.parse::<u32>().expect(PARSE_ERROR)<< ((3-i) * 8) 
                            }
                        );
        Ok(Self{prefix, lenght:lenght.parse::<u8>().expect(PARSE_ERROR)})
     }
} 



fn main() {
   let root = IPV4Node::from_str("101.102.103.114/32").unwrap();
   println!("integer value {}, string value {}", root.prefix, root);
}
