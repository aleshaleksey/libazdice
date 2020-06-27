use super::distribution::*;
use std::result::Result;

/// An important piece of shorthand.
const CANT: &str = "Can't parse, won't parse!";

/// an enum to store various parsing groups dynamically.
#[derive(Debug,Clone,PartialEq)]
enum ModifierGroup {    // The number is just there as demo.
    DropLowest(usize),  // dl2
    KeepHighest(usize), // kh2
    DropHighest(usize), // dh2
    KeepLowest(usize),  // kl2
    ReRollCount(usize), // rr4
    ReRollAbove(i64),   // ab2
    ReRollBelow(i64),   // be2
    CutOffMaximum(i64), // mx5
    CutOffMinimum(i64), // mn2
}

// A bunch of constants to define additional modifiers.
const DL: &str = "dl";  // DropLowest (KeepHighest)
const KH: &str = "kh";  // KeepHighest
const DH: &str = "dh";  // DropHighest
const KL: &str = "kl";  // KeepLowest (DropLowest)
const RR: &str = "rr";  // ReRoll
const AB: &str = "ab";  // Above
const BE: &str = "be";  // Below
const MX: &str = "mx";  // MaximumOf
const MN: &str = "mn";  // MinimumOf

// The logic of the parser is to consecutively split the string:
// 1) Split by operation to make dicegroups.

// 2) examine dicegroups for keywords `d`, `dl`, `dh` (in reverse order.)
/// The main outer parser function.
/// ```
/// use libazdice::parse::parse;
/// use libazdice::distribution::DiceBag;
/// // Let's say we have an elemental sorcerer who cannot roll lower than 2 rolling the dice.
/// let input_string = "8d6mn2".to_string();
/// let dice_bag = parse(input_string);
///
/// assert!(dice_bag.is_ok());
/// let dice_bag: DiceBag = dice_bag.unwrap();
/// for _ in 0..10_000 {
///     // Minimum is 8 x 2 = 16. Maximum  is 8 x 6 = 48.
///     assert!((dice_bag.roll().total() >= 16) && (dice_bag.roll().total() <= 48));
/// }
/// ```
pub fn parse(input:String) -> Result<DiceBag,String> {
    // Lowercase the string for simplicity.
    let input = input.to_lowercase();

    // Remove spaces and other crud.
    let input = input.chars().filter(|c| !c.is_whitespace() && !c.is_control()).collect::<String>();

    //Initial check.
    let chars = input.chars();
    for c in chars {
        if !valid_chars(c) {
            return Result::Err(format!("Input contained invalid character ({}).",c));
        }
    }

    // Parse to dice.
    let parsed_groups = map_ops_and_parse(input)?;

    // Convert to dicebag.
    let mut dice_bag = DiceBag::from_dice(parsed_groups);
    dice_bag.calculate_range();

    Ok(dice_bag)
}

/// Splits a whitespaceless String into ops.
pub(crate) fn map_ops_and_parse(input:String) -> Result<Vec<DiceGroup>,String> {
    let ops = std::iter::repeat(DiceOp::Add).take(1).chain(
        input.chars().filter_map(|c|char_to_op(c))
    );

    let groups = input.split(|c| (c=='+')||(c=='-'));
    if groups.clone().count()==0 {
        // A little parsing on the side.
        return Result::Err("Input contains no valid dice groups.".to_owned());
    }
    let mut output = Vec::with_capacity(groups.clone().count());
    for (op,group) in ops.zip(groups) {
        let mut dice_group = parse_string_to_dicegroup2(group)?;
        dice_group.add_op(op);
        output.push(dice_group);
    }
    Ok(output)
}

// REWORKING OF PARSING STRATEGY:
// 1) Check that numeric only: Then it becomes a bonus.
// 2) Check for d followed by numeric. If failed, then we don't want it.
// 2.5) Check for last character being exclamation maek (not implemented!) and remove it.
// 3) Take until first non 'd' non numeric.
// 3.5) Parse first section.
// 4) Split by two letter groups.
// 4.5) Check for undesired groups and for possibility of parsing in between values.
// 5) Parse various clauses.

