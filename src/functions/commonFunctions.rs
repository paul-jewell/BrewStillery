use std::f64::consts::E;
use gtk;
use gdk::RGBA;
use std::f64::NAN;

pub const FINAL_GRAVITY_IDEAL: f64 = 1.01627;
// when finalGravity is unknown, this constant is ideal. it is the average of all BJCP styles

pub const FINAL_BRIX_IDEAL: f64 = 4.1480675;
// when finalBrix is unknown, this constant is ideal. it is the average of all BJCP styles

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum imperialOrMetric {
    imperialGB,
    imperialUS,
    metric,
}

#[derive(Clone, Debug)]
pub struct colourOverlay {
    pub overlay: gtk::Overlay,
    pub colourOutput: gtk::ColorButton,
    pub dimplePint: gtk::Image,
    pub nonickPint: gtk::Image,
    pub tulipPint: gtk::Image,
    pub pilsner: gtk::Image,
    pub mafs: gtk::Image,
    pub dimpleHalfPint: gtk::Image,
    pub nonickHalfPint: gtk::Image,
    pub tulipHalfPint: gtk::Image,
}

pub trait allOverlays {
    fn new(&self) -> colourOverlay;
}

impl allOverlays for gtk::Builder {
    fn new(&self) -> colourOverlay {
        let allOverlays: colourOverlay = colourOverlay {
            overlay: self.get_object("grainABVColourOverlay").expect("commonFunctions, allOverlays"),
            colourOutput: gtk::ColorButton::new(),
            dimplePint: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/Dimple.png"),
            nonickPint: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/Nonick.png"),
            tulipPint: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/Tulip.png"),
            pilsner: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/Pilsner.png"),
            mafs: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/Mafs.png"),
            dimpleHalfPint: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/DimpleHalf.png"),
            nonickHalfPint: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/NonickHalf.png"),
            tulipHalfPint: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/TulipHalf.png"),
        };

        allOverlays
    }
}

pub trait inputMatching {
    fn validInput(&self) -> f64;
    fn grainGravity(&self) -> f64;
    fn grainLovibond(&self) -> f64;
    fn glassSize(&self) -> f64;
    fn unitMatch(&self) -> imperialOrMetric;
}

impl inputMatching for str {
    fn validInput(&self) -> f64 {
        match self.parse::<f64>() {
            Ok(number) => number,
            Err(_) => NAN,
        }
    }

    fn grainGravity(&self) -> f64 {
        match self {
            "2-Row" => 1.037,
            "6-Row" => 1.035,
            "Black" => 1.025,
            "Caramel 60" => 1.034,
            "Caramel 80" => 1.034,
            "Caramel 120" => 1.033,
            "Chocolate" => 1.034,
            "Corn" => 1.037,
            "Dextrine" => 1.033,
            "Oat" => 1.034,
            "Rice" => 1.032,
            "Rye" => 1.036,
            "Wheat" => 1.036,
            _ => 1.0,
            // Gravity set to water as a catch
        }
    }

    fn grainLovibond(&self) -> f64 {
        match self {
            "2-Row" => 1.8,
            "6-Row" => 1.8,
            "Black" => 500.0,
            "Caramel 60" => 60.0,
            "Caramel 80" => 80.0,
            "Caramel 120" => 120.0,
            "Chocolate" => 350.0,
            "Corn" => 1.3,
            "Dextrine" => 1.5,
            "Oat" => 1.0,
            "Rice" => 1.0,
            "Rye" => 2.0,
            "Wheat" => 1.6,
            _ => 0.0,
            // Color to nothing as a catch
        }
    }

    fn glassSize(&self) -> f64 {
        match self {
            // Value is in centimeters
            "Dimple Pint" => 9.8,
            "Nonick Pint" => 9.0,
            "Tulip Pint" => 8.5,
            "Pilsner" => 8.0,
            "Maß" => 10.5,
            "Dimple Half Pint" => 8.0,
            "Nonick Half Pint" => 7.0,
            "Tulip Half Pint" => 7.2,
            _ => 9.8,
            // Size to Dimple Pint as a catch
        }
    }

    fn unitMatch(&self) -> imperialOrMetric {
        match self {
            "imperialGB" => imperialOrMetric::imperialGB,
            "imperialUS" => imperialOrMetric::imperialUS,
            "metric" => imperialOrMetric::metric,
            _ => imperialOrMetric::imperialGB,
            // Unit set to imperialGB as a catch
        }
    }
}

pub trait singleInput {
    fn brixToGravity(&self) -> f64;
    fn gravityToBrix(&self) -> f64;
    fn gravityToPlato(&self) -> f64;
    fn gramsToOunces(&self) -> f64;
    fn poundsToOunces(&self) -> f64;
    fn ouncesToPounds(&self) -> f64;
    fn kilosToPounds(&self) -> f64;
    fn poundsToKilos(&self) -> f64;
    fn gramsToKilograms(&self) -> f64;
    fn litresToGallonsGB(&self) -> f64;
    fn litresToGallonsUS(&self) -> f64;
    fn gallonsGBToLitres(&self) -> f64;
    fn gallonsUSToLitres(&self) -> f64;
    fn gallonsGBToGallonsUS(&self) -> f64;
    fn gallonsUSToGallonsGB(&self) -> f64;
    fn sugarToHoney(&self) -> f64;
    fn volumeToSugarChampagne(&self) -> f64;
    fn remainingOunces(&self) -> f64;
}

