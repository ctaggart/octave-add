fn main() {
    println!(r"cargo:rustc-link-search=C:\Octave\Octave-5.1.0.0\mingw64\bin");
    println!(r"cargo:rustc-link-search=C:\Octave\Octave-5.1.0.0\mingw64\lib");
    // println!(r"C:\msys64\mingw64\x86_64-w64-mingw32\lib");
    println!("cargo:rustc-link-lib=octave-7");
    println!("cargo:rustc-link-lib=octinterp-7");
}