use std::io::Write;
use std::process::Stdio;

pub fn install_spicetify() {
    // This is designed for arch with spotify + yay specifically.

    // Install spicetify over yay

    let mut install_necessary = false;
    match std::process::Command::new("spicetify").arg("apply").spawn() {
        Ok(_) => println!("Executable found!"),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                install_necessary = true;
                println!("Spicetify not found. Installing...");
            } else {
                panic!("WHAT THE FUCK DID YOU DO?");
            }
        }
    }

    if install_necessary {
        let spicetify = std::process::Command::new("yay")
            .arg("-S")
            .arg("--noconfirm")
            .arg("spicetify-cli")
            .stdin(Stdio::piped()) // Allow input
            .stdout(Stdio::piped()) // Allow input
            .spawn()
            .expect("failed to exspecute process");

        let _ = spicetify.wait_with_output();
    }

    let mut spicetify_install_cmd = std::process::Command::new("spicetify")
        .arg("apply")
        .stdin(Stdio::piped())
        .spawn()
        .expect("ermmm");
    if let Some(ref mut stdin) = spicetify_install_cmd.stdin {
        // yeet y for the spicetify marketplace to be installed
        writeln!(stdin, "y").expect("Failed to write to stdin");
    }

    let _ = spicetify_install_cmd.wait();
}

pub fn install_spotx() {
    let mut spotx_install_cmd = std::process::Command::new("bash")
        .arg("\"<(curl -sSL https://spotx-official.github.io/run.sh)\"")
        .spawn()
        .expect("wtf");

    let _ = spotx_install_cmd.wait(); // account for the user not having the perl, unzip, and zip
}
