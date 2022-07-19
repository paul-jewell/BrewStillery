use gtk;
use gtk::prelude::*;
use functions::waterSparge::waterSpargePrep;

pub fn waterSpargeGUI(builder: &gtk::Builder) {
    let spargePreFermentVolume: gtk::Entry = builder.get_object("spargePreFermentVolume").expect("waterSpargeGUI(), spargePreFermentVolume");
    spargePreFermentVolume.connect_key_release_event(clone!(builder => move |_,_key| {
        waterSpargePrep(&builder);
        Inhibit(false)
    }));

    let spargeTotalGrain: gtk::Entry = builder.get_object("spargeTotalGrain").expect("waterSpargeGUI(), spargeTotalGrain");
    spargeTotalGrain.connect_key_release_event(clone!(builder => move |_,_key| {
        waterSpargePrep(&builder);
        Inhibit(false)
    }));

    let spargeBoilTime: gtk::Entry = builder.get_object("spargeBoilTime").expect("waterSpargeGUI(), spargeBoilTime");
    spargeBoilTime.connect_key_release_event(clone!(builder => move |_,_key| {
        waterSpargePrep(&builder);
        Inhibit(false)
    }));

    let waterSpargeUnits: gtk::ComboBoxText = builder.get_object("waterSpargeUnits").expect("waterSpargeGUI(), waterSpargeUnits");
    waterSpargeUnits.connect_changed(clone!(builder => move |_| {
        waterSpargePrep(&builder);
    }));
}