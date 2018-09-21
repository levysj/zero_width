//! # zero_width
//! 
//! `zero_width` is a collection of functions for creating 
//! and embedding zero width strings
use std::error::Error;
use std::fmt;
use std::thread;
use std::sync::Mutex;

pub mod zero_width_fun;

///Custom Error For Zero Width Strings
#[derive(Debug)]
pub struct ZeroWidthError {
    details: String,
}

impl ZeroWidthError {
    ///Method for Creating new ZeroWidthErrors
    pub fn new(msg: &str) -> ZeroWidthError {
        ZeroWidthError {details: msg.to_string()}
    }
    ///returns the details message
    pub fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for ZeroWidthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.details)
    }
}

impl Error for ZeroWidthError {
    fn description(&self) -> &str {
        &self.details
    }
}

///embbeds zero width strings in to text.
pub fn embbed_zero_width(pre: &String, to_zero: &String, post: &String) -> String {
    format!("{}{}{}",pre,zero_width_fun::to_zero_width(to_zero), post)
}
///returns the embbed zero width string if it exists
pub fn dembbed_zero_width(str: &String) -> Result<String, ZeroWidthError> {
    let zero_width_space: char = '​';  //let ZERO_WIDTH_SPACE: char = char::from_u32(0x200b).unwrap();
    let zero_width_nonjoiner: char = '‌';  //let ZERO_WIDTH_NONJOINER: char = char::from_u32(0x200c).unwrap();
    let zero_width_joiner: char = '‍';  //let ZERO_WIDTH_JOINER: char = char::from_u32(0x200d).unwrap();
    let mut zero_width = String::new();
    if zero_width_fun::detect_zero_width(str) {
        for char in str.chars() {
            if char == zero_width_space || char == zero_width_nonjoiner || char == zero_width_joiner {
                zero_width.push(char);
            }
        }
        zero_width_fun::from_zero_width(&zero_width)
    } else {
        Err(ZeroWidthError::new("Error: Message does not contain any Zero Width Strings"))
    }
}

pub fn concur_dembbed_zero_width(str: &'static String) -> Vec<String>{
    let mut ret: Vec<String> = Vec::new();
    let mut threads = vec![];
    for line in str.lines() {
        let handle = thread::spawn(move || {
            match dembbed_zero_width(&String::from(line)) {
                Ok(o) => ret.push(o),
                Err(_) => {},
            }
        });
        threads.push(handle);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    ret
}


#[cfg(test)]
mod tests {
    use std::char;
    use zero_width_fun;
    use { embbed_zero_width, dembbed_zero_width };
    #[test]
    fn test_to_zero_ascii() {
        let zero_width_space: char = char::from_u32(0x200b).unwrap();
        let zero_width_nonjoiner: char = char::from_u32(0x200c).unwrap();
        let zero_width_joiner: char = char::from_u32(0x200d).unwrap();
        let test = String::from("ABC");
        let mut answer = String::new();
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_joiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_joiner);
        assert_eq!(zero_width_fun::to_zero_width(&test), answer);
    }

    #[test]
    fn test_to_zero_utf8() {
        let zero_width_space: char = char::from_u32(0x200b).unwrap();
        let zero_width_nonjoiner: char = char::from_u32(0x200c).unwrap();
        let zero_width_joiner: char = char::from_u32(0x200d).unwrap();
        let test = String::from("Здрав");

        let mut answer = String::new();
        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        assert_eq!(zero_width_fun::to_zero_width(&test),answer);
    }

    #[test]
    fn test_from_zero_ascii() {
        let zero_width_space: char = char::from_u32(0x200b).unwrap();
        let zero_width_nonjoiner: char = char::from_u32(0x200c).unwrap();
        let zero_width_joiner: char = char::from_u32(0x200d).unwrap();
        let mut qes = String::new();
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_space);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_space);
        qes.push(zero_width_joiner);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_space);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_space);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_joiner);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_space);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_nonjoiner);
        qes.push(zero_width_space);
        qes.push(zero_width_space);
        qes.push(zero_width_joiner);

        let res = match zero_width_fun::from_zero_width(&qes) {
            Ok(r) => r,
            Err(e) => String::from(e.description()),
        };

        assert_eq!(res,"ABC");
    }

    #[test]
    fn test_from_zero_utf8() {
        let zero_width_space: char = char::from_u32(0x200b).unwrap();
        let zero_width_nonjoiner: char = char::from_u32(0x200c).unwrap();
        let zero_width_joiner: char = char::from_u32(0x200d).unwrap();

        let mut answer = String::new();
        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_space);
        answer.push(zero_width_nonjoiner);
        answer.push(zero_width_joiner);

        let res = match zero_width_fun::from_zero_width(&answer) {
            Ok(r) => r,
            Err(e) => String::from(e.description()),
        };

        assert_eq!(res,"Здрав");
    }

    #[test]
    fn test_from_zero_err_invalid_character() {
        let a = match zero_width_fun::from_zero_width_str("A") {
            Ok(o) => o,
            Err(e) => String::from(e.description()),
        };
        assert_eq!(a, String::from("Error: Invalid Character"))
    }

    #[test]
    fn test_detect_zero_width_succ() {
        let test = format!("{}{}{}","ABC",zero_width_fun::to_zero_width_str("ABC"),"efg");
        assert!(zero_width_fun::detect_zero_width(&test));
    }

    #[test]
    fn test_detect_zero_width_fail() {
        let test = String::from("ABCefg");
        assert!(!zero_width_fun::detect_zero_width(&test));
    }

    #[test]
    fn test_dembbed_zero_width() {
        let temp = String::from("ABC");
        let test = embbed_zero_width(&temp, &temp, &temp);

        let ans = match dembbed_zero_width(&test){
            Ok(o) => o,
            Err(e) => String::from(e.description()),
        };
        assert_eq!(temp, ans);
    }
}