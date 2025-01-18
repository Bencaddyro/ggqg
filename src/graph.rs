use std::fmt;
use std::fmt::Display;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;
use std::fmt::Write;

use crate::node::Node;


pub struct Graph<N, E>
where
    N: Clone + Display,
    E: Clone + Display,
{
    nodes: HashMap<Uuid, Node<N, E>>,
    // edges: HashMap<Uuid, Edge<N, E>>,
}

impl<N, E> Default for Graph<N, E>
where
    N: Clone + Display,
    E: Clone + Display,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<N, E> Graph<N, E>
where
    N: Clone + Display,
    E: Clone + Display,
{
    pub fn new() -> Self {
        Self {
            nodes: HashMap::default(),
            // edges: HashMap::default(),
        }
    }

    pub fn contains(&self, key: &Uuid) -> bool {
        self.nodes.contains_key(key)
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    pub fn get(&self, key: &Uuid) -> Option<Node<N, E>> {
        self.nodes.get(key).cloned()
    }
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    pub fn insert(&mut self, node: Node<N, E>) -> bool {
        if self.nodes.contains_key(&node.key()) {
            false
        } else {
            self.nodes.insert(node.key(), node.clone());
            true
        }
    }
    // pub fn insert_edge(&mut self, edge: Edge<N, E>) -> bool {
    //     if self.edges.contains_key(&edge.key()) {
    //         false
    //     } else {
    //         self.edges.insert(edge.key(), edge.clone());
    //         true
    //     }
    // }
    pub fn remove(&mut self, key: &Uuid) -> Option<Node<N, E>> {
        self.nodes.remove(key)
    }
    // pub fn remove_edge(&mut self, key: &Uuid) -> Option<Edge<N, E>> {
    //     self.edges.remove(key)
    // }
    pub fn to_dot(&self) -> String {
        let mut string = String::new();
        string.push_str("digraph {\n");
        for (key, node) in self.nodes.iter() {
            write!(
                &mut string,
                "    \"{}\"[label=\"{}\"]",
                key.clone(),
                node.value().read().ok().unwrap()
            )
            .unwrap();
            // if let Ok(edges) = node.edges().read() { // edges label
            //     for (edge, next) in &edges.1 {
            //         if let Some(target) = next.upgrade() {
            //             write!(
            //                 &mut string,
            //                 "\n    \"{}\" -> \"{}\" [label=\"{}\"]",
            //                 key.clone(),
            //                 target.0,
            //                 edge
            //             )
            //             .unwrap();
            //         }
            //     }
            //     string.push('\n');
            // }
        }
        string.push('}');
        string
    }
}

