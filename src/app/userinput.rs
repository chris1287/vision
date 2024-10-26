#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UserInputState {
    Column,
    Row,
}

#[derive(Debug)]
pub struct UserInput {
    state: UserInputState,
    history: Vec<String>,
    data: Vec<char>,
}

impl UserInput {
    pub fn new() -> Self {
        Self {
            state: UserInputState::Column,
            data: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn enter_data(&mut self, c: char) {
        match self.state {
            UserInputState::Column => {
                if let 'a'..='h' = c {
                    self.data.push(c);
                    self.state = UserInputState::Row;
                }
            }
            UserInputState::Row => {
                if let '1'..='8' = c {
                    self.data.push(c);
                    self.state = UserInputState::Column;
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.state = UserInputState::Column;
        if self.data.len() == 2 {
            self.history.push(self.data());
        }
        self.data.clear();
    }

    pub fn data(&self) -> String {
        String::from_iter(&self.data)
    }

    pub fn state(&self) -> UserInputState {
        self.state
    }
}
