use functions::commonFunctions::imperialOrMetric;
use functions::increaseABV::{increaseABVData, finalSugarFloat, sugarOutputStrings, increaseABVMaths, increaseABVFormatting};

#[test]
fn increaseABVMathsImperialGBTest() {
    let allInputs: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.0,
        currentVolume: 15.0,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let output: finalSugarFloat = increaseABVMaths(&allInputs);

    assert_eq!(output.newStartingBrixFinal, 14.480999999998625);
    assert_eq!(output.sugarToAdd, 3.7298409493028104);
    assert_eq!(output.honeyToAdd, 4.548586523085154);
}

#[test]
fn increaseABVMathsImperialUSTest() {
    let allInputs: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.0,
        currentVolume: 15.0,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let output: finalSugarFloat = increaseABVMaths(&allInputs);

    assert_eq!(output.newStartingBrixFinal, 14.480999999998625);
    assert_eq!(output.sugarToAdd, 3.1057422712565317);
    assert_eq!(output.honeyToAdd, 3.7874905743243388);
}

#[test]
fn increaseABVMathsMetricTest() {
    let allInputs: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.0,
        currentVolume: 45.0,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let output: finalSugarFloat = increaseABVMaths(&allInputs);

    assert_eq!(output.newStartingBrixFinal, 14.480999999998625);
    assert_eq!(output.sugarToAdd, 1.116449999999381);
    assert_eq!(output.honeyToAdd, 1.3615243901069953);
}

#[test]
fn increaseABVFormattingImperialGBUnderOverOneWithOuncesTest() {
    let allInputsUnderOne: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 6.01,
        currentVolume: 5.0,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let finalOutputFloatUnderOne: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 12.993999999999449,
        sugarToAdd: 0.49811391960325063,
        honeyToAdd: 0.6074559994554137,
    };

    let outputUnderOne: sugarOutputStrings = increaseABVFormatting(&allInputsUnderOne, finalOutputFloatUnderOne);

    assert_eq!(outputUnderOne.newStartingBrixFinal, "12.99°Bx");
    assert_eq!(outputUnderOne.sugarAmount, "7.97 oz");
    assert_eq!(outputUnderOne.honeyAmount, "9.72 oz");

    let allInputsOverOne: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.00,
        currentVolume: 15.0,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let finalOutputFloatOverOne: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 14.480999999998625,
        sugarToAdd: 3.7298409493028104,
        honeyToAdd: 4.548586523085154,
    };

    let outputOverOne: sugarOutputStrings = increaseABVFormatting(&allInputsOverOne, finalOutputFloatOverOne);

    assert_eq!(outputOverOne.newStartingBrixFinal, "14.48°Bx");
    assert_eq!(outputOverOne.sugarAmount, "3 lbs 11.68 oz");
    assert_eq!(outputOverOne.honeyAmount, "4 lbs 8.78 oz");

    let allInputsOneWithOunces: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.00,
        currentVolume: 5.0,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let finalOutputFloatOneWithOunces: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 14.480999999998625,
        sugarToAdd: 1.2432803164342703,
        honeyToAdd: 1.5161955076950515,
    };

    let outputOneWithOunces: sugarOutputStrings = increaseABVFormatting(&allInputsOneWithOunces, finalOutputFloatOneWithOunces);

    assert_eq!(outputOneWithOunces.newStartingBrixFinal, "14.48°Bx");
    assert_eq!(outputOneWithOunces.sugarAmount, "1 lb 3.89 oz");
    assert_eq!(outputOneWithOunces.honeyAmount, "1 lb 8.26 oz");
}

#[test]
fn increaseABVFormattingImperialGBEqualToOneTest() {
    let allInputsSugarOne: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.00,
        currentVolume: 4.005,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let finalOutputFloatSugarOne: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 14.480999999998625,
        sugarToAdd: 0.9958675334638506,
        honeyToAdd: 1.2144726016637364,
    };

    let outputSugarOne: sugarOutputStrings = increaseABVFormatting(&allInputsSugarOne, finalOutputFloatSugarOne);

    assert_eq!(outputSugarOne.newStartingBrixFinal, "14.48°Bx");
    assert_eq!(outputSugarOne.sugarAmount, "1 lb");
    assert_eq!(outputSugarOne.honeyAmount, "1 lb 3.43 oz");

    let allInputsHoneyOne: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.00,
        currentVolume: 3.3,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let finalOutputFloatHoneyOne: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 14.480999999998625,
        sugarToAdd: 0.8205650088466184,
        honeyToAdd: 1.000689035078734,
    };

    let outputHoneyOne: sugarOutputStrings = increaseABVFormatting(&allInputsHoneyOne, finalOutputFloatHoneyOne);

    assert_eq!(outputHoneyOne.newStartingBrixFinal, "14.48°Bx");
    assert_eq!(outputHoneyOne.sugarAmount, "13.13 oz");
    assert_eq!(outputHoneyOne.honeyAmount, "1 lb");
}