impl singleInput for f64 {
    fn brixToGravity(&self) -> f64 {
        (self / (258.6 - ((self / 258.2) * 227.1))) + 1.0
    }

    fn gravityToBrix(&self) -> f64 {
        ((258.6 * self) - 258.6) / ((0.87955073 * self) + 0.12044926)
    }

    fn gravityToPlato(&self) -> f64 {
        135.997 * self.powi(3) - 630.272 * self.powi(2) + 1111.14 * self - 616.868
    }

    fn gramsToOunces(&self) -> f64 {
        0.035273962 * self
    }

    fn poundsToOunces(&self) -> f64 {
        self * 16.0
    }

    fn ouncesToPounds(&self) -> f64 {
        self / 16.0
    }

    fn kilosToPounds(&self) -> f64 {
        2.204622622 * self
    }

    fn poundsToKilos(&self) -> f64 {
        0.45359237 * self
    }

    fn gramsToKilograms(&self) -> f64 {
        self / 1000.0
    }

    fn litresToGallonsGB(&self) -> f64 {
        0.219969248 * self
    }

    fn litresToGallonsUS(&self) -> f64 {
        0.264172052 * self
    }

    fn gallonsGBToLitres(&self) -> f64 {
        4.54609 * self
    }

    fn gallonsUSToLitres(&self) -> f64 {
        3.785411784 * self
    }

    fn gallonsGBToGallonsUS(&self) -> f64 {
        1.200950 * self
    }

    fn gallonsUSToGallonsGB(&self) -> f64 {
        0.8326742 * self
    }

    fn sugarToHoney(&self) -> f64 {
        // 82 grams of sugar equals 100 grams of honey
        1.219512195 * self
    }

    fn volumeToSugarChampagne(&self) -> f64 {
        // 1 pound of sugar carbonates 5 US gallons at 6 atmospheres of pressure, which is standard champagne carbonation
        0.2 * self
    }

    fn remainingOunces(&self) -> f64 {
        self.fract() * 16.0
    }
}

pub fn realIBU(brix: f64, wortVolume: f64, alphaAcid: f64, hopAmount: f64, boilTime: f64) -> f64 {
    (1.65 * 0.000125_f64.powf(brix.brixToGravity() - 1.0)) * ((1.0 - E.powf(-0.04 * boilTime)) / 4.15) * (( (alphaAcid / 100.0) * hopAmount * 7490.0 ) / wortVolume)
}

pub fn realABVAndAttenuation(startingBrix: f64, finalBrix: f64) -> (f64, f64) {
    let wortCorrectionFactor: f64 = 1.040;
    let initialRefractiveIndex: f64 = startingBrix / wortCorrectionFactor;
    let finalRefractiveIndex: f64 = finalBrix / wortCorrectionFactor;

    let newCubic: f64 = (1.0 - 0.0044992999999999995 * initialRefractiveIndex) + (0.0117741 * finalRefractiveIndex) + (0.000275806 * initialRefractiveIndex.powi(2)) - (0.00127169 * finalRefractiveIndex.powi(2)) - (7.27999e-06 * initialRefractiveIndex.powi(3)) + (6.32929e-05 * finalRefractiveIndex.powi(3));

    let apparentAttenuation: f64 = 1.0 - (0.18080000000000002 * (668.72 * newCubic - 463.37 - 205.347 * newCubic.powi(2)) + 0.8192 * (668.72 * newCubic - 463.37 - 205.347 * newCubic.powi(2))) / (initialRefractiveIndex);

    let attenuation: f64 = apparentAttenuation * 0.8192 * 100.0;

    let abv: f64 = ((0.01 / 0.8192) * (initialRefractiveIndex - (0.18080000000000002 * initialRefractiveIndex + 0.8192 * (668.72 * newCubic - 463.37 - 205.347 * newCubic.powi(2)))) / (2.0665-0.010665 * initialRefractiveIndex)) * 100.0;

    (abv, attenuation)
}

pub fn grainToGravity(volumeInGallonsUS: f64, weightInPounds: f64, grainGravity: f64) -> f64 {
    let extractPotential: f64 = grainGravity.fract() * 1000.0;
    let extractionEfficiency: f64 = 0.57;
    // sane default here of 57%
    let mut originalGravity: f64 = 1.0;
    // set to the gravity of water

    let specificGravityPoints: f64 = (weightInPounds * extractPotential * extractionEfficiency) / volumeInGallonsUS;

    if specificGravityPoints != 0.0 {
        originalGravity = specificGravityPoints / 1000.0 + 1.0;
        originalGravity
    } else {
        originalGravity
    }
}

// when const fn hits stable, we can create a constant called RGBA_ZERO that uses zeroRGBA()
pub fn zeroRGBA() -> RGBA {
    RGBA {
        red: 255.0,
        green: 255.0,
        blue: 255.0,
        alpha: 1.0,
    }
}