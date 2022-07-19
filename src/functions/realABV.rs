use gtk;
use gtk::prelude::*;
use functions::commonFunctions::{inputMatching, realABVAndAttenuation};

pub fn realABVPrep(realABVBuilder: &gtk::Builder) {
    let realABVStartingBrix: gtk::Entry = realABVBuilder.get_object("realABVStartingBrix").expect("realABVPrep(), realABVStartingBrix");
    let realABVStartingBrixBuffer: String = realABVStartingBrix.get_text().expect("realABVPrep(), realABVStartingBrixBuffer");
    let startingBrix: f64 = realABVStartingBrixBuffer.validInput();

    let realABVFinalBrix: gtk::Entry = realABVBuilder.get_object("realABVFinalBrix").expect("realABVPrep(), realABVFinalrealABVFinalBrixBrixInput");
    let realABVFinalBrixInputBuffer: String = realABVFinalBrix.get_text().expect("realABVPrep(), realABVFinalBrixInputBuffer");
    let finalBrix: f64 = realABVFinalBrixInputBuffer.validInput();

    let realABVFinalABV: gtk::Entry = realABVBuilder.get_object("realABVFinalABV").expect("realABVPrep(), realABVFinalABV");
    let realABVRealAttenuation: gtk::Entry = realABVBuilder.get_object("realABVRealAttenuation").expect("realABVPrep(), realABVRealAttenuation");

    if startingBrix.is_nan() || finalBrix.is_nan() {
        realABVFinalABV.set_text("Enter both inputs");
        realABVRealAttenuation.set_text("");
    } else if startingBrix <= 0.0 || finalBrix <= 0.0 {
        realABVFinalABV.set_text("Enter a positive number");
        realABVRealAttenuation.set_text("");
    } else if startingBrix <= finalBrix {
        realABVFinalABV.set_text("Starting Brix must be");
        realABVRealAttenuation.set_text("greater than Final Brix");
    } else if finalBrix < 2.57 {
        realABVFinalABV.set_text("Enter a Final Brix");
        realABVRealAttenuation.set_text("greater than 2.57");
    } else if startingBrix > 49.48 {
        realABVFinalABV.set_text("Enter a Starting Brix");
        realABVRealAttenuation.set_text("less than 49.48");
    } else {
        realABVOutput(startingBrix, finalBrix, realABVBuilder);
    }
}

pub fn realABVFormatting(abvFloat: f64, attenuationFloat: f64) -> (String, String) {
    let mut abvFormatted: String = String::from("");
    let mut attenuationFormatted: String = String::from("Enter legimate amounts");

    if abvFloat > 0.0 && abvFloat < 26.0 {
        abvFormatted = format!("{:.2}%", abvFloat);
        attenuationFormatted = format!("{:.2}%", attenuationFloat);
    }

    (abvFormatted, attenuationFormatted)
}

fn realABVOutput(startingBrix: f64, finalBrix: f64, realABVBuilder: &gtk::Builder) {
    let realABVRealAttenuation: gtk::Entry = realABVBuilder.get_object("realABVRealAttenuation").expect("realABVOutput(), realABVRealAttenuation");
    let realABVFinalABV: gtk::Entry = realABVBuilder.get_object("realABVFinalABV").expect("realABVOutput(), realABVFinalABV");

    let (abv, attenuation) = realABVAndAttenuation(startingBrix, finalBrix);
    let (abvFormatted, attenuationFormatted) = realABVFormatting(abv, attenuation);

    realABVFinalABV.set_text(&abvFormatted);
    realABVRealAttenuation.set_text(&attenuationFormatted);
}