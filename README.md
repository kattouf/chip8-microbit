# Embedded CHIP-8 Interpreter

This is a CHIP-8 interpreter written in Rust that runs on a micro:bit board. Programs are not embedded in the interpreter binary and are instead loaded at runtime via a serial interface.

## Demo

// Add video or photo of the interpreter running

## Project Structure

- `chip8` - interpreter itself
- `rom-transfer` - CLI tool for uploading CHIP-8 ROM to micro:bit via serial interface
- `scp` - Simple(Stupid) Communication Protocol, used for communication between micro:bit and PC

## Sources

- [CHIP‐8 Instruction Set](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set)
- [High-level guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)

## TODO

* [ ] To avoid flickering when moving objects, draw one frame late and compare it with the previous frame to only draw the difference.
* [ ] Save the last loaded program (Hardware support is available but not implemented in the board crate).
* [ ] Implement execution ticks via interrupts.
* [ ] Replace timers with RTIC for improved energy efficiency.
* [ ] Add the ability to select programs via UI.
* [ ] Implement error handling via UI.
