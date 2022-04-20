use super::{
    DataStructureEntry,
    OctetString,
    OID,
    Dcm,
    Device
};

#[repr(u8)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VehCriteria { // 9.1
    Unknown,
    VCTMaxNumRows(i32),
    VCTCurNumRows(i32),
    VehCriteriaTable,
    AxleNumber(u8),
    NumAxles(u8),
    VehicleClass(u8),
    VehicleSpeed(u16),
    VehicleLength(u16),
    FrontOverhang(u16),
    RearOverhang(u16),
    AxleSpacing(Vec<u16>),
    VehWheelbase(u16),
    VehClearance(u8),
    VehicleHeight(u16),
    AxleWidth(Vec<u16>),
    VehicleWidth(u16),
    AxleTireCount(Vec<u8>),
    AxleTireTrack(Vec<u16>),
    VehicleGap(u32), // 3
    VehicleHeadway(u32), // 3
    LeftWheelWeigh(Vec<u16>),
    RightWheelWeight(Vec<u16>),
    AxleWeight(Vec<u16>),
    SensorWeight(Vec<u16>),
    GrossVehicleWeight(u32), // 3
    VehicleSeqNum(u16),
    VehicleStatusFlag(u32),
    AxleWeightViolationCode(Vec<u32>), // 3
    VehicleAcceleration(i16),
    NumVehicleIDs(u8),
    VehicleID,
    VehicleTimeTag1000(u16),

    NotHere(u8)
}

impl VehCriteria {
    pub fn new(oid: isize) -> Self {
        match oid {
            1 => Self::VCTMaxNumRows(Default::default()),
            2 => Self::VCTCurNumRows(Default::default()),
            3 => Self::VehCriteriaTable,
            4 => Self::AxleNumber(Default::default()),
            5 => Self::NumAxles(Default::default()),
            6 => Self::VehicleClass(Default::default()),
            7 => Self::VehicleSpeed(Default::default()),
            8 => Self::VehicleLength(Default::default()),
            9 => Self::FrontOverhang(Default::default()),
            10 => Self::RearOverhang(Default::default()),
            11 => Self::AxleSpacing(Default::default()),
            12 => Self::VehWheelbase(Default::default()),
            13 => Self::VehClearance(Default::default()),
            14 => Self::VehicleHeight(Default::default()),
            15 => Self::AxleWidth(Default::default()),
            16 => Self::VehicleWidth(Default::default()),
            17 => Self::AxleTireCount(Default::default()),
            18 => Self::AxleTireTrack(Default::default()),
            19 => Self::VehicleGap(Default::default()),
            20 => Self::VehicleHeadway(Default::default()),
            21 => Self::LeftWheelWeigh(Default::default()),
            22 => Self::RightWheelWeight(Default::default()),
            23 => Self::AxleWeight(Default::default()),
            24 => Self::SensorWeight(Default::default()),
            25 => Self::GrossVehicleWeight(Default::default()),
            26 => Self::VehicleSeqNum(Default::default()),
            27 => Self::VehicleStatusFlag(Default::default()),
            28 => Self::AxleWeightViolationCode(Default::default()),
            29 => Self::VehicleAcceleration(Default::default()),
            30 => Self::NumVehicleIDs(Default::default()),
            31 => Self::VehicleID,
            32 => Self::VehicleTimeTag1000(Default::default()),
            9999 => Self::NotHere(Default::default()), // temp
            _ => Self::Unknown
        }
    }