#[test]
fn increaseABVFormattingImperialGBPoundsNoOuncesTest() {
    let allInputsSugarPounds: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.0,
        currentVolume: 12.065,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let finalOutputFloatSugarPounds: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 14.480999999998625,
        sugarToAdd: 3.000035403555894,
        honeyToAdd: 3.6585797600681595,
    };

    let outputSugarPounds: sugarOutputStrings = increaseABVFormatting(&allInputsSugarPounds, finalOutputFloatSugarPounds);

    assert_eq!(outputSugarPounds.newStartingBrixFinal, "14.48°Bx");
    assert_eq!(outputSugarPounds.sugarAmount, "3 lbs");
    assert_eq!(outputSugarPounds.honeyAmount, "3 lbs 10.54 oz");

    let allInputsHoneyPounds: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.0,
        currentVolume: 9.894,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let finalOutputFloatHoneyPounds: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 14.480999999998625,
        sugarToAdd: 2.460203090160134,
        honeyToAdd: 3.000247670626968,
    };

    let outputHoneyPounds: sugarOutputStrings = increaseABVFormatting(&allInputsHoneyPounds, finalOutputFloatHoneyPounds);

    assert_eq!(outputHoneyPounds.newStartingBrixFinal, "14.48°Bx");
    assert_eq!(outputHoneyPounds.sugarAmount, "2 lbs 7.36 oz");
    assert_eq!(outputHoneyPounds.honeyAmount, "3 lbs");
}

#[test]
fn increaseABVFormattingImperialUSUnderOverOneWithOuncesTest() {
    let allInputsUnderOne: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 6.01,
        currentVolume: 1.0,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let finalOutputFloatUnderOne: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 12.993999999999449,
        sugarToAdd: 0.08295332037159728,
        honeyToAdd: 0.10116258580890482,
    };

    let outputUnderOne: sugarOutputStrings = increaseABVFormatting(&allInputsUnderOne, finalOutputFloatUnderOne);

    assert_eq!(outputUnderOne.newStartingBrixFinal, "12.99°Bx");
    assert_eq!(outputUnderOne.sugarAmount, "1.33 oz");
    assert_eq!(outputUnderOne.honeyAmount, "1.62 oz");

    let allInputsOverOne: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.00,
        currentVolume: 15.0,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let finalOutputFloatOverOne: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 14.480999999998625,
        sugarToAdd: 3.1057422712565317,
        honeyToAdd: 3.7874905743243388,
    };

    let outputOverOne: sugarOutputStrings = increaseABVFormatting(&allInputsOverOne, finalOutputFloatOverOne);

    assert_eq!(outputOverOne.newStartingBrixFinal, "14.48°Bx");
    assert_eq!(outputOverOne.sugarAmount, "3 lbs 1.69 oz");
    assert_eq!(outputOverOne.honeyAmount, "3 lbs 12.60 oz");

    let allInputsOneWithOunces: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.00,
        currentVolume: 5.0,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let finalOutputFloatOneWithOunces: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 14.480999999998625,
        sugarToAdd: 1.0352474237521774,
        honeyToAdd: 1.262496858108113,
    };

    let outputOneWithOunces: sugarOutputStrings = increaseABVFormatting(&allInputsOneWithOunces, finalOutputFloatOneWithOunces);

    assert_eq!(outputOneWithOunces.newStartingBrixFinal, "14.48°Bx");
    assert_eq!(outputOneWithOunces.sugarAmount, "1 lb 0.56 oz");
    assert_eq!(outputOneWithOunces.honeyAmount, "1 lb 4.20 oz");
}

#[test]
fn increaseABVFormattingImperialUSEqualToOneTest() {
    let allInputsSugarOne: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.00,
        currentVolume: 4.85,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let finalOutputFloatSugarOne: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 14.480999999998625,
        sugarToAdd: 1.0041900010396119,
        honeyToAdd: 1.2246219523648694,
    };

    let outputSugarOne: sugarOutputStrings = increaseABVFormatting(&allInputsSugarOne, finalOutputFloatSugarOne);

    assert_eq!(outputSugarOne.newStartingBrixFinal, "14.48°Bx");
    assert_eq!(outputSugarOne.sugarAmount, "1 lb");
    assert_eq!(outputSugarOne.honeyAmount, "1 lb 3.59 oz");

    let allInputsHoneyOne: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.00,
        currentVolume: 3.95,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let finalOutputFloatHoneyOne: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 14.480999999998625,
        sugarToAdd: 0.8178454647642199,
        honeyToAdd: 0.997372517905409,
    };

    let outputHoneyOne: sugarOutputStrings = increaseABVFormatting(&allInputsHoneyOne, finalOutputFloatHoneyOne);

    assert_eq!(outputHoneyOne.newStartingBrixFinal, "14.48°Bx");
    assert_eq!(outputHoneyOne.sugarAmount, "13.09 oz");
    assert_eq!(outputHoneyOne.honeyAmount, "1 lb");
}

