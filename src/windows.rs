use std::{io::{BufRead, BufReader, Error}, path::Path, process::Stdio, thread};

use std::io::Write;









pub fn remove_all_spotify_tweaks(){
    let _ = std::process::Command::new(
        "powershell")
        .arg("-NonInteractive")
        .arg("-Command")
        
        .arg("spicetify restore ; rmdir -r -fo $env:APPDATA\\spicetify; rmdir -r -fo $env:LOCALAPPDATA\\spicetify
    ")
        .spawn()
        .expect("failed to exspecute process").wait();
    
    let _ = std::process::Command::new(
        "powershell")
        .arg("-NonInteractive")
        .arg("-Command")
        .arg("Invoke-WebRequest -Uri \"https://raw.githubusercontent.com/SpotX-Official/SpotX/refs/heads/main/Uninstall.bat\" -OutFile \"$env:TEMP\\Uninstall.bat\"
       
    ").spawn()
    .expect("failed to exspecute process").wait();

    let _ = remove_pause_from_script(Path::new(&(std::env::var("TEMP").unwrap() + "\\Uninstall.bat")));
    
    let _ = std::process::Command::new(
        "powershell")
        .arg("-NonInteractive")
        .arg("-Command")
        .arg(" & \"$env:TEMP\\Uninstall.bat\" ;
        Remove-Item \"$env:TEMP\\Uninstall.bat\" -Force 
    ").spawn()
        .expect("failed to exspecute process").wait();
    
    match std::fs::remove_file(std::env::var("APPDATA").unwrap() + "\\Spotify\\dpapi.dll") {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to remove dpapi.dll: {}", e),
    }
    
    match std::fs::remove_file(std::env::var("APPDATA").unwrap() + "\\Spotify\\SoggfyUIC.js") {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to remove SoggfyUIC.js: {}", e),
    }
    
    match std::fs::remove_dir_all(std::env::var("LOCALAPPDATA").unwrap() + "\\Soggfy") {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to remove Soggfy directory: {}", e),
    }

}



pub fn install_spicetify() {
    let spicetify_dir = std::env::var("USERPROFILE").unwrap_or_else(|_| "Unknown Home".to_string()) + "\\AppData\\Local\\spicetify";

    //check if spicetify is actually installed
    match std::process::Command::new("spicetify").spawn() {
        Ok(_) => println!("Executable found and started."),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                println!("Spicetify not found. Installing...");

                let mut spicetify = std::process::Command::new(
                    "powershell")
                    .arg("-Command")
                    .arg("iwr -useb https://raw.githubusercontent.com/spicetify/cli/main/install.ps1 | iex")
                    .stdin(Stdio::piped()) // Allow input
                    .spawn()
                    .expect("failed to exspecute process");
                    
                if let Some(ref mut stdin) = spicetify.stdin {
                        // yeet y for the spicetify marketplace to be installed
                    writeln!(stdin, "y").expect("Failed to write to stdin");
                }

                let result: Result<std::process::ExitStatus, Error> = spicetify.wait();
                match result {
                    Ok(_) => {
                        println!("Spicetify installed successfully");
                        return
                    
                    }
                        ,
                    Err(_) =>  {
                        println!("Spicetify failed! Rerunning with backup...");
                        let mut spicetify_backup_cmd = std::process::Command::new(&spicetify_dir)
                            .arg("backup")
                            .arg("apply")

                            .stdout(Stdio::piped())
                            .spawn()
                            .expect("failed to execute process");
                
                            let stdout = spicetify_backup_cmd.stdout.take();
                
                            let output_thread = thread::spawn(move || {
                                let reader = BufReader::new(stdout.unwrap());
                                let mut error_file = std::fs::File::create("spicetify_backup.log").unwrap();
                                //thanks copilot
                                error_file.write_all(reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>().join("\n").as_bytes()).unwrap();
                                
                            });
                            

                
                        let _ = spicetify_backup_cmd.wait().expect("Failed to wait for process");
                        output_thread.join().expect("Output thread panicked");
                    }
                }
            }
            _ => println!("Other error occurred: {:?}", e)
        },
    }

    println!("Running spicetify"); 
}

pub fn install_spotx() {
    let mut spotx_cmd = std::process::Command::new(
        "powershell")
        .arg("-Command")
        .arg("[Net.ServicePointManager]::SecurityProtocol = 3072; 
    iex \"& { $(iwr -useb 'https://spotx-official.github.io/run.ps1') } -v 1.2.31.1205.g4d59ad7c-1561 -podcasts_off -block_update_on\"")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute process");

    let stdin = spotx_cmd.stdin.as_mut().expect("Failed to open stdin");
    let stdout = spotx_cmd.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);


    // Read each line from stdout as it’s generated
    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("Output: {}", line); // Optional: print each line to track progress

                if line.contains("update") {

                    writeln!(stdin, "n").expect("Failed to write 'n' to stdin");

                }
            }
            _ => {
                //i dont care,
        }
    }}

    let _ = spotx_cmd.wait();

}
use tokio::runtime::Runtime;

