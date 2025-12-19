use log::{info, warn};

use crate::truth::Truth;
use crate::util::seed_from_system;

pub struct TrutherScheduler;

impl TrutherScheduler {
    pub fn run() {
        warn!("scx_truther is NOT a real scheduler");
        warn!("This scheduler does NOT optimize anything");
        warn!("This scheduler exists only as a demo");

        let seed = seed_from_system();
        let truth = Truth::new(seed);

        info!("The truth for this system:");
        truth.log();

        // Intentionally do nothing else.
        // rustland-core will keep the process alive
        // when integrated with a real BPF scheduler.
        std::thread::park();
    }
}
