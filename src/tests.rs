#![cfg(test)]
use super::distribution::*;
use super::parse;

#[test]
fn zero_test() {
    assert!(true)
}

#[cfg(test)]
fn single_parse_inner(size: i64) {
    let input = format!("d{}", size);

    let dice_bag = parse::parse(input).expect("should parse");

    assert_eq!(dice_bag.range, MinMax([1, size]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(Dice {
            size: size,
            count: 1,
            drop: Drop::Non,
            cutoff: CutOff::Non,
            reroll: ReRoll::Never,
            op: DiceOp::Add,
            explosive: false,
        })]
    );
}

#[cfg(test)]
fn simple_parse_inner(count: usize, size: i64) {
    let input = format!("{}d{}", count, size);

    let dice_bag = parse::parse(input).expect("should parse");

    assert_eq!(
        dice_bag.range,
        MinMax([1 * count as i64, count as i64 * size])
    );
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(Dice {
            size: size,
            count: count,
            drop: Drop::Non,
            cutoff: CutOff::Non,
            reroll: ReRoll::Never,
            op: DiceOp::Add,
            explosive: false,
        })]
    );
}

#[test]
fn parse_3d20dl1dh1() {
    let input = "3d20dl1dh1".to_owned();
    let input2 = "3d20kh2kl2".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag, dice_bag2);
    assert_eq!(dice_bag.range, MinMax([1, 20]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(Dice {
            size: 20,
            count: 3,
            drop: Drop::Custom(vec![1]),
            cutoff: CutOff::Non,
            reroll: ReRoll::Never,
            op: DiceOp::Add,
            explosive: false,
        })]
    );
}

#[test]
fn parse_5d20dl3() {
    let input = "5d20dl3".to_owned();
    let input2 = "5d20kh2".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag, dice_bag2);
    assert_eq!(dice_bag.range, MinMax([2, 40]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(Dice {
            size: 20,
            count: 5,
            drop: Drop::Lowest(3),
            cutoff: CutOff::Non,
            reroll: ReRoll::Never,
            op: DiceOp::Add,
            explosive: false,
        })]
    );
}

#[test]
fn parse_5d20dh3() {
    let input = "5d20dh3".to_owned();
    let input2 = "5d20kl2".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag, dice_bag2);
    assert_eq!(dice_bag.range, MinMax([2, 40]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(Dice {
            size: 20,
            count: 5,
            drop: Drop::Highest(3),
            cutoff: CutOff::Non,
            reroll: ReRoll::Never,
            op: DiceOp::Add,
            explosive: false,
        })]
    );
}

#[test]
fn parse_12d20dl4dh3() {
    let input = "12d20dl4dh3".to_owned();
    let input2 = "12d20kh8kl9".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag, dice_bag2);
    assert_eq!(dice_bag.range, MinMax([5, 100]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(Dice {
            size: 20,
            count: 12,
            drop: Drop::Custom(vec![4, 5, 6, 7, 8]),
            cutoff: CutOff::Non,
            reroll: ReRoll::Never,
            op: DiceOp::Add,
            explosive: false,
        })]
    );
}

#[test]
fn parse_15d20dl4dh3rr3be4() {
    let input = "15d20dl4dh3rr3be4".to_owned();
    let input2 = "15d20kh11kl12rr3be4".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag, dice_bag2);
    assert_eq!(dice_bag.range, MinMax([8, 160]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(Dice {
            size: 20,
            count: 15,
            drop: Drop::Custom(vec![4, 5, 6, 7, 8, 9, 10, 11]),
            cutoff: CutOff::Non,
            reroll: ReRoll::IfBelow(ReRollType {
                count: 3,
                ex_threshold: 4,
            }),
            op: DiceOp::Add,
            explosive: false,
        })]
    );
}

#[test]
fn parse_15d20dl4dh3rr3ab4mn2() {
    let input = "15d20dl4dh3rr3ab4mn2".to_owned();
    let input2 = "15d20kh11kl12rr3ab4mn2".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag, dice_bag2);
    assert_eq!(dice_bag.range, MinMax([16, 160]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(Dice {
            size: 20,
            count: 15,
            drop: Drop::Custom(vec![4, 5, 6, 7, 8, 9, 10, 11]),
            cutoff: CutOff::Minimum(2),
            reroll: ReRoll::IfAbove(ReRollType {
                count: 3,
                ex_threshold: 4,
            }),
            op: DiceOp::Add,
            explosive: false,
        })]
    );
}

