use light::lightFunctions::{singleMCU, beerSRM, grainSRMToLAB, grainLABToXYZ, grainXYZToRGBA};

#[test]
fn singleMCUTest() {
    let volumeInGallons: f64 = 20.0;
    let weightInPounds: f64 = 30.0;
    let grainLVB: f64 = 1.8;

    assert_eq!(singleMCU(volumeInGallons, weightInPounds, grainLVB), 2.7);
}

#[test]
fn beerSRMTest() {
    assert_eq!(beerSRM(8.1), 6.265516962844715);
}

// used 20 gallons and 30lbs of 2-row in a dimple pint
#[test]
fn grainSRMToLABTest() {
    let beerSRM: f64 = 2.94916751482842;
    let glassDiameter: f64 = 9.8;
    let (lOut, aOut, bOut) = grainSRMToLAB(glassDiameter, beerSRM);

    assert_eq!(lOut, 65.29929503106153);
    assert_eq!(aOut, 17.905799803172474);
    assert_eq!(bOut, 76.97459931149983);
}

#[test]
fn grainLABToXYZTest() {
    let lOut: f64 = 65.29929503106153;
    let aOut: f64 = 17.905799803172474;
    let bOut: f64 = 76.97459931149983;
    let (xOut, yOut, zOut) = grainLABToXYZ(lOut, aOut, bOut);

    assert_eq!(xOut, 37.99733219885034);
    assert_eq!(yOut, 34.425984706398424);
    assert_eq!(zOut, 3.4351935539850422);
}

#[test]
fn grainXYZToRGBATest() {
    let xOut: f64 = 37.99733219885034;
    let yOut: f64 = 34.425984706398424;
    let zOut: f64 = 3.4351935539850422;
    let rgbaOutput = grainXYZToRGBA(xOut, yOut, zOut);

    assert_eq!(rgbaOutput.red, 0.842840006924834);
    assert_eq!(rgbaOutput.green, 0.5626270741600671);
    assert_eq!(rgbaOutput.blue, -0.16414442218027928);
    assert_eq!(rgbaOutput.alpha, 1.0);
}