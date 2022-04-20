use crate::{
    AsnOID,
    Device,
    OID,
    ASN,
};
use super::Dcm;
use std::ops::{
    Deref,
    DerefMut
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelativeAddrOID(Option<AsnOID>);

impl RelativeAddrOID {
    pub fn new(relative: ASN) -> Self {
        let relative = if let ASN::OID(oid) = relative {
            Some(oid)
        } else { None };

        Self(relative)
    }

    pub fn set(&mut self, relative: ASN) {
        let relative = if let ASN::OID(oid) = relative {
            Some(oid.into())
        } else { None };

        self.0 = relative;
    }
}

impl OID for RelativeAddrOID {
    fn to_oid(&self) -> Vec<isize> {
        vec![0, 9]
    }

    fn absolute(self) -> Device {
        Dcm::RelativeAddrOID(self).absolute()
    }

    fn get_name(&self) -> &'static str {
        "RelativeAddrOID"
    }

    fn get_values(&self) -> Vec<isize> {
        self.0.as_ref().map(|asn| (**asn).clone()).unwrap_or(vec![])
    }
    
    fn len_value(&self) -> i32 {
        -1
    }
}

impl Deref for RelativeAddrOID {
    type Target = Option<AsnOID>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RelativeAddrOID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for RelativeAddrOID {
    fn default() -> Self {
        Self(None)
    }
}