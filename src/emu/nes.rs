use std::fmt;

use crate::{
    concat_u8,
    emu::{buses::Buses, cartridge::Cartridge, cpu::CPU},
};

pub struct NES {
    pub buses: Buses,
    pub cpu: CPU,
}

impl NES {
    pub fn new(cart: Cartridge) -> Self {
        Self {
            buses: Buses::new(cart),
            cpu: CPU::default(),
        }
    }
}

impl fmt::Display for NES {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (pch, pcl) = self.cpu.registers.pc;

        let pc0 = concat_u8!(self.cpu.registers.pc.0, self.cpu.registers.pc.1);
        let pc1 = pc0.wrapping_add(1);
        let pc2 = pc0.wrapping_add(2);
        let pc_mem0 = self.buses.peek(pc0);
        let pc_mem1 = self.buses.peek(pc1);
        let pc_mem2 = self.buses.peek(pc2);

        let (addr_bus_high, addr_bus_low) = self.buses.addr;
        let ab0 = concat_u8!(addr_bus_high, addr_bus_low);
        let ab1 = ab0.wrapping_add(1);
        let ab2 = ab0.wrapping_add(2);
        let ab_mem0 = self.buses.peek(ab0);
        let ab_mem1 = self.buses.peek(ab1);
        let ab_mem2 = self.buses.peek(ab2);

        let data_bus = self.buses.data;

        let ir = self.cpu.registers.ir;
        let accumulator = self.cpu.registers.a;
        let x_index = self.cpu.registers.x_index;
        let y_index = self.cpu.registers.y_index;
        let psr = self.cpu.registers.psr;
        let sp = self.cpu.registers.sp;
        let cycle_count = self.cpu.half_cycle_count / 2;

        let sp0 = concat_u8!(0x10, self.cpu.registers.sp);
        let sp1 = sp0.wrapping_add(1);
        let sp2 = sp0.wrapping_add(2);
        let sp_mem0 = self.buses.peek(sp0);
        let sp_mem1 = self.buses.peek(sp1);
        let sp_mem2 = self.buses.peek(sp2);

        write!(
            f,
            "{pch:02X}{pcl:02X} [{pc_mem0:02X} {pc_mem1:02X} {pc_mem2:02X}] \
            ADDR_BUS: {addr_bus_high:02X}{addr_bus_low:02X} \
            [{ab_mem0:02X} {ab_mem1:02X} {ab_mem2:02X}] \
            DATA_BUS: {data_bus:02X} \
            IR:{ir:02X} A:{accumulator:02X} X:{x_index:02X} Y:{y_index:02X} \
            P:{psr:02X} SP:{sp:02X} [{sp_mem0:02X} {sp_mem1:02X} {sp_mem2:02X}] \
            CYC:{cycle_count:}"
        )
    }
}

impl fmt::Debug for NES {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (pch, pcl) = self.cpu.registers.pc;

        let pc0 = concat_u8!(self.cpu.registers.pc.0, self.cpu.registers.pc.1);
        let pc1 = pc0.wrapping_add(1);
        let pc2 = pc0.wrapping_add(2);
        let pc_mem0 = self.buses.peek(pc0);
        let pc_mem1 = self.buses.peek(pc1);
        let pc_mem2 = self.buses.peek(pc2);

        let accumulator = self.cpu.registers.a;
        let x_index = self.cpu.registers.x_index;
        let y_index = self.cpu.registers.y_index;
        let psr = self.cpu.registers.psr;
        let sp = self.cpu.registers.sp;
        let cycle_count = self.cpu.half_cycle_count / 2;

        write!(
            f,
            "{pch:02X}{pcl:02X}  {pc_mem0:02X} {pc_mem1:02X} {pc_mem2:02X}  \
            \t\t\t\t\tA:{accumulator:02X} X:{x_index:02X} \
            Y:{y_index:02X} P:{psr:02X} SP:{sp:02X} CYC:{cycle_count:}"
        )
    }
}
