use std::process::Command;

pub fn is_known(target: &str) -> bool {
    let known_list = Command::new("idevicepair").arg("list").output();
    let raw_list = known_list.unwrap().stdout;
    let string_list = String::from_utf8_lossy(&raw_list);
    let dev_ids = string_list.lines().find(|&x| x == target);

    if let Some(_) = dev_ids {
        return true;
    }
    false
}
