use functions::commonFunctions::grainToGravity;

#[test]
fn grainToGravityTest() {
    let volumeInGallonsUS: f64 = 20.0;
    let weightInPounds: f64 = 60.0;
    let grainGravity: f64 = 1.037;

    assert_eq!(grainToGravity(volumeInGallonsUS, weightInPounds, grainGravity), 1.06327);
}