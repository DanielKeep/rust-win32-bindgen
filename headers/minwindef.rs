#[cfg(feature="winapi_app")] pub type ULONG = ::libc::c_ulong; /* minwindef.h:51:23, minwindef.h:51:23, minwindef.h:51:23 */
#[cfg(feature="winapi_app")] pub type PULONG = *mut ::libc::c_ulong; /* minwindef.h:52:16, minwindef.h:52:16, minwindef.h:52:16 */
#[cfg(feature="winapi_app")] pub type USHORT = ::libc::c_ushort; /* minwindef.h:53:24, minwindef.h:53:24, minwindef.h:53:24 */
#[cfg(feature="winapi_app")] pub type PUSHORT = *mut ::libc::c_ushort; /* minwindef.h:54:17, minwindef.h:54:17, minwindef.h:54:17 */
#[cfg(feature="winapi_app")] pub type UCHAR = ::libc::c_uchar; /* minwindef.h:55:23, minwindef.h:55:23, minwindef.h:55:23 */
#[cfg(feature="winapi_app")] pub type PUCHAR = *mut ::libc::c_uchar; /* minwindef.h:56:16, minwindef.h:56:16, minwindef.h:56:16 */
#[cfg(feature="winapi_app")] pub type PSZ = *mut ::libc::c_schar; /* minwindef.h:57:33, minwindef.h:57:33, minwindef.h:57:33 */
#[cfg(feature="winapi_app")] pub type DWORD = ::libc::c_ulong; /* minwindef.h:156:29, minwindef.h:156:29, minwindef.h:156:29 */
#[cfg(feature="winapi_app")] pub type BOOL = ::libc::c_int; /* minwindef.h:157:29, minwindef.h:157:29, minwindef.h:157:29 */
#[cfg(feature="winapi_app")] pub type BYTE = ::libc::c_uchar; /* minwindef.h:158:29, minwindef.h:158:29, minwindef.h:158:29 */
#[cfg(feature="winapi_app")] pub type WORD = ::libc::c_ushort; /* minwindef.h:159:29, minwindef.h:159:29, minwindef.h:159:29 */
#[cfg(feature="winapi_app")] pub type FLOAT = ::libc::c_float; /* minwindef.h:160:29, minwindef.h:160:29, minwindef.h:160:29 */
#[cfg(feature="winapi_app")] pub type PFLOAT = *mut ::libc::c_float; /* minwindef.h:161:30, minwindef.h:161:30, minwindef.h:161:30 */
#[cfg(feature="winapi_app")] pub type PBOOL = *mut ::libc::c_int; /* minwindef.h:162:30, minwindef.h:162:30, minwindef.h:162:30 */
#[cfg(feature="winapi_app")] pub type LPBOOL = *mut ::libc::c_int; /* minwindef.h:163:30, minwindef.h:163:30, minwindef.h:163:30 */
#[cfg(feature="winapi_app")] pub type PBYTE = *mut ::libc::c_uchar; /* minwindef.h:164:30, minwindef.h:164:30, minwindef.h:164:30 */
#[cfg(feature="winapi_app")] pub type LPBYTE = *mut ::libc::c_uchar; /* minwindef.h:165:30, minwindef.h:165:30, minwindef.h:165:30 */
#[cfg(feature="winapi_app")] pub type PINT = *mut ::libc::c_int; /* minwindef.h:166:30, minwindef.h:166:30, minwindef.h:166:30 */
#[cfg(feature="winapi_app")] pub type LPINT = *mut ::libc::c_int; /* minwindef.h:167:30, minwindef.h:167:30, minwindef.h:167:30 */
#[cfg(feature="winapi_app")] pub type PWORD = *mut ::libc::c_ushort; /* minwindef.h:168:30, minwindef.h:168:30, minwindef.h:168:30 */
#[cfg(feature="winapi_app")] pub type LPWORD = *mut ::libc::c_ushort; /* minwindef.h:169:30, minwindef.h:169:30, minwindef.h:169:30 */
#[cfg(feature="winapi_app")] pub type LPLONG = *mut ::libc::c_long; /* minwindef.h:170:30, minwindef.h:170:30, minwindef.h:170:30 */
#[cfg(feature="winapi_app")] pub type PDWORD = *mut ::libc::c_ulong; /* minwindef.h:171:30, minwindef.h:171:30, minwindef.h:171:30 */
#[cfg(feature="winapi_app")] pub type LPDWORD = *mut ::libc::c_ulong; /* minwindef.h:172:30, minwindef.h:172:30, minwindef.h:172:30 */
#[cfg(feature="winapi_app")] pub type LPVOID = *mut ::libc::c_void; /* minwindef.h:173:30, minwindef.h:173:30, minwindef.h:173:30 */
#[cfg(feature="winapi_app")] pub type LPCVOID = *const ::libc::c_void; /* minwindef.h:174:30, minwindef.h:174:30, minwindef.h:174:30 */
#[cfg(feature="winapi_app")] pub type UINT = ::libc::c_uint; /* minwindef.h:177:29, minwindef.h:177:29, minwindef.h:177:29 */
#[cfg(feature="winapi_app")] pub type PUINT = *mut ::libc::c_uint; /* minwindef.h:178:30, minwindef.h:178:30, minwindef.h:178:30 */
#[cfg(feature="winapi_app")] pub type WPARAM = ::basetsd::UINT_PTR; /* minwindef.h:186:29, minwindef.h:186:29, minwindef.h:186:29 */
#[cfg(feature="winapi_app")] pub type LPARAM = ::basetsd::LONG_PTR; /* minwindef.h:187:29, minwindef.h:187:29, minwindef.h:187:29 */
#[cfg(feature="winapi_app")] pub type LRESULT = ::basetsd::LONG_PTR; /* minwindef.h:188:29, minwindef.h:188:29, minwindef.h:188:29 */
#[cfg(feature="winapi_app")] pub type SPHANDLE = *mut *mut ::libc::c_void; /* minwindef.h:209:30, minwindef.h:209:30, minwindef.h:209:30 */
#[cfg(feature="winapi_app")] pub type LPHANDLE = *mut *mut ::libc::c_void; /* minwindef.h:210:30, minwindef.h:210:30, minwindef.h:210:30 */
#[cfg(feature="winapi_app")] pub type HGLOBAL = ::winnt::HANDLE; /* minwindef.h:211:29, minwindef.h:211:29, minwindef.h:211:29 */
#[cfg(feature="winapi_app")] pub type HLOCAL = ::winnt::HANDLE; /* minwindef.h:212:29, minwindef.h:212:29, minwindef.h:212:29 */
#[cfg(feature="winapi_app")] pub type GLOBALHANDLE = ::winnt::HANDLE; /* minwindef.h:213:29, minwindef.h:213:29, minwindef.h:213:29 */
#[cfg(feature="winapi_app")] pub type LOCALHANDLE = ::winnt::HANDLE; /* minwindef.h:214:29, minwindef.h:214:29, minwindef.h:214:29 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type FARPROC = extern "system" fn() -> ::libc::c_int; /* minwindef.h:222:26, minwindef.h:222:26 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type NEARPROC = extern "system" fn() -> ::libc::c_int; /* minwindef.h:223:27, minwindef.h:223:27 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type PROC = extern "system" fn() -> ::libc::c_int; /* minwindef.h:224:22, minwindef.h:224:22 */
#[cfg(feature="winapi_app")] pub type ATOM = ::minwindef::WORD; /* minwindef.h:237:29, minwindef.h:237:29, minwindef.h:237:29 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HKEY__ { unused: ::libc::c_int } /* minwindef.h:239:1, minwindef.h:239:1, minwindef.h:239:1 */
#[cfg(feature="winapi_app")] pub type HKEY = *mut ::minwindef::HKEY__; /* minwindef.h:239:1, minwindef.h:239:1, minwindef.h:239:1 */
#[cfg(feature="winapi_app")] pub type PHKEY = *mut *mut ::minwindef::HKEY__; /* minwindef.h:240:15, minwindef.h:240:15, minwindef.h:240:15 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HMETAFILE__ { unused: ::libc::c_int } /* minwindef.h:241:1, minwindef.h:241:1, minwindef.h:241:1 */
#[cfg(feature="winapi_app")] pub type HMETAFILE = *mut ::minwindef::HMETAFILE__; /* minwindef.h:241:1, minwindef.h:241:1, minwindef.h:241:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HINSTANCE__ { unused: ::libc::c_int } /* minwindef.h:242:1, minwindef.h:242:1, minwindef.h:242:1 */
#[cfg(feature="winapi_app")] pub type HINSTANCE = *mut ::minwindef::HINSTANCE__; /* minwindef.h:242:1, minwindef.h:242:1, minwindef.h:242:1 */
#[cfg(feature="winapi_app")] pub type HMODULE = ::minwindef::HINSTANCE; /* minwindef.h:243:19, minwindef.h:243:19, minwindef.h:243:19 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HRGN__ { unused: ::libc::c_int } /* minwindef.h:244:1, minwindef.h:244:1, minwindef.h:244:1 */
#[cfg(feature="winapi_app")] pub type HRGN = *mut ::minwindef::HRGN__; /* minwindef.h:244:1, minwindef.h:244:1, minwindef.h:244:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HRSRC__ { unused: ::libc::c_int } /* minwindef.h:245:1, minwindef.h:245:1, minwindef.h:245:1 */
#[cfg(feature="winapi_app")] pub type HRSRC = *mut ::minwindef::HRSRC__; /* minwindef.h:245:1, minwindef.h:245:1, minwindef.h:245:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HSPRITE__ { unused: ::libc::c_int } /* minwindef.h:246:1, minwindef.h:246:1, minwindef.h:246:1 */
#[cfg(feature="winapi_app")] pub type HSPRITE = *mut ::minwindef::HSPRITE__; /* minwindef.h:246:1, minwindef.h:246:1, minwindef.h:246:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HLSURF__ { unused: ::libc::c_int } /* minwindef.h:247:1, minwindef.h:247:1, minwindef.h:247:1 */
#[cfg(feature="winapi_app")] pub type HLSURF = *mut ::minwindef::HLSURF__; /* minwindef.h:247:1, minwindef.h:247:1, minwindef.h:247:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HSTR__ { unused: ::libc::c_int } /* minwindef.h:248:1, minwindef.h:248:1, minwindef.h:248:1 */
#[cfg(feature="winapi_app")] pub type HSTR = *mut ::minwindef::HSTR__; /* minwindef.h:248:1, minwindef.h:248:1, minwindef.h:248:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HTASK__ { unused: ::libc::c_int } /* minwindef.h:249:1, minwindef.h:249:1, minwindef.h:249:1 */
#[cfg(feature="winapi_app")] pub type HTASK = *mut ::minwindef::HTASK__; /* minwindef.h:249:1, minwindef.h:249:1, minwindef.h:249:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HWINSTA__ { unused: ::libc::c_int } /* minwindef.h:250:1, minwindef.h:250:1, minwindef.h:250:1 */
#[cfg(feature="winapi_app")] pub type HWINSTA = *mut ::minwindef::HWINSTA__; /* minwindef.h:250:1, minwindef.h:250:1, minwindef.h:250:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HKL__ { unused: ::libc::c_int } /* minwindef.h:251:1, minwindef.h:251:1, minwindef.h:251:1 */
#[cfg(feature="winapi_app")] pub type HKL = *mut ::minwindef::HKL__; /* minwindef.h:251:1, minwindef.h:251:1, minwindef.h:251:1 */
#[cfg(feature="winapi_app")] pub type HFILE = ::libc::c_int; /* minwindef.h:254:13, minwindef.h:254:13, minwindef.h:254:13 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct FILETIME { dwLowDateTime: ::minwindef::DWORD, dwHighDateTime: ::minwindef::DWORD } /* minwindef.h:263:16, minwindef.h:263:16, minwindef.h:263:16 */
#[cfg(feature="winapi_app")] pub type PFILETIME = *mut ::minwindef::FILETIME; /* minwindef.h:266:14, minwindef.h:266:14, minwindef.h:266:14 */
#[cfg(feature="winapi_app")] pub type LPFILETIME = *mut ::minwindef::FILETIME; /* minwindef.h:266:26, minwindef.h:266:26, minwindef.h:266:26 */
#[cfg(feature="winapi_app")] pub const STRICT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* minwindef.h:23:9, minwindef.h:23:9, minwindef.h:23:9 */
#[cfg(feature="winapi_app")] pub const MAX_PATH: i32 = 0x104i32; /* Integer(260, Yes, Unknown) */ /* minwindef.h:60:9, minwindef.h:60:9, minwindef.h:60:9 */
#[cfg(feature="winapi_app")] pub const FALSE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* minwindef.h:71:9, minwindef.h:71:9, minwindef.h:71:9 */
#[cfg(feature="winapi_app")] pub const TRUE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* minwindef.h:75:9, minwindef.h:75:9, minwindef.h:75:9 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] pub type FARPROC = extern "system" fn() -> ::libc::c_longlong; /* minwindef.h:218:30 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] pub type NEARPROC = extern "system" fn() -> ::libc::c_longlong; /* minwindef.h:219:31 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] pub type PROC = extern "system" fn() -> ::libc::c_longlong; /* minwindef.h:220:26 */
