use std::io::{Write, stdout};

use crossterm::{
    ExecutableCommand,
    cursor::{self, MoveTo},
    event::{Event, KeyCode, KeyModifiers, read},
    execute,
    style::{Print, Stylize},
    terminal::{Clear, ClearType},
};

use crate::{game::Game, options::Options};

pub struct UI {
    pub input_row: u16,
    pub input_col: u16,

    pub options: Options,

    pub underline_char: char,
    pub placeholder_char: char,
}

impl UI {
    pub fn new(options: Options) -> Self {
        let (width, height) = crossterm::terminal::size().unwrap();
        let centered_input_row = height / 2;
        let centered_input_col = (width / 2) - (options.length as u16 / 2);
        stdout().execute(cursor::Hide).unwrap();

        Self {
            input_row: centered_input_row,
            input_col: centered_input_col,
            options: options.clone(),
            underline_char: '_',
            placeholder_char: '-',
        }
    }

    fn clear() {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
    }

    pub fn render_game_ui(&self, game: &Game) {
        Self::clear();
        let mut out = stdout();

        self.render_stats(game);
        self.render_history(game);

        self.render_input_placeholder(&mut out);
    }

    fn render_history(&self, game: &Game) {
        let mut out = stdout();
        let (width, _) = crossterm::terminal::size().unwrap();
        
        let history_entries: Vec<(String, String)> = game.history.iter().rev().take(5).map(|(exact, partial, _, guess)| {
            let guess_plain = format!("{:?}", guess);
            let feedback_plain = format!("{} exact, {} partial", exact, partial);
            (guess_plain, feedback_plain)
        }).collect();
        
        let max_len = history_entries.iter()
            .map(|(guess, feedback)| guess.chars().count() + feedback.chars().count() + 1) // +1 for space
            .max().unwrap_or(0);
        
        let right_col = if width > max_len as u16 + 4 { 
            width - max_len as u16 - 4 
        } else { 
            2 
        };
        
        let start_row = 2;
        
        for (i, (guess, feedback)) in history_entries.iter().enumerate() {
            let guess_styled = guess.as_str().bold().cyan();
            let feedback_styled = feedback.as_str().yellow().italic();
            let full_text = format!("{} {}", guess_styled, feedback_styled);
            
            out.execute(MoveTo(right_col, start_row + i as u16)).unwrap();
            out.execute(Print(full_text)).unwrap();
        }
    }

    fn render_stats(&self, game: &Game) {
        let mut out = stdout();
        let attempts_text = format!(
            "{} {}",
            "Attempts left:".bold(),
            game.attempts_left.to_string().blue().italic(),
        );
        out.execute(MoveTo(2, 2)).unwrap();
        out.execute(Print(attempts_text)).unwrap();

        let exact_matches_text = format!(
            "{} {}",
            "Exact matches:".bold(),
            game.current_exact_matches.to_string().blue().italic(),
        );
        out.execute(MoveTo(2, 3)).unwrap();
        out.execute(Print(exact_matches_text)).unwrap();

        let partial_matches_text = format!(
            "{} {}",
            "Partial matches:".bold(),
            game.current_partial_matches.to_string().blue().italic(),
        );
        out.execute(MoveTo(2, 4)).unwrap();
        out.execute(Print(partial_matches_text)).unwrap();

        let score_text = format!(
            "{} {}",
            "Score:".bold(),
            game.current_score.to_string().green().italic(),
        );
        out.execute(MoveTo(2, 5)).unwrap();
        out.execute(Print(score_text)).unwrap();
    }

