use WinVersion;
use features::{Features, Partitions, WinVersions};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Bool(bool),
    Int(u32),
    Feat(Features),
    Part(Partitions),
    FullVersionValue(u32),
    ShortVersionValue(u32),
    FullVersion,
    ShortVersion,
    OsVersion,
    SpVersion,
    Ignore,
}

impl Value {
    pub fn to_features(self) -> Result<Features, String> {
        use self::Value::*;
        match self {
            Bool(_) => Ok(Features::default()),
            Feat(f) => Ok(f),
            Part(p) => Ok(p.into()),
            FullVersionValue(v) => {
                let wv = WinVersion::from_u32_round_up(v).expect("valid full version");
                Ok(WinVersions::from(wv).into())
            },
            ShortVersionValue(v) => {
                let wv = WinVersion::from_u32_round_up(v << 16).expect("valid full version");
                Ok(WinVersions::from(wv).into())
            },
            FullVersion
            | ShortVersion
            | Ignore => Ok(Features::default()),
            n => Err(format!("cannot convert to Features: {:?}", n))
        }
    }

    pub fn complement(self) -> Result<Value, String> {
        use self::Value::*;
        match self {
            Ignore => Ok(Ignore),
            Bool(b) => Ok(Bool(!b)),
            Feat(f) => Ok(Feat(f.complement())),
            Part(p) => Ok(Part(!p)),
            n => Err(format!("invalid op: ! {:?}", n))
        }
    }

