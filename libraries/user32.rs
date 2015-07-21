#[doc(inline)] pub use self::CharLowerA as AnsiLower; /* winuser.h:5512:9 */
#[doc(inline)] pub use self::CharLowerBuffA as AnsiLowerBuff; /* winuser.h:5513:9 */
#[doc(inline)] pub use self::CharNextA as AnsiNext; /* winuser.h:5514:9 */
#[doc(inline)] pub use self::CharPrevA as AnsiPrev; /* winuser.h:5515:9 */
#[doc(inline)] pub use self::CharToOemA as AnsiToOem; /* winuser.h:5506:9 */
#[doc(inline)] pub use self::CharToOemBuffA as AnsiToOemBuff; /* winuser.h:5508:9 */
#[doc(inline)] pub use self::CharUpperA as AnsiUpper; /* winuser.h:5510:9 */
#[doc(inline)] pub use self::CharUpperBuffA as AnsiUpperBuff; /* winuser.h:5511:9 */
#[doc(inline)] pub use self::OemToCharA as OemToAnsi; /* winuser.h:5507:9 */
#[doc(inline)] pub use self::OemToCharBuffA as OemToAnsiBuff; /* winuser.h:5509:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] 
extern "system" {
    pub fn EndTask(hWnd: ::windef::HWND, fShutDown: ::minwindef::BOOL, fForce: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:12847:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] 
extern "C" {
    pub fn wsprintfA(_: ::winnt::LPSTR, _: ::winnt::LPCSTR, ...) -> ::libc::c_int; /* winuser.h:295:1 */
    pub fn wsprintfW(_: ::winnt::LPWSTR, _: ::winnt::LPCWSTR, ...) -> ::libc::c_int; /* winuser.h:302:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn GetClassLongPtrA(hWnd: ::windef::HWND, nIndex: ::libc::c_int) -> ::basetsd::ULONG_PTR; /* winuser.h:9436:1 */
    pub fn GetClassLongPtrW(hWnd: ::windef::HWND, nIndex: ::libc::c_int) -> ::basetsd::ULONG_PTR; /* winuser.h:9442:1 */
    pub fn GetWindowLongPtrA(hWnd: ::windef::HWND, nIndex: ::libc::c_int) -> ::basetsd::LONG_PTR; /* winuser.h:9323:1 */
    pub fn GetWindowLongPtrW(hWnd: ::windef::HWND, nIndex: ::libc::c_int) -> ::basetsd::LONG_PTR; /* winuser.h:9329:1 */
    pub fn SetClassLongPtrA(hWnd: ::windef::HWND, nIndex: ::libc::c_int, dwNewLong: ::basetsd::LONG_PTR) -> ::basetsd::ULONG_PTR; /* winuser.h:9454:1 */
    pub fn SetClassLongPtrW(hWnd: ::windef::HWND, nIndex: ::libc::c_int, dwNewLong: ::basetsd::LONG_PTR) -> ::basetsd::ULONG_PTR; /* winuser.h:9461:1 */
    pub fn SetWindowLongPtrA(hWnd: ::windef::HWND, nIndex: ::libc::c_int, dwNewLong: ::basetsd::LONG_PTR) -> ::basetsd::LONG_PTR; /* winuser.h:9341:1 */
    pub fn SetWindowLongPtrW(hWnd: ::windef::HWND, nIndex: ::libc::c_int, dwNewLong: ::basetsd::LONG_PTR) -> ::basetsd::LONG_PTR; /* winuser.h:9348:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] #[doc(inline)] pub use self::GetClassLongA as GetClassLongPtrA; /* winuser.h:9473:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] #[doc(inline)] pub use self::GetClassLongW as GetClassLongPtrW; /* winuser.h:9474:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] #[doc(inline)] pub use self::GetWindowLongA as GetWindowLongPtrA; /* winuser.h:9360:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] #[doc(inline)] pub use self::GetWindowLongW as GetWindowLongPtrW; /* winuser.h:9361:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] #[doc(inline)] pub use self::SetClassLongA as SetClassLongPtrA; /* winuser.h:9481:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] #[doc(inline)] pub use self::SetClassLongW as SetClassLongPtrW; /* winuser.h:9482:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] #[doc(inline)] pub use self::SetWindowLongA as SetWindowLongPtrA; /* winuser.h:9368:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] #[doc(inline)] pub use self::SetWindowLongW as SetWindowLongPtrW; /* winuser.h:9369:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64", target_arch="arm"))] 
extern "system" {
    pub fn wsprintfA(_: ::winnt::LPSTR, _: ::winnt::LPCSTR, ...) -> ::libc::c_int; /* winuser.h:295:1 */
    pub fn wsprintfW(_: ::winnt::LPWSTR, _: ::winnt::LPCWSTR, ...) -> ::libc::c_int; /* winuser.h:302:1 */
}
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AdjustWindowRect(lpRect: ::windef::LPRECT, dwStyle: ::minwindef::DWORD, bMenu: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:8546:1 */
    pub fn AdjustWindowRectEx(lpRect: ::windef::LPRECT, dwStyle: ::minwindef::DWORD, bMenu: ::minwindef::BOOL, dwExStyle: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:8554:1 */
    pub fn AnyPopup() -> ::minwindef::BOOL; /* winuser.h:4632:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AppendMenuW as AppendMenu; /* winuser.h:7192:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AppendMenuA(hMenu: ::windef::HMENU, uFlags: ::minwindef::UINT, uIDNewItem: ::basetsd::UINT_PTR, lpNewItem: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winuser.h:7178:1 */
    pub fn AppendMenuW(hMenu: ::windef::HMENU, uFlags: ::minwindef::UINT, uIDNewItem: ::basetsd::UINT_PTR, lpNewItem: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winuser.h:7186:1 */
    pub fn ArrangeIconicWindows(hWnd: ::windef::HWND) -> ::minwindef::UINT; /* winuser.h:11535:1 */
    pub fn AttachThreadInput(idAttach: ::minwindef::DWORD, idAttachTo: ::minwindef::DWORD, fAttach: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:3950:1 */
    pub fn BeginDeferWindowPos(nNumWindows: ::libc::c_int) -> ::winuser::HDWP; /* winuser.h:4586:1 */
    pub fn BeginPaint(hWnd: ::windef::HWND, lpPaint: ::winuser::LPPAINTSTRUCT) -> ::windef::HDC; /* winuser.h:8115:1 */
    pub fn BringWindowToTop(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4638:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CallMsgFilterW as CallMsgFilter; /* winuser.h:5126:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CallMsgFilterA(lpMsg: ::winuser::LPMSG, nCode: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:5116:1 */
    pub fn CallMsgFilterW(lpMsg: ::winuser::LPMSG, nCode: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:5122:1 */
    pub fn CallNextHookEx(hhk: ::windef::HHOOK, nCode: ::libc::c_int, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:9804:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CallWindowProcW as CallWindowProc; /* winuser.h:4038:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CallWindowProcA(lpPrevWndFunc: ::winuser::WNDPROC, hWnd: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:4022:1 */
    pub fn CallWindowProcW(lpPrevWndFunc: ::winuser::WNDPROC, hWnd: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:4031:1 */
    pub fn CancelShutdown() -> ::minwindef::BOOL; /* winuser.h:12856:1 */
    pub fn ChangeClipboardChain(hWndRemove: ::windef::HWND, hWndNewNext: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:5183:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ChangeDisplaySettingsW as ChangeDisplaySettings; /* winuser.h:12377:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ChangeDisplaySettingsA(lpDevMode: *mut ::wingdi::DEVMODEA, dwFlags: ::minwindef::DWORD) -> ::winnt::LONG; /* winuser.h:12367:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ChangeDisplaySettingsExW as ChangeDisplaySettingsEx; /* winuser.h:12401:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ChangeDisplaySettingsExA(lpszDeviceName: ::winnt::LPCSTR, lpDevMode: *mut ::wingdi::DEVMODEA, hwnd: ::windef::HWND, dwflags: ::minwindef::DWORD, lParam: ::minwindef::LPVOID) -> ::winnt::LONG; /* winuser.h:12385:1 */
    pub fn ChangeDisplaySettingsExW(lpszDeviceName: ::winnt::LPCWSTR, lpDevMode: *mut ::wingdi::DEVMODEW, hwnd: ::windef::HWND, dwflags: ::minwindef::DWORD, lParam: ::minwindef::LPVOID) -> ::winnt::LONG; /* winuser.h:12394:1 */
    pub fn ChangeDisplaySettingsW(lpDevMode: *mut ::wingdi::DEVMODEW, dwFlags: ::minwindef::DWORD) -> ::winnt::LONG; /* winuser.h:12373:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ChangeMenuW as ChangeMenu; /* winuser.h:7031:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ChangeMenuA(hMenu: ::windef::HMENU, cmd: ::minwindef::UINT, lpszNewItem: ::winnt::LPCSTR, cmdInsert: ::minwindef::UINT, flags: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7015:1 */
    pub fn ChangeMenuW(hMenu: ::windef::HMENU, cmd: ::minwindef::UINT, lpszNewItem: ::winnt::LPCWSTR, cmdInsert: ::minwindef::UINT, flags: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7024:1 */
    pub fn CharLowerA(lpsz: ::winnt::LPSTR) -> ::winnt::LPSTR; /* winuser.h:5416:1 */
    pub fn CharLowerBuffA(lpsz: ::winnt::LPSTR, cchLength: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:5432:1 */
    pub fn CharNextA(lpsz: ::winnt::LPCSTR) -> ::winnt::LPSTR; /* winuser.h:5450:1 */
    pub fn CharPrevA(lpszStart: ::winnt::LPCSTR, lpszCurrent: ::winnt::LPCSTR) -> ::winnt::LPSTR; /* winuser.h:5466:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharToOemW as CharToOem; /* winuser.h:5314:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharToOemA(pSrc: ::winnt::LPCSTR, pDst: ::winnt::LPSTR) -> ::minwindef::BOOL; /* winuser.h:5304:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharToOemBuffW as CharToOemBuff; /* winuser.h:5354:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharToOemBuffA(lpszSrc: ::winnt::LPCSTR, lpszDst: ::winnt::LPSTR, cchDstLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:5342:1 */
    pub fn CharToOemBuffW(lpszSrc: ::winnt::LPCWSTR, lpszDst: ::winnt::LPSTR, cchDstLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:5349:1 */
    pub fn CharToOemW(pSrc: ::winnt::LPCWSTR, pDst: ::winnt::LPSTR) -> ::minwindef::BOOL; /* winuser.h:5310:1 */
    pub fn CharUpperA(lpsz: ::winnt::LPSTR) -> ::winnt::LPSTR; /* winuser.h:5382:1 */
    pub fn CharUpperBuffA(lpsz: ::winnt::LPSTR, cchLength: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:5398:1 */
    pub fn CheckDlgButton(hDlg: ::windef::HWND, nIDButton: ::libc::c_int, uCheck: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:4990:1 */
    pub fn CheckMenuItem(hMenu: ::windef::HMENU, uIDCheckItem: ::minwindef::UINT, uCheck: ::minwindef::UINT) -> ::minwindef::DWORD; /* winuser.h:7118:1 */
    pub fn CheckRadioButton(hDlg: ::windef::HWND, nIDFirstButton: ::libc::c_int, nIDLastButton: ::libc::c_int, nIDCheckButton: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:4998:1 */
    pub fn ChildWindowFromPoint(hWndParent: ::windef::HWND, Point: ::windef::POINT) -> ::windef::HWND; /* winuser.h:9023:1 */
    pub fn ClientToScreen(hWnd: ::windef::HWND, lpPoint: ::windef::LPPOINT) -> ::minwindef::BOOL; /* winuser.h:8952:1 */
    pub fn ClipCursor(lpRect: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:8881:1 */
    pub fn CloseClipboard() -> ::minwindef::BOOL; /* winuser.h:5148:1 */
    pub fn CloseDesktop(hDesktop: ::windef::HDESK) -> ::minwindef::BOOL; /* winuser.h:1446:1 */
    pub fn CloseWindow(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4514:1 */
    pub fn CloseWindowStation(hWinSta: ::minwindef::HWINSTA) -> ::minwindef::BOOL; /* winuser.h:1554:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CopyAcceleratorTableW as CopyAcceleratorTable; /* winuser.h:6751:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CopyAcceleratorTableA(hAccelSrc: ::windef::HACCEL, lpAccelDst: ::winuser::LPACCEL, cAccelEntries: ::libc::c_int) -> ::libc::c_int; /* winuser.h:6739:1 */
    pub fn CopyAcceleratorTableW(hAccelSrc: ::windef::HACCEL, lpAccelDst: ::winuser::LPACCEL, cAccelEntries: ::libc::c_int) -> ::libc::c_int; /* winuser.h:6746:1 */
    pub fn CopyIcon(hIcon: ::windef::HICON) -> ::windef::HICON; /* winuser.h:10331:1 */
    pub fn CopyRect(lprcDst: ::windef::LPRECT, lprcSrc: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9199:1 */
    pub fn CountClipboardFormats() -> ::libc::c_int; /* winuser.h:5219:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateAcceleratorTableW as CreateAcceleratorTable; /* winuser.h:6725:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateAcceleratorTableA(paccel: ::winuser::LPACCEL, cAccel: ::libc::c_int) -> ::windef::HACCEL; /* winuser.h:6715:1 */
    pub fn CreateAcceleratorTableW(paccel: ::winuser::LPACCEL, cAccel: ::libc::c_int) -> ::windef::HACCEL; /* winuser.h:6721:1 */
    pub fn CreateCaret(hWnd: ::windef::HWND, hBitmap: ::windef::HBITMAP, nWidth: ::libc::c_int, nHeight: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:8900:1 */
    pub fn CreateCursor(hInst: ::minwindef::HINSTANCE, xHotSpot: ::libc::c_int, yHotSpot: ::libc::c_int, nWidth: ::libc::c_int, nHeight: ::libc::c_int, pvANDPlane: *const ::libc::c_void, pvXORPlane: *const ::libc::c_void) -> ::windef::HCURSOR; /* winuser.h:10040:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateDesktopW as CreateDesktop; /* winuser.h:1326:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateDesktopA(lpszDesktop: ::winnt::LPCSTR, lpszDevice: ::winnt::LPCSTR, pDevmode: *mut ::wingdi::DEVMODEA, dwFlags: ::minwindef::DWORD, dwDesiredAccess: ::winnt::ACCESS_MASK, lpsa: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::windef::HDESK; /* winuser.h:1308:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateDesktopExW as CreateDesktopEx; /* winuser.h:1356:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateDesktopExA(lpszDesktop: ::winnt::LPCSTR, lpszDevice: ::winnt::LPCSTR, pDevmode: *mut ::wingdi::DEVMODEA, dwFlags: ::minwindef::DWORD, dwDesiredAccess: ::winnt::ACCESS_MASK, lpsa: ::minwinbase::LPSECURITY_ATTRIBUTES, ulHeapSize: ::minwindef::ULONG, pvoid: ::winnt::PVOID) -> ::windef::HDESK; /* winuser.h:1334:1 */
    pub fn CreateDesktopExW(lpszDesktop: ::winnt::LPCWSTR, lpszDevice: ::winnt::LPCWSTR, pDevmode: *mut ::wingdi::DEVMODEW, dwFlags: ::minwindef::DWORD, dwDesiredAccess: ::winnt::ACCESS_MASK, lpsa: ::minwinbase::LPSECURITY_ATTRIBUTES, ulHeapSize: ::minwindef::ULONG, pvoid: ::winnt::PVOID) -> ::windef::HDESK; /* winuser.h:1346:1 */
    pub fn CreateDesktopW(lpszDesktop: ::winnt::LPCWSTR, lpszDevice: ::winnt::LPCWSTR, pDevmode: *mut ::wingdi::DEVMODEW, dwFlags: ::minwindef::DWORD, dwDesiredAccess: ::winnt::ACCESS_MASK, lpsa: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::windef::HDESK; /* winuser.h:1318:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateDialogIndirectParamW as CreateDialogIndirectParam; /* winuser.h:4818:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateDialogIndirectParamA(hInstance: ::minwindef::HINSTANCE, lpTemplate: ::winuser::LPCDLGTEMPLATEA, hWndParent: ::windef::HWND, lpDialogFunc: ::winuser::DLGPROC, dwInitParam: ::minwindef::LPARAM) -> ::windef::HWND; /* winuser.h:4802:1 */
    pub fn CreateDialogIndirectParamW(hInstance: ::minwindef::HINSTANCE, lpTemplate: ::winuser::LPCDLGTEMPLATEW, hWndParent: ::windef::HWND, lpDialogFunc: ::winuser::DLGPROC, dwInitParam: ::minwindef::LPARAM) -> ::windef::HWND; /* winuser.h:4811:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateDialogParamW as CreateDialogParam; /* winuser.h:4794:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateDialogParamA(hInstance: ::minwindef::HINSTANCE, lpTemplateName: ::winnt::LPCSTR, hWndParent: ::windef::HWND, lpDialogFunc: ::winuser::DLGPROC, dwInitParam: ::minwindef::LPARAM) -> ::windef::HWND; /* winuser.h:4778:1 */
    pub fn CreateDialogParamW(hInstance: ::minwindef::HINSTANCE, lpTemplateName: ::winnt::LPCWSTR, hWndParent: ::windef::HWND, lpDialogFunc: ::winuser::DLGPROC, dwInitParam: ::minwindef::LPARAM) -> ::windef::HWND; /* winuser.h:4787:1 */
    pub fn CreateIcon(hInstance: ::minwindef::HINSTANCE, nWidth: ::libc::c_int, nHeight: ::libc::c_int, cPlanes: ::minwindef::BYTE, cBitsPixel: ::minwindef::BYTE, lpbANDbits: *const ::libc::c_uchar, lpbXORbits: *const ::libc::c_uchar) -> ::windef::HICON; /* winuser.h:10163:1 */
    pub fn CreateIconFromResource(presbits: ::minwindef::PBYTE, dwResSize: ::minwindef::DWORD, fIcon: ::minwindef::BOOL, dwVer: ::minwindef::DWORD) -> ::windef::HICON; /* winuser.h:10200:1 */
    pub fn CreateIconIndirect(piconinfo: ::winuser::PICONINFO) -> ::windef::HICON; /* winuser.h:10325:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateMDIWindowW as CreateMDIWindow; /* winuser.h:11567:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateMDIWindowA(lpClassName: ::winnt::LPCSTR, lpWindowName: ::winnt::LPCSTR, dwStyle: ::minwindef::DWORD, X: ::libc::c_int, Y: ::libc::c_int, nWidth: ::libc::c_int, nHeight: ::libc::c_int, hWndParent: ::windef::HWND, hInstance: ::minwindef::HINSTANCE, lParam: ::minwindef::LPARAM) -> ::windef::HWND; /* winuser.h:11541:1 */
    pub fn CreateMDIWindowW(lpClassName: ::winnt::LPCWSTR, lpWindowName: ::winnt::LPCWSTR, dwStyle: ::minwindef::DWORD, X: ::libc::c_int, Y: ::libc::c_int, nWidth: ::libc::c_int, nHeight: ::libc::c_int, hWndParent: ::windef::HWND, hInstance: ::minwindef::HINSTANCE, lParam: ::minwindef::LPARAM) -> ::windef::HWND; /* winuser.h:11555:1 */
    pub fn CreateMenu() -> ::windef::HMENU; /* winuser.h:7100:1 */
    pub fn CreatePopupMenu() -> ::windef::HMENU; /* winuser.h:7106:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateWindowExW as CreateWindowEx; /* winuser.h:4267:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateWindowExA(dwExStyle: ::minwindef::DWORD, lpClassName: ::winnt::LPCSTR, lpWindowName: ::winnt::LPCSTR, dwStyle: ::minwindef::DWORD, X: ::libc::c_int, Y: ::libc::c_int, nWidth: ::libc::c_int, nHeight: ::libc::c_int, hWndParent: ::windef::HWND, hMenu: ::windef::HMENU, hInstance: ::minwindef::HINSTANCE, lpParam: ::minwindef::LPVOID) -> ::windef::HWND; /* winuser.h:4237:1 */
    pub fn CreateWindowExW(dwExStyle: ::minwindef::DWORD, lpClassName: ::winnt::LPCWSTR, lpWindowName: ::winnt::LPCWSTR, dwStyle: ::minwindef::DWORD, X: ::libc::c_int, Y: ::libc::c_int, nWidth: ::libc::c_int, nHeight: ::libc::c_int, hWndParent: ::windef::HWND, hMenu: ::windef::HMENU, hInstance: ::minwindef::HINSTANCE, lpParam: ::minwindef::LPVOID) -> ::windef::HWND; /* winuser.h:4253:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateWindowStationW as CreateWindowStation; /* winuser.h:1508:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateWindowStationA(lpwinsta: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD, dwDesiredAccess: ::winnt::ACCESS_MASK, lpsa: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::HWINSTA; /* winuser.h:1494:1 */
    pub fn CreateWindowStationW(lpwinsta: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, dwDesiredAccess: ::winnt::ACCESS_MASK, lpsa: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::HWINSTA; /* winuser.h:1502:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DefDlgProcW as DefDlgProc; /* winuser.h:5089:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DefDlgProcA(hDlg: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:5070:1 */
    pub fn DefDlgProcW(hDlg: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:5083:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DefFrameProcW as DefFrameProc; /* winuser.h:11484:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DefFrameProcA(hWnd: ::windef::HWND, hWndMDIClient: ::windef::HWND, uMsg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:11468:1 */
    pub fn DefFrameProcW(hWnd: ::windef::HWND, hWndMDIClient: ::windef::HWND, uMsg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:11477:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DefMDIChildProcW as DefMDIChildProc; /* winuser.h:11516:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DefMDIChildProcA(hWnd: ::windef::HWND, uMsg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:11497:1 */
    pub fn DefMDIChildProcW(hWnd: ::windef::HWND, uMsg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:11510:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DefWindowProcW as DefWindowProc; /* winuser.h:4006:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DefWindowProcA(hWnd: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:3987:1 */
    pub fn DefWindowProcW(hWnd: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:4000:1 */
    pub fn DeferWindowPos(hWinPosInfo: ::winuser::HDWP, hWnd: ::windef::HWND, hWndInsertAfter: ::windef::HWND, x: ::libc::c_int, y: ::libc::c_int, cx: ::libc::c_int, cy: ::libc::c_int, uFlags: ::minwindef::UINT) -> ::winuser::HDWP; /* winuser.h:4592:1 */
    pub fn DeleteMenu(hMenu: ::windef::HMENU, uPosition: ::minwindef::UINT, uFlags: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7231:1 */
    pub fn DeregisterShellHookWindow(hwnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:9595:1 */
    pub fn DestroyAcceleratorTable(hAccel: ::windef::HACCEL) -> ::minwindef::BOOL; /* winuser.h:6733:1 */
    pub fn DestroyCaret() -> ::minwindef::BOOL; /* winuser.h:8921:1 */
    pub fn DestroyCursor(hCursor: ::windef::HCURSOR) -> ::minwindef::BOOL; /* winuser.h:10052:1 */
    pub fn DestroyIcon(hIcon: ::windef::HICON) -> ::minwindef::BOOL; /* winuser.h:10175:1 */
    pub fn DestroyMenu(hMenu: ::windef::HMENU) -> ::minwindef::BOOL; /* winuser.h:7112:1 */
    pub fn DestroyWindow(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4316:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DialogBoxIndirectParamW as DialogBoxIndirectParam; /* winuser.h:4886:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DialogBoxIndirectParamA(hInstance: ::minwindef::HINSTANCE, hDialogTemplate: ::winuser::LPCDLGTEMPLATEA, hWndParent: ::windef::HWND, lpDialogFunc: ::winuser::DLGPROC, dwInitParam: ::minwindef::LPARAM) -> ::basetsd::INT_PTR; /* winuser.h:4870:1 */
    pub fn DialogBoxIndirectParamW(hInstance: ::minwindef::HINSTANCE, hDialogTemplate: ::winuser::LPCDLGTEMPLATEW, hWndParent: ::windef::HWND, lpDialogFunc: ::winuser::DLGPROC, dwInitParam: ::minwindef::LPARAM) -> ::basetsd::INT_PTR; /* winuser.h:4879:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DialogBoxParamW as DialogBoxParam; /* winuser.h:4862:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DialogBoxParamA(hInstance: ::minwindef::HINSTANCE, lpTemplateName: ::winnt::LPCSTR, hWndParent: ::windef::HWND, lpDialogFunc: ::winuser::DLGPROC, dwInitParam: ::minwindef::LPARAM) -> ::basetsd::INT_PTR; /* winuser.h:4846:1 */
    pub fn DialogBoxParamW(hInstance: ::minwindef::HINSTANCE, lpTemplateName: ::winnt::LPCWSTR, hWndParent: ::windef::HWND, lpDialogFunc: ::winuser::DLGPROC, dwInitParam: ::minwindef::LPARAM) -> ::basetsd::INT_PTR; /* winuser.h:4855:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DispatchMessageW as DispatchMessage; /* winuser.h:3341:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DispatchMessageA(lpMsg: *const ::winuser::MSG) -> ::minwindef::LRESULT; /* winuser.h:3333:1 */
    pub fn DispatchMessageW(lpMsg: *const ::winuser::MSG) -> ::minwindef::LRESULT; /* winuser.h:3338:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DlgDirListW as DlgDirList; /* winuser.h:10932:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DlgDirListA(hDlg: ::windef::HWND, lpPathSpec: ::winnt::LPSTR, nIDListBox: ::libc::c_int, nIDStaticPath: ::libc::c_int, uFileType: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:10916:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DlgDirListComboBoxW as DlgDirListComboBox; /* winuser.h:10998:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DlgDirListComboBoxA(hDlg: ::windef::HWND, lpPathSpec: ::winnt::LPSTR, nIDComboBox: ::libc::c_int, nIDStaticPath: ::libc::c_int, uFiletype: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:10982:1 */
    pub fn DlgDirListComboBoxW(hDlg: ::windef::HWND, lpPathSpec: ::winnt::LPWSTR, nIDComboBox: ::libc::c_int, nIDStaticPath: ::libc::c_int, uFiletype: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:10991:1 */
    pub fn DlgDirListW(hDlg: ::windef::HWND, lpPathSpec: ::winnt::LPWSTR, nIDListBox: ::libc::c_int, nIDStaticPath: ::libc::c_int, uFileType: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:10925:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DlgDirSelectComboBoxExW as DlgDirSelectComboBoxEx; /* winuser.h:11020:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DlgDirSelectComboBoxExA(hwndDlg: ::windef::HWND, lpString: ::winnt::LPSTR, cchOut: ::libc::c_int, idComboBox: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:11006:1 */
    pub fn DlgDirSelectComboBoxExW(hwndDlg: ::windef::HWND, lpString: ::winnt::LPWSTR, cchOut: ::libc::c_int, idComboBox: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:11014:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DlgDirSelectExW as DlgDirSelectEx; /* winuser.h:10974:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DlgDirSelectExA(hwndDlg: ::windef::HWND, lpString: ::winnt::LPSTR, chCount: ::libc::c_int, idListBox: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:10960:1 */
    pub fn DlgDirSelectExW(hwndDlg: ::windef::HWND, lpString: ::winnt::LPWSTR, chCount: ::libc::c_int, idListBox: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:10968:1 */
    pub fn DrawFocusRect(hDC: ::windef::HDC, lprc: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9153:1 */
    pub fn DrawIcon(hDC: ::windef::HDC, X: ::libc::c_int, Y: ::libc::c_int, hIcon: ::windef::HICON) -> ::minwindef::BOOL; /* winuser.h:7666:1 */
    pub fn DrawMenuBar(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:7080:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DrawTextW as DrawText; /* winuser.h:7765:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DrawTextA(hdc: ::windef::HDC, lpchText: ::winnt::LPCSTR, cchText: ::libc::c_int, lprc: ::windef::LPRECT, format: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:7744:1 */
    pub fn DrawTextW(hdc: ::windef::HDC, lpchText: ::winnt::LPCWSTR, cchText: ::libc::c_int, lprc: ::windef::LPRECT, format: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:7756:1 */
    pub fn EmptyClipboard() -> ::minwindef::BOOL; /* winuser.h:5251:1 */
    pub fn EnableMenuItem(hMenu: ::windef::HMENU, uIDEnableItem: ::minwindef::UINT, uEnable: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7126:1 */
    pub fn EnableScrollBar(hWnd: ::windef::HWND, wSBflags: ::minwindef::UINT, wArrows: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:8355:1 */
    pub fn EnableWindow(hWnd: ::windef::HWND, bEnable: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:6684:1 */
    pub fn EndDeferWindowPos(hWinPosInfo: ::winuser::HDWP) -> ::minwindef::BOOL; /* winuser.h:4606:1 */
    pub fn EndDialog(hDlg: ::windef::HWND, nResult: ::basetsd::INT_PTR) -> ::minwindef::BOOL; /* winuser.h:4914:1 */
    pub fn EndPaint(hWnd: ::windef::HWND, lpPaint: *const ::winuser::PAINTSTRUCT) -> ::minwindef::BOOL; /* winuser.h:8122:1 */
    pub fn EnumChildWindows(hWndParent: ::windef::HWND, lpEnumFunc: ::winuser::WNDENUMPROC, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:9530:1 */
    pub fn EnumClipboardFormats(format: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:5225:1 */
    pub fn EnumDesktopWindows(hDesktop: ::windef::HDESK, lpfn: ::winuser::WNDENUMPROC, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:1424:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumDesktopsW as EnumDesktops; /* winuser.h:1416:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumDesktopsA(hwinsta: ::minwindef::HWINSTA, lpEnumFunc: ::winuser::DESKTOPENUMPROCA, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:1404:1 */
    pub fn EnumDesktopsW(hwinsta: ::minwindef::HWINSTA, lpEnumFunc: ::winuser::DESKTOPENUMPROCW, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:1411:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumDisplaySettingsW as EnumDisplaySettings; /* winuser.h:12425:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumDisplaySettingsA(lpszDeviceName: ::winnt::LPCSTR, iModeNum: ::minwindef::DWORD, lpDevMode: *mut ::wingdi::DEVMODEA) -> ::minwindef::BOOL; /* winuser.h:12413:1 */
    pub fn EnumDisplaySettingsW(lpszDeviceName: ::winnt::LPCWSTR, iModeNum: ::minwindef::DWORD, lpDevMode: *mut ::wingdi::DEVMODEW) -> ::minwindef::BOOL; /* winuser.h:12420:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumPropsW as EnumProps; /* winuser.h:8468:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumPropsA(hWnd: ::windef::HWND, lpEnumFunc: ::winuser::PROPENUMPROCA) -> ::libc::c_int; /* winuser.h:8458:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumPropsExW as EnumPropsEx; /* winuser.h:8450:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumPropsExA(hWnd: ::windef::HWND, lpEnumFunc: ::winuser::PROPENUMPROCEXA, lParam: ::minwindef::LPARAM) -> ::libc::c_int; /* winuser.h:8438:1 */
    pub fn EnumPropsExW(hWnd: ::windef::HWND, lpEnumFunc: ::winuser::PROPENUMPROCEXW, lParam: ::minwindef::LPARAM) -> ::libc::c_int; /* winuser.h:8445:1 */
    pub fn EnumPropsW(hWnd: ::windef::HWND, lpEnumFunc: ::winuser::PROPENUMPROCW) -> ::libc::c_int; /* winuser.h:8464:1 */
    pub fn EnumThreadWindows(dwThreadId: ::minwindef::DWORD, lpfn: ::winuser::WNDENUMPROC, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:9608:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumWindowStationsW as EnumWindowStations; /* winuser.h:1546:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumWindowStationsA(lpEnumFunc: ::winuser::WINSTAENUMPROCA, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:1536:1 */
    pub fn EnumWindowStationsW(lpEnumFunc: ::winuser::WINSTAENUMPROCW, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:1542:1 */
    pub fn EnumWindows(lpEnumFunc: ::winuser::WNDENUMPROC, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:9601:1 */
    pub fn EqualRect(lprc1: *const ::windef::RECT, lprc2: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9252:1 */
    pub fn ExcludeUpdateRgn(hDC: ::windef::HDC, hWnd: ::windef::HWND) -> ::libc::c_int; /* winuser.h:8178:1 */
    pub fn ExitWindowsEx(uFlags: ::minwindef::UINT, dwReason: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:3496:1 */
    pub fn FillRect(hDC: ::windef::HDC, lprc: *const ::windef::RECT, hbr: ::windef::HBRUSH) -> ::libc::c_int; /* winuser.h:9160:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindWindowW as FindWindow; /* winuser.h:9549:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindWindowA(lpClassName: ::winnt::LPCSTR, lpWindowName: ::winnt::LPCSTR) -> ::windef::HWND; /* winuser.h:9539:1 */
    pub fn FindWindowW(lpClassName: ::winnt::LPCWSTR, lpWindowName: ::winnt::LPCWSTR) -> ::windef::HWND; /* winuser.h:9545:1 */
    pub fn FlashWindow(hWnd: ::windef::HWND, bInvert: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:4470:1 */
    pub fn FrameRect(hDC: ::windef::HDC, lprc: *const ::windef::RECT, hbr: ::windef::HBRUSH) -> ::libc::c_int; /* winuser.h:9168:1 */
    pub fn GetActiveWindow() -> ::windef::HWND; /* winuser.h:5600:1 */
    pub fn GetAsyncKeyState(vKey: ::libc::c_int) -> ::winnt::SHORT; /* winuser.h:5624:1 */
    pub fn GetCapture() -> ::windef::HWND; /* winuser.h:6527:1 */
    pub fn GetCaretBlinkTime() -> ::minwindef::UINT; /* winuser.h:8909:1 */
    pub fn GetCaretPos(lpPoint: ::windef::LPPOINT) -> ::minwindef::BOOL; /* winuser.h:8946:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetClassInfoW as GetClassInfo; /* winuser.h:4171:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetClassInfoA(hInstance: ::minwindef::HINSTANCE, lpClassName: ::winnt::LPCSTR, lpWndClass: ::winuser::LPWNDCLASSA) -> ::minwindef::BOOL; /* winuser.h:4158:1 */
    pub fn GetClassInfoW(hInstance: ::minwindef::HINSTANCE, lpClassName: ::winnt::LPCWSTR, lpWndClass: ::winuser::LPWNDCLASSW) -> ::minwindef::BOOL; /* winuser.h:4166:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetClassLongW as GetClassLong; /* winuser.h:9406:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetClassLongA(hWnd: ::windef::HWND, nIndex: ::libc::c_int) -> ::minwindef::DWORD; /* winuser.h:9396:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetClassLongPtrW as GetClassLongPtr; /* winuser.h:9476:9, winuser.h:9446:9, winuser.h:9476:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetClassLongW(hWnd: ::windef::HWND, nIndex: ::libc::c_int) -> ::minwindef::DWORD; /* winuser.h:9402:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetClassNameW as GetClassName; /* winuser.h:9633:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetClassNameA(hWnd: ::windef::HWND, lpClassName: ::winnt::LPSTR, nMaxCount: ::libc::c_int) -> ::libc::c_int; /* winuser.h:9619:1 */
    pub fn GetClassNameW(hWnd: ::windef::HWND, lpClassName: ::winnt::LPWSTR, nMaxCount: ::libc::c_int) -> ::libc::c_int; /* winuser.h:9627:1 */
    pub fn GetClassWord(hWnd: ::windef::HWND, nIndex: ::libc::c_int) -> ::minwindef::WORD; /* winuser.h:9381:1 */
    pub fn GetClientRect(hWnd: ::windef::HWND, lpRect: ::windef::LPRECT) -> ::minwindef::BOOL; /* winuser.h:8532:1 */
    pub fn GetClipCursor(lpRect: ::windef::LPRECT) -> ::minwindef::BOOL; /* winuser.h:8888:1 */
    pub fn GetClipboardData(uFormat: ::minwindef::UINT) -> ::winnt::HANDLE; /* winuser.h:5197:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetClipboardFormatNameW as GetClipboardFormatName; /* winuser.h:5243:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetClipboardFormatNameA(format: ::minwindef::UINT, lpszFormatName: ::winnt::LPSTR, cchMaxCount: ::libc::c_int) -> ::libc::c_int; /* winuser.h:5231:1 */
    pub fn GetClipboardFormatNameW(format: ::minwindef::UINT, lpszFormatName: ::winnt::LPWSTR, cchMaxCount: ::libc::c_int) -> ::libc::c_int; /* winuser.h:5238:1 */
    pub fn GetClipboardOwner() -> ::windef::HWND; /* winuser.h:5165:1 */
    pub fn GetClipboardViewer() -> ::windef::HWND; /* winuser.h:5177:1 */
    pub fn GetCursor() -> ::windef::HCURSOR; /* winuser.h:8894:1 */
    pub fn GetCursorPos(lpPoint: ::windef::LPPOINT) -> ::minwindef::BOOL; /* winuser.h:8867:1 */
    pub fn GetDC(hWnd: ::windef::HWND) -> ::windef::HDC; /* winuser.h:8065:1 */
    pub fn GetDCEx(hWnd: ::windef::HWND, hrgnClip: ::minwindef::HRGN, flags: ::minwindef::DWORD) -> ::windef::HDC; /* winuser.h:8071:1 */
    pub fn GetDesktopWindow() -> ::windef::HWND; /* winuser.h:9510:1 */
    pub fn GetDialogBaseUnits() -> ::libc::c_long; /* winuser.h:5060:1 */
    pub fn GetDlgCtrlID(hWnd: ::windef::HWND) -> ::libc::c_int; /* winuser.h:5054:1 */
    pub fn GetDlgItem(hDlg: ::windef::HWND, nIDDlgItem: ::libc::c_int) -> ::windef::HWND; /* winuser.h:4921:1 */
    pub fn GetDlgItemInt(hDlg: ::windef::HWND, nIDDlgItem: ::libc::c_int, lpTranslated: *mut ::libc::c_int, bSigned: ::minwindef::BOOL) -> ::minwindef::UINT; /* winuser.h:4937:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetDlgItemTextW as GetDlgItemText; /* winuser.h:4982:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetDlgItemTextA(hDlg: ::windef::HWND, nIDDlgItem: ::libc::c_int, lpString: ::winnt::LPSTR, cchMax: ::libc::c_int) -> ::minwindef::UINT; /* winuser.h:4967:1 */
    pub fn GetDlgItemTextW(hDlg: ::windef::HWND, nIDDlgItem: ::libc::c_int, lpString: ::winnt::LPWSTR, cchMax: ::libc::c_int) -> ::minwindef::UINT; /* winuser.h:4976:1 */
    pub fn GetDoubleClickTime() -> ::minwindef::UINT; /* winuser.h:4111:1 */
    pub fn GetFocus() -> ::windef::HWND; /* winuser.h:5606:1 */
    pub fn GetForegroundWindow() -> ::windef::HWND; /* winuser.h:8011:1 */
    pub fn GetIconInfo(hIcon: ::windef::HICON, piconinfo: ::winuser::PICONINFO) -> ::minwindef::BOOL; /* winuser.h:10337:1 */
    pub fn GetInputState() -> ::minwindef::BOOL; /* winuser.h:6514:1 */
    pub fn GetKBCodePage() -> ::minwindef::UINT; /* winuser.h:5612:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetKeyNameTextW as GetKeyNameText; /* winuser.h:5655:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetKeyNameTextA(lParam: ::winnt::LONG, lpString: ::winnt::LPSTR, cchSize: ::libc::c_int) -> ::libc::c_int; /* winuser.h:5643:1 */
    pub fn GetKeyNameTextW(lParam: ::winnt::LONG, lpString: ::winnt::LPWSTR, cchSize: ::libc::c_int) -> ::libc::c_int; /* winuser.h:5650:1 */
    pub fn GetKeyState(nVirtKey: ::libc::c_int) -> ::winnt::SHORT; /* winuser.h:5618:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetKeyboardLayoutNameW as GetKeyboardLayoutName; /* winuser.h:1216:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetKeyboardLayoutNameA(pwszKLID: ::winnt::LPSTR) -> ::minwindef::BOOL; /* winuser.h:1208:1 */
    pub fn GetKeyboardLayoutNameW(pwszKLID: ::winnt::LPWSTR) -> ::minwindef::BOOL; /* winuser.h:1213:1 */
    pub fn GetKeyboardState(lpKeyState: ::minwindef::PBYTE) -> ::minwindef::BOOL; /* winuser.h:5631:1 */
    pub fn GetKeyboardType(nTypeFlag: ::libc::c_int) -> ::libc::c_int; /* winuser.h:5663:1 */
    pub fn GetLastActivePopup(hWnd: ::windef::HWND) -> ::windef::HWND; /* winuser.h:9695:1 */
    pub fn GetMenu(hWnd: ::windef::HWND) -> ::windef::HMENU; /* winuser.h:7002:1 */
    pub fn GetMenuCheckMarkDimensions() -> ::winnt::LONG; /* winuser.h:7249:1 */
    pub fn GetMenuItemCount(hMenu: ::windef::HMENU) -> ::libc::c_int; /* winuser.h:7148:1 */
    pub fn GetMenuItemID(hMenu: ::windef::HMENU, nPos: ::libc::c_int) -> ::minwindef::UINT; /* winuser.h:7141:1 */
    pub fn GetMenuState(hMenu: ::windef::HMENU, uId: ::minwindef::UINT, uFlags: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:7072:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetMenuStringW as GetMenuString; /* winuser.h:7064:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetMenuStringA(hMenu: ::windef::HMENU, uIDItem: ::minwindef::UINT, lpString: ::winnt::LPSTR, cchMax: ::libc::c_int, flags: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:7048:1 */
    pub fn GetMenuStringW(hMenu: ::windef::HMENU, uIDItem: ::minwindef::UINT, lpString: ::winnt::LPWSTR, cchMax: ::libc::c_int, flags: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:7057:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetMessageW as GetMessage; /* winuser.h:3294:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetMessageA(lpMsg: ::winuser::LPMSG, hWnd: ::windef::HWND, wMsgFilterMin: ::minwindef::UINT, wMsgFilterMax: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:3280:1 */
    pub fn GetMessageExtraInfo() -> ::minwindef::LPARAM; /* winuser.h:3521:1 */
    pub fn GetMessagePos() -> ::minwindef::DWORD; /* winuser.h:3509:1 */
    pub fn GetMessageTime() -> ::winnt::LONG; /* winuser.h:3515:1 */
    pub fn GetMessageW(lpMsg: ::winuser::LPMSG, hWnd: ::windef::HWND, wMsgFilterMin: ::minwindef::UINT, wMsgFilterMax: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:3288:1 */
    pub fn GetNextDlgGroupItem(hDlg: ::windef::HWND, hCtl: ::windef::HWND, bPrevious: ::minwindef::BOOL) -> ::windef::HWND; /* winuser.h:5038:1 */
    pub fn GetNextDlgTabItem(hDlg: ::windef::HWND, hCtl: ::windef::HWND, bPrevious: ::minwindef::BOOL) -> ::windef::HWND; /* winuser.h:5046:1 */
    pub fn GetOpenClipboardWindow() -> ::windef::HWND; /* winuser.h:5270:1 */
    pub fn GetParent(hWnd: ::windef::HWND) -> ::windef::HWND; /* winuser.h:9517:1 */
    pub fn GetPriorityClipboardFormat(paFormatPriorityList: *mut ::libc::c_uint, cFormats: ::libc::c_int) -> ::libc::c_int; /* winuser.h:5263:1 */
    pub fn GetProcessWindowStation() -> ::minwindef::HWINSTA; /* winuser.h:1566:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetPropW as GetProp; /* winuser.h:8412:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetPropA(hWnd: ::windef::HWND, lpString: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winuser.h:8402:1 */
    pub fn GetPropW(hWnd: ::windef::HWND, lpString: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winuser.h:8408:1 */
    pub fn GetQueueStatus(flags: ::minwindef::UINT) -> ::minwindef::DWORD; /* winuser.h:6520:1 */
    pub fn GetScrollPos(hWnd: ::windef::HWND, nBar: ::libc::c_int) -> ::libc::c_int; /* winuser.h:8321:1 */
    pub fn GetScrollRange(hWnd: ::windef::HWND, nBar: ::libc::c_int, lpMinPos: ::minwindef::LPINT, lpMaxPos: ::minwindef::LPINT) -> ::minwindef::BOOL; /* winuser.h:8338:1 */
    pub fn GetSubMenu(hMenu: ::windef::HMENU, nPos: ::libc::c_int) -> ::windef::HMENU; /* winuser.h:7134:1 */
    pub fn GetSysColor(nIndex: ::libc::c_int) -> ::minwindef::DWORD; /* winuser.h:9121:1 */
    pub fn GetSystemMenu(hWnd: ::windef::HWND, bRevert: ::minwindef::BOOL) -> ::windef::HMENU; /* winuser.h:7092:1 */
    pub fn GetSystemMetrics(nIndex: ::libc::c_int) -> ::libc::c_int; /* winuser.h:6951:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTabbedTextExtentW as GetTabbedTextExtent; /* winuser.h:7990:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTabbedTextExtentA(hdc: ::windef::HDC, lpString: ::winnt::LPCSTR, chCount: ::libc::c_int, nTabPositions: ::libc::c_int, lpnTabStopPositions: *const ::libc::c_int) -> ::minwindef::DWORD; /* winuser.h:7974:1 */
    pub fn GetTabbedTextExtentW(hdc: ::windef::HDC, lpString: ::winnt::LPCWSTR, chCount: ::libc::c_int, nTabPositions: ::libc::c_int, lpnTabStopPositions: *const ::libc::c_int) -> ::minwindef::DWORD; /* winuser.h:7983:1 */
    pub fn GetThreadDesktop(dwThreadId: ::minwindef::DWORD) -> ::windef::HDESK; /* winuser.h:1452:1 */
    pub fn GetTopWindow(hWnd: ::windef::HWND) -> ::windef::HWND; /* winuser.h:9665:1 */
    pub fn GetUpdateRect(hWnd: ::windef::HWND, lpRect: ::windef::LPRECT, bErase: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:8129:1 */
    pub fn GetUpdateRgn(hWnd: ::windef::HWND, hRgn: ::minwindef::HRGN, bErase: ::minwindef::BOOL) -> ::libc::c_int; /* winuser.h:8137:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetUserObjectInformationW as GetUserObjectInformation; /* winuser.h:1637:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetUserObjectInformationA(hObj: ::winnt::HANDLE, nIndex: ::libc::c_int, pvInfo: ::winnt::PVOID, nLength: ::minwindef::DWORD, lpnLengthNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winuser.h:1621:1 */
    pub fn GetUserObjectInformationW(hObj: ::winnt::HANDLE, nIndex: ::libc::c_int, pvInfo: ::winnt::PVOID, nLength: ::minwindef::DWORD, lpnLengthNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winuser.h:1630:1 */
    pub fn GetUserObjectSecurity(hObj: ::winnt::HANDLE, pSIRequested: ::winnt::PSECURITY_INFORMATION, pSID: ::winnt::PSECURITY_DESCRIPTOR, nLength: ::minwindef::DWORD, lpnLengthNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winuser.h:1590:1 */
    pub fn GetWindow(hWnd: ::windef::HWND, uCmd: ::minwindef::UINT) -> ::windef::HWND; /* winuser.h:9717:1 */
    pub fn GetWindowDC(hWnd: ::windef::HWND) -> ::windef::HDC; /* winuser.h:8102:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetWindowLongW as GetWindowLong; /* winuser.h:9293:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetWindowLongA(hWnd: ::windef::HWND, nIndex: ::libc::c_int) -> ::winnt::LONG; /* winuser.h:9283:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetWindowLongPtrW as GetWindowLongPtr; /* winuser.h:9363:9, winuser.h:9333:9, winuser.h:9363:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetWindowLongW(hWnd: ::windef::HWND, nIndex: ::libc::c_int) -> ::winnt::LONG; /* winuser.h:9289:1 */
    pub fn GetWindowPlacement(hWnd: ::windef::HWND, lpwndpl: *mut ::winuser::WINDOWPLACEMENT) -> ::minwindef::BOOL; /* winuser.h:4543:1 */
    pub fn GetWindowRect(hWnd: ::windef::HWND, lpRect: ::windef::LPRECT) -> ::minwindef::BOOL; /* winuser.h:8539:1 */
    pub fn GetWindowRgn(hWnd: ::windef::HWND, hRgn: ::minwindef::HRGN) -> ::libc::c_int; /* winuser.h:8160:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetWindowTextW as GetWindowText; /* winuser.h:8508:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetWindowTextA(hWnd: ::windef::HWND, lpString: ::winnt::LPSTR, nMaxCount: ::libc::c_int) -> ::libc::c_int; /* winuser.h:8495:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetWindowTextLengthW as GetWindowTextLength; /* winuser.h:8524:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetWindowTextLengthA(hWnd: ::windef::HWND) -> ::libc::c_int; /* winuser.h:8516:1 */
    pub fn GetWindowTextLengthW(hWnd: ::windef::HWND) -> ::libc::c_int; /* winuser.h:8521:1 */
    pub fn GetWindowTextW(hWnd: ::windef::HWND, lpString: ::winnt::LPWSTR, nMaxCount: ::libc::c_int) -> ::libc::c_int; /* winuser.h:8503:1 */
    pub fn GetWindowThreadProcessId(hWnd: ::windef::HWND, lpdwProcessId: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winuser.h:9675:1 */
    pub fn GetWindowWord(hWnd: ::windef::HWND, nIndex: ::libc::c_int) -> ::minwindef::WORD; /* winuser.h:9268:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GrayStringW as GrayString; /* winuser.h:7868:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GrayStringA(hDC: ::windef::HDC, hBrush: ::windef::HBRUSH, lpOutputFunc: ::winuser::GRAYSTRINGPROC, lpData: ::minwindef::LPARAM, nCount: ::libc::c_int, X: ::libc::c_int, Y: ::libc::c_int, nWidth: ::libc::c_int, nHeight: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:7844:1 */
    pub fn GrayStringW(hDC: ::windef::HDC, hBrush: ::windef::HBRUSH, lpOutputFunc: ::winuser::GRAYSTRINGPROC, lpData: ::minwindef::LPARAM, nCount: ::libc::c_int, X: ::libc::c_int, Y: ::libc::c_int, nWidth: ::libc::c_int, nHeight: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:7857:1 */
    pub fn HideCaret(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:8927:1 */
    pub fn HiliteMenuItem(hWnd: ::windef::HWND, hMenu: ::windef::HMENU, uIDHiliteItem: ::minwindef::UINT, uHilite: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7039:1 */
    pub fn InSendMessage() -> ::minwindef::BOOL; /* winuser.h:4074:1 */
    pub fn InflateRect(lprc: ::windef::LPRECT, dx: ::libc::c_int, dy: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:9206:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::InsertMenuW as InsertMenu; /* winuser.h:7170:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn InsertMenuA(hMenu: ::windef::HMENU, uPosition: ::minwindef::UINT, uFlags: ::minwindef::UINT, uIDNewItem: ::basetsd::UINT_PTR, lpNewItem: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winuser.h:7154:1 */
    pub fn InsertMenuW(hMenu: ::windef::HMENU, uPosition: ::minwindef::UINT, uFlags: ::minwindef::UINT, uIDNewItem: ::basetsd::UINT_PTR, lpNewItem: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winuser.h:7163:1 */
    pub fn InternalGetWindowText(hWnd: ::windef::HWND, pString: ::winnt::LPWSTR, cchMaxCount: ::libc::c_int) -> ::libc::c_int; /* winuser.h:12837:1 */
    pub fn IntersectRect(lprcDst: ::windef::LPRECT, lprcSrc1: *const ::windef::RECT, lprcSrc2: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9214:1 */
    pub fn InvalidateRect(hWnd: ::windef::HWND, lpRect: *const ::windef::RECT, bErase: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:8185:1 */
    pub fn InvalidateRgn(hWnd: ::windef::HWND, hRgn: ::minwindef::HRGN, bErase: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:8200:1 */
    pub fn InvertRect(hDC: ::windef::HDC, lprc: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9176:1 */
    pub fn IsCharAlphaA(ch: ::winnt::CHAR) -> ::minwindef::BOOL; /* winuser.h:5528:1 */
    pub fn IsCharAlphaNumericA(ch: ::winnt::CHAR) -> ::minwindef::BOOL; /* winuser.h:5544:1 */
    pub fn IsCharLowerA(ch: ::winnt::CHAR) -> ::minwindef::BOOL; /* winuser.h:5576:1 */
    pub fn IsCharUpperA(ch: ::winnt::CHAR) -> ::minwindef::BOOL; /* winuser.h:5560:1 */
    pub fn IsChild(hWndParent: ::windef::HWND, hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4309:1 */
    pub fn IsClipboardFormatAvailable(format: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:5257:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsDialogMessageW as IsDialogMessage; /* winuser.h:10899:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn IsDialogMessageA(hDlg: ::windef::HWND, lpMsg: ::winuser::LPMSG) -> ::minwindef::BOOL; /* winuser.h:10889:1 */
    pub fn IsDialogMessageW(hDlg: ::windef::HWND, lpMsg: ::winuser::LPMSG) -> ::minwindef::BOOL; /* winuser.h:10895:1 */
    pub fn IsDlgButtonChecked(hDlg: ::windef::HWND, nIDButton: ::libc::c_int) -> ::minwindef::UINT; /* winuser.h:5007:1 */
    pub fn IsHungAppWindow(hwnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:1757:1 */
    pub fn IsIconic(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4626:1 */
    pub fn IsMenu(hMenu: ::windef::HMENU) -> ::minwindef::BOOL; /* winuser.h:4303:1 */
    pub fn IsRectEmpty(lprc: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9246:1 */
    pub fn IsWindow(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4296:1 */
    pub fn IsWindowEnabled(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:6691:1 */
    pub fn IsWindowUnicode(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:6678:1 */
    pub fn IsWindowVisible(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4620:1 */
    pub fn IsZoomed(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4644:1 */
    pub fn KillTimer(hWnd: ::windef::HWND, uIDEvent: ::basetsd::UINT_PTR) -> ::minwindef::BOOL; /* winuser.h:6671:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadAcceleratorsW as LoadAccelerators; /* winuser.h:6707:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadAcceleratorsA(hInstance: ::minwindef::HINSTANCE, lpTableName: ::winnt::LPCSTR) -> ::windef::HACCEL; /* winuser.h:6697:1 */
    pub fn LoadAcceleratorsW(hInstance: ::minwindef::HINSTANCE, lpTableName: ::winnt::LPCWSTR) -> ::windef::HACCEL; /* winuser.h:6703:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadBitmapW as LoadBitmap; /* winuser.h:9998:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadBitmapA(hInstance: ::minwindef::HINSTANCE, lpBitmapName: ::winnt::LPCSTR) -> ::windef::HBITMAP; /* winuser.h:9988:1 */
    pub fn LoadBitmapW(hInstance: ::minwindef::HINSTANCE, lpBitmapName: ::winnt::LPCWSTR) -> ::windef::HBITMAP; /* winuser.h:9994:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadCursorW as LoadCursor; /* winuser.h:10016:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadCursorA(hInstance: ::minwindef::HINSTANCE, lpCursorName: ::winnt::LPCSTR) -> ::windef::HCURSOR; /* winuser.h:10006:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadCursorFromFileW as LoadCursorFromFile; /* winuser.h:10032:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadCursorFromFileA(lpFileName: ::winnt::LPCSTR) -> ::windef::HCURSOR; /* winuser.h:10024:1 */
    pub fn LoadCursorFromFileW(lpFileName: ::winnt::LPCWSTR) -> ::windef::HCURSOR; /* winuser.h:10029:1 */
    pub fn LoadCursorW(hInstance: ::minwindef::HINSTANCE, lpCursorName: ::winnt::LPCWSTR) -> ::windef::HCURSOR; /* winuser.h:10012:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadIconW as LoadIcon; /* winuser.h:10124:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadIconA(hInstance: ::minwindef::HINSTANCE, lpIconName: ::winnt::LPCSTR) -> ::windef::HICON; /* winuser.h:10114:1 */
    pub fn LoadIconW(hInstance: ::minwindef::HINSTANCE, lpIconName: ::winnt::LPCWSTR) -> ::windef::HICON; /* winuser.h:10120:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadKeyboardLayoutW as LoadKeyboardLayout; /* winuser.h:1163:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadKeyboardLayoutA(pwszKLID: ::winnt::LPCSTR, Flags: ::minwindef::UINT) -> ::minwindef::HKL; /* winuser.h:1153:1 */
    pub fn LoadKeyboardLayoutW(pwszKLID: ::winnt::LPCWSTR, Flags: ::minwindef::UINT) -> ::minwindef::HKL; /* winuser.h:1159:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadMenuW as LoadMenu; /* winuser.h:6978:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadMenuA(hInstance: ::minwindef::HINSTANCE, lpMenuName: ::winnt::LPCSTR) -> ::windef::HMENU; /* winuser.h:6968:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadMenuIndirectW as LoadMenuIndirect; /* winuser.h:6994:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadMenuIndirectA(lpMenuTemplate: *const ::libc::c_void) -> ::windef::HMENU; /* winuser.h:6986:1 */
    pub fn LoadMenuIndirectW(lpMenuTemplate: *const ::libc::c_void) -> ::windef::HMENU; /* winuser.h:6991:1 */
    pub fn LoadMenuW(hInstance: ::minwindef::HINSTANCE, lpMenuName: ::winnt::LPCWSTR) -> ::windef::HMENU; /* winuser.h:6974:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadStringW as LoadString; /* libloaderapi.h:453:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadStringA(hInstance: ::minwindef::HINSTANCE, uID: ::minwindef::UINT, lpBuffer: ::winnt::LPSTR, cchBufferMax: ::libc::c_int) -> ::libc::c_int; /* libloaderapi.h:435:1 */
    pub fn LoadStringW(hInstance: ::minwindef::HINSTANCE, uID: ::minwindef::UINT, lpBuffer: ::winnt::LPWSTR, cchBufferMax: ::libc::c_int) -> ::libc::c_int; /* libloaderapi.h:445:1 */
    pub fn LockWindowUpdate(hWndLock: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:8256:1 */
    pub fn LookupIconIdFromDirectory(presbits: ::minwindef::PBYTE, fIcon: ::minwindef::BOOL) -> ::libc::c_int; /* winuser.h:10181:1 */
    pub fn MapDialogRect(hDlg: ::windef::HWND, lpRect: ::windef::LPRECT) -> ::minwindef::BOOL; /* winuser.h:10909:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::MapVirtualKeyW as MapVirtualKey; /* winuser.h:6476:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn MapVirtualKeyA(uCode: ::minwindef::UINT, uMapType: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:6466:1 */
    pub fn MapVirtualKeyW(uCode: ::minwindef::UINT, uMapType: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:6472:1 */
    pub fn MapWindowPoints(hWndFrom: ::windef::HWND, hWndTo: ::windef::HWND, lpPoints: ::windef::LPPOINT, cPoints: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:9000:1 */
    pub fn MessageBeep(uType: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:8824:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::MessageBoxW as MessageBox; /* winuser.h:8703:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn MessageBoxA(hWnd: ::windef::HWND, lpText: ::winnt::LPCSTR, lpCaption: ::winnt::LPCSTR, uType: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:8689:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::MessageBoxExW as MessageBoxEx; /* winuser.h:8751:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn MessageBoxExA(hWnd: ::windef::HWND, lpText: ::winnt::LPCSTR, lpCaption: ::winnt::LPCSTR, uType: ::minwindef::UINT, wLanguageId: ::minwindef::WORD) -> ::libc::c_int; /* winuser.h:8735:1 */
    pub fn MessageBoxExW(hWnd: ::windef::HWND, lpText: ::winnt::LPCWSTR, lpCaption: ::winnt::LPCWSTR, uType: ::minwindef::UINT, wLanguageId: ::minwindef::WORD) -> ::libc::c_int; /* winuser.h:8744:1 */
    pub fn MessageBoxW(hWnd: ::windef::HWND, lpText: ::winnt::LPCWSTR, lpCaption: ::winnt::LPCWSTR, uType: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:8697:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ModifyMenuW as ModifyMenu; /* winuser.h:7216:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ModifyMenuA(hMnu: ::windef::HMENU, uPosition: ::minwindef::UINT, uFlags: ::minwindef::UINT, uIDNewItem: ::basetsd::UINT_PTR, lpNewItem: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winuser.h:7200:1 */
    pub fn ModifyMenuW(hMnu: ::windef::HMENU, uPosition: ::minwindef::UINT, uFlags: ::minwindef::UINT, uIDNewItem: ::basetsd::UINT_PTR, lpNewItem: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winuser.h:7209:1 */
    pub fn MoveWindow(hWnd: ::windef::HWND, X: ::libc::c_int, Y: ::libc::c_int, nWidth: ::libc::c_int, nHeight: ::libc::c_int, bRepaint: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:4520:1 */
    pub fn MsgWaitForMultipleObjects(nCount: ::minwindef::DWORD, pHandles: *const *mut ::libc::c_void, fWaitAll: ::minwindef::BOOL, dwMilliseconds: ::minwindef::DWORD, dwWakeMask: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:6545:1 */
    pub fn MsgWaitForMultipleObjectsEx(nCount: ::minwindef::DWORD, pHandles: *const *mut ::libc::c_void, dwMilliseconds: ::minwindef::DWORD, dwWakeMask: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:6555:1 */
    pub fn OemKeyScan(wOemChar: ::minwindef::WORD) -> ::minwindef::DWORD; /* winuser.h:5703:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OemToCharW as OemToChar; /* winuser.h:5334:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OemToCharA(pSrc: ::winnt::LPCSTR, pDst: ::winnt::LPSTR) -> ::minwindef::BOOL; /* winuser.h:5323:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OemToCharBuffW as OemToCharBuff; /* winuser.h:5374:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OemToCharBuffA(lpszSrc: ::winnt::LPCSTR, lpszDst: ::winnt::LPSTR, cchDstLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:5362:1 */
    pub fn OemToCharBuffW(lpszSrc: ::winnt::LPCSTR, lpszDst: ::winnt::LPWSTR, cchDstLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:5369:1 */
    pub fn OemToCharW(pSrc: ::winnt::LPCSTR, pDst: ::winnt::LPWSTR) -> ::minwindef::BOOL; /* winuser.h:5330:1 */
    pub fn OffsetRect(lprc: ::windef::LPRECT, dx: ::libc::c_int, dy: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:9238:1 */
    pub fn OpenClipboard(hWndNewOwner: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:5142:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenDesktopW as OpenDesktop; /* winuser.h:1387:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenDesktopA(lpszDesktop: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD, fInherit: ::minwindef::BOOL, dwDesiredAccess: ::winnt::ACCESS_MASK) -> ::windef::HDESK; /* winuser.h:1373:1 */
    pub fn OpenDesktopW(lpszDesktop: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, fInherit: ::minwindef::BOOL, dwDesiredAccess: ::winnt::ACCESS_MASK) -> ::windef::HDESK; /* winuser.h:1381:1 */
    pub fn OpenIcon(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4508:1 */
    pub fn OpenInputDesktop(dwFlags: ::minwindef::DWORD, fInherit: ::minwindef::BOOL, dwDesiredAccess: ::winnt::ACCESS_MASK) -> ::windef::HDESK; /* winuser.h:1395:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenWindowStationW as OpenWindowStation; /* winuser.h:1528:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenWindowStationA(lpszWinSta: ::winnt::LPCSTR, fInherit: ::minwindef::BOOL, dwDesiredAccess: ::winnt::ACCESS_MASK) -> ::minwindef::HWINSTA; /* winuser.h:1516:1 */
    pub fn OpenWindowStationW(lpszWinSta: ::winnt::LPCWSTR, fInherit: ::minwindef::BOOL, dwDesiredAccess: ::winnt::ACCESS_MASK) -> ::minwindef::HWINSTA; /* winuser.h:1523:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PeekMessageW as PeekMessage; /* winuser.h:3389:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PeekMessageA(lpMsg: ::winuser::LPMSG, hWnd: ::windef::HWND, wMsgFilterMin: ::minwindef::UINT, wMsgFilterMax: ::minwindef::UINT, wRemoveMsg: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:3373:1 */
    pub fn PeekMessageW(lpMsg: ::winuser::LPMSG, hWnd: ::windef::HWND, wMsgFilterMin: ::minwindef::UINT, wMsgFilterMax: ::minwindef::UINT, wRemoveMsg: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:3382:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PostMessageW as PostMessage; /* winuser.h:3895:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PostMessageA(hWnd: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:3881:1 */
    pub fn PostMessageW(hWnd: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:3889:1 */
    pub fn PostQuitMessage(nExitCode: ::libc::c_int); /* winuser.h:4014:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PostThreadMessageW as PostThreadMessage; /* winuser.h:3917:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PostThreadMessageA(idThread: ::minwindef::DWORD, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:3903:1 */
    pub fn PostThreadMessageW(idThread: ::minwindef::DWORD, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:3911:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PrivateExtractIconsW as PrivateExtractIcons; /* winuser.h:10155:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PrivateExtractIconsA(szFileName: ::winnt::LPCSTR, nIconIndex: ::libc::c_int, cxIcon: ::libc::c_int, cyIcon: ::libc::c_int, phicon: *mut *mut ::windef::HICON__, piconid: *mut ::libc::c_uint, nIcons: ::minwindef::UINT, flags: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:10133:1 */
    pub fn PrivateExtractIconsW(szFileName: ::winnt::LPCWSTR, nIconIndex: ::libc::c_int, cxIcon: ::libc::c_int, cyIcon: ::libc::c_int, phicon: *mut *mut ::windef::HICON__, piconid: *mut ::libc::c_uint, nIcons: ::minwindef::UINT, flags: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:10145:1 */
    pub fn PtInRect(lprc: *const ::windef::RECT, pt: ::windef::POINT) -> ::minwindef::BOOL; /* winuser.h:9259:1 */
    pub fn RedrawWindow(hWnd: ::windef::HWND, lprcUpdate: *const ::windef::RECT, hrgnUpdate: ::minwindef::HRGN, flags: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:8216:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegisterClassW as RegisterClass; /* winuser.h:4131:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegisterClassA(lpWndClass: *const ::winuser::WNDCLASSA) -> ::minwindef::ATOM; /* winuser.h:4123:1 */
    pub fn RegisterClassW(lpWndClass: *const ::winuser::WNDCLASSW) -> ::minwindef::ATOM; /* winuser.h:4128:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegisterClipboardFormatW as RegisterClipboardFormat; /* winuser.h:5211:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegisterClipboardFormatA(lpszFormat: ::winnt::LPCSTR) -> ::minwindef::UINT; /* winuser.h:5203:1 */
    pub fn RegisterClipboardFormatW(lpszFormat: ::winnt::LPCWSTR) -> ::minwindef::UINT; /* winuser.h:5208:1 */
    pub fn RegisterHotKey(hWnd: ::windef::HWND, id: ::libc::c_int, fsModifiers: ::minwindef::UINT, vk: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:3419:1 */
    pub fn RegisterShellHookWindow(hwnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:9589:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegisterWindowMessageW as RegisterWindowMessage; /* winuser.h:2541:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegisterWindowMessageA(lpString: ::winnt::LPCSTR) -> ::minwindef::UINT; /* winuser.h:2533:1 */
    pub fn RegisterWindowMessageW(lpString: ::winnt::LPCWSTR) -> ::minwindef::UINT; /* winuser.h:2538:1 */
    pub fn ReleaseCapture() -> ::minwindef::BOOL; /* winuser.h:6539:1 */
    pub fn ReleaseDC(hWnd: ::windef::HWND, hDC: ::windef::HDC) -> ::libc::c_int; /* winuser.h:8108:1 */
    pub fn RemoveMenu(hMenu: ::windef::HMENU, uPosition: ::minwindef::UINT, uFlags: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7223:8 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RemovePropW as RemoveProp; /* winuser.h:8430:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RemovePropA(hWnd: ::windef::HWND, lpString: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winuser.h:8420:1 */
    pub fn RemovePropW(hWnd: ::windef::HWND, lpString: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winuser.h:8426:1 */
    pub fn ReplyMessage(lResult: ::minwindef::LRESULT) -> ::minwindef::BOOL; /* winuser.h:3959:1 */
    pub fn ScreenToClient(hWnd: ::windef::HWND, lpPoint: ::windef::LPPOINT) -> ::minwindef::BOOL; /* winuser.h:8959:1 */
    pub fn ScrollDC(hDC: ::windef::HDC, dx: ::libc::c_int, dy: ::libc::c_int, lprcScroll: *const ::windef::RECT, lprcClip: *const ::windef::RECT, hrgnUpdate: ::minwindef::HRGN, lprcUpdate: ::windef::LPRECT) -> ::minwindef::BOOL; /* winuser.h:8272:1 */
    pub fn ScrollWindow(hWnd: ::windef::HWND, XAmount: ::libc::c_int, YAmount: ::libc::c_int, lpRect: *const ::windef::RECT, lpClipRect: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:8262:1 */
    pub fn ScrollWindowEx(hWnd: ::windef::HWND, dx: ::libc::c_int, dy: ::libc::c_int, prcScroll: *const ::windef::RECT, prcClip: *const ::windef::RECT, hrgnUpdate: ::minwindef::HRGN, prcUpdate: ::windef::LPRECT, flags: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:8284:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SendDlgItemMessageW as SendDlgItemMessage; /* winuser.h:5030:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SendDlgItemMessageA(hDlg: ::windef::HWND, nIDDlgItem: ::libc::c_int, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:5014:1 */
    pub fn SendDlgItemMessageW(hDlg: ::windef::HWND, nIDDlgItem: ::libc::c_int, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:5023:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SendMessageW as SendMessage; /* winuser.h:3565:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SendMessageA(hWnd: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:3551:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SendMessageCallbackW as SendMessageCallback; /* winuser.h:3667:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SendMessageCallbackA(hWnd: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM, lpResultCallBack: ::winuser::SENDASYNCPROC, dwData: ::basetsd::ULONG_PTR) -> ::minwindef::BOOL; /* winuser.h:3649:1 */
    pub fn SendMessageCallbackW(hWnd: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM, lpResultCallBack: ::winuser::SENDASYNCPROC, dwData: ::basetsd::ULONG_PTR) -> ::minwindef::BOOL; /* winuser.h:3659:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SendMessageTimeoutW as SendMessageTimeout; /* winuser.h:3619:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SendMessageTimeoutA(hWnd: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM, fuFlags: ::minwindef::UINT, uTimeout: ::minwindef::UINT, lpdwResult: ::basetsd::PDWORD_PTR) -> ::minwindef::LRESULT; /* winuser.h:3599:1 */
    pub fn SendMessageTimeoutW(hWnd: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM, fuFlags: ::minwindef::UINT, uTimeout: ::minwindef::UINT, lpdwResult: ::basetsd::PDWORD_PTR) -> ::minwindef::LRESULT; /* winuser.h:3610:1 */
    pub fn SendMessageW(hWnd: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:3559:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SendNotifyMessageW as SendNotifyMessage; /* winuser.h:3641:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SendNotifyMessageA(hWnd: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:3627:1 */
    pub fn SendNotifyMessageW(hWnd: ::windef::HWND, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:3635:1 */
    pub fn SetActiveWindow(hWnd: ::windef::HWND) -> ::windef::HWND; /* winuser.h:8004:1 */
    pub fn SetCapture(hWnd: ::windef::HWND) -> ::windef::HWND; /* winuser.h:6533:1 */
    pub fn SetCaretBlinkTime(uMSeconds: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:8915:1 */
    pub fn SetCaretPos(X: ::libc::c_int, Y: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:8939:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetClassLongW as SetClassLong; /* winuser.h:9426:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetClassLongA(hWnd: ::windef::HWND, nIndex: ::libc::c_int, dwNewLong: ::winnt::LONG) -> ::minwindef::DWORD; /* winuser.h:9414:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetClassLongPtrW as SetClassLongPtr; /* winuser.h:9484:9, winuser.h:9466:9, winuser.h:9484:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetClassLongW(hWnd: ::windef::HWND, nIndex: ::libc::c_int, dwNewLong: ::winnt::LONG) -> ::minwindef::DWORD; /* winuser.h:9421:1 */
    pub fn SetClassWord(hWnd: ::windef::HWND, nIndex: ::libc::c_int, wNewWord: ::minwindef::WORD) -> ::minwindef::WORD; /* winuser.h:9388:1 */
    pub fn SetClipboardData(uFormat: ::minwindef::UINT, hMem: ::winnt::HANDLE) -> ::winnt::HANDLE; /* winuser.h:5190:1 */
    pub fn SetClipboardViewer(hWndNewViewer: ::windef::HWND) -> ::windef::HWND; /* winuser.h:5171:1 */
    pub fn SetCursor(hCursor: ::windef::HCURSOR) -> ::windef::HCURSOR; /* winuser.h:8861:1 */
    pub fn SetCursorPos(X: ::libc::c_int, Y: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:8845:1 */
    pub fn SetDebugErrorLevel(dwLevel: ::minwindef::DWORD); /* winuser.h:12810:1 */
    pub fn SetDlgItemInt(hDlg: ::windef::HWND, nIDDlgItem: ::libc::c_int, uValue: ::minwindef::UINT, bSigned: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:4928:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetDlgItemTextW as SetDlgItemText; /* winuser.h:4958:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetDlgItemTextA(hDlg: ::windef::HWND, nIDDlgItem: ::libc::c_int, lpString: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winuser.h:4946:1 */
    pub fn SetDlgItemTextW(hDlg: ::windef::HWND, nIDDlgItem: ::libc::c_int, lpString: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winuser.h:4953:1 */
    pub fn SetDoubleClickTime(_: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:4117:1 */
    pub fn SetFocus(hWnd: ::windef::HWND) -> ::windef::HWND; /* winuser.h:5594:1 */
    pub fn SetForegroundWindow(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:8033:1 */
    pub fn SetKeyboardState(lpKeyState: ::minwindef::LPBYTE) -> ::minwindef::BOOL; /* winuser.h:5637:1 */
    pub fn SetLastErrorEx(dwErrCode: ::minwindef::DWORD, dwType: ::minwindef::DWORD); /* winuser.h:12830:1 */
    pub fn SetMenu(hWnd: ::windef::HWND, hMenu: ::windef::HMENU) -> ::minwindef::BOOL; /* winuser.h:7008:1 */
    pub fn SetMenuItemBitmaps(hMenu: ::windef::HMENU, uPosition: ::minwindef::UINT, uFlags: ::minwindef::UINT, hBitmapUnchecked: ::windef::HBITMAP, hBitmapChecked: ::windef::HBITMAP) -> ::minwindef::BOOL; /* winuser.h:7239:1 */
    pub fn SetMessageQueue(cMessagesMax: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:3367:1 */
    pub fn SetParent(hWndChild: ::windef::HWND, hWndNewParent: ::windef::HWND) -> ::windef::HWND; /* winuser.h:9523:1 */
    pub fn SetProcessWindowStation(hWinSta: ::minwindef::HWINSTA) -> ::minwindef::BOOL; /* winuser.h:1560:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetPropW as SetProp; /* winuser.h:8394:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetPropA(hWnd: ::windef::HWND, lpString: ::winnt::LPCSTR, hData: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winuser.h:8382:1 */
    pub fn SetPropW(hWnd: ::windef::HWND, lpString: ::winnt::LPCWSTR, hData: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winuser.h:8389:1 */
    pub fn SetRect(lprc: ::windef::LPRECT, xLeft: ::libc::c_int, yTop: ::libc::c_int, xRight: ::libc::c_int, yBottom: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:9183:1 */
    pub fn SetRectEmpty(lprc: ::windef::LPRECT) -> ::minwindef::BOOL; /* winuser.h:9193:1 */
    pub fn SetScrollPos(hWnd: ::windef::HWND, nBar: ::libc::c_int, nPos: ::libc::c_int, bRedraw: ::minwindef::BOOL) -> ::libc::c_int; /* winuser.h:8312:1 */
    pub fn SetScrollRange(hWnd: ::windef::HWND, nBar: ::libc::c_int, nMinPos: ::libc::c_int, nMaxPos: ::libc::c_int, bRedraw: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:8328:1 */
    pub fn SetSysColors(cElements: ::libc::c_int, lpaElements: *const ::libc::c_int, lpaRgbValues: *const ::libc::c_ulong) -> ::minwindef::BOOL; /* winuser.h:9137:1 */
    pub fn SetSystemCursor(hcur: ::windef::HCURSOR, id: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:10098:1 */
    pub fn SetThreadDesktop(hDesktop: ::windef::HDESK) -> ::minwindef::BOOL; /* winuser.h:1440:1 */
    pub fn SetTimer(hWnd: ::windef::HWND, nIDEvent: ::basetsd::UINT_PTR, uElapse: ::minwindef::UINT, lpTimerFunc: ::winuser::TIMERPROC) -> ::basetsd::UINT_PTR; /* winuser.h:6642:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetUserObjectInformationW as SetUserObjectInformation; /* winuser.h:1659:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetUserObjectInformationA(hObj: ::winnt::HANDLE, nIndex: ::libc::c_int, pvInfo: ::winnt::PVOID, nLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:1645:1 */
    pub fn SetUserObjectInformationW(hObj: ::winnt::HANDLE, nIndex: ::libc::c_int, pvInfo: ::winnt::PVOID, nLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:1653:1 */
    pub fn SetUserObjectSecurity(hObj: ::winnt::HANDLE, pSIRequested: ::winnt::PSECURITY_INFORMATION, pSID: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* winuser.h:1582:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetWindowLongW as SetWindowLong; /* winuser.h:9313:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetWindowLongA(hWnd: ::windef::HWND, nIndex: ::libc::c_int, dwNewLong: ::winnt::LONG) -> ::winnt::LONG; /* winuser.h:9301:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetWindowLongPtrW as SetWindowLongPtr; /* winuser.h:9371:9, winuser.h:9353:9, winuser.h:9371:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetWindowLongW(hWnd: ::windef::HWND, nIndex: ::libc::c_int, dwNewLong: ::winnt::LONG) -> ::winnt::LONG; /* winuser.h:9308:1 */
    pub fn SetWindowPlacement(hWnd: ::windef::HWND, lpwndpl: *const ::winuser::WINDOWPLACEMENT) -> ::minwindef::BOOL; /* winuser.h:4550:1 */
    pub fn SetWindowPos(hWnd: ::windef::HWND, hWndInsertAfter: ::windef::HWND, X: ::libc::c_int, Y: ::libc::c_int, cx: ::libc::c_int, cy: ::libc::c_int, uFlags: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:4531:1 */
    pub fn SetWindowRgn(hWnd: ::windef::HWND, hRgn: ::minwindef::HRGN, bRedraw: ::minwindef::BOOL) -> ::libc::c_int; /* winuser.h:8145:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetWindowTextW as SetWindowText; /* winuser.h:8486:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetWindowTextA(hWnd: ::windef::HWND, lpString: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winuser.h:8476:1 */
    pub fn SetWindowTextW(hWnd: ::windef::HWND, lpString: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winuser.h:8482:1 */
    pub fn SetWindowWord(hWnd: ::windef::HWND, nIndex: ::libc::c_int, wNewWord: ::minwindef::WORD) -> ::minwindef::WORD; /* winuser.h:9275:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetWindowsHookW as SetWindowsHook; /* winuser.h:9739:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetWindowsHookA(nFilterType: ::libc::c_int, pfnFilterProc: ::winuser::HOOKPROC) -> ::windef::HHOOK; /* winuser.h:9729:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetWindowsHookExW as SetWindowsHookEx; /* winuser.h:9790:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetWindowsHookExA(idHook: ::libc::c_int, lpfn: ::winuser::HOOKPROC, hmod: ::minwindef::HINSTANCE, dwThreadId: ::minwindef::DWORD) -> ::windef::HHOOK; /* winuser.h:9776:1 */
    pub fn SetWindowsHookExW(idHook: ::libc::c_int, lpfn: ::winuser::HOOKPROC, hmod: ::minwindef::HINSTANCE, dwThreadId: ::minwindef::DWORD) -> ::windef::HHOOK; /* winuser.h:9784:1 */
    pub fn SetWindowsHookW(nFilterType: ::libc::c_int, pfnFilterProc: ::winuser::HOOKPROC) -> ::windef::HHOOK; /* winuser.h:9735:1 */
    pub fn ShowCaret(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:8933:1 */
    pub fn ShowCursor(bShow: ::minwindef::BOOL) -> ::libc::c_int; /* winuser.h:8839:1 */
    pub fn ShowOwnedPopups(hWnd: ::windef::HWND, fShow: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:4501:1 */
    pub fn ShowScrollBar(hWnd: ::windef::HWND, wBar: ::libc::c_int, bShow: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:8347:1 */
    pub fn ShowWindow(hWnd: ::windef::HWND, nCmdShow: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:4322:1 */
    pub fn ShutdownBlockReasonCreate(hWnd: ::windef::HWND, pwszReason: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winuser.h:14961:1 */
    pub fn ShutdownBlockReasonDestroy(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:14976:1 */
    pub fn ShutdownBlockReasonQuery(hWnd: ::windef::HWND, pwszBuff: ::winnt::LPWSTR, pcchBuff: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winuser.h:14968:1 */
    pub fn SubtractRect(lprcDst: ::windef::LPRECT, lprcSrc1: *const ::windef::RECT, lprcSrc2: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9230:1 */
    pub fn SwapMouseButton(fSwap: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:3503:1 */
    pub fn SwitchDesktop(hDesktop: ::windef::HDESK) -> ::minwindef::BOOL; /* winuser.h:1433:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SystemParametersInfoW as SystemParametersInfo; /* winuser.h:12555:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SystemParametersInfoA(uiAction: ::minwindef::UINT, uiParam: ::minwindef::UINT, pvParam: ::winnt::PVOID, fWinIni: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:12540:1 */
    pub fn SystemParametersInfoW(uiAction: ::minwindef::UINT, uiParam: ::minwindef::UINT, pvParam: ::winnt::PVOID, fWinIni: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:12549:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::TabbedTextOutW as TabbedTextOut; /* winuser.h:7966:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn TabbedTextOutA(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, lpString: ::winnt::LPCSTR, chCount: ::libc::c_int, nTabPositions: ::libc::c_int, lpnTabStopPositions: *const ::libc::c_int, nTabOrigin: ::libc::c_int) -> ::winnt::LONG; /* winuser.h:7944:1 */
    pub fn TabbedTextOutW(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, lpString: ::winnt::LPCWSTR, chCount: ::libc::c_int, nTabPositions: ::libc::c_int, lpnTabStopPositions: *const ::libc::c_int, nTabOrigin: ::libc::c_int) -> ::winnt::LONG; /* winuser.h:7956:1 */
    pub fn ToAscii(uVirtKey: ::minwindef::UINT, uScanCode: ::minwindef::UINT, lpKeyState: *const ::libc::c_uchar, lpChar: ::minwindef::LPWORD, uFlags: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:5669:1 */
    pub fn ToUnicode(wVirtKey: ::minwindef::UINT, wScanCode: ::minwindef::UINT, lpKeyState: *const ::libc::c_uchar, pwszBuff: ::winnt::LPWSTR, cchBuff: ::libc::c_int, wFlags: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:5692:1 */
    pub fn TrackPopupMenu(hMenu: ::windef::HMENU, uFlags: ::minwindef::UINT, x: ::libc::c_int, y: ::libc::c_int, nReserved: ::libc::c_int, hWnd: ::windef::HWND, prcRect: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:7255:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::TranslateAcceleratorW as TranslateAccelerator; /* winuser.h:6773:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn TranslateAcceleratorA(hWnd: ::windef::HWND, hAccTable: ::windef::HACCEL, lpMsg: ::winuser::LPMSG) -> ::libc::c_int; /* winuser.h:6761:1 */
    pub fn TranslateAcceleratorW(hWnd: ::windef::HWND, hAccTable: ::windef::HACCEL, lpMsg: ::winuser::LPMSG) -> ::libc::c_int; /* winuser.h:6768:1 */
    pub fn TranslateMDISysAccel(hWndClient: ::windef::HWND, lpMsg: ::winuser::LPMSG) -> ::minwindef::BOOL; /* winuser.h:11526:1 */
    pub fn TranslateMessage(lpMsg: *const ::winuser::MSG) -> ::minwindef::BOOL; /* winuser.h:3327:1 */
    pub fn UnhookWindowsHook(nCode: ::libc::c_int, pfnFilterProc: ::winuser::HOOKPROC) -> ::minwindef::BOOL; /* winuser.h:9769:1 */
    pub fn UnhookWindowsHookEx(hhk: ::windef::HHOOK) -> ::minwindef::BOOL; /* winuser.h:9798:1 */
    pub fn UnionRect(lprcDst: ::windef::LPRECT, lprcSrc1: *const ::windef::RECT, lprcSrc2: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9222:1 */
    pub fn UnloadKeyboardLayout(hkl: ::minwindef::HKL) -> ::minwindef::BOOL; /* winuser.h:1202:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::UnregisterClassW as UnregisterClass; /* winuser.h:4149:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn UnregisterClassA(lpClassName: ::winnt::LPCSTR, hInstance: ::minwindef::HINSTANCE) -> ::minwindef::BOOL; /* winuser.h:4139:1 */
    pub fn UnregisterClassW(lpClassName: ::winnt::LPCWSTR, hInstance: ::minwindef::HINSTANCE) -> ::minwindef::BOOL; /* winuser.h:4145:1 */
    pub fn UnregisterHotKey(hWnd: ::windef::HWND, id: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:3428:1 */
    pub fn UpdateWindow(hWnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:7998:1 */
    pub fn ValidateRect(hWnd: ::windef::HWND, lpRect: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:8193:1 */
    pub fn ValidateRgn(hWnd: ::windef::HWND, hRgn: ::minwindef::HRGN) -> ::minwindef::BOOL; /* winuser.h:8208:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VkKeyScanW as VkKeyScan; /* winuser.h:5717:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VkKeyScanA(ch: ::winnt::CHAR) -> ::winnt::SHORT; /* winuser.h:5709:1 */
    pub fn VkKeyScanW(ch: ::winnt::WCHAR) -> ::winnt::SHORT; /* winuser.h:5714:1 */
    pub fn WaitForInputIdle(hProcess: ::winnt::HANDLE, dwMilliseconds: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:3975:1 */
    pub fn WaitMessage() -> ::minwindef::BOOL; /* winuser.h:3965:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WinHelpW as WinHelp; /* winuser.h:11722:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WinHelpA(hWndMain: ::windef::HWND, lpszHelp: ::winnt::LPCSTR, uCommand: ::minwindef::UINT, dwData: ::basetsd::ULONG_PTR) -> ::minwindef::BOOL; /* winuser.h:11708:1 */
    pub fn WinHelpW(hWndMain: ::windef::HWND, lpszHelp: ::winnt::LPCWSTR, uCommand: ::minwindef::UINT, dwData: ::basetsd::ULONG_PTR) -> ::minwindef::BOOL; /* winuser.h:11716:1 */
    pub fn WindowFromDC(hDC: ::windef::HDC) -> ::windef::HWND; /* winuser.h:8059:1 */
    pub fn WindowFromPoint(Point: ::windef::POINT) -> ::windef::HWND; /* winuser.h:9009:1 */
    pub fn keybd_event(bVk: ::minwindef::BYTE, bScan: ::minwindef::BYTE, dwFlags: ::minwindef::DWORD, dwExtraInfo: ::basetsd::ULONG_PTR); /* winuser.h:5751:1 */
    pub fn mouse_event(dwFlags: ::minwindef::DWORD, dx: ::minwindef::DWORD, dy: ::minwindef::DWORD, dwData: ::minwindef::DWORD, dwExtraInfo: ::basetsd::ULONG_PTR); /* winuser.h:5785:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::wsprintfW as wsprintf; /* winuser.h:307:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::wvsprintfW as wvsprintf; /* winuser.h:287:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn wvsprintfA(_: ::winnt::LPSTR, _: ::winnt::LPCSTR, arglist: ::libc::c_int) -> ::libc::c_int; /* winuser.h:275:1 */
    pub fn wvsprintfW(_: ::winnt::LPWSTR, _: ::winnt::LPCWSTR, arglist: ::libc::c_int) -> ::libc::c_int; /* winuser.h:282:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn ActivateKeyboardLayout(hkl: ::minwindef::HKL, Flags: ::minwindef::UINT) -> ::minwindef::HKL; /* winuser.h:1173:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::BroadcastSystemMessageW as BroadcastSystemMessage; /* winuser.h:3735:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn BroadcastSystemMessageA(flags: ::minwindef::DWORD, lpInfo: ::minwindef::LPDWORD, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::libc::c_long; /* winuser.h:3719:1 */
    pub fn BroadcastSystemMessageW(flags: ::minwindef::DWORD, lpInfo: ::minwindef::LPDWORD, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM) -> ::libc::c_long; /* winuser.h:3728:1 */
    pub fn CascadeWindows(hwndParent: ::windef::HWND, wHow: ::minwindef::UINT, lpRect: *const ::windef::RECT, cKids: ::minwindef::UINT, lpKids: *const *mut ::windef::HWND__) -> ::minwindef::WORD; /* winuser.h:11585:8 */
    pub fn CharNextExA(CodePage: ::minwindef::WORD, lpCurrentChar: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD) -> ::winnt::LPSTR; /* winuser.h:5485:1 */
    pub fn CharPrevExA(CodePage: ::minwindef::WORD, lpStart: ::winnt::LPCSTR, lpCurrentChar: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD) -> ::winnt::LPSTR; /* winuser.h:5493:1 */
    pub fn CheckMenuRadioItem(hmenu: ::windef::HMENU, first: ::minwindef::UINT, last: ::minwindef::UINT, check: ::minwindef::UINT, flags: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:9908:1 */
    pub fn ChildWindowFromPointEx(hwnd: ::windef::HWND, pt: ::windef::POINT, flags: ::minwindef::UINT) -> ::windef::HWND; /* winuser.h:9042:1 */
    pub fn CopyImage(h: ::winnt::HANDLE, type_: ::minwindef::UINT, cx: ::libc::c_int, cy: ::libc::c_int, flags: ::minwindef::UINT) -> ::winnt::HANDLE; /* winuser.h:10287:1 */
    pub fn CreateIconFromResourceEx(presbits: ::minwindef::PBYTE, dwResSize: ::minwindef::DWORD, fIcon: ::minwindef::BOOL, dwVer: ::minwindef::DWORD, cxDesired: ::libc::c_int, cyDesired: ::libc::c_int, Flags: ::minwindef::UINT) -> ::windef::HICON; /* winuser.h:10210:1 */
    pub fn DragDetect(hwnd: ::windef::HWND, pt: ::windef::POINT) -> ::minwindef::BOOL; /* winuser.h:7651:1 */
    pub fn DragObject(hwndParent: ::windef::HWND, hwndFrom: ::windef::HWND, fmt: ::minwindef::UINT, data: ::basetsd::ULONG_PTR, hcur: ::windef::HCURSOR) -> ::minwindef::DWORD; /* winuser.h:7641:1 */
    pub fn DrawAnimatedRects(hwnd: ::windef::HWND, idAni: ::libc::c_int, lprcFrom: *const ::windef::RECT, lprcTo: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:2981:1 */
    pub fn DrawCaption(hwnd: ::windef::HWND, hdc: ::windef::HDC, lprect: *const ::windef::RECT, flags: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:2962:1 */
    pub fn DrawEdge(hdc: ::windef::HDC, qrc: ::windef::LPRECT, edge: ::minwindef::UINT, grfFlags: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:2869:1 */
    pub fn DrawFrameControl(_: ::windef::HDC, _: ::windef::LPRECT, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:2932:1 */
    pub fn DrawIconEx(hdc: ::windef::HDC, xLeft: ::libc::c_int, yTop: ::libc::c_int, hIcon: ::windef::HICON, cxWidth: ::libc::c_int, cyWidth: ::libc::c_int, istepIfAniCur: ::minwindef::UINT, hbrFlickerFreeDraw: ::windef::HBRUSH, diFlags: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:10303:24 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::DrawStateW as DrawState; /* winuser.h:7928:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn DrawStateA(hdc: ::windef::HDC, hbrFore: ::windef::HBRUSH, qfnCallBack: ::winuser::DRAWSTATEPROC, lData: ::minwindef::LPARAM, wData: ::minwindef::WPARAM, x: ::libc::c_int, y: ::libc::c_int, cx: ::libc::c_int, cy: ::libc::c_int, uFlags: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7902:1 */
    pub fn DrawStateW(hdc: ::windef::HDC, hbrFore: ::windef::HBRUSH, qfnCallBack: ::winuser::DRAWSTATEPROC, lData: ::minwindef::LPARAM, wData: ::minwindef::WPARAM, x: ::libc::c_int, y: ::libc::c_int, cx: ::libc::c_int, cy: ::libc::c_int, uFlags: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7916:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::DrawTextExW as DrawTextEx; /* winuser.h:7827:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn DrawTextExA(hdc: ::windef::HDC, lpchText: ::winnt::LPSTR, cchText: ::libc::c_int, lprc: ::windef::LPRECT, format: ::minwindef::UINT, lpdtp: ::winuser::LPDRAWTEXTPARAMS) -> ::libc::c_int; /* winuser.h:7802:1 */
    pub fn DrawTextExW(hdc: ::windef::HDC, lpchText: ::winnt::LPWSTR, cchText: ::libc::c_int, lprc: ::windef::LPRECT, format: ::minwindef::UINT, lpdtp: ::winuser::LPDRAWTEXTPARAMS) -> ::libc::c_int; /* winuser.h:7816:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::FindWindowExW as FindWindowEx; /* winuser.h:9572:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn FindWindowExA(hWndParent: ::windef::HWND, hWndChildAfter: ::windef::HWND, lpszClass: ::winnt::LPCSTR, lpszWindow: ::winnt::LPCSTR) -> ::windef::HWND; /* winuser.h:9558:1 */
    pub fn FindWindowExW(hWndParent: ::windef::HWND, hWndChildAfter: ::windef::HWND, lpszClass: ::winnt::LPCWSTR, lpszWindow: ::winnt::LPCWSTR) -> ::windef::HWND; /* winuser.h:9566:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::GetClassInfoExW as GetClassInfoEx; /* winuser.h:4210:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn GetClassInfoExA(hInstance: ::minwindef::HINSTANCE, lpszClass: ::winnt::LPCSTR, lpwcx: ::winuser::LPWNDCLASSEXA) -> ::minwindef::BOOL; /* winuser.h:4197:1 */
    pub fn GetClassInfoExW(hInstance: ::minwindef::HINSTANCE, lpszClass: ::winnt::LPCWSTR, lpwcx: ::winuser::LPWNDCLASSEXW) -> ::minwindef::BOOL; /* winuser.h:4205:1 */
    pub fn GetKeyboardLayout(idThread: ::minwindef::DWORD) -> ::minwindef::HKL; /* winuser.h:1232:1 */
    pub fn GetKeyboardLayoutList(nBuff: ::libc::c_int, lpList: *mut *mut ::minwindef::HKL__) -> ::libc::c_int; /* winuser.h:1225:1 */
    pub fn GetMenuContextHelpId(_: ::windef::HMENU) -> ::minwindef::DWORD; /* winuser.h:8604:1 */
    pub fn GetMenuDefaultItem(hMenu: ::windef::HMENU, fByPos: ::minwindef::UINT, gmdiFlags: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:7530:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::GetMenuItemInfoW as GetMenuItemInfo; /* winuser.h:7496:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn GetMenuItemInfoA(hmenu: ::windef::HMENU, item: ::minwindef::UINT, fByPosition: ::minwindef::BOOL, lpmii: ::winuser::LPMENUITEMINFOA) -> ::minwindef::BOOL; /* winuser.h:7482:1 */
    pub fn GetMenuItemInfoW(hmenu: ::windef::HMENU, item: ::minwindef::UINT, fByPosition: ::minwindef::BOOL, lpmii: ::winuser::LPMENUITEMINFOW) -> ::minwindef::BOOL; /* winuser.h:7490:1 */
    pub fn GetMenuItemRect(hWnd: ::windef::HWND, hMenu: ::windef::HMENU, uItem: ::minwindef::UINT, lprcItem: ::windef::LPRECT) -> ::minwindef::BOOL; /* winuser.h:7546:1 */
    pub fn GetScrollInfo(hwnd: ::windef::HWND, nBar: ::libc::c_int, lpsi: ::winuser::LPSCROLLINFO) -> ::minwindef::BOOL; /* winuser.h:11397:1 */
    pub fn GetShellWindow() -> ::windef::HWND; /* winuser.h:9580:1 */
    pub fn GetSysColorBrush(nIndex: ::libc::c_int) -> ::windef::HBRUSH; /* winuser.h:9128:1 */
    pub fn GetWindowContextHelpId(_: ::windef::HWND) -> ::minwindef::DWORD; /* winuser.h:8591:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::InsertMenuItemW as InsertMenuItem; /* winuser.h:7474:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn InsertMenuItemA(hmenu: ::windef::HMENU, item: ::minwindef::UINT, fByPosition: ::minwindef::BOOL, lpmi: ::winuser::LPCMENUITEMINFOA) -> ::minwindef::BOOL; /* winuser.h:7460:1 */
    pub fn InsertMenuItemW(hmenu: ::windef::HMENU, item: ::minwindef::UINT, fByPosition: ::minwindef::BOOL, lpmi: ::winuser::LPCMENUITEMINFOW) -> ::minwindef::BOOL; /* winuser.h:7468:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::LoadImageW as LoadImage; /* winuser.h:10279:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn LoadImageA(hInst: ::minwindef::HINSTANCE, name: ::winnt::LPCSTR, type_: ::minwindef::UINT, cx: ::libc::c_int, cy: ::libc::c_int, fuLoad: ::minwindef::UINT) -> ::winnt::HANDLE; /* winuser.h:10261:1 */
    pub fn LoadImageW(hInst: ::minwindef::HINSTANCE, name: ::winnt::LPCWSTR, type_: ::minwindef::UINT, cx: ::libc::c_int, cy: ::libc::c_int, fuLoad: ::minwindef::UINT) -> ::winnt::HANDLE; /* winuser.h:10271:1 */
    pub fn LookupIconIdFromDirectoryEx(presbits: ::minwindef::PBYTE, fIcon: ::minwindef::BOOL, cxDesired: ::libc::c_int, cyDesired: ::libc::c_int, Flags: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:10189:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::MapVirtualKeyExW as MapVirtualKeyEx; /* winuser.h:6497:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn MapVirtualKeyExA(uCode: ::minwindef::UINT, uMapType: ::minwindef::UINT, dwhkl: ::minwindef::HKL) -> ::minwindef::UINT; /* winuser.h:6485:1 */
    pub fn MapVirtualKeyExW(uCode: ::minwindef::UINT, uMapType: ::minwindef::UINT, dwhkl: ::minwindef::HKL) -> ::minwindef::UINT; /* winuser.h:6492:1 */
    pub fn MenuItemFromPoint(hWnd: ::windef::HWND, hMenu: ::windef::HMENU, ptScreen: ::windef::POINT) -> ::libc::c_int; /* winuser.h:7555:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::MessageBoxIndirectW as MessageBoxIndirect; /* winuser.h:8807:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn MessageBoxIndirectA(lpmbp: *const ::winuser::MSGBOXPARAMSA) -> ::libc::c_int; /* winuser.h:8799:1 */
    pub fn MessageBoxIndirectW(lpmbp: *const ::winuser::MSGBOXPARAMSW) -> ::libc::c_int; /* winuser.h:8804:1 */
    pub fn PaintDesktop(hdc: ::windef::HDC) -> ::minwindef::BOOL; /* winuser.h:8018:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::RegisterClassExW as RegisterClassEx; /* winuser.h:4188:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn RegisterClassExA(_: *const ::winuser::WNDCLASSEXA) -> ::minwindef::ATOM; /* winuser.h:4180:1 */
    pub fn RegisterClassExW(_: *const ::winuser::WNDCLASSEXW) -> ::minwindef::ATOM; /* winuser.h:4185:1 */
    pub fn SetMenuContextHelpId(_: ::windef::HMENU, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:8597:1 */
    pub fn SetMenuDefaultItem(hMenu: ::windef::HMENU, uItem: ::minwindef::UINT, fByPos: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7538:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::SetMenuItemInfoW as SetMenuItemInfo; /* winuser.h:7518:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn SetMenuItemInfoA(hmenu: ::windef::HMENU, item: ::minwindef::UINT, fByPositon: ::minwindef::BOOL, lpmii: ::winuser::LPCMENUITEMINFOA) -> ::minwindef::BOOL; /* winuser.h:7504:1 */
    pub fn SetMenuItemInfoW(hmenu: ::windef::HMENU, item: ::minwindef::UINT, fByPositon: ::minwindef::BOOL, lpmii: ::winuser::LPCMENUITEMINFOW) -> ::minwindef::BOOL; /* winuser.h:7512:1 */
    pub fn SetMessageExtraInfo(lParam: ::minwindef::LPARAM) -> ::minwindef::LPARAM; /* winuser.h:3544:1 */
    pub fn SetScrollInfo(hwnd: ::windef::HWND, nBar: ::libc::c_int, lpsi: ::winuser::LPCSCROLLINFO, redraw: ::minwindef::BOOL) -> ::libc::c_int; /* winuser.h:11388:1 */
    pub fn SetWindowContextHelpId(_: ::windef::HWND, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:8584:1 */
    pub fn ShowWindowAsync(hWnd: ::windef::HWND, nCmdShow: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:4462:1 */
    pub fn SwitchToThisWindow(hwnd: ::windef::HWND, fUnknown: ::minwindef::BOOL); /* winuser.h:8024:1 */
    pub fn TileWindows(hwndParent: ::windef::HWND, wHow: ::minwindef::UINT, lpRect: *const ::windef::RECT, cKids: ::minwindef::UINT, lpKids: *const *mut ::windef::HWND__) -> ::minwindef::WORD; /* winuser.h:11576:1 */
    pub fn ToAsciiEx(uVirtKey: ::minwindef::UINT, uScanCode: ::minwindef::UINT, lpKeyState: *const ::libc::c_uchar, lpChar: ::minwindef::LPWORD, uFlags: ::minwindef::UINT, dwhkl: ::minwindef::HKL) -> ::libc::c_int; /* winuser.h:5680:1 */
    pub fn ToUnicodeEx(wVirtKey: ::minwindef::UINT, wScanCode: ::minwindef::UINT, lpKeyState: *const ::libc::c_uchar, pwszBuff: ::winnt::LPWSTR, cchBuff: ::libc::c_int, wFlags: ::minwindef::UINT, dwhkl: ::minwindef::HKL) -> ::libc::c_int; /* winuser.h:1189:1 */
    pub fn TrackMouseEvent(lpEventTrack: ::winuser::LPTRACKMOUSEEVENT) -> ::minwindef::BOOL; /* winuser.h:2658:1 */
    pub fn TrackPopupMenuEx(hMenu: ::windef::HMENU, uFlags: ::minwindef::UINT, x: ::libc::c_int, y: ::libc::c_int, hwnd: ::windef::HWND, lptpm: ::winuser::LPTPMPARAMS) -> ::minwindef::BOOL; /* winuser.h:7281:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::VkKeyScanExW as VkKeyScanEx; /* winuser.h:5736:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn VkKeyScanExA(ch: ::winnt::CHAR, dwhkl: ::minwindef::HKL) -> ::winnt::SHORT; /* winuser.h:5726:1 */
    pub fn VkKeyScanExW(ch: ::winnt::WCHAR, dwhkl: ::minwindef::HKL) -> ::winnt::SHORT; /* winuser.h:5732:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn AllowSetForegroundWindow(dwProcessId: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:8040:1 */
    pub fn AnimateWindow(hWnd: ::windef::HWND, dwTime: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:4330:1 */
    pub fn BlockInput(fBlockIt: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:13706:1 */
    pub fn EndMenu() -> ::minwindef::BOOL; /* winuser.h:7348:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumDisplayDevicesW as EnumDisplayDevices; /* winuser.h:12475:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumDisplayDevicesA(lpDevice: ::winnt::LPCSTR, iDevNum: ::minwindef::DWORD, lpDisplayDevice: ::wingdi::PDISPLAY_DEVICEA, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:12461:1 */
    pub fn EnumDisplayDevicesW(lpDevice: ::winnt::LPCWSTR, iDevNum: ::minwindef::DWORD, lpDisplayDevice: ::wingdi::PDISPLAY_DEVICEW, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:12469:1 */
    pub fn EnumDisplayMonitors(hdc: ::windef::HDC, lprcClip: ::windef::LPCRECT, lpfnEnum: ::winuser::MONITORENUMPROC, dwData: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:12976:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumDisplaySettingsExW as EnumDisplaySettingsEx; /* winuser.h:12449:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumDisplaySettingsExA(lpszDeviceName: ::winnt::LPCSTR, iModeNum: ::minwindef::DWORD, lpDevMode: *mut ::wingdi::DEVMODEA, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:12435:1 */
    pub fn EnumDisplaySettingsExW(lpszDeviceName: ::winnt::LPCWSTR, iModeNum: ::minwindef::DWORD, lpDevMode: *mut ::wingdi::DEVMODEW, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:12443:1 */
    pub fn FlashWindowEx(pfwi: ::winuser::PFLASHWINFO) -> ::minwindef::BOOL; /* winuser.h:4486:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::GetAltTabInfoW as GetAltTabInfo; /* winuser.h:14029:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn GetAltTabInfoA(hwnd: ::windef::HWND, iItem: ::libc::c_int, pati: ::winuser::PALTTABINFO, pszItemText: ::winnt::LPSTR, cchItemText: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:14013:1 */
    pub fn GetAltTabInfoW(hwnd: ::windef::HWND, iItem: ::libc::c_int, pati: ::winuser::PALTTABINFO, pszItemText: ::winnt::LPWSTR, cchItemText: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:14022:1 */
    pub fn GetAncestor(hwnd: ::windef::HWND, gaFlags: ::minwindef::UINT) -> ::windef::HWND; /* winuser.h:13947:1 */
    pub fn GetClipboardSequenceNumber() -> ::minwindef::DWORD; /* winuser.h:5157:1 */
    pub fn GetComboBoxInfo(hwndCombo: ::windef::HWND, pcbi: ::winuser::PCOMBOBOXINFO) -> ::minwindef::BOOL; /* winuser.h:13927:1 */
    pub fn GetCursorInfo(pci: ::winuser::PCURSORINFO) -> ::minwindef::BOOL; /* winuser.h:13810:1 */
    pub fn GetGUIThreadInfo(idThread: ::minwindef::DWORD, pgui: ::winuser::PGUITHREADINFO) -> ::minwindef::BOOL; /* winuser.h:13699:1 */
    pub fn GetGuiResources(hProcess: ::winnt::HANDLE, uiFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:11753:1 */
    pub fn GetLastInputInfo(plii: ::winuser::PLASTINPUTINFO) -> ::minwindef::BOOL; /* winuser.h:6459:1 */
    pub fn GetListBoxInfo(hwnd: ::windef::HWND) -> ::minwindef::DWORD; /* winuser.h:14041:1 */
    pub fn GetMenuBarInfo(hwnd: ::windef::HWND, idObject: ::winnt::LONG, idItem: ::winnt::LONG, pmbi: ::winuser::PMENUBARINFO) -> ::minwindef::BOOL; /* winuser.h:13882:1 */
    pub fn GetMenuInfo(_: ::windef::HMENU, _: ::winuser::LPMENUINFO) -> ::minwindef::BOOL; /* winuser.h:7334:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::GetMonitorInfoW as GetMonitorInfo; /* winuser.h:12966:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn GetMonitorInfoA(hMonitor: ::windef::HMONITOR, lpmi: ::winuser::LPMONITORINFO) -> ::minwindef::BOOL; /* winuser.h:12956:1 */
    pub fn GetMonitorInfoW(hMonitor: ::windef::HMONITOR, lpmi: ::winuser::LPMONITORINFO) -> ::minwindef::BOOL; /* winuser.h:12962:1 */
    pub fn GetMouseMovePointsEx(cbSize: ::minwindef::UINT, lppt: ::winuser::LPMOUSEMOVEPOINT, lpptBuf: ::winuser::LPMOUSEMOVEPOINT, nBufPoints: ::libc::c_int, resolution: ::minwindef::DWORD) -> ::libc::c_int; /* winuser.h:1268:1 */
    pub fn GetProcessDefaultLayout(pdwDefaultLayout: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winuser.h:9497:1 */
    pub fn GetScrollBarInfo(hwnd: ::windef::HWND, idObject: ::winnt::LONG, psbi: ::winuser::PSCROLLBARINFO) -> ::minwindef::BOOL; /* winuser.h:13905:1 */
    pub fn GetTitleBarInfo(hwnd: ::windef::HWND, pti: ::winuser::PTITLEBARINFO) -> ::minwindef::BOOL; /* winuser.h:13852:1 */
    pub fn GetWindowInfo(hwnd: ::windef::HWND, pwi: ::winuser::PWINDOWINFO) -> ::minwindef::BOOL; /* winuser.h:13835:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::GetWindowModuleFileNameW as GetWindowModuleFileName; /* winuser.h:13742:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn GetWindowModuleFileNameA(hwnd: ::windef::HWND, pszFileName: ::winnt::LPSTR, cchFileNameMax: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:13730:1 */
    pub fn GetWindowModuleFileNameW(hwnd: ::windef::HWND, pszFileName: ::winnt::LPWSTR, cchFileNameMax: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:13737:1 */
    pub fn InSendMessageEx(lpReserved: ::minwindef::LPVOID) -> ::minwindef::DWORD; /* winuser.h:4088:1 */
    pub fn LockSetForegroundWindow(uLockCode: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:8048:1 */
    pub fn LockWorkStation() -> ::minwindef::BOOL; /* winuser.h:14058:1 */
    pub fn MonitorFromPoint(pt: ::windef::POINT, dwFlags: ::minwindef::DWORD) -> ::windef::HMONITOR; /* winuser.h:12879:1 */
    pub fn MonitorFromRect(lprc: ::windef::LPCRECT, dwFlags: ::minwindef::DWORD) -> ::windef::HMONITOR; /* winuser.h:12886:1 */
    pub fn MonitorFromWindow(hwnd: ::windef::HWND, dwFlags: ::minwindef::DWORD) -> ::windef::HMONITOR; /* winuser.h:12893:1 */
    pub fn NotifyWinEvent(event: ::minwindef::DWORD, hwnd: ::windef::HWND, idObject: ::winnt::LONG, idChild: ::winnt::LONG); /* winuser.h:12997:1 */
    pub fn RealChildWindowFromPoint(hwndParent: ::windef::HWND, ptParentClientCoords: ::windef::POINT) -> ::windef::HWND; /* winuser.h:13961:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::RealGetWindowClassW as RealGetWindowClass; /* winuser.h:13989:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn RealGetWindowClassA(hwnd: ::windef::HWND, ptszClassName: ::winnt::LPSTR, cchClassNameMax: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:13973:1 */
    pub fn RealGetWindowClassW(hwnd: ::windef::HWND, ptszClassName: ::winnt::LPWSTR, cchClassNameMax: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:13984:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::RegisterDeviceNotificationW as RegisterDeviceNotification; /* winuser.h:3814:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn RegisterDeviceNotificationA(hRecipient: ::winnt::HANDLE, NotificationFilter: ::minwindef::LPVOID, Flags: ::minwindef::DWORD) -> ::winuser::HDEVNOTIFY; /* winuser.h:3802:1 */
    pub fn RegisterDeviceNotificationW(hRecipient: ::winnt::HANDLE, NotificationFilter: ::minwindef::LPVOID, Flags: ::minwindef::DWORD) -> ::winuser::HDEVNOTIFY; /* winuser.h:3809:1 */
    pub fn SendInput(cInputs: ::minwindef::UINT, pInputs: ::winuser::LPINPUT, cbSize: ::libc::c_int) -> ::minwindef::UINT; /* winuser.h:5841:1 */
    pub fn SetLayeredWindowAttributes(hwnd: ::windef::HWND, crKey: ::windef::COLORREF, bAlpha: ::minwindef::BYTE, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:4432:1 */
    pub fn SetMenuInfo(_: ::windef::HMENU, _: ::winuser::LPCMENUINFO) -> ::minwindef::BOOL; /* winuser.h:7341:1 */
    pub fn SetProcessDefaultLayout(dwDefaultLayout: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:9503:1 */
    pub fn SetWinEventHook(eventMin: ::minwindef::DWORD, eventMax: ::minwindef::DWORD, hmodWinEventProc: ::minwindef::HMODULE, pfnWinEventProc: ::winuser::WINEVENTPROC, idProcess: ::minwindef::DWORD, idThread: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD) -> ::windef::HWINEVENTHOOK; /* winuser.h:13015:1 */
    pub fn UnhookWinEvent(hWinEventHook: ::windef::HWINEVENTHOOK) -> ::minwindef::BOOL; /* winuser.h:13049:1 */
    pub fn UnregisterDeviceNotification(Handle: ::winuser::HDEVNOTIFY) -> ::minwindef::BOOL; /* winuser.h:3822:1 */
    pub fn UpdateLayeredWindow(hWnd: ::windef::HWND, hdcDst: ::windef::HDC, pptDst: *mut ::windef::POINT, psize: *mut ::windef::SIZE, hdcSrc: ::windef::HDC, pptSrc: *mut ::windef::POINT, crKey: ::windef::COLORREF, pblend: *mut ::wingdi::BLENDFUNCTION, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:4348:1 */
    pub fn UpdateLayeredWindowIndirect(hWnd: ::windef::HWND, pULWInfo: *const ::winuser::UPDATELAYEREDWINDOWINFO) -> ::minwindef::BOOL; /* winuser.h:4383:1 */
    pub fn UserHandleGrantAccess(hUserHandle: ::winnt::HANDLE, hJob: ::winnt::HANDLE, bGrant: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:14067:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::BroadcastSystemMessageExW as BroadcastSystemMessageEx; /* winuser.h:3701:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn BroadcastSystemMessageExA(flags: ::minwindef::DWORD, lpInfo: ::minwindef::LPDWORD, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM, pbsmInfo: ::winuser::PBSMINFO) -> ::libc::c_long; /* winuser.h:3683:1 */
    pub fn BroadcastSystemMessageExW(flags: ::minwindef::DWORD, lpInfo: ::minwindef::LPDWORD, Msg: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM, pbsmInfo: ::winuser::PBSMINFO) -> ::libc::c_long; /* winuser.h:3693:1 */
    pub fn DefRawInputProc(paRawInput: *mut *mut ::winuser::RAWINPUT, nInput: ::winnt::INT, cbSizeHeader: ::minwindef::UINT) -> ::minwindef::LRESULT; /* winuser.h:14506:1 */
    pub fn DisableProcessWindowsGhosting(); /* winuser.h:1765:1 */
    pub fn GetLayeredWindowAttributes(hwnd: ::windef::HWND, pcrKey: *mut ::libc::c_ulong, pbAlpha: *mut ::libc::c_uchar, pdwFlags: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winuser.h:4400:1 */
    pub fn GetRawInputBuffer(pData: ::winuser::PRAWINPUT, pcbSize: ::minwindef::PUINT, cbSizeHeader: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:14420:1 */
    pub fn GetRawInputData(hRawInput: ::winuser::HRAWINPUT, uiCommand: ::minwindef::UINT, pData: ::minwindef::LPVOID, pcbSize: ::minwindef::PUINT, cbSizeHeader: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:14333:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetRawInputDeviceInfoW as GetRawInputDeviceInfo; /* winuser.h:14408:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetRawInputDeviceInfoA(hDevice: ::winnt::HANDLE, uiCommand: ::minwindef::UINT, pData: ::minwindef::LPVOID, pcbSize: ::minwindef::PUINT) -> ::minwindef::UINT; /* winuser.h:14394:1 */
    pub fn GetRawInputDeviceInfoW(hDevice: ::winnt::HANDLE, uiCommand: ::minwindef::UINT, pData: ::minwindef::LPVOID, pcbSize: ::minwindef::PUINT) -> ::minwindef::UINT; /* winuser.h:14402:1 */
    pub fn GetRawInputDeviceList(pRawInputDeviceList: ::winuser::PRAWINPUTDEVICELIST, puiNumDevices: ::minwindef::PUINT, cbSize: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:14498:1 */
    pub fn GetRegisteredRawInputDevices(pRawInputDevices: ::winuser::PRAWINPUTDEVICE, puiNumDevices: ::minwindef::PUINT, cbSize: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:14484:1 */
    pub fn GetWindowRgnBox(hWnd: ::windef::HWND, lprc: ::windef::LPRECT) -> ::libc::c_int; /* winuser.h:8169:1 */
    pub fn IsGUIThread(bConvert: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:9683:1 */
    pub fn IsWinEventHookInstalled(event: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:13028:1 */
    pub fn IsWow64Message() -> ::minwindef::BOOL; /* winuser.h:3536:1 */
    pub fn PrintWindow(hwnd: ::windef::HWND, hdcBlt: ::windef::HDC, nFlags: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:4416:1 */
    pub fn RegisterRawInputDevices(pRawInputDevices: ::winuser::PCRAWINPUTDEVICE, uiNumDevices: ::minwindef::UINT, cbSize: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:14476:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] 
extern "system" {
    pub fn RegisterPowerSettingNotification(hRecipient: ::winnt::HANDLE, PowerSettingGuid: ::guiddef::LPCGUID, Flags: ::minwindef::DWORD) -> ::winuser::HPOWERNOTIFY; /* winuser.h:3840:1 */
    pub fn RegisterSuspendResumeNotification(hRecipient: ::winnt::HANDLE, Flags: ::minwindef::DWORD) -> ::winuser::HPOWERNOTIFY; /* winuser.h:3856:1 */
    pub fn UnregisterPowerSettingNotification(Handle: ::winuser::HPOWERNOTIFY) -> ::minwindef::BOOL; /* winuser.h:3849:1 */
    pub fn UnregisterSuspendResumeNotification(Handle: ::winuser::HPOWERNOTIFY) -> ::minwindef::BOOL; /* winuser.h:3864:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn AddClipboardFormatListener(hwnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:5277:1 */
    pub fn ChangeWindowMessageFilter(message: ::minwindef::UINT, dwFlag: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:14659:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetIconInfoExW as GetIconInfoEx; /* winuser.h:10385:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetIconInfoExA(hicon: ::windef::HICON, piconinfo: ::winuser::PICONINFOEXA) -> ::minwindef::BOOL; /* winuser.h:10375:1 */
    pub fn GetIconInfoExW(hicon: ::windef::HICON, piconinfo: ::winuser::PICONINFOEXW) -> ::minwindef::BOOL; /* winuser.h:10381:1 */
    pub fn GetPhysicalCursorPos(lpPoint: ::windef::LPPOINT) -> ::minwindef::BOOL; /* winuser.h:8874:1 */
    pub fn GetUpdatedClipboardFormats(lpuiFormats: ::minwindef::PUINT, cFormats: ::minwindef::UINT, pcFormatsOut: ::minwindef::PUINT) -> ::minwindef::BOOL; /* winuser.h:5289:1 */
    pub fn IsProcessDPIAware() -> ::minwindef::BOOL; /* winuser.h:13722:1 */
    pub fn LogicalToPhysicalPoint(hWnd: ::windef::HWND, lpPoint: ::windef::LPPOINT) -> ::minwindef::BOOL; /* winuser.h:8967:1 */
    pub fn PhysicalToLogicalPoint(hWnd: ::windef::HWND, lpPoint: ::windef::LPPOINT) -> ::minwindef::BOOL; /* winuser.h:8974:1 */
    pub fn RemoveClipboardFormatListener(hwnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:5283:1 */
    pub fn SetPhysicalCursorPos(X: ::libc::c_int, Y: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:8853:1 */
    pub fn SetProcessDPIAware() -> ::minwindef::BOOL; /* winuser.h:13716:1 */
    pub fn SoundSentry() -> ::minwindef::BOOL; /* winuser.h:12769:1 */
    pub fn WindowFromPhysicalPoint(Point: ::windef::POINT) -> ::windef::HWND; /* winuser.h:9016:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn CalculatePopupWindowPosition(anchorPoint: *const ::windef::POINT, windowSize: *const ::windef::SIZE, flags: ::minwindef::UINT, excludeRect: *mut ::windef::RECT, popupWindowPosition: *mut ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:7294:1 */
    pub fn ChangeWindowMessageFilterEx(hwnd: ::windef::HWND, message: ::minwindef::UINT, action: ::minwindef::DWORD, pChangeFilterStruct: ::winuser::PCHANGEFILTERSTRUCT) -> ::minwindef::BOOL; /* winuser.h:14702:1 */
    pub fn CloseGestureInfoHandle(hGestureInfo: ::winuser::HGESTUREINFO) -> ::minwindef::BOOL; /* winuser.h:14838:1 */
    pub fn CloseTouchInputHandle(hTouchInput: ::winuser::HTOUCHINPUT) -> ::minwindef::BOOL; /* winuser.h:5922:1 */
    pub fn DisplayConfigGetDeviceInfo(requestPacket: *mut ::wingdi::DISPLAYCONFIG_DEVICE_INFO_HEADER) -> ::winnt::LONG; /* winuser.h:12521:1 */
    pub fn DisplayConfigSetDeviceInfo(setPacket: *mut ::wingdi::DISPLAYCONFIG_DEVICE_INFO_HEADER) -> ::winnt::LONG; /* winuser.h:12527:1 */
    pub fn GetAutoRotationState(pState: ::winuser::PAR_STATE) -> ::minwindef::BOOL; /* winuser.h:15093:1 */
    pub fn GetCIMSSM(inputMessageSource: *mut ::winuser::INPUT_MESSAGE_SOURCE) -> ::minwindef::BOOL; /* winuser.h:15030:1 */
    pub fn GetCurrentInputMessageSource(inputMessageSource: *mut ::winuser::INPUT_MESSAGE_SOURCE) -> ::minwindef::BOOL; /* winuser.h:15024:1 */
    pub fn GetDisplayAutoRotationPreferences(pOrientation: *mut ::winuser::ORIENTATION_PREFERENCE) -> ::minwindef::BOOL; /* winuser.h:15099:1 */
    pub fn GetDisplayConfigBufferSizes(flags: ::basetsd::UINT32, numPathArrayElements: *mut ::libc::c_uint, numModeInfoArrayElements: *mut ::libc::c_uint) -> ::winnt::LONG; /* winuser.h:12490:1 */
    pub fn GetGestureConfig(hwnd: ::windef::HWND, dwReserved: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD, pcIDs: ::minwindef::PUINT, pGestureConfig: ::winuser::PGESTURECONFIG, cbSize: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:14921:1 */
    pub fn GetGestureExtraArgs(hGestureInfo: ::winuser::HGESTUREINFO, cbExtraArgs: ::minwindef::UINT, pExtraArgs: ::minwindef::PBYTE) -> ::minwindef::BOOL; /* winuser.h:14820:1 */
    pub fn GetGestureInfo(hGestureInfo: ::winuser::HGESTUREINFO, pGestureInfo: ::winuser::PGESTUREINFO) -> ::minwindef::BOOL; /* winuser.h:14807:1 */
    pub fn GetTouchInputInfo(hTouchInput: ::winuser::HTOUCHINPUT, cInputs: ::minwindef::UINT, pInputs: ::winuser::PTOUCHINPUT, cbSize: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:5913:1 */
    pub fn GetWindowDisplayAffinity(hWnd: ::windef::HWND, pdwAffinity: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winuser.h:4562:1 */
    pub fn IsImmersiveProcess(hProcess: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winuser.h:15130:1 */
    pub fn IsTouchWindow(hwnd: ::windef::HWND, pulFlags: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winuser.h:5962:1 */
    pub fn QueryDisplayConfig(flags: ::basetsd::UINT32, numPathArrayElements: *mut ::libc::c_uint, pathArray: *mut ::wingdi::DISPLAYCONFIG_PATH_INFO, numModeInfoArrayElements: *mut ::libc::c_uint, modeInfoArray: *mut ::wingdi::DISPLAYCONFIG_MODE_INFO, currentTopologyId: *mut ::wingdi::DISPLAYCONFIG_TOPOLOGY_ID) -> ::winnt::LONG; /* winuser.h:12508:1 */
    pub fn RegisterTouchWindow(hwnd: ::windef::HWND, ulFlags: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winuser.h:5949:1 */
    pub fn SetCoalescableTimer(hWnd: ::windef::HWND, nIDEvent: ::basetsd::UINT_PTR, uElapse: ::minwindef::UINT, lpTimerFunc: ::winuser::TIMERPROC, uToleranceDelay: ::minwindef::ULONG) -> ::basetsd::UINT_PTR; /* winuser.h:6659:1 */
    pub fn SetDisplayAutoRotationPreferences(orientation: ::winuser::ORIENTATION_PREFERENCE) -> ::minwindef::BOOL; /* winuser.h:15113:1 */
    pub fn SetDisplayConfig(numPathArrayElements: ::basetsd::UINT32, pathArray: *mut ::wingdi::DISPLAYCONFIG_PATH_INFO, numModeInfoArrayElements: ::basetsd::UINT32, modeInfoArray: *mut ::wingdi::DISPLAYCONFIG_MODE_INFO, flags: ::basetsd::UINT32) -> ::winnt::LONG; /* winuser.h:12498:1 */
    pub fn SetGestureConfig(hwnd: ::windef::HWND, dwReserved: ::minwindef::DWORD, cIDs: ::minwindef::UINT, pGestureConfig: ::winuser::PGESTURECONFIG, cbSize: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:14906:1 */
    pub fn SetProcessRestrictionExemption(fEnableExemption: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:15136:1 */
    pub fn SetWindowDisplayAffinity(hWnd: ::windef::HWND, dwAffinity: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:4569:1 */
    pub fn UnregisterTouchWindow(hwnd: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:5956:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn EnableMouseInPointer(fEnable: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:6297:1 */
    pub fn EvaluateProximityToPolygon(numVertices: ::basetsd::UINT32, controlPolygon: *const ::windef::POINT, pHitTestingInput: *const ::winuser::TOUCH_HIT_TESTING_INPUT, pProximityEval: *mut ::winuser::TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> ::minwindef::BOOL; /* winuser.h:6352:1 */
    pub fn EvaluateProximityToRect(controlBoundingBox: *const ::windef::RECT, pHitTestingInput: *const ::winuser::TOUCH_HIT_TESTING_INPUT, pProximityEval: *mut ::winuser::TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> ::minwindef::BOOL; /* winuser.h:6344:1 */
    pub fn GetPointerCursorId(pointerId: ::basetsd::UINT32, cursorId: *mut ::libc::c_uint) -> ::minwindef::BOOL; /* winuser.h:6173:1 */
    pub fn GetPointerDevice(device: ::winnt::HANDLE, pointerDevice: *mut ::winuser::POINTER_DEVICE_INFO) -> ::minwindef::BOOL; /* winuser.h:14592:1 */
    pub fn GetPointerDeviceCursors(device: ::winnt::HANDLE, cursorCount: *mut ::libc::c_uint, deviceCursors: *mut ::winuser::POINTER_DEVICE_CURSOR_INFO) -> ::minwindef::BOOL; /* winuser.h:14622:1 */
    pub fn GetPointerDeviceProperties(device: ::winnt::HANDLE, propertyCount: *mut ::libc::c_uint, pointerProperties: *mut ::winuser::POINTER_DEVICE_PROPERTY) -> ::minwindef::BOOL; /* winuser.h:14599:1 */
    pub fn GetPointerDeviceRects(device: ::winnt::HANDLE, pointerDeviceRect: *mut ::windef::RECT, displayRect: *mut ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:14614:1 */
    pub fn GetPointerDevices(deviceCount: *mut ::libc::c_uint, pointerDevices: *mut ::winuser::POINTER_DEVICE_INFO) -> ::minwindef::BOOL; /* winuser.h:14585:1 */
    pub fn GetPointerFrameInfo(pointerId: ::basetsd::UINT32, pointerCount: *mut ::libc::c_uint, pointerInfo: *mut ::winuser::POINTER_INFO) -> ::minwindef::BOOL; /* winuser.h:6195:1 */
    pub fn GetPointerFrameInfoHistory(pointerId: ::basetsd::UINT32, entriesCount: *mut ::libc::c_uint, pointerCount: *mut ::libc::c_uint, pointerInfo: *mut ::winuser::POINTER_INFO) -> ::minwindef::BOOL; /* winuser.h:6203:1 */
    pub fn GetPointerFramePenInfo(pointerId: ::basetsd::UINT32, pointerCount: *mut ::libc::c_uint, penInfo: *mut ::winuser::POINTER_PEN_INFO) -> ::minwindef::BOOL; /* winuser.h:6259:1 */
    pub fn GetPointerFramePenInfoHistory(pointerId: ::basetsd::UINT32, entriesCount: *mut ::libc::c_uint, pointerCount: *mut ::libc::c_uint, penInfo: *mut ::winuser::POINTER_PEN_INFO) -> ::minwindef::BOOL; /* winuser.h:6267:1 */
    pub fn GetPointerFrameTouchInfo(pointerId: ::basetsd::UINT32, pointerCount: *mut ::libc::c_uint, touchInfo: *mut ::winuser::POINTER_TOUCH_INFO) -> ::minwindef::BOOL; /* winuser.h:6227:1 */
    pub fn GetPointerFrameTouchInfoHistory(pointerId: ::basetsd::UINT32, entriesCount: *mut ::libc::c_uint, pointerCount: *mut ::libc::c_uint, touchInfo: *mut ::winuser::POINTER_TOUCH_INFO) -> ::minwindef::BOOL; /* winuser.h:6235:1 */
    pub fn GetPointerInfo(pointerId: ::basetsd::UINT32, pointerInfo: *mut ::winuser::POINTER_INFO) -> ::minwindef::BOOL; /* winuser.h:6180:1 */
    pub fn GetPointerInfoHistory(pointerId: ::basetsd::UINT32, entriesCount: *mut ::libc::c_uint, pointerInfo: *mut ::winuser::POINTER_INFO) -> ::minwindef::BOOL; /* winuser.h:6187:1 */
    pub fn GetPointerPenInfo(pointerId: ::basetsd::UINT32, penInfo: *mut ::winuser::POINTER_PEN_INFO) -> ::minwindef::BOOL; /* winuser.h:6244:1 */
    pub fn GetPointerPenInfoHistory(pointerId: ::basetsd::UINT32, entriesCount: *mut ::libc::c_uint, penInfo: *mut ::winuser::POINTER_PEN_INFO) -> ::minwindef::BOOL; /* winuser.h:6251:1 */
    pub fn GetPointerTouchInfo(pointerId: ::basetsd::UINT32, touchInfo: *mut ::winuser::POINTER_TOUCH_INFO) -> ::minwindef::BOOL; /* winuser.h:6212:1 */
    pub fn GetPointerTouchInfoHistory(pointerId: ::basetsd::UINT32, entriesCount: *mut ::libc::c_uint, touchInfo: *mut ::winuser::POINTER_TOUCH_INFO) -> ::minwindef::BOOL; /* winuser.h:6219:1 */
    pub fn GetPointerType(pointerId: ::basetsd::UINT32, pointerType: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winuser.h:6166:1 */
    pub fn GetRawPointerDeviceData(pointerId: ::basetsd::UINT32, historyCount: ::basetsd::UINT32, propertiesCount: ::basetsd::UINT32, pProperties: *mut ::winuser::POINTER_DEVICE_PROPERTY, pValues: *mut ::libc::c_long) -> ::minwindef::BOOL; /* winuser.h:14630:1 */
    pub fn GetUnpredictedMessagePos() -> ::minwindef::DWORD; /* winuser.h:3528:1 */
    pub fn GetWindowFeedbackSetting(hwnd: ::windef::HWND, feedback: ::winuser::FEEDBACK_TYPE, dwFlags: ::minwindef::DWORD, pSize: *mut ::libc::c_uint, config: *mut ::libc::c_void) -> ::minwindef::BOOL; /* winuser.h:6387:1 */
    pub fn InitializeTouchInjection(maxCount: ::basetsd::UINT32, dwMode: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:6152:1 */
    pub fn InjectTouchInput(count: ::basetsd::UINT32, contacts: *const ::winuser::POINTER_TOUCH_INFO) -> ::minwindef::BOOL; /* winuser.h:6159:1 */
    pub fn IsMouseInPointerEnabled() -> ::minwindef::BOOL; /* winuser.h:6303:1 */
    pub fn PackTouchHitTestingProximityEvaluation(pHitTestingInput: *const ::winuser::TOUCH_HIT_TESTING_INPUT, pProximityEval: *const ::winuser::TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> ::minwindef::LRESULT; /* winuser.h:6361:1 */
    pub fn RegisterPointerDeviceNotifications(window: ::windef::HWND, notifyRange: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:14607:1 */
    pub fn RegisterPointerInputTarget(hwnd: ::windef::HWND, pointerType: ::winuser::POINTER_INPUT_TYPE) -> ::minwindef::BOOL; /* winuser.h:6282:1 */
    pub fn RegisterTouchHitTestingWindow(hwnd: ::windef::HWND, value: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winuser.h:6314:1 */
    pub fn SetWindowFeedbackSetting(hwnd: ::windef::HWND, feedback: ::winuser::FEEDBACK_TYPE, dwFlags: ::minwindef::DWORD, size: ::basetsd::UINT32, configuration: *const ::libc::c_void) -> ::minwindef::BOOL; /* winuser.h:6397:1 */
    pub fn SkipPointerFrameMessages(pointerId: ::basetsd::UINT32) -> ::minwindef::BOOL; /* winuser.h:6276:1 */
    pub fn UnregisterPointerInputTarget(hwnd: ::windef::HWND, pointerType: ::winuser::POINTER_INPUT_TYPE) -> ::minwindef::BOOL; /* winuser.h:6289:1 */
}
