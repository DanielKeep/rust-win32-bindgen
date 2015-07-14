#[cfg(any(target_arch="x86"))] 
extern "system" {
    pub fn Int64ShllMod32(_: ::winnt::ULONGLONG, _: ::minwindef::DWORD) -> ::winnt::ULONGLONG; /* winnt.h:858:1 */
    pub fn Int64ShraMod32(_: ::winnt::LONGLONG, _: ::minwindef::DWORD) -> ::winnt::LONGLONG; /* winnt.h:865:1 */
    pub fn Int64ShrlMod32(_: ::winnt::ULONGLONG, _: ::minwindef::DWORD) -> ::winnt::ULONGLONG; /* winnt.h:872:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::ua_CharUpperW as ua_CharUpper; /* stralign.h:649:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::ua_lstrcmpW as ua_lstrcmp; /* stralign.h:650:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::ua_lstrcmpiW as ua_lstrcmpi; /* stralign.h:651:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::ua_lstrlenW as ua_lstrlen; /* stralign.h:652:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64", target_arch="arm"))] 
extern "system" {
    pub fn uaw_CharUpperW(_: ::winnt::LPUWSTR) -> ::winnt::LPUWSTR; /* stralign.h:153:1 */
}
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn BemCopyReference(_: *mut ::libc::c_void, _: *mut *mut ::libc::c_void) -> ::winnt::HRESULT; /* bemapiset.h:72:1 */
    pub fn BemCreateContractFrom(_: ::winnt::LPCWSTR, _: *const ::guiddef::GUID, _: *const ::libc::c_void, _: *mut ::libc::c_void, _: *mut *mut ::libc::c_void) -> ::winnt::HRESULT; /* bemapiset.h:61:1 */
    pub fn BemCreateReference(_: *const ::guiddef::GUID, _: *mut ::libc::c_void, _: ::bemapiset::BEM_FREE_INTERFACE_CALLBACK, _: *mut *mut ::libc::c_void) -> ::winnt::HRESULT; /* bemapiset.h:51:1 */
    pub fn BemFreeContract(_: *mut ::libc::c_void); /* bemapiset.h:87:1 */
    pub fn BemFreeReference(_: *mut ::libc::c_void); /* bemapiset.h:80:1 */
    pub fn GetFileVersionInfoExA(_: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winver.h:146:15 */
    pub fn GetFileVersionInfoSizeExA(_: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winver.h:138:16 */
    pub fn wWinMain(_: ::minwindef::HINSTANCE, _: ::minwindef::HINSTANCE, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winbase.h:918:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn WNetRestoreSingleConnectionW(_: ::windef::HWND, _: ::winnt::LPCWSTR, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:277:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn GetDisplayAutoRotationPreferencesByProcessId(_: ::minwindef::DWORD, _: *mut ::winuser::ORIENTATION_PREFERENCE, _: *mut ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:15105:1 */
    pub fn LoadStringByReference(_: ::minwindef::DWORD, _: ::winnt::PCWSTR, _: ::winnt::PCWSTR, _: ::winnt::PWSTR, _: ::minwindef::ULONG, _: ::winnt::PCWSTR, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:1615:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[doc(inline)] pub use self::LookupAccountNameLocalW as LookupAccountNameLocal; /* winbase.h:6542:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn LookupAccountNameLocalA(_: ::winnt::LPCSTR, _: ::winnt::PSID, _: ::minwindef::LPDWORD, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6522:1 */
    pub fn LookupAccountNameLocalW(_: ::winnt::LPCWSTR, _: ::winnt::PSID, _: ::minwindef::LPDWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6533:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[doc(inline)] pub use self::LookupAccountSidLocalW as LookupAccountSidLocal; /* winbase.h:6570:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn LookupAccountSidLocalA(_: ::winnt::PSID, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6550:1 */
    pub fn LookupAccountSidLocalW(_: ::winnt::PSID, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6561:1 */
}
