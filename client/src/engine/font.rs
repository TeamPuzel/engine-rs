use core::f32::MIN;


pub type Symbol = [[bool; 3]; 5];

pub fn symbol_from_char(char: char) -> Option<&'static Symbol> {
    match char {
        '0' => Some(&DIGIT_0),
        '1' => Some(&DIGIT_1),
        '2' => Some(&DIGIT_2),
        '3' => Some(&DIGIT_3),
        '4' => Some(&DIGIT_4),
        '5' => Some(&DIGIT_5),
        '6' => Some(&DIGIT_6),
        '7' => Some(&DIGIT_7),
        '8' => Some(&DIGIT_8),
        '9' => Some(&DIGIT_9),
        
        'a' | 'A' => Some(&CHAR_A),
        'b' | 'B' => Some(&CHAR_B),
        'c' | 'C' => Some(&CHAR_C),
        'd' | 'D' => Some(&CHAR_D),
        'e' | 'E' => Some(&CHAR_E),
        'f' | 'F' => Some(&CHAR_F),
        'g' | 'G' => Some(&CHAR_G),
        'h' | 'H' => Some(&CHAR_H),
        'i' | 'I' => Some(&CHAR_I),
        'j' | 'J' => Some(&CHAR_J),
        'k' | 'K' => Some(&CHAR_K),
        'l' | 'L' => Some(&CHAR_L),
        'm' | 'M' => Some(&CHAR_M),
        'n' | 'N' => Some(&CHAR_N),
        'o' | 'O' => Some(&CHAR_O),
        'p' | 'P' => Some(&CHAR_P),
        'q' | 'Q' => Some(&CHAR_Q),
        'r' | 'R' => Some(&CHAR_R),
        's' | 'S' => Some(&CHAR_S),
        't' | 'T' => Some(&CHAR_T),
        'u' | 'U' => Some(&CHAR_U),
        'v' | 'V' => Some(&CHAR_V),
        'w' | 'W' => Some(&CHAR_W),
        'x' | 'X' => Some(&CHAR_X),
        'y' | 'Y' => Some(&CHAR_Y),
        'z' | 'Z' => Some(&CHAR_Z),
        
        '!'  => Some(&EXCL),
        '?'  => Some(&QUEST),
        '"'  => Some(&DQUOT),
        '\'' => Some(&QUOT),
        '+'  => Some(&PLUS),
        '-'  => Some(&MINUS),
        '='  => Some(&EQ),
        '.'  => Some(&PERIOD),
        ','  => Some(&COMMA),
        ':'  => Some(&COLON),
        ';'  => Some(&SEMICOLON),
        '/'  => Some(&SLASH),
        '%'  => Some(&PERCENT),
        '('  => Some(&RBRL),
        ')'  => Some(&RBRR),
        '_'  => Some(&UNDERSCORE),
        ' '  => Some(&SPACE),
        
        _ => None
    }
}

pub const DIGIT_0: Symbol = [
    [true, true, true],
    [true, false, true],
    [true, false, true],
    [true, false, true],
    [true, true, true]
];

pub const DIGIT_1: Symbol = [
    [true, true, false],
    [false, true, false],
    [false, true, false],
    [false, true, false],
    [true, true, true]
];

pub const DIGIT_2: Symbol = [
    [true, true, true],
    [false, false, true],
    [true, true, true],
    [true, false, false],
    [true, true, true]
];

pub const DIGIT_3: Symbol = [
    [true, true, true],
    [false, false, true],
    [false, true, true],
    [false, false, true],
    [true, true, true]
];

pub const DIGIT_4: Symbol = [
    [true, false, true],
    [true, false, true],
    [true, true, true],
    [false, false, true],
    [false, false, true]
];

pub const DIGIT_5: Symbol = [
    [true, true, true],
    [true, false, false],
    [true, true, true],
    [false, false, true],
    [true, true, true]
];

pub const DIGIT_6: Symbol = [
    [true, false, false],
    [true, false, false],
    [true, true, true],
    [true, false, true],
    [true, true, true]
];

pub const DIGIT_7: Symbol = [
    [true, true, true],
    [false, false, true],
    [false, false, true],
    [false, false, true],
    [false, false, true]
];

pub const DIGIT_8: Symbol = [
    [true, true, true],
    [true, false, true],
    [true, true, true],
    [true, false, true],
    [true, true, true]
];

pub const DIGIT_9: Symbol = [
    [true, true, true],
    [true, false, true],
    [true, true, true],
    [false, false, true],
    [false, false, true]
];

pub const CHAR_A: Symbol = [
    [true, true, true],
    [true, false, true],
    [true, true, true],
    [true, false, true],
    [true, false, true]
];

pub const CHAR_B: Symbol = [
    [true, true, true],
    [true, false, true],
    [true, true, false],
    [true, false, true],
    [true, true, true]
];

pub const CHAR_C: Symbol = [
    [false, true, true],
    [true, false, false],
    [true, false, false],
    [true, false, false],
    [false, true, true]
];

pub const CHAR_D: Symbol = [
    [true, true, false],
    [true, false, true],
    [true, false, true],
    [true, false, true],
    [true, true, true]
];

pub const CHAR_E: Symbol = [
    [true, true, true],
    [true, false, false],
    [true, true, false],
    [true, false, false],
    [true, true, true]
];

pub const CHAR_F: Symbol = [
    [true, true, true],
    [true, false, false],
    [true, true, false],
    [true, false, false],
    [true, false, false]
];

pub const CHAR_G: Symbol = [
    [false, true, true],
    [true, false, false],
    [true, false, false],
    [true, false, true],
    [true, true, true]
];

