use gdk::RGBA;
use functions::commonFunctions::imperialOrMetric;
use functions::grainToABV::{grainABVData, grainToABVMaths, grainToABVFormatting, grainToABVColour};

#[test]
fn grainToABVMathsImperialGBTest() {
    let mut allInputs: grainABVData = grainABVData {
        wortAmount: 20.0,
        firstGrainAmount: 60.0,
        firstGrainGravity: 1.037,
        firstGrainLovibond: 1.8,
        secondGrainAmount: 0.0,
        secondGrainGravity: 1.0,
        secondGrainLovibond:0.0 ,
        thirdGrainAmount: 0.0,
        thirdGrainGravity: 1.0,
        thirdGrainLovibond: 0.0,
        fourthGrainAmount: 0.0,
        fourthGrainGravity: 1.0,
        fourthGrainLovibond: 0.0,
        fifthGrainAmount: 0.0,
        fifthGrainGravity: 1.0,
        fifthGrainLovibond: 0.0,
        sixthGrainAmount: 0.0,
        sixthGrainGravity: 1.0,
        sixthGrainLovibond: 0.0,
        seventhGrainAmount: 0.0,
        seventhGrainGravity: 1.0,
        seventhGrainLovibond: 0.0,
        totalGrain1: 0.0,
        totalGrain2: 0.0,
        totalGrain3: 0.0,
        totalGrain4: 0.0,
        totalGrain5: 0.0,
        totalGrain6: 0.0,
        totalGrain7: 0.0,
        glassSize: 9.8,
        startingBrix: 0.0,
        abvFinal: 0.0,
        estimatedBrixFormatted: String::from("Enter legimate amounts"),
        abvFormatted: String::from(""),
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    grainToABVMaths(&mut allInputs);

    assert_eq!(allInputs.startingBrix, 13.02055777686189);
    assert_eq!(allInputs.abvFinal, 6.027879973540519);
}

#[test]
fn grainToABVMathsImperialUSTest() {
    let mut allInputs = grainABVData {
        wortAmount: 20.0,
        firstGrainAmount: 60.0,
        firstGrainGravity: 1.037,
        firstGrainLovibond: 1.8,
        secondGrainAmount: 0.0,
        secondGrainGravity: 1.0,
        secondGrainLovibond:0.0 ,
        thirdGrainAmount: 0.0,
        thirdGrainGravity: 1.0,
        thirdGrainLovibond: 0.0,
        fourthGrainAmount: 0.0,
        fourthGrainGravity: 1.0,
        fourthGrainLovibond: 0.0,
        fifthGrainAmount: 0.0,
        fifthGrainGravity: 1.0,
        fifthGrainLovibond: 0.0,
        sixthGrainAmount: 0.0,
        sixthGrainGravity: 1.0,
        sixthGrainLovibond: 0.0,
        seventhGrainAmount: 0.0,
        seventhGrainGravity: 1.0,
        seventhGrainLovibond: 0.0,
        totalGrain1: 0.0,
        totalGrain2: 0.0,
        totalGrain3: 0.0,
        totalGrain4: 0.0,
        totalGrain5: 0.0,
        totalGrain6: 0.0,
        totalGrain7: 0.0,
        glassSize: 9.8,
        startingBrix: 0.0,
        abvFinal: 0.0,
        estimatedBrixFormatted: String::from("Enter legimate amounts"),
        abvFormatted: String::from(""),
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    grainToABVMaths(&mut allInputs);

    assert_eq!(allInputs.startingBrix, 15.499109502776603);
    assert_eq!(allInputs.abvFinal, 7.695621290396818);
}

#[test]
fn grainToABVMathsMetricTest() {
    let mut allInputs: grainABVData = grainABVData {
        wortAmount: 75.0,
        firstGrainAmount: 27.0,
        firstGrainGravity: 1.037,
        firstGrainLovibond: 1.8,
        secondGrainAmount: 0.0,
        secondGrainGravity: 1.0,
        secondGrainLovibond:0.0 ,
        thirdGrainAmount: 0.0,
        thirdGrainGravity: 1.0,
        thirdGrainLovibond: 0.0,
        fourthGrainAmount: 0.0,
        fourthGrainGravity: 1.0,
        fourthGrainLovibond: 0.0,
        fifthGrainAmount: 0.0,
        fifthGrainGravity: 1.0,
        fifthGrainLovibond: 0.0,
        sixthGrainAmount: 0.0,
        sixthGrainGravity: 1.0,
        sixthGrainLovibond: 0.0,
        seventhGrainAmount: 0.0,
        seventhGrainGravity: 1.0,
        seventhGrainLovibond: 0.0,
        totalGrain1: 0.0,
        totalGrain2: 0.0,
        totalGrain3: 0.0,
        totalGrain4: 0.0,
        totalGrain5: 0.0,
        totalGrain6: 0.0,
        totalGrain7: 0.0,
        glassSize: 9.8,
        startingBrix: 0.0,
        abvFinal: 0.0,
        estimatedBrixFormatted: String::from("Enter legimate amounts"),
        abvFormatted: String::from(""),
        imperialOrMetric: imperialOrMetric::metric,
    };

    grainToABVMaths(&mut allInputs);

    assert_eq!(allInputs.startingBrix, 15.520375373329689);
    assert_eq!(allInputs.abvFinal, 7.710344253574942);
}

#[test]
fn grainToABVFormattingTest() {
    let mut allInputsLegitimateABV: grainABVData = grainABVData {
        wortAmount: 75.0,
        firstGrainAmount: 27.0,
        firstGrainGravity: 1.037,
        firstGrainLovibond: 1.8,
        secondGrainAmount: 0.0,
        secondGrainGravity: 1.0,
        secondGrainLovibond:0.0 ,
        thirdGrainAmount: 0.0,
        thirdGrainGravity: 1.0,
        thirdGrainLovibond: 0.0,
        fourthGrainAmount: 0.0,
        fourthGrainGravity: 1.0,
        fourthGrainLovibond: 0.0,
        fifthGrainAmount: 0.0,
        fifthGrainGravity: 1.0,
        fifthGrainLovibond: 0.0,
        sixthGrainAmount: 0.0,
        sixthGrainGravity: 1.0,
        sixthGrainLovibond: 0.0,
        seventhGrainAmount: 0.0,
        seventhGrainGravity: 1.0,
        seventhGrainLovibond: 0.0,
        totalGrain1: 0.0,
        totalGrain2: 0.0,
        totalGrain3: 0.0,
        totalGrain4: 0.0,
        totalGrain5: 0.0,
        totalGrain6: 0.0,
        totalGrain7: 0.0,
        glassSize: 9.8,
        startingBrix: 15.520375373329689,
        abvFinal: 7.710344253574942,
        estimatedBrixFormatted: String::from("Enter legimate amounts"),
        abvFormatted: String::from(""),
        imperialOrMetric: imperialOrMetric::metric,
    };

    grainToABVFormatting(&mut allInputsLegitimateABV);

    assert_eq!(allInputsLegitimateABV.estimatedBrixFormatted, "15.52°Bx");
    assert_eq!(allInputsLegitimateABV.abvFormatted, "7.71%");


    let mut allInputsLowABV: grainABVData = grainABVData {
        wortAmount: 75.0,
        firstGrainAmount: 27.0,
        firstGrainGravity: 1.037,
        firstGrainLovibond: 1.8,
        secondGrainAmount: 0.0,
        secondGrainGravity: 1.0,
        secondGrainLovibond:0.0 ,
        thirdGrainAmount: 0.0,
        thirdGrainGravity: 1.0,
        thirdGrainLovibond: 0.0,
        fourthGrainAmount: 0.0,
        fourthGrainGravity: 1.0,
        fourthGrainLovibond: 0.0,
        fifthGrainAmount: 0.0,
        fifthGrainGravity: 1.0,
        fifthGrainLovibond: 0.0,
        sixthGrainAmount: 0.0,
        sixthGrainGravity: 1.0,
        sixthGrainLovibond: 0.0,
        seventhGrainAmount: 0.0,
        seventhGrainGravity: 1.0,
        seventhGrainLovibond: 0.0,
        totalGrain1: 0.0,
        totalGrain2: 0.0,
        totalGrain3: 0.0,
        totalGrain4: 0.0,
        totalGrain5: 0.0,
        totalGrain6: 0.0,
        totalGrain7: 0.0,
        glassSize: 9.8,
        startingBrix: 15.520375373329689,
        abvFinal: -3.0,
        estimatedBrixFormatted: String::from("Enter legimate amounts"),
        abvFormatted: String::from(""),
        imperialOrMetric: imperialOrMetric::metric,
    };

    grainToABVFormatting(&mut allInputsLowABV);

    assert_eq!(allInputsLowABV.estimatedBrixFormatted, "Enter legimate amounts");
    assert_eq!(allInputsLowABV.abvFormatted, "");


    let mut allInputsHighABV: grainABVData = grainABVData {
        wortAmount: 75.0,
        firstGrainAmount: 27.0,
        firstGrainGravity: 1.037,
        firstGrainLovibond: 1.8,
        secondGrainAmount: 0.0,
        secondGrainGravity: 1.0,
        secondGrainLovibond:0.0 ,
        thirdGrainAmount: 0.0,
        thirdGrainGravity: 1.0,
        thirdGrainLovibond: 0.0,
        fourthGrainAmount: 0.0,
        fourthGrainGravity: 1.0,
        fourthGrainLovibond: 0.0,
        fifthGrainAmount: 0.0,
        fifthGrainGravity: 1.0,
        fifthGrainLovibond: 0.0,
        sixthGrainAmount: 0.0,
        sixthGrainGravity: 1.0,
        sixthGrainLovibond: 0.0,
        seventhGrainAmount: 0.0,
        seventhGrainGravity: 1.0,
        seventhGrainLovibond: 0.0,
        totalGrain1: 0.0,
        totalGrain2: 0.0,
        totalGrain3: 0.0,
        totalGrain4: 0.0,
        totalGrain5: 0.0,
        totalGrain6: 0.0,
        totalGrain7: 0.0,
        glassSize: 9.8,
        startingBrix: 15.520375373329689,
        abvFinal: 333.333,
        estimatedBrixFormatted: String::from("Enter legimate amounts"),
        abvFormatted: String::from(""),
        imperialOrMetric: imperialOrMetric::metric,
    };

    grainToABVFormatting(&mut allInputsHighABV);

    assert_eq!(allInputsHighABV.estimatedBrixFormatted, "Enter legimate amounts");
    assert_eq!(allInputsHighABV.abvFormatted, "");
}

#[test]
fn grainToABVColourTest() {
    let allInputs: grainABVData = grainABVData {
        wortAmount: 75.0,
        firstGrainAmount: 27.0,
        firstGrainGravity: 1.037,
        firstGrainLovibond: 1.8,
        secondGrainAmount: 0.0,
        secondGrainGravity: 1.0,
        secondGrainLovibond:0.0 ,
        thirdGrainAmount: 0.0,
        thirdGrainGravity: 1.0,
        thirdGrainLovibond: 0.0,
        fourthGrainAmount: 0.0,
        fourthGrainGravity: 1.0,
        fourthGrainLovibond: 0.0,
        fifthGrainAmount: 0.0,
        fifthGrainGravity: 1.0,
        fifthGrainLovibond: 0.0,
        sixthGrainAmount: 0.0,
        sixthGrainGravity: 1.0,
        sixthGrainLovibond: 0.0,
        seventhGrainAmount: 0.0,
        seventhGrainGravity: 1.0,
        seventhGrainLovibond: 0.0,
        totalGrain1: 0.0,
        totalGrain2: 0.0,
        totalGrain3: 0.0,
        totalGrain4: 0.0,
        totalGrain5: 0.0,
        totalGrain6: 0.0,
        totalGrain7: 0.0,
        glassSize: 9.8,
        startingBrix: 15.520375373329689,
        abvFinal: 7.710344253574942,
        estimatedBrixFormatted: String::from("Enter legimate amounts"),
        abvFormatted: String::from(""),
        imperialOrMetric: imperialOrMetric::metric,
    };

    let finalRGBA: RGBA = grainToABVColour(&allInputs);

    assert_eq!(finalRGBA.red, 0.9410550035212963);
    assert_eq!(finalRGBA.green, 0.8100014920708686);
    assert_eq!(finalRGBA.blue, 0.45931824333551147);
    assert_eq!(finalRGBA.alpha, 1.0);
}