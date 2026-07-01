#![allow(unused)]
use iptrie::ip::{Cidr, CidrTrait, CidrTrie};

fn main() {
    let root: Cidr<u32> = "101.102.103.114/27".parse().unwrap();

    let address = root
        .iter()
        .map(Cidr::<u32>::from)
        .collect::<Vec<Cidr<u32>>>();
    for addr in address {
        println!("{}", addr);
    }

    let mut trie = CidrTrie::<Cidr<u32>>::new();
    trie.insert("10.0.0.0/8".parse().unwrap());
    trie.insert("192.168.0.0/16".parse().unwrap());

    let mut result = trie.search("10.0.0.1/32".parse().unwrap());
    assert!(result.is_none());

    result = trie.search("192.168.0.0/16".parse().unwrap());
    assert!(result.is_some());
    dbg!(result);
}
