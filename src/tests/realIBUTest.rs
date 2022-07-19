use functions::commonFunctions::realIBU;

#[test]
fn realIBUTest() {
    let brix: f64 = 20.0;
    let wortVolume: f64 = 20.0;
    let alphaAcid: f64 = 7.0;
    let hopAmount: f64 = 3.0;
    let boilTime: f64 = 60.0;
    let output: f64 = realIBU(brix, wortVolume, alphaAcid, hopAmount, boilTime);

    assert_eq!(output, 13.486826596469873);
}