use std::fmt;

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
