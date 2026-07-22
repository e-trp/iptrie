#![allow(unused)]

const PARSE_ERROR: &str = "parse string error";

pub trait IpUsignedInt:
    Copy
    + Eq
    + Ord
    + std::fmt::Display
    + std::ops::BitAnd<Output = Self>
    + std::ops::BitOr<Output = Self>
    + std::ops::Not<Output = Self>
    + std::ops::Shl<u8, Output = Self>
    + std::ops::Shr<u8, Output = Self>
{
    const BITS: u8;
    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;
}

impl IpUsignedInt for u32 {
    const BITS: u8 = 32;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MAX: Self = u32::MAX;
}

impl IpUsignedInt for u128 {
    const BITS: u8 = 128;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const MAX: Self = u128::MAX;
}

#[derive(Debug, PartialEq)]
pub struct Cidr<T: IpUsignedInt> {
    address: T,
    length: u8,
}

pub struct CidrIter<T: IpUsignedInt> {
    start: T,
    end: T,
}

impl<T: IpUsignedInt> Cidr<T> {
    pub fn new(address: T, length: u8) -> Self {
        assert!(length <= T::BITS);
        Self { address, length }
    }
}

pub trait CidrTrait {
    type AddrType: IpUsignedInt;

    fn prefix_len(&self) -> u8;

    fn address(&self) -> Self::AddrType;

    #[inline(always)]
    fn bits(&self) -> impl Iterator<Item = u8> + '_ {
        let addr = self.network();
        let bits = Self::AddrType::BITS;
        let len = self.prefix_len();
        (0..len).map(move |i| {
            (((addr >> (bits - i - 1)) & Self::AddrType::ONE) == Self::AddrType::ONE) as u8
        })
    }

    #[inline(always)]
    fn mask(&self) -> Self::AddrType {
        Self::AddrType::MAX << (Self::AddrType::BITS - self.prefix_len())
    }

    fn network(&self) -> Self::AddrType {
        self.address() & self.mask()
    }

    fn broadcast(&self) -> Self::AddrType {
        self.network() | (!self.mask())
    }

    #[inline(always)]
    fn is_set(&self, bit: u8) -> bool {
        ((self.network() >> (bit - 1)) & Self::AddrType::ONE) == Self::AddrType::ONE
    }

    fn common_bit_len(&self, other: &Self) -> u8 {
        self.bits()
            .zip(other.bits())
            .take_while(|(l, r)| *l == *r)
            .count() as u8
    }

    fn iter(&self) -> CidrIter<Self::AddrType> {
        CidrIter {
            start: self.network(),
            end: self.broadcast(),
        }
    }
}

impl std::fmt::Display for Cidr<u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let [o1, o2, o3, o4] = self.address().to_be_bytes();
        write!(f, "{o1}.{o2}.{o3}.{o4}/{0}", self.prefix_len())
    }
}

/// Parse str to get cidr object
/// ```
/// use iptrie::ip::Cidr;
///
/// let network = Cidr::new(1701209970u32, 27);
/// let net_from_str: Cidr<u32> = "101.102.103.114/27".parse().unwrap();
/// assert_eq!(network, net_from_str);
/// ```
impl std::str::FromStr for Cidr<u32> {
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
    fn prefix_len(&self) -> u8 {
        self.length
    }

