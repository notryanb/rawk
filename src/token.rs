/*
    This will represent a single token
*/

pub enum Token {
    Invalid,
    Eof,
    Newline,
    Concat,

    Print,
    PrintF,
    Return,
}

