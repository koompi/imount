use imount::is_known;
use libc::{c_int, c_short, c_ulong, c_void};
use std::{io, os::unix::io::AsRawFd, ptr, thread, time::Duration};
use subprocess::{Exec, NullFile};
#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

#[repr(C)]
struct sigset_t {
    __private: c_void,
}

#[allow(non_camel_case_types)]
type nfds_t = c_ulong;

const POLLIN: c_short = 0x0001;

extern "C" {
    fn ppoll(
        fds: *mut pollfd,
        nfds: nfds_t,
        timeout_ts: *mut libc::timespec,
        sigmask: *const sigset_t,
    ) -> c_int;
}

fn main() -> io::Result<()> {
    let mut socket = udev::MonitorBuilder::new()?
        .match_subsystem_devtype("usb", "usb_device")?
        .listen()?;

    let mut fds = vec![pollfd {
        fd: socket.as_raw_fd(),
        events: POLLIN,
        revents: 0,
    }];

    let mut event_id = String::new();

    loop {
        let result = unsafe {
            ppoll(
                (&mut fds[..]).as_mut_ptr(),
                fds.len() as nfds_t,
                ptr::null_mut(),
                ptr::null(),
            )
        };

        if result < 0 {
            return Err(io::Error::last_os_error());
        }

        let event = match socket.next() {
            Some(evt) => evt,
            None => {
                thread::sleep(Duration::from_millis(100));
                continue;
            }
        };

        let bin_path = if cfg!(debug_assertions) {
            "./target/debug/imount"
        } else {
            "/usr/bin/imount"
        };

        if event.event_type() == udev::EventType::Bind {
            if let Some(ip) = event.property_value("ID_VENDOR_ID") {
                if ip == "05ac" {
                    println!("Apple_Inc. device detected");
                    thread::sleep(Duration::from_millis(1000));
                    println!(
                        "{}",
                        event
                            .property_value("ID_MODEL_FROM_DATABASE")
                            .unwrap()
                            .to_str()
                            .unwrap()
                    );
                    let usec = event.property_value("USEC_INITIALIZED").unwrap();
                    event_id = usec.to_str().unwrap().to_string();

                    let device_id = event.property_value("ID_SERIAL_SHORT").unwrap();

                    if is_known(device_id.to_str().unwrap()) {
                        Exec::cmd(bin_path)
                            .arg("--connect")
                            .env("PAIRED", "true")
                            .detached()
                            .stdout(NullFile)
                            .capture()
                            .unwrap();
                    } else {
                        println!("pair and connect");
                        Exec::cmd(bin_path)
                            .arg("--connect")
                            .detached()
                            .stdout(NullFile)
                            .capture()
                            .unwrap();
                    }
                }
            }
        }

        if event.event_type() == udev::EventType::Remove {
            if let Some(evid) = event.property_value("USEC_INITIALIZED") {
                if evid.to_str().unwrap().to_string() == event_id {
                    println!("Unmount device");
                    Exec::cmd(bin_path)
                        .arg("-d")
                        .detached()
                        .stdout(NullFile)
                        .capture()
                        .unwrap();
                }
            }
        }
    }
}
