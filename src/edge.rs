use std::fmt;
use std::fmt::Display;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;
use std::fmt::Write;
use crate::node::{Node, WeakNode};



#[derive(Clone)]
pub struct Edge<N, E>
where
    N: Clone,
    E: Clone,
{

    internal: Arc<(Uuid, RwLock<E>, WeakNode<N, E>)>,
}


impl<N, E> Edge<N, E>
where
    N: Clone + Display,
    E: Clone + Display,
{
    pub fn new(value: E, node: WeakNode<N, E>) -> Self {
        Edge {
            internal: Arc::new((Uuid::new_v4(), RwLock::new(value), node)),
        }
    }
    pub fn key(&self) -> Uuid {
        self.internal.0
    }
    pub fn value(&self) -> &RwLock<E> {
        &self.internal.1
    }
}
impl<N, E> Display for Edge<N,E>
where
    N: Clone + Display,
    E: Clone + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value().read().unwrap())
    }
}
