use process_memory::{DataMember, Memory, TryIntoProcessHandle};
use sysinfo::{ProcessExt, SystemExt};

pub fn get_pid_by_name(name: &str) -> Option<i32> {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    let pid = system.processes_by_name(name).map(|p| p.pid()).collect::<Vec<_>>();

    match pid.get(0) {
        Some(pid) => {
            let pid = pid.to_string().parse::<i32>().unwrap();
            Some(pid)
        },
        None => None
    }
}

pub fn get_mem_value(pid: i32, mem_addr: usize) -> i32 {
    unsafe {
        let handle = ( pid as process_memory::Pid ).try_into_process_handle().unwrap();

        let mut member = DataMember::new(handle);

        member.set_offset(vec![mem_addr]);

        let value: i32 = member.read().unwrap();

        return value;
    }
}

pub fn write_mem_value(pid: i32, mem_addr: usize, value: i32) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let handle = match ( pid as process_memory::Pid ).try_into_process_handle() {
            Ok(handle) => handle,
            Err(e) => return Err(Box::new(e))
        };

        let mut member = DataMember::new(handle);

        member.set_offset(vec![mem_addr]);

        match member.write(&value) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e))
        }
    }
}