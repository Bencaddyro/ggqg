// - [x] Define tree struct (in Rust ownership logic)
// - [x] Spawn base tree
// - [x] Print base tree

// Rewrite Engine
// - [ ] Rewrite node based on rule
// - [ ] Define rule

// Map toy example
// - [ ] Inner logic of expanding tree until all node is atomic
// - [ ] Basic operation
// - [ ] Toy example: random grow, pick a unexpanded node, expand it, repeat N time in 3 Direction

// Printer
// print to dot file directly, with iteration numbert

use crate::graph::Graph;
use std::fmt::{Display, Formatter, Result};

mod edge;
mod graph;
mod node;

use crate::node::Node;

#[derive(Clone, Debug)]
pub enum Edge {
    None,
    Direction(Direction),
}
#[derive(Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
impl Display for Edge {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Edge::None => write!(f, ""),
            Edge::Direction(d) => write!(f, "{d}"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum NodeState {
    New,
    Old(String),
}

impl Display for NodeState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            NodeState::New => write!(f, "New"),
            NodeState::Old(value) => write!(f, "Old({value})",),
        }
    }
}

// #[derive(Clone, Default)]
// pub struct Void {}
//
// impl Display for Void {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "")
//     }
// }

fn main() {
    let mut g: Graph<NodeState, Edge> = Graph::new();

    let root = Node::new(NodeState::New);
    // let n2 = Node::new(NodeState::New);
    // let n3 = Node::new(NodeState::New);
    // let n4 = Node::new(NodeState::New);

    g.insert(root.clone());
    // g.insert(n2.clone());
    // g.insert(n3.clone());
    // g.insert(n4.clone());

    // n1.add_direct_successor(&n2, Void::default());
    // n2.add_direct_successor(&n3, Void::default());
    // n3.add_direct_predecessor(&n4, Void::default());
    // n4.add_direct_predecessor(&n1, Void::default());

    growth(&mut g, &root);

    println!("{}", g.to_dot());
    // println!("{:?}", root.get_direct_successor())
}

fn growth(tree: &mut Graph<NodeState, Edge>, node: &Node<NodeState, Edge>) {
    let mut around = [false, false, false, false];
    if let Ok(edges) = node.get_direct_successor().read() {
        for element in edges.values() {
            if let Ok(ref value) = element.value().read() {
                match **value {
                    Edge::Direction(Direction::North) => around[0] = true,
                    Edge::Direction(Direction::South) => around[1] = true,
                    Edge::Direction(Direction::East) => around[2] = true,
                    Edge::Direction(Direction::West) => around[3] = true,
                    _ => (),
                }
            }
        }
    }
    if !around[0] {
        let new_node = Node::new(NodeState::New);
        node.add_direct_successor(&new_node, Edge::Direction(Direction::North));
        tree.insert(new_node);
    }
    if !around[1] {
        let new_node = Node::new(NodeState::New);
        node.add_direct_successor(&new_node, Edge::Direction(Direction::South));
        tree.insert(new_node);
    }
    if !around[2] {
        let new_node = Node::new(NodeState::New);
        node.add_direct_successor(&new_node, Edge::Direction(Direction::East));
        tree.insert(new_node);
    }
    if !around[3] {
        let new_node = Node::new(NodeState::New);
        node.add_direct_successor(&new_node, Edge::Direction(Direction::West));
        tree.insert(new_node);
    }
    if let Ok(mut value) = node.value().write() {
        *value = NodeState::Old(String::from("done"));
    }
}
