#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetAddConnectionW as WNetAddConnection; /* winnetwk.h:164:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetAddConnection2W as WNetAddConnection2; /* winnetwk.h:186:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetAddConnection2A(_: ::winnetwk::LPNETRESOURCEA, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:171:1 */
    pub fn WNetAddConnection2W(_: ::winnetwk::LPNETRESOURCEW, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:179:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetAddConnection3W as WNetAddConnection3; /* winnetwk.h:210:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetAddConnection3A(_: ::windef::HWND, _: ::winnetwk::LPNETRESOURCEA, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:193:1 */
    pub fn WNetAddConnection3W(_: ::windef::HWND, _: ::winnetwk::LPNETRESOURCEW, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:202:1 */
    pub fn WNetAddConnectionA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::DWORD; /* winnetwk.h:151:1 */
    pub fn WNetAddConnectionW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::DWORD; /* winnetwk.h:158:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetCancelConnectionW as WNetCancelConnection; /* winnetwk.h:228:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetCancelConnection2W as WNetCancelConnection2; /* winnetwk.h:248:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetCancelConnection2A(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:235:1 */
    pub fn WNetCancelConnection2W(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:242:1 */
    pub fn WNetCancelConnectionA(_: ::winnt::LPCSTR, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:217:1 */
    pub fn WNetCancelConnectionW(_: ::winnt::LPCWSTR, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:223:1 */
    pub fn WNetCloseEnum(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winnetwk.h:490:1 */
    pub fn WNetConnectionDialog(_: ::windef::HWND, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:332:1 */
    pub fn WNetDisconnectDialog(_: ::windef::HWND, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:339:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetEnumResourceW as WNetEnumResource; /* winnetwk.h:483:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetEnumResourceA(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:468:1 */
    pub fn WNetEnumResourceW(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:476:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetConnectionW as WNetGetConnection; /* winnetwk.h:268:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetConnectionA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:255:1 */
    pub fn WNetGetConnectionW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:262:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetLastErrorW as WNetGetLastError; /* winnetwk.h:718:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetLastErrorA(_: ::minwindef::LPDWORD, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:701:1 */
    pub fn WNetGetLastErrorW(_: ::minwindef::LPDWORD, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:710:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetUniversalNameW as WNetGetUniversalName; /* winnetwk.h:594:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetUniversalNameA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:579:1 */
    pub fn WNetGetUniversalNameW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:587:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetUserW as WNetGetUser; /* winnetwk.h:620:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetUserA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:604:1 */
    pub fn WNetGetUserW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:614:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetOpenEnumW as WNetOpenEnum; /* winnetwk.h:461:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetOpenEnumA(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnetwk::LPNETRESOURCEA, _: ::minwindef::LPHANDLE) -> ::minwindef::DWORD; /* winnetwk.h:444:1 */
    pub fn WNetOpenEnumW(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnetwk::LPNETRESOURCEW, _: ::minwindef::LPHANDLE) -> ::minwindef::DWORD; /* winnetwk.h:453:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::MultinetGetConnectionPerformanceW as MultinetGetConnectionPerformance; /* winnetwk.h:818:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn MultinetGetConnectionPerformanceA(_: ::winnetwk::LPNETRESOURCEA, _: ::winnetwk::LPNETCONNECTINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:807:1 */
    pub fn MultinetGetConnectionPerformanceW(_: ::winnetwk::LPNETRESOURCEW, _: ::winnetwk::LPNETCONNECTINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:813:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetConnectionDialog1W as WNetConnectionDialog1; /* winnetwk.h:391:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetConnectionDialog1A(_: ::winnetwk::LPCONNECTDLGSTRUCTA) -> ::minwindef::DWORD; /* winnetwk.h:382:1 */
    pub fn WNetConnectionDialog1W(_: ::winnetwk::LPCONNECTDLGSTRUCTW) -> ::minwindef::DWORD; /* winnetwk.h:387:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetDisconnectDialog1W as WNetDisconnectDialog1; /* winnetwk.h:432:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetDisconnectDialog1A(_: ::winnetwk::LPDISCDLGSTRUCTA) -> ::minwindef::DWORD; /* winnetwk.h:423:1 */
    pub fn WNetDisconnectDialog1W(_: ::winnetwk::LPDISCDLGSTRUCTW) -> ::minwindef::DWORD; /* winnetwk.h:428:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetNetworkInformationW as WNetGetNetworkInformation; /* winnetwk.h:688:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetNetworkInformationA(_: ::winnt::LPCSTR, _: ::winnetwk::LPNETINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:677:1 */
    pub fn WNetGetNetworkInformationW(_: ::winnt::LPCWSTR, _: ::winnetwk::LPNETINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:683:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetProviderNameW as WNetGetProviderName; /* winnetwk.h:655:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetProviderNameA(_: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:642:1 */
    pub fn WNetGetProviderNameW(_: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:649:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetResourceInformationW as WNetGetResourceInformation; /* winnetwk.h:532:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetResourceInformationA(_: ::winnetwk::LPNETRESOURCEA, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD, _: *mut *mut ::libc::c_schar) -> ::minwindef::DWORD; /* winnetwk.h:517:1 */
    pub fn WNetGetResourceInformationW(_: ::winnetwk::LPNETRESOURCEW, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD, _: *mut *mut ::libc::c_ushort) -> ::minwindef::DWORD; /* winnetwk.h:525:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetResourceParentW as WNetGetResourceParent; /* winnetwk.h:510:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetResourceParentA(_: ::winnetwk::LPNETRESOURCEA, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:497:1 */
    pub fn WNetGetResourceParentW(_: ::winnetwk::LPNETRESOURCEW, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:504:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetUseConnectionW as WNetUseConnection; /* winnetwk.h:320:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetUseConnectionA(_: ::windef::HWND, _: ::winnetwk::LPNETRESOURCEA, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:297:1 */
    pub fn WNetUseConnectionW(_: ::windef::HWND, _: ::winnetwk::LPNETRESOURCEW, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:309:1 */
}
