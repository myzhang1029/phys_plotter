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
use gtk::prelude::*;

pub fn app(application: &gtk::Application) {
    // Main window
    let win = gtk::ApplicationWindow::new(application);
    win.set_default_size(320, 200);
    win.set_title("Physics Plotter");

    // Create system menus
    build_menu(application);
    register_actions(application, &win);

    // Make all widgets visible.
    win.show_all();
}
