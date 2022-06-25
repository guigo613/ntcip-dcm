mod asn;
mod oid;
mod dcm;
pub mod bin;

pub use asn::*;
pub use oid::{
    dcm as oid_dcm,
    *,
};
pub use dcm::*;

use std::{
    mem::ManuallyDrop,
    ffi::CString,
    mem
};


#[repr(C)]
#[derive(Debug)]
pub struct Lanes {
    site_id: *const i8,
    site_description: *const i8,
    ds_study_num: i32,
    data_struc_index: i32,
    start_time: i32,
    end_time: i32,
    data_num_records: i32,
    data_encoding: i32,
    size_oids: i32,
    oids: *const *const i8,
    size: i32,
    lanes: *const Lane,
}

impl Lanes {
    // fn new(dcm: DCM) -> Self {
    //     let lanes = dcm.to_tuples();
    //     let site_id = ManuallyDrop::new(CString::new((*dcm.header.site_id).clone()).unwrap_or_default()).as_ptr();
    //     let site_description = ManuallyDrop::new(CString::new((*dcm.header.site_description).clone()).unwrap_or_default()).as_ptr();
    //     let mut vec = Vec::new();
    //     for (k, v) in lanes {
    //         let len = vec.len() as isize;
    //         let idx = if k > len { len } else { k - 1 };
    //         vec.insert(idx as usize, Lane::new(k as i32, v));
    //     }
    //     let size = vec.len() as i32;
    //     let lanes = ManuallyDrop::new(vec).as_ptr();
    //     let len = dcm.header.content.len();
    //     let mut oids = ManuallyDrop::new(Vec::with_capacity(len));
    //     oids.resize(len, 0 as * const _);
    //     for (k, v) in dcm.header.content.iter() {
    //         let v = ManuallyDrop::new(CString::new(v.pvr_param.stringfy()).unwrap_or_default()).as_ptr();
    //         let k = *k as usize - 1;
    //         *oids.get_mut(k).unwrap() = v;
    //     }

    //     unsafe {
    //         Self {
    //             site_id,
    //             site_description,
    //             ds_study_num: *dcm.ds_study_num as i32,
    //             data_struc_index: *dcm.data_struc_index as i32,
    //             start_time: i32::from_be_bytes(*(&dcm.start_time[..4] as *const _ as *const [u8; 4])),
    //             end_time: i32::from_be_bytes(*(&dcm.end_time[..4] as *const _ as *const [u8; 4])),
    //             data_num_records: *dcm.data_num_records as i32,
    //             data_encoding: *dcm.data_encoding as i32,
    //             size_oids: oids.len() as i32,
    //             oids: oids.as_ptr(),
    //             size,
    //             lanes 
    //         }
    //     }
    // }
}

