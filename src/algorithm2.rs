use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    let mut pre_time = 0;
    let mut post_time = 0;

    fn dfs(
        node: &Node,
        graph: &CfgGraph,
        visited: &mut HashSet<Node>,
        pre: &mut HashMap<Node, usize>,
        post: &mut HashMap<Node, usize>,
        spanning_tree: &mut HashMap<Node, Node>,
        pre_time: &mut usize,
        post_time: &mut usize,
    ) {
        if !visited.insert(*node) {
            return;
        }
        pre.insert(*node, *pre_time);
        *pre_time += 1;

        if let Some(successors) = graph.edges.get(node) {
            for successor in successors {
                if visited.contains(successor) {
                    continue;
                }
                spanning_tree.insert(*node, *successor);
                dfs(
                    successor,
                    graph,
                    visited,
                    pre,
                    post,
                    spanning_tree,
                    pre_time,
                    post_time,
                );
            }
        }

        post.insert(*node, *post_time);
        *post_time += 1;
    }

    dfs(
        &graph.root,
        graph,
        &mut visited,
        &mut pre,
        &mut post,
        &mut spanning_tree,
        &mut pre_time,
        &mut post_time,
    );

    VisitOrder {
        pre,
        post,
        spanning_tree,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_search() {
        let a = Node { id: 0 };
        let b = Node { id: 1 };
        let c = Node { id: 2 };
        let d = Node { id: 3 };
        let graph = CfgGraph {
            root: a,
            nodes: vec![a, b, c, d],
            edges: HashMap::from([(a, vec![b]), (b, vec![c]), (c, vec![b]), (c, vec![d])]),
        };
        let order = dfs_search(&graph);
        assert_eq!(order.pre, HashMap::from([(a, 0), (b, 1), (c, 2), (d, 3),]));
        assert_eq!(order.post, HashMap::from([(a, 3), (b, 2), (c, 1), (d, 0),]));
        assert_eq!(
            order.spanning_tree,
            HashMap::from([(a, b), (b, c), (c, d),])
        );
    }
}
