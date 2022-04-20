use crate::{
    Device,
    ASN,
    Integer,
    Application,
    StructASN,
    Sequence,
    OID
};
use super::Dcm;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StudyConfig { // 9.5
    Unknown,
    Reserved1,
    Reserved2,
    Reserved3,
    Reserved4,
    StudyConfigTable(StudyConfigEntry)
}

impl StudyConfig {
    pub fn new(oid: isize) -> Self {
        match oid {
            5 => Self::StudyConfigTable(Default::default()),
            _ => Self::Unknown
        }
    }
}

impl OID for StudyConfig {
    fn to_oid(&self) -> Vec<isize> {
        unsafe {
            vec![0, 5, *(self as *const _ as *const u8) as isize]
        }
    }

    fn absolute(self) -> Device {
        Dcm::StudyConfig(self).absolute()
    }

    fn get_name(&self) -> &'static str {
        match self {
            Self::StudyConfigTable(_) => "StudyConfigTable",
            _ => "StudyConfig"
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
pub struct StudyConfigEntry {
    sc_study_num: Integer,
    study_type: Integer,
    start_type: Integer,
    start_date_time: Application,
    end_type: Integer,
    end_date_time: Application,
    sc_file_study_index: Integer,
    study_config_row_admin: Integer
}

impl StudyConfigEntry {
    pub fn set(&mut self, values: ASN) {
        if let ASN::Sequence(seq) = values {
            let mut iterator = seq.into_iter();

            self.sc_study_num = iterator.next().unwrap().try_into().unwrap();
            self.study_type = iterator.next().unwrap().try_into().unwrap();
            self.start_type = iterator.next().unwrap().try_into().unwrap();
            self.start_date_time = iterator.next().unwrap().try_into().unwrap();
            self.end_type = iterator.next().unwrap().try_into().unwrap();
            self.end_date_time = iterator.next().unwrap().try_into().unwrap();
            self.sc_file_study_index = iterator.next().unwrap().try_into().unwrap();
            self.study_config_row_admin = iterator.next().unwrap().try_into().unwrap();
        }
    }

    pub fn to_sequence(&self) -> Sequence {
        let mut seq = Sequence::new();

        seq.push(self.sc_study_num.to_asn());
        seq.push(self.study_type.to_asn());
        seq.push(self.start_type.to_asn());
        seq.push(self.start_date_time.clone().to_asn());
        seq.push(self.end_type.to_asn());
        seq.push(self.end_date_time.clone().to_asn());
        seq.push(self.sc_file_study_index.to_asn());
        seq.push(self.study_config_row_admin.to_asn());

        seq
    }
}

impl Default for StudyConfigEntry {
    fn default() -> Self {
        Self { 
            sc_study_num: Default::default(),
            study_type: Default::default(),
            start_type: Default::default(),
            start_date_time: Default::default(),
            end_type: Default::default(),
            end_date_time: Default::default(),
            sc_file_study_index: Default::default(),
            study_config_row_admin: Default::default()
         }
    }
}