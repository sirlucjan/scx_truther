// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

use std::time::{SystemTime, UNIX_EPOCH};

/// Derive a starting seed from wall clock, PID, and hostname so every run
/// (and every periodic reading) gets its own flavor of nonsense.
#[must_use]
pub fn seed_from_system() -> u64 {
    // Truncating u128 nanos to u64 is the point here: we are folding
    // entropy, not measuring time.
    #[allow(clippy::cast_possible_truncation)]
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos() as u64);

    let pid = u64::from(std::process::id());

    let host = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("HOST"))
        .unwrap_or_default();

    let host_hash = host.bytes().fold(0u64, |acc, b| {
        acc.wrapping_mul(31).wrapping_add(u64::from(b))
    });

    nanos ^ pid.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ host_hash
}

/// Minimal splitmix64 PRNG. Deterministic and dependency-free, good enough
/// to pick horoscopes. Not good enough to run a real scheduler — but then
/// again, neither is anything else in this crate.
pub struct Rng(u64);

impl Rng {
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// One index in `0..len`. The u64-to-usize cast cannot truncate: the
    /// value is reduced modulo `len` first, and `len` arrived as a `usize`.
    #[allow(clippy::cast_possible_truncation)]
    pub fn pick_index(&mut self, len: usize) -> usize {
        (self.next_u64() % len as u64) as usize
    }

    pub fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        let idx = self.pick_index(items.len());
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
