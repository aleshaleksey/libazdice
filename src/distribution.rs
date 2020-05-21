#![allow(dead_code)]
extern crate rand;
use crate::distribution::rand::Rng;
use std::collections::BTreeMap;

#[derive(Debug,Clone,Copy,PartialEq)]
/// A range.
pub(crate) struct MinMax(pub [i64;2]);

#[derive(Debug,Clone,Copy,PartialEq)]
/// Represents the interactions of a Dicegroup with other groups.
pub(crate) enum DiceOp {
    Add,
    Sub,
    // Mul,
    // Div,
    // Other Ops to be added
}

impl DiceOp {
    /// Apply a diceop to a set of numbers to add to a total.
    pub(crate) fn operate(self, acc:i64, x:i64) -> i64 {
        match self {
            DiceOp::Add => acc + x,
            DiceOp::Sub => acc - x,
            // DiceOp::Mul => acc * x,
            // DiceOp::Div => acc / x,
        }
    }
}

/// An enum representing the drop clause on a set of dice duch as the "dl4" on "6d6dl4".
#[derive(Debug,Clone,PartialEq)]
pub(crate) enum Drop {
    Non,
    Highest(usize),
    Lowest(usize),
    Custom(Vec<usize>),
    // None, <- Why would we bother to roll if we don't want to Drop any?
}

/// An enum representing the minimal or maximal value that a die in a `DiceGroup` can have.
/// This is supposed to accomodate syntax like "8d6mn2" or "8d6mx2".
#[derive(Debug,Clone,Copy,PartialEq)]
pub(crate) enum CutOff {
    Non,
    Minimum(i64),
    Maximum(i64),
    Both(MinMax),
}

impl CutOff {
    /// Compare a rolled value to a cutoff and modify if appropriate.
    fn use_to_cut_off(&self, val: &mut i64) {
        match self {
            CutOff::Minimum(n) => if *val < *n {*val = *n; },
            CutOff::Maximum(n) => if *val > *n {*val = *n; },
            CutOff::Both(MinMax([mn, mx])) => {
                if *val > *mx {
                    *val = *mx;
                } else if *val < *mn {
                    *val = *mn
                }
            }
            _ => {}
        }
    }
}

/// Allow a clause for rerolling a certain number of dice if the result is above/below a value.
#[derive(Debug,Clone,Copy,PartialEq)]
pub(crate) struct ReRollType {
    /// Number of dice to reroll.
    pub(crate) count: usize,
    /// Exclusive condition for reroll `roll < x` or `roll > x`.
    pub(crate) ex_threshold: i64,
}

/// A condiitonal clause for rerolling dice.
/// This is supposed to accomodate syntax such as "6d6rr4be2".
/// "be" codes for below, "ab" codes for "above".
#[derive(Debug,Clone,Copy,PartialEq)]
pub(crate) enum ReRoll {
    Never,
    IfAbove(ReRollType),
    IfBelow(ReRollType),

}

impl ReRoll {
    /// Create a default instance of `ReRoll`.
    pub(crate) fn new() -> ReRoll {
        ReRoll::default()
    }

    fn default() -> ReRoll {
        ReRoll::Never
    }

    /// An instance of reroll if above a certain value.
    pub(crate) fn if_above(ex_threshold: i64, count: usize) -> ReRoll {
        let rrt = ReRollType { count, ex_threshold, };
        ReRoll::IfAbove(rrt)
    }

    /// An instance of reroll if below a certain value.
    pub(crate) fn if_below(ex_threshold: i64, count: usize) -> ReRoll {
        let rrt = ReRollType { count, ex_threshold, };
        ReRoll::IfBelow(rrt)
    }
}

impl Drop {
    /// Creates a new blank instance of `Drop`
    pub(crate) fn non() -> Drop {
        Drop::Non
    }

    /// Creates a "drop lowest" instance.
    pub(crate) fn highest(n: usize) -> Drop {
        Drop::Highest(n)
    }

    /// creates a "drop highest" instance.
    pub(crate) fn lowest(n: usize) -> Drop {
        Drop::Lowest(n)
    }

    /// Creates an instance where some dice are dropped.
    pub(crate) fn custom(v: Vec<usize>) -> Drop {
        Drop::Custom(v)
    }
}

