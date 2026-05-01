use iptrie::ip::{Cidr, CidrTrie};
use std::str::FromStr;

fn main() {
   let root = Cidr::<u32>::from_str("101.102.103.114/32").unwrap();
   println!("integer value {}, string value {}", root.net, root);
   let trie = CidrTrie::<Cidr<u32>>::new(Some(root));
}
