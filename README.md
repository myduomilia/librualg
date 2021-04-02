## Collection of basic algorithms for everyday development
[![Build Status](https://travis-ci.org/myduomilia/librualg.svg?branch=master)](https://travis-ci.org/myduomilia/librualg)
[![Rust](https://github.com/myduomilia/librualg/actions/workflows/rust.yml/badge.svg)](https://github.com/myduomilia/librualg/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/librualg)](https://crates.io/crates/librualg)

### LIst of algorithms:
<hr/>

#### Search algorithms:
- Binary search

#### Segment Tree:
- RSQ (Range Sum Query)
- RMQMin (Range Minimum Query)
- RMQMax (Range Maximum Query)
- RMQMin (Range Minimum Query)

#### String Algorithms:
- Knuth–Morris–Pratt string-searching algorithm (or KMP algorithm)
- Trie or prefix tree

#### Combinatorics and enumeration algorithms
- Permutation generation
#### Graph algorithms:
- bfs (Breadth-First Search)
- dfs (Depth-First Search)
- dijkstra

#### Mathematics algorithms:
- The Greatest Common Divisor (GCD)

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