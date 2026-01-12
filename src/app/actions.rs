use libadwaita as adw;
use libadwaita::prelude::AdwDialogExt;
use relm4::actions::{RelmAction};
use super::constants::{APP_ID, APP_NAME};

relm4::new_action_group!(pub WindowActionGroup, "win");

relm4::new_stateless_action!(pub AboutAction, WindowActionGroup, "about");


pub fn create_about_action(parent: adw::ApplicationWindow, app_version: &str) -> RelmAction<AboutAction> {
    let app_version = app_version.to_string();
    RelmAction::<AboutAction>::new_stateless(move |_| {
        let dialog = adw::AboutDialog::builder()
            .application_icon(APP_ID)
            .license_type(gtk4::License::MitX11)
            .website("https://github.com/XRayAdams/icdofflinedb-rs")
            .issue_url("https://github.com/XRayAdams/icdofflinedb-rs/issues")
            .application_name(APP_NAME)
            .version(&app_version)
            .copyright("© 2025, 26 Konstantin Adamov")
            .developers(vec!["Konstantin Adamov"])
            .can_close(true)
            .build();
        dialog.present(Some(&parent));
    })
}
