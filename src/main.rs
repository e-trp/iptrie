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
    trie.insert("10.10.0.0/16".parse().unwrap());
    trie.insert("192.168.0.0/16".parse().unwrap());

    let mut lookup_value = "10.0.0.1/32".parse::<Cidr<u32>>().unwrap();
    let mut result = trie.search( &lookup_value);
    assert!(result.is_none());

    lookup_value = "192.168.0.0/16".parse().unwrap();
    result = trie.search(&lookup_value);
    assert!(result.is_some());

    lookup_value = "10.10.10.0/24".parse().unwrap();
    println!("{:?}",trie.search_supernets(&lookup_value));
    
}
