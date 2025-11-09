use crate::emu::{
    cartridge::{Cartridge, NametableMirroring},
    ppu::nametable::Nametable,
};

// Character Memory

const CHR_ROM_SIZE: u16 = 8192;

const CHR_ROM_START_ADDR: u16 = 0x0000;
const CHR_ROM_END_ADDR: u16 = CHR_ROM_START_ADDR + CHR_ROM_SIZE;

// Nametables

const NAMETABLE_SIZE: u16 = 1024;

const NAMETABLE_0_START_ADDR: u16 = CHR_ROM_END_ADDR;
const NAMETABLE_0_END_ADDR: u16 = NAMETABLE_0_START_ADDR + NAMETABLE_SIZE;

const NAMETABLE_1_START_ADDR: u16 = NAMETABLE_0_END_ADDR;
const NAMETABLE_1_END_ADDR: u16 = NAMETABLE_1_START_ADDR + NAMETABLE_SIZE;

const NAMETABLE_2_START_ADDR: u16 = NAMETABLE_1_END_ADDR;
const NAMETABLE_2_END_ADDR: u16 = NAMETABLE_2_START_ADDR + NAMETABLE_SIZE;

const NAMETABLE_3_START_ADDR: u16 = NAMETABLE_2_END_ADDR;
const NAMETABLE_3_END_ADDR: u16 = NAMETABLE_3_START_ADDR + NAMETABLE_SIZE;

const NAMETABLE_START_ADDR: u16 = NAMETABLE_0_START_ADDR;
const NAMETABLE_END_ADDR: u16 = NAMETABLE_3_END_ADDR;

// Unused

const UNUSED_SIZE: u16 = 3840;

const UNUSED_START_ADDR: u16 = NAMETABLE_END_ADDR;
const UNUSED_END_ADDR: u16 = UNUSED_START_ADDR + UNUSED_SIZE;

// Palette Memory

const PALETTE_RAM_SIZE: u16 = 32;
const MIRRORS_SIZE: u16 = 224;

const PALETTE_RAM_BACKGROUND_START_ADDR: u16 = UNUSED_END_ADDR;
const PALETTE_RAM_BACKGROUND_END_ADDR: u16 =
    PALETTE_RAM_BACKGROUND_START_ADDR + (PALETTE_RAM_SIZE / 2);

const PALETTE_RAM_SPRITE_START_ADDR: u16 = PALETTE_RAM_BACKGROUND_END_ADDR;
const PALETTE_RAM_SPITE_END_ADDR: u16 = PALETTE_RAM_SPRITE_START_ADDR + (PALETTE_RAM_SIZE / 2);

const PALETTE_RAM_START_ADDR: u16 = PALETTE_RAM_BACKGROUND_START_ADDR;
const PALETTE_RAM_END_ADDR: u16 = PALETTE_RAM_SPITE_END_ADDR;

const MIRRORS_START_ADDR: u16 = PALETTE_RAM_END_ADDR;
const MIRRORS_END_ADDR: u16 = MIRRORS_START_ADDR + MIRRORS_SIZE;

#[derive(Clone)]
pub struct Buses {
    nametable_a: Nametable,
    nametable_b: Nametable,
    palette_ram: [u8; PALETTE_RAM_SIZE as usize],
    cart: Cartridge,
}

impl Buses {
    pub fn new(cart: Cartridge) -> Self {
        Buses {
            nametable_a: Nametable::default(),
            nametable_b: Nametable::default(),
            palette_ram: [0; PALETTE_RAM_SIZE as usize],
            cart,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let mapped_addr = addr & 0b_0011_1111_1111_1111; // 14-bit address space

        match mapped_addr {
            CHR_ROM_START_ADDR..CHR_ROM_END_ADDR => self.cart.mapper.borrow().chr_read(mapped_addr),
            NAMETABLE_START_ADDR..NAMETABLE_END_ADDR => do_nametable_read(
                self,
                mapped_addr,
                self.cart.mapper.borrow().get_nametable_arrangement(),
            ),
            UNUSED_START_ADDR..UNUSED_END_ADDR => {
                todo!("ppu read failed: address 0x{addr:04X} is in unused memory")
            }
            PALETTE_RAM_START_ADDR..PALETTE_RAM_END_ADDR => do_palette_read(self, addr),
            MIRRORS_START_ADDR..MIRRORS_END_ADDR => do_palette_read(self, addr),
            _ => unreachable!(
                "ppu read failed: address 0x{addr:04X} is outside of the 14-bit address space"
            ),
        }
    }
}

fn do_palette_read(buses: &Buses, addr: u16) -> u8 {
    let mapped_addr = addr % PALETTE_RAM_SIZE;

    buses.palette_ram[mapped_addr as usize]
}

fn do_nametable_read(buses: &Buses, addr: u16, arrangement: NametableMirroring) -> u8 {
    match arrangement {
        NametableMirroring::Horizontal => do_horizontal_nametable_read(buses, addr),
        NametableMirroring::Vertical => do_vertical_nametable_read(buses, addr),
    }
}

fn do_horizontal_nametable_read(buses: &Buses, addr: u16) -> u8 {
    match addr {
        NAMETABLE_0_START_ADDR..NAMETABLE_0_END_ADDR => buses.nametable_a.read(addr),
        NAMETABLE_1_START_ADDR..NAMETABLE_1_END_ADDR => buses.nametable_a.read(addr),
        NAMETABLE_2_START_ADDR..NAMETABLE_2_END_ADDR => buses.nametable_b.read(addr),
        NAMETABLE_3_START_ADDR..NAMETABLE_3_END_ADDR => buses.nametable_b.read(addr),
        _ => panic!(
            "horizontal nametable read failed: address 0x{addr:04X} is not a valid nametable address"
        ),
    }
}

fn do_vertical_nametable_read(buses: &Buses, addr: u16) -> u8 {
    match addr {
        NAMETABLE_0_START_ADDR..NAMETABLE_0_END_ADDR => buses.nametable_a.read(addr),
        NAMETABLE_1_START_ADDR..NAMETABLE_1_END_ADDR => buses.nametable_b.read(addr),
        NAMETABLE_2_START_ADDR..NAMETABLE_2_END_ADDR => buses.nametable_a.read(addr),
        NAMETABLE_3_START_ADDR..NAMETABLE_3_END_ADDR => buses.nametable_b.read(addr),
        _ => panic!(
            "vertical nametable read failed: address 0x{addr:04X} is not a valid nametable address"
        ),
    }
}
