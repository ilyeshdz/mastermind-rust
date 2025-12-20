use crate::{game::Code, rules::CodeLen};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feedback {
    exact: CodeLen,
    misplaced: CodeLen,
}

impl Feedback {
    pub fn new(secret: &Code, guess: &Code) -> Self {
        let exact = secret
            .iter()
            .zip(guess.iter())
            .filter(|&(s, g)| s == g)
            .count() as CodeLen;
        let misplaced = secret.iter().filter(|s| guess.contains(s)).count() as CodeLen - exact;
        Feedback { exact, misplaced }
    }
}
