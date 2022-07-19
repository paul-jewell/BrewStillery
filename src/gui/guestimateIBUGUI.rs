use gtk;
use gtk::prelude::*;
use functions::guestimateIBU::guestimateIBUPrep;

pub fn guestimateIBUGUI(builder: &gtk::Builder) {
    let totalIBUPreBoilBrix: gtk::Entry = builder.get_object("totalIBUPreBoilBrix").expect("guestimateIBUGUI(), totalIBUPreBoilBrix");
    totalIBUPreBoilBrix.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUWortVolume: gtk::Entry = builder.get_object("totalIBUWortVolume").expect("guestimateIBUGUI(), totalIBUWortVolume");
    totalIBUWortVolume.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUFirstHopAlpha: gtk::Entry = builder.get_object("totalIBUFirstHopAlpha").expect("guestimateIBUGUI(), totalIBUFirstHopAlpha");
    totalIBUFirstHopAlpha.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUFirstHopAmount: gtk::Entry = builder.get_object("totalIBUFirstHopAmount").expect("guestimateIBUGUI(), totalIBUFirstHopAmount");
    totalIBUFirstHopAmount.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUFirstHopBoilTime: gtk::Entry = builder.get_object("totalIBUFirstHopBoilTime").expect("guestimateIBUGUI(), totalIBUFirstHopBoilTime");
    totalIBUFirstHopBoilTime.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSecondHopAlpha: gtk::Entry = builder.get_object("totalIBUSecondHopAlpha").expect("guestimateIBUGUI(), totalIBUSecondHopAlpha");
    totalIBUSecondHopAlpha.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSecondHopAmount: gtk::Entry = builder.get_object("totalIBUSecondHopAmount").expect("guestimateIBUGUI(), totalIBUSecondHopAmount");
    totalIBUSecondHopAmount.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSecondHopBoilTime: gtk::Entry = builder.get_object("totalIBUSecondHopBoilTime").expect("guestimateIBUGUI(), totalIBUSecondHopBoilTime");
    totalIBUSecondHopBoilTime.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUThirdHopAlpha: gtk::Entry = builder.get_object("totalIBUThirdHopAlpha").expect("guestimateIBUGUI(), totalIBUThirdHopAlpha");
    totalIBUThirdHopAlpha.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUThirdHopAmount: gtk::Entry = builder.get_object("totalIBUThirdHopAmount").expect("guestimateIBUGUI(), totalIBUThirdHopAmount");
    totalIBUThirdHopAmount.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUThirdHopBoilTime: gtk::Entry = builder.get_object("totalIBUThirdHopBoilTime").expect("guestimateIBUGUI(), totalIBUThirdHopBoilTime");
    totalIBUThirdHopBoilTime.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUFourthHopAlpha: gtk::Entry = builder.get_object("totalIBUFourthHopAlpha").expect("guestimateIBUGUI(), totalIBUFourthHopAlpha");
    totalIBUFourthHopAlpha.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUFourthHopAmount: gtk::Entry = builder.get_object("totalIBUFourthHopAmount").expect("guestimateIBUGUI(), totalIBUFourthHopAmount");
    totalIBUFourthHopAmount.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUFourthHopBoilTime: gtk::Entry = builder.get_object("totalIBUFourthHopBoilTime").expect("guestimateIBUGUI(), totalIBUFourthHopBoilTime");
    totalIBUFourthHopBoilTime.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUFifthHopAlpha: gtk::Entry = builder.get_object("totalIBUFifthHopAlpha").expect("guestimateIBUGUI(), totalIBUFifthHopAlpha");
    totalIBUFifthHopAlpha.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUFifthHopAmount: gtk::Entry = builder.get_object("totalIBUFifthHopAmount").expect("guestimateIBUGUI(), totalIBUFifthHopAmount");
    totalIBUFifthHopAmount.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUFifthHopBoilTime: gtk::Entry = builder.get_object("totalIBUFifthHopBoilTime").expect("guestimateIBUGUI(), totalIBUFifthHopBoilTime");
    totalIBUFifthHopBoilTime.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSixthHopAlpha: gtk::Entry = builder.get_object("totalIBUSixthHopAlpha").expect("guestimateIBUGUI(), totalIBUSixthHopAlpha");
    totalIBUSixthHopAlpha.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSixthHopAmount: gtk::Entry = builder.get_object("totalIBUSixthHopAmount").expect("guestimateIBUGUI(), totalIBUSixthHopAmount");
    totalIBUSixthHopAmount.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSixthHopBoilTime: gtk::Entry = builder.get_object("totalIBUSixthHopBoilTime").expect("guestimateIBUGUI(), totalIBUSixthHopBoilTime");
    totalIBUSixthHopBoilTime.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSeventhHopAlpha: gtk::Entry = builder.get_object("totalIBUSeventhHopAlpha").expect("guestimateIBUGUI(), totalIBUSeventhHopAlpha");
    totalIBUSeventhHopAlpha.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSeventhHopAmount: gtk::Entry = builder.get_object("totalIBUSeventhHopAmount").expect("guestimateIBUGUI(), totalIBUSeventhHopAmount");
    totalIBUSeventhHopAmount.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSeventhHopBoilTime: gtk::Entry = builder.get_object("totalIBUSeventhHopBoilTime").expect("guestimateIBUGUI(), totalIBUSeventhHopBoilTime");
    totalIBUSeventhHopBoilTime.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUUnits: gtk::ComboBoxText = builder.get_object("totalIBUUnits").expect("guestimateIBUGUI(), totalIBUUnits");
    totalIBUUnits.connect_changed(clone!(builder => move |_| {
        guestimateIBUPrep(&builder);
    }));
}