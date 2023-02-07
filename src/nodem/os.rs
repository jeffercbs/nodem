use std::env;
use std::path::Path;
use std::process::Command;

pub(crate) struct OsArch {
    pub os: String,
    pub arch: String,
    pub path: String,
}

impl OsArch {
    pub(crate) fn new(os: String, arch: String, path: String) -> Self {
        Self { os, arch, path }
    }
}

pub(crate) fn get_arch() -> OsArch {
    let os: String = env::consts::OS.into();
    let (arch, path) = match os.as_str() {
        "windows" => (
            env::var("PROCESSOR_ARCHITECTURE").unwrap_or("unknown".to_string()),
            Path::new(&env::var("USERPROFILE").unwrap())
                .join("AppData\\Roaming\\nodem")
                .to_str()
                .unwrap()
                .to_owned(),
        ),
        "linux" | "macos" => {
            let opt = Command::new("uname")
                .arg("-m")
                .output()
                .expect("Failed to execute process");
            let arch = String::from_utf8_lossy(&opt.stdout).into_owned();
            let path = Path::new(&env::var("HOME").unwrap())
                .to_str()
                .unwrap()
                .to_owned();

            (arch, path)
        }
        _ => ("".to_string(), "".to_string()),
    };

    OsArch::new(os, arch, path)
}