    pub fn get_value(&mut self, values: &mut DataStructureEntry) {
        match self {
            Self::VCTMaxNumRows(ref mut val) => *val = i32::from_be_bytes([values.remove(0), values.remove(0), values.remove(0), values.remove(0)]),
            Self::VCTCurNumRows(ref mut val) => *val = i32::from_be_bytes([values.remove(0), values.remove(0), values.remove(0), values.remove(0)]),
            Self::VehCriteriaTable => (),
            Self::AxleNumber(ref mut val) => *val = values.remove(0),
            Self::NumAxles(ref mut val) => *val = values.remove(0),
            Self::VehicleClass(ref mut val) => *val = values.remove(0),
            Self::VehicleSpeed(ref mut val) => *val = u16::from_be_bytes([values.remove(0), values.remove(0)]),
            Self::VehicleLength(ref mut val) => *val = u16::from_be_bytes([values.remove(0), values.remove(0)]),
            Self::FrontOverhang(ref mut val) => *val = u16::from_be_bytes([values.remove(0), values.remove(0)]),
            Self::RearOverhang(ref mut val) => *val = u16::from_be_bytes([values.remove(0), values.remove(0)]),
            Self::AxleSpacing(ref mut val) => {
                for _ in 0..values.remove(0) {
                    val.push(u16::from_be_bytes([values.remove(0), values.remove(0)]));
                }
            },
            Self::VehWheelbase(ref mut val) => *val = u16::from_be_bytes([values.remove(0), values.remove(0)]),
            Self::VehClearance(ref mut val) => *val = values.remove(0),
            Self::VehicleHeight(ref mut val) => *val = u16::from_be_bytes([values.remove(0), values.remove(0)]),
            Self::AxleWidth(ref mut val) => {
                for _ in 0..values.remove(0) {
                    val.push(u16::from_be_bytes([values.remove(0), values.remove(0)]));
                }
            },
            Self::VehicleWidth(ref mut val) => *val = u16::from_be_bytes([values.remove(0), values.remove(0)]),
            Self::AxleTireCount(ref mut val) => {
                for _ in 0..values.remove(0) {
                    val.push(values.remove(0));
                }
            },
            Self::AxleTireTrack(ref mut val) => {
                for _ in 0..values.remove(0) {
                    val.push(u16::from_be_bytes([values.remove(0), values.remove(0)]));
                }
            },
            Self::VehicleGap(ref mut val) => *val = u32::from_be_bytes([0, values.remove(0), values.remove(0), values.remove(0)]),
            Self::VehicleHeadway(ref mut val) => *val = u32::from_be_bytes([0, values.remove(0), values.remove(0), values.remove(0)]),
            Self::LeftWheelWeigh(ref mut val) => {
                for _ in 0..values.remove(0) {
                    val.push(u16::from_be_bytes([values.remove(0), values.remove(0)]));
                }
            },
            Self::RightWheelWeight(ref mut val) => {
                for _ in 0..values.remove(0) {
                    val.push(u16::from_be_bytes([values.remove(0), values.remove(0)]));
                }
            },
            Self::AxleWeight(ref mut val) => {
                for _ in 0..values.remove(0)  {
                    val.push(u16::from_be_bytes([values.remove(0), values.remove(0)]));
                }
            },
            Self::SensorWeight(ref mut val) => {
                for _ in 0..values.remove(0)  {
                    val.push(u16::from_be_bytes([values.remove(0), values.remove(0)]));
                }
            },
            Self::GrossVehicleWeight(ref mut val) => *val = u32::from_be_bytes([0, values.remove(0), values.remove(0), values.remove(0)]),
            Self::VehicleSeqNum(ref mut val) => *val = u16::from_be_bytes([values.remove(0), values.remove(0)]),
            Self::VehicleStatusFlag(ref mut val) => *val = u32::from_be_bytes([values.remove(0), values.remove(0), values.remove(0), values.remove(0)]),
            Self::AxleWeightViolationCode(ref mut val) => {
                for _ in 0..values.remove(0)  {
                    val.push(u32::from_be_bytes([0, values.remove(0), values.remove(0), values.remove(0)]));
                }
            },
            Self::VehicleAcceleration(ref mut val) => *val = i16::from_be_bytes([values.remove(0), values.remove(0)]),
            Self::NumVehicleIDs(ref mut val) => *val = values.remove(0),
            Self::VehicleID => (),
            Self::VehicleTimeTag1000(ref mut val) => *val = u16::from_be_bytes([values.remove(0), values.remove(0)]),
            Self::NotHere(ref mut val) => *val = values.remove(0),
            _ => ()
        }
    }
}

impl OID for VehCriteria {
    fn to_oid(&self) -> Vec<isize> {
        unsafe {
            let oid = *(self as *const _ as *const u8) as isize;
            if oid != 33 {
                vec![0, 1, oid]
            } else {
                vec![0, 4, 9, 1, 1]
            }
        }
    }

    fn absolute(self) -> Device {
        Dcm::VehCriteria(self).absolute()
    }

