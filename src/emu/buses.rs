use crate::concat_u8;
use crate::emu::cartridge::Cartridge;

/*
Total Memory Size: 65536 (16-bit address space)
┌────────────────────────────────────────┐
│ Internal RAM (2048 bytes)              │
│ 0x0000 - 0x07FF                        │
├────────────────────────────────────────┤
│ Internal RAM Mirrors (6144 bytes)      │
│ 0x0800 - 0x1FFF                        │
├────────────────────────────────────────┤
│ NES PPU Registers (8 bytes)            │
│ 0x2000 - 0x2007                        │
├────────────────────────────────────────┤
│ NES PPU Registers Mirrors (8184 bytes) │
│ 0x2008 - 0x3FFF                        │
├────────────────────────────────────────┤
│ NES APU & IO Registers (24 bytes)      │
│ 0x4000 - 0x4017                        │
├────────────────────────────────────────┤
│ CPU Test Mode Registers (8 bytes)      │
│ 0x4018 - 0x401F                        │
├────────────────────────────────────────┤
│ Unmapped Cartridge Space (49120 bytes) │
│ 0x4020 - 0xFFFF                        │
└────────────────────────────────────────┘
*/

pub const RAM_MIN_ADDR: u16 = 0x0000;
pub const RAM_MAX_ADDR: u16 = 0x07FF;
pub const RAM_MIRROR_MAX_ADDR: u16 = 0x1FFF;
pub const RAM_SIZE: usize = (RAM_MAX_ADDR - RAM_MIN_ADDR + 1) as usize;

const CARTRIDGE_ROM_MAPPER_MIN_ADDRESS: u16 = 0x8000;
const CARTRIDGE_ROM_MAPPER_MAX_ADDRESS: u16 = 0xFFFF;

pub struct Buses {
    ram: [u8; RAM_SIZE],
    pub addr: (u8, u8),
    pub data: u8,
    cart: Cartridge,
}

impl Buses {
    pub fn new(cart: Cartridge) -> Self {
        Buses {
            ram: [0; RAM_SIZE],
            addr: (0, 0),
            data: 0,
            cart,
        }
    }

    /// Returns the byte stored at the given memory address.
    pub fn peek(&self, addr: (u8, u8)) -> u8 {
        let addr = concat_u8!(addr.0, addr.1);

        match addr {
            RAM_MIN_ADDR..=RAM_MIRROR_MAX_ADDR => {
                let mapped_addr = addr & 0b_0000_0111_1111_1111;
                self.ram[mapped_addr as usize]
            }
            CARTRIDGE_ROM_MAPPER_MIN_ADDRESS..=CARTRIDGE_ROM_MAPPER_MAX_ADDRESS => {
                let mapped_addr = addr - CARTRIDGE_ROM_MAPPER_MIN_ADDRESS;

                let mapped_addr = if self.cart.prg_rom.len() == 0x4000 && mapped_addr >= 0x4000 {
                    mapped_addr % 0x4000
                } else {
                    mapped_addr
                };

                self.cart.prg_rom[mapped_addr as usize]
            }
            _ => {
                todo!("read 0x{addr:04X} is unmapped")
            }
        }
    }

    /// Reads from the memory location specified on the address bus.
    pub fn read(&mut self) -> u8 {
        let data = self.peek(self.addr);
        self.data = data;

        data
    }

    /// Writes the given byte to the memory location specified on the address bus.
    pub fn write(&mut self, data: u8) {
        self.data = data;

        let addr = concat_u8!(self.addr.0, self.addr.1);
        match addr {
            RAM_MIN_ADDR..=RAM_MIRROR_MAX_ADDR => {
                let mapped_addr = addr & 0b_0000_0111_1111_1111;
                self.ram[mapped_addr as usize] = data;
            }
            _ => {
                todo!("write {addr} is unmapped")
            }
        }
    }
}
