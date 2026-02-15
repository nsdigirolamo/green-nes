#[derive(Default, Clone, Copy)]
/// Internal CPU buses.
pub struct Buses {
    /// Base (BAH, BAL) address bus.
    pub base_addr: (u8, u8),
    /// Effective (ADH, ADL) address bus.
    pub effective_addr: (u8, u8),
    /// Indirect (IAH, IAL) address bus.
    pub indirect_addr: (u8, u8),
}
