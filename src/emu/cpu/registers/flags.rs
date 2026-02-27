use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

/// Flags in the CPU's processor status word register.
#[derive(Default, Clone, Copy)]
pub struct Flags(pub u8);

impl Flags {
    /// Carry
    pub const C: u8 = 1 << 0;
    /// Zero
    pub const Z: u8 = 1 << 1;
    /// Interrupt Disable
    pub const I: u8 = 1 << 2;
    /// Decimal
    pub const D: u8 = 1 << 3;
    /// Break
    pub const B: u8 = 1 << 4;
    /// Unused
    pub const U: u8 = 1 << 5;
    /// Overflow
    pub const V: u8 = 1 << 6;
    /// Negative
    pub const N: u8 = 1 << 7;

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

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Self(value | Self::U)
    }
}

impl From<Flags> for u8 {
    fn from(value: Flags) -> u8 {
        value.0 | Flags::U
    }
}

impl BitAnd<u8> for Flags {
    type Output = u8;

    fn bitand(self, rhs: u8) -> Self::Output {
        self.0.bitand(rhs)
    }
}

impl BitAndAssign<u8> for Flags {
    fn bitand_assign(&mut self, rhs: u8) {
        *self = Self(self.bitand(rhs));
    }
}

impl BitOr<u8> for Flags {
    type Output = u8;

    fn bitor(self, rhs: u8) -> Self::Output {
        self.0.bitor(rhs)
    }
}

impl BitOrAssign<u8> for Flags {
    fn bitor_assign(&mut self, rhs: u8) {
        *self = Self(self.bitor(rhs))
    }
}

impl BitXor<u8> for Flags {
    type Output = u8;

    fn bitxor(self, rhs: u8) -> Self::Output {
        self.0.bitxor(rhs)
    }
}

impl BitXorAssign<u8> for Flags {
    fn bitxor_assign(&mut self, rhs: u8) {
        *self = Self(self.bitxor(rhs))
    }
}
