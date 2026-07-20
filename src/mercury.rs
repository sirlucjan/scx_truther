// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

use log::info;
use std::time::{SystemTime, UNIX_EPOCH};

/// Mercury is retrograde exactly as often as your last three deploys failed
/// for reasons you couldn't explain: roughly one day in five, computed from
/// today's date so it's at least consistent within a day. Astronomically
/// nonsense. Emotionally accurate.
pub fn is_retrograde() -> bool {
    let days = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() / 86_400)
        .unwrap_or(0);

    days % 5 == 0
}

pub fn log() {
    if is_retrograde() {
        info!("Mercury is retrograde. This explains your last three deploys.");
    } else {
        info!("Mercury is not retrograde. You have no excuse today.");
    }
}
