use crate::helper;

struct Assignments {
    x1: u32,
    x2: u32,
    y1: u32,
    y2: u32,
}

fn is_full_overlap(x1: u32, x2: u32, y1: u32, y2: u32) -> bool {
    x1 >= y1 && x2 <= y2 || y1 >= x1 && y2 <= x2
}

fn is_overlap(x1: u32, x2: u32, y1: u32, y2: u32) -> bool {
    x1 >= y1 && x1 <= y2 || x2 >= y1 && x2 <= y2 || y1 >= x1 && y1 <= x2 || y2 >= x1 && y2 <= x2
}

fn parse_as_assignments(content: &str) -> Assignments {
    let split = content.split(",").collect::<Vec<&str>>();
    let first = split.first().unwrap();
    let last = split.last().unwrap();

    let first_range = first.split("-").collect::<Vec<&str>>();
    let last_range = last.split("-").collect::<Vec<&str>>();

    let first_start = first_range.first().unwrap().parse::<u32>().unwrap();
    let first_end = first_range.last().unwrap().parse::<u32>().unwrap();
    let last_start = last_range.first().unwrap().parse::<u32>().unwrap();
    let last_end = last_range.last().unwrap().parse::<u32>().unwrap();

    Assignments {
        x1: first_start,
        x2: first_end,
        y1: last_start,
        y2: last_end,
    }
}

fn solve_part_1(content: &str) {
    let overlaps: Vec<bool> = content
        .lines()
        .map(|x| {
            let assignments = parse_as_assignments(x);

            is_full_overlap(
                assignments.x1,
                assignments.x2,
                assignments.y1,
                assignments.y2,
            )
        })
        .collect::<Vec<bool>>();

    println!(
        "Day 4 - Part 1: {:?}",
        overlaps
            .into_iter()
            .filter(|x| *x == true)
            .collect::<Vec<bool>>()
            .len()
    );
}

fn solve_part_2(content: &str) {
    let overlaps: Vec<bool> = content
        .lines()
        .map(|x| {
            let assignments = parse_as_assignments(x);

            is_overlap(
                assignments.x1,
                assignments.x2,
                assignments.y1,
                assignments.y2,
            )
        })
        .collect::<Vec<bool>>();

    println!(
        "Day 4 - Part 2: {:?}",
        overlaps
            .into_iter()
            .filter(|x| *x == true)
            .collect::<Vec<bool>>()
            .len()
    );
}

pub fn solve() {
    let input: String = helper::read_input_day(4);

    solve_part_1(&input);
    solve_part_2(&input);
}
