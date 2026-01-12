/*
 * Copyright (c) 2025 Konstantin Adamov
 *  SPDX-License-Identifier: MIT
 *
 *  For full license text, see the LICENSE file in the repo root.
 */

use adw::ToastOverlay;
use super::constants::{SPACING_LARGE, SPACING_MEDIUM};
use gtk4::prelude::*;
use libadwaita as adw;
use relm4::prelude::*;

pub struct ConversionDialog {
    visible: bool,
    source_code: String,
    source_type: String,
    source_description: String,
    target_code: String,
    target_type: String,
    target_description: String,
    toast_overlay: Option<ToastOverlay>,
}

#[derive(Debug)]
pub enum ConversionDialogMsg {
    Show,
    Close,
    UpdateState(String, String, String, String, String, String),
    CopyToClipboard,
}

#[relm4::component(pub)]
impl SimpleComponent for ConversionDialog {
    type Init = (String, String, String, String, String, String);
    type Input = ConversionDialogMsg;
    type Output = ();

    view! {
        #[root]
        gtk::Dialog {
            set_title: Some("Details"),
            set_modal: true,
            set_default_size: (600, 500),
            #[watch]
            set_visible: model.visible,

            connect_close_request[sender] => move |_| {
                sender.input(ConversionDialogMsg::Close);
                gtk4::glib::Propagation::Stop
            },
            #[name = "toast_overlay"]
            adw::ToastOverlay {
                gtk4::Box {
                    set_orientation: gtk4::Orientation::Vertical,

                    gtk4::ScrolledWindow {
                        set_vexpand: true,
                        set_hexpand: true,

                        gtk4::Box {
                            set_orientation: gtk4::Orientation::Vertical,
                            set_margin_all: SPACING_LARGE,
                            set_spacing: SPACING_LARGE * 2,

                            // Source section
                            gtk4::Box {
                                set_orientation: gtk4::Orientation::Vertical,
                                set_spacing: SPACING_MEDIUM,

                                gtk4::Label {
                                    set_label: "Source",
                                    set_halign: gtk4::Align::Start,
                                    add_css_class: "title-3",
                                },

                                gtk4::Box {
                                    set_orientation: gtk4::Orientation::Horizontal,
                                    set_spacing: 6,

                                    gtk4::Label {
                                        set_label: "Code :",
                                        set_halign: gtk4::Align::Start,
                                    },

                                    gtk4::Label {
                                        #[watch]
                                        set_label: &model.source_code,
                                        set_halign: gtk4::Align::Start,
                                        add_css_class: "accent",
                                    },
                                },

                                gtk4::Box {
                                    set_orientation: gtk4::Orientation::Horizontal,
                                    set_spacing: 6,

                                    gtk4::Label {
                                        set_label: "Type :",
                                        set_halign: gtk4::Align::Start,
                                    },

                                    gtk4::Label {
                                        #[watch]
                                        set_label: &model.source_type,
                                        set_halign: gtk4::Align::Start,
                                    },
                                },
                            },

                            // Source Description
                            gtk4::Box {
                                set_orientation: gtk4::Orientation::Vertical,
                                set_spacing: SPACING_MEDIUM,

                                gtk4::Label {
                                    set_label: "Description",
                                    set_halign: gtk4::Align::Start,
                                    add_css_class: "title-4",
                                },

                                gtk4::Label {
                                    #[watch]
                                    set_label: &model.source_description,
                                    set_halign: gtk4::Align::Start,
                                    set_wrap: true,
                                    set_xalign: 0.0,
                                },
                            },

                            // Conversion section
                            gtk4::Box {
                                set_orientation: gtk4::Orientation::Vertical,
                                set_spacing: SPACING_MEDIUM,

                                gtk4::Label {
                                    #[watch]
                                    set_label: &if model.target_code.is_empty() {
                                        "No Conversion Available".to_string()
                                    } else if model.source_type.contains("ICD-10") {
                                        "Conversion to ICD9".to_string()
                                    } else {
                                        "Conversion to ICD10".to_string()
                                    },
                                    set_halign: gtk4::Align::Start,
                                    add_css_class: "title-3",
                                },

                                gtk4::Box {
                                    set_orientation: gtk4::Orientation::Horizontal,
                                    set_spacing: 6,
                                    #[watch]
                                    set_visible: !model.target_code.is_empty(),

                                    gtk4::Label {
                                        set_label: "Code :",
                                        set_halign: gtk4::Align::Start,
                                    },

                                    gtk4::Label {
                                        #[watch]
                                        set_label: &model.target_code,
                                        set_halign: gtk4::Align::Start,
                                        add_css_class: "accent",
                                    },
                                },

                                gtk4::Box {
                                    set_orientation: gtk4::Orientation::Horizontal,
                                    set_spacing: 6,
                                    #[watch]
                                    set_visible: !model.target_code.is_empty(),

                                    gtk4::Label {
                                        set_label: "Type :",
                                        set_halign: gtk4::Align::Start,
                                    },

                                    gtk4::Label {
                                        #[watch]
                                        set_label: &model.target_type,
                                        set_halign: gtk4::Align::Start,
                                    },
                                },
                            },

                            // Target Description
                            gtk4::Box {
                                set_orientation: gtk4::Orientation::Vertical,
                                set_spacing: SPACING_MEDIUM,
                                #[watch]
                                set_visible: !model.target_code.is_empty(),

                                gtk4::Label {
                                    set_label: "Description",
                                    set_halign: gtk4::Align::Start,
                                    add_css_class: "title-4",
                                },

                                gtk4::Label {
                                    #[watch]
                                    set_label: &model.target_description,
                                    set_halign: gtk4::Align::Start,
                                    set_wrap: true,
                                    set_xalign: 0.0,
                                },
                            },

                            // Copy button
                            gtk4::Button {
                                set_label: "Copy to clipboard",
                                set_halign: gtk4::Align::Start,
                                set_margin_top: SPACING_LARGE,
                                #[watch]
                                set_visible: !model.target_code.is_empty(),
                                connect_clicked[sender] => move |_| {
                                    sender.input(ConversionDialogMsg::CopyToClipboard);
                                },
                            },
                        },
                    },
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = ConversionDialog {
            visible: false,
            source_code: init.0,
            source_type: init.1,
            source_description: init.2,
            target_code: init.3,
            target_type: init.4,
            target_description: init.5,
            toast_overlay : None,
        };

        let widgets = view_output!();
        
        model.toast_overlay = Some(widgets.toast_overlay.clone());

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ConversionDialogMsg::Show => {
                self.visible = true;
            }
            ConversionDialogMsg::Close => {
                self.visible = false;
            }
            ConversionDialogMsg::UpdateState(
                source_code,
                source_type,
                source_description,
                target_code,
                target_type,
                target_description,
            ) => {
                self.source_code = source_code;
                self.source_type = source_type;
                self.source_description = source_description;
                self.target_code = target_code;
                self.target_type = target_type;
                self.target_description = target_description;
            }
            ConversionDialogMsg::CopyToClipboard => {
                let text = format!(
                    "Source:\nCode: {}\nType: {}\nDescription: {}\n\nConversion:\nCode: {}\nType: {}\nDescription: {}",
                    self.source_code,
                    self.source_type,
                    self.source_description,
                    self.target_code,
                    self.target_type,
                    self.target_description
                );

                if let Some(display) = gtk4::gdk::Display::default() {
                    display.clipboard().set_text(&text);

                    self.toast_overlay
                        .clone()
                        .unwrap()
                        .add_toast(adw::Toast::new("Copied to clipboard"));
                }
            }
        }
    }
}
