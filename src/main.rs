// use std::thread;

use windows::{
    core::PCSTR,
    Win32::{
        Foundation::BOOL,
        Security::SECURITY_ATTRIBUTES,
        System::{
            JobObjects::{
                self, AssignProcessToJobObject, IsProcessInJob, JobObjectCpuRateControlInformation,
                SetInformationJobObject, JOBOBJECT_CPU_RATE_CONTROL_INFORMATION,
                JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0, JOB_OBJECT_CPU_RATE_CONTROL_ENABLE,
                JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP,
            },
            Threading::GetCurrentProcess,
        },
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        control_cpu_rate()?;
    }
    Ok(())
}

unsafe fn control_cpu_rate() -> Result<(), Box<dyn std::error::Error>> {
    let ps = PCSTR::from_raw(b"test\0".as_ptr()); // pcstr
    let sa = &SECURITY_ATTRIBUTES::default() as *const SECURITY_ATTRIBUTES;
    let job = JobObjects::CreateJobObjectA::<Option<PCSTR>>(sa, Some(ps))?;

    let cpu_info = JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
        ControlFlags: JOB_OBJECT_CPU_RATE_CONTROL_ENABLE | JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP,
        Anonymous: JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 { CpuRate: 20 * 100 },
    };

    SetInformationJobObject(
        job,
        JobObjectCpuRateControlInformation,
        &cpu_info as *const JOBOBJECT_CPU_RATE_CONTROL_INFORMATION as *const _,
        std::mem::size_of_val(&cpu_info) as u32,
    );

    let cur = GetCurrentProcess();
    let mut process_in_job = BOOL::default();
    println!("bInJob: {process_in_job:?}");
    match AssignProcessToJobObject(job, cur).as_bool() {
        true => {
            println!("assign process to job object success");
        }
        false => {
            println!("AssignProcessToJobObject failed");
        }
    };
    IsProcessInJob(cur, job, &mut process_in_job);
    match process_in_job.as_bool() {
        true => {
            println!("handle is in job")
        }
        false => {
            println!("handle is not in job")
        }
    };
    let mut a = 1u64;
    loop {
        if a == u64::MAX {
            a = 0;
        }
        a += 1;
    }

    Ok(())
}
