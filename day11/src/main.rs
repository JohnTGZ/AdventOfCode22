use std::collections::{VecDeque, BinaryHeap};
use std::fs;
// use std::io::{self, stdout, Read, Write};

#[derive(Clone, PartialEq, Debug)]
pub enum Operation {
    MULTIPLY(u128),
    SQUARE,
    ADD(u128),
    // MINUS(u128),
    // DIVIDE(u128),
}

#[derive(Clone)]
pub struct Monkey {
    pub id: usize,
    // Current list of items
    pub items: VecDeque<u128>,
    // Operation on the item
    pub operation: Operation,

    // Test the item
    pub test: u128,
    // If test is true, who to pass to
    pub if_true: usize,
    // If test is false, who to pass to
    pub if_false: usize,

    // Number of item inspections made
    pub num_inspections: u128,
}

impl Monkey {
    pub fn item_operation(&self, worry_level: &u128) -> u128 {
        match self.operation {
            Operation::MULTIPLY(x) => {
                return worry_level * x as u128;
            }
            Operation::SQUARE => {
                return worry_level * worry_level;
            }
            Operation::ADD(x) => {
                return worry_level + x as u128;
            }
        }
    }

    pub fn test_item(&self, worry_level: &u128) -> bool {
        worry_level % self.test == 0
    }
}

fn build_monkeys(input_file: &str) -> Vec<Monkey> {

    let content = fs::read_to_string(input_file).expect("Unable to read the file");

    let mut lines = content.split("\n").into_iter();

    let mut monkey_list: Vec<Monkey> = Vec::new();

    let mut current_monkey = Monkey {
        id: 0,
        items: VecDeque::new(),
        operation: Operation::MULTIPLY(1),
        test: 0,
        if_true: 0,
        if_false: 0,
        num_inspections: 0,
    };

    loop {
        let mut attributes = lines.next().unwrap_or_else(|| "ENDINPUT").split(":");

        match attributes.next().unwrap().trim_start() {
            "Starting items" => {
                // write!(stdout(), "  Starting items\n").unwrap();

                for item in attributes.next().unwrap().split(", ") {
                    // write!(stdout(), "{} ", item.trim()).unwrap();
                    current_monkey
                        .items
                        .push_back(item.trim().parse::<u128>().unwrap());
                }

                // write!(stdout(), "\n").unwrap();
            }
            "Operation" => {
                // write!(stdout(), "  Operation\n").unwrap();

                let rhs = attributes.next().unwrap().split("=").last().unwrap();

                let op_val = rhs.split(" ").last().unwrap();

                let operation: Operation;

                match rhs {
                    expr if expr.contains("*") => {
                        if op_val == "old" {
                            operation = Operation::SQUARE;
                        } else {
                            operation = Operation::MULTIPLY(op_val.parse::<u128>().unwrap());
                        }
                    }
                    expr if expr.contains("+") => {
                        // write!(stdout(), "  expr: {}", expr).unwrap();
                        operation = Operation::ADD(op_val.parse::<u128>().unwrap());
                    }
                    others => {
                        panic!("  Match arm not covered for {others}!");
                    }
                }
                current_monkey.operation = operation;

                // write!(stdout(), "{}", attributes.next().unwrap()).unwrap();
            }
            "Test" => {
                // write!(stdout(), "  Test\n").unwrap();

                let test_num = attributes
                    .next()
                    .unwrap()
                    .split(" ")
                    .last()
                    .unwrap()
                    .parse::<u128>()
                    .unwrap();

                // Read the next 2 lines
                let if_true_monkey_id = lines
                    .next()
                    .unwrap()
                    .trim_start()
                    .split("monkey")
                    .last()
                    .unwrap()
                    .trim_start()
                    .parse::<u128>()
                    .unwrap();
                let if_false_monkey_id = lines
                    .next()
                    .unwrap()
                    .trim_start()
                    .split("monkey")
                    .last()
                    .unwrap()
                    .trim_start()
                    .parse::<u128>()
                    .unwrap();
                // write!(stdout(), "    If true, pass to {}\n", if_true_monkey_id).unwrap();
                // write!(stdout(), "    If false, pass to {}\n", if_false_monkey_id).unwrap();

                current_monkey.test = test_num;
                current_monkey.if_false = if_false_monkey_id as usize;
                current_monkey.if_true = if_true_monkey_id as usize;

                monkey_list.push(current_monkey.clone());

                current_monkey.items.clear();
            }
            monkey_id if monkey_id.contains("Monkey") => {
                let monkey_num = monkey_id.split(" ").last().unwrap().parse::<u128>().unwrap();
                // write!(stdout(), "Monkey {}\n", monkey_num).unwrap();

                current_monkey.id = monkey_num as usize;
            }
            "" => {
                continue;
            }
            _ => {
                // write!(stdout(), "End of input \n").unwrap();
                break;
            }
        }
    }

    monkey_list
}

fn test_monkey_struct(monkey_list: &Vec<Monkey>) -> () {
    assert_eq!(monkey_list.len(), 4);

    assert_eq!(monkey_list[0].items, vec![79, 98]);
    assert_eq!(monkey_list[2].items, vec![79, 60, 97]);
    assert_eq!(monkey_list[3].items, vec![74]);

    assert_eq!(monkey_list[0].operation, Operation::MULTIPLY(19));
    assert_eq!(monkey_list[1].operation, Operation::ADD(6));
    assert_eq!(monkey_list[2].operation, Operation::SQUARE);
    assert_eq!(monkey_list[3].operation, Operation::ADD(3));

    assert_eq!(monkey_list[0].test, 23);
    assert_eq!(monkey_list[1].test, 19);
    assert_eq!(monkey_list[2].test, 13);
    assert_eq!(monkey_list[3].test, 17);

    assert_eq!(monkey_list[2].if_true, 1);
    assert_eq!(monkey_list[3].if_false, 1);
}

