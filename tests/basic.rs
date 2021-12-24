use cap_std::fs::OpenOptions;
use open_ambient::{open_ambient_dir, open_ambient_file, open_ambient_file_with};

#[test]
fn basic() {
    let _file_err = open_ambient_file!("lmot.ograC").unwrap_err();
    let _file = open_ambient_file!("Cargo.toml").unwrap();
    let _file_with_err =
        open_ambient_file_with!("lmot.ograC", OpenOptions::new().read(true)).unwrap_err();
    let _file_with = open_ambient_file_with!("Cargo.toml", OpenOptions::new().read(true)).unwrap();
    let _dir_err = open_ambient_dir!("Cargo.toml").unwrap_err();
    let _dir = open_ambient_dir!(".").unwrap();
}
