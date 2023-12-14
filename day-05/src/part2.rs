use std::collections::{BTreeMap, VecDeque};

struct Step {
    move_count: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        let split = value.split_whitespace().collect::<Vec<_>>();

        Step {
            move_count: split[1].parse().unwrap(),
            from: split[3].parse::<usize>().unwrap() - 1,
            to: split[5].parse::<usize>().unwrap() - 1,
        }
    }
}

fn parse_crates_str(s: &str) -> BTreeMap<usize, VecDeque<char>> {
    s.lines()
        .map(|l| l.chars())
        .fold(BTreeMap::new(), |mut map, chars| {
            chars.enumerate().for_each(|(i, c)| {
                if c.is_ascii_alphabetic() {
                    let dest_index = i / 4;
                    map.entry(dest_index)
                        .and_modify(|stack| stack.push_back(c))
                        .or_insert(VecDeque::from([c]));
                }
            });
            map
        })
}

fn parse_procedures_str(s: &str) -> Vec<Step> {
    s.lines().map(|l| Step::from(l)).collect::<Vec<_>>()
}

pub fn solve(input: &str) -> String {
    let mut crates_map = input
        .split_once("\n\n")
        .map(|(crates_str, procedures_str)| {
            (
                parse_crates_str(&crates_str),
                parse_procedures_str(&procedures_str),
            )
        })
        .map(|(mut crates, steps)| {
            let mut temp: VecDeque<char> = VecDeque::new();

            for step in steps.iter() {
                temp.clear();

                for _ in 0..step.move_count {
                    temp.push_back(crates.get_mut(&step.from).unwrap().pop_front().unwrap());
                }

                while let Some(c) = temp.pop_back() {
                    crates.get_mut(&step.to).unwrap().push_front(c);
                }
            }

            crates
        })
        .unwrap();

    crates_map.values_mut().fold(String::new(), |mut acc, v| {
        acc.push(v.pop_front().unwrap());
        acc
    })
}
