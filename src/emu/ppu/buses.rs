use crate::emu::{
    cartridge::{Cartridge, NametableMirroring},
    ppu::{
        nametable::{
            ATTRIBUTE_TABLE_0_END_ADDR, ATTRIBUTE_TABLE_1_END_ADDR, ATTRIBUTE_TABLE_2_END_ADDR,
            ATTRIBUTE_TABLE_3_END_ADDR, NAMETABLE_0_START_ADDR, NAMETABLE_1_START_ADDR,
            NAMETABLE_2_START_ADDR, NAMETABLE_3_START_ADDR, NAMETABLES_END_ADDR,
            NAMETABLES_START_ADDR, Nametable,
        },
        patterns::{PATTERN_TABLES_END_ADDR, PATTERN_TABLES_START_ADDR},
    },
};

// Unused

pub const UNUSED_SIZE: u16 = 3840;

pub const UNUSED_START_ADDR: u16 = NAMETABLES_END_ADDR;
pub const UNUSED_END_ADDR: u16 = UNUSED_START_ADDR + UNUSED_SIZE;

// Palette RAM

pub const PALETTE_RAM_SIZE: u16 = 32;
pub const PALETTE_RAM_MIRRORS_SIZE: u16 = 224;

pub const PALETTE_RAM_START_ADDR: u16 = UNUSED_END_ADDR;

pub const PALETTE_RAM_BACKGROUND_START_ADDR: u16 = PALETTE_RAM_START_ADDR;
pub const PALETTE_RAM_BACKGROUND_END_ADDR: u16 =
    PALETTE_RAM_BACKGROUND_START_ADDR + (PALETTE_RAM_SIZE / 2);

pub const PALETTE_RAM_SPRITE_START_ADDR: u16 = PALETTE_RAM_BACKGROUND_END_ADDR;
pub const PALETTE_RAM_SPITE_END_ADDR: u16 = PALETTE_RAM_SPRITE_START_ADDR + (PALETTE_RAM_SIZE / 2);

pub const PALETTE_RAM_MIRRORS_START_ADDR: u16 = PALETTE_RAM_SPITE_END_ADDR;
pub const PALETTE_RAM_MIRRORS_END_ADDR: u16 =
    PALETTE_RAM_MIRRORS_START_ADDR + PALETTE_RAM_MIRRORS_SIZE;

pub const PALETTE_RAM_END_ADDR: u16 = PALETTE_RAM_MIRRORS_END_ADDR;

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
            PATTERN_TABLES_START_ADDR..PATTERN_TABLES_END_ADDR => {
                self.cart.mapper.borrow().chr_read(mapped_addr)
            }
            NAMETABLES_START_ADDR..NAMETABLES_END_ADDR => do_nametable_read(self, mapped_addr),
            UNUSED_START_ADDR..UNUSED_END_ADDR => {
                todo!("ppu read failed: &{addr:04X} is in unused memory")
            }
            PALETTE_RAM_START_ADDR..PALETTE_RAM_END_ADDR => do_palette_read(self, addr),
            _ => {
                unreachable!("ppu read failed: &{addr:04X} is outside of the 14-bit address space")
            }
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        let mapped_addr = addr & 0b_0011_1111_1111_1111; // 14-bit address space

        match mapped_addr {
            PATTERN_TABLES_START_ADDR..PATTERN_TABLES_END_ADDR => {
                self.cart.mapper.borrow_mut().chr_write(mapped_addr, data);
            }
            NAMETABLES_START_ADDR..NAMETABLES_END_ADDR => {
                do_nametable_write(self, mapped_addr, data);
            }
            UNUSED_START_ADDR..UNUSED_END_ADDR => {
                todo!("ppu write failed: &{addr:04X} is in unused memory")
            }
            PALETTE_RAM_START_ADDR..PALETTE_RAM_END_ADDR => do_palette_write(self, addr, data),
            _ => {
                unreachable!("ppu write failed: &{addr:04X} is outside of the 14-bit address space")
            }
        }
    }
}

fn do_palette_read(buses: &Buses, addr: u16) -> u8 {
    let mapped_addr = match addr % PALETTE_RAM_SIZE {
        0x0010 => 0x0000,
        0x0014 => 0x0004,
        0x0018 => 0x0008,
        0x001C => 0x000C,
        a => a,
    };

    buses.palette_ram[mapped_addr as usize]
}

