use std::time::{Duration, Instant};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub priority: u8, // Lower number = higher priority
    pub period: Duration,
    pub last_run: Option<Instant>,
    pub next_run: Instant,
    pub state: TaskState,
    pub run_function: fn() -> TaskResult,
    pub metrics: TaskMetrics,
}

#[derive(Debug, Default)]
pub struct TaskMetrics {
    pub executions: u64,
    pub total_runtime: Duration,
    pub max_runtime: Duration,
    pub missed_deadlines: u32,
    pub avg_jitter: Duration,
}

#[derive(Debug)]
pub enum TaskResult {
    Success(Duration), // Execution time
    Error(String),
    Blocked,
}

impl Task {
    pub fn new(
        id: String,
        priority: u8,
        period: Duration,
        run_function: fn() -> TaskResult,
    ) -> Self {
        Self {
            id,
            priority,
            period,
            last_run: None,
            next_run: Instant::now(),
            state: TaskState::Ready,
            run_function,
            metrics: TaskMetrics::default(),
        }
    }
    
    pub fn should_run(&self) -> bool {
        Instant::now() >= self.next_run && matches!(self.state, TaskState::Ready)
    }
    
    pub fn execute(&mut self) -> TaskResult {
        let start = Instant::now();
        self.state = TaskState::Running;
        
        let result = (self.run_function)();
        
        let execution_time = start.elapsed();
        self.update_metrics(execution_time);
        
        // Schedule next execution
        self.last_run = Some(start);
        self.next_run = start + self.period;
        self.state = TaskState::Ready;
        
        result
    }
    
    fn update_metrics(&mut self, execution_time: Duration) {
        self.metrics.executions += 1;
        self.metrics.total_runtime += execution_time;
        
        if execution_time > self.metrics.max_runtime {
            self.metrics.max_runtime = execution_time;
        }
        
        // Check for missed deadline
        if execution_time > self.period {
            self.metrics.missed_deadlines += 1;
        }
    }
    
    pub fn avg_execution_time(&self) -> Duration {
        if self.metrics.executions > 0 {
            self.metrics.total_runtime / self.metrics.executions as u32
        } else {
            Duration::ZERO
        }
    }
}

