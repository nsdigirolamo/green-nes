# Green Nintendo Entertainment System (GreenNES)

GreenNES is a hobbyist emulator for the Nintendo Entertainment System (NES)
written in Rust. The emulator in its current state is still incomplete, but it
should be capable of playing basic early-era NES games like _Pac-Man_ or
_Donkey Kong_.

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
