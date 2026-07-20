// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use log::{info, warn};

use crate::truth::{Mode, Oracle};
use crate::util::seed_from_system;

/// How often the scheduler shares a new piece of wisdom instead of, you
/// know, scheduling anything. 13 seconds because it felt right.
const READING_INTERVAL: Duration = Duration::from_secs(13);

const BANNER: &str = r#"
        .-"""-.
       /  ⁺  ⁺  \
      |  ✦  ⁺  ✦  |
       \  ⁺  ⁺  /
        `-.....-'
         scx_truther
"#;

pub struct TrutherScheduler;

impl TrutherScheduler {
    pub fn run(mode: Mode) {
        for line in BANNER.lines() {
            info!("{line}");
        }

        warn!("scx_truther is NOT a real scheduler");
        warn!("This scheduler does NOT optimize anything");
        warn!("This scheduler exists only as a demo, a joke, and a cry for help");
        info!("Mode: {}", mode.name());

        let running = Arc::new(AtomicBool::new(true));
        let handler_flag = running.clone();

        if let Err(e) = ctrlc::set_handler(move || {
            handler_flag.store(false, Ordering::SeqCst);
        }) {
            warn!("Could not install Ctrl-C handler ({e}); the truth cannot be stopped");
        }

        let mut oracle = Oracle::new(seed_from_system(), mode);

        info!("The truth for this system:");
        oracle.full_reading();

        while running.load(Ordering::SeqCst) {
            std::thread::sleep(READING_INTERVAL);
            if !running.load(Ordering::SeqCst) {
                break;
            }
            oracle.one_truth();
        }

        info!("Final truth: you ran a fake scheduler on purpose. No judgment.");
    }
}
