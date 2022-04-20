use crate::{
    OID,
    OctetString,
    Device
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Date(u32);

impl Date {
    pub fn new(date: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(date))
    }
}

impl Into<OctetString> for Date {
    fn into(self) -> OctetString {
        OctetString::new(self.0.to_be_bytes().to_vec())
    }
}

impl OID for Date {
    fn to_oid(&self) -> Vec<isize> {
        vec![0, 6, 3, 6]
    }

    fn absolute(self) -> Device {
        Device::GLOBAL(self).absolute()
    }

    fn get_name(&self) -> &'static str {
        "Date"
    }

    fn get_values(&self) -> Vec<isize> {
        vec![self.0 as isize]
    }
    
    fn len_value(&self) -> i32 {
        4
    }
}