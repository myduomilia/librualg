use librualg::dsu::{DSURef, DSU, DSUNum};

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

#[test]
fn test_dsu_num() {
    let mut dsu = DSUNum::new(10);
    for i in 1..=10 {
        dsu.make_set(i);
    }
    dsu.union_sets(1, 2);
    dsu.union_sets(2, 3);
    dsu.union_sets(2, 7);

    assert_eq!(dsu.find_set(2), dsu.find_set(7));
    assert_ne!(dsu.find_set(2), dsu.find_set(8));
    assert_eq!(dsu.find_set(1), dsu.find_set(3));
    assert_ne!(dsu.find_set(1), dsu.find_set(9));
}