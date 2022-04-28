use super::{
    io,
    fmt,
    mem,
	FromIterator,
    ops::{
        Deref,
        DerefMut
    },
    chrono::{
        DateTime,
        FixedOffset,
        Timelike
    }
};

pub trait StructASN {
    fn to_asn(self) -> ASN;
    fn encode(&self) -> Vec<u8>;
}

#[repr(u8)]
#[derive(Debug)]
pub enum Type {
	Sequence = 0x10,
	Integer = 0x02,
	OID = 0x06,
	OctetString = 0x04,
	Application = 0x41,
	UTCTime = 0x17,
	Unknown
}

impl From<u8> for Type {
	fn from(id: u8) -> Self {
		match id {
			0x10 => Type::Sequence,
			0x02 => Type::Integer,
			0x06 => Type::OID,
			0x04 => Type::OctetString,
			0x41 => Type::Application,
			0x17 => Type::UTCTime,
			_ => Type::Unknown
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ASN {
	Sequence(Sequence),
	Integer(Integer),
	OID(AsnOID),
	OctetString(OctetString),
	UTCTime(UTCTime),
	Application(Application),
	Unknown(Unknown),
}

impl ASN {
	pub fn get_sequence(&self) -> Result<&Sequence, io::Error> {
		if let Self::Sequence(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
	}

	pub fn get_sequence_mut(&mut self) -> Result<&mut Sequence, io::Error> {
		if let Self::Sequence(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
	}

	pub fn get_integer(&self) -> Result<&Integer, io::Error> {
		if let Self::Integer(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
	}

	pub fn get_integer_mut(&mut self) -> Result<&mut Integer, io::Error> {
		if let Self::Integer(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
	}

	pub fn get_oid(&self) -> Result<&AsnOID, io::Error> {
		if let Self::OID(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
	}

	pub fn get_oid_mut(&mut self) -> Result<&mut AsnOID, io::Error> {
		if let Self::OID(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
	}

	pub fn get_octet_string(&self) -> Result<&OctetString, io::Error> {
		if let Self::OctetString(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
	}

	pub fn get_octet_string_mut(&mut self) -> Result<&mut OctetString, io::Error> {
		if let Self::OctetString(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
	}

	pub fn get_utc_time(&self) -> Result<&UTCTime, io::Error> {
		if let Self::UTCTime(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
	}

	pub fn get_utc_time_mut(&mut self) -> Result<&mut UTCTime, io::Error> {
		if let Self::UTCTime(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
	}

	pub fn get_application(&self) -> Result<&Application, io::Error> {
		if let Self::Application(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
	}

	pub fn get_application_mut(&mut self) -> Result<&mut Application, io::Error> {
		if let Self::Application(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
	}

	pub fn get_unknown(&self) -> Result<&Unknown, io::Error> {
		if let Self::Unknown(seq) = self {
			Ok(seq)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
	}

	pub fn get_unknown_mut(&mut self) -> Result<&mut Unknown, io::Error> {
		if let Self::Unknown(seq) = self {
			Ok(seq)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
	}
}

impl TryInto<Sequence> for ASN {
    type Error = io::Error;

    fn try_into(self) -> Result<Sequence, Self::Error> {
        if let Self::Sequence(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
    }
}

impl TryInto<Integer> for ASN {
    type Error = io::Error;

    fn try_into(self) -> Result<Integer, Self::Error> {
        if let Self::Integer(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
    }
}

impl TryInto<AsnOID> for ASN {
    type Error = io::Error;

    fn try_into(self) -> Result<AsnOID, Self::Error> {
        if let Self::OID(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
    }
}

impl TryInto<OctetString> for ASN {
    type Error = io::Error;

    fn try_into(self) -> Result<OctetString, Self::Error> {
        if let Self::OctetString(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
    }
}

impl TryInto<Application> for ASN {
    type Error = io::Error;

    fn try_into(self) -> Result<Application, Self::Error> {
        if let Self::Application(val) = self {
			Ok(val)
		} else { Err(io::Error::from(io::ErrorKind::InvalidData)) }
    }
}

impl StructASN for ASN {
    fn to_asn(self) -> Self {
        self
    }
    
	fn encode(&self) -> Vec<u8> {
		match self {
			ASN::Sequence(a) => a.encode(),
			ASN::Integer(a) => a.encode(),
			ASN::OID(a) => a.encode(),
			ASN::OctetString(a) => a.encode(),
			ASN::UTCTime(a) => a.encode(),
			ASN::Application(a) => a.encode(),
			ASN::Unknown(a) => a.encode()
		}
	}
}

impl fmt::Display for ASN {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ASN::Sequence(a) => write!(f, "{}", a),
			ASN::Integer(a) => write!(f, "{}", a),
			ASN::OID(a) => write!(f, "{}", a),
			ASN::OctetString(a) => write!(f, "{}", a),
			ASN::UTCTime(a) => write!(f, "{}", a),
			ASN::Application(a) => write!(f, "{}", a),
			ASN::Unknown(a) => write!(f, "{}", a)
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sequence {
	content: Vec<ASN>,
}

impl Sequence {
	pub fn new() -> Self {
		Self {
			content: Vec::new(),
		}
	}

	pub fn replace(&mut self, idx: usize, val: ASN) -> ASN {
		self.content.push(val);
		self.content.swap_remove(idx)
	}
}

impl StructASN for Sequence {
	fn to_asn(self) -> ASN {
		ASN::Sequence(self)
	}

    fn encode(&self) -> Vec<u8> {
		let mut bytes = Vec::new();
		bytes.push(Type::Sequence as u8 | 0x20);
		for asn in &self.content {
			bytes.extend(asn.encode());
		}
		
		insert_length(&mut bytes);

		bytes
	}
}

impl<A: StructASN> FromIterator<A> for Sequence {
	fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
		let mut seq = Self::new();
		seq.extend(iter.into_iter().map(|x| x.to_asn()).collect::<Vec<ASN>>());
		seq
	}
}

impl IntoIterator for Sequence {
    type Item = ASN;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.content.into_iter()
    }
}

impl Deref for Sequence {
    type Target = Vec<ASN>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl DerefMut for Sequence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

impl fmt::Display for Sequence {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut val = String::from("Sequence: [\r\n");
		val += &self.content.iter().fold(String::new(), |a, b| {
			if a.is_empty() {
				b.to_string()
			} else {
				format!("{}, {}", a, b.to_string())
			}
		});
		val += "\r\n.]";

		write!(f, "{}", val.replace(", ", ",\r\n").replace("\r\n", "\r\n\t").replace("\t.]", "]"))
	}
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub struct Integer {
	content: isize
}

impl Integer {
	pub fn new(content: isize) -> Self {
		Self {
			content
		}
	}

	pub fn set(&mut self, val: isize) {
		self.content = val;
	}
}

impl Deref for Integer {
    type Target = isize;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl StructASN for Integer {
	fn to_asn(self) -> ASN {
		ASN::Integer(self)
	}

    fn encode(&self) -> Vec<u8> {
		let mut bytes = Vec::new();
		let b = self.content.to_be_bytes();
		
		let mut offset = 0;
		for o in 0..b.len() {
			if b[o] != 0 {
				break;
			} else {
				offset += 1;
			}
		}

		offset = match offset {
			0..=3 => 0,
			5 | 8 => offset - 1,
			4 | 6..=7 => offset,
			_ => unreachable!()
		};

		bytes.push(Type::Integer as u8);
		bytes.push((b.len() - offset) as u8);
		for o in offset..b.len() {
			bytes.push(b[o]);
		}

		bytes
	}
}

impl fmt::Display for Integer {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", format!("Integer: {}", self.content))
	}
}

impl Default for Integer {
    fn default() -> Self {
        Self {
            content: Default::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsnOID {
	content: Vec<isize>
}

impl AsnOID {
	pub fn new(content: Vec<isize>) -> Self {
		Self {
			content
		}
	}

	pub fn stringfy(&self) -> String {
		self.content.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(".")
	}
}

impl StructASN for AsnOID {
	fn to_asn(self) -> ASN {
		ASN::OID(self)
	}

    fn encode(&self) -> Vec<u8> {
		let mut bytes = Vec::new();
		bytes.push(Type::OID as u8);
		bytes.push((self.content[0] * 40 + self.content[1]) as u8);
		for i in 2..self.content.len()
		{
			let mut c = self.content[i];
			if c > 0x7F {
				let len = bytes.len();
				while c != 0 {
					let b = (c & 0xFF) | 0x80;
					c >>= 7;

					bytes.insert(len, b as u8);
				}
				let len = bytes.len();

				bytes[len - 1] &= 0x7F;
			} else {
				bytes.push(c as u8);
			}
		}

		insert_length(&mut bytes);

		bytes
	}
}

impl Into<Vec<isize>> for AsnOID {
    fn into(self) -> Vec<isize> {
        self.content
    }
}

impl Deref for AsnOID {
    type Target = Vec<isize>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl DerefMut for AsnOID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

impl fmt::Display for AsnOID {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", format!("OID: {}", self.content.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(".")))
	}
}

impl Default for AsnOID {
    fn default() -> Self {
        Self {
            content: Default::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OctetString {
	content: Vec<u8>
}

impl OctetString {
	pub fn new(content: Vec<u8>) -> Self {
		Self {
			content
		}
	}
}

impl StructASN for OctetString {
	fn to_asn(self) -> ASN {
		ASN::OctetString(self)
	}

	fn encode(&self) -> Vec<u8> {
		let mut bytes = Vec::new();
		bytes.push(Type::OctetString as u8);
		bytes.extend_from_slice(&self.content);
		insert_length(&mut bytes);

		bytes
	}
}

impl Into<Vec<u8>> for OctetString {
    fn into(self) -> Vec<u8> {
        self.content
    }
}

impl Deref for OctetString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl DerefMut for OctetString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

impl fmt::Display for OctetString {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let formated = String::from_utf8(self.content.to_vec()).unwrap_or(format!("{:02X?}", self.content).replace(", ", ""));
		write!(f, "{}", format!("OctetString: {}", formated))
	}
}

impl Default for OctetString {
    fn default() -> Self {
        Self {
            content: Default::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UTCTime {
	content: DateTime<FixedOffset>
}

impl UTCTime {
	pub fn new(content: DateTime<FixedOffset>) -> Self {
		Self {
			content
		}
	}
}

impl StructASN for UTCTime {
	fn to_asn(self) -> ASN {
		ASN::UTCTime(self)
	}

	fn encode(&self) -> Vec<u8> {
		let date_str = format!("%y%m%d%H%M{}%z", if self.content.second() == 0 { "" } else { "%S" });
		let date = self.content.format(&date_str);

		let mut bytes = Vec::new();
		bytes.push(Type::UTCTime as u8);
		bytes.extend_from_slice(date.to_string().replace("+0000", "Z").as_bytes());
		insert_length(&mut bytes);

		bytes
	}
}

impl fmt::Display for UTCTime {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", format!("UTCTime: {:?}", self.content))
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Application {
	content: Vec<u8>
}

impl Application {
	pub fn new(content: Vec<u8>) -> Self {
		Self {
			content
		}
	}

    pub fn as_ptr(&self) -> *const u8 {
        self.content.as_ptr()
    }
}

impl StructASN for Application {
	fn to_asn(self) -> ASN {
		ASN::Application(self)
	}

	fn encode(&self) -> Vec<u8> {
		let mut bytes = Vec::new();
		bytes.push(Type::Application as u8);
		bytes.extend(&self.content);
		insert_length(&mut bytes);

		bytes
	}
}

impl Deref for Application {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl DerefMut for Application {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

impl fmt::Display for Application {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", format!("Application: {:?}", self.content))
	}
}

impl Default for Application {
    fn default() -> Self {
        Self {
            content: Default::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unknown {
	content: Vec<u8>
}

impl Unknown {
	pub fn new(content: Vec<u8>) -> Self {
		Self {
			content
		}
	}
}

impl StructASN for Unknown {
	fn to_asn(self) -> ASN {
		ASN::Unknown(self)
	}

	fn encode(&self) -> Vec<u8> {
		let mut bytes = Vec::new();
		bytes.push(Type::Application as u8);
		bytes.extend(&self.content);
		insert_length(&mut bytes);

		bytes
	}
}

impl fmt::Display for Unknown {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", format!("Application: {:?}", self.content))
	}
}

fn insert_length(bytes: &mut Vec<u8>) {
	let length = bytes.len() - 1;

	if length > 0x7F {
		let len_bytes = length.to_be_bytes();
		let mut offset = 0;

		for l in len_bytes {
			if l != 0 {
				break;
			} else {
				offset += 1;
			}
		}

		offset = match offset {
			0..=3 => 0,
			5 => 4,
			4 | 6..=7 => offset,
			_ => unreachable!()
		};

		for i in (offset..8).rev() {
			bytes.insert(1, len_bytes[i]);
		}

		bytes.insert(1, ((8 - offset) | 0x80) as u8);
	}
	else {
		bytes.insert(1, length as u8);
	}
}