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
            bitwise::{and::BitwiseAnd, bit::BitTest, eor::BitwiseExclusiveOr, ora::BitwiseOr},
            branch::{
                bcc::BranchIfCarryClear, bcs::BranchIfCarrySet, beq::BranchIfEqual,
                bmi::BranchIfMinus, bne::BranchIfNotEqual, bpl::BranchIfPlus,
                bvc::BranchIfOverflowClear, bvs::BranchIfOverflowSet,
            },
            compare::{cmp::CompareAccumulator, cpx::CompareX, cpy::CompareY},
            jump::{
                brk::Break, jmp::Jump, jsr::JumpToSubroutine, rti::ReturnFromInterrupt,
                rts::ReturnFromSubroutine,
            },
            shift::{
                asl::ArithmeticShiftLeft, lsr::LogicalShiftRight, rol::RotateLeft, ror::RotateRight,
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
pub mod bitwise;
pub mod branch;
pub mod compare;
pub mod jump;
pub mod shift;
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
    NOP(NoOperation),

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

    // Transfer Operations
    ASL(ArithmeticShiftLeft),
    LSR(LogicalShiftRight),
    ROL(RotateLeft),
    ROR(RotateRight),

    // Bitwise Operations
    AND(BitwiseAnd),
    ORA(BitwiseOr),
    EOR(BitwiseExclusiveOr),
    BIT(BitTest),

    // Comparison Operations
    CMP(CompareAccumulator),
    CPX(CompareX),
    CPY(CompareY),

    // Branch Operations
    BCC(BranchIfCarryClear),
    BCS(BranchIfCarrySet),
    BEQ(BranchIfEqual),
    BNE(BranchIfNotEqual),
    BPL(BranchIfPlus),
    BMI(BranchIfMinus),
    BVC(BranchIfOverflowClear),
    BVS(BranchIfOverflowSet),

    // Jump Operations
    JMP(Jump),
    JSR(JumpToSubroutine),
    RTS(ReturnFromSubroutine),
    BRK(Break),
    RTI(ReturnFromInterrupt),
}

impl Operation for Instruction {
    fn execute_on(&self, state: State) -> State {
        match self {
            Instruction::NOP(nop) => nop.execute_on(state),

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

            // Shift Operations
            Instruction::ASL(asl) => asl.execute_on(state),
            Instruction::LSR(lsr) => lsr.execute_on(state),
            Instruction::ROL(rol) => rol.execute_on(state),
            Instruction::ROR(ror) => ror.execute_on(state),

            // Bitwise Operations
            Instruction::AND(and) => and.execute_on(state),
            Instruction::ORA(ora) => ora.execute_on(state),
            Instruction::EOR(eor) => eor.execute_on(state),
            Instruction::BIT(bit) => bit.execute_on(state),

            // Comparison Operations
            Instruction::CMP(cmp) => cmp.execute_on(state),
            Instruction::CPX(cpx) => cpx.execute_on(state),
            Instruction::CPY(cpy) => cpy.execute_on(state),

            // Branch Operations
            Instruction::BCC(bcc) => bcc.execute_on(state),
            Instruction::BCS(bcs) => bcs.execute_on(state),
            Instruction::BEQ(beq) => beq.execute_on(state),
            Instruction::BNE(bne) => bne.execute_on(state),
            Instruction::BPL(bpl) => bpl.execute_on(state),
            Instruction::BMI(bmi) => bmi.execute_on(state),
            Instruction::BVC(bvc) => bvc.execute_on(state),
            Instruction::BVS(bvs) => bvs.execute_on(state),

            // Jump Operations
            Instruction::JMP(jmp) => jmp.execute_on(state),
            Instruction::JSR(jsr) => jsr.execute_on(state),
            Instruction::RTS(rts) => rts.execute_on(state),
            Instruction::BRK(brk) => brk.execute_on(state),
            Instruction::RTI(rti) => rti.execute_on(state),
        }
    }

