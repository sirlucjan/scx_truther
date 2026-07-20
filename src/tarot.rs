// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

use log::info;

use crate::util::Rng;

pub const MAJOR_ARCANA: [(&str, &str); 22] = [
    (
        "The Fool",
        "You are about to `git push --force` to main. Proceed.",
    ),
    (
        "The Magician",
        "You have all the tools you need. You will use none of them correctly.",
    ),
    ("The High Priestess", "The bug knows something you don't."),
    (
        "The Empress",
        "Your codebase is fertile ground for technical debt.",
    ),
    (
        "The Emperor",
        "Someone will demand a status update before you have written a line.",
    ),
    (
        "The Hierophant",
        "Follow the style guide. Nobody else does.",
    ),
    ("The Lovers", "You and the linter will never agree."),
    (
        "The Chariot",
        "Momentum favors whoever ships first, correctness be damned.",
    ),
    (
        "Strength",
        "You will resist the urge to rewrite it in Rust. Eventually you will not.",
    ),
    (
        "The Hermit",
        "The answer was in the documentation you did not read.",
    ),
    (
        "Wheel of Fortune",
        "The CI will pass, then fail, then pass again, for no discernible reason.",
    ),
    ("Justice", "Code review comes for everyone, eventually."),
    (
        "The Hanged Man",
        "Your process is blocked on I/O and has accepted its fate.",
    ),
    ("Death", "A branch you loved will be deleted today."),
    (
        "Temperance",
        "Balance your commits. Nobody wants a 4,000-line diff.",
    ),
    (
        "The Devil",
        "You are chained to a dependency you swore you'd remove.",
    ),
    (
        "The Tower",
        "Production is down. It was always going to be production.",
    ),
    (
        "The Star",
        "A green checkmark appears where you least expect it.",
    ),
    ("The Moon", "Nothing is as it appears in the logs at 3am."),
    (
        "The Sun",
        "The build passes on the first try. Trust nothing.",
    ),
    (
        "Judgement",
        "Your old code has been resurrected in a merge conflict.",
    ),
    (
        "The World",
        "The ticket is closed. Another will open within the hour.",
    ),
];

pub struct TarotReading {
    index: usize,
    reversed: bool,
}

impl TarotReading {
    pub fn draw(rng: &mut Rng) -> Self {
        Self {
            index: (rng.next_u64() as usize) % MAJOR_ARCANA.len(),
            reversed: rng.next_u64() % 2 == 0,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn reversed(&self) -> bool {
        self.reversed
    }

    pub fn log(&self) {
        let (name, meaning) = MAJOR_ARCANA[self.index];
        if self.reversed {
            info!("Tarot draw: {name} (reversed)");
            info!("Reversed meaning: the opposite of \"{meaning}\" — good luck with that.");
        } else {
            info!("Tarot draw: {name}");
            info!("{meaning}");
        }
    }
}
