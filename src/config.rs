use clap::{Parser, Subcommand};

use crate::screen::{Choice, Question};

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(arg_required_else_help = true)]
    Add {
        #[clap(short, long)]
        question: String,
        #[clap(short, long)]
        choices: String,
        #[clap(short, long)]
        answer: usize,
    },
}

pub fn add_to_question(question: String, choices: String, answer: usize) -> Question {
    // choices should be comma separated
    let choices = choices.trim().to_string();
    let question = question.trim().to_string();
    Question {
        question,
        choices: choices
            .split(',')
            .enumerate()
            .map(|(i, s)| Choice {
                text: s.to_string(),
                correct: i == answer,
            })
            .collect(),
        index: 0,
    }
}
