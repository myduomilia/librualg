use std::collections::BTreeMap;

/// DSU
/// ```
/// use librualg::dsu::{DSURef, DSU};
///
/// let mut dsu = DSURef::new();
/// let v = (0..10).collect::<Vec<u32>>();
/// for i in &v {
///     dsu.make_set(i);
/// }
/// dsu.union_sets(&v[1], &v[2]);
/// dsu.union_sets(&v[2], &v[3]);
/// dsu.union_sets(&v[2], &v[7]);
///
/// assert_eq!(dsu.find_set(&v[2]), dsu.find_set(&v[7]));
/// assert_ne!(dsu.find_set(&v[2]), dsu.find_set(&v[8]));
/// assert_eq!(dsu.find_set(&v[1]), dsu.find_set(&v[3]));
///
/// let mut dsu = DSU::new();
/// for i in 0..10 {
///     dsu.make_set(i);
/// }
/// dsu.union_sets(1, 2);
/// dsu.union_sets(2, 3);
/// dsu.union_sets(2, 7);
///
/// assert_eq!(dsu.find_set(2), dsu.find_set(7));
/// assert_ne!(dsu.find_set(2), dsu.find_set(8));
/// assert_eq!(dsu.find_set(1), dsu.find_set(3));
/// ```

pub struct DSURef<'a, T> where T: Eq + Ord {
    parent: BTreeMap<&'a T, &'a T>,
    ranks: BTreeMap<&'a T, usize>
}

impl <'a, T> DSURef<'a, T> where T: Eq + Ord {
    pub fn new() -> Self {
        DSURef {
            parent: BTreeMap::new(),
            ranks: BTreeMap::new()
        }
    }
    pub fn make_set(&mut self, value: &'a T) {
        if !self.parent.contains_key(&value) {
            self.parent.insert(value, value);
            self.ranks.insert(value, 1);
        }
    }

    pub fn find_set(&mut self, value: &'a T) -> Option<&'a T> {
        if !self.parent.contains_key(value) {
            return None;
        }
        let reference = *self.parent.get(value).unwrap();
        if *value == *reference {
            return Some(value);
        }
        let next = self.find_set(reference).unwrap();
        *self.parent.get_mut(value).unwrap() = next;
        return Some(next);
    }

    pub fn union_sets(&mut self, first: &'a T, second: &'a T) {
        let first = self.find_set(first);
        let second = self.find_set(second);
        if first.is_some() && second.is_some() {
            if *first.unwrap() != *second.unwrap() {
                let first_rank = *self.ranks.get(first.as_ref().unwrap()).unwrap();
                let second_rank = *self.ranks.get(second.as_ref().unwrap()).unwrap();
                if second_rank >= first_rank {
                    let key = second.unwrap();
                    *self.parent.get_mut(&key).unwrap() = first.unwrap();
                    *self.ranks.get_mut(&key).unwrap() = first_rank + second_rank;
                } else {
                    let key = first.unwrap();
                    *self.parent.get_mut(&key).unwrap() = second.unwrap();
                    *self.ranks.get_mut(&key).unwrap() = first_rank + second_rank;
                }
            }
        }
    }
}

pub struct DSU<T> where T: Eq + Ord + Clone {
    parent: BTreeMap<T, T>,
    ranks: BTreeMap<T, usize>
}

impl <T> DSU<T> where T: Eq + Ord + Clone {
    pub fn new() -> Self {
        DSU {
            parent: BTreeMap::new(),
            ranks: BTreeMap::new()
        }
    }
    pub fn make_set(&mut self, value: T) {
        if !self.parent.contains_key(&value) {
            self.parent.insert(value.clone(), value.clone());
            self.ranks.insert(value.clone(), 1);
        }
    }

    pub fn find_set(&mut self, value: T) -> Option<T> {
        if !self.parent.contains_key(&value) {
            return None;
        }
        let reference = self.parent.get(&value).unwrap().clone();
        if value == reference {
            return Some(value);
        }
        let next = self.find_set(reference).unwrap();
        *self.parent.get_mut(&value).unwrap() = next.clone();
        return Some(next);
    }

    pub fn union_sets(&mut self, first: T, second: T) {
        let first = self.find_set(first);
        let second = self.find_set(second);
        if first.is_some() && second.is_some() {
            if first.as_ref().unwrap() != second.as_ref().unwrap() {
                let first_rank = *self.ranks.get(&first.as_ref().unwrap()).unwrap();
                let second_rank = *self.ranks.get(&second.as_ref().unwrap()).unwrap();
                if second_rank >= first_rank {
                    let key = second.unwrap();
                    *self.parent.get_mut(&key).unwrap() = first.unwrap();
                    *self.ranks.get_mut(&key).unwrap() = first_rank + second_rank;
                } else {
                    let key = first.unwrap();
                    *self.parent.get_mut(&key).unwrap() = second.unwrap();
                    *self.ranks.get_mut(&key).unwrap() = first_rank + second_rank;
                }
            }
        }
    }
}

#[test]
fn test_dsu_ref() {
    let mut dsu = DSURef::new();
    let v = (0..10).collect::<Vec<u32>>();
    for i in &v {
        dsu.make_set(i);
    }
    dsu.union_sets(&v[1], &v[2]);
    dsu.union_sets(&v[2], &v[3]);
    dsu.union_sets(&v[2], &v[7]);

    assert_eq!(dsu.find_set(&v[2]), dsu.find_set(&v[7]));
    assert_ne!(dsu.find_set(&v[2]), dsu.find_set(&v[8]));
    assert_eq!(dsu.find_set(&v[1]), dsu.find_set(&v[3]));
    assert_ne!(dsu.find_set(&v[1]), dsu.find_set(&v[9]));
    assert_eq!(dsu.find_set(&11), None);
}

#[test]
fn test_dsu() {
    let mut dsu = DSU::new();
    for i in 0..10 {
        dsu.make_set(i);
    }
    dsu.union_sets(1, 2);
    dsu.union_sets(2, 3);
    dsu.union_sets(2, 7);

    assert_eq!(dsu.find_set(2), dsu.find_set(7));
    assert_ne!(dsu.find_set(2), dsu.find_set(8));
    assert_eq!(dsu.find_set(1), dsu.find_set(3));
    assert_ne!(dsu.find_set(1), dsu.find_set(9));
    assert_eq!(dsu.find_set(11), None);
}