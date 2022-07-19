use functions::commonFunctions::imperialOrMetric;
use functions::gyleCarbonation::{gyleData, gyleCarbonationMaths, gyleCarbonationFormatting};

#[test]
fn gyleCarbonationMathsImperialGBTest() {
    let allInputs: gyleData = gyleData {
        startingGravity: 1.0525962648284455,
        desiredCO2Level: 4.0,
        finalVolume: 90.9218,
        gyleVolumeFloat: 0.0,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let output: gyleData = gyleCarbonationMaths(allInputs);

    assert_eq!(output.gyleVolumeFloat, 9.855318842315654);
}

#[test]
fn gyleCarbonationMathsImperialUSTest() {
    let allInputs: gyleData = gyleData {
        startingGravity: 1.0525962648284455,
        desiredCO2Level: 4.0,
        finalVolume: 75.70823568,
        gyleVolumeFloat: 0.0,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let output: gyleData = gyleCarbonationMaths(allInputs);

    assert_eq!(output.gyleVolumeFloat, 8.206269581283896);
}

#[test]
fn gyleCarbonationMathsMetricTest() {
    let allInputs: gyleData = gyleData {
        startingGravity: 1.0525962648284455,
        desiredCO2Level: 4.0,
        finalVolume: 75.0,
        gyleVolumeFloat: 0.0,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let output: gyleData = gyleCarbonationMaths(allInputs);

    assert_eq!(output.gyleVolumeFloat, 8.129501540595038);
}

#[test]
fn gyleCarbonationFormattingImperialGBTest() {
    let allInputsOne: gyleData = gyleData {
        startingGravity: 1.0525962648284455,
        desiredCO2Level: 1.85,
        finalVolume: 90.9218,
        gyleVolumeFloat: 4.55808496457099,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let outputOne: String = gyleCarbonationFormatting(allInputsOne);

    assert_eq!(outputOne, "1 gallon");

    let allInputsMultiple: gyleData = gyleData {
        startingGravity: 1.0525962648284455,
        desiredCO2Level: 3.69,
        finalVolume: 90.9218,
        gyleVolumeFloat: 9.09153163203619,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let outputMultiple: String = gyleCarbonationFormatting(allInputsMultiple);

    assert_eq!(outputMultiple, "2 gallons");

    let allInputsDecimal: gyleData = gyleData {
        startingGravity: 1.0525962648284455,
        desiredCO2Level: 4.0,
        finalVolume: 90.9218,
        gyleVolumeFloat: 9.855318842315654,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let outputDecimal: String = gyleCarbonationFormatting(allInputsDecimal);

    assert_eq!(outputDecimal, "2.17 gallons");
}

#[test]
fn gyleCarbonationFormattingImperialUSTest() {
    let allInputsOne: gyleData = gyleData {
        startingGravity: 1.0525962648284455,
        desiredCO2Level: 1.85,
        finalVolume: 75.70823568,
        gyleVolumeFloat: 3.795399681343802,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let outputOne: String = gyleCarbonationFormatting(allInputsOne);

    assert_eq!(outputOne, "1 gallon");

    let allInputsMultiple: gyleData = gyleData {
        startingGravity: 1.0525962648284455,
        desiredCO2Level: 3.69,
        finalVolume: 75.70823568,
        gyleVolumeFloat: 7.570283688734394,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let outputMultiple: String = gyleCarbonationFormatting(allInputsMultiple);

    assert_eq!(outputMultiple, "2 gallons");

    let allInputsDecimal: gyleData = gyleData {
        startingGravity: 1.0525962648284455,
        desiredCO2Level: 4.0,
        finalVolume: 75.70823568,
        gyleVolumeFloat: 8.206269581283896,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let outputDecimal: String = gyleCarbonationFormatting(allInputsDecimal);

    assert_eq!(outputDecimal, "2.17 gallons");
}

#[test]
fn gyleCarbonationFormattingImperialMetricTest() {
    let allInputsOne: gyleData = gyleData {
        startingGravity: 1.0525962648284455,
        desiredCO2Level: 1.0,
        finalVolume: 37.0,
        gyleVolumeFloat: 1.0026385233400545,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let outputOne: String = gyleCarbonationFormatting(allInputsOne);

    assert_eq!(outputOne, "1 litre");

    let allInputsMultiple: gyleData = gyleData {
        startingGravity: 1.0525962648284455,
        desiredCO2Level: 1.99,
        finalVolume: 37.0,
        gyleVolumeFloat: 1.9952506614467087,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let outputMultiple: String = gyleCarbonationFormatting(allInputsMultiple);

    assert_eq!(outputMultiple, "2 litres");

    let allInputsDecimal: gyleData = gyleData {
        startingGravity: 1.0525962648284455,
        desiredCO2Level: 4.0,
        finalVolume: 75.0,
        gyleVolumeFloat: 8.129501540595038,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let outputDecimal: String = gyleCarbonationFormatting(allInputsDecimal);

    assert_eq!(outputDecimal, "8.13 litres");
}