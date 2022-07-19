use gtk;
use gtk::prelude::*;
use functions::commonFunctions::{inputMatching, realABVAndAttenuation, FINAL_BRIX_IDEAL};

pub fn guestimateABVPrep(guestimatorBuilder: &gtk::Builder) {
    let guestimatorStartingBrix: gtk::Entry = guestimatorBuilder.get_object("guestimatorStartingBrix").expect("guestimateABVPrep(), guestimatorStartingBrix");
    let guestimatorStartingBrixBuffer: String = guestimatorStartingBrix.get_text().expect("guestimateABVPrep(), guestimatorStartingBrixBuffer");
    let startingBrix: f64 = guestimatorStartingBrixBuffer.validInput();
    let guestimatorTemporaryOutput: gtk::Entry = guestimatorBuilder.get_object("guestimatorABV").expect("guestimateABVPrep(), guestimatorTemporaryOutput");

    if startingBrix.is_nan() {
        guestimatorTemporaryOutput.set_text("Enter a number");
    } else if startingBrix < 2.57 {
        guestimatorTemporaryOutput.set_text("Enter a Brix greater than 2.57");
    } else if startingBrix > 49.48 {
        guestimatorTemporaryOutput.set_text("Enter a Brix less than 49.48");
    } else {
        guestimateABVOutput(startingBrix, guestimatorBuilder);
    }
}

pub fn guestimateABVFormatting(startingBrix: f64) -> String {
    let finalABV: String = format!("{:.2}%", realABVAndAttenuation(startingBrix, FINAL_BRIX_IDEAL).0);
    finalABV
}

fn guestimateABVOutput(startingBrix: f64, guestimatorBuilder: &gtk::Builder) {
    let guestimatorABV: gtk::Entry = guestimatorBuilder.get_object("guestimatorABV").expect("guestimateABVOutput(), guestimatorABV");

    let finalABV: String = guestimateABVFormatting(startingBrix);

    guestimatorABV.set_text(&finalABV);
}