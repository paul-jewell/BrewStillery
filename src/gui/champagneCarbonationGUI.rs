use gtk;
use gtk::prelude::*;
use functions::champagneCarbonation::champagneCarbonationPrep;

pub fn champagneCarbonationGUI(builder: &gtk::Builder) {
    let champagneCarbonationVolume: gtk::Entry = builder.get_object("champagneCarbonationVolume").expect("champagneCarbonationGUI(), champagneCarbonationVolume");
    champagneCarbonationVolume.connect_key_release_event(clone!(builder => move |_,_key| {
        champagneCarbonationPrep(&builder);
        Inhibit(false)
    }));

    let champagneCarbonationUnits: gtk::ComboBoxText = builder.get_object("champagneCarbonationUnits").expect("champagneCarbonationGUI(), champagneCarbonationUnits");
    champagneCarbonationUnits.connect_changed(clone!(builder => move |_| {
        champagneCarbonationPrep(&builder);
    }));
}