pub fn install_soggfy() {
    use std::fs;

    match fs::create_dir("./soggfy") {
        Ok(()) => {
            println!("Created directory:" );
        },

        Err(e) => {
            println!("Error creating directory: {}", e);
            if !fs::exists("./soggfy").unwrap() {
                println!("the fuck did you do");
                panic!("the fuck?")
            }
        }
    };
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match download_file("https://github.com/Rafiuth/Soggfy/releases/download/v2.7.3/Soggfy-2.7.3.zip", "soggfy").await {
            Ok(()) => {

                println!("Download completed. Extracting...");
                zip_extract::extract(
                std::fs::File::open("soggfy/Soggfy-2.7.3.zip").unwrap(), 
                Path::new("soggfy/"), 
                false).unwrap(); //im very glad that im the only one working on this

                println!("Extracted soggfy. Installing...");
                let _ = std::fs::remove_file("soggfy/Soggfy-2.7.3.zip");
            },


            Err(e) => println!("Download failed: {:?}", e),
        }

    });//hardcoded link
    println!("Installing soggfy. This will also install ffmpeg so please wait!");

    let _ = remove_pause_from_script(Path::new("./soggfy/Install.ps1"));
    let mut soggfy_install_cmd = std::process::Command::new(
        "powershell.exe")
        .args(["-executionpolicy", "bypass", "-file", "./soggfy/Install.ps1"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())

    .spawn()
    .expect("failed to execute process");

    let stdin = soggfy_install_cmd.stdin.as_mut().expect("Failed to open stdin");
    let stdout = soggfy_install_cmd.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);


    // Read each line from stdout as it’s generated
    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("Output: {}", line); // Optional: print each line to track progress

                if line.contains("Do you want to install SpotX?") {

                    writeln!(stdin, "n").expect("Failed to write 'n' to stdin");

                }
                if line.contains("1205") {
                    writeln!(stdin, "y").expect("Failed to write 'y' to stdin");
                }
            }
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }

    // Wait for the command to finish
    let _ = soggfy_install_cmd.wait().expect("Failed to wait for process");
 

}

use std::path::PathBuf;

use trauma::{download::Download, downloader::DownloaderBuilder};


async fn download_file(link : &str, location : &str) -> Result<(), Error> {
    let downloads = vec![Download::try_from(link).unwrap()];
    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from(location))
        .build();
    downloader.download(&downloads).await;
    
    Ok(())
}

fn remove_pause_from_script(file_path: &Path) -> std::io::Result<()> {
    // Open the file for reading
    let file = std::fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    
    // Read lines except the last "Pause" if it exists
    let mut lines: Vec<String> = reader.lines()
        .map_while(Result::ok)
        .collect();

    // Check if the last line is "Pause" and remove it
    if let Some(last_line) = lines.last() {
        if last_line.trim().eq_ignore_ascii_case("pause") {
            lines.pop();
        }
    }

    // Write the modified lines back to the file
    let mut file = std::fs::File::create(file_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}