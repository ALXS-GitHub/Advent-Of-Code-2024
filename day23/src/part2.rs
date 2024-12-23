use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use petgraph::algo::connected_components;
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

fn find_largest_clique(
    graph: &Graph<&str, (), Undirected>,
) -> Vec<NodeIndex> {
    // todo
    unimplemented!()
}




pub fn part2(input: &Vec<String>) -> i64 {
    let graph = parse_input(input);
    // let largest_cluster = find_largest_connected_component(&graph);
    // println!("{:?}", largest_cluster);  

    return  0;
}