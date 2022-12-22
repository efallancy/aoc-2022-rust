use crate::helper;

pub fn solve() {
    let content = helper::read_input_day(1);

    let mut sum_grouped: Vec<u32> = content
        .split("\n\n")
        .map(|x| x.lines().filter_map(|s| s.parse::<u32>().ok()).sum::<u32>())
        .collect();

    sum_grouped.sort();
    sum_grouped.reverse();

    let asc_sum_grouped = &sum_grouped;

    println!("Day 1 - Part 1: {}", asc_sum_grouped[0]);
    println!(
        "Day 1 - Part 2: {}",
        asc_sum_grouped[0..=2].iter().sum::<u32>()
    );
}
