pub const LEFT: [u8; 6] = *b"\x1b[0;75";
pub const RIGHT: [u8; 6] = *b"\x1b[0;77";
pub const UP: [u8; 4] = *b"\x1b[[A";
pub const DOWN: [u8; 6] = *b"\x1b[0;80";
pub const SEMICOLON: [u8; 1] = *b";";
pub const ESCAPE: [u8; 1] = *b"\\";
pub const NEWLINE: [u8; 1] = *b"\n";

pub enum Character {
    Left,
    Right,
    Up,
    Down,
    SemiColon,
    Escape,
    Shift,
    Other,
}

pub fn char_from_byte(byte: &[u8]) -> Character {
    println!("bytes: {:?}", byte);
    return match byte {
        _ => {
            Character::Other
        }
    };
}