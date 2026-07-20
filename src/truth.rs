// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

use log::info;

use crate::astrology::Astrology;
use crate::biorhythm;
use crate::conspiracy::Conspiracy;
use crate::cryptid;
use crate::eightball;
use crate::mercury;
use crate::numerology::LifePath;
use crate::omen;
use crate::runes::RuneCast;
use crate::tarot::TarotReading;
use crate::util::Rng;

/// Which flavor of nonsense the periodic readings should stick to. `All`
/// (the default) picks a different category every time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    All,
    Astrology,
    Conspiracy,
    Numerology,
    Mercury,
    Cryptid,
    Tarot,
    Runes,
    EightBall,
    Omen,
    Biorhythm,
}

impl Mode {
    pub fn parse(arg: &str) -> Option<Self> {
        match arg {
            "all" => Some(Mode::All),
            "astrology" => Some(Mode::Astrology),
            "conspiracy" => Some(Mode::Conspiracy),
            "numerology" => Some(Mode::Numerology),
            "mercury" => Some(Mode::Mercury),
            "cryptid" => Some(Mode::Cryptid),
            "tarot" => Some(Mode::Tarot),
            "runes" => Some(Mode::Runes),
            "eightball" | "8ball" => Some(Mode::EightBall),
            "omen" => Some(Mode::Omen),
            "biorhythm" => Some(Mode::Biorhythm),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Mode::All => "all",
            Mode::Astrology => "astrology",
            Mode::Conspiracy => "conspiracy",
            Mode::Numerology => "numerology",
            Mode::Mercury => "mercury",
            Mode::Cryptid => "cryptid",
            Mode::Tarot => "tarot",
            Mode::Runes => "runes",
            Mode::EightBall => "eightball",
            Mode::Omen => "omen",
            Mode::Biorhythm => "biorhythm",
        }
    }
}

/// The one true source of truth. Dispenses astrology, conspiracy theories,
/// numerology, tarot, runes, a magic 8-ball, mystically-read load averages,
/// and biorhythms — never anything about actual scheduling, because this
/// scheduler does not do that.
pub struct Oracle {
    rng: Rng,
    pid: u32,
    mode: Mode,
}

impl Oracle {
    pub fn new(seed: u64, mode: Mode) -> Self {
        Self {
            rng: Rng::new(seed),
            pid: std::process::id(),
            mode,
        }
    }

    /// The full startup ritual: one reading from every category, regardless
    /// of which mode the periodic readings will stick to afterward.
    pub fn full_reading(&mut self) {
        info!("=== The Truth, in full ===");
        Astrology::roll(&mut self.rng).log();
        Conspiracy::roll(&mut self.rng).log();
        LifePath::from_pid(self.pid).log();
        mercury::log();
        cryptid::sighting(&mut self.rng);
        TarotReading::draw(&mut self.rng).log();
        RuneCast::cast(&mut self.rng).log();
        eightball::shake(&mut self.rng);
        omen::log();
        biorhythm::log(self.pid);
        info!("===========================");
    }

    /// A single truth, for periodic reminders that this is still not a real
    /// scheduler. Respects the configured mode.
    pub fn one_truth(&mut self) {
        match self.mode {
            Mode::All => self.random_truth(),
            Mode::Astrology => Astrology::roll(&mut self.rng).log(),
            Mode::Conspiracy => Conspiracy::roll(&mut self.rng).log(),
            Mode::Numerology => LifePath::from_pid(self.pid).log(),
            Mode::Mercury => mercury::log(),
            Mode::Cryptid => cryptid::sighting(&mut self.rng),
            Mode::Tarot => TarotReading::draw(&mut self.rng).log(),
            Mode::Runes => RuneCast::cast(&mut self.rng).log(),
            Mode::EightBall => eightball::shake(&mut self.rng),
            Mode::Omen => omen::log(),
            Mode::Biorhythm => biorhythm::log(self.pid),
        }
    }

    fn random_truth(&mut self) {
        match self.rng.next_u64() % 9 {
            0 => Astrology::roll(&mut self.rng).log(),
            1 => Conspiracy::roll(&mut self.rng).log(),
            2 => mercury::log(),
            3 => cryptid::sighting(&mut self.rng),
            4 => TarotReading::draw(&mut self.rng).log(),
            5 => RuneCast::cast(&mut self.rng).log(),
            6 => eightball::shake(&mut self.rng),
            7 => omen::log(),
            _ => biorhythm::log(self.pid),
        }
    }
}
