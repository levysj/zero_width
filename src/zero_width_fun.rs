use std::char;
use std::u8;
use ZeroWidthError;
use std::error::Error;



static ZERO_WIDTH_SPACE: char = '​';  //let ZERO_WIDTH_SPACE: char = char::from_u32(0x200b).unwrap();
static ZERO_WIDTH_NONJOINER: char = '‌';  //let ZERO_WIDTH_NONJOINER: char = char::from_u32(0x200c).unwrap();
static ZERO_WIDTH_JOINER: char = '‍';  //let ZERO_WIDTH_JOINER: char = char::from_u32(0x200d).unwrap();

///detects if a string has any zero width string characters
pub fn detect_zero_width(str: &String) -> bool {
    str.contains(ZERO_WIDTH_SPACE) && str.contains(ZERO_WIDTH_NONJOINER) && str.contains(ZERO_WIDTH_JOINER) 
}



///converts a string to a zero width string
pub fn to_zero_width(strs: &String) -> String {
    let mut zero_width = String::new();
    for b in strs.bytes() {
        for i in (0..8).rev() {
            if (b & 2_u8.pow(i)) >= 1 {
                zero_width.push(ZERO_WIDTH_SPACE); //zero width space
            } else if (b & 2_u8.pow(i)) == 0 {
                zero_width.push(ZERO_WIDTH_NONJOINER); //zero width non-joiner
            }
        }
        zero_width.push(ZERO_WIDTH_JOINER);
    }
    zero_width
}

///converts a zero width string to its equivilent normal string
pub fn from_zero_width(strs: &String) -> Result<String, ZeroWidthError> {
    let mut ret: Vec<u8> = Vec::new();
    let mut byte: u8 = 0000_0000;
    for ch in strs.chars() {
        //println!("{}", ch.escape_unicode().to_string());
        if ch == ZERO_WIDTH_SPACE {
            byte <<= 1;
            byte += 1;
        } else if ch == ZERO_WIDTH_NONJOINER {
            byte <<= 1;
        } else if ch == ZERO_WIDTH_JOINER {
            ret.push(byte);
            byte = 0000_0000;
        } else {
            return Err(ZeroWidthError::new("Error: Invalid Character")) 
        }
    }
    match String::from_utf8(ret) {
        Ok(o) => Ok(o),
        Err(e) => Err(ZeroWidthError::new(e.description())),
    }
}

///Creates string from zero width String Literal
pub fn from_zero_width_str(strs: &str) -> Result<String, ZeroWidthError> {
    from_zero_width(&String::from(strs))
}

///Create a zero width string from String Literal
pub fn to_zero_width_str(strs: &str) -> String {
    to_zero_width(&String::from(strs))
}