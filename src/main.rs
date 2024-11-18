use clap::ArgAction;
use clap::Parser;
use figlet_rs::FIGfont;

mod linux;
mod windows;
#[derive(Parser)]
#[command(name = "ToolTweaker")]
#[command(version = "1.0.2")]
#[command(about = FIGfont::standard().unwrap().convert("ToolTweaker").unwrap().to_string()+
"The stupidest installer for all sorts of tweaks!", long_about = None)]
struct Cli {
    /// Removes all tweaks from Spotify
    #[arg(long, short='r', action = ArgAction::SetTrue)]
    removeall: bool,

    /// Adds all tweaks from Spotify
    #[arg(long, short='a', action = ArgAction::SetTrue)]
    all: bool,

    /// Enable spicetify functionality
    #[arg(long, short='s', action = ArgAction::SetTrue)]
    spicetify: bool,

    /// Patch Spotify with spotx
    #[arg(long, short='x', action = ArgAction::SetTrue)]
    spotx: bool,
    /// Apply Premium patches to SpotX
    #[arg(long, short='p', action = ArgAction::SetTrue)]
    premium_spotify: bool,

    /// Add soggfy to Spotify (WINDOWS ONLY!!) This will also use an outdated, x86_32 version of spotify
    #[arg(long, short='g', action = ArgAction::SetTrue)]
    soggfy: bool,
}
fn main() {
    let cli = Cli::parse();

    let mut nomatch = false;
    if cli.removeall || cli.all || cli.spicetify || cli.spotx || cli.soggfy {
        nomatch = true;
    }
    if !nomatch {
        <Cli as clap::CommandFactory>::command().print_help().unwrap();        return;
    }

    match std::env::consts::OS {
        "windows" => {
            if cli.removeall {
                println!("Removing tweaks...");
                windows::remove_all_spotify_tweaks();
                // https://raw.githubusercontent.com/SpotX-Official/SpotX/refs/heads/main/Uninstall.bat run this
                return;
            }
            if cli.all {
                println!("Adding all tweaks to Spotify");

                windows::install_soggfy();
                println!("Adding spotx to Spotify");
                windows::install_spotx();
                println!("Applying spicetify to Spotify");
                windows::install_spicetify();

                return;
            }

            //really specific order dw about it (this also restricts the spotify version to 1.2.31.1205-x86_32)
            if cli.soggfy {
                println!("Adding soggfy to Spotify");
                windows::install_soggfy();
            } else {
                println!("Skipping soggfy installation");
            }

            if cli.spotx {
                println!("Adding spotx to Spotify");
                windows::install_spotx();
            } else {
                println!("Skipping spotx installation");
            }

            if cli.spicetify {
                println!("Applying spicetify to Spotify");
                windows::install_spicetify();
            } else {
                println!("Skipping spicetify installation");
            }
        }

        "linux" => {
            if cli.spicetify {
                println!("Enabling spicetify functionality");
                linux::install_spicetify();
            }
            if cli.spotx {
                println!("Installing SpotX...");
                linux::install_spotx(cli.premium_spotify);
            }
        }
        _ => {
            println!("Unsupported OS");
        }
    }
}
