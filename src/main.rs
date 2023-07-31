use std::fmt::Display;
use std::mem::size_of;

use num_traits::PrimInt;
use num_traits::Unsigned;

use Bit::*;
use State::*;
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

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Zero => f.write_str("0"),
            One => f.write_str("1"),
        }
    }
}

#[derive(Clone, Copy)]
enum TapeMotion {
    Left,
    Right,
}

impl Display for TapeMotion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Left => f.write_str("<-"),
            Right => f.write_str("->"),
        }
    }
}

#[derive(Clone, Copy)]
enum State {
    HALT,
    Index(usize),
}

impl From<usize> for State {
    fn from(value: usize) -> Self {
        Index(value)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HALT => f.write_str("HALT"),
            Index(i) => f.write_fmt(format_args!("s{}", i)),
        }
    }
}

//const HALT: isize = -1;

struct TuringStep {
    print: Bit,
    motion: TapeMotion,
    next_state: State,
}

struct TuringState {
    zero: TuringStep,
    one: TuringStep,
}

struct TuringMachine<const N: usize> {
    states: [TuringState; N],
    state: State,
}

macro_rules! turing_machine {
    ( $(($print0:expr, $motion0:expr, $state0:expr; $print1:expr, $motion1:expr, $state1:expr)),+ ) => {
        TuringMachine {
            states: [$(
                TuringState {
                    zero: TuringStep {
                        print: $print0,
                        motion: $motion0,
                        next_state: $state0.into(),
                    },
                    one: TuringStep {
                        print: $print1,
                        motion: $motion1,
                        next_state: $state1.into(),
                    },
                },
            )*],
            state: 0.into(),
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

    fn get_index(&self) -> isize {
        let bits = size_of::<T>() * 8;
        let shift = (bits.ilog2()) as usize;
        match self.half {
            Right => ((self.vec_index << shift) | self.bit_index) as isize,
            // bitwise negation happens to be perfect here, since (Left, 0, 0) maps to -1
            Left => (!((self.vec_index << shift) | self.bit_index)) as isize,
        }
    }

    fn get_display_index(&self) -> usize {
        let bits = size_of::<T>() * 8;
        ((self.left.len() * bits) as isize + self.get_index()) as usize
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

fn show_state<const N: usize, T: Unsigned + PrimInt>(tm: &TuringMachine<N>, tape: &Tape<T>) {
    print!(
        "{}^{} \t{}",
        " ".repeat(tape.get_display_index()),
        tape.get_index(),
        tm.state
    );
    if let Index(state) = tm.state {
        let bit = tape.get();
        let step = match bit {
            Zero => &tm.states[state].zero,
            One => &tm.states[state].one,
        };
        println!(
            ".{}: {} {} {}",
            bit, step.print, step.motion, step.next_state
        )
    }
}

impl<const N: usize> TuringMachine<N> {
    fn step<T: Unsigned + PrimInt>(&mut self, tape: &mut Tape<T>, state: usize) {
        let step = match tape.get() {
            Zero => &self.states[state].zero,
            One => &self.states[state].one,
        };
        tape.set(step.print);
        tape.move_tape(step.motion);
        self.state = step.next_state;
    }

    fn run<T: Unsigned + PrimInt>(&mut self, tape: &mut Tape<T>) {
        while let Index(state) = self.state {
            self.step(tape, state);
        }
    }

    fn run_verbose<T: Unsigned + PrimInt>(&mut self, tape: &mut Tape<T>) {
        println!("{}", tape);
        show_state(&self, tape);
        while let Index(state) = self.state {
            self.step(tape, state);
            println!("{}", tape);
            show_state(&self, tape);
        }
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

/*
static BB3_MACH: TuringMachine<3> = turing_machine!(
    (One, Right, 1; One, Left, 2),
    (One, Left, 0; One, Right, 1),
    (One, Left, 1; One, Right, HALT),
);
*/

/*
static COPY_MACH: TuringMachine<5> = turing_machine!(
    (Zero, Right, HALT; Zero, Right, 1),
    (Zero, Right, 2; One, Right, 1),
    (One, Left, 3; One, Right, 2),
    (Zero, Left, 4; One, Left, 3),
    (One, Right, 0; One, Left, 4),
);
*/

struct CompiledTuringMachine<T: Unsigned + PrimInt> {
    lut: Box<[(T, TapeMotion, State)]>,
}

fn main() {
    let mut tm = turing_machine!(
        (Zero, Right, HALT; Zero, Right, 1),
        (Zero, Right, 2; One, Right, 1),
        (One, Left, 3; One, Right, 2),
        (Zero, Left, 4; One, Left, 3),
        (One, Right, 0; One, Left, 4)
    );
    let mut tape = Tape::<u8>::new();
    tape.right[0] = 0x03;
    tm.run_verbose(&mut tape);
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