fn parse_string_to_dicegroup2(input:&str) -> Result<DiceGroup,String> {
    // Check for pure number. Then we have a bonus.
    let mut input = input.to_owned();
    if !input.contains(|c:char| !c.is_numeric()) {
        return match input.parse::<i64>() {
            Ok(n) => Ok(DiceGroup::bonus(n)),
            Err(e) => Result::Err(format!("Could not parse dice: {:?}",e)),
        };
    }

    // If we do not have a bonus, we must have a dice. If we do not have "dX", we have a poo.
    if !has_d_numeric(&input) {
        return Err(format!(
            "Dice group ({}) is neither dice nor bonus. Can't parse! Won't parse!",
            input,
        ));
    }

    let explosive = if input.chars().last()==Some('!') {
        input.pop();
        true
    }else{
        false
    };

    let (base_group, remainder) = take_until_nx(input, 2, &is_letter);
    let mut base_dice = parse_base_dice2(base_group)?;
    base_dice.is_explosive(explosive);

    if let Some(c) = remainder.chars().next() {
        // If the first character aint a letter, something's wrong. Very, very wrong.
        if !is_letter(c) {
            return Err(format!("Modifiers must start with a letter but start with ({}). {}",c, CANT));
        }else if let DiceGroup::Bonus(_) = base_dice {
            return Err(format!("A bonus must not have modifiers! {}", CANT));
        }
    } else {
        // If remainder is empty then we have no modifiers.
        return Ok(base_dice);
    }

    if let DiceGroup::Dice(ref mut dice) = base_dice {
        parse_conditional_clauses(remainder, dice)?;
    }
    Ok(base_dice)
}

/// A function which deals with the tail group eg "dl6dh3rr3be3mn2"
/// Strategy:
/// Split the group into letter and number groups. Zip and decode each one.
fn parse_conditional_clauses(
    input:String,
    base_dice: &mut Dice
) -> Result<(), String> {
    let mut list_cond: Vec<String> = Vec::new();
    let mut list_num: Vec<String> = Vec::new();

    let mut input2 = input;
    while !input2.is_empty() {
        // Letters stage.
        let (in_l, rem) = take_until_nx(input2, 1, &is_numeric);
        list_cond.push(in_l);
        if rem.is_empty() { break; }
        // Numbers stage.
        let (in2, rem) = take_until_nx(rem, 2, &is_letter);
        list_num.push(in2);
        input2 = rem;
    }
    // SAN check.
    if list_cond.len() != list_num.len() {
        return Err(format!("We've got a really weird input string! {}", CANT));
    }

    // A little parsing on the side.
    let list_num = list_num.into_iter();

    // Now we try to parse.
    let mut groups = Vec::with_capacity(list_cond.len());
    for (num, cond) in list_num.zip(list_cond.into_iter()) {
        groups.push(make_group(num, cond)?);
    }
    // Finalise the dice
    fill_dice(groups, base_dice)
}

/// Turn splits into groups.
/// Errors can come from `num` failing to parse, or `cond` being invalid.
fn make_group(num: String, cond: String) -> Result<ModifierGroup, String> {
    // Nothing should be below 0.
    let n = num.parse::<usize>().map_err(|e| e.to_string())?;

    let group = match cond.as_str() {
         DL => ModifierGroup::DropLowest(n),
         KH => ModifierGroup::KeepHighest(n),
         KL => ModifierGroup::KeepLowest(n),
         DH => ModifierGroup::DropHighest(n),
         RR => ModifierGroup::ReRollCount(n),
         AB => ModifierGroup::ReRollAbove(n as i64),
         BE => ModifierGroup::ReRollBelow(n as i64),
         MX => ModifierGroup::CutOffMaximum(n as i64),
         MN => ModifierGroup::CutOffMinimum(n as i64),
         _  => return Err(format!("({}{}) not a valid modifier. {}", cond, n, CANT)),
    };
    Ok(group)
}

