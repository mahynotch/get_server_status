use sysinfo::{System, SystemExt};
use psutil::cpu;
use std::{thread, time};

fn main() {
    let mut sys = System::new_all();
    // let memory_usage = (used_memory as f64 / total_memory as f64) * 100.0;
    let mut cpu_percent_collector = cpu::CpuPercentCollector::new().unwrap();
    loop {
    sys.refresh_all();
    let mut cpu_percents = cpu_percent_collector.cpu_percent().unwrap();
    println!(
        "Mem: {:.2}GB/{:.2}GB, CPU: {:.2}%",
        sys.used_memory() as f64 / 1024.0 / 1024.0,
        sys.total_memory() as f64 / 1024.0 / 1024.0,
        cpu_percents as f64,
    );
    thread::sleep(time::Duration::from_millis(1000));
    }
}
