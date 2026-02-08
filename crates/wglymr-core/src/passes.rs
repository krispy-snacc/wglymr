use std::collections::{HashMap, HashSet, VecDeque};

use crate::{Graph, GraphError, NodeId};

pub fn detect_cycles(graph: &Graph) -> Vec<Vec<NodeId>> {
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();
    let mut cycles = Vec::new();

    for node_id in graph.node_ids() {
        if !visited.contains(&node_id) {
            let mut path = Vec::new();
            dfs_detect_cycle(
                graph,
                node_id,
                &mut visited,
                &mut rec_stack,
                &mut path,
                &mut cycles,
            );
        }
    }

    cycles
}

fn dfs_detect_cycle(
    graph: &Graph,
    node_id: NodeId,
    visited: &mut HashSet<NodeId>,
    rec_stack: &mut HashSet<NodeId>,
    path: &mut Vec<NodeId>,
    cycles: &mut Vec<Vec<NodeId>>,
) {
    visited.insert(node_id);
    rec_stack.insert(node_id);
    path.push(node_id);

    if let Some(node) = graph.node(node_id) {
        for input_socket in &node.inputs {
            for link in graph.links_into(*input_socket) {
                if let Some(from_socket) = graph.socket(link.from) {
                    let upstream_node = from_socket.node;

                    if rec_stack.contains(&upstream_node) {
                        if let Some(cycle_start) = path.iter().position(|&n| n == upstream_node) {
                            let cycle = path[cycle_start..].to_vec();
                            if !cycles.iter().any(|c| cycles_equal(c, &cycle)) {
                                cycles.push(cycle);
                            }
                        }
                    } else if !visited.contains(&upstream_node) {
                        dfs_detect_cycle(graph, upstream_node, visited, rec_stack, path, cycles);
                    }
                }
            }
        }
    }

    path.pop();
    rec_stack.remove(&node_id);
}

fn cycles_equal(a: &[NodeId], b: &[NodeId]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    if a.is_empty() {
        return true;
    }

    let start = b.iter().position(|&n| n == a[0]);
    if let Some(start_pos) = start {
        for i in 0..a.len() {
            if a[i] != b[(start_pos + i) % b.len()] {
                return false;
            }
        }
        true
    } else {
        false
    }
}

pub fn topological_sort(graph: &Graph) -> Result<Vec<NodeId>, GraphError> {
    if !detect_cycles(graph).is_empty() {
        return Err(GraphError::CycleDetected);
    }

    let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
    let mut adjacency: HashMap<NodeId, Vec<NodeId>> = HashMap::new();

    for node_id in graph.node_ids() {
        in_degree.insert(node_id, 0);
        adjacency.insert(node_id, Vec::new());
    }

    for node in graph.nodes() {
        for input_socket in &node.inputs {
            for link in graph.links_into(*input_socket) {
                if let Some(from_socket) = graph.socket(link.from) {
                    let from_node = from_socket.node;
                    adjacency.entry(from_node).or_default().push(node.id);
                    *in_degree.entry(node.id).or_insert(0) += 1;
                }
            }
        }
    }

    let mut queue: VecDeque<NodeId> = in_degree
        .iter()
        .filter_map(|(&node, &degree)| if degree == 0 { Some(node) } else { None })
        .collect();

    let mut result = Vec::new();

    while let Some(node) = queue.pop_front() {
        result.push(node);

        if let Some(neighbors) = adjacency.get(&node) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    Ok(result)
}

pub fn reachable_from(graph: &Graph, roots: &[NodeId]) -> HashSet<NodeId> {
    let mut reachable = HashSet::new();
    let mut stack = roots.to_vec();

    while let Some(node_id) = stack.pop() {
        if reachable.insert(node_id) {
            if let Some(node) = graph.node(node_id) {
                for input_socket in &node.inputs {
                    for link in graph.links_into(*input_socket) {
                        if let Some(from_socket) = graph.socket(link.from) {
                            let upstream_node = from_socket.node;
                            if !reachable.contains(&upstream_node) {
                                stack.push(upstream_node);
                            }
                        }
                    }
                }
            }
        }
    }

    reachable
}

#[derive(Debug)]
pub struct GraphView<'a> {
    pub graph: &'a Graph,
    pub roots: Vec<NodeId>,
    pub reachable: HashSet<NodeId>,
    pub topo_order: Vec<NodeId>,
}

pub fn build_graph_view<'a>(
    graph: &'a Graph,
    roots: &[NodeId],
) -> Result<GraphView<'a>, GraphError> {
    // Validate that all root NodeIds exist
    for &root in roots {
        if graph.node(root).is_none() {
            return Err(GraphError::NodeNotFound { node: root });
        }
    }

    let cycles = detect_cycles(graph);
    if !cycles.is_empty() {
        return Err(GraphError::CycleDetected);
    }

    let topo_order = topological_sort(graph)?;
    let reachable = reachable_from(graph, roots);

    Ok(GraphView {
        graph,
        roots: roots.to_vec(),
        reachable,
        topo_order,
    })
}
