use std::env;
use std::io::{self, BufRead};
use std::fs;


trait Command {
    fn get_duration(&self) -> usize {
        1
    }

    fn execute(&self, value: &mut isize) {
        ()
    }
}

struct AddXCommand {
    value: isize
}

impl Command for AddXCommand {
    fn get_duration(&self) -> usize {
        2
    }

    fn execute(&self, value: &mut isize) {
        *value += self.value;
    }
}

struct NoOpCommand {}

impl Command for NoOpCommand {}


fn main() {
    let path = env::args().nth(1).expect("Missing argument input path.");

    let data: Vec<Box<dyn Command>> = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!"))
        .lines()
        .map(|line| {
            let text = line.expect("Could not read line!");
            match text.as_str() {
                "noop" | "noop\n" => Box::new(NoOpCommand {}) as Box<dyn Command>,
                cmd if cmd.starts_with("addx ") => {
                    let (_, value) = cmd.split_at(5);
                    Box::new(AddXCommand { value: value.parse::<isize>().expect("Could not parse addx value!") }) as Box<dyn Command>
                },
                _ => panic!("Invalid command!")
            }
        }).collect();

    let sample_cycles: Vec<usize> = (1..=6).into_iter().map(|x| (x * 40 - 20) as usize).collect();

    let mut current_cycle: usize = 1;
    let mut value: isize = 1;
    let mut sample_acc: isize = 0;
    let mut command_count: usize = 0;
    for command in data {
        for _ in 0..command.get_duration() {
            if sample_cycles.contains(&current_cycle) {
                sample_acc += value * current_cycle as isize;
            }
            current_cycle += 1;
            
        }
        command.execute(&mut value);
        command_count += 1;
    }

    println!("{} command executed", command_count);
    println!("Samples accumulated: {}", sample_acc);
}


#[test]
fn test_addx() {
    let mut signal_value = 0;
    let op = AddXCommand { value: 10 };
    op.execute(&mut signal_value);

    assert_eq!(signal_value, 10);
}
