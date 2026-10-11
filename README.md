# Green Nintendo Entertainment System (GreenNES)

GreenNES is a hobbyist emulator for the Nintendo Entertainment System (NES)
written in Rust. The emulator in its current state is still incomplete, but it
should be capable of playing basic early-era NES games like _Pac-Man_ or
_Donkey Kong_.

<p align="center">
  <img width="384" height="360" alt="a recorded pacman demo" src="https://github.com/user-attachments/assets/f26693a1-2b32-46e5-967c-ed0caa83fd1e" />
</p>

## Purpose & Objectives

GreenNES was created as a fun exercise to learn more about emulator creation!
The goals outlined below reflect that original purpose.

GreenNES's CPU implementation is mostly complete, with the exception of a few
missing unofficial opcodes. The remaining work is centered around the other
components: the PPU, the APU, and I/O.

### Short-Term Goals

- Lock frame generation to the native framerate (~30 fps).
- Fix sprite rendering issues.
- Implement PPU scrolling behavior.
- Check emulation accuracy against a wider range of test ROMs.

### Long-Term Goals

- Overhaul PPU implementation to be pixel-accurate.
- Implement APU.
- Implement the most common cartridge mappers.
- Improve UI to include useful tools like save states.

## Resources

Below is a list of resources that I have found to be useful reference material
as I build this emulator:

- The NesDev Wiki's [NES Reference Guide](https://www.nesdev.org/wiki/NES_reference_guide) and [forums](https://forums.nesdev.org/index.php).
- [_MOS MCS6500 Microcomputer Family Hardware Manual_](https://archive.org/details/mcs-6500-family-hardware-manual-1976-01/page/n1/mode/2up)
- [_MOS MCS6500 Microcomputer Programming Manual_](https://archive.org/details/mos_microcomputers_programming_manual)
- [_6502 User's Manual_](https://archive.org/details/6502UsersManual) by Joseph J. Carr
- [_NES eBook_](https://bugzmanov.github.io/nes_ebook) by Rafael Bagmanov et al.
- [_64doc_](https://atarihq.com/danb/files/64doc.txt) by John West and Marko Mäkelä (also available [here](https://www.zimmers.net/anonftp/pub/cbm/documents/chipdata/64doc) and [here](https://nerdy-nights.nes.science/downloads/missing/64doc.txt)).
- [_6502 “Illegal” Opcodes Demystified_](https://www.masswerk.at/nowgobang/2021/6502-illegal-opcodes) by Norbert Landsteiner
- [_How MOS 6502 Illegal Opcodes really work_](https://www.pagetable.com/?p=39) by Michael Steil

## Building & Running from Source

If you want to run GreenNES on your machine, you'll first need to build it from
source. Start by installing the following tools:

- [Bash](https://www.gnu.org/software/bash/)
- [Make](https://www.gnu.org/software/make)
- [Git](https://git-scm.com)
- [Rust](https://rust-lang.org)

Then clone the repository with Git:

```sh
git clone https://github.com/nsdigirolamo/green-nes.git
```

Running the following command will build and save the emulator's binary in
`./target/release/green-nes`:

```sh
make build
```

### Verify a Successful Build


The `download-tests.sh` script will download the [nestest](https://www.qmtpro.com/~nes/misc/nestest.txt) 
test ROM to your machine, alongside its README and a known good log of the test.

Running the following command will execute the above script and then run a 
single unit test against the downloaded ROM:

```sh
make test
```

This unit test passing indicates GreenNES is properly emulating the NES CPU. 

### Running NES Games

Game support on GreenNES is currently limited to only games from [NROM](https://www.nesdev.org/wiki/NROM) 
cartridge boards in the [iNES](https://www.nesdev.org/wiki/INES#iNES_file_format) 
1.0 file format. 

To run a game in GreenNES, provide its path to the `run` argument:

```sh
./target/release/green-nes run game.nes
```

And running the following will show a list of the emulator's other options:

```sh
./target/release/green-nes help
```