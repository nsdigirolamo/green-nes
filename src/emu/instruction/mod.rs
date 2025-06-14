use crate::{
    concat_u8,
    emu::{
        Operation, State,
        instruction::{
            access::{
                lda::LoadAccumulator, ldx::LoadX, ldy::LoadY, sta::StoreAccumulator, stx::StoreX,
                sty::StoreY,
            },
            arithmetic::{
                adc::AddWithCarry, dec::Decrement, dex::DecrementX, dey::DecrementY,
                inc::Increment, inx::IncrementX, iny::IncrementY, sbc::SubtractWithCarry,
            },
            transfer::{
                tax::TransferAccumulatorToX, tay::TransferAccumulatorToY,
                txa::TransferXToAccumulator, tya::TransferYToAccumulator,
            },
        },
    },
};

pub mod access;
pub mod arithmetic;
pub mod transfer;

pub enum NoOperation {
    Implied,
}

impl Operation for NoOperation {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        1
    }
}

pub enum Instruction {
    // Access Operations
    LDA(LoadAccumulator),
    STA(StoreAccumulator),
    LDX(LoadX),
    STX(StoreX),
    LDY(LoadY),
    STY(StoreY),

    // Transfer Operations
    TAX(TransferAccumulatorToX),
    TXA(TransferXToAccumulator),
    TAY(TransferAccumulatorToY),
    TYA(TransferYToAccumulator),

    // Arithmetic Operations
    ADC(AddWithCarry),
    SBC(SubtractWithCarry),
    INC(Increment),
    DEC(Decrement),
    INX(IncrementX),
    DEX(DecrementX),
    INY(IncrementY),
    DEY(DecrementY),

    NOP(NoOperation),
}

impl Operation for Instruction {
    fn execute_on(&self, state: State) -> State {
        match self {
            // Access Operations
            Instruction::LDA(lda) => lda.execute_on(state),
            Instruction::STA(sta) => sta.execute_on(state),
            Instruction::LDX(ldx) => ldx.execute_on(state),
            Instruction::STX(stx) => stx.execute_on(state),
            Instruction::LDY(ldy) => ldy.execute_on(state),
            Instruction::STY(sty) => sty.execute_on(state),

            // Transfer Operations
            Instruction::TAX(tax) => tax.execute_on(state),
            Instruction::TXA(txa) => txa.execute_on(state),
            Instruction::TAY(tay) => tay.execute_on(state),
            Instruction::TYA(tya) => tya.execute_on(state),

            // Arithmetic Operations
            Instruction::ADC(adc) => adc.execute_on(state),
            Instruction::SBC(svc) => svc.execute_on(state),
            Instruction::INC(inc) => inc.execute_on(state),
            Instruction::DEC(dec) => dec.execute_on(state),
            Instruction::INX(inx) => inx.execute_on(state),
            Instruction::DEX(dex) => dex.execute_on(state),
            Instruction::INY(iny) => iny.execute_on(state),
            Instruction::DEY(dey) => dey.execute_on(state),

            Instruction::NOP(nop) => nop.execute_on(state),
        }
    }

    fn get_size(&self) -> u8 {
        match self {
            // Access Operations
            Instruction::LDA(lda) => lda.get_size(),
            Instruction::STA(sta) => sta.get_size(),
            Instruction::LDX(ldx) => ldx.get_size(),
            Instruction::STX(stx) => stx.get_size(),
            Instruction::LDY(ldy) => ldy.get_size(),
            Instruction::STY(sty) => sty.get_size(),

            // Transfer Operations
            Instruction::TAX(tax) => tax.get_size(),
            Instruction::TXA(txa) => txa.get_size(),
            Instruction::TAY(tay) => tay.get_size(),
            Instruction::TYA(tya) => tya.get_size(),

            // Arithmetic Operations
            Instruction::ADC(adc) => adc.get_size(),
            Instruction::SBC(svc) => svc.get_size(),
            Instruction::INC(inc) => inc.get_size(),
            Instruction::DEC(dec) => dec.get_size(),
            Instruction::INX(inx) => inx.get_size(),
            Instruction::DEX(dex) => dex.get_size(),
            Instruction::INY(iny) => iny.get_size(),
            Instruction::DEY(dey) => dey.get_size(),

            Instruction::NOP(nop) => nop.get_size(),
        }
    }
}