    #[inline(always)]
    fn address(&self) -> Self::AddrType {
        self.address
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
    /// let mut lookup = "10.0.0.1/32".parse::<Cidr<u32>>().unwrap();
    /// let mut result = trie.search(&lookup);
    /// assert!(result.is_none());
    ///
    /// lookup = "192.168.0.0/16".parse().unwrap();
    /// result = trie.search(&lookup);
    /// assert!(result.is_some());
    /// ```
    pub fn insert(&mut self, cidr: T) -> Option<bool> {
        let mut current_node = self.root.as_mut()?;

        for bit in cidr.bits() {
            if bit == 1 {
                if current_node.right.is_none() {
                    current_node.right = Some(Box::new(CidrNode::<T>::default()));
                }
                current_node = current_node.right.as_mut()?;
            } else {
                if current_node.left.is_none() {
                    current_node.left = Some(Box::new(CidrNode::<T>::default()));
                }
                current_node = current_node.left.as_mut()?;
            }
        }

        current_node.value = Some(cidr);

        Some(true)
    }

    pub fn search(&self, cidr: &T) -> Option<&T> {
        let mut current_node = self.root.as_ref()?;
        for bit in cidr.bits() {
            current_node = if bit == 1 {
                current_node.right.as_ref()?
            } else {
                current_node.left.as_ref()?
            };
        }
        current_node.value.as_ref()
    }

    pub fn search_supernets(&self, cidr: &T) -> Option<Vec<&T>> {
        let mut current_node = self.root.as_ref()?;
        let len = cidr.prefix_len();
        let network = cidr.network();
        let mut result = Vec::<&T>::with_capacity(5);
        for (index, bit) in cidr.bits().enumerate() {
            if bit == 1 {
                if let Some(node) = current_node.right.as_ref() {
                    if let Some(cidr) = node.value.as_ref() {
                        result.push(cidr);
                    }
                    current_node = node;
                } else {
                    break;
                }
            } else {
                if let Some(node) = current_node.left.as_ref() {
                    if let Some(cidr) = node.value.as_ref() {
                        result.push(cidr);
                    }
                    current_node = node;
                } else {
                    break;
                }
            }
        }
        Some(result)
    }

    fn travers_values_from_node<'a>(node: Option<&'a CidrNode<T>>, result: &mut Vec<&'a T>) {
        if let Some(node) = node {
            if let Some(value) = &node.value {
                result.push(value);
            }
            if let Some(right) = node.right.as_ref() {
                Self::travers_values_from_node(Some(right), result);
            }
            if let Some(left) = node.left.as_ref() {
                Self::travers_values_from_node(Some(left), result);
            }
        }
    }

    pub fn search_subnets(&self, cidr: &T) -> Option<Vec<&T>> {
        let mut result: Vec<&T> = Vec::with_capacity(5);
        let mut current_node = self.root.as_ref()?;
        let len = cidr.prefix_len();
        let network = cidr.network();
        for bit in cidr.bits() {
            if bit == 1 {
                current_node = current_node.right.as_ref()?;
            } else {
                current_node = current_node.left.as_ref()?;
            }
        }
        Self::travers_values_from_node(Some(current_node), &mut result);
        Some(result)
    }
}

#[derive(Debug, Clone)]
pub struct CidrPatriciaNode<T: CidrTrait> {
    value: Option<T>,
    skip: u8,
    left: Option<Box<CidrPatriciaNode<T>>>,
    right: Option<Box<CidrPatriciaNode<T>>>,
}

impl<T: CidrTrait> Default for CidrPatriciaNode<T> {
    fn default() -> Self {
        Self {
            value: None,
            skip: 0u8,
            left: None,
            right: None,
        }
    }
}

impl<T: CidrTrait> From<T> for CidrPatriciaNode<T> {
    fn from(value: T) -> Self {
        Self {
            value: Some(value),
            skip: 0u8,
            left: None,
            right: None,
        }
    }
}

pub struct PatriciaCidrTrie<T: CidrTrait> {
    pub root: Option<CidrPatriciaNode<T>>,
}

impl<T: CidrTrait> Default for PatriciaCidrTrie<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: CidrTrait> PatriciaCidrTrie<T> {
    pub fn new() -> Self {
        Self {
            root: Some(CidrPatriciaNode::<T>::default()),
        }
    }
}

impl<T: CidrTrait> PatriciaCidrTrie<T> {
    pub fn insert(&mut self, cidr: T) -> Option<bool> {
        let mut current_node = self.root.as_mut()?;
        let prefix_len = cidr.prefix_len();
        let network = cidr.network();

        if current_node.skip == 0u8 {
            current_node.skip = prefix_len - 1;
            if cidr.is_set(1) {
                current_node.right = Some(Box::new(CidrPatriciaNode::<T>::from(cidr)));
            } else {
                current_node.left = Some(Box::new(CidrPatriciaNode::<T>::from(cidr)));
            }
        } else {
            todo!()
        }

        Some(false)
    }
}
