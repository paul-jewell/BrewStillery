#!/bin/sh

read -p "Enter root password: " -s rootPassword

echo "$rootPassword" | sudo -S rm /usr/bin/BrewStillery &&
echo "$rootPassword" | sudo -S rm /usr/share/applications/BrewStillery.desktop &&
echo "$rootPassword" | sudo -S rm /usr/share/icons/hicolor/scalable/apps/BrewStillery.svg &&
echo "$rootPassword" | sudo -S rm -rf /usr/share/BrewStillery