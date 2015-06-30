pub mod cc;

use std::fmt;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};
use itertools::Itertools;
use clang;

pub use generated::winver::WinVersion;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Features {
    pub parts: Option<Partitions>,
    pub winver: Option<WinVersions>,
    pub arch: Option<Architectures>,
}

impl Features {
    pub fn check_valid(self) -> Result<Features, &'static str> {
        if let Some(ref parts) = self.parts {
            if !parts.is_any() { return Err("cannot have empty partition set"); }
        }
        if let Some(ref winver) = self.winver {
            if !winver.is_any() { return Err("cannot have empty winver set"); }
        }
        if let Some(ref arch) = self.arch {
            if !arch.is_any() { return Err("cannot have empty architecture set"); }
        }
        Ok(self)
    }

    pub fn complement(self) -> Features {
        Features {
            parts: { debug!(".. parts..."); self.parts.map(|p| !p) },
            winver: { debug!(".. winver..."); self.winver.map(WinVersions::complement) },
            arch: { debug!(".. arch..."); self.arch.map(|a| !a) },
        }
    }

    pub fn and(self, other: Features) -> Features {
        Features {
            parts: match (self.parts, other.parts) {
                (None, None) => None,
                (Some(a), None) | (None, Some(a)) => Some(a),
                (Some(a), Some(b)) => Some(a & b)
            },
            winver: match (self.winver, other.winver) {
                (None, None) => None,
                (Some(a), None) | (None, Some(a)) => Some(a),
                (Some(a), Some(b)) => Some(a.intersect(b))
            },
            arch: match (self.arch, other.arch) {
                (None, None) => None,
                (Some(a), None) | (None, Some(a)) => Some(a),
                (Some(a), Some(b)) => Some(a & b)
            },
        }
    }

    pub fn or(self, other: Features) -> Features {
        Features {
            parts: match (self.parts, other.parts) {
                (None, None) => None,
                (Some(a), None) | (None, Some(a)) => Some(a),
                (Some(a), Some(b)) => Some(a | b)
            },
            winver: match (self.winver, other.winver) {
                (None, None) => None,
                (Some(a), None) | (None, Some(a)) => Some(a),
                (Some(a), Some(b)) => Some(a.union(b))
            },
            arch: match (self.arch, other.arch) {
                (None, None) => None,
                (Some(a), None) | (None, Some(a)) => Some(a),
                (Some(a), Some(b)) => Some(a | b)
            },
        }
    }
}

impl Default for Features {
    fn default() -> Self {
        Features {
            parts: None,
            winver: None,
            arch: None,
        }
    }
}

impl fmt::Display for Features {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if let Some(ref parts) = self.parts { try!(write!(fmt, "{}", parts)); }
        if let Some(ref winver) = self.winver { try!(write!(fmt, "{}", winver)); }
        if let Some(ref arch) = self.arch { try!(write!(fmt, "{}", arch)); }
        Ok(())
    }
}

impl From<Partitions> for Features {
    fn from(v: Partitions) -> Features {
        Features {
            parts: Some(v),
            ..Features::default()
        }
    }
}

impl From<WinVersions> for Features {
    fn from(v: WinVersions) -> Features {
        Features {
            winver: Some(v),
            ..Features::default()
        }
    }
}

impl From<Architectures> for Features {
    fn from(v: Architectures) -> Features {
        Features {
            arch: Some(v),
            ..Features::default()
        }
    }
}

bitflags! {
    flags Partitions: u8 {
        const All           = 0b111,
        const Desktop       = 0b001,
        const App           = 0b010,
        const DesktopApp    = 0b011,
        const Phone         = 0b100,
    }
}

impl Partitions {
    pub fn from_define(s: &str) -> Option<Partitions> {
        match s {
            "WINAPI_PARTITION_DESKTOP"  => Some(Partitions::Desktop),
            "WINAPI_PARTITION_APP"      => Some(Partitions::App),
            "WINAPI_PARTITION_PC_APP"   => Some(Partitions::DesktopApp),
            "WINAPI_PARTITION_PHONE"    => Some(Partitions::Phone),
            _ => None
        }
    }
}

pub const CFG_FEATURE_PARTITION_DESKTOP: &'static str = "winapi_desktop";
pub const CFG_FEATURE_PARTITION_APP: &'static str = "winapi_app";
pub const CFG_FEATURE_PARTITION_PHONE: &'static str = "winapi_phone";

