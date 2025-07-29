# Green Nintendo Entertainment System (GreenNES)

An emulator for the Nintendo Entertainment System (NES) written in Rust.

## More Information

The emulator in its current state is still incomplete. My initial focus is to
create an accurate emulator of the NES' CPU. After that is complete, I will move
on to the Pixel Processing Unit (PPU) and Audio Processing Unit (APU).

_**NES CPU Emulation Progress**_:

**196 of 256** total opcodes are implemented (76.56%).

All official opcodes are complete. I will begin testing/debugging for correctness after every opcode is implemented.

## Resources

Below is a list of resources that I have found to be useful reference material as I build this emulator:

- [_MOS MCS6500 Microcomputer Family Hardware Manual_](https://archive.org/details/mcs-6500-family-hardware-manual-1976-01/page/n1/mode/2up)
- [_MOS MCS6500 Microcomputer Programming Manual_](https://archive.org/details/mos_microcomputers_programming_manual)
- [_6502 User's Manual_](https://archive.org/details/6502UsersManual) by Joseph J. Carr
- The NesDev Wiki's [NES Reference Guide](https://www.nesdev.org/wiki/NES_reference_guide) and [forums](https://forums.nesdev.org/index.php).
- [_64doc_](https://atarihq.com/danb/files/64doc.txt) by John West and Marko Mäkelä (also [here](https://www.zimmers.net/anonftp/pub/cbm/documents/chipdata/64doc) and [here](https://nerdy-nights.nes.science/downloads/missing/64doc.txt)).
- [_6502 “Illegal” Opcodes Demystified_](https://www.masswerk.at/nowgobang/2021/6502-illegal-opcodes) by Norbert Landsteiner
- [_How MOS 6502 Illegal Opcodes really work_](https://www.pagetable.com/?p=39) by Michael Steil
