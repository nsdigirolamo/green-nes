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
        buses::Buses,
        cartridge::Cartridge,
        cpu::{CPU, registers::Registers},
        nes::debug::get_debug_text,
        ppu::frame::Frame,
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
            cpu: CPU::new(14, Registers::default()),
        }
    }

    pub fn run(&mut self, debug_level: DebugLevel) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let width = Frame::WIDTH as u32;
        let height = Frame::HEIGHT as u32;

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
            if self.cpu.get_cycle_queue().is_empty() && debug_level == DebugLevel::Low {
                println!("{self:?}")
            }

            self.buses.tick();

            if self.buses.ppu.is_frame_ready() {
                let mut frame = Frame::default();
                self.buses.ppu.draw_frame(&mut frame);

                texture
                    .update(
                        None,
                        &frame.get_pixel_data(),
                        Frame::WIDTH * Frame::BYTES_PER_PIXEL,
                    )
                    .unwrap();

                canvas.copy(&texture, None, None).unwrap();
                canvas.present();

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

            self.cpu.tick(&mut self.buses);
        }
    }

    pub fn run_headless(&mut self, debug_level: DebugLevel) {
        while !self.cpu.is_halted() {
            if self.cpu.get_cycle_queue().is_empty() && debug_level == DebugLevel::Low {
                println!("{self:?}")
            }

            self.buses.tick();
            self.cpu.tick(&mut self.buses);
        }
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
        let c = if psr.get_carry() { "C" } else { "." };
        let z = if psr.get_zero() { "Z" } else { "." };
        let i = if psr.get_interrupt_disable() {
            "I"
        } else {
            "."
        };
        let d = if psr.get_decimal() { "D" } else { "." };
        let b = if psr.get_break() { "B" } else { "." };
        let one = "1";
        let v = if psr.get_overflow() { "V" } else { "." };
        let n = if psr.get_negative() { "N" } else { "." };
        let flags = format!("{n}{v}{one}{b}{d}{i}{z}{c}");

        let sp = registers.sp;
        let cycle_count = self.cpu.get_cycle_count();

        let ppu = self.buses.get_ppu();
        let frame = ppu.get_frame_count();
        let scanline = ppu.get_scanline_index();

        write!(
            f,
            "{pc:04X}  {pc0:02X} {pc1:02X} {pc2:02X}  {debug_text:45} \
            A:{accumulator:02X} X:{x_index:02X} \
            Y:{y_index:02X} P:{:02X} ({flags}) SP:{sp:02X} F:{frame:04} L:{scanline:03} CYC:{cycle_count:}",
            u8::from(psr),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        DebugLevel,
        emu::{
            buses::Buses,
            cpu::{CPU, registers::Registers},
            nes::NES,
        },
        load_cart,
    };

    #[test]
    fn nestest() {
        let cart = load_cart("tests/nestest/nestest.nes");

        let mut nes = NES {
            buses: Buses::new(cart),
            // Hardcoded starting CPU state that the test expects.
            cpu: CPU::new(
                14,
                Registers {
                    a: 0x00,
                    x_index: 0x00,
                    y_index: 0x00,
                    pc: (0xC0, 0x00),
                    sp: 0xFD,
                    psr: 0b100100.into(),
                    ir: 0x00,
                },
            ),
        };

        nes.run_headless(DebugLevel::None);

        assert_eq!(nes.buses.peek(0x0002), 0x00);
        assert_eq!(nes.buses.peek(0x0003), 0x00);
    }
}
