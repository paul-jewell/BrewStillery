use gtk;
use gtk::prelude::*;
use functions::realABV::realABVPrep;

pub fn realABVGUI(builder: &gtk::Builder) {
    let realABVStartingBrix: gtk::Entry = builder.get_object("realABVStartingBrix").expect("realABVGUI(), realABVStartingBrix");
    realABVStartingBrix.connect_key_release_event(clone!(builder => move |_,_key| {
        realABVPrep(&builder);
        Inhibit(false)
    }));

    let realABVFinalBrix: gtk::Entry = builder.get_object("realABVFinalBrix").expect("realABVGUI(), realABVFinalBrix");
    realABVFinalBrix.connect_key_release_event(clone!(builder => move |_,_key| {
        realABVPrep(&builder);
        Inhibit(false)
    }));
}