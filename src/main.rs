// -[ ] Define tree struct (in Rust ownership logic)
// -[ ]  Spawn base tree
// -[ ]  Print base tree
// -[ ]  Rewrite node based on rule
// -[ ]  Define rule
// -[ ]  Inner logic of expanding tree until all node is atomic

use std::collections::HashMap;
use std::fmt::Write;
use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;

pub struct Graph<N> {
    nodes: HashMap<Uuid, Node<N>>,
}

impl<N> Graph<N>
where N: Clone,
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
    pub fn get(&self, key: &Uuid) -> Option<Node<N>> {
        self.nodes.get(key).cloned()
    }
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    pub fn insert(&mut self, node: Node<N>) -> bool {
        if self.nodes.contains_key(&node.inner.0) {
            false
        } else {
            self.nodes.insert(node.inner.0, node.clone());
            true
        }
    }

    pub fn remove(&mut self, key: &Uuid) -> Option<Node<N>> {
        self.nodes.remove(key)
    }

    // pub fn to_dot(&self) -> String {
    //     let mut string = String::new();
    //     string.push_str("digraph {\n");
    //     for (key, node) in self.nodes.iter() {
    //         write!(
    //             &mut string,
    //             "    \"{}[label=\"{:?}\"]\"",
    //             key.clone(),
    //             node.inner.2
    //         )
    //         .unwrap();
    //         if let Ok(edges) = node.inner.1.outbound.read() {
    //             for (_, edge) in &*edges {
    //                 if let Some(target) = edge.inner.upgrade() {
    //                     write!(&mut string, "\n    \"{}\" -> \"{}\"", key.clone(), target.0)
    //                         .unwrap();
    //                 }
    //             }
    //             string.push('\n');
    //         }
    //     }
    //     string.push('}');
    //     string
    // }
}

pub struct Edges<N> {
    outbound: Vec<(EdgeContent, WeakNode<N>)>,
    inbound: Vec<(EdgeContent, WeakNode<N>)>,
}
impl<N> Edges<N>
where N: Clone,
{
    pub fn new() -> Self {
        Edges {
            outbound: Vec::new(),
            inbound: Vec::new(),
        }
    }
}

pub struct WeakNode<N>
where N: Clone,
 {
    inner: Weak<(Uuid, RwLock<Edges<N>, RwLock<N>)>,
}

#[derive(Clone)]
pub struct Node<N> {
    inner: Arc<(Uuid, RwLock<Edges<N>>, RwLock<N>)>,
}

impl<N> Node<N>
where N: Clone,
{
    // pub fn insert_outbound(&self, node: Node<N>) {
    //     if let Ok(mut edges) = self.inner.1.outbound.write() {
    //         edges.push((
    //             EdgeContent::None,
    //             WeakNode {
    //                 inner: Arc::downgrade(&node.inner),
    //             },
    //         ));
    //     }
    //     if let Ok(mut edges) = node.inner.1.inbound.write() {
    //         edges.push((
    //             EdgeContent::None,
    //             WeakNode {
    //                 inner: Arc::downgrade(&self.inner),
    //             },
    //         ));
    //     }
    // }
    // 
    // pub fn insert_inbound(&self, node: Node<N>) {
    //     if let Ok(mut edges) = self.inner.1.inbound.write() {
    //         edges.push((
    //             EdgeContent::None,
    //             WeakNode {
    //                 inner: Arc::downgrade(&node.inner),
    //             },
    //         ));
    //     }
    //     if let Ok(mut edges) = node.inner.1.outbound.write() {
    //         edges.push((
    //             EdgeContent::None,
    //             WeakNode {
    //                 inner: Arc::downgrade(&self.inner),
    //             },
    //         ));
    //     }
    // }

    pub fn new(value: N) -> Self {
        Node {
            inner: Arc::new((Uuid::new_v4(), Edges::new(), RwLock::new(value))),
        }
    }
}
/*
pub struct NodeContent {
    r#type : NodeType
}*/


pub enum EdgeContent {
    None,
    // North,
    // South,
    // East,
    // West,
}

fn main() {
    let mut g: Graph<()> = Graph::new();

    // let n1 = Node::new(NodeContent::Empty);
    //     let n2 = Node::new();
    //     let n3 = Node::new();
    //     let n4 = Node::new();
    //
    // g.insert(n1.clone());
    //     g.insert(n2.clone());
    //     g.insert(n3.clone());
    //     g.insert(n4.clone());
    //
    //     n1.insert_outbound(n2.clone());
    //     n1.insert_outbound(n3.clone());
    //     n2.insert_inbound(n4.clone());

    println!("{}", g.to_dot());
}
