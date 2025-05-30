use std::time::Duration;

#[derive(Debug)]
pub struct SystemMetrics {
    total_loop_time: Duration,
    loop_count: u64,
    pub max_loop_time: Duration,
}

impl SystemMetrics {
    pub fn new() -> Self {
        Self {
            total_loop_time: Duration::ZERO,
            loop_count: 0,
            max_loop_time: Duration::ZERO,
        }
    }
    
    pub fn update_loop_time(&mut self, loop_time: Duration) {
        self.total_loop_time += loop_time;
        self.loop_count += 1;
        
        if loop_time > self.max_loop_time {
            self.max_loop_time = loop_time;
        }
    }
    
    pub fn avg_loop_time(&self) -> Duration {
        if self.loop_count > 0 {
            self.total_loop_time / self.loop_count as u32
        } else {
            Duration::ZERO
        }
    }
}