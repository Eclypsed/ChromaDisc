use num_enum::IntoPrimitive;

use super::{Command, Control};

#[derive(Debug)]
pub enum LoadEjectOperation {
    StopDisc,
    StartDisc,
    EjectIfPermitted,
    LoadAndStartDisc,
    JumptToFormatLayer(u8),
    ChangePowerCondition(PowerCondition),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
pub enum PowerCondition {
    NoChange = 0x0,
    Idle = 0x2,
    StandBy = 0x3,
    Sleep = 0x5,
}

#[derive(Debug)]
pub struct StartStopUnit {
    immediate: bool,
    load_eject_operation: LoadEjectOperation,
    control: Control,
}

impl StartStopUnit {
    pub fn new(immediate: bool, operation: LoadEjectOperation, control: Control) -> Self {
        Self {
            immediate,
            load_eject_operation: operation,
            control,
        }
    }
}

impl Command<6> for StartStopUnit {
    const OP_CODE: u8 = 0x1B;

    type Response = Vec<u8>;

    fn as_cdb(&self) -> [u8; 6] {
        let mut bytes = [0u8; 6];

        bytes[0] = Self::OP_CODE;
        bytes[1] |= u8::from(self.immediate);

        let (fl, fl_num, loej, start, pow_cond) = match self.load_eject_operation {
            LoadEjectOperation::StopDisc => (0u8, 0u8, 0u8, 0u8, 0u8),
            LoadEjectOperation::StartDisc => (0u8, 0u8, 0u8, 1u8, 0u8),
            LoadEjectOperation::EjectIfPermitted => (0u8, 0u8, 1u8, 0u8, 0u8),
            LoadEjectOperation::LoadAndStartDisc => (0u8, 0u8, 1u8, 1u8, 0u8),
            LoadEjectOperation::JumptToFormatLayer(n) => (1u8, n, 1u8, 1u8, 0u8),
            LoadEjectOperation::ChangePowerCondition(c) => (0u8, 0u8, 0u8, 0u8, c.into()),
        };

        bytes[3] |= fl_num & 0x11;
        bytes[4] |= pow_cond << 4;
        bytes[4] |= fl << 2;
        bytes[4] |= loej << 1;
        bytes[4] |= start;
        bytes[5] = self.control.into();

        bytes
    }

    fn allocation_len(&self) -> usize {
        128 // IDK lol
    }
}
