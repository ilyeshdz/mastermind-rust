use crate::{feedback::Feedback, rules::Rules};
use rand::Rng;

pub type Code = Vec<u8>;

pub struct Game {
    secret_code: Code,
    rules: Rules,
    state: GameState,
}

impl Game {
    pub fn new(rules: Rules) -> Self {
        let secret_code = Self::generate_secret_code(&rules);
        Self {
            secret_code,
            rules,
            state: GameState {
                history: Vec::new(),
            },
        }
    }

    pub fn guess(&mut self, guess: Code) -> Option<Feedback> {
        if guess.len() != self.rules.code_len().into() {
            return None;
        }
        let feedback = Feedback::new(&self.secret_code, &guess);
        self.state.history.push(Turn {
            guess,
            feedback: feedback.clone(),
        });
        Some(feedback)
    }

    /// Generates the secret code for the game according to the rules.
    ///
    /// # Returns
    ///
    /// A vector of integers (`Vec<u8>`) representing the secret code.
    fn generate_secret_code(rules: &Rules) -> Vec<u8> {
        let mut secret_code: Vec<u8> = Vec::new();
        let mut rng = rand::rng();
        let code_len = rules.code_len();
        let available_symbols = rules.available_symbols();
        for _ in 0..code_len {
            secret_code.push(rng.random_range(0..available_symbols))
        }
        secret_code
    }
}

pub struct GameState {
    history: Vec<Turn>,
}

impl GameState {
    pub fn last_feedback(&self) -> Option<&Feedback> {
        match self.history.last() {
            Some(turn) => Some(&turn.feedback),
            None => None,
        }
    }
}

pub struct Turn {
    guess: Code,
    feedback: Feedback,
}
