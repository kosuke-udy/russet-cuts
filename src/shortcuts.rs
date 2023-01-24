use std::process::Command;
use std::string::String; 

type Result<T> = std::result::Result<T, ShortcutsError>;

pub enum ShortcutsError {
    OSError,
    CommandError,
}

pub fn run(name: &str) -> Result<String> {
    run_macos_command(&"shortcuts", vec!["run", name]) 
}

// todo: add support for file input
pub fn run_with_input(
    name: &str,
    input_file_path: &str
) -> Result<String> {
    run_macos_command(
        &"shortcuts", 
        vec!["run", name, "-i", input_file_path]
    ) 
}

pub fn print_result(result: Result<String>) {
    match result {
        Ok(result) => {
            println!("{}", result);
        },
        Err(_) => {
            println!("Error");
        }
    }; 
}

fn run_macos_command(program: &str, args: Vec<&str>) -> Result<String> {
    if cfg!(not(target_os = "macos")) {
        return Err(ShortcutsError::OSError); 
    }

    let output 
        = Command::new(program)
                    .args(args)
                    .output(); 
    match output {
        Ok(output) => {
            Ok(String::from_utf8(output.stdout).unwrap())
        },
        Err(_) => Err(ShortcutsError::CommandError)
    }
}

#[test]
fn test_command() {
    let output = run("Rust.HelloRust"); 
    match output {
        Ok(output) => {
            println!("{}", output);
        },
        Err(_) => {
            println!("Error");
        }
    }
}