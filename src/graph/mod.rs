use std::collections::{BTreeSet, VecDeque, BTreeMap, BinaryHeap};
use std::option::Option::Some;
use std::cmp::Ordering;

enum Color {
    GREY = 1,
    BLACK = 2
}

pub struct VertexProperties<Indent> where Indent: Eq + Ord + Clone {
    parent: Option<Indent>,
    time_in: Option<u32>,
    time_out: Option<u32>
}

#[derive(Clone)]
struct Edge <Indent> where Indent: Eq + Ord + Clone {
    to: Indent,
    weight: f32,
}

pub struct Graph <Indent> where Indent: Eq + Ord + Clone {
    adj: BTreeMap<Indent, Vec<Edge<Indent>>>,
}

impl <Indent> Graph <Indent> where Indent: Eq + Ord + Clone {
    pub fn new() -> Self {
        let graph = Graph { adj: BTreeMap::new() };
        graph
    }

    /// BFS (Breadth-First Search) algorithm.
    /// Returns an ancestor vector along the graph traversal path
    ///```
    /// use librualg::graph::Graph;
    ///
    /// let mut graph = Graph::new();
    /// graph.add_oriented_edge(1, 2, 0.0);
    /// graph.add_oriented_edge(2, 3, 0.0);
    /// graph.add_oriented_edge(2, 4, 0.0);
    /// graph.add_oriented_edge(2, 5, 0.0);
    /// graph.add_oriented_edge(4, 8, 0.0);
    /// graph.add_oriented_edge(8, 17, 0.0);
    /// let parents = graph.bfs(1);
    /// assert_eq!(graph.search_path(5, &parents).unwrap(), vec![1, 2, 5]);
    /// assert_eq!(graph.search_path(17, &parents).unwrap(), vec![1, 2, 4, 8, 17]);
    /// ```

    pub fn bfs(&self, from: Indent) -> BTreeMap::<Indent, VertexProperties<Indent>> {
        let mut queue = VecDeque::new();
        let mut parents = BTreeMap::<Indent, VertexProperties<Indent>>::new();
        let mut visits = BTreeSet::new();


        if self.adj.get(&from).is_some() {
            queue.push_back(&from);
            visits.insert(&from);
            parents.insert(from.clone(), VertexProperties {parent: None, time_in: None, time_out: None});
            while let Some(vertex) = queue.pop_front(){
                if self.adj.get(&vertex).is_some() {
                    for edge in self.adj.get(&vertex).unwrap().iter() {
                        if !visits.contains(&edge.to) {
                            parents.insert(edge.to.clone(), VertexProperties {parent: Some(vertex.clone()), time_in: None, time_out: None});
                            queue.push_back(&edge.to);
                            visits.insert(&edge.to);
                        }
                    }
                }
            }
        }
        parents
    }

    fn _dfs(&self, from: Indent, timer: &mut u32,  parents: &mut BTreeMap::<Indent, VertexProperties<Indent>>, colors: &mut BTreeMap::<Indent, Color>) {
        *timer += 1;
        parents.insert(from.clone(), VertexProperties{parent: None, time_in: Some(*timer), time_out: None});
        colors.insert(from.clone(), Color::GREY);
        if self.adj.get(&from).is_some() {
            for edge in self.adj.get(&from).unwrap().iter() {
                if colors.get(&edge.to).is_none() {
                    parents.insert(edge.to.clone(), VertexProperties{parent: Some(from.clone()), time_in: None, time_out: None});
                    self._dfs(edge.to.clone(), timer, parents, colors);
                }
            }
        }
        *(colors.get_mut(&from).unwrap()) = Color::BLACK;
        *timer += 1;
        parents.get_mut(&from).unwrap().time_out = Some(*timer);
    }

