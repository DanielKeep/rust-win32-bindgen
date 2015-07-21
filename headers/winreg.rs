#[cfg(feature="winapi_desktop")] pub type REGSAM = ::winnt::ACCESS_MASK; /* winreg.h:89:21, winreg.h:89:21, winreg.h:89:21 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct val_context { valuelen: ::libc::c_int, value_context: ::minwindef::LPVOID, val_buff_ptr: ::minwindef::LPVOID } /* winreg.h:119:8, winreg.h:119:8, winreg.h:119:8 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PVALCONTEXT = *mut ::winreg::val_context; /* winreg.h:125:33, winreg.h:125:33, winreg.h:125:33 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct pvalueA { pv_valuename: ::winnt::LPSTR, pv_valuelen: ::libc::c_int, pv_value_context: ::minwindef::LPVOID, pv_type: ::minwindef::DWORD } /* winreg.h:127:16, winreg.h:127:16, winreg.h:127:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PVALUEA = ::winreg::pvalueA; /* winreg.h:132:2, winreg.h:132:2, winreg.h:132:2 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PPVALUEA = *mut ::winreg::pvalueA; /* winreg.h:132:16, winreg.h:132:16, winreg.h:132:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct pvalueW { pv_valuename: ::winnt::LPWSTR, pv_valuelen: ::libc::c_int, pv_value_context: ::minwindef::LPVOID, pv_type: ::minwindef::DWORD } /* winreg.h:133:16, winreg.h:133:16, winreg.h:133:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PVALUEW = ::winreg::pvalueW; /* winreg.h:138:2, winreg.h:138:2, winreg.h:138:2 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PPVALUEW = *mut ::winreg::pvalueW; /* winreg.h:138:16, winreg.h:138:16, winreg.h:138:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PVALUE = ::winreg::PVALUEW; /* winreg.h:140:17, winreg.h:140:17, winreg.h:140:17 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PPVALUE = ::winreg::PPVALUEW; /* winreg.h:141:18, winreg.h:141:18, winreg.h:141:18 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[cfg(any(target_arch="x86"))] pub type QUERYHANDLER = Option<extern "C" fn(*mut ::libc::c_void, *mut ::winreg::val_context, ::libc::c_ulong, *mut ::libc::c_void, *mut ::libc::c_ulong, ::libc::c_ulong) -> ::libc::c_ulong>; /* winreg.h:149:1 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[cfg(any(target_arch="x86"))] pub type PQUERYHANDLER = Option<extern "C" fn(*mut ::libc::c_void, *mut ::winreg::val_context, ::libc::c_ulong, *mut ::libc::c_void, *mut ::libc::c_ulong, ::libc::c_ulong) -> ::libc::c_ulong>; /* winreg.h:152:27 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct provider_info { pi_R0_1val: ::winreg::PQUERYHANDLER, pi_R0_allvals: ::winreg::PQUERYHANDLER, pi_R3_1val: ::winreg::PQUERYHANDLER, pi_R3_allvals: ::winreg::PQUERYHANDLER, pi_flags: ::minwindef::DWORD, pi_key_context: ::minwindef::LPVOID } /* winreg.h:154:16, winreg.h:154:16, winreg.h:154:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type REG_PROVIDER = ::winreg::provider_info; /* winreg.h:161:2, winreg.h:161:2, winreg.h:161:2 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PPROVIDER = *mut ::winreg::provider_info; /* winreg.h:163:35, winreg.h:163:35, winreg.h:163:35 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct value_entA { ve_valuename: ::winnt::LPSTR, ve_valuelen: ::minwindef::DWORD, ve_valueptr: ::basetsd::DWORD_PTR, ve_type: ::minwindef::DWORD } /* winreg.h:165:16, winreg.h:165:16, winreg.h:165:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type VALENTA = ::winreg::value_entA; /* winreg.h:170:2, winreg.h:170:2, winreg.h:170:2 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PVALENTA = *mut ::winreg::value_entA; /* winreg.h:170:16, winreg.h:170:16, winreg.h:170:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct value_entW { ve_valuename: ::winnt::LPWSTR, ve_valuelen: ::minwindef::DWORD, ve_valueptr: ::basetsd::DWORD_PTR, ve_type: ::minwindef::DWORD } /* winreg.h:171:16, winreg.h:171:16, winreg.h:171:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type VALENTW = ::winreg::value_entW; /* winreg.h:176:2, winreg.h:176:2, winreg.h:176:2 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PVALENTW = *mut ::winreg::value_entW; /* winreg.h:176:16, winreg.h:176:16, winreg.h:176:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type VALENT = ::winreg::VALENTW; /* winreg.h:178:17, winreg.h:178:17, winreg.h:178:17 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PVALENT = ::winreg::PVALENTW; /* winreg.h:179:18, winreg.h:179:18, winreg.h:179:18 */
#[cfg(feature="winapi_desktop")] pub type LSTATUS = ::winnt::LONG; /* winreg.h:201:59, winreg.h:201:59, winreg.h:201:59 */
#[cfg(feature="winapi_desktop")] pub const RRF_RT_REG_NONE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winreg.h:60:9, winreg.h:60:9, winreg.h:60:9 */
#[cfg(feature="winapi_desktop")] pub const RRF_RT_REG_SZ: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winreg.h:61:9, winreg.h:61:9, winreg.h:61:9 */
#[cfg(feature="winapi_desktop")] pub const RRF_RT_REG_EXPAND_SZ: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winreg.h:62:9, winreg.h:62:9, winreg.h:62:9 */
#[cfg(feature="winapi_desktop")] pub const RRF_RT_REG_BINARY: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winreg.h:63:9, winreg.h:63:9, winreg.h:63:9 */
#[cfg(feature="winapi_desktop")] pub const RRF_RT_REG_DWORD: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winreg.h:64:9, winreg.h:64:9, winreg.h:64:9 */
#[cfg(feature="winapi_desktop")] pub const RRF_RT_REG_MULTI_SZ: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winreg.h:65:9, winreg.h:65:9, winreg.h:65:9 */
#[cfg(feature="winapi_desktop")] pub const RRF_RT_REG_QWORD: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winreg.h:66:9, winreg.h:66:9, winreg.h:66:9 */
#[cfg(feature="winapi_desktop")] pub const RRF_RT_ANY: i32 = 0xffffi32; /* Integer(65535, Yes, Unknown) */ /* winreg.h:70:9, winreg.h:70:9, winreg.h:70:9 */
#[cfg(feature="winapi_desktop")] pub const RRF_NOEXPAND: i32 = 0x10000000i32; /* Integer(268435456, Yes, Unknown) */ /* winreg.h:72:9, winreg.h:72:9, winreg.h:72:9 */
#[cfg(feature="winapi_desktop")] pub const RRF_ZEROONFAILURE: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* winreg.h:73:9, winreg.h:73:9, winreg.h:73:9 */
#[cfg(feature="winapi_desktop")] pub const REG_PROCESS_APPKEY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winreg.h:78:9, winreg.h:78:9, winreg.h:78:9 */
#[cfg(feature="winapi_desktop")] pub const REG_MUI_STRING_TRUNCATE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winreg.h:83:9, winreg.h:83:9, winreg.h:83:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const REG_SECURE_CONNECTION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winreg.h:112:9, winreg.h:112:9, winreg.h:112:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const PROVIDER_KEEPS_VALUE_LENGTH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winreg.h:118:9, winreg.h:118:9, winreg.h:118:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::reason::SHTDN_REASON_UNKNOWN as REASON_UNKNOWN; /* winreg.h:1326:9, winreg.h:1326:9, winreg.h:1326:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::reason::SHTDN_REASON_FLAG_PLANNED as REASON_PLANNED_FLAG; /* winreg.h:1328:9, winreg.h:1328:9, winreg.h:1328:9 */
#[cfg(feature="winapi_desktop")] pub const SHUTDOWN_FORCE_OTHERS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winreg.h:1373:9, winreg.h:1373:9, winreg.h:1373:9 */
#[cfg(feature="winapi_desktop")] pub const SHUTDOWN_FORCE_SELF: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winreg.h:1374:9, winreg.h:1374:9, winreg.h:1374:9 */
#[cfg(feature="winapi_desktop")] pub const SHUTDOWN_RESTART: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winreg.h:1375:9, winreg.h:1375:9, winreg.h:1375:9 */
#[cfg(feature="winapi_desktop")] pub const SHUTDOWN_POWEROFF: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winreg.h:1376:9, winreg.h:1376:9, winreg.h:1376:9 */
#[cfg(feature="winapi_desktop")] pub const SHUTDOWN_NOREBOOT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winreg.h:1377:9, winreg.h:1377:9, winreg.h:1377:9 */
#[cfg(feature="winapi_desktop")] pub const SHUTDOWN_GRACE_OVERRIDE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winreg.h:1378:9, winreg.h:1378:9, winreg.h:1378:9 */
#[cfg(feature="winapi_desktop")] pub const SHUTDOWN_INSTALL_UPDATES: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winreg.h:1379:9, winreg.h:1379:9, winreg.h:1379:9 */
#[cfg(feature="winapi_desktop")] pub const SHUTDOWN_RESTARTAPPS: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winreg.h:1380:9, winreg.h:1380:9, winreg.h:1380:9 */
#[cfg(feature="winapi_desktop")] pub const SHUTDOWN_SKIP_SVC_PRESHUTDOWN: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winreg.h:1381:9, winreg.h:1381:9, winreg.h:1381:9 */
#[cfg(feature="winapi_desktop")] pub const SHUTDOWN_HYBRID: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winreg.h:1382:9, winreg.h:1382:9, winreg.h:1382:9 */
#[cfg(feature="winapi_desktop")] pub const SHUTDOWN_RESTART_BOOTOPTIONS: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winreg.h:1383:9, winreg.h:1383:9, winreg.h:1383:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[cfg(any(target_arch="x86_64", target_arch="arm"))] pub type QUERYHANDLER = Option<extern "system" fn(*mut ::libc::c_void, *mut ::winreg::val_context, ::libc::c_ulong, *mut ::libc::c_void, *mut ::libc::c_ulong, ::libc::c_ulong) -> ::libc::c_ulong>; /* winreg.h:149:1, winreg.h:149:1 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[cfg(any(target_arch="x86_64", target_arch="arm"))] pub type PQUERYHANDLER = Option<extern "system" fn(*mut ::libc::c_void, *mut ::winreg::val_context, ::libc::c_ulong, *mut ::libc::c_void, *mut ::libc::c_ulong, ::libc::c_ulong) -> ::libc::c_ulong>; /* winreg.h:152:27, winreg.h:152:27 */
