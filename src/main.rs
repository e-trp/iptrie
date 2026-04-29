use iptrie::ip::{IPV4Node,IPV4Trie};
use std::str::FromStr;

fn main() {
   let root = IPV4Node::from_str("101.102.103.114/32").unwrap();
   println!("integer value {}, string value {}", root.prefix, root);
   let trie = IPV4Trie::new(Some(root));
}
