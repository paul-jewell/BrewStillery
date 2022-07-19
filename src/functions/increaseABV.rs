use gtk;
use gtk::prelude::*;
use functions::commonFunctions::{imperialOrMetric, inputMatching, singleInput, realABVAndAttenuation, FINAL_BRIX_IDEAL};

#[derive(Debug)]
pub struct increaseABVData {
    pub startingBrix: f64,
    pub desiredABV: f64,
    pub currentVolume: f64,
    pub imperialOrMetric: imperialOrMetric,
}

#[derive(Debug)]
pub struct finalSugarFloat {
    pub newStartingBrixFinal: f64,
    pub sugarToAdd: f64,
    pub honeyToAdd: f64,
}

#[derive(Debug)]
pub struct sugarOutputStrings {
    pub newStartingBrixFinal: String,
    pub sugarAmount: String,
    pub honeyAmount: String,
}

pub fn increaseABVPrep(increaseABVBuilder: &gtk::Builder) {
    let increaseABVStartingBrix: gtk::Entry = increaseABVBuilder.get_object("increaseABVStartingBrix").expect("increaseABVPrep(), increaseABVStartingBrix");
    let increaseABVBrixBuffer: String = increaseABVStartingBrix.get_text().expect("increaseABVPrep(), increaseABVBrixBuffer");

    let increaseABVDesiredABV: gtk::Entry = increaseABVBuilder.get_object("increaseABVDesiredABV").expect("increaseABVPrep(), increaseABVDesiredABV");
    let increaseABVABVBuffer: String = increaseABVDesiredABV.get_text().expect("increaseABVPrep(), increaseABVABVBuffer");

    let increaseABVCurrentVolume: gtk::Entry = increaseABVBuilder.get_object("increaseABVCurrentVolume").expect("increaseABVPrep(), increaseABVCurrentVolume");
    let increaseABVVolumeBuffer: String = increaseABVCurrentVolume.get_text().expect("increaseABVPrep(), increaseABVVolumeBuffer");

    let increaseABVFinalBrix: gtk::Entry = increaseABVBuilder.get_object("increaseABVFinalBrix").expect("increaseABVPrep(), increaseABVFinalBrix");
    let increaseABVSugar: gtk::Entry = increaseABVBuilder.get_object("increaseABVSugar").expect("increaseABVPrep(), increaseABVSugar");
    let increaseABVHoney: gtk::Entry = increaseABVBuilder.get_object("increaseABVHoney").expect("increaseABVPrep(), increaseABVHoney");

    let increaseABVUnits: gtk::ComboBoxText = increaseABVBuilder.get_object("increaseABVUnits").expect("increaseABVPrep(), increaseABVUnits");
    let increaseABVUnitsBuffer: String = increaseABVUnits.get_active_id().expect("increaseABVPrep(), increaseABVUnitsBuffer");

    let allInputs = increaseABVData {
        startingBrix: increaseABVBrixBuffer.validInput(),
        desiredABV: increaseABVABVBuffer.validInput(),
        currentVolume: increaseABVVolumeBuffer.validInput(),
        imperialOrMetric: increaseABVUnitsBuffer.unitMatch(),
    };

    if allInputs.startingBrix < 2.57 {
        increaseABVFinalBrix.set_text("Enter a brix greater than 2.57");
        increaseABVSugar.set_text("");
        increaseABVHoney.set_text("");
    } else if allInputs.startingBrix > 49.48 {
        increaseABVFinalBrix.set_text("Enter a brix less than 49.48");
        increaseABVSugar.set_text("");
        increaseABVHoney.set_text("");
    } else if allInputs.startingBrix.is_nan() || allInputs.desiredABV.is_nan() || allInputs.currentVolume.is_nan() {
        increaseABVFinalBrix.set_text("Enter all 3 inputs");
        increaseABVSugar.set_text("");
        increaseABVHoney.set_text("");
    } else if allInputs.desiredABV <= 0.0 || allInputs.currentVolume <= 0.0 {
        increaseABVFinalBrix.set_text("Enter a positive number");
        increaseABVSugar.set_text("");
        increaseABVHoney.set_text("");
    } else {
        increaseABVOutput(allInputs, increaseABVBuilder);
    }
}

