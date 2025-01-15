// - [x] Define tree struct (in Rust ownership logic)
// - [ ] Spawn base tree
// - [x] Print base tree
// - [ ] Rewrite node based on rule
// - [ ] Define rule
// - [ ] Inner logic of expanding tree until all node is atomic
// - [ ] Basic operation
// - [ ] Toy example: random grow, pick a unexpanded node, expand it, repeat N time in 3 Direction
// - [ ] Refacto: split graph struct from app code
// - [ ] Redo: handle edge to get internal mutability, store them in hasmap + node contain WeakEdge

use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Write;
use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;

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
    pub fn insert(&mut self, node: Node<N, E>) -> bool {
        if self.nodes.contains_key(&node.internal.0) {
            false
        } else {
            self.nodes.insert(node.key(), node.clone());
            true
        }
    }
    pub fn remove(&mut self, key: &Uuid) -> Option<Node<N, E>> {
        self.nodes.remove(key)
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
            if let Ok(edges) = node.internal.1.read() {
                for (edge, next) in &edges.next {
                    if let Some(target) = next.internal.upgrade() {
                        write!(
                            &mut string,
                            "\n    \"{}\" -> \"{}\" [label=\"{}\"]",
                            key.clone(),
                            target.0,
                            edge
                        )
                        .unwrap();
                    }
                }
                string.push('\n');
            }
        }
        string.push('}');
        string
    }
}

pub struct Edges<N, E>
where
    N: Clone,
    E: Clone,
{
    next: Vec<(E, WeakNode<N, E>)>,
    previous: Vec<(E, WeakNode<N, E>)>,
}
impl<N, E> Default for Edges<N, E>
where
    N: Clone,
    E: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<N, E> Edges<N, E>
where
    N: Clone,
    E: Clone,
{
    pub fn new() -> Self {
        Edges {
            next: Vec::new(),
            previous: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct WeakNode<N, E>
where
    N: Clone,
    E: Clone,
{
    internal: Weak<(Uuid, RwLock<Edges<N, E>>, RwLock<N>)>,
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
    internal: Arc<(Uuid, RwLock<Edges<N, E>>, RwLock<N>)>,
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
                RwLock::new(Edges::new()),
                RwLock::new(value),
            )),
        }
    }
    pub fn key(&self) -> Uuid {
        self.internal.0
    }
    pub fn value(&self) -> &RwLock<N> {
        &self.internal.2
    }
    pub fn downgrade(&self) -> WeakNode<N, E> {
        WeakNode {
            internal: Arc::downgrade(&self.internal),
        }
    }

    pub fn add_next(&self, node: Node<N, E>, edge: E) {
        if let Ok(mut edges) = self.internal.1.write() {
            edges.next.push((edge.clone(), node.downgrade()));
        }
        if let Ok(mut edges) = node.internal.1.write() {
            edges.previous.push((edge.clone(), self.downgrade()));
        }
    }
    pub fn add_previous(&self, node: Node<N, E>, edge: E) {
        if let Ok(mut edges) = self.internal.1.write() {
            edges.previous.push((edge.clone(), node.downgrade()));
        }
        if let Ok(mut edges) = node.internal.1.write() {
            edges.next.push((edge.clone(), self.downgrade()));
        }
    }

    // find
    // remove
}
#[derive(Clone)]
pub enum Edge {
    None,
    Direction(Direction),
}
#[derive(Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
        }
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Edge::None => write!(f, ""),
            Edge::Direction(d) => d.fmt(f),
        }
    }
}

fn main() {
    let mut g: Graph<String, Edge> = Graph::new();

    let n1 = Node::new(String::from("N1"));
    let n2 = Node::new(String::from("N2"));
    let n3 = Node::new(String::from("N3"));
    let n4 = Node::new(String::from("N4"));

    g.insert(n1.clone());
    g.insert(n2.clone());
    g.insert(n3.clone());
    g.insert(n4.clone());

    n1.add_next(n2.clone(), Edge::Direction(Direction::West));
    n1.add_previous(n2.clone(), Edge::Direction(Direction::East));

    // n1.add_next(n3.clone());
    // n2.add_previous(n4.clone());
    // n3.add_next(n1.clone());
    // n4.add_previous(n4.clone());

    println!("{}", g.to_dot());
}
