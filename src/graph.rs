use std::collections::HashMap;
use std::fmt::{Display, Write};
use uuid::Uuid;

use crate::node::Node;

pub struct Graph<N, E>
where
    N: Clone + Display,
    E: Clone + Display,
{
    nodes: HashMap<Uuid, Node<N, E>>,
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
    pub fn insert(&mut self, node: Node<N, E>) {
        self.nodes.insert(node.key(), node.clone());
    }
    pub fn remove(&mut self, key: &Uuid) -> Option<Node<N, E>> {
        self.nodes.remove(key)
    }
    pub fn filter(&self, f: fn(&Node<N,E>) -> bool) -> Vec<Node<N,E>> {
        let mut res = Vec::new();
        for elem in self.nodes.values() {
            if f(elem) { res.push(elem.clone()) };
        }
        res
        // TODO make more elegant with map / filter ?
    }
    
    
    
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
            if let Ok(successors) = node.get_direct_successor().read() {
                // edges label
                for edge in successors.values() {
                    if let Some(head) = edge.head().read().unwrap().upgrade() {
                        if let Some(tail) = edge.tail().read().unwrap().upgrade() {
                            if let Ok(value) = edge.value().read() {
                                write!(
                                    &mut string,
                                    "\n    \"{}\" -> \"{}\" [label=\"{}\"]",
                                    tail.key(),
                                    head.key(),
                                    value,
                                )
                                .unwrap();
                            }
                        }
                    }
                }
            }
            string.push('\n');
        }
        string.push('}');
        string
    }
}
