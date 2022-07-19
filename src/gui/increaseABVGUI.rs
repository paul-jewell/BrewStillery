use gtk;
use gtk::prelude::*;
use functions::increaseABV::increaseABVPrep;

pub fn increaseABVGUI(builder: &gtk::Builder) {
    let increaseABVStartingBrix: gtk::Entry = builder.get_object("increaseABVStartingBrix").expect("increaseABVGUI(), increaseABVStartingBrix");
    increaseABVStartingBrix.connect_key_release_event(clone!(builder => move |_,_key| {
        increaseABVPrep(&builder);
        Inhibit(false)
    }));

    let increaseABVDesiredABV: gtk::Entry = builder.get_object("increaseABVDesiredABV").expect("increaseABVGUI(), increaseABVDesiredABV");
    increaseABVDesiredABV.connect_key_release_event(clone!(builder => move |_,_key| {
        increaseABVPrep(&builder);
        Inhibit(false)
    }));

    let increaseABVCurrentVolume: gtk::Entry = builder.get_object("increaseABVCurrentVolume").expect("increaseABVGUI(), increaseABVCurrentVolume");
    increaseABVCurrentVolume.connect_key_release_event(clone!(builder => move |_,_key| {
        increaseABVPrep(&builder);
        Inhibit(false)
    }));

    let increaseABVUnits: gtk::ComboBoxText = builder.get_object("increaseABVUnits").expect("increaseABVGUI(), increaseABVUnits");
    increaseABVUnits.connect_changed(clone!(builder => move |_| {
        increaseABVPrep(&builder);
    }));
}