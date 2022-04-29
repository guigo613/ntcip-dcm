use super::{
    Sequence,
    TryIntoOID,
    OctetString,
    StructASN,
    AsnOID,
    ASN,
    Integer,
    Application,
    OIDTuple,
    OID,
    oid_dcm::{
        RelativeAddrOID,
        MibVersionNumber,
        StudyDataSetup,
        PvrStudyConfigEntry,
        DataStructureEntry,
        SiteSetup,
        SiteId,
        SiteDescription,
        DataStructureTable,
        VehCriteria,
        StudyConfig,
        Dcm
    },
    Device,
};
use std::{
    ops::{
        DerefMut,
        Deref
    },
    ptr,
    error::Error,
    collections::HashMap,
    io::{
        Error as IoError,
        ErrorKind
    }
};

#[derive(Debug)]
pub struct DCMConfig {
    study_config_table: Vec<StudyConfig>
}

impl DCMConfig {
    // fn new(int idx, dcmVehCriteria val) {
    //     ordering.Add(idx, val);
    // }
}

impl Deref for DCMConfig {
    type Target = Vec<StudyConfig>;

    fn deref(&self) -> &Self::Target {
        &self.study_config_table
    }
}

impl DerefMut for DCMConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.study_config_table
    }
}

impl Default for DCMConfig {
    fn default() -> Self {
        Self {
            study_config_table: Default::default()
        }
    }
}

#[derive(Debug)]
pub struct DCMHeader {
    pub site_id: SiteId,
    pub site_description: SiteDescription,
    pub config: DCMConfig,
    pub content: HashMap<isize, PvrStudyConfigEntry>,
    pub relative: RelativeAddrOID
}

impl DCMHeader {
    fn new(mut header: Sequence) -> Self {
        let head = header.remove(0);
        let mut dcm = Self::default();
        if let ASN::Sequence(seq) = head {
            let mut s = seq.into_iter();

            for _ in 0..s.len() / 2 {
                if let Some(ASN::OID(oid)) = s.next() {
                    dcm.add(oid, s.next());
                }
            }
        }

        dcm
    }

    pub fn get_content_position(&self, oid: &AsnOID) -> Option<usize> {
        self.content.values().find_map(|x| if x.param() == oid { Some(x.index() as usize) } else { None })
    }

    fn read_body(&self, body: ASN) -> DCMBody {
        let seq: Sequence = body.try_into().unwrap();
        let mut s = seq.into_iter();
        let mut dcm = DCMBody::default();

        for _ in 0..s.len() {
            if let Some(ASN::OID(mut oid)) = s.next() {
                self.get_absolute(&mut oid);

                match oid.try_into_oid() {
                    Ok(Device::DCM(Dcm::StudyDataSetup(StudyDataSetup::DataStructureTable(mut entry)))) => {
                        if let Some(v) = s.next() {
                            entry.set(v);
                        }
                        
                        for e in entry.iter() {
                            let mut cloned = e.clone();
                            let mut study = Vec::new();
                            while cloned.len() > self.content.len() {
                                let mut vehicle = Vec::new();
                                for idx in 1..=self.content.len() {
                                    let mut oid = self.content.get(&(idx as isize)).unwrap().param().try_into_oid().unwrap();
                                    oid.get_value(&mut cloned);
                                    vehicle.push(oid);
                                }
                                study.push(vehicle);
                            }
                            let data = DCMDataStructEntry::new(e, study);
                            dcm.set(data);
                        }
                    },
                    Ok(dbg) => { dbg!(dbg); },
                    dbg => { dbg!(dbg); },
                };
            }
        }

        dcm
    }

    fn get_absolute(&self, oid: &mut AsnOID) {
        if oid.starts_with(&[0]) && self.relative.is_some() {
            let mut absolute = (*self.relative).clone().unwrap();
            absolute.extend_from_slice(&oid[1..]);
            *oid = absolute;
        }
    }

    fn to_relative(&self, oid: impl OID) -> Vec<isize> {
        let temp = AsnOID::default();
        let relative = self.relative.as_ref().unwrap_or(&temp);
        let mut oid = oid.absolute().to_oid();
        if !relative.is_empty() && oid.starts_with(&relative) {
            oid = oid.drain(relative.len()..).collect();
            oid.insert(0, 0);
        }
        oid
    }

