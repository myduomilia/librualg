use librualg::sheduling::{Job, johnson_algorithm};

#[test]
fn test(){
    let jobs = vec![
        Job::new(1, 4, 5),
        Job::new(2, 4, 1),
        Job::new(3, 30, 4),
        Job::new(4, 6, 30),
        Job::new(5, 2, 3)
    ];

    let (jobs, total_time) = johnson_algorithm(&jobs);
    assert_eq!(vec![5, 1, 4, 3, 2], jobs.iter().map(|value| value.get_id()).collect::<Vec<u32>>());
    assert_eq!(47, total_time);
}