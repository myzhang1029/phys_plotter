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

use gtk::prelude::*;

pub fn build_menu(application: &gtk::Application) {
    // System menu bar
    let menu_bar = gio::Menu::new();

    // Application menu
    let menu = gio::Menu::new();
    menu.append(Some("About"), Some("app.about"));
    menu.append(Some("Quit"), Some("app.quit"));
    application.set_app_menu(Some(&menu));

    // First menu: files
    // - new
    // - open
    // - save
    // - save as
    let files_menu = gio::Menu::new();
    files_menu.append(Some("New"), Some("app.new"));
    files_menu.append(Some("Open"), Some("app.open"));
    files_menu.append(Some("Save"), Some("app.save"));
    files_menu.append(Some("Save As"), Some("app.save_as"));
    menu_bar.append_submenu(Some("_Files"), &files_menu);

    // Second menu: plot
    let plot_menu = gio::Menu::new();
    plot_menu.append(Some("Change Backend"), Some("app.change_backend"));
    plot_menu.append(Some("Generate Plot"), Some("app.plot"));
    menu_bar.append_submenu(Some("_Plot"), &plot_menu);

    application.set_menubar(Some(&menu_bar));
}
