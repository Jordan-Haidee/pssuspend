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
    suspend_status == 0
}
fn resume_process(pid: u32) -> bool {
    let process_handle = unsafe { OpenProcess(PROCESS_SUSPEND_RESUME, 0, pid) };
    if process_handle == NULL {
        return false;
    }
    let resume_status = unsafe { NtResumeProcess(process_handle) };
    resume_status == 0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // dbg!(&args);
    match args.len() {
        2 => {
            let pid = args[1].parse::<u32>().expect("failed to parse pid!");
            if suspend_process(pid) {
                println!("process {} has been suspended successfully!", pid);
            } else {
                println!("failed to suspend process {}", pid);
            }
        }
        3 => {
            assert!(args[1].as_str() == "-r");
            let pid = args[2].parse::<u32>().expect("failed to parse pid!");
            if resume_process(pid) {
                println!("process {} has been resumed successfully!", pid);
            } else {
                println!("failed to resume process {}", pid);
            }
        }
        _ => println!("uncorrect input args!"),
    }
}
