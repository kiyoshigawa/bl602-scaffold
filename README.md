# bl602-scaffold
A basic minimal repo for starting a rust project using the BL602 SOC, and the bl602-hal with serial debug messages.

This repo is based on https://github.com/9names/bl602-rust-example but all new code is my own and to be licensed under the GPLv3 instead of the original repo's MIT licensing.

This is all being built and tested on Windows using the latest stable rust compiler. I'm avoiding nightly builds and unstable features for the time being.

This scaffold supports debug messages over the USB serial connection. It runs at a 2000000 baud rate. By using the heapless library, you're also able to send variables to the serial port even though there is no memory allocator or heap available.

To flash your BL602, you'll need it to be running a bootloader, and be in flashing mode. For my Pinecone BL602 board, I have to swap the jumper on pin IO8 from L to H and reset the board to enter bootloader mode. Once it's in bootloader mode, just run `cargo blflash --release --port=COMXX` and it will compile and upload the code. Then to actually run the program I need to swap the jumper back to L and reset it again.

Alternatively, I've added an alias in the `.cargo/config.toml` which can be run as `cargo flash`. You will need to edit the file to match your actual com port number for it to work properly.

I would eventually like to switch to using the [blash flasher](https://github.com/bjoernQ/blash) but for now I'm going to keep using what works rather than fight the toolchain before I even have LEDs working.
