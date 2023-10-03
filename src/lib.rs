mod ascii;
mod convert;
use colored::Colorize;
use convert::ascii_convert;

// Resto do seu código...
pub enum Cores {
    AsciiCor(String),
}

fn convert(text: String, randomizer_color: bool) -> Result<String, String> {
    ascii_convert(text, randomizer_color)
}

impl Cores {
    pub fn green(&self) -> bool {
        match self {
            Cores::AsciiCor(valor) => {
                if let Ok(text) = convert(valor.to_string(), false) {
                    println!("{}", text.green());
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn red(&self) -> bool {
        match self {
            Cores::AsciiCor(valor) => {
                if let Ok(text) = convert(valor.to_string(), false) {
                    println!("{}", text.red());
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn cyan(&self) -> bool {
        match self {
            Cores::AsciiCor(valor) => {
                if let Ok(text) = convert(valor.to_string(), false) {
                    println!("{}", text.cyan());
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn yellow(&self) -> bool {
        match self {
            Cores::AsciiCor(valor) => {
                if let Ok(text) = convert(valor.to_string(), false) {
                    println!("{}", text.yellow());
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn colored(&self) {
        match self {
            Cores::AsciiCor(valor) => {
                if let Ok(text) = convert(valor.to_string(), true) {
                    println!("{}", text);
                }
            }
        }
    }
}
