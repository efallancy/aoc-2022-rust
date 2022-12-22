use crate::helper;
use std::collections::HashMap;

fn position_as_map(position: &str) -> HashMap<usize, Vec<String>> {
    let mut positions = position.lines().collect::<Vec<&str>>();

    positions.reverse();

    let mut map: HashMap<usize, Vec<String>> = HashMap::new();

    for (i, position) in positions.iter().enumerate() {
        let split_chars: Vec<char> = position.chars().collect();
        let items: Vec<String> = split_chars
            .chunks(4)
            .map(|x| x.get(1).unwrap().to_string())
            .collect();
        if i == 0 {
            for lane in items {
                map.insert(lane.parse::<usize>().unwrap(), Vec::new());
            }
        } else {
            for (i, item) in items.iter().enumerate() {
                if item != " " {
                    let key = i + 1;
                    let mut value = map.get(&key).unwrap().to_owned();
                    value.push(item.to_owned());
                    map.insert(i + 1, value);
                }
            }
        }
    }

    map
}

fn move_items(map: &mut HashMap<usize, Vec<String>>, from: usize, to: usize, total_items: usize) {
    let origin_value = map.get(&from).unwrap();
    let destination_value = map.get(&to).unwrap();

    let range_to = origin_value.len();
    let range_from = range_to - total_items;

    let to_move = &mut origin_value[range_from..range_to].to_vec().to_owned();
    to_move.reverse();

    let to_keep = &origin_value[..range_from].to_vec();

    let destination_new_value = [destination_value.to_owned(), to_move.to_owned()].concat();

    map.insert(from, to_keep.to_owned());
    map.insert(to, destination_new_value);
}

fn move_items_as_is(map: &mut HashMap<usize, Vec<String>>, from: usize, to: usize, total_items: usize) {
    let origin_value = map.get(&from).unwrap();
    let destination_value = map.get(&to).unwrap();

    let range_to = origin_value.len();
    let range_from = range_to - total_items;

    let to_move = &mut origin_value[range_from..range_to].to_vec().to_owned();

    let to_keep = &origin_value[..range_from].to_vec();

    let destination_new_value = [destination_value.to_owned(), to_move.to_owned()].concat();

    map.insert(from, to_keep.to_owned());
    map.insert(to, destination_new_value);
}

#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    total_items: usize,
}

fn parsed_instruction(instruction: &str) -> Instruction {
    let parsed = instruction
        .split(" ")
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    Instruction {
        total_items: parsed.get(0).unwrap().to_owned(),
        from: parsed.get(1).unwrap().to_owned(),
        to: parsed.get(2).unwrap().to_owned(),
    }
}

fn print_result(map: &HashMap<usize, Vec<String>>) -> String {
    let total_columns = map.keys().count();
    let mut final_elements: Vec<&str> = vec![];

    for column in (1..=total_columns).collect::<Vec<usize>>() {
        match map.get(&column).unwrap().last() {
            Some(v) => {
                final_elements.push(v);
            }
            _ => {
                final_elements.push(" ");
            }
        }
    }

    final_elements.join("")
}

pub fn solve() {
    let input = helper::read_input_day(5);

    let content = input.split("\n\n").collect::<Vec<&str>>();
    let position = content.first().unwrap().to_owned();
    let instructions = content.last().unwrap().to_owned();

    let mut position_map_part_1 = position_as_map(position);
    let mut position_map_part_2 = position_as_map(position);

    for instruction in instructions.lines().collect::<Vec<&str>>() {
        let parsed = parsed_instruction(instruction);
        move_items(
            &mut position_map_part_1,
            parsed.from,
            parsed.to,
            parsed.total_items,
        );
    }

    for instruction in instructions.lines().collect::<Vec<&str>>() {
        let parsed = parsed_instruction(instruction);
        move_items_as_is(
            &mut position_map_part_2,
            parsed.from,
            parsed.to,
            parsed.total_items,
        );
    }

    println!("Day 5 - Part 1: {:?}", print_result(&position_map_part_1)); // QNNTGTPFN
    println!("Day 5 - Part 2: {:?}", print_result(&position_map_part_2)); // GGNPJBTTR
}
