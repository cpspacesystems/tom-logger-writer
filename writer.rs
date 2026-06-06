use std::io;
use std::str::FromStr;


#[repr(C)]
#[derive(Clone)]
enum Command {
    Start,
    Stop,
    Continue,
}

impl TryFrom<i32> for Command {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Command::Start as i32 => Ok(Command::Start),
            x if x == Command::Stop as i32 => Ok(Command::Stop),
            x if x == Command::Continue as i32 => Ok(Command::Continue),
            _ => Err(()),
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(input:&str) -> Result<Self,Self::Err> {
        let mut userInput = input.trim().split_whitespace();

        match userInput.next() {
            Some("Start") => Ok(Command::Start),
            Some("Stop") => Ok(Command::Stop),
            Some("Continue") => Ok(Command::Continue),
            Some(_) => Ok(Command::Start),
            None => Ok(Command::Start),
        }
    }
} 

fn main() {
    let cmd: Command = Command::Start; // we will auto start

    let mut command_shm = tism::create("command_shm", cmd).unwrap();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = match input.trim() {
            "0" => Command::Start,
            "1" => Command::Stop,
            "2" => Command::Continue,
            _ => {
                println!("Invalid command");
                continue;
            }
        };

        command_shm.write(command).unwrap();

        println!("Sent command");
    }
}
