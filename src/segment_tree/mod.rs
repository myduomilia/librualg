use std::ops::Add;
use std::mem::swap;
use std::cmp::{min, max};

/// Range Sum Query

pub struct Rsq<T: Default + Clone + Copy + Add<Output = T>> {
    data: Vec<T>,
    len: usize,
}

impl <T> Rsq<T>  where T: Default + Clone + Copy + Add<Output = T> {

    /// Build Segment Tree (RSQ) from slice.
    ///```
    /// use librualg::segment_tree::Rsq;
    ///
    /// let arr = [1, 2, 3, 4, 5];
    /// let tree = Rsq::new(&arr);
    /// ```
    pub fn new(src: &[T]) -> Self {
        if src.is_empty() {
            return Rsq { data: vec![], len: src.len() };
        }
        let n = determine_necessary_size_tree(src.len());
        let mut dst = vec![T::default(); n];
        for (i, value) in src.iter().enumerate() {
            dst[n / 2 + i] = *value;
        }
        for i in (1..n / 2).rev() {
            dst[i] = dst[2 * i] + dst[2 * i + 1];
        }
        Rsq { data: dst, len: src.len() }
    }

    /// Returns the sum on the interval l to r
    ///```
    /// use librualg::segment_tree::Rsq;
    ///
    /// let arr = [1, 2, 3, 4, 5];
    /// let tree = Rsq::new(&arr);
    ///
    /// assert_eq!(tree.query(0, 4).unwrap(), 15);
    /// assert_eq!(tree.query(1, 4).unwrap(), 14);
    /// assert_eq!(tree.query(4, 1).unwrap(), 14);
    /// assert_eq!(tree.query(3, 1).unwrap(), 9);
    /// assert_eq!(tree.query(3, 11), None);
    /// ```
    pub fn query(&self, l: usize, r: usize) -> Option<T> {
        if self.data.is_empty() || l >= self.len || r >= self.len {
            return None;
        }
        let mut l = l + self.data.len() / 2;
        let mut r = r + self.data.len() / 2;
        if l > r {
            swap(&mut l, &mut r);
        }
        let mut res = T::default();
        while l <= r {
            if l % 2 != 0 {
                res = res + self.data[l];
            }
            l = (l + 1) >> 1;
            if r % 2 == 0 {
                res = res + self.data[r];
            }
            r = (r - 1 ) >> 1;
        }
        Some(res)
    }

    /// Update value by index
    ///```
    /// use librualg::segment_tree::Rsq;
    /// let arr = [1, 2, 3, 4, 5];
    /// let mut tree = Rsq::new(&arr);
    ///
    /// assert_eq!(tree.query(0, 4).unwrap(), 15);
    /// tree.update(1, 7);
    /// assert_eq!(tree.query(0, 4).unwrap(), 20);
    /// ```
    pub fn update(&mut self, mut idx: usize, value: T) {
        if !self.data.is_empty() && idx < self.len {
            idx += self.data.len() / 2;
            self.data[idx] = value;
            while idx >= 1 {
                if idx % 2 == 0 {
                    self.data[idx / 2] = self.data[idx] + self.data[idx + 1];
                } else {
                    self.data[idx / 2] = self.data[idx] + self.data[idx - 1];
                }
                idx /= 2;
            }
        }
    }
}

/// Range Minimum Query

pub struct RmqMin<T: Default + Clone + Copy + SegmentTreeMin + SegmentTreeMax + Ord > {
    data: Vec<T>,
    len: usize,
}

impl <T> RmqMin<T>  where T: Default + Clone + Copy + SegmentTreeMin + SegmentTreeMax + Ord {

    /// Build Segment Tree (RMQMin) from slice.
    ///```
    /// use librualg::segment_tree::{RmqMin, SegmentTreeMin, SegmentTreeMax};
    /// use std::prelude::v1::*;
    ///
    /// let arr = [1, 2, 3, 4, 5];
    /// let tree = RmqMin::new(&arr);
    /// ```
    pub fn new(src: &[T]) -> Self {
        if src.is_empty() {
            return RmqMin { data: vec![], len: src.len() };
        }
        let n = determine_necessary_size_tree(src.len());
        let mut dst = vec![T::maximal(); n];
        for (i, value) in src.iter().enumerate() {
            dst[n / 2 + i] = *value;
        }
        for i in (1..n / 2).rev() {
            dst[i] = Ord::min(dst[2 * i], dst[2 * i + 1]);
        }
        RmqMin { data: dst, len: src.len() }
    }

