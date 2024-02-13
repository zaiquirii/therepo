use std::mem;
use macroquad::math::{Rect, Vec2};
use macroquad::miniquad::start;

#[derive(Debug)]
pub struct Node<T> {
    pub bounds: Rect,
    pub elements: Vec<(Vec2, T)>,
    pub children: Option<[NodeId; 4]>,
}

#[derive(Clone, Copy, Debug)]
pub struct NodeId {
    pub index: usize,
}

pub struct Quadtree<T> {
    pub nodes: Vec<Node<T>>,
    pub root: NodeId,
}

impl<T: std::fmt::Debug> Quadtree<T> {
    pub fn new(bounds: Rect, starting_capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(starting_capacity);
        nodes.push(Node {
            bounds,
            elements: vec![],
            children: None,
        });
        Self {
            nodes,
            root: NodeId { index: 0 },
        }
    }

    pub fn insert(&mut self, pos: Vec2, value: T) {
        self.insert_help(self.root, pos, value);
    }

    fn insert_help(&mut self, node_id: NodeId, pos: Vec2, value: T) {
        let node = &mut self.nodes[node_id.index];
        // if !node.bounds.contains(pos) {
        //     for (i, n) in self.nodes.iter().enumerate() {
        //         println!("{} {:?} {} {:?}", i, n.bounds, n.elements.len(), n.children);
        //     }
        // }

        let node = &mut self.nodes[node_id.index];
        // println!("nodeId: {} Bounds: {:?}, pos: {:?}", node_id.index, node.bounds, pos);
        debug_assert!(node.bounds.contains(pos));

        let b = node.bounds;
        let half_width = b.w / 2.0;
        let half_height = b.h / 2.0;
        if let Some(children) = node.children {
            let lower_w = pos.x < b.x + half_width;
            let lower_h = pos.y < b.y + half_height;

            let index = match (lower_w, lower_h) {
                (true, true) => 0,
                (false, true) => 1,
                (true, false) => 2,
                (false, false) => 3
            };
            self.insert_help(children[index], pos, value);
            return;
        }

        node.elements.push((pos, value));
        if node.elements.len() > 16 {
            debug_assert!(node.children.is_none());
            let new_children = [
                self.new_node(Rect::new(b.x, b.y, half_width, half_height)),
                self.new_node(Rect::new(b.x + half_width, b.y, half_width, half_height)),
                self.new_node(Rect::new(b.x, b.y + half_height, half_width, half_height)),
                self.new_node(Rect::new(b.x + half_width, b.y + half_height, half_width, half_height)),
            ];

            let node = &mut self.nodes[node_id.index];
            let elements = mem::replace(&mut node.elements, vec![]);
            for e in elements {
                let lower_w = e.0.x < b.x + half_width;
                let lower_h = e.0.y < b.y + half_height;

                let index = match (lower_w, lower_h) {
                    (true, true) => 0,
                    (false, true) => 1,
                    (true, false) => 2,
                    (false, false) => 3
                };
                self.nodes[new_children[index].index].elements.push(e);
            }
            self.nodes[node_id.index].children = Some(new_children);
        }
    }

    fn new_node(&mut self, bounds: Rect) -> NodeId {
        let id = NodeId { index: self.nodes.len() };
        self.nodes.push(Node {
            bounds,
            elements: Vec::new(),
            children: None,
        });
        id
    }

    pub fn query(&self, pos: Vec2, radius: f32) -> MatchingElements {
        todo!()
    }
}

pub struct MatchingElements {}