impl fmt::Display for Partitions {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        assert!(self.is_any(), "can't have no partitions enabled");
        if !self.is_all() {
            if (*self & Partitions::Desktop).is_any() {
                try!(write!(fmt, "#[cfg(feature={:?})] ", CFG_FEATURE_PARTITION_DESKTOP));
            }
            if (*self & Partitions::App).is_any() {
                try!(write!(fmt, "#[cfg(feature={:?})] ", CFG_FEATURE_PARTITION_APP));
            }
            if (*self & Partitions::Phone).is_any() {
                try!(write!(fmt, "#[cfg(feature={:?})] ", CFG_FEATURE_PARTITION_PHONE));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WinVersions(Vec<Range<u32>>);

impl WinVersions {
    fn is_any(&self) -> bool {
        &*self.0 != [0..0]
    }

    pub fn complement(self) -> WinVersions {
        debug!("WinVersions::complement({:?})", self);
        if &*self.0 == &[0..!0] {
            return WinVersions(vec![0..0]);
        }

        if &*self.0 == &[0..0] {
            return WinVersions(vec![0..!0]);
        }

        let pts: Vec<_> = self.0.into_iter().flat_map(|ab| vec![ab.start, ab.end].into_iter()).collect();
        debug!(".. pts: {:?}", pts);

        let pts: Vec<_> = match (pts[0] == 0, pts[pts.len()-1] == !0) {
            (true, true) => pts[1..pts.len()-1].into(),
            (true, false) => pts[1..].iter().cloned().chain(Some(!0)).collect(),
            (false, true) => Some(0).into_iter().chain(pts[..pts.len()-1].iter().cloned()).collect(),
            (false, false) => Some(0).into_iter()
                .chain(pts.iter().cloned())
                .chain(Some(!0))
                .collect()
        };
        debug!(".. pts: {:?}", pts);

        let ranges = pts.iter().cloned()
            .batching(|mut it| it.next().and_then(
                |a| it.next().map(
                    |b| a..b)))
            .collect();
        debug!(".. ranges: {:?}", ranges);

        WinVersions(ranges)
    }

    pub fn intersect(self, other: WinVersions) -> WinVersions {
        let mut abs = &self.0[..];
        let mut ijs = &other.0[..];

        let mut acc = vec![];
        while abs.len() > 0 && ijs.len() > 0 {
            let Range { start: a, end: b } = abs[0].clone();
            let Range { start: i, end: j } = ijs[0].clone();
            assert!(a <= b);
            assert!(i <= j);

            if b < i {
                /*
                Drop ab.

                    a .. b
                            i .. j
                */
                abs = &abs[1..];
            } else if a <= i && i <= b && b <= j {
                /*
                Emit i..b, drop ab.

                    a .. b
                       i .. j
                */
                acc.push(i..b);
                abs = &abs[1..];
            } else if i <= a && a <= j && j <= b {
                /*
                Emit a..j, drop ij.

                       a .. b
                    i .. j
                */
                acc.push(a..j);
                ijs = &ijs[1..];
            } else if j < b {
                /*
                Drop ij.

                            a .. b
                    i .. j
                */
                ijs = &ijs[1..];
            } else {
                panic!("unreachable: 0x{:08x}..0x{:08x}, 0x{:08x}..0x{:08x}", a, b, i, j);
            }
        }

        WinVersions(acc)
    }

    pub fn union(mut self, mut other: WinVersions) -> WinVersions {
        fn inner(mut acc: Vec<Range<u32>>, abs: &mut [Range<u32>], ijs: &mut [Range<u32>]) -> Vec<Range<u32>> {
            if abs.len() == 0 || ijs.len() == 0 {
                acc.extend(abs.iter().cloned());
                acc.extend(ijs.iter().cloned());
                acc
            } else {
                let Range { start: a, end: b } = abs[0].clone();
                let Range { start: i, end: j } = ijs[0].clone();
                assert!(a <= b);
                assert!(i <= j);

                if a == b {
                    /*
                    Drop ab.
                    */
                    inner(acc, &mut abs[1..], ijs)
                } else if i == j {
                    /*
                    Drop ij.
                    */
                    inner(acc, abs, &mut ijs[1..])
                }
                else if b < i {
                    /*
                    Emit a..b, drop ab.

                        a .. b
                                i .. j
                    */
                    acc.push(a..b);
                    inner(acc, &mut abs[1..], ijs)
                } else if a <= i && i <= b && b <= j {
                    /*
                    ij = a..j, drop ab.

                        a .. b
                           i .. j
                    */
                    ijs[0] = a..j;
                    inner(acc, &mut abs[1..], ijs)
                } else if i <= a && a <= j && j <= b {
                    /*
                    ab = i..b, drop ij.

                           a .. b
                        i .. j
                    */
                    abs[0] = i..b;
                    inner(acc, abs, &mut ijs[1..])
                } else if j < a {
                    /*
                    Emit i..j, drop ij.

                                a .. b
                        i .. j
                    */
                    acc.push(i..j);
                    inner(acc, abs, &mut ijs[1..])
                } else {
                    panic!("unreachable: 0x{:08x}..0x{:08x}, 0x{:08x}..0x{:08x}", a, b, i, j);
                }
            }
        }

        let mut ranges = inner(vec![], &mut self.0, &mut other.0);

        if ranges.len() == 0 {
            ranges.push(0..0);
        }

        WinVersions(ranges)
    }
}

pub const CFG_FEATURE_VERSION_PREFIX: &'static str = "winapi_ver_";

impl fmt::Display for WinVersions {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        assert!(&*self.0 != &[0..0], "can't have no versions enabled");
        const PREFIX: &'static str = CFG_FEATURE_VERSION_PREFIX;
        const END: u32 = !0;

        // Skip if there are no restrictions.
        if &*self.0 == &[0..!0] { return Ok(()); }

        // Here we go!
        try!(write!(fmt, "#[cfg(any("));
        for (i, Range { start: a, end: b }) in self.0.iter().cloned().enumerate() {
            try!(write!(fmt, "{}", if i == 0 { "" } else { ", " }));
            match (a, b) {
                (0, b) => try!(write!(fmt, "not(feature=\"{}{:08x}\")", PREFIX, b)),
                (a, END) => try!(write!(fmt, "feature=\"{}{:08x}\"", PREFIX, a)),
                (a, b) => try!(write!(fmt,
                    "all(feature=\"{0}{1:08x}\", not(feature=\"{0}{2:08x}\"))",
                    PREFIX, a, b))
            }
        }
        try!(write!(fmt, "))] "));
        Ok(())
    }
}

impl From<WinVersion> for WinVersions {
    fn from(v: WinVersion) -> WinVersions {
        match v.next_version() {
            Some(n) => WinVersions(vec![(v as u32)..(n as u32)]),
            None => WinVersions(vec![(v as u32)..!0])
        }
    }
}

impl From<Range<Option<WinVersion>>> for WinVersions {
    fn from(v: Range<Option<WinVersion>>) -> WinVersions {
        match (v.start, v.end) {
            (None, None) => (..).into(),
            (Some(a), None) => (a..).into(),
            (None, Some(b)) => (..b).into(),
            (Some(a), Some(b)) => (a..b).into()
        }
    }
}

impl From<Range<WinVersion>> for WinVersions {
    fn from(v: Range<WinVersion>) -> WinVersions {
        assert!(v.start < v.end);
        WinVersions(vec![(v.start as u32)..(v.end as u32)])
    }
}

impl From<RangeFrom<WinVersion>> for WinVersions {
    fn from(v: RangeFrom<WinVersion>) -> WinVersions {
        WinVersions(vec![(v.start as u32)..!0])
    }
}

impl From<RangeFull> for WinVersions {
    fn from(_: RangeFull) -> WinVersions {
        WinVersions(vec![0..!0])
    }
}

impl From<RangeTo<WinVersion>> for WinVersions {
    fn from(v: RangeTo<WinVersion>) -> WinVersions {
        WinVersions(vec![0..(v.end as u32)])
    }
}

impl From<(RangeTo<WinVersion>, RangeFrom<WinVersion>)> for WinVersions {
    fn from((i, j): (RangeTo<WinVersion>, RangeFrom<WinVersion>)) -> WinVersions {
        assert!(i.end < j.start);
        WinVersions(vec![0..(i.end as u32), (j.start as u32)..!0])
    }
}

bitflags! {
    flags Architectures: u8 {
        const None      = 0b000,
        const All       = 0b111,
        const X86_32    = 0b001,
        const X86_64    = 0b010,
        const X86_All   = 0b011,
        const Arm       = 0b100,

        const Bits64    = 0b010,
    }
}

impl Architectures {
    pub fn from_define(s: &str) -> Option<Architectures> {
        match s {
            "_X86_"
            | "__X86__"
            | "__i386__"
            | "_M_IX86"
            => Some(Architectures::X86_32),

            "_AMD64_"
            | "__x86_64"
            | "__x86_64__"
            | "_M_AMD64"
            | "_M_X64"
            => Some(Architectures::X86_64),

            "__arm__"
            | "_ARM_"
            | "_M_ARM"
            => Some(Architectures::Arm),

            "WIN64" | "_WIN64" | "__WIN64" | "__WIN64__"
            => Some(Architectures::Bits64),

            "_M_MRX000"
            | "_M_ALPHA"
            | "_M_PPC"
            | "__ia64__"
            | "_IA64_"
            | "_M_IA64"
            => Some(Architectures::None),

            _ => None
        }
    }
}

impl fmt::Display for Architectures {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        assert!(self.is_any(), "can't have no architectures enabled");
        if !self.is_all() {
            try!(write!(fmt, "#[cfg(any("));
            let mut sep = "";
            if (*self & Architectures::X86_32).is_any() {
                try!(write!(fmt, "{}target_arch=\"x86\"", sep));
                sep = ", ";
            }
            if (*self & Architectures::X86_64).is_any() {
                try!(write!(fmt, "{}target_arch=\"x86_64\"", sep));
                sep = ", ";
            }
            if (*self & Architectures::Arm).is_any() {
                try!(write!(fmt, "{}target_arch=\"arm\"", sep));
            }
            try!(write!(fmt, "))] "));
        }
        Ok(())
    }
}

pub fn has_important_defines(toks: &[String]) -> bool {
    toks.iter().any(|tok| is_important_define(&**tok))
}

pub fn is_important_define(tok: &str) -> bool {
    match tok {
        // Architecture defines
        "_X86_"
        | "__X86__"
        | "__i386__"
        | "_M_IX86"
        | "_AMD64_"
        | "__x86_64"
        | "__x86_64__"
        | "_M_AMD64"
        | "_M_X64"
        | "__arm__"
        | "_ARM_"
        | "_M_ARM"

        // Obsolete or uninteresting architectures
        | "_M_MRX000"
        | "_M_ALPHA"
        | "_M_PPC"
        | "__ia64__"
        | "_IA64_"
        | "_M_IA64"

        // Important winapi defines
        | "WIN64" | "_WIN64" | "__WIN64" | "__WIN64__"
        | "WINVER"
        | "_WIN32_WINNT"
        | "NTDDI_VERSION"
        | "WINAPI_FAMILY_PARTITION"
        | "WINAPI_FAMILY_ONE_PARTITION"
        | "WINAPI_PARTITION_DESKTOP"
        | "WINAPI_PARTITION_APP"
        | "WINAPI_PARTITION_PC_APP"
        | "WINAPI_PARTITION_PHONE_APP"

        => true,
        _ => false
    }
}

pub fn define_feature(name: &str) -> Features {
    debug!("define_feature({:?})", name);
    match cc::Node::eval_defined(name) {
        Ok(value) => {
            match value.to_features() {
                Ok(f) => f,
                Err(err) => panic!("error defining feature {:?}: {}", name, err)
            }
        },
        Err(err) => panic!("error defining feature {:?}: {}", name, err)
    }
}

pub fn define_feature_expr(toks: &[String], loc: &clang::SourceLocation) -> Features {
    debug!("define_feature_expr({:?}, {})", toks, loc.display_short());
    if !has_important_defines(toks) {
        debug!(".. nothing important");
        return Features::default();
    }

    let node = match cc::parse_conditional_expr(toks) {
        Ok(Some((node, tail))) => {
            if tail.len() != 0 {
                panic!("could not fully parse cc expr at {} {:?}; leftover: {:?}", loc.display_short(), toks, tail)
            }
            node
        },
        Ok(None) => panic!("could not parse cc expr at {} {:?}", loc.display_short(), toks),
        Err(err) => panic!("could not parse cc expr at {} {:?}: {}", loc.display_short(), toks, err)
    };

    debug!(".. node: {:?}", node);

    match node.clone().eval().and_then(|v| v.to_features()) {
        Ok(f) => { debug!(".. f: {:?}", f); f },
        Err(err) => panic!("error evaluating expr at {} {:?}: {}", loc.display_short(), node, err)
    }
}