impl Default for Lanes {
    fn default() -> Self {
        let lanes = ManuallyDrop::new(Vec::new()).as_ptr();

        Self {
            site_id: 0 as *const _,
            site_description: 0 as *const _,
            ds_study_num: Default::default(),
            data_struc_index: Default::default(),
            start_time: Default::default(),
            end_time: Default::default(),
            data_num_records: Default::default(),
            data_encoding: Default::default(),
            size_oids: Default::default(),
            oids: 0 as *const _,
            size: Default::default(),
            lanes 
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Lane {
    lane: i32,
    size: i32,
    vehicles: *const Vehicle
}

impl Lane {
    fn new(lane: i32, vehicles: Vec<Vec<OIDTuple>>) -> Self {
        let size = vehicles.len() as i32;
        let vehicles = ManuallyDrop::new(vehicles.into_iter().map(|v| {
            Vehicle::new(v)
        }).collect::<Vec<Vehicle>>()).as_ptr();

        Self {
            lane,
            size,
            vehicles 
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Vehicle {
    size: i32,
    values: *const Value
}

impl Vehicle {
    fn new(values: Vec<OIDTuple>) -> Self {
        let size = values.len() as i32;
        let values = ManuallyDrop::new(values.into_iter().map(|v| {
            Value::new(v)
        }).collect::<Vec<Value>>()).as_ptr();

        Self {
            size,
            values 
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Value {
    oid: *const i8,
    name: *const i8,
    value: i32
}

impl Value {
    fn new(values: OIDTuple) -> Self {
        let oid = ManuallyDrop::new(CString::new(values.0).unwrap_or_default()).as_ptr();
        let name = ManuallyDrop::new(CString::new(values.1).unwrap_or_default()).as_ptr();
        let value = values.2 as i32;

        Self {
            oid,
            name,
            value 
        }
    }
}

// #[no_mangle]
// pub unsafe extern fn reader(file: *mut u8, size: i32) -> Lanes {
//     let file = ManuallyDrop::new(Vec::from_raw_parts(file, size as usize, size as usize));
// 	let mut p = AsnReader::new();
    
//     if let Some(asn) = p.read_ber(&file) {
//         let seq: Sequence = asn.try_into().unwrap();
//         if let Ok(dcm) = seq.try_into() {
//             return Lanes::new(dcm)
//         }
//     }

//     Default::default()
// }

#[repr(C)]
#[derive(Debug)]
pub struct OIDInfo {
    name: *const i8,
    len: i32,
}

impl OIDInfo {
    fn new(oid: Vec<isize>) -> Self {
        let oid = oid.try_into_oid().unwrap_or_default();
        let name = ManuallyDrop::new(CString::new(oid.get_name()).unwrap_or_default()).as_ptr();
        let len = oid.len_value();

        Self {
            name,
            len
        }
    }
}

#[no_mangle]
pub unsafe extern fn get_oid_type(o: *mut i32, size: i32) -> OIDInfo {
    let o = ManuallyDrop::new(Vec::from_raw_parts(o, size as usize, size as usize));
    let oid: Vec<isize> = o.iter().map(|x| *x as isize).collect();

    OIDInfo::new(oid)
}

#[no_mangle]
pub unsafe extern fn create_dcm() -> *mut DCM {
    Box::into_raw(Box::new(DCM::new()))
}

#[no_mangle]
pub unsafe extern fn set_header(dcm: *mut DCM, c: *mut u8, size: i32) {
    let mut asn = AsnReader::new();
    let content = Vec::from_raw_parts(c, size as usize, size as usize);
    let seq = asn.read_ber(&*content).expect("Header invalido!");
    (*dcm).header = DCMHeader::new(seq.try_into().expect("Deve começar com uma Sequencia"));
    mem::forget(content);
}

#[no_mangle]
pub unsafe extern fn time_header(dcm: *mut DCM, idx: u32, start_time: u32, end_time: u32) {
    if let Some(v) = (*dcm).header.config.get_mut(idx as usize) {
        if let StudyConfig::StudyConfigTable(v2) = v {
            v2.start_date_time = Application::new(start_time.to_be_bytes().to_vec());
            v2.end_date_time = Application::new(end_time.to_be_bytes().to_vec());
        }
    }
}

#[no_mangle]
pub unsafe extern fn set_body(
        dcm: *mut DCM,
        ds_study_num: i32,
        data_struc_index: i32,
        start_time: u32,
        end_time: u32,
        c: *mut u8,
        size: i32
    ) {
    let mut s = DataStructureEntry::default();
    let content = Vec::from_raw_parts(c, size as usize, size as usize);
    s.ds_study_num = Integer::new(ds_study_num as isize);
    s.data_struc_index = Integer::new(data_struc_index as isize);
    s.start_time = Application::new(start_time.to_be_bytes().to_vec());
    s.end_time = Application::new(end_time.to_be_bytes().to_vec());
    s.data_encoding = Integer::new(2);
    s.data = OctetString::new(content.clone());
    let data = (*dcm).header.translate_data(s);
    (*dcm).body.set(data);
    mem::forget(content);
}

#[no_mangle]
pub unsafe extern fn get_bytes(dcm: *mut DCM) -> Bytes {
    let seq: Sequence = (*dcm).clone().into();
    let v = seq.encode();

    let bytes = v.into();

    bytes
}

#[repr(C)]
pub struct Bytes {
    pointer: *const u8,
    size: i32
}

impl Bytes {
    fn new(v: Vec<u8>) -> Self {
        let pointer = v.as_ptr();
        let size = v.len() as i32;
        mem::forget(v);
        
        Self {
            pointer,
            size
        }
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(v: Vec<u8>) -> Self {
        Self::new(v)
    }
}

#[cfg(test)]
mod test {
    use std::fs;
    use crate::{
        asn::{
            AsnReader,
            Sequence,
            AsnOID,
            OctetString,
            StructASN
        },
        dcm::DCM,
    };

    #[test]
    fn encode_decode() {
        let file_reader = r"E:\User\Desktop\Rust\asn_ber\target\debug\1-1646388000.ber";

        let read = fs::read(file_reader).unwrap();
        let mut p = AsnReader::new();

        if let Some(asn) = p.read_ber(&read) {
            let sequence: Sequence = asn.try_into().unwrap();
            let seq = sequence.clone();
		    let dcm: DCM = sequence.try_into().unwrap();

            let encoded: Sequence = dcm.into();

            assert_eq!(encoded, seq);
            assert_eq!(read, encoded.encode());
        }
    }

    #[test]
    fn edit_dcm() {
        let file_reader = r"E:\User\Desktop\Rust\asn_ber\target\debug\1-1646392500.ber";

        let read = fs::read(file_reader).unwrap();
        let mut p = AsnReader::new();

        if let Some(asn) = p.read_ber(&read) {
            let sequence: Sequence = asn.try_into().unwrap();
            let mut seq = sequence.clone();
		    let mut dcm: DCM = sequence.try_into().unwrap();

            let header = seq.get_mut(2).unwrap().get_sequence_mut().unwrap();
            let head = header.get_mut(0).unwrap().get_sequence_mut().unwrap();
            if head.get(2) == Some(&AsnOID::new(vec![0, 4, 1]).to_asn()) {
                head.replace(3, OctetString::new("Teste Edit".as_bytes().to_vec()).to_asn());
            } else {
                head.insert(2, AsnOID::new(vec![0, 4, 1]).to_asn());
                head.insert(3, OctetString::new("Teste Edit".as_bytes().to_vec()).to_asn());
            }
            if head.get(4) == Some(&AsnOID::new(vec![0, 4, 3]).to_asn()) {
                head.replace(5, OctetString::new("1N,2S".as_bytes().to_vec()).to_asn());
            } else {
                head.insert(4, AsnOID::new(vec![0, 4, 3]).to_asn());
                head.insert(5, OctetString::new("1N,2S".as_bytes().to_vec()).to_asn());
            }


            dcm.set_site_id("Teste Edit");
            dcm.set_site_description("1N,2S");

            let encoded: Sequence = dcm.into();

            assert_eq!(encoded.get(0), seq.get(0));
            assert_eq!(encoded.get(1), seq.get(1));
            assert_eq!(encoded.get(2), seq.get(2));
            assert_eq!(encoded.get(3), seq.get(3));
        }
    }
}