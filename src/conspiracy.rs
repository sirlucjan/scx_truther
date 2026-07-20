// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

use log::info;

use crate::util::Rng;

/// Entirely harmless, entirely pop-culture, entirely made up. Explanations
/// are deliberately about the scheduler, never about anything real.
pub const CONSPIRACIES: [(&str, &str); 15] = [
    ("Flat Earth", "Your call stack is flat too. There is no bottom. There never was."),
    ("Moon Landing Was Staged", "This scheduler is also staged. It has never scheduled anything."),
    ("Reptilian Overlords", "The reptilians control your interrupt handlers. That's why they're always cold."),
    ("Birds Aren't Real", "Neither is your uptime counter. Both are surveillance."),
    ("The Illuminati", "The Illuminati sets your process priorities. The kernel just takes the blame."),
    ("Simulation Theory", "We live in a simulation, and this scheduler is the proof: no sane designer would build hardware this weird."),
    ("Bigfoot", "Bigfoot was last seen optimizing a hot loop. Neither he nor the optimization has been seen since."),
    ("The Loch Ness Monster", "Nessie lives in your swap partition. Do not disturb her."),
    ("Ancient Aliens", "Ancient aliens built the pyramids, and also your build pipeline. Nobody fully understands either."),
    ("The Mandela Effect", "You remember this scheduler working differently. It did not. It has always been this useless."),
    ("Time Travelers", "A time traveler visited your codebase once. They left a single TODO comment and vanished."),
    ("Hollow Earth", "Your hard drive is hollow. That's where the deleted files go."),
    ("Area 51", "Area 51 is hiding the source code for a scheduler that actually works."),
    ("Schrodinger's Deadline", "Your deadline is both met and missed until someone opens the ticket."),
    ("The Backrooms", "Somewhere between your L1 and L2 cache, there is a room that should not exist."),
];

pub struct Conspiracy {
    index: usize,
}

impl Conspiracy {
    pub fn roll(rng: &mut Rng) -> Self {
        Self {
            index: (rng.next_u64() as usize) % CONSPIRACIES.len(),
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn log(&self) {
        let (name, explanation) = CONSPIRACIES[self.index];
        info!("Conspiracy of the moment: {name}");
        info!("{explanation}");
    }
}
