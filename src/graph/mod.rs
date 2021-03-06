use std::collections::{BTreeSet, VecDeque, BTreeMap, BinaryHeap};
use std::option::Option::Some;
use std::cmp::{Ordering};
use crate::dsu::{DSU, DSUNum};

#[derive(Copy, Clone, PartialOrd, PartialEq)]
enum Color {
    White = 0,
    Grey = 1,
    Black = 2
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

pub struct VertexProperties<Indent> where Indent: Eq + Ord + Clone {
    parent: Option<Indent>,
    #[allow(dead_code)]
    color: Color,
    #[allow(dead_code)]
    time_in: Option<u32>,
    #[allow(dead_code)]
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

impl<Indent> Default for Graph<Indent> where Indent: Eq + Ord + Clone {
    fn default() -> Self {
        Graph { adj: BTreeMap::new() }
    }
}

impl <Indent> Graph <Indent> where Indent: Eq + Ord + Clone {
    pub fn new() -> Self {
        Graph::default()
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
        let mut visited = BTreeSet::new();

        if self.adj.get(&from).is_some() {
            queue.push_back(&from);
            visited.insert(&from);
            parents.insert(Clone::clone(&from), VertexProperties {parent: None, time_in: None, time_out: None, color: Color::White});
            while let Some(vertex) = queue.pop_front(){
                if self.adj.get(&vertex).is_some() {
                    for edge in self.adj.get(&vertex).unwrap().iter() {
                        if !visited.contains(&edge.to) {
                            parents.insert(edge.to.clone(), VertexProperties {parent: Some(vertex.clone()), time_in: None, time_out: None, color: Color::White});
                            queue.push_back(&edge.to);
                            visited.insert(&edge.to);
                        }
                    }
                }
            }
        }
        parents
    }

    fn _dfs(&self, from: Indent, timer: &mut u32,  parents: &mut BTreeMap::<Indent, VertexProperties<Indent>>, colors: &mut BTreeMap::<Indent, Color>) {
        *timer += 1;
        colors.insert(from.clone(), Color::Grey);
        if self.adj.get(&from).is_some() {
            for edge in self.adj.get(&from).unwrap().iter() {
                if colors.get(&edge.to).is_none() {
                    parents.insert(edge.to.clone(), VertexProperties{parent: Some(from.clone()), time_in: None, time_out: None, color: Color::White});
                    self._dfs(edge.to.clone(), timer, parents, colors);
                }
            }
        }
        *(colors.get_mut(&from).unwrap()) = Color::Black;
        *timer += 1;
        parents.get_mut(&from).unwrap().time_out = Some(*timer);
        parents.get_mut(&from).unwrap().color = Color::Black;
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
    /// let res = graph.dfs(1);
    /// assert_eq!(graph.search_path(5, &res).unwrap(), vec![1, 2, 3, 5]);
    /// ```

    pub fn dfs(&self, from: Indent) -> BTreeMap::<Indent, VertexProperties<Indent>> {
        let mut parents = BTreeMap::<Indent, VertexProperties<Indent>>::new();
        let mut colors = BTreeMap::<Indent, Color>::new();
        let mut timer = 0;
        parents.insert(from.clone(), VertexProperties{parent: None, time_in: Some(timer), time_out: None, color: Color::White});
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
        let mut visited = BTreeSet::<Indent>::new();
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
            visited.insert(d.node.clone());
            if self.adj.get(&d.node).is_some() {
                for edge in self.adj.get(&d.node).unwrap() {
                    if !visited.contains(&edge.to) && edge.weight + d.dist < *distances.get(&edge.to).unwrap_or(&f32::MAX) {
                        parents.insert(edge.to.clone(), VertexProperties{parent: Some(d.node.clone()), time_in: None, time_out: None, color: Color::White});
                        distances.insert(edge.to.clone(), edge.weight + d.dist);
                        heap.push(D{node: edge.to.clone(), dist: *distances.get(&edge.to).unwrap()});
                    }
                }
            }
        }
        (parents, distances)
    }

