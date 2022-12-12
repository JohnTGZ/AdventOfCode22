enum Direction {
    UP = (0, -1),
    DOWN = (0, 1),
    LEFT = (-1, 0),
    RIGHT = (1, 0),
}

fn test_direction(direction: Direction) -> (i32, i32) {
    direction
}

fn main() {
    println!("{}", test_direction(Direction::UP));
}
