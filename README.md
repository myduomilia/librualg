## Collection of basic algorithms for everyday development
[![Build Status](https://travis-ci.org/myduomilia/librualg.svg?branch=master)](https://travis-ci.org/myduomilia/librualg)
[![Rust](https://github.com/myduomilia/librualg/actions/workflows/rust.yml/badge.svg)](https://github.com/myduomilia/librualg/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/librualg)](https://crates.io/crates/librualg)
[![Coverage Status](https://coveralls.io/repos/github/myduomilia/librualg/badge.svg?branch=master)](https://coveralls.io/github/myduomilia/librualg?branch=master)

### LIst of algorithms:
<hr/>

#### Search algorithms:
- Binary search
- Bloom Filter
- Binary Tree

#### Segment Tree:
- RSQ (Range Sum Query)
- RMQMin (Range Minimum Query)
- RMQMax (Range Maximum Query)

#### Sparse Tables:
- SparseTableMin (Range Minimum Queries) 
- SparseTableMax (Range Maximum Queries)

#### String Algorithms:
- Knuth–Morris–Pratt string-searching algorithm (or KMP algorithm)
- Trie or prefix tree
- Levenshtein distance (Metric of the difference between two symbol sequences)
- Search for the minimum string period 
- Search distinct substrings
- Suffix Array
- The Longest Common Prefix
- Search for a common substring (hashing)
- Algorithm Aho Corasick. Search for a set of substring from the dictionary in the given string.

#### Combinatorics and enumeration algorithms
- Permutation generation
#### Graph algorithms:
- BFS (Breadth-First Search)
- DFS (Depth-First Search)
- Dijkstra
- Connected components
- Strongly connected components
- Topologic sort (for DAG)
- Kruskal's algorithm

#### Mathematics algorithms:
- The Greatest Common Divisor (GCD)
- Fast pow
- Fast pow by module
- Checking a Number for Simplicity (Fermat's test)

#### Data compression:
- Huffman algorithm

#### Data Structure:
- DSU (disjoint-set-union)

#### Sheduling:
- Johnson's Algorithm For Scheduling

#### Sort:
- Insertion sort

<hr/>

### Example
```rust
extern crate librualg;
use librualg::*;

fn main(){
    let seq = [1, 2, 3, 3, 4, 5];
    assert_eq!(binary_search::upper_bound(&seq, &3), Some(3));
}
```