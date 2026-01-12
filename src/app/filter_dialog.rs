/*
 * Copyright (c) 2025 Konstantin Adamov
 *  SPDX-License-Identifier: MIT
 *
 *  For full license text, see the LICENSE file in the repo root.
 */

use gtk4::prelude::*;
use relm4::prelude::*;
use super::constants::{SPACING_LARGE, SPACING_MEDIUM};
pub struct FilterDialog {
    icd9_diag: bool,
    icd9_proc: bool,
    icd10_diag: bool,
    icd10_proc: bool,
    visible: bool,
}

#[derive(Debug)]
pub enum FilterDialogMsg {
    Show,
    Close,
    ToggleIcd9Diag(bool),
    ToggleIcd9Proc(bool),
    ToggleIcd10Diag(bool),
    ToggleIcd10Proc(bool),
    UpdateState(bool, bool, bool, bool),
}

#[derive(Debug)]
pub enum FilterDialogOutput {
    ToggleIcd9Diag(bool),
    ToggleIcd9Proc(bool),
    ToggleIcd10Diag(bool),
    ToggleIcd10Proc(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for FilterDialog {
    type Init = (bool, bool, bool, bool);
    type Input = FilterDialogMsg;
    type Output = FilterDialogOutput;

    view! {
        gtk::Dialog {
            set_title: Some("Filter"),
            set_modal: true,
            set_resizable: false,
            set_default_size: (300, 250),
            #[watch]
            set_visible: model.visible,
            connect_close_request[sender] => move |_| {
                sender.input(FilterDialogMsg::Close);
                gtk::glib::Propagation::Stop
            },
            
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: SPACING_LARGE,
                set_spacing: SPACING_MEDIUM,

                gtk::CheckButton {
                    set_label: Some("ICD 9 Diagnosis"),
                    set_active: model.icd9_diag,
                    connect_toggled[sender] => move |btn| {
                        sender.input(FilterDialogMsg::ToggleIcd9Diag(btn.is_active()));
                    }
                },
                gtk::CheckButton {
                    set_label: Some("ICD 9 Procedures"),
                    set_active: model.icd9_proc,
                    connect_toggled[sender] => move |btn| {
                        sender.input(FilterDialogMsg::ToggleIcd9Proc(btn.is_active()));
                    }
                },
                gtk::CheckButton {
                    set_label: Some("ICD 10 Diagnosis"),
                    set_active: model.icd10_diag,
                    connect_toggled[sender] => move |btn| {
                        sender.input(FilterDialogMsg::ToggleIcd10Diag(btn.is_active()));
                    }
                },
                gtk::CheckButton {
                    set_label: Some("ICD 10 Procedures"),
                    set_active: model.icd10_proc,
                    connect_toggled[sender] => move |btn| {
                        sender.input(FilterDialogMsg::ToggleIcd10Proc(btn.is_active()));
                    }
                },
                
                gtk::Button {
                    set_label: "Close",
                    connect_clicked[sender] => move |_| {
                        sender.input(FilterDialogMsg::Close);
                    }
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = FilterDialog {
            icd9_diag: init.0,
            icd9_proc: init.1,
            icd10_diag: init.2,
            icd10_proc: init.3,
            visible: false,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            FilterDialogMsg::Show => self.visible = true,
            FilterDialogMsg::Close => self.visible = false,
            FilterDialogMsg::UpdateState(d9, p9, d10, p10) => {
                 self.icd9_diag = d9;
                 self.icd9_proc = p9;
                 self.icd10_diag = d10;
                 self.icd10_proc = p10;
            }
            FilterDialogMsg::ToggleIcd9Diag(v) => {
                self.icd9_diag = v;
                let _ = sender.output(FilterDialogOutput::ToggleIcd9Diag(v));
            }
            FilterDialogMsg::ToggleIcd9Proc(v) => {
                self.icd9_proc = v;
                let _ = sender.output(FilterDialogOutput::ToggleIcd9Proc(v));
            }
            FilterDialogMsg::ToggleIcd10Diag(v) => {
                self.icd10_diag = v;
                let _ = sender.output(FilterDialogOutput::ToggleIcd10Diag(v));
            }
            FilterDialogMsg::ToggleIcd10Proc(v) => {
                self.icd10_proc = v;
                let _ = sender.output(FilterDialogOutput::ToggleIcd10Proc(v));
            }
        }
    }
}
