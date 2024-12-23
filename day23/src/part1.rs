use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

fn parse_input(input: &Vec<String>) -> Graph<&str, (), Undirected> {
    let mut graph = Graph::new_undirected();
    let mut node_indices = HashMap::new();
    for line in input {
        let mut nodes: Vec<&str> = line.split("-").collect();
        let a = *node_indices.entry(nodes[0]).or_insert_with(|| graph.add_node(nodes[0]));
        let b = *node_indices.entry(nodes[1]).or_insert_with(|| graph.add_node(nodes[1]));
        graph.add_edge(a, b, ());
    }
    return graph;
}

fn sort_by_node_index(v: &Vec<NodeIndex>) -> Vec<NodeIndex> {
    let mut v = v.clone();
    v.sort_by(|a, b| a.index().cmp(&b.index()));
    return v;
}

fn find_clusters_3(graph: &Graph<&str, (), Undirected>) -> HashSet<Vec<NodeIndex>> {
    let mut clusters = HashSet::new();
    for node in graph.node_indices() {
        let neighbors: Vec<_> = graph.neighbors(node).collect();
        for i in 0..neighbors.len() {
            for j in (i + 1)..neighbors.len() {
                if graph.contains_edge(neighbors[i], neighbors[j]) {
                    let node_indices = vec![node, neighbors[i], neighbors[j]];
                    clusters.insert(sort_by_node_index(&node_indices));
                }
            }
        }
    }
    return clusters;
}

fn find_clusters_with_chief(clusters: HashSet<Vec<NodeIndex>>, graph: &Graph<&str, (), Undirected>) -> HashSet<Vec<NodeIndex>> {
    let mut chiefs = HashSet::new();
    for cluster in clusters {
        let mut chief = cluster[0];
        for node in &cluster {
            if let Some(name) = graph.node_weight(*node) {
                // if name start with "t"
                if name.starts_with("t") {
                    chiefs.insert(cluster);
                    break;
                }
            }
        }
    }
    return chiefs;
}

pub fn part1(input: &Vec<String>) -> i64 {

    let graph = parse_input(input);
    let clusters = find_clusters_3(&graph);
    // println!("{:?}", graph);
    // println!("{:?}", clusters);
    let chiefs = find_clusters_with_chief(clusters, &graph);

    return chiefs.len() as i64;
}