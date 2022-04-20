use crate::{
    Device,
    ASN,
    OID,
    Integer
};
use super::Dcm;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MibVersionNumber {
    version: Integer
}

impl MibVersionNumber {
    pub fn new(version: ASN) -> Self {
        let version = if let ASN::Integer(integer) = version {
            integer
        } else { Integer::new(0) };

        Self { version }
    }

    pub fn set(&mut self, version: ASN) {
        if let ASN::Integer(integer) = version {
            self.version = integer
        }
    }

    pub fn version(&self) -> f64 {
        (*self.version) as f64 / 100.0
    }
}

impl OID for MibVersionNumber {
    fn to_oid(&self) -> Vec<isize> {
        vec![0, 10]
    }

    fn absolute(self) -> Device {
        Dcm::MibVersionNumber(self).absolute()
    }

    fn get_name(&self) -> &'static str {
        "MibVersionNumber"
    }

    fn get_values(&self) -> Vec<isize> {
        vec![*self.version as isize]
    }
    
    fn len_value(&self) -> i32 {
        2
    }
}

impl Default for MibVersionNumber {
    fn default() -> Self {
        Self { version: Integer::new(0) }
    }
}