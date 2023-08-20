use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::io;
use rand::prelude::IteratorRandom;

fn thingy(boo: &HashSet<i32>) -> i32 {
    let mut rnd = rand::thread_rng();
    *boo.iter().choose(&mut rnd).unwrap()
}

fn smart_idea(numbers: &Vec<i32>) -> bool {
    for a in 0..numbers.len() {
        for b in a + 1..numbers.len() {
            for c in b + 1..numbers.len() {
                if numbers[a] + numbers[b] + numbers[c] == 15 {
                    return true;
                }
            }
        }
    }
    false
}

fn small_idea(el1: i32, el2: i32, set: &HashSet<i32>) -> Option<i32> {
    let required = 15 - (el1 + el2);
    if set.contains(&required) {
        Some(required)
    } else {
        None
    }
}

fn smaller_idea(nums: &Vec<i32>, set: &HashSet<i32>) -> Option<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if let Some(required_num) = small_idea(nums[i], nums[j], set) {
                return Some(required_num);
            }
        }
    }
    None
}

fn implement(required: i32, nums: &mut Vec<i32>, set: &mut HashSet<i32>) {
    nums.push(required);
    set.remove(&required);
}

fn matrix(abc: &Vec<i32>, def: &Vec<i32>) -> Vec<char> {
    let mut soup = vec![' '; 10];
    for &x in abc {
        soup[x as usize] = 'x';
    }
    for &o in def {
        soup[o as usize] = 'o';
    }
    soup
}

fn chaos(alphabet: &std::collections::HashMap<char, i32>, free: &HashSet<i32>) -> i32 {
    loop {
        println!("Enter a symbol of your move: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_lowercase().chars().next();
        if let Some(inp) = input {
            if let Some(number) = alphabet.get(&inp) {
                if free.contains(number) {
                    return *number;
                } else {
                    println!("The cell corresponding to symbol '{}' is busy.", inp);
                }
            } else {
                println!("Invalid input '{}'. Please enter one of the following symbols: {:?}", inp, alphabet.keys());
            }
        }
    }
}

fn main() {
    let char_to_int: std::collections::HashMap<char, i32> =
        [('a', 6), ('b', 1), ('c', 8), ('d', 7), ('e', 5), ('f', 3), ('g', 2), ('h', 9), ('i', 4)]
            .iter()
            .cloned()
            .collect();

    let mut gibberish: HashSet<i32> = [1, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned().collect();
    let mut uwu: Vec<i32> = Vec::new();
    let mut qwq: Vec<i32> = Vec::new();

    // the beginning
    magic_field(&uwu, &qwq);

    let mut uwu_game_over = false;
    // Continue game as long as there are free cells and nobody has won yet
    while !gibberish.is_empty() && !uwu_game_over {
        if let Some(big_win) = smaller_idea(&uwu, &gibberish) {
            implement(big_win, &mut uwu, &mut gibberish);
            magic_field(&uwu, &qwq);
            println!("I win!");
            uwu_game_over = true;
        } else {
            if let Some(small_block) = smaller_idea(&qwq, &gibberish) {
                implement(small_block, &mut uwu, &mut gibberish);
            } else {
                let random_move = thingy(&gibberish);
                implement(random_move, &mut uwu, &mut gibberish);
            }
            magic_field(&uwu, &qwq);
        }

        if !gibberish.is_empty() && !uwu_game_over {
            let user_input = chaos(&char_to_int, &gibberish);
            implement(user_input, &mut qwq, &mut gibberish);
            if smart_idea(&qwq) {
                magic_field(&uwu, &qwq);
                println!("You win!");
                uwu_game_over = true;
            } else {
                magic_field(&uwu, &qwq);
            }
        }
    }

    if !uwu_game_over {
        println!("It's a draw");
    }
}

fn magic_field(meow: &Vec<i32>, woof: &Vec<i32>) {
    let c = matrix(meow, woof);
    println!("a|b|c  {}|{}|{}", c[6], c[1], c[8]);
    println!("-----  -----");
    println!("d|e|f  {}|{}|{}", c[7], c[5], c[3]);
    println!("-----  -----");
    println!("g|h|i  {}|{}|{}", c[2], c[9], c[4]);
    println!();
}
