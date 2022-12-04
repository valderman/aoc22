use aoc22::day::dec3::priority;
use itertools::Itertools;

fn main() {
    let lines = aoc22::io::safe_lines().chunks(3);
    let priorities = lines.into_iter().map(|lines|
        find_shared_item_priority(lines).unwrap()
    );
    println!("{}", priorities.sum::<u32>())
}

fn find_shared_item_priority(mut lines: impl Iterator<Item = String>) -> Option<u32> {
    let lines_vec = lines.collect_vec();
    let first = &lines_vec[0];
    let rest = &lines_vec[1..];
    for c in first.chars() {
        if rest.into_iter().all(|s| s.contains(c)) {
            return Some(priority(c))
        }
    }
    None
}
