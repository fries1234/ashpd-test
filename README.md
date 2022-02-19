# ashpd-test
ASHPD Trash test. This program should create a file in the same directory that you are in and trash it using ASHPD's Trash portal.

## Install the Flatpak
```
sudo flatpak-builder --install --force-clean build com.github.fries1234.ashpd-test.json
``` 
## Build it without Flatpak
```
meson build
ninja -C build
```
## Running with Flatpak
```
flatpak run com.github.fries1234.ashpd-test
```
## Running without Flatpak
```
build/src/ashpd-test
```
