use gtk;
use gtk::prelude::*;
use functions::commonFunctions::{imperialOrMetric, inputMatching, singleInput};

#[derive(Debug)]
pub struct waterSpargeData {
    pub preFermentVolume: f64,
    pub totalGrain: f64,
    pub boilTime: f64,
    pub grainAbsorption: f64,
    pub mashThickness: f64,
    pub wortShrinkage: f64,
    pub percentBoiloff: f64,
    pub imperialOrMetric: imperialOrMetric,
}

#[derive(Debug)]
pub struct finalSpargeFloat {
    pub mashVolume: f64,
    pub spargeVolume: f64,
    pub totalVolume: f64,
}

#[derive(Debug)]
pub struct spargeOutputStrings {
    pub mashVolumeFinal: String,
    pub spargeVolumeFinal: String,
    pub totalVolumeFinal: String,
}

pub fn waterSpargePrep(waterSpargeBuilder: &gtk::Builder) {
    let spargePreFermentVolume: gtk::Entry = waterSpargeBuilder.get_object("spargePreFermentVolume").expect("waterSpargePrep(), spargePreFermentVolume");
    let spargePreFermentVolumeBuffer: String = spargePreFermentVolume.get_text().expect("waterSpargePrep(), spargePreFermentVolumeBuffer");

    let spargeTotalGrain: gtk::Entry = waterSpargeBuilder.get_object("spargeTotalGrain").expect("waterSpargePrep(), spargeTotalGrain");
    let spargeTotalGrainBuffer: String = spargeTotalGrain.get_text().expect("waterSpargePrep(), spargeTotalGrainBuffer");

    let spargeBoilTime: gtk::Entry = waterSpargeBuilder.get_object("spargeBoilTime").expect("waterSpargePrep(), spargeBoilTime");
    let spargeBoilTimeBuffer: String = spargeBoilTime.get_text().expect("waterSpargePrep(), spargeBoilTimeBuffer");

    let spargeMashWater: gtk::Entry = waterSpargeBuilder.get_object("spargeMashWater").expect("waterSpargePrep(), spargeMashWater");
    let spargeSpargeWater: gtk::Entry = waterSpargeBuilder.get_object("spargeSpargeWater").expect("waterSpargePrep(), spargeSpargeWater");
    let spargeTotalWater: gtk::Entry = waterSpargeBuilder.get_object("spargeTotalWater").expect("waterSpargePrep(), spargeTotalWater");

    let waterSpargeUnits: gtk::ComboBoxText = waterSpargeBuilder.get_object("waterSpargeUnits").expect("waterSpargePrep(), waterSpargeUnits");
    let waterSpargeUnitsBuffer: String = waterSpargeUnits.get_active_id().expect("waterSpargePrep(), waterSpargeUnitsBuffer");

    let mut allInputs: waterSpargeData = waterSpargeData {
        preFermentVolume: spargePreFermentVolumeBuffer.validInput(),
        totalGrain: spargeTotalGrainBuffer.validInput(),
        boilTime: spargeBoilTimeBuffer.validInput(),
        grainAbsorption: 0.15,
        // constant value of 0.15 gallonsUS/lb
        mashThickness: 4.0 / 3.0,
        // constant value of 1.333 quartsUS/lb
        wortShrinkage: 0.04,
        // constant value of 4%
        percentBoiloff: 0.1,
        // constant value of 10%
        imperialOrMetric: waterSpargeUnitsBuffer.unitMatch(),
    };

    if allInputs.preFermentVolume <= 0.0  || allInputs.totalGrain <= 0.0 || allInputs.boilTime * 60.0 <= 0.0 {
        spargeMashWater.set_text("Enter a positive number");
        spargeSpargeWater.set_text("");
        spargeTotalWater.set_text("");
    } else if allInputs.preFermentVolume.is_nan()  || allInputs.totalGrain.is_nan() || allInputs.boilTime.is_nan() {
        spargeMashWater.set_text("Enter all 3 inputs");
        spargeSpargeWater.set_text("");
        spargeTotalWater.set_text("");
    } else {
        allInputs = waterSpargeSetValues(allInputs);
        waterSpargeOutput(allInputs, waterSpargeBuilder)
    }
}

fn waterSpargeSetValues(mut allInputs: waterSpargeData) -> waterSpargeData {

    if allInputs.imperialOrMetric == imperialOrMetric::imperialGB {
        allInputs.preFermentVolume = allInputs.preFermentVolume.gallonsGBToGallonsUS();
    } else if allInputs.imperialOrMetric == imperialOrMetric::metric {
        allInputs.preFermentVolume = allInputs.preFermentVolume.litresToGallonsUS();
        allInputs.totalGrain = allInputs.totalGrain.kilosToPounds();
    }

    allInputs
}

