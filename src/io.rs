use std::io;

pub fn safe_lines() -> impl Iterator<Item = String> {
    io::stdin().lines()
    .filter_map(|line_or_error|
        line_or_error.map_or(None, |line| Some(line))
    )
}
