pub mod nrom;

pub trait Mapper {
    fn prg_read(&self, addr: u16) -> u8;
    fn prg_write(&mut self, addr: u16, data: u8);
    fn chr_read(&self, addr: u16) -> u8;
    fn chr_write(&mut self, addr: u16, data: u8);
}
