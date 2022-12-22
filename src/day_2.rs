use crate::helper;

fn get_hand_score(hand: &str) -> u32 {
    match hand {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn get_winning_score(opponent: &str, player: &str) -> u32 {
    match (opponent, player) {
        ("C", "X") => 6,
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "Z") => 3,
        _ => 0,
    }
}

fn invert_player_hand(opponent: &str, player: &str) -> String {
    match (opponent, player) {
        ("A", "X") => String::from("Z"),
        ("A", "Y") => String::from("X"),
        ("A", "Z") => String::from("Y"),
        ("B", "X") => String::from("X"),
        ("B", "Y") => String::from("Y"),
        ("B", "Z") => String::from("Z"),
        ("C", "X") => String::from("Y"),
        ("C", "Y") => String::from("Z"),
        ("C", "Z") => String::from("X"),
        _ => panic!("Invalid player hand game")
    }
}

fn solve_part_1(content: &str) {
    let games: u32 = content
        .lines()
        .map(|x| {
            let game = x.split_whitespace().collect::<Vec<&str>>();
            match game.as_slice() {
                [opponent, player] => {
                    let player_hand_score = get_hand_score(player);
                    let winning_score = get_winning_score(opponent, player);
                    winning_score + player_hand_score
                },
                _ => panic!("Invalid matching of game")
            }
        })
        .sum();

    println!("Day 2 - Part 1: {:?}", games);
}

fn solve_part_2(content: &str) {
    let games: u32 = content
        .lines()
        .map(|x| {
            let game = x.split_whitespace().collect::<Vec<&str>>();
            match game.as_slice() {
                [opponent, player] => {
                    let player_hand = invert_player_hand(opponent, player);
                    let player_hand_score = get_hand_score(&player_hand);
                    let winning_score = get_winning_score(opponent, &player_hand);
                    winning_score + player_hand_score
                },
                _ => panic!("Invalid matching of game")
            }
        })
        .sum();

    println!("Day 2 - Part 2: {:?}", games);
}

pub fn solve() {
    let content = helper::read_input_day(2);

    solve_part_1(&content);
    solve_part_2(&content);
}
