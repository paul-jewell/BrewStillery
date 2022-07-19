use gtk;
use gtk::prelude::*;
use functions::commonFunctions::{imperialOrMetric, inputMatching, singleInput, realIBU};

#[derive(Debug)]
pub struct guestimateIBUData {
    pub preBoilBrix: f64,
    pub wortVolume: f64,
    pub firstHopAlpha: f64,
    pub firstHopAmount: f64,
    pub firstHopBoilTime: f64,
    pub secondHopAlpha: f64,
    pub secondHopAmount: f64,
    pub secondHopBoilTime: f64,
    pub thirdHopAlpha: f64,
    pub thirdHopAmount: f64,
    pub thirdHopBoilTime: f64,
    pub fourthHopAlpha: f64,
    pub fourthHopAmount: f64,
    pub fourthHopBoilTime: f64,
    pub fifthHopAlpha: f64,
    pub fifthHopAmount: f64,
    pub fifthHopBoilTime: f64,
    pub sixthHopAlpha: f64,
    pub sixthHopAmount: f64,
    pub sixthHopBoilTime: f64,
    pub seventhHopAlpha: f64,
    pub seventhHopAmount: f64,
    pub seventhHopBoilTime: f64,
    pub totalIBU1: f64,
    pub totalIBU2: f64,
    pub totalIBU3: f64,
    pub totalIBU4: f64,
    pub totalIBU5: f64,
    pub totalIBU6: f64,
    pub totalIBU7: f64,
    pub imperialOrMetric: imperialOrMetric,
}

