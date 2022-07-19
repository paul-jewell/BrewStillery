use gtk;
use gtk::prelude::*;
use functions::gyleCarbonation::gyleCarbonationPrep;

pub fn gyleCarbonationGUI(builder: &gtk::Builder) {
    let gyleStartingBrix: gtk::Entry = builder.get_object("gyleStartingBrix").expect("gyleCarbonationGUI(), gyleStartingBrix");
    gyleStartingBrix.connect_key_release_event(clone!(builder => move |_,_key| {
        gyleCarbonationPrep(&builder);
        Inhibit(false)
    }));

    let gyleDesiredCO2: gtk::Entry = builder.get_object("gyleDesiredCO2").expect("gyleCarbonationGUI(), gyleDesiredCO2");
    gyleDesiredCO2.connect_key_release_event(clone!(builder => move |_,_key| {
        gyleCarbonationPrep(&builder);
        Inhibit(false)
    }));

    let gyleWortVolume: gtk::Entry = builder.get_object("gyleWortVolume").expect("gyleCarbonationGUI(), gyleWortVolume");
    gyleWortVolume.connect_key_release_event(clone!(builder => move |_,_key| {
        gyleCarbonationPrep(&builder);
        Inhibit(false)
    }));

    let gyleCarbonationUnits: gtk::ComboBoxText = builder.get_object("gyleCarbonationUnits").expect("gyleCarbonationGUI(), gyleCarbonationUnits");
    gyleCarbonationUnits.connect_changed(clone!(builder => move |_| {
        gyleCarbonationPrep(&builder);
    }));
}