    fn get_size(&self) -> u8 {
        match self {
            Instruction::NOP(nop) => nop.get_size(),

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

            // Shift Operations
            Instruction::ASL(asl) => asl.get_size(),
            Instruction::LSR(lsr) => lsr.get_size(),
            Instruction::ROL(rol) => rol.get_size(),
            Instruction::ROR(ror) => ror.get_size(),

            // Bitwise Operations
            Instruction::AND(and) => and.get_size(),
            Instruction::ORA(ora) => ora.get_size(),
            Instruction::EOR(eor) => eor.get_size(),
            Instruction::BIT(bit) => bit.get_size(),

            // Comparison Operations
            Instruction::CMP(cmp) => cmp.get_size(),
            Instruction::CPX(cpx) => cpx.get_size(),
            Instruction::CPY(cpy) => cpy.get_size(),

            // Branch Operations
            Instruction::BCC(bcc) => bcc.get_size(),
            Instruction::BCS(bcs) => bcs.get_size(),
            Instruction::BEQ(beq) => beq.get_size(),
            Instruction::BNE(bne) => bne.get_size(),
            Instruction::BPL(bpl) => bpl.get_size(),
            Instruction::BMI(bmi) => bmi.get_size(),
            Instruction::BVC(bvc) => bvc.get_size(),
            Instruction::BVS(bvs) => bvs.get_size(),

            // Jump Operations
            Instruction::JMP(jmp) => jmp.get_size(),
            Instruction::JSR(jsr) => jsr.get_size(),
            Instruction::RTS(rts) => rts.get_size(),
            Instruction::BRK(brk) => brk.get_size(),
            Instruction::RTI(rti) => rti.get_size(),
        }
    }
}

