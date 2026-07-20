// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

use log::info;

use crate::util::Rng;

const CRYPTIDS: [&str; 8] = [
    "Mothman",
    "Chupacabra",
    "The Jersey Devil",
    "A Skinwalker",
    "The Flatwoods Monster",
    "A Dogman",
    "The Wendigo",
    "Slender Man's cousin nobody talks about",
];

/// Picks a real, currently-running process and jokingly accuses it of
/// being a cryptid in disguise. This is the one place `procfs` — a
/// dependency that has been sitting in Cargo.toml unused — finally does
/// something.
pub fn sighting(rng: &mut Rng) {
    let cryptid = rng.pick(&CRYPTIDS);

    let procs: Vec<_> = procfs::process::all_processes()
        .map(|iter| iter.filter_map(Result::ok).collect())
        .unwrap_or_default();

    if procs.is_empty() {
        info!("Cryptid sighting: {cryptid} was spotted, but /proc denied all knowledge of it.");
        return;
    }

    let idx = (rng.next_u64() as usize) % procs.len();
    let target = &procs[idx];
    let comm = target
        .stat()
        .map(|s| s.comm)
        .unwrap_or_else(|_| "an unnamed process".to_string());

    info!("Cryptid sighting: {cryptid} has disguised itself as PID {} ({comm}).", target.pid);
}
