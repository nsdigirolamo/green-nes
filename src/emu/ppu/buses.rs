use crate::emu::{
    cartridge::{Cartridge, NametableMirroring},
    ppu::{mappings::*, nametable::Nametable},
};

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
    // If the address is entry 0 in a palette, remap to the backdrop color.
    let addr = match addr {
        0x3F00 | 0x3F04 | 0x3F08 | 0x3F0C | 0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => 0x3F00,
        _ => addr,
    };

    buses.palette_ram[(addr % PALETTE_RAM_SIZE) as usize]
}

fn do_palette_write(buses: &mut Buses, addr: u16, data: u8) {
    // If the address is entry 0 in a palette, remap to the backdrop color.
    let addr = match addr {
        0x3F00 | 0x3F04 | 0x3F08 | 0x3F0C | 0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => 0x3F00,
        _ => addr,
    };

    buses.palette_ram[(addr % PALETTE_RAM_SIZE) as usize] = data;
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
