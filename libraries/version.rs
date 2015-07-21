#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileVersionInfoW as GetFileVersionInfo; /* winver.h:133:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileVersionInfoA(lptstrFilename: ::winnt::LPCSTR, dwHandle: ::minwindef::DWORD, dwLen: ::minwindef::DWORD, lpData: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winver.h:117:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileVersionInfoSizeW as GetFileVersionInfoSize; /* winver.h:109:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileVersionInfoSizeA(lptstrFilename: ::winnt::LPCSTR, lpdwHandle: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winver.h:97:1 */
    pub fn GetFileVersionInfoSizeW(lptstrFilename: ::winnt::LPCWSTR, lpdwHandle: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winver.h:104:1 */
    pub fn GetFileVersionInfoW(lptstrFilename: ::winnt::LPCWSTR, dwHandle: ::minwindef::DWORD, dwLen: ::minwindef::DWORD, lpData: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winver.h:126:1 */
    pub fn VerFindFileA(uFlags: ::minwindef::DWORD, szFileName: ::winnt::LPCSTR, szWinDir: ::winnt::LPCSTR, szAppDir: ::winnt::LPCSTR, szCurDir: ::winnt::LPSTR, puCurDirLen: ::minwindef::PUINT, szDestDir: ::winnt::LPSTR, puDestDirLen: ::minwindef::PUINT) -> ::minwindef::DWORD; /* winver.h:36:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VerInstallFileW as VerInstallFile; /* winver.h:89:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VerInstallFileA(uFlags: ::minwindef::DWORD, szSrcFileName: ::winnt::LPCSTR, szDestFileName: ::winnt::LPCSTR, szSrcDir: ::winnt::LPCSTR, szDestDir: ::winnt::LPCSTR, szCurDir: ::winnt::LPCSTR, szTmpFile: ::winnt::LPSTR, puTmpFileLen: ::minwindef::PUINT) -> ::minwindef::DWORD; /* winver.h:66:1 */
    pub fn VerInstallFileW(uFlags: ::minwindef::DWORD, szSrcFileName: ::winnt::LPCWSTR, szDestFileName: ::winnt::LPCWSTR, szSrcDir: ::winnt::LPCWSTR, szDestDir: ::winnt::LPCWSTR, szCurDir: ::winnt::LPCWSTR, szTmpFile: ::winnt::LPWSTR, puTmpFileLen: ::minwindef::PUINT) -> ::minwindef::DWORD; /* winver.h:78:1 */
    pub fn VerQueryValueA(pBlock: ::minwindef::LPCVOID, lpSubBlock: ::winnt::LPCSTR, lplpBuffer: *mut *mut ::libc::c_void, puLen: ::minwindef::PUINT) -> ::minwindef::BOOL; /* winver.h:185:1 */
}
