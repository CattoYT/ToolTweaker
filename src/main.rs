use clap::{Arg, Command};
mod windows;

fn main() {

    // create cmd and also add args
    let matches = Command::new("tltw")
        .arg(
            Arg::new("removeall")
            .long("removeall")
            .help("Removes all tweaks from Spotify")
            .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("spicetify")
                .long("spicetify")
                .help("Enable spicetify functionality")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("spotx")
                .long("spotx")
                .help("Patch Spotify with spotx")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("soggify")
                .long("soggify")
                .help("Add soggify to Spotify (WINDOWS ONLY!!")
                .action(clap::ArgAction::SetTrue),
        )

        .get_matches();

    

    match std::env::consts::OS {
        "windows" => {

            if matches.get_flag("removeall") {
                println!("Removing tweaks...");
                windows::remove_all_spotify_tweaks();
                // https://raw.githubusercontent.com/SpotX-Official/SpotX/refs/heads/main/Uninstall.bat run this
                return
            }
            
            if matches.get_flag("spotx") {
                println!("Adding spotx to Spotify");
                windows::install_spotx();
            } else {
                println!("Skipping spotx installation");
            }
            
            if matches.get_flag("spicetify") {
                println!("Applying spicetify to Spotify");
                let _ = windows::install_spicetify();
            } else {
                println!("Skipping spicetify installation");
            }

            if matches.get_flag("soggify") {
                println!("Adding soggify to Spotify");
                windows::install_soggify();
            } else {
                println!("Skipping soggify installation");
            }
        }
        
        "linux" => {
            if matches.get_flag("spicetify") {
                println!("Enabling spicetify functionality");
                todo!("Implement spicetify for linux");
            }
        }
        _ => {
            println!("Unsupported OS");
        }
    }

}
