// Fixed src/spacecraft/telemetry.rs

use std::time::Instant;
use std::sync::Mutex;
use std::sync::OnceLock;
use crate::rtos::task::TaskResult;

// Use safe static instead of static mut
static TELEMETRY_PACKET_COUNT: OnceLock<Mutex<u64>> = OnceLock::new();

fn get_packet_count() -> &'static Mutex<u64> {
    TELEMETRY_PACKET_COUNT.get_or_init(|| Mutex::new(0))
}

pub fn run() -> TaskResult {
    let start = Instant::now();
    
    // Get and increment packet count safely
    let packet_count_mutex = get_packet_count();
    let mut packet_count = match packet_count_mutex.lock() {
        Ok(count) => count,
        Err(_) => return TaskResult::Error("Failed to lock packet count".to_string()),
    };
    
    *packet_count += 1;
    let current_count = *packet_count;
    
    // Release the lock early
    drop(packet_count);
    
    // Simulate telemetry collection
    let battery_voltage = simulate_battery_reading(current_count);
    let temperature = simulate_temperature_reading();
    let memory_usage = simulate_memory_usage();
    
    // In real system, would transmit to ground
    if current_count % 10 == 0 {
        println!("📡 Telemetry #{}: Battery: {:.1}V, Temp: {:.1}°C, Mem: {:.1}%", 
                 current_count, battery_voltage, temperature, memory_usage);
    }
    
    TaskResult::Success(start.elapsed())
}

fn simulate_battery_reading(packet_count: u64) -> f64 {
    // Simulate slowly draining battery
    28.0 - (packet_count as f64 * 0.001)
}

fn simulate_temperature_reading() -> f64 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    20.0 + rng.r#gen::<f64>() * 10.0 // 20-30°C
}

fn simulate_memory_usage() -> f64 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    30.0 + rng.r#gen::<f64>() * 40.0 // 30-70%
}