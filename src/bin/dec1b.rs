use aoc22::io;

fn main() {
    let lines = crate::io::safe_lines();
    let elves = aoc22::day::dec1::best_calorie_elves(lines);
    let best_elf = elves.into_iter().take(3).sum::<u32>();
    println!("{}", best_elf)
}
