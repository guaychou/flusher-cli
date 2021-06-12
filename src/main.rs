mod cli;
mod pkg;

fn main() {
    pkg::log::env_log::log_init();
    cli::flusher::run();
}
