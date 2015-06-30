use std::fmt;

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
