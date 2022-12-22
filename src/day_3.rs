use crate::helper;
use std::collections::HashSet;

fn get_alphabets() -> String {
    let lowercase_alphabets: Vec<char> = ('a'..='z').into_iter().collect();
    let uppercase_alphabets: Vec<char> = ('A'..='Z').into_iter().collect();
    let alphabets = String::from_iter([lowercase_alphabets, uppercase_alphabets].concat());

    alphabets
}

fn calculate_priority(alphabets: &str, words: &str) -> usize {
    let word_count = words.len();

    let (first, last) = words.split_at(word_count / 2);

    let x: HashSet<&str> = HashSet::from_iter(
        first
            .split("")
            .filter(|w| !w.is_empty())
            .collect::<Vec<&str>>(),
    );
    let y: HashSet<&str> = HashSet::from_iter(
        last.split("")
            .filter(|w| !w.is_empty())
            .collect::<Vec<&str>>(),
    );

    let intersection: Vec<&str> = x.intersection(&y).cloned().collect();

    let alphabet = intersection.first().unwrap();

    let priority = alphabets.find(alphabet).unwrap() + 1;

    priority
}

fn calculate_chunk_priority(alphabets: &str, chunk: &[&str]) -> usize {
    let x = chunk.get(0).unwrap();
    let y = chunk.get(1).unwrap();
    let z = chunk.get(2).unwrap();

    let chars_x: Vec<char>  = x.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>();
    let chars_y: Vec<char>  = y.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>();
    let chars_z: Vec<char>  = z.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>();

    let intersect: Vec<char> = [chars_x, chars_y, chars_z].into_iter().reduce(|acc, item| {
        let acc_set: HashSet<char> = HashSet::from_iter(acc.into_iter());
        let item_set: HashSet<char> = HashSet::from_iter(item.into_iter());

        let intersection = acc_set.intersection(&item_set).cloned().collect::<Vec<char>>();

        intersection
    }).unwrap();


    let alphabet = intersect.first().unwrap().to_string();

    let priority = alphabets.find(&alphabet).unwrap() + 1;

    priority
}

fn solve_part_1(input: &str) {
    let words: Vec<&str> = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    let alphabets = get_alphabets();

    let priority_sum: usize = words
        .into_iter()
        .map(|w| calculate_priority(&alphabets, &w))
        .sum();

    println!("Day 3 - Part 1: {:?}", priority_sum);
}

fn solve_part_2(input: &str) {
    let words: Vec<&str> = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let alphabets = get_alphabets();

    let word_chunks = words
        .chunks(3)
        .collect::<Vec<&[&str]>>();

    let priority: usize = word_chunks.into_iter().map(|c| calculate_chunk_priority(&alphabets, &c)).sum();

    println!("Day 3 - Part 2: {:?}", priority);
}

pub fn solve() {
    let content = helper::read_input_day(3);

    solve_part_1(&content);
    solve_part_2(&content);
}
