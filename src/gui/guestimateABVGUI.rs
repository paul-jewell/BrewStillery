use gtk;
use gtk::prelude::*;
use functions::guestimateABV::guestimateABVPrep;

pub fn guestimateABVGUI(builder: &gtk::Builder) {
    let guestimatorStartingBrix: gtk::Entry = builder.get_object("guestimatorStartingBrix").expect("guestimateABVGUI(), guestimatorStartingBrix");
    guestimatorStartingBrix.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateABVPrep(&builder);
        Inhibit(false)
    }));
}