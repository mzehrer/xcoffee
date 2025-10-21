# XCoffee - Trojan Room Coffee Pot Viewer

A Rust implementation inspired by the legendary Trojan Room coffee pot - the world's first webcam. This application displays a live feed from the historical coffee pot now housed at the Heinz Nixdorf MuseumsForum in Paderborn, Germany.

[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

## Features

- **Live Coffee Pot Feed**: Displays continuous MJPEG stream from https://kaffee.hnf.de
- **Trojan View Mode**: Optional filter to simulate the original 128×128 grayscale view
- **Classic X11 Aesthetic**: Dark theme reminiscent of the original xcoffee

## Historical Background

### The Original Trojan Room Coffee Pot (1991-2001)

The Trojan Room coffee pot was a coffee machine located in the Computer Laboratory of the University of Cambridge, England. It became the subject of the world's first webcam, created by Quentin Stafford-Fraser and Paul Jardetzky in 1991[1].

The project began with a practical problem: researchers working in the building were often disappointed after making the trip to the coffee room only to find the coffee pot empty. To solve this issue, a camera was set up to provide a live grayscale picture (128×128 pixels) of the coffee pot to all desktop computers on the office network.

#### Technical Implementation
- A grayscale camera connected to a video capture card in an Acorn Archimedes computer
- Quentin Stafford-Fraser wrote the client software (XCoffee) using X Window System protocol
- Paul Jardetzky wrote the server program

In November 1993, web browsers gained the ability to display images. Computer scientists Daniel Gordon and Martyn Johnson connected the camera to the Internet, making the coffee pot visible worldwide via HTTP[1]. It quickly became a popular landmark of the early World Wide Web, receiving millions of visitors.

After the laboratory's move to the William Gates Building, the webcam was switched off at 09:54 UTC on August 22, 2001. The shutdown received front-page coverage in major newspapers including The Times and The Washington Post.

### Current Location

The Krups coffee maker (the last of several machines used) was auctioned on eBay and purchased by the German news website Der Spiegel for £3,350[1]. After being refurbished by Krups employees, it was displayed in Der Spiegel's editorial office[2].

Since summer 2016, the coffee pot has been on permanent loan to the Heinz Nixdorf MuseumsForum (HNF) in Paderborn, Germany, where it is displayed in the Internet section on the second floor[2]. As in Cambridge, the pot is monitored by a video camera, continuing the tradition started over 30 years ago.

## Building

Make sure you have Rust installed (https://rustup.rs), then run:

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

Or run the compiled binary:

```bash
./target/release/xcoffee
```

## Dependencies

- **iced**: Modern GUI framework for Rust
- **reqwest**: HTTP client for fetching images
- **tokio**: Async runtime
- **image**: Image processing



## Cultural Impact

The Trojan Room coffee pot has had a lasting impact on internet culture[1]:
- Inspired the Hyper Text Coffee Pot Control Protocol (HTCPCP), a 1998 April Fools' Day specification
- Referenced in the video game Hitman 2: Silent Assassin
- Mentioned in the BBC Radio 4 drama The Archers
- Considered a pioneering example of the Internet of Things before the term existed

## Sources

The historical information in this README is based on:

1. [Trojan Room coffee pot - Wikipedia](https://en.wikipedia.org/wiki/Trojan_Room_coffee_pot)
2. [Der Kaffee ist fertig - HNF Blog](https://blog.hnf.de/der-kaffee-ist-fertig/)

## License

MIT
