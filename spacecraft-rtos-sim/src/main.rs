use std::time::Duration;
use tokio::time;

mod rtos;
mod spacecraft;
mod drivers;
mod utils;

use rtos::scheduler::Scheduler;
use spacecraft::*;

#[tokio::main]
async fn main() {
    println!("🚀 Spacecraft RTOS Simulator Starting...");

    // Initialize the RTOS scheduler
    let mut scheduler = Scheduler::new();

    // Create spacecraft subsystem tasks
    scheduler.create_task(
        "attitude_control",
        1, // Highest priority
        Duration::from_millis(1),
        attitude_control::run,
    );
    
    scheduler.create_task(
        "navigation",
        2,
        Duration::from_millis(10),
        navigation::run,
    );
    
    scheduler.create_task(
        "telemetry",
        3,
        Duration::from_millis(100),
        telemetry::run,
    );
    
    scheduler.create_task(
        "housekeeping", 
        4, // Lowest priority
        Duration::from_millis(1000),
        housekeeping::run,
    );

    // Run the RTOS for 30 seconds
    println!("🎯 RTOS running with {} tasks", scheduler.task_count());
    scheduler.run(Duration::from_secs(30)).await;

    println!("📊 Final Performance Metrics:");
    scheduler.print_metrics();
}
