//
//  Copyright (C) 2021 Zhang Maiyun <myzhang1029@hotmail.com>
//
//  This file is part of physics plotter.
//
//  Physics plotter is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  Physics plotter is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with physics plotter.  If not, see <https://www.gnu.org/licenses/>.
//

use crate::actions::register_actions;
use crate::menu::build_menu;
use crate::state::UIState;
use glib::clone;
use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    Box, EntryBuilder, HeaderBarBuilder, IconSize, Image, Label, MessageDialog,
    MessageDialogBuilder, Paned, ScrolledWindowBuilder, Separator, TextViewBuilder,
    ToolButtonBuilder, ToolItem, Toolbar,
};
use phys_plotter::default_values as defv;
use std::cell::RefCell;
use std::rc::Rc;

/// Draw a toolbar at the top of the window
fn draw_toolbar() -> Toolbar {
    let toolbar = Toolbar::new();
    let button_new = ToolButtonBuilder::new()
        .label("_New")
        .label_widget(&Image::from_icon_name(Some("document-new"), IconSize::Menu))
        .tooltip_text("Create a new plotting space")
        .is_important(true)
        .action_name("app.new")
        .build();
    toolbar.add(&button_new);
    let button_open = ToolButtonBuilder::new()
        .label("_Open")
        .label_widget(&Image::from_icon_name(
            Some("document-open"),
            IconSize::Menu,
        ))
        .tooltip_text("Open a dataset or plotting space")
        .action_name("app.open")
        .build();
    toolbar.add(&button_open);
    let button_save = ToolButtonBuilder::new()
        .label("_Save")
        .label_widget(&Image::from_icon_name(
            Some("document-save"),
            IconSize::Menu,
        ))
        .tooltip_text("Save the document")
        .action_name("app.save")
        .build();
    toolbar.add(&button_save);
    let button_save_as = ToolButtonBuilder::new()
        .label("Save _As")
        .label_widget(&Image::from_icon_name(
            Some("document-save-as"),
            IconSize::Menu,
        ))
        .tooltip_text("Save the document as")
        .action_name("app.save_as")
        .build();
    toolbar.add(&button_save_as);
    let divider = ToolItem::new();
    divider.add(&Separator::new(Vertical));
    toolbar.add(&divider);
    let button_change_backend = ToolButtonBuilder::new()
        .label("Change _Backend")
        .label_widget(&Image::from_icon_name(
            Some("document-properties"),
            IconSize::Menu,
        ))
        .tooltip_text("Change the plotting backend")
        .action_name("app.change_backend")
        .build();
    toolbar.add(&button_change_backend);
    let button_generate = ToolButtonBuilder::new()
        .label("_Generate Plot")
        .label_widget(&Image::from_icon_name(
            Some("document-print"),
            IconSize::Menu,
        ))
        .tooltip_text("Generate a plotting image")
        .action_name("app.plot")
        .build();
    toolbar.add(&button_generate);
    toolbar
}

/// Create text input area
macro_rules! text_input {
    ($buffer: expr, $placeholder: expr) => {
        EntryBuilder::new()
            .editable(true)
            .buffer($buffer)
            .height_request(1)
            .width_request(10)
            .margin(5)
            .placeholder_text($placeholder)
            .build()
    };
}

