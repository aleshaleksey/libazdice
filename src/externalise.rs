// This submodule is for externing various parts of the module to C or C++
use super::parse::parse;

use std::ffi::{CString,CStr};
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// This structure returns a list of results in a mostly human readable format.
pub struct Rolls {
    /// Length of the input string.
    pub len_input: u64,
    /// The original imput string.
    pub input: *const c_char,
    /// The length of the result vector.
    pub len_dice_groups: u64,
    /// The actual results.
    pub groups: *const i64,
    /// the results from a bonus.
    pub bonus: i64,
    /// The grand total.
    pub total: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Just in case we need to return lots of results.
pub struct ListRolls {
    /// The number of entries in the list.
    pub len: u64,
    /// A pointer to the detailed results (`Roll`s).
    pub results: *const Rolls,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Basically a result. Which allows the error string to be returned if necessary.
pub struct ResultListRolls {
    /// A pointer to a structure containing a list of roll results (`ListRolls`).
    pub succ: *const ListRolls,
    /// A pointer to an error string.
    pub err: *const CString,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Basically a result. Which allows the error string to be returned if necessary.
pub struct SingleRollResult {
    /// A value representing the numerical value of a dice roll.
    pub roll: i64,
    /// A pointer to an error string.
    pub err: *const CString,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// A structure representing a value and frequency.
/// It exists as a tidier way of using a direct array.
pub struct XY {
    /// A value representing the numerical value of a dice roll.
    pub value: i64,
    /// The number of times a value was rolled.
    pub frequency: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// A probability distribution generated from a parsed `DiceBag`. X and Y values are seperate.
pub struct Distribution {
    /// A list of `XY` values. Value of roll and Frequency of how often it came up.
    pub rolls_and_frequency: *const XY,
    /// Count of X-Values.
    pub count: u64,
    /// Length of the input string.
    pub len_input: u64,
    /// The original imput string.
    pub input: *const c_char,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// A wrapper that shows whether the `Distribution` has been returned succesfully.
pub struct DistributionResult {
    /// A pointer to a resulting `Distribution`.
    pub succ: *mut Distribution,
    /// A pointer to an error string.
    pub err: *const CString,
}

#[no_mangle]
/// A wrapper around `parse::parse` + `DiceBag::make_count_distribution` for C. As arguments it
/// takes:
///
/// `input`: the equivalent of C `char *`, (a string of bytes).
///
/// `l`; the byte length of `input`.
///
/// `n`: The number of rolls required. It is recommended to use at least 1,000,000 for this value.
///
/// This function returns a `DistributionResult` containing a pointer to a `Distribution` if
/// if succesful, or a pointer to a `CString` error if it fails.
pub unsafe extern "C" fn parse_and_generate_distribution(
    input: &*const c_char,
    l: u64,
    n: u64,
) -> DistributionResult {
    let input = input.to_owned();

    let mut final_result = DistributionResult {
        succ: ptr::null_mut(),
        err: ptr::null(),
    };
    // A little dangerous. But what can one expect from C-chan?
    // let input_string = unsafe { CString::from_raw(input) };
    let input_string = if let Ok(s) = CStr::from_ptr(input).to_str() {
        s.to_owned()
    } else {
        let err = b"Invalid dice string in calling environment.".to_vec();
        let err = Box::new(CString::from_vec_unchecked(err));
        final_result.err = Box::into_raw(err);
        return final_result;
    };

    let dice = match parse(input_string.to_string()) {
        // Error is fully dealt with. If future me messes up the error message, this should catch.
        Err(e) => {
            let e: Vec<u8> = e.as_bytes().to_vec();
            let e = if e.contains(&0) { b"Error parsing initial roll".to_vec() } else { e };
            let err = Box::new(CString::from_vec_unchecked(e));
            final_result.err = Box::into_raw(err);
            return final_result;
        }
        Ok(r)=> r,
    };

    let mut roll_and_frequencies = dice
        .make_count_distribution(n as usize)
        .into_iter()
        .map(|(x, y)| XY {
            value: x,
            frequency: y as u64,
        })
        .collect::<Vec<XY>>();

    // println!("rolls: {:?}\n freq: {:?}", roll_totals, frequency);
    println!("rolls_and_frequency: {:?}", roll_and_frequencies);
    let count = roll_and_frequencies.len() as u64; // Cannot be odd!

    let distribution = Distribution {
        rolls_and_frequency: roll_and_frequencies.as_mut_ptr(),
        count,
        len_input: l,
        input: input.clone(),
    };

    final_result.succ = Box::into_raw(Box::new(distribution));
    final_result
}

#[no_mangle]
/// A wrapper for `parse::parse` and `DiceBag::roll`, allowing parsing a string from C and generating
/// N rolls. As arguments it takes:
///
/// `input`: the equivalent of C `char *`, (a string of bytes).
///
/// `l`; the byte length of `input`.
///
/// `n`: The number of rolls required.
///
/// This function returns a `ResultListRolls`, which either gives a complex report of subrolls
/// if succesful (pointer to `ListRolls`), or a pointer to an error string otherwise.
///
/// NB: This function is fairly dangerous as it can fail if the input from C/C++ cannot be
/// expressed as a rust String, but what's a dice roller without a little risk?
pub unsafe extern "C" fn parse_and_roll_n_times(
    input: &*const c_char,
    l: u64,
    n: u64
) -> ResultListRolls {
    let input = input.to_owned();

    let mut final_result = ResultListRolls {
        succ: ptr::null(),
        err: ptr::null(),
    };
    // A little dangerous. But what can one expect from C-chan?
    // let input_string = unsafe { CString::from_raw(input) };
    let input_string = if let Ok(s) = CStr::from_ptr(input).to_str() {
        s.to_owned()
    } else {
        let err = b"Invalid dice string in calling environment.".to_vec();
        let err = Box::new(CString::from_vec_unchecked(err));
        final_result.err = Box::into_raw(err);
        return final_result;
    };

    let dice = match parse(input_string.to_string()) {
        // Error is fully dealt with. If future me messes up the error message, this should catch.
        Err(e) => {
            let e: Vec<u8> = e.as_bytes().to_vec();
            let e = if e.contains(&0) { b"Error parsing initial roll".to_vec() } else { e };
            let err = Box::new(CString::from_vec_unchecked(e));
            final_result.err = Box::into_raw(err);
            return final_result;
        }
        Ok(r)=> r,
    };

    let mut results = Vec::with_capacity(n as usize);
    for _ in 0..n {
        results.push(dice.roll());
    }

    let results:Vec<_> = results.iter().map(|res| {
        let results: Vec<i64> = res.get_dice_groups().iter().map(|x|x.total()).collect();

        Rolls {
            len_input: l,
            input: input.clone(),
            len_dice_groups: results.len() as u64,
            groups: results.as_ptr(),
            bonus: res.get_bonus().total() as i64,
            total: res.total() as i64,
        }
    }).collect();
    println!("Results construced!");
    let len = results.len() as u64;
    let results = results.as_ptr();
    let results = Box::new(ListRolls {
        len,
        results,
    });

    final_result.succ = Box::into_raw(results);
    final_result
}

#[no_mangle]
/// A wrapper for `parse::parse` and `DiceBag::roll`, allowing parsing a string from C and generating
/// a single roll. As arguments it takes:
///
/// `input`: the equivalent of C `char *`, (a string of bytes).
///
/// This function returns a `SingleRollResult`, with a `i64` (`long int`) regardless of success or
/// failure. Importantly a pointer to `CString` indicates an error, allowing errors to be
/// examined by the caller.
///
/// NB: This function is fairly dangerous as it can fail if the input from C/C++ cannot be
/// expressed as a rust String, but what's a dice roller without a little risk?
pub unsafe extern "C" fn parse_and_roll(input: &*const c_char) -> SingleRollResult {
    let input = input.to_owned();
    let mut final_result = SingleRollResult {
        roll: 0,
        err: ptr::null(),
    };

    let input_string = if let Ok(s) = CStr::from_ptr(input).to_str() {
        s.to_owned()
    } else {
        let err = b"Invalid dice string in calling environment.".to_vec();
        let err = Box::new(CString::from_vec_unchecked(err));
        final_result.err = Box::into_raw(err);
        return final_result;
    };

    let dice = match parse(input_string.to_owned()) {
        Err(e) => {
            // An error at the parsing stage is good here.
            // Of course the error string must make sense.
            let e: Vec<u8> = e.as_bytes().to_vec();
            let e = if e.contains(&0) { b"Error parsing initial roll".to_vec() } else { e };
            let err = Box::new(CString::from_vec_unchecked(e));
            final_result.err = Box::into_raw(err);
            return final_result;
        }
        Ok(r)=> r,
    };

    final_result.roll = dice.roll().total() as i64;
    final_result
}

#[no_mangle]
/// A wrapper for `parse::parse` and `DiceBag::roll`, allowing parsing a string from C and generating
/// a single roll. As arguments it takes:
///
/// `input`: the equivalent of C `char *`, (a string of bytes).
///
/// This function returns a `i64` (`long int`), and will crash the thread upon failure.
///
/// NB: This function is fairly dangerous as it can fail if the input from C/C++ cannot be
/// expressed as a rust String, but what's a dice roller without a little risk?
pub unsafe extern "C" fn parse_and_roll2(input: &*const c_char) -> i64 {
    let input = input.to_owned();
    // A little dangerous. But what can one expect from C-chan?
    let input_string = CStr::from_ptr(input).to_str().expect("Poop").to_owned();

    let dice = match parse(input_string.to_string()) {
        // We return a number so we must simply crash if we could not parse the input.
        Err(_e) => panic!("Panic! We can't parse!"),
        Ok(r)=> r,
    };
    let i = dice.roll().total();
    i
}

#[no_mangle]
/// A test function for crossing ffi.
pub extern "C" fn test(i:i64) -> *const c_char {
    let string = format!("{}",i);
    let string = string.as_bytes();
    let string = Box::new(CString::new(string).unwrap());
    string.into_raw()
}

#[no_mangle]
/// Another test function for crossing ffi.
pub unsafe extern "C" fn test2(i: &*const c_char) -> i64 {
    let input_string = CStr::from_ptr(*i).to_str().expect("Poop").to_owned();
    let r = input_string.len() as i64;
    r
}
