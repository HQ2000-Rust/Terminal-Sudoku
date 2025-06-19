use crate::utils::field_utils::{Field, Number, decode};
use crate::utils::general::get_input;
use crate::utils::general::menu::settings::Flags;

mod playing_field_templates;
mod utils;
#[allow(unused_labels)]

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
    let mut stats = crate::utils::general::stats::Stats::new();
    'round: loop {
        utils::general::menu::general_menu();
        //replace PlayingField::new() with crate::playing_field_templates::get_template([number assigned to the wanted pattern])
        //to use your own standard templates
        let mut playing_field = PlayingField::new();
        //saved regardless of the flags to prevent compiler warnings about possibly uninitialized values
        let start = std::time::Instant::now();
        //loop labels: here to make orientation easier, they aren't needed (mostly)
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
                Field::Number({
                    crate::utils::field_utils::i32_to_Number(&field_type_i32)
                        .expect("already bound and type checked, so this should be impossible")
                }),
            );
            println!("before if-maker");
            if settings.templates {
                'template: loop {
                    println!("Do you want to apply a template? (y/n)");
                    match get_input().trim() {
                        "y" => {
                            println!("Please input a saved state code and press ENTER to apply it");
                            match decode(&get_input().trim().to_string()) {
                                None => {
                                    println!("Incorrect state code!");
                                    println!("Do you want to apply another state code? (y/n)");
                                    match get_input().trim() {
                                        "y" => {
                                            println!("Ok, so");
                                            continue 'template;
                                        }
                                        _ => break 'template,
                                    }
                                }
                                Some(field) => {
                                    playing_field = field;
                                    println!("Applied template!");
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            if settings.sudoku_maker {
                'save: loop {
                    println!(
                        "Do you want to save this state? Type in the corresponding number and press ENTER."
                    );
                    println!("1. yes");
                    println!("2. no");
                    match get_input().trim().parse::<i32>() {
                        Ok(1) => {
                            println!("Nice! Here is your code to copy:");
                            println!("{}", playing_field.encode());
                            println!("(Press ENTER to continue with the game)");
                        }
                        _ => {
                            println!("Are you sure you don't want to get a code? (y/n)");
                            match get_input().trim() {
                                "n" => {
                                    println!("Ok, so");
                                }
                                _ => {
                                    println!("Fine. Good luck and have fun!")
                                }
                            }
                        }
                    }
                }
            }
            use crate::utils::field_utils::SolvingState;
            match playing_field.solving_state() {
                SolvingState::Solvable => {}
                SolvingState::Unsolvable => {
                    stats.add_lost();
                    playing_field.print();
                    println!("It's unsolvable now! (Press ENTER)");
                    get_input();
                    break 'turn;
                }
                SolvingState::Solved => {
                    stats.add_won();
                    if settings.stopwatch {
                        println!(
                            "You needed {}m and {}s!",
                            start.elapsed().as_secs() - (start.elapsed().as_secs() % 60),
                            start.elapsed().as_secs() % 60
                        );
                    }

                    if stats.fastest_run == None {
                        stats.fastest_run = Some(start.elapsed());
                        println!("That's your first and fastest run! (With stopwatch enabled)")
                    } else {
                        if start.elapsed()
                            > stats
                                .fastest_run
                                .expect("Impossible to reach (at least logically)")
                        {
                            stats.fastest_run = Some(start.elapsed());
                            println!("That's a new best!");
                        }
                    }
                    println!("Solved! (Press ENTER)");
                    get_input();
                    break 'turn;
                }
            }
        }
    }
}
