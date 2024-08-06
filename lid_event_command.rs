use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Cmd example: lid_event /proc/acpi/button/lid/LID0/state 'your_command_here'");
        std::process::exit(1);
    }
    let state_path = &args[1];
    let command = args[2..].join(" ");
    let mut last_state = is_open(state_path);
    loop {
        std::thread::sleep(std::time::Duration::new(1, 0));
        let state = is_open(state_path);
        if last_state && !state {
            println!("Lid closed, executing command: {}", command);
            match Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
            {
                Ok(output) => {
                    println!("Command executed successfully");
                    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                }
                Err(e) => println!("Failed to execute command: {}", e),
            }
        }
        last_state = state;
    }
}

fn is_open(filename: &str) -> bool {
    let contents = std::fs::read_to_string(filename)
        .expect("failed reading the state file");
    contents.contains("open")
}
