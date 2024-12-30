use cliclack::spinner;
use colored::Colorize;
use once_cell::sync::Lazy;

pub fn banner() {
    print!(
        "{}",
        r#"
  ____                  _               ____            _       _     ____            _   
 |  _ \   _   _   ___  | |_   _   _    |  _ \    __ _  (_)   __| |   | __ )    ___   | |_ 
 | |_) | | | | | / __| | __| | | | |   | |_) |  / _` | | |  / _` |   |  _ \   / _ \  | __|
 |  _ <  | |_| | \__ \ | |_  | |_| |   |  _ <  | (_| | | | | (_| |   | |_) | | (_) | | |_ 
 |_| \_\  \__,_| |___/  \__|  \__, |   |_| \_\  \__,_| |_|  \__,_|   |____/   \___/   \__|
                              |___/
"#
        .red()
    );

    println!(
        "{}",
        r#"
Made by Nehuén <https://github.com/nehu3n>
"#
        .magenta()
    );
}

pub static SPINNER_CLIENT: Lazy<cliclack::ProgressBar> = Lazy::new(|| spinner());