#[test]
fn parse_15d20dl4dh3rr3ab4mn2mx18() {
    let input = "15d20dl4dh3rr3ab4mn2mx18".to_owned();
    let input2 = "15d20kh11kl12rr3ab4mn2mx18".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag, dice_bag2);
    assert_eq!(dice_bag.range, MinMax([16, 144]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(Dice {
            size: 20,
            count: 15,
            drop: Drop::Custom(vec![4, 5, 6, 7, 8, 9, 10, 11]),
            cutoff: CutOff::Both(MinMax([2, 18])),
            reroll: ReRoll::IfAbove(ReRollType {
                count: 3,
                ex_threshold: 4,
            }),
            op: DiceOp::Add,
            explosive: false,
        })]
    );
}

#[test]
fn parse_15d20dl4dh3rr3ab4mn15mx5() {
    let input = "15d20dl4dh3rr3ab4mn15mx5".to_owned();
    let input2 = "15d20kh11kl12rr3ab4mn15mx5".to_owned();

    let dice_bag = parse::parse(input);
    let dice_bag2 = parse::parse(input2);

    assert_eq!(dice_bag, dice_bag2);
    assert!(dice_bag.is_err());
}

#[test]
fn parse_15d20dl4dh3rr3ab4mn2_explosive() {
    let input = "15d20dl4dh3rr3ab4mn2!".to_owned();
    let input2 = "15d20kh11kl12rr3ab4mn2!".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");
    let dice_bag2 = parse::parse(input2).expect("should parse");

    assert_eq!(dice_bag, dice_bag2);
    assert_eq!(dice_bag.range, MinMax([16, 160]));
    assert_eq!(
        dice_bag.dice,
        vec![DiceGroup::Dice(Dice {
            size: 20,
            count: 15,
            drop: Drop::Custom(vec![4, 5, 6, 7, 8, 9, 10, 11]),
            cutoff: CutOff::Minimum(2),
            reroll: ReRoll::IfAbove(ReRollType {
                count: 3,
                ex_threshold: 4,
            }),
            op: DiceOp::Add,
            explosive: true,
        })]
    );
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

    assert_eq!(dice_bag.range, MinMax([18, 172]));
    assert_eq!(
        dice_bag.dice,
        vec![
            DiceGroup::Dice(Dice {
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
        ]
    );
}

#[test]
fn parse_7d23_explosive_plus_11() {
    let input = "7d23! + 11".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");

    assert_eq!(dice_bag.range, MinMax([18, 172]));
    assert_eq!(
        dice_bag.dice,
        vec![
            DiceGroup::Dice(Dice {
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
        ]
    );
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

    assert_eq!(dice_bag.range, MinMax([-4, 150]));
    assert_eq!(
        dice_bag.dice,
        vec![
            DiceGroup::Dice(Dice {
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
        ]
    );
}

#[test]
fn parse_10d6dl11_fail() {
    let input = "10d6dl11".to_owned();

    let dice_bag = parse::parse(input);
    println!("Failed dice_bag: {:?}", dice_bag);
    assert!(dice_bag.is_err());
}

#[test]
fn parse_10d6kl11_fail() {
    let input = "10d6kl11".to_owned();

    let dice_bag = parse::parse(input);
    println!("Failed dice_bag: {:?}", dice_bag);
    assert!(dice_bag.is_err());
}

#[test]
fn parse_5d6_minus_10d10() {
    let input = "5d6 - 10d10".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");

    assert_eq!(dice_bag.range, MinMax([-70, -5]));
    assert_eq!(
        dice_bag.dice,
        vec![
            DiceGroup::Dice(Dice {
                size: 6,
                count: 5,
                drop: Drop::Non,
                cutoff: CutOff::Non,
                reroll: ReRoll::Never,
                op: DiceOp::Add,
                explosive: false,
            }),
            DiceGroup::Dice(Dice {
                size: 10,
                count: 10,
                drop: Drop::Non,
                cutoff: CutOff::Non,
                reroll: ReRoll::Never,
                op: DiceOp::Sub,
                explosive: false,
            }),
        ]
    );
}

#[test]
fn parse_5d6_minus_10d10_explosive() {
    let input = "5d6 - 10d10!".to_owned();

    let dice_bag = parse::parse(input).expect("should parse");

    assert_eq!(dice_bag.range, MinMax([-70, -5]));
    assert_eq!(
        dice_bag.dice,
        vec![
            DiceGroup::Dice(Dice {
                size: 6,
                count: 5,
                drop: Drop::Non,
                cutoff: CutOff::Non,
                reroll: ReRoll::Never,
                op: DiceOp::Add,
                explosive: false,
            }),
            DiceGroup::Dice(Dice {
                size: 10,
                count: 10,
                drop: Drop::Non,
                cutoff: CutOff::Non,
                reroll: ReRoll::Never,
                op: DiceOp::Sub,
                explosive: true,
            }),
        ]
    );
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
    simple_parse_inner(1, 6)
}

#[test]
fn parse_1d8_test() {
    simple_parse_inner(1, 8)
}

#[test]
fn parse_1d10_test() {
    simple_parse_inner(1, 10)
}

#[test]
fn parse_1d12_test() {
    simple_parse_inner(1, 12)
}

#[test]
fn parse_1d20_test() {
    simple_parse_inner(1, 20)
}

#[test]
fn parse_1d100_test() {
    simple_parse_inner(1, 100)
}

#[test]
fn parse_5d20_test() {
    simple_parse_inner(5, 20)
}

#[test]
fn parse_345d4653_test() {
    simple_parse_inner(345, 4653)
}

#[test]
/// This test is meant to test the randomness and unbiasedness of the dice.
/// NB, this is not a statistically rigorous test (yet!).
/// The mean of 1d10 is of course 5.5. The error margin should be small.
/// Therefore occasionally this test might fail at random.
fn test_distribution_average_1d10() {
    let one_d_ten = super::parse("1d10".to_owned()).unwrap();

    let mean = one_d_ten
        .make_count_distribution(50_000_000)
        .into_iter()
        .fold(0, |acc, (val, n)| acc + val * n as i64) as f64
        / 50_000_000.0;

    assert!((mean > 5.499) && (mean < 5.501));
}

#[test]
/// This test is meant to test the randomness and unbiasedness of the dice.
/// NB, this is not a statistically rigorous test (yet!).
/// The frequency for each value should be 10%, the error margin should be small.
/// Therefore occasionally this test might fail at random.
fn test_distribution_frequency() {
    let one_d_ten = super::parse("1d10".to_owned()).unwrap();

    let distribution = one_d_ten
        .make_frequency_distribution(50_000_000)
        .into_iter();

    for (_val, f) in distribution {
        assert!((f > 9.99) && (f < 10.01));
    }
}

#[test]
/// Even if a distribution is "uniform" theoretically, it may be irregular in general. However,
/// there may be clustering (for example the probability of getting three 20s in a row from 1d20)
/// rolls is 1/8000, so in a non-clustered distribution it shoule be approximately of the correct
/// value.
fn test_random_clustering_d_ten() {
    let one_d_ten = super::parse("1d10".to_owned()).unwrap();

    let mut chains = Vec::with_capacity(10_000_000);

    let mut last_roll = 0;
    let mut last_chain = 0;
    for _ in 0..20_000_000 {
        let roll = one_d_ten.roll().total();
        if roll == last_roll {
            last_chain += 1;
        } else {
            chains.push(last_chain);
            last_chain = 0;
        }
        last_roll = roll;
    }

    // NB 0 means 1 in a row. 1 means 2 in a row, 1 means 3 in a row and so on.
    let zero = chains.iter().filter(|x| **x == 0).count() as f64 / chains.len() as f64 * 100.0;
    let zero = (zero > 89.95) && (zero < 90.05);

    let one = chains.iter().filter(|x| **x >= 1).count() as f64 / chains.len() as f64 * 100.0;
    let one = (one > 9.8) && (one < 10.2);

    let two = chains.iter().filter(|x| **x >= 2).count() as f64 / chains.len() as f64 * 100.0;
    let two = (two > 0.98) && (two < 1.02);

    // Statistics is weaker for small numbers.
    let three = chains.iter().filter(|x| **x >= 3).count() as f64 / chains.len() as f64 * 100.0;
    let three = (three > 0.096) && (three < 0.104);
    assert!(zero && one && two && three);
}
