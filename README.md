# ansi-map-dk

A text terminal application written in Rust that fetches elevation GeoTiff data from datafordeler.dk. 
It then uses the data to render a geographic map of Denmark.

## Compiling from source

You'll need [Rust/Cargo](https://www.rust-lang.org/learn/get-started) to compile from source.

Clone the repository:
```sh
git clone https://github.com/iamfrank/ansi-map-dk.git
cd ansi-map-dk
```

**You'll need to set some environment variables before compiling.**

This includes setting a username and password for accessing data from datafordeler.dk.

The application can read environment variables from an `.env` file. So you can just copy the `.env.example` file and edit it to your liking before compiling.

Now you can compile the application.
```sh
cargo run
```

Then you can run the program in your terminal of choice:
```sh
cd ansi-map-dk/target/debug
./ansi-map-dk
```
