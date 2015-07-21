#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct ENUMUILANG { NumOfEnumUILang: ::minwindef::ULONG, SizeOfEnumUIBuffer: ::minwindef::ULONG, pEnumUIBuffer: *mut ::libc::c_ushort } /* libloaderapi.h:59:16, libloaderapi.h:59:16, libloaderapi.h:59:16 */
#[cfg(feature="winapi_desktop")] pub type PENUMUILANG = *mut ::libloaderapi::ENUMUILANG; /* libloaderapi.h:63:16, libloaderapi.h:63:16, libloaderapi.h:63:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type ENUMRESLANGPROCA = Option<extern "system" fn(*mut ::minwindef::HINSTANCE__, *const ::libc::c_schar, *const ::libc::c_schar, ::libc::c_ushort, ::libc::c_long) -> ::libc::c_int>; /* libloaderapi.h:67:25, libloaderapi.h:67:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type ENUMRESLANGPROCW = Option<extern "system" fn(*mut ::minwindef::HINSTANCE__, *const ::libc::c_ushort, *const ::libc::c_ushort, ::libc::c_ushort, ::libc::c_long) -> ::libc::c_int>; /* libloaderapi.h:73:25, libloaderapi.h:73:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type ENUMRESNAMEPROCA = Option<extern "system" fn(*mut ::minwindef::HINSTANCE__, *const ::libc::c_schar, *mut ::libc::c_schar, ::libc::c_long) -> ::libc::c_int>; /* libloaderapi.h:85:25, libloaderapi.h:85:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type ENUMRESNAMEPROCW = Option<extern "system" fn(*mut ::minwindef::HINSTANCE__, *const ::libc::c_ushort, *mut ::libc::c_ushort, ::libc::c_long) -> ::libc::c_int>; /* libloaderapi.h:90:25, libloaderapi.h:90:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type ENUMRESTYPEPROCA = Option<extern "system" fn(*mut ::minwindef::HINSTANCE__, *mut ::libc::c_schar, ::libc::c_long) -> ::libc::c_int>; /* libloaderapi.h:101:25, libloaderapi.h:101:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type ENUMRESTYPEPROCW = Option<extern "system" fn(*mut ::minwindef::HINSTANCE__, *mut ::libc::c_ushort, ::libc::c_long) -> ::libc::c_int>; /* libloaderapi.h:106:25, libloaderapi.h:106:25 */
#[cfg(feature="winapi_desktop")] pub type PGET_MODULE_HANDLE_EXA = Option<extern "system" fn(::libc::c_ulong, *const ::libc::c_schar, *mut *mut ::minwindef::HINSTANCE__) -> ::libc::c_int>; /* libloaderapi.h:305:1, libloaderapi.h:305:1, libloaderapi.h:305:1 */
#[cfg(feature="winapi_desktop")] pub type PGET_MODULE_HANDLE_EXW = Option<extern "system" fn(::libc::c_ulong, *const ::libc::c_ushort, *mut *mut ::minwindef::HINSTANCE__) -> ::libc::c_int>; /* libloaderapi.h:313:1, libloaderapi.h:313:1, libloaderapi.h:313:1 */
#[cfg(feature="winapi_desktop")] pub type DLL_DIRECTORY_COOKIE = ::winnt::PVOID; /* libloaderapi.h:493:15, libloaderapi.h:493:15, libloaderapi.h:493:15 */
#[cfg(feature="winapi_desktop")] pub type PDLL_DIRECTORY_COOKIE = *mut *mut ::libc::c_void; /* libloaderapi.h:493:38, libloaderapi.h:493:38, libloaderapi.h:493:38 */
#[cfg(feature="winapi_desktop")] pub const SUPPORT_LANG_NUMBER: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* libloaderapi.h:57:9, libloaderapi.h:57:9, libloaderapi.h:57:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::libloaderapi::ENUMRESLANGPROCW as ENUMRESLANGPROC; /* libloaderapi.h:80:9, libloaderapi.h:80:9, libloaderapi.h:80:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::libloaderapi::ENUMRESNAMEPROCW as ENUMRESNAMEPROC; /* libloaderapi.h:96:9, libloaderapi.h:96:9, libloaderapi.h:96:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::libloaderapi::ENUMRESTYPEPROCW as ENUMRESTYPEPROC; /* libloaderapi.h:112:9, libloaderapi.h:112:9, libloaderapi.h:112:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::libloaderapi::PGET_MODULE_HANDLE_EXW as PGET_MODULE_HANDLE_EX; /* libloaderapi.h:319:9, libloaderapi.h:319:9, libloaderapi.h:319:9 */
#[cfg(feature="winapi_desktop")] pub const DONT_RESOLVE_DLL_REFERENCES: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* libloaderapi.h:399:9, libloaderapi.h:399:9, libloaderapi.h:399:9 */
#[cfg(feature="winapi_desktop")] pub const LOAD_LIBRARY_AS_DATAFILE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* libloaderapi.h:400:9, libloaderapi.h:400:9, libloaderapi.h:400:9 */
#[cfg(feature="winapi_desktop")] pub const LOAD_WITH_ALTERED_SEARCH_PATH: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* libloaderapi.h:402:9, libloaderapi.h:402:9, libloaderapi.h:402:9 */
#[cfg(feature="winapi_desktop")] pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* libloaderapi.h:403:9, libloaderapi.h:403:9, libloaderapi.h:403:9 */
#[cfg(feature="winapi_desktop")] pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* libloaderapi.h:404:9, libloaderapi.h:404:9, libloaderapi.h:404:9 */
#[cfg(feature="winapi_desktop")] pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* libloaderapi.h:405:9, libloaderapi.h:405:9, libloaderapi.h:405:9 */
#[cfg(feature="winapi_desktop")] pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* libloaderapi.h:406:9, libloaderapi.h:406:9, libloaderapi.h:406:9 */
#[cfg(feature="winapi_desktop")] pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* libloaderapi.h:407:9, libloaderapi.h:407:9, libloaderapi.h:407:9 */
#[cfg(feature="winapi_desktop")] pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* libloaderapi.h:408:9, libloaderapi.h:408:9, libloaderapi.h:408:9 */
#[cfg(feature="winapi_desktop")] pub const LOAD_LIBRARY_SEARCH_USER_DIRS: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* libloaderapi.h:409:9, libloaderapi.h:409:9, libloaderapi.h:409:9 */
#[cfg(feature="winapi_desktop")] pub const LOAD_LIBRARY_SEARCH_SYSTEM32: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* libloaderapi.h:410:9, libloaderapi.h:410:9, libloaderapi.h:410:9 */
#[cfg(feature="winapi_desktop")] pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* libloaderapi.h:411:9, libloaderapi.h:411:9, libloaderapi.h:411:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type ENUMRESLANGPROCA = Option<extern "system" fn(*mut ::minwindef::HINSTANCE__, *const ::libc::c_schar, *const ::libc::c_schar, ::libc::c_ushort, ::libc::c_longlong) -> ::libc::c_int>; /* libloaderapi.h:67:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type ENUMRESLANGPROCW = Option<extern "system" fn(*mut ::minwindef::HINSTANCE__, *const ::libc::c_ushort, *const ::libc::c_ushort, ::libc::c_ushort, ::libc::c_longlong) -> ::libc::c_int>; /* libloaderapi.h:73:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type ENUMRESNAMEPROCA = Option<extern "system" fn(*mut ::minwindef::HINSTANCE__, *const ::libc::c_schar, *mut ::libc::c_schar, ::libc::c_longlong) -> ::libc::c_int>; /* libloaderapi.h:85:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type ENUMRESNAMEPROCW = Option<extern "system" fn(*mut ::minwindef::HINSTANCE__, *const ::libc::c_ushort, *mut ::libc::c_ushort, ::libc::c_longlong) -> ::libc::c_int>; /* libloaderapi.h:90:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type ENUMRESTYPEPROCA = Option<extern "system" fn(*mut ::minwindef::HINSTANCE__, *mut ::libc::c_schar, ::libc::c_longlong) -> ::libc::c_int>; /* libloaderapi.h:101:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type ENUMRESTYPEPROCW = Option<extern "system" fn(*mut ::minwindef::HINSTANCE__, *mut ::libc::c_ushort, ::libc::c_longlong) -> ::libc::c_int>; /* libloaderapi.h:106:25 */
