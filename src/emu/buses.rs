use crate::concat_u8;
use crate::emu::cartridge::Cartridge;
use crate::emu::ppu::PPU;

// Internal RAM

const RAM_SIZE: usize = 2048;
const RAM_MIRROR_COUNT: u16 = 4;

const RAM_START_ADDR: u16 = 0x0000;
const RAM_END_ADDR: u16 = RAM_START_ADDR + (RAM_SIZE as u16 * RAM_MIRROR_COUNT);

// PPU Registers

const PPU_REGISTERS_SIZE: usize = 8;
const PPU_REGISTERS_MIRROR_COUNT: u16 = 1024;

const PPU_REGISTERS_START_ADDR: u16 = RAM_END_ADDR;
const PPU_REGISTERS_END_ADDR: u16 =
    PPU_REGISTERS_START_ADDR + (PPU_REGISTERS_SIZE as u16 * PPU_REGISTERS_MIRROR_COUNT);

// APU and I/O

const IO_SIZE: usize = 24;
const TEST_MODE_SIZE: usize = 8;

const IO_START_ADDR: u16 = PPU_REGISTERS_END_ADDR;
const IO_END_ADDR: u16 = IO_START_ADDR + IO_SIZE as u16;

const TEST_MODE_START_ADDR: u16 = IO_END_ADDR;
const TEST_MODE_END_ADDR: u16 = TEST_MODE_START_ADDR + TEST_MODE_SIZE as u16;

// Cartridge (Unmapped)

const CARTRIDGE_ROM_MAPPER_START_ADDR: u16 = TEST_MODE_END_ADDR;

#[derive(Clone)]
pub struct Buses {
    // Data
    ram: [u8; RAM_SIZE],
    pub addr: (u8, u8),
    pub data: u8,
    // Connected Devices
    ppu: PPU,
    cart: Cartridge,
    // Misc
    irq: bool,
    nmi: bool,
}

impl Buses {
    pub fn new(cart: Cartridge) -> Self {
        Buses {
            ram: [0; RAM_SIZE],
            addr: (0, 0),
            data: 0,
            cart: cart.clone(),
            nmi: false,
            irq: false,
            ppu: PPU::new(cart.clone()),
        }
    }

    pub fn tick(&mut self) {
        self.nmi = self.ppu.get_nmi()
    }

    /// Returns a byte from the given memory address.
    fn fetch_data(&mut self, addr: u16) -> u8 {
        match addr {
            RAM_START_ADDR..RAM_END_ADDR => {
                let mapped_addr = addr & 0b_0000_0111_1111_1111;
                self.ram[mapped_addr as usize]
            }
            PPU_REGISTERS_START_ADDR..PPU_REGISTERS_END_ADDR => {
                let mapped_addr = addr % 8;
                match mapped_addr {
                    0 => self.ppu.read_ppu_ctrl(),
                    1 => self.ppu.read_ppu_mask(),
                    2 => self.ppu.read_ppu_status(),
                    3 => self.ppu.read_oam_addr(),
                    4 => self.ppu.read_oam_data(),
                    5 => self.ppu.read_ppu_scroll(),
                    6 => self.ppu.read_ppu_addr(),
                    7 => self.ppu.read_ppu_data(),
                    _ => unreachable!(
                        "bus read failed: address 0x{addr:04X} is mapped to ppu register and should not be greater than 7"
                    ),
                }
            }
            IO_START_ADDR..IO_END_ADDR => {
                todo!("bus fetch failed: apu address 0x{addr:04X} is unmapped")
            }
            TEST_MODE_START_ADDR..TEST_MODE_END_ADDR => {
                todo!("bus fetch failed: test mode address 0x{addr:04X} is unmapped")
            }
            CARTRIDGE_ROM_MAPPER_START_ADDR.. => self.cart.mapper.borrow().prg_read(addr),
        }
    }

    /// Fetches a byte from the memory address specified on the address bus
    /// without modifying the data bus. Returns the fetched byte.
    pub fn peek(&self, addr: u16) -> u8 {
        match addr {
            RAM_START_ADDR..RAM_END_ADDR => {
                let mapped_addr = addr & 0b_0000_0111_1111_1111;
                self.ram[mapped_addr as usize]
            }
            PPU_REGISTERS_START_ADDR..PPU_REGISTERS_END_ADDR => {
                panic!("bus peek failed: address 0x{addr:04X} is a ppu register")
            }
            IO_START_ADDR..IO_END_ADDR => {
                todo!("bus peek failed: apu address 0x{addr:04X} is unmapped")
            }
            TEST_MODE_START_ADDR..TEST_MODE_END_ADDR => {
                todo!("bus peek failed: test mode address 0x{addr:04X} is unmapped")
            }
            CARTRIDGE_ROM_MAPPER_START_ADDR.. => self.cart.mapper.borrow().prg_read(addr),
        }
    }

    /// Fetches a byte from the memory address specified on the address bus,
    /// then places that byte on the data bus. Returns the fetched byte.
    pub fn read(&mut self) -> u8 {
        let addr = concat_u8!(self.addr.0, self.addr.1);
        let data = self.fetch_data(addr);
        self.data = data;

        data
    }

    /// Places the given byte onto the data bus, and then writes that byte to
    /// the memory address specified on the addres bus.
    pub fn write(&mut self, data: u8) {
        self.data = data;

        let addr = concat_u8!(self.addr.0, self.addr.1);
        match addr {
            RAM_START_ADDR..RAM_END_ADDR => {
                let mapped_addr = addr & 0b_0000_0111_1111_1111;
                self.ram[mapped_addr as usize] = data;
            }
            PPU_REGISTERS_START_ADDR..PPU_REGISTERS_END_ADDR => {
                let mapped_addr = addr % 8;
                match mapped_addr {
                    0 => self.ppu.write_ppu_ctrl(data),
                    1 => self.ppu.write_ppu_mask(data),
                    2 => self.ppu.write_ppu_status(data),
                    3 => self.ppu.write_oam_addr(data),
                    4 => self.ppu.write_oam_data(data),
                    5 => self.ppu.write_ppu_scroll(data),
                    6 => self.ppu.write_ppu_addr(data),
                    7 => self.ppu.write_ppu_data(data),
                    _ => unreachable!(
                        "bus write failed: address 0x{addr:04X} is mapped to ppu register and should not be greater than 7"
                    ),
                }
            }
            IO_START_ADDR..IO_END_ADDR => {
                todo!("bus write failed: apu address 0x{addr:04X} is unmapped")
            }
            TEST_MODE_START_ADDR..TEST_MODE_END_ADDR => {
                todo!("bus write failed: test mode address 0x{addr:04X} is unmapped")
            }
            CARTRIDGE_ROM_MAPPER_START_ADDR.. => {
                self.cart.mapper.borrow_mut().prg_write(addr, data)
            }
        }
    }

    pub fn get_ppu(&self) -> PPU {
        self.ppu.clone()
    }

    pub fn get_cart(&self) -> Cartridge {
        self.cart.clone()
    }

    pub fn get_irq(&self) -> bool {
        self.irq
    }

    pub fn get_nmi(&self) -> bool {
        self.nmi
    }
}
