// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

use log::info;
use procfs::{Current, LoadAverage};

/// Reads real system state and misinterprets it mystically. This is the
/// closest thing this crate has to monitoring, and it is still useless.
pub fn log() {
    let proc_count = procfs::process::all_processes()
        .map(|it| it.count())
        .unwrap_or(0);

    match LoadAverage::current() {
        Ok(l) => {
            let verdict = if l.one < 1.0 {
                "The omens are calm. The system rests."
            } else if l.one < 4.0 {
                "The omens are stirring. Something is building in the background."
            } else {
                "The omens are furious. Whatever you did, undo it."
            };
            info!(
                "Load average (1m): {:.2} across {proc_count} processes",
                l.one
            );
            info!("{verdict}");
        }
        Err(_) => {
            info!("The omens are unreadable. /proc/loadavg would not speak.");
        }
    }
}
