use env;
use figlet_rs::FIGfont;
use log::info;
use {chrono::Local, log::LevelFilter, std::io::Write};

pub fn log_init() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .parse_default_env()
        .init();
    print_banner();
}

fn print_banner() {
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert("Flusher-CLI");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    info!(
        "Starting Flusher-CLI version: {}",
        env!("CARGO_PKG_VERSION")
    )
}
