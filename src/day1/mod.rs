use num::abs;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn main() -> anyhow::Result<()> {
    let mut lines = BufReader::new(File::open("src/day1/data.txt").await?).lines();
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();

    while let Some(line) = lines.next_line().await? {
        let parts = line.splitn(2, " ").collect::<Vec<&str>>();
        l1.push(parse_i32(parts[0]));
        l2.push(parse_i32(parts[1]));
    }
    l1.sort();
    l2.sort();
    let diff = l1.iter().zip(l2.iter())
        .map(|(a, b)| abs(a - b))
        .sum::<i32>();
    println!("diff: {}", diff);
    let similarity = l1.iter()
        .map(|a| (l2.iter().filter(|b| *b == a).count() as i32) * (*a))
        .sum::<i32>();
    println!("similarity: {}", similarity);

    Ok(())
}

fn parse_i32(s: &str) -> i32 {
    s.trim().parse::<i32>().expect(format!("[{}] must be a number", s).as_str())
}


