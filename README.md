# SwSh_randomizer

Pokemon Sword and Shield randomizer plugin for Nintendo Switch.

## Prerequisites

* A Nintendo Switch capable of running a Custom Firmware
* The latest release of [Atmosphere CFW](https://github.com/Atmosphere-NX/Atmosphere/releases) by [SciresM](https://github.com/SciresM)

## Setup

1. Download the latest [release](https://github.com/Raytwo/swsh_randomizer/releases) and extract the content of the ``sd`` directory at the root of your SD card.
2. Edit your system settings (atmosphere/config/system_settings.ini) and make sure to edit the ``ease_nro_restriction`` line so that it is toggled on. (``ease_nro_restriction = u8!0x1``)

## Usage

* Just run into any trainer or wild encounter
* ???
* Profit!

## Notes

- Species, Form, Gender (still respects gender legality) and Ability are the parameters being randomized. More to come, eventually.
- Please do note that trainer pokemons still have their original movesets for the time being, as we are encountering problems with the checksum.
- Local and online communication is cut off when using the plugin, mostly as a safety measure for now. We might change that at some point.

## Credits
* [jam1garner](https://github.com/jam1garner) - [cargo-skyline](https://github.com/jam1garner/cargo-skyline), [skyline-rs](https://github.com/ultimate-research/skyline-rs), [skyline-rs-template](https://github.com/ultimate-research/skyline-rs-template)
* [shadowninja108](https://github.com/shadowninja108) and the Skyline contributors - [Skyline](https://github.com/shadowninja108/Skyline)
