use std::env::args;

use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_pcg::Mcg128Xsl64;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ABO {
    A,
    B,
    O,
}

impl ABO {
    pub fn from_random<R: Rng>(random: &mut R) -> ABO {
        match random.gen_range(0, 3) {
            0 => ABO::A,
            1 => ABO::B,
            2 => ABO::O,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Expression {
    A,
    B,
    O,
    AB,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Gene(ABO, ABO);

impl Gene {
    pub fn new(x: ABO, y: ABO) -> Gene {
        Gene(x, y)
    }

    pub fn from_random<R: Rng>(random: &mut R) -> Gene {
        Gene::new(ABO::from_random(random), ABO::from_random(random))
    }

    pub fn expr(&self) -> Expression {
        match (self.0, self.1) {
            (ABO::A, ABO::A) | (ABO::A, ABO::O) | (ABO::O, ABO::A) => Expression::A,
            (ABO::B, ABO::B) | (ABO::B, ABO::O) | (ABO::O, ABO::B) => Expression::B,
            (ABO::O, ABO::O) => Expression::O,
            (ABO::A, ABO::B) | (ABO::B, ABO::A) => Expression::AB,
        }
    }

    fn chromosome<R: Rng>(&self, random: &mut R) -> ABO {
        if random.gen_bool(0.5) {
            self.0
        } else {
            self.1
        }
    }

    pub fn cross<R: Rng>(&self, other: &Gene, random: &mut R) -> Gene {
        Gene::new(self.chromosome(random), other.chromosome(random))
    }
}

fn progress<R: Rng>(random: &mut R, now: &[Gene]) -> Vec<Gene> {
    let mut nex = Vec::with_capacity(now.len());
    for _ in 0..now.len() {
        let a = now.choose(random).unwrap();
        let b = now.choose(random).unwrap();
        nex.push(a.cross(b, random));
    }
    nex
}

fn count(people: &[Gene]) -> (u64, u64, u64, u64) {
    let mut a = 0;
    let mut b = 0;
    let mut o = 0;
    let mut ab = 0;
    for p in people {
        match p.expr() {
            Expression::A => a += 1,
            Expression::B => b += 1,
            Expression::O => o += 1,
            Expression::AB => ab += 1,
        }
    }
    (a, b, o, ab)
}

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    if args.len() != 6 {
        eprintln!("Usage: <num_people> <rate_a> <rate_b> <rate_o> <rate_ab> <num_gen>");
        return;
    }
    let num_people = args[0].parse::<usize>().unwrap();
    let rate_a = args[1].parse::<usize>().unwrap();
    let rate_b = args[2].parse::<usize>().unwrap();
    let rate_o = args[3].parse::<usize>().unwrap();
    let rate_ab = args[4].parse::<usize>().unwrap();
    let num_gen = args[5].parse().unwrap();
    let mut random = Mcg128Xsl64::from_entropy();

    let ao_rate = 3;

    let rate_sum = rate_a + rate_b + rate_o * (1 + ao_rate) + rate_ab * (1 + ao_rate);
    let num_people = (num_people + rate_sum - 1) / rate_sum * rate_sum;
    let mut now_people = Vec::with_capacity(num_people);
    for _ in 0..num_people * rate_a / rate_sum {
        now_people.push(Gene::new(ABO::A, ABO::A));
        for _ in 0..ao_rate {
            now_people.push(Gene::new(ABO::A, ABO::O));
        }
    }
    for _ in 0..num_people * rate_b / rate_sum {
        now_people.push(Gene::new(ABO::B, ABO::B));
        for _ in 0..ao_rate {
            now_people.push(Gene::new(ABO::B, ABO::O));
        }
    }
    for _ in 0..num_people * rate_o / rate_sum {
        now_people.push(Gene::new(ABO::O, ABO::O));
    }
    for _ in 0..num_people * rate_ab / rate_sum {
        now_people.push(Gene::new(ABO::A, ABO::B));
    }
    for i in 0..=num_gen {
        let (a, b, o, ab) = count(&now_people);
        println!("{} {} {} {} {}", i, a, b, o, ab);
        now_people = progress(&mut random, &now_people);
    }
}