    pub fn and(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Bool(b)) | (Bool(b), Ignore) => Ok(Bool(b)),
            (Ignore, Feat(f)) | (Feat(f), Ignore) => Ok(Feat(f)),
            (Bool(true), Feat(f)) | (Feat(f), Bool(true)) => Ok(Feat(f)),
            (Bool(false), Feat(_)) | (Feat(_), Bool(false)) => Ok(Ignore),
            (Feat(l), Feat(r)) => Ok(Feat(l.and(r))),
            (Part(l), Part(r)) => Ok(Part(l & r)),
            (l, r) => Err(format!("invalid op: {:?} && {:?}", l, r))
        }
    }

    pub fn or(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Bool(b)) | (Bool(b), Ignore) => Ok(Bool(b)),
            (Ignore, Feat(f)) | (Feat(f), Ignore) => Ok(Feat(f)),
            (Feat(l), Feat(r)) => Ok(Feat(l.or(r))),
            (Part(l), Part(r)) => Ok(Part(l | r)),
            (l, r) => Err(format!("invalid op: {:?} || {:?}", l, r))
        }
    }

    pub fn eq(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (FullVersion, FullVersionValue(i)) => {
                let start = WinVersion::from_u32_round_up(i).expect("valid version for fs == fvv");
                let end = WinVersion::from_u32_round_up((i & 0xFFFF_0000) + 1);
                let wv = match end {
                    Some(end) => WinVersions::from(start..end),
                    None => WinVersions::from(start..)
                };
                Ok(Feat(wv.into()))
            },
            (OsVersion, FullVersionValue(i)) => {
                let start = WinVersion::from_u32_round_up(i).expect("valid version for os == fvv");
                let end = WinVersion::from_u32_round_up((i & 0xFFFF_0000) + 0x1_0000);
                let wv = match end {
                    Some(end) => WinVersions::from(start..end),
                    None => WinVersions::from(start..)
                };
                Ok(Feat(wv.into()))
            },
            (ShortVersion, Int(v)) | (Int(v), ShortVersion) => {
                let start = WinVersion::from_u32_round_up(v << 16).expect("valid version for sv == int");
                let end = WinVersion::from_u32_round_up(((v << 16) + 0x1_0000));
                let wv = WinVersions::from(Some(start)..end);
                Ok(Feat(wv.into()))
            }
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Int(_)) | (Int(_), Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} == {:?}", l, r))
        }
    }

    pub fn ne(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (OsVersion, FullVersionValue(i)) => {
                let start = WinVersion::from_u32_round_up(i).expect("valid full os version for !=");
                let end = WinVersion::from_u32_round_up((i & 0xFFFF_0000) + 0x1_0000);
                let wv = match end {
                    Some(end) => WinVersions::from((..start, end..)),
                    None => WinVersions::from(..start)
                };
                Ok(Feat(wv.into()))
            },
            (ShortVersion, ShortVersionValue(i)) => {
                let start = WinVersion::from_u32_round_up(i << 16).expect("valid short version for !=");
                let end = start.next_version();
                let wv = match end {
                    Some(end) => WinVersions::from((..start, end..)),
                    None => WinVersions::from(..start)
                };
                Ok(Feat(wv.into()))
            },
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Int(_)) | (Int(_), Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} != {:?}", l, r))
        }
    }

    pub fn lt(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (FullVersion, FullVersionValue(i)) | (FullVersion, Int(i)) => {
                let end = WinVersion::from_u32_round_up(i).expect("valid full version for <");
                Ok(Feat(WinVersions::from(..end).into()))
            },
            (ShortVersion, ShortVersionValue(i)) | (ShortVersion, Int(i)) => {
                let end = WinVersion::from_u32_round_up(i << 16).expect("valid short version for <");
                Ok(Feat(WinVersions::from(..end).into()))
            },
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Int(_)) | (Int(_), Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} < {:?}", l, r))
        }
    }

    pub fn le(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (FullVersion, FullVersionValue(i)) | (FullVersion, Int(i)) => {
                let end = WinVersion::from_u32_round_up(i + 1).expect("valid full version for <=");
                Ok(Feat(WinVersions::from(..end).into()))
            },
            (ShortVersion, ShortVersionValue(i)) | (ShortVersion, Int(i)) => {
                let end = WinVersion::from_u32_round_up((i << 16) + 1).expect("valid short version for <=");
                Ok(Feat(WinVersions::from(..end).into()))
            },
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Int(_)) | (Int(_), Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} <= {:?}", l, r))
        }
    }

    pub fn gt(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (FullVersion, FullVersionValue(i)) | (FullVersion, Int(i)) => {
                let start = WinVersion::from_u32_round_up(i + 1).expect("valid full version for >");
                Ok(Feat(WinVersions::from(start..).into()))
            },
            (ShortVersion, ShortVersionValue(i)) | (ShortVersion, Int(i)) => {
                let start = WinVersion::from_u32_round_up((i << 16) + 1).expect("valid full version for >");
                Ok(Feat(WinVersions::from(start..).into()))
            },
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Int(_)) | (Int(_), Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} > {:?}", l, r))
        }
    }

    pub fn ge(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (FullVersion, FullVersionValue(i)) | (FullVersion, Int(i)) => {
                let start = WinVersion::from_u32_round_up(i).expect("valid full version for >=");
                Ok(Feat(WinVersions::from(start..).into()))
            },
            (ShortVersion, ShortVersionValue(i)) | (ShortVersion, Int(i)) => {
                let start = WinVersion::from_u32_round_up(i << 16).expect("valid full version for >=");
                Ok(Feat(WinVersions::from(start..).into()))
            },
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Int(_)) | (Int(_), Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} >= {:?}", l, r))
        }
    }

    pub fn rs(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (FullVersionValue(v), Int(16)) => Ok(ShortVersionValue(v >> 16)),
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Int(_)) | (Int(_), Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} >> {:?}", l, r))
        }
    }

    pub fn os_ver(self) -> Result<Value, String> {
        use self::Value::*;
        match self {
            FullVersion => Ok(OsVersion),
            FullVersionValue(v) => Ok(FullVersionValue(v & 0xFFFF_0000)),
            n => Err(format!("invalid op: OSVER({:?})", n))
        }
    }

    pub fn sp_ver(self) -> Result<Value, String> {
        use self::Value::*;
        match self {
            FullVersion => Ok(SpVersion),
            n => Err(format!("invalid op: SPVER({:?})", n))
        }
    }

    pub fn ignore(self) -> Result<Value, String> {
        use self::Value::*;
        match self {
            Ignore => Ok(Ignore),
            n => Err(format!("cannot ignore {:?}", n))
        }
    }
}
