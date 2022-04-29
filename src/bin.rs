use super::*;
use std::error::Error;
use rand::{
	distributions::uniform::*,
	Rng,
};

pub fn init(file: &[u8], id: &str, desc: &str) -> Result<Vec<u8>, Box<dyn Error>> {
	let mut p = AsnReader::new();

	if let Some(asn) = p.read_ber(&file) {

		let sequence: Sequence = asn.try_into()?;
		let mut dcm: DCM = sequence.try_into()?;

		dcm.set_version(123);
		dcm.set_site_id(id);
		dcm.set_site_description(desc);

		let class1 = VehCriteria::VehicleClass(88).absolute();
		let class2 = VehCriteria::VehicleClass(89).absolute();

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

		let back: Sequence = dcm.into();

		Ok(back.encode())
	} else {
        Ok(file.to_vec())
    }
}

fn alter_value<R: SampleRange<u16> + Clone>(length: Option<usize>, num_axles: Option<usize>, axle_spacing: Option<usize>, axle_weight: Option<usize>, devices: Vec<&mut Vec<Device>>, len_range: R, w1_range: R, w2_range: R, diff: u16) {
	let mut rng = rand::thread_rng();

	for device in devices {
		let len_r = rng.gen_range(len_range.clone()) as isize * 5;
		let w1 = rng.gen_range(w1_range.clone()) * 10;
		let w2 = rng.gen_range(w2_range.clone()) * 10;
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