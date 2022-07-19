use functions::commonFunctions::realABVAndAttenuation;
use functions::realABV::realABVFormatting;

#[test]
fn realABVAndAttenuationMathsTest() {
    let startingBrix: f64 = 21.0;
    let finalBrix: f64 = 14.0;
    let (abv, attenuation) = realABVAndAttenuation(startingBrix, finalBrix);

    assert_eq!(abv, 4.973244658620157);
    assert_eq!(attenuation, 37.349534965963777);
}

#[test]
fn realABVFormattingTest() {
    let abvFloatLegitimate: f64 = 4.973244658620157;
    let attenuationFloatLegitimate: f64 = 37.349534965963777;
    let (abvFormattedLegitimate, attenuationFormattedLegitimate) = realABVFormatting(abvFloatLegitimate, attenuationFloatLegitimate);

    assert_eq!(abvFormattedLegitimate, "4.97%");
    assert_eq!(attenuationFormattedLegitimate, "37.35%");

    let abvFloatNotLegitimateLow: f64 = -0.3674338443656469;
    let attenuationFloatNotLegitimateSmall: f64 = -24.5597908010995;
    let (abvFormattedNotLegitimateSmall, attenuationFormattedNotLegitimateSmall) = realABVFormatting(abvFloatNotLegitimateLow, attenuationFloatNotLegitimateSmall);

    assert_eq!(abvFormattedNotLegitimateSmall, "");
    assert_eq!(attenuationFormattedNotLegitimateSmall, "Enter legimate amounts");

    let abvFloatNotLegitimateHigh: f64 = 39.46206731404319;
    let attenuationFloatNotLegitimateBig: f64 = 139.2143947212355;
    let (abvFormattedNotLegitimateBig, attenuationFormattedNotLegitimateBig) = realABVFormatting(abvFloatNotLegitimateHigh, attenuationFloatNotLegitimateBig);

    assert_eq!(abvFormattedNotLegitimateBig, "");
    assert_eq!(attenuationFormattedNotLegitimateBig, "Enter legimate amounts");
}