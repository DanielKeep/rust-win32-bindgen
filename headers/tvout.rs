#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct VIDEOPARAMETERS { Guid: ::guiddef::GUID, dwOffset: ::minwindef::ULONG, dwCommand: ::minwindef::ULONG, dwFlags: ::minwindef::ULONG, dwMode: ::minwindef::ULONG, dwTVStandard: ::minwindef::ULONG, dwAvailableModes: ::minwindef::ULONG, dwAvailableTVStandard: ::minwindef::ULONG, dwFlickerFilter: ::minwindef::ULONG, dwOverScanX: ::minwindef::ULONG, dwOverScanY: ::minwindef::ULONG, dwMaxUnscaledX: ::minwindef::ULONG, dwMaxUnscaledY: ::minwindef::ULONG, dwPositionX: ::minwindef::ULONG, dwPositionY: ::minwindef::ULONG, dwBrightness: ::minwindef::ULONG, dwContrast: ::minwindef::ULONG, dwCPType: ::minwindef::ULONG, dwCPCommand: ::minwindef::ULONG, dwCPStandard: ::minwindef::ULONG, dwCPKey: ::minwindef::ULONG, bCP_APSTriggerBits: ::minwindef::ULONG, bOEMCopyProtection: *mut [::minwindef::UCHAR; 256] } /* tvout.h:22:16, tvout.h:22:16, tvout.h:22:16 */
#[cfg(feature="winapi_desktop")] pub type PVIDEOPARAMETERS = *mut ::tvout::VIDEOPARAMETERS; /* tvout.h:46:21, tvout.h:46:21, tvout.h:46:21 */
#[cfg(feature="winapi_desktop")] pub type LPVIDEOPARAMETERS = *mut ::tvout::VIDEOPARAMETERS; /* tvout.h:46:40, tvout.h:46:40, tvout.h:46:40 */
#[cfg(feature="winapi_desktop")] pub const VP_COMMAND_GET: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* tvout.h:48:9, tvout.h:48:9, tvout.h:48:9 */
#[cfg(feature="winapi_desktop")] pub const VP_COMMAND_SET: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* tvout.h:50:9, tvout.h:50:9, tvout.h:50:9 */
#[cfg(feature="winapi_desktop")] pub const VP_FLAGS_TV_MODE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* tvout.h:52:9, tvout.h:52:9, tvout.h:52:9 */
#[cfg(feature="winapi_desktop")] pub const VP_FLAGS_TV_STANDARD: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* tvout.h:53:9, tvout.h:53:9, tvout.h:53:9 */
#[cfg(feature="winapi_desktop")] pub const VP_FLAGS_FLICKER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* tvout.h:54:9, tvout.h:54:9, tvout.h:54:9 */
#[cfg(feature="winapi_desktop")] pub const VP_FLAGS_OVERSCAN: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* tvout.h:55:9, tvout.h:55:9, tvout.h:55:9 */
#[cfg(feature="winapi_desktop")] pub const VP_FLAGS_MAX_UNSCALED: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* tvout.h:56:9, tvout.h:56:9, tvout.h:56:9 */
#[cfg(feature="winapi_desktop")] pub const VP_FLAGS_POSITION: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* tvout.h:57:9, tvout.h:57:9, tvout.h:57:9 */
#[cfg(feature="winapi_desktop")] pub const VP_FLAGS_BRIGHTNESS: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* tvout.h:58:9, tvout.h:58:9, tvout.h:58:9 */
#[cfg(feature="winapi_desktop")] pub const VP_FLAGS_CONTRAST: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* tvout.h:59:9, tvout.h:59:9, tvout.h:59:9 */
#[cfg(feature="winapi_desktop")] pub const VP_FLAGS_COPYPROTECT: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* tvout.h:60:9, tvout.h:60:9, tvout.h:60:9 */
#[cfg(feature="winapi_desktop")] pub const VP_MODE_WIN_GRAPHICS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* tvout.h:62:9, tvout.h:62:9, tvout.h:62:9 */
#[cfg(feature="winapi_desktop")] pub const VP_MODE_TV_PLAYBACK: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* tvout.h:63:9, tvout.h:63:9, tvout.h:63:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_NTSC_M: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* tvout.h:65:9, tvout.h:65:9, tvout.h:65:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_NTSC_M_J: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* tvout.h:66:9, tvout.h:66:9, tvout.h:66:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_PAL_B: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* tvout.h:67:9, tvout.h:67:9, tvout.h:67:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_PAL_D: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* tvout.h:68:9, tvout.h:68:9, tvout.h:68:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_PAL_H: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* tvout.h:69:9, tvout.h:69:9, tvout.h:69:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_PAL_I: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* tvout.h:70:9, tvout.h:70:9, tvout.h:70:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_PAL_M: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* tvout.h:71:9, tvout.h:71:9, tvout.h:71:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_PAL_N: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* tvout.h:72:9, tvout.h:72:9, tvout.h:72:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_SECAM_B: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* tvout.h:73:9, tvout.h:73:9, tvout.h:73:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_SECAM_D: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* tvout.h:74:9, tvout.h:74:9, tvout.h:74:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_SECAM_G: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* tvout.h:75:9, tvout.h:75:9, tvout.h:75:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_SECAM_H: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* tvout.h:76:9, tvout.h:76:9, tvout.h:76:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_SECAM_K: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* tvout.h:77:9, tvout.h:77:9, tvout.h:77:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_SECAM_K1: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* tvout.h:78:9, tvout.h:78:9, tvout.h:78:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_SECAM_L: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* tvout.h:79:9, tvout.h:79:9, tvout.h:79:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_WIN_VGA: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* tvout.h:80:9, tvout.h:80:9, tvout.h:80:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_NTSC_433: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* tvout.h:82:9, tvout.h:82:9, tvout.h:82:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_PAL_G: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* tvout.h:83:9, tvout.h:83:9, tvout.h:83:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_PAL_60: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* tvout.h:84:9, tvout.h:84:9, tvout.h:84:9 */
#[cfg(feature="winapi_desktop")] pub const VP_TV_STANDARD_SECAM_L1: i32 = 0x80000i32; /* Integer(524288, Yes, Unknown) */ /* tvout.h:85:9, tvout.h:85:9, tvout.h:85:9 */
#[cfg(feature="winapi_desktop")] pub const VP_CP_TYPE_APS_TRIGGER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* tvout.h:87:9, tvout.h:87:9, tvout.h:87:9 */
#[cfg(feature="winapi_desktop")] pub const VP_CP_TYPE_MACROVISION: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* tvout.h:88:9, tvout.h:88:9, tvout.h:88:9 */
#[cfg(feature="winapi_desktop")] pub const VP_CP_CMD_ACTIVATE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* tvout.h:90:9, tvout.h:90:9, tvout.h:90:9 */
#[cfg(feature="winapi_desktop")] pub const VP_CP_CMD_DEACTIVATE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* tvout.h:91:9, tvout.h:91:9, tvout.h:91:9 */
#[cfg(feature="winapi_desktop")] pub const VP_CP_CMD_CHANGE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* tvout.h:92:9, tvout.h:92:9, tvout.h:92:9 */
