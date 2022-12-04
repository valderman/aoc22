use aoc22::day::dec3::priority;

fn main() {
    let lines = aoc22::io::safe_lines();
    let priorities = lines.map(|line|
        find_misplaced_item_priority(&line).unwrap()
    );
    println!("{}", priorities.sum::<u32>())
}

fn find_misplaced_item_priority(s: &String) -> Option<u32> {
    let compartments = s.split_at(s.len() / 2);
    for c in compartments.0.chars() {
        if compartments.1.contains(c) {
            return Some(priority(c))
        }
    }
    None
}
