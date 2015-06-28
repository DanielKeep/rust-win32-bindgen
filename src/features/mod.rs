pub mod cc;

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
        const All     = 0b11,
        const Desktop = 0b01,
        const App     = 0b10,
    }
}

impl Partitions {
    pub fn from_define(s: &str) -> Option<Partitions> {
        match s {
            "WINAPI_PARTITION_DESKTOP" => Some(Partitions::Desktop),
            "WINAPI_PARTITION_APP" => Some(Partitions::App),
            _ => None
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WinVersions(Vec<Range<u32>>);

impl WinVersions {
    pub fn complement(self) -> WinVersions {
        if &*self.0 == &[0..!0] {
            return WinVersions(vec![0..0]);
        }

        if &*self.0 == &[0..0] {
            return WinVersions(vec![0..!0]);
        }

        let pts: Vec<_> = self.0.into_iter().flat_map(|ab| vec![ab.start, ab.end].into_iter()).collect();

        let pts: Vec<_> = match (pts[0] == 0, pts[pts.len()-1] == !0) {
            (true, true) => pts[1..pts.len()-1].into(),
            (true, false) => pts[1..].iter().cloned().chain(Some(!0)).collect(),
            (false, true) => Some(0).into_iter().chain(pts[..pts.len()-1].iter().cloned()).collect(),
            (false, false) => Some(0).into_iter()
                .chain(pts.iter().cloned())
                .chain(Some(!0))
                .collect()
        };

        let ranges = pts.iter().cloned()
            .batching(|mut it| it.next().and_then(
                |a| it.next().map(
                    |b| a..b)))
            .collect();

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

            if a < i && b < i {
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
            } else if i <= a && a <= j && j <= b {
                /*
                Drop ij.

                            a .. b
                    i .. j
                */
                ijs = &ijs[1..];
            } else {
                unreachable!()
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

                if a < i && b < i {
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
                } else if i <= a && a <= j && j <= b {
                    /*
                    Emit i..j, drop ij.

                                a .. b
                        i .. j
                    */
                    acc.push(i..j);
                    inner(acc, abs, &mut ijs[1..])
                } else {
                    unreachable!()
                }
            }
        }

        WinVersions(inner(vec![], &mut self.0, &mut other.0))
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
    }
}

impl Architectures {
    pub fn from_define(s: &str) -> Option<Architectures> {
        match s {
            "__X86__"
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

            _ => None
        }
    }
}

pub fn has_important_defines(toks: &[String]) -> bool {
    toks.iter().any(|tok| is_important_define(&**tok))
}

pub fn is_important_define(tok: &str) -> bool {
    match tok {
        // Architecture defines
        "__X86__"
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
        | "WINVER"
        | "_WIN32_WINNT"
        | "NTDDI_VERSION"
        | "WINAPI_FAMILY_PARTITION"
        | "WINAPI_FAMILY_ONE_PARTITION"

        => true,
        _ => false
    }
}

pub fn define_feature(name: &str) -> Option<Features> {
    match name {
        "__X86__"
        | "__i386__"
        | "_M_IX86"
        => Some(Architectures::X86_32.into()),
        "_AMD64_"
        | "__x86_64"
        | "__x86_64__"
        | "_M_AMD64"
        | "_M_X64"
        => Some(Architectures::X86_64.into()),
        _ => None
    }
}

pub fn define_feature_complement(name: &str) -> Option<Features> {
    match name {
        "__X86__"
        | "__i386__"
        | "_M_IX86"
        => Some(Architectures::X86_64.into()),
        "_AMD64_"
        | "__x86_64"
        | "__x86_64__"
        | "_M_AMD64"
        | "_M_X64"
        => Some(Architectures::X86_32.into()),
        _ => None
    }
}

pub enum PrimaryBranch { Yes, No }

pub fn define_feature_expr(_toks: &[String], _pb: PrimaryBranch, _loc: &clang::SourceLocation) -> Option<Features> {
    unimplemented!();
}
