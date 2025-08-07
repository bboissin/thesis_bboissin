use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub id: usize,
}

pub struct CfgGraph {
    pub root: Node,
    pub nodes: Vec<Node>,
    pub edges: HashMap<Node, Vec<Node>>,
}

pub struct VisitOrder {
    pub pre: HashMap<Node, usize>,
    pub post: HashMap<Node, usize>,
    pub spanning_tree: HashMap<Node, Node>,
}

pub fn dfs_search(graph: &CfgGraph) -> VisitOrder {
    let mut pre = HashMap::new();
    let mut post = HashMap::new();
    let mut spanning_tree = HashMap::new();
    let mut visited = HashSet::new();
    let mut time = 0;

    fn dfs(node: &Node, graph: &CfgGraph, visited: &mut HashSet<Node>, pre: &mut HashMap<Node, usize>, post: &mut HashMap<Node, usize>, spanning_tree: &mut HashMap<Node, Node>, time: &mut usize) {
        if !visited.insert(node.clone()) {
            return;
        }
        pre.insert(node.clone(), *time);
        *time += 1;

        if let Some(successors) = graph.edges.get(node) {
            for successor in successors {
                if !visited.contains(successor) {
                    spanning_tree.insert(node.clone(), successor.clone());
                    dfs(successor, graph, visited, pre, post, spanning_tree, time);
                }
            }
        }

        post.insert(node.clone(), *time);
    }

    dfs(&graph.root, graph, &mut visited, &mut pre, &mut post, &mut spanning_tree, &mut time);

    VisitOrder { pre, post , spanning_tree}
}