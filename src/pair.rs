use crate::mount::mount_device;
use crate::notification::paired_failed;
use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
    process::Command,
};
pub fn pair_and_mount(mount_dir: &PathBuf) -> Result<(), Error> {
    let pair = Command::new("idevicepair").arg("pair").output();

    match pair {
        Ok(process) => match process.status.code() {
            Some(0) => {
                mount_device(&mount_dir);
                Ok(())
            }
            Some(_) => {
                println!("Failed to pair your device.");

                let exit_message = String::from_utf8_lossy(&process.stdout);
                if exit_message.contains("No device found.") {
                    println!("Please connect your device");
                }
                if exit_message
                    .contains("ERROR: Please accept the trust dialog on the screen of device")
                {
                    println!("Please accept the trust dialog on the screen of your device");
                    paired_failed(&mount_dir);
                }
                Ok(())
            }
            None => {
                println!("Bad status: no status response.");
                Err(Error::new(ErrorKind::Other, "No status code response"))
            }
        },
        Err(e) => {
            eprintln!("Error pairing device. {}", &e);
            Err(e)
        }
    }
}
