use std::fmt::Display;
use std::mem::size_of;

use num_traits::PrimInt;
use num_traits::Unsigned;

use Bit::*;
use TapeMotion::*;

fn as_bits<T: PrimInt>(x: T) -> String {
    (0..8 * size_of::<T>())
        .rev()
        .map(|i| {
            // These are guaranteed to be either 1 or 0 so no need for double-checking
            if (x >> i) & T::one() == T::one() {
                '1'
            } else {
                '0'
            }
        })
        .collect()
}

fn as_bits_rev<T: PrimInt>(x: T) -> String {
    (0..8 * size_of::<T>())
        .map(|i| {
            // These are guaranteed to be either 1 or 0 so no need for double-checking
            if (x >> i) & T::one() == T::one() {
                '1'
            } else {
                '0'
            }
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Bit {
    Zero,
    One,
}

fn get_bit<T: PrimInt>(x: T, pos: usize) -> Bit {
    if x & (T::one() << pos) == T::zero() {
        Zero
    } else {
        One
    }
}

fn set_bit<T: PrimInt>(x: &mut T, pos: usize, b: Bit) {
    match b {
        Zero => *x = *x & !(T::one() << pos),
        One => *x = *x | (T::one() << pos),
    }
}

#[derive(Clone, Copy)]
enum TapeMotion {
    Left,
    Right,
}

const HALT: isize = -1;

struct TuringStep {
    print: Bit,
    motion: TapeMotion,
    next_state: isize,
}

struct TuringState {
    zero: TuringStep,
    one: TuringStep,
}

struct TuringMachine<const N: usize> {
    states: [TuringState; N],
}

macro_rules! turing_machine {
    ( $(($print0:expr, $motion0:expr, $state0:expr; $print1:expr, $motion1:expr, $state1:expr)),+ ) => {
        TuringMachine {
            states: [$(
                TuringState {
                    zero: TuringStep {
                        print: $print0,
                        motion: $motion0,
                        next_state: $state0,
                    },
                    one: TuringStep {
                        print: $print1,
                        motion: $motion1,
                        next_state: $state1,
                    },
                },
            )*],
        }
    };
}

struct Tape<T: Unsigned + PrimInt> {
    right: Vec<T>,
    left: Vec<T>,
    vec_index: usize,
    bit_index: usize,
    half: TapeMotion,
}

impl<T: Unsigned + PrimInt> Tape<T> {
    fn new() -> Tape<T> {
        Tape {
            right: vec![T::zero()],
            left: vec![T::zero()],
            vec_index: 0,
            bit_index: 0,
            half: Right,
        }
    }

    fn get(&self) -> Bit {
        // bytes * 8 = bits
        let vec = match self.half {
            Left => &self.left,
            Right => &self.right,
        };
        let vec_value = vec[self.vec_index];
        return get_bit(vec_value, self.bit_index);
    }

    fn set(&mut self, b: Bit) {
        let vec = match self.half {
            Left => &mut self.left,
            Right => &mut self.right,
        };
        let vec_value = vec.get_mut(self.vec_index).unwrap();
        return set_bit(vec_value, self.bit_index, b);
    }

    fn move_tape(&mut self, motion: TapeMotion) {
        let bits = 8 * size_of::<T>();
        match (self.half, motion) {
            (Left, Left) | (Right, Right) => {
                if self.bit_index == bits - 1 {
                    self.bit_index = 0;
                    self.vec_index += 1;
                    let vec = match self.half {
                        Left => &mut self.left,
                        Right => &mut self.right,
                    };
                    if self.vec_index == vec.len() {
                        vec.push(T::zero());
                    }
                } else {
                    self.bit_index += 1;
                }
            }
            (Left, Right) | (Right, Left) => {
                if self.bit_index == 0 {
                    if self.vec_index == 0 {
                        self.half = match self.half {
                            Left => Right,
                            Right => Left,
                        }
                    } else {
                        self.bit_index = bits - 1;
                        self.vec_index -= 1;
                    }
                } else {
                    self.bit_index -= 1;
                }
            }
        }
    }
}

impl<T: Unsigned + PrimInt> Display for Tape<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output: String = self
            .left
            .iter()
            .rev()
            .map(|x| as_bits(*x))
            .chain(self.right.iter().map(|x| as_bits_rev(*x)))
            .collect();
        f.write_str(output.as_str())
    }
}

/*
// I think this is literally impossible?
// Because there's no way I could link a bit to the vec value without a very quirky return type
impl<T: Unsigned + PrimInt> IndexMut<isize> for Tape<T> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        todo!()
    }
}
*/

/*
static BB2_MACH: TuringMachine<2> = turing_machine!(
    (One, Right, 1; One, Left, 1),
    (One, Left, 0; One, Right, HALT)
);
*/

struct CompiledTuringMachine<T: Unsigned + PrimInt> {
    lut: Box<[T]>,
}

fn main() {
    let tm = turing_machine!(
        (One, Right, 1; One, Left, 1),
        (One, Left, 0; One, Right, HALT)
    );
    let mut tape = Tape::<u8>::new();
    let mut state: isize = 0;
    while state != HALT {
        let step = match tape.get() {
            Zero => &tm.states[state as usize].zero,
            One => &tm.states[state as usize].one,
        };
        tape.set(step.print);
        tape.move_tape(step.motion);
        state = step.next_state;
        println!("{}", tape);
    }
}

/*

0:
    0 1 -> 1
    1 1 <- 1
1:
    0 1 <- 0
    1 1 -> HALT

000000
  0^
000100
   1^
000110
  0^
000110
 1^
001110
0^
011110
 1^
011110
   ^HALT

*/
