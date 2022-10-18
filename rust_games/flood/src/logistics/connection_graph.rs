use std::cell::RefCell;
use std::collections::hash_map::Keys;
use std::collections::{BinaryHeap, HashMap};

use super::network::{NodeId, NodePath};

type ConnectionCost = f32;

#[derive(Default)]
pub struct ConnectionGraph {
    connections: HashMap<NodeId, HashMap<NodeId, ConnectionCost>>,
}

/// NetworkGraph only tracks the connections between nodes
impl ConnectionGraph {
    /// Creates or updates connection between specified nodes
    pub fn set_connection(&mut self, node_one: NodeId, node_two: NodeId, cost: ConnectionCost) {
        self.add_link(node_one, node_two, cost);
        self.add_link(node_two, node_one, cost);
    }

    fn add_link(&mut self, node_one: NodeId, node_two: NodeId, cost: ConnectionCost) {
        let connections = match self.connections.get_mut(&node_one) {
            Some(node_connections) => node_connections,
            None => {
                self.connections.insert(node_one, HashMap::new());
                self.connections.get_mut(&node_one).unwrap()
            }
        };
        connections.insert(node_two, cost);
    }

    /// Removes individual connection from
    pub fn remove_connection(&mut self, _node_one: NodeId, _node_two: NodeId) {
        todo!()
    }

    pub fn connections(&self, node: NodeId) -> Option<Keys<NodeId, ConnectionCost>> {
        match self.connections.get(&node) {
            Some(connections) => Some(connections.keys()),
            None => None,
        }
    }

    pub fn connection_count(&self, node: NodeId) -> u32 {
        match self.connections.get(&node) {
            Some(connections) => connections.len() as u32,
            None => 0,
        }
    }

    /// Removes all connections for a single node
    pub fn remove_connections(&mut self, node: NodeId) {
        if let Some(node_connections) = self.connections.remove(&node) {
            for key in node_connections.keys() {
                if let Some(other_node_connections) = self.connections.get_mut(&node) {
                    other_node_connections.remove(key);
                }
            }
        }
    }

    /// Calculate shortest path between src and dst, path does not contain source node
    pub fn shortest_path(&self, src: NodeId, dst: NodeId) -> NodePath {
        assert_ne!(src, dst);
        dijkstra_shortest_path(self, src, dst)
    }
    // pub fn connections(&self, node: NodeId) -> Vec<NodeId> {
    //     todo!()
    // }
}

use std::cmp::Ordering;

// State pulled from binary heap docs for rust
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    pub cost: usize,
    pub node_id: u32,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node_id.cmp(&other.node_id))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra_shortest_path(graph: &ConnectionGraph, src: NodeId, dst: NodeId) -> NodePath {
    let mut exploration = BinaryHeap::new();
    exploration.push(State {
        cost: 0,
        node_id: src,
    });
    todo!()
}
