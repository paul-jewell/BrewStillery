use gtk;
use gtk::prelude::*;

use gui::gtkCSS;
use gui::guestimateABVGUI;
use gui::increaseABVGUI;
use gui::grainToABVGUI;
use gui::realABVGUI;
use gui::waterSpargeGUI;
use gui::guestimateIBUGUI;
use gui::gyleCarbonationGUI;
use gui::champagneCarbonationGUI;


pub fn startGTK(application: &gtk::Application) {

    let builder = gtk::Builder::new_from_string(include_str!("BrewStillery.glade"));

    let mainWindow: gtk::ApplicationWindow = builder.get_object("mainWindow").expect("startGTK(), mainWindow");

    mainWindow.set_application(application);

    mainWindow.connect_delete_event(clone!(mainWindow => move |_, _| {
        mainWindow.destroy();
        Inhibit(false)
    }));


    // css section

    gtkCSS::gtkCSS(&mainWindow);


    // guestimateABV Section

    guestimateABVGUI::guestimateABVGUI(&builder);


    // increaseABV Section

    increaseABVGUI::increaseABVGUI(&builder);

    // realABV Section

    realABVGUI::realABVGUI(&builder);


    // grainABV Section

    grainToABVGUI::grainToABVGUI(&builder);


    // waterSparge Section

    waterSpargeGUI::waterSpargeGUI(&builder);


    // guestimateIBU Section

    guestimateIBUGUI::guestimateIBUGUI(&builder);


    // gyleCarbonation Section

    gyleCarbonationGUI::gyleCarbonationGUI(&builder);


    // champagneCarbonation Section

    champagneCarbonationGUI::champagneCarbonationGUI(&builder);


    mainWindow.show_all();
}