pub mod client;
pub mod server;

fn main() {
    // We'll run either the server or the client based on the presence of the `--server` flag.
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 && args[1] == "--server" {
        server::run_server();
    } else {
        client::run_client();
    }
}
