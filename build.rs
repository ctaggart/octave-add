use std::env;
fn main() {
    let target = &env::var("TARGET").unwrap();
    // add libraries for linking
    match target.as_str() {
        "x86_64-pc-windows-gnu" => {
            println!(r"cargo:rustc-link-search=C:\Octave\Octave-5.1.0.0\mingw64\bin");
            println!("cargo:rustc-link-lib=octave-7");
            println!("cargo:rustc-link-lib=octinterp-7");
        },
        "x86_64-unknown-linux-gnu" => {
            println!("cargo:rustc-link-search=/snap/octave/5/usr/lib/x86_64-linux-gnu");
            println!("cargo:rustc-link-search=/snap/octave/5/usr/lib/octave/5.1.0");
            println!("cargo:rustc-link-lib=octave");
            println!("cargo:rustc-link-lib=octinterp");
        },
        _ => (),
    }
}