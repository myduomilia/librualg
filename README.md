## Collection of basic algorithms for everyday development
[![Build Status](https://travis-ci.org/myduomilia/librualg.svg?branch=master)](https://travis-ci.org/myduomilia/librualg)

### LIst of algorithms:
- Binary search algorithm.

### Example
```rust
extern crate librualg;
use librualg::*;

fn main(){
    let seq = [1, 2, 3, 3, 4, 5];
    assert_eq!(binary_search::upper_bound(&seq, &3), Some(3));
}
```