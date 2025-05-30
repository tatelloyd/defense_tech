use std::time::Instant;
use std::sync::Mutex;
use std::sync::OnceLock;
use crate::rtos::task::TaskResult;

static HOUSEKEEPING_CYCLES: OnceLock<Mutex<u32>> = OnceLock::new();

fn get_housekeeping_cycles() -> &'static Mutex<u32> {
    HOUSEKEEPING_CYCLES.get_or_init(|| Mutex::new(0))
}

pub fn run() -> TaskResult {
    let start = Instant::now();
    
    // Get and increment cycle count safely
    let cycles_mutex = get_housekeeping_cycles();
    let mut cycles = match cycles_mutex.lock() {
        Ok(count) => count,
        Err(_) => return TaskResult::Error("Failed to lock housekeeping cycles".to_string()),
    };
    
    *cycles += 1;
    let current_cycles = *cycles;
    
    // Release the lock early
    drop(cycles);
    
    // Simulate low-priority maintenance tasks
    perform_memory_cleanup();
    check_system_health();
    update_flight_software_logs();
    
    if current_cycles % 10 == 0 {
        println!("🧹 Housekeeping cycle #{}: System health OK", current_cycles);
    }
    
    TaskResult::Success(start.elapsed())
}

fn perform_memory_cleanup() {
    // Simulate memory defragmentation
    std::thread::sleep(std::time::Duration::from_micros(100));
}

fn check_system_health() {
    // Simulate health monitoring
    std::thread::sleep(std::time::Duration::from_micros(200));
}

fn update_flight_software_logs() {
    // Simulate log rotation/cleanup
    std::thread::sleep(std::time::Duration::from_micros(150));
}