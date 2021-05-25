use std::collections::BTreeMap;

/// DSU
/// ```
/// use librualg::dsu::{DSU, DSUCloneable};
///
/// let mut dsu = DSU::new();
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
/// let mut dsu = DSUCloneable::new();
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

pub struct DSU <'a, T> where T: Eq + Ord {
    parent: BTreeMap<&'a T, &'a T>
}

impl <'a, T> DSU<'a, T> where T: Eq + Ord {
    pub fn new() -> Self {
        DSU{
            parent: BTreeMap::new(),
        }
    }
    pub fn make_set(&mut self, value: &'a T) {
        if !self.parent.contains_key(&value) {
            self.parent.insert(value, value);
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
                *self.parent.get_mut(second.unwrap()).unwrap() = first.unwrap();
            }
        }
    }
}

pub struct DSUCloneable<T> where T: Eq + Ord + Clone {
    parent: BTreeMap<T, T>
}

impl <T> DSUCloneable<T> where T: Eq + Ord + Clone {
    pub fn new() -> Self {
        DSUCloneable{
            parent: BTreeMap::new(),
        }
    }
    pub fn make_set(&mut self, value: T) {
        if !self.parent.contains_key(&value) {
            self.parent.insert(value.clone(), value);
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
                *self.parent.get_mut(&second.unwrap()).unwrap() = first.unwrap();
            }
        }
    }
}

#[test]
fn test_dsu() {
    let mut dsu = DSU::new();
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
fn test_dsu_cloneable() {
    let mut dsu = DSUCloneable::new();
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