    /// Get connected components
    ///```
    /// use librualg::graph::Graph;
    ///
    /// let mut graph = Graph::new();
    /// graph.add_oriented_edge(1, 2, 0.0);
    /// graph.add_oriented_edge(2, 3, 0.0);
    /// graph.add_oriented_edge(3, 4, 0.0);
    ///
    /// graph.add_oriented_edge(5, 6, 0.0);
    /// graph.add_oriented_edge(6, 7, 0.0);
    ///
    /// graph.add_oriented_edge(8, 9, 0.0);
    /// graph.add_oriented_edge(9, 10, 0.0);
    /// graph.add_oriented_edge(10, 11, 0.0);
    ///
    /// let components = graph.connected_components();
    /// assert_eq!(components[0], [1, 2, 3, 4]);
    /// assert_eq!(components[1], [5, 6, 7]);
    /// assert_eq!(components[2], [8, 9, 10, 11]);
    /// ```

    pub fn connected_components(&self) -> Vec<Vec<Indent>> {
        let mut components = vec![];
        let mut visited = BTreeSet::new();
        for vertex in self.adj.keys() {
            if !visited.contains(vertex) {
                let mut queue = VecDeque::new();
                let mut vec = vec![];
                visited.insert(vertex);
                queue.push_back(vertex);
                while let Some(vertex) = queue.pop_front(){
                    vec.push(vertex.clone());
                    if self.adj.get(&vertex).is_some() {
                        for edge in self.adj.get(&vertex).unwrap().iter() {
                            if !visited.contains(&edge.to) {
                                queue.push_back(&edge.to);
                                visited.insert(&edge.to);
                            }
                        }
                    }
                }
                components.push(vec)
            }
        }
        components
    }

    /// Get strongly connected components
    /// ```
    /// use librualg::graph::Graph;
    ///
    /// let mut graph = Graph::new();
    /// graph.add_oriented_edge("a", "b", 0.0);
    /// graph.add_oriented_edge("b", "f", 0.0);
    /// graph.add_oriented_edge("e", "a", 0.0);
    /// graph.add_oriented_edge("b", "e", 0.0);
    /// graph.add_oriented_edge("e", "f", 0.0);
    ///
    /// graph.add_oriented_edge("b", "c", 0.0);
    /// graph.add_oriented_edge("f", "g", 0.0);
    /// graph.add_oriented_edge("g", "f", 0.0);
    /// graph.add_oriented_edge("c", "g", 0.0);
    ///
    /// graph.add_oriented_edge("c", "d", 0.0);
    /// graph.add_oriented_edge("d", "c", 0.0);
    /// graph.add_oriented_edge("d", "h", 0.0);
    /// graph.add_oriented_edge("h", "d", 0.0);
    /// graph.add_oriented_edge("h", "g", 0.0);
    ///
    /// let components = graph.strongly_connected_components();
    /// assert_eq!(components[0], ["a", "b", "e"]);
    /// assert_eq!(components[1], ["c", "d", "h"]);
    /// assert_eq!(components[2], ["f", "g"]);
    ///```

    pub fn strongly_connected_components(&self) -> Vec<Vec<Indent>> {
        let mut components = vec![];
        let mut graph_transp = Graph::new();
        for (vertex, edges) in &self.adj {
            for edge in edges {
                graph_transp.add_oriented_edge(edge.to.clone(), vertex.clone(), edge.weight);
            }
        }
        let mut visited = BTreeSet::new();
        let mut orders = Vec::with_capacity(self.adj.len());
        for vertex in self.adj.keys(){
            if !visited.contains(vertex) {
                for (vertex, _) in self.dfs(vertex.clone()){
                    if !visited.contains(&vertex) {
                        visited.insert(vertex.clone());
                        orders.push(vertex.clone());
                    }
                }
            }
        }
        visited.clear();
        for vertex in &orders {
            if !visited.contains(vertex) {
                let mut vec = vec![];
                for (vertex, _) in graph_transp.dfs(vertex.clone()) {
                    if !visited.contains(&vertex) {
                        visited.insert(vertex.clone());
                        vec.push(vertex);
                    }
                }
                components.push(vec);
            }
        }
        components
    }

