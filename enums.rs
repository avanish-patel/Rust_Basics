
enum Direction {
    Up,
    Down,
    Right,
    Left
}

fn main() {

    let player_direction : Direction = Direction::Down;

    match player_direction {

        Direction::Up => println!("Player is heading Up"),
        Direction::Down => println!("Player is moving Down"),
        Direction::Right => println!("Player is turn Right"),
        Direction::Left => println!("Player is turn Left"),

    }
}