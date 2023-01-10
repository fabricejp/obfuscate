use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide an email address as the command line argument");
    }

    let email = &args[1];
    let obfuscated_email = obfuscate_email(email);

    //print obfuscated email address: {}", obfuscated_email);
    println!("{}", obfuscated_email);
}

fn obfuscate_email(email: &String) -> String {
    let mut obfuscated_email = String::new();
    let mut at_index = 0;

    for (index, c) in email.chars().enumerate() {
        if c == '@' {
            at_index = index;
            break;
        }
    }

    for (index, c) in email.chars().enumerate() {
        if index < at_index - 2 || index > at_index + 2 {
            obfuscated_email.push('*');
        } else {
            obfuscated_email.push(c);
        }
    }

    obfuscated_email
}
