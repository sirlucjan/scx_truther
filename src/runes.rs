// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

use log::info;

use crate::util::Rng;

pub const RUNES: [(&str, &str, &str); 24] = [
    (
        "ᚠ",
        "Fehu",
        "Wealth. Your disk usage will only grow from here.",
    ),
    ("ᚢ", "Uruz", "Strength. The monolith holds, for now."),
    (
        "ᚦ",
        "Thurisaz",
        "A thorn. Something in your dependency tree will prick you.",
    ),
    (
        "ᚨ",
        "Ansuz",
        "A message is coming. It will be a chat ping during your focus block.",
    ),
    (
        "ᚱ",
        "Raidho",
        "A journey. Your PR is about to travel through four rounds of review.",
    ),
    (
        "ᚲ",
        "Kenaz",
        "A torch. Someone finally understands the legacy code. Briefly.",
    ),
    (
        "ᚷ",
        "Gebo",
        "A gift. An unsolicited refactor, delivered as a 2,000-line PR.",
    ),
    (
        "ᚹ",
        "Wunjo",
        "Joy. The tests pass on the first run. Be suspicious.",
    ),
    (
        "ᚺ",
        "Hagalaz",
        "Hail. A dependency bump breaks everything downstream.",
    ),
    (
        "ᚾ",
        "Nauthiz",
        "Need. You need coffee. You will not get coffee.",
    ),
    (
        "ᛁ",
        "Isa",
        "Ice. The deploy pipeline is frozen, as is your patience.",
    ),
    (
        "ᛃ",
        "Jera",
        "A harvest. The sprint ends. Nothing was finished.",
    ),
    (
        "ᛇ",
        "Eihwaz",
        "Endurance. The build has been running for eleven minutes.",
    ),
    (
        "ᛈ",
        "Perthro",
        "Mystery. Nobody knows why it works. Nobody will ask.",
    ),
    (
        "ᛉ",
        "Algiz",
        "Protection. Your branch protection rules save you from yourself.",
    ),
    (
        "ᛊ",
        "Sowilo",
        "The sun. The dashboard is all green. For now.",
    ),
    (
        "ᛏ",
        "Tiwaz",
        "Victory. You close the ticket before the retro.",
    ),
    (
        "ᛒ",
        "Berkano",
        "Growth. The repository grows. So does the technical debt.",
    ),
    (
        "ᛖ",
        "Ehwaz",
        "Movement. The migration finally runs, three quarters late.",
    ),
    (
        "ᛗ",
        "Mannaz",
        "Humanity. Someone on the team remembers to write a comment.",
    ),
    (
        "ᛚ",
        "Laguz",
        "Flow. The data pipeline works, and nobody knows why.",
    ),
    (
        "ᛜ",
        "Ingwaz",
        "Completion. A feature ships. Users do not notice.",
    ),
    (
        "ᛞ",
        "Dagaz",
        "Breakthrough. You finally find the off-by-one.",
    ),
    (
        "ᛟ",
        "Othala",
        "Inheritance. You have inherited a codebase. May the gods help you.",
    ),
];

pub struct RuneCast {
    index: usize,
}

impl RuneCast {
    pub fn cast(rng: &mut Rng) -> Self {
        Self {
            index: (rng.next_u64() as usize) % RUNES.len(),
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn log(&self) {
        let (symbol, name, meaning) = RUNES[self.index];
        info!("Rune cast: {symbol} {name}");
        info!("{meaning}");
    }
}
