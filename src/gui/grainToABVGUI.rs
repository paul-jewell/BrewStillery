use gtk;
use gtk::prelude::*;
use functions::grainToABV::grainToABVPrep;
use functions::commonFunctions::{allOverlays, zeroRGBA};

pub fn grainToABVGUI(builder: &gtk::Builder) {
    let allOverlays = allOverlays::new(builder);

    allOverlays.colourOutput.set_tooltip_text("Expected Colour");
    allOverlays.colourOutput.set_sensitive(false);
    allOverlays.colourOutput.set_property_show_editor(false);
    allOverlays.colourOutput.set_rgba(&zeroRGBA());

    allOverlays.overlay.add_overlay(&allOverlays.colourOutput);
    allOverlays.overlay.add_overlay(&allOverlays.dimplePint);
    allOverlays.overlay.add_overlay(&allOverlays.nonickPint);
    allOverlays.overlay.add_overlay(&allOverlays.tulipPint);
    allOverlays.overlay.add_overlay(&allOverlays.pilsner);
    allOverlays.overlay.add_overlay(&allOverlays.mafs);
    allOverlays.overlay.add_overlay(&allOverlays.dimpleHalfPint);
    allOverlays.overlay.add_overlay(&allOverlays.nonickHalfPint);
    allOverlays.overlay.add_overlay(&allOverlays.tulipHalfPint);

    allOverlays.overlay.set_child_index(&allOverlays.dimplePint, 8);
    allOverlays.overlay.set_child_index(&allOverlays.nonickPint, 7);
    allOverlays.overlay.set_child_index(&allOverlays.tulipPint, 6);
    allOverlays.overlay.set_child_index(&allOverlays.pilsner, 5);
    allOverlays.overlay.set_child_index(&allOverlays.mafs, 4);
    allOverlays.overlay.set_child_index(&allOverlays.dimpleHalfPint, 3);
    allOverlays.overlay.set_child_index(&allOverlays.nonickHalfPint, 2);
    allOverlays.overlay.set_child_index(&allOverlays.tulipHalfPint, 1);
    allOverlays.overlay.set_child_index(&allOverlays.colourOutput, 0);

    allOverlays.nonickPint.set_opacity(0.0);
    allOverlays.tulipPint.set_opacity(0.0);
    allOverlays.pilsner.set_opacity(0.0);
    allOverlays.mafs.set_opacity(0.0);
    allOverlays.dimpleHalfPint.set_opacity(0.0);
    allOverlays.nonickHalfPint.set_opacity(0.0);
    allOverlays.tulipHalfPint.set_opacity(0.0);


    let grainABVWortVolume: gtk::Entry = builder.get_object("grainABVWortVolume").expect("grainToABVGUI(), grainABVWortVolume");
    grainABVWortVolume.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainToABVPrep(&allOverlays, &builder);
        Inhibit(false)
    }));

    let grainABVFirstAmount: gtk::Entry = builder.get_object("grainABVFirstAmount").expect("grainToABVGUI(), grainABVFirstAmount");
    grainABVFirstAmount.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainToABVPrep(&allOverlays, &builder);
        Inhibit(false)
    }));

    let grainABVFirstType: gtk::ComboBoxText = builder.get_object("grainABVFirstType").expect("grainToABVGUI(), grainABVFirstType");
    grainABVFirstType.connect_changed(clone!(builder, allOverlays => move |_| {
        grainToABVPrep(&allOverlays, &builder);
    }));

    let grainABVSecondAmount: gtk::Entry = builder.get_object("grainABVSecondAmount").expect("grainToABVGUI(), grainABVSecondAmount");
    grainABVSecondAmount.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainToABVPrep(&allOverlays, &builder);
        Inhibit(false)
    }));

    let grainABVSecondType: gtk::ComboBoxText = builder.get_object("grainABVSecondType").expect("grainToABVGUI(), grainABVSecondType");
    grainABVSecondType.connect_changed(clone!(builder, allOverlays => move |_| {
        grainToABVPrep(&allOverlays, &builder);
    }));

    let grainABVThirdAmount: gtk::Entry = builder.get_object("grainABVThirdAmount").expect("grainToABVGUI(), grainABVThirdAmount");
    grainABVThirdAmount.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainToABVPrep(&allOverlays, &builder);
        Inhibit(false)
    }));

    let grainABVThirdType: gtk::ComboBoxText = builder.get_object("grainABVThirdType").expect("grainToABVGUI(), grainABVThirdType");
    grainABVThirdType.connect_changed(clone!(builder, allOverlays => move |_| {
        grainToABVPrep(&allOverlays, &builder);
    }));

    let grainABVFourthAmount: gtk::Entry = builder.get_object("grainABVFourthAmount").expect("grainToABVGUI(), grainABVFourthAmount");
    grainABVFourthAmount.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainToABVPrep(&allOverlays, &builder);
        Inhibit(false)
    }));

    let grainABVFourthType: gtk::ComboBoxText = builder.get_object("grainABVFourthType").expect("grainToABVGUI(), grainABVFourthType");
    grainABVFourthType.connect_changed(clone!(builder, allOverlays => move |_| {
        grainToABVPrep(&allOverlays, &builder);
    }));

    let grainABVFifthAmount: gtk::Entry = builder.get_object("grainABVFifthAmount").expect("grainToABVGUI(), grainABVFifthAmount");
    grainABVFifthAmount.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainToABVPrep(&allOverlays, &builder);
        Inhibit(false)
    }));

    let grainABVFifthType: gtk::ComboBoxText = builder.get_object("grainABVFifthType").expect("grainToABVGUI(), grainABVFifthType");
    grainABVFifthType.connect_changed(clone!(builder, allOverlays => move |_| {
        grainToABVPrep(&allOverlays, &builder);
    }));

    let grainABVSixthAmount: gtk::Entry = builder.get_object("grainABVSixthAmount").expect("grainToABVGUI(), grainABVSixthAmount");
    grainABVSixthAmount.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainToABVPrep(&allOverlays, &builder);
        Inhibit(false)
    }));

    let grainABVSixthType: gtk::ComboBoxText = builder.get_object("grainABVSixthType").expect("grainToABVGUI(), grainABVSixthType");
    grainABVSixthType.connect_changed(clone!(builder, allOverlays => move |_| {
        grainToABVPrep(&allOverlays, &builder);
    }));

    let grainABVSeventhAmount: gtk::Entry = builder.get_object("grainABVSeventhAmount").expect("grainToABVGUI(), grainABVSeventhAmount");
    grainABVSeventhAmount.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainToABVPrep(&allOverlays, &builder);
        Inhibit(false)
    }));

    let grainABVSeventhType: gtk::ComboBoxText = builder.get_object("grainABVSeventhType").expect("grainToABVGUI(), grainABVSeventhType");
    grainABVSeventhType.connect_changed(clone!(builder, allOverlays => move |_| {
        grainToABVPrep(&allOverlays, &builder);
    }));

    let grainABVGlassSize: gtk::ComboBoxText = builder.get_object("grainABVGlassSize").expect("grainToABVGUI(), grainABVGlassSize");
    grainABVGlassSize.connect_changed(clone!(builder, allOverlays => move |_| {
        grainToABVPrep(&allOverlays, &builder);
    }));

    let grainABVUnits: gtk::ComboBoxText = builder.get_object("grainABVUnits").expect("grainToABVGUI(), grainABVUnits");
    grainABVUnits.connect_changed(clone!(builder, allOverlays => move |_| {
        grainToABVPrep(&allOverlays, &builder);
    }));
}