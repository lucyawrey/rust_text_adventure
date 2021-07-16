use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;
use std::str::Split;

fn main() {
    let path = Path::new("script.txt");

    let file = match File::open(&path) {
        Err(error) => panic!("Oh no! We couldn't open the script!\n\nShutting down..."),
        Ok(file) => file
    };

    let buffer = BufReader::new(file);
    let script: Vec<String> = buffer.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    game_loop(script);

    println!("\nShutting down...");
    return;
}

fn game_loop(script: Vec<String>) {
    let mut location = 0;
    loop {
        let line = &script[location];

        let first_char = match line.chars().next() {
            Some(character) => character,
            None => ' '
        };

        if first_char == '>' {
            let command = line.replace('>', "");
            let mut split = command.split(" ");
            let operator = split.next().unwrap();
            let status = runCommand(operator, &mut split, &mut location, &script);
            match status {
                Status::End => { return },
                _ => {}
            }
        } else {
            print(line);
        }

        // End
        location += 1;
    }
}

static DELAY: Duration = Duration::from_millis(1000);

fn print(string: &str) {
    sleep(DELAY);
    println!("{}", string);
}

enum Status {
    Continue,
    End
}

fn runCommand(operator: &str, params: &mut Split<&str>, location: &mut usize, script: &Vec<String>) -> Status {
    match operator {
        "end" => Status::End,
        "travel" => {
            let target = params.next().unwrap().parse::<usize>().unwrap();
            *location = target - 1;
            Status::Continue
        },
        "choice" => {
            Status::Continue
        },
        _ => panic!("Encountered invalid script command!\n\nShutting down...")
    }
}