pub fn guestimateIBUPrep(IBUBuilder: &gtk::Builder) {
    let totalIBUPreBoilBrix: gtk::Entry = IBUBuilder.get_object("totalIBUPreBoilBrix").expect("guestimateIBUPrep(), totalIBUPreBoilBrix");
    let totalIBUPreBoilBrixBuffer: String = totalIBUPreBoilBrix.get_text().expect("guestimateIBUPrep(), totalIBUPreBoilBrixBuffer");

    let totalIBUWortVolume: gtk::Entry = IBUBuilder.get_object("totalIBUWortVolume").expect("guestimateIBUPrep(), totalIBUWortVolume");
    let totalIBUWortVolumeBuffer: String = totalIBUWortVolume.get_text().expect("guestimateIBUPrep(), totalIBUWortVolumeBuffer");

    let totalIBUFirstHopAlpha: gtk::Entry = IBUBuilder.get_object("totalIBUFirstHopAlpha").expect("guestimateIBUPrep(), totalIBUFirstHopAlpha");
    let totalIBUFirstHopAlphaBuffer: String = totalIBUFirstHopAlpha.get_text().expect("guestimateIBUPrep(), totalIBUFirstHopAlphaBuffer");

    let totalIBUFirstHopAmount: gtk::Entry = IBUBuilder.get_object("totalIBUFirstHopAmount").expect("guestimateIBUPrep(), totalIBUFirstHopAmount");
    let totalIBUFirstHopAmountBuffer: String = totalIBUFirstHopAmount.get_text().expect("guestimateIBUPrep(), totalIBUFirstHopAmountBuffer");

    let totalIBUFirstHopBoilTime: gtk::Entry = IBUBuilder.get_object("totalIBUFirstHopBoilTime").expect("guestimateIBUPrep(), totalIBUFirstHopBoilTime");
    let totalIBUFirstHopBoilTimeBuffer: String = totalIBUFirstHopBoilTime.get_text().expect("guestimateIBUPrep(), totalIBUFirstHopBoilTimeBuffer");

    let totalIBUSecondHopAlpha: gtk::Entry = IBUBuilder.get_object("totalIBUSecondHopAlpha").expect("guestimateIBUPrep(), totalIBUSecondHopAlpha");
    let totalIBUSecondHopAlphaBuffer: String = totalIBUSecondHopAlpha.get_text().expect("guestimateIBUPrep(), totalIBUSecondHopAlphaBuffer");

    let totalIBUSecondHopAmount: gtk::Entry = IBUBuilder.get_object("totalIBUSecondHopAmount").expect("guestimateIBUPrep(), totalIBUSecondHopAmount");
    let totalIBUSecondHopAmountBuffer: String = totalIBUSecondHopAmount.get_text().expect("guestimateIBUPrep(), totalIBUSecondHopAmountBuffer");

    let totalIBUSecondHopBoilTime: gtk::Entry = IBUBuilder.get_object("totalIBUSecondHopBoilTime").expect("guestimateIBUPrep(), totalIBUSecondHopBoilTime");
    let totalIBUSecondHopBoilTimeBuffer: String = totalIBUSecondHopBoilTime.get_text().expect("guestimateIBUPrep(), totalIBUSecondHopBoilTimeBuffer");

    let totalIBUThirdHopAlpha: gtk::Entry = IBUBuilder.get_object("totalIBUThirdHopAlpha").expect("guestimateIBUPrep(), totalIBUThirdHopAlpha");
    let totalIBUThirdHopAlphaBuffer: String = totalIBUThirdHopAlpha.get_text().expect("guestimateIBUPrep(), totalIBUThirdHopAlphaBuffer");

    let totalIBUThirdHopAmount: gtk::Entry = IBUBuilder.get_object("totalIBUThirdHopAmount").expect("guestimateIBUPrep(), totalIBUThirdHopAmount");
    let totalIBUThirdHopAmountBuffer: String = totalIBUThirdHopAmount.get_text().expect("guestimateIBUPrep(), totalIBUThirdHopAmountBuffer");

    let totalIBUThirdHopBoilTime: gtk::Entry = IBUBuilder.get_object("totalIBUThirdHopBoilTime").expect("guestimateIBUPrep(), totalIBUThirdHopBoilTime");
    let totalIBUThirdHopBoilTimeBuffer: String = totalIBUThirdHopBoilTime.get_text().expect("guestimateIBUPrep(), totalIBUThirdHopBoilTimeBuffer");

    let totalIBUFourthHopAlpha: gtk::Entry = IBUBuilder.get_object("totalIBUFourthHopAlpha").expect("guestimateIBUPrep(), totalIBUFourthHopAlpha");
    let totalIBUFourthHopAlphaBuffer: String = totalIBUFourthHopAlpha.get_text().expect("guestimateIBUPrep(), totalIBUFourthHopAlphaBuffer");

    let totalIBUFourthHopAmount: gtk::Entry = IBUBuilder.get_object("totalIBUFourthHopAmount").expect("guestimateIBUPrep(), totalIBUFourthHopAmount");
    let totalIBUFourthHopAmountBuffer: String = totalIBUFourthHopAmount.get_text().expect("guestimateIBUPrep(), totalIBUFourthHopAmountBuffer");

    let totalIBUFourthHopBoilTime: gtk::Entry = IBUBuilder.get_object("totalIBUFourthHopBoilTime").expect("guestimateIBUPrep(), totalIBUFourthHopBoilTime");
    let totalIBUFourthHopBoilTimeBuffer: String = totalIBUFourthHopBoilTime.get_text().expect("guestimateIBUPrep(), totalIBUFourthHopBoilTimeBuffer");

    let totalIBUFifthHopAlpha: gtk::Entry = IBUBuilder.get_object("totalIBUFifthHopAlpha").expect("guestimateIBUPrep(), totalIBUFifthHopAlpha");
    let totalIBUFifthHopAlphaBuffer: String = totalIBUFifthHopAlpha.get_text().expect("guestimateIBUPrep(), totalIBUFifthHopAlphaBuffer");

    let totalIBUFifthHopAmount: gtk::Entry = IBUBuilder.get_object("totalIBUFifthHopAmount").expect("guestimateIBUPrep(), totalIBUFifthHopAmount");
    let totalIBUFifthHopAmountBuffer: String = totalIBUFifthHopAmount.get_text().expect("guestimateIBUPrep(), totalIBUFifthHopAmountBuffer");

    let totalIBUFifthHopBoilTime: gtk::Entry = IBUBuilder.get_object("totalIBUFifthHopBoilTime").expect("guestimateIBUPrep(), totalIBUFifthHopBoilTime");
    let totalIBUFifthHopBoilTimeBuffer: String = totalIBUFifthHopBoilTime.get_text().expect("guestimateIBUPrep(), totalIBUFifthHopBoilTimeBuffer");

    let totalIBUSixthHopAlpha: gtk::Entry = IBUBuilder.get_object("totalIBUSixthHopAlpha").expect("guestimateIBUPrep(), totalIBUSixthHopAlpha");
    let totalIBUSixthHopAlphaBuffer: String = totalIBUSixthHopAlpha.get_text().expect("guestimateIBUPrep(), totalIBUSixthHopAlphaBuffer");

    let totalIBUSixthHopAmount: gtk::Entry = IBUBuilder.get_object("totalIBUSixthHopAmount").expect("guestimateIBUPrep(), totalIBUSixthHopAmount");
    let totalIBUSixthHopAmountBuffer: String = totalIBUSixthHopAmount.get_text().expect("guestimateIBUPrep(), totalIBUSixthHopAmountBuffer");

    let totalIBUSixthHopBoilTime: gtk::Entry = IBUBuilder.get_object("totalIBUSixthHopBoilTime").expect("guestimateIBUPrep(), totalIBUSixthHopBoilTime");
    let totalIBUSixthHopBoilTimeBuffer: String = totalIBUSixthHopBoilTime.get_text().expect("guestimateIBUPrep(), totalIBUSixthHopBoilTimeBuffer");

    let totalIBUSeventhHopAlpha: gtk::Entry = IBUBuilder.get_object("totalIBUSeventhHopAlpha").expect("guestimateIBUPrep(), totalIBUSeventhHopAlpha");
    let totalIBUSeventhHopAlphaBuffer: String = totalIBUSeventhHopAlpha.get_text().expect("guestimateIBUPrep(), totalIBUSeventhHopAlphaBuffer");

    let totalIBUSeventhHopAmount: gtk::Entry = IBUBuilder.get_object("totalIBUSeventhHopAmount").expect("guestimateIBUPrep(), totalIBUSeventhHopAmount");
    let totalIBUSeventhHopAmountBuffer: String = totalIBUSeventhHopAmount.get_text().expect("guestimateIBUPrep(), totalIBUSeventhHopAmountBuffer");

    let totalIBUSeventhHopBoilTime: gtk::Entry = IBUBuilder.get_object("totalIBUSeventhHopBoilTime").expect("guestimateIBUPrep(), totalIBUSeventhHopBoilTime");
    let totalIBUSeventhHopBoilTimeBuffer: String = totalIBUSeventhHopBoilTime.get_text().expect("guestimateIBUPrep(), totalIBUSeventhHopBoilTimeBuffer");

    let totalIBUUnits: gtk::ComboBoxText = IBUBuilder.get_object("totalIBUUnits").expect("guestimateIBUPrep(), totalIBUUnits");
    let totalIBUUnitsBuffer: String = totalIBUUnits.get_active_id().expect("guestimateIBUPrep(), totalIBUUnitsBuffer");

    let allInputs: guestimateIBUData = guestimateIBUData {
        preBoilBrix: totalIBUPreBoilBrixBuffer.validInput(),
        wortVolume: totalIBUWortVolumeBuffer.validInput(),
        firstHopAlpha: totalIBUFirstHopAlphaBuffer.validInput(),
        firstHopAmount: totalIBUFirstHopAmountBuffer.validInput(),
        firstHopBoilTime: totalIBUFirstHopBoilTimeBuffer.validInput(),
        secondHopAlpha: totalIBUSecondHopAlphaBuffer.validInput(),
        secondHopAmount: totalIBUSecondHopAmountBuffer.validInput(),
        secondHopBoilTime: totalIBUSecondHopBoilTimeBuffer.validInput(),
        thirdHopAlpha: totalIBUThirdHopAlphaBuffer.validInput(),
        thirdHopAmount: totalIBUThirdHopAmountBuffer.validInput(),
        thirdHopBoilTime: totalIBUThirdHopBoilTimeBuffer.validInput(),
        fourthHopAlpha: totalIBUFourthHopAlphaBuffer.validInput(),
        fourthHopAmount: totalIBUFourthHopAmountBuffer.validInput(),
        fourthHopBoilTime: totalIBUFourthHopBoilTimeBuffer.validInput(),
        fifthHopAlpha: totalIBUFifthHopAlphaBuffer.validInput(),
        fifthHopAmount: totalIBUFifthHopAmountBuffer.validInput(),
        fifthHopBoilTime: totalIBUFifthHopBoilTimeBuffer.validInput(),
        sixthHopAlpha: totalIBUSixthHopAlphaBuffer.validInput(),
        sixthHopAmount: totalIBUSixthHopAmountBuffer.validInput(),
        sixthHopBoilTime: totalIBUSixthHopBoilTimeBuffer.validInput(),
        seventhHopAlpha: totalIBUSeventhHopAlphaBuffer.validInput(),
        seventhHopAmount: totalIBUSeventhHopAmountBuffer.validInput(),
        seventhHopBoilTime: totalIBUSeventhHopBoilTimeBuffer.validInput(),
        totalIBU1: 0.0,
        totalIBU2: 0.0,
        totalIBU3: 0.0,
        totalIBU4: 0.0,
        totalIBU5: 0.0,
        totalIBU6: 0.0,
        totalIBU7: 0.0,
        imperialOrMetric: totalIBUUnitsBuffer.unitMatch(),
    };

    let temporaryIBUOutput: gtk::Entry = IBUBuilder.get_object("totalIBUFinal").expect("guestimateIBUPrep(), temporaryIBUOutput");

    if allInputs.preBoilBrix < 2.57 {
        temporaryIBUOutput.set_text("Enter a Pre-Boil Brix greater than 2.57");
    } else if allInputs.preBoilBrix > 49.48 {
        temporaryIBUOutput.set_text("Enter a Pre-Boil Brix less than 49.48");
    } else if allInputs.preBoilBrix.is_nan() || allInputs.wortVolume.is_nan() || allInputs.firstHopAlpha.is_nan() || allInputs.firstHopAmount.is_nan() || allInputs.firstHopBoilTime.is_nan() {
        temporaryIBUOutput.set_text("Enter at least the first 5 inputs");
    } else if allInputs.preBoilBrix <= 0.0 || allInputs.wortVolume <= 0.0 || allInputs.firstHopAlpha <= 0.0 || allInputs.firstHopAmount <= 0.0 || allInputs.firstHopBoilTime <= 0.0 {
        temporaryIBUOutput.set_text("Enter a positive number");
    } else {
        totalIBUOutput(allInputs, IBUBuilder);
    }
}

