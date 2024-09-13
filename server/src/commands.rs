use once_cell::sync::Lazy;
use shared::TaskType;
use std::collections::HashMap;
use std::sync::Mutex;

/// The commands queue
static COMMANDS: Lazy<Mutex<HashMap<u128, TaskType>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Add a command to the queue
pub fn add_command(collector_id: u128, command: TaskType) {
    let mut commands = COMMANDS.lock().unwrap();
    commands.insert(collector_id, command);
}

/// Get a command from the queue
pub fn get_commands(collector_id: u128) -> Option<TaskType> {
    let mut commands = COMMANDS.lock().unwrap();
    commands.remove(&collector_id)
}
