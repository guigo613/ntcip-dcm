use crate::{
    Device,
    ASN,
    Integer,
    OctetString,
    Application,
    StructASN,
    Sequence,
    AsnOID,
    OID
};
use super::Dcm;
use std::ops::{
    Deref,
    DerefMut
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StudyDataSetup { // 9.6
    Unknown,
    Reserved1,
    Reserved2,
    Reserved3,
    Reserved4,
    Reserved5,
    Reserved6,
    Reserved7,
    Reserved8,
    Reserved9,
    Reserved10,
    Reserved11,
    Reserved12,
    Reserved13,
    PvrStudyConfigTable(PvrStudyConfigEntry),
    Reserved15,
    Reserved16,
    Reserved17,
    Reserved18,
    Reserved19,
    Reserved20,
    Reserved21,
    Reserved22,
    Reserved23,
    Reserved24,
    Reserved25,
    Reserved26,
    Reserved27,
    Reserved28,
    Reserved29,
    DataStructureTable(DataStructureTable),
}

impl StudyDataSetup {
    pub fn new(oid: isize) -> Self {
        match oid {
            14 => Self::PvrStudyConfigTable(Default::default()),
            30 => Self::DataStructureTable(Default::default()),
            _ => Self::Unknown
        }
    }
}

impl OID for StudyDataSetup {
    fn to_oid(&self) -> Vec<isize> {
        unsafe {
            vec![0, 6, *(self as *const _ as *const u8) as isize]
        }
    }

    fn absolute(self) -> Device {
        Dcm::StudyDataSetup(self).absolute()
    }

    fn get_name(&self) -> &'static str {
        match self {
            Self::PvrStudyConfigTable(_) => "PvrStudyConfigTable",
            Self::DataStructureTable(_) => "DataStructureTable",
            _ => "StudyDataSetup"
        }
    }

    fn get_values(&self) -> Vec<isize> {
        match self {
            _ => unimplemented!()
        }
    }
    
    fn len_value(&self) -> i32 {
        -1
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PvrStudyConfigEntry {
    pub psc_study_num: Integer,
    pub pvr_study_index: Integer,
    pub pvr_param: AsnOID,
    pub pvr_study_config_row_admin: Integer
}

impl PvrStudyConfigEntry {
    pub fn set(&mut self, values: ASN) {
        if let ASN::Sequence(seq) = values {
            let mut iterator = seq.into_iter();

            self.psc_study_num = iterator.next().unwrap().try_into().unwrap();
            self.pvr_study_index = iterator.next().unwrap().try_into().unwrap();
            self.pvr_param = iterator.next().unwrap().try_into().unwrap();
            self.pvr_study_config_row_admin = iterator.next().unwrap().try_into().unwrap();
        }
    }

    pub fn index(&self) -> isize {
        *self.pvr_study_index
    }

    pub fn param(&self) -> &AsnOID {
        &self.pvr_param
    }
    
    pub fn to_sequence(&self) -> Sequence {
        let mut seq = Sequence::new();

        seq.push(self.psc_study_num.to_asn());
        seq.push(self.pvr_study_index.to_asn());
        seq.push(self.pvr_param.clone().to_asn());
        seq.push(self.pvr_study_config_row_admin.to_asn());

        seq
    }
}

impl Default for PvrStudyConfigEntry {
    fn default() -> Self {
        Self { 
            psc_study_num: Default::default(),
            pvr_study_index: Default::default(),
            pvr_param: Default::default(),
            pvr_study_config_row_admin: Default::default()
         }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataStructureTable(Vec<DataStructureEntry>);

impl DataStructureTable {
    pub fn set(&mut self, values: ASN) {
        if let ASN::Sequence(seq) = values {
            for s in seq {
                self.push(DataStructureEntry::new(s));
            }
        }
    }

    pub fn to_vec(self) -> Vec<DataStructureEntry> {
        let DataStructureTable(vec) = self;
        vec
    }
}

impl Deref for DataStructureTable {
    type Target = Vec<DataStructureEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DataStructureTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for DataStructureTable {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataStructureEntry {
    pub ds_study_num: Integer,
    pub data_struc_index: Integer,
    pub start_time: Application,
    pub end_time: Application,
    pub data_num_records: Integer,
    pub data_encoding: Integer,
    pub data: OctetString
}

impl DataStructureEntry {
    pub fn new(values: ASN) -> Self {
        if let ASN::Sequence(seq) = values {
            let s: Sequence = seq.try_into().unwrap();
            let mut iterator = s.into_iter();

            Self {
                ds_study_num: iterator.next().unwrap().try_into().unwrap(),
                data_struc_index: iterator.next().unwrap().try_into().unwrap(),
                start_time: iterator.next().unwrap().try_into().unwrap(),
                end_time: iterator.next().unwrap().try_into().unwrap(),
                data_num_records: iterator.next().unwrap().try_into().unwrap(),
                data_encoding: iterator.next().unwrap().try_into().unwrap(),
                data: iterator.next().unwrap().try_into().unwrap(),
            }
        } else { Default::default() }
    }

    pub fn study_num(&self) -> Integer {
        self.ds_study_num
    }

    pub fn index(&self) -> Integer {
        self.data_struc_index
    }

    pub fn start(&self) -> &Application {
        &self.start_time
    }

    pub fn end(&self) -> &Application {
        &self.end_time
    }

    pub fn records(&self) -> Integer {
        self.data_num_records
    }

    pub fn encoding(&self) -> Integer {
        self.data_encoding
    }


    pub fn set(&mut self, values: ASN) {
        if let ASN::Sequence(seq) = values {
            let s: Sequence = seq.into_iter().next().unwrap().try_into().unwrap();
            let mut iterator = s.into_iter();

            self.ds_study_num = iterator.next().unwrap().try_into().unwrap();
            self.data_struc_index = iterator.next().unwrap().try_into().unwrap();
            self.start_time = iterator.next().unwrap().try_into().unwrap();
            self.end_time = iterator.next().unwrap().try_into().unwrap();
            self.data_num_records = iterator.next().unwrap().try_into().unwrap();
            self.data_encoding = iterator.next().unwrap().try_into().unwrap();
            self.data = iterator.next().unwrap().try_into().unwrap();
        }
    }
}

impl Deref for DataStructureEntry {
    type Target = OctetString;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for DataStructureEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Default for DataStructureEntry {
    fn default() -> Self {
        Self { 
            ds_study_num: Default::default(),
            data_struc_index: Default::default(),
            start_time: Default::default(),
            end_time: Default::default(),
            data_num_records: Default::default(),
            data_encoding: Default::default(),
            data: Default::default()
         }
    }
}