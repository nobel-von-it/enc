use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};
use serde::{Deserialize, Serialize};

const GAME_NAME: &str = "ENCARD";

#[derive(Debug, Clone)]
pub struct Screen {
    pub state: State,
    pub question: Question,
    pub score: Score,
}
impl Screen {
    pub fn new_menu() -> Self {
        Screen {
            state: State::Menu,
            question: Question {
                question: String::from("Welcome to English card game!"),
                choices: vec![
                    Choice {
                        text: String::from("Play"),
                        correct: false,
                    },
                    Choice {
                        text: String::from("Exit"),
                        correct: false,
                    },
                ],
                index: 0,
            },
            score: Score::new(),
        }
    }
    pub fn start_game(&mut self) {
        if self.state != State::Menu {
            return;
        }
        if self.question.index == 0 {
            self.state = State::Game;
            self.update();
        } else {
            std::process::exit(0);
        }
    }
    pub fn update(&mut self) {
        if self.state != State::Game {
            return;
        }
        if self.question.cmp() {
            self.score.inc();
        }
        self.question.index = 0;
        let questions = Questions::load_from_file();
        self.question =
            questions.questions[rand::thread_rng().gen_range(0..questions.questions.len())].clone();
    }
    pub fn draw(&mut self, f: &mut ratatui::Frame) {
        let full_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .split(f.size());
        let choices_layout = Layout::new(
            Direction::Vertical,
            (0..self.question.choices.len())
                .map(|_| Constraint::Length(3))
                .collect::<Vec<_>>(),
        )
        .split(full_layout[1]);

        let question_widget = Paragraph::new(Text::raw(&self.question.question).centered())
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title(format!(" {} -------- {} ", GAME_NAME, self.score.score)),
            )
            .centered();

        let choices = self
            .question
            .choices
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if i == self.question.index {
                    Text::raw(c.text.clone()).blue().centered()
                } else {
                    Text::raw(c.text.clone()).centered()
                }
            })
            .collect::<Vec<_>>();

        f.render_widget(question_widget, full_layout[0]);
        for (i, c) in choices.iter().enumerate() {
            f.render_widget(c, choices_layout[i]);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Menu,
    Exit,
    Game,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Questions {
    pub questions: Vec<Question>,
}
impl Questions {
    pub fn load_from_file() -> Self {
        let dir_path = format!("/home/{}/.enc/", whoami::username());
        let file_path = format!("{}/questions.json", dir_path);
        let mut file = std::fs::File::open(file_path.clone()).unwrap_or_else(|_| {
            let _ = std::fs::create_dir_all(dir_path.clone());
            let _ = std::fs::File::create(file_path.clone());
            std::fs::File::open(file_path).unwrap()
        });
        let questions: Questions =
            serde_json::from_reader(&mut file).unwrap_or(Questions { questions: vec![] });
        questions
    }
    pub fn save_to_file(&self) {
        let dir_path = format!("/home/{}/.enc/", whoami::username());
        let file_path = format!("{}/questions.json", dir_path);
        let mut file = std::fs::File::create(file_path.clone()).unwrap_or_else(|_| {
            let _ = std::fs::create_dir_all(dir_path.clone());
            std::fs::File::create(file_path.clone()).unwrap()
        });
        serde_json::to_writer(&mut file, self).unwrap();
    }
    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
        self.save_to_file();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub choices: Vec<Choice>,
    pub index: usize,
}

impl Question {
    pub fn cmp(&self) -> bool {
        self.choices[self.index].correct
    }
    pub fn up(&mut self) {
        if self.index == 0 {
            self.index = self.choices.len() - 1;
        } else {
            self.index -= 1
        }
    }
    pub fn down(&mut self) {
        if self.index == self.choices.len() - 1 {
            self.index = 0
        } else {
            self.index += 1
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub text: String,
    pub correct: bool,
}

#[derive(Debug, Clone)]
pub struct Score {
    pub score: u32,
}

impl Score {
    pub fn new() -> Self {
        Score { score: 0 }
    }
    pub fn inc(&mut self) {
        // TODO: make more complex scoring
        self.score += 1;
    }
}
