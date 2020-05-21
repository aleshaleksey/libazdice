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
                cutoff: CutOff::Non,
                reroll: ReRoll::Never,
                op: DiceOp::Add,
                explosive: false,
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
                cutoff: CutOff::Non,
                reroll: ReRoll::Never,
                op: DiceOp::Add,
                explosive: false,
            }
    )]);
}

#[test]
fn parse_3d20dl1dh1() {
    let input = "3d20dl1dh1".to_owned();
    let input2 = "3d20kh2kl2".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag,dice_bag2);
    assert_eq!(dice_bag.range,MinMax([1,20]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(
            Dice {
                size: 20,
                count: 3,
                drop: Drop::Custom(vec![1]),
                cutoff: CutOff::Non,
                reroll: ReRoll::Never,
                op: DiceOp::Add,
                explosive: false,
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
                cutoff: CutOff::Non,
                reroll: ReRoll::Never,
                op: DiceOp::Add,
                explosive: false,
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
                cutoff: CutOff::Non,
                reroll: ReRoll::Never,
                op: DiceOp::Add,
                explosive: false,
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
                drop: Drop::Custom(vec![4,5,6,7,8]),
                cutoff: CutOff::Non,
                reroll: ReRoll::Never,
                op: DiceOp::Add,
                explosive: false,
            }
    )]);
}

#[test]
fn parse_15d20dl4dh3rr3be4() {
    let input = "15d20dl4dh3rr3be4".to_owned();
    let input2 = "15d20kh11kl12rr3be4".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag,dice_bag2);
    assert_eq!(dice_bag.range,MinMax([8,160]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(
            Dice {
                size: 20,
                count: 15,
                drop: Drop::Custom(vec![4,5,6,7,8,9,10,11]),
                cutoff: CutOff::Non,
                reroll: ReRoll::IfBelow(ReRollType{ count: 3, ex_threshold: 4, }),
                op: DiceOp::Add,
                explosive: false,
            }
    )]);
}

#[test]
fn parse_15d20dl4dh3rr3ab4mn2() {
    let input = "15d20dl4dh3rr3ab4mn2".to_owned();
    let input2 = "15d20kh11kl12rr3ab4mn2".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag,dice_bag2);
    assert_eq!(dice_bag.range,MinMax([8,160]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(
            Dice {
                size: 20,
                count: 15,
                drop: Drop::Custom(vec![4,5,6,7,8,9,10,11]),
                cutoff: CutOff::Minimum(2),
                reroll: ReRoll::IfAbove(ReRollType{ count: 3, ex_threshold: 4, }),
                op: DiceOp::Add,
                explosive: false,
            }
    )]);
}

#[test]
fn parse_15d20dl4dh3rr3ab4mn2mx18() {
    let input = "15d20dl4dh3rr3ab4mn2mx18".to_owned();
    let input2 = "15d20kh11kl12rr3ab4mn2mx18".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag,dice_bag2);
    assert_eq!(dice_bag.range,MinMax([8,160]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(
            Dice {
                size: 20,
                count: 15,
                drop: Drop::Custom(vec![4,5,6,7,8,9,10,11]),
                cutoff: CutOff::Both(MinMax([2,18])),
                reroll: ReRoll::IfAbove(ReRollType{ count: 3, ex_threshold: 4, }),
                op: DiceOp::Add,
                explosive: false,
            }
    )]);
}

#[test]
fn parse_15d20dl4dh3rr3ab4mn15mx5() {
    let input = "15d20dl4dh3rr3ab4mn15mx5".to_owned();
    let input2 = "15d20kh11kl12rr3ab4mn15mx5".to_owned();

    let dice_bag = parse::parse(input);
    let dice_bag2 = parse::parse(input2);

    assert_eq!(dice_bag,dice_bag2);
    assert!(dice_bag.is_err());
}

#[test]
fn parse_15d20dl4dh3rr3ab4mn2_explosive() {
    let input = "15d20dl4dh3rr3ab4mn2!".to_owned();
    let input2 = "15d20kh11kl12rr3ab4mn2!".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag,dice_bag2);
    assert_eq!(dice_bag.range,MinMax([8,160]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(
            Dice {
                size: 20,
                count: 15,
                drop: Drop::Custom(vec![4,5,6,7,8,9,10,11]),
                cutoff: CutOff::Minimum(2),
                reroll: ReRoll::IfAbove(ReRollType{ count: 3, ex_threshold: 4, }),
                op: DiceOp::Add,
                explosive: true,
            }
    )]);
}

#[test]
fn parse_15d20dl4dh3rr3ab4mn2_explosive_fail() {
    let input = "15d20dl4dh3rr3ab4!mn2".to_owned();
    let input2 = "15d20kh11kl12rr3ab4!mn2".to_owned();

    let dice_bag = parse::parse(input);
    let dice_bag2 = parse::parse(input2);

    assert_eq!(dice_bag, dice_bag2);
    assert!(dice_bag.is_err());
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
                cutoff: CutOff::Non,
                reroll: ReRoll::Never,
                op: DiceOp::Add,
                explosive: false,
            }),
            DiceGroup::Bonus(Bonus {
                bonus: 11,
                op: DiceOp::Add,
            }),
    ]);
}

#[test]
fn parse_7d23_explosive_plus_11() {
    let input = "7d23! + 11".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");

    assert_eq!(dice_bag.range,MinMax([18,172]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(
            Dice {
                size: 23,
                count: 7,
                drop: Drop::Non,
                cutoff: CutOff::Non,
                reroll: ReRoll::Never,
                op: DiceOp::Add,
                explosive: true,
            }),
            DiceGroup::Bonus(Bonus {
                bonus: 11,
                op: DiceOp::Add,
            }),
    ]);
}

#[test]
fn parse_7d23_explosive_plus_11_fail() {
    let input = "!7d23 + 11".to_owned();

    let dice_bag = parse::parse(input);

    assert!(dice_bag.is_err());
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
                cutoff: CutOff::Non,
                reroll: ReRoll::Never,
                op: DiceOp::Add,
                explosive: false,
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
                    cutoff: CutOff::Non,
                    reroll: ReRoll::Never,
                    op: DiceOp::Add,
                    explosive: false,
                }),
            DiceGroup::Dice(
                Dice {
                    size: 10,
                    count: 10,
                    drop: Drop::Non,
                    cutoff: CutOff::Non,
                    reroll: ReRoll::Never,
                    op: DiceOp::Sub,
                    explosive: false,
                }),
    ]);
}

#[test]
fn parse_5d6_minus_10d10_explosive() {
    let input = "5d6 - 10d10!".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");

    assert_eq!(dice_bag.range,MinMax([-70,-5]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(
                Dice {
                    size: 6,
                    count: 5,
                    drop: Drop::Non,
                    cutoff: CutOff::Non,
                    reroll: ReRoll::Never,
                    op: DiceOp::Add,
                    explosive: false,
                }),
            DiceGroup::Dice(
                Dice {
                    size: 10,
                    count: 10,
                    drop: Drop::Non,
                    cutoff: CutOff::Non,
                    reroll: ReRoll::Never,
                    op: DiceOp::Sub,
                    explosive: true,
                }),
    ]);
}

#[test]
fn parse_5d6_minus_10d10_explosive_fail() {
    let input = "5d6 - 10!d10".to_owned();

    assert!(parse::parse(input).is_err());
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
