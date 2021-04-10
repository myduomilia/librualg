use librualg::bloom_filter::BloomFilter;

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