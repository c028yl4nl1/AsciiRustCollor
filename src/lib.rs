mod ascii;
mod convert;
use colored::{Color, ColoredString, Colorize};
use convert::AsciiConvert;
use rand::Rng;

use std::io::{self, Write};
// Resto do seu código...
pub enum Cores {
    AsciiCor(String),
}

fn Convert(text: String) -> Result<String, String> {
    AsciiConvert(text)
}

impl Cores {
    pub fn Green(&self) -> bool {
        match self {
            Cores::AsciiCor(valor) => {
                if let Ok(text) = Convert(valor.to_string()) {
                    println!("{}", text.green());
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn Red(&self) -> bool {
        match self {
            Cores::AsciiCor(valor) => {
                if let Ok(text) = Convert(valor.to_string()) {
                    println!("{}", text.red());
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn Cyan(&self) -> bool {
        match self {
            Cores::AsciiCor(valor) => {
                if let Ok(text) = Convert(valor.to_string()) {
                    println!("{}", text.cyan());
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn Yellow(&self) -> bool {
        match self {
            Cores::AsciiCor(valor) => {
                if let Ok(text) = Convert(valor.to_string()) {
                    println!("{}", text.yellow());
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn Colored(&self) {
        match self {
            Cores::AsciiCor(valor) => {
                let mut colored_string = String::new();

                for letra in valor.chars() {
                    if let Ok(text) = Convert(letra.to_string()) {
                        let color = random_color();
                        colored_string.push_str(&text.color(color).to_string());
                    }
                }

                println!("{}", colored_string);
            }
            _ => println!("s"),
        }
    }
}

fn random_color() -> Color {
    let colors = [
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
    ];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..colors.len());
    colors[index]
}
