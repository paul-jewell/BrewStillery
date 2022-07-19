use gtk;
use gtk::prelude::*;
use gdk::RGBA;
use functions::commonFunctions::{imperialOrMetric, inputMatching, singleInput, zeroRGBA, grainToGravity, FINAL_BRIX_IDEAL, realABVAndAttenuation};
use light::lightFunctions::{singleMCU, beerSRM, grainSRMToLAB, grainLABToXYZ, grainXYZToRGBA};
use functions::commonFunctions::colourOverlay;

#[derive(Debug)]
pub struct grainABVData {
    pub wortAmount: f64,
    pub firstGrainAmount: f64,
    pub firstGrainGravity: f64,
    pub firstGrainLovibond: f64,
    pub secondGrainAmount: f64,
    pub secondGrainGravity: f64,
    pub secondGrainLovibond: f64,
    pub thirdGrainAmount: f64,
    pub thirdGrainGravity: f64,
    pub thirdGrainLovibond: f64,
    pub fourthGrainAmount: f64,
    pub fourthGrainGravity: f64,
    pub fourthGrainLovibond: f64,
    pub fifthGrainAmount: f64,
    pub fifthGrainGravity: f64,
    pub fifthGrainLovibond: f64,
    pub sixthGrainAmount: f64,
    pub sixthGrainGravity: f64,
    pub sixthGrainLovibond: f64,
    pub seventhGrainAmount: f64,
    pub seventhGrainGravity: f64,
    pub seventhGrainLovibond: f64,
    pub totalGrain1: f64,
    pub totalGrain2: f64,
    pub totalGrain3: f64,
    pub totalGrain4: f64,
    pub totalGrain5: f64,
    pub totalGrain6: f64,
    pub totalGrain7: f64,
    pub glassSize: f64,
    pub abvFinal: f64,
    pub startingBrix: f64,
    pub estimatedBrixFormatted: String,
    pub abvFormatted: String,
    pub imperialOrMetric: imperialOrMetric,
}

