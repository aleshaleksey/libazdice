// This submodule is for externing various parts of the module to C or C++
use super::parse::parse;

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
#[no_mangle]
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
#[no_mangle]
#[derive(Debug, Clone, Copy)]
/// Just in case we need to return lots of results.
pub struct ListRolls {
    pub len: u64,
    pub results: *const Rolls,
}

#[repr(C)]
#[no_mangle]
#[derive(Debug, Clone, Copy)]
/// Basically a result. Which allows the error string to be returned if necessary.
pub struct ResultListRolls {
    pub succ: *const ListRolls,
    pub err: *const CString,
}

#[repr(C)]
#[no_mangle]
#[derive(Debug, Clone, Copy)]
/// Basically a result. Which allows the error string to be returned if necessary.
pub struct SingleRollResult {
    pub roll: i64,
    pub err: *const CString,
}

#[no_mangle]
/// A wrapper for parsing a string and returning a lot of rolls all at once.
/// As arguments it takes
/// `input`: the equivalent of C `char *`, (a string of bytes).
/// `l`; the byte length of `input`.
/// `n`: The number of rolls required.
///
/// NB: This function is fairly dangerous as it can fail if the input from C/C++ cannot be
/// expressed as a rust String, but what's a dice roller without a little risk?
pub extern "C" fn parse_and_roll_n_times(
    input: *mut c_char,
    l: u64,
    n: u64
) -> ResultListRolls {
    println!("Rustside!");
    // A little dangerous. But what can one expect from C-chan?
    let input_string = unsafe { CString::from_raw(input.clone()) };
    let input_string = input_string.to_string_lossy();
    println!("Made input_string! {}",input_string);

    let dice = match parse(input_string.to_string()) {
        Err(e) => {
            // An error at the parsing stage is good here.
            // Of course the error string must make sense.
            let err = Box::new(CString::new(e.as_bytes()).expect("Error! Error!"));
            println!("We have a parsing error.");

            return ResultListRolls {
                succ: ptr::null(),
                err: Box::into_raw(err),
            };
        }
        Ok(r)=> r,
    };
    println!("Parsed succesfully!: {:?}", dice);

    let mut results = Vec::with_capacity(n as usize);
    for _ in 0..n {
        results.push(dice.roll());
    }

    let results:Vec<_> = results.iter().map(|res| {
        let results: Vec<i64> = res.get_dice_groups().iter().map(|x|x.total()).collect();

        Rolls {
            len_input: l,
            input: input,
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

    let r = ResultListRolls {
        succ: Box::into_raw(results),
        err: ptr::null(),
    };

    println!("sizeof<ResultListRolls> ={}", std::mem::size_of::<ResultListRolls>());
    println!("sizeof<ListRolls> ={}", std::mem::size_of::<ListRolls>());
    println!("sizeof<Rolls> ={}", std::mem::size_of::<Rolls>());
    println!("Final R = {:?}",r.clone());
    unsafe {
        println!("in r = {:?}", (*r.succ).results.clone());
        println!("in r.input = {:?}", (*(*r.succ).results).input.clone());
        println!("in r.groups = {:?}", (*(*r.succ).results).groups.clone());
    }
    r

}

#[no_mangle]
/// A wrapper for parsing a string and returning the results of a single roll.
/// As arguments it takes
/// `input`: the equivalent of C `char *`, (a string of bytes).
///
/// NB: This function is fairly dangerous as it can fail if the input from C/C++ cannot be
/// expressed as a rust String, but what's a dice roller without a little risk?
pub extern "C" fn parse_and_roll(input: *mut c_char) -> SingleRollResult {
    // A little dangerous. But what can one expect from C-chan?
    let input_string = unsafe { CString::from_raw(input.clone()) };
    let input_string = input_string.to_string_lossy();

    let dice = match parse(input_string.to_string()) {
        Err(e) => {
            // An error at the parsing stage is good here.
            // Of course the error string must make sense.
            let err = Box::new(CString::new(e.as_bytes()).expect("Error! Error!"));
            return SingleRollResult {
                roll: 0,
                err: Box::into_raw(err),
            }
        }
        Ok(r)=> r,
    };
    println!("Parsed succesfully!: {:?}", dice);
    SingleRollResult {
        roll: dice.roll().total() as i64,
        err: ptr::null(),
    }
}

#[no_mangle]
/// A wrapper for parsing a string and returning the results of a single roll.
/// As arguments it takes
/// `input`: the equivalent of C `char *`, (a string of bytes).
///
/// NB: This function is fairly dangerous as it can fail if the input from C/C++ cannot be
/// expressed as a rust String, but what's a dice roller without a little risk?
pub extern "C" fn parse_and_roll2(input: *mut c_char) -> i64 {
    // A little dangerous. But what can one expect from C-chan?
    let input_string = unsafe { CString::from_raw(input.clone()) };
    let input_string = input_string.to_string_lossy();

    let dice = match parse(input_string.to_string()) {
        Err(_e) => {
            // An error at the parsing stage is good here.
            // Of course the error string must make sense.
            panic!("Panic! We can't parse!")
        }
        Ok(r)=> r,
    };
    println!("Parsed succesfully!: {:?}", dice);
    let i = dice.roll().total();
    println!("We have a winner:{}",i);
    i
}

#[no_mangle]
pub extern "C" fn test(i:i64) -> *const c_char {
    let string = format!("{}",i);
    let string = string.as_bytes();
    let string = Box::new(CString::new(string).unwrap());
    string.into_raw()
    // CString::new(string).unwrap()
}

#[no_mangle]
pub extern "C" fn test2(i:*mut c_char) -> i64 {
    let input_string = unsafe { CString::from_raw(i.clone()) };
    let input_string = input_string.to_string_lossy();
    println!("rustside = {}",input_string.len() as i64);
    let r = input_string.len() as i64;
    r
    // CString::new(string).unwrap()
}
