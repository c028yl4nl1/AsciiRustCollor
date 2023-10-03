use Erros::Cores::AsciiCor;

fn main () {   
    let string_text = String::from("Rust");
    AsciiCor(string_text.clone()).green();
    AsciiCor(string_text.clone()).red();    
    AsciiCor(string_text.clone()).colored();

}