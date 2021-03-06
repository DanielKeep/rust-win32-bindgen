/*!
Contains the `Value` type that represents a conditional compilation expression's value.
*/
use WinVersion;
use features::{Features, Partitions, WinVersions};

/**
Represents a conditional compilation expression's value.

Under normal circumstances, this would be synonymous with "an integer".  These are not normal circumstances.

This type also defines operations for combining values.  *Many* combinations are undefined.  The general attitude is that *because* there are no complete, well-defined semantics for this, operations are implemented as-needed with whatever semantics make sense for that narrow case.

In other words, don't speculatively implement operations, because it may not be clear *what* an operation even *means*.
*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    /**
    A boolean true/false value.  These are *not* use to represent logical values.  They exist as a kludge for when an expression needs to be "disableable" in the abstract.

    The motivating example was something akin to:

    ```c
    #if !(defined(_CONTRACT_GEN) && ... && defined(_X86_))
    ...
    ```

    The expression was falsy, but because unknown symbols are simply ignored, it was being evaluated as a feature set that excluded the x86 architecture.

    The solution was to represent the result of `defined(_CONTRACT_GEN)` as `false`, and ensure that taking the intersection of `false` and any feature set resulted in `false`.

    Also note that converting *either* true or false to a feature set yields the default "says nothing" feature set.
    */
    Bool(bool),
    /**
    An integer value.

    We are only interested in these when used as version numbers.
    */
    Int(u32),
    /**
    A feature set.
    */
    Feat(Features),
    /**
    An API partition.

    This is distinct from a full feature set, mostly to ensure `WINAPI_FAMILY_PARTITION(_)` expressions have vaguely reasonable semantics (it ends up acting as a type-check).
    */
    Part(Partitions),
    /**
    A "full" (*i.e.* `NTDDI_VERSION`) version value.  Usually generated by `NTDDI_*` identifiers.
    */
    FullVersionValue(u32),
    /**
    A "short" (*i.e.* `_WIN32_WINNT`) version value.  Usually generated by `WIN32_WINNT_*` identifiers, but can *also* come from right-shift expressions (*i.e.* `FullVersionValue(_) >> Int(16)`).
    */
    ShortVersionValue(u32),
    /**
    The abstract "full version".  Only useful when compared to a version value, or passed through an `OSVER(_)` or `SPVER(_)` expression.
    */
    FullVersion,
    /**
    The abstract "short version".  Only useful when compared to a version value.
    */
    ShortVersion,
    /**
    The abstract "full OS version".  Differs from `FullVersion` in that it covers all service packs.
    */
    OsVersion,
    /**
    The abstract "full service pack version".  Differs from `FullVersion` in that it doesn't cover any specific major OS version.
    */
    SpVersion,
    /**
    An ignorable value.
    */
    Ignore,
}

impl Value {
    /**
    Tries to convert this value into a feature set.
    */
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

    /**
    Computes the complement of this value.
    */
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

    /**
    Computes the intersection of two values.
    */
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

    /**
    Computes the union of two values.
    */
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

    /**
    Computes the feature set given by an equality comparison of two values.
    */
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

    /**
    Computes the feature set given by an inequality comparison of two values.
    */
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

    /**
    Computes the feature set given by a less-than comparison of two values.
    */
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

    /**
    Computes the feature set given by a less-than or equal comparison of two values.
    */
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

    /**
    Computes the feature set given by a greater-than comparison of two values.
    */
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

    /**
    Computes the feature set given by a greater-than or equal comparison of two values.
    */
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

    /**
    Computes the result of a right-shift of a value.
    */
    pub fn rs(self, rhs: Value) -> Result<Value, String> {
        use self::Value::*;
        match (self, rhs) {
            (FullVersionValue(v), Int(16)) => Ok(ShortVersionValue(v >> 16)),
            (Ignore, Ignore) => Ok(Ignore),
            (Ignore, Int(_)) | (Int(_), Ignore) => Ok(Ignore),
            (l, r) => Err(format!("invalid op: {:?} >> {:?}", l, r))
        }
    }

    /**
    Computes the result of taking the "os version" of this value.
    */
    pub fn os_ver(self) -> Result<Value, String> {
        use self::Value::*;
        match self {
            FullVersion => Ok(OsVersion),
            FullVersionValue(v) => Ok(FullVersionValue(v & 0xFFFF_0000)),
            n => Err(format!("invalid op: OSVER({:?})", n))
        }
    }

    /**
    Computes the result of taking the "service pack version" of this value.
    */
    pub fn sp_ver(self) -> Result<Value, String> {
        use self::Value::*;
        match self {
            FullVersion => Ok(SpVersion),
            n => Err(format!("invalid op: SPVER({:?})", n))
        }
    }

    /**
    Attempts to ignore this value.
    */
    pub fn ignore(self) -> Result<Value, String> {
        use self::Value::*;
        match self {
            Ignore => Ok(Ignore),
            n => Err(format!("cannot ignore {:?}", n))
        }
    }
}
