use crate::node::{Node, WeakNode};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Write;
use std::hash::Hash;
use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;

pub struct Edge<N, E>
where
    N: Clone,
    E: Clone,
{
    internal: Arc<(RwLock<E>, RwLock<WeakNode<N, E>>, RwLock<WeakNode<N, E>>)>,
}
impl<N, E> Edge<N, E>
where
    N: Clone + Display,
    E: Clone + Display,
{
    pub fn new(value: E, head: WeakNode<N, E>, tail: WeakNode<N, E>) -> Self {
        Edge {
            internal: Arc::new((RwLock::new(value), RwLock::new(head), RwLock::new(tail))),
        }
    }
    pub fn value(&self) -> &RwLock<E> {
        &self.internal.0
    }
    pub fn head(&self) -> &RwLock<WeakNode<N, E>> {
        &self.internal.1
    }
    pub fn tail(&self) -> &RwLock<WeakNode<N, E>> {
        &self.internal.2
    }
}
impl<N, E> Display for Edge<N, E>
where
    N: Clone + Display,
    E: Clone + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value().read().unwrap())
    }
}
