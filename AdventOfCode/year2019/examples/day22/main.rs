use std::collections::VecDeque;
use std::io::{stdin, BufRead};
use std::mem::MaybeUninit;

const NUM_CARDS: usize = 10007;
const WANTED_CARD: usize = 2019;

const NEW_NUM_CARDS: usize = 119315717514047;
const NEW_SHUFFLE_COUNT: usize = 101741582076661;
const NEW_WANTED_CARD: usize = 2020;

#[derive(Clone, Copy, Debug)]
struct Card(usize);

#[derive(Debug)]
enum Shuffle {
    DealInto,
    Cut(i32),
    DealWith { increment: usize },
}

impl Shuffle {
    fn apply(&self, deck: &mut VecDeque<Card>) {
        match self {
            Self::DealInto => {
                let mut buffer = Vec::with_capacity(deck.len());
                while let Some(card) = deck.pop_front() {
                    buffer.push(card);
                }
                while let Some(card) = buffer.pop() {
                    deck.push_back(card);
                }
            },
            Self::Cut(n) => {
                if *n > 0 {
                    for _ in 0..*n {
                        let card = deck.pop_front().expect("empty deck");
                        deck.push_back(card);
                    }
                } else {
                    for _ in 0..n.abs() {
                        let card = deck.pop_back().expect("empty deck");
                        deck.push_front(card);
                    }
                }
            },
            Self::DealWith { increment } => {
                let mut buffer: Vec<MaybeUninit<Card>> = vec![MaybeUninit::uninit(); deck.len()];
                let mut i = 0;
                while let Some(card) = deck.pop_front() {
                    // SAFETY: we are pointing to a valid pointer.
                    unsafe { buffer[i].as_mut_ptr().write(card); }
                    i += increment;
                    if i >= buffer.len() {
                        i -= buffer.len();
                    }
                }
                for card in buffer {
                    // SAFETY: `increment` and `deck.len()` are coprimes so we wrote to all of them.
                    deck.push_back(unsafe { card.assume_init() });
                }
            }
        }
    }
}

impl From<String> for Shuffle {
    fn from(error: String) -> Self {
        // Don't use `Self::` because there's a lint bug.
        // https://github.com/rust-lang/rust/issues/64362
        if error[0..1] == *"c" {
            Shuffle::Cut(
                error[4..].parse().expect("malformed cut input")
            )
        } else if error[5..6] == *"w" {
            Shuffle::DealWith {
                increment: error[20..].parse().expect("malformed deal with input")
            }
        } else {
            Shuffle::DealInto
        }
    }
}

fn read_instructions() -> Vec<Shuffle> {
    stdin()
        .lock()
        .lines()
        .map(|line| line.expect("failed to read input").into())
        .collect()
}

fn load_deck() -> VecDeque<Card> {
    (0..NUM_CARDS).map(Card).collect()
}

fn main() {
    let ins = read_instructions();
    let mut deck = load_deck();

    ins.iter().for_each(|s| s.apply(&mut deck));
    println!("{}", deck.iter().position(|card| card.0 == WANTED_CARD).expect("card not found"));
}
