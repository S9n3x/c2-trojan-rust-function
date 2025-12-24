use sysinfo::System;

#[derive(Debug)]
pub struct Process {
    pid: u32,
    name: String,
    user: String,
    cpu_usage: f32,
    memory_usage: u64,
}

pub fn list_processes() -> Vec<Process> {
    let mut system = System::new_all();
    system.refresh_all();

    let mut result = Vec::new();

    for (pid, proc_) in system.processes().iter() {
        result.push(Process {
            pid: pid.as_u32(),
            name: proc_.name().to_str().unwrap_or("").to_string(),
            user: proc_.user_id().map_or("".to_string(), |u| u.to_string()),
            cpu_usage: proc_.cpu_usage(),
            memory_usage: proc_.memory(),
        });
    }

    result
}
