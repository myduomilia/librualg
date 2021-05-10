extern crate librualg;

use librualg::graph::Graph;

#[test]
fn test_bfs() {
    let mut graph = Graph::new();
    graph.add_oriented_edge(1, 2, 0.0);
    graph.add_oriented_edge(2, 3, 0.0);
    graph.add_oriented_edge(2, 4, 0.0);
    graph.add_oriented_edge(2, 5, 0.0);
    graph.add_oriented_edge(4, 8, 0.0);
    graph.add_oriented_edge(8, 17, 0.0);
    let parents = graph.bfs(1);
    assert_eq!(graph.search_path(5, &parents).unwrap(), vec![1, 2, 5]);
    assert_eq!(graph.search_path(17, &parents).unwrap(), vec![1, 2, 4, 8, 17]);

    graph.add_oriented_edge(17, 1, 0.0);
    let parents = graph.bfs(1);
    assert_eq!(graph.search_path(5, &parents).unwrap(), vec![1, 2, 5]);
    assert_eq!(graph.search_path(17, &parents).unwrap(), vec![1, 2, 4, 8, 17]);

    let parents = graph.bfs(101);
    assert_eq!(graph.search_path(101, &parents), None);
}

#[test]
fn test_connected_components() {
    let mut graph = Graph::new();
    graph.add_oriented_edge(1, 2, 0.0);
    graph.add_oriented_edge(2, 3, 0.0);
    graph.add_oriented_edge(3, 4, 0.0);

    graph.add_oriented_edge(5, 6, 0.0);
    graph.add_oriented_edge(6, 7, 0.0);

    graph.add_oriented_edge(8, 9, 0.0);
    graph.add_oriented_edge(9, 10, 0.0);
    graph.add_oriented_edge(10, 11, 0.0);

    let components = graph.connected_components();
    assert_eq!(components[0], [1, 2, 3, 4]);
    assert_eq!(components[1], [5, 6, 7]);
    assert_eq!(components[2], [8, 9, 10, 11]);
}