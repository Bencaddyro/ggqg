use crate::node::WeakNode;
use std::fmt::{Display, Formatter, Result};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

type ArcEdge<N, E> = Arc<(
    Uuid,
    RwLock<E>,
    RwLock<WeakNode<N, E>>,
    RwLock<WeakNode<N, E>>,
)>;
#[derive(Clone, Debug)]
pub struct Edge<N, E>
where
    N: Clone,
    E: Clone,
{
    internal: ArcEdge<N, E>,
}
impl<N, E> Edge<N, E>
where
    N: Clone + Display,
    E: Clone + Display,
{
    pub fn new(value: E, head: WeakNode<N, E>, tail: WeakNode<N, E>) -> Self {
        Edge {
            internal: Arc::new((
                Uuid::new_v4(),
                RwLock::new(value),
                RwLock::new(head),
                RwLock::new(tail),
            )),
        }
    }
    pub fn key(&self) -> Uuid {
        self.internal.0
    }
    pub fn value(&self) -> &RwLock<E> {
        &self.internal.1
    }
    pub fn tail(&self) -> &RwLock<WeakNode<N, E>> {
        &self.internal.2
    }
    pub fn head(&self) -> &RwLock<WeakNode<N, E>> {
        &self.internal.3
    }
}
impl<N, E> Display for Edge<N, E>
where
    N: Clone + Display,
    E: Clone + Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.value().read().unwrap())
    }
}
