mod echo_client;
mod echo_server;
mod utils;

use std::thread::sleep;
use std::time::Duration;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Command {
    command: String,
}

fn main() {
    let args = Command::from_args();
    println!("run {}", args.command);
    if args.command == "server" {
        echo_server::start();
    } else if args.command == "client" {
        utils::count_report::start();
        echo_client::start();
    } else if args.command == "all" {
        echo_server::start();
        utils::count_report::start();
        echo_client::start();
    }
    sleep(Duration::from_secs(15));
}
