use std::fs::File;
use std::path::Path;
use std::{thread, time};

use chrono::{DateTime, Utc};
use clap::Parser;
use daemonize::Daemonize;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the user who runs the daemon.
    #[clap(short, long, value_parser)]
    user: String,
    /// Location of the std output file to use
    #[clap(short, long, value_parser, default_value = "/tmp/n8w8-agent.log")]
    outfile: String,
    /// Location of the std err file to use
    #[clap(short, long, value_parser, default_value = "/tmp/n8w8-agent-error.log")]
    errfile: String,
    /// Location of the std output file to use
    #[clap(short, long, value_parser, default_value = "/tmp/n8w8-agent.pid")]
    pidfile: String,
    /// The work dir
    #[clap(short, long, value_parser, default_value = "/tmp")]
    workdir: String,
    /// Refresh timeout for the agent query loop.
    #[clap(short, long, value_parser, default_value_t = 5000)]
    refresh: u64,
}

#[cfg(not(windows))]
fn main() {
    // Parse args first ;)
    let args = Args::parse();
    let stdout = File::create(args.outfile.as_str()).unwrap();
    let stderr = File::create(args.errfile.as_str()).unwrap();
    let sleep_time = time::Duration::from_millis(args.refresh);
    let p = Path::new(args.pidfile.as_str());

    //OK, we can start
    println!("Starting N8w8 Agent..");
    println!(
        "Will write logs to {}, errors to {}, and pid should be at {}, timeout of {}ms",
        args.outfile, args.errfile, args.pidfile, args.refresh
    );
    let daemonize = Daemonize::new()
        .pid_file(p) // Every method except `new` and `start`
        // .chown_pid_file(true) // is optional, see `Daemonize` documentation
        .working_directory(Path::new(args.workdir.as_str())) // for default behaviour.
        .user(args.user.as_str())
        // .group("adm") // Group name
        // .group(2) // or group id.
        // .umask(0o777) // Set umask, `0o027` by default.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .exit_action(|| println!("Should be running now. Please check via pid file! :)"))
        .privileged_action(|| println!("Will enter loop now.."));

    match daemonize.start() {
        Ok(_) => loop {
            let now: DateTime<Utc> = Utc::now();
            let date = format!("UTC now is: {}", now);
            println!("Date is now {}", date);
            // file.write_all(date.as_bytes()).unwrap();
            thread::sleep(sleep_time);
        },
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}
