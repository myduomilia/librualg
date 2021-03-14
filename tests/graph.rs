extern crate librualg;

use librualg::graph::Graph;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_bfs() {
    let mut graph = Graph::new();
    graph.add_oriented_edge(1, 2, 0);
    graph.add_oriented_edge(2, 3, 0);
    graph.add_oriented_edge(2, 4, 0);
    graph.add_oriented_edge(2, 5, 0);
    graph.add_oriented_edge(4, 8, 0);
    graph.add_oriented_edge(8, 17, 0);
    let parents = graph.bfs(1);
    assert_eq!(graph.search_path(5, &parents).unwrap(), vec![1, 2, 5]);
    assert_eq!(graph.search_path(17, &parents).unwrap(), vec![1, 2, 4, 8, 17]);

    graph.add_oriented_edge(17, 1, 0);
    let parents = graph.bfs(1);
    assert_eq!(graph.search_path(5, &parents).unwrap(), vec![1, 2, 5]);
    assert_eq!(graph.search_path(17, &parents).unwrap(), vec![1, 2, 4, 8, 17]);

    let parents = graph.bfs(101);
    assert_eq!(graph.search_path(101, &parents), None);
}