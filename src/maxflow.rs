//! This module provides a [Maximum Flow](https://en.wikipedia.org/wiki/Maximum_flow_problem)
//! implementation over a generic directed graph with edge capacities.
//!
//! Maximum flow finds the greatest possible flow from a source vertex to a sink
//! vertex in a flow network, where each edge has a capacity that limits how much
//! flow can pass through it.

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// A directed edge in the flow network.
pub struct Edge<T> {
    /// The value of the vertex this edge points to.
    pub to: T,
    /// The maximum flow that can pass through this edge.
    pub capacity: u64,
    /// Remaining capacity available during the algorithm.
    /// Starts equal to `capacity` and decreases as flow is pushed.
    remaining: u64,
}

impl<T> Edge<T> {
    pub fn new(to: T, capacity: u64) -> Self {
        Self {
            to,
            capacity,
            remaining: capacity,
        }
    }
}

/// A single vertex in the flow network.
pub struct Vertex<T> {
    /// The identifier of this vertex.
    pub value: T,
    /// The outgoing edges from this vertex.
    pub edges: Vec<Edge<T>>,
}

impl<T> Vertex<T> {
    pub fn new(value: T, edges: Vec<Edge<T>>) -> Self {
        Self { value, edges }
    }
}

/// BFS from `source` to `sink` over edges with positive remaining capacity.
fn bfs<'a, T>(
    source: &'a T,
    sink: &T,
    graph: &'a HashMap<T, Vertex<T>>,
) -> Option<HashMap<&'a T, &'a T>>
where
    T: Eq + Hash,
{
    let mut parent: HashMap<&T, &T> = HashMap::from([(source, source)]);
    let mut queue = VecDeque::from([source]);

    while let Some(current) = queue.pop_front() {
        for edge in &graph[current].edges {
            if !parent.contains_key(&edge.to) && edge.remaining > 0 {
                parent.insert(&edge.to, current);
                if &edge.to == sink {
                    return Some(parent);
                }
                queue.push_back(&edge.to);
            }
        }
    }

    None
}

/// Computes the maximum flow from `source` to `sink` in the flow network.
///
/// Uses the [Edmonds-Karp](https://en.wikipedia.org/wiki/Edmonds%E2%80%93Karp_algorithm)
/// algorithm (BFS-based Ford-Fulkerson), running in O(VE²) time.
pub fn maxflow<T>(vertices: Vec<Vertex<T>>, source: &T, sink: &T) -> u64
where
    T: Eq + Hash + Clone,
{
    let mut graph: HashMap<T, Vertex<T>> =
        vertices.into_iter().map(|v| (v.value.clone(), v)).collect();

    let forward_edges: Vec<(T, T)> = graph
        .values()
        .flat_map(|v| v.edges.iter().map(|e| (v.value.clone(), e.to.clone())))
        .collect();

    for (from, to) in forward_edges {
        if !graph[&to].edges.iter().any(|e| e.to == from) {
            graph.get_mut(&to).unwrap().edges.push(Edge {
                to: from,
                capacity: 0,
                remaining: 0,
            });
        }
    }

    let mut total_flow = 0u64;

    while let Some(parent) = bfs(source, sink, &graph) {
        let path: Vec<(T, T)> = {
            let mut steps = Vec::new();
            let mut node = sink;
            while node != source {
                let prev = parent[node];
                steps.push((prev.clone(), node.clone()));
                node = prev;
            }
            steps
        };

        let bottleneck = path
            .iter()
            .map(|(from, to)| {
                graph[from]
                    .edges
                    .iter()
                    .find(|e| &e.to == to)
                    .unwrap()
                    .remaining
            })
            .min()
            .unwrap();

        for (from, to) in path {
            graph
                .get_mut(&from)
                .unwrap()
                .edges
                .iter_mut()
                .find(|e| e.to == to)
                .unwrap()
                .remaining -= bottleneck;
            graph
                .get_mut(&to)
                .unwrap()
                .edges
                .iter_mut()
                .find(|e| e.to == from)
                .unwrap()
                .remaining += bottleneck;
        }

        total_flow += bottleneck;
    }

    total_flow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_edge() {
        // source --10--> sink
        let vertices = vec![
            Vertex::new("source", vec![Edge::new("sink", 10)]),
            Vertex::new("sink", vec![]),
        ];
        assert_eq!(maxflow(vertices, &"source", &"sink"), 10);
    }

    #[test]
    fn parallel_paths() {
        // source --5--> a --3--> sink
        //        \--8---------->/
        let vertices = vec![
            Vertex::new("source", vec![Edge::new("a", 5), Edge::new("sink", 8)]),
            Vertex::new("a", vec![Edge::new("sink", 3)]),
            Vertex::new("sink", vec![]),
        ];
        // via a: min(5,3)=3, direct: 8 → total 11
        assert_eq!(maxflow(vertices, &"source", &"sink"), 11);
    }

    #[test]
    fn bottleneck() {
        // source --2--> mid --100--> sink
        let vertices = vec![
            Vertex::new("source", vec![Edge::new("mid", 2)]),
            Vertex::new("mid", vec![Edge::new("sink", 100)]),
            Vertex::new("sink", vec![]),
        ];
        assert_eq!(maxflow(vertices, &"source", &"sink"), 2);
    }

    #[test]
    fn unreachable_sink() {
        // source --5--> a      sink (disconnected)
        let vertices = vec![
            Vertex::new("source", vec![Edge::new("a", 5)]),
            Vertex::new("a", vec![]),
            Vertex::new("sink", vec![]),
        ];
        assert_eq!(maxflow(vertices, &"source", &"sink"), 0);
    }

    #[test]
    fn diamond() {
        //        /--3--> b --3--\
        // source                 sink
        //        \--4--> c --4--/
        let vertices = vec![
            Vertex::new("source", vec![Edge::new("b", 3), Edge::new("c", 4)]),
            Vertex::new("b", vec![Edge::new("sink", 3)]),
            Vertex::new("c", vec![Edge::new("sink", 4)]),
            Vertex::new("sink", vec![]),
        ];
        // via b: 3, via c: 4 → total 7
        assert_eq!(maxflow(vertices, &"source", &"sink"), 7);
    }
}
