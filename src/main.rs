use clap::Parser;
use serde::Deserialize;
use std::{
    collections::HashMap,
    env, fs,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

#[derive(Deserialize)]
struct Cargotoml {
    dev: HashMap<String, String>,
}

#[derive(Parser)]
#[structopt(name = "cargo-dev", bin_name = "cargo dev")]
struct Cli {
    #[clap(default_value = "dev")]
    cmd: String,

    #[clap(value_delimiter = ' ')]
    args: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let expanded_cmd = get_cmd(&cli.cmd);

    println!("> {}: {}", cli.cmd, expanded_cmd);
    exec(&expanded_cmd);
}

fn exec(exec_cmd: &str) {
    let mut cmd = Command::new("bash")
        .args(vec!["-c", exec_cmd])
        .env_clear()
        .stdout(Stdio::piped())
        .spawn()
        .expect("CMD to run");

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            println!("{}", line.unwrap());
        }
    }

    cmd.wait().unwrap();
}

fn get_cmd(name: &String) -> String {
    let cmd_toml_str = fs::read_to_string(format!(
        "{}/Cargo.toml",
        env::var("CARGO_MANIFEST_DIR").unwrap_or("./".to_string())
    ))
    .unwrap();

    let cmd_toml: Cargotoml = toml::from_str(&cmd_toml_str[..]).expect("ASD");

    let cargo_commands = cmd_toml.dev;
    let specific_command = cargo_commands.get(name).expect("ASD");

    specific_command.to_string()
}
