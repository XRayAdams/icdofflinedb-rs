/*
 * Copyright (c) 2025 Konstantin Adamov
 *  SPDX-License-Identifier: MIT
 *
 *  For full license text, see the LICENSE file in the repo root.
 */

use gtk4::Align;
use gtk4::glib;
use gtk4::prelude::*;
use libadwaita as adw;
use relm4::actions::RelmActionGroup;
use relm4::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use super::actions::{AboutAction, WindowActionGroup, create_about_action};
use super::constants::{APP_NAME, SPACING_LARGE, SPACING_MEDIUM};
use super::filter_dialog::{FilterDialog, FilterDialogMsg, FilterDialogOutput};
use super::conversion_dialog::{ConversionDialog, ConversionDialogMsg};
use crate::db::database::{CodeSection, Database, IcdCode};

#[derive(Serialize, Deserialize)]
struct AppSettings {
    selected_section: usize,
    icd9_diag: bool,
    icd9_proc: bool,
    icd10_diag: bool,
    icd10_proc: bool,
}
impl Default for AppSettings {
    fn default() -> Self {
        Self {
            selected_section: 0,
            icd9_diag: true,
            icd9_proc: true,
            icd10_diag: true,
            icd10_proc: true,
        }
    }
}

pub struct App {
    selected_section: usize,
    icd9_diag: bool,
    icd9_proc: bool,
    icd10_diag: bool,
    icd10_proc: bool,
    search_term: String,
    sections: Vec<CodeSection>,
    sections_model: gtk::StringList,
    search_results: Vec<IcdCode>,
    results_store: gtk::gio::ListStore,
    db: Database,
    filter_dialog: Controller<FilterDialog>,
    conversion_dialog: Controller<ConversionDialog>,
    _search_timeout_id: Rc<RefCell<Option<glib::SourceId>>>,
}

impl App {
    fn get_config_path() -> PathBuf {
        let mut path = gtk4::glib::user_config_dir();
        path.push("icdofflinedb");
        std::fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    }

    fn load_config() -> AppSettings {
        let path = Self::get_config_path();
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(settings) = serde_json::from_str::<AppSettings>(&content) {
                return settings;
            }
        }
        AppSettings::default()
    }
    fn save_config(&self) {
        let path = Self::get_config_path();
        let settings = AppSettings {
            selected_section: self.selected_section,
            icd9_diag: self.icd9_diag,
            icd9_proc: self.icd9_proc,
            icd10_diag: self.icd10_diag,
            icd10_proc: self.icd10_proc,
        };
        if let Ok(content) = serde_json::to_string_pretty(&settings) {
            let _ = fs::write(path, content);
        }
    }

    fn get_app_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn get_database_path() -> PathBuf {
        // Check for SNAP environment variable
        if let Ok(snap_path) = std::env::var("SNAP") {
            let db_path = std::path::Path::new(&snap_path).join("assets").join("icddb.db");
            if db_path.exists() {
                return db_path;
            }
        }

        // Fallback for local development
        let local_db = PathBuf::from("assets/icddb.db");
        if local_db.exists() {
            return local_db;
        }

        // Check paths relative to the executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // 1. Assets next to executable (e.g. portable tarball)
                let local_db = exe_dir.join("assets").join("icddb.db");
                if local_db.exists() {
                    return local_db;
                }

                // 2. Standard Linux install: ../share/icdofflinedb/assets
                // (assuming binary is in /usr/bin or /usr/local/bin)
                if let Some(prefix) = exe_dir.parent() {
                    let system_db = prefix.join("share").join("icdofflinedb").join("assets").join("icddb.db");
                    if system_db.exists() {
                        return system_db;
                    }
                }
            }
        }

        // Final fallback
        PathBuf::from("assets/icddb.db")
    }
}

