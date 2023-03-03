use clap::Parser;
use sysinfo::{System, SystemExt};
use std::{thread, time::Duration, process::Command};

#[derive(Parser)]
struct Cli {
    process_name: String,
    app_path: std::path::PathBuf,
    timeout: u64,
}

fn main() {
    let args = Cli::parse();
    if !args.app_path.exists() {
        println!("The file {} does not exist!", args.app_path.display());
        return;
    }
    let current_dir = args.app_path.parent().expect("The path does not have a parent directory.");
    let app_name = args.app_path.file_name().expect("The path does not have a file name.");
    let mut s = System::new_all();
    loop {
        s.refresh_processes();
        if s.processes_by_exact_name(&args.process_name).count() == 0 {
            Command::new("open")
                .arg(app_name)
                .current_dir(current_dir)
                .status()
                .expect("Failed to run process.");
            println!("Launching {}", app_name.to_str().unwrap());
        }
        thread::sleep(Duration::from_secs(args.timeout));
    }
}