pub const CHAR_H: Symbol = [
    [true, false, true],
    [true, false, true],
    [true, true, true],
    [true, false, true],
    [true, false, true]
];

pub const CHAR_I: Symbol = [
    [true, true, true],
    [false, true, false],
    [false, true, false],
    [false, true, false],
    [true, true, true]
];

pub const CHAR_J: Symbol = [
    [true, true, true],
    [false, true, false],
    [false, true, false],
    [false, true, false],
    [true, true, false]
];

pub const CHAR_K: Symbol = [
    [true, false, true],
    [true, false, true],
    [true, true, false],
    [true, false, true],
    [true, false, true]
];

pub const CHAR_L: Symbol = [
    [true, false, false],
    [true, false, false],
    [true, false, false],
    [true, false, false],
    [true, true, true]
];

pub const CHAR_M: Symbol = [
    [true, true, true],
    [true, true, true],
    [true, false, true],
    [true, false, true],
    [true, false, true]
];

pub const CHAR_N: Symbol = [
    [true, true, false],
    [true, false, true],
    [true, false, true],
    [true, false, true],
    [true, false, true]
];

pub const CHAR_O: Symbol = [
    [false, true, true],
    [true, false, true],
    [true, false, true],
    [true, false, true],
    [true, true, false]
];

pub const CHAR_P: Symbol = [
    [true, true, true],
    [true, false, true],
    [true, true, true],
    [true, false, false],
    [true, false, false]
];

pub const CHAR_Q: Symbol = [
    [false, true, false],
    [true, false, true],
    [true, false, true],
    [true, true, false],
    [false, true, true]
];

pub const CHAR_R: Symbol = [
    [true, true, true],
    [true, false, true],
    [true, true, false],
    [true, false, true],
    [true, false, true]
];

pub const CHAR_S: Symbol = [
    [false, true, true],
    [true, false, false],
    [true, true, true],
    [false, false, true],
    [true, true, false]
];

pub const CHAR_T: Symbol = [
    [true, true, true],
    [false, true, false],
    [false, true, false],
    [false, true, false],
    [false, true, false]
];

pub const CHAR_U: Symbol = [
    [true, false, true],
    [true, false, true],
    [true, false, true],
    [true, false, true],
    [false, true, true]
];

pub const CHAR_V: Symbol = [
    [true, false, true],
    [true, false, true],
    [true, false, true],
    [true, true, true],
    [false, true, false]
];

pub const CHAR_W: Symbol = [
    [true, false, true],
    [true, false, true],
    [true, false, true],
    [true, true, true],
    [true, true, true]
];

pub const CHAR_X: Symbol = [
    [true, false, true],
    [true, false, true],
    [false, true, false],
    [true, false, true],
    [true, false, true]
];

pub const CHAR_Y: Symbol = [
    [true, false, true],
    [true, false, true],
    [true, true, true],
    [false, false, true],
    [true, true, true]
];

pub const CHAR_Z: Symbol = [
    [true, true, true],
    [false, false, true],
    [false, true, false],
    [true, false, false],
    [true, true, true]
];

pub const EXCL: Symbol = [
    [false, true, false],
    [false, true, false],
    [false, true, false],
    [false, false, false],
    [false, true, false]
];

pub const QUEST: Symbol = [
    [true, true, true],
    [false, false, true],
    [false, true, true],
    [false, false, false],
    [false, true, false]
];

pub const DQUOT: Symbol = [
    [true, false, true],
    [true, false, true],
    [false, false, false],
    [false, false, false],
    [false, false, false]
];

pub const QUOT: Symbol = [
    [false, true, false],
    [false, true, false],
    [false, false, false],
    [false, false, false],
    [false, false, false]
];

pub const PLUS: Symbol = [
    [false, false, false],
    [false, true, false],
    [true, true, true],
    [false, true, false],
    [false, false, false]
];

pub const MINUS: Symbol = [
    [false, false, false],
    [false, false, false],
    [true, true, true],
    [false, false, false],
    [false, false, false]
];

pub const EQ: Symbol = [
    [false, false, false],
    [true, true, true],
    [false, false, false],
    [true, true, true],
    [false, false, false]
];

pub const PERIOD: Symbol = [
    [false, false, false],
    [false, false, false],
    [false, false, false],
    [false, false, false],
    [false, true, false]
];

pub const COMMA: Symbol = [
    [false, false, false],
    [false, false, false],
    [false, false, false],
    [false, true, false],
    [true, false, false]
];

pub const COLON: Symbol = [
    [false, false, false],
    [false, true, false],
    [false, false, false],
    [false, true, false],
    [false, false, false]
];

pub const SEMICOLON: Symbol = [
    [false, false, false],
    [false, true, false],
    [false, false, false],
    [false, true, false],
    [true, false, false]
];

pub const SLASH: Symbol = [
    [false, false, true],
    [false, true, false],
    [false, true, false],
    [false, true, false],
    [true, false, false]
];

pub const PERCENT: Symbol = [
    [true, false, true],
    [false, false, true],
    [false, true, false],
    [true, false, false],
    [true, false, true]
];

pub const RBRL: Symbol = [
    [false, true, false],
    [true, false, false],
    [true, false, false],
    [true, false, false],
    [false, true, false]
];

pub const RBRR: Symbol = [
    [false, true, false],
    [false, false, true],
    [false, false, true],
    [false, false, true],
    [false, true, false]
];

pub const UNDERSCORE: Symbol = [
    [false, false, false],
    [false, false, false],
    [false, false, false],
    [false, false, false],
    [true, true, true]
];

pub const SPACE: Symbol = [
    [false, false, false],
    [false, false, false],
    [false, false, false],
    [false, false, false],
    [false, false, false]
];