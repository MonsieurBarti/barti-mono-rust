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
    match command(std::env::args().nth(1).as_deref()) {
        Some("migrate") => migrate::run().await,
        Some("serve") => serve::run().await,
        _ => {
            eprintln!("usage: app <migrate|serve>");
            std::process::exit(2);
        }
    }
}

fn command(arg: Option<&str>) -> Option<&'static str> {
    match arg {
        Some("migrate") => Some("migrate"),
        Some("serve") => Some("serve"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::command;

    #[test]
    fn unknown_argument_is_usage() {
        assert_eq!(command(None), None);
        assert_eq!(command(Some("wat")), None);
        assert_eq!(command(Some("")), None);
    }

    #[test]
    fn known_entries() {
        assert_eq!(command(Some("migrate")), Some("migrate"));
        assert_eq!(command(Some("serve")), Some("serve"));
    }
}
