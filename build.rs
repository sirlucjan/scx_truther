// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.
//
// scx_truther does not carry its own .bpf.c: the FIFO dispatch logic lives
// entirely in src/scheduler.rs, riding on top of the generic BPF backend
// that ships inside scx_rustland_core. This build script just compiles that
// backend and generates the Rust bindings for it (src/bpf.rs, bpf_skel.rs,
// bpf_intf.rs — none of which are meant to be edited or committed).

fn main() {
    scx_rustland_core::RustLandBuilder::new()
        .unwrap()
        .build()
        .unwrap();
}