    fn get_name(&self) -> &'static str {
        match self {
            Self::VCTMaxNumRows(_) => "VCTMaxNumRows",
            Self::VCTCurNumRows(_) => "VCTCurNumRows",
            Self::VehCriteriaTable => "VehCriteriaTable",
            Self::AxleNumber(_) => "AxleNumber",
            Self::NumAxles(_) => "NumAxles",
            Self::VehicleClass(_) => "VehicleClass",
            Self::VehicleSpeed(_) => "VehicleSpeed",
            Self::VehicleLength(_) => "VehicleLength",
            Self::FrontOverhang(_) => "FrontOverhang",
            Self::RearOverhang(_) => "RearOverhang",
            Self::AxleSpacing(_) => "AxleSpacing",
            Self::VehWheelbase(_) => "VehWheelbase",
            Self::VehClearance(_) => "VehClearance",
            Self::VehicleHeight(_) => "VehicleHeight",
            Self::AxleWidth(_) => "AxleWidth",
            Self::VehicleWidth(_) => "VehicleWidth",
            Self::AxleTireCount(_) => "AxleTireCount",
            Self::AxleTireTrack(_) => "AxleTireTrack",
            Self::VehicleGap(_) => "VehicleGap",
            Self::VehicleHeadway(_) => "VehicleHeadway",
            Self::LeftWheelWeigh(_) => "LeftWheelWeigh",
            Self::RightWheelWeight(_) => "RightWheelWeight",
            Self::AxleWeight(_) => "AxleWeight",
            Self::SensorWeight(_) => "SensorWeight",
            Self::GrossVehicleWeight(_) => "GrossVehicleWeight",
            Self::VehicleSeqNum(_) => "VehicleSeqNum",
            Self::VehicleStatusFlag(_) => "VehicleStatusFlag",
            Self::AxleWeightViolationCode(_) => "AxleWeightViolationCode",
            Self::VehicleAcceleration(_) => "VehicleAcceleration",
            Self::NumVehicleIDs(_) => "NumVehicleIDs",
            Self::VehicleID => "VehicleID",
            Self::VehicleTimeTag1000(_) => "VehicleTimeTag1000",
            Self::NotHere(_) => "Lane", // temp
            _ => "VehCriteria"
        }
    }

    fn get_values(&self) -> Vec<isize> {
        match self {
            Self::VCTMaxNumRows(v) => vec![*v as isize],
            Self::VCTCurNumRows(v) => vec![*v as isize],
            Self::VehCriteriaTable => vec![],
            Self::AxleNumber(v) => vec![*v as isize],
            Self::NumAxles(v) => vec![*v as isize],
            Self::VehicleClass(v) => vec![*v as isize],
            Self::VehicleSpeed(v) => vec![*v as isize],
            Self::VehicleLength(v) => vec![*v as isize],
            Self::FrontOverhang(v) => vec![*v as isize],
            Self::RearOverhang(v) => vec![*v as isize],
            Self::AxleSpacing(v) => v.iter().map(|x| *x as isize).collect(),
            Self::VehWheelbase(v) => vec![*v as isize],
            Self::VehClearance(v) => vec![*v as isize],
            Self::VehicleHeight(v) => vec![*v as isize],
            Self::AxleWidth(v) => v.iter().map(|x| *x as isize).collect(),
            Self::VehicleWidth(v) => vec![*v as isize],
            Self::AxleTireCount(v) => v.iter().map(|x| *x as isize).collect(),
            Self::AxleTireTrack(v) => v.iter().map(|x| *x as isize).collect(),
            Self::VehicleGap(v) => vec![*v as isize],
            Self::VehicleHeadway(v) => vec![*v as isize],
            Self::LeftWheelWeigh(v) => v.iter().map(|x| *x as isize).collect(),
            Self::RightWheelWeight(v) => v.iter().map(|x| *x as isize).collect(),
            Self::AxleWeight(v) => v.iter().map(|x| *x as isize).collect(),
            Self::SensorWeight(v) => v.iter().map(|x| *x as isize).collect(),
            Self::GrossVehicleWeight(v) => vec![*v as isize],
            Self::VehicleSeqNum(v) => vec![*v as isize],
            Self::VehicleStatusFlag(v) => vec![*v as isize],
            Self::AxleWeightViolationCode(v) => v.iter().map(|x| *x as isize).collect(),
            Self::VehicleAcceleration(v) => vec![*v as isize],
            Self::NumVehicleIDs(v) => vec![*v as isize],
            Self::VehicleID => vec![],
            Self::VehicleTimeTag1000(v) => vec![*v as isize],
            Self::NotHere(v) => vec![*v as isize], // temp
            _ => vec![]
        }
    }

    
    
    fn len_value(&self) -> i32 {
        match self {
            Self::VCTMaxNumRows(_) => 4,
            Self::VCTCurNumRows(_) => 4,
            Self::VehCriteriaTable => -1,
            Self::AxleNumber(_) => 1,
            Self::NumAxles(_) => 1,
            Self::VehicleClass(_) => 1,
            Self::VehicleSpeed(_) => 2,
            Self::VehicleLength(_) => 2,
            Self::FrontOverhang(_) => 2,
            Self::RearOverhang(_) => 2,
            Self::AxleSpacing(_) => 2,
            Self::VehWheelbase(_) => 2,
            Self::VehClearance(_) => 1,
            Self::VehicleHeight(_) => 2,
            Self::AxleWidth(_) => 2,
            Self::VehicleWidth(_) => 2,
            Self::AxleTireCount(_) => 2,
            Self::AxleTireTrack(_) => 1,
            Self::VehicleGap(_) => 3,
            Self::VehicleHeadway(_) => 3,
            Self::LeftWheelWeigh(_) => 2,
            Self::RightWheelWeight(_) => 2,
            Self::AxleWeight(_) => 2,
            Self::SensorWeight(_) => 2,
            Self::GrossVehicleWeight(_) => 3,
            Self::VehicleSeqNum(_) => 2,
            Self::VehicleStatusFlag(_) => 4,
            Self::AxleWeightViolationCode(_) => 3,
            Self::VehicleAcceleration(_) => 2,
            Self::NumVehicleIDs(_) => 2,
            Self::VehicleID => 1,
            Self::VehicleTimeTag1000(_) => 2,
            Self::NotHere(_) => 1, // temp
            _ => -1
        }
    }
}

