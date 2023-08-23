use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::io;
use rand::prelude::IteratorRandom;

fn get_random_cell(cells: &HashSet<i32>) -> i32 {
    let mut rng = rand::thread_rng();
    *cells.iter().choose(&mut rng).unwrap()
}

fn check_win(moves: &Vec<i32>) -> bool {
    for i in 0..moves.len() {
        for j in i + 1..moves.len() {
            for k in j + 1..moves.len() {
                if moves[i] + moves[j] + moves[k] == 15 {
                    return true;
                }
            }
        }
    }
    false
}

fn find_required_cell(el1: i32, el2: i32, free_cells: &HashSet<i32>) -> Option<i32> {
    let req_num = 15 - (el1 + el2);
    if free_cells.contains(&req_num) {
        Some(req_num)
    } else {
        None
    }
}

fn find_move(moves: &Vec<i32>, free_cells: &HashSet<i32>) -> Option<i32> {
    for i in 0..moves.len() {
        for j in i + 1..moves.len() {
            if let Some(req_num) = find_required_cell(moves[i], moves[j], free_cells) {
                return Some(req_num);
            }
        }
    }
    None
}

fn apply_move(required_number: i32, moves: &mut Vec<i32>, free_cells: &mut HashSet<i32>) {
    moves.push(required_number);
    free_cells.remove(&required_number);
}

fn generate_game_field(my_moves: &Vec<i32>, user_moves: &Vec<i32>) -> Vec<char> {
    let mut field = vec![' '; 10];
    for &x in my_moves {
        field[x as usize] = 'x';
    }
    for &o in user_moves {
        field[o as usize] = 'o';
    }
    field
}

fn ask_user_input(symbol_to_number: &std::collections::HashMap<char, i32>, free_cells: &HashSet<i32>) -> i32 {
    loop {
        println!("Enter a symbol of your move: ");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let user_input = user_input.trim().to_lowercase().chars().next();
        if let Some(input_char) = user_input {
            if let Some(number) = symbol_to_number.get(&input_char) {
                if free_cells.contains(number) {
                    return *number;
                } else {
                    println!("The cell corresponding to symbol '{}' is busy.", input_char);
                }
            } else {
                println!("Invalid input '{}'. Please enter one of the following symbols: {:?}", input_char, symbol_to_number.keys());
            }
        }
    }
}

fn print_game_field(my_moves: &Vec<i32>, user_moves: &Vec<i32>) {
    let c = generate_game_field(my_moves, user_moves);
    println!("a|b|c  {}|{}|{}", c[6], c[1], c[8]);
    println!("-----  -----");
    println!("d|e|f  {}|{}|{}", c[7], c[5], c[3]);
    println!("-----  -----");
    println!("g|h|i  {}|{}|{}", c[2], c[9], c[4]);
    println!();
}

fn main() {
    let symbol_to_number: std::collections::HashMap<char, i32> =
        [('a', 6), ('b', 1), ('c', 8), ('d', 7), ('e', 5), ('f', 3), ('g', 2), ('h', 9), ('i', 4)]
            .iter()
            .cloned()
            .collect();

    let mut free_cells: HashSet<i32> = [1, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned().collect();
    let mut my_moves: Vec<i32> = Vec::new();
    let mut user_moves: Vec<i32> = Vec::new();

   
    print_game_field(&my_moves, &user_moves);

    let mut game_over = false;

    while !free_cells.is_empty() && !game_over {
        if let Some(winning_move) = find_move(&my_moves, &free_cells) {
            apply_move(winning_move, &mut my_moves, &mut free_cells);
            print_game_field(&my_moves, &user_moves);
            println!("I win!");
            game_over = true;
        } else {
            if let Some(blocking_move) = find_move(&user_moves, &free_cells) {
                apply_move(blocking_move, &mut my_moves, &mut free_cells);
            } else {
                let rnd_move = get_random_cell(&free_cells);
                apply_move(rnd_move, &mut my_moves, &mut free_cells);
            }
            print_game_field(&my_moves, &user_moves);
        }

        if !free_cells.is_empty() && !game_over {
            let user_input = ask_user_input(&symbol_to_number, &free_cells);
            apply_move(user_input, &mut user_moves, &mut free_cells);
            if check_win(&user_moves) {
                print_game_field(&my_moves, &user_moves);
                println!("You win!");
                game_over = true;
            } else {
                print_game_field(&my_moves, &user_moves);
            }
        }
    }

    if !game_over {
        println!("It's a draw");
    }
}
