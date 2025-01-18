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


use crate::graph::Graph;
use std::fmt;
use std::fmt::Display;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;

mod graph;
mod edge;
mod node;



use crate::node::Node;




// #[derive(Clone)]
// pub enum Edge {
//     None,
//     Direction(Direction),
// }
// #[derive(Clone)]
// pub enum Direction {
//     North,
//     South,
//     East,
//     West,
// }
// impl Display for Direction {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Direction::North => write!(f, "North"),
//             Direction::South => write!(f, "South"),
//             Direction::East => write!(f, "East"),
//             Direction::West => write!(f, "West"),
//         }
//     }
// }
//
// impl Display for Edge {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Edge::None => write!(f, ""),
//             Edge::Direction(d) => d.fmt(f),
//         }
//     }
// }

#[derive(Clone)]
pub struct Void {}

impl Display for Void {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

fn main() {
    let mut g: Graph<String, Void> = Graph::new();

    let n1 = Node::new(String::from("N1"));
    let n2 = Node::new(String::from("N2"));
    let n3 = Node::new(String::from("N3"));
    let n4 = Node::new(String::from("N4"));

    g.insert(n1.clone());
    g.insert(n2.clone());
    g.insert(n3.clone());
    g.insert(n4.clone());

    //
    // n1.add_next(n2.clone(), Edge::Direction(Direction::West));
    // n1.add_previous(n2.clone(), Edge::Direction(Direction::East));

    // n1.add_next(n3.clone());
    // n2.add_previous(n4.clone());
    // n3.add_next(n1.clone());
    // n4.add_previous(n4.clone());

    // println!("{}", g.to_dot());
}
