import networkx as nx

def parse_input(input_lines):
    graph = nx.Graph()
    for line in input_lines:
        a, b = line.strip().split("-")
        graph.add_edge(a, b)
    return graph

def find_largest_clique(graph):
    cliques = list(nx.find_cliques(graph))
    largest_clique = max(cliques, key=lambda x: len(x)) if cliques else []
    return largest_clique

def sort_alphabetically(clique):
    return sorted(clique)

def format_output(clique):
    return ",".join(clique)

if __name__ == "__main__":
    with open("input.txt") as file:
        input_lines = file.readlines()
    graph = parse_input(input_lines)
    largest_clique = find_largest_clique(graph)
    print(f"Largest Clique: {largest_clique}")
    sorted_clique = sort_alphabetically(largest_clique)
    print(f"Sorted Clique: {sorted_clique}")
    output = format_output(sorted_clique)
    print(f"Output: {output}")