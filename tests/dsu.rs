use librualg::dsu::DSURef;

#[test]
fn test_dsu() {
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