/// Finalises a dice group based on the `ModifierGroups`. Groups to add.
/// 1) `Reroll` clause,
/// 2) `CutOff` clause,
/// 3) `Drop` clause.
fn fill_dice(mods: Vec<ModifierGroup>, die: &mut Dice) -> Result<(), String> {
    use self::ModifierGroup::*;
    // Make Drop.
    {
        let mut drop_highest = Drop::Non;
        let mut drop_lowest = Drop::Non;
        let mut had_lowest = false;
        let mut had_highest = false;
        'drop_loop: for mod_group in mods.iter() {
            match mod_group {
                DropLowest(n) => {
                    if had_lowest { return Err(format!("Multiple (dl/kh) clauses found! {}!", CANT)); }
                    had_lowest = true;
                    if die.count <= *n {
                        return Err(format!("Keeping more dice than you have({} vs {})! {}!", n, die.count, CANT));
                    }
                    drop_lowest = Drop::Lowest(*n);
                }
                KeepHighest(n) => {
                    if had_lowest { return Err(format!("Multiple (dl/kh) clauses found! {}!", CANT)); }
                    had_lowest = true;
                    if die.count <= *n {
                        return Err(format!("Keeping more dice than you have({} vs {})! {}!", n, die.count, CANT));
                    }
                    drop_lowest = Drop::Lowest(die.count - *n);
                }
                DropHighest(n) => {
                    if had_highest { return Err(format!("Multiple (dh/kl) clauses found! {}!", CANT)); }
                    had_highest = true;
                    if die.count <= *n {
                        return Err(format!("Keeping more dice than you have({} vs {})! {}!", n, die.count, CANT));
                    }
                    drop_highest = Drop::Highest(*n);
                }
                KeepLowest(n) => {
                    if had_highest { return Err(format!("Multiple (dh/kl) clauses found! {}!", CANT)); }
                    had_highest = true;
                    if die.count <= *n {
                        return Err(format!("Keeping more dice than you have({} vs {})! {}!", n, die.count, CANT));
                    }
                    drop_highest = Drop::Highest(die.count - *n);
                }
                _ => continue 'drop_loop,
            }
            // We must keep cycling to the end to make sure that we do not have contradictory clauses.
        }

        match (drop_highest, drop_lowest) {
            (Drop::Non, Drop::Non) => die.add_drop(Drop::non()),
            (Drop::Non, Drop::Lowest(n)) => die.add_drop(Drop::lowest(n)),
            (Drop::Highest(n), Drop::Non) => die.add_drop(Drop::highest(n)),
            (Drop::Highest(n), Drop::Lowest(m)) => {
                if m + n >= die.count {
                    return Err(format!("Dropping more dice than you have({} vs {})! {}!", n + m, die.count, CANT));
                }
                let drop_vector = ((m)..(die.count - n)).collect::<Vec<_>>();
                die.add_drop(Drop::custom(drop_vector));
            },
            _ => return Err("Impossible drop combo. How'd you do it?".to_owned()),
        }
    }

    // Make ReRoll
    {
        let mut count = None;
        let mut ex_threshold = None;
        let mut above = false;
        'reroll_loop: for mod_group in mods.iter() {
            match mod_group {
                ModifierGroup::ReRollCount(n) => {
                    if count.is_none() {
                        if die.count < *n {
                            return Err(format!("Re-rolling more dice than you have({} vs {})! {}!", n, die.count, CANT));
                        }
                        count = Some(*n);
                    } else {
                        return Err(format!("Multiple reroll counts found! {}", CANT));
                    }
                }
                ModifierGroup::ReRollAbove(x) => {
                    above = true;
                    if ex_threshold.is_none() {
                        if die.size <= *x {
                            return Err(format!("Re-rolling more dice than you have({} vs {})! {}!", x, die.size, CANT));
                        }
                        ex_threshold = Some(*x);
                    } else {
                        return Err(format!("Multiple reroll conditions found! {}", CANT));
                    }
                }
                ModifierGroup::ReRollBelow(x) => {
                    if ex_threshold.is_none() {
                        if 1 >= *x {
                            return Err(format!("Re-rolling more dice than you have({} vs {})! {}!", x, die.count, CANT));
                        }
                        ex_threshold = Some(*x);
                    } else {
                        return Err(format!("Multiple reroll conditions found! {}", CANT));
                    }
                }
                _ => continue 'reroll_loop,
            }
            // We must keep cycling to the end to make sure there is only one of each.
        }

        match (count, ex_threshold) {
            (None, Some(_))|(Some(_), None) => {
                return Err(format!("Incomplete reroll clause: {}", CANT));
            }
            (Some(n), Some(x)) => if above {
                die.add_reroll_if_above(x, n);
            } else {
                die.add_reroll_if_below(x, n);
            },
            _ => {}
        }
    }

    // Make CutOff
    {
        let mut cutoff_min = CutOff::Non;
        let mut cutoff_max = CutOff::Non;
        'cutoff_loop: for mod_group in mods.iter() {
            match mod_group {
                ModifierGroup::CutOffMaximum(x) => {
                    if cutoff_max != CutOff::Non { return Err(format!("Multiple cutoff clauses found! {}!", CANT)); }
                    if (die.size <= *x) || (*x < 1) {
                        return Err(format!("Cut-off Maximum is ridiculous ({} for a d{}). {}.",x, die.size, CANT));
                    }
                    cutoff_max = CutOff::Maximum(*x);
                }
                ModifierGroup::CutOffMinimum(x) => {
                    if cutoff_min != CutOff::Non { return Err(format!("Multiple cutoff clauses found! {}!", CANT)); }
                    if (die.size < *x) || (*x <= 1) {
                        return Err(format!("Cut-off Minimum is ridiculous ({} for a d{}). {}.",x, die.size, CANT));
                    }
                    cutoff_min = CutOff::Minimum(*x);
                }
                _ => continue 'cutoff_loop,
            }
        }
        let cutoff = match (cutoff_min, cutoff_max) {
                (CutOff::Non, CutOff::Non) => CutOff::Non,
                (CutOff::Minimum(x), CutOff::Non) => CutOff::Minimum(x),
                (CutOff::Non, CutOff::Maximum(x)) => CutOff::Maximum(x),
                (CutOff::Minimum(x), CutOff::Maximum(y)) => {
                    if x > y {
                        return Err(format!("Maximum is bigger than minimum. {}!", CANT));
                    } else {
                        CutOff::Both(MinMax([x, y]))
                    }
                }
                _ => return Err(format!("MinMax Error! {}!", CANT)),
        };

        die.add_checked_cutoff(cutoff);
    }
    Ok(())
}

