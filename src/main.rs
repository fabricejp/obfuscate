use rand::{thread_rng, Rng};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let email: &str = &args[1];

    let mut before: Vec<char> = Vec::new();
    let mut after: Vec<char> = Vec::new();

    for (i, c) in email.chars().enumerate() {
        if c == '@' {
            before = email[..i].chars().collect();
            after = email[i+1..].chars().collect();
            break;
        }
    }

    let mut rng = thread_rng();
    let before_index1 = rng.gen_range(0, before.len());
    let before_index2 = rng.gen_range(0, before.len());
    let after_index1 = rng.gen_range(0, after.len());
    let after_index2 = rng.gen_range(0, after.len());

    for i in 0..before.len() {
        if i == before_index1 || i == before_index2 {
            continue;
        } else {
            before[i] = '*';
        }
    }

    for i in 0..after.len() {
        if i == after_index1 || i == after_index2 {
            continue;
        } else {
            after[i] = '*';
        }
    }

    println!("{}{}{}", before.into_iter().collect::<String>(), "@", after.into_iter().collect::<String>());
}
