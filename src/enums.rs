// Types with definite values

enum Movement {
    //Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Perform action on input

    match m {
        Movement::Up => println!("Moving up!"),
        Movement::Down => println!("Moving down!"),
        Movement::Left => println!("Moving left!"),
        Movement::Right => println!("Moving right!"),
    }
}
pub fn run() {
    let av1 = Movement::Up;
    let av2 = Movement::Down;
    let av3 = Movement::Left;
    let av4 = Movement::Right;

    move_avatar(av1);
    move_avatar(av2);
    move_avatar(av3);
    move_avatar(av4);
}
