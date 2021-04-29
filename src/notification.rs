use crate::mount_device;
use notify_rust::{Notification, Timeout};
use std::path::PathBuf;
use subprocess::{Exec, NullFile};
pub fn disconnected() {
    Notification::new()
        .summary("Disconnect")
        .body("Your device is disconnected.")
        .icon("ejecter")
        .timeout(Timeout::Milliseconds(6000))
        .show()
        .unwrap();
}

pub fn paired_failed(mount_dir: &PathBuf) {
    Notification::new()
        .summary("USB Connection")
        .body("Please confirm \"Trust\" on your device dialog, and click continue")
        .icon("dialog-error")
        .action("default", "default")
        .action("continue", "Continue")
        .timeout(Timeout::Never)
        .show()
        .unwrap()
        .wait_for_action(|action| match action {
            "default" | "continue" => {
                mount_device(&mount_dir);
            }
            "__closed" => (),
            _ => (),
        });
}

pub fn connected(mount_dir: &PathBuf) {
    Notification::new()
        .summary("USB Connection")
        .body("Your Apple device is successfully connected.")
        .icon("smartphone")
        .action("default", "default")
        .action("clicked", "Open Folder")
        .timeout(Timeout::Milliseconds(6000))
        .show()
        .unwrap()
        .wait_for_action(|action| match action {
            "default" | "clicked" => {
                Exec::cmd("xdg-open")
                    .arg(mount_dir)
                    .detached()
                    .stdout(NullFile)
                    .capture()
                    .unwrap();
            }
            "__closed" => (),
            _ => (),
        });
}
