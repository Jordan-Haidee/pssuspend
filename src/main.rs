use clap::{value_parser, Arg, ArgAction, Command};
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
    let matches = Command::new("pssuspend")
        .about("An alternative to sysinternals/pssuspend tool on Windows platform.")
        .arg(
            Arg::new("resume")
                .short('r')
                .long("resume")
                .action(ArgAction::SetTrue)
                .help("whether to resume the process."),
        )
        .arg(
            Arg::new("pid")
                .required(true)
                .value_parser(value_parser!(u32))
                .index(1)
                .help("the process id to be suspended or resumed."),
        )
        .get_matches();
    let resume_flag = matches.get_flag("resume");
    if let Some(pid) = matches.get_one::<u32>("pid") {
        if resume_flag {
            if resume_process(*pid) {
                println!("process {} has been resumed successfully!", pid);
            } else {
                println!("failed to resume process {}", pid);
            }
        } else {
            if suspend_process(*pid) {
                println!("process {} has been suspended successfully!", pid);
            } else {
                println!("failed to suspend process {}", pid);
            }
        }
    }
}
