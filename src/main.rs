use env_logger::Env;
use scx_truther::scheduler::TrutherScheduler;
use scx_truther::truth::Mode;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mode = std::env::args()
        .find_map(|arg| arg.strip_prefix("--mode=").map(str::to_string))
        .and_then(|value| Mode::parse(&value))
        .unwrap_or(Mode::All);

    TrutherScheduler::run(mode);
}
