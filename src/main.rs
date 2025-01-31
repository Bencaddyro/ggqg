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

use crate::files::write_to_file;
use crate::graph::Graph;
use std::fmt::{Display, Formatter, Result};
use std::{io, thread, time};

mod edge;
mod files;
mod graph;
mod node;
use crate::node::Node;

#[derive(Clone, Debug)]
pub enum Edge {
    None,
    Direction(Direction),
    Proximity(String),
}
#[derive(PartialEq, Clone, Debug)]
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
            Edge::Proximity(_) => write!(f, ""),
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
    Border,
    D(String),
}

impl Display for NodeState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            NodeState::New => write!(f, "New"),
            NodeState::Border => write!(f, "B"),
            NodeState::D(value) => write!(f, "D({value})",),
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
    let mut current_node = Node::new(NodeState::Border);
    g.insert(current_node.clone());

    // TODO prompt to be improve with https://github.com/mikaelmello/inquire crate
    println!("ggqg v 0.0");
    println!("x to quit, zqsd to move");
    let mut input_string = String::new();

    loop {
        growth(&mut g, &current_node);
        write_to_file(&g.to_dot(), None).unwrap();
        write_to_file(&g.to_dot(), Some("live")).unwrap();

        println!("You are in {}", current_node.value().read().unwrap());
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        match input_string.trim() {
            "x" => break,
            "z" => current_node = get_next(current_node, Direction::North).unwrap(),
            "q" => current_node = get_next(current_node, Direction::West).unwrap(),
            "s" => current_node = get_next(current_node, Direction::South).unwrap(),
            "d" => current_node = get_next(current_node, Direction::East).unwrap(),
            _ => println!("x to quit, zqsd to move"),
        }
    }
}

fn get_next(node: Node<NodeState, Edge>, direction: Direction) -> Option<Node<NodeState, Edge>> {
    for elem in (*node.get_direct_successor().read().unwrap()).values() {
        if let Edge::Direction(d) = &*elem.value().read().unwrap() {
            if d == &direction {
                return Some(elem.head().read().unwrap().upgrade().unwrap());
            }
        }
    }
    None
}
/*
fn generate_map_wrapped(
    tree: &mut Graph<NodeState, Edge>,
    node: &Node<NodeState, Edge>,
    size: u32,
    current: u32,
) {
    if current > size {
        return;
    }
    match *node.value().read().unwrap() {
        NodeState::New => (),
        NodeState::D(_) => return,
    }

    let mut around = [false, false, false, false];
    if let Ok(edges) = node.get_direct_predecessor().read() {
        for edge in edges.values() {
            if let Ok(ref value) = edge.value().read() {
                match **value {
                    Edge::Direction(Direction::North) => {
                        node.add_direct_successor(
                            &edge.tail().read().unwrap().upgrade().unwrap(),
                            Edge::Direction(Direction::South),
                        );
                        around[0] = true
                    }
                    Edge::Direction(Direction::South) => {
                        node.add_direct_successor(
                            &edge.tail().read().unwrap().upgrade().unwrap(),
                            Edge::Direction(Direction::North),
                        );
                        around[1] = true
                    }
                    Edge::Direction(Direction::East) => {
                        node.add_direct_successor(
                            &edge.tail().read().unwrap().upgrade().unwrap(),
                            Edge::Direction(Direction::West),
                        );
                        around[2] = true
                    }
                    Edge::Direction(Direction::West) => {
                        node.add_direct_successor(
                            &edge.tail().read().unwrap().upgrade().unwrap(),
                            Edge::Direction(Direction::East),
                        );
                        around[3] = true
                    }
                    _ => (),
                }
            }
        }
    }
    if let Ok(mut value) = node.value().write() {
        *value = NodeState::D(node.key().to_string()[..6].to_string());
    }
    if !around[0] {
        let new_node = Node::new(NodeState::New);
        node.add_direct_successor(&new_node, Edge::Direction(Direction::South));
        tree.insert(new_node.clone());
        generate_map_wrapped(tree, &new_node, size, current + 1);
    }
    // if !around[1] {
    //     let new_node = Node::new(NodeState::New);
    //     node.add_direct_successor(&new_node, Edge::Direction(Direction::North));
    //     tree.insert(new_node.clone());
    //     generate_map_wrapped(tree,&new_node, size, current+1);
    // }
    if !around[2] {
        let new_node = Node::new(NodeState::New);
        node.add_direct_successor(&new_node, Edge::Direction(Direction::West));
        tree.insert(new_node.clone());
        generate_map_wrapped(tree, &new_node, size, current + 1);
    }
    // if !around[3] {
    //     let new_node = Node::new(NodeState::New);
    //     node.add_direct_successor(&new_node, Edge::Direction(Direction::East));
    //     tree.insert(new_node.clone());
    //     generate_map_wrapped(tree,&new_node, size, current+1);
    // }
}*/

