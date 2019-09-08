use std::process::Command;
use std::str;
use std::env;

fn main() {
    let cmus_call = Command::new("cmus-remote")
        .arg("-Q")
        .output()
        .expect("Failed to query cmus-remote");

    let output = match str::from_utf8(&cmus_call.stdout) {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    };

    let mut artist: Option<String> = None;
    let mut title: Option<String> = None;
    for item in output.split("\n") {
        if item.starts_with("tag artist ") {
            artist = Some(String::from(&item[11..]));
        }
        else if item.starts_with("tag title ") {
            title = Some(String::from(&item[10..]));
        }
    }

    match artist {
        Some(a) => {
            match title {
                Some(t) => println!("{} - {}", a, t),
                None => println!("{}", a)
            }
        }
        None => {
            match title {
                Some(t) => println!("{}", t),
                None => ()
            }
        }
    }

    let env_button = match env::var("BLOCK_BUTTON") {
        Ok(t) => t,
        Err(_) => "".into()
    };

    let arg: Option<String> = match env_button.as_str() {
        "1" => Some("--prev".into()),
        "2" =>  Some("--pause".into()),
        "3" =>  Some("--next".into()),
        _ => None,
    };

    if arg.is_some() {
        let _ = Command::new("cmus-remote")
            .arg(arg.unwrap())
            .spawn()
            .expect("Failed to run cmus-remote");
    }
}
