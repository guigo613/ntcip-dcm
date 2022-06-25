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
		// let mut rng = rand::thread_rng();

		dcm.set_version(123);
		dcm.set_site_id(id);
		dcm.set_site_description(desc);

		let class1 = VehCriteria::VehicleClass(88).absolute();
		let class2 = VehCriteria::VehicleClass(89).absolute();
		let class3 = VehCriteria::VehicleClass(119).absolute();
		let class4 = VehCriteria::VehicleClass(121).absolute();


		let length = dcm.header.get_content_position(&AsnOID::new(VehCriteria::new(8).absolute().to_oid()));
		// let speed = dcm.header.get_content_position(&AsnOID::new(VehCriteria::new(7).absolute().to_oid()));
		let num_axles = dcm.header.get_content_position(&AsnOID::new(VehCriteria::new(5).absolute().to_oid()));
		let axle_spacing = dcm.header.get_content_position(&AsnOID::new(VehCriteria::new(11).absolute().to_oid()));
		let axle_weight = dcm.header.get_content_position(&AsnOID::new(VehCriteria::new(23).absolute().to_oid()));
		let gross_weight = dcm.header.get_content_position(&AsnOID::new(VehCriteria::new(25).absolute().to_oid()));

		for d in dcm.iter_mut() {
			let devices1 = d.filter_mut(|x| *x == class1);
			alter_value(length, num_axles, axle_spacing, axle_weight, gross_weight, devices1, 28..46, 10..12, 1.0, 10);

			d.filter_remove(|x| *x == class4);
			// let mut devices4 = d.filter_mut(|x| *x == class4);
			// for device in &mut devices4 {
			// 	if let Some(idx) = speed {
			// 		device[idx - 1] = VehCriteria::VehicleSpeed(rng.gen_range(1500..2800)).absolute();
			// 	}
			// }
			// alter_value(length, num_axles, axle_spacing, axle_weight, gross_weight, devices4, 28..46, 10..12, 12..15, 10);


			let devices2 = d.filter_mut(|x| *x == class2);
			alter_value(length, num_axles, axle_spacing, axle_weight, gross_weight, devices2, 46..60, 20..80, -1.0, 180);

			let devices3 = d.filter_mut(|x| *x == class3);
			alter_value(length, num_axles, axle_spacing, axle_weight, gross_weight, devices3, 260..320, 70..180, -1.0, 210);
		}

		let back: Sequence = dcm.into();

		Ok(back.encode())
	} else {
        Ok(file.to_vec())
    }
}

fn alter_value<R: SampleRange<u16> + Clone>(length: Option<usize>, num_axles: Option<usize>, axle_spacing: Option<usize>, axle_weight: Option<usize>, gross_weight: Option<usize>, devices: Vec<&mut Vec<Device>>, len_range: R, w_range: R, w_multi: f64, diff: u16) {
	let mut rng = rand::thread_rng();

	let dev = devices.into_iter().filter(|x| x.iter().fold(false, |y, z| {
		return match z {
			Device::DCM(Dcm::VehCriteria(VehCriteria::AxleSpacing(v))) 
				if v.iter().sum::<u16>() == 0
				=> true,
			Device::DCM(Dcm::VehCriteria(VehCriteria::AxleWeight(v))) 
				if v.iter().sum::<u16>() == 0 || v.len() < 2
				=> true,
			Device::DCM(Dcm::VehCriteria(VehCriteria::GrossVehicleWeight(0))) => true,
			_ => false
		} || y
	}));

	for device in dev {
		let len_r = rng.gen_range(len_range.clone()) * 5;
		let w1 = rng.gen_range(w_range.clone());
		let w2 = 100.0 + rng.gen_range(5.0..=20.0) * w_multi;
		let w = vec![
			w1 * 10,
			(w1 as f64 * (w2 / 100.0)) as u16 * 10
		];

		let len = if let Some(n) = length {
			match device[n - 1].get_values().get(0) {
				Some(l) if *l > diff as isize => *l as u16,
				_ => {
					device[n - 1] = VehCriteria::VehicleLength(len_r).absolute();
					len_r
				}
			}
		} else { len_r };

		if let Some(n) = num_axles {
			device[n - 1] = VehCriteria::AxleNumber(2).absolute();
		}
		if let Some(n) = axle_spacing {
			if device[n - 1].get_values().iter().sum::<isize>() == 0 {
				device[n - 1] = VehCriteria::AxleSpacing(vec![len - diff]).absolute();
			}
		}
		if let Some(n) = gross_weight {
			device[n - 1] = VehCriteria::GrossVehicleWeight(w.iter().sum::<u16>() as u32).absolute();
		}
		if let Some(n) = axle_weight {
			device[n - 1] = VehCriteria::AxleWeight(w).absolute();
		}
	}
}