/// Draw the properties area, on the left of the editing area
fn draw_properties_area(state: &UIState) -> Box {
    let properties_area = Box::new(Vertical, 1);
    let properties_area_title = HeaderBarBuilder::new().title("Properties").build();
    let title_label = Label::new(Some("Plot title"));
    let title_input = text_input!(&state.title, defv::TITLE);
    let xlabel_label = Label::new(Some("X axis label"));
    let xlabel_input = text_input!(&state.x_label, defv::X_LABEL);
    let ylabel_label = Label::new(Some("Y axis label"));
    let ylabel_input = text_input!(&state.y_label, defv::Y_LABEL);
    let ux_label = Label::new(Some("Default x uncertainty"));
    let ux_input = text_input!(&state.default_x_uncertainty, defv::X_UNCERTAINTY);
    let uy_label = Label::new(Some("Default y uncertainty"));
    let uy_input = text_input!(&state.default_y_uncertainty, defv::Y_UNCERTAINTY);
    properties_area.add(&properties_area_title);
    properties_area.add(&title_label);
    properties_area.add(&title_input);
    properties_area.add(&xlabel_label);
    properties_area.add(&xlabel_input);
    properties_area.add(&ylabel_label);
    properties_area.add(&ylabel_input);
    properties_area.add(&ux_label);
    properties_area.add(&ux_input);
    properties_area.add(&uy_label);
    properties_area.add(&uy_input);
    properties_area
}

/// Draw the editing area
fn draw_editing_area(state: &UIState) -> Paned {
    let editing_area = Paned::new(Horizontal);
    let properties_area = draw_properties_area(state);
    let text_area = Box::new(Vertical, 10);
    let text_area_title = HeaderBarBuilder::new().title("Dataset").build();
    text_area.add(&text_area_title);
    let text_area_view = TextViewBuilder::new().buffer(&state.dataset).build();
    let text_area_text = ScrolledWindowBuilder::new()
        .child(&text_area_view)
        // Have a border around
        .border_width(10)
        // Fill the entire box
        .vexpand(true)
        .hexpand(true)
        .can_focus(true)
        .build();
    text_area.add(&text_area_text);
    editing_area.add1(&properties_area);
    editing_area.add2(&text_area);

    editing_area
}

/// Create the main window
pub fn app(application: &gtk::Application) {
    let ui_state = Rc::new(RefCell::new(UIState::new()));
    // Main window
    let window = gtk::ApplicationWindow::new(application);
    // Set the size and the title
    window.set_default_size(840, 630);
    window.set_title("Physics Plotter");
    window.set_show_menubar(true);

    // Create system menus
    build_menu(application);

    // Main app container
    let container = Box::new(Vertical, 10);
    // At top: toolbar
    let toolbar = draw_toolbar();
    container.add(&toolbar);
    container.add(&Separator::new(Horizontal));
    // Below: Editing area
    let editing_area = draw_editing_area(&ui_state.borrow());
    container.add(&editing_area);

    window.add(&container);

    /*button.connect_clicked(move |_| {
        &label.set_label("Hello, World!");
    });*/

    register_actions(application, &window, ui_state);
    // Make all widgets visible.
    window.show_all();
}

/// Create an error popup that belons to window, with title and error message.
/// When the dismiss button is clicked, the dialog is destroyed automatically.
pub fn create_error_popup(
    window: &gtk::ApplicationWindow,
    title: &str,
    error: &str,
) -> MessageDialog {
    let dialog = MessageDialogBuilder::new()
        .transient_for(window)
        .window_position(gtk::WindowPosition::CenterOnParent)
        .message_type(gtk::MessageType::Error)
        .title(title)
        .icon_name("dialog-error")
        .text(error)
        .buttons(gtk::ButtonsType::Close)
        .build();
    dialog.connect_response(clone!(@strong dialog => move |_, _| {
        unsafe { dialog.destroy(); }
    }));
    dialog.show_all();
    dialog
}

/// Unwrap an option or create an error popup
#[macro_export]
macro_rules! unwrap_option_or_error_return {
    ($thing: expr, $window: expr, $msg: expr, $retv: block) => {
        match $thing {
            Some(result) => result,
            None => {
                create_error_popup($window, "Error", $msg);
                return $retv;
            }
        }
    };
}

/// Unwrap a result or create an error popup
#[macro_export]
macro_rules! unwrap_result_or_error_return {
    ($thing: expr, $window: expr, $msg: expr, $retv: block) => {
        match $thing {
            Ok(result) => result,
            Err(error) => {
                create_error_popup($window, "Error", &format!("{}: {:?}", $msg, error));
                return $retv;
            }
        }
    };
}
