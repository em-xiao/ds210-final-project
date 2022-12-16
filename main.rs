mod graph {
    use std::collections::HashMap;
    use petgraph::Graph;

    pub fn create_graph<'a>(edges: &'a [(&'a str, &'a str, i32)]) -> Graph<&'a str, i32> {
        let mut node_map = HashMap::new();
        let mut graph = Graph::new(); 

        for (src, tar, weight) in edges {
            let src_id = *node_map.entry(src).or_insert_with(|| {
                graph.add_node(*src)
            });
            let tar_id = *node_map.entry(tar).or_insert_with(|| {
                graph.add_node(*tar)
            });
            graph.add_edge(src_id, tar_id, *weight);
        }

        graph
    }
}

mod degree {
    use std::collections::HashMap;

    pub fn degree_distribution<'a>(edges: &'a [(&'a str, &'a str, i32)]) -> HashMap<&'a str, i32> {
        let mut degrees = HashMap::new();

        for &(u, v, _) in edges {
            *degrees.entry(u).or_insert(0) += 1;
            *degrees.entry(v).or_insert(0) += 1;
        }

        degrees
    }
}

mod traversal {
    use std::collections::{HashMap, VecDeque};
    use petgraph::graph::{Graph, NodeIndex};

    pub fn bfs(graph: &Graph<&str, i32>, source: NodeIndex, target: NodeIndex) -> Option<Vec<NodeIndex>> {
        let mut queue = VecDeque::new();

        queue.push_back(source);

        let mut distances = HashMap::new();
        distances.insert(source, 0);

        let mut previous = HashMap::new();

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();

            if node == target {
                let mut path = Vec::new();
                path.push(node);

                let mut current = node;
                while let Some(prev) = previous.get(&current) {
                    path.push(*prev);
                    current = *prev;
                }

                return Some(path.into_iter().rev().collect());
            }
            for neighbor in graph.neighbors(node) {
                if distances.contains_key(&neighbor) {
                    continue;
                }

                distances.insert(neighbor, distances[&node] + 1);
                previous.insert(neighbor, node);
                queue.push_back(neighbor);
            }
        }

        None
    }
}

fn main() {
    use graph::create_graph;
    use degree::degree_distribution;
    use traversal::bfs;

    let edges = vec![
        ("Ada", "Cora", 1),
        ("Cora", "Ada", 1),
        ("Louise", "Marion", 1),
        ("Jean", "Helen", 1),
        ("Helen", "Jean", 1),
        ("Martha", "Anna", 1),
        ("Alice", "Eva", 1),
        ("Robin", "Eva", 1),
        ("Marion", "Martha", 1),
        ("Maxine", "Adele", 1),
        ("Lena", "Marion", 1),
        ("Hazel", "Hilda", 1),
        ("Hilda", "Betty", 1),
        ("Frances", "Eva", 1),
        ("Eva", "Maxine", 1),
        ("Ruth", "Jane", 1),
        ("Edna", "Mary", 1),
        ("Adele", "Frances", 1),
        ("Jane", "Adele", 1),
        ("Anna", "Maxine", 1),
        ("Mary", "Edna", 1),
        ("Betty", "Edna", 1),
        ("Ella", "Ellen", 1),
        ("Ellen", "Anna", 1),
        ("Laura", "Eva", 1),
        ("Irene", "Hilda", 1),
    ];

    let graph = create_graph(&edges);
        println!("graph: {:?}", graph);

    let degrees = degree_distribution(&edges);

    for (node, degree) in &degrees {
        println!("node {} has degree {}", node, degree);
    }

    let source = graph.node_indices().find(|n| graph[*n] == "Eva").unwrap();

    let target = graph.node_indices().find(|n| graph[*n] == "Maxine").unwrap();

    let path = bfs(&graph, source, target);
        println!("shortest path from Eva to Maxine: {:?}", path);
}

// For this analysis, I selected a dataset on dining table partners. It represents the choices 
// of the girls within a dormitory. Each node of the directed graph is converted into a specific girl (their name) 
// while each edge in this network resembled each girl's first choice of their top dining partner. I was interested in 
// finding out which girl was the most popular and interpret other patterns of social ties in this network. 
// I transferred the dataset into a graph by adding the edges using the source and target values. 
// I had mapped the node labels which originally was their names (strings) into numbers. 
// Then, with the graph, I was able to implement degree distribution where I looked into the number of nodes that 
// one specific node was connected to. Overall, it seems that Eva has the greatest amount of nodes, making her 
// the most popular choice. The remaining nodes split into a distribution that is slightly more even.
// Eva's choice is Maxine, who also has a decent number of connections. The Breadth–First Search 
// algorithm interprets that the shortest path to get from Eva to Maxine would be just between themselves. 