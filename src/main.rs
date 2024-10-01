use clap::Arg;
use serde_json::Value;
use std::env;
use std::io::{self, Read};
use std::process::Command;

fn env_to_str(env: &serde_json::Map<String, Value>) -> String {
    env.iter()
        .map(|(key, value)| format!("--env {}={}", key, value.as_str().unwrap_or("")))
        .collect::<Vec<String>>()
        .join(" ")
}

fn cmdline_to_str(cmdline: &Vec<Value>) -> String {
    cmdline
        .iter()
        .map(|e| e.as_str().unwrap_or("").to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn fg_proc_to_str(fg: &Vec<Value>) -> String {
    let fg = &fg[0];
    let cmdline_str = cmdline_to_str(fg["cmdline"].as_array().unwrap());
    if cmdline_str == "kitty @ ls" {
        env::var("SHELL").unwrap_or_default()
    } else {
        cmdline_str
    }
}

fn convert(session: &Vec<Value>) {
    for os_window in session {
        println!("new_os_window");

        for tab in os_window["tabs"].as_array().unwrap() {
            println!();
            println!("new_tab {}", tab["title"].as_str().unwrap_or(""));
            println!("layout {}", tab["layout"].as_str().unwrap_or(""));

            if let Some(windows) = tab["windows"].as_array() {
                if !windows.is_empty() {
                    println!("cd {}", windows[0]["cwd"].as_str().unwrap_or(""));
                }

                for w in windows {
                    println!("title {}", w["title"].as_str().unwrap_or(""));
                    println!(
                        "launch {} {}",
                        env_to_str(w["env"].as_object().unwrap()),
                        fg_proc_to_str(w["foreground_processes"].as_array().unwrap())
                    );
                    if w["is_focused"].as_bool().unwrap_or(false) {
                        println!("focus");
                    }
                }
            }
        }
    }
}

fn main() {
    let args = clap::Command::new("kitty-sessionizer")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Converts Kitty Session JSON to Session File")
        .arg(
            Arg::new("stdin")
                .short('s')
                .long("stdin")
                .help("Read Kitty JSON from stdin")
                .action(clap::ArgAction::SetTrue)
                .required(false),
        )
        .get_matches();

    let stdin = args.get_flag("stdin");

    if stdin {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");

        let session: Vec<Value> = serde_json::from_str(&buffer).unwrap();

        convert(&session);
    } else if env::var("TERM").unwrap_or_default() == "xterm-kitty" {
        let output = Command::new("kitty")
            .arg("@")
            .arg("ls")
            .output()
            .expect("Failed to execute kitty command");

        if output.status.success() {
            let session: Vec<Value> = serde_json::from_slice(&output.stdout).unwrap();

            convert(&session);
        } else {
            eprintln!("Error: Failed to get session data from kitty");
            std::process::exit(1);
        }
    } else {
        eprintln!("Error: Stdin not enabled & failed to find TERM=xterm-kitty to request data");
        std::process::exit(1);
    }
}
