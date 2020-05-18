#![allow(dead_code)]
extern crate rand;
use crate::distribution::rand::Rng;
use std::collections::BTreeMap;

#[derive(Debug,Clone,PartialEq)]
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

/// A structure represenuse crate::distribution::rand::Rng;ting dice.
#[derive(Debug,Clone,PartialEq)]
pub(crate) enum Drop {
    Non,
    Highest(usize),
    Lowest(usize),
    Custom(Vec<usize>),
    // None, <- Why would we bother to roll if we don't want to Drop any?
}

impl Drop {
    /// Creates a new blank instance of `Drop`
    pub(crate) fn all() -> Drop {
        Drop::Non
    }

    /// Creates a "drop lowest" instance.
    pub(crate) fn highest(n:usize) -> Drop {
        Drop::Highest(n)
    }

    /// creates a "drop highest" instance.
    pub(crate) fn lowest(n:usize) -> Drop {
        Drop::Lowest(n)
    }

    /// Creates an instance where some dice are dropped.
    pub(crate) fn custom(v:Vec<usize>) -> Drop {
        Drop::Custom(v)
    }
}

// NB: This structure does not representa single die, but a dice-set of a single-sidedness.
#[derive(Debug,Clone,PartialEq)]
pub struct Dice {
    pub(crate) size: i64,
    pub(crate) count: usize,
    pub(crate) drop: Drop,
    pub(crate) op: DiceOp,
}

impl Dice {
    /// A new instance of dice.
    pub(crate) fn new() -> Dice {
        Self::default()
    }

    /// A default instance.
    pub(crate) fn default() -> Dice {
        Dice {
            size: 6,
            count: 1,
            drop: Drop::Non,
            op: DiceOp::Add,
        }
    }

    pub(crate) fn with_size_and_count(size:i64,count:usize) -> Dice {
        Dice {
            size,
            count,
            drop: Drop::Non,
            op: DiceOp::Add,
        }
    }

    pub(crate) fn add_size(&mut self, n:i64) {
        self.size = n;
    }

    pub(crate) fn add_count(&mut self, n:usize) {
        self.count = n;
    }

    pub(crate) fn add_drop(&mut self, k:Drop) {
        self.drop = k;
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
    pub(crate) fn default() -> Bonus {
        Bonus {
            bonus: 0,
            op: DiceOp::Add,
        }
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
    pub(crate) fn default() -> DiceGroup {
        let d = Dice {
            size: 6,
            count: 1,
            drop: Drop::Non,
            op: DiceOp::Add,
        };
        DiceGroup::Dice(d)
    }

    pub(crate) fn dice(size:i64, count: usize) -> DiceGroup {
        let d = Dice::with_size_and_count(size,count);
        DiceGroup::Dice(d)
    }

    pub(crate) fn bonus(n:i64) -> DiceGroup {
        let b = Bonus::of(n);
        DiceGroup::Bonus(b)
    }

    /// Insert a `DiceOp` into a dicegroup.
    pub(crate) fn add_op(&mut self, op:DiceOp) {
        match self {
            DiceGroup::Dice(ref mut d) => d.op = op,
            DiceGroup::Bonus(ref mut b) => b.op = op,
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
    pub(crate) fn add_to_range(&self,min_max: &mut [i64;2]) {
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
}

#[derive(Debug,Clone,PartialEq)]
pub struct DiceBag {
    pub(crate) dice: Vec<DiceGroup>,
    pub(crate) range: MinMax,
}

impl DiceBag {
    /// Create a distribution for a dice set.
    pub(crate) fn from_dice(dice:Vec<DiceGroup>) -> DiceBag {
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
    pub fn roll(&self) -> i64 {
        // NB: Will need serious reworking for multiplication and division.
        self.dice.iter().fold(0,|acc,x| {
            match x {
                &DiceGroup::Bonus(Bonus{bonus,op}) => op.operate(acc, bonus),
                &DiceGroup::Dice(Dice{size, count, ref drop, op}) => {
                    // Roll all the dice.
                    let mut answer = std::iter::repeat(0).take(count).map(|_| {
                        rand::thread_rng().gen_range(1,size+1)
                    }).collect::<Vec<_>>();
                    answer.sort();

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

                    // Spit out an answer.
                    op.operate(acc, answer.iter().fold(0,|acc,x| acc + x))
                }
            }
        })
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
    pub fn make_count_distribution(&self,roll_count:usize) -> BTreeMap<i64,usize> {
        let mut range = self.get_range_as_btreemap();
        for _ in 0..roll_count {
            let roll = self.roll();
            if let Some (c) = range.get_mut(&roll) {
                *c+= 1;
            }else{
                // This is excessive in this codebase, but just in case.
                range.insert(roll,1);
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