pub fn get_instruction(bytes: (u8, u8, u8)) -> Instruction {
    match bytes.0 {
        0xA9 => Instruction::LDA(LoadAccumulator::Immediate { operand: bytes.1 }),
        0xA5 => Instruction::LDA(LoadAccumulator::ZeroPage { operand: bytes.1 }),
        0xB5 => Instruction::LDA(LoadAccumulator::ZeroPageX { operand: bytes.1 }),
        0xAD => Instruction::LDA(LoadAccumulator::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0xBD => Instruction::LDA(LoadAccumulator::AbsoluteX {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0xB9 => Instruction::LDA(LoadAccumulator::AbsoluteY {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0xA1 => Instruction::LDA(LoadAccumulator::IndirectX { operand: bytes.1 }),
        0xB1 => Instruction::LDA(LoadAccumulator::IndirectY { operand: bytes.1 }),

        0x85 => Instruction::STA(StoreAccumulator::ZeroPage { operand: bytes.1 }),
        0x95 => Instruction::STA(StoreAccumulator::ZeroPageX { operand: bytes.1 }),
        0x8D => Instruction::STA(StoreAccumulator::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0x9D => Instruction::STA(StoreAccumulator::AbsoluteX {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0x99 => Instruction::STA(StoreAccumulator::AbsoluteY {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0x81 => Instruction::STA(StoreAccumulator::IndirectX { operand: bytes.1 }),
        0x91 => Instruction::STA(StoreAccumulator::IndirectY { operand: bytes.1 }),

        0xA2 => Instruction::LDX(LoadX::Immediate { operand: bytes.1 }),
        0xA6 => Instruction::LDX(LoadX::ZeroPage { operand: bytes.1 }),
        0xB6 => Instruction::LDX(LoadX::ZeroPageX { operand: bytes.1 }),
        0xAE => Instruction::LDX(LoadX::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0xBE => Instruction::LDX(LoadX::AbsoluteX {
            operand: concat_u8!(bytes.2, bytes.1),
        }),

        0x86 => Instruction::STX(StoreX::ZeroPage { operand: bytes.1 }),
        0x96 => Instruction::STX(StoreX::ZeroPageY { operand: bytes.1 }),
        0x8E => Instruction::STX(StoreX::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),

        0xA0 => Instruction::LDY(LoadY::Immediate { operand: bytes.1 }),
        0xA4 => Instruction::LDY(LoadY::ZeroPage { operand: bytes.1 }),
        0xB4 => Instruction::LDY(LoadY::ZeroPageX { operand: bytes.1 }),
        0xAC => Instruction::LDY(LoadY::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0xBC => Instruction::LDY(LoadY::AbsoluteX {
            operand: concat_u8!(bytes.2, bytes.1),
        }),

        0x84 => Instruction::STY(StoreY::ZeroPage { operand: bytes.1 }),
        0x94 => Instruction::STY(StoreY::ZeroPageX { operand: bytes.1 }),
        0x8C => Instruction::STY(StoreY::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),

        0xAA => Instruction::TAX(TransferAccumulatorToX::Implied),

        0x8A => Instruction::TXA(TransferXToAccumulator::Implied),

        0xA8 => Instruction::TAY(TransferAccumulatorToY::Implied),

        0x98 => Instruction::TYA(TransferYToAccumulator::Implied),

        0x69 => Instruction::ADC(AddWithCarry::Immediate { operand: bytes.1 }),
        0x65 => Instruction::ADC(AddWithCarry::ZeroPage { operand: bytes.1 }),
        0x75 => Instruction::ADC(AddWithCarry::ZeroPageX { operand: bytes.1 }),
        0x6D => Instruction::ADC(AddWithCarry::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0x7D => Instruction::ADC(AddWithCarry::AbsoluteX {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0x79 => Instruction::ADC(AddWithCarry::AbsoluteY {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0x61 => Instruction::ADC(AddWithCarry::IndirectX { operand: bytes.1 }),
        0x71 => Instruction::ADC(AddWithCarry::IndirectY { operand: bytes.1 }),

        0xE9 => Instruction::SBC(SubtractWithCarry::Immediate { operand: bytes.1 }),
        0xE5 => Instruction::SBC(SubtractWithCarry::ZeroPage { operand: bytes.1 }),
        0xF5 => Instruction::SBC(SubtractWithCarry::ZeroPageX { operand: bytes.1 }),
        0xED => Instruction::SBC(SubtractWithCarry::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0xFD => Instruction::SBC(SubtractWithCarry::AbsoluteX {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0xF9 => Instruction::SBC(SubtractWithCarry::AbsoluteY {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0xE1 => Instruction::SBC(SubtractWithCarry::IndirectX { operand: bytes.1 }),
        0xF1 => Instruction::SBC(SubtractWithCarry::IndirectY { operand: bytes.1 }),

        0xE6 => Instruction::INC(Increment::ZeroPage { operand: bytes.1 }),
        0xF6 => Instruction::INC(Increment::ZeroPageX { operand: bytes.1 }),
        0xEE => Instruction::INC(Increment::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0xFE => Instruction::INC(Increment::AbsoluteX {
            operand: concat_u8!(bytes.2, bytes.1),
        }),

        0xC6 => Instruction::DEC(Decrement::ZeroPage { operand: bytes.1 }),
        0xD6 => Instruction::DEC(Decrement::ZeroPageX { operand: bytes.1 }),
        0xCE => Instruction::DEC(Decrement::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0xDE => Instruction::DEC(Decrement::AbsoluteX {
            operand: concat_u8!(bytes.2, bytes.1),
        }),

        0xE8 => Instruction::INX(IncrementX::Implied),

        0xCA => Instruction::DEX(DecrementX::Implied),

        0xC8 => Instruction::INY(IncrementY::Implied),

        0x88 => Instruction::DEY(DecrementY::Implied),

        0x0A => Instruction::ASL(ArithmeticShiftLeft::Accumulator),
        0x06 => Instruction::ASL(ArithmeticShiftLeft::ZeroPage { operand: bytes.1 }),
        0x16 => Instruction::ASL(ArithmeticShiftLeft::ZeroPageX { operand: bytes.1 }),
        0x0E => Instruction::ASL(ArithmeticShiftLeft::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0x1E => Instruction::ASL(ArithmeticShiftLeft::AbsoluteX {
            operand: concat_u8!(bytes.2, bytes.1),
        }),

        0x4A => Instruction::LSR(LogicalShiftRight::Accumulator),
        0x46 => Instruction::LSR(LogicalShiftRight::ZeroPage { operand: bytes.1 }),
        0x56 => Instruction::LSR(LogicalShiftRight::ZeroPageX { operand: bytes.1 }),
        0x4E => Instruction::LSR(LogicalShiftRight::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0x5E => Instruction::LSR(LogicalShiftRight::AbsoluteX {
            operand: concat_u8!(bytes.2, bytes.1),
        }),

        0x2A => Instruction::ROL(RotateLeft::Accumulator),
        0x26 => Instruction::ROL(RotateLeft::ZeroPage { operand: bytes.1 }),
        0x36 => Instruction::ROL(RotateLeft::ZeroPageX { operand: bytes.1 }),
        0x2E => Instruction::ROL(RotateLeft::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0x3E => Instruction::ROL(RotateLeft::AbsoluteX {
            operand: concat_u8!(bytes.2, bytes.1),
        }),

        0x6A => Instruction::ROR(RotateRight::Accumulator),
        0x66 => Instruction::ROR(RotateRight::ZeroPage { operand: bytes.1 }),
        0x76 => Instruction::ROR(RotateRight::ZeroPageX { operand: bytes.1 }),
        0x6E => Instruction::ROR(RotateRight::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0x7E => Instruction::ROR(RotateRight::AbsoluteX {
            operand: concat_u8!(bytes.2, bytes.1),
        }),

        0xC9 => Instruction::CMP(CompareAccumulator::Immediate { operand: bytes.1 }),
        0xC5 => Instruction::CMP(CompareAccumulator::ZeroPage { operand: bytes.1 }),
        0xD5 => Instruction::CMP(CompareAccumulator::ZeroPageX { operand: bytes.1 }),
        0xCD => Instruction::CMP(CompareAccumulator::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0xDD => Instruction::CMP(CompareAccumulator::AbsoluteX {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0xD9 => Instruction::CMP(CompareAccumulator::AbsoluteY {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0xC1 => Instruction::CMP(CompareAccumulator::IndirectX { operand: bytes.1 }),
        0xD1 => Instruction::CMP(CompareAccumulator::IndirectY { operand: bytes.1 }),

        0xE0 => Instruction::CPX(CompareX::Immediate { operand: bytes.1 }),
        0xE4 => Instruction::CPX(CompareX::ZeroPage { operand: bytes.1 }),
        0xEC => Instruction::CPX(CompareX::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),

        0xC0 => Instruction::CPY(CompareY::Immediate { operand: bytes.1 }),
        0xC4 => Instruction::CPY(CompareY::ZeroPage { operand: bytes.1 }),
        0xCC => Instruction::CPY(CompareY::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),

        0x90 => Instruction::BCC(BranchIfCarryClear::Relative { operand: bytes.1 }),

        0xB0 => Instruction::BCS(BranchIfCarrySet::Relative { operand: bytes.1 }),

        0xF0 => Instruction::BEQ(BranchIfEqual::Relative { operand: bytes.1 }),

        0xD0 => Instruction::BNE(BranchIfNotEqual::Relative { operand: bytes.1 }),

        0x10 => Instruction::BPL(BranchIfPlus::Relative { operand: bytes.1 }),

        0x30 => Instruction::BMI(BranchIfMinus::Relative { operand: bytes.1 }),

        0x50 => Instruction::BVC(BranchIfOverflowClear::Relative { operand: bytes.1 }),

        0x70 => Instruction::BVS(BranchIfOverflowSet::Relative { operand: bytes.1 }),

        0x4C => Instruction::JMP(Jump::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),
        0x6C => Instruction::JMP(Jump::Indirect {
            operand: concat_u8!(bytes.2, bytes.1),
        }),

        0x20 => Instruction::JSR(JumpToSubroutine::Absolute {
            operand: concat_u8!(bytes.2, bytes.1),
        }),

        0x60 => Instruction::RTS(ReturnFromSubroutine::Implied),

        0x00 => Instruction::BRK(Break::Implied),

        0x40 => Instruction::RTI(ReturnFromInterrupt::Implied),

        _ => Instruction::NOP(NoOperation::Implied), // @TODO: Remove this once all opcodes are matched.
    }
}
