#![allow(non_snake_case, non_camel_case_types)]

#![cfg_attr(feature="clippy", feature(plugin))]

#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate gtk;
extern crate gdk;
extern crate gio;

use gio::prelude::*;
use std::env::args;

#[macro_use]
mod macros;
mod gui;
mod functions;
mod light;
#[cfg(test)]
mod tests;

fn main() {
    let application = gtk::Application::new("in.monkeylog.BrewStillery",
                                            gio::ApplicationFlags::empty())
                                       .expect("Initialisation failed");

    application.connect_startup(move |app| {
        gui::startGTK::startGTK(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}