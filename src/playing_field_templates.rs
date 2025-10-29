use crate::utils::field_utils::{PlayingField, decode};

//can be used if you want to load a standard config or add your own one
pub fn get_template(number: u32) -> PlayingField {
    #[allow(unused_imports)]
    use crate::{Field, Number};
    //space for templates
    match number {
        1 => decode(
            &"61872E5345296341877345E82969621873E545329671E1873456292E645187334587296187196345E"
                .to_string(),
        )
        .expect("can't panic if it's correct"),
        2 => decode(
            &"7EEEE13466EEEE3E1E1E2EE4E7E5631EEE92EEE2EEEE3E9E53E6E4EEEE8E341EE4E1956EEE7E659EE"
                .to_string(),
        )
        .expect("can't panic if it's correct"),
        3 => decode(
            &"EE6EE13EEEEE38EEE1E9E5EEEE7E3EEEE689E17EEEEEEEEE8E21EEEEE2E3E46EE4EEEEEEE23E56E1E"
                .to_string(),
        )
        .expect("can't panic if it's correct"),
        4 => decode(
            &"398EEE7E1EEE9EE4EEEE2E8EE6EEE4E3EE5EEEEEE1EE9E2EEE8EEEEE6574EE34E531E2EEEE9862E7E"
                .to_string(),
        )
        .expect("can't panic if it's correct"),
        5 => decode(
            &"EE13E98EEEEEE8EEE3EEE4EE65E5EE7EEE96E6E2EE5E72EEE6EEEEEEEEE84E1EEEEE47EE17EE2693E"
                .to_string(),
        )
        .expect("can't panic if it's correct"),

        _ => PlayingField::from([
            [Field::Empty; 9],
            [Field::Empty; 9],
            [Field::Empty; 9],
            [Field::Empty; 9],
            [Field::Empty; 9],
            [Field::Empty; 9],
            [Field::Empty; 9],
            [Field::Empty; 9],
            [Field::Empty; 9],
        ]),
    }
}
