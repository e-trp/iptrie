use iptrie::ip::{Cidr, CidrTrait, CidrTrie};

fn main() {
   let root: Cidr<u32> = "101.102.103.114/27".parse().unwrap();

   let address = root.iter().map(Cidr::<u32>::from).collect::<Vec<Cidr<u32>>>();
   for addr in address {
      println!("{}", addr);
   }

   println!("integer value {}, string value {}", root.address, root);
   let _trie = CidrTrie::<Cidr<u32>>::new(Some(root));
}
