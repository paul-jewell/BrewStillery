## New In 6.2.0
  * * Annotated types to all variables
  * * Cleaned up unnecessary logic in `realABVFormatting()`
  * * Updated tests to include all logic cases

## New In 6.1.0
  * * Made use lines explicit
  * * Fixed strict floating point comparisons
  * * Changed `.validInput()` to return NAN instead of 0.0. Also changed all logic to use `.is_nan()` to match.

## New In 6.0.2
  * Changed BrewStilleryIcon to be an svg. Also changed all corresponding files to match. Fixed install script permissions.

## New In 6.0.1
  * Moved `startGTK()` into its own module so the `mod.rs` is just modules.
  * Properly configured the tests module in `main.rs` with the `#[cfg(test)]` flag and removed unused imports in some tests.
  * Updated `gtkCSS()` to the latest `.get_default_screen()` arguments.
  * Moved the `clone!()` macro into `macroDefinitions.rs` so the `mod.rs` is just modules.

## New In 6.0.0
  * Changed from switches to dropdowns for units.
  * Added Imperial (British) units.
  * In Total IBU, changed Boil Time to per hop.
  * Under the bonnet, complete rewrite/cleanup.
  * Broke apart large functions.
  * Wrote tests for all of the Maths, Formatting, Implementations, and Common Functions.

## New In 5.3.1
  * Fixed the percentage display for Real Attenuation.

## New In 5.3.0
  * Added unit tests for verifying maths. To check, run `cargo test`

## New In 5.2.0
  * Created traits and implementations for common functions.
  * Removed the arch PKGBUILD. It's on the AUR.

## New In 5.1.0
  * Made things more rusty, changed two images from svg to png, and code cleanup.

## New In 5.0.0
  * MASSIVE overhaul!!! Hence the jump from Version 3.0.0 to 5.0.0.
  * Huge code cleanup: removed tons of unneeded borrows and things of that nature.
  * Created a complete theme with CSS and images.
  * Switched from a GtkWindow to a GtkApplicationWindow, which allows us to have proper domain naming.
  * Added finished beer colour output:
    * To calculate what the finished beer will look like, the flow goes like this:
        * Individual grains go through singleMCU, those are added up and passed into beerSRM, and that gives us a total SRM value.
        * We then convert that SRM value to L\*ab, from L\*ab to XYZ, and finally to RGBA.
        * That gives us the colour you would expect to see.
    * Also, because of the diameter of a glass, larger vessels will make beer appear darker. So, we accounted for that with standard glassware and averaged lighting data.
        * We now have a dropdown menu where you can pick your glassware and it will show you the appropriate colour in that glass.
  * Switched from an entry/button setup to a dynamic one: everything is calculated on the fly.
  * Refactored our ABV functions. It's now based off of one source, which is very accurate.
  * Changed from using 32 bit floating point to 64 bit.

## New In 3.0.0
  * Added the grain calculator to the beer tab.
  * Reworked the maths in the RealABV calculator, because it was completely inaccurate.
  * Reworked the graphical layout as well.

## New In 2.0.0
  * Added the IBU calculator to the beer tab.