    fn strip_ansi_codes(text: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '\x1b' && chars.peek() == Some(&'[') {
                chars.next();
                while chars.peek() != Some(&'m') && chars.peek().is_some() {
                    chars.next();
                }
                if chars.peek() == Some(&'m') {
                    chars.next();
                }
            } else {
                result.push(ch);
            }
        }
        result
    }

    fn render_text_align_center<T>(&self, out: &mut std::io::Stdout, text: T, row: u16) 
    where T: std::fmt::Display {
        let text_str = format!("{}", text);
        let clean_text = Self::strip_ansi_codes(&text_str);
        let center_col = (crossterm::terminal::size().unwrap().0 / 2) as i32;
        let text_len = clean_text.chars().count() as i32;
        let start_col = center_col - (text_len / 2);
        out.execute(MoveTo(start_col as u16, row)).unwrap();
        out.execute(Print(text)).unwrap();
    }

    pub fn get_guess(&self) -> Vec<i32> {
        let mut buffer = vec![self.placeholder_char; self.options.length];
        let mut pos = 0;

        crossterm::terminal::enable_raw_mode().unwrap();

        loop {
            let mut out = stdout();
            out.execute(MoveTo(self.input_col, self.input_row)).unwrap();
            out.execute(Print(buffer.iter().collect::<String>()))
                .unwrap();
            out.flush().unwrap();

            if let Event::Key(event) = read().unwrap() {
                match event.code {
                    KeyCode::Char('c') if event.modifiers == KeyModifiers::CONTROL => {
                        crossterm::terminal::disable_raw_mode().unwrap();
                        Self::clear();
                        std::process::exit(0);
                    }
                    KeyCode::Char(c) if c.is_digit(10) && pos < self.options.length => {
                        buffer[pos] = c;
                        pos += 1;
                    }
                    KeyCode::Backspace if pos > 0 => {
                        pos -= 1;
                        buffer[pos] = self.placeholder_char;
                    }
                    KeyCode::Enter if pos == self.options.length => break,
                    _ => {}
                }
            }
        }

        crossterm::terminal::disable_raw_mode().unwrap();

        buffer
            .iter()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect()
    }

    fn render_input_placeholder(&self, out: &mut std::io::Stdout) {
        let underline_characters = vec![self.underline_char; self.options.length];
        out.execute(MoveTo(self.input_col, self.input_row)).unwrap();
        out.execute(Print(underline_characters.iter().collect::<String>()))
            .unwrap();
        out.execute(MoveTo(self.input_col, self.input_row)).unwrap();
    }

    pub fn render_lost_ui(&self, game: &Game) {
        Self::clear();
        let mut out = stdout();

        let (_, terminal_height) = crossterm::terminal::size().unwrap();
        let total_lines = 5;
        let start_row = (terminal_height - total_lines) / 2;

        self.render_text_align_center(&mut out, "Game Over! You ran out of attempts.".bold().red(), start_row);
        
        self.render_text_align_center(&mut out, "", start_row + 1);
        
        self.render_text_align_center(&mut out, "The secret code was:".bold().white(), start_row + 2);
        
        let secret_code_text = format!("{:?}", game.secret_code).bold().blue();
        self.render_text_align_center(&mut out, &secret_code_text, start_row + 3);
        
        self.render_text_align_center(&mut out, "", start_row + 4);
        
        self.render_text_align_center(&mut out, "Press any key to exit...".italic().dim(), start_row + 5);
        
        out.flush().unwrap();

        crossterm::terminal::enable_raw_mode().unwrap();
        let _ = read();
        crossterm::terminal::disable_raw_mode().unwrap();
    }

    pub fn render_win_ui(&self, game: &Game) {
        Self::clear();
        let mut out = stdout();

        let (_, terminal_height) = crossterm::terminal::size().unwrap();
        let total_lines = 5;
        let start_row = (terminal_height - total_lines) / 2;

        self.render_text_align_center(&mut out, "Congratulations! You won!".bold().green(), start_row);
        
        self.render_text_align_center(&mut out, "", start_row + 1);
        
        self.render_text_align_center(&mut out, "The secret code was:".bold().white(), start_row + 2);
        
        let secret_code_text = format!("{:?}", game.secret_code).bold().blue();
        self.render_text_align_center(&mut out, &secret_code_text, start_row + 3);
        
        self.render_text_align_center(&mut out, "", start_row + 4);
        
        self.render_text_align_center(&mut out, "Press any key to exit...".italic().dim(), start_row + 5);
        
        out.flush().unwrap();

        crossterm::terminal::enable_raw_mode().unwrap();
        let _ = read();
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}
