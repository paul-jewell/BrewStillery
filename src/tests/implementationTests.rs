use functions::commonFunctions::{inputMatching, imperialOrMetric, singleInput};
use std::f64::NAN;

#[test]
fn validInputTest() {
    let emptyString: &str = "";
    let validString: &str = "3.0";
    assert_eq!(emptyString.validInput().is_nan(), NAN.is_nan());
    assert_eq!(validString.validInput(), 3.0);
}

#[test]
fn grainGravityTest() {
    let twoRow: &str = "2-Row";
    let sixRow: &str = "6-Row";
    let black: &str = "Black";
    let caramelSixty: &str = "Caramel 60";
    let caramelEighty: &str = "Caramel 80";
    let caramelOneTwenty: &str = "Caramel 120";
    let chocolate: &str = "Chocolate";
    let corn: &str = "Corn";
    let dextrine: &str = "Dextrine";
    let oat: &str = "Oat";
    let rice: &str = "Rice";
    let wheat: &str = "Wheat";
    let invalidString: &str = "";

    assert_eq!(twoRow.grainGravity(), 1.037);
    assert_eq!(sixRow.grainGravity(), 1.035);
    assert_eq!(black.grainGravity(), 1.025);
    assert_eq!(caramelSixty.grainGravity(), 1.034);
    assert_eq!(caramelEighty.grainGravity(), 1.034);
    assert_eq!(caramelOneTwenty.grainGravity(), 1.033);
    assert_eq!(chocolate.grainGravity(), 1.034);
    assert_eq!(corn.grainGravity(), 1.037);
    assert_eq!(dextrine.grainGravity(), 1.033);
    assert_eq!(oat.grainGravity(), 1.034);
    assert_eq!(rice.grainGravity(), 1.032);
    assert_eq!(wheat.grainGravity(), 1.036);
    assert_eq!(invalidString.grainGravity(), 1.0);
}

#[test]
fn grainLovibondTest() {
    let twoRow: &str = "2-Row";
    let sixRow: &str = "6-Row";
    let black: &str = "Black";
    let caramelSixty: &str = "Caramel 60";
    let caramelEighty: &str = "Caramel 80";
    let caramelOneTwenty: &str = "Caramel 120";
    let chocolate: &str = "Chocolate";
    let corn: &str = "Corn";
    let dextrine: &str = "Dextrine";
    let oat: &str = "Oat";
    let rice: &str = "Rice";
    let wheat: &str = "Wheat";
    let invalidString: &str = "";

    assert_eq!(twoRow.grainLovibond(), 1.8);
    assert_eq!(sixRow.grainLovibond(), 1.8);
    assert_eq!(black.grainLovibond(), 500.0);
    assert_eq!(caramelSixty.grainLovibond(), 60.0);
    assert_eq!(caramelEighty.grainLovibond(), 80.0);
    assert_eq!(caramelOneTwenty.grainLovibond(), 120.0);
    assert_eq!(chocolate.grainLovibond(), 350.0);
    assert_eq!(corn.grainLovibond(), 1.3);
    assert_eq!(dextrine.grainLovibond(), 1.5);
    assert_eq!(oat.grainLovibond(), 1.0);
    assert_eq!(rice.grainLovibond(), 1.0);
    assert_eq!(wheat.grainLovibond(), 1.6);
    assert_eq!(invalidString.grainLovibond(), 0.0);
}

#[test]
fn glassSizeTest() {
    let dimplePint: &str = "Dimple Pint";
    let nonickPint: &str = "Nonick Pint";
    let tulipPint: &str = "Tulip Pint";
    let pilsner: &str = "Pilsner";
    let mafs: &str = "Maß";
    let dimpleHalfPint: &str = "Dimple Half Pint";
    let nonickHalfPint: &str = "Nonick Half Pint";
    let tulipHalfPint: &str = "Tulip Half Pint";
    let invalidString: &str = "";

    assert_eq!(dimplePint.glassSize(), 9.8);
    assert_eq!(nonickPint.glassSize(), 9.0);
    assert_eq!(tulipPint.glassSize(), 8.5);
    assert_eq!(pilsner.glassSize(), 8.0);
    assert_eq!(mafs.glassSize(), 10.5);
    assert_eq!(dimpleHalfPint.glassSize(), 8.0);
    assert_eq!(nonickHalfPint.glassSize(), 7.0);
    assert_eq!(tulipHalfPint.glassSize(), 7.2);
    assert_eq!(invalidString.glassSize(), 9.8);
}

#[test]
fn unitMatchTest() {
    let imperialGB: &str = "imperialGB";
    let imperialUS: &str = "imperialUS";
    let metric: &str = "metric";
    let invalidString: &str = "";

    assert_eq!(imperialGB.unitMatch(), imperialOrMetric::imperialGB);
    assert_eq!(imperialUS.unitMatch(), imperialOrMetric::imperialUS);
    assert_eq!(metric.unitMatch(), imperialOrMetric::metric);
    assert_eq!(invalidString.unitMatch(), imperialOrMetric::imperialGB);
}

#[test]
fn brixToGravityTest() {
    assert_eq!(21.0.brixToGravity(), 1.0874528357576327);
}

#[test]
fn gravityToBrixTest() {
    assert_eq!(1.0874528357576327.gravityToBrix(), 21.000000205000255);
}

#[test]
fn gravityToPlatoTest() {
    assert_eq!(1.014.gravityToPlato(), 3.5740240997679393);
}

#[test]
fn gramsToOuncesTest() {
    assert_eq!(300.0.gramsToOunces(), 10.5821886);
}

#[test]
fn poundsToOuncesTest() {
    assert_eq!(3.5.poundsToOunces(), 56.0);
}

#[test]
fn ouncesToPoundsTest() {
    assert_eq!(16.0.ouncesToPounds(), 1.0);
}

#[test]
fn kilosToPoundsTest() {
    assert_eq!(45.0.kilosToPounds(), 99.20801799);
}

#[test]
fn poundsToKilosTest() {
    assert_eq!(99.20801799.poundsToKilos(), 45.00000000308674);
}

#[test]
fn gramsToKilogramsTest() {
    assert_eq!(1000.0.gramsToKilograms(), 1.0);
}

#[test]
fn litresToGallonsGBTest() {
    assert_eq!(75.0.litresToGallonsGB(), 16.4976936);
}

#[test]
fn litresToGallonsUSTest() {
    assert_eq!(75.0.litresToGallonsUS(), 19.8129039);
}

#[test]
fn gallonsGBToLitresTest() {
    assert_eq!(20.0.gallonsGBToLitres(), 90.9218);
}

#[test]
fn gallonsUSToLitresTest() {
    assert_eq!(20.0.gallonsUSToLitres(), 75.70823568);
}

#[test]
fn gallonsGBToGallonsUSTest() {
    assert_eq!(20.0.gallonsGBToGallonsUS(), 24.019);
}

#[test]
fn gallonsUSToGallonsGBTest() {
    assert_eq!(20.0.gallonsUSToGallonsGB(), 16.653484);
}

#[test]
fn sugarToHoneyTest() {
    assert_eq!(1.0.sugarToHoney(), 1.219512195);
}

#[test]
fn volumeToSugarChampagneTest() {
    assert_eq!(5.0.volumeToSugarChampagne(), 1.0);
}

#[test]
fn remainingOuncesTest() {
    assert_eq!(1.2.remainingOunces(), 3.1999999999999993);
}