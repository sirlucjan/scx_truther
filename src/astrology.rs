// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

use log::info;

use crate::util::Rng;

pub const ZODIAC: [(&str, &str); 12] = [
    (
        "Rat 🐀",
        "Small, fast, and first in line for the CPU. Everyone else is annoyed.",
    ),
    (
        "Ox 🐂",
        "Slow and steady. Wins nothing, but never times out.",
    ),
    (
        "Tiger 🐅",
        "Pounces on the run queue before the scheduler is ready.",
    ),
    (
        "Rabbit 🐇",
        "Context-switches nervously. Never finishes anything.",
    ),
    ("Dragon 🐉", "Demands root. Rarely needs it."),
    (
        "Snake 🐍",
        "Silent, slithering through your call stack unseen.",
    ),
    (
        "Horse 🐎",
        "Gallops through benchmarks it will never publish.",
    ),
    ("Goat 🐐", "Climbs the priority queue out of spite."),
    (
        "Monkey 🐒",
        "Copy-pastes code from the internet and calls it architecture.",
    ),
    ("Rooster 🐓", "Wakes every process at 4am for no reason."),
    ("Dog 🐕", "Loyal to whatever branch you happen to be on."),
    ("Pig 🐖", "Consumes all available memory and is not sorry."),
];

pub const HOROSCOPES: [&str; 12] = [
    "Today your latency will be emotional, not technical.",
    "A stray thread from your past will rejoin you unannounced.",
    "Beware of colleagues bearing 'quick' pull requests.",
    "Your uptime is a lie you tell yourself.",
    "The kernel is watching. It has always been watching.",
    "A single semicolon will change your fate today.",
    "Someone will ask 'is it slow?' and you will say 'it depends'.",
    "Cache invalidation and your love life have more in common than you think.",
    "You will rebase, and you will regret it.",
    "Mercury is not retrograde. Your code just doesn't work.",
    "Trust no benchmark you did not run yourself.",
    "The answer to your bug was always an off-by-one.",
];

pub struct Astrology {
    index: usize,
}

impl Astrology {
    /// Deterministic sign from an arbitrary seed. Kept for testability.
    #[must_use]
    pub fn from_seed(seed: u64) -> Self {
        // Cannot truncate: reduced modulo ZODIAC.len() first.
        #[allow(clippy::cast_possible_truncation)]
        let index = (seed % ZODIAC.len() as u64) as usize;
        Self { index }
    }

    /// Roll a fresh sign from the shared RNG.
    pub fn roll(rng: &mut Rng) -> Self {
        Self {
            index: rng.pick_index(ZODIAC.len()),
        }
    }

    #[must_use]
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn log(&self) {
        let (sign, trait_) = ZODIAC[self.index];
        info!("Year of the {sign}");
        info!("{trait_}");
        info!("Horoscope: {}", HOROSCOPES[self.index]);
    }
}
