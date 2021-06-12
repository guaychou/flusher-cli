use {crate::pkg::flusher::flusher as runner, structopt::StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(name = "Flusher CLI tool")]

struct Options {
    /// Flusher address
    #[structopt(long = "address")]
    address: String,
    /// Config address path
    #[structopt(long = "config", default_value = "flusher.yaml")]
    config: String,
    /// To dry run purpose
    #[structopt(long = "dry-run")]
    dry_run: bool,
    /// Application name , it should be same in vault
    #[structopt(long = "app", default_value = "flusher")]
    app_name: String,
}

pub fn run() {
    let options = Options::from_args();
    let app_path = format!("config/{}", options.app_name);
    runner::hit_flusher(
        &options.address,
        &options.dry_run,
        &options.config,
        &app_path,
    )
}