    /// Topologic sort
    /// ```
    /// use librualg::graph::Graph;
    /// let mut graph = Graph::new();
    /// graph.add_oriented_edge("a", "b", 0.0);
    /// graph.add_oriented_edge("a", "c", 0.0);
    /// graph.add_oriented_edge("a", "e", 0.0);
    /// graph.add_oriented_edge("a", "d", 0.0);
    /// graph.add_oriented_edge("b", "d", 0.0);
    /// graph.add_oriented_edge("c", "d", 0.0);
    /// graph.add_oriented_edge("c", "e", 0.0);
    ///
    /// assert_eq!(graph.topological_sort(), vec!["a", "b", "c", "d", "e"]);
    ///```

    pub fn topological_sort(&self) -> Vec<Indent> {
        let mut visited = BTreeSet::new();
        let mut topology_vec = Vec::with_capacity(self.adj.len());
        for vertex in self.adj.keys() {
            if !visited.contains(vertex) {
                for (vertex, _) in self.dfs(vertex.clone()).iter(){
                    if !visited.contains(vertex) {
                        visited.insert(vertex.clone());
                        topology_vec.push(vertex.clone());
                    }
                }
            }
        }
        topology_vec
    }

    /// Kruskal's algorithm
    /// ```
    /// use librualg::graph::Graph;
    ///
    /// let mut graph = Graph::new();
    /// graph.add_oriented_edge('A', 'B', 7.0);
    /// graph.add_oriented_edge('B', 'A', 7.0);
    /// graph.add_oriented_edge('A', 'D', 5.0);
    /// graph.add_oriented_edge('D', 'A', 5.0);
    /// let tree = graph.kruskal();
    /// ```