// NB: This structure does not representa single die, but a dice-set of a single-sidedness.
#[derive(Debug,Clone,PartialEq)]
pub struct Dice {
    pub(crate) size: i64,
    pub(crate) count: usize,
    pub(crate) drop: Drop,
    pub(crate) reroll: ReRoll,
    pub(crate) cutoff: CutOff,
    pub(crate) op: DiceOp,
    pub(crate) explosive: bool,
}

impl Dice {
    /// A new instance of dice.
    pub(crate) fn new() -> Dice {
        Self::default()
    }

    /// A default instance.
    fn default() -> Dice {
        Dice {
            size: 6,
            count: 1,
            drop: Drop::Non,
            reroll: ReRoll::new(),
            cutoff: CutOff::Non,
            op: DiceOp::Add,
            explosive: false,
        }
    }

    pub(crate) fn with_size_and_count(size: i64, count: usize) -> Dice {
        Dice {
            size,
            count,
            drop: Drop::Non,
            reroll: ReRoll::new(),
            cutoff: CutOff::Non,
            op: DiceOp::Add,
            explosive: false,
        }
    }

    pub(crate) fn add_drop(&mut self, k: Drop) {
        self.drop = k;
    }

    pub(crate) fn add_reroll_if_above(&mut self, threshold: i64, count: usize) {
        self.reroll = ReRoll::if_above(threshold, count);
    }

    pub(crate) fn add_reroll_if_below(&mut self, threshold: i64, count: usize) {
        self.reroll = ReRoll::if_below(threshold, count);
    }

    /// Add a prechecked cutoff.
    pub(crate) fn add_checked_cutoff(&mut self, cutoff: CutOff) {
        self.cutoff = cutoff;
    }

    pub fn add_minimum_roll(&mut self, min: i64) -> Result<(), String> {
        if min > self.size { return Err("Minimum cutoff is bigger than dice sidedness!".to_owned()); }
        self.cutoff = CutOff::Minimum(min);
        Ok(())
    }

    pub fn add_maximum_roll(&mut self, max: i64) -> Result<(), String> {
        if max < 1 { return Err("Maximum cutoff is less than one!".to_owned()); }
        self.cutoff = CutOff::Minimum(max);
        Ok(())
    }

    pub(crate) fn add_op(&mut self, op: DiceOp) {
        self.op = op;
    }

