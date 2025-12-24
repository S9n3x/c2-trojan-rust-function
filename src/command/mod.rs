use std::process::Command;

pub fn command() {
    let output = Command::new("dir").output().expect("执行异常，提示");
    let ls_list = String::from_utf8(output.stdout);
    print!("{}", ls_list.unwrap());
}

pub fn win_command(cmd: &str) -> Vec<u8> {
    let output = Command::new("cmd")
        .args(["/C", cmd])
        .output()
        .expect("执行异常,提示");
    output.stdout
}
