use rand::Rng;

use crate::{options::Options, ui::UI};

#[derive(Debug)]
pub enum MastermindError {
    LengthMismatch { expected: usize, got: usize }
}

pub struct Game {
    pub options: Options,
    pub secret_code: Vec<i32>,
    pub attempts_left: usize,
    pub history: Vec<(usize, usize, usize, Vec<i32>)>,
    pub ui: UI,
    pub current_exact_matches: usize,
    pub current_partial_matches: usize,
    pub current_score: usize,
}

impl Game {
    pub fn new(options: Options) -> Self {
        let secret_code = Self::generate_secret_code(&options);
        let attempts_left = options.max_attempts;
        let ui = UI::new(options.clone());
        Self {
            options,
            secret_code,
            attempts_left,
            history: Vec::new(),
            ui,
            current_exact_matches: 0,
            current_partial_matches: 0,
            current_score: 0,
        }
    }

    fn generate_secret_code(options: &Options) -> Vec<i32> {
        let mut arr = vec![];
        let mut rng = rand::rng();

        for _ in 0..options.length {
            let random_integer = rng.random_range(options.number_range.clone()); 
            arr.push(random_integer);
        }

        arr
    }

    fn count_exact_matches(&self, guess: &[i32]) -> Result<usize, MastermindError> {
        if self.secret_code.len() != self.options.length {
            return Err(MastermindError::LengthMismatch { expected: self.options.length, got: self.secret_code.len() })
        }

        if guess.len() != self.options.length {
            return Err(MastermindError::LengthMismatch { expected: self.options.length, got: guess.len() })
        }

        let mut counter = 0;

        for (i, val) in self.secret_code.iter().enumerate() {
            let second_val = &guess[i];

            if val == second_val {
                counter = counter + 1;
            } 
        }

        Ok(counter) 
    }

    fn count_partial_matches(&self, guess: &[i32]) -> Result<usize, MastermindError> {
        if self.secret_code.len() != self.options.length {
            return Err(MastermindError::LengthMismatch { expected: self.options.length, got: self.secret_code.len() })
        }

        if guess.len() != self.options.length {
            return Err(MastermindError::LengthMismatch { expected: self.options.length, got: guess.len() })
        }

        let mut counter = 0;
        let mut guess_used = vec![false; self.options.length];

        for &s in &self.secret_code {
            for (i, &g) in guess.iter().enumerate() {
                if !guess_used[i] && g == s {
                    counter += 1;
                    guess_used[i] = true;
                    break;
                }
            }
        }

        Ok(counter)
    }

    

    pub fn play(&mut self) {
        loop {
            self.ui.render_game_ui(self);
            let guess = self.ui.get_guess();

            match self.count_exact_matches(&guess) {
                Ok(exact) if exact == self.options.length => {
                    self.current_exact_matches = exact;
                    self.current_partial_matches = 0;
                    self.current_score = exact * 10;
                    self.ui.render_win_ui(self);
                    break;
                }
                Ok(exact) => {
                    let partial = self.count_partial_matches(&guess).unwrap_or(0) - exact;
                    self.current_exact_matches = exact;
                    self.current_partial_matches = partial;
                    self.current_score = exact * 10 + partial * 5;
                    self.attempts_left -= 1;
                    if self.attempts_left == 0 {
                        self.ui.render_lost_ui(self);
                        break;
                    }

                    self.history.push((exact, partial, self.current_score, guess))
                }
                Err(MastermindError::LengthMismatch { expected, got }) => {
                    eprintln!("Error: Length mismatch. Expected {}, got {}", expected, got);
                    continue;
                }
            }
        }
    }

}