fn growth(tree: &mut Graph<NodeState, Edge>, node: &Node<NodeState, Edge>) {
    match *node.value().read().unwrap() {
        NodeState::Border => (),
        _ => return,
    }

    let mut border = vec![]; // new border

    // get neighbours
    let mut around = [false, false, false, false];
    if let Ok(edges) = node.get_direct_predecessor().read() {
        for edge in edges.values() {
            if let Ok(ref value) = edge.value().read() {
                match **value {
                    Edge::Direction(Direction::North) => {
                        node.add_direct_successor(
                            &edge.tail().read().unwrap().upgrade().unwrap(),
                            Edge::Direction(Direction::South),
                        );
                        around[0] = true
                    }
                    Edge::Direction(Direction::South) => {
                        node.add_direct_successor(
                            &edge.tail().read().unwrap().upgrade().unwrap(),
                            Edge::Direction(Direction::North),
                        );
                        around[1] = true
                    }
                    Edge::Direction(Direction::East) => {
                        node.add_direct_successor(
                            &edge.tail().read().unwrap().upgrade().unwrap(),
                            Edge::Direction(Direction::West),
                        );
                        around[2] = true
                    }
                    Edge::Direction(Direction::West) => {
                        node.add_direct_successor(
                            &edge.tail().read().unwrap().upgrade().unwrap(),
                            Edge::Direction(Direction::East),
                        );
                        around[3] = true
                    }
                    _ => (),
                }
            }
        }
    }

    // create neighbours if need
    if !around[0] {
        let new_node = Node::new(NodeState::Border);
        node.add_direct_successor(&new_node, Edge::Direction(Direction::South));
        border.push(new_node.clone());
        tree.insert(new_node);
    }
    if !around[1] {
        let new_node = Node::new(NodeState::Border);
        node.add_direct_successor(&new_node, Edge::Direction(Direction::North));
        border.push(new_node.clone());
        tree.insert(new_node);
    }
    if !around[2] {
        let new_node = Node::new(NodeState::Border);
        node.add_direct_successor(&new_node, Edge::Direction(Direction::West));
        border.push(new_node.clone());
        tree.insert(new_node);
    }
    if !around[3] {
        let new_node = Node::new(NodeState::Border);
        node.add_direct_successor(&new_node, Edge::Direction(Direction::East));
        border.push(new_node.clone());
        tree.insert(new_node);
    }
    if let Ok(mut value) = node.value().write() {
        *value = NodeState::D(node.key().to_string()[..6].to_string());
    }

    // link neighbours to proximity in NxN pattern
    for i in 0..border.len() {
        for j in i + 1..border.len() {
            border[i].add_direct_successor(&border[j], Edge::Proximity(String::new()));
            border[j].add_direct_successor(&border[i], Edge::Proximity(String::new()));
        }
    }

    // move promiximity of current to new Border
    let mut remove_head = vec![];
    let mut remove_tail = vec![];

    for edge in node.get_direct_successor().read().unwrap().values() {
        match *edge.value().read().unwrap() {
            Edge::Proximity(_) => remove_head.push(edge.clone()),
            _ => (),
        }
    }

    for edge in node.get_direct_predecessor().read().unwrap().values() {
        match *edge.value().read().unwrap() {
            Edge::Proximity(_) => remove_tail.push(edge.clone()),
            _ => (),
        }
    }

    // println!("Remove head edge");

    for edge in remove_head {
        node.remove_edge(edge.clone());
        edge.head()
            .read()
            .unwrap()
            .upgrade()
            .unwrap()
            .remove_edge(edge.clone());

        for bnode in &border {
            edge.head()
                .read()
                .unwrap()
                .upgrade()
                .unwrap()
                .add_direct_predecessor(&bnode, Edge::Proximity(String::new()))
        }
    }
    // println!("Remove tail edge");

    for edge in remove_tail {
        node.remove_edge(edge.clone());
        edge.tail()
            .read()
            .unwrap()
            .upgrade()
            .unwrap()
            .remove_edge(edge.clone());

        for bnode in &border {
            edge.tail()
                .read()
                .unwrap()
                .upgrade()
                .unwrap()
                .add_direct_successor(&bnode, Edge::Proximity(String::new()))
        }
    }
}
