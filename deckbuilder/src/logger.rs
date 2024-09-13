pub struct GameLogger {
    log: Vec<String>,
}

impl GameLogger {
    pub fn new() -> Self {
        Self { log: Vec::new() }
    }

    pub fn add_entry(&mut self, entry: String) {
        let entry_clone = entry.clone();
        self.log.push(entry);
        println!("{}", entry_clone); // Print to console immediately
    }

    pub fn get_log(&self) -> &[String] {
        &self.log
    }
}
