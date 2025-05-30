use rand::Rng;

#[derive(Debug, Clone)]
pub struct ImuData {
    pub accel: [f64; 3], // m/s²
    pub gyro: [f64; 3],  // rad/s
    pub mag: [f64; 3],   // Tesla
}

pub fn read_imu() -> ImuData {
    let mut rng = rand::thread_rng();
    
    ImuData {
        accel: [
            rng.gen_range(-0.1..0.1), // Small random accelerations
            rng.gen_range(-0.1..0.1),
            -9.81 + rng.gen_range(-0.5..0.5), // Earth gravity + noise
        ],
        gyro: [
            rng.gen_range(-0.01..0.01), // Small random rotations
            rng.gen_range(-0.01..0.01),
            rng.gen_range(-0.01..0.01),
        ],
        mag: [
            25.0 + rng.gen_range(-2.0..2.0), // Earth's magnetic field + noise
            0.0 + rng.gen_range(-1.0..1.0),
            43.0 + rng.gen_range(-2.0..2.0),
        ],
    }
}
