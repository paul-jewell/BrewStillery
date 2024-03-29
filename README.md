# BrewStillery
BrewStillery is a brewer's, vintner's, and distiller's calculator.
It has a multitude of great functions, such as calculating ABV, determining carbonation, and total sparge water needed.

Written in Rust, using GTK3

The original author no longer maintains this package (at least publicly), so I decided to download the source and upload to github in case anyone else is interested in it. 

## New In 6.2.0
  * * Annotated types to all variables
  * * Cleaned up unnecessary logic in `realABVFormatting()`
  * * Updated tests to include all logic cases

## Full Changelog
  * [Available Here](CHANGELOG.md)

## Screenshots:
![General Tab Screenshot](media/screenshots/BrewStilleryGeneralTab.png)

![General Tab Filled In Screenshot](media/screenshots/BrewStilleryGeneralTabFilledIn.png)

![Beer Tab Top Screenshot](media/screenshots/BrewStilleryBeerTabTop.png)

![Beer Tab Top Filled In Screenshot](media/screenshots/BrewStilleryBeerTabTopFilledIn.png)

![Beer Tab Bottom Screenshot](media/screenshots/BrewStilleryBeerTabBottom.png)

![Beer Tab Bottom Screenshot](media/screenshots/BrewStilleryBeerTabBottomFilledIn.png)

![Champagne Tab Screenshot](media/screenshots/BrewStilleryChampagneTab.png)

![Champagne Tab Screenshot](media/screenshots/BrewStilleryChampagneTabFilledIn.png)

![About Tab Screenshot](media/screenshots/BrewStilleryAboutTab.png)


## To Do:
  * Add Sugars And Fruits To ABV From Grain
  * Add Water Minerals Calculator
  * Add Documentation To The Info Tab
  * Add Export To Gourmet Option


## Building:
  To install on Arch, the AUR package name is ```brewstillery```.

  BrewStillery requires GTK 3.22.

  To build, just run ```cargo build --release```. The resultant binary will be ```target/release/BrewStillery```.

  To install/uninstall system-wide, run ```install.sh``` or ```uninstall.sh``` from the scripts directory as your regular user.

  If you're installing with ```cargo install BrewStillery```, the logo image will be missing.

  To fix this, you will need ```BrewStilleryLogo.png``` from the media directory.

  Install it with ```sudo install -Dm755 ../media/BrewStilleryLogo.png /usr/share/BrewStillery/BrewStilleryLogo.png```


## Sources:
  * ["Brew By Numbers - Add Up What's In Your Beer"](https://www.homebrewersassociation.org/attachments/0000/2497/Math_in_Mash_SummerZym95.pdf) - Michael L. Hall. PH.D., Zymurgy (Summer) 1995
  * ["Refractometer FG Results « SeanTerrill.com"](http://seanterrill.com/2011/04/07/refractometer-fg-results/) - SeanTerrill.com, Modified: January 2010
  * ["Kraeusening"](http://www.braukaiser.com/wiki/index.php?title=Kraeusening) - Braukaiser.com, Modified: January 2, 2010
  * ["Glenn Tinseth's Hop Page"](http://realbeer.com/hops/) - Glenn Tinseth, 1995-1999
  * ["Calculating Gravity, Bitterness, And Color: Techniques"](https://byo.com/bock/item/409-calculating-gravity-bitterness-and-color-techniques) - Chris Colby, 2000
  * ["Formulas And C Source Code"](https://web.archive.org/web/20090807084643/http://www.primetab.com:80/formulas) - PrimeTab, Modified: March 25, 2002
  * ["Estimating Color"](http://brewwiki.com/index.php/Estimating_Color) - BrewWiki.com, Modified: May, 17 2008
  * ["Technical Information for Brewers"](http://wetnewf.org/pdfs/Brewing_articles/MOAWorkbook.xls) - A.J. DeLange, 2013
  * ["Color math and programming code examples"](https://www.easyrgb.com/en/math.php) - IRO Group Limited, 2018
