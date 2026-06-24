use iptrie::ip::{Cidr, CidrTrie};

fn main() {
   let root: Cidr<u32> = "101.102.103.114/32".parse().unwrap();
   println!("integer value {}, string value {}", root.address, root);
   let _trie = CidrTrie::<Cidr<u32>>::new(Some(root));
}
