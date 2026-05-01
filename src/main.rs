use iptrie::ip::{Cidr, IpTrie};
use std::str::FromStr;

fn main() {
   let root = Cidr::<u32>::from_str("101.102.103.114/32").unwrap();
   println!("integer value {}, string value {}", root.net, root);
   let trie = IpTrie::<u32>::new(Some(root));
}
