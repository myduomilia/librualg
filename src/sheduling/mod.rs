use std::cmp::{Ordering, min, max};

/// Johnson's Algorithm For Scheduling
/// There are n parts and two machines. Each part must first be machined on the first machine, then on the second one.
/// The i-th part is machined on the first machine in a_i time, and on the second machine in b_i time.
/// Each machine can handle only one part at any given time.
/// You need to create an order of feeding parts to the machines, so that the total machining time of all parts would be minimal.
///```
/// use librualg::sheduling::{johnson_algorithm, Job};
///
/// let jobs = vec![
///     Job::new(1, 4, 5),
///     Job::new(2, 4, 1),
///     Job::new(3, 30, 4),
///     Job::new(4, 6, 30),
///     Job::new(5, 2, 3)
/// ];
///
/// let (jobs, total_time) = johnson_algorithm(&jobs);
/// assert_eq!(vec![5, 1, 4, 3, 2], jobs.iter().map(|value| value.get_id()).collect::<Vec<u32>>());
/// assert_eq!(47, total_time);
/// ```

#[derive(Copy, Clone, Debug)]
pub struct Job {
    id: u32,
    first_time: u32,
    second_time: u32
}

impl Job {
    pub fn new(id: u32, first_time: u32, second_time: u32) -> Self {
        Job{
            id,
            first_time,
            second_time
        }
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
}

pub fn johnson_algorithm(jobs: &[Job]) -> (Vec<Job>, u32) {
    let mut res = jobs.to_vec();
    let mut total = 0;
    res.sort_by(|a, b| {
        let mm = min(min(a.first_time, a.second_time), min(b.first_time, b.second_time));
        if mm == a.first_time || mm == b.second_time {
            return Ordering::Less;
        }
        return Ordering::Greater
    });
    let mut t = 0;
    for job in &res {
        t += job.first_time;
        total = max(t, total) + job.second_time;
    }
    (res, total)
}

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
    assert_eq!(vec![5, 1, 4, 3, 2], jobs.iter().map(|value| value.id).collect::<Vec<u32>>());
    assert_eq!(47, total_time);
}