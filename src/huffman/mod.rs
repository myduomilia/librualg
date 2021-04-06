use std::collections::{BTreeMap, BinaryHeap};
use std::cmp::Ordering;
use std::ops::Add;

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
        other.weight.cmp(&self.weight).reverse()
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.weight.cmp(&other.weight).reverse())
    }
}

impl Huffman {
    pub fn encode(text: &str) -> Option<(BTreeMap<String, u8>, Vec<u8>)> {
        let mut dict = BTreeMap::new();
        let mut length = 0usize;
        for ch in text.as_bytes() {
            let entry = dict.entry(ch).or_insert(0i32);
            *entry += 1;
        }
        let mut values = dict.iter().map(|(key, value)| Pair{weight: *value, edge: Edge{value: Some(**key), children:Box::new(None)}}).collect::<BinaryHeap<Pair>>();
        while values.len() > 1 {
            let first = values.pop();
            let second = values.pop();
            let weight = first.as_ref().unwrap().weight + second.as_ref().unwrap().weight;
            values.push(Pair{ weight, edge: Edge{value: None, children: Box::new(Some([first.unwrap().edge, second.unwrap().edge]))}});
        }
        if !values.is_empty() {
            let mut encode_table = BTreeMap::new();
            let mut decode_table = BTreeMap::new();
            extract_character_codes(&values.pop().unwrap().edge, "".to_string(), &mut encode_table);
            for (key, value) in &encode_table {
                length += *dict.get(key).unwrap() as usize * value.len();
                decode_table.insert(value.clone(), *key);
            }
            let mut data: Vec<u8> = vec![0; length / 8 + 2];
            data[0] = (data.len() * 8 - length) as u8;
            let mut idx = data.len() * 8 - length;
            for ch in text.as_bytes() {
                println!("{}", encode_table.get(ch).unwrap());
                for bit in encode_table.get(ch).unwrap().as_bytes() {
                    if *bit == b'1' {
                        let mask = 128 >> idx % 8;
                        data[idx / 8] |= mask
                    }
                    idx += 1;
                }
            }
            return Some((decode_table, data));
        }
        None
    }
}

fn extract_character_codes(edge: &Edge, code: String, table: &mut BTreeMap<u8, String>) {
    if let Some(ch) = edge.value {
        table.insert(ch, code);
    } else {
        if let Some(ref children) = *edge.children {
            extract_character_codes(&children[0], code.clone().add("0"), table);
            extract_character_codes(&children[1], code.clone().add("1"), table);
        }
    }
}


#[test]
fn test(){
    Huffman::encode("");
    let (_, bytes) = Huffman::encode("abracadabra").unwrap();
}
