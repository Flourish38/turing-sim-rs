use core::ops::Index;
use std::mem::size_of;

use num_traits::PrimInt;
use num_traits::Unsigned;

use Bit::*;
use TapeMotion::*;

struct Tape<T: Unsigned + PrimInt> {
    positive: Vec<T>,
    negative: Vec<T>,
    position: isize,
}

enum Bit {
    Zero,
    One,
}

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

impl<T: PrimInt> Index<T> for TuringState {
    type Output = TuringStep;

    fn index(&self, i: T) -> &Self::Output {
        match i {
            _ if i == T::zero() => &self.zero,
            _ if i == T::one() => &self.one,
            _ => panic!(),
        }
    }
}

fn as_bits<T: PrimInt>(x: T) -> String {
    (0..8 * size_of::<T>())
        .rev()
        .map(|i| {
            if (x >> i) & T::one() == T::one() {
                '1'
            } else {
                '0'
            }
        })
        .collect()
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

/*
static BB2_MACH: TuringMachine<2> = turing_machine!(
    (One, Right, 1; One, Left, 1),
    (One, Left, 0; One, Right, HALT)
);
*/

struct CompiledTuringMachine<T: Unsigned + PrimInt, const N: usize> {
    lut: Box<[T]>,
}

fn main() {
    let tm = turing_machine!(
        (One, Right, 1; One, Left, 1),
        (One, Left, 0; One, Right, HALT)
    );
    /*let tm = TuringMachine {
        states: [
            TuringState {
                zero: TuringStep {
                    print: One,
                    motion: Right,
                    next_state: 1,
                },
                one: TuringStep {
                    print: One,
                    motion: Left,
                    next_state: 1,
                },
            },
            TuringState {
                zero: TuringStep {
                    print: One,
                    motion: Left,
                    next_state: 0,
                },
                one: TuringStep {
                    print: One,
                    motion: Right,
                    next_state: -1,
                },
            },
        ],
    };*/
    let mut tape: u8 = 0;
    let mut position: isize = 3;
    let mut state: isize = 0;
    while state != -1 {
        let step = match tape & 1 << position {
            0 => &tm.states[state as usize][0],
            _ => &tm.states[state as usize][1], // That bit must not have been zero, regardless which bit it is
        };
        match step.print {
            Zero => tape &= !(1 << position),
            One => tape |= 1 << position,
        };
        position += match step.motion {
            Left => 1,
            Right => -1,
        };
        state = step.next_state;
        println!("{}", as_bits(tape));
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
