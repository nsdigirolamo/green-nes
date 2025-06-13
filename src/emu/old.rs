enum AddWithCarry {
    Immediate { operand: u8 },
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
    AbsoluteY { operand: u16 },
    IndirectX { operand: u8 },
    IndirectY { operand: u8 },
}

pub enum TransferAccumulatorToX {
    Implied,
}

pub enum TransferXToAccumulator {
    Implied,
}

pub enum TransferAccumulatorToY {
    Implied,
}

pub enum TransferYToAccumulator {
    Implied,
}

pub enum LoadY {
    Immediate { operand: u8 },
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
}

pub enum StoreAccumulator {
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
    AbsoluteY { operand: u16 },
    IndirectX { operand: u8 },
    IndirectY { operand: u8 },
}

pub enum StoreX {
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
}

pub enum StoreY {
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
}

enum BitwiseAnd {
    Immediate { operand: u8 },
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
    AbsoluteY { operand: u16 },
    IndirectX { operand: u8 },
    IndirectY { operand: u8 },
}

enum ArithmeticShiftLeft {
    Accumulator,
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
}

enum BranchOnCarryClear {
    Relative { operand: u8 },
}

enum BranchOnCarrySet {
    Relative { operand: u8 },
}

enum BranchOnEqual {
    Relative { operand: u8 },
}

enum BitTest {
    ZeroPage { operand: u8 },
    Absolute { operand: u16 },
}

enum BranchOnMinus {
    Relative { operand: u8 },
}

enum BranchOnNotEqual {
    Relative { operand: u8 },
}

enum BranchOnPlus {
    Relative { operand: u8 },
}

enum Break {
    Implied,
}

enum BranchOnOverflowClear {
    Relative { operand: u8 },
}

enum BranchOnOverflowSet {
    Relative { operand: u8 },
}

enum ClearCarry {
    Implied,
}

enum ClearDecimal {
    Implied,
}

enum ClearInterruptDisable {
    Implied,
}

enum ClearOverflow {
    Implied,
}

enum CompareAccumulator {
    Immediate { operand: u8 },
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
    AbsoluteY { operand: u16 },
    IndirectX { operand: u8 },
    IndirectY { operand: u8 },
}

enum CompareX {
    Immediate { operand: u8 },
    ZeroPage { operand: u8 },
    Absolute { operand: u16 },
}

enum CompareY {
    Immediate { operand: u8 },
    ZeroPage { operand: u8 },
    Absolute { operand: u16 },
}

enum DecrementMemory {
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
}

enum DecrementX {
    Implied,
}

enum DecrementY {
    Implied,
}

enum BitwiseExclusiveOr {
    Immediate { operand: u8 },
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
    AbsoluteY { operand: u16 },
    IndirectX { operand: u8 },
    IndirectY { operand: u8 },
}

enum IncrementMemory {
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
}

enum IncrementX {
    Implied,
}

enum IncrementY {
    Implied,
}

enum Jump {
    Absolute { operand: u16 },
    Indirect { operand: u16 },
}

enum JumpToSubroutine {
    Absolute { operand: u16 },
}

enum LogicalShiftRight {
    Accumulator,
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
}

enum NoOperation {
    Implied,
}

enum BitwiseOr {
    Immediate { operand: u8 },
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
    AbsoluteY { operand: u16 },
    IndirectX { operand: u8 },
    IndirectY { operand: u8 },
}

enum PushAccumulator {
    Implied,
}

enum PushProcessorStatus {
    Implied,
}

enum PullAccumulator {
    Implied,
}

enum PullProcessorStatus {
    Implied,
}

enum RotateLeft {
    Accumulator,
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
}

enum RotateRight {
    Accumulator,
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
}

enum ReturnFromInterrupt {
    Implied,
}

enum ReturnFromSubroutine {
    Implied,
}

enum SubtractWithCarry {
    Immediate { operand: u8 },
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
    AbsoluteY { operand: u16 },
    IndirectX { operand: u8 },
    IndirectY { operand: u8 },
}

enum SetCarry {
    Implied,
}

enum SetDecimal {
    Implied,
}

enum SetInterruptDisable {
    Implied,
}

enum TransferStackPointerToX {
    Implied,
}

enum TransferXToStackPointer {
    Implied,
}
