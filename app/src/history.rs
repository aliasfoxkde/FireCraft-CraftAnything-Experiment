#[derive(Debug, Clone)]
pub struct History {
    pub states: Vec<String>,
    pub index: usize,
}

impl History {
    pub fn new() -> Self { History { states: Vec::new(), index: 0 } }
    pub fn push(&mut self, desc: &str) {
        self.states.truncate(self.index);
        self.states.push(desc.to_string());
        self.index = self.states.len();
    }
    pub fn undo(&mut self) -> bool { if self.index > 0 { self.index -= 1; true } else { false } }
    pub fn redo(&mut self) -> bool { if self.index < self.states.len() { self.index += 1; true } else { false } }
}