pub fn increaseABVMaths(allInputs: &increaseABVData) -> finalSugarFloat {
    let mut newStartingBrix: f64 = allInputs.startingBrix;
    let mut newEstimatedABV: f64 = realABVAndAttenuation(newStartingBrix, FINAL_BRIX_IDEAL).0;

    while newEstimatedABV <= allInputs.desiredABV {
        newStartingBrix = newStartingBrix + 0.001;
        newEstimatedABV = realABVAndAttenuation(newStartingBrix, FINAL_BRIX_IDEAL).0;
    }

    let differenceBrix: f64 = newStartingBrix - allInputs.startingBrix;

    let mut finalOutputFloat: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: newStartingBrix,
        sugarToAdd: 0.0,
        honeyToAdd: 0.0,
    };

    if allInputs.imperialOrMetric == imperialOrMetric::imperialGB {
        let sugarInOunces = (allInputs.currentVolume.gallonsGBToLitres() * 10.0 * differenceBrix).gramsToOunces();
        finalOutputFloat.sugarToAdd = sugarInOunces.ouncesToPounds();
        finalOutputFloat.honeyToAdd = sugarInOunces.sugarToHoney().ouncesToPounds();
    } else if allInputs.imperialOrMetric == imperialOrMetric::imperialUS {
        let sugarInOunces = (allInputs.currentVolume.gallonsUSToLitres() * 10.0 * differenceBrix).gramsToOunces();
        finalOutputFloat.sugarToAdd = sugarInOunces.ouncesToPounds();
        finalOutputFloat.honeyToAdd = sugarInOunces.sugarToHoney().ouncesToPounds();
    } else if allInputs.imperialOrMetric == imperialOrMetric::metric {
        finalOutputFloat.sugarToAdd = (allInputs.currentVolume * 10.0 * differenceBrix).gramsToKilograms();
        // Add 10g of sugar for every liter to raise 1°Bx
        finalOutputFloat.honeyToAdd = finalOutputFloat.sugarToAdd.sugarToHoney();
    }
    finalOutputFloat
}

