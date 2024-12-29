fn total_dist(x: &mut [i64], y: &mut [i64]) -> i64 {
    x.sort(); y.sort();
    x.iter().zip(y.iter()).map(|(a, b)| (a-b).abs()).sum()
}

use std::fs::File;
use std::io::{self, BufRead, ErrorKind};
use std::path::Path;

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

fn format_input(path: &str) -> io::Result<(Vec<i64>, Vec<i64>)> {
    let mut x: Vec<i64> = Vec::new();
    let mut y: Vec<i64> = Vec::new();
    let lines = read_lines(path)?;
    for line in lines.map_while(Result::ok) {
        let mut cols = line.split_whitespace();
        if let (Some(a), Some(b)) = (cols.next(), cols.next()) {
            x.push(a.parse().map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?);
            y.push(b.parse().map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?);
        }
    }
    Ok((x, y))
}

use std::collections::HashMap;

fn similarity_score(x: &[i64], y: &[i64]) -> i64 {
    let mut count: HashMap<i64, i64> = HashMap::new();
    for &i in y {
        *count.entry(i).or_insert(0) += 1;
    }
    x.iter().filter_map(|i| count.get(i).map(|v| v*i)).sum()
}

pub fn run() {
    match format_input("./assets/one_input.txt") {
        Ok((mut x, mut y)) => {
            println!("Total Distance: {}", total_dist(&mut x, &mut y));
            println!("Similarity Score: {}", similarity_score(&x, &y));
        }
        Err(e) => println!("Error: {}", e)
    }
}
