# libazdice[<img src="https://api.travis-ci.org/aleshaleksey/AZDice.svg?branch=master">](https://travis-ci.org/aleshaleksey/AZDice)
A fairly light-weight dice rolling library for use in "AZDice" and similar dice rollers and distribution simulators.

This document is a placeholder. It will be updated and clarified at the earliest opportunity.
This library is meant to be the new dice "engine" for AZDice once it is "finished".

Current status: Builds with `cargo 1.43.0 (2cbe9048e 2020-05-03)`


__Intended Dice Features:__

-Parser for dice strings eg. "1d100+5d20dl3-4" (Mostly done).

-API for creating "DiceBag"s via functions. (Needs to be externalised).

-API for rolling a "DiceBag" once or more, or creating a probability distribution.


__Intended Parsing Features:__

Ability to parse and roll:

-Accommodate for any virtual dice size with a whole number of size.

-Basic rolls such as "3d20+5" with and without white-spaces. (Roll 3 twenty-sided dice and get the total)

-Compound rolls such as "3d20-20d4". (Roll three twenty-sided dice and then subtract the total of the roll of twenty four-sided dice.)

-Drop rolls such as "5d6dl2" or "2d20dh1". (Roll five six-sided dice and drop the two lowest, or roll two twenty-sided dice and drop the highest.)

-Re-roll rolls which fall above or below a certain value such as "5d6r2b3". (Roll five six-sided dice re-roll up to two dice which roll below three). (TODO)


__Intended Rolling functionality:__

Accomodate:

-Single rolls of a DiceBag.

-Multiple rolls of a DiceBag.

-Building of a probability distribution from multiple rolls of a DiceBag.


__Maybe Intended Features:__

Ability to parse "explosive" dice such as "1d20!" (on a twenty, roll an extra dice).


__TODO__

-Ability to re-roll above or below a given value.

-"Explosive" dice.

-Sensible API for use.
