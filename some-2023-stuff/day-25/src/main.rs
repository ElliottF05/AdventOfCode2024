use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut adj_map = HashMap::new();
    let mut edges = Vec::new();

    for line in input.lines() {
        let parts = line.split(": ").collect::<Vec<_>>();
        let u = parts[0].to_owned();
        let next = parts[1].split(" ").collect::<Vec<_>>();

        adj_map.entry(u.clone()).or_insert(Vec::new());

        for v in next {
            adj_map.get_mut(&u).unwrap().push(v.to_owned());
            adj_map.entry(v.to_owned()).or_insert(Vec::new()).push(u.clone());
            edges.push((u.clone(), v.to_owned()));
            edges.push((v.to_owned(), u.clone()));
        }
    }


    let vertices = adj_map.keys().collect::<Vec<_>>();
    let mut candidates: HashSet<(String, String)> = HashSet::from_iter(edges);

    // let mut edges_visited = HashSet::new();
    // for _ in 0..4 {
    //     println!("{}", bfs(&"jqt".to_owned(), &"rhn".to_owned(), &mut edges_visited, &adj_map));
    // }

    // there must exist >= 4 paths with no repeated edges between nodes in the same group
    let mut found_ans = false;
    for i in 0..vertices.len() {
        for j in i+1..vertices.len() {
            let u = vertices[i];
            let v = vertices[j];

            let mut edges_visited = HashSet::new();
            for _ in 0..4 {
                if bfs(u, v, &mut edges_visited, &adj_map) == false {
                    let mut next_candidates = HashSet::new();
                    for edge in &candidates {
                        if edges_visited.contains(edge) {
                            next_candidates.insert(edge.clone());
                        }
                    }
                    candidates = next_candidates;
                    // println!("candidates.len(): {}", candidates.len());
                    if candidates.len() == 6 {
                        found_ans = true;
                    }
                    break;
                }
            }
            if found_ans {
                break;
            }
        }
        if found_ans {
            break;
        }
    }

    let banned_edge = candidates.iter().next().unwrap();
    let size_1 = search(&banned_edge.0, &candidates, &adj_map);
    let size_2 = vertices.len() as i32 - size_1;

    return size_1 * size_2;


    fn search(start: &String, prohibited_edges: &HashSet<(String, String)>, adj_map: &HashMap<String, Vec<String>>) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut nodes_visited = HashSet::new();

        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            if nodes_visited.contains(curr) {
                continue;
            }
            nodes_visited.insert(curr);

            for next in adj_map.get(curr).unwrap() {
                if prohibited_edges.contains(&(curr.clone(),next.clone())) || prohibited_edges.contains(&(next.clone(),curr.clone())) {
                    continue;
                }
                queue.push_back(next);
            }
        }
        return nodes_visited.len() as i32;
    }

    fn bfs(start: &String, end: &String, edges_visited: &mut HashSet<(String, String)>, adj_map: &HashMap<String, Vec<String>>) -> bool {
        let mut queue = VecDeque::new();
        let mut nodes_visited = HashSet::new();
        let mut prev_map = HashMap::new();
        queue.push_back(start);

        let mut found_path = false;
        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();
            if u == end {
                found_path = true;
                break;
            }
            for v in adj_map.get(u).unwrap() {
                if nodes_visited.contains(v) {
                    continue;
                }
                if edges_visited.contains(&(u.clone(),v.clone())) || edges_visited.contains(&(v.clone(),u.clone())) {
                    continue;
                }
                queue.push_back(v);
                prev_map.insert(v, u);
                nodes_visited.insert(v);
            }
        }

        if found_path {
            let mut curr = end;
            while curr != start {
                let prev = prev_map.get(curr).unwrap();
                edges_visited.insert(((*prev).clone(), curr.clone()));
                edges_visited.insert((curr.clone(), (*prev).clone()));
                curr = prev;
            }
            return true;
        }

        return false;
    }

}

fn part2(input: &str) -> i32 {
    return -1;
}
