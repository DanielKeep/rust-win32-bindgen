#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileVersionInfoW as GetFileVersionInfo; /* winver.h:133:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileVersionInfoA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winver.h:117:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileVersionInfoSizeW as GetFileVersionInfoSize; /* winver.h:109:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileVersionInfoSizeA(_: ::winnt::LPCSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winver.h:97:1 */
    pub fn GetFileVersionInfoSizeW(_: ::winnt::LPCWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winver.h:104:1 */
    pub fn GetFileVersionInfoW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winver.h:126:1 */
    pub fn VerFindFileA(_: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::PUINT, _: ::winnt::LPSTR, _: ::minwindef::PUINT) -> ::minwindef::DWORD; /* winver.h:36:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VerInstallFileW as VerInstallFile; /* winver.h:89:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VerInstallFileA(_: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::PUINT) -> ::minwindef::DWORD; /* winver.h:66:1 */
    pub fn VerInstallFileW(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::PUINT) -> ::minwindef::DWORD; /* winver.h:78:1 */
    pub fn VerQueryValueA(_: ::minwindef::LPCVOID, _: ::winnt::LPCSTR, _: *mut *mut ::libc::c_void, _: ::minwindef::PUINT) -> ::minwindef::BOOL; /* winver.h:185:1 */
}
