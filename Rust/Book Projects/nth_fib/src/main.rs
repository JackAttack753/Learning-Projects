use std::io;

fn main() {
    loop{
        println!("Which fibonacci number would you like?");

        let mut n_string = String::new();

        io::stdin()
            .read_line(&mut n_string)
            .expect("Failed to read line");

        //Credit to u/thiez on reddit for an efficent way to remove the newlines.
        let len = n_string.trim_end_matches(&['\r', '\n'][..]).len();
        n_string.truncate(len);

        let n_num = n_string.parse::<u32>();
        
        match n_num{
            Ok(n) => {
                let mut sum = 0;
                let mut prev_num = 1;
                for _ in 0..n{
                    sum += prev_num;
                    prev_num = sum - prev_num;
                }
                let n_ordinal = match n % 100{
                    0 | 4..=20 | 24..=30 | 34..=40 | 44..=50 | 54..=60 | 64..=70 | 74..=80 | 84..=90 | 94..100 => "th",
                    1 | 21 | 31 | 41 | 51 | 61 | 71 | 81 | 91 => "st",
                    2 | 22 | 32 | 42 | 52 | 62 | 72 | 82 | 92 => "nd",
                    3 | 23 | 33 | 43 | 53 | 63 | 73 | 83 | 93 => "rd",
                    _ => "How'd you get up here? This number should be mod 100."
                };
                println!("The {n}{n_ordinal} fibonacci number is {sum}.");
                break;
            }
            Err(_) => println!("Input not recognised. Expected a non-negative integer.")
        }
    }
}