pub fn grainToABVPrep(allOverlays: &colourOverlay, grainABVBuilder: &gtk::Builder) {
    let grainABVWortVolume: gtk::Entry = grainABVBuilder.get_object("grainABVWortVolume").expect("grainToABVPrep(), grainABVWortVolume");
    let grainABVWortVolumeBuffer: String = grainABVWortVolume.get_text().expect("grainToABVPrep(), grainABVWortVolumeBuffer");

    let grainABVFirstAmount: gtk::Entry = grainABVBuilder.get_object("grainABVFirstAmount").expect("grainToABVPrep(), grainABVFirstAmount");
    let grainABVFirstAmountBuffer: String = grainABVFirstAmount.get_text().expect("grainToABVPrep(), grainABVFirstAmountBuffer");

    let grainABVFirstType: gtk::ComboBoxText = grainABVBuilder.get_object("grainABVFirstType").expect("grainToABVPrep(), grainABVFirstType");
    let grainABVFirstTypeBuffer: String = grainABVFirstType.get_active_id().expect("grainToABVPrep(), grainABVFirstTypeBuffer");

    let grainABVSecondAmount: gtk::Entry = grainABVBuilder.get_object("grainABVSecondAmount").expect("grainToABVPrep(), grainABVSecondAmount");
    let grainABVSecondAmountBuffer: String = grainABVSecondAmount.get_text().expect("grainToABVPrep(), grainABVSecondAmountBuffer");

    let grainABVSecondType: gtk::ComboBoxText = grainABVBuilder.get_object("grainABVSecondType").expect("grainToABVPrep(), grainABVSecondType");
    let grainABVSecondTypeBuffer: String = grainABVSecondType.get_active_id().expect("grainToABVPrep(), grainABVSecondTypeBuffer");

    let grainABVThirdAmount: gtk::Entry = grainABVBuilder.get_object("grainABVThirdAmount").expect("grainToABVPrep(), grainABVThirdAmount");
    let grainABVThirdAmountBuffer: String = grainABVThirdAmount.get_text().expect("grainToABVPrep(), grainABVThirdAmountBuffer");

    let grainABVThirdType: gtk::ComboBoxText = grainABVBuilder.get_object("grainABVThirdType").expect("grainToABVPrep(), grainABVThirdType");
    let grainABVThirdTypeBuffer: String = grainABVThirdType.get_active_id().expect("grainToABVPrep(), grainABVThirdTypeBuffer");

    let grainABVFourthAmount: gtk::Entry = grainABVBuilder.get_object("grainABVFourthAmount").expect("grainToABVPrep(), grainABVFourthAmount");
    let grainABVFourthAmountBuffer: String = grainABVFourthAmount.get_text().expect("grainToABVPrep(), grainABVFourthAmountBuffer");

    let grainABVFourthType: gtk::ComboBoxText = grainABVBuilder.get_object("grainABVFourthType").expect("grainToABVPrep(), grainABVFourthType");
    let grainABVFourthTypeBuffer: String = grainABVFourthType.get_active_id().expect("grainToABVPrep(), grainABVFourthTypeBuffer");

    let grainABVFifthAmount: gtk::Entry = grainABVBuilder.get_object("grainABVFifthAmount").expect("grainToABVPrep(), grainABVFifthAmount");
    let grainABVFifthAmountBuffer: String = grainABVFifthAmount.get_text().expect("grainToABVPrep(), grainABVFifthAmountBuffer");

    let grainABVFifthType: gtk::ComboBoxText = grainABVBuilder.get_object("grainABVFifthType").expect("grainToABVPrep(), grainABVFifthType");
    let grainABVFifthTypeBuffer: String = grainABVFifthType.get_active_id().expect("grainToABVPrep(), grainABVFifthTypeBuffer");

    let grainABVSixthAmount: gtk::Entry = grainABVBuilder.get_object("grainABVSixthAmount").expect("grainToABVPrep(), grainABVSixthAmount");
    let grainABVSixthAmountBuffer: String = grainABVSixthAmount.get_text().expect("grainToABVPrep(), grainABVSixthAmountBuffer");

    let grainABVSixthType: gtk::ComboBoxText = grainABVBuilder.get_object("grainABVSixthType").expect("grainToABVPrep(), grainABVSixthType");
    let grainABVSixthTypeBuffer: String = grainABVSixthType.get_active_id().expect("grainToABVPrep(), grainABVSixthTypeBuffer");

    let grainABVSeventhAmount: gtk::Entry = grainABVBuilder.get_object("grainABVSeventhAmount").expect("grainToABVPrep(), grainABVSeventhAmount");
    let grainABVSeventhAmountBuffer: String = grainABVSeventhAmount.get_text().expect("grainToABVPrep(), grainABVSeventhAmountBuffer");

    let grainABVSeventhType: gtk::ComboBoxText = grainABVBuilder.get_object("grainABVSeventhType").expect("grainToABVPrep(), grainABVSeventhType");
    let grainABVSeventhTypeBuffer: String = grainABVSeventhType.get_active_id().expect("grainToABVPrep(), grainABVSeventhTypeBuffer");

    let grainABVGlassSize: gtk::ComboBoxText = grainABVBuilder.get_object("grainABVGlassSize").expect("grainToABVPrep(), grainABVGlassSize");
    let grainABVGlassSizeBuffer: String = grainABVGlassSize.get_active_id().expect("grainToABVPrep(), grainABVGlassSizeBuffer");

    let grainABVUnits: gtk::ComboBoxText = grainABVBuilder.get_object("grainABVUnits").expect("grainToABVPrep(), grainABVUnits");
    let grainABVUnitsBuffer: String = grainABVUnits.get_active_id().expect("grainToABVPrep(), grainABVUnitsBuffer");

    let allInputs: grainABVData = grainABVData {
        wortAmount: grainABVWortVolumeBuffer.validInput(),
        firstGrainAmount: grainABVFirstAmountBuffer.validInput(),
        firstGrainGravity: grainABVFirstTypeBuffer.grainGravity(),
        firstGrainLovibond: grainABVFirstTypeBuffer.grainLovibond(),
        secondGrainAmount: grainABVSecondAmountBuffer.validInput(),
        secondGrainGravity: grainABVSecondTypeBuffer.grainGravity(),
        secondGrainLovibond: grainABVSecondTypeBuffer.grainLovibond(),
        thirdGrainAmount: grainABVThirdAmountBuffer.validInput(),
        thirdGrainGravity: grainABVThirdTypeBuffer.grainGravity(),
        thirdGrainLovibond: grainABVThirdTypeBuffer.grainLovibond(),
        fourthGrainAmount: grainABVFourthAmountBuffer.validInput(),
        fourthGrainGravity: grainABVFourthTypeBuffer.grainGravity(),
        fourthGrainLovibond: grainABVFourthTypeBuffer.grainLovibond(),
        fifthGrainAmount: grainABVFifthAmountBuffer.validInput(),
        fifthGrainGravity: grainABVFifthTypeBuffer.grainGravity(),
        fifthGrainLovibond: grainABVFifthTypeBuffer.grainLovibond(),
        sixthGrainAmount: grainABVSixthAmountBuffer.validInput(),
        sixthGrainGravity: grainABVSixthTypeBuffer.grainGravity(),
        sixthGrainLovibond: grainABVSixthTypeBuffer.grainLovibond(),
        seventhGrainAmount: grainABVSeventhAmountBuffer.validInput(),
        seventhGrainGravity: grainABVSeventhTypeBuffer.grainGravity(),
        seventhGrainLovibond: grainABVSeventhTypeBuffer.grainLovibond(),
        totalGrain1: 0.0,
        totalGrain2: 0.0,
        totalGrain3: 0.0,
        totalGrain4: 0.0,
        totalGrain5: 0.0,
        totalGrain6: 0.0,
        totalGrain7: 0.0,
        glassSize: grainABVGlassSizeBuffer.glassSize(),
        abvFinal: 0.0,
        startingBrix: 0.0,
        estimatedBrixFormatted: String::from("Enter legimate amounts"),
        abvFormatted: String::from(""),
        imperialOrMetric: grainABVUnitsBuffer.unitMatch(),
    };

    let grainABVFinalBrix: gtk::Entry = grainABVBuilder.get_object("grainABVFinalBrix").expect("grainToABVPrep(), grainABVFinalBrix");
    let grainABVFinalABV: gtk::Entry = grainABVBuilder.get_object("grainABVFinalABV").expect("grainToABVPrep(), grainABVFinalABV");


    if grainABVGlassSizeBuffer == "Dimple Pint" {
        allOverlays.dimplePint.set_opacity(1.0);
        allOverlays.nonickPint.set_opacity(0.0);
        allOverlays.tulipPint.set_opacity(0.0);
        allOverlays.pilsner.set_opacity(0.0);
        allOverlays.mafs.set_opacity(0.0);
        allOverlays.dimpleHalfPint.set_opacity(0.0);
        allOverlays.nonickHalfPint.set_opacity(0.0);
        allOverlays.tulipHalfPint.set_opacity(0.0);
    } else if grainABVGlassSizeBuffer == "Nonick Pint"{
        allOverlays.dimplePint.set_opacity(0.0);
        allOverlays.nonickPint.set_opacity(1.0);
        allOverlays.tulipPint.set_opacity(0.0);
        allOverlays.pilsner.set_opacity(0.0);
        allOverlays.mafs.set_opacity(0.0);
        allOverlays.dimpleHalfPint.set_opacity(0.0);
        allOverlays.nonickHalfPint.set_opacity(0.0);
        allOverlays.tulipHalfPint.set_opacity(0.0);
    } else if grainABVGlassSizeBuffer == "Tulip Pint" {
        allOverlays.dimplePint.set_opacity(0.0);
        allOverlays.nonickPint.set_opacity(0.0);
        allOverlays.tulipPint.set_opacity(1.0);
        allOverlays.pilsner.set_opacity(0.0);
        allOverlays.mafs.set_opacity(0.0);
        allOverlays.dimpleHalfPint.set_opacity(0.0);
        allOverlays.nonickHalfPint.set_opacity(0.0);
        allOverlays.tulipHalfPint.set_opacity(0.0);
    } else if grainABVGlassSizeBuffer == "Pilsner" {
        allOverlays.dimplePint.set_opacity(0.0);
        allOverlays.nonickPint.set_opacity(0.0);
        allOverlays.tulipPint.set_opacity(0.0);
        allOverlays.pilsner.set_opacity(1.0);
        allOverlays.mafs.set_opacity(0.0);
        allOverlays.dimpleHalfPint.set_opacity(0.0);
        allOverlays.nonickHalfPint.set_opacity(0.0);
        allOverlays.tulipHalfPint.set_opacity(0.0);
    } else if grainABVGlassSizeBuffer ==  "Maß" {
        allOverlays.dimplePint.set_opacity(0.0);
        allOverlays.nonickPint.set_opacity(0.0);
        allOverlays.tulipPint.set_opacity(0.0);
        allOverlays.pilsner.set_opacity(0.0);
        allOverlays.mafs.set_opacity(1.0);
        allOverlays.dimpleHalfPint.set_opacity(0.0);
        allOverlays.nonickHalfPint.set_opacity(0.0);
        allOverlays.tulipHalfPint.set_opacity(0.0);
    } else if grainABVGlassSizeBuffer == "Dimple Half Pint" {
        allOverlays.dimplePint.set_opacity(0.0);
        allOverlays.nonickPint.set_opacity(0.0);
        allOverlays.tulipPint.set_opacity(0.0);
        allOverlays.pilsner.set_opacity(0.0);
        allOverlays.mafs.set_opacity(0.0);
        allOverlays.dimpleHalfPint.set_opacity(1.0);
        allOverlays.nonickHalfPint.set_opacity(0.0);
        allOverlays.tulipHalfPint.set_opacity(0.0);
    } else if grainABVGlassSizeBuffer == "Nonick Half Pint" {
        allOverlays.dimplePint.set_opacity(0.0);
        allOverlays.nonickPint.set_opacity(0.0);
        allOverlays.tulipPint.set_opacity(0.0);
        allOverlays.pilsner.set_opacity(0.0);
        allOverlays.mafs.set_opacity(0.0);
        allOverlays.dimpleHalfPint.set_opacity(0.0);
        allOverlays.nonickHalfPint.set_opacity(1.0);
        allOverlays.tulipHalfPint.set_opacity(0.0);
    } else if grainABVGlassSizeBuffer == "Tulip Half Pint" {
        allOverlays.dimplePint.set_opacity(0.0);
        allOverlays.nonickPint.set_opacity(0.0);
        allOverlays.tulipPint.set_opacity(0.0);
        allOverlays.pilsner.set_opacity(0.0);
        allOverlays.mafs.set_opacity(0.0);
        allOverlays.dimpleHalfPint.set_opacity(0.0);
        allOverlays.nonickHalfPint.set_opacity(0.0);
        allOverlays.tulipHalfPint.set_opacity(1.0);
    }

    if allInputs.wortAmount.is_nan() {
        grainABVFinalBrix.set_text("Enter a Wort Volume");
        grainABVFinalABV.set_text("");
        allOverlays.colourOutput.set_rgba(&zeroRGBA());
    } else if allInputs.wortAmount <= 0.0 || allInputs.firstGrainAmount <= 0.0 {
        grainABVFinalBrix.set_text("Enter a positive number");
        grainABVFinalABV.set_text("");
        allOverlays.colourOutput.set_rgba(&zeroRGBA());
    } else if allInputs.firstGrainAmount.is_nan() {
        grainABVFinalBrix.set_text("Enter at least");
        grainABVFinalABV.set_text("1 grain amount");
        allOverlays.colourOutput.set_rgba(&zeroRGBA());
    } else {
        grainToABVOutput(allInputs, allOverlays, grainABVBuilder);
    }
}

