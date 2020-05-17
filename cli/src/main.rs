use anyhow::Result;
use log::error;
use signal_hook::flag as signal_flag;
use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use structopt::StructOpt;
use taskstats::{TaskstatsConnection, TaskstatsListener};

#[derive(StructOpt, Debug)]
struct Opt {
    /// Silence all output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    stderrlog::new()
        .quiet(opt.quiet)
        .verbosity(opt.verbose)
        .timestamp(stderrlog::Timestamp::Off)
        .init()?;

    let conn = TaskstatsConnection::new()?;
    let cpu_mask = format!("0-{}\0", num_cpus::get() - 1);
    let cpu_mask = CString::new(cpu_mask).unwrap().into_boxed_c_str();
    let mut listener = TaskstatsListener::register(conn, cpu_mask)?;

    let term = Arc::new(AtomicBool::new(false));
    signal_flag::register(signal_hook::SIGINT, Arc::clone(&term))?;
    signal_flag::register(signal_hook::SIGTERM, Arc::clone(&term))?;
    signal_flag::register(signal_hook::SIGQUIT, Arc::clone(&term))?;
    while !term.load(Ordering::Relaxed) {
        match listener.get_next() {
            Ok((pid, stats)) => println!("{}: {:?}", pid, stats),
            Err(e) => error!("{}", e),
        }
    }

    Ok(())
}