#[test]
fn increaseABVFormattingImperialUSPoundsNoOuncesTest() {
    let allInputsSugarPounds: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.0,
        currentVolume: 14.49,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let finalOutputFloatSugarPounds: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 14.480999999998625,
        sugarToAdd: 3.0001470340338097,
        honeyToAdd: 3.6587158947973113,
    };

    let outputSugarPounds: sugarOutputStrings = increaseABVFormatting(&allInputsSugarPounds, finalOutputFloatSugarPounds);

    assert_eq!(outputSugarPounds.newStartingBrixFinal, "14.48°Bx");
    assert_eq!(outputSugarPounds.sugarAmount, "3 lbs");
    assert_eq!(outputSugarPounds.honeyAmount, "3 lbs 10.54 oz");

    let allInputsHoneyPounds: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.0,
        currentVolume: 11.88,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let finalOutputFloatHoneyPounds: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 14.480999999998625,
        sugarToAdd: 2.459747878835173,
        honeyToAdd: 2.9996925348648764,
    };

    let outputHoneyPounds: sugarOutputStrings = increaseABVFormatting(&allInputsHoneyPounds, finalOutputFloatHoneyPounds);

    assert_eq!(outputHoneyPounds.newStartingBrixFinal, "14.48°Bx");
    assert_eq!(outputHoneyPounds.sugarAmount, "2 lbs 7.36 oz");
    assert_eq!(outputHoneyPounds.honeyAmount, "3 lbs");
}

#[test]
fn increaseABVFormattingMetricUnderOverOneTest() {
    let allInputsUnderOne: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 6.01,
        currentVolume: 23.0,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let finalOutputFloatUnderOne: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 12.993999999999449,
        sugarToAdd: 0.2286199999998733,
        honeyToAdd: 0.2788048780207455,
    };

    let outputUnderOne: sugarOutputStrings = increaseABVFormatting(&allInputsUnderOne, finalOutputFloatUnderOne);

    assert_eq!(outputUnderOne.newStartingBrixFinal, "12.99°Bx");
    assert_eq!(outputUnderOne.sugarAmount, "0.23 kilos");
    assert_eq!(outputUnderOne.honeyAmount, "0.28 kilos");

    let allInputsOverOne: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.00,
        currentVolume: 45.0,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let finalOutputFloatOverOne: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 14.480999999998625,
        sugarToAdd: 1.116449999999381,
        honeyToAdd: 1.3615243901069953,
    };

    let outputOverOne: sugarOutputStrings = increaseABVFormatting(&allInputsOverOne, finalOutputFloatOverOne);

    assert_eq!(outputOverOne.newStartingBrixFinal, "14.48°Bx");
    assert_eq!(outputOverOne.sugarAmount, "1.12 kilos");
    assert_eq!(outputOverOne.honeyAmount, "1.36 kilos");
}



#[test]
fn increaseABVFormattingMetricEqualToOneTest() {
    let allInputsSugarOne: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.00,
        currentVolume: 45.0,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let finalOutputFloatSugarOne: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 14.480999999998625,
        sugarToAdd: 1.004804999999443,
        honeyToAdd: 1.225371951096296,
    };

    let outputSugarOne: sugarOutputStrings = increaseABVFormatting(&allInputsSugarOne, finalOutputFloatSugarOne);

    assert_eq!(outputSugarOne.newStartingBrixFinal, "14.48°Bx");
    assert_eq!(outputSugarOne.sugarAmount, "1 kilo");
    assert_eq!(outputSugarOne.honeyAmount, "1.23 kilos");

    let allInputsHoneyOne: increaseABVData = increaseABVData {
        startingBrix: 12.0,
        desiredABV: 7.00,
        currentVolume: 33.0,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let finalOutputFloatHoneyOne: finalSugarFloat = finalSugarFloat {
        newStartingBrixFinal: 14.480999999998625,
        sugarToAdd: 0.8187299999995463,
        honeyToAdd: 0.9984512194117968,
    };

    let outputHoneyOne: sugarOutputStrings = increaseABVFormatting(&allInputsHoneyOne, finalOutputFloatHoneyOne);

    assert_eq!(outputHoneyOne.newStartingBrixFinal, "14.48°Bx");
    assert_eq!(outputHoneyOne.sugarAmount, "0.82 kilos");
    assert_eq!(outputHoneyOne.honeyAmount, "1 kilo");
}