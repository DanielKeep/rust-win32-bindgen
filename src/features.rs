use std::ops::{Range, RangeFrom, RangeFull, RangeTo};
use clang;

#[derive(Clone, Eq, PartialEq)]
pub struct Features {
    pub parts: Partitions,
    pub winver: WinVersion,
    pub arch: Architectures,
}

impl Default for Features {
    fn default() -> Self {
        Features {
            parts: Partitions::All,
            winver: WinVersion::All,
            arch: Architectures::All,
        }
    }
}

impl<Ver> From<(Partitions, Ver, Architectures)> for Features
where Ver: Into<WinVersion> {
    fn from(v: (Partitions, Ver, Architectures)) -> Features {
        Features {
            parts: v.0,
            winver: v.1.into(),
            arch: v.2,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WinVersion {
    All,
    After(u32),
    Between(u32, u32),
    Until(u32),
}

impl From<()> for WinVersion {
    fn from(_: ()) -> WinVersion {
        WinVersion::All
    }
}

impl From<Range<i32>> for WinVersion {
    fn from(v: Range<i32>) -> WinVersion {
        WinVersion::Between(v.start as u32, v.end as u32)
    }
}

impl From<RangeFrom<i32>> for WinVersion {
    fn from(v: RangeFrom<i32>) -> WinVersion {
        WinVersion::After(v.start as u32)
    }
}

impl From<RangeFull> for WinVersion {
    fn from(_: RangeFull) -> WinVersion {
        WinVersion::All
    }
}

impl From<RangeTo<i32>> for WinVersion {
    fn from(v: RangeTo<i32>) -> WinVersion {
        WinVersion::Until(v.end as u32)
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

pub fn has_important_defines(toks: &[String]) -> bool {
    toks.iter().any(|tok| match &**tok {
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
    })
}

pub fn define_feature(name: &str) -> Option<Features> {
    match name {
        "__X86__"
        | "__i386__"
        | "_M_IX86"
        => Some(feat!(All, .., X86_32)),
        "_AMD64_"
        | "__x86_64"
        | "__x86_64__"
        | "_M_AMD64"
        | "_M_X64"
        => Some(feat!(All, .., X86_64)),
        _ => None
    }
}

pub fn define_feature_complement(name: &str) -> Option<Features> {
    match name {
        "__X86__"
        | "__i386__"
        | "_M_IX86"
        => Some(feat!(All, .., X86_64)),
        "_AMD64_"
        | "__x86_64"
        | "__x86_64__"
        | "_M_AMD64"
        | "_M_X64"
        => Some(feat!(All, .., X86_32)),
        _ => None
    }
}

pub enum PrimaryBranch { Yes, No }

pub fn define_feature_expr(_toks: &[String], _pb: PrimaryBranch, _loc: &clang::SourceLocation) -> Option<Features> {
    unimplemented!();
}
