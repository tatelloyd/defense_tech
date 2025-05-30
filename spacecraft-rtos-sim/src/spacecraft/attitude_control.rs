use std::time::{Duration, Instant};
use std::sync::Mutex;
use std::sync::OnceLock;
use crate::rtos::task::TaskResult;
use crate::drivers::sensor_driver::read_imu;
use crate::utils::math::*;

// Use a safe static with Mutex instead of static mut
static ATTITUDE_STATE: OnceLock<Mutex<AttitudeState>> = OnceLock::new();

#[derive(Debug)]
struct AttitudeState {
    roll: f64,
    pitch: f64, 
    yaw: f64,
    angular_rates: [f64; 3],
    last_update: Option<Instant>,
}

impl AttitudeState {
    fn new() -> Self {
        Self {
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
            angular_rates: [0.0; 3],
            last_update: None,
        }
    }
}

fn get_attitude_state() -> &'static Mutex<AttitudeState> {
    ATTITUDE_STATE.get_or_init(|| Mutex::new(AttitudeState::new()))
}

pub fn run() -> TaskResult {
    let start = Instant::now();
    
    // Get the attitude state mutex
    let attitude_mutex = get_attitude_state();
    
    // Lock the mutex for the duration of our operations
    let mut attitude_state = match attitude_mutex.lock() {
        Ok(state) => state,
        Err(_) => return TaskResult::Error("Failed to lock attitude state".to_string()),
    };
    
    // Read IMU data (simulated)
    let imu_data = read_imu();
    
    // Calculate time delta
    let now = Instant::now();
    let dt = if let Some(last) = attitude_state.last_update {
        (now - last).as_secs_f64()
    } else {
        0.001 // 1ms initial
    };
    
    // Simple attitude integration (normally would use quaternions)
    attitude_state.roll += imu_data.gyro[0] * dt;
    attitude_state.pitch += imu_data.gyro[1] * dt;  
    attitude_state.yaw += imu_data.gyro[2] * dt;
    
    // Store angular rates
    attitude_state.angular_rates = imu_data.gyro;
    attitude_state.last_update = Some(now);
    
    // Simulate control output (would drive actuators)
    let control_torque = calculate_control_torque(&attitude_state);
    
    // Drop the lock before checking execution time
    drop(attitude_state);
    
    // Critical: This must complete within 1ms
    let execution_time = start.elapsed();
    if execution_time > Duration::from_millis(1) {
        return TaskResult::Error(format!("Attitude control overrun: {:?}", execution_time));
    }
    
    TaskResult::Success(start.elapsed())
}

fn calculate_control_torque(state: &AttitudeState) -> [f64; 3] {
    // Simple PID controller simulation
    let kp = 0.5;
    let target_attitude = [0.0, 0.0, 0.0]; // Level flight
    
    [
        kp * (target_attitude[0] - state.roll),
        kp * (target_attitude[1] - state.pitch), 
        kp * (target_attitude[2] - state.yaw),
    ]
}