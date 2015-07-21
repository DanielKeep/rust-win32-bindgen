#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetAddConnectionW as WNetAddConnection; /* winnetwk.h:164:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetAddConnection2W as WNetAddConnection2; /* winnetwk.h:186:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetAddConnection2A(lpNetResource: ::winnetwk::LPNETRESOURCEA, lpPassword: ::winnt::LPCSTR, lpUserName: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:171:1 */
    pub fn WNetAddConnection2W(lpNetResource: ::winnetwk::LPNETRESOURCEW, lpPassword: ::winnt::LPCWSTR, lpUserName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:179:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetAddConnection3W as WNetAddConnection3; /* winnetwk.h:210:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetAddConnection3A(hwndOwner: ::windef::HWND, lpNetResource: ::winnetwk::LPNETRESOURCEA, lpPassword: ::winnt::LPCSTR, lpUserName: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:193:1 */
    pub fn WNetAddConnection3W(hwndOwner: ::windef::HWND, lpNetResource: ::winnetwk::LPNETRESOURCEW, lpPassword: ::winnt::LPCWSTR, lpUserName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:202:1 */
    pub fn WNetAddConnectionA(lpRemoteName: ::winnt::LPCSTR, lpPassword: ::winnt::LPCSTR, lpLocalName: ::winnt::LPCSTR) -> ::minwindef::DWORD; /* winnetwk.h:151:1 */
    pub fn WNetAddConnectionW(lpRemoteName: ::winnt::LPCWSTR, lpPassword: ::winnt::LPCWSTR, lpLocalName: ::winnt::LPCWSTR) -> ::minwindef::DWORD; /* winnetwk.h:158:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetCancelConnectionW as WNetCancelConnection; /* winnetwk.h:228:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetCancelConnection2W as WNetCancelConnection2; /* winnetwk.h:248:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetCancelConnection2A(lpName: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD, fForce: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:235:1 */
    pub fn WNetCancelConnection2W(lpName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, fForce: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:242:1 */
    pub fn WNetCancelConnectionA(lpName: ::winnt::LPCSTR, fForce: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:217:1 */
    pub fn WNetCancelConnectionW(lpName: ::winnt::LPCWSTR, fForce: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:223:1 */
    pub fn WNetCloseEnum(hEnum: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winnetwk.h:490:1 */
    pub fn WNetConnectionDialog(hwnd: ::windef::HWND, dwType: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:332:1 */
    pub fn WNetDisconnectDialog(hwnd: ::windef::HWND, dwType: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:339:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetEnumResourceW as WNetEnumResource; /* winnetwk.h:483:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetEnumResourceA(hEnum: ::winnt::HANDLE, lpcCount: ::minwindef::LPDWORD, lpBuffer: ::minwindef::LPVOID, lpBufferSize: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:468:1 */
    pub fn WNetEnumResourceW(hEnum: ::winnt::HANDLE, lpcCount: ::minwindef::LPDWORD, lpBuffer: ::minwindef::LPVOID, lpBufferSize: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:476:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetConnectionW as WNetGetConnection; /* winnetwk.h:268:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetConnectionA(lpLocalName: ::winnt::LPCSTR, lpRemoteName: ::winnt::LPSTR, lpnLength: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:255:1 */
    pub fn WNetGetConnectionW(lpLocalName: ::winnt::LPCWSTR, lpRemoteName: ::winnt::LPWSTR, lpnLength: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:262:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetLastErrorW as WNetGetLastError; /* winnetwk.h:718:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetLastErrorA(lpError: ::minwindef::LPDWORD, lpErrorBuf: ::winnt::LPSTR, nErrorBufSize: ::minwindef::DWORD, lpNameBuf: ::winnt::LPSTR, nNameBufSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:701:1 */
    pub fn WNetGetLastErrorW(lpError: ::minwindef::LPDWORD, lpErrorBuf: ::winnt::LPWSTR, nErrorBufSize: ::minwindef::DWORD, lpNameBuf: ::winnt::LPWSTR, nNameBufSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:710:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetUniversalNameW as WNetGetUniversalName; /* winnetwk.h:594:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetUniversalNameA(lpLocalPath: ::winnt::LPCSTR, dwInfoLevel: ::minwindef::DWORD, lpBuffer: ::minwindef::LPVOID, lpBufferSize: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:579:1 */
    pub fn WNetGetUniversalNameW(lpLocalPath: ::winnt::LPCWSTR, dwInfoLevel: ::minwindef::DWORD, lpBuffer: ::minwindef::LPVOID, lpBufferSize: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:587:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetUserW as WNetGetUser; /* winnetwk.h:620:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetUserA(lpName: ::winnt::LPCSTR, lpUserName: ::winnt::LPSTR, lpnLength: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:604:1 */
    pub fn WNetGetUserW(lpName: ::winnt::LPCWSTR, lpUserName: ::winnt::LPWSTR, lpnLength: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:614:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetOpenEnumW as WNetOpenEnum; /* winnetwk.h:461:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetOpenEnumA(dwScope: ::minwindef::DWORD, dwType: ::minwindef::DWORD, dwUsage: ::minwindef::DWORD, lpNetResource: ::winnetwk::LPNETRESOURCEA, lphEnum: ::minwindef::LPHANDLE) -> ::minwindef::DWORD; /* winnetwk.h:444:1 */
    pub fn WNetOpenEnumW(dwScope: ::minwindef::DWORD, dwType: ::minwindef::DWORD, dwUsage: ::minwindef::DWORD, lpNetResource: ::winnetwk::LPNETRESOURCEW, lphEnum: ::minwindef::LPHANDLE) -> ::minwindef::DWORD; /* winnetwk.h:453:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::MultinetGetConnectionPerformanceW as MultinetGetConnectionPerformance; /* winnetwk.h:818:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn MultinetGetConnectionPerformanceA(lpNetResource: ::winnetwk::LPNETRESOURCEA, lpNetConnectInfoStruct: ::winnetwk::LPNETCONNECTINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:807:1 */
    pub fn MultinetGetConnectionPerformanceW(lpNetResource: ::winnetwk::LPNETRESOURCEW, lpNetConnectInfoStruct: ::winnetwk::LPNETCONNECTINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:813:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetConnectionDialog1W as WNetConnectionDialog1; /* winnetwk.h:391:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetConnectionDialog1A(lpConnDlgStruct: ::winnetwk::LPCONNECTDLGSTRUCTA) -> ::minwindef::DWORD; /* winnetwk.h:382:1 */
    pub fn WNetConnectionDialog1W(lpConnDlgStruct: ::winnetwk::LPCONNECTDLGSTRUCTW) -> ::minwindef::DWORD; /* winnetwk.h:387:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetDisconnectDialog1W as WNetDisconnectDialog1; /* winnetwk.h:432:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetDisconnectDialog1A(lpConnDlgStruct: ::winnetwk::LPDISCDLGSTRUCTA) -> ::minwindef::DWORD; /* winnetwk.h:423:1 */
    pub fn WNetDisconnectDialog1W(lpConnDlgStruct: ::winnetwk::LPDISCDLGSTRUCTW) -> ::minwindef::DWORD; /* winnetwk.h:428:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetNetworkInformationW as WNetGetNetworkInformation; /* winnetwk.h:688:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetNetworkInformationA(lpProvider: ::winnt::LPCSTR, lpNetInfoStruct: ::winnetwk::LPNETINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:677:1 */
    pub fn WNetGetNetworkInformationW(lpProvider: ::winnt::LPCWSTR, lpNetInfoStruct: ::winnetwk::LPNETINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:683:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetProviderNameW as WNetGetProviderName; /* winnetwk.h:655:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetProviderNameA(dwNetType: ::minwindef::DWORD, lpProviderName: ::winnt::LPSTR, lpBufferSize: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:642:1 */
    pub fn WNetGetProviderNameW(dwNetType: ::minwindef::DWORD, lpProviderName: ::winnt::LPWSTR, lpBufferSize: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:649:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetResourceInformationW as WNetGetResourceInformation; /* winnetwk.h:532:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetResourceInformationA(lpNetResource: ::winnetwk::LPNETRESOURCEA, lpBuffer: ::minwindef::LPVOID, lpcbBuffer: ::minwindef::LPDWORD, lplpSystem: *mut *mut ::libc::c_schar) -> ::minwindef::DWORD; /* winnetwk.h:517:1 */
    pub fn WNetGetResourceInformationW(lpNetResource: ::winnetwk::LPNETRESOURCEW, lpBuffer: ::minwindef::LPVOID, lpcbBuffer: ::minwindef::LPDWORD, lplpSystem: *mut *mut ::libc::c_ushort) -> ::minwindef::DWORD; /* winnetwk.h:525:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetResourceParentW as WNetGetResourceParent; /* winnetwk.h:510:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetResourceParentA(lpNetResource: ::winnetwk::LPNETRESOURCEA, lpBuffer: ::minwindef::LPVOID, lpcbBuffer: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:497:1 */
    pub fn WNetGetResourceParentW(lpNetResource: ::winnetwk::LPNETRESOURCEW, lpBuffer: ::minwindef::LPVOID, lpcbBuffer: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:504:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetUseConnectionW as WNetUseConnection; /* winnetwk.h:320:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetUseConnectionA(hwndOwner: ::windef::HWND, lpNetResource: ::winnetwk::LPNETRESOURCEA, lpPassword: ::winnt::LPCSTR, lpUserId: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD, lpAccessName: ::winnt::LPSTR, lpBufferSize: ::minwindef::LPDWORD, lpResult: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:297:1 */
    pub fn WNetUseConnectionW(hwndOwner: ::windef::HWND, lpNetResource: ::winnetwk::LPNETRESOURCEW, lpPassword: ::winnt::LPCWSTR, lpUserId: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, lpAccessName: ::winnt::LPWSTR, lpBufferSize: ::minwindef::LPDWORD, lpResult: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:309:1 */
}
