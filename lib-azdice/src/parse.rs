use super::distribution::*;
use std::result::Result;

// The logic of the parser is to consecutively split the string:
// 1) Split by operation to make dicegroups.

// 2) examine dicegroups for keywords `d`, `dl`, `dh` (in reverse order.)
/// The main outer parser function.
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
    let parsed_groups = map_ops(input)?;


    // Convert to dicebag.
    let mut dice_bag = DiceBag::from_dice(parsed_groups);
    dice_bag.calculate_range();

    Ok(dice_bag)
}

/// Splits a whitespaceless String into ops.
pub(crate) fn map_ops(input:String) -> Result<Vec<DiceGroup>,String> {
    let ops = std::iter::repeat(DiceOp::Add).take(1).chain(
        input.chars().filter_map(|c|char_to_op(c))
    );

    let groups = input.split(|c| (c=='+')||(c=='-'));
    if groups.clone().count()==0 {
        return Result::Err("Input contains no valid dice groups.".to_owned());
    }
    let mut output = Vec::with_capacity(groups.clone().count());
    for (op,group) in ops.zip(groups) {
        let mut dice_group = parse_string_to_dicegroup(group)?;
        dice_group.add_op(op);
        output.push(dice_group);
    }

    Ok(output)
}

pub(crate) fn parse_string_to_dicegroup(input:&str) -> Result<DiceGroup,String> {
    // This is a bad way of doing this.
    let count_drop_lowest = input.matches("dl").count() + input.matches("kh").count();
    let count_drop_highest = input.matches("dh").count() + input.matches("kl").count();

    // Sanity check for drop clauses. Simplifies code in inner.
    let dice = if count_drop_lowest > 1 || count_drop_highest > 1 {
        return Result::Err(
            "A dicegroup cannot have more than one drop lowest and one drop highest clause!".to_owned()
        );
    } else {
        // Parse the hopefully valid dice group.
        // Errors still possible, but the code should exclude most silliness.
        parse_to_dice_group_inner(input)?
    };


    Ok(dice)
}

/// This function assumes a single drop highest and/or a single Drop highest clause.
fn parse_to_dice_group_inner(input:&str) -> Result<DiceGroup,String> {
    let mut chars = input.chars().peekable();

    let mut accumulator = String::new();
    let mut substrings = Vec::new();
    let mut droppings = Vec::new();
    while let Some(c) =  chars.next() {
        let c2 = chars.peek();

        if (c=='d') && (c2==Some(&'l')) {
            droppings.push("dl");
            substrings.push(accumulator.clone());
            accumulator.clear();
        } else if (c=='k') && (c2==Some(&'h')) {
            droppings.push("kh");
            substrings.push(accumulator.clone());
            accumulator.clear();
        } else if (c=='d') && (c2==Some(&'h')) {
            droppings.push("dh");
            substrings.push(accumulator.clone());
            accumulator.clear();
        } else if (c=='k') && (c2==Some(&'l')) {
            droppings.push("kl");
            substrings.push(accumulator.clone());
            accumulator.clear();
        }

        let do_not = (c2==Some(&'l')) || (c2==Some(&'h'));
        if c.is_numeric() || ((c=='d') && !do_not) {
            accumulator.push(c);
        }
    }
    substrings.push(accumulator);

    // SAN checks please!
    if substrings.is_empty() {
        return Result::Err("Empty Dice group detected! Can't parse, won't parse!".to_owned());
    } else if substrings.len() - 1 != droppings.len() {
        println!("Substrings: {:?}", substrings);
        println!("drops: {:?}", droppings);
        return Result::Err(format!("({}) is not a valid dice group.",input));
    }

    let remainder = substrings.remove(0).to_owned();

    let mut dice = parse_base_dice(remainder)?;

    match dice {
        DiceGroup::Dice(_) if droppings.is_empty() => Ok(dice),
        DiceGroup::Bonus(_) if droppings.is_empty() => Ok(dice),
        DiceGroup::Dice(ref mut d) if droppings.len() < 3 => {
            let mut final_drop_groups = Vec::new();
            for drop in droppings {
                // Parse count
                let count:usize = match substrings.pop().expect("We checked").parse() {
                    Ok(n) => n,
                    Err(e) => return Result::Err(format!("Could not parse drop count: {:?}",e)),
                };
                //Sanity check
                if count >= d.count {
                    return Result::Err(
                        "Dropping all the dice in a group or more. This is silly".to_owned()
                    );
                }
                let final_drop = if drop == "kh" {
                    Drop::lowest(d.count - count)
                } else if drop == "dl" {
                    Drop::lowest(count)
                } else if drop == "kl" {
                    Drop::highest(d.count - count)
                } else if drop == "dh" {
                    Drop::highest(count)
                } else {
                    unreachable!("Impossible drop!")
                };
                final_drop_groups.push(final_drop);
            }

            let first_drop = final_drop_groups.pop().expect("We checked.");

            // If only one drop we can add it and return the dicegroup.
            // else we continue.
            let second_drop = if let Some(drop) = final_drop_groups.pop() {
                drop
            }else{
                d.add_drop(first_drop);
                return Ok(dice);
            };

            let final_drop = match (first_drop,second_drop) {
                (Drop::Lowest(l),Drop::Highest(h))|(Drop::Highest(h),Drop::Lowest(l)) => {
                    let range = (l..(d.count-h)).collect::<Vec<usize>>();
                    Drop::custom(range)
                }
                _ => return Result::Err(
                    "Two identical drop clauses. Can't parse! Won't parse!".to_owned()
                ),
            };
            d.add_drop(final_drop);
            Ok(dice)
        }
        _ => Result::Err(
                "Incompatible drop groups found in dice group. Can't parse! Won't parse!".to_owned()
            ),
    }
}

fn parse_base_dice(base_group: String) -> Result<DiceGroup,String> {
    // If we have a singular number parse as a bonus.
    if !base_group.contains('d') {
        return match base_group.parse::<i64>() {
            Ok(n) => Ok(DiceGroup::bonus(n)),
            Err(e) => Result::Err(format!("Could not parse dice: {:?}",e)),
        };
    }

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
        '+'|'-'|'d'|'l'|'k'|'h' => true,
        c => c.is_numeric(),
    }
}
