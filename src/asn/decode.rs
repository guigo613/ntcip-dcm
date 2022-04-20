use super::{
    chrono::DateTime,
    asn::*
};

pub struct AsnReader {
	reader_idx: usize
}

impl AsnReader {
	pub fn new() -> Self {
		Self {
			reader_idx: 0
		}
	}

	pub fn read_ber(&mut self, file: &[u8]) -> Option<ASN> {
		if file.len() > 0 {
			let b = file[self.reader_idx];
			self.reader_idx += 1;
			let _class_asn = b >> 6;
			let type_asn = (b & 0x1F) as u8;
			
			match type_asn.into() {
				Type::Sequence => {
					let len = self.get_length(file);
					Some(ASN::Sequence(self.get_sequence(file, len)))
				},
				_ => unimplemented!()
			}
		} else {
			None
		}
	}

	fn get_length(&mut self, file: &[u8]) -> usize {
		let val = file[self.reader_idx];
		self.reader_idx += 1;

		if val >> 7 == 1 {
			let len = val & 0x7F;
			let mut b = [0; 8];

			for i in 1..=len {
				b[(len - i) as usize] = file[self.reader_idx];
				self.reader_idx += 1;
			}

			usize::from_le_bytes(b)
		} else {
			val as usize
		}
	}

	fn get_sequence(&mut self, file: &[u8], len: usize) -> Sequence {
		let mut format;
		let mut seq = Sequence::new();
		let init_read = self.reader_idx;
		let mut read = self.reader_idx - init_read;
		let mut length;
		while read < len {
			let b = file[self.reader_idx];
			self.reader_idx += 1;
			let class_asn = b >> 6;
			length = self.get_length(file);
			match class_asn {
				0 => format = ((b & 0x1F) as u8).into(),
				1 => format = Type::Application,
				2 | 3 => unimplemented!(),
				_ => unreachable!(),
			}

			match format {
				Type::Sequence => {
					let new_seq = self.get_sequence(file, length);
					seq.push(new_seq.to_asn());
				},
				Type::Integer => {
					let integer = self.get_integer(file, length);
					seq.push(integer.to_asn());
				},
				Type::OID => {
					let oid = self.get_oid(file, length);
					seq.push(oid.to_asn());
				},
				Type::OctetString => {
					let octet = self.get_octet_string(file, length);
					seq.push(octet.to_asn());
				},
				Type::UTCTime => {
					let time = self.get_utc_time(file, length);
					seq.push(time.to_asn());
				},
				Type::Application => {
					let app = self.get_application(file, length);
					seq.push(app.to_asn());
				},
				Type::Unknown => {
					let uk = self.get_unknown(file, length);
					seq.push(uk.to_asn());
				}
			}

			read = self.reader_idx - init_read;
		}

		seq
	}

	fn get_integer(&mut self, file: &[u8], len: usize) -> Integer {
		let mut b = [0; 8];

		for i in 1..=len {
			b[len - i] = file[self.reader_idx];
			self.reader_idx += 1;
		}

		let value = isize::from_le_bytes(b);

		Integer::new(value)
	}

	fn get_oid(&mut self, file: &[u8], len: usize) -> AsnOID {
		let mut bytes = Vec::new();
		let init_read = self.reader_idx;
		let mut read = self.reader_idx - init_read;
		let first = file[self.reader_idx];
		self.reader_idx += 1;

		bytes.push((first / 40) as isize);
		bytes.push((first % 40) as isize);

		while read < len {
			let b = file[self.reader_idx];
			self.reader_idx += 1;

			if b >> 7 == 1 {
				let mut greater = [0; 2];
				greater[1] = (file[self.reader_idx] & 0x7F) | (b << 7);
				greater[0] = (b & 0x7F) >> 1;
				self.reader_idx += 1;

				bytes.push(u16::from_be_bytes(greater) as isize);
			} else {
				bytes.push(b as isize);
			}

			read = self.reader_idx - init_read;
		}

		AsnOID::new(bytes)
	}

	fn get_octet_string(&mut self, file: &[u8], len: usize) -> OctetString {
		let mut b = Vec::new();

		for _ in 0..len {
			b.push(file[self.reader_idx]);
			self.reader_idx += 1;
		}

		OctetString::new(b)
	}

	fn get_utc_time(&mut self, file: &[u8], len: usize) -> UTCTime {
		let mut b = Vec::with_capacity(len);
		let check = b.contains(&0x2d);

		for _ in 0..len {
			b.push(file[self.reader_idx]);
			self.reader_idx += 1;
		}

		
		let value = String::from_utf8_lossy(&b).replace("Z", "+0000");
		let idx = if check {
			value.find("-")
		} else {
			value.find("+")
		}.unwrap_or(0);
		let format = if idx > 10 {
			"%y%m%d%H%M%S%z"
		} else { "%y%m%d%H%M%z" };
		let date = DateTime::parse_from_str(&value, format).unwrap();

		UTCTime::new(date)
	}

	fn get_application(&mut self, file: &[u8], len: usize) -> Application {
		let mut b = Vec::new();

		for _ in 0..len {
			b.push(file[self.reader_idx]);
			self.reader_idx += 1;
		}

		Application::new(b)
	}

	fn get_unknown(&mut self, file: &[u8], len: usize) -> Unknown {
		let mut b = Vec::new();

		for _ in 0..len {
			b.push(file[self.reader_idx]);
			self.reader_idx += 1;
		}

		Unknown::new(b)
	}
}
