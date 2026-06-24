use iptrie::ip::{Cidr, CidrTrait, CidrTrie};

fn main() {
   let root: Cidr<u32> = "101.102.103.114/24".parse().unwrap();
   println!("{:?} {:?} {:?}", root.mask(), root.network(), root.broadcast());
   println!("integer value {}, string value {}", root.address, root);
   let _trie = CidrTrie::<Cidr<u32>>::new(Some(root));
}