fn do_palette_write(buses: &mut Buses, addr: u16, data: u8) {
    let mapped_addr = match addr % PALETTE_RAM_SIZE {
        0x0010 => 0x0000,
        0x0014 => 0x0004,
        0x0018 => 0x0008,
        0x001C => 0x000C,
        a => a,
    };

    buses.palette_ram[mapped_addr as usize] = data
}

fn do_nametable_read(buses: &Buses, addr: u16) -> u8 {
    let arrangement = buses.cart.mapper.borrow().get_nametable_arrangement();

    match arrangement {
        NametableMirroring::Horizontal => do_horizontal_nametable_read(buses, addr),
        NametableMirroring::Vertical => do_vertical_nametable_read(buses, addr),
    }
}

fn do_nametable_write(buses: &mut Buses, addr: u16, data: u8) {
    let arrangement = buses.cart.mapper.borrow().get_nametable_arrangement();
    match arrangement {
        NametableMirroring::Horizontal => do_horizontal_nametable_write(buses, addr, data),
        NametableMirroring::Vertical => do_vertical_nametable_write(buses, addr, data),
    }
}

fn do_horizontal_nametable_read(buses: &Buses, addr: u16) -> u8 {
    match addr {
        NAMETABLE_0_START_ADDR..ATTRIBUTE_TABLE_0_END_ADDR => buses.nametable_a.read(addr),
        NAMETABLE_1_START_ADDR..ATTRIBUTE_TABLE_1_END_ADDR => buses.nametable_a.read(addr),
        NAMETABLE_2_START_ADDR..ATTRIBUTE_TABLE_2_END_ADDR => buses.nametable_b.read(addr),
        NAMETABLE_3_START_ADDR..ATTRIBUTE_TABLE_3_END_ADDR => buses.nametable_b.read(addr),
        _ => {
            panic!("horizontal nametable read failed: &{addr:04X} is not a valid nametable address")
        }
    }
}

fn do_horizontal_nametable_write(buses: &mut Buses, addr: u16, data: u8) {
    match addr {
        NAMETABLE_0_START_ADDR..ATTRIBUTE_TABLE_0_END_ADDR => buses.nametable_a.write(addr, data),
        NAMETABLE_1_START_ADDR..ATTRIBUTE_TABLE_1_END_ADDR => buses.nametable_a.write(addr, data),
        NAMETABLE_2_START_ADDR..ATTRIBUTE_TABLE_2_END_ADDR => buses.nametable_b.write(addr, data),
        NAMETABLE_3_START_ADDR..ATTRIBUTE_TABLE_3_END_ADDR => buses.nametable_b.write(addr, data),
        _ => {
            panic!("horizontal nametable read failed: &{addr:04X} is not a valid nametable address")
        }
    }
}

fn do_vertical_nametable_read(buses: &Buses, addr: u16) -> u8 {
    match addr {
        NAMETABLE_0_START_ADDR..ATTRIBUTE_TABLE_0_END_ADDR => buses.nametable_a.read(addr),
        NAMETABLE_1_START_ADDR..ATTRIBUTE_TABLE_1_END_ADDR => buses.nametable_b.read(addr),
        NAMETABLE_2_START_ADDR..ATTRIBUTE_TABLE_2_END_ADDR => buses.nametable_a.read(addr),
        NAMETABLE_3_START_ADDR..ATTRIBUTE_TABLE_3_END_ADDR => buses.nametable_b.read(addr),
        _ => panic!("vertical nametable read failed: &{addr:04X} is not a valid nametable address"),
    }
}

fn do_vertical_nametable_write(buses: &mut Buses, addr: u16, data: u8) {
    match addr {
        NAMETABLE_0_START_ADDR..ATTRIBUTE_TABLE_0_END_ADDR => buses.nametable_a.write(addr, data),
        NAMETABLE_1_START_ADDR..ATTRIBUTE_TABLE_1_END_ADDR => buses.nametable_b.write(addr, data),
        NAMETABLE_2_START_ADDR..ATTRIBUTE_TABLE_2_END_ADDR => buses.nametable_a.write(addr, data),
        NAMETABLE_3_START_ADDR..ATTRIBUTE_TABLE_3_END_ADDR => buses.nametable_b.write(addr, data),
        _ => panic!(
            "vertical nametable read failed: address &{addr:04X} is not a valid nametable address"
        ),
    }
}
