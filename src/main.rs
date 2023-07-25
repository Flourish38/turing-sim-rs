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

struct CompiledTuringMachine<T: Unsigned + PrimInt, const N: usize> {
    lut: Box<[T]>,
}

fn main() {
    let tm = TuringMachine {
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
    };
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
