# hyperrust

RGB utility for HyperX Alloy Origins 60 keyboard

## Installation

```bash
cargo install hyperrust
```

## Usage

```bash
hyperrust --help # show help
hyperrust --color D17A00 # set color to orange, hex format or "#D17A00"
hyperrust -a FF0000 -b FFFFFF # set animated gradient from red to white
hyperrust --theme default # set theme, currently only default is available
hyperrust --rainbow # rainbow effect
hyperrust --rainbow & # run in background
```
## Note
As app needs to be running all the time to keep the effect, it is recommended to run it in background. Otherwise press ctrl+c to close the app and stop the effect.
