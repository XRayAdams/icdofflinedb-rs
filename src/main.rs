/*
 * Copyright (c) 2025 Konstantin Adamov
 *  SPDX-License-Identifier: MIT
 *
 *  For full license text, see the LICENSE file in the repo root.
 */

use libadwaita as adw;
use relm4::prelude::*;
use libadwaita::prelude::ApplicationExt;
mod app;
mod db;
use app::init_icon::init_app_icon;
use app::app::App;
use app::constants::APP_ID;

fn main() {
    gtk4::init().expect("Failed to initialize GTK");
    
    let gtk_app = adw::Application::builder()
        .application_id(APP_ID)
        .flags(gtk4::gio::ApplicationFlags::NON_UNIQUE)
        .build();

    gtk_app.connect_activate(|_| {
        init_app_icon();    
    });
    
    let app = RelmApp::from_app(gtk_app);
    app.run::<App>(());
}