    /// DFS (Depth-First Search) algorithm.
    /// Returns an ancestor vector along the graph traversal path
    ///```
    /// use librualg::graph::Graph;
    ///
    /// let mut graph = Graph::new();
    /// graph.add_oriented_edge(1, 2, 0.0);
    /// graph.add_oriented_edge(2, 3, 0.0);
    /// graph.add_oriented_edge(3, 5, 0.0);
    ///
    /// let res = graph.bfs(1);
    /// assert_eq!(graph.search_path(5, &res).unwrap(), vec![1, 2, 3, 5]);
    /// ```

    pub fn dfs(&self, from: Indent) -> BTreeMap::<Indent, VertexProperties<Indent>> {
        let mut parents = BTreeMap::<Indent, VertexProperties<Indent>>::new();
        let mut colors = BTreeMap::<Indent, Color>::new();
        let mut timer = 0;
        self._dfs(from, &mut timer, &mut parents, &mut colors);
        parents
    }

    /// Dijkstra algorithm.
    /// Returns an ancestor vector along the graph traversal path and distances to the other vertexs
    ///```
    /// use librualg::graph::Graph;
    ///
    /// let mut graph = Graph::new();
    /// graph.add_oriented_edge(1, 2, 2.0);
    /// graph.add_oriented_edge(2, 3, 5.0);
    /// graph.add_oriented_edge(3, 5, 7.0);
    /// graph.add_oriented_edge(1, 5, 19.0);
    ///
    /// let (parents, distances) = graph.dijkstra(1);
    /// assert_eq!(graph.search_path(5, &parents).unwrap(), vec![1, 2, 3, 5]);
    /// assert_eq!(graph.search_path(3, &parents).unwrap(), vec![1, 2, 3]);
    /// assert_eq!(*distances.get(&5).unwrap(), 14.0);
    /// ```


    pub fn dijkstra(&self, from: Indent) -> (BTreeMap::<Indent, VertexProperties<Indent>>, BTreeMap::<Indent, f32>) {
        let mut parents = BTreeMap::<Indent, VertexProperties<Indent>>::new();
        let mut visits = BTreeSet::<Indent>::new();
        let mut distances = BTreeMap::<Indent, f32>::new();

        struct D<Indent> {
            node: Indent,
            dist: f32,
        }

        impl <Indent> std::cmp::PartialEq for D<Indent> {
            fn eq(&self, other: &D<Indent>) -> bool {
                self.dist == other.dist
            }
        }

        impl <Indent> Eq for D<Indent> {}

        impl <Indent> std::cmp::Ord for D<Indent> {
            fn cmp(&self, other: &Self) -> Ordering {
                other.dist.partial_cmp(&self.dist).unwrap()
            }
        }

        impl <Indent> std::cmp::PartialOrd for D <Indent> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(other.dist.partial_cmp(&self.dist).unwrap())
            }
        }


        let mut heap = BinaryHeap::<D<Indent>>::new();
        distances.insert(from.clone(), 0.0);
        heap.push(D{ node: from, dist: 0.0});
        while !heap.is_empty() {
            let d = heap.pop().unwrap();
            visits.insert(d.node.clone());
            if self.adj.get(&d.node).is_some() {
                for edge in self.adj.get(&d.node).unwrap() {
                    if !visits.contains(&edge.to){
                        if &edge.weight + &d.dist < *distances.get(&edge.to).unwrap_or(&std::f32::MAX) {
                            parents.insert(edge.to.clone(), VertexProperties{parent: Some(d.node.clone()), time_in: None, time_out: None});
                            distances.insert(edge.to.clone(), edge.weight + d.dist);
                            heap.push(D{node: edge.to.clone(), dist: *distances.get(&edge.to).unwrap()});
                        }
                    }
                }
            }
        }
        (parents, distances)
    }

    /// Adds a new oriented edge to the graph
    pub fn add_oriented_edge(&mut self, from: Indent, to: Indent, weight: f32) {
        match self.adj.get_mut(&from) {
            Some(ref mut vertex) => {
                vertex.push(Edge { to, weight});
            }
            None => {
                let seq = vec![Edge { to, weight}];
                self.adj.insert(from, seq);
            }
        }
    }

    /// Returns the path in the graph between two vertices based on the ancestor vector
    /// Returns None if the path does not exist
    /// ```
    /// use librualg::graph::Graph;
    ///
    /// let mut graph = Graph::new();
    /// graph.add_oriented_edge(1, 2, 0.0);
    /// graph.add_oriented_edge(2, 3, 0.0);
    /// graph.add_oriented_edge(2, 4, 0.0);
    /// graph.add_oriented_edge(2, 5, 0.0);
    /// graph.add_oriented_edge(4, 8, 0.0);
    /// graph.add_oriented_edge(8, 17, 0.0);
    /// let parents = graph.bfs(1);
    /// assert_eq!(graph.search_path(5, &parents).unwrap(), vec![1, 2, 5]);
    /// assert_eq!(graph.search_path(17, &parents).unwrap(), vec![1, 2, 4, 8, 17]);
    ///
    /// let parents = graph.bfs(101);
    /// assert_eq!(graph.search_path(101, &parents), None);
    /// ```

    pub fn search_path(&self, mut target: Indent, parents: &BTreeMap<Indent, VertexProperties<Indent>>) -> Option<Vec<Indent>> {
        if !parents.contains_key(&target) {
            return None;
        }
        let mut path = vec![target.clone()];
        while let Some(next) = parents.get(&target) {
            if next.parent.is_none() {
                break;
            }
            path.push(next.parent.clone().unwrap());
            target = next.parent.clone().unwrap();
        }
        path.reverse();
        Some(path)
    }
}

