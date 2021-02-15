mod echo_client;
mod echo_server;
mod utils;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Command {
    command: String,
}

#[tokio::main]
async fn main() {
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
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
}
