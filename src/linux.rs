use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    match std::env::consts::OS {
        "windows" => {
            
        }
        _ => todo!()
    }

    let spicetify_cmd = Command::new("spicetify")
        .arg("apply")
        .output()
        .expect("failed to execute process"
    );

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--premium" => {
                println!("Skipping spotx!");
            },
            _ => {
                println!("");
                let spotx_cmd = Command::new("spotx")
                .arg("")
                
                .output()
                .expect("failed to execute process");
                println!("Running spotx");
                print!("{}", String::from_utf8_lossy(&spotx_cmd.stdout));
            }
        }

    }



    println!("Running spicetify");
    let spicetify_output = String::from_utf8_lossy(&spicetify_cmd.stdout);
    print!("{}", spicetify_output);
    // check if there is the "warning" substring inside of spicetify_output
    if spicetify_output.contains("spicetify backup apply") {
        println!("Spicetify failed! Rerunning with backup...");
        let spicetify_backup_cmd = Command::new("spicetify")
            .arg("backup")
            .arg("apply")
            .output()
            .expect("failed to execute process");
            
        print!("{}", String::from_utf8_lossy(&spicetify_backup_cmd.stdout));
    } else {
        println!("Spicetify has been applied successfully");
    }
}