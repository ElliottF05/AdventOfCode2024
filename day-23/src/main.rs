use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let connections: HashSet<_> = input
        .lines()
        .map(|line| {
            let x = line.split("-").collect::<Vec<_>>();
            return (x[0], x[1]);
        })
        .collect();
    
    let mut computers_set = HashSet::new();
    for (u,v) in &connections {
        computers_set.insert(*u);
        computers_set.insert(*v);
    }

    let mut computers_vec: Vec<_> = computers_set.into_iter().collect();
    computers_vec.sort();
    // println!("{:?}", computers_vec);

    let mut res = 0;

    for (i,c1) in computers_vec.iter().enumerate() {
        for (j,c2) in computers_vec.iter().skip(i+1).enumerate() {

            let c1_to_c2 = connections.contains(&(c1, c2)) || connections.contains(&(c2, c1));
            if !c1_to_c2 {
                continue;
            }

            for c3 in computers_vec.iter().skip(i+j+2) {

                let c1_to_c3 = connections.contains(&(c1, c3)) || connections.contains(&(c3, c1));
                let c2_to_c3 = connections.contains(&(c2, c3)) || connections.contains(&(c3, c2));

                if !c1_to_c3 || !c2_to_c3 {
                    continue;
                }

                if !(c1.chars().next().unwrap() == 't' || c2.chars().next().unwrap() == 't' || c3.chars().next().unwrap() == 't') {
                    continue;
                }

                // println!("{},{},{}", c1, c2, c3);
                res += 1;

            }
        }
    }

    return res;
}




fn part2(input: &str) -> String {
    // answer is: bg,bl,ch,fn,fv,gd,jn,kk,lk,pv,rr,tb,vw
    // CLIQUE SOLVER!!

    let mut connections = HashSet::new();
    let mut computers_set = HashSet::new();
    input.lines().for_each(|line| {
        let x: Vec<_> = line.split("-").collect();
        connections.insert((x[0], x[1]));
        connections.insert((x[1], x[0]));
        computers_set.insert(x[0]);
        computers_set.insert(x[1]);
    });

    let computers_vec: Vec<&str> = computers_set.into_iter().collect();
    let mut lan_parties = computers_vec.iter().map(|s| vec![*s]).collect::<HashSet<_>>();

    for lan_size in 0..1000 {
        println!("lan_size: {}, number of cliques: {}", lan_size, lan_parties.len());

        let mut next_lan_parties = HashSet::new();

        for prev_lan_party in &lan_parties {
            for new_computer in &computers_vec {

                let mut found_new_lan = true;
                for existing_computer in prev_lan_party {
                    if !connections.contains(&(new_computer, existing_computer)) {
                        found_new_lan = false;
                        break;
                    }
                }

                if found_new_lan {
                    let mut new_lan_party = prev_lan_party.clone();
                    new_lan_party.push(&new_computer);
                    new_lan_party.sort();
                    if next_lan_parties.contains(&new_lan_party) {
                        continue;
                    }
                    next_lan_parties.insert(new_lan_party);
                }
            }
        }

        if next_lan_parties.len() == 0 {
            return lan_parties.iter().next().unwrap().join(",");
        }

        lan_parties = next_lan_parties;
        
    }

    return "error".to_owned();
}
