use flate2::read::GzDecoder;
use std::env::consts;
use std::path::Path;
use tar::Archive;

fn main() {
    if !Path::new("./lib/release/lib/libpdfium.a").exists() {
        let resp = reqwest::blocking::get(format!(
            "https://github.com/paulocoutinhox/pdfium-lib/releases/download/7623/{}.tgz",
            consts::OS
        ))
        .expect("Failed to download pdfium lib");
        let ungz = GzDecoder::new(resp);
        let mut untar = Archive::new(ungz);
        untar
            .unpack("./lib")
            .expect("Failed to unarchive pdfium lib");
    }
    println!("cargo:rustc-link-lib=static=pdfium");
    println!("cargo:rustc-link-search=native=./lib/release/lib/");
    if consts::OS == "macos" {
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
    }
}
