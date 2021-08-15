use std::cell::RefCell;
use std::convert::AsRef;
use std::fmt::{self, Debug};
use std::rc::Rc;

pub type NodeRef<T, W> = Rc<RefCell<Node<T, W>>>;
pub type EdgeRef<T, W> = Rc<RefCell<Edge<T, W>>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Incoming,
    Outgoing,
    Both,
}

pub trait TEdge<T, W> {
    fn set_weight(&self, weight: W) -> EdgeRef<T, W>;
    fn set_direction(&self, direction: Direction) -> EdgeRef<T, W>;
    fn source(&self) -> NodeRef<T, W>;
    fn target(&self) -> NodeRef<T, W>;
    fn weight(&self) -> W;
    fn direction(&self) -> Direction;
}

pub struct Edge<T, W> {
    source: NodeRef<T, W>,
    target: NodeRef<T, W>,
    weight: W,
    direction: Direction,
}

impl<T, W> Edge<T, W> {
    pub fn new(
        source: NodeRef<T, W>,
        target: NodeRef<T, W>,
        weight: W,
        direction: Direction,
    ) -> Self {
        Edge {
            source,
            target,
            weight,
            direction,
        }
    }

    pub fn source(&self) -> NodeRef<T, W> {
        self.source.clone()
    }

    pub fn target(&self) -> NodeRef<T, W> {
        self.target.clone()
    }

    pub fn weight(&self) -> &W {
        &self.weight
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn new_ref(
        source: NodeRef<T, W>,
        target: NodeRef<T, W>,
        weight: W,
        direction: Direction,
    ) -> EdgeRef<T, W> {
        Rc::new(RefCell::new(Edge::new(source, target, weight, direction)))
    }

    pub fn set_weight(&mut self, weight: W) {
        self.weight = weight;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}

impl<T, W> Debug for Edge<T, W>
where
    T: PartialEq + Clone + Debug,
    W: Default + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}, {:?}, {:?}",
            self.target.val(),
            self.weight,
            self.direction
        )
    }
}

impl<T, W> TEdge<T, W> for EdgeRef<T, W>
where
    W: Clone,
{
    fn set_weight(&self, weight: W) -> EdgeRef<T, W> {
        self.borrow_mut().set_weight(weight);
        self.clone()
    }

    fn set_direction(&self, direction: Direction) -> EdgeRef<T, W> {
        self.borrow_mut().set_direction(direction);
        self.clone()
    }

    fn source(&self) -> NodeRef<T, W> {
        self.borrow().source.clone()
    }

    fn target(&self) -> NodeRef<T, W> {
        self.borrow().target.clone()
    }

    fn weight(&self) -> W {
        self.borrow().weight.clone()
    }

    fn direction(&self) -> Direction {
        self.borrow().direction.clone()
    }
}

pub trait TNode<T, W> {
    fn add_edge(&self, node: NodeRef<T, W>) -> EdgeRef<T, W>;
    fn val(&self) -> T;
    fn edges(&self) -> Vec<EdgeRef<T, W>>;
}

pub struct Node<T, W> {
    val: T,
    edges: Vec<EdgeRef<T, W>>,
}

impl<T, W> Node<T, W> {
    pub fn new(val: T) -> Self {
        Node { val, edges: vec![] }
    }

    pub fn val(&self) -> &T {
        &self.val
    }

    pub fn new_ref(val: T) -> NodeRef<T, W> {
        Rc::new(RefCell::new(Node::new(val)))
    }

    pub fn edges(&self) -> impl Iterator<Item = EdgeRef<T, W>> + '_ {
        self.edges.iter().map(Clone::clone)
    }
}

impl<T, W> TNode<T, W> for NodeRef<T, W>
where
    T: PartialEq + Clone,
    W: Default,
{
    fn add_edge(&self, target: NodeRef<T, W>) -> EdgeRef<T, W> {
        let edge = Edge::new_ref(self.clone(), target, Default::default(), Direction::Both);
        let existing = self.as_ref().borrow().find_edge(edge.clone());
        existing.unwrap_or_else(|| {
            self.as_ref().borrow_mut().edges.push(edge.clone());
            edge
        })
    }

    fn val(&self) -> T {
        self.as_ref().borrow().val().clone()
    }

    fn edges(&self) -> Vec<EdgeRef<T, W>> {
        self.as_ref().borrow().edges().collect()
    }
}

impl<T, W> AsRef<T> for Node<T, W> {
    fn as_ref(&self) -> &T {
        &self.val
    }
}

impl<T, W> Node<T, W>
where
    T: PartialEq + Clone,
    W: Default,
{
    pub fn find_edge(&self, edge: EdgeRef<T, W>) -> Option<EdgeRef<T, W>> {
        self.edges
            .iter()
            .find(|e| e.as_ref().borrow().target.val() == edge.as_ref().borrow().target.val())
            .map(Clone::clone)
    }
}

pub struct Graph<T, W> {
    name: String,
    nodes: Vec<NodeRef<T, W>>,
}

impl<T, W> Graph<T, W> {
    pub fn new(name: String) -> Self {
        Graph {
            name,
            nodes: vec![],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_node(&mut self, val: T) -> NodeRef<T, W> {
        let node = Node::new_ref(val);
        self.nodes.push(node.clone());
        node
    }

    pub fn nodes(&self) -> impl Iterator<Item = NodeRef<T, W>> + '_ {
        self.nodes.iter().map(Clone::clone)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl<T, W> Graph<T, W>
where
    T: PartialEq + Clone,
    W: Default,
{
    pub fn find_node(&self, val: T) -> Option<NodeRef<T, W>> {
        self.nodes.iter().find(|n| n.val() == val).map(Clone::clone)
    }
}

impl<T, W> Debug for Graph<T, W>
where
    T: PartialEq + Clone + Debug,
    W: Debug + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for n in self.nodes.iter() {
            write!(f, "{:?} -> [", n.val())?;
            for (i, e) in n.as_ref().borrow().edges().enumerate() {
                let comma = if i == 0 { "" } else { ", " };
                write!(f, "{}({:?})", comma, e.as_ref().borrow())?;
            }
            writeln!(f, "]")?;
        }

        Ok(())
    }
}
