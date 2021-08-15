pub mod graph;

use std::borrow::Cow;
use std::fmt::{Debug, Display};
use std::io::{self, BufRead};

use dot::LabelText;
use graph::{Direction, EdgeRef, Graph, NodeRef, TEdge, TNode};

type Node = NodeRef<String, usize>;

fn visit_node(
    graph: &Graph<String, usize>,
    current: Node,
    cost: usize,
    path: &mut Vec<Node>,
    paths: &mut Vec<(Vec<Node>, usize)>,
) {
    // push current node into path
    path.push(current.clone());

    let edges = current.edges();
    for edge in edges {
        let target = edge.target();
        let target_val = target.val();
        if path.iter().find(|n| n.val() == target_val).is_none() {
            visit_node(graph, target, cost + edge.weight(), path, paths);
        }
    }

    if path.len() == graph.len() {
        paths.push((path.clone(), cost));
    }

    path.pop();
}

fn visit(graph: &Graph<String, usize>) -> Vec<(Vec<Node>, usize)> {
    let mut path = vec![];
    let mut paths = vec![];

    for node in graph.nodes() {
        visit_node(graph, node, 0, &mut path, &mut paths);
    }

    paths
}

fn main() {
    let mut graph = Graph::new("day09".to_string());
    let stdin = io::stdin();

    // build the graph
    for line in stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
    {
        let tokens = line.split(" = ").collect::<Vec<_>>();
        let edge_spec = tokens[0];
        let weight = usize::from_str_radix(tokens[1], 10).unwrap();
        let tokens = edge_spec.split(' ').collect::<Vec<_>>();
        let from = tokens[0].to_string();
        let to = tokens[2].to_string();

        let from = graph
            .find_node(from.clone())
            .unwrap_or_else(|| graph.add_node(from));
        let to = graph
            .find_node(to.clone())
            .unwrap_or_else(|| graph.add_node(to));

        let edge = from.add_edge(to.clone());
        edge.set_weight(weight).set_direction(Direction::Outgoing);

        let edge = to.add_edge(from);
        edge.set_weight(weight).set_direction(Direction::Outgoing);
    }

    let mut stdout = io::stdout();
    dot::render(&graph, &mut stdout).unwrap();

    let paths = visit(&graph);
    for (p, cost) in paths.iter() {
        println!(
            "{} = {}",
            p.iter().map(|n| n.val()).collect::<Vec<_>>().join(" -> "),
            cost
        );
    }

    let min_cost = paths.iter().map(|(_, cost)| cost).min();
    let max_cost = paths.iter().map(|(_, cost)| cost).max();
    println!("Min: {:?}, Max: {:?}", min_cost, max_cost);
}

impl<'a, T, W> dot::Labeller<'a, NodeRef<T, W>, EdgeRef<T, W>> for Graph<T, W>
where
    T: Debug + Display,
    W: Debug + Display,
{
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new(self.name()).unwrap()
    }

    fn node_id(&'a self, n: &NodeRef<T, W>) -> dot::Id<'a> {
        dot::Id::new(format!("{}", n.as_ref().borrow().val())).unwrap()
    }

    fn edge_label(&'a self, e: &EdgeRef<T, W>) -> dot::LabelText<'a> {
        LabelText::LabelStr(Cow::from(format!("{}", e.as_ref().borrow().weight())))
    }
}

impl<'a, T, W> dot::GraphWalk<'a, NodeRef<T, W>, EdgeRef<T, W>> for Graph<T, W> {
    fn nodes(&'a self) -> dot::Nodes<'a, NodeRef<T, W>> {
        self.nodes().collect()
    }

    fn edges(&'a self) -> dot::Edges<'a, EdgeRef<T, W>> {
        let mut edges = vec![];
        for node in self.nodes() {
            edges.extend(node.as_ref().borrow().edges());
        }

        edges.into()
    }

    fn source(&'a self, edge: &EdgeRef<T, W>) -> NodeRef<T, W> {
        edge.as_ref().borrow().source()
    }

    fn target(&'a self, edge: &EdgeRef<T, W>) -> NodeRef<T, W> {
        edge.as_ref().borrow().target()
    }
}
