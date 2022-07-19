use gtk;
use gtk::prelude::*;
use gdk::DisplayExt;

pub fn gtkCSS(mainWindow: &gtk::ApplicationWindow) {
    let css = gtk::CssProvider::new();
    let cssFile = include_bytes!("BrewStillery.css");
    css.load_from_data(cssFile).expect("gtkCSS(), .load_from_data()");
    let screen = mainWindow.get_display().expect("gtkCSS(), screen").get_default_screen();
    gtk::StyleContext::add_provider_for_screen(&screen, &css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}