impl Into<OctetString> for VehCriteria {
    fn into(self) -> OctetString {
        match self {
            Self::VCTMaxNumRows(v) => OctetString::new(v.to_be_bytes().to_vec()),
            Self::VCTCurNumRows(v) => OctetString::new(v.to_be_bytes().to_vec()),
            Self::VehCriteriaTable => OctetString::default(),
            Self::AxleNumber(v) => OctetString::new(vec![v]),
            Self::NumAxles(v) => OctetString::new(vec![v]),
            Self::VehicleClass(v) => OctetString::new(vec![v]),
            Self::VehicleSpeed(v) => OctetString::new(v.to_be_bytes().to_vec()),
            Self::VehicleLength(v) => OctetString::new(v.to_be_bytes().to_vec()),
            Self::FrontOverhang(v) => OctetString::new(v.to_be_bytes().to_vec()),
            Self::RearOverhang(v) => OctetString::new(v.to_be_bytes().to_vec()),
            Self::AxleSpacing(v) => OctetString::new(v.iter().fold(vec![v.len() as u8], |mut x, y| {
                x.extend(y.to_be_bytes());
                x
            })),
            Self::VehWheelbase(v) => OctetString::new(v.to_be_bytes().to_vec()),
            Self::VehClearance(v) => OctetString::new(vec![v]),
            Self::VehicleHeight(v) => OctetString::new(v.to_be_bytes().to_vec()),
            Self::AxleWidth(v) => OctetString::new(v.iter().fold(vec![v.len() as u8], |mut x, y| {
                x.extend(y.to_be_bytes());
                x
            })),
            Self::VehicleWidth(v) => OctetString::new(v.to_be_bytes().to_vec()),
            Self::AxleTireCount(mut v) => OctetString::new({
                v.insert(0, v.len() as u8);
                v
            }),
            Self::AxleTireTrack(v) => OctetString::new(v.iter().fold(vec![v.len() as u8], |mut x, y| {
                x.extend(y.to_be_bytes());
                x
            })),
            Self::VehicleGap(v) => OctetString::new(v.to_be_bytes()[1..4].to_vec()),
            Self::VehicleHeadway(v) => OctetString::new(v.to_be_bytes()[1..4].to_vec()),
            Self::LeftWheelWeigh(v) => OctetString::new(v.iter().fold(vec![v.len() as u8], |mut x, y| {
                x.extend(y.to_be_bytes());
                x
            })),
            Self::RightWheelWeight(v) => OctetString::new(v.iter().fold(vec![v.len() as u8], |mut x, y| {
                x.extend(y.to_be_bytes());
                x
            })),
            Self::AxleWeight(v) => OctetString::new(v.iter().fold(vec![v.len() as u8], |mut x, y| {
                x.extend(y.to_be_bytes());
                x
            })),
            Self::SensorWeight(v) => OctetString::new(v.iter().fold(vec![v.len() as u8], |mut x, y| {
                x.extend(y.to_be_bytes());
                x
            })),
            Self::GrossVehicleWeight(v) => OctetString::new(v.to_be_bytes()[1..4].to_vec()),
            Self::VehicleSeqNum(v) => OctetString::new(v.to_be_bytes().to_vec()),
            Self::VehicleStatusFlag(v) => OctetString::new(v.to_be_bytes().to_vec()),
            Self::AxleWeightViolationCode(v) => OctetString::new(v.iter().fold(vec![v.len() as u8], |mut x, y| {
                x.extend(&y.to_be_bytes()[1..4]);
                x
            })),
            Self::VehicleAcceleration(v) => OctetString::new(v.to_be_bytes().to_vec()),
            Self::NumVehicleIDs(v) => OctetString::new(vec![v]),
            Self::VehicleID => OctetString::default(),
            Self::VehicleTimeTag1000(v) => OctetString::new(v.to_be_bytes().to_vec()),
            Self::NotHere(v) => OctetString::new(vec![v]), // temp
            _ => OctetString::default()
        }
    }
}