fn test_monkey_game(monkeys: &mut Vec<Monkey>) -> () {
    for _ in 0..20 { 

        for idx in 0..monkeys.len() {

            // Clone monkey to avoid borrowing twice from Monkey vector
            let mut current_monkey = monkeys[idx].clone();
            
            while let Some(worry_lvl) = current_monkey.items.pop_front() { 
                // Monkey inspects each item (and divide worry level by 3)
                let new_worry_level = (current_monkey.item_operation(&worry_lvl)) / 3;
                current_monkey.num_inspections += 1;
                if current_monkey.test_item(&new_worry_level) {
                    monkeys[current_monkey.if_true].items.push_back(new_worry_level);
                } else {
                    monkeys[current_monkey.if_false].items.push_back(new_worry_level);
                }
            }

            // Assign back new monkey
            monkeys[idx] = current_monkey;
        }
    }

    assert_eq!(monkeys[0].items, vec![10, 12, 14, 26, 34]);
    assert_eq!(monkeys[1].items, vec![245, 93, 53, 199, 115]);
    assert_eq!(monkeys[2].items, vec![]);
    assert_eq!(monkeys[3].items, vec![]);

    assert_eq!(monkeys[0].num_inspections, 101);
    assert_eq!(monkeys[1].num_inspections, 95);
    assert_eq!(monkeys[2].num_inspections, 7);
    assert_eq!(monkeys[3].num_inspections, 105);

}

fn get_part_1_answer(monkeys: &mut Vec<Monkey>) -> u128 {
    for _ in 0..20 { 

        for idx in 0..monkeys.len() {

            // Clone monkey to avoid borrowing twice from Monkey vector
            let mut current_monkey = monkeys[idx].clone();
            
            while let Some(worry_lvl) = current_monkey.items.pop_front() { 
                // Monkey inspects each item (and divide worry level by 3)
                let new_worry_level = (current_monkey.item_operation(&worry_lvl)) / 3;
                current_monkey.num_inspections += 1;
                if current_monkey.test_item(&new_worry_level) {
                    monkeys[current_monkey.if_true].items.push_back(new_worry_level);
                } else {
                    monkeys[current_monkey.if_false].items.push_back(new_worry_level);
                }
            }

            // Assign back new monkey
            monkeys[idx] = current_monkey;
        }
    }

    let mut monkey_inspection_max_heap = BinaryHeap::new();

    for monkey in monkeys {
        monkey_inspection_max_heap.push(monkey.num_inspections);
    }

    let mut monkey_business = 1;
    for _ in 0..2 {
        monkey_business *= monkey_inspection_max_heap.pop().unwrap();
    }

    monkey_business

}

fn test_monkey_game_part2(monkeys: &mut Vec<Monkey>) -> () {

    println!("Before operation");

    for _ in 0..1000 { 

        for idx in 0..monkeys.len() {

            // Clone monkey to avoid borrowing twice from Monkey vector
            let mut current_monkey = monkeys[idx].clone();
            
            while let Some(worry_lvl) = current_monkey.items.pop_front() { 
                // Monkey inspects each item (and divide worry level by 3)
                let new_worry_level = current_monkey.item_operation(&worry_lvl);
                current_monkey.num_inspections += 1;
                if current_monkey.test_item(&new_worry_level) {
                    monkeys[current_monkey.if_true].items.push_back(new_worry_level);
                } else {
                    monkeys[current_monkey.if_false].items.push_back(new_worry_level);
                }
            }

            // Assign back new monkey
            monkeys[idx] = current_monkey;
        }
    }
    println!("FInished operation");

    // assert_eq!(monkeys[0].items, vec![10, 12, 14, 26, 34]);
    // assert_eq!(monkeys[1].items, vec![245, 93, 53, 199, 115]);
    // assert_eq!(monkeys[2].items, vec![]);
    // assert_eq!(monkeys[3].items, vec![]);

    assert_eq!(monkeys[0].num_inspections, 52166);
    assert_eq!(monkeys[1].num_inspections, 47830);
    assert_eq!(monkeys[2].num_inspections, 1938);
    assert_eq!(monkeys[3].num_inspections, 52013);

    let mut monkey_inspection_max_heap = BinaryHeap::new();

    for monkey in monkeys {
        monkey_inspection_max_heap.push(monkey.num_inspections);
    }

    let mut monkey_business: u128 = 1;
    for _ in 0..2 {
        monkey_business *= monkey_inspection_max_heap.pop().unwrap();
    }

    assert_eq!(monkey_business, 2713310158);

}


fn main() {
    let input_test_file = "input/day11/test_input.txt";
    let input_final_file = "input/day11/final_input.txt";

    let mut monkeys_test_list = build_monkeys(input_test_file);
    test_monkey_struct(&monkeys_test_list);
    test_monkey_game(&mut monkeys_test_list.clone());

    test_monkey_game_part2(&mut monkeys_test_list.clone());

    let mut monkeys_final_list = build_monkeys(input_final_file);
    println!("Part1: monkey business: {}", get_part_1_answer(&mut monkeys_final_list));
}