/// Needs to be done because rust is typed and I am not clever.
fn is_letter(c:char) -> bool {
    c.is_alphabetic() && !c.is_numeric()
}

/// Needs to be done because rust is typed and I am not clever.
fn is_numeric(c:char) -> bool {
    c.is_numeric()
}

/// A nom-like function to split a string when N chars meet a certain condition.
/// NB: This is a copy function. Also it is not quite good.
fn take_until_nx(input:String, n:usize, fullfills_condition: &dyn Fn(char)->bool) -> (String,String) {
    let input = input.chars().collect::<Vec<char>>();

    let mut base_group = String::new();
    let mut tail = String::new();

    // Fill out the basae group, and inevitably start on tail.
    let mut tail_start = 0;
    let mut over = false;

    if input.len() < n {
        return (input.iter().collect::<String>(), tail);
    } else if n==1 {
        let mut start_tail = input.len();
        for i in 0..input.len() {
            if !fullfills_condition(input[i]) {
                base_group.push(input[i]);
            } else {
                start_tail = i;
                break;
            }
        }
        for i in start_tail..input.len() {
            tail.push(input[i]);
        }
        return (base_group, tail);
    }
    // First part.
    'outer: for i in 0..(input.len()-n) {
        let mut done = true;
        'inner: for j in 0..n {
            if !fullfills_condition(input[i+j]) {
                done = false;
                break 'inner;
            }
        }
        if done {
            over = true;
            tail_start = i;
            break 'outer;
        } else {
            base_group.push(input[i]);
        }
    }

    // Second part
    if !over {
        for i in (input.len()-n)..input.len() {
            base_group.push(input[i]);
        }
        tail_start = input.len();
    }

    // Fill out the tail
    for i in tail_start..input.len() {
        tail.push(input[i]);
    }
    (base_group, tail)
}

/// A nom-like non-macro function. Checks for "dX". If has then is a dice.
fn has_d_numeric(checked: &str) -> bool {
    let mut chars = checked.chars().peekable();

    // The fact that the string might be longer is irrelevant. We are using `Option`.
    while let Some(c1) = chars.next() {
        if let Some(c2) = chars.peek() {
            if (c1=='d') && c2.is_numeric() {
                return true;
            }
        }
    }
    false
}

/// NB: Must not be a bonus. Must be "dX" or "YdX".
fn parse_base_dice2(base_group: String) -> Result<DiceGroup,String> {
    // If we have a `d` (eg `2d6`), try to parse it as such.
    let splits = base_group.split('d').collect::<Vec<_>>();
    if splits.len() != 2 {
        // only one `d` is allowed in `XdY`.
        return Result::Err(format!("({}) is not",base_group));
    }

    // If format `d20` is used, this is assumed to be `1d20`.
    let counts = if let Ok(n) = splits[0].parse::<usize>() {
        n
    } else if splits[0].chars().count()==0 {
        1
    } else {
        return Result::Err(format!("Could not parse dice: {:?}",base_group));
    };

    // anything afer the `d` is the dice size.
    let size = if let Ok(n) = splits[1].parse::<i64>() {
        n
    } else {
        return Result::Err(format!("Could not parse dice: {:?}",base_group));
    };

    Ok(DiceGroup::dice(size,counts))
}


fn char_to_op(char:char) -> Option<DiceOp> {
    match char {
        '+' => Some(DiceOp::Add),
        '-' => Some(DiceOp::Sub),
        _ => None
    }
}


fn valid_chars(c:char) -> bool {
    match c {
        '+'|'-'|'d'|'l'|'k'|'x'|'h'|'r'|'b'|'e'|'a'|'m'|'!'|'n' => true,
        c => c.is_numeric(),
    }
}
