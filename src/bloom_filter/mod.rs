/// Bloom Filter
///```
/// use librualg::bloom_filter::BloomFilter;;
///
/// let mut bloom_filter = BloomFilter::build(8 * 1024 * 1024, 1000000, 2);
/// bloom_filter.insert("google");
/// bloom_filter.insert("facebook");
/// bloom_filter.insert("yandex");
///
/// assert_eq!(bloom_filter.contains("google"), true);
/// assert_eq!(bloom_filter.contains("facebook"), true);
/// assert_eq!(bloom_filter.contains("yandex"), true);
/// assert_eq!(bloom_filter.contains("microsoft"), false);
/// assert_eq!(bloom_filter.contains("oracle"), false);
/// assert_eq!(bloom_filter.contains("redhat"), false);
/// ```
pub struct BloomFilter {
    data: Vec<u8>,
    hash_count: usize,
    false_positive_probability: f64
}

impl BloomFilter {
    /// Build Bloom Filter
    /// # Arguments
    /// * `n` - bit array size (number of bytes)
    /// * `m` - number of inserted elements
    /// * `k` - number hash functions

    pub fn build(n: usize, m: usize, k: usize) -> Self {
        BloomFilter {
            data: vec![0; n],
            hash_count: k as usize,
            false_positive_probability: (1.0 - f64::exp(-1.0 * k as f64 * m as f64 / (8.0 * n as f64))).powf(k as f64)
        }
    }

    pub fn insert(&mut self, key: &str) {
        for idx in 0..self.hash_count {
            let hash = self.hash(&format!("{}{}", key, idx));
            let mask = 128 >> (hash % 8);
            self.data[hash / 8] |= mask
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        for idx in 0..self.hash_count {
            let hash = self.hash(&format!("{}{}", key, idx));
            let mask = 128 >> (hash % 8);
            if self.data[hash / 8] & mask != mask {
                return false;
            }
        }
        true
    }

    fn hash(&self, s: &str) -> usize {
        let mut hash = 5381_usize;
        for x in s.as_bytes() {
            hash = hash.wrapping_shl(5).wrapping_add(hash).wrapping_add(*x as usize);
        }
        hash % (8 * self.data.len())
    }

    pub fn get_false_positive_probability(&self) -> f64 {
        self.false_positive_probability
    }
}



#[test]
fn test_bloom_filter() {
    let mut bloom_filter = BloomFilter::build(8 * 1024 * 1024, 1000000, 2);
    // println!("{:.4}", bloom_filter.get_false_positive_probability());

    bloom_filter.insert("google");
    bloom_filter.insert("facebook");
    bloom_filter.insert("yandex");

    assert_eq!(bloom_filter.contains("google"), true);
    assert_eq!(bloom_filter.contains("facebook"), true);
    assert_eq!(bloom_filter.contains("yandex"), true);
    assert_eq!(bloom_filter.contains("microsoft"), false);
    assert_eq!(bloom_filter.contains("oracle"), false);
    assert_eq!(bloom_filter.contains("redhat"), false);
}