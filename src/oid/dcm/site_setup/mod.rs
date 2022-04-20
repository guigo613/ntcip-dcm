mod logical_io_array_map_table;

use crate::{
    Device,
    OID,
    ASN
};
use super::Dcm;
use std::ops::Deref;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SiteSetup { // 9.4
    Unknown,
    SiteId(SiteId),
    Reserved2,
    SiteDescription(SiteDescription),
    Reserved4,
    Reserved5,
    Reserved6,
    Reserved7,
    Reserved8,
    LogicalIOArrayMapTable
}

impl SiteSetup {
    pub fn new(oid: isize) -> Self {
        match oid {
            1 => Self::SiteId(Default::default()),
            3 => Self::SiteDescription(Default::default()),
            _ => Self::Unknown
        }
    }
}

impl OID for SiteSetup {
    fn to_oid(&self) -> Vec<isize> {
        unsafe {
            let oid = *(self as *const _ as *const u8) as isize;
            vec![0, 4, oid]
        }
    }

    fn absolute(self) -> Device {
        Dcm::SiteSetup(self).absolute()
    }

    fn get_name(&self) -> &'static str {
        "SiteSetup"
    }

    fn get_values(&self) -> Vec<isize> {
        unimplemented!()
    }
    
    fn len_value(&self) -> i32 {
        -1
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SiteId {
    id: String
}

impl SiteId {
    pub fn set(&mut self, id: ASN) {
        let id: Option<Vec<u8>> = if let ASN::OctetString(oid) = id {
            Some(oid.into())
        } else { None };

        self.id = String::from_utf8_lossy(&id.unwrap_or(b"SiteId Unknown".to_vec())).to_string();
    }
}

impl OID for SiteId {
    fn to_oid(&self) -> Vec<isize> {
        vec![0, 4, 1]
    }

    fn absolute(self) -> Device {
        Dcm::SiteSetup(SiteSetup::SiteId(self)).absolute()
    }

    fn get_name(&self) -> &'static str {
        "SiteId"
    }

    fn get_values(&self) -> Vec<isize> {
        self.id.as_bytes().into_iter().map(|&v| v as isize).collect()
    }
    
    fn len_value(&self) -> i32 {
        -1
    }
}

impl Deref for SiteId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl Default for SiteId {
    fn default() -> Self {
        Self { 
            id: Default::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SiteDescription {
    description: String
}

impl SiteDescription {
    pub fn set(&mut self, description: ASN) {
        let description: Option<Vec<u8>> = if let ASN::OctetString(oid) = description {
            Some(oid.into())
        } else { None };

        self.description = String::from_utf8_lossy(&description.unwrap_or(b"Description Unknown".to_vec())).to_string();
    }
}

impl OID for SiteDescription {
    fn to_oid(&self) -> Vec<isize> {
        vec![0, 4, 3]
    }

    fn absolute(self) -> Device {
        Dcm::SiteSetup(SiteSetup::SiteDescription(self)).absolute()
    }

    fn get_name(&self) -> &'static str {
        "SiteDescription"
    }

    fn get_values(&self) -> Vec<isize> {
        self.description.as_bytes().into_iter().map(|&v| v as isize).collect()
    }
    
    fn len_value(&self) -> i32 {
        -1
    }
}

impl Deref for SiteDescription {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.description
    }
}

impl Default for SiteDescription {
    fn default() -> Self {
        Self { 
            description: Default::default()
        }
    }
}