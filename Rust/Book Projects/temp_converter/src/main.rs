use std::io;

fn main() {
    loop{
        println!("Would you like to 
        \n(1) convert from Farenheit to Celsius or 
        \n(2) convert from Celsius to Farenheit?:
        \n(enter 'quit' to exit program)");

        let mut conversion_mode = String::new();

        io::stdin()
            .read_line(&mut conversion_mode)
            .expect("Failed to read line");

        //Credit to u/thiez on reddit for an efficent way to remove the newlines.
        let len = conversion_mode.trim_end_matches(&['\r', '\n'][..]).len();
        conversion_mode.truncate(len);

        println!("{conversion_mode}");

        match conversion_mode.as_str(){
            "1" => {
                loop{
                    println!("Please input your temperature in Farenheit:");

                    let mut f_temp = String::new();

                    io::stdin()
                        .read_line(&mut f_temp)
                        .expect("Failed to read line");

                    let len = f_temp.trim_end_matches(&['\r', '\n'][..]).len();
                    f_temp.truncate(len);

                    let temp = f_temp.parse::<i32>();
                    match temp{
                        Ok(c_temp) => {
                            println!("This temperature is {} degrees Celsius.", (c_temp - 32) * 5/9 );
                            break;
                        },
                        Err(_e) => println!("Input not recognised. Expected an integer.")
                    }
                }
            }
            "2" => {
                loop{
                    println!("Please input your temperature in Celsius:");

                    let mut c_temp = String::new();

                    io::stdin()
                        .read_line(&mut c_temp)
                        .expect("Failed to read line");
                    
                    let len = c_temp.trim_end_matches(&['\r', '\n'][..]).len();
                    c_temp.truncate(len);

                    let temp = c_temp.parse::<i32>();
                    match temp{
                        Ok(f_temp) => {
                            println!("This temperature is {} degrees Farenheit.", f_temp * 9/5 + 32 );
                            break;
                        },
                        Err(_e) => println!("Input not recognised. Expected an integer.")
                    }
                }
            }
            "quit" => {
                println!("Exiting now.");
                break;
            }
            _ => {
                println!("Input not recognised. Expected '1', '2', or 'quit'.");
                continue
            }
        }

    }
}