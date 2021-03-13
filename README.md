## Collection of basic algorithms for everyday development
[![Build Status](https://travis-ci.org/myduomilia/librualg.svg?branch=master)](https://travis-ci.org/myduomilia/librualg)
[![Rust](https://github.com/myduomilia/librualg/actions/workflows/rust.yml/badge.svg)](https://github.com/myduomilia/librualg/actions/workflows/rust.yml)
![crates.io](https://img.shields.io/crates/v/librualg)

### LIst of algorithms:
- Binary search algorithm.
- Knuth–Morris–Pratt string-searching algorithm (or KMP algorithm)
- Algorithm of permutation generation

### Example
```rust
extern crate librualg;
use librualg::*;

fn main(){
    let seq = [1, 2, 3, 3, 4, 5];
    assert_eq!(binary_search::upper_bound(&seq, &3), Some(3));
}
```