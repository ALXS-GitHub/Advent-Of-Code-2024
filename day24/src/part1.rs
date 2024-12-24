use std::collections::HashMap;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;

#[derive(Debug, Clone)]
struct EquationGraph {
    graph: Graph<String, String>,
    nodes: HashMap<String, NodeIndex>,
    values: HashMap<String, i64>,
}

impl EquationGraph {
    fn new() -> Self {
        EquationGraph {
            graph: Graph::new(),
            nodes: HashMap::new(),
            values: HashMap::new(),
        }
    }

    fn add_node(&mut self, name: &str) -> NodeIndex {
        if let Some(&index) = self.nodes.get(name) {
            index
        } else {
            let index = self.graph.add_node(name.to_string());
            self.nodes.insert(name.to_string(), index);
            index
        }
    }

    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, operation: &str) {
        self.graph.add_edge(from, to, operation.to_string());
    }

    fn set_value(&mut self, name: &str, value: i64) {
        self.values.insert(name.to_string(), value);
    }

    fn get_value(&mut self, name: &str) -> i64 {
        if let Some(&value) = self.values.get(name) {
            value
        } else {
            -1
        }
    }
}


fn parse_input(input: &Vec<String>) -> EquationGraph {
    let mut operations_sections = false;
    let mut booleans = HashMap::new();
    let mut eq_graph = EquationGraph::new();

    for line in input {
        if line == "" {
            operations_sections = true;
            continue;
        }

        if operations_sections {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let equation = parts[0];
            let output = parts[1];
            let eq_parts: Vec<&str> = equation.split(" ").collect();
            let operand1 = eq_parts[0];
            let operator = eq_parts[1];
            let operand2 = eq_parts[2];

            let operand1_node = eq_graph.add_node(operand1);
            let operand2_node = eq_graph.add_node(operand2);
            let output_node = eq_graph.add_node(output);

            eq_graph.add_edge(operand1_node, output_node, operator);
            eq_graph.add_edge(operand2_node, output_node, operator);

        } else {
            let parts: Vec<&str> = line.split(": ").collect();
            let name = parts[0];
            let value = parts[1].parse::<i64>().unwrap();
            booleans.insert(name, value);
        }
    }

    for (name, value) in booleans {
        eq_graph.set_value(name, value);
    }

    eq_graph
}

fn calculate_values_rec(graph: &mut EquationGraph, node: NodeIndex) -> i64 {
    let mut value = 0;
    // find edges where node is "to"
    let mut froms = vec![];
    for edge in graph.clone().graph.edges_directed(node, petgraph::Direction::Incoming) {
        let from = edge.source();
        let operation = graph.graph.edge_weight(edge.id()).unwrap().clone();
        let from_name = graph.graph.node_weight(from).unwrap().clone();
        let mut from_value = graph.get_value(&from_name);
        if from_value == -1 {
            from_value = calculate_values_rec(graph, from);
        }

        froms.push((from_value, operation));
        
    }

    assert_eq!(froms.len(), 2);
    assert_eq!(froms[0].1, froms[1].1);

    let operation = &froms[0].1;
    let operand1 = froms[0].0;
    let operand2 = froms[1].0;

    match operation.as_str() {
        "AND" => value = operand1 & operand2,
        "OR" => value = operand1 | operand2,
        "XOR" => value = operand1 ^ operand2,
        _ => panic!("Unknown operation"),
    }

    graph.set_value(&graph.graph.node_weight(node).unwrap().clone(), value);

    return value
}

fn calculate_all_values(graph: &mut EquationGraph) {
    for node in graph.graph.node_indices() {
        let name = graph.graph.node_weight(node).unwrap().clone();
        let value = graph.get_value(&name);
        if value != -1 {
            continue;
        }

        calculate_values_rec(graph, node);

    }
}

fn find_z_nodes(graph: &mut EquationGraph) -> Vec<(String, i64)> {
    let mut z_nodes = vec![];
    for node in graph.graph.node_indices() {
        let name = graph.graph.node_weight(node).unwrap().clone();
        if name.starts_with("z") {
            let value = graph.get_value(&name);
            if value == -1 {
                panic!("Node {} has no value", name);
            }
            z_nodes.push((name, value));
        }
    }

    // sort by name number part
    z_nodes.sort_by(|a, b| a.0[1..].parse::<i64>().unwrap().cmp(&b.0[1..].parse::<i64>().unwrap()));

    return z_nodes;
} 

fn get_binary(z_nodes: &Vec<(String, i64)>) -> i64 {
    let mut binary = 0;
    for (i, (name, value)) in z_nodes.iter().enumerate() {
        // println!("{:?} {:?}, {}", binary, value, i);
        binary += value * 2i64.pow(i as u32);
    }
    return binary;
}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut graph = parse_input(input);
    
    calculate_all_values(&mut graph);
    // println!("{:?}", graph);

    let z_nodes = find_z_nodes(&mut graph);
    // println!("{:?}", z_nodes);

    let binary = get_binary(&z_nodes);

    return binary;
}