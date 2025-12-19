use env_logger::Env;
use scx_truther::scheduler::TrutherScheduler;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    TrutherScheduler::run();
}
