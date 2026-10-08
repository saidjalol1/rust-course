


pub enum Weekdays {
    Monday(String), 
    Tuesday(String),
    Wednesday(String),
    Thursday(String),
    Friday(String),
    Saturday { display_name: String},
    Sunday  {display_name: String}
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub enum Message {
    Quit,
    Move { x: u32, y:u32},
    Write(String),
    ChangeColor(u8,u8,u8)
}

enum Option {
    None,
    Some(T),
}