pub fn grainToABVMaths(allInputs: &mut grainABVData) -> &mut grainABVData {
    if allInputs.imperialOrMetric == imperialOrMetric::imperialGB {
        allInputs.totalGrain1 = grainToGravity(allInputs.wortAmount.gallonsGBToGallonsUS(), allInputs.firstGrainAmount, allInputs.firstGrainGravity);
        allInputs.totalGrain2 = grainToGravity(allInputs.wortAmount.gallonsGBToGallonsUS(), allInputs.secondGrainAmount, allInputs.secondGrainGravity);
        allInputs.totalGrain3 = grainToGravity(allInputs.wortAmount.gallonsGBToGallonsUS(), allInputs.thirdGrainAmount, allInputs.thirdGrainGravity);
        allInputs.totalGrain4 = grainToGravity(allInputs.wortAmount.gallonsGBToGallonsUS(), allInputs.fourthGrainAmount, allInputs.fourthGrainGravity);
        allInputs.totalGrain5 = grainToGravity(allInputs.wortAmount.gallonsGBToGallonsUS(), allInputs.fifthGrainAmount, allInputs.fifthGrainGravity);
        allInputs.totalGrain6 = grainToGravity(allInputs.wortAmount.gallonsGBToGallonsUS(), allInputs.sixthGrainAmount, allInputs.sixthGrainGravity);
        allInputs.totalGrain7 = grainToGravity(allInputs.wortAmount.gallonsGBToGallonsUS(), allInputs.seventhGrainAmount, allInputs.seventhGrainGravity);
        if allInputs.totalGrain2.is_nan() { allInputs.totalGrain2 = 0.0; }
        if allInputs.totalGrain3.is_nan() { allInputs.totalGrain3 = 0.0; }
        if allInputs.totalGrain4.is_nan() { allInputs.totalGrain4 = 0.0; }
        if allInputs.totalGrain5.is_nan() { allInputs.totalGrain5 = 0.0; }
        if allInputs.totalGrain6.is_nan() { allInputs.totalGrain6 = 0.0; }
        if allInputs.totalGrain7.is_nan() { allInputs.totalGrain7 = 0.0; }
        let grainSum: f64 = 1.0 + allInputs.totalGrain1.fract() + allInputs.totalGrain2.fract() + allInputs.totalGrain3.fract() + allInputs.totalGrain4.fract() + allInputs.totalGrain5.fract() + allInputs.totalGrain6.fract() + allInputs.totalGrain7.fract();
        allInputs.startingBrix = grainSum.gravityToBrix();
        allInputs.abvFinal = realABVAndAttenuation(allInputs.startingBrix, FINAL_BRIX_IDEAL).0;
    } else if allInputs.imperialOrMetric == imperialOrMetric::imperialUS {
        allInputs.totalGrain1 = grainToGravity(allInputs.wortAmount, allInputs.firstGrainAmount, allInputs.firstGrainGravity);
        allInputs.totalGrain2 = grainToGravity(allInputs.wortAmount, allInputs.secondGrainAmount, allInputs.secondGrainGravity);
        allInputs.totalGrain3 = grainToGravity(allInputs.wortAmount, allInputs.thirdGrainAmount, allInputs.thirdGrainGravity);
        allInputs.totalGrain4 = grainToGravity(allInputs.wortAmount, allInputs.fourthGrainAmount, allInputs.fourthGrainGravity);
        allInputs.totalGrain5 = grainToGravity(allInputs.wortAmount, allInputs.fifthGrainAmount, allInputs.fifthGrainGravity);
        allInputs.totalGrain6 = grainToGravity(allInputs.wortAmount, allInputs.sixthGrainAmount, allInputs.sixthGrainGravity);
        allInputs.totalGrain7 = grainToGravity(allInputs.wortAmount, allInputs.seventhGrainAmount, allInputs.seventhGrainGravity);
        if allInputs.totalGrain2.is_nan() { allInputs.totalGrain2 = 0.0; }
        if allInputs.totalGrain3.is_nan() { allInputs.totalGrain3 = 0.0; }
        if allInputs.totalGrain4.is_nan() { allInputs.totalGrain4 = 0.0; }
        if allInputs.totalGrain5.is_nan() { allInputs.totalGrain5 = 0.0; }
        if allInputs.totalGrain6.is_nan() { allInputs.totalGrain6 = 0.0; }
        if allInputs.totalGrain7.is_nan() { allInputs.totalGrain7 = 0.0; }
        let grainSum: f64 = 1.0 + allInputs.totalGrain1.fract() + allInputs.totalGrain2.fract() + allInputs.totalGrain3.fract() + allInputs.totalGrain4.fract() + allInputs.totalGrain5.fract() + allInputs.totalGrain6.fract() + allInputs.totalGrain7.fract();
        allInputs.startingBrix = grainSum.gravityToBrix();
        allInputs.abvFinal = realABVAndAttenuation(allInputs.startingBrix, FINAL_BRIX_IDEAL).0;
    } else if allInputs.imperialOrMetric == imperialOrMetric::metric {
        allInputs.totalGrain1 = grainToGravity(allInputs.wortAmount.litresToGallonsUS(), allInputs.firstGrainAmount.kilosToPounds(), allInputs.firstGrainGravity);
        allInputs.totalGrain2 = grainToGravity(allInputs.wortAmount.litresToGallonsUS(), allInputs.secondGrainAmount.kilosToPounds(), allInputs.secondGrainGravity);
        allInputs.totalGrain3 = grainToGravity(allInputs.wortAmount.litresToGallonsUS(), allInputs.thirdGrainAmount.kilosToPounds(), allInputs.thirdGrainGravity);
        allInputs.totalGrain4 = grainToGravity(allInputs.wortAmount.litresToGallonsUS(), allInputs.fourthGrainAmount.kilosToPounds(), allInputs.fourthGrainGravity);
        allInputs.totalGrain5 = grainToGravity(allInputs.wortAmount.litresToGallonsUS(), allInputs.fifthGrainAmount.kilosToPounds(), allInputs.fifthGrainGravity);
        allInputs.totalGrain6 = grainToGravity(allInputs.wortAmount.litresToGallonsUS(), allInputs.sixthGrainAmount.kilosToPounds(), allInputs.sixthGrainGravity);
        allInputs.totalGrain7 = grainToGravity(allInputs.wortAmount.litresToGallonsUS(), allInputs.seventhGrainAmount.kilosToPounds(), allInputs.seventhGrainGravity);
        if allInputs.totalGrain2.is_nan() { allInputs.totalGrain2 = 0.0; }
        if allInputs.totalGrain3.is_nan() { allInputs.totalGrain3 = 0.0; }
        if allInputs.totalGrain4.is_nan() { allInputs.totalGrain4 = 0.0; }
        if allInputs.totalGrain5.is_nan() { allInputs.totalGrain5 = 0.0; }
        if allInputs.totalGrain6.is_nan() { allInputs.totalGrain6 = 0.0; }
        if allInputs.totalGrain7.is_nan() { allInputs.totalGrain7 = 0.0; }
        let grainSum: f64 = 1.0 + allInputs.totalGrain1.fract() + allInputs.totalGrain2.fract() + allInputs.totalGrain3.fract() + allInputs.totalGrain4.fract() + allInputs.totalGrain5.fract() + allInputs.totalGrain6.fract() + allInputs.totalGrain7.fract();
        allInputs.startingBrix = grainSum.gravityToBrix();
        allInputs.abvFinal = realABVAndAttenuation(allInputs.startingBrix, FINAL_BRIX_IDEAL).0;
    }

    allInputs
}

