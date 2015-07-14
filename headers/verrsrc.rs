#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct VS_FIXEDFILEINFO { dwSignature: ::minwindef::DWORD, dwStrucVersion: ::minwindef::DWORD, dwFileVersionMS: ::minwindef::DWORD, dwFileVersionLS: ::minwindef::DWORD, dwProductVersionMS: ::minwindef::DWORD, dwProductVersionLS: ::minwindef::DWORD, dwFileFlagsMask: ::minwindef::DWORD, dwFileFlags: ::minwindef::DWORD, dwFileOS: ::minwindef::DWORD, dwFileType: ::minwindef::DWORD, dwFileSubtype: ::minwindef::DWORD, dwFileDateMS: ::minwindef::DWORD, dwFileDateLS: ::minwindef::DWORD } /* verrsrc.h:147:16, verrsrc.h:147:16, verrsrc.h:147:16 */
#[cfg(feature="winapi_app")] pub const VS_VERSION_INFO: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* verrsrc.h:18:9, verrsrc.h:18:9, verrsrc.h:18:9 */
#[cfg(feature="winapi_app")] pub const VS_USER_DEFINED: i32 = 0x64i32; /* Integer(100, Yes, Unknown) */ /* verrsrc.h:19:9, verrsrc.h:19:9, verrsrc.h:19:9 */
#[cfg(feature="winapi_app")] pub const VS_FFI_SIGNATURE: i64 = 0xfeef04bdi64; /* Integer(4277077181, Yes, Long) */ /* verrsrc.h:23:9, verrsrc.h:23:9, verrsrc.h:23:9 */
#[cfg(feature="winapi_app")] pub const VS_FFI_STRUCVERSION: i64 = 0x10000i64; /* Integer(65536, Yes, Long) */ /* verrsrc.h:27:9, verrsrc.h:27:9, verrsrc.h:27:9 */
#[cfg(feature="winapi_app")] pub const VS_FFI_FILEFLAGSMASK: i64 = 0x3fi64; /* Integer(63, Yes, Long) */ /* verrsrc.h:28:9, verrsrc.h:28:9, verrsrc.h:28:9 */
#[cfg(feature="winapi_app")] pub const VS_FF_DEBUG: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* verrsrc.h:31:9, verrsrc.h:31:9, verrsrc.h:31:9 */
#[cfg(feature="winapi_app")] pub const VS_FF_PRERELEASE: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* verrsrc.h:32:9, verrsrc.h:32:9, verrsrc.h:32:9 */
#[cfg(feature="winapi_app")] pub const VS_FF_PATCHED: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* verrsrc.h:33:9, verrsrc.h:33:9, verrsrc.h:33:9 */
#[cfg(feature="winapi_app")] pub const VS_FF_PRIVATEBUILD: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* verrsrc.h:34:9, verrsrc.h:34:9, verrsrc.h:34:9 */
#[cfg(feature="winapi_app")] pub const VS_FF_INFOINFERRED: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* verrsrc.h:35:9, verrsrc.h:35:9, verrsrc.h:35:9 */
#[cfg(feature="winapi_app")] pub const VS_FF_SPECIALBUILD: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* verrsrc.h:36:9, verrsrc.h:36:9, verrsrc.h:36:9 */
#[cfg(feature="winapi_app")] pub const VOS_UNKNOWN: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* verrsrc.h:39:9, verrsrc.h:39:9, verrsrc.h:39:9 */
#[cfg(feature="winapi_app")] pub const VOS_DOS: i64 = 0x10000i64; /* Integer(65536, Yes, Long) */ /* verrsrc.h:40:9, verrsrc.h:40:9, verrsrc.h:40:9 */
#[cfg(feature="winapi_app")] pub const VOS_OS216: i64 = 0x20000i64; /* Integer(131072, Yes, Long) */ /* verrsrc.h:41:9, verrsrc.h:41:9, verrsrc.h:41:9 */
#[cfg(feature="winapi_app")] pub const VOS_OS232: i64 = 0x30000i64; /* Integer(196608, Yes, Long) */ /* verrsrc.h:42:9, verrsrc.h:42:9, verrsrc.h:42:9 */
#[cfg(feature="winapi_app")] pub const VOS_NT: i64 = 0x40000i64; /* Integer(262144, Yes, Long) */ /* verrsrc.h:43:9, verrsrc.h:43:9, verrsrc.h:43:9 */
#[cfg(feature="winapi_app")] pub const VOS_WINCE: i64 = 0x50000i64; /* Integer(327680, Yes, Long) */ /* verrsrc.h:44:9, verrsrc.h:44:9, verrsrc.h:44:9 */
#[cfg(feature="winapi_app")] pub const VOS_DOS_WINDOWS16: i64 = 0x10001i64; /* Integer(65537, Yes, Long) */ /* verrsrc.h:52:9, verrsrc.h:52:9, verrsrc.h:52:9 */
#[cfg(feature="winapi_app")] pub const VOS_DOS_WINDOWS32: i64 = 0x10004i64; /* Integer(65540, Yes, Long) */ /* verrsrc.h:53:9, verrsrc.h:53:9, verrsrc.h:53:9 */
#[cfg(feature="winapi_app")] pub const VOS_OS216_PM16: i64 = 0x20002i64; /* Integer(131074, Yes, Long) */ /* verrsrc.h:54:9, verrsrc.h:54:9, verrsrc.h:54:9 */
#[cfg(feature="winapi_app")] pub const VOS_OS232_PM32: i64 = 0x30003i64; /* Integer(196611, Yes, Long) */ /* verrsrc.h:55:9, verrsrc.h:55:9, verrsrc.h:55:9 */
#[cfg(feature="winapi_app")] pub const VOS_NT_WINDOWS32: i64 = 0x40004i64; /* Integer(262148, Yes, Long) */ /* verrsrc.h:56:9, verrsrc.h:56:9, verrsrc.h:56:9 */
#[cfg(feature="winapi_app")] pub const VFT_UNKNOWN: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* verrsrc.h:59:9, verrsrc.h:59:9, verrsrc.h:59:9 */
#[cfg(feature="winapi_app")] pub const VFT_APP: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* verrsrc.h:60:9, verrsrc.h:60:9, verrsrc.h:60:9 */
#[cfg(feature="winapi_app")] pub const VFT_DLL: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* verrsrc.h:61:9, verrsrc.h:61:9, verrsrc.h:61:9 */
#[cfg(feature="winapi_app")] pub const VFT_DRV: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* verrsrc.h:62:9, verrsrc.h:62:9, verrsrc.h:62:9 */
#[cfg(feature="winapi_app")] pub const VFT_FONT: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* verrsrc.h:63:9, verrsrc.h:63:9, verrsrc.h:63:9 */
#[cfg(feature="winapi_app")] pub const VFT_VXD: i64 = 0x5i64; /* Integer(5, Yes, Long) */ /* verrsrc.h:64:9, verrsrc.h:64:9, verrsrc.h:64:9 */
#[cfg(feature="winapi_app")] pub const VFT_STATIC_LIB: i64 = 0x7i64; /* Integer(7, Yes, Long) */ /* verrsrc.h:65:9, verrsrc.h:65:9, verrsrc.h:65:9 */
#[cfg(feature="winapi_app")] pub const VFT2_UNKNOWN: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* verrsrc.h:68:9, verrsrc.h:68:9, verrsrc.h:68:9 */
#[cfg(feature="winapi_app")] pub const VFT2_DRV_PRINTER: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* verrsrc.h:69:9, verrsrc.h:69:9, verrsrc.h:69:9 */
#[cfg(feature="winapi_app")] pub const VFT2_DRV_KEYBOARD: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* verrsrc.h:70:9, verrsrc.h:70:9, verrsrc.h:70:9 */
#[cfg(feature="winapi_app")] pub const VFT2_DRV_LANGUAGE: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* verrsrc.h:71:9, verrsrc.h:71:9, verrsrc.h:71:9 */
#[cfg(feature="winapi_app")] pub const VFT2_DRV_DISPLAY: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* verrsrc.h:72:9, verrsrc.h:72:9, verrsrc.h:72:9 */
#[cfg(feature="winapi_app")] pub const VFT2_DRV_MOUSE: i64 = 0x5i64; /* Integer(5, Yes, Long) */ /* verrsrc.h:73:9, verrsrc.h:73:9, verrsrc.h:73:9 */
#[cfg(feature="winapi_app")] pub const VFT2_DRV_NETWORK: i64 = 0x6i64; /* Integer(6, Yes, Long) */ /* verrsrc.h:74:9, verrsrc.h:74:9, verrsrc.h:74:9 */
#[cfg(feature="winapi_app")] pub const VFT2_DRV_SYSTEM: i64 = 0x7i64; /* Integer(7, Yes, Long) */ /* verrsrc.h:75:9, verrsrc.h:75:9, verrsrc.h:75:9 */
#[cfg(feature="winapi_app")] pub const VFT2_DRV_INSTALLABLE: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* verrsrc.h:76:9, verrsrc.h:76:9, verrsrc.h:76:9 */
#[cfg(feature="winapi_app")] pub const VFT2_DRV_SOUND: i64 = 0x9i64; /* Integer(9, Yes, Long) */ /* verrsrc.h:77:9, verrsrc.h:77:9, verrsrc.h:77:9 */
#[cfg(feature="winapi_app")] pub const VFT2_DRV_COMM: i64 = 0xai64; /* Integer(10, Yes, Long) */ /* verrsrc.h:78:9, verrsrc.h:78:9, verrsrc.h:78:9 */
#[cfg(feature="winapi_app")] pub const VFT2_DRV_INPUTMETHOD: i64 = 0xbi64; /* Integer(11, Yes, Long) */ /* verrsrc.h:79:9, verrsrc.h:79:9, verrsrc.h:79:9 */
#[cfg(feature="winapi_app")] pub const VFT2_DRV_VERSIONED_PRINTER: i64 = 0xci64; /* Integer(12, Yes, Long) */ /* verrsrc.h:80:9, verrsrc.h:80:9, verrsrc.h:80:9 */
#[cfg(feature="winapi_app")] pub const VFT2_FONT_RASTER: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* verrsrc.h:83:9, verrsrc.h:83:9, verrsrc.h:83:9 */
#[cfg(feature="winapi_app")] pub const VFT2_FONT_VECTOR: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* verrsrc.h:84:9, verrsrc.h:84:9, verrsrc.h:84:9 */
#[cfg(feature="winapi_app")] pub const VFT2_FONT_TRUETYPE: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* verrsrc.h:85:9, verrsrc.h:85:9, verrsrc.h:85:9 */
#[cfg(feature="winapi_desktop")] pub const VFFF_ISSHAREDFILE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* verrsrc.h:94:9, verrsrc.h:94:9, verrsrc.h:94:9 */
#[cfg(feature="winapi_desktop")] pub const VFF_CURNEDEST: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* verrsrc.h:96:9, verrsrc.h:96:9, verrsrc.h:96:9 */
#[cfg(feature="winapi_desktop")] pub const VFF_FILEINUSE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* verrsrc.h:97:9, verrsrc.h:97:9, verrsrc.h:97:9 */
#[cfg(feature="winapi_desktop")] pub const VFF_BUFFTOOSMALL: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* verrsrc.h:98:9, verrsrc.h:98:9, verrsrc.h:98:9 */
#[cfg(feature="winapi_desktop")] pub const VIFF_FORCEINSTALL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* verrsrc.h:101:9, verrsrc.h:101:9, verrsrc.h:101:9 */
#[cfg(feature="winapi_desktop")] pub const VIFF_DONTDELETEOLD: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* verrsrc.h:102:9, verrsrc.h:102:9, verrsrc.h:102:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_TEMPFILE: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* verrsrc.h:104:9, verrsrc.h:104:9, verrsrc.h:104:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_MISMATCH: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* verrsrc.h:105:9, verrsrc.h:105:9, verrsrc.h:105:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_SRCOLD: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* verrsrc.h:106:9, verrsrc.h:106:9, verrsrc.h:106:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_DIFFLANG: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* verrsrc.h:108:9, verrsrc.h:108:9, verrsrc.h:108:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_DIFFCODEPG: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* verrsrc.h:109:9, verrsrc.h:109:9, verrsrc.h:109:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_DIFFTYPE: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* verrsrc.h:110:9, verrsrc.h:110:9, verrsrc.h:110:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_WRITEPROT: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* verrsrc.h:112:9, verrsrc.h:112:9, verrsrc.h:112:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_FILEINUSE: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* verrsrc.h:113:9, verrsrc.h:113:9, verrsrc.h:113:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_OUTOFSPACE: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* verrsrc.h:114:9, verrsrc.h:114:9, verrsrc.h:114:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_ACCESSVIOLATION: i64 = 0x200i64; /* Integer(512, Yes, Long) */ /* verrsrc.h:115:9, verrsrc.h:115:9, verrsrc.h:115:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_SHARINGVIOLATION: i64 = 0x400i64; /* Integer(1024, Yes, Long) */ /* verrsrc.h:116:9, verrsrc.h:116:9, verrsrc.h:116:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_CANNOTCREATE: i64 = 0x800i64; /* Integer(2048, Yes, Long) */ /* verrsrc.h:117:9, verrsrc.h:117:9, verrsrc.h:117:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_CANNOTDELETE: i64 = 0x1000i64; /* Integer(4096, Yes, Long) */ /* verrsrc.h:118:9, verrsrc.h:118:9, verrsrc.h:118:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_CANNOTRENAME: i64 = 0x2000i64; /* Integer(8192, Yes, Long) */ /* verrsrc.h:119:9, verrsrc.h:119:9, verrsrc.h:119:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_CANNOTDELETECUR: i64 = 0x4000i64; /* Integer(16384, Yes, Long) */ /* verrsrc.h:120:9, verrsrc.h:120:9, verrsrc.h:120:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_OUTOFMEMORY: i64 = 0x8000i64; /* Integer(32768, Yes, Long) */ /* verrsrc.h:121:9, verrsrc.h:121:9, verrsrc.h:121:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_CANNOTREADSRC: i64 = 0x10000i64; /* Integer(65536, Yes, Long) */ /* verrsrc.h:123:9, verrsrc.h:123:9, verrsrc.h:123:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_CANNOTREADDST: i64 = 0x20000i64; /* Integer(131072, Yes, Long) */ /* verrsrc.h:124:9, verrsrc.h:124:9, verrsrc.h:124:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_BUFFTOOSMALL: i64 = 0x40000i64; /* Integer(262144, Yes, Long) */ /* verrsrc.h:126:9, verrsrc.h:126:9, verrsrc.h:126:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_CANNOTLOADLZ32: i64 = 0x80000i64; /* Integer(524288, Yes, Long) */ /* verrsrc.h:127:9, verrsrc.h:127:9, verrsrc.h:127:9 */
#[cfg(feature="winapi_desktop")] pub const VIF_CANNOTLOADCABINET: i64 = 0x100000i64; /* Integer(1048576, Yes, Long) */ /* verrsrc.h:128:9, verrsrc.h:128:9, verrsrc.h:128:9 */
#[cfg(feature="winapi_desktop")] pub const FILE_VER_GET_LOCALISED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* verrsrc.h:141:9, verrsrc.h:141:9, verrsrc.h:141:9 */
#[cfg(feature="winapi_desktop")] pub const FILE_VER_GET_NEUTRAL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* verrsrc.h:142:9, verrsrc.h:142:9, verrsrc.h:142:9 */
#[cfg(feature="winapi_desktop")] pub const FILE_VER_GET_PREFETCHED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* verrsrc.h:143:9, verrsrc.h:143:9, verrsrc.h:143:9 */
