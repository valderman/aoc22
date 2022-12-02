use itertools::Itertools;

pub fn best_calorie_elves<T : Iterator<Item = String>>(lines: T) -> Vec<u32> {
    let elves_grouping = lines.group_by(|x| x.is_empty());
    elves_grouping.into_iter()
        .filter_map(|(empty, calorie_numbers)|
            if empty {
                 None
            } else {
                Some(
                    calorie_numbers.map(|s|
                        str::parse::<u32>(s.as_str()).unwrap()
                    )
                    .sum()
                )
            }
        )
        .sorted()
        .rev()
        .collect_vec()
}