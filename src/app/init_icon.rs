use gtk4::{IconTheme};

pub fn init_app_icon() {
    let display = gtk4::gdk::Display::default().expect("Could not get default display.");
    let icon_theme = IconTheme::for_display(&display);
    if let Ok(snap_path) = std::env::var("SNAP") {
        let assets_path = std::path::Path::new(&snap_path).join("assets");
        icon_theme.add_search_path(assets_path);
    } else {
        // Fallback for local development
        icon_theme.add_search_path("assets");

        // Check paths relative to the executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // 1. Assets next to executable (e.g. portable tarball)
                let local_assets = exe_dir.join("assets");
                if local_assets.exists() {
                    icon_theme.add_search_path(local_assets);
                }

                // 2. Standard Linux install: ../share/icdofflinedb/assets
                // (assuming binary is in /usr/bin or /usr/local/bin)
                if let Some(prefix) = exe_dir.parent() {
                    let system_assets = prefix.join("share").join("icdofflinedb").join("assets");
                    if system_assets.exists() {
                        icon_theme.add_search_path(system_assets);
                    }
                }
            }
        }
    }
}
