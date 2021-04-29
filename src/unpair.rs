use std::process::Command;

pub fn unpair_device() {
    let unmount = Command::new("idevicepair").arg("unpair").output();

    match unmount {
        Ok(process) => match process.status.code() {
            Some(0) => {
                println!("Successfully unpaired your device.");
            }
            Some(_) => {
                println!(
                    "Error unpairing your device with status code: {}",
                    &process.status.code().unwrap()
                );
                std::process::exit(1);
            }
            None => {
                println!("Error unpairing device with no status response.");
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error unmounting your device.");
            eprintln!("{}", e)
        }
    }
}
