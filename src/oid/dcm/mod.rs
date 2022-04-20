pub mod study_config;
pub mod veh_criteria;
pub mod site_setup;
pub mod study_data_setup;
pub mod mib_version_number;
pub mod relative_addr_oid;

use super::{
    OID,
    OctetString
};
use std::ops::Deref;
pub use veh_criteria::VehCriteria;
pub use site_setup::{
    SiteSetup,
    SiteDescription,
    SiteId
};
pub use study_config::*;
pub use study_data_setup::*;
pub use mib_version_number::MibVersionNumber;
pub use relative_addr_oid::RelativeAddrOID;
pub use super::Device;

#[allow(unused)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Dcm { // 9
    Unknown,
    VehCriteria(VehCriteria),
    Reserved2,
    Reserved3,
    SiteSetup(SiteSetup),
    StudyConfig(StudyConfig),
    StudyDataSetup(StudyDataSetup),
    Reserved7,
    Reserved8,
    RelativeAddrOID(RelativeAddrOID),
    MibVersionNumber(MibVersionNumber),

}

impl Dcm {
    pub fn new(oid: &[isize]) -> Self {
        match oid.get(1) {
            Some(1) => Self::VehCriteria(VehCriteria::new(*oid.get(2).unwrap_or(&0))),
            Some(4) => {
                let num = *oid.get(2).unwrap_or(&0);
                if num == 9 {
                    Self::VehCriteria(VehCriteria::new(9999)) // Self::SiteSetup(SiteSetup::new(*oid.get(2).unwrap_or(&0)))
                } else {
                    Self::SiteSetup(SiteSetup::new(*oid.get(2).unwrap_or(&0)))
                }
            },
            Some(5) => Self::StudyConfig(StudyConfig::new(*oid.get(2).unwrap_or(&0))),
            Some(6) => Self::StudyDataSetup(StudyDataSetup::new(*oid.get(2).unwrap_or(&0))),
            Some(9) => Self::RelativeAddrOID(Default::default()),
            Some(10) => Self::MibVersionNumber(Default::default()),
            None | Some(_) => Default::default()
        }
    }
}

impl Into<OctetString> for Dcm {
    fn into(self) -> OctetString {
        match self {
            Self::VehCriteria(criteria) => criteria.into(),
            _ => unimplemented!(),
        }
    }
}

impl Deref for Dcm {
    type Target = dyn OID;
    
    fn deref(&self) -> &Self::Target {
        match self {
            Self::VehCriteria(v) => v,
            Self::StudyConfig(v) => v,
            Self::StudyDataSetup(v) => v,
            Self::RelativeAddrOID(v) => v,
            Self::MibVersionNumber(v) => v,
            Self::SiteSetup(v) => v,
            _ => unimplemented!()
        }
    }
}

impl OID for Dcm {
    fn to_oid(&self) -> Vec<isize> {
        let mut oid = Vec::from([0, 9]);
        if let Self::Unknown = self {
            oid
        } else {
            oid.extend_from_slice(&(**self).to_oid()[1..]);
    
            oid
        }
    }

    fn absolute(self) -> Device {
        Device::DCM(self).absolute()
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

impl Default for Dcm {
    fn default() -> Self {
        Self::Unknown
    }
}