pub fn totalIBUMaths(mut allInputs: guestimateIBUData) -> f64 {
    let mut finalOutputFloat: f64 = 0.0;

    if allInputs.imperialOrMetric == imperialOrMetric::imperialGB {
        allInputs.totalIBU1 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume.gallonsGBToGallonsUS(), allInputs.firstHopAlpha, allInputs.firstHopAmount, allInputs.firstHopBoilTime);
        allInputs.totalIBU2 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume.gallonsGBToGallonsUS(), allInputs.secondHopAlpha, allInputs.secondHopAmount, allInputs.secondHopBoilTime);
        allInputs.totalIBU3 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume.gallonsGBToGallonsUS(), allInputs.thirdHopAlpha, allInputs.thirdHopAmount, allInputs.thirdHopBoilTime);
        allInputs.totalIBU4 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume.gallonsGBToGallonsUS(), allInputs.fourthHopAlpha, allInputs.fourthHopAmount, allInputs.fourthHopBoilTime);
        allInputs.totalIBU5 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume.gallonsGBToGallonsUS(), allInputs.fifthHopAlpha, allInputs.fifthHopAmount, allInputs.fifthHopBoilTime);
        allInputs.totalIBU6 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume.gallonsGBToGallonsUS(), allInputs.sixthHopAlpha, allInputs.sixthHopAmount, allInputs.sixthHopBoilTime);
        allInputs.totalIBU7 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume.gallonsGBToGallonsUS(), allInputs.seventhHopAlpha, allInputs.seventhHopAmount, allInputs.seventhHopBoilTime);
        if allInputs.totalIBU2.is_nan() { allInputs.totalIBU2 = 0.0; }
        if allInputs.totalIBU3.is_nan() { allInputs.totalIBU3 = 0.0; }
        if allInputs.totalIBU4.is_nan() { allInputs.totalIBU4 = 0.0; }
        if allInputs.totalIBU5.is_nan() { allInputs.totalIBU5 = 0.0; }
        if allInputs.totalIBU6.is_nan() { allInputs.totalIBU6 = 0.0; }
        if allInputs.totalIBU7.is_nan() { allInputs.totalIBU7 = 0.0; }
        finalOutputFloat = allInputs.totalIBU1 + allInputs.totalIBU2 + allInputs.totalIBU3 + allInputs.totalIBU4 + allInputs.totalIBU5 + allInputs.totalIBU6 + allInputs.totalIBU7;
    } else if allInputs.imperialOrMetric == imperialOrMetric::imperialUS {
        allInputs.totalIBU1 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume, allInputs.firstHopAlpha, allInputs.firstHopAmount, allInputs.firstHopBoilTime);
        allInputs.totalIBU2 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume, allInputs.secondHopAlpha, allInputs.secondHopAmount, allInputs.secondHopBoilTime);
        allInputs.totalIBU3 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume, allInputs.thirdHopAlpha, allInputs.thirdHopAmount, allInputs.thirdHopBoilTime);
        allInputs.totalIBU4 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume, allInputs.fourthHopAlpha, allInputs.fourthHopAmount, allInputs.fourthHopBoilTime);
        allInputs.totalIBU5 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume, allInputs.fifthHopAlpha, allInputs.fifthHopAmount, allInputs.fifthHopBoilTime);
        allInputs.totalIBU6 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume, allInputs.sixthHopAlpha, allInputs.sixthHopAmount, allInputs.sixthHopBoilTime);
        allInputs.totalIBU7 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume, allInputs.seventhHopAlpha, allInputs.seventhHopAmount, allInputs.seventhHopBoilTime);
        if allInputs.totalIBU2.is_nan() { allInputs.totalIBU2 = 0.0; }
        if allInputs.totalIBU3.is_nan() { allInputs.totalIBU3 = 0.0; }
        if allInputs.totalIBU4.is_nan() { allInputs.totalIBU4 = 0.0; }
        if allInputs.totalIBU5.is_nan() { allInputs.totalIBU5 = 0.0; }
        if allInputs.totalIBU6.is_nan() { allInputs.totalIBU6 = 0.0; }
        if allInputs.totalIBU7.is_nan() { allInputs.totalIBU7 = 0.0; }
        finalOutputFloat = allInputs.totalIBU1 + allInputs.totalIBU2 + allInputs.totalIBU3 + allInputs.totalIBU4 + allInputs.totalIBU5 + allInputs.totalIBU6 + allInputs.totalIBU7;
    } else if allInputs.imperialOrMetric == imperialOrMetric::metric {
        allInputs.totalIBU1 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume.litresToGallonsUS(), allInputs.firstHopAlpha, allInputs.firstHopAmount.gramsToOunces(), allInputs.firstHopBoilTime);
        allInputs.totalIBU2 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume.litresToGallonsUS(), allInputs.secondHopAlpha, allInputs.secondHopAmount.gramsToOunces(), allInputs.secondHopBoilTime);
        allInputs.totalIBU3 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume.litresToGallonsUS(), allInputs.thirdHopAlpha, allInputs.thirdHopAmount.gramsToOunces(), allInputs.thirdHopBoilTime);
        allInputs.totalIBU4 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume.litresToGallonsUS(), allInputs.fourthHopAlpha, allInputs.fourthHopAmount.gramsToOunces(), allInputs.fourthHopBoilTime);
        allInputs.totalIBU5 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume.litresToGallonsUS(), allInputs.fifthHopAlpha, allInputs.fifthHopAmount.gramsToOunces(), allInputs.fifthHopBoilTime);
        allInputs.totalIBU6 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume.litresToGallonsUS(), allInputs.sixthHopAlpha, allInputs.sixthHopAmount.gramsToOunces(), allInputs.sixthHopBoilTime);
        allInputs.totalIBU7 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume.litresToGallonsUS(), allInputs.seventhHopAlpha, allInputs.seventhHopAmount.gramsToOunces(), allInputs.seventhHopBoilTime);
        if allInputs.totalIBU2.is_nan() { allInputs.totalIBU2 = 0.0; }
        if allInputs.totalIBU3.is_nan() { allInputs.totalIBU3 = 0.0; }
        if allInputs.totalIBU4.is_nan() { allInputs.totalIBU4 = 0.0; }
        if allInputs.totalIBU5.is_nan() { allInputs.totalIBU5 = 0.0; }
        if allInputs.totalIBU6.is_nan() { allInputs.totalIBU6 = 0.0; }
        if allInputs.totalIBU7.is_nan() { allInputs.totalIBU7 = 0.0; }
        finalOutputFloat = allInputs.totalIBU1 + allInputs.totalIBU2 + allInputs.totalIBU3 + allInputs.totalIBU4 + allInputs.totalIBU5 + allInputs.totalIBU6 + allInputs.totalIBU7;
    }

    finalOutputFloat
}

pub fn totalIBUFormatting(finalOutputFloat: f64) -> String {
    let finalOutput: String;

    if finalOutputFloat >= 0.995 && finalOutputFloat < 1.005 {
        finalOutput = format!("{:.0} IBU", finalOutputFloat);
    } else if finalOutputFloat.fract() >= 0.995 || finalOutputFloat.fract() < 0.005 {
        finalOutput = format!("{:.0} IBUs", finalOutputFloat);
    } else {
        finalOutput = format!("{:.2} IBUs", finalOutputFloat);
    }

    finalOutput
}

fn totalIBUOutput(allInputs: guestimateIBUData, IBUBuilder: &gtk::Builder) {
    let totalIBUFinal: gtk::Entry = IBUBuilder.get_object("totalIBUFinal").expect("guestimateIBUPrep(), totalIBUFinal");

    let finalOutputFloat: f64 = totalIBUMaths(allInputs);
    let finalOutput: String = totalIBUFormatting(finalOutputFloat);

    totalIBUFinal.set_text(&finalOutput);
}