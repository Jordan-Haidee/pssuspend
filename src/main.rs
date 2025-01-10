use ntapi::ntpsapi::{NtResumeProcess, NtSuspendProcess};
use winapi::{
    shared::ntdef::NULL,
    um::{processthreadsapi::OpenProcess, winnt::PROCESS_SUSPEND_RESUME},
};

fn suspend_process(pid: u32) -> bool {
    let process_handle = unsafe { OpenProcess(PROCESS_SUSPEND_RESUME, 0, pid) };
    if process_handle == NULL {
        return false;
    }
    let suspend_status = unsafe { NtSuspendProcess(process_handle) };
    if suspend_status != 0 {
        return false;
    }
    println!("Process {} has been suspended successfully!", pid);
    return true;
}
fn resume_process(pid: u32) -> bool {
    let process_handle = unsafe { OpenProcess(PROCESS_SUSPEND_RESUME, 0, pid) };

    if process_handle == NULL {
        return false;
    }
    let suspend_status = unsafe { NtResumeProcess(process_handle) };
    if suspend_status != 0 {
        return false;
    }
    println!("Process {} has been resumed successfully!", pid);
    return true;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // dbg!(&args);
    match args.len() {
        2 => {
            let pid = args[1].parse::<u32>().expect("failed to parse pid!");
            if !suspend_process(pid) {
                println!("failed to suspend process");
            }
        }
        3 => {
            assert!(args[1].as_str() == "-r");
            let pid = args[2].parse::<u32>().expect("failed to parse pid!");
            if !resume_process(pid) {
                println!("failed to resume process");
            }
        }
        _ => println!("uncorrect input args!"),
    }
}