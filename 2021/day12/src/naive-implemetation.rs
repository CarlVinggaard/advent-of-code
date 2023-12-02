use std::fs;
use regex::Regex;

fn extend_paths<'a>(paths: &mut Vec<Vec<&'a str>>, tunnels: &Vec<(&'a str, &'a str)>) -> Vec<Vec<&'a str>> {
    let mut new_paths: Vec<Vec<&'a str>> = vec!();

    while paths.len() > 0 {
        let opt = paths.pop();

        if let Some(path) = opt {
            let opt = path.last();

            if let Some(last) = opt {
                if last == &"end" {
                    new_paths.push(path);
                    continue
                }

                for tunnel in tunnels.iter() {
                    if &tunnel.0 == last && !(tunnel.1 == "start") && !(is_small_cave(tunnel.1) && path.contains(&tunnel.1) && has_visited_small_cave_twice(&path)) {
                        let mut new_tunnel = path.to_vec();
                        new_tunnel.push(tunnel.1);

                        new_paths.push(new_tunnel);
                    }

                    if &tunnel.1 == last && !(tunnel.0 == "start") && !(is_small_cave(tunnel.0) && path.contains(&tunnel.0) && has_visited_small_cave_twice(&path)) {
                        let mut new_tunnel = path.to_vec();
                        new_tunnel.push(tunnel.0);

                        new_paths.push(new_tunnel);
                    }
                }
            }
        }
    }

    new_paths
}

fn is_small_cave(cave: &str) -> bool {
    if cave == "start" || cave == "end" {
        return false
    }

    let re = Regex::new(r"^[a-z]*$").unwrap();

    re.is_match(cave)
}

fn has_visited_small_cave_twice(path: &Vec<&str>) -> bool {
    let mut small_caves: Vec<&str> = vec!();

    for cave in path {
        if is_small_cave(cave) {
            if small_caves.contains(&cave) {
                return true
            } else {
                small_caves.push(cave);
            }
        }
    }

    false
}

fn has_found_all(paths: &Vec<Vec<&str>>) -> bool {
    for path in paths {
        let opt = path.last();

        if let Some(last) = opt {
            if last != &"end" {
                return false
            }
        }
    }

    true
}

fn main() {
    let filename = "./input.txt";

    let input = fs::read_to_string(filename).expect("There was a problem reading the file");

    let tunnels: Vec<(&str, &str)> = input.split("\n").map(|l| {
        let parts: Vec<&str> = l.split("-").collect();
        (parts[0], parts[1])
    }).collect();
    
    let mut paths: Vec<Vec<&str>> = vec!();

    // Add start tunnels to paths
    for tunnel in tunnels.iter() {
        if tunnel.0 == "start" {
            paths.push(Vec::from([tunnel.0, tunnel.1]))
        }

        if tunnel.1 == "start" {
            paths.push(Vec::from([tunnel.1, tunnel.0]))
        }
    }

    while !has_found_all(&paths) {
        paths = extend_paths(&mut paths, &tunnels);
        println!("{} paths found", paths.len());
    }

    println!("All paths found. Total number of paths: {}", paths.len());
}