    /// Returns the minimal on the interval l to r
    ///```
    /// use librualg::segment_tree::{Rsq, RmqMin};
    ///
    /// let arr = [1, 2, 3, 4, 5];
    /// let tree = RmqMin::new(&arr);
    ///
    /// assert_eq!(tree.query(0, 4).unwrap(), 1);
    /// assert_eq!(tree.query(1, 4).unwrap(), 2);
    /// assert_eq!(tree.query(4, 1).unwrap(), 2);
    /// assert_eq!(tree.query(3, 1).unwrap(), 2);
    /// assert_eq!(tree.query(3, 11), None);
    /// ```
    pub fn query(&self, l: usize, r: usize) -> Option<T> {
        if self.data.is_empty() || l >= self.len || r >= self.len {
            return None;
        }
        let mut l = l + self.data.len() / 2;
        let mut r = r + self.data.len() / 2;
        if l > r {
            swap(&mut l, &mut r);
        }
        let mut res = T::maximal();
        while l <= r {
            if l % 2 != 0 {
                res = min(res, self.data[l]);
            }
            l = (l + 1) >> 1;
            if r % 2 == 0 {
                res = min(res, self.data[r]);
            }
            r = (r - 1 ) >> 1;
        }
        Some(res)
    }

    /// Update value by index
    ///```
    /// use librualg::segment_tree::{RmqMin};
    /// let arr = [1, 2, 3, 4, 5];
    /// let mut tree = RmqMin::new(&arr);
    ///
    /// assert_eq!(tree.query(0, 4).unwrap(), 1);
    /// tree.update(0, 7);
    /// assert_eq!(tree.query(0, 4).unwrap(), 2);
    /// ```
    pub fn update(&mut self, mut idx: usize, value: T) {
        if !self.data.is_empty() && idx < self.len {
            idx += self.data.len() / 2;
            self.data[idx] = value;
            while idx >= 1 {
                if idx % 2 == 0 {
                    self.data[idx / 2] = min(self.data[idx], self.data[idx + 1]);
                } else {
                    self.data[idx / 2] = min(self.data[idx], self.data[idx - 1]);
                }
                idx /= 2;
            }
        }
    }
}

/// Range Maximum Query

pub struct RmqMax<T: Default + Clone + Copy + SegmentTreeMin + SegmentTreeMax + Ord > {
    data: Vec<T>,
    len: usize,
}

impl <T> RmqMax<T>  where T: Default + Clone + Copy + SegmentTreeMin + SegmentTreeMax + Ord {

    /// Build Segment Tree (RMQMax) from slice.
    ///```
    /// use librualg::segment_tree::{RmqMin, SegmentTreeMin, SegmentTreeMax, RmqMax};
    /// use std::prelude::v1::*;
    ///
    /// let arr = [1, 2, 3, 4, 5];
    /// let tree = RmqMax::new(&arr);
    /// ```
    pub fn new(src: &[T]) -> Self {
        if src.is_empty() {
            return RmqMax { data: vec![], len: src.len() };
        }
        let n = determine_necessary_size_tree(src.len());
        let mut dst = vec![T::minimal(); n];
        for (i, value) in src.iter().enumerate() {
            dst[n / 2 + i] = *value;
        }
        for i in (1..n / 2).rev() {
            dst[i] = Ord::max(dst[2 * i], dst[2 * i + 1]);
        }
        RmqMax { data: dst, len: src.len() }
    }

    /// Returns the maximum on the interval l to r
    ///```
    /// use librualg::segment_tree::{Rsq, RmqMax};
    ///
    /// let arr = [1, 2, 3, 4, 5];
    /// let tree = RmqMax::new(&arr);
    ///
    /// assert_eq!(tree.query(0, 4).unwrap(), 5);
    /// assert_eq!(tree.query(1, 4).unwrap(), 5);
    /// assert_eq!(tree.query(4, 1).unwrap(), 5);
    /// assert_eq!(tree.query(3, 1).unwrap(), 4);
    /// assert_eq!(tree.query(3, 11), None);
    /// ```
    pub fn query(&self, l: usize, r: usize) -> Option<T> {
        if self.data.is_empty() || l >= self.len || r >= self.len {
            return None;
        }
        let mut l = l + self.data.len() / 2;
        let mut r = r + self.data.len() / 2;
        if l > r {
            swap(&mut l, &mut r);
        }
        let mut res = T::minimal();
        while l <= r {
            if l % 2 != 0 {
                res = max(res, self.data[l]);
            }
            l = (l + 1) >> 1;
            if r % 2 == 0 {
                res = max(res, self.data[r]);
            }
            r = (r - 1 ) >> 1;
        }
        Some(res)
    }

