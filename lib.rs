#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(overflowing_literals)]
extern crate libc;

macro_rules! union_field {
    ($base:ident.{$get:ident, $get_mut:ident}: $t:ty) => {
        impl $base {
            pub fn $get(&self) -> &$t {
                unsafe {
                    ::std::mem::transmute(self)
                }
            }

            pub fn $get_mut(&mut self) -> &mut $t {
                unsafe {
                    ::std::mem::transmute(self)
                }
            }
        }
    };
}

pub use self::apiset::*;
pub use self::basetsd::*;
pub use self::bemapiset::*;
pub use self::errhandlingapi::*;
pub use self::fileapi::*;
pub use self::guiddef::*;
pub use self::heapapi::*;
pub use self::ime_cmodes::*;
pub use self::imm::*;
pub use self::kernelspecs::*;
pub use self::ktmtypes::*;
pub use self::libloaderapi::*;
pub use self::mcx::*;
pub use self::memoryapi::*;
pub use self::minwinbase::*;
pub use self::minwindef::*;
pub use self::namespaceapi::*;
pub use self::processthreadsapi::*;
pub use self::reason::*;
pub use self::stralign::*;
pub use self::synchapi::*;
pub use self::sysinfoapi::*;
pub use self::threadpoolapiset::*;
pub use self::timezoneapi::*;
pub use self::tvout::*;
pub use self::verrsrc::*;
pub use self::winapifamily::*;
pub use self::winbase::*;
pub use self::wincon::*;
pub use self::windef::*;
pub use self::winerror::*;
pub use self::wingdi::*;
pub use self::winnetwk::*;
pub use self::winnls::*;
pub use self::winnt::*;
pub use self::winreg::*;
pub use self::winsvc::*;
pub use self::winuser::*;
pub use self::wnnc::*;

#[path="headers/apiset.rs"] pub mod apiset;
#[path="headers/basetsd.rs"] pub mod basetsd;
#[path="headers/bemapiset.rs"] pub mod bemapiset;
#[path="headers/errhandlingapi.rs"] pub mod errhandlingapi;
#[path="headers/fileapi.rs"] pub mod fileapi;
#[path="headers/guiddef.rs"] pub mod guiddef;
#[path="headers/heapapi.rs"] pub mod heapapi;
#[path="headers/ime_cmodes.rs"] pub mod ime_cmodes;
#[path="headers/imm.rs"] pub mod imm;
#[path="headers/kernelspecs.rs"] pub mod kernelspecs;
#[path="headers/ktmtypes.rs"] pub mod ktmtypes;
#[path="headers/libloaderapi.rs"] pub mod libloaderapi;
#[path="headers/mcx.rs"] pub mod mcx;
#[path="headers/memoryapi.rs"] pub mod memoryapi;
#[path="headers/minwinbase.rs"] pub mod minwinbase;
#[path="headers/minwindef.rs"] pub mod minwindef;
#[path="headers/namespaceapi.rs"] pub mod namespaceapi;
#[path="headers/processthreadsapi.rs"] pub mod processthreadsapi;
#[path="headers/reason.rs"] pub mod reason;
#[path="headers/stralign.rs"] pub mod stralign;
#[path="headers/synchapi.rs"] pub mod synchapi;
#[path="headers/sysinfoapi.rs"] pub mod sysinfoapi;
#[path="headers/threadpoolapiset.rs"] pub mod threadpoolapiset;
#[path="headers/timezoneapi.rs"] pub mod timezoneapi;
#[path="headers/tvout.rs"] pub mod tvout;
#[path="headers/verrsrc.rs"] pub mod verrsrc;
#[path="headers/winapifamily.rs"] pub mod winapifamily;
#[path="headers/winbase.rs"] pub mod winbase;
#[path="headers/wincon.rs"] pub mod wincon;
#[path="headers/windef.rs"] pub mod windef;
#[path="headers/winerror.rs"] pub mod winerror;
#[path="headers/wingdi.rs"] pub mod wingdi;
#[path="headers/winnetwk.rs"] pub mod winnetwk;
#[path="headers/winnls.rs"] pub mod winnls;
#[path="headers/winnt.rs"] pub mod winnt;
#[path="headers/winreg.rs"] pub mod winreg;
#[path="headers/winsvc.rs"] pub mod winsvc;
#[path="headers/winuser.rs"] pub mod winuser;
#[path="headers/wnnc.rs"] pub mod wnnc;

