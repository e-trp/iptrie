#![allow(unused)]
use std::fmt;
use std::str::FromStr;

const PARSE_ERROR: &str = "parse string error";

#[derive(Debug, PartialEq)]
pub struct Cidr<T> {
    pub address: T,
    pub length: u8,
}

pub struct CidrIter<T> {
    start: T,
    end: T,
}

pub trait CidrTrait {
    type AddrType;

    fn mask(&self) -> Self::AddrType;

    fn network(&self) -> Self::AddrType;

    fn broadcast(&self) -> Self::AddrType;

    fn prefix_len(&self) -> u8;

    fn iter(&self) -> CidrIter<Self::AddrType>;

    fn bits(&self) -> impl Iterator<Item = u8> + '_;
}

impl fmt::Display for Cidr<u32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [o1, o2, o3, o4] = self.address.to_be_bytes();
        write!(f, "{o1}.{o2}.{o3}.{o4}/{0}", self.length)
    }
}

/// Parse str to get cidr object
/// ```
/// use iptrie::ip::Cidr;
///
/// let network = Cidr{address: 1701209970u32, length: 27};
/// let net_from_str: Cidr<u32> = "101.102.103.114/27".parse().unwrap();
/// assert_eq!(network, net_from_str);
/// ```
impl FromStr for Cidr<u32> {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (address_str, length) = string.split_once("/").expect(PARSE_ERROR);
        let address = address_str
            .split(".")
            .filter_map(|x| x.parse::<u8>().ok())
            .enumerate()
            .map(|(i, val)| (val as u32) << ((3 - i) * 8))
            .sum();
        Ok(Self {
            address,
            length: length.parse::<u8>().expect(PARSE_ERROR),
        })
    }
}

impl From<u32> for Cidr<u32> {
    fn from(value: u32) -> Self {
        Self {
            address: value,
            length: 32u8,
        }
    }
}

impl CidrTrait for Cidr<u32> {
    type AddrType = u32;

    #[inline(always)]
    fn mask(&self) -> Self::AddrType {
        !0u32 << (32 - self.length)
    }

    fn network(&self) -> Self::AddrType {
        self.address & self.mask()
    }

    fn broadcast(&self) -> Self::AddrType {
        self.network() | (!self.mask())
    }

    fn prefix_len(&self) -> u8 {
        self.length
    }

    fn iter(&self) -> CidrIter<u32> {
        CidrIter {
            start: self.network(),
            end: self.broadcast(),
        }
    }

    fn bits(&self) -> impl Iterator<Item = u8> + '_ {
        let addr = self.network();
        let len = self.prefix_len();
        (0..len).map(move |i| ((addr >> (31 - i)) & 1) as u8)
    }
}

impl Iterator for CidrIter<u32> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.start += 1;
        if self.start >= self.end {
            return None;
        }
        Some(self.start)
    }
}

#[derive(Debug, Clone)]
pub struct CidrNode<T: CidrTrait> {
    value: Option<T>,
    left: Option<Box<CidrNode<T>>>,
    right: Option<Box<CidrNode<T>>>,
}

impl<T: CidrTrait> Default for CidrNode<T> {
    fn default() -> Self {
        Self {
            value: None,
            left: None,
            right: None,
        }
    }
}

impl<T: CidrTrait> From<T> for CidrNode<T> {
    fn from(value: T) -> Self {
        Self {
            value: Some(value),
            left: None,
            right: None,
        }
    }
}

pub struct CidrTrie<T: CidrTrait> {
    pub root: Option<CidrNode<T>>,
}

impl<T: CidrTrait> Default for CidrTrie<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: CidrTrait> CidrTrie<T> {
    pub fn new() -> Self {
        Self {
            root: Some(CidrNode::<T>::default()),
        }
    }

    /// # Example (exact match trie)
    /// ```
    /// use iptrie::ip::{Cidr, CidrTrie};
    ///
    /// let mut trie = CidrTrie::<Cidr<u32>>::new();
    ///
    /// trie.insert("10.0.0.0/8".parse().unwrap());
    /// trie.insert("192.168.0.0/16".parse().unwrap());
    ///
    /// let mut result = trie.search("10.0.0.1/32".parse().unwrap());
    /// assert!(result.is_none());
    ///
    /// result = trie.search("192.168.0.0/16".parse().unwrap());
    /// assert!(result.is_some());
    /// ```
    pub fn insert(&mut self, cidr: T) {
        let mut current_node = self.root.as_mut().unwrap();

        for bit in cidr.bits() {
            if bit == 1 {
                if current_node.right.is_none() {
                    current_node.right = Some(Box::new(CidrNode::<T>::default()));
                }
                current_node = current_node.right.as_mut().unwrap();
            } else {
                if current_node.left.is_none() {
                    current_node.left = Some(Box::new(CidrNode::<T>::default()));
                }
                current_node = current_node.left.as_mut().unwrap();
            }
        }

        current_node.value = Some(cidr);
    }

    pub fn search(&self, cidr: T) -> Option<&T> {
        let mut current_node = self.root.as_ref().unwrap();
        for bit in cidr.bits() {
            current_node = if bit == 1 {
                current_node.right.as_ref()?
            } else {
                current_node.left.as_ref()?
            };
        }
        current_node.value.as_ref()
    }
}
