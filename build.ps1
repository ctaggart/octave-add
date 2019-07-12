$env:PATH="C:\Octave\Octave-5.1.0.0\mingw64\bin;$env:PATH"
# $env:PATH="C:\msys64\mingw64\bin;$env:PATH"
$env:RUSTFLAGS="-Clink-arg=-v"
# $env:RUSTFLAGS="-Clinker=lld -Clink-arg=-v"
# rm -r ~/.xargo
# xargo clean
# xargo build --target x86_64-pc-windows-gnu --release
cargo clean
cargo build --target x86_64-pc-windows-gnu --release