    pub fn kruskal(&self) -> Graph<Indent> {

        struct D<Indent> {
            from: Indent,
            to: Indent,
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

        let mut graph: Graph<Indent> = Graph::new();
        let mut heap = BinaryHeap::new();
        let mut dsu = DSU::new();
        for (from, edges) in &self.adj {
            dsu.make_set(from.clone());
            for edge in edges {
                heap.push(D{
                    from: from.clone(),
                    to: edge.to.clone(),
                    dist: edge.weight
                });
            }
        }

        while let Some (value) = heap.pop() {
            if dsu.find_set(value.from.clone()) != dsu.find_set(value.to.clone()) {
                dsu.union_sets(value.from.clone(), value.to.clone());
                graph.add_oriented_edge(value.from.clone(), value.to.clone(), value.dist);
                graph.add_oriented_edge(value.to.clone(), value.from.clone(), value.dist);
            }
        }
        graph
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

#[derive(Clone, Copy, Default)]
pub struct VertexNumProperties {
    parent: Option<usize>,
    #[allow(dead_code)]
    color: Color,
    #[allow(dead_code)]
    time_in: Option<usize>,
    #[allow(dead_code)]
    time_out: Option<usize>
}

#[derive(Clone, Copy)]
struct EdgeNum  {
    to: usize,
    weight: f32,
}

pub struct GraphNum  {
    adj: Vec::<Option<Vec<EdgeNum>>>,
}

impl GraphNum {
    pub fn new(n: usize) -> Self {
        GraphNum {
            adj: vec![None; n + 1]
        }
    }

    /// Adds a new vertex to the graph
    pub fn add_vertex(&mut self, vertex: usize) {
        self.adj[vertex] = Some(Vec::new());
    }
    /// Adds a new oriented edge to the graph
    pub fn add_oriented_edge(&mut self, from: usize, to: usize, weight: f32) {
        self.adj[from].as_mut().unwrap().push(EdgeNum{ to, weight });
    }

    /// BFS (Breadth-First Search) algorithm.
    /// Returns an ancestor vector along the graph traversal path
    ///```
    /// use librualg::graph::GraphNum;
    ///
    /// let mut graph = GraphNum::new(20);
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    /// graph.add_vertex(4);
    /// graph.add_vertex(5);
    /// graph.add_vertex(8);
    /// graph.add_vertex(17);
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
    /// graph.add_oriented_edge(17, 1, 0.0);
    /// let parents = graph.bfs(1);
    /// assert_eq!(graph.search_path(5, &parents).unwrap(), vec![1, 2, 5]);
    ///  assert_eq!(graph.search_path(17, &parents).unwrap(), vec![1, 2, 4, 8, 17]);
    ///
    /// let parents = graph.bfs(11);
    /// assert_eq!(graph.search_path(11, &parents), None);
    ///```

    pub fn bfs(&self, from: usize) -> Vec<VertexNumProperties> {
        let mut queue = VecDeque::new();
        let mut parents = vec![VertexNumProperties::default(); self.adj.len()];
        let mut visited = vec![false; self.adj.len()];

        if self.adj[from].is_some() {
            queue.push_back(from);
            visited[from] = true;
            parents[from] = VertexNumProperties {parent: None, time_in: None, time_out: None, color: Color::White};
            while let Some(vertex) = queue.pop_front(){
                if self.adj[vertex].is_some() {
                    for edge in self.adj[vertex].as_ref().unwrap().iter() {
                        if !visited[edge.to] {
                            parents[edge.to] = VertexNumProperties {parent: Some(vertex.clone()), time_in: None, time_out: None, color: Color::White};
                            queue.push_back(edge.to);
                            visited[edge.to] = true;
                        }
                    }
                }
            }
        }
        parents
    }

    fn _dfs(&self, from: usize, timer: &mut usize,  parents: &mut Vec<VertexNumProperties>, colors: &mut Vec<Color>) {
        *timer += 1;
        colors[from] = Color::Grey;
        if self.adj[from].is_some() {
            for edge in self.adj[from].as_ref().unwrap().iter() {
                if colors[edge.to] == Color::White {
                    parents[edge.to] = VertexNumProperties{parent: Some(from.clone()), time_in: None, time_out: None, color: Color::White};
                    self._dfs(edge.to, timer, parents, colors);
                }
            }
        }
        colors[from] = Color::Black;
        *timer += 1;
        parents[from].time_out = Some(*timer);
        parents[from].color = Color::Black;
    }

    /// DFS (Depth-First Search) algorithm.
    /// Returns an ancestor vector along the graph traversal path
    ///```
    /// use librualg::graph::GraphNum;
    ///
    /// let mut graph = GraphNum::new(10);
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    /// graph.add_vertex(5);
    /// graph.add_oriented_edge(1, 2, 0.0);
    /// graph.add_oriented_edge(2, 3, 0.0);
    /// graph.add_oriented_edge(3, 5, 0.0);
    ///
    /// let res = graph.dfs(1);
    /// assert_eq!(graph.search_path(5, &res).unwrap(), vec![1, 2, 3, 5]);
    /// ```

    pub fn dfs(&self, from: usize) -> Vec<VertexNumProperties> {
        let mut parents = vec![VertexNumProperties::default(); self.adj.len()];
        let mut colors = vec![Color::White; self.adj.len()];
        let mut timer = 0;
        parents[from] = VertexNumProperties{parent: None, time_in: Some(timer), time_out: None, color: Color::White};
        self._dfs(from, &mut timer, &mut parents, &mut colors);
        parents
    }

    /// Dijkstra algorithm.
    /// Returns an ancestor vector along the graph traversal path and distances to the other vertexs
    ///```
    /// use librualg::graph::GraphNum;
    ///
    /// let mut graph = GraphNum::new(10);
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    /// graph.add_vertex(5);
    /// graph.add_vertex(7);
    ///
    /// graph.add_oriented_edge(1, 2, 2.0);
    /// graph.add_oriented_edge(2, 3, 5.0);
    /// graph.add_oriented_edge(3, 5, 7.0);
    /// graph.add_oriented_edge(1, 5, 19.0);
    ///
    /// let (parents, distances) = graph.dijkstra(1);
    /// assert_eq!(graph.search_path(5, &parents).unwrap(), vec![1, 2, 3, 5]);
    /// assert_eq!(graph.search_path(3, &parents).unwrap(), vec![1, 2, 3]);
    /// assert_eq!(distances[5].unwrap(), 14.0);
    /// assert_eq!(distances[7], None);
    /// ```
    pub fn dijkstra(&self, from: usize) -> (Vec<VertexNumProperties>, Vec<Option<f32>>) {
        let mut parents = vec![VertexNumProperties::default(); self.adj.len()];
        let mut visited = vec![false; self.adj.len()];
        let mut distances = vec![None; self.adj.len()];

        struct D {
            node: usize,
            dist: f32,
        }
        impl std::cmp::PartialEq for D {
            fn eq(&self, other: &D) -> bool {
                self.dist == other.dist
            }
        }

        impl Eq for D {}

        impl std::cmp::Ord for D {
            fn cmp(&self, other: &Self) -> Ordering {
                other.dist.partial_cmp(&self.dist).unwrap()
            }
        }

        impl std::cmp::PartialOrd for D {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(other.dist.partial_cmp(&self.dist).unwrap())
            }
        }

        let mut heap = BinaryHeap::<D>::new();
        distances[from] = Some(0.0);
        heap.push(D{ node: from, dist: 0.0});
        while !heap.is_empty() {
            let d = heap.pop().unwrap();
            visited[d.node] = true;
            if self.adj[d.node].as_ref().is_some() {
                for edge in self.adj[d.node].as_ref().unwrap() {
                    if !visited[edge.to] && edge.weight + d.dist < distances[edge.to].unwrap_or(f32::MAX) {
                        parents[edge.to] = VertexNumProperties{parent: Some(d.node.clone()), time_in: None, time_out: None, color: Color::White};
                        distances[edge.to] = Some(edge.weight + d.dist);
                        heap.push(D{node: edge.to.clone(), dist: distances[edge.to].unwrap()});
                    }
                }
            }
        }
        (parents, distances)
    }

    /// Get connected components
    ///```
    /// use librualg::graph::GraphNum;
    ///
    /// let mut graph = GraphNum::new(20);
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    /// graph.add_vertex(4);
    /// graph.add_vertex(5);
    /// graph.add_vertex(6);
    /// graph.add_vertex(7);
    /// graph.add_vertex(8);
    /// graph.add_vertex(9);
    /// graph.add_vertex(10);
    /// graph.add_vertex(11);
    /// graph.add_oriented_edge(1, 2, 0.0);
    /// graph.add_oriented_edge(2, 3, 0.0);
    /// graph.add_oriented_edge(3, 4, 0.0);
    ///
    /// graph.add_oriented_edge(5, 6, 0.0);
    /// graph.add_oriented_edge(6, 7, 0.0);
    ///
    /// graph.add_oriented_edge(8, 9, 0.0);
    /// graph.add_oriented_edge(9, 10, 0.0);
    /// graph.add_oriented_edge(10, 11, 0.0);
    ///
    /// let components = graph.connected_components();
    /// assert_eq!(components[0], [1, 2, 3, 4]);
    /// assert_eq!(components[1], [5, 6, 7]);
    /// assert_eq!(components[2], [8, 9, 10, 11]);
    /// ```
    pub fn connected_components(&self) -> Vec<Vec<usize>> {
        let mut components = vec![];
        let mut visited = vec![false; self.adj.len()];
        for (vertex, edges) in self.adj.iter().enumerate() {
            if let Some(_) = edges {
                if !visited[vertex] {
                    let mut queue = VecDeque::new();
                    let mut vec = vec![];
                    visited[vertex] = true;
                    queue.push_back(vertex);
                    while let Some(vertex) = queue.pop_front(){
                        vec.push(vertex);
                        if self.adj[vertex].is_some() {
                            for edge in self.adj[vertex].as_ref().unwrap() {
                                if !visited[edge.to] {
                                    queue.push_back(edge.to);
                                    visited[edge.to] = true;
                                }
                            }
                        }
                    }
                    components.push(vec)
                }
            }
        }
        components
    }

    pub fn strongly_connected_components(&self) -> Vec<Vec<usize>> {
        let mut components = vec![];
        let mut graph_transp = GraphNum::new(self.adj.len() - 1);
        for (vertex, edges) in self.adj.iter().enumerate() {
            if let Some(_) = edges {
                graph_transp.add_vertex(vertex);
            }
        }
        for (vertex, edges) in self.adj.iter().enumerate() {
            if let Some(edges) = edges {
                for edge in edges {
                    graph_transp.add_oriented_edge(edge.to, vertex, edge.weight);
                }
            }
        }
        let mut visited = vec![false; self.adj.len()];
        let mut orders = Vec::with_capacity(self.adj.len());
        for (vertex, edges) in self.adj.iter().enumerate() {
            if let Some(_) = edges {
                if !visited[vertex] {
                    for (vertex, property) in self.dfs(vertex).iter().enumerate(){
                        if self.adj[vertex].is_some() {
                            if !visited[vertex] && property.color == Color::Black {
                                visited[vertex] = true;
                                orders.push(vertex);
                            }
                        }
                    }
                }
            }
        }
        let mut visited = vec![false; self.adj.len()];
        for vertex in orders {
            if !visited[vertex] {
                let mut vec = vec![];
                for (vertex, property) in graph_transp.dfs(vertex).iter().enumerate() {
                    if graph_transp.adj[vertex].is_some() && property.color == Color::Black {
                        if !visited[vertex] {
                            visited[vertex] = true;
                            vec.push(vertex);
                        }
                    }
                }
                components.push(vec);
            }
        }
        components
    }

    pub fn topological_sort(&self) -> Vec<usize> {
        let mut visited = vec![false; self.adj.len()];
        let mut topology_vec = Vec::with_capacity(self.adj.len());
        for (vertex, edges) in self.adj.iter().enumerate() {
            if let Some(_) = edges {
                if !visited[vertex] {
                    for (vertex, property) in self.dfs(vertex).iter().enumerate() {
                        if !visited[vertex] && property.color == Color::Black {
                            visited[vertex] = true;
                            topology_vec.push(vertex);
                        }
                    }
                }
            }
        }
        topology_vec
    }

    /// Kruskal's algorithm
    /// ```
    /// use librualg::graph::GraphNum;
    ///
    /// let mut graph = GraphNum::new(20);
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    /// graph.add_vertex(4);
    /// graph.add_vertex(5);
    /// graph.add_vertex(6);
    /// graph.add_vertex(7);
    ///
    /// graph.add_oriented_edge(1, 2, 7.0);
    /// graph.add_oriented_edge(2, 1, 7.0);
    /// graph.add_oriented_edge(1, 4, 5.0);
    /// graph.add_oriented_edge(4, 1, 5.0);
    /// graph.add_oriented_edge(2, 3, 8.0);
    /// graph.add_oriented_edge(3, 2, 8.0);
    /// graph.add_oriented_edge(2, 5, 7.0);
    /// graph.add_oriented_edge(5, 2, 7.0);
    /// graph.add_oriented_edge(2, 4, 9.0);
    /// graph.add_oriented_edge(4, 2, 9.0);
    /// graph.add_oriented_edge(3, 5, 5.0);
    /// graph.add_oriented_edge(5, 3, 5.0);
    /// graph.add_oriented_edge(5, 7, 9.0);
    /// graph.add_oriented_edge(7, 5, 9.0);
    /// graph.add_oriented_edge(5, 6, 8.0);
    /// graph.add_oriented_edge(6, 5, 8.0);
    /// graph.add_oriented_edge(5, 4, 15.0);
    /// graph.add_oriented_edge(4, 5, 15.0);
    /// graph.add_oriented_edge(6, 7, 11.0);
    /// graph.add_oriented_edge(7, 6, 11.0);
    /// graph.add_oriented_edge(6, 4, 6.0);
    /// graph.add_oriented_edge(4, 6, 6.0);
    /// let tree = graph.kruskal();
    /// assert_eq!(vec![1, 2, 5, 7], tree.search_path(7, &tree.bfs(1)).unwrap());
    /// assert_eq!(vec![1, 2, 5, 3], tree.search_path(3, &tree.bfs(1)).unwrap());
    /// ```

    pub fn kruskal(&self) -> GraphNum {
        struct D {
            from: usize,
            to: usize,
            dist: f32,
        }

        impl std::cmp::PartialEq for D {
            fn eq(&self, other: &D) -> bool {
                self.dist == other.dist
            }
        }

        impl Eq for D {}

        impl std::cmp::Ord for D {
            fn cmp(&self, other: &Self) -> Ordering {
                other.dist.partial_cmp(&self.dist).unwrap()
            }
        }

        impl std::cmp::PartialOrd for D {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(other.dist.partial_cmp(&self.dist).unwrap())
            }
        }

        let mut graph: GraphNum = GraphNum::new(self.adj.len() + 1);
        let mut heap = BinaryHeap::new();
        let mut dsu = DSUNum::new(graph.adj.len());
        for (from, edges) in self.adj.iter().enumerate() {
            if let Some(edges) = edges {
                graph.add_vertex(from);
                dsu.make_set(from);
                for edge in edges {
                    heap.push(D{
                        from,
                        to: edge.to,
                        dist: edge.weight
                    });
                }
            }
        }

        while let Some (value) = heap.pop() {
            if dsu.find_set(value.from) != dsu.find_set(value.to) {
                dsu.union_sets(value.from, value.to);
                graph.add_oriented_edge(value.from, value.to, value.dist);
                graph.add_oriented_edge(value.to, value.from, value.dist);
            }
        }
        graph
    }

    pub fn search_path(&self, mut target: usize, parents: &[VertexNumProperties]) -> Option<Vec<usize>> {
        if parents[target].parent.is_none() {
            return None;
        }
        let mut path = vec![target];
        while let Some(next) = parents[target].parent{
            path.push(next);
            target = next;
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

    let res = graph.dfs(1);
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

    graph.add_oriented_edge("k", "m", 0.0);
    graph.add_oriented_edge("m", "k", 0.0);

    let components = graph.strongly_connected_components();
    assert_eq!(components[0], ["a", "b", "e"]);
    assert_eq!(components[1], ["c", "d", "h"]);
    assert_eq!(components[2], ["f", "g"]);
    assert_eq!(components[3], ["k", "m"]);
    assert_eq!(components.len(), 4);
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

    graph.add_oriented_edge("x", "y", 0.0);
    graph.add_oriented_edge("y", "z", 0.0);

    assert_eq!(graph.topological_sort(), vec!["a", "b", "c", "d", "e", "x", "y", "z"]);
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
    assert_eq!(components.len(), 3);
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

#[test]
fn topology_sort_num() {
    let mut graph = GraphNum::new(10);

    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    graph.add_vertex(4);
    graph.add_vertex(5);

    graph.add_oriented_edge(1, 2, 0.0);
    graph.add_oriented_edge(1, 3, 0.0);
    graph.add_oriented_edge(1, 5, 0.0);
    graph.add_oriented_edge(1, 4, 0.0);
    graph.add_oriented_edge(2, 4, 0.0);
    graph.add_oriented_edge(3, 4, 0.0);
    graph.add_oriented_edge(3, 5, 0.0);

    assert_eq!(graph.topological_sort(), vec![1, 2, 3, 4, 5]);

    graph.add_vertex(6);
    graph.add_vertex(7);
    graph.add_vertex(8);
    graph.add_oriented_edge(6, 7, 0.0);
    graph.add_oriented_edge(7, 8, 0.0);

    assert_eq!(graph.topological_sort(), vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_kruskal_num() {
    let mut graph = GraphNum::new(20);
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    graph.add_vertex(4);
    graph.add_vertex(5);
    graph.add_vertex(6);
    graph.add_vertex(7);


    graph.add_oriented_edge(1, 2, 7.0);
    graph.add_oriented_edge(2, 1, 7.0);
    graph.add_oriented_edge(1, 4, 5.0);
    graph.add_oriented_edge(4, 1, 5.0);
    graph.add_oriented_edge(2, 3, 8.0);
    graph.add_oriented_edge(3, 2, 8.0);
    graph.add_oriented_edge(2, 5, 7.0);
    graph.add_oriented_edge(5, 2, 7.0);
    graph.add_oriented_edge(2, 4, 9.0);
    graph.add_oriented_edge(4, 2, 9.0);
    graph.add_oriented_edge(3, 5, 5.0);
    graph.add_oriented_edge(5, 3, 5.0);
    graph.add_oriented_edge(5, 7, 9.0);
    graph.add_oriented_edge(7, 5, 9.0);
    graph.add_oriented_edge(5, 6, 8.0);
    graph.add_oriented_edge(6, 5, 8.0);
    graph.add_oriented_edge(5, 4, 15.0);
    graph.add_oriented_edge(4, 5, 15.0);
    graph.add_oriented_edge(6, 7, 11.0);
    graph.add_oriented_edge(7, 6, 11.0);
    graph.add_oriented_edge(6, 4, 6.0);
    graph.add_oriented_edge(4, 6, 6.0);
    let tree = graph.kruskal();
    assert_eq!(vec![1, 2, 5, 7], tree.search_path(7, &tree.bfs(1)).unwrap());
    assert_eq!(vec![1, 2, 5, 3], tree.search_path(3, &tree.bfs(1)).unwrap());
}