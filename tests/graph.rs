extern crate librualg;

use librualg::graph::{Graph, GraphNum};

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
    assert_eq!(components.len(), 3);
}

#[test]
fn test_strongly_connected_components() {
    let mut graph = Graph::new();
    graph.add_oriented_edge("a", "b", 0.0);
    graph.add_oriented_edge("b", "f", 0.0);
    graph.add_oriented_edge("e", "a", 0.0);
    graph.add_oriented_edge("b", "e", 0.0);
    graph.add_oriented_edge("e", "f", 0.0);

    graph.add_oriented_edge("b", "c", 0.0);
    graph.add_oriented_edge("f", "g", 0.0);
    graph.add_oriented_edge("g", "f", 0.0);
    graph.add_oriented_edge("c", "g", 0.0);

    graph.add_oriented_edge("c", "d", 0.0);
    graph.add_oriented_edge("d", "c", 0.0);
    graph.add_oriented_edge("d", "h", 0.0);
    graph.add_oriented_edge("h", "d", 0.0);
    graph.add_oriented_edge("h", "g", 0.0);

    let components = graph.strongly_connected_components();
    assert_eq!(components[0], ["a", "b", "e"]);
    assert_eq!(components[1], ["c", "d", "h"]);
    assert_eq!(components[2], ["f", "g"]);
}

#[test]
fn topology_sort() {
    let mut graph = Graph::new();
    graph.add_oriented_edge("a", "b", 0.0);
    graph.add_oriented_edge("a", "c", 0.0);
    graph.add_oriented_edge("a", "e", 0.0);
    graph.add_oriented_edge("a", "d", 0.0);
    graph.add_oriented_edge("b", "d", 0.0);
    graph.add_oriented_edge("c", "d", 0.0);
    graph.add_oriented_edge("c", "e", 0.0);

    assert_eq!(graph.topological_sort(), vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn test_kruskal() {
    let mut graph = Graph::new();
    graph.add_oriented_edge('A', 'B', 7.0);
    graph.add_oriented_edge('B', 'A', 7.0);
    graph.add_oriented_edge('A', 'D', 5.0);
    graph.add_oriented_edge('D', 'A', 5.0);
    graph.add_oriented_edge('B', 'C', 8.0);
    graph.add_oriented_edge('C', 'B', 8.0);
    graph.add_oriented_edge('B', 'E', 7.0);
    graph.add_oriented_edge('E', 'B', 7.0);
    graph.add_oriented_edge('B', 'D', 9.0);
    graph.add_oriented_edge('D', 'B', 9.0);
    graph.add_oriented_edge('C', 'E', 5.0);
    graph.add_oriented_edge('E', 'C', 5.0);
    graph.add_oriented_edge('E', 'G', 9.0);
    graph.add_oriented_edge('G', 'E', 9.0);
    graph.add_oriented_edge('E', 'F', 8.0);
    graph.add_oriented_edge('F', 'E', 8.0);
    graph.add_oriented_edge('E', 'D', 15.0);
    graph.add_oriented_edge('D', 'E', 15.0);
    graph.add_oriented_edge('F', 'G', 11.0);
    graph.add_oriented_edge('G', 'F', 11.0);
    graph.add_oriented_edge('F', 'D', 6.0);
    graph.add_oriented_edge('D', 'F', 6.0);
    let tree = graph.kruskal();
    assert_eq!(vec!['A', 'B', 'E', 'G'], tree.search_path('G', &tree.bfs('A')).unwrap());
    assert_eq!(vec!['A', 'B', 'E', 'C'], tree.search_path('C', &tree.bfs('A')).unwrap());
}

#[test]
fn test_bfs_num() {
    let mut graph = GraphNum::new(20);
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    graph.add_vertex(4);
    graph.add_vertex(5);
    graph.add_vertex(8);
    graph.add_vertex(17);
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

    let parents = graph.bfs(11);
    assert_eq!(graph.search_path(11, &parents), None);
}

#[test]
fn test_dfs_num() {
    let mut graph = GraphNum::new(10);
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    graph.add_vertex(5);
    graph.add_oriented_edge(1, 2, 0.0);
    graph.add_oriented_edge(2, 3, 0.0);
    graph.add_oriented_edge(3, 5, 0.0);

    let res = graph.dfs(1);
    assert_eq!(graph.search_path(5, &res).unwrap(), vec![1, 2, 3, 5]);
}
#[test]
fn test_dijkstra_num() {
    let mut graph = GraphNum::new(10);

    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    graph.add_vertex(5);
    graph.add_vertex(7);

    graph.add_oriented_edge(1, 2, 2.0);
    graph.add_oriented_edge(2, 3, 5.0);
    graph.add_oriented_edge(3, 5, 7.0);
    graph.add_oriented_edge(1, 5, 19.0);

    let (parents, distances) = graph.dijkstra(1);
    assert_eq!(graph.search_path(5, &parents).unwrap(), vec![1, 2, 3, 5]);
    assert_eq!(graph.search_path(3, &parents).unwrap(), vec![1, 2, 3]);
    assert_eq!(distances[5].unwrap(), 14.0);
    assert_eq!(distances[7], None);
}

#[test]
fn test_connected_components_num() {
    let mut graph = GraphNum::new(20);
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    graph.add_vertex(4);
    graph.add_vertex(5);
    graph.add_vertex(6);
    graph.add_vertex(7);
    graph.add_vertex(8);
    graph.add_vertex(9);
    graph.add_vertex(10);
    graph.add_vertex(11);
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

#[test]
fn test_strongly_connected_components_num() {
    let mut graph = GraphNum::new(10);
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    graph.add_vertex(4);
    graph.add_vertex(5);
    graph.add_vertex(6);
    graph.add_vertex(7);
    graph.add_vertex(8);

    graph.add_oriented_edge(1, 2, 0.0);
    graph.add_oriented_edge(2, 6, 0.0);
    graph.add_oriented_edge(5, 1, 0.0);
    graph.add_oriented_edge(2, 5, 0.0);
    graph.add_oriented_edge(5, 6, 0.0);

    graph.add_oriented_edge(2, 3, 0.0);
    graph.add_oriented_edge(6, 7, 0.0);
    graph.add_oriented_edge(7, 6, 0.0);
    graph.add_oriented_edge(3, 7, 0.0);

    graph.add_oriented_edge(3, 4, 0.0);
    graph.add_oriented_edge(4, 3, 0.0);
    graph.add_oriented_edge(4, 8, 0.0);
    graph.add_oriented_edge(8, 4, 0.0);
    graph.add_oriented_edge(8, 7, 0.0);

    let components = graph.strongly_connected_components();
    assert_eq!(components[0], [1, 2, 5]);
    assert_eq!(components[1], [3, 4, 8]);
    assert_eq!(components[2], [6, 7]);
}