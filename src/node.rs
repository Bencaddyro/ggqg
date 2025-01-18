use std::fmt;
use std::fmt::Display;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;
use std::fmt::Write;

use crate::edge::{Edge};


#[derive(Clone)]
pub struct WeakNode<N, E>
where
    N: Clone,
    E: Clone,
{
    internal: Weak<(
        Uuid,
        RwLock<N>,
        RwLock<Vec<Edge<N, E>>>,
        RwLock<Vec<Edge<N, E>>>,
    )>,
}
impl<N, E> WeakNode<N, E>
where
    N: Clone,
    E: Clone,
{
    pub fn upgrade(&self) -> Option<Node<N, E>> {
        self.internal.upgrade().map(|internal| Node { internal })
    }
}

#[derive(Clone)]
pub struct Node<N, E>
where
    N: Clone,
    E: Clone,
{
    internal: Arc<(
        Uuid,
        RwLock<N>,
        RwLock<Vec<Edge<N, E>>>, //head
        RwLock<Vec<Edge<N, E>>>, //tail
    )>,
}

impl<N, E> Node<N, E>
where
    N: Clone,
    E: Clone,
{
    pub fn new(value: N) -> Self {
        Node {
            internal: Arc::new((
                Uuid::new_v4(),
                RwLock::new(value),
                RwLock::new(Vec::default()),
                RwLock::new(Vec::default()),
            )),
        }
    }
    pub fn key(&self) -> Uuid {
        self.internal.0
    }
    pub fn value(&self) -> &RwLock<N> {
        &self.internal.1
    }
    pub fn downgrade(&self) -> WeakNode<N, E> {
        WeakNode {
            internal: Arc::downgrade(&self.internal),
        }
    }
//     pub fn add_head(&self, node: Node<N, E>, edge: E) {
//         
//         if let Ok(mut head) = self.internal.2.write() {
//             head.push(Edge::new(edge.clone(), node.downgrade()));
//         }
//         
//         
//         if let Ok(mut edges) = node.internal.3.write() {
//             edges.previous.push((edge.clone(), self.downgrade()));
//         }
//         
//         
//     }
//     pub fn add_previous(&self, node: Node<N, E>, edge: E) {
//         if let Ok(mut edges) = self.internal.1.write() {
//             edges.previous.push((edge.clone(), node.downgrade()));
//         }
//         if let Ok(mut edges) = node.internal.1.write() {
//             edges.next.push((edge.clone(), self.downgrade()));
//         }
//     }

    // find
    // remove
}
