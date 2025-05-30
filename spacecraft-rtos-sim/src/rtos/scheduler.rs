use std::collections::{BinaryHeap, HashMap};
use std::time::{Duration, Instant};
use std::cmp::Ordering;
use tokio::time;

use super::task::{Task, TaskResult, TaskState};
use crate::utils::metrics::SystemMetrics;

#[derive(Debug)]
pub struct Scheduler {
    tasks: HashMap<String, Task>,
    ready_queue: BinaryHeap<TaskHandle>,
    system_metrics: SystemMetrics,
    start_time: Instant,
}

#[derive(Debug, Clone)]
struct TaskHandle {
    id: String,
    priority: u8,
    next_run: Instant,
}

impl PartialEq for TaskHandle {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.next_run == other.next_run
    }
}

impl Eq for TaskHandle {}

impl PartialOrd for TaskHandle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TaskHandle {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority tasks (lower numbers) come first
        // If priority is equal, earlier deadline comes first
        other.priority.cmp(&self.priority)
            .then_with(|| self.next_run.cmp(&other.next_run))
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            ready_queue: BinaryHeap::new(),
            system_metrics: SystemMetrics::new(),
            start_time: Instant::now(),
        }
    }
    
    pub fn create_task(
        &mut self,
        id: &str,
        priority: u8,
        period: Duration,
        run_function: fn() -> TaskResult,
    ) {
        let task = Task::new(id.to_string(), priority, period, run_function);
        
        let handle = TaskHandle {
            id: id.to_string(),
            priority,
            next_run: task.next_run,
        };
        
        self.tasks.insert(id.to_string(), task);
        self.ready_queue.push(handle);
        
        println!("📋 Created task '{}' with priority {} and period {:?}", 
                 id, priority, period);
    }
    

pub async fn run(&mut self, duration: Duration) {
    let end_time = Instant::now() + duration;
    let mut tick_count = 0u64;
    
    while Instant::now() < end_time {
        let loop_start = Instant::now();
        
        // Update ready queue with tasks that should run
        self.update_ready_queue();
        
        // Execute highest priority ready task
        // Fix: Store the handle first to avoid multiple mutable borrows
        let handle_opt = self.ready_queue.pop();
        if let Some(handle) = handle_opt {
            // Now we can safely borrow self.tasks mutably
            if let Some(task) = self.tasks.get_mut(&handle.id) {
                if task.should_run() {
                    let result = task.execute();
                    let task_id = handle.id.clone(); // Clone the ID before moving handle
                    self.handle_task_result(&task_id, result);
                    
                    // Re-queue the task for its next execution
                    // Get the task again to access its next_run time
                    if let Some(task) = self.tasks.get(&handle.id) {
                        let new_handle = TaskHandle {
                            id: handle.id,
                            priority: handle.priority,
                            next_run: task.next_run,
                        };
                        self.ready_queue.push(new_handle);
                    }
                }
            }
        }
        
        // Update system metrics
        tick_count += 1;
        let loop_time = loop_start.elapsed();
        self.system_metrics.update_loop_time(loop_time);
        
        // Print status every 1000 ticks
        if tick_count % 1000 == 0 {
            self.print_status();
        }
        
        // Small delay to prevent 100% CPU usage
        time::sleep(Duration::from_micros(100)).await;
    }
    
    println!("🏁 RTOS simulation completed after {} ticks", tick_count);
}

     // Here
    
    fn update_ready_queue(&mut self) {
        let now = Instant::now();
        
        // Check all tasks and add ready ones to queue
        for (id, task) in &self.tasks {
            if task.should_run() && task.next_run <= now {
                let handle = TaskHandle {
                    id: id.clone(),
                    priority: task.priority,
                    next_run: task.next_run,
                };
                // Note: In a real RTOS, we'd check if already queued
                // For simplicity, we allow duplicates here
            }
        }
    }
    
    fn handle_task_result(&mut self, task_id: &str, result: TaskResult) {
        match result {
            TaskResult::Success(execution_time) => {
                if execution_time > Duration::from_millis(10) {
                    println!("⚠️  Task '{}' took {:?} (long execution)", task_id, execution_time);
                }
            }
            TaskResult::Error(msg) => {
                println!("❌ Task '{}' error: {}", task_id, msg);
            }
            TaskResult::Blocked => {
                println!("🔒 Task '{}' blocked", task_id);
            }
        }
    }
    
    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }
    
    fn print_status(&self) {
        let uptime = self.start_time.elapsed();
        println!("⏱️  Uptime: {:?} | Ready queue: {} | Avg loop time: {:?}", 
                 uptime, 
                 self.ready_queue.len(),
                 self.system_metrics.avg_loop_time());
    }
    
    pub fn print_metrics(&self) {
        println!("\n📈 TASK PERFORMANCE METRICS:");
        println!("{:-<80}", "");
        
        for (id, task) in &self.tasks {
            println!("Task: {}", id);
            println!("  Executions: {}", task.metrics.executions);
            println!("  Avg Runtime: {:?}", task.avg_execution_time());
            println!("  Max Runtime: {:?}", task.metrics.max_runtime);
            println!("  Missed Deadlines: {}", task.metrics.missed_deadlines);
            println!("  Priority: {}", task.priority);
            println!();
        }
        
        println!("System Metrics:");
        println!("  Total Runtime: {:?}", self.start_time.elapsed());
        println!("  Avg Loop Time: {:?}", self.system_metrics.avg_loop_time());
        println!("  Max Loop Time: {:?}", self.system_metrics.max_loop_time);
    }
}