pub fn waterSpargeMaths(allInputs: &waterSpargeData) -> finalSpargeFloat {
    let trubLoss: f64 = allInputs.preFermentVolume * 0.05;
    // 5% is an acceptable norm
    let equipmentLoss: f64 = allInputs.preFermentVolume * 0.08;
    // 8% is an acceptable norm

    let totalWater: f64 = (((allInputs.preFermentVolume + trubLoss) / (1.0 - allInputs.wortShrinkage)) / (1.0 - ((allInputs.boilTime / 60.0) * allInputs.percentBoiloff))) + equipmentLoss + (allInputs.totalGrain * allInputs.grainAbsorption);
    // dividing boilTime by 60.0 because the formula requires hours
    let mashWater: f64 = (allInputs.totalGrain * allInputs.mashThickness) / 4.0;
    let spargeWater: f64 = totalWater - mashWater;

    let finalOutputFloat: finalSpargeFloat = finalSpargeFloat {
        mashVolume: mashWater,
        spargeVolume: spargeWater,
        totalVolume: totalWater,
    };

    finalOutputFloat
}

pub fn waterSpargeFormatting(allInputs: &waterSpargeData, finalOutputFloat: finalSpargeFloat) -> spargeOutputStrings {
    let mut finalOutput: spargeOutputStrings = spargeOutputStrings {
        mashVolumeFinal: String::from(""),
        spargeVolumeFinal: String::from(""),
        totalVolumeFinal: String::from(""),
    };

    if allInputs.imperialOrMetric == imperialOrMetric::imperialGB {
        if finalOutputFloat.mashVolume.gallonsUSToGallonsGB() >= 0.995 && finalOutputFloat.mashVolume.gallonsUSToGallonsGB() < 1.005 {
            finalOutput.mashVolumeFinal = format!("{:.0} gallon", finalOutputFloat.mashVolume.gallonsUSToGallonsGB());
        } else if finalOutputFloat.mashVolume.gallonsUSToGallonsGB().fract() >= 0.995 || finalOutputFloat.mashVolume.gallonsUSToGallonsGB().fract() < 0.005 {
            finalOutput.mashVolumeFinal = format!("{:.0} gallons", finalOutputFloat.mashVolume.gallonsUSToGallonsGB());
        } else {
            finalOutput.mashVolumeFinal = format!("{:.2} gallons", finalOutputFloat.mashVolume.gallonsUSToGallonsGB());
        }

        if finalOutputFloat.spargeVolume.gallonsUSToGallonsGB() >= 0.995 && finalOutputFloat.spargeVolume.gallonsUSToGallonsGB() < 1.005 {
            finalOutput.spargeVolumeFinal = format!("{:.0} gallon", finalOutputFloat.spargeVolume.gallonsUSToGallonsGB());
        } else if finalOutputFloat.spargeVolume.gallonsUSToGallonsGB().fract() >= 0.995 || finalOutputFloat.spargeVolume.gallonsUSToGallonsGB().fract() < 0.005 {
            finalOutput.spargeVolumeFinal = format!("{:.0} gallons", finalOutputFloat.spargeVolume.gallonsUSToGallonsGB());
        } else {
            finalOutput.spargeVolumeFinal = format!("{:.2} gallons", finalOutputFloat.spargeVolume.gallonsUSToGallonsGB());
        }

        if finalOutputFloat.totalVolume.gallonsUSToGallonsGB() >= 0.995 && finalOutputFloat.totalVolume.gallonsUSToGallonsGB() < 1.005 {
            finalOutput.totalVolumeFinal = format!("{:.0} gallon", finalOutputFloat.totalVolume.gallonsUSToGallonsGB());
        } else if finalOutputFloat.totalVolume.gallonsUSToGallonsGB().fract() >= 0.995 || finalOutputFloat.totalVolume.gallonsUSToGallonsGB().fract() < 0.005 {
            finalOutput.totalVolumeFinal = format!("{:.0} gallons", finalOutputFloat.totalVolume.gallonsUSToGallonsGB());
        } else {
            finalOutput.totalVolumeFinal = format!("{:.2} gallons", finalOutputFloat.totalVolume.gallonsUSToGallonsGB());
        }

    } else if allInputs.imperialOrMetric == imperialOrMetric::imperialUS {
        if finalOutputFloat.mashVolume >= 0.995 && finalOutputFloat.mashVolume < 1.005 {
            finalOutput.mashVolumeFinal = format!("{:.0} gallon", finalOutputFloat.mashVolume);
        } else if finalOutputFloat.mashVolume.fract() >= 0.995 || finalOutputFloat.mashVolume.fract() < 0.005 {
            finalOutput.mashVolumeFinal = format!("{:.0} gallons", finalOutputFloat.mashVolume);
        } else {
            finalOutput.mashVolumeFinal = format!("{:.2} gallons", finalOutputFloat.mashVolume);
        }

        if finalOutputFloat.spargeVolume >= 0.995 && finalOutputFloat.spargeVolume < 1.005 {
            finalOutput.spargeVolumeFinal = format!("{:.0} gallon", finalOutputFloat.spargeVolume);
        } else if finalOutputFloat.spargeVolume.fract() >= 0.995 || finalOutputFloat.spargeVolume.fract() < 0.005 {
            finalOutput.spargeVolumeFinal = format!("{:.0} gallons", finalOutputFloat.spargeVolume);
        } else {
            finalOutput.spargeVolumeFinal = format!("{:.2} gallons", finalOutputFloat.spargeVolume);
        }

        if finalOutputFloat.totalVolume >= 0.995 && finalOutputFloat.totalVolume < 1.005 {
            finalOutput.totalVolumeFinal = format!("{:.0} gallon", finalOutputFloat.totalVolume);
        } else if finalOutputFloat.totalVolume.fract() >= 0.995 || finalOutputFloat.totalVolume.fract() < 0.005 {
            finalOutput.totalVolumeFinal = format!("{:.0} gallons", finalOutputFloat.totalVolume);
        } else {
            finalOutput.totalVolumeFinal = format!("{:.2} gallons", finalOutputFloat.totalVolume);
        }

    } else if allInputs.imperialOrMetric == imperialOrMetric::metric {
        if finalOutputFloat.mashVolume.gallonsUSToLitres() >= 0.995 && finalOutputFloat.mashVolume.gallonsUSToLitres() < 1.005 {
            finalOutput.mashVolumeFinal = format!("{:.0} litre", finalOutputFloat.mashVolume.gallonsUSToLitres());
        } else if finalOutputFloat.mashVolume.gallonsUSToLitres().fract() >= 0.995 || finalOutputFloat.mashVolume.gallonsUSToLitres().fract() < 0.005 {
            finalOutput.mashVolumeFinal = format!("{:.0} litres", finalOutputFloat.mashVolume.gallonsUSToLitres());
        } else {
            finalOutput.mashVolumeFinal = format!("{:.2} litres", finalOutputFloat.mashVolume.gallonsUSToLitres());
        }

        if finalOutputFloat.spargeVolume.gallonsUSToLitres() >= 0.995 && finalOutputFloat.spargeVolume.gallonsUSToLitres() < 1.005 {
            finalOutput.spargeVolumeFinal = format!("{:.0} litre", finalOutputFloat.spargeVolume.gallonsUSToLitres());
        } else if finalOutputFloat.spargeVolume.gallonsUSToLitres().fract() >= 0.995 || finalOutputFloat.spargeVolume.gallonsUSToLitres().fract() < 0.005 {
            finalOutput.spargeVolumeFinal = format!("{:.0} litres", finalOutputFloat.spargeVolume.gallonsUSToLitres());
        } else {
            finalOutput.spargeVolumeFinal = format!("{:.2} litres", finalOutputFloat.spargeVolume.gallonsUSToLitres());
        }

        if finalOutputFloat.totalVolume.gallonsUSToLitres() >= 0.995 && finalOutputFloat.totalVolume.gallonsUSToLitres()< 1.005 {
            finalOutput.totalVolumeFinal = format!("{:.0} litre", finalOutputFloat.totalVolume.gallonsUSToLitres());
        } else if finalOutputFloat.totalVolume.gallonsUSToLitres().fract() >= 0.995 || finalOutputFloat.totalVolume.gallonsUSToLitres().fract() < 0.005 {
            finalOutput.totalVolumeFinal = format!("{:.0} litres", finalOutputFloat.totalVolume.gallonsUSToLitres());
        } else {
            finalOutput.totalVolumeFinal = format!("{:.2} litres", finalOutputFloat.totalVolume.gallonsUSToLitres());
        }
    }

    finalOutput
}

fn waterSpargeOutput(allInputs: waterSpargeData, waterSpargeBuilder: &gtk::Builder) {
    let spargeMashWater: gtk::Entry = waterSpargeBuilder.get_object("spargeMashWater").expect("waterSpargeOutput(), spargeMashWater");
    let spargeSpargeWater: gtk::Entry = waterSpargeBuilder.get_object("spargeSpargeWater").expect("waterSpargeOutput(), spargeSpargeWater");
    let spargeTotalWater: gtk::Entry = waterSpargeBuilder.get_object("spargeTotalWater").expect("waterSpargeOutput(), spargeTotalWater");

    let finalOutputFloat: finalSpargeFloat = waterSpargeMaths(&allInputs);
    let finalOutput: spargeOutputStrings = waterSpargeFormatting(&allInputs, finalOutputFloat);

    spargeMashWater.set_text(&finalOutput.mashVolumeFinal);
    spargeSpargeWater.set_text(&finalOutput.spargeVolumeFinal);
    spargeTotalWater.set_text(&finalOutput.totalVolumeFinal);
}