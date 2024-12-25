# rdo

A minimal radio player for the terminal.

![rdo screenhot](docs/rdo.gif)

## Usage

Use [Nix](https://nixos.org/) to execute:

```
nix run github:pascalj/rdo
```

## Keymap

- Enter: play selected station
- Space: stop playing
- `j`/`k`: select stations
- `J`/`K`: move selected station
- `n`: new station
- `e`: edit selected station
- `d`: delete selected station
- `q`: quit

## Stations

The stations are saved as CSV in `$XDG_CONFIG_HOME/rdo/stations.csv`, e.g.:

```
name,url
detektor.fm - Wort,https://streams.detektor.fm/wort/mp3-256/website/
radio lclhst,http://radio.lclhst.net/listen.m3u
```
