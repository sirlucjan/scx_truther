// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

use std::time::{SystemTime, UNIX_EPOCH};

/// Derive a starting seed from wall clock, PID, and hostname so every run
/// (and every periodic reading) gets its own flavor of nonsense.
pub fn seed_from_system() -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);

    let pid = std::process::id() as u64;

    let host = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("HOST"))
        .unwrap_or_default();

    let host_hash = host
        .bytes()
        .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));

    nanos ^ pid.wrapping_mul(0x9E3779B97F4A7C15) ^ host_hash
}

/// Minimal splitmix64 PRNG. Deterministic and dependency-free, good enough
/// to pick horoscopes. Not good enough to run a real scheduler — but then
/// again, neither is anything else in this crate.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    pub fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        let idx = (self.next_u64() as usize) % items.len();
        &items[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rng_is_deterministic_for_seed() {
        let mut r1 = Rng::new(1234);
        let mut r2 = Rng::new(1234);
        assert_eq!(r1.next_u64(), r2.next_u64());
        assert_eq!(r1.next_u64(), r2.next_u64());
    }
}
