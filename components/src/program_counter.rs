use color_eyre::owo_colors::OwoColorize;

#[derive(Debug, Default)]
pub struct ProgramCounter(usize);

impl ProgramCounter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment_by(&mut self, value: usize) {
        self.0 += value;
    }

    pub fn set_exact(&mut self, value: usize) {
        self.0 = value;
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

impl std::fmt::Display for ProgramCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r"{} {}", "PC:".green(), self.0)
    }
}
