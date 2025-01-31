use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;

use crate::edge::Edge;

type _WeakNode<N, E> = Weak<(
    Uuid,
    RwLock<N>,
    RwLock<HashMap<Uuid, Edge<N, E>>>, // direct successor
    RwLock<HashMap<Uuid, Edge<N, E>>>, // direct predecessor
)>;
#[derive(Clone, Debug)]
pub struct WeakNode<N, E>
where
    N: Clone,
    E: Clone,
{
    internal: _WeakNode<N, E>,
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
type ArcNode<N, E> = Arc<(
    Uuid,
    RwLock<N>,
    RwLock<HashMap<Uuid, Edge<N, E>>>, // direct successor
    RwLock<HashMap<Uuid, Edge<N, E>>>, // direct predecessor
)>;
#[derive(Clone, Debug)]
pub struct Node<N, E>
where
    N: Clone,
    E: Clone,
{
    internal: ArcNode<N, E>,
}

impl<N, E> Node<N, E>
where
    N: Clone + Display,
    E: Clone + Display,
{
    pub fn new(value: N) -> Self {
        Node {
            internal: Arc::new((
                Uuid::new_v4(),
                RwLock::new(value),
                RwLock::new(HashMap::default()),
                RwLock::new(HashMap::default()),
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
    pub fn add_direct_successor(&self, node: &Self, edge: E) {
        let edge = Edge::new(edge.clone(), self.downgrade(), node.downgrade());
        if let Ok(mut successor) = self.internal.2.write() {
            successor.insert(edge.key(), edge.clone());
        }
        if let Ok(mut predecessor) = node.internal.3.write() {
            predecessor.insert(edge.key(), edge);
        }
    }
    pub fn add_direct_predecessor(&self, node: &Self, edge: E) {
        let edge = Edge::new(edge.clone(), node.downgrade(), self.downgrade());
        if let Ok(mut successor) = node.internal.2.write() {
            successor.insert(edge.key(), edge.clone());
        }
        if let Ok(mut predecessor) = self.internal.3.write() {
            predecessor.insert(edge.key(), edge);
        }
    }
    pub fn get_direct_successor(&self) -> &RwLock<HashMap<Uuid, Edge<N, E>>> {
        &self.internal.2
    }
    pub fn get_direct_predecessor(&self) -> &RwLock<HashMap<Uuid, Edge<N, E>>> {
        &self.internal.3
    }
    pub fn remove_edge(&self, edge: Edge<N, E>) {
        //  println!("Remove edge from {} to {}",
        //          edge.tail().read().unwrap().upgrade().unwrap().key().to_string()[..6].to_string(),
        //          edge.head().read().unwrap().upgrade().unwrap().key().to_string()[..6].to_string(),
        //          );

        if let Ok(mut head) = self.internal.2.write() {
            head.remove(&edge.key());
        }
        if let Ok(mut tail) = self.internal.3.write() {
            tail.remove(&edge.key());
        }
    }
    // pub fn search(node, edge uuid ?) -> Edge<N,E> {} TODO depend on needs
}

impl<N, E> Display for Node<N, E>
where
    N: Clone + Display,
    E: Clone + Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.value().read().unwrap())
    }
}
