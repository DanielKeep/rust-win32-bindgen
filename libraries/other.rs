#[cfg(any(target_arch="x86"))] 
extern "system" {
    pub fn Int64ShllMod32(Value: ::winnt::ULONGLONG, ShiftCount: ::minwindef::DWORD) -> ::winnt::ULONGLONG; /* winnt.h:858:1 */
    pub fn Int64ShraMod32(Value: ::winnt::LONGLONG, ShiftCount: ::minwindef::DWORD) -> ::winnt::LONGLONG; /* winnt.h:865:1 */
    pub fn Int64ShrlMod32(Value: ::winnt::ULONGLONG, ShiftCount: ::minwindef::DWORD) -> ::winnt::ULONGLONG; /* winnt.h:872:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::ua_CharUpperW as ua_CharUpper; /* stralign.h:649:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::ua_lstrcmpW as ua_lstrcmp; /* stralign.h:650:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::ua_lstrcmpiW as ua_lstrcmpi; /* stralign.h:651:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::ua_lstrlenW as ua_lstrlen; /* stralign.h:652:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64", target_arch="arm"))] 
extern "system" {
    pub fn uaw_CharUpperW(String: ::winnt::LPUWSTR) -> ::winnt::LPUWSTR; /* stralign.h:153:1 */
}
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn BemCopyReference(reference: *mut ::libc::c_void, copiedReference: *mut *mut ::libc::c_void) -> ::winnt::HRESULT; /* bemapiset.h:72:1 */
    pub fn BemCreateContractFrom(dllPath: ::winnt::LPCWSTR, extensionId: *const ::guiddef::GUID, contractDescription: *const ::libc::c_void, hostContract: *mut ::libc::c_void, contract: *mut *mut ::libc::c_void) -> ::winnt::HRESULT; /* bemapiset.h:61:1 */
    pub fn BemCreateReference(iid: *const ::guiddef::GUID, interfaceInstance: *mut ::libc::c_void, freeCallback: ::bemapiset::BEM_FREE_INTERFACE_CALLBACK, reference: *mut *mut ::libc::c_void) -> ::winnt::HRESULT; /* bemapiset.h:51:1 */
    pub fn BemFreeContract(contract: *mut ::libc::c_void); /* bemapiset.h:87:1 */
    pub fn BemFreeReference(reference: *mut ::libc::c_void); /* bemapiset.h:80:1 */
    pub fn GetFileVersionInfoExA(dwFlags: ::minwindef::DWORD, lpwstrFilename: ::winnt::LPCSTR, dwHandle: ::minwindef::DWORD, dwLen: ::minwindef::DWORD, lpData: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winver.h:146:15 */
    pub fn GetFileVersionInfoSizeExA(dwFlags: ::minwindef::DWORD, lpwstrFilename: ::winnt::LPCSTR, lpdwHandle: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winver.h:138:16 */
    pub fn wWinMain(hInstance: ::minwindef::HINSTANCE, hPrevInstance: ::minwindef::HINSTANCE, lpCmdLine: ::winnt::LPWSTR, nShowCmd: ::libc::c_int) -> ::libc::c_int; /* winbase.h:918:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn WNetRestoreSingleConnectionW(hwndParent: ::windef::HWND, lpDevice: ::winnt::LPCWSTR, fUseUI: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:277:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn GetDisplayAutoRotationPreferencesByProcessId(dwProcessId: ::minwindef::DWORD, pOrientation: *mut ::winuser::ORIENTATION_PREFERENCE, fRotateScreen: *mut ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:15105:1 */
    pub fn LoadStringByReference(Flags: ::minwindef::DWORD, Language: ::winnt::PCWSTR, SourceString: ::winnt::PCWSTR, Buffer: ::winnt::PWSTR, cchBuffer: ::minwindef::ULONG, Directory: ::winnt::PCWSTR, pcchBufferOut: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:1615:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[doc(inline)] pub use self::LookupAccountNameLocalW as LookupAccountNameLocal; /* winbase.h:6542:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn LookupAccountNameLocalA(lpAccountName: ::winnt::LPCSTR, Sid: ::winnt::PSID, cbSid: ::minwindef::LPDWORD, ReferencedDomainName: ::winnt::LPSTR, cchReferencedDomainName: ::minwindef::LPDWORD, peUse: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6522:1 */
    pub fn LookupAccountNameLocalW(lpAccountName: ::winnt::LPCWSTR, Sid: ::winnt::PSID, cbSid: ::minwindef::LPDWORD, ReferencedDomainName: ::winnt::LPWSTR, cchReferencedDomainName: ::minwindef::LPDWORD, peUse: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6533:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[doc(inline)] pub use self::LookupAccountSidLocalW as LookupAccountSidLocal; /* winbase.h:6570:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn LookupAccountSidLocalA(Sid: ::winnt::PSID, Name: ::winnt::LPSTR, cchName: ::minwindef::LPDWORD, ReferencedDomainName: ::winnt::LPSTR, cchReferencedDomainName: ::minwindef::LPDWORD, peUse: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6550:1 */
    pub fn LookupAccountSidLocalW(Sid: ::winnt::PSID, Name: ::winnt::LPWSTR, cchName: ::minwindef::LPDWORD, ReferencedDomainName: ::winnt::LPWSTR, cchReferencedDomainName: ::minwindef::LPDWORD, peUse: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6561:1 */
}
