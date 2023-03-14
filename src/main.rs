<<<<<<< HEAD
//add test
// <!-- License -->

=======
>>>>>>> a144cf89219816bc4a5632e7c0819ac4ccdf560a
// MIT License
//
// Copyright (c) 2022 ~/fjp
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
