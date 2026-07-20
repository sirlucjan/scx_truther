// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

use log::info;

const PHYSICAL_PERIOD: f64 = 23.0;
const EMOTIONAL_PERIOD: f64 = 28.0;
const INTELLECTUAL_PERIOD: f64 = 33.0;

fn phase(day: f64, period: f64) -> f64 {
    (2.0 * std::f64::consts::PI * day / period).sin()
}

fn describe(value: f64) -> &'static str {
    if value > 0.7 {
        "peaking"
    } else if value > 0.2 {
        "rising"
    } else if value > -0.2 {
        "critical — brace for a transition"
    } else if value > -0.7 {
        "falling"
    } else {
        "at rock bottom"
    }
}

/// Classic pseudoscientific biorhythm, except instead of a birth date we use
/// the PID as day zero — this process's only birth is the one that matters
/// to this scheduler.
pub fn log(pid: u32) {
    let day = pid as f64;

    let physical = phase(day, PHYSICAL_PERIOD);
    let emotional = phase(day, EMOTIONAL_PERIOD);
    let intellectual = phase(day, INTELLECTUAL_PERIOD);

    info!("Biorhythm (day {pid} of this process's life):");
    info!("  Physical: {physical:.2} ({})", describe(physical));
    info!("  Emotional: {emotional:.2} ({})", describe(emotional));
    info!(
        "  Intellectual: {intellectual:.2} ({})",
        describe(intellectual)
    );
}
