use gtk;
use gtk::prelude::*;
use functions::commonFunctions::{imperialOrMetric, inputMatching, singleInput};

pub fn champagneCarbonationPrep(champagneCarbonationBuilder: &gtk::Builder) {
    let champagneCarbonationVolume: gtk::Entry = champagneCarbonationBuilder.get_object("champagneCarbonationVolume").expect("champagneCarbonationPrep(), champagneCarbonationVolume");
    let champagneCarbonationVolumeBuffer: String = champagneCarbonationVolume.get_text().expect("champagneCarbonationPrep(), champagneCarbonationVolumeBuffer");
    let champagneVolume: f64 = champagneCarbonationVolumeBuffer.validInput();

    let champagneCarbonationSugar: gtk::Entry = champagneCarbonationBuilder.get_object("champagneCarbonationSugar").expect("champagneCarbonationPrep(), champagneCarbonationSugar");

    let champagneCarbonationUnits: gtk::ComboBoxText = champagneCarbonationBuilder.get_object("champagneCarbonationUnits").expect("champagneCarbonationPrep(), champagneCarbonationUnits");
    let champagneCarbonationUnitsBuffer: String = champagneCarbonationUnits.get_active_id().expect("champagneCarbonationPrep(), champagneCarbonationUnitsBuffer");
    let imperialOrMetric: imperialOrMetric = champagneCarbonationUnitsBuffer.unitMatch();

    if champagneVolume.is_nan() {
        champagneCarbonationSugar.set_text("Enter a number");
    } else if champagneVolume <= 0.0 {
        champagneCarbonationSugar.set_text("Enter a positive number");
    } else {
        let totalSugar: f64 = champagneCarbonationMaths(champagneVolume, imperialOrMetric);
        champagneOutput(totalSugar, imperialOrMetric, champagneCarbonationBuilder)
    }
}

pub fn champagneCarbonationMaths(champagneVolume: f64, imperialOrMetric: imperialOrMetric) -> f64 {
    let mut totalSugar: f64 = 0.0;

    if imperialOrMetric == imperialOrMetric::imperialGB {
        totalSugar = champagneVolume.gallonsGBToGallonsUS().volumeToSugarChampagne();
    } else if imperialOrMetric == imperialOrMetric::imperialUS {
        totalSugar = champagneVolume.volumeToSugarChampagne();
    } else if imperialOrMetric == imperialOrMetric::metric {
        totalSugar = champagneVolume.litresToGallonsUS().volumeToSugarChampagne().poundsToKilos();
    }

    totalSugar
}

pub fn champagneCarbonationFormatting(totalSugar: f64, imperialOrMetric: imperialOrMetric) -> String {
    let mut sugar: String = String::from("");

    if imperialOrMetric == imperialOrMetric::imperialGB || imperialOrMetric == imperialOrMetric::imperialUS {
        if totalSugar < 0.995 {
            sugar = format!("{:.2} oz", totalSugar.remainingOunces());
        } else if totalSugar >= 0.995 && totalSugar < 1.005 {
            sugar = format!("{:.0} lb", totalSugar);
        } else if totalSugar.trunc() as i64 == 1 && totalSugar.remainingOunces() < 15.995 {
            sugar = format!("{:.0} lb {:.2} oz", totalSugar.trunc(), totalSugar.remainingOunces());
        } else if totalSugar.fract() == 0.0 || totalSugar.remainingOunces() < 0.005 || totalSugar.remainingOunces() >= 15.995 {
            sugar = format!("{:.0} lbs", totalSugar);
        } else {
            sugar = format!("{:.0} lbs {:.2} oz", totalSugar.trunc(), totalSugar.remainingOunces());
        }
    } else if imperialOrMetric == imperialOrMetric::metric {
        if totalSugar >= 0.995 && totalSugar < 1.005 {
            sugar = format!("{:.0} kilo", totalSugar);
        } else if totalSugar.fract() >= 0.995 || totalSugar.fract() < 0.005 {
            sugar = format!("{:.0} kilos", totalSugar);
        } else {
            sugar = format!("{:.2} kilos", totalSugar);
        }
    }

    sugar
}

fn champagneOutput(totalSugar: f64, imperialOrMetric: imperialOrMetric, champagneCarbonationBuilder: &gtk::Builder) {
    let champagneCarbonationSugar: gtk::Entry = champagneCarbonationBuilder.get_object("champagneCarbonationSugar").expect("champagneCarbonationPrep(), champagneCarbonationSugar");

    let sugar: String = champagneCarbonationFormatting(totalSugar, imperialOrMetric);

    champagneCarbonationSugar.set_text(&sugar);
}