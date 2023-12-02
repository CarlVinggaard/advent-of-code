use std::fs;
use std::collections::HashMap;

fn count_paths<'a>(graph: &HashMap<&'a str, Vec<&'a str>>, current: &'a str, path: &mut Vec<&'a str>) -> usize {
    // println!("Count graphs called");
    // "End" always ends any path
    if current == "end" {
        return 1;
    }

    // If we loop back to a small cave or to start, abandon the path
    if path.contains(&current) && current.chars().all(|c| c.is_lowercase() && current == "start") {
        return 0;
    }

    // Add current to path and count length of all possible tunnels from here to end
    path.push(current);

    let sum = graph[current].iter().map(|n| { count_paths(graph, n, path) }).sum();

    path.pop();

    sum
}

fn main() {
    let mut graph = HashMap::new();

    let filename = "./test.txt";

    let input = fs::read_to_string(filename).expect("There was a problem reading the file");

    let lines: Vec<&str> = input.split("\n").map(|l| l).collect();

    for line in lines {
        let (a, b) = line.split_once('-').unwrap();
        graph.entry(a).or_insert(Vec::new()).push(b);
        graph.entry(b).or_insert(Vec::new()).push(a);
    }

    println!("Graph has {} edges", graph.keys().len());

    let path_count = count_paths(&graph, "start", &mut Vec::new());

    println!("Found {} paths", path_count);
}