use gtk;
use gtk::prelude::*;
use functions::commonFunctions::{imperialOrMetric, inputMatching, singleInput, FINAL_GRAVITY_IDEAL};

#[derive(Debug)]
pub struct gyleData {
    pub startingGravity: f64,
    pub desiredCO2Level: f64,
    pub finalVolume: f64,
    pub gyleVolumeFloat: f64,
    pub imperialOrMetric: imperialOrMetric,
}

pub fn gyleCarbonationPrep(gyleBuilder: &gtk::Builder) {
    let gyleStartingBrix: gtk::Entry = gyleBuilder.get_object("gyleStartingBrix").expect("gyleCarbonationPrep(), gyleStartingBrix");
    let gyleStartingBrixBuffer: String = gyleStartingBrix.get_text().expect("gyleCarbonationPrep(), gyleStartingBrixBuffer");
    let startingBrix: f64 = gyleStartingBrixBuffer.validInput();

    let gyleDesiredCO2: gtk::Entry = gyleBuilder.get_object("gyleDesiredCO2").expect("gyleCarbonationPrep(), gyleDesiredCO2");
    let gyleDesiredCO2Buffer: String = gyleDesiredCO2.get_text().expect("gyleCarbonationPrep(), gyleDesiredCO2Buffer");
    let desiredCO2LevelTemp: f64 = gyleDesiredCO2Buffer.validInput();

    let gyleWortVolume: gtk::Entry = gyleBuilder.get_object("gyleWortVolume").expect("gyleCarbonationPrep(), gyleWortVolume");
    let gyleWortVolumeBuffer: String = gyleWortVolume.get_text().expect("gyleCarbonationPrep(), gyleWortVolumeBuffer");
    let finalVolumeTemp: f64 = gyleWortVolumeBuffer.validInput();

    let gyleTemporaryOutput: &gtk::Entry = &gyleBuilder.get_object("gyleCarbonationOutput").expect("gyleCarbonationPrep(), gyleTemporaryOutput");

    let gyleCarbonationUnits: gtk::ComboBoxText = gyleBuilder.get_object("gyleCarbonationUnits").expect("gyleCarbonationPrep(), gyleCarbonationUnits");
    let gyleCarbonationUnitsBuffer: String = gyleCarbonationUnits.get_active_id().expect("gyleCarbonationPrep(), gyleCarbonationUnitsBuffer");
    let imperialOrMetric: imperialOrMetric = gyleCarbonationUnitsBuffer.unitMatch();

    if startingBrix < 2.57 {
        gyleTemporaryOutput.set_text("Enter a Starting Brix greater than 2.57");
    } else if startingBrix > 49.48 {
        gyleTemporaryOutput.set_text("Enter a Starting Brix less than 49.48");
    } else if startingBrix.is_nan() || desiredCO2LevelTemp.is_nan() || finalVolumeTemp.is_nan() {
        gyleTemporaryOutput.set_text("Enter all 3 inputs");
    } else if startingBrix <= 0.0 || desiredCO2LevelTemp <= 0.0  || finalVolumeTemp <= 0.0 {
        gyleTemporaryOutput.set_text("Enter a positive number");
    } else {
        let allInputs: gyleData = gyleData {
            startingGravity: startingBrix.brixToGravity(),
            desiredCO2Level: desiredCO2LevelTemp,
            finalVolume: finalVolumeTemp,
            gyleVolumeFloat: 0.0,
            imperialOrMetric: imperialOrMetric,
        };

        gyleCarbonationOutput(allInputs, gyleBuilder);
    }
}

pub fn gyleCarbonationVolumeUnits(allInputs: &mut gyleData) -> &mut gyleData {
    if allInputs.imperialOrMetric == imperialOrMetric::imperialGB {
        allInputs.finalVolume = allInputs.finalVolume.gallonsGBToLitres();
    } else if allInputs.imperialOrMetric == imperialOrMetric::imperialUS {
        allInputs.finalVolume = allInputs.finalVolume.gallonsUSToLitres();
    }

    allInputs
}

pub fn gyleCarbonationMaths(mut allInputs: gyleData) -> gyleData {
    let startingPlato: f64 = allInputs.startingGravity.gravityToPlato();
    let finalPlatoIdeal: f64 = FINAL_GRAVITY_IDEAL.gravityToPlato();

    allInputs.gyleVolumeFloat = (0.24 * allInputs.finalVolume * allInputs.desiredCO2Level) / (startingPlato - finalPlatoIdeal);

    allInputs
}


pub fn gyleCarbonationFormatting(allInputs: gyleData) -> String {
    let mut finalOutput: String = String::from("");
    let gyleCarbonationMathsOutput: gyleData;
    let gyleVolumeFinal: f64;

    if allInputs.imperialOrMetric == imperialOrMetric::imperialGB {
        gyleCarbonationMathsOutput = gyleCarbonationMaths(allInputs);
        gyleVolumeFinal = gyleCarbonationMathsOutput.gyleVolumeFloat.litresToGallonsGB();
        if gyleVolumeFinal >= 0.995 && gyleVolumeFinal < 1.005 {
            finalOutput = format!("{:.0} gallon", gyleVolumeFinal);
        } else if gyleVolumeFinal.fract() >= 0.995 || gyleVolumeFinal.fract() < 0.005 {
            finalOutput = format!("{:.0} gallons", gyleVolumeFinal);
        } else {
            finalOutput = format!("{:.2} gallons", gyleVolumeFinal);
        }
    } else if allInputs.imperialOrMetric == imperialOrMetric::imperialUS {
        gyleCarbonationMathsOutput = gyleCarbonationMaths(allInputs);
        gyleVolumeFinal = gyleCarbonationMathsOutput.gyleVolumeFloat.litresToGallonsUS();
        if gyleVolumeFinal >= 0.995 && gyleVolumeFinal < 1.005 {
            finalOutput = format!("{:.0} gallon", gyleVolumeFinal);
        } else if gyleVolumeFinal.fract() >= 0.995 || gyleVolumeFinal.fract() < 0.005 {
            finalOutput = format!("{:.0} gallons", gyleVolumeFinal);
        } else {
            finalOutput = format!("{:.2} gallons", gyleVolumeFinal);
        }
    } else if allInputs.imperialOrMetric == imperialOrMetric::metric {
        gyleCarbonationMathsOutput = gyleCarbonationMaths(allInputs);
        gyleVolumeFinal = gyleCarbonationMathsOutput.gyleVolumeFloat;
        if gyleVolumeFinal >= 0.995 && gyleVolumeFinal < 1.005 {
            finalOutput = format!("{:.0} litre", gyleVolumeFinal);
        } else if gyleVolumeFinal.fract() >= 0.995 || gyleVolumeFinal.fract() < 0.005 {
            finalOutput = format!("{:.0} litres", gyleVolumeFinal);
        } else {
            finalOutput = format!("{:.2} litres", gyleVolumeFinal);
        }
    }

    finalOutput
}

fn gyleCarbonationOutput(mut allInputs: gyleData, gyleBuilder: &gtk::Builder) {
    let gyleCarbonationOutput: &gtk::Entry = &gyleBuilder.get_object("gyleCarbonationOutput").expect("gyleCarbonationOutput(), gyleCarbonationOutput");

    gyleCarbonationVolumeUnits(&mut allInputs);

    let finalOutputFloat: gyleData = gyleCarbonationMaths(allInputs);

    let finalOutput: String = gyleCarbonationFormatting(finalOutputFloat);

    gyleCarbonationOutput.set_text(&finalOutput);
}