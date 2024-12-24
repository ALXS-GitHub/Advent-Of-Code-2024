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

    fn print_node(&self, name: &str, indent: usize) {
        if let Some(&node_index) = self.nodes.get(name) {
            let incoming_edges: Vec<_> = self.graph.edges_directed(node_index, petgraph::Direction::Incoming).collect();
            if incoming_edges.len() == 2 {
                let op1 = self.graph.node_weight(incoming_edges[0].source()).unwrap();
                let op2 = self.graph.node_weight(incoming_edges[1].source()).unwrap();
                let operation = incoming_edges[0].weight();
                println!(
                    "{}{} : {} {} {}",
                    "\t".repeat(indent),
                    name,
                    operation,
                    op1,
                    op2
                );
                self.print_node(op1, indent + 1);
                self.print_node(op2, indent + 1);
            } else {
                if let Some(&value) = self.values.get(name) {
                    println!(
                        "{}{} : VALUE {}",
                        "\t".repeat(indent),
                        name,
                        value
                    );
                } else {
                    println!(
                        "{}{} : INPUT",
                        "\t".repeat(indent),
                        name
                    );
                }
            }
        } else {
            println!(
                "{}{} : NOT FOUND",
                "\t".repeat(indent),
                name
            );
        }
    }

    fn get_formatted_node(&self, name: &str, indent: usize) -> String {
        let mut output = String::new();
        let indent_str = "\t".repeat(indent);

        if let Some(&node_index) = self.nodes.get(name) {
            let incoming_edges: Vec<_> = self.graph.edges_directed(node_index, petgraph::Direction::Incoming).collect();
            if incoming_edges.len() == 2 {
                let op1 = self.graph.node_weight(incoming_edges[0].source()).unwrap();
                let op2 = self.graph.node_weight(incoming_edges[1].source()).unwrap();
                let operation = incoming_edges[0].weight();
                output.push_str(&format!(
                    "{}{} : {} {} {}\n",
                    indent_str, name, operation, op1, op2
                ));
                output.push_str(&self.get_formatted_node(op1, indent + 1));
                output.push_str(&self.get_formatted_node(op2, indent + 1));
            } else {
                if let Some(&value) = self.values.get(name) {
                    output.push_str(&format!(
                        "{}{} : VALUE {}\n",
                        indent_str, name, value
                    ));
                } else {
                    output.push_str(&format!(
                        "{}{} : INPUT\n",
                        indent_str, name
                    ));
                }
            }
        } else {
            output.push_str(&format!(
                "{}{} : NOT FOUND\n",
                indent_str, name
            ));
        }

        output
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

fn get_x_values_as_binary(graph: &mut EquationGraph) -> i64 {
    let mut x_values = vec![];
    for node in graph.graph.node_indices() {
        let name = graph.graph.node_weight(node).unwrap().clone();
        if name.starts_with("x") {
            let value = graph.get_value(&name);
            if value == -1 {
                panic!("Node {} has no value", name);
            }
            x_values.push((name[1..].parse::<i64>().unwrap(), value));
        }
    }

    // sort by name number part
    x_values.sort_by(|a, b| a.0.cmp(&b.0));

    let mut binary = 0;
    for (i, (_, value)) in x_values.iter().enumerate() {
        binary += value * 2i64.pow(i as u32);
    }
    return binary;
}

fn get_y_values_as_binary(graph: &mut EquationGraph) -> i64 {
    let mut y_values = vec![];
    for node in graph.graph.node_indices() {
        let name = graph.graph.node_weight(node).unwrap().clone();
        if name.starts_with("y") {
            let value = graph.get_value(&name);
            if value == -1 {
                panic!("Node {} has no value", name);
            }
            y_values.push((name[1..].parse::<i64>().unwrap(), value));
        }
    }

    // sort by name number part
    y_values.sort_by(|a, b| a.0.cmp(&b.0));

    let mut binary = 0;
    for (i, (_, value)) in y_values.iter().enumerate() {
        binary += value * 2i64.pow(i as u32);
    }
    return binary;
}

fn find_linked_z_rec(graph: &EquationGraph, node: NodeIndex) -> Vec<i64> {
    let mut associated_z = 0;
    let mut linked_z = vec![];
    for edge in graph.graph.edges_directed(node, petgraph::Direction::Outgoing) {
        // print len directed edges
        // println!("{:?}", graph.graph.edges_directed(node, petgraph::Direction::Outgoing).count());
        // if graph.graph.edges_directed(node, petgraph::Direction::Outgoing).count() > 1 {
        //     println!("{:?}", graph.graph.node_weight(node).unwrap());
        // }
        let to = edge.target();
        let to_name = graph.graph.node_weight(to).unwrap().clone();
        if to_name.starts_with("z") {
            associated_z = to_name[1..].parse::<i64>().unwrap();
            linked_z.extend(vec![associated_z]);
        } else {
            linked_z.extend(find_linked_z_rec(graph, to));
        }
    }

    return linked_z;
}

fn find_bad_linked_x(graph: &mut EquationGraph) -> Vec<String> {
    let mut bad_linked_x = vec![];
    for node in graph.graph.node_indices() {
        let name = graph.graph.node_weight(node).unwrap().clone();
        if name.starts_with("x") {
            let mut current_x = name[1..].parse::<i64>().unwrap();
            // let linked_z = find_linked_z_rec(graph, node);

            // println!("{:?} {:?}", current_x, linked_z);
            // if !linked_z.contains(&current_x) {
            //     bad_linked_x.push(name);
            // }

            // * separate starting edge case
            for edge in graph.graph.edges_directed(node, petgraph::Direction::Outgoing) {
                let mut linked_z = vec![];
                let mut associated_z = 0;
                let to = edge.target();
                let to_name = graph.graph.node_weight(to).unwrap().clone();
                if to_name.starts_with("z") {
                    associated_z = to_name[1..].parse::<i64>().unwrap();
                    linked_z.extend(vec![associated_z]);
                } else {
                    linked_z.extend(find_linked_z_rec(graph, to));
                }

                if !linked_z.contains(&current_x) {
                    bad_linked_x.push(graph.graph.node_weight(node).unwrap().clone());
                    // println!("{:?} {:?}", current_x, linked_z);
                }
            }
        }
    }

    return bad_linked_x;
}

fn swap_nodes(graph: &mut EquationGraph, swaps: &Vec<(&str, &str)>) {
    for (a, b) in swaps {

        let graph_clone = graph.clone();

        // just change all the edges coming to a to now go to b and vice versa
        let a_node = graph.nodes.get(*a).unwrap();
        let b_node = graph_clone.nodes.get(*b).unwrap();

        let a_edges: Vec<_> = graph_clone.graph.edges_directed(*a_node, petgraph::Direction::Incoming).collect();
        let b_edges: Vec<_> = graph_clone.graph.edges_directed(*b_node, petgraph::Direction::Incoming).collect();

        for edge in a_edges {
            let from = edge.source();
            let operation = graph.graph.edge_weight(edge.id()).unwrap().clone();
            // remove the edge
            graph.graph.remove_edge(edge.id());
            graph.graph.add_edge(from, *b_node, operation);
        }

        for edge in b_edges {
            let from = edge.source();
            let operation = graph.graph.edge_weight(edge.id()).unwrap().clone();
            // remove the edge
            graph.graph.remove_edge(edge.id());
            graph.graph.add_edge(from, *a_node, operation);
        }

        // // update nodes
        // graph.nodes.insert(b.to_string(), *a_node);
        // graph.nodes.insert(a.to_string(), *b_node);
        
    }
}

pub fn part2(input: &Vec<String>) -> String {

    let mut graph = parse_input(input);
    
    // println!("{:?}", graph);
    
    // let bad_linked = find_bad_linked_x(&mut graph);
    // println!("{:?}", bad_linked);
    
    let swaps = vec![("hnd", "z09"), ("tdv", "z16"), ("bks", "z23"), ("nrn", "tjp")];
    swap_nodes(&mut graph, &swaps);

    calculate_all_values(&mut graph);

    // print distinct z values in bad_linked
    // graph.print_node("z01", 0);

    let x = get_x_values_as_binary(&mut graph);
    let y = get_y_values_as_binary(&mut graph);

    // println!("{:b}, {:b}", x, y);

    let expected_res = x + y;

    // graph.print_node("z09", 0);

    let z_nodes = find_z_nodes(&mut graph);
    // println!("{:?}", z_nodes);

    let binary = get_binary(&z_nodes);

    // println!("{:?}, {:?}", expected_res, binary);
    // now as binary
    // println!("{:b}, {:b}", expected_res, binary);

    let bin_ex_res = format!("{:b}", expected_res);
    let bin_res = format!("{:b}", binary);

    // get indexes of different bits
    let mut diff_bits = vec![];
    for (i, (a, b)) in bin_ex_res.chars().rev().zip(bin_res.chars().rev()).enumerate() {
        if a != b {
            diff_bits.push(i);
        }
    }
    // println!("{:?}", diff_bits);

    let mut output_string = String::new();
    for z in 0..=45 {
        output_string.push_str(graph.get_formatted_node(&format!("z{:02}", z), 0).as_str());
        output_string.push_str("\n");
    }

    // save output to file
    std::fs::write("output.txt", output_string).expect("Unable to write file");

    let mut flat_swaps = swaps.iter().flat_map(|(a, b)| vec![a, b]).collect::<Vec<_>>();
    // sort by name alphabetically
    let mut sorted_swaps = flat_swaps.iter().map(|&x| x).collect::<Vec<_>>();
    sorted_swaps.sort();

    let mut output_swaps = String::new();
    for (i, swap) in sorted_swaps.iter().enumerate() {
        output_swaps.push_str(swap);
        if i < sorted_swaps.len() - 1 {
            output_swaps.push_str(",");
        }
    }

    // println!("{:?}", output_swaps);  

    return output_swaps;
}