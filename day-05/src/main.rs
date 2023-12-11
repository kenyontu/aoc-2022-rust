use regex::Regex;
use std::fs;

fn read_file(file: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file)
}

struct Step {
    move_count: u32,
    from_stack: usize,
    to_stack: usize,
}

impl Step {
    fn new(move_count: u32, from_stack: usize, to_stack: usize) -> Self {
        Self {
            move_count,
            from_stack,
            to_stack,
        }
    }

    fn steps_from_lines(lines: Vec<&str>) -> Vec<Self> {
        let mut steps: Vec<Self> = Vec::new();

        // Using regex with capturing groups to obtain the values from the steps
        let re = Regex::new(r"move (?<count>\d+) from (?<from>\d+) to (?<to>\d+)").unwrap();

        for line in lines.iter() {
            match re.captures(line) {
                Some(captures) => {
                    let move_count = captures["count"].parse::<u32>().unwrap();
                    let from_stack = captures["from"].parse::<usize>().unwrap();
                    let to_stack = captures["to"].parse::<usize>().unwrap();

                    steps.push(Self::new(move_count, from_stack - 1, to_stack - 1));
                }
                None => (),
            }
        }

        steps
    }
}

struct Storage {
    crate_stacks: Vec<Vec<char>>,
}

impl Storage {
    fn new() -> Self {
        Self {
            crate_stacks: Vec::new(),
        }
    }

    fn load_from_str_lines(&mut self, lines: Vec<&str>) {
        let mut crate_idx = 1;
        while crate_idx < lines[0].len() {
            let mut stack: Vec<char> = Vec::new();

            for line in lines.iter().rev() {
                match line.chars().nth(crate_idx) {
                    Some(c) => {
                        if c == ' ' {
                            break;
                        }

                        stack.push(c);
                    }
                    None => break,
                };
            }

            self.crate_stacks.push(stack);
            crate_idx += 4;
        }
    }

    fn exec_step(&mut self, step: &Step) {
        let mut temp = Vec::new();

        for _ in 0..step.move_count {
            match self.crate_stacks[step.from_stack].pop() {
                Some(c) => temp.push(c),
                _ => (),
            }
        }

        for _ in 0..temp.len() {
            match temp.pop() {
                Some(c) => self.crate_stacks[step.to_stack].push(c),
                _ => (),
            }
        }
    }
}

fn main() {
    let file = "input.txt";
    let contents = read_file(file).expect("Error reading the input.txt file.");
    let lines: Vec<&str> = contents.lines().collect();

    // A line with length 0 separates the initial stacks and the list of steps,
    // step through the lines until we find it
    let mut i = 0;
    while i < lines.len() {
        if lines[i].len() == 0 {
            break;
        }
        i += 1;
    }

    let (stacks, steps) = lines.split_at(i + 1);

    let mut storage = Storage::new();
    storage.load_from_str_lines(stacks[..stacks.len() - 2].to_vec());

    let steps = Step::steps_from_lines(steps.to_vec());
    for step in steps.iter() {
        storage.exec_step(step);
    }

    let mut at_top = String::new();
    for stack in storage.crate_stacks.iter() {
        let Some(c) = stack.last() else { break };
        at_top.push(*c);
    }

    println!("{}", at_top);
}
