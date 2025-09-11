#[derive(Default)]
pub struct Buses {
    // External Buses
    pub addr: (u8, u8), // Address Bus (ABH, ABL)
    pub data: u8,       // Data Bus
    // Internal Buses
    pub base_addr: (u8, u8),      // Base Address Bus (BAH, BAL)
    pub effective_addr: (u8, u8), // Effective Address Bus (ADH, ADL)
    pub indirect_addr: (u8, u8),  // Indirect Address Bus (IAH, IAL)
}
