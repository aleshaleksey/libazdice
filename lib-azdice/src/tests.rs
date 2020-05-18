#![cfg(test)]
use super::distribution::*;
use super::parse;

#[test]
fn zero_test() {
    assert!(true)
}

#[cfg(test)]
fn single_parse_inner(size:i64) {
    let input = format!("d{}",size);

    let dice_bag = parse::parse(input).expect("should parse");

    assert_eq!(dice_bag.range,MinMax([1,size]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(
            Dice {
                size: size,
                count: 1,
                drop: Drop::Non,
                op: DiceOp::Add,
            }
    )]);
}

#[cfg(test)]
fn simple_parse_inner(count:usize,size:i64) {
    let input = format!("{}d{}",count,size);

    let dice_bag = parse::parse(input).expect("should parse");

    assert_eq!(dice_bag.range,MinMax([1*count as i64,count as i64*size]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(
            Dice {
                size: size,
                count: count,
                drop: Drop::Non,
                op: DiceOp::Add,
            }
    )]);
}

#[test]
fn parse_5d20dl3() {
    let input = "5d20dl3".to_owned();
    let input2 = "5d20kh2".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag,dice_bag2);
    assert_eq!(dice_bag.range,MinMax([2,40]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(
            Dice {
                size: 20,
                count: 5,
                drop: Drop::Lowest(3),
                op: DiceOp::Add,
            }
    )]);
}

#[test]
fn parse_5d20dh3() {
    let input = "5d20dh3".to_owned();
    let input2 = "5d20kl2".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag,dice_bag2);
    assert_eq!(dice_bag.range,MinMax([2,40]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(
            Dice {
                size: 20,
                count: 5,
                drop: Drop::Highest(3),
                op: DiceOp::Add,
            }
    )]);
}

#[test]
fn parse_12d20dl4dh3() {
    let input = "12d20dl4dh3".to_owned();
    let input2 = "12d20kh8kl9".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag,dice_bag2);
    assert_eq!(dice_bag.range,MinMax([5,100]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(
            Dice {
                size: 20,
                count: 12,
                drop: Drop::Custom(vec![3,4,5,6,7]),
                op: DiceOp::Add,
            }
    )]);
}

#[test]
fn parse_7d23_plus_11() {
    let input = "7d23 + 11".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");

    assert_eq!(dice_bag.range,MinMax([18,172]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(
            Dice {
                size: 23,
                count: 7,
                drop: Drop::Non,
                op: DiceOp::Add,
            }),
            DiceGroup::Bonus(Bonus {
                bonus: 11,
                op: DiceOp::Add,
            }),
    ]);
}

#[test]
fn parse_7d23_minus_11() {
    let input = "7d23 - 11".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");

    assert_eq!(dice_bag.range,MinMax([-4,150]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(
            Dice {
                size: 23,
                count: 7,
                drop: Drop::Non,
                op: DiceOp::Add,
            }),
            DiceGroup::Bonus(Bonus {
                bonus: 11,
                op: DiceOp::Sub,
            }),
    ]);
}

#[test]
fn parse_10d6dl11_fail() {
    let input = "10d6dl11".to_owned();

    let dice_bag = parse::parse(input);
    println!("Failed dice_bag: {:?}",dice_bag);
    assert!(dice_bag.is_err());
}

#[test]
fn parse_10d6kl11_fail() {
    let input = "10d6kl11".to_owned();

    let dice_bag = parse::parse(input);
    println!("Failed dice_bag: {:?}",dice_bag);
    assert!(dice_bag.is_err());
}

#[test]
fn parse_5d6_minus_10d10() {
    let input = "5d6 - 10d10".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");

    assert_eq!(dice_bag.range,MinMax([-70,-5]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(
                Dice {
                    size: 6,
                    count: 5,
                    drop: Drop::Non,
                    op: DiceOp::Add,
                }),
            DiceGroup::Dice(
                Dice {
                    size: 10,
                    count: 10,
                    drop: Drop::Non,
                    op: DiceOp::Sub,
                }),
    ]);
}

#[test]
fn parse_1d6_test_no_prefix() {
    single_parse_inner(6)
}

#[test]
fn parse_1d8_test_no_prefix() {
    single_parse_inner(8)
}

#[test]
fn parse_1d10_test_no_prefix() {
    single_parse_inner(10)
}

#[test]
fn parse_1d12_test_no_prefix() {
    single_parse_inner(12)
}

#[test]
fn parse_1d20_test_no_prefix() {
    single_parse_inner(20)
}

#[test]
fn parse_1d100_test_no_prefix() {
    single_parse_inner(100)
}

#[test]
fn parse_1d6_test() {
    simple_parse_inner(1,6)
}

#[test]
fn parse_1d8_test() {
    simple_parse_inner(1,8)
}

#[test]
fn parse_1d10_test() {
    simple_parse_inner(1,10)
}

#[test]
fn parse_1d12_test() {
    simple_parse_inner(1,12)
}

#[test]
fn parse_1d20_test() {
    simple_parse_inner(1,20)
}

#[test]
fn parse_1d100_test() {
    simple_parse_inner(1,100)
}

#[test]
fn parse_5d20_test() {
    simple_parse_inner(5,20)
}

#[test]
fn parse_345d4653_test() {
    simple_parse_inner(345,4653)
}
