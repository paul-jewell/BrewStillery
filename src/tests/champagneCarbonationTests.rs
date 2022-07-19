use functions::commonFunctions::imperialOrMetric;
use functions::champagneCarbonation::{champagneCarbonationMaths, champagneCarbonationFormatting};

#[test]
fn champagneMathsImperialGBTest() {
    let champagneVolume: f64 = 20.0;
    let imperialOrMetric: imperialOrMetric = imperialOrMetric::imperialGB;

    assert_eq!(champagneCarbonationMaths(champagneVolume, imperialOrMetric), 4.8038);
}

#[test]
fn champagneMathsImperialUSTest() {
    let champagneVolume: f64 = 20.0;
    let imperialOrMetric: imperialOrMetric = imperialOrMetric::imperialUS;

    assert_eq!(champagneCarbonationMaths(champagneVolume, imperialOrMetric), 4.0);
}

#[test]
fn champagneMathsMetricTest() {
    let champagneVolume: f64 = 60.0;
    let imperialOrMetric: imperialOrMetric = imperialOrMetric::metric;

    assert_eq!(champagneCarbonationMaths(champagneVolume, imperialOrMetric), 1.437917125853319);
}

#[test]
fn champagneFormattingGBTest() {
    let totalSugarUnderOne: f64 = 0.72057;
    let totalSugarOne: f64 = 0.9967885000000001;
    let totalSugarPoundWithOunces: f64 = 1.68133;
    let totalSugarPounds: f64 = 4.00012426;
    let totalSugarPoundsWithOunces: f64 = 4.8038;
    let imperialOrMetric: imperialOrMetric = imperialOrMetric::imperialGB;

    assert_eq!(champagneCarbonationFormatting(totalSugarUnderOne, imperialOrMetric), "11.53 oz");
    assert_eq!(champagneCarbonationFormatting(totalSugarOne, imperialOrMetric), "1 lb");
    assert_eq!(champagneCarbonationFormatting(totalSugarPoundWithOunces, imperialOrMetric), "1 lb 10.90 oz");
    assert_eq!(champagneCarbonationFormatting(totalSugarPounds, imperialOrMetric), "4 lbs");
    assert_eq!(champagneCarbonationFormatting(totalSugarPoundsWithOunces, imperialOrMetric), "4 lbs 12.86 oz");
}

#[test]
fn champagneFormattingUSTest() {
    let totalSugarUnderOne: f64 = 0.6000000000000001;
    let totalSugarOne: f64 = 1.0;
    let totalSugarPoundWithOunces: f64 = 1.4000000000000001;
    let totalSugarPounds: f64 = 4.0;
    let totalSugarPoundsWithOunces: f64 = 4.6000000000000005;
    let imperialOrMetric: imperialOrMetric = imperialOrMetric::imperialUS;

    assert_eq!(champagneCarbonationFormatting(totalSugarUnderOne, imperialOrMetric), "9.60 oz");
    assert_eq!(champagneCarbonationFormatting(totalSugarOne, imperialOrMetric), "1 lb");
    assert_eq!(champagneCarbonationFormatting(totalSugarPoundWithOunces, imperialOrMetric), "1 lb 6.40 oz");
    assert_eq!(champagneCarbonationFormatting(totalSugarPounds, imperialOrMetric), "4 lbs");
    assert_eq!(champagneCarbonationFormatting(totalSugarPoundsWithOunces, imperialOrMetric), "4 lbs 9.60 oz");
}

#[test]
fn champagneFormattingMetricTest() {
    let totalSugarOne: f64 = 0.9969558739249678;
    let totalSugarWithDecimals: f64 = 1.437917125853319;
    let totalSugarWithoutDecimals: f64 = 2.0011013334792023;
    let imperialOrMetric: imperialOrMetric = imperialOrMetric::metric;

    assert_eq!(champagneCarbonationFormatting(totalSugarOne, imperialOrMetric), "1 kilo");
    assert_eq!(champagneCarbonationFormatting(totalSugarWithDecimals, imperialOrMetric), "1.44 kilos");
    assert_eq!(champagneCarbonationFormatting(totalSugarWithoutDecimals, imperialOrMetric), "2 kilos");
}