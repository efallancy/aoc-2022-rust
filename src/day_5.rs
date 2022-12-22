use crate::helper;
use std::collections::HashMap;

#[derive(Debug)]
struct StackPosition {
    position: HashMap<usize, Vec<String>>,
}

impl StackPosition {
    fn new(position: &str) -> StackPosition {
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

        StackPosition { position: map }
    }

    fn _move_items(&mut self, from: usize, to: usize, total_items: usize, as_stack: bool) {
        let origin_value = self.position.get(&from).unwrap();
        let destination_value = self.position.get(&to).unwrap();

        let range_to = origin_value.len();
        let range_from = range_to - total_items;

        let to_move = &mut origin_value[range_from..range_to].to_vec().to_owned();

        if as_stack {
            to_move.reverse();
        }

        let to_keep = &origin_value[..range_from].to_vec();

        let destination_new_value = [destination_value.to_owned(), to_move.to_owned()].concat();

        self.position.insert(from, to_keep.to_owned());
        self.position.insert(to, destination_new_value);
    }

    fn move_items_as_stack(&mut self, from: usize, to: usize, total_items: usize) {
        self._move_items(from, to, total_items, true);
    }

    fn move_items_as_is(&mut self, from: usize, to: usize, total_items: usize) {
        self._move_items(from, to, total_items, false);
    }

    fn get_final_elements_result(&self) -> String {
        let total_columns = self.position.keys().count();
        let mut final_elements: Vec<&str> = vec![];

        for column in (1..=total_columns).collect::<Vec<usize>>() {
            match self.position.get(&column).unwrap().last() {
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

pub fn solve() {
    let input = helper::read_input_day(5);

    let content = input.split("\n\n").collect::<Vec<&str>>();
    let position = content.first().unwrap().to_owned();
    let instructions = content.last().unwrap().to_owned();

    let mut position_map_part_1 = StackPosition::new(position);
    let mut position_map_part_2 = StackPosition::new(position);

    for instruction in instructions.lines().collect::<Vec<&str>>() {
        let parsed = parsed_instruction(instruction);
        position_map_part_1.move_items_as_stack(parsed.from, parsed.to, parsed.total_items);
    }

    for instruction in instructions.lines().collect::<Vec<&str>>() {
        let parsed = parsed_instruction(instruction);
        position_map_part_2.move_items_as_is(parsed.from, parsed.to, parsed.total_items);
    }

    println!("Day 5 - Part 1: {:?}", position_map_part_1.get_final_elements_result()); // QNNTGTPFN
    println!("Day 5 - Part 2: {:?}", position_map_part_2.get_final_elements_result()); // GGNPJBTTR
}