    fn get_true_count(&self) -> usize {
        let sub = match self.drop {
            Drop::Lowest(n) => n,
            Drop::Highest(n) => n,
            Drop::Custom(ref v) => self.count - v.len(),
            Drop::Non => 0,
        };

        self.count - sub
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct Bonus {
    pub(crate) bonus: i64,
    pub(crate) op: DiceOp,
}

impl Bonus {
    /// Create default instance of a `Bonus`.
    pub(crate) fn new() -> Bonus {
        Self::default()
    }

    /// Create default instance of a `Bonus`.
    pub(crate) fn of(n:i64) -> Bonus {
        Bonus {
            bonus: n,
            op: DiceOp::Add,
        }
    }

    /// Default bonus is 0, default operation is addition.
     fn default() -> Bonus {
        Bonus {
            bonus: 0,
            op: DiceOp::Add,
        }
    }

    fn add_op(&mut self, op: DiceOp) {
        self.op = op;
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum DiceGroup {
    Bonus(Bonus),
    Dice(Dice),

}

impl DiceGroup {
    /// A new default instance of dice.
    pub fn new() -> DiceGroup {
        Self::default()
    }

    /// A default instance for dice.
    fn default() -> DiceGroup {
        let d = Dice::with_size_and_count(6,1);
        DiceGroup::Dice(d)
    }

    pub(crate) fn dice(size: i64, count: usize) -> DiceGroup {
        let d = Dice::with_size_and_count(size, count);
        DiceGroup::Dice(d)
    }

    pub(crate) fn bonus(n:i64) -> DiceGroup {
        let b = Bonus::of(n);
        DiceGroup::Bonus(b)
    }

    /// Insert a `DiceOp` into a dicegroup.
    pub(crate) fn add_op(&mut self, op: DiceOp) {
        match self {
            DiceGroup::Dice(ref mut d) => d.add_op(op),
            DiceGroup::Bonus(ref mut b) => b.add_op(op),
        }
    }

    /// Calculate the minmax for a single dice set.
    pub(crate) fn calculate_minmax(&self) -> MinMax {
        match self {
            &DiceGroup::Bonus(ref n) => MinMax([n.bonus,n.bonus]),
            &DiceGroup::Dice(ref d) => {
                let drop = match d.drop {
                    Drop::Highest(n)|Drop::Lowest(n) => n,
                    Drop::Custom(ref v) => v.len(),
                    _ => 0,
                };
                if drop>=d.count {
                    return MinMax([0,0]);
                }
                let f = (d.count - drop) as i64;
                let min_0 = f;
                let max_0 = d.size*f;

                if min_0 > max_0 {
                    MinMax([max_0,min_0])
                } else {
                    MinMax([min_0,max_0])
                }
            }
        }
    }

    /// Takes a minmax and performs an operation on a minmax.
    pub(crate) fn add_to_range(&self, min_max: &mut [i64;2]) {
        match self {
            DiceGroup::Bonus(n) => {
                let min_0 = n.op.operate(min_max[0], n.bonus);
                let max_0 = n.op.operate(min_max[1], n.bonus);

                if min_0 > max_0 {
                    // min is max.
                    min_max[0] = max_0;
                    min_max[1] = min_0;
                } else {
                    // max is max.
                    min_max[0] = min_0;
                    min_max[1] = max_0;
                }
            }
            DiceGroup::Dice(ref d) => {
                let true_count = d.get_true_count();
                let min_0 = d.op.operate(min_max[0], true_count as i64);
                let max_0 = d.op.operate(min_max[1], d.size * true_count as i64);

                if min_0 > max_0 {
                    // min is max.
                    min_max[0] = max_0;
                    min_max[1] = min_0;
                } else {
                    // max is max.
                    min_max[0] = min_0;
                    min_max[1] = max_0;
                }
            }
        }
    }

    /// Add whether explosive or not.
    pub(crate) fn is_explosive(&mut self, x: bool) {
        match self {
            DiceGroup::Dice(ref mut d) => d.explosive = x,
            _ => {},
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct DiceResult {
    dice: Dice,
    results: Vec<i64>,
    total: i64,
}

#[derive(Debug,Clone,PartialEq)]
pub struct BonusResult {
    boni: Vec<i64>,
    total: i64,
}

impl BonusResult {
    /// Create an empty total of boni.
    fn new() -> Self {
        BonusResult {
            boni: Vec::new(),
            total: 0,
        }
    }
}

impl DiceResult {
    /// NB, the total is calculated within the function.
    fn new(dice: &Dice, results: Vec<i64>) -> Self {
        let dice = dice.to_owned();
        let total = dice.op.operate(0, results.iter().fold(0,|acc,x| acc + x));
        DiceResult { dice, results, total, }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct RollResults {
    dice_groups: Vec<DiceResult>,
    bonus: BonusResult,
    total: i64,
}

impl RollResults {
    /// Make a new, empty instance of results.
    fn new_empty() -> Self {
        RollResults {
            dice_groups: Vec::new(),
            bonus: BonusResult::new(),
            total: 0,
        }
    }

    /// Add the roll result from the roll of a `Dice`.
    fn add_dice_result(&mut self, dice: DiceResult) {
        self.total+= dice.total;
        self.dice_groups.push(dice);
    }

    /// Add to bonus. NB: The +/- from `DiceOp` is calculated in the function.
    fn add_to_bonus(&mut self, b: &Bonus) {
        let Bonus {bonus, op, } = b;
        self.bonus.total+= op.operate(self.bonus.total, *bonus);
        self.bonus.boni.push(*bonus);
    }

    /// Get your own total.
    pub fn total(&self) -> i64 {
        self.total
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct DiceBag {
    pub(crate) dice: Vec<DiceGroup>,
    pub(crate) range: MinMax,
}

impl DiceBag {
    /// Create a distribution for a dice set.
    pub(crate) fn from_dice(dice: Vec<DiceGroup>) -> DiceBag {
        let mut dist = DiceBag {
            dice,
            range: MinMax([0,0]),
        };
        dist.calculate_range();
        dist
    }

    /// Calculates a range for a distribution.
    pub(crate) fn calculate_range(&mut self) {
        let range = self.dice.iter().fold([0;2],|mut acc,x| {
            x.add_to_range(&mut acc);
            acc
        });
        self.range = MinMax(range);
    }

    /// Roll the dicebag.
    pub fn roll(&self) -> RollResults {
        // NB: Will need serious reworking for multiplication and division.
        let mut final_result = RollResults::new_empty();
        for x in self.dice.iter() {
            match x {
                &DiceGroup::Bonus(ref b) => final_result.add_to_bonus(b),
                &DiceGroup::Dice(ref d) => {

                    let Dice { size, count, ref drop, ref reroll, ref cutoff, op: _, explosive } = d;
                    // Roll all the dice.
                    let mut answer = std::iter::repeat(0).take(*count).map(|_| {
                        let mut result = Vec::new();
                        if !explosive {
                            let roll = rand::thread_rng().gen_range(1,size+1);
                            result.push(roll);
                        }else{
                            explode(&mut result,* size);
                        }
                        result
                    }).flatten().collect::<Vec<_>>();

                    // Deal with the reroll clause.
                    let mut reroll_count = 0;
                    let mut answer_cycler = answer.iter_mut();
                    match reroll {
                        ReRoll::IfAbove(ReRollType{ count, ex_threshold }) => {
                            while let Some(ref mut roll) = answer_cycler.next() {
                                if **roll > *ex_threshold {
                                    **roll = rand::thread_rng().gen_range(1,size+1);
                                    reroll_count+= 1;
                                }
                                if reroll_count == *count { break; }
                            }
                        }
                        ReRoll::IfBelow(ReRollType{ count, ex_threshold }) => {
                            while let Some(ref mut roll) = answer_cycler.next() {
                                if **roll < *ex_threshold {
                                    **roll = rand::thread_rng().gen_range(1,size+1);
                                    reroll_count+= 1;
                                }
                                if reroll_count == *count { break; }
                            }
                        }
                        _ => {}
                    }

                    // Deal with the min-max clause:
                    for val in answer.iter_mut() {
                        cutoff.use_to_cut_off(val);
                    }

                    // Decide what to Drop.
                    let answer = match drop {
                        // On drop lowest, drop the lowest N dice. Custom sorting is needed.
                        Drop::Lowest(n) => {
                            answer.sort_by(|n1, n2| n2.cmp(&n1));
                            for _ in 0..*n { answer.pop(); }
                            answer
                        }
                        // On highest, drop the highest N dice.
                        Drop::Highest(n) => {
                            answer.sort();
                            for _ in 0..*n { answer.pop(); }
                            answer
                        }
                        // On custom, take selected dice and put in new vector.
                        Drop::Custom(v) => {
                            answer.sort();
                            let mut output = Vec::with_capacity(v.len());
                            for i in v.iter() { output.push(answer[*i]); }
                            output
                        }
                        _ => answer
                    };

                    final_result.add_dice_result(DiceResult::new(&d, answer));
                }
            }
        }
        final_result
    }

    /// A function to get a range as `[i64; 2]` (basically a minimum and maximum value).
    pub fn get_range(&self) -> [i64; 2] {
        let MinMax(range) = self.range;
        range
    }

    /// Get the range in a format which is useful.
    pub fn get_range_as_list(&self) -> Vec<i64> {
        let MinMax([min,max]) = self.range;
        (min..max).collect::<Vec<_>>()
    }

    /// Used for building frequency distributions from multiple rolls.
    pub fn get_range_as_btreemap(&self) -> BTreeMap<i64,usize> {
        let MinMax([min,max]) = self.range;
        (min..max).map(|i| (i,0)).collect::<BTreeMap<i64,usize>>()
    }


    /// Make a probability distribution by count.
    pub fn make_count_distribution(&self, roll_count:usize) -> BTreeMap<i64,usize> {
        let mut range = self.get_range_as_btreemap();
        for _ in 0..roll_count {
            let roll = self.roll();
            if let Some (c) = range.get_mut(&roll.total) {
                *c+= 1;
            }else{
                // This is excessive in this codebase, but just in case.
                range.insert(roll.total,1);
            }
        }
        range
    }

    /// Makes a probability distribution on the base of 0-100% percent.
    pub fn make_frequency_distribution(&self, roll_count:usize) -> BTreeMap<i64,f64> {
        self.make_count_distribution(roll_count).into_iter().map(|(i,c)| {
            (i,c as f64/roll_count as f64 * 100.0)
        }).collect::<BTreeMap<i64,f64>>()
    }
}

/// A function to make explosive dice explode
fn explode(vec: &mut Vec<i64>, max: i64) {
    let roll = rand::thread_rng().gen_range(1,max+1);
    vec.push(roll);
    if vec.last()==Some(&max) { explode(vec, max); }
}