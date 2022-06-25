pub mod dcm;
pub mod global;

use std::{
    ops::Deref,
    io::{
        Error,
        ErrorKind
    }
};
use super::OctetString;
pub use dcm::{
    DataStructureEntry,
    StudyConfig,
    Dcm,
    veh_criteria::*
};
use global::Date;

pub type OIDResult = Result<Device, Error>;
pub type OIDTuple = (String, String, isize);

pub const BASE: [isize; 9] = [1, 3, 6, 1, 4, 1, 1206, 4, 2];

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Device {
    Unknown,
    ASC,
    GLOBAL(Date),
    DCM(Dcm),
}

impl Device {
    pub fn get_value(&mut self, values: &mut DataStructureEntry) {
        match self {
            Self::DCM(Dcm::VehCriteria(val)) => val.get_value(values),
            Self::GLOBAL(val) => *val = Date::new([values.remove(0), values.remove(0), values.remove(0), values.remove(0)]),
            _ => ()
        }
    }
}

impl Deref for Device {
    type Target = dyn OID;
    
    fn deref(&self) -> &Self::Target {
        match self {
            Self::GLOBAL(v) => v,
            Self::DCM(v) => v,
            _ => unimplemented!()
        }
    }
}

impl Into<OctetString> for Device {
    fn into(self) -> OctetString {
        match self {
            Self::GLOBAL(date) => date.into(),
            Self::DCM(dcm) => dcm.into(),
            _ => unimplemented!(),
        }
    }
}

impl OID for Device {
    fn to_oid(&self) -> Vec<isize> {
        let mut oid = Vec::from(BASE);
        if let Self::Unknown = self {
            oid
        } else {
            oid.extend_from_slice(&(**self).to_oid()[1..]);
    
            oid
        }
    }

    fn absolute(self) -> Self {
        self
    }

    fn get_name(&self) -> &'static str {
        (**self).get_name()
    }

    fn get_values(&self) -> Vec<isize> {
        (**self).get_values()
    }

    fn len_value(&self) -> i32 {
        (**self).len_value()
    }
}

impl Default for Device {
    fn default() -> Self {
        Self::Unknown
    }
}

pub trait OID {
    fn to_oid(&self) -> Vec<isize>;
    fn absolute(self) -> Device;
    fn get_name(&self) -> &'static str;
    fn get_values(&self) -> Vec<isize>;
    fn len_value(&self) -> i32;

    fn get_tuples(&self) -> Vec<OIDTuple> {
        let mut vec = Vec::new();
        let values = self.get_values();
        
        for (n, &v) in values.iter().enumerate() {
            let i = if values.len() > 1 { (n + 1).to_string() } else { String::new() };
            let oid = self.to_oid().iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>().join(".");

            vec.push((oid, format!("{}{}", self.get_name(), i), v))
        }

        vec
    }
}

pub trait TryIntoOID {
    fn try_into_oid(&self) -> OIDResult;
}

impl TryIntoOID for Vec<isize> {
    fn try_into_oid(&self) -> OIDResult {
        if self.starts_with(&BASE) {
            let oid = match self.get(9) {
                Some(9) => Device::DCM(Dcm::new(&self[9..])),
                Some(6) => Device::GLOBAL(Date::new([0; 4])),
                None | Some(_) => unimplemented!()
            };
            
            Ok(oid)
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }
}