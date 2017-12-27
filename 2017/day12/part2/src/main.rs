use std::io;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_source(line: &str) -> i32{
    let v: Vec<&str> = line.trim().split(' ').collect();

    let target_str = match v.get(0) {
        Some(s) => s,
        None => {
            println!("Couldn't get target. Vec is {:?}", v.get(0));
            std::process::exit(1);
        }
    };

    let target_num: i32 = match target_str.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Couldn't parse target as integer: {}", target_str);
            std::process::exit(1);
        }
    };

    target_num
}

fn get_targets(line: &str) -> Vec<i32>{
    let nums: Vec<i32> = line.trim()
        .split(' ')
        .skip(2) // remove the source and -> token
        .map(|s| if &s[s.len()-1..s.len()] == "," { &s[..s.len()-1] } else { &s } ) // remove the trailing comma
        .map(|s| {
            let v: i32 = match s.parse() {
                Ok(num) => num,
                Err(_) => panic!("Failed to parse num: {}", s)
            };
            v
        })
        .collect();

    nums
}

fn record_connections(matrix: &mut HashMap<i32, Vec<i32>>, line: &str) {
    let src = get_source(line);
    let targets = get_targets(line);
    for target in &targets {
        { // establish new scope for mutable borrowing of matrix
            let forward_list = matrix.entry(src).or_insert(Vec::new());
            forward_list.push(*target);
        }

        { // another mutable borrow of matrix
            let reverse_list = matrix.entry(*target).or_insert(Vec::new());
            reverse_list.push(src);
        }
    }
}

fn union_find(adj: &HashMap<i32, Vec<i32>>, start: i32) -> HashSet<i32> {
    let mut reachable: HashSet<i32> = HashSet::new();

    let mut pending = HashSet::new();
    pending.insert(start);

    while pending.len() > 0 {
        let mut new_pending = HashSet::new();

        for item in pending.iter() {
            if !reachable.contains(&item) {
                let new_reachables = adj.get(&item).unwrap();
                for new_item in new_reachables {
                    new_pending.insert(new_item.clone());
                }
                reachable.insert(item.clone());
            }
        }

        pending.clear();
        new_pending.iter().for_each(|i| {pending.insert(i.clone());});
    }

    reachable
}

fn find_all_groups(adj: &mut HashMap<i32, Vec<i32>>) -> Vec<HashSet<i32>> {
    let mut all_groups = Vec::new();

    while adj.len() > 0 {
        let next_root = adj.iter().next().unwrap().0.clone();
        let next_group = union_find(adj, next_root.clone());

        all_groups.push(union_find(adj, next_root.clone()));

        for i in next_group.iter() {
            adj.remove(i);
        }
    }

    all_groups
}

fn main() {
    let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => if n == 0 { break; },
            Err(_) => { break; },
        }

        record_connections(&mut adj, input.trim());
        println!("{}", input.trim());
    }

    // now, the "how many X are transitively associated with Y" question is a classic
    // union-find application. So all we really need here is a HashSet
    println!("{}", find_all_groups(&mut adj).len());
}
