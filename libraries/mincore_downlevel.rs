#[doc(inline)] pub use self::CharLowerA as AnsiLower; /* winuser.h:5512:9 */
#[doc(inline)] pub use self::CharLowerBuffA as AnsiLowerBuff; /* winuser.h:5513:9 */
#[doc(inline)] pub use self::CharNextA as AnsiNext; /* winuser.h:5514:9 */
#[doc(inline)] pub use self::CharPrevA as AnsiPrev; /* winuser.h:5515:9 */
#[doc(inline)] pub use self::CharUpperA as AnsiUpper; /* winuser.h:5510:9 */
#[doc(inline)] pub use self::CharUpperBuffA as AnsiUpperBuff; /* winuser.h:5511:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::CharUpperW as ua_CharUpperW; /* stralign.h:93:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharLowerW as CharLower; /* winuser.h:5424:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharLowerA(lpsz: ::winnt::LPSTR) -> ::winnt::LPSTR; /* winuser.h:5416:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharLowerBuffW as CharLowerBuff; /* winuser.h:5442:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharLowerBuffA(lpsz: ::winnt::LPSTR, cchLength: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:5432:1 */
    pub fn CharLowerBuffW(lpsz: ::winnt::LPWSTR, cchLength: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:5438:1 */
    pub fn CharLowerW(lpsz: ::winnt::LPWSTR) -> ::winnt::LPWSTR; /* winuser.h:5421:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharNextW as CharNext; /* winuser.h:5458:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharNextA(lpsz: ::winnt::LPCSTR) -> ::winnt::LPSTR; /* winuser.h:5450:1 */
    pub fn CharNextW(lpsz: ::winnt::LPCWSTR) -> ::winnt::LPWSTR; /* winuser.h:5455:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharPrevW as CharPrev; /* winuser.h:5476:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharPrevA(lpszStart: ::winnt::LPCSTR, lpszCurrent: ::winnt::LPCSTR) -> ::winnt::LPSTR; /* winuser.h:5466:1 */
    pub fn CharPrevW(lpszStart: ::winnt::LPCWSTR, lpszCurrent: ::winnt::LPCWSTR) -> ::winnt::LPWSTR; /* winuser.h:5472:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharUpperW as CharUpper; /* winuser.h:5390:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharUpperA(lpsz: ::winnt::LPSTR) -> ::winnt::LPSTR; /* winuser.h:5382:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharUpperBuffW as CharUpperBuff; /* winuser.h:5408:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharUpperBuffA(lpsz: ::winnt::LPSTR, cchLength: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:5398:1 */
    pub fn CharUpperBuffW(lpsz: ::winnt::LPWSTR, cchLength: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:5404:1 */
    pub fn CharUpperW(lpsz: ::winnt::LPWSTR) -> ::winnt::LPWSTR; /* winuser.h:5387:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileVersionInfoExW as GetFileVersionInfoEx; /* winver.h:157:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileVersionInfoExW(dwFlags: ::minwindef::DWORD, lpwstrFilename: ::winnt::LPCWSTR, dwHandle: ::minwindef::DWORD, dwLen: ::minwindef::DWORD, lpData: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winver.h:151:15 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileVersionInfoSizeExW as GetFileVersionInfoSizeEx; /* winver.h:141:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileVersionInfoSizeExW(dwFlags: ::minwindef::DWORD, lpwstrFilename: ::winnt::LPCWSTR, lpdwHandle: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winver.h:139:16 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsCharAlphaW as IsCharAlpha; /* winuser.h:5536:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn IsCharAlphaA(ch: ::winnt::CHAR) -> ::minwindef::BOOL; /* winuser.h:5528:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsCharAlphaNumericW as IsCharAlphaNumeric; /* winuser.h:5552:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn IsCharAlphaNumericA(ch: ::winnt::CHAR) -> ::minwindef::BOOL; /* winuser.h:5544:1 */
    pub fn IsCharAlphaNumericW(ch: ::winnt::WCHAR) -> ::minwindef::BOOL; /* winuser.h:5549:1 */
    pub fn IsCharAlphaW(ch: ::winnt::WCHAR) -> ::minwindef::BOOL; /* winuser.h:5533:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsCharLowerW as IsCharLower; /* winuser.h:5584:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn IsCharLowerA(ch: ::winnt::CHAR) -> ::minwindef::BOOL; /* winuser.h:5576:1 */
    pub fn IsCharLowerW(ch: ::winnt::WCHAR) -> ::minwindef::BOOL; /* winuser.h:5581:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsCharUpperW as IsCharUpper; /* winuser.h:5568:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn IsCharUpperA(ch: ::winnt::CHAR) -> ::minwindef::BOOL; /* winuser.h:5560:1 */
    pub fn IsCharUpperW(ch: ::winnt::WCHAR) -> ::minwindef::BOOL; /* winuser.h:5565:1 */
    pub fn RevertToSelf() -> ::minwindef::BOOL; /* securitybaseapi.h:1149:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VerFindFileW as VerFindFile; /* winver.h:59:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VerFindFileA(uFlags: ::minwindef::DWORD, szFileName: ::winnt::LPCSTR, szWinDir: ::winnt::LPCSTR, szAppDir: ::winnt::LPCSTR, szCurDir: ::winnt::LPSTR, puCurDirLen: ::minwindef::PUINT, szDestDir: ::winnt::LPSTR, puDestDirLen: ::minwindef::PUINT) -> ::minwindef::DWORD; /* winver.h:36:1 */
    pub fn VerFindFileW(uFlags: ::minwindef::DWORD, szFileName: ::winnt::LPCWSTR, szWinDir: ::winnt::LPCWSTR, szAppDir: ::winnt::LPCWSTR, szCurDir: ::winnt::LPWSTR, puCurDirLen: ::minwindef::PUINT, szDestDir: ::winnt::LPWSTR, puDestDirLen: ::minwindef::PUINT) -> ::minwindef::DWORD; /* winver.h:48:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VerQueryValueW as VerQueryValue; /* winver.h:200:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VerQueryValueA(pBlock: ::minwindef::LPCVOID, lpSubBlock: ::winnt::LPCSTR, lplpBuffer: *mut *mut ::libc::c_void, puLen: ::minwindef::PUINT) -> ::minwindef::BOOL; /* winver.h:185:1 */
    pub fn VerQueryValueW(pBlock: ::minwindef::LPCVOID, lpSubBlock: ::winnt::LPCWSTR, lplpBuffer: *mut *mut ::libc::c_void, puLen: ::minwindef::PUINT) -> ::minwindef::BOOL; /* winver.h:193:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn CharNextExA(CodePage: ::minwindef::WORD, lpCurrentChar: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD) -> ::winnt::LPSTR; /* winuser.h:5485:1 */
    pub fn CharPrevExA(CodePage: ::minwindef::WORD, lpStart: ::winnt::LPCSTR, lpCurrentChar: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD) -> ::winnt::LPSTR; /* winuser.h:5493:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn IdnToAscii(dwFlags: ::minwindef::DWORD, lpUnicodeCharStr: ::winnt::LPCWSTR, cchUnicodeChar: ::libc::c_int, lpASCIICharStr: ::winnt::LPWSTR, cchASCIIChar: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2312:8 */
    pub fn IdnToUnicode(dwFlags: ::minwindef::DWORD, lpASCIICharStr: ::winnt::LPCWSTR, cchASCIIChar: ::libc::c_int, lpUnicodeCharStr: ::winnt::LPWSTR, cchUnicodeChar: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2328:8 */
}
