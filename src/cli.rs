use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    // Construct a command

    let command: Vec<String> = args.clone();

    if command[1] == "root" {
        println!("OS: {}", env::consts::OS);

        for (key, val) in env::vars() {
            println!("Environment Variables: \n key: {}, Value: {}", key, val);
        }
    }
}
