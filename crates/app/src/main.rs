mod env;
mod http;
mod migrate;
mod serve;
mod telemetry;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    match std::env::args().nth(1).as_deref() {
        Some("migrate") => migrate::run().await,
        Some("serve") => serve::run().await,
        _ => {
            eprintln!("usage: app <migrate|serve>");
            std::process::exit(2);
        }
    }
}
