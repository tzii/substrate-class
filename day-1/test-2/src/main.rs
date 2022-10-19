use std::io;

fn main() {
    println!("Input the string: ");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line.");
    println!("Enter \":q\" to quit. ");
    s = s.trim().to_ascii_lowercase();
    loop {
        println!("Enter charactor:");
        let mut c = String::new();
        io::stdin().read_line(&mut c).expect("Failed to read line.");
        c = c.trim().to_ascii_lowercase();
        match c.len() {
            1 => {
                let mut count = 0;
                let mut rest = String::new();
                let c = c.chars().nth(0).unwrap();
                for x in s.chars().into_iter() {
                    if c == x {
                        count += 1;
                    } else {
                        rest.push(x)
                    }
                }
                println!("{},{}", count, rest);
            }
            2 => {
                if c == ":q" {
                    break;
                } else {
                    println!("Wrong format!");
                }
            }
            _ => println!("Wrong format!"),
        }
    }
}
