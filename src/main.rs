mod commands;
mod parser;

use std::env;
use std::fs::File;
use std::path::Path;
use std::os::unix::io::AsRawFd;

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => {
            println!();
            println!("Exited successfully");
            0
        },
        Err(err) => {
            eprintln!("ERROR :: {:?}", err);
            1
        }
    });
}

fn run_app() -> Result<(), ()> {
    // Collect command-line arguments
    let args : Vec<String> = env::args().collect();
    let drive_path = Path::new(&args[1]);
    // Open the file given for desired drive
    let sg = match File::open(&drive_path) {
        Err(err) => panic!("Could not open {}: {}", drive_path.display(), err),
        Ok(file) => file,
    };
    let sg_fd : i32 = sg.as_raw_fd();
    // Check if sg device
    commands::scsi_send_get_version_number(sg_fd, &drive_path);
    // Send inquiry command
    commands::scsi_send_inquiry(sg_fd, &drive_path);
    Ok(())
}