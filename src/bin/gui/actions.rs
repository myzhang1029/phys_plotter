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

use clap::crate_version;
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::AboutDialogBuilder;
use gtk::License::Gpl30;

pub fn register_actions(application: &gtk::Application, window: &gtk::ApplicationWindow) {
    // About action to show an about dialog
    let about = gio::SimpleAction::new("about", None);
    about.connect_activate(clone!(@weak window => move |_, _| {
        AboutDialogBuilder::new()
            .program_name("Physics Plotter GTK Interface")
            .version(crate_version!())
            .title("About")
            .website_label("Project page on GitHub")
            .website("https://github.com/myzhang1029/phys_plotter")
            .authors(vec![String::from("Zhang Maiyun")])
            .copyright("Copyright (C) 2021 Zhang Maiyun.")
            .license_type(Gpl30)
            .transient_for(&window)
            .build()
            .show_all();
    }));
    application.add_action(&about);
}