#[derive(Debug)]
pub enum Messages {
    Search(String),
    OpenFilterDialog,
    UpdateSelectedSection(usize),
    ShowDetailsDialog(usize),
    ToggleIcd9Diag(bool),
    ToggleIcd9Proc(bool),
    ToggleIcd10Diag(bool),
    ToggleIcd10Proc(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = Messages;
    type Output = ();

    menu! {
        main_menu: {
            section! {
                "_About" => AboutAction,
            }
        }
    }

    view! {
            #[root]
            main_window = adw::ApplicationWindow {
                set_visible: true,
                set_title: Some(APP_NAME),
                set_default_size: (1200, 800),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&main_menu),
                        set_direction: gtk::ArrowType::Down,
                        set_can_focus: false,
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_all: SPACING_LARGE,
                    #[name = "search_entry"]
                    gtk::Entry {
                        set_margin_end: SPACING_MEDIUM,
                        set_hexpand: true,
                        set_placeholder_text: Some("Enter search term (Code, Description)"),
                        connect_changed[sender, search_timeout_id] => move |entry| {
                            let text = entry.text().to_string();

                            // Cancel previous timeout if exists
                            if let Some(id) = search_timeout_id.borrow_mut().take() {
                                id.remove();
                            }

                            if text.len() >= 3 {
                                let sender = sender.clone();
                                let timeout_id_clone = search_timeout_id.clone();

                                // Create new timeout
                                let id = glib::timeout_add_local_once(
                                    std::time::Duration::from_millis(300),
                                    move || {
                                        sender.input(Messages::Search(text));
                                        timeout_id_clone.borrow_mut().take();
                                    }
                                );

                                *search_timeout_id.borrow_mut() = Some(id);
                            }
                        },
                    },
                    gtk::Button {
                        set_label: "_Filter",
                        set_use_underline: true,
                        connect_clicked[sender] => move |_| {
                            sender.input(Messages::OpenFilterDialog);
                        },
                    },
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_start: SPACING_LARGE,
                    set_margin_end: SPACING_LARGE,
                    set_margin_bottom: SPACING_LARGE,

                    gtk::Label {
                        set_label: "Section: ",
                        set_margin_end: SPACING_MEDIUM,
                    },
                    gtk::DropDown {
                        set_halign: Align::Start,
                        set_model: Some(&model.sections_model),
                        set_hexpand: true,
                        set_selected :model.selected_section as u32,

                        connect_selected_item_notify[sender] => move |dropdown| {
                            sender.input(Messages::UpdateSelectedSection(dropdown.selected() as usize));
                        },
                    }
                },
                gtk::ScrolledWindow {
                    set_vexpand: true,
                    set_margin_start: SPACING_LARGE,
                    set_margin_end: SPACING_LARGE,
                    set_margin_bottom: SPACING_LARGE,

                    #[local_ref]
                    results_view -> gtk::ColumnView {
                        set_show_row_separators: true,
                        set_show_column_separators: true,
                        set_single_click_activate: true,
                        set_cursor_from_name: Some("pointer"),
                        connect_activate[sender] => move |_view, position| {
                            sender.input(Messages::ShowDetailsDialog(position as usize));
                        },
                    }
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_start: SPACING_LARGE,
                    set_margin_end: SPACING_LARGE,
                    set_margin_bottom: SPACING_LARGE,
                    gtk::Label {
                        set_halign: Align::Start,
                        #[watch]
                        set_label: &if model.search_results.len() >= 500 {
                            String::from("Showing 500 results (maximum) - please narrow your search")
                        } else {
                            format!("Showing {} result(s)", model.search_results.len())
                        },
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings = Self::load_config();

        let db_path = Self::get_database_path();
        let db = Database::open(&db_path).unwrap_or_else(|_| {
            panic!("Failed to open database at {:?}", db_path);
        });

        // Store sections
        let mut sections = vec![];

        if let Ok(db_sections) = db.get_sections() {
            sections.extend(db_sections);
        }

        // Create sections model for dropdown as a vec of string slices
        let sections_strings: Vec<&str> = sections.iter().map(|s| s.description.as_str()).collect();
        let sections_model = gtk::StringList::new(&sections_strings);

        let filter_dialog = FilterDialog::builder()
            .launch((
                settings.icd9_diag,
                settings.icd9_proc,
                settings.icd10_diag,
                settings.icd10_proc,
            ))
            .forward(sender.input_sender(), |output| match output {
                FilterDialogOutput::ToggleIcd9Diag(v) => Messages::ToggleIcd9Diag(v),
                FilterDialogOutput::ToggleIcd9Proc(v) => Messages::ToggleIcd9Proc(v),
                FilterDialogOutput::ToggleIcd10Diag(v) => Messages::ToggleIcd10Diag(v),
                FilterDialogOutput::ToggleIcd10Proc(v) => Messages::ToggleIcd10Proc(v),
            });

        let conversion_dialog = ConversionDialog::builder()
            .launch((
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ))
            .detach();

        // Create table columns
        let results_store = gtk::gio::ListStore::new::<gtk::StringList>();
        let results_selection = gtk::NoSelection::new(Some(results_store.clone()));

        let search_timeout_id = Rc::new(RefCell::new(None));

        let model = App {
            selected_section: settings.selected_section,
            icd9_diag: settings.icd9_diag,
            icd9_proc: settings.icd9_proc,
            icd10_diag: settings.icd10_diag,
            icd10_proc: settings.icd10_proc,
            search_term: String::new(),
            sections,
            sections_model,
            search_results: Vec::new(),
            results_store: results_store,
            db,
            filter_dialog,
            conversion_dialog,
            _search_timeout_id: search_timeout_id.clone(),
        };

        let results_view = gtk::ColumnView::new(Some(results_selection));

        // Code column
        let code_factory = gtk::SignalListItemFactory::new();
        code_factory.connect_setup(move |_, list_item| {
            let label = gtk::Label::new(None);
            label.set_xalign(0.0);
            list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("ListItem")
                .set_child(Some(&label));
        });
        code_factory.connect_bind(move |_, list_item| {
            let list_item = list_item.downcast_ref::<gtk::ListItem>().expect("ListItem");
            let item = list_item
                .item()
                .and_downcast::<gtk::StringList>()
                .expect("StringList");
            let label = list_item
                .child()
                .and_downcast::<gtk::Label>()
                .expect("Label");
            if let Some(code) = item.string(0) {
                label.set_text(&code);
            }
        });

        let code_column = gtk::ColumnViewColumn::new(Some("Code"), Some(code_factory));
        code_column.set_expand(false);
        results_view.append_column(&code_column);

        // Type column
        let type_factory = gtk::SignalListItemFactory::new();
        type_factory.connect_setup(move |_, list_item| {
            let label = gtk::Label::new(None);
            label.set_xalign(0.0);
            list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("ListItem")
                .set_child(Some(&label));
        });
        type_factory.connect_bind(move |_, list_item| {
            let list_item = list_item.downcast_ref::<gtk::ListItem>().expect("ListItem");
            let item = list_item
                .item()
                .and_downcast::<gtk::StringList>()
                .expect("StringList");
            let label = list_item
                .child()
                .and_downcast::<gtk::Label>()
                .expect("Label");
            if let Some(icd_type) = item.string(1) {
                label.set_text(&icd_type);
            }
        });

        let type_column = gtk::ColumnViewColumn::new(Some("Type"), Some(type_factory));
        type_column.set_expand(false);
        results_view.append_column(&type_column);

        // Description column
        let desc_factory = gtk::SignalListItemFactory::new();
        desc_factory.connect_setup(move |_, list_item| {
            let label = gtk::Label::new(None);
            label.set_xalign(0.0);
            label.set_ellipsize(gtk::pango::EllipsizeMode::End);
            list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("ListItem")
                .set_child(Some(&label));
        });
        desc_factory.connect_bind(move |_, list_item| {
            let list_item = list_item.downcast_ref::<gtk::ListItem>().expect("ListItem");
            let item = list_item
                .item()
                .and_downcast::<gtk::StringList>()
                .expect("StringList");
            let label = list_item
                .child()
                .and_downcast::<gtk::Label>()
                .expect("Label");
            if let Some(desc) = item.string(2) {
                label.set_text(&desc);
            }
        });

        let desc_column = gtk::ColumnViewColumn::new(Some("Description"), Some(desc_factory));
        desc_column.set_expand(true);
        results_view.append_column(&desc_column);

        let widgets = view_output!();

        widgets.search_entry.grab_focus();

        model
            .filter_dialog
            .widget()
            .set_transient_for(Some(&widgets.main_window));

        model
            .conversion_dialog
            .widget()
            .set_transient_for(Some(&widgets.main_window));

        let about_action =
            create_about_action(widgets.main_window.clone(), Self::get_app_version());

        let mut window_actions = RelmActionGroup::<WindowActionGroup>::new();
        window_actions.add_action(about_action);
        window_actions.register_for_widget(&widgets.main_window);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            Messages::Search(query) => {
                if query.len() < 3 {
                    return;
                }

                self.search_term = query;

                let selected_section = if self.selected_section > 0 {
                    self.sections.get(self.selected_section - 1)
                } else {
                    None
                };

                if let Ok(results) = self.db.search(
                    &self.search_term,
                    self.icd9_diag,
                    self.icd9_proc,
                    self.icd10_diag,
                    self.icd10_proc,
                    selected_section,
                ) {
                    self.search_results = results;

                    // Update table
                    self.results_store.remove_all();
                    for result in &self.search_results {
                        let row = gtk::StringList::new(&[
                            &result.code,
                            &result.icd_type,
                            &result.description,
                        ]);
                        self.results_store.append(&row);
                    }
                }
            }
            Messages::UpdateSelectedSection(index) => {
                self.selected_section = index;
                self.save_config();
                sender.input(Messages::Search(self.search_term.clone()));
            }
            Messages::ToggleIcd9Diag(enabled) => {
                self.icd9_diag = enabled;
                self.save_config();
                sender.input(Messages::Search(self.search_term.clone()));
            }
            Messages::ToggleIcd9Proc(enabled) => {
                self.icd9_proc = enabled;
                self.save_config();
                sender.input(Messages::Search(self.search_term.clone()));
            }
            Messages::ToggleIcd10Diag(enabled) => {
                self.icd10_diag = enabled;
                self.save_config();
                sender.input(Messages::Search(self.search_term.clone()));
            }
            Messages::ToggleIcd10Proc(enabled) => {
                self.icd10_proc = enabled;
                self.save_config();
                sender.input(Messages::Search(self.search_term.clone()));
            }
            Messages::OpenFilterDialog => {
                let _ = self
                    .filter_dialog
                    .sender()
                    .send(FilterDialogMsg::UpdateState(
                        self.icd9_diag,
                        self.icd9_proc,
                        self.icd10_diag,
                        self.icd10_proc,
                    ));
                let _ = self.filter_dialog.sender().send(FilterDialogMsg::Show);
            }
            Messages::ShowDetailsDialog(index) => {
                if let Some(result) = self.search_results.get(index) {
                    let is_icd9 = result.type_id == 2 || result.type_id == 3;
                    
                    let (target_code, target_type, target_description) = 
                        if let Ok(conversion_id) = self.db.find_proper_id(is_icd9, result.cpk_id as i64) {
                            if conversion_id != 0 {
                                if let Ok(Some(conversion)) = self.db.get_master_record(conversion_id) {
                                    (conversion.code, conversion.icd_type, conversion.description)
                                } else {
                                    (String::new(), String::new(), String::new())
                                }
                            } else {
                                (String::new(), String::new(), String::new())
                            }
                        } else {
                            (String::new(), String::new(), String::new())
                        };
                    
                    let _ = self.conversion_dialog.sender().send(ConversionDialogMsg::UpdateState(
                        result.code.clone(),
                        result.icd_type.clone(),
                        result.description.clone(),
                        target_code,
                        target_type,
                        target_description,
                    ));
                    let _ = self.conversion_dialog.sender().send(ConversionDialogMsg::Show);
                }
            }
        }
    }
}
