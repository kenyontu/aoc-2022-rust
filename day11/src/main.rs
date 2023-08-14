use std::{cmp::Ordering, fs};

use regex::Regex;

type WorryLevel = u64;

#[derive(Debug)]
enum Operation {
    Add(WorryLevel),
    Multiply(WorryLevel),
    Square,
}

#[derive(Debug)]
struct DestinationTest {
    divisible_by: i32,
    destination_if_true: usize,
    destination_if_false: usize,
}

impl DestinationTest {
    fn get_destination(&self, worry_level: WorryLevel) -> usize {
        if worry_level as f32 % self.divisible_by as f32 == 0.0 {
            self.destination_if_true
        } else {
            self.destination_if_false
        }
    }
}

#[derive(Debug)]
struct Monkey {
    inspected_count: u32,
    items: Vec<WorryLevel>,
    operation: Operation,
    test: DestinationTest,
}

impl Monkey {
    fn new(items: Vec<WorryLevel>, operation: Operation, test: DestinationTest) -> Self {
        Monkey {
            inspected_count: 0,
            items,
            operation,
            test,
        }
    }
}

struct ItemToThrow {
    destination: usize,
    value: WorryLevel,
}

impl ItemToThrow {
    fn new(destination: usize, value: WorryLevel) -> Self {
        Self { destination, value }
    }
}

impl Monkey {
    fn throw_items(&mut self, lcm: &i32) -> Vec<ItemToThrow> {
        let mut items_to_throw: Vec<ItemToThrow> = Vec::new();

        for item in self.items.iter() {
            let worry_level = match self.operation {
                Operation::Add(v) => item + v,
                Operation::Multiply(v) => item * v,
                Operation::Square => item * item,
            } % *lcm as WorryLevel;

            let destination = self.test.get_destination(worry_level);
            items_to_throw.push(ItemToThrow::new(destination, worry_level));

            self.inspected_count += 1;
        }

        self.items.clear();

        items_to_throw
    }

    fn receive_item(&mut self, item: WorryLevel) {
        self.items.push(item);
    }
}

fn parse_input(input: String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let monkey_regex = Regex::new(
        r"Starting items: (?<items>.*)\n  Operation: new = old (?<operation>.{1}) (?<operation_num>\d+|old)\n  Test: divisible by (?<divisible_by>\d+)\n.* (?<destination_if_true>\d+)\n.* (?<destination_if_false>\d+)",
    ).unwrap();

    for captures in monkey_regex.captures_iter(&input) {
        // Obtain starting items
        let items_str = &captures["items"];
        let items: Vec<WorryLevel> = items_str
            .split(", ")
            .map(|item| item.parse::<WorryLevel>().unwrap())
            .collect();

        // Obtain operation
        let operation_symbol = &captures["operation"];
        let operation_num = &captures["operation_num"];

        let operation = if operation_num == "old" {
            Operation::Square
        } else {
            let operation_num = operation_num.parse::<WorryLevel>().unwrap();
            match operation_symbol {
                "+" => Operation::Add(operation_num),
                "*" => Operation::Multiply(operation_num),
                _ => panic!("Unsupported operation symbol: {}", operation_symbol),
            }
        };

        // Obtain divisible by
        let divisible_by = captures["divisible_by"].parse::<i32>().unwrap();
        let destination_if_true = captures["destination_if_true"].parse::<usize>().unwrap();
        let destination_if_false = captures["destination_if_false"].parse::<usize>().unwrap();
        let test = DestinationTest {
            divisible_by,
            destination_if_true,
            destination_if_false,
        };

        monkeys.push(Monkey::new(items, operation, test));
    }

    monkeys
}

fn calc_lcm(nums: &Vec<i32>) -> i32 {
    let mut max = 0;
    let mut common_multiple = 1;

    // Multiplies the numbers while also keeping track of the largest
    for num in nums.iter() {
        common_multiple *= num;
        if *num > max {
            max = *num;
        }
    }

    // Multiplying all the numbers results in a valid common multiplier, but it is
    // not always the least common multiplier (eg. 4 and 10), so we test all the
    // possible multiples of the largest number are divisible by all of them
    let mut n = common_multiple.clone();
    while n > 0 {
        let mut is_divisible_by_all = true;

        for num in nums.iter() {
            if n % *num != 0 {
                is_divisible_by_all = false;
                break;
            }
        }

        if is_divisible_by_all {
            return n;
        }
        n -= max;
    }

    common_multiple
}

fn main() {
    // For part 2 I understood that I had to find a way to make the numbers
    // smaller without interfering with which monkey would receive each item and that
    // Operations only consisted of addition and multiplication.
    //
    // At first I thought about using modulus to retain only part of the worry levels,
    // but it did not work and after more tries, I've concluded it was a math problem
    // I would not be able to solve and decided to look for the solution.
    //
    // The catch was that reducing the worry level by calculating the modulus of it with the
    // least common multiplier of all the "divisible_by" of all monkeys, does not interfere with
    // which monkey receives which items. It's important to note that this is only possible
    // because the only operations we do on the worry numbers are sum and multiplication.
    let file = "input.txt";
    let contents = fs::read_to_string(file).expect(&format!("Error reading the {} file", file));

    let mut monkeys = parse_input(contents);

    let lcm = calc_lcm(
        &monkeys
            .iter()
            .map(|m| m.test.divisible_by)
            .collect::<Vec<i32>>(),
    );

    for _ in 0..10000 {
        for monkey_idx in 0..monkeys.len() {
            let throwed_items = monkeys[monkey_idx].throw_items(&lcm);
            for item in throwed_items.iter() {
                monkeys[item.destination].receive_item(item.value)
            }
        }
    }

    // Sort monkeys by inspected count in a decreasing order
    monkeys.sort_by(|a, b| {
        if a.inspected_count > b.inspected_count {
            Ordering::Less
        } else if a.inspected_count < b.inspected_count {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    let monkey_business_level =
        monkeys[0].inspected_count as WorryLevel * monkeys[1].inspected_count as WorryLevel;

    println!("Monkey business level is: {}", monkey_business_level);
}
