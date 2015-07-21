#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmAssociateContext(_: ::windef::HWND, _: ::imm::HIMC) -> ::imm::HIMC; /* imm.h:265:13 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmConfigureIMEW as ImmConfigureIME; /* imm.h:336:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmConfigureIMEA(_: ::minwindef::HKL, _: ::windef::HWND, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* imm.h:333:16 */
    pub fn ImmConfigureIMEW(_: ::minwindef::HKL, _: ::windef::HWND, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* imm.h:334:16 */
    pub fn ImmCreateContext() -> ::imm::HIMC; /* imm.h:261:13 */
    pub fn ImmDestroyContext(_: ::imm::HIMC) -> ::minwindef::BOOL; /* imm.h:262:13 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmEnumRegisterWordW as ImmEnumRegisterWord; /* imm.h:412:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmEnumRegisterWordA(_: ::minwindef::HKL, _: ::imm::REGISTERWORDENUMPROCA, lpszReading: ::winnt::LPCSTR, _: ::minwindef::DWORD, lpszRegister: ::winnt::LPCSTR, _: ::minwindef::LPVOID) -> ::minwindef::UINT; /* imm.h:409:13 */
    pub fn ImmEnumRegisterWordW(_: ::minwindef::HKL, _: ::imm::REGISTERWORDENUMPROCW, lpszReading: ::winnt::LPCWSTR, _: ::minwindef::DWORD, lpszRegister: ::winnt::LPCWSTR, _: ::minwindef::LPVOID) -> ::minwindef::UINT; /* imm.h:410:13 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmEscapeW as ImmEscape; /* imm.h:344:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmEscapeA(_: ::minwindef::HKL, _: ::imm::HIMC, _: ::minwindef::UINT, _: ::minwindef::LPVOID) -> ::minwindef::LRESULT; /* imm.h:341:16 */
    pub fn ImmEscapeW(_: ::minwindef::HKL, _: ::imm::HIMC, _: ::minwindef::UINT, _: ::minwindef::LPVOID) -> ::minwindef::LRESULT; /* imm.h:342:16 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmGetCandidateListW as ImmGetCandidateList; /* imm.h:297:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmGetCandidateListA(_: ::imm::HIMC, deIndex: ::minwindef::DWORD, lpCandList: ::imm::LPCANDIDATELIST, dwBufLen: ::minwindef::DWORD) -> ::minwindef::DWORD; /* imm.h:294:14 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmGetCandidateListCountW as ImmGetCandidateListCount; /* imm.h:289:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmGetCandidateListCountA(_: ::imm::HIMC, lpdwListCount: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* imm.h:286:14 */
    pub fn ImmGetCandidateListCountW(_: ::imm::HIMC, lpdwListCount: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* imm.h:287:14 */
    pub fn ImmGetCandidateListW(_: ::imm::HIMC, deIndex: ::minwindef::DWORD, lpCandList: ::imm::LPCANDIDATELIST, dwBufLen: ::minwindef::DWORD) -> ::minwindef::DWORD; /* imm.h:295:14 */
    pub fn ImmGetCandidateWindow(_: ::imm::HIMC, _: ::minwindef::DWORD, lpCandidate: ::imm::LPCANDIDATEFORM) -> ::minwindef::BOOL; /* imm.h:363:13 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmGetCompositionFontW as ImmGetCompositionFont; /* imm.h:319:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmGetCompositionFontA(_: ::imm::HIMC, lplf: ::wingdi::LPLOGFONTA) -> ::minwindef::BOOL; /* imm.h:316:13 */
    pub fn ImmGetCompositionFontW(_: ::imm::HIMC, lplf: ::wingdi::LPLOGFONTW) -> ::minwindef::BOOL; /* imm.h:317:13 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmGetCompositionStringW as ImmGetCompositionString; /* imm.h:273:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmGetCompositionStringA(_: ::imm::HIMC, _: ::minwindef::DWORD, lpBuf: ::minwindef::LPVOID, dwBufLen: ::minwindef::DWORD) -> ::winnt::LONG; /* imm.h:270:14 */
    pub fn ImmGetCompositionStringW(_: ::imm::HIMC, _: ::minwindef::DWORD, lpBuf: ::minwindef::LPVOID, dwBufLen: ::minwindef::DWORD) -> ::winnt::LONG; /* imm.h:271:14 */
    pub fn ImmGetCompositionWindow(_: ::imm::HIMC, lpCompForm: ::imm::LPCOMPOSITIONFORM) -> ::minwindef::BOOL; /* imm.h:361:13 */
    pub fn ImmGetContext(_: ::windef::HWND) -> ::imm::HIMC; /* imm.h:263:13 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmGetConversionListW as ImmGetConversionList; /* imm.h:352:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmGetConversionListA(_: ::minwindef::HKL, _: ::imm::HIMC, lpSrc: ::winnt::LPCSTR, lpDst: ::imm::LPCANDIDATELIST, dwBufLen: ::minwindef::DWORD, uFlag: ::minwindef::UINT) -> ::minwindef::DWORD; /* imm.h:349:16 */
    pub fn ImmGetConversionListW(_: ::minwindef::HKL, _: ::imm::HIMC, lpSrc: ::winnt::LPCWSTR, lpDst: ::imm::LPCANDIDATELIST, dwBufLen: ::minwindef::DWORD, uFlag: ::minwindef::UINT) -> ::minwindef::DWORD; /* imm.h:350:16 */
    pub fn ImmGetConversionStatus(_: ::imm::HIMC, lpfdwConversion: ::minwindef::LPDWORD, lpfdwSentence: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* imm.h:310:13 */
    pub fn ImmGetDefaultIMEWnd(_: ::windef::HWND) -> ::windef::HWND; /* imm.h:237:13 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmGetDescriptionW as ImmGetDescription; /* imm.h:242:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmGetDescriptionA(_: ::minwindef::HKL, lpszDescription: ::winnt::LPSTR, uBufLen: ::minwindef::UINT) -> ::minwindef::UINT; /* imm.h:239:13 */
    pub fn ImmGetDescriptionW(_: ::minwindef::HKL, lpszDescription: ::winnt::LPWSTR, uBufLen: ::minwindef::UINT) -> ::minwindef::UINT; /* imm.h:240:13 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmGetGuideLineW as ImmGetGuideLine; /* imm.h:305:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmGetGuideLineA(_: ::imm::HIMC, dwIndex: ::minwindef::DWORD, lpBuf: ::winnt::LPSTR, dwBufLen: ::minwindef::DWORD) -> ::minwindef::DWORD; /* imm.h:302:14 */
    pub fn ImmGetGuideLineW(_: ::imm::HIMC, dwIndex: ::minwindef::DWORD, lpBuf: ::winnt::LPWSTR, dwBufLen: ::minwindef::DWORD) -> ::minwindef::DWORD; /* imm.h:303:14 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmGetIMEFileNameW as ImmGetIMEFileName; /* imm.h:250:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmGetIMEFileNameA(_: ::minwindef::HKL, lpszFileName: ::winnt::LPSTR, uBufLen: ::minwindef::UINT) -> ::minwindef::UINT; /* imm.h:247:13 */
    pub fn ImmGetIMEFileNameW(_: ::minwindef::HKL, lpszFileName: ::winnt::LPWSTR, uBufLen: ::minwindef::UINT) -> ::minwindef::UINT; /* imm.h:248:13 */
    pub fn ImmGetOpenStatus(_: ::imm::HIMC) -> ::minwindef::BOOL; /* imm.h:312:13 */
    pub fn ImmGetProperty(_: ::minwindef::HKL, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* imm.h:255:14 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmGetRegisterWordStyleW as ImmGetRegisterWordStyle; /* imm.h:404:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmGetRegisterWordStyleA(_: ::minwindef::HKL, nItem: ::minwindef::UINT, lpStyleBuf: ::imm::LPSTYLEBUFA) -> ::minwindef::UINT; /* imm.h:401:13 */
    pub fn ImmGetRegisterWordStyleW(_: ::minwindef::HKL, nItem: ::minwindef::UINT, lpStyleBuf: ::imm::LPSTYLEBUFW) -> ::minwindef::UINT; /* imm.h:402:13 */
    pub fn ImmGetStatusWindowPos(_: ::imm::HIMC, lpptPos: ::windef::LPPOINT) -> ::minwindef::BOOL; /* imm.h:359:13 */
    pub fn ImmGetVirtualKey(_: ::windef::HWND) -> ::minwindef::UINT; /* imm.h:375:13 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmInstallIMEW as ImmInstallIME; /* imm.h:232:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmInstallIMEA(lpszIMEFileName: ::winnt::LPCSTR, lpszLayoutText: ::winnt::LPCSTR) -> ::minwindef::HKL; /* imm.h:229:13 */
    pub fn ImmInstallIMEW(lpszIMEFileName: ::winnt::LPCWSTR, lpszLayoutText: ::winnt::LPCWSTR) -> ::minwindef::HKL; /* imm.h:230:13 */
    pub fn ImmIsIME(_: ::minwindef::HKL) -> ::minwindef::BOOL; /* imm.h:257:13 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmIsUIMessageW as ImmIsUIMessage; /* imm.h:369:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmIsUIMessageA(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* imm.h:366:13 */
    pub fn ImmIsUIMessageW(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* imm.h:367:13 */
    pub fn ImmNotifyIME(_: ::imm::HIMC, dwAction: ::minwindef::DWORD, dwIndex: ::minwindef::DWORD, dwValue: ::minwindef::DWORD) -> ::minwindef::BOOL; /* imm.h:357:16 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmRegisterWordW as ImmRegisterWord; /* imm.h:388:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmRegisterWordA(_: ::minwindef::HKL, lpszReading: ::winnt::LPCSTR, _: ::minwindef::DWORD, lpszRegister: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* imm.h:385:13 */
    pub fn ImmRegisterWordW(_: ::minwindef::HKL, lpszReading: ::winnt::LPCWSTR, _: ::minwindef::DWORD, lpszRegister: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* imm.h:386:13 */
    pub fn ImmReleaseContext(_: ::windef::HWND, _: ::imm::HIMC) -> ::minwindef::BOOL; /* imm.h:264:13 */
    pub fn ImmSetCandidateWindow(_: ::imm::HIMC, lpCandidate: ::imm::LPCANDIDATEFORM) -> ::minwindef::BOOL; /* imm.h:364:13 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmSetCompositionFontW as ImmSetCompositionFont; /* imm.h:327:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmSetCompositionFontA(_: ::imm::HIMC, lplf: ::wingdi::LPLOGFONTA) -> ::minwindef::BOOL; /* imm.h:324:13 */
    pub fn ImmSetCompositionFontW(_: ::imm::HIMC, lplf: ::wingdi::LPLOGFONTW) -> ::minwindef::BOOL; /* imm.h:325:13 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmSetCompositionStringW as ImmSetCompositionString; /* imm.h:281:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmSetCompositionStringA(_: ::imm::HIMC, dwIndex: ::minwindef::DWORD, lpComp: ::minwindef::LPVOID, dwCompLen: ::minwindef::DWORD, lpRead: ::minwindef::LPVOID, dwReadLen: ::minwindef::DWORD) -> ::minwindef::BOOL; /* imm.h:278:14 */
    pub fn ImmSetCompositionStringW(_: ::imm::HIMC, dwIndex: ::minwindef::DWORD, lpComp: ::minwindef::LPVOID, dwCompLen: ::minwindef::DWORD, lpRead: ::minwindef::LPVOID, dwReadLen: ::minwindef::DWORD) -> ::minwindef::BOOL; /* imm.h:279:14 */
    pub fn ImmSetCompositionWindow(_: ::imm::HIMC, lpCompForm: ::imm::LPCOMPOSITIONFORM) -> ::minwindef::BOOL; /* imm.h:362:13 */
    pub fn ImmSetConversionStatus(_: ::imm::HIMC, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* imm.h:311:13 */
    pub fn ImmSetOpenStatus(_: ::imm::HIMC, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* imm.h:313:13 */
    pub fn ImmSetStatusWindowPos(_: ::imm::HIMC, lpptPos: ::windef::LPPOINT) -> ::minwindef::BOOL; /* imm.h:360:13 */
    pub fn ImmSimulateHotKey(_: ::windef::HWND, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* imm.h:259:13 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ImmUnregisterWordW as ImmUnregisterWord; /* imm.h:396:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ImmUnregisterWordA(_: ::minwindef::HKL, lpszReading: ::winnt::LPCSTR, _: ::minwindef::DWORD, lpszUnregister: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* imm.h:393:13 */
    pub fn ImmUnregisterWordW(_: ::minwindef::HKL, lpszReading: ::winnt::LPCWSTR, _: ::minwindef::DWORD, lpszUnregister: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* imm.h:394:13 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn ImmAssociateContextEx(_: ::windef::HWND, _: ::imm::HIMC, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* imm.h:267:13 */
    pub fn ImmDisableIME(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* imm.h:418:13 */
    pub fn ImmDisableTextFrameService(idThread: ::minwindef::DWORD) -> ::minwindef::BOOL; /* imm.h:428:13 */
    pub fn ImmEnumInputContext(idThread: ::minwindef::DWORD, lpfn: ::imm::IMCENUMPROC, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* imm.h:419:13 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::ImmGetImeMenuItemsW as ImmGetImeMenuItems; /* imm.h:423:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn ImmGetImeMenuItemsA(_: ::imm::HIMC, _: ::minwindef::DWORD, _: ::minwindef::DWORD, lpImeParentMenu: ::imm::LPIMEMENUITEMINFOA, lpImeMenu: ::imm::LPIMEMENUITEMINFOA, dwSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* imm.h:420:14 */
    pub fn ImmGetImeMenuItemsW(_: ::imm::HIMC, _: ::minwindef::DWORD, _: ::minwindef::DWORD, lpImeParentMenu: ::imm::LPIMEMENUITEMINFOW, lpImeMenu: ::imm::LPIMEMENUITEMINFOW, dwSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* imm.h:421:14 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn ImmDisableLegacyIME() -> ::minwindef::BOOL; /* imm.h:432:13 */
}
