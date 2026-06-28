use std::io;

fn main() {
    println!("What phrase would you like to convert into pig latin?");

    let mut s = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");

    //Credit to u/thiez on reddit for an efficent way to remove the newlines.
    let len = s.trim_end_matches(&['\r', '\n'][..]).len();
    s.truncate(len);

    let words:Vec<&str> = s.split(' ').collect();
    let mut pig_words:Vec<String> = Vec::new();

    for w in words{
        let mut w_string = String::from(w);
        let first_char = w_string.chars().next();
        let pig_word = match first_char.unwrap_or(' '){
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'=> format!("{w_string}-hay"),

            'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' 
            | 'n' | 'p' | 'q' | 'r' | 's' | 't' | 'v' | 'w' | 'x' | 'y' | 'z' 
            | 'B' | 'C' | 'D' | 'F' | 'G' | 'H' | 'J' | 'K' | 'L' | 'M' 
            | 'N' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'V' | 'W' | 'X' | 'Y' | 'Z'=> {
                let c = w_string.remove(0);
                format!("{w_string}-{c}ay")
            }
            _ => {
                println!("The first letter of the word \"{w_string}\" is not recognised.");
                continue;
            }
        };
        pig_words.push(pig_word);
    }

    let mut pig_string = String::new();

    for pig_word in pig_words{
        pig_string.push_str(pig_word.as_str());
        pig_string.push(' ');
    }

    println!("Your phrase translates to this in pig latin:");
    println!("{pig_string}");
}