    fn add(&mut self, mut oid: AsnOID, value: Option<ASN>) {
        self.get_absolute(&mut oid);
        match oid.try_into_oid() {
            Ok(Device::DCM(Dcm::SiteSetup(SiteSetup::SiteId(mut site_id)))) => {
                if let Some(v) = value {
                    site_id.set(v);
                }
                self.site_id = site_id;
            },
            Ok(Device::DCM(Dcm::SiteSetup(SiteSetup::SiteDescription(mut description)))) => {
                if let Some(v) = value {
                    description.set(v);
                }
                self.site_description = description;
            },
            Ok(Device::DCM(Dcm::RelativeAddrOID(mut relative))) => {
                if let Some(v) = value {
                    relative.set(v);
                }
                self.relative = relative;
            },
            Ok(Device::DCM(Dcm::StudyDataSetup(StudyDataSetup::PvrStudyConfigTable(mut entry)))) => {
                if let Some(v) = value {
                    entry.set(v);
                }
                self.content.insert(entry.index(), entry);
            },
            Ok(Device::DCM(Dcm::StudyConfig(mut config))) => {
                if let StudyConfig::StudyConfigTable(ref mut entry) = config {
                    if let Some(v) = value {
                        entry.set(v);
                    }
                }
                self.config.push(config);
            },
            Ok(dbg) => { dbg!(dbg); },
            dbg => { dbg!(dbg); },
        };
    }
}

impl Default for DCMHeader {
    fn default() -> Self {
        Self {
            site_id             : Default::default(),
            site_description    : Default::default(),
            config              : Default::default(),
            content             : Default::default(),
            relative            : Default::default()
        }
    }
}

impl Into<Sequence> for DCMHeader {
    fn into(mut self) -> Sequence {
        let mut seq = Sequence::new();
        let mut s = Sequence::new();
        let r = self.relative.clone().take().unwrap_or(AsnOID::default());

        if !r.is_empty()  {
            s.push(AsnOID::new(self.relative.clone().absolute().to_oid()).to_asn());
            s.push(r.clone().to_asn());
        }
        if !self.site_id.is_empty() {
            let site_id = &self.site_id;
            let value = site_id.as_bytes().to_vec();
            let oid = self.to_relative(site_id.clone());
            s.push(AsnOID::new(oid).to_asn());
            s.push(OctetString::new(value).to_asn());
        }
        if !self.site_description.is_empty() {
            let site_description = &self.site_description;
            let value = self.site_description.as_bytes().to_vec();
            let oid = self.to_relative(site_description.clone());
            s.push(AsnOID::new(oid).to_asn());
            s.push(OctetString::new(value).to_asn());
        }
        for cfg in self.config.iter() {
            if let StudyConfig::StudyConfigTable(c) = cfg {
                let oid = self.to_relative(cfg.clone());
                s.push(AsnOID::new(oid).to_asn());
                s.push(c.to_sequence().to_asn());
            }
        }
        for idx in 1..=self.content.len() {
            if let Some(content) = self.content.remove(&(idx as isize)) {
                let oid = self.to_relative(StudyDataSetup::PvrStudyConfigTable(content.clone()));
                s.push(AsnOID::new(oid).to_asn());
                s.push(content.to_sequence().to_asn());
            }
        }
        seq.push(s.to_asn());

        seq
    }
}

#[derive(Debug)]
pub struct DCMBody {
    pub data_struct: Vec<DCMDataStructEntry>,
}

impl DCMBody {
    fn set(&mut self, v: DCMDataStructEntry) {
        self.data_struct.push(v);
    }

    // pub fn to_tuples(&self) -> HashMap<isize, Vec<Vec<OIDTuple>>> {
    //     let mut map = HashMap::new();

    //     for (&k, v) in self.iter() {
    //         map.insert(k, v.iter().map(|x| {
    //             let mut z = Vec::new();
    //             for y in x {
    //                 let tuples = Device::get_tuples(y);
    //                 z.extend(tuples);
    //             }
    //             z
    //         }).collect());
    //     }

    //     map
    // }
}

impl Deref for DCMBody {
    type Target = Vec<DCMDataStructEntry>;

    fn deref(&self) -> &Self::Target {
        &self.data_struct
    }
}

impl DerefMut for DCMBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data_struct
    }
}

impl Default for DCMBody {
    fn default() -> Self {
        Self {
            data_struct: Default::default(),
        }
    }
}

impl DCMBody {
    fn into(self, header: &DCMHeader) -> Sequence {
        let mut seq = Sequence::new();
        let data: Sequence = self.data_struct.into();
        let oid = header.to_relative(StudyDataSetup::new(30));
        seq.push(AsnOID::new(oid).to_asn());
        seq.push(data.to_asn());
        
        seq
    }
}

impl Into<Sequence> for Vec<DCMDataStructEntry> {
    fn into(self) -> Sequence {
        self.into_iter().map(|x| {
            let v: Sequence = x.into();
            v
        }).collect()
    }
}

