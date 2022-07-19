use functions::commonFunctions::{imperialOrMetric, singleInput};
use functions::waterSparge::{waterSpargeData, finalSpargeFloat, spargeOutputStrings, waterSpargeMaths, waterSpargeFormatting};

#[test]
fn waterSpargeMathsImperialGBTest() {
    let allInputs: waterSpargeData = waterSpargeData {
        preFermentVolume: 20.0.gallonsGBToGallonsUS(),
        totalGrain: 39.64,
        boilTime: 60.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let output: finalSpargeFloat = waterSpargeMaths(&allInputs);

    assert_eq!(output.mashVolume, 13.213333333333333);
    assert_eq!(output.spargeVolume, 23.843943611111115);
    assert_eq!(output.totalVolume, 37.057276944444446);
}

#[test]
fn waterSpargeMathsImperialUSTest() {
    let allInputs: waterSpargeData = waterSpargeData {
        preFermentVolume: 20.0,
        totalGrain: 60.0,
        boilTime: 60.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let output: finalSpargeFloat = waterSpargeMaths(&allInputs);

    assert_eq!(output.mashVolume, 20.0);
    assert_eq!(output.spargeVolume, 14.905555555555551);
    assert_eq!(output.totalVolume, 34.90555555555555);
}

#[test]
fn waterSpargeMathsMetricTest() {
    let allInputs: waterSpargeData = waterSpargeData {
        preFermentVolume: 75.0.litresToGallonsUS(),
        totalGrain: 17.975.kilosToPounds(),
        boilTime: 60.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let output: finalSpargeFloat = waterSpargeMaths(&allInputs);

    assert_eq!(output.mashVolume, 13.209363876816667);
    assert_eq!(output.spargeVolume, 18.398064002667496);
    assert_eq!(output.totalVolume, 31.607427879484163);
}

#[test]
fn waterSpargeFormattingImperialGBTest() {
    let allInputsMashNoDecimal: waterSpargeData = waterSpargeData {
        preFermentVolume: 20.0.gallonsGBToGallonsUS(),
        totalGrain: 39.64,
        boilTime: 60.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let finalOutputFloatMashNoDecimal: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 13.213333333333333,
        spargeVolume: 23.843943611111115,
        totalVolume: 37.057276944444446,
    };

    let outputMashNoDecimal: spargeOutputStrings = waterSpargeFormatting(&allInputsMashNoDecimal, finalOutputFloatMashNoDecimal);

    assert_eq!(outputMashNoDecimal.mashVolumeFinal, "11 gallons");
    assert_eq!(outputMashNoDecimal.spargeVolumeFinal, "19.85 gallons");
    assert_eq!(outputMashNoDecimal.totalVolumeFinal, "30.86 gallons");

    let allInputsSpargeNoDecimal: waterSpargeData = waterSpargeData {
        preFermentVolume: 20.0.gallonsGBToGallonsUS(),
        totalGrain: 38.7,
        boilTime: 60.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let finalOutputFloatSpargeNoDecimal: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 12.9,
        spargeVolume: 24.01627694444445,
        totalVolume: 36.91627694444445,
    };

    let outputSpargeNoDecimal: spargeOutputStrings = waterSpargeFormatting(&allInputsSpargeNoDecimal, finalOutputFloatSpargeNoDecimal);

    assert_eq!(outputSpargeNoDecimal.mashVolumeFinal, "10.74 gallons");
    assert_eq!(outputSpargeNoDecimal.spargeVolumeFinal, "20 gallons");
    assert_eq!(outputSpargeNoDecimal.totalVolumeFinal, "30.74 gallons");

    let allInputsTotalNoDecimal: waterSpargeData = waterSpargeData {
        preFermentVolume: 20.0.gallonsGBToGallonsUS(),
        totalGrain: 32.8,
        boilTime: 60.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let finalOutputFloatTotalNoDecimal: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 10.933333333333332,
        spargeVolume: 25.097943611111113,
        totalVolume: 36.03127694444444,
    };

    let outputTotalNoDecimal: spargeOutputStrings = waterSpargeFormatting(&allInputsTotalNoDecimal, finalOutputFloatTotalNoDecimal);

    assert_eq!(outputTotalNoDecimal.mashVolumeFinal, "9.10 gallons");
    assert_eq!(outputTotalNoDecimal.spargeVolumeFinal, "20.90 gallons");
    assert_eq!(outputTotalNoDecimal.totalVolumeFinal, "30 gallons");

    let allInputsMashOne: waterSpargeData = waterSpargeData {
        preFermentVolume: 5.0.gallonsGBToGallonsUS(),
        totalGrain: 3.6,
        boilTime: 60.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let finalOutputFloatMashOne: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 1.2,
        spargeVolume: 7.117819236111111,
        totalVolume: 8.317819236111111,
    };

    let outputMashOne: spargeOutputStrings = waterSpargeFormatting(&allInputsMashOne, finalOutputFloatMashOne);

    assert_eq!(outputMashOne.mashVolumeFinal, "1 gallon");
    assert_eq!(outputMashOne.spargeVolumeFinal, "5.93 gallons");
    assert_eq!(outputMashOne.totalVolumeFinal, "6.93 gallons");

    let allInputsSpargeOne: waterSpargeData = waterSpargeData {
        preFermentVolume: 5.0.gallonsGBToGallonsUS(),
        totalGrain: 35.9,
        boilTime: 60.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let finalOutputFloatSpargeOne: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 11.966666666666665,
        spargeVolume: 1.1961525694444468,
        totalVolume: 13.162819236111112,
    };

    let outputSpargeOne: spargeOutputStrings = waterSpargeFormatting(&allInputsSpargeOne, finalOutputFloatSpargeOne);

    assert_eq!(outputSpargeOne.mashVolumeFinal, "9.96 gallons");
    assert_eq!(outputSpargeOne.spargeVolumeFinal, "1 gallon");
    assert_eq!(outputSpargeOne.totalVolumeFinal, "10.96 gallons");

    let allInputsTotalOne: waterSpargeData = waterSpargeData {
        preFermentVolume: 0.84.gallonsGBToGallonsUS(),
        totalGrain: 0.1,
        boilTime: 1.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::imperialGB,
    };

    let finalOutputFloatTotalOne: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 0.03333333333333333,
        spargeVolume: 1.167585343895381,
        totalVolume: 1.2009186772287144,
    };

    let outputTotalOne: spargeOutputStrings = waterSpargeFormatting(&allInputsTotalOne, finalOutputFloatTotalOne);

    assert_eq!(outputTotalOne.mashVolumeFinal, "0.03 gallons");
    assert_eq!(outputTotalOne.spargeVolumeFinal, "0.97 gallons");
    assert_eq!(outputTotalOne.totalVolumeFinal, "1 gallon");
}

#[test]
fn waterSpargeFormattingImperialUSTest() {
    let allInputsMashNoDecimal: waterSpargeData = waterSpargeData {
        preFermentVolume: 20.0,
        totalGrain: 60.0,
        boilTime: 60.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let finalOutputFloatMashNoDecimal: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 20.0,
        spargeVolume: 14.905555555555551,
        totalVolume: 34.90555555555555,
    };

    let outputMashNoDecimal: spargeOutputStrings = waterSpargeFormatting(&allInputsMashNoDecimal, finalOutputFloatMashNoDecimal);

    assert_eq!(outputMashNoDecimal.mashVolumeFinal, "20 gallons");
    assert_eq!(outputMashNoDecimal.spargeVolumeFinal, "14.91 gallons");
    assert_eq!(outputMashNoDecimal.totalVolumeFinal, "34.91 gallons");

    let allInputsSpargeNoDecimal: waterSpargeData = waterSpargeData {
        preFermentVolume: 5.0,
        totalGrain: 6.3,
        boilTime: 30.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let finalOutputFloatSpargeNoDecimal: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 2.0999999999999996,
        spargeVolume: 5.001578947368422,
        totalVolume: 7.101578947368422,
    };

    let outputSpargeNoDecimal: spargeOutputStrings = waterSpargeFormatting(&allInputsSpargeNoDecimal, finalOutputFloatSpargeNoDecimal);

    assert_eq!(outputSpargeNoDecimal.mashVolumeFinal, "2.10 gallons");
    assert_eq!(outputSpargeNoDecimal.spargeVolumeFinal, "5 gallons");
    assert_eq!(outputSpargeNoDecimal.totalVolumeFinal, "7.10 gallons");

    let allInputsTotalNoDecimal: waterSpargeData = waterSpargeData {
        preFermentVolume: 10.04,
        totalGrain: 20.0,
        boilTime: 60.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let finalOutputFloatTotalNoDecimal: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 6.666666666666666,
        spargeVolume: 9.337922222222224,
        totalVolume: 16.00458888888889,
    };

    let outputTotalNoDecimal: spargeOutputStrings = waterSpargeFormatting(&allInputsTotalNoDecimal, finalOutputFloatTotalNoDecimal);

    assert_eq!(outputTotalNoDecimal.mashVolumeFinal, "6.67 gallons");
    assert_eq!(outputTotalNoDecimal.spargeVolumeFinal, "9.34 gallons");
    assert_eq!(outputTotalNoDecimal.totalVolumeFinal, "16 gallons");

    let allInputsMashOne: waterSpargeData = waterSpargeData {
        preFermentVolume: 5.0,
        totalGrain: 3.0,
        boilTime: 60.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let finalOutputFloatMashOne: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 1.0,
        spargeVolume: 5.926388888888889,
        totalVolume: 6.926388888888889,
    };

    let outputMashOne: spargeOutputStrings = waterSpargeFormatting(&allInputsMashOne, finalOutputFloatMashOne);

    assert_eq!(outputMashOne.mashVolumeFinal, "1 gallon");
    assert_eq!(outputMashOne.spargeVolumeFinal, "5.93 gallons");
    assert_eq!(outputMashOne.totalVolumeFinal, "6.93 gallons");

    let allInputsSpargeOne: waterSpargeData = waterSpargeData {
        preFermentVolume: 2.0,
        totalGrain: 8.0,
        boilTime: 30.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let finalOutputFloatSpargeOne: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 2.6666666666666665,
        spargeVolume:0.9959649122807019,
        totalVolume: 3.6626315789473685,
    };

    let outputSpargeOne: spargeOutputStrings = waterSpargeFormatting(&allInputsSpargeOne, finalOutputFloatSpargeOne);

    assert_eq!(outputSpargeOne.mashVolumeFinal, "2.67 gallons");
    assert_eq!(outputSpargeOne.spargeVolumeFinal, "1 gallon");
    assert_eq!(outputSpargeOne.totalVolumeFinal, "3.66 gallons");

    let allInputsTotalOne: waterSpargeData = waterSpargeData {
        preFermentVolume: 0.7,
        totalGrain: 1.0,
        boilTime: 20.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::imperialUS,
    };

    let finalOutputFloatTotalOne: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 0.3333333333333333,
        spargeVolume: 0.6646925287356322,
        totalVolume: 0.9980258620689656,
    };

    let outputTotalOne: spargeOutputStrings = waterSpargeFormatting(&allInputsTotalOne, finalOutputFloatTotalOne);

    assert_eq!(outputTotalOne.mashVolumeFinal, "0.33 gallons");
    assert_eq!(outputTotalOne.spargeVolumeFinal, "0.66 gallons");
    assert_eq!(outputTotalOne.totalVolumeFinal, "1 gallon");
}

#[test]
fn waterSpargeFormattingImperialMetricTest() {
    let allInputsMashNoDecimal: waterSpargeData = waterSpargeData {
        preFermentVolume: 75.0.litresToGallonsUS(),
        totalGrain: 17.975.kilosToPounds(),
        boilTime: 60.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let finalOutputFloatMashNoDecimal: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 13.209363876816667,
        spargeVolume: 18.398064002667496,
        totalVolume: 31.607427879484163,
    };

    let outputMashNoDecimal: spargeOutputStrings = waterSpargeFormatting(&allInputsMashNoDecimal, finalOutputFloatMashNoDecimal);

    assert_eq!(outputMashNoDecimal.mashVolumeFinal, "50 litres");
    assert_eq!(outputMashNoDecimal.spargeVolumeFinal, "69.64 litres");
    assert_eq!(outputMashNoDecimal.totalVolumeFinal, "119.65 litres");

    let allInputsSpargeNoDecimal: waterSpargeData = waterSpargeData {
        preFermentVolume: 75.0.litresToGallonsUS(),
        totalGrain: 17.74.kilosToPounds(),
        boilTime: 60.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let finalOutputFloatSpargeNoDecimal: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 13.036668438093331,
        spargeVolume: 18.49304649396533,
        totalVolume: 31.529714932058663,
    };

    let outputSpargeNoDecimal: spargeOutputStrings = waterSpargeFormatting(&allInputsSpargeNoDecimal, finalOutputFloatSpargeNoDecimal);

    assert_eq!(outputSpargeNoDecimal.mashVolumeFinal, "49.35 litres");
    assert_eq!(outputSpargeNoDecimal.spargeVolumeFinal, "70 litres");
    assert_eq!(outputSpargeNoDecimal.totalVolumeFinal, "119.35 litres");

    let allInputsTotalNoDecimal: waterSpargeData = waterSpargeData {
        preFermentVolume: 75.0.litresToGallonsUS(),
        totalGrain: 18.26.kilosToPounds(),
        boilTime: 60.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let finalOutputFloatTotalNoDecimal: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 13.418803025906666,
        spargeVolume: 18.282872470668,
        totalVolume: 31.701675496574666,
    };

    let outputTotalNoDecimal: spargeOutputStrings = waterSpargeFormatting(&allInputsTotalNoDecimal, finalOutputFloatTotalNoDecimal);

    assert_eq!(outputTotalNoDecimal.mashVolumeFinal, "50.80 litres");
    assert_eq!(outputTotalNoDecimal.spargeVolumeFinal, "69.21 litres");
    assert_eq!(outputTotalNoDecimal.totalVolumeFinal, "120 litres");

    let allInputsMashOne: waterSpargeData = waterSpargeData {
        preFermentVolume: 5.0.litresToGallonsUS(),
        totalGrain: 0.36.kilosToPounds(),
        boilTime: 20.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let finalOutputFloatMashOne: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 0.26455471463999997,
        spargeVolume: 1.4546715650324826,
        totalVolume: 1.7192262796724826,
    };

    let outputMashOne: spargeOutputStrings = waterSpargeFormatting(&allInputsMashOne, finalOutputFloatMashOne);

    assert_eq!(outputMashOne.mashVolumeFinal, "1 litre");
    assert_eq!(outputMashOne.spargeVolumeFinal, "5.51 litres");
    assert_eq!(outputMashOne.totalVolumeFinal, "6.51 litres");

    let allInputsSpargeOne: waterSpargeData = waterSpargeData {
        preFermentVolume: 31.0.litresToGallonsUS(),
        totalGrain: 23.89.kilosToPounds(),
        boilTime: 20.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let finalOutputFloatSpargeOne: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 17.556144813193335,
        spargeVolume: 0.2652156328674593,
        totalVolume: 17.821360446060794,
    };

    let outputSpargeOne: spargeOutputStrings = waterSpargeFormatting(&allInputsSpargeOne, finalOutputFloatSpargeOne);

    assert_eq!(outputSpargeOne.mashVolumeFinal, "66.46 litres");
    assert_eq!(outputSpargeOne.spargeVolumeFinal, "1 litre");
    assert_eq!(outputSpargeOne.totalVolumeFinal, "67.46 litres");

    let allInputsTotalOne: waterSpargeData = waterSpargeData {
        preFermentVolume: 0.815.litresToGallonsUS(),
        totalGrain: 0.01.kilosToPounds(),
        boilTime: 20.0,
        grainAbsorption: 0.15,
        mashThickness: 4.0 / 3.0,
        wortShrinkage: 0.04,
        percentBoiloff: 0.1,
        imperialOrMetric: imperialOrMetric::metric,
    };

    let finalOutputFloatTotalOne: finalSpargeFloat = finalSpargeFloat {
        mashVolume: 0.007348742073333333,
        spargeVolume: 0.25678698712743736,
        totalVolume: 0.2641357292007707,
    };

    let outputTotalOne: spargeOutputStrings = waterSpargeFormatting(&allInputsTotalOne, finalOutputFloatTotalOne);

    assert_eq!(outputTotalOne.mashVolumeFinal, "0.03 litres");
    assert_eq!(outputTotalOne.spargeVolumeFinal, "0.97 litres");
    assert_eq!(outputTotalOne.totalVolumeFinal, "1 litre");
}