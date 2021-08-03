use std::rc::Rc;

pub struct CLList<T> {
    items:  Vec<Rc<T>>,
    index:  usize
}

impl<T> CLList<T> {
    pub fn new() -> Self {
        CLList { items: Vec::new(), index: 0 }
    }

    pub fn with_capacity(capacity:  usize) -> Self {
        CLList { items: Vec::with_capacity(capacity), index: 0 }
    }

    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    pub fn push(&mut self, value: T, replecas: usize) {
        let sourc = Rc::new(value);
        for _ in 0..replecas {
            self.items.push(Rc::clone(&sourc));
        }
    }

    pub fn next(&mut self) -> Option<&T> {
        use std::borrow::Borrow;
        let original_index = self.index;
        self.index = (self.index + 1) % self.items.len();
        self.items
            .get(original_index)
            .map(|x| x.borrow())
    }
}

use std::iter::FromIterator;
impl<T> FromIterator<(usize, T)> for CLList<T> {
    fn from_iter<I: IntoIterator<Item = (usize, T)>>(iter: I) -> Self {
        let mut items = CLList::new();

        for (r, i) in iter {
            items.push(i,r);
        }

        items
    }
}

use serde::de::{self};
use crate::de::Deserialize;
use crate::de::Deserializer;
use serde_derive::Deserialize;


#[derive(Deserialize)]
struct Des<T> {
    data: T,
    replicas: usize
}

impl<'de, T> Deserialize<'de> for CLList<T> 
    where
        T: Deserialize<'de> ,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {

        let map: Vec<Des<T>> = Vec::deserialize(deserializer)?;
        Ok(map
            .into_iter()
            .map(|x| (x.replicas, x.data))
            .collect::<CLList<T>>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter_test() {
        let mut cl = [(5, 50),(8, 6),(2, 6)]
            .to_vec()
            .into_iter()
            .collect::<CLList<u64>>();

        assert_eq!(cl.next(), Some(&50u64));
        assert_eq!(cl.next(), Some(&50u64));
        assert_eq!(cl.next(), Some(&50u64));
        assert_eq!(cl.next(), Some(&50u64));
        assert_eq!(cl.next(), Some(&50u64));
        assert_eq!(cl.next(), Some(&6));
    }

    #[test]
    fn balancing() {
        let mut cl = [(1, 50),(1, 6),(1, 80)]
            .to_vec()
            .into_iter()
            .collect::<CLList<u64>>();

        assert_eq!(cl.next(), Some(&50u64));
        assert_eq!(cl.next(), Some(&6u64));
        assert_eq!(cl.next(), Some(&80u64));
        assert_eq!(cl.next(), Some(&50u64));
        assert_eq!(cl.next(), Some(&6u64));
        assert_eq!(cl.next(), Some(&80u64));
    }
    #[test]
    fn deserialize_test() {
        use std::net::SocketAddr;

        #[derive(Deserialize)]
        struct Test {
            #[serde(rename = "list")]
            _list:   CLList<(SocketAddr, String)>,
        }
        let s = include_str!("../input.toml");
        let _cl: Test = toml::de::from_str(s)
            .unwrap();
    }
}
