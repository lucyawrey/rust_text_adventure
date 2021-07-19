use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;
use std::str::Split;
use std::collections::HashMap;

fn main() {
    let world = open_world("start");

    game_loop(world);

    println!("\nShutting down...");
    return;
}

fn open_world(world: &str) -> Vec<String> {
    let str_path = format!("../script/{}.txt", world);
    let path = Path::new(&str_path);

    let file = match File::open(&path) {
        Err(_) => panic!("Oh no! We couldn't open the script!\n\nShutting down..."),
        Ok(file) => file
    };

    let buffer = BufReader::new(file);
    let opened: Vec<String> = buffer.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    
    return opened;
}

struct GameState {
    world: Vec<String>,
    location: usize,
    variables: HashMap<string, >,
}

fn game_loop(starting_world: Vec<String>) {
    let mut state = GameState {
        world: starting_world,
        location: 1,
        variables: HashMap,
    };

    loop {
        let line = &state.world[state.location - 1];

        let first_char = match line.chars().next() {
            Some(character) => character,
            None => ' '
        };

        if first_char == '>' {
            let command = line.replace('>', "");
            let mut split = command.split(" ");
            let operator = split.next().unwrap();
            let status = run_command(operator, &mut split, &mut state);
            match status {
                Status::End => { return },
                _ => {}
            }
        } else {
            print(line);
        }

        // End
        state.location += 1;
    }
}

static DELAY: Duration = Duration::from_millis(1000);

fn print(string: &str) {
    sleep(DELAY);
    println!("{}", string);
}

enum Status {
    Continue,
    End,
}

fn run_command(operator: &str, params: &mut Split<&str>, state: &mut GameState) -> Status {
    match operator {
        "end" => Status::End,
        "travel" => {
            let target = params.next().unwrap().parse::<usize>().unwrap();
            state.location = target - 1;
            Status::Continue
        },
        "warp" => {
            let world_name = params.next().unwrap();
            let target = params.next().unwrap().parse::<usize>().unwrap();
            let world = open_world(world_name);
            state.world = world;
            state.location = target - 1;
            Status::Continue
        },
        "choice" => {
            Status::Continue
        },
        _ => panic!("Encountered invalid script command!\n\nShutting down...")
    }
}
