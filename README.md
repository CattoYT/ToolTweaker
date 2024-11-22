```
  _____                   _   _____                             _   
 |_   _|   ___     ___   | | |_   _| __      __   ___    __ _  | | __   ___   _ __   
   | |    / _ \   / _ \  | |   | |   \ \ /\ / /  / _ \  / _` | | |/ /  / _ \ | '__|   
   | |   | (_) | | (_) | | |   | |    \ V  V /  |  __/ | (_| | |   <  |  __/ | |   
   |_|    \___/   \___/  |_|   |_|     \_/\_/    \___|  \__,_| |_|\_\  \___| |_|   
```
A small rust cli app made to install Soggify, Spicetify, and SpotX, completely automatically!   

## Installation:
Run the following using cargo:   
```cargo install ToolTweaker```

Or build from source:
```
git clone https://github.com/CattoYT/ToolTweaker
cd ToolTweaker
cargo build --release
```

## Usage:
Options:   
  -r, --removeall        Removes all tweaks from Spotify   
  -a, --all              Adds all tweaks from Spotify   
  -s, --spicetify        Enable spicetify functionality   
  -x, --spotx            Patch Spotify with spotx   
  -p, --premium-spotify  Apply Premium patches to SpotX   
  -g, --soggfy           Add soggfy to Spotify (WINDOWS ONLY!!) This will also use an outdated, x86_32 version of spotify   
  -h, --help             Print help   
  -V, --version          Print version   

Made for Hack Club's Rust YSWS and High Seas!


Tested on Arch Linux (6.11.7) with spotify 1.2.48.405-1 
 and Windows 10 22h2 with spotify 1.2.31.1205.g4d59ad7c (Latest supported by Soggfy)



## Credits

- [Spicetify](https://spicetify.app/)
- [SpotX](https://github.com/SpotX-Official)
- [Soggfy](https://github.com/Rafiuth/Soggfy)
