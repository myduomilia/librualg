use std::collections::{BTreeMap, BinaryHeap};
use std::cmp::Ordering;

struct Huffman {

}

#[derive(Clone, Eq, PartialEq)]
struct Edge {
    value: Option<u8>,
    children: Box<Option<[Edge; 2]>>,
}

#[derive(Clone, Eq, PartialEq)]
struct Pair {
    weight: i32,
    edge: Edge
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

impl Huffman {
    pub fn encode(text: &str) {
        let mut dict = BTreeMap::new();
        for ch in text.as_bytes() {
            let mut entry = dict.entry(ch).or_insert(0i32);
            *entry += 1;
        }
        let mut values = dict.iter().map(|(key, value)| Pair{weight: *value * -1, edge: Edge{value: Some(**key), children:Box::new(None)}}).collect::<BinaryHeap<Pair>>();
        while values.len() > 1 {
            let first = values.pop();
            let second = values.pop();
            let weight = first.as_ref().unwrap().weight + second.as_ref().unwrap().weight;
            values.push(Pair{ weight, edge: Edge{value: None, children: Box::new(Some([first.unwrap().edge, second.unwrap().edge]))}});
        }
    }
}

#[test]
fn test(){

    Huffman::encode("abracadabra ");

}