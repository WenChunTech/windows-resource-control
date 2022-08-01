use std::thread;

use windows::{
    core::PCSTR,
    Win32::{
        Security::SECURITY_ATTRIBUTES,
        System::{
            JobObjects::{
                self, AssignProcessToJobObject, JobObjectCpuRateControlInformation,
                SetInformationJobObject, JOBOBJECT_CPU_RATE_CONTROL_INFORMATION,
                JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0,
                JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0, JOB_OBJECT_CPU_RATE_CONTROL_ENABLE,
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
    let mut cpu_rate_info: JOBOBJECT_CPU_RATE_CONTROL_INFORMATION =
        JOBOBJECT_CPU_RATE_CONTROL_INFORMATION::default();

    cpu_rate_info.ControlFlags =
        JOB_OBJECT_CPU_RATE_CONTROL_ENABLE | JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP;

    let mut anonymous = JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0::default();
    anonymous.CpuRate = 50 * 100;
    anonymous.Weight = 5;
    anonymous.Anonymous = JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0::default();

    cpu_rate_info.Anonymous = anonymous;

    SetInformationJobObject(
        job,
        JobObjectCpuRateControlInformation,
        &cpu_rate_info as *const JOBOBJECT_CPU_RATE_CONTROL_INFORMATION as *const _,
        // std::mem::size_of::<JOBOBJECT_CPU_RATE_CONTROL_INFORMATION>() as u32,
        std::mem::size_of_val(&cpu_rate_info) as u32,
    );
    let cur = GetCurrentProcess();
    match AssignProcessToJobObject(job, cur).as_bool() {
        true => {
            println!("assign process to job object success");
        }
        false => {
            println!("AssignProcessToJobObject failed");
        }
    };
    let mut a = 1u64;
    let mut handles = Vec::new();
    for _ in 0..6400 {
        let handle = thread::spawn(move || loop {
            if a == u64::MAX {
                a = 0;
            }
            a += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}
