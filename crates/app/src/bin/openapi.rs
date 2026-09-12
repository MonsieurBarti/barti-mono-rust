#[path = "../openapi.rs"]
mod openapi;

use utoipa_axum::router::OpenApiRouter;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut json = openapi::merge(OpenApiRouter::with_openapi(loads::openapi()))
        .into_openapi()
        .to_pretty_json()?;
    if !json.ends_with('\n') {
        json.push('\n');
    }
    let dest =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/openapi/openapi.json");
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(dest, json)?;
    Ok(())
}
