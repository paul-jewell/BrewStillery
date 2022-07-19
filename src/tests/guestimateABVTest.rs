use functions::guestimateABV::guestimateABVFormatting;

#[test]
fn guestimateABVFormattingTest() {
    let startingBrix: f64 = 13.0;

    assert_eq!(guestimateABVFormatting(startingBrix), "6.01%");
}