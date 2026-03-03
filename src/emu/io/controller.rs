#[derive(Copy, Clone)]
pub struct Buttons(u8);

impl Buttons {
    pub const A: u8 = 1 << 0;
    pub const B: u8 = 1 << 1;
    pub const SELECT: u8 = 1 << 2;
    pub const START: u8 = 1 << 3;
    pub const UP: u8 = 1 << 4;
    pub const DOWN: u8 = 1 << 5;
    pub const LEFT: u8 = 1 << 6;
    pub const RIGHT: u8 = 1 << 7;

    pub fn is_pressed(&self, button: u8) -> bool {
        (self.0 & button) != 0
    }

    pub fn set_button(&mut self, button: u8, pressed: bool) {
        if pressed {
            self.0 |= button;
        } else {
            self.0 &= !button;
        }
    }
}

impl From<u8> for Buttons {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Buttons> for u8 {
    fn from(value: Buttons) -> u8 {
        value.0
    }
}

#[derive(Copy, Clone)]
pub struct Controller {
    strobe: bool,
    button_index: u8,
    buttons: Buttons,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            strobe: false,
            button_index: 0,
            buttons: Buttons(0),
        }
    }

    pub fn write(&mut self, data: u8) {
        self.strobe = data & 0b_0000_0001 == 1;
        if self.strobe {
            self.button_index = 0;
        }
    }

    pub fn read(&mut self) -> u8 {
        if self.button_index > 7 {
            return 1;
        }

        let response = (u8::from(self.buttons) & (1 << self.button_index)) >> self.button_index;

        if !self.strobe {
            self.button_index += 1;
        }

        response
    }

    pub fn set_button_pressed(&mut self, button: Buttons, pressed: bool) {
        self.buttons.set_button(u8::from(button), pressed);
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}
