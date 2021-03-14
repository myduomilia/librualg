use std::cmp::Ordering;
use std::collections::{BTreeSet, VecDeque, BTreeMap};
use std::option::Option::Some;

#[derive(Clone)]
struct Edge <Indent, W> where Indent: Eq + Ord + Clone {
    to: Indent,
    weight: W,
}

impl <Indent, W> std::cmp::PartialEq for Edge <Indent, W> where Indent: Eq + Ord + Clone, W: Eq + Ord {
    fn eq(&self, other: &Edge<Indent, W>) -> bool {
        self.to == other.to && self.weight == other.weight
    }
}

impl <Indent, W> Ord for Edge <Indent, W> where Indent: Eq + Ord + Clone, W: Eq + Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to.cmp(&other.to)
    }
}

impl <Indent, W> PartialOrd for Edge <Indent, W> where Indent: Eq + Ord + Clone, W: Eq + Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <Indent, W> Eq for Edge <Indent, W> where Indent: Eq + Ord + Clone, W: Eq + Ord {}

pub struct Graph <Indent, W> where Indent: Eq + Ord +Clone {
    adj: BTreeMap<Indent, BTreeSet<Edge<Indent, W>>>,
}

impl <Indent, W> Graph <Indent, W> where Indent: Eq + Ord + Clone, W: Eq + Ord {
    pub fn new() -> Self {
        let graph = Graph { adj: BTreeMap::new() };
        graph
    }

    /// BFS (Breadth-First Search) algorithm.
    /// Return an ancestor vector along the graph traversal path
    ///```
    /// use librualg::graph::Graph;
    ///
    /// let mut graph = Graph::new();
    /// graph.add_oriented_edge(1, 2, 0);
    /// graph.add_oriented_edge(2, 3, 0);
    /// graph.add_oriented_edge(2, 4, 0);
    /// graph.add_oriented_edge(2, 5, 0);
    /// graph.add_oriented_edge(4, 8, 0);
    /// graph.add_oriented_edge(8, 17, 0);
    /// let parents = graph.bfs(1);
    /// assert_eq!(graph.search_path(5, &parents).unwrap(), vec![1, 2, 5]);
    /// assert_eq!(graph.search_path(17, &parents).unwrap(), vec![1, 2, 4, 8, 17]);
    /// ```

    pub fn bfs(&self, from: Indent) -> BTreeMap::<Indent, Option<Indent>> {
        let mut queue = VecDeque::new();
        let mut parents = BTreeMap::<Indent, Option<Indent>>::new();
        let mut visits = BTreeSet::new();


        if self.adj.get(&from).is_some() {
            queue.push_back(&from);
            visits.insert(&from);
            parents.insert(from.clone(), None);
            while let Some(vertex) = queue.pop_front(){
                if self.adj.get(&vertex).is_some() {
                    for edge in self.adj.get(&vertex).unwrap().iter() {
                        if !visits.contains(&edge.to) {
                            parents.insert(edge.to.clone(), Some(vertex.clone()));
                            queue.push_back(&edge.to);
                            visits.insert(&edge.to);
                        }
                    }
                }
            }
        }
        parents
    }

    /// Add a new edge to the graph
    pub fn add_oriented_edge(&mut self, from: Indent, to: Indent, weight: W) {
        match self.adj.get_mut(&from) {
            Some(ref mut vertex) => {
                vertex.insert(Edge { to, weight});
            }
            None => {
                let mut set = BTreeSet::new();
                set.insert(Edge { to, weight});
                self.adj.insert(from, set);
            }
        }
    }

    /// Return the path in the graph between two vertices based on the ancestor vector
    /// Return None if the path does not exist
    /// ```
    /// use librualg::graph::Graph;
    ///
    /// let mut graph = Graph::new();
    /// graph.add_oriented_edge(1, 2, 0);
    /// graph.add_oriented_edge(2, 3, 0);
    /// graph.add_oriented_edge(2, 4, 0);
    /// graph.add_oriented_edge(2, 5, 0);
    /// graph.add_oriented_edge(4, 8, 0);
    /// graph.add_oriented_edge(8, 17, 0);
    /// let parents = graph.bfs(1);
    /// assert_eq!(graph.search_path(5, &parents).unwrap(), vec![1, 2, 5]);
    /// assert_eq!(graph.search_path(17, &parents).unwrap(), vec![1, 2, 4, 8, 17]);
    ///
    /// let parents = graph.bfs(101);
    /// assert_eq!(graph.search_path(101, &parents), None);
    /// ```

    pub fn search_path(&self, mut target: Indent, parents: &BTreeMap<Indent, Option<Indent>>) -> Option<Vec<Indent>> {
        if !parents.contains_key(&target) {
            return None;
        }
        let mut path = vec![target.clone()];
        while let Some(parent) = parents.get(&target) {
            if parent.is_none() {
                break;
            }
            path.push(parent.clone().unwrap());
            target = parent.clone().unwrap();
        }
        path.reverse();
        Some(path)
    }
}

#[test]
fn test_bfs() {
    let mut graph = Graph::<usize, u8>::new();
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

#[test]
fn test_bfs_with_string() {
    let mut graph = Graph::<String, u8>::new();
    graph.add_oriented_edge("1".to_string(), "2".to_string(), 0);
    graph.add_oriented_edge("2".to_string(), "3".to_string(), 0);
    graph.add_oriented_edge("2".to_string(), "4".to_string(), 0);
    graph.add_oriented_edge("2".to_string(), "5".to_string(), 0);
    graph.add_oriented_edge("4".to_string(), "8".to_string(), 0);
    graph.add_oriented_edge("8".to_string(), "17".to_string(), 0);
    let parents = graph.bfs("1".to_string());
    assert_eq!(graph.search_path("5".to_string(), &parents).unwrap(), vec!["1".to_string(), "2".to_string(), "5".to_string()]);
}