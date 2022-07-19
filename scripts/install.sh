#!/bin/sh

read -p "Enter root password: " -s rootPassword

# cargo build --release &&
echo "$rootPassword" | sudo -S install -Dm755 ../target/release/BrewStillery /usr/bin/BrewStillery &&
echo "$rootPassword" | sudo -S install -Dm644 ../Desktop/BrewStillery.desktop /usr/share/applications/BrewStillery.desktop &&
echo "$rootPassword" | sudo -S install -Dm644 ../media/BrewStilleryIcon.svg /usr/share/icons/hicolor/scalable/apps/BrewStillery.svg &&
echo "$rootPassword" | sudo -S install -Dm644 ../media/BrewStilleryLogo.png /usr/share/BrewStillery/BrewStilleryLogo.png &&
echo "$rootPassword" | sudo -S install -Ddm755 /usr/share/BrewStillery/glassware &&
echo "$rootPassword" | sudo -S install -Dm644 ../media/glassware/*.png /usr/share/BrewStillery/glassware/ &&
echo "$rootPassword" | sudo -S install -Ddm755 /usr/share/BrewStillery/buttons &&
echo "$rootPassword" | sudo -S install -Dm644 ../media/buttons/*.png /usr/share/BrewStillery/buttons/