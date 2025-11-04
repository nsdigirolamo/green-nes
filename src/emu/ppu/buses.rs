/*
Total Memory Size: 16384 (14-bit address space)
┌─────────────────────────────────────────┐
│ Pattern Table 0 (4096 bytes)            │
│ 0x0000 - 0x0FFF                         │
├─────────────────────────────────────────┤
│ Pattern Table 1 (4096 bytes)            │
│ 0x1000 - 0x1FFF                         │
├─────────────────────────────────────────┤
│ Nametable 0 (960 bytes)                 │
│ 0x2000 - 0x23BF                         │
├─────────────────────────────────────────┤
│ Attribute Table 0 (64 bytes)            │
│ 0x23C0 - 0x23FF                         │
├─────────────────────────────────────────┤
│ Nametable 1 (960 bytes)                 │
│ 0x2400 - 0x27BF                         │
├─────────────────────────────────────────┤
│ Attribute Table 1 (64 bytes)            │
│ 0x27C0 - 0x27FF                         │
├─────────────────────────────────────────┤
│ Nametable 2 (960 bytes)                 │
│ 0x2800 - 0x2BBF                         │
├─────────────────────────────────────────┤
│ Attribute Table 2 (64 bytes)            │
│ 0x2BC0 - 0x2BFF                         │
├─────────────────────────────────────────┤
│ Nametable 3 (960 bytes)                 │
│ 0x2C00 - 0x2FBF                         │
├─────────────────────────────────────────┤
│ Attribute Table 3 (64 bytes)            │
│ 0x2FC0 - 0x2FFF                         │
├─────────────────────────────────────────┤
│ Unused (3840 bytes)                     │
│ 0x3000 - 0x3EFF                         │
├─────────────────────────────────────────┤
│ Palette RAM Indices (32 bytes)          │
│ 0x3F00 - 0x3F1F                         │
├─────────────────────────────────────────┤
│ Palette RAM Indices Mirrors (224 bytes) │
│ 0x3F20 - 0x3FFF                         │
└─────────────────────────────────────────┘
*/

use crate::emu::cartridge::Cartridge;

const CHR_ROM_MIN_ADDR: u16 = 0x0000;
const CHR_ROM_MAX_ADDR: u16 = 0x1FFF;

const VRAM_MIN_ADDR: u16 = 0x2000;
const VRAM_MAX_ADDR: u16 = 0x2FFF;
const VRAM_SIZE: usize = (VRAM_MAX_ADDR - VRAM_MIN_ADDR) as usize + 1;

const PALETTE_RAM_MIN_ADDR: u16 = 0x3F00;
const PALETTE_RAM_MAX_MIRROR_ADDR: u16 = 0x3FFF;

#[derive(Clone)]
pub struct Buses {
    vram: [u8; VRAM_SIZE],
    cart: Cartridge,
}

impl Buses {
    pub fn new(cart: Cartridge) -> Self {
        Buses {
            vram: [0u8; VRAM_SIZE],
            cart,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            CHR_ROM_MIN_ADDR..=CHR_ROM_MAX_ADDR => self.cart.mapper.borrow().chr_read(addr),
            VRAM_MIN_ADDR..=VRAM_MAX_ADDR => self.vram[addr as usize],
            PALETTE_RAM_MIN_ADDR..=PALETTE_RAM_MAX_MIRROR_ADDR => {
                todo!("ppu read failed: address 0x{addr:04X} should be mapped to palette ram")
            }
            _ => panic!("ppu read failed: address 0x{addr:04X} is not mapped."),
        }
    }
}
