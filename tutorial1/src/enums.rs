/*
 * enums have a few definite values
 */

enum Movement {
    Up,
    Down,
    Left,
    Right,
}

pub fn run () {
    let av1 = Movement::Up;
    let av2 = Movement::Down;
    let av3 = Movement::Left;
    let av4 = Movement::Right;

    move_avatar(av1);
    move_avatar(av2);
    move_avatar(av3);
    move_avatar(av4);
}

fn move_avatar(m: Movement) {
    // perform movement
    match m {
        Movement::Up => println!("move up"),
        Movement::Down => println!("move down"),
        Movement::Left => println!("move left"),
        Movement::Right => println!("move right")
    }
}
