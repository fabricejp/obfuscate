/*
Use the rand crate to create a new thread-local random number generator (RNG).
The thread_rng() function from the rand crate is used to create the RNG that can be used to generate random numbers.
*/

use rand::{thread_rng, Rng};

fn main() {

    /*
    Create a vector of strings called args.
    
    The vector is populated with the command line arguments 
    passed to the program when it is run.
    
    The std::env::args() function returns an iterator over the arguments,
    which is then collected into the vector.
    */

    let args: Vec<String> = std::env::args().collect();

    /*
    Declare a variable named "email" and assigning it the value
    of the second element of the "args" array.
    The type of the variable is a string reference (&str).
    */

    let email: &str = &args[1];

    /*
    Create mutable (meaning it can be changed) vectors of characters named 'before' and 'after'
    a vector is a data structure used to store multiple values of the same type.
    In this case, the vector will store characters
    */

    let mut before: Vec<char> = Vec::new();
    let mut after: Vec<char> = Vec::new();

    /*
    Iterate through the characters of an email string,
    searching for the '@' character.
    When it finds the '@' character,
    it creates two new strings, one containing all of the characters before the '@
    and one containing all of the characters after the '@'.
    The before and after strings are collected using the chars() method
    and stored in the before and after variables.
    The loop then breaks.
    */

    for (i, c) in email.chars().enumerate() {
        if c == '@' {
            before = email[..i].chars().collect();
            after = email[i+1..].chars().collect();
            break;
        }
    }

    /*
    Create a mutable random number generator (RNG) called "rng"
    and assigns it to the thread's RNG, allowing the thread to access the RNG.
    */

    let mut rng = thread_rng();

    /*
    Generate four random numbers between 0 and the length of two given vectors, before and after.
    Store their values in four separate variables before_index1, before_index2, after_index1, and after_index2.
    The random numbers will be used to index into the given vectors.
    */

    let before_index1 = rng.gen_range(0, before.len());
    let before_index2 = rng.gen_range(0, before.len());
    let after_index1 = rng.gen_range(0, after.len());
    let after_index2 = rng.gen_range(0, after.len());

    /*
    Loop through an array named "before". 
    It is checking the indices of each item in the array,
    and if the index is equal to before_index1 or before_index2, it will skip it.
    Otherwise, it will set the item at that index to '*'.
    */

    for i in 0..before.len() {
        if i == before_index1 || i == before_index2 {
            continue;
        } else {
            before[i] = '*';
        }
    }

    /*
    Loop through the vector 'after' and replace all of the elements except
    for the ones at 'after_index1' and 'after_index2' with an asterisk '*'.
    */    

    for i in 0..after.len() {
        if i == after_index1 || i == after_index2 {
            continue;
        } else {
            after[i] = '*';
        }
    }

    /*
    Print out a string that is composed of three components: "before", "@", and "after".
    Use the iter() method to convert the "before" and "after" variables into an iterable collection,
    then use the collect() method to convert them into a string.
    Finally, the println! macro is used to print out the combined string to the console.
    */      

    println!("{}{}{}", before.into_iter().collect::<String>(), "@", after.into_iter().collect::<String>());
}