pub fn increaseABVFormatting(allInputs: &increaseABVData, finalOutputFloat: finalSugarFloat) -> sugarOutputStrings {
    let mut finalOutput: sugarOutputStrings = sugarOutputStrings {
        newStartingBrixFinal: format!("{:.2}°Bx", finalOutputFloat.newStartingBrixFinal),
        sugarAmount: String::from(""),
        honeyAmount: String::from(""),
    };

    if allInputs.imperialOrMetric == imperialOrMetric::imperialGB || allInputs.imperialOrMetric == imperialOrMetric::imperialUS {
        if finalOutputFloat.sugarToAdd < 0.995 {
            finalOutput.sugarAmount = format!("{:.2} oz", finalOutputFloat.sugarToAdd.remainingOunces());
        } else if finalOutputFloat.sugarToAdd >= 0.995 && finalOutputFloat.sugarToAdd < 1.005 {
            finalOutput.sugarAmount = format!("{:.0} lb", finalOutputFloat.sugarToAdd);
        } else if finalOutputFloat.sugarToAdd.trunc() as i64 == 1 && finalOutputFloat.sugarToAdd.remainingOunces() < 15.995 {
            finalOutput.sugarAmount = format!("{:.0} lb {:.2} oz", finalOutputFloat.sugarToAdd.trunc(), finalOutputFloat.sugarToAdd.remainingOunces());
        } else if finalOutputFloat.sugarToAdd.fract() == 0.0 || finalOutputFloat.sugarToAdd.remainingOunces() < 0.005 || finalOutputFloat.sugarToAdd.remainingOunces() >= 15.995 {
            finalOutput.sugarAmount = format!("{:.0} lbs", finalOutputFloat.sugarToAdd);
        } else {
            finalOutput.sugarAmount = format!("{:.0} lbs {:.2} oz", finalOutputFloat.sugarToAdd.trunc(), finalOutputFloat.sugarToAdd.remainingOunces());
        }

        if finalOutputFloat.honeyToAdd < 0.995 {
            finalOutput.honeyAmount = format!("{:.2} oz", finalOutputFloat.honeyToAdd.remainingOunces());
        } else if finalOutputFloat.honeyToAdd >= 0.995 && finalOutputFloat.honeyToAdd < 1.005 {
            finalOutput.honeyAmount = format!("{:.0} lb", finalOutputFloat.honeyToAdd);
        } else if finalOutputFloat.honeyToAdd.trunc() as i64 == 1 && finalOutputFloat.honeyToAdd.remainingOunces() < 15.995 {
            finalOutput.honeyAmount = format!("{:.0} lb {:.2} oz", finalOutputFloat.honeyToAdd.trunc(), finalOutputFloat.honeyToAdd.remainingOunces());
        } else if finalOutputFloat.honeyToAdd.fract() == 0.0 || finalOutputFloat.honeyToAdd.remainingOunces() < 0.005 || finalOutputFloat.honeyToAdd.remainingOunces() >= 15.995 {
            finalOutput.honeyAmount = format!("{:.0} lbs", finalOutputFloat.honeyToAdd);
        } else {
            finalOutput.honeyAmount = format!("{:.0} lbs {:.2} oz", finalOutputFloat.honeyToAdd.trunc(), finalOutputFloat.honeyToAdd.remainingOunces());
        }
    } else if allInputs.imperialOrMetric == imperialOrMetric::metric {
        if finalOutputFloat.sugarToAdd >= 0.995 && finalOutputFloat.sugarToAdd < 1.005 {
            finalOutput.sugarAmount = format!("{:.0} kilo", finalOutputFloat.sugarToAdd);
        } else if finalOutputFloat.sugarToAdd.fract() >= 0.995 || finalOutputFloat.sugarToAdd.fract() < 0.005 {
            finalOutput.sugarAmount = format!("{:.0} kilos", finalOutputFloat.sugarToAdd);
        } else {
            finalOutput.sugarAmount = format!("{:.2} kilos", finalOutputFloat.sugarToAdd);
        }

        if finalOutputFloat.honeyToAdd >= 0.995 && finalOutputFloat.honeyToAdd < 1.005 {
            finalOutput.honeyAmount = format!("{:.0} kilo", finalOutputFloat.honeyToAdd);
        } else if finalOutputFloat.honeyToAdd.fract() >= 0.995 || finalOutputFloat.honeyToAdd.fract() < 0.005 {
            finalOutput.honeyAmount = format!("{:.0} kilos", finalOutputFloat.honeyToAdd);
        } else {
            finalOutput.honeyAmount  = format!("{:.2} kilos", finalOutputFloat.honeyToAdd);
        }
    }

    finalOutput
}

fn increaseABVOutput(allInputs: increaseABVData, increaseABVBuilder: &gtk::Builder) {
    let increaseABVFinalBrix: gtk::Entry = increaseABVBuilder.get_object("increaseABVFinalBrix").expect("increaseABVOutput(), increaseABVFinalBrix");
    let increaseABVSugar: gtk::Entry = increaseABVBuilder.get_object("increaseABVSugar").expect("increaseABVOutput(), increaseABVFinalBrix");
    let increaseABVHoney: gtk::Entry = increaseABVBuilder.get_object("increaseABVHoney").expect("increaseABVOutput(), increaseABVFinalBrix");

    let finalOutputFloat: finalSugarFloat = increaseABVMaths(&allInputs);
    let finalOutput: sugarOutputStrings = increaseABVFormatting(&allInputs, finalOutputFloat);

    increaseABVFinalBrix.set_text(&finalOutput.newStartingBrixFinal);
    increaseABVSugar.set_text(&finalOutput.sugarAmount);
    increaseABVHoney.set_text(&finalOutput.honeyAmount);
}