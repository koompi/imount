use crate::notification::disconnected;
use std::{path::PathBuf, process::Command};
pub fn unmount_device(mount_dir: &PathBuf) {
    let unmount = Command::new("fusermount")
        .arg("-u")
        .arg(mount_dir.to_str().unwrap())
        .output();

    match unmount {
        Ok(process) => match process.status.code() {
            Some(0) => {
                // println!("Successfully unmounted your device.");
                disconnected();
            }
            Some(_) => {
                println!(
                    "Error unmounting your device with status code: {}",
                    &process.status.code().unwrap()
                );
                std::process::exit(1);
            }
            None => {
                println!("Error unmounting device with no status response.");
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error unmounting your device.");
            eprintln!("{}", e)
        }
    }
}
