# ansi-map-dk

A text terminal application written in Rust that fetches elevation GeoTiff data from datafordeler.dk. 
It then uses the data to render a geographic map of Denmark.

## Compiling from source

Assuming you have Rust and Cargo installed:
```sh
git clone https://github.com/iamfrank/ansi-map-dk.git
cd ansi-map-dk
cargo run
```

Then you can run the program in your terminal of choice:
```sh
cd ansi-map-dk/target/debug
./ansi-map-dk
```
