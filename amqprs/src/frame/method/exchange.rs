use amqp_serde::types::{AmqpExchangeName, Boolean, FieldTable, Octect, ShortStr, ShortUint};
use serde::{Deserialize, Serialize};

mod bit_flag {
    pub mod declare {
        // continous bits packed into one or more octets, starting from the low bit in each octet.
        pub const NO_WAIT: u8 = 0b0001_0000;
        pub const INTERNAL: u8 = 0b0000_1000;
        pub const AUTO_DELETE: u8 = 0b0000_0100;
        pub const DURABLE: u8 = 0b0000_0010;
        pub const PASSIVE: u8 = 0b0000_0001;
    }
    pub mod delete {

        pub const IF_UNUSED: u8 = 0b0000_0001;
        pub const NO_WAIT: u8 = 0b0000_0010;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Declare {
    pub ticket: ShortUint,
    pub exchange: AmqpExchangeName,
    pub typ: ShortStr,
    pub bits: Octect,
    pub arguments: FieldTable,
}

impl Declare {
    pub fn set_passive(&mut self, value: bool) {
        if value {
            self.bits |= bit_flag::declare::PASSIVE;
        } else {
            self.bits &= !bit_flag::declare::PASSIVE;
        }
    }
    pub fn set_durable(&mut self, value: bool) {
        if value {
            self.bits |= bit_flag::declare::DURABLE;
        } else {
            self.bits &= !bit_flag::declare::DURABLE;
        }
    }

    pub fn set_auto_delete(&mut self, value: bool) {
        if value {
            self.bits |= bit_flag::declare::AUTO_DELETE;
        } else {
            self.bits &= !bit_flag::declare::AUTO_DELETE;
        }
    }

    pub fn set_internal(&mut self, value: bool) {
        if value {
            self.bits |= bit_flag::declare::INTERNAL;
        } else {
            self.bits &= !bit_flag::declare::INTERNAL;
        }
    }

    pub fn set_no_wait(&mut self, value: bool) {
        if value {
            self.bits |= bit_flag::declare::NO_WAIT;
        } else {
            self.bits &= !bit_flag::declare::NO_WAIT;
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclareOk;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Delete {
    pub ticket: ShortUint,
    pub exchange: AmqpExchangeName,
    pub bits: Octect,
}
impl Delete {
    pub fn set_if_unused(&mut self, value: bool) {
        if value {
            self.bits |= bit_flag::delete::IF_UNUSED;
        } else {
            self.bits &= !bit_flag::delete::IF_UNUSED;
        }
    }

    pub fn set_no_wait(&mut self, value: bool) {
        if value {
            self.bits |= bit_flag::delete::NO_WAIT;
        } else {
            self.bits &= !bit_flag::delete::NO_WAIT;
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOk;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bind {
    pub ticket: ShortUint,
    pub destination: AmqpExchangeName,
    pub source: AmqpExchangeName,
    pub routing_key: ShortStr,
    pub nowait: Boolean,
    pub arguments: FieldTable,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BindOk;

#[derive(Debug, Serialize, Deserialize)]
pub struct Unbind {
    pub ticket: ShortUint,
    pub destination: AmqpExchangeName,
    pub source: AmqpExchangeName,
    pub routing_key: ShortStr,
    pub nowait: Boolean,
    pub arguments: FieldTable,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnbindOk;
