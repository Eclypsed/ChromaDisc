use crate::core::Control;

pub struct Inquiry {
    pub cmd_dt: bool, // Obsolete as of SPC-3
    pub evpd: bool,
    pub page_code: u8,
    pub allocation_length: u16,
    pub control: Control,
}

// type InquiryOpCode = OpCode<0x12>;
