use std::fmt;

pub mod debug;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
};

use crate::{
    DebugLevel, concat_u8,
    emu::{
        buses::Buses, cartridge::Cartridge, cpu::CPU, nes::debug::get_debug_text, ppu::frame::Frame,
    },
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

    pub fn run(&mut self, debug_level: DebugLevel) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let width = Frame::WIDTH_PIXELS as u32;
        let height = Frame::HEIGHT_PIXELS as u32;

        let window = video_subsystem
            .window("Green NES", width * 2, height * 2)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_logical_size(width, height).unwrap();
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let creator = canvas.texture_creator();
        let mut texture = creator
            .create_texture_target(PixelFormatEnum::RGB24, width, height)
            .unwrap();

        let mut event_pump = sdl_context.event_pump().unwrap();

        'running: while !self.cpu.is_halted() {
            if self.cpu.get_cycle_queue().is_empty() && debug_level == DebugLevel::High {
                println!("{self:?}")
            }

            self.buses.tick();

            let frame = self.buses.take_frame();
            if let Some(frame) = frame {
                texture
                    .update(
                        None,
                        &frame.get_pixel_data(),
                        Frame::WIDTH_PIXELS * Frame::BYTES_PER_PIXEL,
                    )
                    .unwrap();

                canvas.copy(&texture, None, None).unwrap();
                canvas.present();
            }

            self.cpu.tick(&mut self.buses);

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
        }
    }
}

impl fmt::Display for NES {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let registers = self.cpu.get_registers();

        let (pch, pcl) = registers.pc;

        let pc0 = concat_u8!(registers.pc.0, registers.pc.1);
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

        let ir = registers.ir;
        let accumulator = registers.a;
        let x_index = registers.x_index;
        let y_index = registers.y_index;
        let psr = registers.psr;
        let sp = registers.sp;
        let cycle_count = self.cpu.half_cycle_count / 2;

        let sp0 = concat_u8!(0x10, registers.sp);
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
        let registers = self.cpu.get_registers();

        let pc = concat_u8!(registers.pc.0, registers.pc.1);
        let pc0 = self.buses.peek(pc);
        let pc1 = self.buses.peek(pc.wrapping_add(1));
        let pc2 = self.buses.peek(pc.wrapping_add(2));
        let debug_text = get_debug_text(self);
        let accumulator = registers.a;
        let x_index = registers.x_index;
        let y_index = registers.y_index;
        let psr = registers.psr;
        let sp = registers.sp;
        let cycle_count = self.cpu.half_cycle_count / 2;

        write!(
            f,
            "{pc:04X}  {pc0:02X} {pc1:02X} {pc2:02X}  {debug_text:45} \
            A:{accumulator:02X} X:{x_index:02X} \
            Y:{y_index:02X} P:{psr:02X} SP:{sp:02X} CYC:{cycle_count:}",
        )
    }
}
