use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    
    // read input,
    // store nums
    

    
    // Part One
    // let mut seen = HashSet::new();

    // if let Ok(lines) = read_lines("./input.txt") {
    //     for line in lines {
    //         if let Ok(n) = line {
    //             let n: i32 = n.trim().parse()
    //                           .expect("Could not parse into i32");
                
    //             let complement: i32 = 2020 - n;
    //             if seen.contains(&complement) {
    //                 println!("Solution: {}", complement * n);
    //             }

    //             seen.insert(n);
    //         }
    //     }
    // }
    
    // Part Two
    let mut nums = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(n) = line {
                let n: i32 = n.trim().parse()
                            .expect("Could not parse into i32");
                nums.push(n);
            }
        }
    }

    for (i, n) in nums.iter().enumerate() {
        let target = 2020 - n;

        let mut seen = HashSet::new();

        for (j, m) in nums.iter().enumerate() {

            if j != i {
                let complement = target - m;
                // println!("complement: {}, target: {}, m: {}", complement, target, m);
                if seen.contains(&complement) {
                    println!("Solution: {}", n * m * complement);
                }
                seen.insert(m);
            }
        }
        
    }
}
