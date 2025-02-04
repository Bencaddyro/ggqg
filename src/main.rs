// Rewrite Engine
// - [ ] Rewrite node based on rule
// - [ ] Define rule

// Cleanup direction & growth
// StateNode : Place(Border/Generated)
// Cleanup rewrite border path (vec ? or hashmap ?)
// growth with a pattern (aka radiux problem)

// Quest engine ?
// rule -> waht format ? graph also ?

use crate::files::write_to_file;
use crate::graph::Graph;
use std::fmt::{Display, Formatter, Result};
use std::{io, thread, time};

mod edge;
mod files;
mod graph;
mod node;
mod quest;
use crate::node::Node;

#[derive(Clone, Debug)]
pub enum Edge {
    None,
    Direction(Direction),
    Proximity(Vec<Direction>),
}
#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    NorthSouth(bool),
    // South,
    EastWest(bool),
    // West,
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Edge::None => write!(f, ""),
            Edge::Proximity(p) => {
                write!(f, "[")?;
                for e in p {
                    write!(f, "{e},")?
                }
                write!(f, "]")
            }
            Edge::Direction(d) => write!(f, "{d}"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Direction::NorthSouth(true) => write!(f, "North"),
            Direction::NorthSouth(false) => write!(f, "South"),
            Direction::EastWest(true) => write!(f, "East"),
            Direction::EastWest(false) => write!(f, "West"),
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

fn normalize(path: &mut Vec<Direction>) {
    if path.contains(&Direction::NorthSouth(true)) && path.contains(&Direction::NorthSouth(false)) {
        if let Some(pos) = path.iter().position(|x| *x == Direction::NorthSouth(true)) {
            path.remove(pos);
        }
        if let Some(pos) = path.iter().position(|x| *x == Direction::NorthSouth(false)) {
            path.remove(pos);
        }
    }
    if path.contains(&Direction::EastWest(true)) && path.contains(&Direction::EastWest(false)) {
        if let Some(pos) = path.iter().position(|x| *x == Direction::EastWest(true)) {
            path.remove(pos);
        }
        if let Some(pos) = path.iter().position(|x| *x == Direction::EastWest(false)) {
            path.remove(pos);
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
            "z" => current_node = get_next(current_node, Direction::NorthSouth(true)).unwrap(),
            "q" => current_node = get_next(current_node, Direction::EastWest(false)).unwrap(),
            "s" => current_node = get_next(current_node, Direction::NorthSouth(false)).unwrap(),
            "d" => current_node = get_next(current_node, Direction::EastWest(true)).unwrap(),
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

    let mut border = [None, None, None, None]; // new border

    // get neighbours
    let mut around = [false, false, false, false];
    if let Ok(edges) = node.get_direct_predecessor().read() {
        for edge in edges.values() {
            if let Ok(ref value) = edge.value().read() {
                match &**value {
                    Edge::Direction(Direction::NorthSouth(true)) => {
                        node.add_direct_successor(
                            &edge.tail().read().unwrap().upgrade().unwrap(),
                            Edge::Direction(Direction::NorthSouth(false)),
                        );
                        around[0] = true
                    }
                    Edge::Direction(Direction::NorthSouth(false)) => {
                        node.add_direct_successor(
                            &edge.tail().read().unwrap().upgrade().unwrap(),
                            Edge::Direction(Direction::NorthSouth(true)),
                        );
                        around[1] = true
                    }
                    Edge::Direction(Direction::EastWest(true)) => {
                        node.add_direct_successor(
                            &edge.tail().read().unwrap().upgrade().unwrap(),
                            Edge::Direction(Direction::EastWest(false)),
                        );
                        around[2] = true
                    }
                    Edge::Direction(Direction::EastWest(false)) => {
                        node.add_direct_successor(
                            &edge.tail().read().unwrap().upgrade().unwrap(),
                            Edge::Direction(Direction::EastWest(true)),
                        );
                        around[3] = true
                    }
                    Edge::Proximity(p) => {
                        if p.len() == 1 {
                            match p[0] {
                                Direction::NorthSouth(true) => {
                                    node.add_direct_successor(
                                        &edge.tail().read().unwrap().upgrade().unwrap(),
                                        Edge::Direction(Direction::NorthSouth(false)),
                                    );
                                    around[0] = true
                                }
                                Direction::NorthSouth(false) => {
                                    node.add_direct_successor(
                                        &edge.tail().read().unwrap().upgrade().unwrap(),
                                        Edge::Direction(Direction::NorthSouth(true)),
                                    );
                                    around[1] = true
                                }
                                Direction::EastWest(true) => {
                                    node.add_direct_successor(
                                        &edge.tail().read().unwrap().upgrade().unwrap(),
                                        Edge::Direction(Direction::EastWest(false)),
                                    );
                                    around[2] = true
                                }
                                Direction::EastWest(false) => {
                                    node.add_direct_successor(
                                        &edge.tail().read().unwrap().upgrade().unwrap(),
                                        Edge::Direction(Direction::EastWest(true)),
                                    );
                                    around[3] = true
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    // create neighbours if need
    if !around[0] {
        let new_node = Node::new(NodeState::Border);
        node.add_direct_successor(&new_node, Edge::Direction(Direction::NorthSouth(false)));
        border[0] = Some(new_node.clone());
        tree.insert(new_node);
    }
    if !around[1] {
        let new_node = Node::new(NodeState::Border);
        node.add_direct_successor(&new_node, Edge::Direction(Direction::NorthSouth(true)));
        border[1] = Some(new_node.clone());
        tree.insert(new_node);
    }
    if !around[2] {
        let new_node = Node::new(NodeState::Border);
        node.add_direct_successor(&new_node, Edge::Direction(Direction::EastWest(false)));
        border[2] = Some(new_node.clone());
        tree.insert(new_node);
    }
    if !around[3] {
        let new_node = Node::new(NodeState::Border);
        node.add_direct_successor(&new_node, Edge::Direction(Direction::EastWest(true)));
        border[3] = Some(new_node.clone());
        tree.insert(new_node);
    }
    if let Ok(mut value) = node.value().write() {
        *value = NodeState::D(node.key().to_string()[..6].to_string());
    }

    // link neighbours to proximity in NxN pattern
    if let (Some(bnode_a), Some(bnode_b)) = (border[0].clone(), border[1].clone()) {
        bnode_a.add_direct_successor(
            &bnode_b,
            Edge::Proximity(vec![
                Direction::NorthSouth(true),
                Direction::NorthSouth(true),
            ]),
        );
        bnode_b.add_direct_successor(
            &bnode_a,
            Edge::Proximity(vec![
                Direction::NorthSouth(false),
                Direction::NorthSouth(false),
            ]),
        );
    }
    if let (Some(bnode_a), Some(bnode_b)) = (border[0].clone(), border[2].clone()) {
        bnode_a.add_direct_successor(
            &bnode_b,
            Edge::Proximity(vec![
                Direction::NorthSouth(true),
                Direction::EastWest(false),
            ]),
        );
        bnode_b.add_direct_successor(
            &bnode_a,
            Edge::Proximity(vec![
                Direction::NorthSouth(false),
                Direction::EastWest(true),
            ]),
        );
    }
    if let (Some(bnode_a), Some(bnode_b)) = (border[0].clone(), border[3].clone()) {
        bnode_a.add_direct_successor(
            &bnode_b,
            Edge::Proximity(vec![Direction::NorthSouth(true), Direction::EastWest(true)]),
        );
        bnode_b.add_direct_successor(
            &bnode_a,
            Edge::Proximity(vec![
                Direction::NorthSouth(false),
                Direction::EastWest(false),
            ]),
        );
    }
    if let (Some(bnode_a), Some(bnode_b)) = (border[1].clone(), border[2].clone()) {
        bnode_a.add_direct_successor(
            &bnode_b,
            Edge::Proximity(vec![
                Direction::NorthSouth(false),
                Direction::EastWest(false),
            ]),
        );
        bnode_b.add_direct_successor(
            &bnode_a,
            Edge::Proximity(vec![Direction::NorthSouth(true), Direction::EastWest(true)]),
        );
    }
    if let (Some(bnode_a), Some(bnode_b)) = (border[1].clone(), border[3].clone()) {
        bnode_a.add_direct_successor(
            &bnode_b,
            Edge::Proximity(vec![
                Direction::NorthSouth(false),
                Direction::EastWest(true),
            ]),
        );
        bnode_b.add_direct_successor(
            &bnode_a,
            Edge::Proximity(vec![
                Direction::NorthSouth(true),
                Direction::EastWest(false),
            ]),
        );
    }
    if let (Some(bnode_a), Some(bnode_b)) = (border[2].clone(), border[3].clone()) {
        bnode_a.add_direct_successor(
            &bnode_b,
            Edge::Proximity(vec![Direction::EastWest(true), Direction::EastWest(true)]),
        );
        bnode_b.add_direct_successor(
            &bnode_a,
            Edge::Proximity(vec![Direction::EastWest(false), Direction::EastWest(false)]),
        );
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

        let basepath = if let Ok(edge) = edge.value().read() {
            match &*edge {
                Edge::Proximity(p) => p.clone(),
                _ => vec![],
            }
        } else {
            vec![]
        };

        for (i, bnode) in border.iter().enumerate() {
            let mut path = basepath.clone();
            match i {
                0 => path.push(Direction::NorthSouth(true)),
                1 => path.push(Direction::NorthSouth(false)),
                2 => path.push(Direction::EastWest(true)),
                3 => path.push(Direction::EastWest(false)),
                _ => (),
            }
            normalize(&mut path);

            if let Some(bnode) = bnode {
                edge.head()
                    .read()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .add_direct_predecessor(&bnode, Edge::Proximity(path))
            }
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
        let basepath = if let Ok(edge) = edge.value().read() {
            match &*edge {
                Edge::Proximity(p) => p.clone(),
                _ => vec![],
            }
        } else {
            vec![]
        };
        for (i, bnode) in border.iter().enumerate() {
            let mut path = basepath.clone();
            match i {
                0 => path.push(Direction::NorthSouth(false)),
                1 => path.push(Direction::NorthSouth(true)),
                2 => path.push(Direction::EastWest(false)),
                3 => path.push(Direction::EastWest(true)),
                _ => (),
            }
            normalize(&mut path);

            if let Some(bnode) = bnode {
                edge.tail()
                    .read()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .add_direct_successor(&bnode, Edge::Proximity(path))
            }
        }
    }
}