pub fn get_instruction(bytes: (u8, u8, u8)) -> Instruction {
    let byte_one = bytes.0;
    let byte_two = bytes.1;
    let byte_three = bytes.2;

    match byte_one {
        0xA9 => Instruction::LDA(LoadAccumulator::Immediate { operand: byte_two }),
        0xA5 => Instruction::LDA(LoadAccumulator::ZeroPage { operand: byte_two }),
        0xB5 => Instruction::LDA(LoadAccumulator::ZeroPageX { operand: byte_two }),
        0xAD => Instruction::LDA(LoadAccumulator::Absolute {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0xBD => Instruction::LDA(LoadAccumulator::AbsoluteX {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0xB9 => Instruction::LDA(LoadAccumulator::AbsoluteY {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0xA1 => Instruction::LDA(LoadAccumulator::IndirectX { operand: byte_two }),
        0xB1 => Instruction::LDA(LoadAccumulator::IndirectY { operand: byte_two }),

        0x85 => Instruction::STA(StoreAccumulator::ZeroPage { operand: byte_two }),
        0x95 => Instruction::STA(StoreAccumulator::ZeroPageX { operand: byte_two }),
        0x8D => Instruction::STA(StoreAccumulator::Absolute {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0x9D => Instruction::STA(StoreAccumulator::AbsoluteX {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0x99 => Instruction::STA(StoreAccumulator::AbsoluteY {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0x81 => Instruction::STA(StoreAccumulator::IndirectX { operand: byte_two }),
        0x91 => Instruction::STA(StoreAccumulator::IndirectY { operand: byte_two }),

        0xA2 => Instruction::LDX(LoadX::Immediate { operand: byte_two }),
        0xA6 => Instruction::LDX(LoadX::ZeroPage { operand: byte_two }),
        0xB6 => Instruction::LDX(LoadX::ZeroPageX { operand: byte_two }),
        0xAE => Instruction::LDX(LoadX::Absolute {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0xBE => Instruction::LDX(LoadX::AbsoluteX {
            operand: concat_u8!(byte_three, byte_two),
        }),

        0x86 => Instruction::STX(StoreX::ZeroPage { operand: byte_two }),
        0x96 => Instruction::STX(StoreX::ZeroPageY { operand: byte_two }),
        0x8E => Instruction::STX(StoreX::Absolute {
            operand: concat_u8!(byte_three, byte_two),
        }),

        0xA0 => Instruction::LDY(LoadY::Immediate { operand: byte_two }),
        0xA4 => Instruction::LDY(LoadY::ZeroPage { operand: byte_two }),
        0xB4 => Instruction::LDY(LoadY::ZeroPageX { operand: byte_two }),
        0xAC => Instruction::LDY(LoadY::Absolute {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0xBC => Instruction::LDY(LoadY::AbsoluteX {
            operand: concat_u8!(byte_three, byte_two),
        }),

        0x84 => Instruction::STY(StoreY::ZeroPage { operand: byte_two }),
        0x94 => Instruction::STY(StoreY::ZeroPageX { operand: byte_two }),
        0x8C => Instruction::STY(StoreY::Absolute {
            operand: concat_u8!(byte_three, byte_two),
        }),

        0xAA => Instruction::TAX(TransferAccumulatorToX::Implied),

        0x8A => Instruction::TXA(TransferXToAccumulator::Implied),

        0xA8 => Instruction::TAY(TransferAccumulatorToY::Implied),

        0x98 => Instruction::TYA(TransferYToAccumulator::Implied),

        0x69 => Instruction::ADC(AddWithCarry::Immediate { operand: byte_two }),
        0x65 => Instruction::ADC(AddWithCarry::ZeroPage { operand: byte_two }),
        0x75 => Instruction::ADC(AddWithCarry::ZeroPageX { operand: byte_two }),
        0x6D => Instruction::ADC(AddWithCarry::Absolute {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0x7D => Instruction::ADC(AddWithCarry::AbsoluteX {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0x79 => Instruction::ADC(AddWithCarry::AbsoluteY {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0x61 => Instruction::ADC(AddWithCarry::IndirectX { operand: byte_two }),
        0x71 => Instruction::ADC(AddWithCarry::IndirectY { operand: byte_two }),

        0xE9 => Instruction::SBC(SubtractWithCarry::Immediate { operand: byte_two }),
        0xE5 => Instruction::SBC(SubtractWithCarry::ZeroPage { operand: byte_two }),
        0xF5 => Instruction::SBC(SubtractWithCarry::ZeroPageX { operand: byte_two }),
        0xED => Instruction::SBC(SubtractWithCarry::Absolute {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0xFD => Instruction::SBC(SubtractWithCarry::AbsoluteX {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0xF9 => Instruction::SBC(SubtractWithCarry::AbsoluteY {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0xE1 => Instruction::SBC(SubtractWithCarry::IndirectX { operand: byte_two }),
        0xF1 => Instruction::SBC(SubtractWithCarry::IndirectY { operand: byte_two }),

        0xE6 => Instruction::INC(Increment::ZeroPage { operand: byte_two }),
        0xF6 => Instruction::INC(Increment::ZeroPageX { operand: byte_two }),
        0xEE => Instruction::INC(Increment::Absolute {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0xFE => Instruction::INC(Increment::AbsoluteX {
            operand: concat_u8!(byte_three, byte_two),
        }),

        0xC6 => Instruction::DEC(Decrement::ZeroPage { operand: byte_two }),
        0xD6 => Instruction::DEC(Decrement::ZeroPageX { operand: byte_two }),
        0xCE => Instruction::DEC(Decrement::Absolute {
            operand: concat_u8!(byte_three, byte_two),
        }),
        0xDE => Instruction::DEC(Decrement::AbsoluteX {
            operand: concat_u8!(byte_three, byte_two),
        }),

        0xE8 => Instruction::INX(IncrementX::Implied),

        0xCA => Instruction::DEX(DecrementX::Implied),

        0xC8 => Instruction::INY(IncrementY::Implied),

        0x88 => Instruction::DEY(DecrementY::Implied),

        _ => Instruction::NOP(NoOperation::Implied), // @TODO: Remove this once all opcodes are matched.
    }
}
