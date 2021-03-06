#!/bin/sh

cargo build --release &&
sudo install -Dm755 ../target/release/BrewStillery /usr/bin/BrewStillery &&
sudo install -Dm755 ../Arch/BrewStillery.desktop /usr/share/applications/BrewStillery.desktop &&
sudo install -Dm755 ../media/BrewStilleryIcon.svg /usr/share/BrewStillery/BrewStilleryIcon.svg &&
sudo install -Dm755 ../media/BrewStilleryLogo.svg /usr/share/BrewStillery/BrewStilleryLogo.svg &&
sudo install -Ddm755 /usr/share/BrewStillery/glassware &&
sudo install -Dm755 ../media/glassware/*.png /usr/share/BrewStillery/glassware/ &&
sudo install -Ddm755 /usr/share/BrewStillery/buttons &&
sudo install -Dm755 ../media/buttons/*.png /usr/share/BrewStillery/buttons/