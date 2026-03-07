
use gtk4::{gio, IconTheme};

pub fn init_app_icon() {
    // Load and register the GResource
    let resources_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/resources.gresource"));
    let resource_data = gtk4::glib::Bytes::from_static(resources_bytes);
    let resource = gio::Resource::from_data(&resource_data).expect("Failed to load GResource");
    gio::resources_register(&resource);
    
    // Add the GResource path to icon theme so AboutDialog can find the icon
    let display = gtk4::gdk::Display::default().expect("Could not get default display.");
    let icon_theme = IconTheme::for_display(&display);
    icon_theme.add_resource_path("/app/rayadams/icdofflinedb/assets");
}
