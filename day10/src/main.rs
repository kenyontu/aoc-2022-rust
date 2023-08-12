use std::fs;

use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

struct Cpu {
    cycle: u32,
    x: i32,
    next_report: u32,
    signal_strength: i32,
    line: String,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            cycle: 0,
            x: 1,
            next_report: 40,
            signal_strength: 0,
            line: String::from(""),
        }
    }

    /// The pixel position moves one positon to the right each cycle and
    /// goes back to 0 after 40 cycles
    fn get_current_pixel_pos(&self) -> u32 {
        self.cycle % 40
    }

    /// The X register contains the position of the mid point of the frame.
    /// The frame has a width of 3 pixels, this means that when X is:
    /// - X = 0, frame is at: ##...
    /// - X = 1, frame is at: ###..
    /// - X = 2, frame is at: .###.
    /// - X = 3, frame is at: ..###
    /// - X = 4, frame is at: ...##
    ///
    /// A pixel is drawn when the current pixel position is within the frame.
    fn should_draw_pixel(&self) -> bool {
        let pixel_pos = self.get_current_pixel_pos() as i32;
        pixel_pos >= self.x - 1 && pixel_pos <= self.x + 1
    }

    fn should_report(&self) -> bool {
        self.cycle == self.next_report
    }

    fn next_cycle(&mut self) {
        if self.should_report() {
            self.print_line();
            self.line.clear();
            self.signal_strength += self.cycle as i32 * self.x;
            self.next_report += 40;
        }

        if self.should_draw_pixel() {
            self.line.push('#');
        } else {
            self.line.push('.');
        }

        self.cycle += 1;
    }

    fn print_line(&self) {
        println!("{}", &self.line);
    }

    fn exec(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {
                self.next_cycle();
            }
            Instruction::Addx(value) => {
                self.next_cycle();
                self.next_cycle();
                self.x += value;
            }
        }
    }
}

fn parse_input(input: String) -> Vec<Instruction> {
    let instr_regex = Regex::new(r"^(?<instruction>addx|noop)\s?(?<value>-\d+|\d+)?$").unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in input.lines() {
        if let Some(captures) = instr_regex.captures(line) {
            match &captures["instruction"] {
                "addx" => {
                    instructions.push(Instruction::Addx(captures["value"].parse::<i32>().unwrap()))
                }
                "noop" => instructions.push(Instruction::Noop),
                _ => (),
            }
        }
    }

    instructions
}

fn main() {
    let file = "input.txt";
    let contents = fs::read_to_string(file).expect(&format!("Error reading the {} file.", file));

    let mut cpu = Cpu::new();

    let instructions = parse_input(contents);
    for instruction in instructions.iter() {
        cpu.exec(instruction);
    }
    cpu.print_line();
}
