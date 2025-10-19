use crate::concat_u8;
use crate::emu::cartridge::Cartridge;
use crate::emu::ppu::PPU;

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

const RAM_MIN_ADDR: u16 = 0x0000;
const RAM_MAX_ADDR: u16 = 0x07FF;
const RAM_MIRROR_MAX_ADDR: u16 = 0x1FFF;
const RAM_SIZE: usize = (RAM_MAX_ADDR - RAM_MIN_ADDR + 1) as usize;

const PPU_REGISTERS_MIN_ADDR: u16 = 0x2000;
const PPU_REGISTERS_MIRROR_MAX_ADDR: u16 = 0x3FFF;

const CARTRIDGE_ROM_MAPPER_MIN_ADDR: u16 = 0x8000;
const CARTRIDGE_ROM_MAPPER_MAX_ADDR: u16 = 0xFFFF;

pub struct Buses {
    ram: [u8; RAM_SIZE],
    pub addr: (u8, u8),
    pub data: u8,
    cart: Cartridge,
    ppu: PPU,
}

impl Buses {
    pub fn new(cart: Cartridge) -> Self {
        Buses {
            ram: [0; RAM_SIZE],
            addr: (0, 0),
            data: 0,
            cart,
            ppu: PPU::default(),
        }
    }

    /// Returns a byte from the given memory address.
    fn get_data(&mut self, addr: u16) -> u8 {
        match addr {
            RAM_MIN_ADDR..=RAM_MIRROR_MAX_ADDR => {
                let mapped_addr = addr & 0b_0000_0111_1111_1111;
                self.ram[mapped_addr as usize]
            }
            CARTRIDGE_ROM_MAPPER_MIN_ADDR..=CARTRIDGE_ROM_MAPPER_MAX_ADDR => {
                self.cart.mapper.prg_read(addr)
            }
            PPU_REGISTERS_MIN_ADDR..=PPU_REGISTERS_MIRROR_MAX_ADDR => {
                let mapped_addr = addr % 8;

                match mapped_addr {
                    0 => todo!("ppu_ctrl read not implemented"),
                    1 => todo!("ppu_mask read not implemented"),
                    2 => todo!("ppu_status read not implemented"),
                    3 => todo!("oam_addr read not implemented"),
                    4 => todo!("oam_data read not implemented"),
                    5 => todo!("ppu_scroll read not implemented"),
                    6 => todo!("ppu_addr read not implemented"),
                    7 => self.ppu.ppu_data_read(addr),
                    _ => unreachable!("mapped_addr is modulo 8 and cannot be greater than 7"),
                }
            }

            _ => {
                todo!("read 0x{addr:04X} is unmapped")
            }
        }
    }

    /// Returns a byte from the memory address specified on the address bus.
    pub fn read(&mut self) -> u8 {
        let addr = concat_u8!(self.addr.0, self.addr.1);
        let data = self.get_data(addr);
        self.data = data;

        data
    }

    /// Returns a byte from the given memory address without modification.
    pub fn peek(&self, addr: u16) -> u8 {
        match addr {
            RAM_MIN_ADDR..=RAM_MIRROR_MAX_ADDR => {
                let mapped_addr = addr & 0b_0000_0111_1111_1111;
                self.ram[mapped_addr as usize]
            }
            CARTRIDGE_ROM_MAPPER_MIN_ADDR..=CARTRIDGE_ROM_MAPPER_MAX_ADDR => {
                self.cart.mapper.prg_read(addr)
            }
            PPU_REGISTERS_MIN_ADDR..=PPU_REGISTERS_MIRROR_MAX_ADDR => {
                panic!("Cannot Peek: Address 0x{addr:04X} is a PPU register.")
            }
            _ => {
                todo!("Cannot Peek: Address 0x{addr:04X} is unmapped.")
            }
        }
    }

    /// Writes the given byte to the memory address specified on the address bus.
    pub fn write(&mut self, data: u8) {
        self.data = data;

        let addr = concat_u8!(self.addr.0, self.addr.1);
        match addr {
            RAM_MIN_ADDR..=RAM_MIRROR_MAX_ADDR => {
                let mapped_addr = addr & 0b_0000_0111_1111_1111;
                self.ram[mapped_addr as usize] = data;
            }
            CARTRIDGE_ROM_MAPPER_MIN_ADDR..=CARTRIDGE_ROM_MAPPER_MAX_ADDR => {
                self.cart.mapper.prg_write(addr, data)
            }
            _ => {
                todo!("write {addr} is unmapped")
            }
        }
    }
}
