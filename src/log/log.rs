use std::time::Instant;

pub struct Log {
    time: Instant,
    action: String
}

impl Log {
    pub fn new(time: Instant, action: String) -> Self {
        Log {
            time, 
            action
        }
    }
    
    pub fn add_to_log_window(&self) {
        
    }
}

