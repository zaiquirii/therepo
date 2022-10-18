use std::{
    collections::{hash_map::Keys, HashMap},
    ops::Deref,
    sync::atomic::{AtomicU32, Ordering},
};

use bevy::{ecs::entity, prelude::Vec2};

use super::connection_graph::ConnectionGraph;

pub type NodePath = Vec<NodeId>;

// Newtype for u32 to standardize id access
// #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// pub struct NodeId(pub u32);

pub type NodeId = u32;
// impl NodeId {}

// impl NodeId {
//     pub fn new(entity_id: u32) -> Self {
//         // static next_id: AtomicU32 = AtomicU32::new(0);
//         // let prev_id = next_id.fetch_add(1, Ordering::SeqCst);
//         NodeId(entity_id)
//     }
// }
pub enum NodeType {
    /// Can only connect to Core nodes
    Leaf,
    /// Can connect to any other node
    Core,
}

pub struct NetworkNode {
    pub id: NodeId,
    pub node_type: NodeType,
    pub location: Vec2,
    pub range: f32,
}

#[derive(Default)]
pub struct Network {
    nodes: HashMap<NodeId, NetworkNode>,
    connections: ConnectionGraph,
}

impl Network {
    pub fn nodes(&self) -> &HashMap<NodeId, NetworkNode> {
        &self.nodes
    }

    pub fn connections(&self, node: NodeId) -> Option<Keys<u32, f32>> {
        self.connections.connections(node)
    }

    pub fn create_node(&mut self, id: NodeId, node_type: NodeType, location: Vec2, range: f32) {
        assert!(!self.nodes.contains_key(&id));
        let node = NetworkNode {
            id,
            node_type,
            location,
            range,
        };
        self.nodes.insert(id, node);
    }

    pub fn remove_node(&mut self, node: NodeId) {
        self.nodes.remove(&node);
        self.connections.remove_connections(node);
    }

    /// node_id should have already been created
    /// returns number of added connections
    pub fn reconcile_connections(&mut self, node_id: NodeId) -> u32 {
        /*
        Works by removing all existing connections for given node, and then
        reconstructing the connections for this node.

        Moving nodes will constantly have their connection costs updated, so it makes
        sense to toss them all. The issue is knowing which nodes to check for new
        connections.

        Improvements can take the form of:
        - detecting node that have not moved
        - creating a quadtree of nodes to limit node checks.
        */

        // The node should exist before calling create connections
        let target_node = self.nodes.get(&node_id).unwrap();

        self.connections.remove_connections(node_id);
        let mut added_connections = 0;
        for (test_id, test_node) in &self.nodes {
            if *test_id == node_id {
                continue;
            }

            let max_distance = test_node.range.max(test_node.range);
            let distance = test_node.location.distance(target_node.location);

            if distance <= max_distance {
                added_connections += 1;
                self.connections.set_connection(node_id, *test_id, distance);
            }
        }
        assert_eq!(
            added_connections,
            self.connections.connection_count(node_id)
        );
        added_connections
    }

    /// Calculate next node on the shortest path between src and dst
    pub fn next_node(&self, src: NodeId, dst: NodeId) -> NodeId {
        self.connections.shortest_path(src, dst)[0]
    }
}
