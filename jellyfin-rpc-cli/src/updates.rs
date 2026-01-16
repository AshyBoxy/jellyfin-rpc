use crate::VERSION;
use colored::Colorize;
use log::warn;
use log::info;

pub fn checker() {
    let current = VERSION.unwrap_or("0.0.0").to_string();
    let latest = get_latest_github().unwrap_or(current.clone());
    if latest != current && !current.contains("+") { // assume + is some custom build
        warn!(
            "{} (Current: v{}, Latest: v{})",
            "You are not running the latest version of Jellyfin-RPC"
                .red()
                .bold(),
            current,
            latest,
        );
        warn!("{}", "A newer version can be found at".red().bold());
        warn!(
            "{}",
            "https://github.com/Radiicall/jellyfin-rpc/releases/latest"
                .green()
                .bold()
        );
        warn!(
            "{}",
            "This can be safely ignored if you are running a prerelease version".bold()
        );
    } else {
        info!("You are running Jellyfin-RPC v{}", current);
    }
}

fn get_latest_github() -> Result<String, reqwest::Error> {
    let url = reqwest::blocking::get("https://github.com/Radiicall/jellyfin-rpc/releases/latest")?
        .url()
        .as_str()
        .trim_start_matches("https://github.com/Radiicall/jellyfin-rpc/releases/tag/")
        .to_string();
    Ok(url)
}
