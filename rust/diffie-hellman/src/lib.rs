extern crate rand;

use rand::prelude::*;

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_exp(b_pub, a, p)
}

// Needed for larger numbers to avoid overflow
fn mod_exp(x: u64, y: u64, m: u64) -> u64 {
    (0..y).fold(1, |result, _| (result * x) % m)
}