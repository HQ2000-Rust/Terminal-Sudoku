use crate::utils::field_utils::{Field, Number, PlayingField};

mod utils;

fn main() {
    println!("Welcome to");
    /*let field=PlayingField::from([[Field::Empty;9],
        [Field::Empty;9],
        [Field::Number(Number::Seven);9],
        [Field::Empty;9],
        [Field::Number(Number::Nine);9],
        [Field::Empty;9],
        [Field::Empty;9],
        [Field::Empty;9],
        [Field::Empty;9],]);
    field.print();*/

    utils::general::menu::general_menu();
    start_game_loop();
}

#[inline]
fn start_game_loop() {
    use utils::field_utils::{Field, PlayingField};
    loop {
        let mut playing_field = PlayingField::new();
    }
}
