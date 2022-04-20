#![allow(unused_mut, unused)]

use asn_ber::*;
use std::{
	error::Error,
	io::prelude::*,
	env,
	fs,
};
use chrono::{
	DateTime,
	offset::Local,
	NaiveDateTime
};
use rand::{
	distributions::uniform::*,
	Rng,
};

const FILE_EXIT: &str = r"./Saida-ASN.txt";
const FILE_EXIT_EDIT: &str = r"./Saida-BODY.txt";

fn main() -> Result<(), Box<dyn Error>> {
	let file_reader = env::args().skip(1).next().unwrap();

	let read = fs::read(&file_reader)?;
	let mut p = AsnReader::new();

	if let Some(mut asn) = p.read_ber(&read) {
		let mut exit = fs::File::create(FILE_EXIT)?;
		let mut new = fs::File::create(file_reader)?;
		let mut edit = fs::File::create(FILE_EXIT_EDIT)?;

		exit.write(asn.to_string().as_bytes())?;

		// Option 1

		// if let ASN::Sequence(ref mut seq) = asn {
		// 	let body = seq.get_mut(3).unwrap().get_sequence_mut()?;
		// 	let content = body.get_mut(1).unwrap().get_sequence_mut()?.get_mut(0).unwrap().get_sequence_mut()?;
		// 	let start;
		// 	let end;
		// 	unsafe {
		// 		let s = content.get(2).unwrap().get_application()?.as_ptr();
		// 		let e = content.get(3).unwrap().get_application()?.as_ptr();
		// 		start = *(s as *const [u8; 4]);
		// 		end = *(e as *const [u8; 4]);
		// 	}
		// 	let now = Local::now();
		// 	let date_start = NaiveDateTime::from_timestamp(i32::from_be_bytes(start) as i64, 0);
		// 	let date_end = NaiveDateTime::from_timestamp(i32::from_be_bytes(end) as i64, 0);

		// 	content.replace(2, UTCTime::new(DateTime::from_utc(date_start, *now.offset())).to_asn());
		// 	content.replace(3, UTCTime::new(DateTime::from_utc(date_end, *now.offset())).to_asn());

		// 	let header = seq.get_mut(2).unwrap().get_sequence_mut()?;
		// 	let head = header.get_mut(0).unwrap().get_sequence_mut()?;
		// 	let len = head.len();

		// 	for i in (0..len).rev() {
		// 		let c = head.get_mut(i);

		// 		if let ASN::Sequence(ref mut seq_head) = c.unwrap() {
		// 			if seq_head.len() == 4 {
		// 				seq_head.remove(0);
		// 			} else {
		// 				head.remove(i);
		// 			}
		// 		} else {
		// 			head.remove(i);
		// 		}
		// 	}

		// 	header.insert(0, OctetString::new("Tracevia S.A.".as_bytes().to_vec()).to_asn());
		// 	header.insert(1, UTCTime::new(DateTime::from_utc(date_start, *now.offset())).to_asn());
		// 	header.insert(2, AsnOID::new(vec![ 1, 3, 6, 1, 4, 1, 1206, 4, 2, 9, 4, 1 ]).to_asn());
		// 	header.insert(3, OctetString::new("2".as_bytes().to_vec()).to_asn());
		// 	header.insert(4, AsnOID::new(vec![ 1, 3, 6, 1, 4, 1, 1206, 4, 2, 9, 4, 3 ]).to_asn());
		// 	header.insert(5, OctetString::new(vec![ 0x41, 0x30, 0x30, 0x32, 0x36, 0x34, 0x30, 0x38, 0x34, 0x31 ]).to_asn());
		// }

		// new.write(&asn.encode())?;
		// edit.write(&asn.to_string().as_bytes())?;

		// Option 2

		let sequence: Sequence = asn.try_into()?;
		let mut dcm: DCM = sequence.try_into()?;

		dcm.set_site_id("511");
		dcm.set_site_description("1N,2N,1S,2S");

		let class1 = VehCriteria::VehicleClass(88).absolute();
		let class2 = VehCriteria::VehicleClass(89).absolute();

		let mut rng = rand::thread_rng();
		let length = dcm.header.get_content_position(&AsnOID::new(VehCriteria::new(8).absolute().to_oid()));
		let num_axles = dcm.header.get_content_position(&AsnOID::new(VehCriteria::new(5).absolute().to_oid()));
		let axle_spacing = dcm.header.get_content_position(&AsnOID::new(VehCriteria::new(11).absolute().to_oid()));
		let axle_weight = dcm.header.get_content_position(&AsnOID::new(VehCriteria::new(23).absolute().to_oid()));

		for d in dcm.iter_mut() {
			let devices1 = d.filter_mut(|x| x == &class1);
			alter_value(length, num_axles, axle_spacing, axle_weight, devices1, 8..14, 10..12, 12..15, 10);
			
			let devices2 = d.filter_mut(|x| x == &class2);
			alter_value(length, num_axles, axle_spacing, axle_weight, devices2, 8..14, 120..140, 80..110, 40);
		}

		// for d in dcm.iter_mut() {
		// 	d.filter_remove(|x| x == &Device::DCM(Dcm::VehCriteria(VehCriteria::VehicleClass(88))));
		// }

		for d in dcm.iter() {

			edit.write(format!("{:#?}\r\n\r\n", d).as_bytes())?;
		}
		
		let mut back: Sequence = dcm.into();

		new.write(&back.encode())?;

		
		// std::thread::sleep(std::time::Duration::from_secs(10));
	}

	Ok(())
}

fn alter_value<R: SampleRange<u16> + Clone>(length: Option<usize>, num_axles: Option<usize>, axle_spacing: Option<usize>, axle_weight: Option<usize>, devices: Vec<&mut Vec<Device>>, len_range: R, w1_range: R, w2_range: R, diff: u16) {
	let mut rng = rand::thread_rng();

	for device in devices {
		let mut len_r = rng.gen_range(len_range.clone()) as isize * 5;
		let mut w1 = rng.gen_range(w1_range.clone()) * 10;
		let mut w2 = rng.gen_range(w2_range.clone()) * 10;
		let len = if let Some(n) = length {
			*device[n - 1].get_values().get(0).unwrap_or(&len_r)
		} else { len_r } as u16;

		if let Some(n) = num_axles {
			device[n - 1] = VehCriteria::AxleNumber(2).absolute();
		}
		if let Some(n) = axle_spacing {
			device[n - 1] = VehCriteria::AxleSpacing(vec![len - diff]).absolute();
		}
		if let Some(n) = axle_weight {
			device[n - 1] = VehCriteria::AxleWeight(vec![w1, w2]).absolute();
		}
	}
}