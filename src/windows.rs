use std::{io::{BufRead, BufReader, Error}, process::{Output, Stdio}, thread};

use std::io::Write;









pub fn remove_all_spotify_tweaks(){
    std::process::Command::new(
        "powershell")
        .arg("-Command")
        .arg("spicetify restore ; rmdir -r -fo $env:APPDATA\\spicetify; rmdir -r -fo $env:LOCALAPPDATA\\spicetify
        ; Invoke-WebRequest -Uri \"https://raw.githubusercontent.com/SpotX-Official/SpotX/refs/heads/main/Uninstall.bat\" -OutFile \"$env:TEMP\\Uninstall.bat\"
        & \"$env:TEMP\\Uninstall.bat\" ;
        Remove-Item \"$env:TEMP\\Uninstall.bat\" -Force
    ")
        .stdin(Stdio::piped()) // Allow input
        .output()
        .expect("failed to exspecute process");
}

pub fn install_spicetify() -> Result<(), Error> {
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
                        return Ok(())
                    
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


    Err(Error::new(std::io::ErrorKind::Other, "Spicetify failed to install"))
    
}

pub fn install_spotx() {
    let spotx_cmd = std::process::Command::new(
        "powershell")
        .arg("-Command")
        .arg("[Net.ServicePointManager]::SecurityProtocol = 3072; 
    iex \"& { $(iwr -useb 'https://spotx-official.github.io/run.ps1') } -sp-over -podcasts_off -block_update_on\"")
    .output()
    .expect("failed to execute process");

    let spotx_out = String::from_utf8_lossy(&spotx_cmd.stdout);

    print!("{}", spotx_out);

}

pub fn install_soggify() {
    use std::fs;

    let _ = match fs::create_dir("./soggify") {
        Ok(dir) => dir,

        Err(e) => {
            println!("Error creating directory: {}", e);
            if !fs::exists("./soggify").unwrap() {
                println!("the fuck did you do");
                panic!("the fuck?")
            } else {
                todo!("download the zip and add the files ig")
            }
        }
    };
}