use std::ffi::CString;
use std::path::PathBuf;
use std::fs;
named!(
    pub parse_string<&str>,
    map_res!(take_until_and_consume!("\0"), ::std::str::from_utf8)
);


pub fn cheat_dir() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Unable to find home dir");
    let cheat_dir = home_dir.join(".csgohack");
    if !cheat_dir.exists() {
        fs::create_dir(&cheat_dir).expect("Directory creation failed");
    }
    cheat_dir
}

