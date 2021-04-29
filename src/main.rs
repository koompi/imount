pub mod known;
pub mod mount;
pub mod notification;
pub mod pair;
pub mod unmount;
pub mod unpair;

use mount::mount_device;
use pair::pair_and_mount;
use std::env;
use subprocess::{Exec, NullFile};
use unmount::unmount_device;

fn main() {
    #[allow(deprecated)]
    let mount_dir = env::home_dir().unwrap().join("my_device");
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("Help");
    } else {
        match args[0].as_ref() {
            "autostart" => {
                let command = r#"
                sudo systemctl enable --now imount-daemon.service
                sudo modprobe fuse
                echo -e fuse | sudo tee /etc/modules-load.d/imount.conf
                "#;
                Exec::shell(command)
                    .detached()
                    .stdout(NullFile)
                    .capture()
                    .unwrap();
            }
            "connect" => {
                if let Some(_) = env::var_os("PAIRED") {
                    println!("Some");
                    mount_device(&mount_dir);
                }
                if let None = env::var_os("PAIRED") {
                    println!("None");
                    pair_and_mount(&mount_dir).unwrap();
                }
            }
            "disconnect" => {
                unmount_device(&mount_dir);
            }
            _ => {
                println!("Help");
            }
        }
    }
}
