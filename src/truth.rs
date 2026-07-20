// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

use log::info;

use crate::astrology::Astrology;
use crate::conspiracy::Conspiracy;
use crate::cryptid;
use crate::mercury;
use crate::numerology::LifePath;
use crate::util::Rng;

/// The one true source of truth. Dispenses astrology, conspiracy theories,
/// numerology, and cryptid sightings — never anything about actual
/// scheduling, because this scheduler does not do that.
pub struct Oracle {
    rng: Rng,
    pid: u32,
}

impl Oracle {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: Rng::new(seed),
            pid: std::process::id(),
        }
    }

    /// The full startup ritual: one reading from every category.
    pub fn full_reading(&mut self) {
        info!("=== The Truth, in full ===");
        Astrology::roll(&mut self.rng).log();
        Conspiracy::roll(&mut self.rng).log();
        LifePath::from_pid(self.pid).log();
        mercury::log();
        cryptid::sighting(&mut self.rng);
        info!("===========================");
    }

    /// A single random truth, for periodic reminders that this is still
    /// not a real scheduler.
    pub fn one_truth(&mut self) {
        match self.rng.next_u64() % 4 {
            0 => Astrology::roll(&mut self.rng).log(),
            1 => Conspiracy::roll(&mut self.rng).log(),
            2 => mercury::log(),
            _ => cryptid::sighting(&mut self.rng),
        }
    }
}
