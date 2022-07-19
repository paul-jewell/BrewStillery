use functions::commonFunctions::imperialOrMetric;
use functions::guestimateIBU::{guestimateIBUData, totalIBUMaths, totalIBUFormatting};

#[test]
fn totalBUMathsImperialGBTest() {
    let allInputs: guestimateIBUData = guestimateIBUData {
        preBoilBrix: 20.0,
        wortVolume: 20.0,
        firstHopAlpha: 7.0,
        firstHopAmount: 3.0,
        firstHopBoilTime: 60.0,
        secondHopAlpha: 0.0,
        secondHopAmount: 0.0,
        secondHopBoilTime: 0.0,
        thirdHopAlpha: 0.0,
        thirdHopAmount: 0.0,
        thirdHopBoilTime: 0.0,
        fourthHopAlpha: 0.0,
        fourthHopAmount: 0.0,
        fourthHopBoilTime: 0.0,
        fifthHopAlpha: 0.0,
        fifthHopAmount: 0.0,
        fifthHopBoilTime: 0.0,
        sixthHopAlpha: 0.0,
        sixthHopAmount: 0.0,
        sixthHopBoilTime: 0.0,
        seventhHopAlpha: 0.0,
        seventhHopAmount: 0.0,
        seventhHopBoilTime: 0.0,
        totalIBU1: 0.0,
        totalIBU2: 0.0,
        totalIBU3: 0.0,
        totalIBU4: 0.0,
        totalIBU5: 0.0,
        totalIBU6: 0.0,
        totalIBU7: 0.0,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let output: f64 = totalIBUMaths(allInputs);

    assert_eq!(output, 11.230131642840979);
}

#[test]
fn totalBUMathsImperialUSTest() {
    let allInputs: guestimateIBUData = guestimateIBUData {
        preBoilBrix: 20.0,
        wortVolume: 20.0,
        firstHopAlpha: 7.0,
        firstHopAmount: 3.0,
        firstHopBoilTime: 60.0,
        secondHopAlpha: 0.0,
        secondHopAmount: 0.0,
        secondHopBoilTime: 0.0,
        thirdHopAlpha: 0.0,
        thirdHopAmount: 0.0,
        thirdHopBoilTime: 0.0,
        fourthHopAlpha: 0.0,
        fourthHopAmount: 0.0,
        fourthHopBoilTime: 0.0,
        fifthHopAlpha: 0.0,
        fifthHopAmount: 0.0,
        fifthHopBoilTime: 0.0,
        sixthHopAlpha: 0.0,
        sixthHopAmount: 0.0,
        sixthHopBoilTime: 0.0,
        seventhHopAlpha: 0.0,
        seventhHopAmount: 0.0,
        seventhHopBoilTime: 0.0,
        totalIBU1: 0.0,
        totalIBU2: 0.0,
        totalIBU3: 0.0,
        totalIBU4: 0.0,
        totalIBU5: 0.0,
        totalIBU6: 0.0,
        totalIBU7: 0.0,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let output: f64 = totalIBUMaths(allInputs);

    assert_eq!(output, 13.486826596469873);
}

#[test]
fn totalBUMathsMetricTest() {
    let allInputs: guestimateIBUData = guestimateIBUData {
        preBoilBrix: 20.0,
        wortVolume: 20.0,
        firstHopAlpha: 7.0,
        firstHopAmount: 3.0,
        firstHopBoilTime: 60.0,
        secondHopAlpha: 0.0,
        secondHopAmount: 0.0,
        secondHopBoilTime: 0.0,
        thirdHopAlpha: 0.0,
        thirdHopAmount: 0.0,
        thirdHopBoilTime: 0.0,
        fourthHopAlpha: 0.0,
        fourthHopAmount: 0.0,
        fourthHopBoilTime: 0.0,
        fifthHopAlpha: 0.0,
        fifthHopAmount: 0.0,
        fifthHopBoilTime: 0.0,
        sixthHopAlpha: 0.0,
        sixthHopAmount: 0.0,
        sixthHopBoilTime: 0.0,
        seventhHopAlpha: 0.0,
        seventhHopAmount: 0.0,
        seventhHopBoilTime: 0.0,
        totalIBU1: 0.0,
        totalIBU2: 0.0,
        totalIBU3: 0.0,
        totalIBU4: 0.0,
        totalIBU5: 0.0,
        totalIBU6: 0.0,
        totalIBU7: 0.0,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let output: f64 = totalIBUMaths(allInputs);

    assert_eq!(output, 1.8008483685642402);
}

#[test]
fn totalIBUFormattingTest() {
    let finalOutputFloatSingle: f64 = 0.9957383389985669;
    let finalOutputFloatMultiple: f64 = 2.9984451486385413;
    let finalOutputFloatDecimal: f64 = 11.230131642840979;

    assert_eq!(totalIBUFormatting(finalOutputFloatSingle), "1 IBU");
    assert_eq!(totalIBUFormatting(finalOutputFloatMultiple), "3 IBUs");
    assert_eq!(totalIBUFormatting(finalOutputFloatDecimal), "11.23 IBUs");
}