    /// Update value by index
    ///```
    /// use librualg::segment_tree::{RmqMax};
    /// let arr = [1, 2, 3, 4, 5];
    /// let mut tree = RmqMax::new(&arr);
    ///
    /// assert_eq!(tree.query(0, 4).unwrap(), 5);
    /// tree.update(0, 7);
    /// assert_eq!(tree.query(0, 4).unwrap(), 7);
    /// ```
    pub fn update(&mut self, mut idx: usize, value: T) {
        if !self.data.is_empty() && idx < self.len {
            idx += self.data.len() / 2;
            self.data[idx] = value;
            while idx >= 1 {
                if idx % 2 == 0 {
                    self.data[idx / 2] = max(self.data[idx], self.data[idx + 1]);
                } else {
                    self.data[idx / 2] = max(self.data[idx], self.data[idx - 1]);
                }
                idx /= 2;
            }
        }
    }
}

pub trait SegmentTreeMin {
    fn minimal() -> Self;
}

pub trait SegmentTreeMax {
    fn maximal() -> Self;
}

fn determine_necessary_size_tree(count: usize) -> usize {
    let mut n = 1usize;
    while n < count {
        n <<= 1;
    }
    n << 1
}

impl SegmentTreeMin for i32 {
    fn minimal() -> i32 {
        i32::MIN
    }
}

impl SegmentTreeMax for i32 {
    fn maximal() -> i32 {
        i32::MAX
    }
}

#[test]
fn test_rsq() {
    let arr = [1, 2, 3, 4, 5];
    let tree = Rsq::new(&arr);

    assert_eq!(tree.query(4, 4).unwrap(), 5);

    assert_eq!(tree.query(0, 4).unwrap(), 15);
    assert_eq!(tree.query(1, 4).unwrap(), 14);
    assert_eq!(tree.query(4, 1).unwrap(), 14);
    assert_eq!(tree.query(3, 1).unwrap(), 9);
    assert_eq!(tree.query(4, 0).unwrap(), 15);

    assert_eq!(tree.query(3, 11), None);
    assert_eq!(tree.query(3, 5), None);

    let tree = Rsq::<i32>::new(&vec![]);
    assert_eq!(tree.query(0, 0), None);
}

#[test]
fn test_rsq_update() {
    let arr = [1, 2, 3, 4, 5];
    let mut tree = Rsq::new(&arr);

    assert_eq!(tree.query(0, 4).unwrap(), 15);
    tree.update(1, 7);
    assert_eq!(tree.query(0, 4).unwrap(), 20);
    tree.update(12, 7);
    assert_eq!(tree.query(0, 4).unwrap(), 20);
    tree.update(4, 0);
    assert_eq!(tree.query(0, 4).unwrap(), 15);
    tree.update(0, 3);
    assert_eq!(tree.query(0, 0).unwrap(), 3);
}

#[test]
fn test_rmq_max() {
    let arr = [1, 2, 3, 4, 5];
    let tree = RmqMax::new(&arr);

    assert_eq!(tree.query(0, 4).unwrap(), 5);
    assert_eq!(tree.query(1, 4).unwrap(), 5);
    assert_eq!(tree.query(4, 1).unwrap(), 5);
    assert_eq!(tree.query(3, 1).unwrap(), 4);
    assert_eq!(tree.query(2, 2).unwrap(), 3);
}

#[test]
fn test_rmq_max_update() {
    let arr = [1, 2, 3, 4, 5];
    let mut tree = RmqMax::new(&arr);
    assert_eq!(tree.query(0, 4).unwrap(), 5);
    tree.update(0, 7);
    assert_eq!(tree.query(0, 4).unwrap(), 7);
}

#[test]
fn test_rmq_min() {
    let arr = [1, 2, 3, 4, 5];
    let tree = RmqMin::new(&arr);

    assert_eq!(tree.query(0, 4).unwrap(), 1);
    assert_eq!(tree.query(1, 4).unwrap(), 2);
    assert_eq!(tree.query(4, 1).unwrap(), 2);
    assert_eq!(tree.query(3, 1).unwrap(), 2);
    assert_eq!(tree.query(2, 2).unwrap(), 3);
}

#[test]
fn test_rmq_min_update() {
    let arr = [1, 2, 3, 4, 5];
    let mut tree = RmqMin::new(&arr);
    assert_eq!(tree.query(0, 4).unwrap(), 1);
    tree.update(0, 7);
    assert_eq!(tree.query(0, 4).unwrap(), 2);
}