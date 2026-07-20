// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

use log::info;

const MEANINGS: [(u32, &str); 12] = [
    (1, "The leader. Insists on writing its own scheduler instead of using yours."),
    (2, "The mediator. Constantly resolving merge conflicts nobody asked it to resolve."),
    (3, "The creative. Adds emojis to log output that nobody reads."),
    (4, "The builder. Trusts `cargo build` more than it trusts itself."),
    (5, "The wanderer. Changes distros more often than it changes its shell config."),
    (6, "The caretaker. Pins dependency versions out of love, not fear."),
    (7, "The thinker. Reads the whole man page before running the command."),
    (8, "The achiever. Measures self-worth in benchmark percentiles."),
    (9, "The old soul. Still uses the editor keybindings it learned first."),
    (11, "Master number. Sees patterns in profiler output that are not there."),
    (22, "Master number. Once rewrote a joke scheduler. Twice."),
    (33, "Master number. Has strong opinions about formatting and will share them."),
];

/// Reduce to a single digit, except that 11, 22, and 33 are preserved as
/// "master numbers" along the way — exactly like real numerology, except
/// this is fake.
fn digital_root(mut n: u32) -> u32 {
    loop {
        if n == 11 || n == 22 || n == 33 || n < 10 {
            return n;
        }
        n = n.to_string().bytes().map(|b| (b - b'0') as u32).sum();
    }
}

pub struct LifePath {
    number: u32,
}

impl LifePath {
    pub fn from_pid(pid: u32) -> Self {
        Self {
            number: digital_root(pid),
        }
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn log(&self) {
        let meaning = MEANINGS
            .iter()
            .find(|(n, _)| *n == self.number)
            .map(|(_, m)| *m)
            .unwrap_or("Undefined behavior. Consult the standard.");

        info!("Your PID's life path number is {}", self.number);
        info!("{meaning}");
    }
}