#[doc(hidden)] pub mod _modules {
    pub use super::apiset;
    pub use super::basetsd;
    pub use super::bemapiset;
    pub use super::errhandlingapi;
    pub use super::fileapi;
    pub use super::guiddef;
    pub use super::heapapi;
    pub use super::ime_cmodes;
    pub use super::imm;
    pub use super::kernelspecs;
    pub use super::ktmtypes;
    pub use super::libloaderapi;
    pub use super::mcx;
    pub use super::memoryapi;
    pub use super::minwinbase;
    pub use super::minwindef;
    pub use super::namespaceapi;
    pub use super::processthreadsapi;
    pub use super::reason;
    pub use super::stralign;
    pub use super::synchapi;
    pub use super::sysinfoapi;
    pub use super::threadpoolapiset;
    pub use super::timezoneapi;
    pub use super::tvout;
    pub use super::verrsrc;
    pub use super::winapifamily;
    pub use super::winbase;
    pub use super::wincon;
    pub use super::windef;
    pub use super::winerror;
    pub use super::wingdi;
    pub use super::winnetwk;
    pub use super::winnls;
    pub use super::winnt;
    pub use super::winreg;
    pub use super::winsvc;
    pub use super::winuser;
    pub use super::wnnc;
}

#[path="libraries/adsiid.rs"] #[link(name="adsiid")] pub mod adsiid;
#[path="libraries/advapi32.rs"] #[link(name="advapi32")] pub mod advapi32;
#[path="libraries/bufferoverflow.rs"] #[link(name="bufferoverflow")] pub mod bufferoverflow;
#[path="libraries/dmoguids.rs"] #[link(name="dmoguids")] pub mod dmoguids;
#[path="libraries/ehstorguids.rs"] #[link(name="ehstorguids")] pub mod ehstorguids;
#[path="libraries/elfapi.rs"] #[link(name="elfapi")] pub mod elfapi;
#[path="libraries/fileextd.rs"] #[link(name="fileextd")] pub mod fileextd;
#[path="libraries/gdi32.rs"] #[link(name="gdi32")] pub mod gdi32;
#[path="libraries/imm32.rs"] #[link(name="imm32")] pub mod imm32;
#[path="libraries/kernel32.rs"] #[link(name="kernel32")] pub mod kernel32;
#[path="libraries/mfuuid.rs"] #[link(name="mfuuid")] pub mod mfuuid;
#[path="libraries/mincore.rs"] #[link(name="mincore")] pub mod mincore;
#[path="libraries/mincore_downlevel.rs"] #[link(name="mincore_downlevel")] pub mod mincore_downlevel;
#[path="libraries/mpr.rs"] #[link(name="mpr")] pub mod mpr;
#[path="libraries/msimg32.rs"] #[link(name="msimg32")] pub mod msimg32;
#[path="libraries/normaliz.rs"] #[link(name="normaliz")] pub mod normaliz;
#[path="libraries/ntdll.rs"] #[link(name="ntdll")] pub mod ntdll;
#[path="libraries/ntdllp.rs"] #[link(name="ntdllp")] pub mod ntdllp;
#[path="libraries/opengl32.rs"] #[link(name="opengl32")] pub mod opengl32;
#[path="libraries/portabledeviceguids.rs"] #[link(name="portabledeviceguids")] pub mod portabledeviceguids;
#[path="libraries/scrnsave.rs"] #[link(name="scrnsave")] pub mod scrnsave;
#[path="libraries/scrnsavw.rs"] #[link(name="scrnsavw")] pub mod scrnsavw;
#[path="libraries/synchronization.rs"] #[link(name="synchronization")] pub mod synchronization;
#[path="libraries/user32.rs"] #[link(name="user32")] pub mod user32;
#[path="libraries/uuid.rs"] #[link(name="uuid")] pub mod uuid;
#[path="libraries/version.rs"] #[link(name="version")] pub mod version;
#[path="libraries/windowssideshowguids.rs"] #[link(name="windowssideshowguids")] pub mod windowssideshowguids;
#[path="libraries/winspool.rs"] #[link(name="winspool")] pub mod winspool;
