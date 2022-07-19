use gdk::RGBA;
use functions::commonFunctions::zeroRGBA;

#[test]
fn zeroRGBATest() {
    let new: RGBA = zeroRGBA();

    assert_eq!(new.red, 255.0);
    assert_eq!(new.green, 255.0);
    assert_eq!(new.blue, 255.0);
    assert_eq!(new.alpha, 1.0);
}