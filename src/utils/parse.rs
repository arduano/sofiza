use std::collections::HashMap;

use lazy_static::lazy_static;

fn build_key_name_map() -> HashMap<String, i8> {
    let mut map = HashMap::new();
    let keynames = [
        ["a", "a"],
        ["a#", "bb"],
        ["b", "b"],
        ["c", "c"],
        ["c#", "db"],
        ["d", "d"],
        ["d#", "eb"],
        ["e", "e"],
        ["f", "f"],
        ["f#", "gb"],
        ["g", "g"],
        ["g#", "ab"],
    ];
    for i in -1i8..=127 {
        let key = (i as i32 + 4) % 12;
        let octave = (i as i32 + 1) / 12 - 1;
        let name = format!("{}{}", keynames[key as usize][0], octave);
        map.insert(name, i as i8);
        let name = format!("{}{}", keynames[key as usize][1], octave);
        map.insert(name, i as i8);
    }
    map
}

fn key_name_map() -> &'static HashMap<String, i8> {
    lazy_static! {
        static ref KEY_MAP: HashMap<String, i8> = build_key_name_map();
    };

    &KEY_MAP
}

/// Receive a string, try to parse it as f32
///
pub(crate) fn check_f32(value: &str) -> f32 {
    let num: f32 = value
        .parse::<f32>()
        .expect(&format!("ERROR: `{}` is not a valid f32 number", value));
    num
}

/// Receive a string, try to parse it as f32 between a given range
///
// NOTE: floating-point types cannot be used in patterns
// https://github.com/rust-lang/rust/issues/41620
// solution from: https://stackoverflow.com/a/58434531/940200
///
pub(crate) fn check_f32_between(value: &str, min: f32, max: f32) -> Option<f32> {
    let num = check_f32(value);
    match num {
        num if (min..=max).contains(&num) => Some(num),
        _ => None,
    }
}

/// Receive a string, try to parse it as key or u8
///
pub(crate) fn check_u8_key(value: &str) -> Option<u8> {
    let keyval = key_name_map().get(&value.to_lowercase());
    if let Some(key) = keyval {
        let key = *key;
        if key < 0 {
            return None;
        }
        return Some(key as u8);
    }
    let num: u8 = value
        .parse::<u8>()
        .expect(&format!("ERROR: `{}` is not a valid i8 number", value));
    if num > 127 {
        return None;
    }
    Some(num)
}

/// Receive a string, try to parse it as key or i8
///
pub(crate) fn check_i8_key(value: &str) -> Option<i8> {
    let keyval = key_name_map().get(&value.to_lowercase());
    if let Some(key) = keyval {
        let key = *key;
        if key < -1 {
            return None;
        }
        return Some(key as i8);
    }
    let num: i8 = value
        .parse::<i8>()
        .expect(&format!("ERROR: `{}` is not a valid i8 number", value));
    if num < -1 {
        return None;
    }
    Some(num)
}

/// Receive a string, try to parse it as u8
///
pub(crate) fn check_u8(value: &str) -> u8 {
    let num: u8 = value
        .parse::<u8>()
        .expect(&format!("ERROR: `{}` is not a valid i8 number", value));
    num
}

/// Receive a string, try to parse it as u8 between a given range
///
pub(crate) fn check_u8_between(value: &str, min: u8, max: u8) -> Option<u8> {
    let num = check_u8(value);
    if num >= min && num <= max {
        return Some(num);
    } else {
        None
    }
}

/// Receive a string, try to parse it as i8
///
pub(crate) fn check_i8(value: &str) -> i8 {
    let num: i8 = value
        .parse::<i8>()
        .expect(&format!("ERROR: `{}` is not a valid i8 number", value));
    num
}

/// Receive a string, try to parse it as i8 between a given range
///
pub(crate) fn check_i8_between(value: &str, min: i8, max: i8) -> Option<i8> {
    let num = check_i8(value);
    if num >= min && num <= max {
        return Some(num);
    } else {
        None
    }
}

/// Receive a string, try to parse it as i16
///
pub(crate) fn check_i16(value: &str) -> i16 {
    let num: i16 = value
        .parse::<i16>()
        .expect(&format!("ERROR: `{}` is not a valid i16 number", value));
    num
}

/// Receive a string, try to parse it as i16 between a given range
///
pub(crate) fn check_i16_between(value: &str, min: i16, max: i16) -> Option<i16> {
    let num = check_i16(value);
    if num >= min && num <= max {
        return Some(num);
    } else {
        None
    }
}

/// Receive a string, try to parse it as u16
///
pub(crate) fn check_u16(value: &str) -> u16 {
    let num: u16 = value
        .parse::<u16>()
        .expect(&format!("ERROR: `{}` is not a valid u16 number", value));
    num
}

/// Receive a string, try to parse it as u16 between a given range
///
pub(crate) fn check_u16_between(value: &str, min: u16, max: u16) -> Option<u16> {
    let num = check_u16(value);
    if num >= min && num <= max {
        return Some(num);
    } else {
        None
    }
}

/// Receive a string, try to parse it as u32
///
pub(crate) fn check_u32(value: &str) -> u32 {
    let num: u32 = value
        .parse::<u32>()
        .expect(&format!("ERROR: `{}` is not a valid u32 number", value));
    num
}

/// Receive a string, try to parse it as u32 between a given range
///
pub(crate) fn check_u32_between(value: &str, min: u32, max: u32) -> Option<u32> {
    let num = check_u32(value);
    if num >= min && num <= max {
        return Some(num);
    } else {
        None
    }
}

/*
// IDEA:WIP create wrapper macro
// check!(value, f32, 0.1, 100.)
//
// https://stackoverflow.com/questions/34214136/how-do-i-match-the-type-of-an-expression-in-a-rust-macro
// problem: know what to return
//
#[macro_export]
macro_rules! check {
    ( $value:expr, f32, $min:expr, $max:expr  ) => {
        let num = utils::check_f32_between($value, $min, $max);
        num.and(Some(Opcode::pan(num.unwrap_or_default())))
    };
}
*/

// IDEA:WIP make helper (generic?) function for parsing numbers
//
// pub(crate) fn test_helper<T: FromStr + Debug>(value: &str) -> T {
//     print_type(value);
//     let num  = value.parse::<T>()
//         .expect(&format!("ERROR: `{}` is not a valid {:?} number", value,
//                 "T"));
//                 //core::any::type_name::<f32>));
//     num
// }

// impl<T> Debug for <T as FromStr>::Err {
// }
