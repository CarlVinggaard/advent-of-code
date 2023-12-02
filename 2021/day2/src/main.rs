use std::fs;

struct Command<'a> {
    direction: &'a str,
    value: i32,
}

fn parse_command(command: &str) -> Command {
    let parsed: Vec<&str> = command.split(" ").collect();

    let direction = parsed[0];
    let value: i32 = parsed[1].parse().unwrap();

    Command { direction, value }
}

fn main() {
    let mut depth = 0;
    let mut v_pos = 0;
    let mut aim = 0;

    let filename = "./input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    for line in lines {
        let cmd = parse_command(line);

        match cmd.direction {
            "forward" => {
                v_pos = v_pos + cmd.value;
                depth = depth + aim * cmd.value;
            }
            "up" => {
                aim = aim - cmd.value;
            }
            "down" => {
                aim = aim + cmd.value;
            }
            _ => {
                println!("Received invalid command: {}", cmd.direction);
            }
        }
    }

    println!("Final position: {}", depth * v_pos);
}