#[test]
fn test_bfs() {
    let mut graph = Graph::<usize>::new();
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
fn test_bfs_with_string() {
    let mut graph = Graph::<String>::new();
    graph.add_oriented_edge("1".to_string(), "2".to_string(), 0.0);
    graph.add_oriented_edge("2".to_string(), "3".to_string(), 0.0);
    graph.add_oriented_edge("2".to_string(), "4".to_string(), 0.0);
    graph.add_oriented_edge("2".to_string(), "5".to_string(), 0.0);
    graph.add_oriented_edge("4".to_string(), "8".to_string(), 0.0);
    graph.add_oriented_edge("8".to_string(), "17".to_string(), 0.0);
    let parents = graph.bfs("1".to_string());
    assert_eq!(graph.search_path("5".to_string(), &parents).unwrap(), vec!["1".to_string(), "2".to_string(), "5".to_string()]);
}

#[test]
fn test_dfs() {
    let mut graph = Graph::new();
    graph.add_oriented_edge(1, 2, 0.0);
    graph.add_oriented_edge(2, 3, 0.0);
    graph.add_oriented_edge(3, 5, 0.0);

    let res = graph.bfs(1);
    assert_eq!(graph.search_path(5, &res).unwrap(), vec![1, 2, 3, 5]);
}

#[test]
fn test_dijkstra() {
    let mut graph = Graph::new();
    graph.add_oriented_edge(1, 2, 2.0);
    graph.add_oriented_edge(2, 3, 5.0);
    graph.add_oriented_edge(3, 5, 7.0);
    graph.add_oriented_edge(1, 5, 19.0);

    let (parents, distances) = graph.dijkstra(1);
    assert_eq!(graph.search_path(5, &parents).unwrap(), vec![1, 2, 3, 5]);
    assert_eq!(graph.search_path(3, &parents).unwrap(), vec![1, 2, 3]);
    assert_eq!(*distances.get(&5).unwrap(), 14.0);


    let mut graph = Graph::new();
    graph.add_oriented_edge(1, 2, 2.0);
    graph.add_oriented_edge(2, 3, 5.0);
    graph.add_oriented_edge(3, 1, 7.0);

    let (parents, distances) = graph.dijkstra(1);
    assert_eq!(graph.search_path(3, &parents).unwrap(), vec![1, 2, 3]);
    assert_eq!(*distances.get(&3).unwrap(), 7.0);
}