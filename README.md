# libazdice[<img src="https://api.travis-ci.org/aleshaleksey/libazdice.svg?branch=master">](https://travis-ci.org/aleshaleksey/libazdice)
A fairly light-weight dice rolling library for use in "AZDice" and similar dice rollers and distribution simulators.

This document is a placeholder. It will be updated and clarified at the earliest opportunity.
This library is meant to be the new dice "engine" for AZDice once it is "finished".

Current status: Builds with `cargo 1.43.0 (2cbe9048e 2020-05-03)`


__Features:__

- Parser for dice strings eg. "1d100+5d20dl3-4" (Done).

- API for creating "DiceBag"s via functions.

- API for rolling a "DiceBag" once or more, or creating a probability distribution.

- Simple C/C++ API for parsing, creating and rolling a "DiceBag".


__Current Parsing Features:__

Ability to parse and roll:

- Accommodate for any virtual dice size with a whole number of size.

- Basic rolls such as "3d20+5" with and without white-spaces. (Roll 3 twenty-sided dice and get the total)

- Compound rolls such as "3d20-20d4". (Roll three twenty-sided dice and then subtract the total of the roll of twenty four-sided dice.)

- Drop rolls such as "5d6dl2" or "2d20dh1". (Roll five six-sided dice and drop the two lowest, or roll two twenty-sided dice and drop the highest.)

- Re-roll rolls which fall above or below a certain value such as "5d6r2b3". (Roll five six-sided dice re-roll up to two dice which roll below three).

- Ability to re-roll above or below a given value. (Once per dice for up to N dice, where N < number of dice.)

- "Explosive" dice. (Roll an extra dice on a max. so if 1d20 -> 20, a second d20 is rolled, recursively).

__Currently Supported Parsing and Functions__

The parser is designed to support most dice roll types that are used by various Table Top Role Playing Games (TTRPGs), as well as their break-downs and generation of their distributions.

Currently the parser can parse an arbitrary number of dice rolls of different dice types and numbers (although at `i64::MAX`) an overflow will be reached, it is not expected that most TTRPG dice rolls will reach near complex enough to "break" it.

The parser supports many elements of typical dice-roller syntax, but others are improvised.

Example use (API may yet be improved):
```rust
extern crate libazdice;
use libazdice::parse;
use libazdice::distribution;

let dice_to_be: String = "3d20dl1dh1+6".to_owned();
let parsed_dice_bag: distribution::DiceBag = parse::parse(parsed_dice_bag.clone())?;
let roll_summary = parsed_dice_bag.roll();
println!("I have rolled {} and got a {}!", dice_to_be, roll_summary.total());
```

**Supported groups:**
(NB 'N', 'M' etc, is used to represent numerical values such as 1, 2, ... etc.)
(NB2: The parser is case and whitespace insentive.)

**Main Groups**
One main dice group is supported per dice group. It must come before any auxillary groups.

--`'N'` : A single number on its own (usually in the format of `1d20 + 3` or `1d20 - 3`, is parsed as a number.

--`d'N'` : Represents a single N-sided dice (eg `d6`). One per dice group is supported. Supports auxilary groups.

--`'M'd'N'` : Represents M N-sided dice (eg `6d8`). One per dice group is supported. Supports auxillary groups

**Auxillary Groups.**
Auxillary groups can come in any order after the main group.

--`rr'N'` : Represents the reroll of N dice in the dice group. (eg `10d4rr5`). Requires a `ab'N'` or `bl'N'` group. One per dice group is allowed (eg `10d4rr5bl2` (more common) or `10d4rr2ab3` (uncommon).

--`ab'N'` : The threshold above which a reroll is to be triggered if `rr` is found (see above, eg `ab5`). One per dice group is supported. 

--`bl'N'` : The threshold below which a reroll is to be triggered if `rr` is found (see above, eg `bl3`). One per dice group is supported.

--`mn'N'` : Sets the floor of a dice roll. (eg `8d10mn3` will set any roll below 3 to a value of 3). Can be used in conjuction with `mx`. One per dice group is supported.

--`mx'N'` : Sets the ceiling of a dice roll. (eg `8d10mx7` will set any roll above 7 to a value of 7). Can be used in conjuction with `mn`. One per dice group is supported.

--`dl'N'` / `kh'M'` : Drops N of the lowest dice, or keeps (total number - M). One per dice group is supported. Can be used in conjunction with `dh` / `kl`.

--`dh'N'` / `kl'M'` : Drops N of the highest dice, or keeps (total number -M). One per dice group is supported. Can be used in conjuction with `dl` / `kh`.

(NB3: "drop" and "keep" clauses should in theory be handled differently when used in conjunction (eg if you have a group of dice with 15 dice and you want to keep the 10 lowest *and* the 10 highest, then in theory you need to keep all of them, but this library handles it the same as drop clauses). 

**Final group**
This group must be used at the end of a dice group, or an error will be triggered.
--`!` : The dice group is explosive. Only one is supported per dice group and it must be the last character. (eg `4d6rr2be2!`).

**Examples**
"5" : 5.
"2d10 + 5" : Roll 2 ten-sided dice and add 5.
"5 - 2d10" : Take a 5, and subtract the rolls of two ten-sided dice.
"10d8dl1dh2rr4be3!" - 1d100 : Roll ten eight-sided dice explosively, reroll the lowest four dice below three, drop the lowest roll, drop two of the hightest rolls.

__Intended Rolling functionality:__

Accommodate:

- Single rolls of a DiceBag.

- Multiple rolls of a DiceBag.

- Building of a probability distribution from multiple rolls of a DiceBag.

- A basic api to extern for use with C or C++. (NB! Destructors needed!)



__TODO__

- Write destructors for C/C++ api functions.

- Implementation of `Display` for the pub types.

~~- Improve documentation.~~

~~- Find a better source of randomness!~~

__Other__
The testing ~~is currently incomplete~~ could be more extensive, so if someone finds a bug, or thinks of an essential dice roller feature that's been overlooked, please make a comment (or something)!

__Compiling__

Can be compiled sensibly with `cargo build --release` for both `rlib` and `cdylib` targets. A "small" (read: stripped) version can be compiled with `cargo small-release` (not recommended, unless it is).
