use log::info;

const ZODIAC: [&str; 12] = [
    "Rat 🐀",
    "Ox 🐂",
    "Tiger 🐅",
    "Rabbit 🐇",
    "Dragon 🐉",
    "Snake 🐍",
    "Horse 🐎",
    "Goat 🐐",
    "Monkey 🐒",
    "Rooster 🐓",
    "Dog 🐕",
    "Pig 🐖",
];

const TRUTHS: [&str; 12] = [
    "Your CPU is fast. Your workload is not.",
    "Hard work is not the same as smart work.",
    "More threads will not fix bad architecture.",
    "Latency is a fact. Throughput is a feeling.",
    "Benchmarks do not measure truth.",
    "Optimizing the wrong thing is still wrong.",
    "The scheduler cannot save bad design.",
    "Parallelism amplifies mistakes.",
    "Balance is an illusion.",
    "Complexity grows faster than performance.",
    "Reality does not care about your flags.",
    "Nothing is free. Especially performance.",
];

pub struct Truth {
    index: usize,
}

impl Truth {
    pub fn new(seed: u64) -> Self {
        Self {
            index: (seed % 12) as usize,
        }
    }

    pub fn log(&self) {
        info!("Year of the {}", ZODIAC[self.index]);
        info!("{}", TRUTHS[self.index]);
    }
}
