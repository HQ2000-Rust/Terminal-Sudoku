use crate::utils::field_utils::{Field, Number, PlayingField};
use crate::utils::general::get_input;
use crate::utils::general::menu::settings::Flags;

mod playing_field_templates;
mod utils;

fn main() {
    println!("Welcome to Terminal Sudoku!");
    let settings = crate::utils::general::menu::settings::get_raw_flags();
    let settings = Flags {
        stopwatch: settings.contains(&("stopwatch".to_string())),
        sudoku_maker: settings.contains(&("sudoku_maker".to_string())),
        templates: settings.contains(&("templates".to_string())),
    };
    game_loop(settings);
}

#[inline]
fn game_loop(settings: Flags) {
    use utils::field_utils::{Field, PlayingField};
    let mut stats=crate::utils::general::stats::Stats::new();
    'round: loop {
        utils::general::menu::general_menu();
        let mut playing_field = PlayingField::new();
        //TODO!: timer
        'turn: loop {
            println!("The field:");
            playing_field.print();
            println!("Which field do you want to change?");
            let mut y_coord_switch = true;
            let mut x_coord: usize = 0;
            let mut y_coord: usize = 0;
            'coords: loop {
                println!("Input the {}-coordinate:", {
                    if y_coord_switch { "y" } else { "x" }
                });
                match get_input().trim().parse::<usize>() {
                    Err(_) => {
                        println!("Please enter a number between 1 and 9 (inclusive)!");
                        continue 'coords;
                    }
                    Ok(number) => {
                        if !(number >= 1 && number <= 9) {
                            println!("Please enter a number between 1 and 9 (inclusive)!");
                            continue 'coords;
                        }
                        match y_coord_switch {
                            true => {
                                y_coord = number;
                            }
                            false => {
                                x_coord = number;
                            }
                        }
                        if !y_coord_switch {
                            let field = playing_field.access(x_coord, y_coord);
                            if !field.eq(&Field::Empty) {
                                println!("You need an empty Field to put something in!");
                                y_coord_switch = true;
                                continue 'coords;
                            }
                            break 'coords;
                        }
                        y_coord_switch = false;
                    }
                }
            }
            println!("y: {y_coord}, x: {x_coord}");
            let field_type_i32 = loop {
                println!("Which number should be inserted? Type it in and press ENTER.");
                if let Ok(number) = get_input().trim().parse::<i32>() {
                    if number >= 1 && number <= 9 {
                        println!("Number: {number}");
                        println!("{number}");
                        break number;
                    }
                }
                println!("Please enter a number between 1 and 9 (inclusive)!");
            };
            playing_field.set(
                x_coord,
                y_coord,
                Field::Number(
                    {
                        crate::utils::field_utils::i32_to_Number(&field_type_i32).expect("already bound and type checked, so this should be impossible")
                    }
                ),
            );
            use crate::utils::field_utils::SolvingState;
            match playing_field.solving_state() {
                SolvingState::Solvable=>{},
                SolvingState::Unsolvable=>{
                    stats.add_lost();
                    println!("It's unsolvable now! (Press ENTER)");
                    get_input();
                    break 'round;
               },
                SolvingState::Solved=>{
                    stats.add_won();
                    println!("Solved! (Press ENTER)");
                    get_input();
                    break 'round;
                }
            }
            
        }
    }
}