pub fn grainToABVFormatting(allInputs: &mut grainABVData) -> &mut grainABVData {
    if allInputs.abvFinal > 0.0 && allInputs.abvFinal < 26.0 {
        allInputs.estimatedBrixFormatted = format!("{:.2}°Bx", allInputs.startingBrix);
        allInputs.abvFormatted = format!("{:.2}%", allInputs.abvFinal);
    }

    allInputs
}

pub fn grainToABVColour(allInputs: &grainABVData) -> RGBA {
    let singleMCU1: f64 = singleMCU(allInputs.wortAmount, allInputs.firstGrainAmount, allInputs.firstGrainLovibond);
    let mut singleMCU2: f64 = 0.0;
    let mut singleMCU3: f64 = 0.0;
    let mut singleMCU4: f64 = 0.0;
    let mut singleMCU5: f64 = 0.0;
    let mut singleMCU6: f64 = 0.0;
    let mut singleMCU7: f64 = 0.0;
    if !allInputs.secondGrainAmount.is_nan() {
        singleMCU2 = singleMCU(allInputs.wortAmount, allInputs.secondGrainAmount, allInputs.secondGrainLovibond);
    }

    if !allInputs.thirdGrainAmount.is_nan() {
        singleMCU3 = singleMCU(allInputs.wortAmount, allInputs.thirdGrainAmount, allInputs.thirdGrainLovibond);
    }

    if !allInputs.fourthGrainAmount.is_nan() {
        singleMCU4 = singleMCU(allInputs.wortAmount, allInputs.fourthGrainAmount, allInputs.fourthGrainLovibond);
    }

    if !allInputs.fifthGrainAmount.is_nan() {
        singleMCU5 = singleMCU(allInputs.wortAmount, allInputs.fifthGrainAmount, allInputs.fifthGrainLovibond);
    }

    if !allInputs.sixthGrainAmount.is_nan() {
        singleMCU6 = singleMCU(allInputs.wortAmount, allInputs.sixthGrainAmount, allInputs.sixthGrainLovibond);
    }

    if !allInputs.seventhGrainAmount.is_nan() {
        singleMCU7 = singleMCU(allInputs.wortAmount, allInputs.seventhGrainAmount, allInputs.seventhGrainLovibond);
    }

    let totalMCU: f64 = singleMCU1 + singleMCU2 + singleMCU3 + singleMCU4 + singleMCU5 + singleMCU6 + singleMCU7;
    let beerSRM: f64 = beerSRM(totalMCU);
    let (lOut, aOut, bOut) = grainSRMToLAB(allInputs.glassSize, beerSRM);
    let (xOut, yOut, zOut) = grainLABToXYZ(lOut, aOut, bOut);
    let rgbaOutput: RGBA = if allInputs.abvFinal > 26.0 || allInputs.abvFinal <= 0.0 {
        zeroRGBA()
    } else {
        grainXYZToRGBA(xOut, yOut, zOut)
    };

    rgbaOutput
}

fn grainToABVOutput(mut allInputs: grainABVData, allOverlays: &colourOverlay, grainABVBuilder: &gtk::Builder) {
    let grainABVFinalBrix: gtk::Entry = grainABVBuilder.get_object("grainABVFinalBrix").expect("grainToABVOutput(), grainABVFinalBrix");
    let grainABVFinalABV: gtk::Entry = grainABVBuilder.get_object("grainABVFinalABV").expect("grainToABVOutput(), grainABVFinalABV");

    grainToABVMaths(&mut allInputs);
    grainToABVFormatting(&mut allInputs);
    let rgbaOutput: RGBA = grainToABVColour(&allInputs);

    grainABVFinalBrix.set_text(&allInputs.estimatedBrixFormatted);
    grainABVFinalABV.set_text(&allInputs.abvFormatted);
    allOverlays.colourOutput.set_rgba(&rgbaOutput);
}