#[derive(Debug)]
pub struct DCMDataStructEntry {
    pub ds_study_num: Integer,
    pub data_struc_index: Integer,
    pub start_time: Application,
    pub end_time: Application,
    pub data_num_records: Integer,
    pub data_encoding: Integer,
    data: Vec<Vec<Device>>
}

impl DCMDataStructEntry {
    fn new(entry: &DataStructureEntry, data: Vec<Vec<Device>>) -> Self {
        Self {
            ds_study_num: entry.ds_study_num,
            data_struc_index: entry.data_struc_index,
            start_time: entry.start_time.clone(),
            end_time: entry.end_time.clone(),
            data_num_records: entry.data_num_records,
            data_encoding: entry.data_encoding,
            data,
        }
    }

    pub fn filter_remove<T>(&mut self, filter: T) 
        where T: Fn(&Device) -> bool
    {
        let data = self.data.as_mut_ptr();
        let mut iterator = self.data.iter_mut().enumerate();
        let mut len = iterator.len();
        while let Some((idx, d)) = iterator.next_back() {
            if d.iter().fold(false, |x, y| filter(y) || x) {
                unsafe {
                    ptr::read(data.add(idx));
                    ptr::copy(data.add(idx + 1), data.add(idx), len - idx);
                }
                len -= 1;
            }
        }
        unsafe { self.data.set_len(len) }
        self.data_num_records.set(len as isize);
    }

    pub fn filter_mut<T>(&mut self, filter: T) -> Vec<&mut Vec<Device>>
        where T: Fn(&Device) -> bool
    {
        self.data.iter_mut().filter(|d| d.iter().fold(false, |x, y| filter(y) || x)).collect()
    }
}

impl Deref for DCMDataStructEntry {
    type Target = Vec<Vec<Device>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for DCMDataStructEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Into<Sequence> for DCMDataStructEntry {
    fn into(self) -> Sequence {
        let mut seq = Sequence::new();
        let mut data = Vec::new();

        for devices in self.data {
            for device in devices {
                let oct: OctetString = device.into();
                let val: Vec<u8> = oct.into();
                data.extend(val);
            }
        }

        seq.push(self.ds_study_num.to_asn());
        seq.push(self.data_struc_index.to_asn());
        seq.push(self.start_time.to_asn());
        seq.push(self.end_time.to_asn());
        seq.push(self.data_num_records.to_asn());
        seq.push(self.data_encoding.to_asn());
        seq.push(OctetString::new(data).to_asn());
        
        seq
    }
}

#[derive(Debug)]
pub struct DCM {
    version: MibVersionNumber,
    pub header: DCMHeader,
    body: DCMBody,
}

impl DCM {
    pub fn set_version(&mut self, value: isize) {
        self.version.set(ASN::Integer(Integer::new(value)));
    }

    pub fn set_site_id(&mut self, site_id: &str) {
        let id = if site_id.len() > 40 {
            site_id[0..40].as_bytes()
        } else {
            site_id.as_bytes()
        };

        self.header.site_id.set(ASN::OctetString(OctetString::new(id.to_owned())));
    }

    pub fn set_site_description(&mut self, site_description: &str) {
        self.header.site_description.set(ASN::OctetString(OctetString::new(site_description.as_bytes().to_owned())));
    }
}

impl Into<Sequence> for DCM {
    fn into(self) -> Sequence {
        let mut seq = Sequence::new();
        let version = self.version.get_values().remove(0);
        let body: Sequence = self.body.into(&self.header);
        let header: Sequence = self.header.into();

        seq.push(AsnOID::new(self.version.absolute().to_oid()).to_asn());
        seq.push(Integer::new(version).to_asn());
        seq.push(header.to_asn());
        seq.push(body.to_asn());

        seq
    }
}

impl TryInto<DCM> for Sequence {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<DCM, Self::Error> {
        let sequence: Sequence = self.to_asn().try_into()?;
        let mut values = sequence.into_iter();
        let value = values.next().ok_or(IoError::from(ErrorKind::InvalidData))?;
        let oid = value.get_oid()?;

        let version = if let Device::DCM(Dcm::MibVersionNumber(mut v)) = oid.try_into_oid()? {
            if let Some(val) = values.next() {
                v.set(val);
            }
            v
        } else {
            panic!("Missing version!")
        };

        let header = DCMHeader::new(values.next()
            .ok_or(IoError::from(ErrorKind::InvalidData))?
            .try_into()?);
        let body = header.read_body(values.next()
            .ok_or(IoError::from(ErrorKind::InvalidData))?);
            
        Ok(DCM {
            version,
            header,
            body
        })
    }
}

impl Deref for DCM {
    type Target = DCMBody;

    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

impl DerefMut for DCM {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.body
    }
}