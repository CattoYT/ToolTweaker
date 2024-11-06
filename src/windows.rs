use std::{io::Error, process::Output};




fn run_with_powershell(cmd: &str) -> Result<Output, Error> {
    let spotx_cmd = std::process::Command::new(
        "powershell")
        .arg("-Command")
        .arg(cmd)
        .output();
        
    

    spotx_cmd
}


pub fn install_spotx() {
    let spotx_cmd = std::process::Command::new(
        "powershell")
        .arg("-Command")
        .arg("[Net.ServicePointManager]::SecurityProtocol = 3072; 
        iex \"& { $(iwr -useb 'https://spotx-official.github.io/run.ps1') } -block_update_on\"")
    .output()
    .expect("failed to execute process");

    let spotx_out = String::from_utf8_lossy(&spotx_cmd.stdout);

    print!("{}", spotx_out);

}

pub fn install_spicetify(){
    
    //check if spicetify is actually installed
    match std::process::Command::new("spicetify").spawn() {
        Ok(_) => println!("Executable found and started."),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                println!("Spicetify not found. Installing...");
                run_with_powershell("iwr -useb https://raw.githubusercontent.com/spicetify/cli/main/install.ps1 | iex").unwrap();}
            _ => println!("Other error occurred: {:?}", e),
        },
    }

    println!("Running spicetify");
    let spicetify_cmd = std::process::Command::new("spicetify")
    .arg("apply")
    .output()
    .expect("failed to execute process");

    let spicetify_output = String::from_utf8_lossy(&spicetify_cmd.stdout);
    print!("{}", spicetify_output);
    // check if there is the "warning" substring inside of spicetify_output
    if spicetify_output.contains("spicetify backup apply") {
        println!("Spicetify failed! Rerunning with backup...");
        let spicetify_backup_cmd = std::process::Command::new("spicetify")
            .arg("backup")
            .arg("apply")
            .output()
            .expect("failed to execute process");

        print!("{}", String::from_utf8_lossy(&spicetify_backup_cmd.stdout));
    } else {
        println!("Spicetify has been applied successfully");
    }
    
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
                std::process::Command::new(
                    "[Net.ServicePointManager]::SecurityProtocol = 3072; 
                    iex \"& { $(iwr -useb 'https://spotx-official.github.io/run.ps1') } -block_update_on\"")
                .output()
                .expect("failed to execute process"
);
            }
        }
    };
}