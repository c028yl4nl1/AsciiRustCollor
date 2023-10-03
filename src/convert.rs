use crate::ascii::CHARS;

use std::io::Error;
fn join_art(s1: &str, s2: &str) -> String {
    match (s1.is_empty(), s2.is_empty()) {
        (true, true) => "".to_string(),
        (true, false) => s2.to_string(),
        (false, true) => s1.to_string(),
        (false, false) => {
            let lines1: Vec<&str> = s1.split('\n').collect();
            let lines2: Vec<&str> = s2.split('\n').collect();

            // concats each line of the 2 ascii art
            let s3: Vec<String> = lines1
                .into_iter()
                .zip(lines2.into_iter())
                .map(|(str1, str2)| str1.to_owned() + " " + str2)
                .collect();

            s3.join("\n")
        }
    }
}

pub fn AsciiConvert(input: String) -> Result<String, String> {
    let art_vector = input
        .chars()
        .map(|ch| CHARS.get(ch as usize).unwrap_or(&"").to_owned())
        .collect::<Vec<&str>>();

    let mut final_string = "".to_string();
    let mut bad_char = false;
    for art in &art_vector {
        if art.is_empty() && !bad_char {
            bad_char = true
        }
    }

    if bad_char {
        Err("Erro: Alguns caracteres não permitidos, você pode usar: a..=Z, 0..=9 ,`; : . , < > ( ) ! *#@^`".to_string())
    } else {
        for art in art_vector {
            final_string = join_art(&final_string, art);
        }
        Ok(final_string)
    }
}
