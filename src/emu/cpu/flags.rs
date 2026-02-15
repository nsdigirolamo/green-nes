pub struct ProcessorStatusRegister(u8);

impl ProcessorStatusRegister {
    /// Carry
    const C: u8 = 1 << 0;
    /// Zero
    const Z: u8 = 1 << 1;
    /// Interrupt Disable
    const I: u8 = 1 << 2;
    /// Decimal
    const D: u8 = 1 << 3;
    /// Break
    const B: u8 = 1 << 4;
    /// Unused
    const U: u8 = 1 << 5;
    /// Overflow
    const V: u8 = 1 << 6;
    /// Negative
    const N: u8 = 1 << 7;

    pub fn get_carry(&self) -> bool {
        self.0 & Self::C != 0
    }

    pub fn set_carry(&mut self, value: bool) {
        if value {
            self.0 |= Self::C
        } else {
            self.0 &= !Self::C
        }
    }

    pub fn get_zero(&self) -> bool {
        self.0 & Self::Z != 0
    }

    pub fn set_zero(&mut self, value: bool) {
        if value {
            self.0 |= Self::Z
        } else {
            self.0 &= !Self::Z
        }
    }

    pub fn get_interrupt_disable(&self) -> bool {
        self.0 & Self::I != 0
    }

    pub fn set_interrupt_disable(&mut self, value: bool) {
        if value {
            self.0 |= Self::I
        } else {
            self.0 &= !Self::I
        }
    }

    pub fn get_decimal(&self) -> bool {
        self.0 & Self::D != 0
    }

    pub fn set_decimal(&mut self, value: bool) {
        if value {
            self.0 |= Self::D
        } else {
            self.0 &= !Self::D
        }
    }

    pub fn get_break(&self) -> bool {
        self.0 & Self::B != 0
    }

    pub fn set_break(&mut self, value: bool) {
        if value {
            self.0 |= Self::B
        } else {
            self.0 &= !Self::B
        }
    }

    pub fn get_unused(&self) -> bool {
        true
    }

    pub fn get_overflow(&self) -> bool {
        self.0 & Self::V != 0
    }

    pub fn set_overflow(&mut self, value: bool) {
        if value {
            self.0 |= Self::V
        } else {
            self.0 &= !Self::V
        }
    }

    pub fn get_negative(&self) -> bool {
        self.0 & Self::N != 0
    }

    pub fn set_negative(&mut self, value: bool) {
        if value {
            self.0 |= Self::N
        } else {
            self.0 &= !Self::N
        }
    }
}

impl From<u8> for ProcessorStatusRegister {
    fn from(value: u8) -> Self {
        Self(value | Self::U)
    }
}

impl From<ProcessorStatusRegister> for u8 {
    fn from(value: ProcessorStatusRegister) -> u8 {
        value.0 | ProcessorStatusRegister::U
    }
}
