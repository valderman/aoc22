pub fn priority(c: char) -> u32 {
    if ('a' ..= 'z').contains(&c) {
        (c as u32 - 'a' as u32) + 1
    } else {
        (c as u32 - 'A' as u32) + 27
    }
}
