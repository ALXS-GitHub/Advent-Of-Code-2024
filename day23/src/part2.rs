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


// found algorithm here : https://en.wikipedia.org/wiki/MaxCliqueDyn_algorithm
// and implemented the MaxClique version
fn find_largest_clique(
    graph: &Graph<&str, (), Undirected>,
) -> Vec<NodeIndex> {
    fn max_clique(
        graph: &Graph<&str, (), Undirected>,
        r: &HashSet<NodeIndex>,
        q: &mut HashSet<NodeIndex>,
        qmax: &mut HashSet<NodeIndex>,
    ) {
        let mut r = r.clone();
        // Q and Qmax are managed outside recursion

        while !r.is_empty() {
            // Choose vertex p with maximum color C(p) (degree in this case)
            let &p = r.iter().max_by_key(|&&node| graph.neighbors(node).count()).unwrap();
            r.remove(&p);

            let c_p = graph.neighbors(p).count();

            if q.len() + c_p > qmax.len() {
                q.insert(p);
                let neighbors_p: HashSet<NodeIndex> = graph.neighbors(p).collect();
                let r_inter_gamma_p: HashSet<NodeIndex> = r
                    .intersection(&neighbors_p)
                    .cloned()
                    .collect();

                if !r_inter_gamma_p.is_empty() {
                    // Obtain vertex-coloring C’ (using degree as color)
                    max_clique(graph, &r_inter_gamma_p, q, qmax);
                } else if q.len() > qmax.len() {
                    *qmax = q.clone();
                }
                q.remove(&p);
            } else {
                return;
            }
        }
    }

    let nodes: HashSet<NodeIndex> = graph.node_indices().collect();
    let mut q = HashSet::new();
    let mut qmax = HashSet::new();
    max_clique(graph, &nodes, &mut q, &mut qmax);
    qmax.into_iter().collect()
}

fn get_clique_names(graph: &Graph<&str, (), Undirected>, clique: &Vec<NodeIndex>) -> Vec<String> {
    let mut names = vec![];
    for node in clique {
        if let Some(name) = graph.node_weight(*node) {
            names.push(name.to_string());
        }
    }
    return names;
}

fn sort_alphabetically(names: Vec<String>) -> Vec<String> {
    let mut names = names;
    names.sort();
    return names;
}

fn format_output(names: Vec<String>) -> String {
    let mut output = String::new();
    for (i, name) in names.iter().enumerate() {
        output.push_str(name);
        if i < names.len() - 1 {
            output.push_str(",");
        }
    }
    return output;
}


pub fn part2(input: &Vec<String>) -> String {
    let graph = parse_input(input);

    let largest_clique = find_largest_clique(&graph);
 
    let names = get_clique_names(&graph, &largest_clique);
    let sorted_names = sort_alphabetically(names);
    let output = format_output(sorted_names);

    return output;
}