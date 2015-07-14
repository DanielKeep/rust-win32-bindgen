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
    pub fn EndTask(_: ::windef::HWND, _: ::minwindef::BOOL, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:12847:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] 
extern "C" {
    pub fn wsprintfA(_: ::winnt::LPSTR, _: ::winnt::LPCSTR) -> ::libc::c_int; /* winuser.h:295:1 */
    pub fn wsprintfW(_: ::winnt::LPWSTR, _: ::winnt::LPCWSTR) -> ::libc::c_int; /* winuser.h:302:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn GetClassLongPtrA(_: ::windef::HWND, _: ::libc::c_int) -> ::basetsd::ULONG_PTR; /* winuser.h:9436:1 */
    pub fn GetClassLongPtrW(_: ::windef::HWND, _: ::libc::c_int) -> ::basetsd::ULONG_PTR; /* winuser.h:9442:1 */
    pub fn GetWindowLongPtrA(_: ::windef::HWND, _: ::libc::c_int) -> ::basetsd::LONG_PTR; /* winuser.h:9323:1 */
    pub fn GetWindowLongPtrW(_: ::windef::HWND, _: ::libc::c_int) -> ::basetsd::LONG_PTR; /* winuser.h:9329:1 */
    pub fn SetClassLongPtrA(_: ::windef::HWND, _: ::libc::c_int, _: ::basetsd::LONG_PTR) -> ::basetsd::ULONG_PTR; /* winuser.h:9454:1 */
    pub fn SetClassLongPtrW(_: ::windef::HWND, _: ::libc::c_int, _: ::basetsd::LONG_PTR) -> ::basetsd::ULONG_PTR; /* winuser.h:9461:1 */
    pub fn SetWindowLongPtrA(_: ::windef::HWND, _: ::libc::c_int, _: ::basetsd::LONG_PTR) -> ::basetsd::LONG_PTR; /* winuser.h:9341:1 */
    pub fn SetWindowLongPtrW(_: ::windef::HWND, _: ::libc::c_int, _: ::basetsd::LONG_PTR) -> ::basetsd::LONG_PTR; /* winuser.h:9348:1 */
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
    pub fn wsprintfA(_: ::winnt::LPSTR, _: ::winnt::LPCSTR) -> ::libc::c_int; /* winuser.h:295:1 */
    pub fn wsprintfW(_: ::winnt::LPWSTR, _: ::winnt::LPCWSTR) -> ::libc::c_int; /* winuser.h:302:1 */
}
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AdjustWindowRect(_: ::windef::LPRECT, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:8546:1 */
    pub fn AdjustWindowRectEx(_: ::windef::LPRECT, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:8554:1 */
    pub fn AnyPopup() -> ::minwindef::BOOL; /* winuser.h:4632:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AppendMenuW as AppendMenu; /* winuser.h:7192:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AppendMenuA(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::basetsd::UINT_PTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winuser.h:7178:1 */
    pub fn AppendMenuW(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::basetsd::UINT_PTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winuser.h:7186:1 */
    pub fn ArrangeIconicWindows(_: ::windef::HWND) -> ::minwindef::UINT; /* winuser.h:11535:1 */
    pub fn AttachThreadInput(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:3950:1 */
    pub fn BeginDeferWindowPos(_: ::libc::c_int) -> ::winuser::HDWP; /* winuser.h:4586:1 */
    pub fn BeginPaint(_: ::windef::HWND, _: ::winuser::LPPAINTSTRUCT) -> ::windef::HDC; /* winuser.h:8115:1 */
    pub fn BringWindowToTop(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4638:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CallMsgFilterW as CallMsgFilter; /* winuser.h:5126:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CallMsgFilterA(_: ::winuser::LPMSG, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:5116:1 */
    pub fn CallMsgFilterW(_: ::winuser::LPMSG, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:5122:1 */
    pub fn CallNextHookEx(_: ::windef::HHOOK, _: ::libc::c_int, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:9804:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CallWindowProcW as CallWindowProc; /* winuser.h:4038:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CallWindowProcA(_: ::winuser::WNDPROC, _: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:4022:1 */
    pub fn CallWindowProcW(_: ::winuser::WNDPROC, _: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:4031:1 */
    pub fn CancelShutdown() -> ::minwindef::BOOL; /* winuser.h:12856:1 */
    pub fn ChangeClipboardChain(_: ::windef::HWND, _: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:5183:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ChangeDisplaySettingsW as ChangeDisplaySettings; /* winuser.h:12377:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ChangeDisplaySettingsA(_: *mut ::wingdi::DEVMODEA, _: ::minwindef::DWORD) -> ::winnt::LONG; /* winuser.h:12367:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ChangeDisplaySettingsExW as ChangeDisplaySettingsEx; /* winuser.h:12401:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ChangeDisplaySettingsExA(_: ::winnt::LPCSTR, _: *mut ::wingdi::DEVMODEA, _: ::windef::HWND, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::winnt::LONG; /* winuser.h:12385:1 */
    pub fn ChangeDisplaySettingsExW(_: ::winnt::LPCWSTR, _: *mut ::wingdi::DEVMODEW, _: ::windef::HWND, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::winnt::LONG; /* winuser.h:12394:1 */
    pub fn ChangeDisplaySettingsW(_: *mut ::wingdi::DEVMODEW, _: ::minwindef::DWORD) -> ::winnt::LONG; /* winuser.h:12373:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ChangeMenuW as ChangeMenu; /* winuser.h:7031:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ChangeMenuA(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::winnt::LPCSTR, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7015:1 */
    pub fn ChangeMenuW(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::winnt::LPCWSTR, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7024:1 */
    pub fn CharLowerA(_: ::winnt::LPSTR) -> ::winnt::LPSTR; /* winuser.h:5416:1 */
    pub fn CharLowerBuffA(_: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:5432:1 */
    pub fn CharNextA(_: ::winnt::LPCSTR) -> ::winnt::LPSTR; /* winuser.h:5450:1 */
    pub fn CharPrevA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::winnt::LPSTR; /* winuser.h:5466:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharToOemW as CharToOem; /* winuser.h:5314:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharToOemA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR) -> ::minwindef::BOOL; /* winuser.h:5304:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharToOemBuffW as CharToOemBuff; /* winuser.h:5354:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharToOemBuffA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:5342:1 */
    pub fn CharToOemBuffW(_: ::winnt::LPCWSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:5349:1 */
    pub fn CharToOemW(_: ::winnt::LPCWSTR, _: ::winnt::LPSTR) -> ::minwindef::BOOL; /* winuser.h:5310:1 */
    pub fn CharUpperA(_: ::winnt::LPSTR) -> ::winnt::LPSTR; /* winuser.h:5382:1 */
    pub fn CharUpperBuffA(_: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:5398:1 */
    pub fn CheckDlgButton(_: ::windef::HWND, _: ::libc::c_int, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:4990:1 */
    pub fn CheckMenuItem(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::DWORD; /* winuser.h:7118:1 */
    pub fn CheckRadioButton(_: ::windef::HWND, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:4998:1 */
    pub fn ChildWindowFromPoint(_: ::windef::HWND, _: ::windef::POINT) -> ::windef::HWND; /* winuser.h:9023:1 */
    pub fn ClientToScreen(_: ::windef::HWND, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* winuser.h:8952:1 */
    pub fn ClipCursor(_: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:8881:1 */
    pub fn CloseClipboard() -> ::minwindef::BOOL; /* winuser.h:5148:1 */
    pub fn CloseDesktop(_: ::windef::HDESK) -> ::minwindef::BOOL; /* winuser.h:1446:1 */
    pub fn CloseWindow(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4514:1 */
    pub fn CloseWindowStation(_: ::minwindef::HWINSTA) -> ::minwindef::BOOL; /* winuser.h:1554:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CopyAcceleratorTableW as CopyAcceleratorTable; /* winuser.h:6751:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CopyAcceleratorTableA(_: ::windef::HACCEL, _: ::winuser::LPACCEL, _: ::libc::c_int) -> ::libc::c_int; /* winuser.h:6739:1 */
    pub fn CopyAcceleratorTableW(_: ::windef::HACCEL, _: ::winuser::LPACCEL, _: ::libc::c_int) -> ::libc::c_int; /* winuser.h:6746:1 */
    pub fn CopyIcon(_: ::windef::HICON) -> ::windef::HICON; /* winuser.h:10331:1 */
    pub fn CopyRect(_: ::windef::LPRECT, _: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9199:1 */
    pub fn CountClipboardFormats() -> ::libc::c_int; /* winuser.h:5219:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateAcceleratorTableW as CreateAcceleratorTable; /* winuser.h:6725:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateAcceleratorTableA(_: ::winuser::LPACCEL, _: ::libc::c_int) -> ::windef::HACCEL; /* winuser.h:6715:1 */
    pub fn CreateAcceleratorTableW(_: ::winuser::LPACCEL, _: ::libc::c_int) -> ::windef::HACCEL; /* winuser.h:6721:1 */
    pub fn CreateCaret(_: ::windef::HWND, _: ::windef::HBITMAP, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:8900:1 */
    pub fn CreateCursor(_: ::minwindef::HINSTANCE, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: *const ::libc::c_void, _: *const ::libc::c_void) -> ::windef::HCURSOR; /* winuser.h:10040:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateDesktopW as CreateDesktop; /* winuser.h:1326:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateDesktopA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: *mut ::wingdi::DEVMODEA, _: ::minwindef::DWORD, _: ::winnt::ACCESS_MASK, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::windef::HDESK; /* winuser.h:1308:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateDesktopExW as CreateDesktopEx; /* winuser.h:1356:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateDesktopExA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: *mut ::wingdi::DEVMODEA, _: ::minwindef::DWORD, _: ::winnt::ACCESS_MASK, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::ULONG, _: ::winnt::PVOID) -> ::windef::HDESK; /* winuser.h:1334:1 */
    pub fn CreateDesktopExW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: *mut ::wingdi::DEVMODEW, _: ::minwindef::DWORD, _: ::winnt::ACCESS_MASK, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::ULONG, _: ::winnt::PVOID) -> ::windef::HDESK; /* winuser.h:1346:1 */
    pub fn CreateDesktopW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: *mut ::wingdi::DEVMODEW, _: ::minwindef::DWORD, _: ::winnt::ACCESS_MASK, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::windef::HDESK; /* winuser.h:1318:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateDialogIndirectParamW as CreateDialogIndirectParam; /* winuser.h:4818:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateDialogIndirectParamA(_: ::minwindef::HINSTANCE, _: ::winuser::LPCDLGTEMPLATEA, _: ::windef::HWND, _: ::winuser::DLGPROC, _: ::minwindef::LPARAM) -> ::windef::HWND; /* winuser.h:4802:1 */
    pub fn CreateDialogIndirectParamW(_: ::minwindef::HINSTANCE, _: ::winuser::LPCDLGTEMPLATEW, _: ::windef::HWND, _: ::winuser::DLGPROC, _: ::minwindef::LPARAM) -> ::windef::HWND; /* winuser.h:4811:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateDialogParamW as CreateDialogParam; /* winuser.h:4794:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateDialogParamA(_: ::minwindef::HINSTANCE, _: ::winnt::LPCSTR, _: ::windef::HWND, _: ::winuser::DLGPROC, _: ::minwindef::LPARAM) -> ::windef::HWND; /* winuser.h:4778:1 */
    pub fn CreateDialogParamW(_: ::minwindef::HINSTANCE, _: ::winnt::LPCWSTR, _: ::windef::HWND, _: ::winuser::DLGPROC, _: ::minwindef::LPARAM) -> ::windef::HWND; /* winuser.h:4787:1 */
    pub fn CreateIcon(_: ::minwindef::HINSTANCE, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::BYTE, _: ::minwindef::BYTE, _: *const ::libc::c_uchar, _: *const ::libc::c_uchar) -> ::windef::HICON; /* winuser.h:10163:1 */
    pub fn CreateIconFromResource(_: ::minwindef::PBYTE, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::DWORD) -> ::windef::HICON; /* winuser.h:10200:1 */
    pub fn CreateIconIndirect(_: ::winuser::PICONINFO) -> ::windef::HICON; /* winuser.h:10325:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateMDIWindowW as CreateMDIWindow; /* winuser.h:11567:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateMDIWindowA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HWND, _: ::minwindef::HINSTANCE, _: ::minwindef::LPARAM) -> ::windef::HWND; /* winuser.h:11541:1 */
    pub fn CreateMDIWindowW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HWND, _: ::minwindef::HINSTANCE, _: ::minwindef::LPARAM) -> ::windef::HWND; /* winuser.h:11555:1 */
    pub fn CreateMenu() -> ::windef::HMENU; /* winuser.h:7100:1 */
    pub fn CreatePopupMenu() -> ::windef::HMENU; /* winuser.h:7106:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateWindowExW as CreateWindowEx; /* winuser.h:4267:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateWindowExA(_: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HWND, _: ::windef::HMENU, _: ::minwindef::HINSTANCE, _: ::minwindef::LPVOID) -> ::windef::HWND; /* winuser.h:4237:1 */
    pub fn CreateWindowExW(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HWND, _: ::windef::HMENU, _: ::minwindef::HINSTANCE, _: ::minwindef::LPVOID) -> ::windef::HWND; /* winuser.h:4253:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateWindowStationW as CreateWindowStation; /* winuser.h:1508:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateWindowStationA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winnt::ACCESS_MASK, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::HWINSTA; /* winuser.h:1494:1 */
    pub fn CreateWindowStationW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::ACCESS_MASK, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::HWINSTA; /* winuser.h:1502:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DefDlgProcW as DefDlgProc; /* winuser.h:5089:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DefDlgProcA(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:5070:1 */
    pub fn DefDlgProcW(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:5083:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DefFrameProcW as DefFrameProc; /* winuser.h:11484:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DefFrameProcA(_: ::windef::HWND, _: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:11468:1 */
    pub fn DefFrameProcW(_: ::windef::HWND, _: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:11477:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DefMDIChildProcW as DefMDIChildProc; /* winuser.h:11516:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DefMDIChildProcA(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:11497:1 */
    pub fn DefMDIChildProcW(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:11510:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DefWindowProcW as DefWindowProc; /* winuser.h:4006:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DefWindowProcA(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:3987:1 */
    pub fn DefWindowProcW(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:4000:1 */
    pub fn DeferWindowPos(_: ::winuser::HDWP, _: ::windef::HWND, _: ::windef::HWND, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT) -> ::winuser::HDWP; /* winuser.h:4592:1 */
    pub fn DeleteMenu(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7231:1 */
    pub fn DeregisterShellHookWindow(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:9595:1 */
    pub fn DestroyAcceleratorTable(_: ::windef::HACCEL) -> ::minwindef::BOOL; /* winuser.h:6733:1 */
    pub fn DestroyCaret() -> ::minwindef::BOOL; /* winuser.h:8921:1 */
    pub fn DestroyCursor(_: ::windef::HCURSOR) -> ::minwindef::BOOL; /* winuser.h:10052:1 */
    pub fn DestroyIcon(_: ::windef::HICON) -> ::minwindef::BOOL; /* winuser.h:10175:1 */
    pub fn DestroyMenu(_: ::windef::HMENU) -> ::minwindef::BOOL; /* winuser.h:7112:1 */
    pub fn DestroyWindow(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4316:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DialogBoxIndirectParamW as DialogBoxIndirectParam; /* winuser.h:4886:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DialogBoxIndirectParamA(_: ::minwindef::HINSTANCE, _: ::winuser::LPCDLGTEMPLATEA, _: ::windef::HWND, _: ::winuser::DLGPROC, _: ::minwindef::LPARAM) -> ::basetsd::INT_PTR; /* winuser.h:4870:1 */
    pub fn DialogBoxIndirectParamW(_: ::minwindef::HINSTANCE, _: ::winuser::LPCDLGTEMPLATEW, _: ::windef::HWND, _: ::winuser::DLGPROC, _: ::minwindef::LPARAM) -> ::basetsd::INT_PTR; /* winuser.h:4879:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DialogBoxParamW as DialogBoxParam; /* winuser.h:4862:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DialogBoxParamA(_: ::minwindef::HINSTANCE, _: ::winnt::LPCSTR, _: ::windef::HWND, _: ::winuser::DLGPROC, _: ::minwindef::LPARAM) -> ::basetsd::INT_PTR; /* winuser.h:4846:1 */
    pub fn DialogBoxParamW(_: ::minwindef::HINSTANCE, _: ::winnt::LPCWSTR, _: ::windef::HWND, _: ::winuser::DLGPROC, _: ::minwindef::LPARAM) -> ::basetsd::INT_PTR; /* winuser.h:4855:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DispatchMessageW as DispatchMessage; /* winuser.h:3341:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DispatchMessageA(_: *const ::winuser::MSG) -> ::minwindef::LRESULT; /* winuser.h:3333:1 */
    pub fn DispatchMessageW(_: *const ::winuser::MSG) -> ::minwindef::LRESULT; /* winuser.h:3338:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DlgDirListW as DlgDirList; /* winuser.h:10932:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DlgDirListA(_: ::windef::HWND, _: ::winnt::LPSTR, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:10916:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DlgDirListComboBoxW as DlgDirListComboBox; /* winuser.h:10998:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DlgDirListComboBoxA(_: ::windef::HWND, _: ::winnt::LPSTR, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:10982:1 */
    pub fn DlgDirListComboBoxW(_: ::windef::HWND, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:10991:1 */
    pub fn DlgDirListW(_: ::windef::HWND, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:10925:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DlgDirSelectComboBoxExW as DlgDirSelectComboBoxEx; /* winuser.h:11020:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DlgDirSelectComboBoxExA(_: ::windef::HWND, _: ::winnt::LPSTR, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:11006:1 */
    pub fn DlgDirSelectComboBoxExW(_: ::windef::HWND, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:11014:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DlgDirSelectExW as DlgDirSelectEx; /* winuser.h:10974:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DlgDirSelectExA(_: ::windef::HWND, _: ::winnt::LPSTR, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:10960:1 */
    pub fn DlgDirSelectExW(_: ::windef::HWND, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:10968:1 */
    pub fn DrawFocusRect(_: ::windef::HDC, _: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9153:1 */
    pub fn DrawIcon(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HICON) -> ::minwindef::BOOL; /* winuser.h:7666:1 */
    pub fn DrawMenuBar(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:7080:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DrawTextW as DrawText; /* winuser.h:7765:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DrawTextA(_: ::windef::HDC, _: ::winnt::LPCSTR, _: ::libc::c_int, _: ::windef::LPRECT, _: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:7744:1 */
    pub fn DrawTextW(_: ::windef::HDC, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::windef::LPRECT, _: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:7756:1 */
    pub fn EmptyClipboard() -> ::minwindef::BOOL; /* winuser.h:5251:1 */
    pub fn EnableMenuItem(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7126:1 */
    pub fn EnableScrollBar(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:8355:1 */
    pub fn EnableWindow(_: ::windef::HWND, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:6684:1 */
    pub fn EndDeferWindowPos(_: ::winuser::HDWP) -> ::minwindef::BOOL; /* winuser.h:4606:1 */
    pub fn EndDialog(_: ::windef::HWND, _: ::basetsd::INT_PTR) -> ::minwindef::BOOL; /* winuser.h:4914:1 */
    pub fn EndPaint(_: ::windef::HWND, _: *const ::winuser::PAINTSTRUCT) -> ::minwindef::BOOL; /* winuser.h:8122:1 */
    pub fn EnumChildWindows(_: ::windef::HWND, _: ::winuser::WNDENUMPROC, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:9530:1 */
    pub fn EnumClipboardFormats(_: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:5225:1 */
    pub fn EnumDesktopWindows(_: ::windef::HDESK, _: ::winuser::WNDENUMPROC, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:1424:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumDesktopsW as EnumDesktops; /* winuser.h:1416:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumDesktopsA(_: ::minwindef::HWINSTA, _: ::winuser::DESKTOPENUMPROCA, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:1404:1 */
    pub fn EnumDesktopsW(_: ::minwindef::HWINSTA, _: ::winuser::DESKTOPENUMPROCW, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:1411:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumDisplaySettingsW as EnumDisplaySettings; /* winuser.h:12425:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumDisplaySettingsA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: *mut ::wingdi::DEVMODEA) -> ::minwindef::BOOL; /* winuser.h:12413:1 */
    pub fn EnumDisplaySettingsW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: *mut ::wingdi::DEVMODEW) -> ::minwindef::BOOL; /* winuser.h:12420:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumPropsW as EnumProps; /* winuser.h:8468:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumPropsA(_: ::windef::HWND, _: ::winuser::PROPENUMPROCA) -> ::libc::c_int; /* winuser.h:8458:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumPropsExW as EnumPropsEx; /* winuser.h:8450:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumPropsExA(_: ::windef::HWND, _: ::winuser::PROPENUMPROCEXA, _: ::minwindef::LPARAM) -> ::libc::c_int; /* winuser.h:8438:1 */
    pub fn EnumPropsExW(_: ::windef::HWND, _: ::winuser::PROPENUMPROCEXW, _: ::minwindef::LPARAM) -> ::libc::c_int; /* winuser.h:8445:1 */
    pub fn EnumPropsW(_: ::windef::HWND, _: ::winuser::PROPENUMPROCW) -> ::libc::c_int; /* winuser.h:8464:1 */
    pub fn EnumThreadWindows(_: ::minwindef::DWORD, _: ::winuser::WNDENUMPROC, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:9608:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumWindowStationsW as EnumWindowStations; /* winuser.h:1546:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumWindowStationsA(_: ::winuser::WINSTAENUMPROCA, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:1536:1 */
    pub fn EnumWindowStationsW(_: ::winuser::WINSTAENUMPROCW, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:1542:1 */
    pub fn EnumWindows(_: ::winuser::WNDENUMPROC, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:9601:1 */
    pub fn EqualRect(_: *const ::windef::RECT, _: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9252:1 */
    pub fn ExcludeUpdateRgn(_: ::windef::HDC, _: ::windef::HWND) -> ::libc::c_int; /* winuser.h:8178:1 */
    pub fn ExitWindowsEx(_: ::minwindef::UINT, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:3496:1 */
    pub fn FillRect(_: ::windef::HDC, _: *const ::windef::RECT, _: ::windef::HBRUSH) -> ::libc::c_int; /* winuser.h:9160:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindWindowW as FindWindow; /* winuser.h:9549:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindWindowA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::windef::HWND; /* winuser.h:9539:1 */
    pub fn FindWindowW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::windef::HWND; /* winuser.h:9545:1 */
    pub fn FlashWindow(_: ::windef::HWND, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:4470:1 */
    pub fn FrameRect(_: ::windef::HDC, _: *const ::windef::RECT, _: ::windef::HBRUSH) -> ::libc::c_int; /* winuser.h:9168:1 */
    pub fn GetActiveWindow() -> ::windef::HWND; /* winuser.h:5600:1 */
    pub fn GetAsyncKeyState(_: ::libc::c_int) -> ::winnt::SHORT; /* winuser.h:5624:1 */
    pub fn GetCapture() -> ::windef::HWND; /* winuser.h:6527:1 */
    pub fn GetCaretBlinkTime() -> ::minwindef::UINT; /* winuser.h:8909:1 */
    pub fn GetCaretPos(_: ::windef::LPPOINT) -> ::minwindef::BOOL; /* winuser.h:8946:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetClassInfoW as GetClassInfo; /* winuser.h:4171:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetClassInfoA(_: ::minwindef::HINSTANCE, _: ::winnt::LPCSTR, _: ::winuser::LPWNDCLASSA) -> ::minwindef::BOOL; /* winuser.h:4158:1 */
    pub fn GetClassInfoW(_: ::minwindef::HINSTANCE, _: ::winnt::LPCWSTR, _: ::winuser::LPWNDCLASSW) -> ::minwindef::BOOL; /* winuser.h:4166:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetClassLongW as GetClassLong; /* winuser.h:9406:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetClassLongA(_: ::windef::HWND, _: ::libc::c_int) -> ::minwindef::DWORD; /* winuser.h:9396:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetClassLongPtrW as GetClassLongPtr; /* winuser.h:9476:9, winuser.h:9446:9, winuser.h:9476:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetClassLongW(_: ::windef::HWND, _: ::libc::c_int) -> ::minwindef::DWORD; /* winuser.h:9402:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetClassNameW as GetClassName; /* winuser.h:9633:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetClassNameA(_: ::windef::HWND, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* winuser.h:9619:1 */
    pub fn GetClassNameW(_: ::windef::HWND, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winuser.h:9627:1 */
    pub fn GetClassWord(_: ::windef::HWND, _: ::libc::c_int) -> ::minwindef::WORD; /* winuser.h:9381:1 */
    pub fn GetClientRect(_: ::windef::HWND, _: ::windef::LPRECT) -> ::minwindef::BOOL; /* winuser.h:8532:1 */
    pub fn GetClipCursor(_: ::windef::LPRECT) -> ::minwindef::BOOL; /* winuser.h:8888:1 */
    pub fn GetClipboardData(_: ::minwindef::UINT) -> ::winnt::HANDLE; /* winuser.h:5197:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetClipboardFormatNameW as GetClipboardFormatName; /* winuser.h:5243:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetClipboardFormatNameA(_: ::minwindef::UINT, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* winuser.h:5231:1 */
    pub fn GetClipboardFormatNameW(_: ::minwindef::UINT, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winuser.h:5238:1 */
    pub fn GetClipboardOwner() -> ::windef::HWND; /* winuser.h:5165:1 */
    pub fn GetClipboardViewer() -> ::windef::HWND; /* winuser.h:5177:1 */
    pub fn GetCursor() -> ::windef::HCURSOR; /* winuser.h:8894:1 */
    pub fn GetCursorPos(_: ::windef::LPPOINT) -> ::minwindef::BOOL; /* winuser.h:8867:1 */
    pub fn GetDC(_: ::windef::HWND) -> ::windef::HDC; /* winuser.h:8065:1 */
    pub fn GetDCEx(_: ::windef::HWND, _: ::minwindef::HRGN, _: ::minwindef::DWORD) -> ::windef::HDC; /* winuser.h:8071:1 */
    pub fn GetDesktopWindow() -> ::windef::HWND; /* winuser.h:9510:1 */
    pub fn GetDialogBaseUnits() -> ::libc::c_long; /* winuser.h:5060:1 */
    pub fn GetDlgCtrlID(_: ::windef::HWND) -> ::libc::c_int; /* winuser.h:5054:1 */
    pub fn GetDlgItem(_: ::windef::HWND, _: ::libc::c_int) -> ::windef::HWND; /* winuser.h:4921:1 */
    pub fn GetDlgItemInt(_: ::windef::HWND, _: ::libc::c_int, _: *mut ::libc::c_int, _: ::minwindef::BOOL) -> ::minwindef::UINT; /* winuser.h:4937:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetDlgItemTextW as GetDlgItemText; /* winuser.h:4982:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetDlgItemTextA(_: ::windef::HWND, _: ::libc::c_int, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::minwindef::UINT; /* winuser.h:4967:1 */
    pub fn GetDlgItemTextW(_: ::windef::HWND, _: ::libc::c_int, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::minwindef::UINT; /* winuser.h:4976:1 */
    pub fn GetDoubleClickTime() -> ::minwindef::UINT; /* winuser.h:4111:1 */
    pub fn GetFocus() -> ::windef::HWND; /* winuser.h:5606:1 */
    pub fn GetForegroundWindow() -> ::windef::HWND; /* winuser.h:8011:1 */
    pub fn GetIconInfo(_: ::windef::HICON, _: ::winuser::PICONINFO) -> ::minwindef::BOOL; /* winuser.h:10337:1 */
    pub fn GetInputState() -> ::minwindef::BOOL; /* winuser.h:6514:1 */
    pub fn GetKBCodePage() -> ::minwindef::UINT; /* winuser.h:5612:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetKeyNameTextW as GetKeyNameText; /* winuser.h:5655:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetKeyNameTextA(_: ::winnt::LONG, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* winuser.h:5643:1 */
    pub fn GetKeyNameTextW(_: ::winnt::LONG, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winuser.h:5650:1 */
    pub fn GetKeyState(_: ::libc::c_int) -> ::winnt::SHORT; /* winuser.h:5618:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetKeyboardLayoutNameW as GetKeyboardLayoutName; /* winuser.h:1216:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetKeyboardLayoutNameA(_: ::winnt::LPSTR) -> ::minwindef::BOOL; /* winuser.h:1208:1 */
    pub fn GetKeyboardLayoutNameW(_: ::winnt::LPWSTR) -> ::minwindef::BOOL; /* winuser.h:1213:1 */
    pub fn GetKeyboardState(_: ::minwindef::PBYTE) -> ::minwindef::BOOL; /* winuser.h:5631:1 */
    pub fn GetKeyboardType(_: ::libc::c_int) -> ::libc::c_int; /* winuser.h:5663:1 */
    pub fn GetLastActivePopup(_: ::windef::HWND) -> ::windef::HWND; /* winuser.h:9695:1 */
    pub fn GetMenu(_: ::windef::HWND) -> ::windef::HMENU; /* winuser.h:7002:1 */
    pub fn GetMenuCheckMarkDimensions() -> ::winnt::LONG; /* winuser.h:7249:1 */
    pub fn GetMenuItemCount(_: ::windef::HMENU) -> ::libc::c_int; /* winuser.h:7148:1 */
    pub fn GetMenuItemID(_: ::windef::HMENU, _: ::libc::c_int) -> ::minwindef::UINT; /* winuser.h:7141:1 */
    pub fn GetMenuState(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:7072:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetMenuStringW as GetMenuString; /* winuser.h:7064:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetMenuStringA(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::winnt::LPSTR, _: ::libc::c_int, _: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:7048:1 */
    pub fn GetMenuStringW(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:7057:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetMessageW as GetMessage; /* winuser.h:3294:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetMessageA(_: ::winuser::LPMSG, _: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:3280:1 */
    pub fn GetMessageExtraInfo() -> ::minwindef::LPARAM; /* winuser.h:3521:1 */
    pub fn GetMessagePos() -> ::minwindef::DWORD; /* winuser.h:3509:1 */
    pub fn GetMessageTime() -> ::winnt::LONG; /* winuser.h:3515:1 */
    pub fn GetMessageW(_: ::winuser::LPMSG, _: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:3288:1 */
    pub fn GetNextDlgGroupItem(_: ::windef::HWND, _: ::windef::HWND, _: ::minwindef::BOOL) -> ::windef::HWND; /* winuser.h:5038:1 */
    pub fn GetNextDlgTabItem(_: ::windef::HWND, _: ::windef::HWND, _: ::minwindef::BOOL) -> ::windef::HWND; /* winuser.h:5046:1 */
    pub fn GetOpenClipboardWindow() -> ::windef::HWND; /* winuser.h:5270:1 */
    pub fn GetParent(_: ::windef::HWND) -> ::windef::HWND; /* winuser.h:9517:1 */
    pub fn GetPriorityClipboardFormat(_: *mut ::libc::c_uint, _: ::libc::c_int) -> ::libc::c_int; /* winuser.h:5263:1 */
    pub fn GetProcessWindowStation() -> ::minwindef::HWINSTA; /* winuser.h:1566:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetPropW as GetProp; /* winuser.h:8412:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetPropA(_: ::windef::HWND, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winuser.h:8402:1 */
    pub fn GetPropW(_: ::windef::HWND, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winuser.h:8408:1 */
    pub fn GetQueueStatus(_: ::minwindef::UINT) -> ::minwindef::DWORD; /* winuser.h:6520:1 */
    pub fn GetScrollPos(_: ::windef::HWND, _: ::libc::c_int) -> ::libc::c_int; /* winuser.h:8321:1 */
    pub fn GetScrollRange(_: ::windef::HWND, _: ::libc::c_int, _: ::minwindef::LPINT, _: ::minwindef::LPINT) -> ::minwindef::BOOL; /* winuser.h:8338:1 */
    pub fn GetSubMenu(_: ::windef::HMENU, _: ::libc::c_int) -> ::windef::HMENU; /* winuser.h:7134:1 */
    pub fn GetSysColor(_: ::libc::c_int) -> ::minwindef::DWORD; /* winuser.h:9121:1 */
    pub fn GetSystemMenu(_: ::windef::HWND, _: ::minwindef::BOOL) -> ::windef::HMENU; /* winuser.h:7092:1 */
    pub fn GetSystemMetrics(_: ::libc::c_int) -> ::libc::c_int; /* winuser.h:6951:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTabbedTextExtentW as GetTabbedTextExtent; /* winuser.h:7990:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTabbedTextExtentA(_: ::windef::HDC, _: ::winnt::LPCSTR, _: ::libc::c_int, _: ::libc::c_int, _: *const ::libc::c_int) -> ::minwindef::DWORD; /* winuser.h:7974:1 */
    pub fn GetTabbedTextExtentW(_: ::windef::HDC, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::libc::c_int, _: *const ::libc::c_int) -> ::minwindef::DWORD; /* winuser.h:7983:1 */
    pub fn GetThreadDesktop(_: ::minwindef::DWORD) -> ::windef::HDESK; /* winuser.h:1452:1 */
    pub fn GetTopWindow(_: ::windef::HWND) -> ::windef::HWND; /* winuser.h:9665:1 */
    pub fn GetUpdateRect(_: ::windef::HWND, _: ::windef::LPRECT, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:8129:1 */
    pub fn GetUpdateRgn(_: ::windef::HWND, _: ::minwindef::HRGN, _: ::minwindef::BOOL) -> ::libc::c_int; /* winuser.h:8137:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetUserObjectInformationW as GetUserObjectInformation; /* winuser.h:1637:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetUserObjectInformationA(_: ::winnt::HANDLE, _: ::libc::c_int, _: ::winnt::PVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winuser.h:1621:1 */
    pub fn GetUserObjectInformationW(_: ::winnt::HANDLE, _: ::libc::c_int, _: ::winnt::PVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winuser.h:1630:1 */
    pub fn GetUserObjectSecurity(_: ::winnt::HANDLE, _: ::winnt::PSECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winuser.h:1590:1 */
    pub fn GetWindow(_: ::windef::HWND, _: ::minwindef::UINT) -> ::windef::HWND; /* winuser.h:9717:1 */
    pub fn GetWindowDC(_: ::windef::HWND) -> ::windef::HDC; /* winuser.h:8102:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetWindowLongW as GetWindowLong; /* winuser.h:9293:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetWindowLongA(_: ::windef::HWND, _: ::libc::c_int) -> ::winnt::LONG; /* winuser.h:9283:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetWindowLongPtrW as GetWindowLongPtr; /* winuser.h:9363:9, winuser.h:9333:9, winuser.h:9363:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetWindowLongW(_: ::windef::HWND, _: ::libc::c_int) -> ::winnt::LONG; /* winuser.h:9289:1 */
    pub fn GetWindowPlacement(_: ::windef::HWND, _: *mut ::winuser::WINDOWPLACEMENT) -> ::minwindef::BOOL; /* winuser.h:4543:1 */
    pub fn GetWindowRect(_: ::windef::HWND, _: ::windef::LPRECT) -> ::minwindef::BOOL; /* winuser.h:8539:1 */
    pub fn GetWindowRgn(_: ::windef::HWND, _: ::minwindef::HRGN) -> ::libc::c_int; /* winuser.h:8160:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetWindowTextW as GetWindowText; /* winuser.h:8508:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetWindowTextA(_: ::windef::HWND, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* winuser.h:8495:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetWindowTextLengthW as GetWindowTextLength; /* winuser.h:8524:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetWindowTextLengthA(_: ::windef::HWND) -> ::libc::c_int; /* winuser.h:8516:1 */
    pub fn GetWindowTextLengthW(_: ::windef::HWND) -> ::libc::c_int; /* winuser.h:8521:1 */
    pub fn GetWindowTextW(_: ::windef::HWND, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winuser.h:8503:1 */
    pub fn GetWindowThreadProcessId(_: ::windef::HWND, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winuser.h:9675:1 */
    pub fn GetWindowWord(_: ::windef::HWND, _: ::libc::c_int) -> ::minwindef::WORD; /* winuser.h:9268:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GrayStringW as GrayString; /* winuser.h:7868:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GrayStringA(_: ::windef::HDC, _: ::windef::HBRUSH, _: ::winuser::GRAYSTRINGPROC, _: ::minwindef::LPARAM, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:7844:1 */
    pub fn GrayStringW(_: ::windef::HDC, _: ::windef::HBRUSH, _: ::winuser::GRAYSTRINGPROC, _: ::minwindef::LPARAM, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:7857:1 */
    pub fn HideCaret(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:8927:1 */
    pub fn HiliteMenuItem(_: ::windef::HWND, _: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7039:1 */
    pub fn InSendMessage() -> ::minwindef::BOOL; /* winuser.h:4074:1 */
    pub fn InflateRect(_: ::windef::LPRECT, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:9206:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::InsertMenuW as InsertMenu; /* winuser.h:7170:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn InsertMenuA(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::basetsd::UINT_PTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winuser.h:7154:1 */
    pub fn InsertMenuW(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::basetsd::UINT_PTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winuser.h:7163:1 */
    pub fn InternalGetWindowText(_: ::windef::HWND, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winuser.h:12837:1 */
    pub fn IntersectRect(_: ::windef::LPRECT, _: *const ::windef::RECT, _: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9214:1 */
    pub fn InvalidateRect(_: ::windef::HWND, _: *const ::windef::RECT, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:8185:1 */
    pub fn InvalidateRgn(_: ::windef::HWND, _: ::minwindef::HRGN, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:8200:1 */
    pub fn InvertRect(_: ::windef::HDC, _: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9176:1 */
    pub fn IsCharAlphaA(_: ::winnt::CHAR) -> ::minwindef::BOOL; /* winuser.h:5528:1 */
    pub fn IsCharAlphaNumericA(_: ::winnt::CHAR) -> ::minwindef::BOOL; /* winuser.h:5544:1 */
    pub fn IsCharLowerA(_: ::winnt::CHAR) -> ::minwindef::BOOL; /* winuser.h:5576:1 */
    pub fn IsCharUpperA(_: ::winnt::CHAR) -> ::minwindef::BOOL; /* winuser.h:5560:1 */
    pub fn IsChild(_: ::windef::HWND, _: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4309:1 */
    pub fn IsClipboardFormatAvailable(_: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:5257:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsDialogMessageW as IsDialogMessage; /* winuser.h:10899:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn IsDialogMessageA(_: ::windef::HWND, _: ::winuser::LPMSG) -> ::minwindef::BOOL; /* winuser.h:10889:1 */
    pub fn IsDialogMessageW(_: ::windef::HWND, _: ::winuser::LPMSG) -> ::minwindef::BOOL; /* winuser.h:10895:1 */
    pub fn IsDlgButtonChecked(_: ::windef::HWND, _: ::libc::c_int) -> ::minwindef::UINT; /* winuser.h:5007:1 */
    pub fn IsHungAppWindow(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:1757:1 */
    pub fn IsIconic(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4626:1 */
    pub fn IsMenu(_: ::windef::HMENU) -> ::minwindef::BOOL; /* winuser.h:4303:1 */
    pub fn IsRectEmpty(_: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9246:1 */
    pub fn IsWindow(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4296:1 */
    pub fn IsWindowEnabled(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:6691:1 */
    pub fn IsWindowUnicode(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:6678:1 */
    pub fn IsWindowVisible(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4620:1 */
    pub fn IsZoomed(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4644:1 */
    pub fn KillTimer(_: ::windef::HWND, _: ::basetsd::UINT_PTR) -> ::minwindef::BOOL; /* winuser.h:6671:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadAcceleratorsW as LoadAccelerators; /* winuser.h:6707:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadAcceleratorsA(_: ::minwindef::HINSTANCE, _: ::winnt::LPCSTR) -> ::windef::HACCEL; /* winuser.h:6697:1 */
    pub fn LoadAcceleratorsW(_: ::minwindef::HINSTANCE, _: ::winnt::LPCWSTR) -> ::windef::HACCEL; /* winuser.h:6703:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadBitmapW as LoadBitmap; /* winuser.h:9998:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadBitmapA(_: ::minwindef::HINSTANCE, _: ::winnt::LPCSTR) -> ::windef::HBITMAP; /* winuser.h:9988:1 */
    pub fn LoadBitmapW(_: ::minwindef::HINSTANCE, _: ::winnt::LPCWSTR) -> ::windef::HBITMAP; /* winuser.h:9994:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadCursorW as LoadCursor; /* winuser.h:10016:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadCursorA(_: ::minwindef::HINSTANCE, _: ::winnt::LPCSTR) -> ::windef::HCURSOR; /* winuser.h:10006:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadCursorFromFileW as LoadCursorFromFile; /* winuser.h:10032:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadCursorFromFileA(_: ::winnt::LPCSTR) -> ::windef::HCURSOR; /* winuser.h:10024:1 */
    pub fn LoadCursorFromFileW(_: ::winnt::LPCWSTR) -> ::windef::HCURSOR; /* winuser.h:10029:1 */
    pub fn LoadCursorW(_: ::minwindef::HINSTANCE, _: ::winnt::LPCWSTR) -> ::windef::HCURSOR; /* winuser.h:10012:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadIconW as LoadIcon; /* winuser.h:10124:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadIconA(_: ::minwindef::HINSTANCE, _: ::winnt::LPCSTR) -> ::windef::HICON; /* winuser.h:10114:1 */
    pub fn LoadIconW(_: ::minwindef::HINSTANCE, _: ::winnt::LPCWSTR) -> ::windef::HICON; /* winuser.h:10120:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadKeyboardLayoutW as LoadKeyboardLayout; /* winuser.h:1163:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadKeyboardLayoutA(_: ::winnt::LPCSTR, _: ::minwindef::UINT) -> ::minwindef::HKL; /* winuser.h:1153:1 */
    pub fn LoadKeyboardLayoutW(_: ::winnt::LPCWSTR, _: ::minwindef::UINT) -> ::minwindef::HKL; /* winuser.h:1159:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadMenuW as LoadMenu; /* winuser.h:6978:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadMenuA(_: ::minwindef::HINSTANCE, _: ::winnt::LPCSTR) -> ::windef::HMENU; /* winuser.h:6968:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadMenuIndirectW as LoadMenuIndirect; /* winuser.h:6994:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadMenuIndirectA(_: *const ::libc::c_void) -> ::windef::HMENU; /* winuser.h:6986:1 */
    pub fn LoadMenuIndirectW(_: *const ::libc::c_void) -> ::windef::HMENU; /* winuser.h:6991:1 */
    pub fn LoadMenuW(_: ::minwindef::HINSTANCE, _: ::winnt::LPCWSTR) -> ::windef::HMENU; /* winuser.h:6974:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadStringW as LoadString; /* libloaderapi.h:453:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadStringA(_: ::minwindef::HINSTANCE, _: ::minwindef::UINT, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* libloaderapi.h:435:1 */
    pub fn LoadStringW(_: ::minwindef::HINSTANCE, _: ::minwindef::UINT, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* libloaderapi.h:445:1 */
    pub fn LockWindowUpdate(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:8256:1 */
    pub fn LookupIconIdFromDirectory(_: ::minwindef::PBYTE, _: ::minwindef::BOOL) -> ::libc::c_int; /* winuser.h:10181:1 */
    pub fn MapDialogRect(_: ::windef::HWND, _: ::windef::LPRECT) -> ::minwindef::BOOL; /* winuser.h:10909:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::MapVirtualKeyW as MapVirtualKey; /* winuser.h:6476:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn MapVirtualKeyA(_: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:6466:1 */
    pub fn MapVirtualKeyW(_: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:6472:1 */
    pub fn MapWindowPoints(_: ::windef::HWND, _: ::windef::HWND, _: ::windef::LPPOINT, _: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:9000:1 */
    pub fn MessageBeep(_: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:8824:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::MessageBoxW as MessageBox; /* winuser.h:8703:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn MessageBoxA(_: ::windef::HWND, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:8689:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::MessageBoxExW as MessageBoxEx; /* winuser.h:8751:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn MessageBoxExA(_: ::windef::HWND, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::UINT, _: ::minwindef::WORD) -> ::libc::c_int; /* winuser.h:8735:1 */
    pub fn MessageBoxExW(_: ::windef::HWND, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::UINT, _: ::minwindef::WORD) -> ::libc::c_int; /* winuser.h:8744:1 */
    pub fn MessageBoxW(_: ::windef::HWND, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:8697:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ModifyMenuW as ModifyMenu; /* winuser.h:7216:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ModifyMenuA(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::basetsd::UINT_PTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winuser.h:7200:1 */
    pub fn ModifyMenuW(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::basetsd::UINT_PTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winuser.h:7209:1 */
    pub fn MoveWindow(_: ::windef::HWND, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:4520:1 */
    pub fn MsgWaitForMultipleObjects(_: ::minwindef::DWORD, _: *const *mut ::libc::c_void, _: ::minwindef::BOOL, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:6545:1 */
    pub fn MsgWaitForMultipleObjectsEx(_: ::minwindef::DWORD, _: *const *mut ::libc::c_void, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:6555:1 */
    pub fn OemKeyScan(_: ::minwindef::WORD) -> ::minwindef::DWORD; /* winuser.h:5703:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OemToCharW as OemToChar; /* winuser.h:5334:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OemToCharA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR) -> ::minwindef::BOOL; /* winuser.h:5323:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OemToCharBuffW as OemToCharBuff; /* winuser.h:5374:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OemToCharBuffA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:5362:1 */
    pub fn OemToCharBuffW(_: ::winnt::LPCSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:5369:1 */
    pub fn OemToCharW(_: ::winnt::LPCSTR, _: ::winnt::LPWSTR) -> ::minwindef::BOOL; /* winuser.h:5330:1 */
    pub fn OffsetRect(_: ::windef::LPRECT, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:9238:1 */
    pub fn OpenClipboard(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:5142:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenDesktopW as OpenDesktop; /* winuser.h:1387:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenDesktopA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::ACCESS_MASK) -> ::windef::HDESK; /* winuser.h:1373:1 */
    pub fn OpenDesktopW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::ACCESS_MASK) -> ::windef::HDESK; /* winuser.h:1381:1 */
    pub fn OpenIcon(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:4508:1 */
    pub fn OpenInputDesktop(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::ACCESS_MASK) -> ::windef::HDESK; /* winuser.h:1395:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenWindowStationW as OpenWindowStation; /* winuser.h:1528:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenWindowStationA(_: ::winnt::LPCSTR, _: ::minwindef::BOOL, _: ::winnt::ACCESS_MASK) -> ::minwindef::HWINSTA; /* winuser.h:1516:1 */
    pub fn OpenWindowStationW(_: ::winnt::LPCWSTR, _: ::minwindef::BOOL, _: ::winnt::ACCESS_MASK) -> ::minwindef::HWINSTA; /* winuser.h:1523:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PeekMessageW as PeekMessage; /* winuser.h:3389:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PeekMessageA(_: ::winuser::LPMSG, _: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:3373:1 */
    pub fn PeekMessageW(_: ::winuser::LPMSG, _: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:3382:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PostMessageW as PostMessage; /* winuser.h:3895:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PostMessageA(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:3881:1 */
    pub fn PostMessageW(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:3889:1 */
    pub fn PostQuitMessage(_: ::libc::c_int); /* winuser.h:4014:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PostThreadMessageW as PostThreadMessage; /* winuser.h:3917:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PostThreadMessageA(_: ::minwindef::DWORD, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:3903:1 */
    pub fn PostThreadMessageW(_: ::minwindef::DWORD, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:3911:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PrivateExtractIconsW as PrivateExtractIcons; /* winuser.h:10155:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PrivateExtractIconsA(_: ::winnt::LPCSTR, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: *mut *mut ::windef::HICON__, _: *mut ::libc::c_uint, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:10133:1 */
    pub fn PrivateExtractIconsW(_: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: *mut *mut ::windef::HICON__, _: *mut ::libc::c_uint, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:10145:1 */
    pub fn PtInRect(_: *const ::windef::RECT, _: ::windef::POINT) -> ::minwindef::BOOL; /* winuser.h:9259:1 */
    pub fn RedrawWindow(_: ::windef::HWND, _: *const ::windef::RECT, _: ::minwindef::HRGN, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:8216:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegisterClassW as RegisterClass; /* winuser.h:4131:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegisterClassA(_: *const ::winuser::WNDCLASSA) -> ::minwindef::ATOM; /* winuser.h:4123:1 */
    pub fn RegisterClassW(_: *const ::winuser::WNDCLASSW) -> ::minwindef::ATOM; /* winuser.h:4128:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegisterClipboardFormatW as RegisterClipboardFormat; /* winuser.h:5211:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegisterClipboardFormatA(_: ::winnt::LPCSTR) -> ::minwindef::UINT; /* winuser.h:5203:1 */
    pub fn RegisterClipboardFormatW(_: ::winnt::LPCWSTR) -> ::minwindef::UINT; /* winuser.h:5208:1 */
    pub fn RegisterHotKey(_: ::windef::HWND, _: ::libc::c_int, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:3419:1 */
    pub fn RegisterShellHookWindow(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:9589:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegisterWindowMessageW as RegisterWindowMessage; /* winuser.h:2541:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegisterWindowMessageA(_: ::winnt::LPCSTR) -> ::minwindef::UINT; /* winuser.h:2533:1 */
    pub fn RegisterWindowMessageW(_: ::winnt::LPCWSTR) -> ::minwindef::UINT; /* winuser.h:2538:1 */
    pub fn ReleaseCapture() -> ::minwindef::BOOL; /* winuser.h:6539:1 */
    pub fn ReleaseDC(_: ::windef::HWND, _: ::windef::HDC) -> ::libc::c_int; /* winuser.h:8108:1 */
    pub fn RemoveMenu(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7223:8 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RemovePropW as RemoveProp; /* winuser.h:8430:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RemovePropA(_: ::windef::HWND, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winuser.h:8420:1 */
    pub fn RemovePropW(_: ::windef::HWND, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winuser.h:8426:1 */
    pub fn ReplyMessage(_: ::minwindef::LRESULT) -> ::minwindef::BOOL; /* winuser.h:3959:1 */
    pub fn ScreenToClient(_: ::windef::HWND, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* winuser.h:8959:1 */
    pub fn ScrollDC(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: *const ::windef::RECT, _: *const ::windef::RECT, _: ::minwindef::HRGN, _: ::windef::LPRECT) -> ::minwindef::BOOL; /* winuser.h:8272:1 */
    pub fn ScrollWindow(_: ::windef::HWND, _: ::libc::c_int, _: ::libc::c_int, _: *const ::windef::RECT, _: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:8262:1 */
    pub fn ScrollWindowEx(_: ::windef::HWND, _: ::libc::c_int, _: ::libc::c_int, _: *const ::windef::RECT, _: *const ::windef::RECT, _: ::minwindef::HRGN, _: ::windef::LPRECT, _: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:8284:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SendDlgItemMessageW as SendDlgItemMessage; /* winuser.h:5030:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SendDlgItemMessageA(_: ::windef::HWND, _: ::libc::c_int, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:5014:1 */
    pub fn SendDlgItemMessageW(_: ::windef::HWND, _: ::libc::c_int, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:5023:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SendMessageW as SendMessage; /* winuser.h:3565:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SendMessageA(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:3551:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SendMessageCallbackW as SendMessageCallback; /* winuser.h:3667:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SendMessageCallbackA(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM, _: ::winuser::SENDASYNCPROC, _: ::basetsd::ULONG_PTR) -> ::minwindef::BOOL; /* winuser.h:3649:1 */
    pub fn SendMessageCallbackW(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM, _: ::winuser::SENDASYNCPROC, _: ::basetsd::ULONG_PTR) -> ::minwindef::BOOL; /* winuser.h:3659:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SendMessageTimeoutW as SendMessageTimeout; /* winuser.h:3619:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SendMessageTimeoutA(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::basetsd::PDWORD_PTR) -> ::minwindef::LRESULT; /* winuser.h:3599:1 */
    pub fn SendMessageTimeoutW(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::basetsd::PDWORD_PTR) -> ::minwindef::LRESULT; /* winuser.h:3610:1 */
    pub fn SendMessageW(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::LRESULT; /* winuser.h:3559:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SendNotifyMessageW as SendNotifyMessage; /* winuser.h:3641:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SendNotifyMessageA(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:3627:1 */
    pub fn SendNotifyMessageW(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:3635:1 */
    pub fn SetActiveWindow(_: ::windef::HWND) -> ::windef::HWND; /* winuser.h:8004:1 */
    pub fn SetCapture(_: ::windef::HWND) -> ::windef::HWND; /* winuser.h:6533:1 */
    pub fn SetCaretBlinkTime(_: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:8915:1 */
    pub fn SetCaretPos(_: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:8939:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetClassLongW as SetClassLong; /* winuser.h:9426:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetClassLongA(_: ::windef::HWND, _: ::libc::c_int, _: ::winnt::LONG) -> ::minwindef::DWORD; /* winuser.h:9414:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetClassLongPtrW as SetClassLongPtr; /* winuser.h:9484:9, winuser.h:9466:9, winuser.h:9484:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetClassLongW(_: ::windef::HWND, _: ::libc::c_int, _: ::winnt::LONG) -> ::minwindef::DWORD; /* winuser.h:9421:1 */
    pub fn SetClassWord(_: ::windef::HWND, _: ::libc::c_int, _: ::minwindef::WORD) -> ::minwindef::WORD; /* winuser.h:9388:1 */
    pub fn SetClipboardData(_: ::minwindef::UINT, _: ::winnt::HANDLE) -> ::winnt::HANDLE; /* winuser.h:5190:1 */
    pub fn SetClipboardViewer(_: ::windef::HWND) -> ::windef::HWND; /* winuser.h:5171:1 */
    pub fn SetCursor(_: ::windef::HCURSOR) -> ::windef::HCURSOR; /* winuser.h:8861:1 */
    pub fn SetCursorPos(_: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:8845:1 */
    pub fn SetDebugErrorLevel(_: ::minwindef::DWORD); /* winuser.h:12810:1 */
    pub fn SetDlgItemInt(_: ::windef::HWND, _: ::libc::c_int, _: ::minwindef::UINT, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:4928:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetDlgItemTextW as SetDlgItemText; /* winuser.h:4958:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetDlgItemTextA(_: ::windef::HWND, _: ::libc::c_int, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winuser.h:4946:1 */
    pub fn SetDlgItemTextW(_: ::windef::HWND, _: ::libc::c_int, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winuser.h:4953:1 */
    pub fn SetDoubleClickTime(_: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:4117:1 */
    pub fn SetFocus(_: ::windef::HWND) -> ::windef::HWND; /* winuser.h:5594:1 */
    pub fn SetForegroundWindow(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:8033:1 */
    pub fn SetKeyboardState(_: ::minwindef::LPBYTE) -> ::minwindef::BOOL; /* winuser.h:5637:1 */
    pub fn SetLastErrorEx(_: ::minwindef::DWORD, _: ::minwindef::DWORD); /* winuser.h:12830:1 */
    pub fn SetMenu(_: ::windef::HWND, _: ::windef::HMENU) -> ::minwindef::BOOL; /* winuser.h:7008:1 */
    pub fn SetMenuItemBitmaps(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::windef::HBITMAP, _: ::windef::HBITMAP) -> ::minwindef::BOOL; /* winuser.h:7239:1 */
    pub fn SetMessageQueue(_: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:3367:1 */
    pub fn SetParent(_: ::windef::HWND, _: ::windef::HWND) -> ::windef::HWND; /* winuser.h:9523:1 */
    pub fn SetProcessWindowStation(_: ::minwindef::HWINSTA) -> ::minwindef::BOOL; /* winuser.h:1560:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetPropW as SetProp; /* winuser.h:8394:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetPropA(_: ::windef::HWND, _: ::winnt::LPCSTR, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winuser.h:8382:1 */
    pub fn SetPropW(_: ::windef::HWND, _: ::winnt::LPCWSTR, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winuser.h:8389:1 */
    pub fn SetRect(_: ::windef::LPRECT, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:9183:1 */
    pub fn SetRectEmpty(_: ::windef::LPRECT) -> ::minwindef::BOOL; /* winuser.h:9193:1 */
    pub fn SetScrollPos(_: ::windef::HWND, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::BOOL) -> ::libc::c_int; /* winuser.h:8312:1 */
    pub fn SetScrollRange(_: ::windef::HWND, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:8328:1 */
    pub fn SetSysColors(_: ::libc::c_int, _: *const ::libc::c_int, _: *const ::libc::c_ulong) -> ::minwindef::BOOL; /* winuser.h:9137:1 */
    pub fn SetSystemCursor(_: ::windef::HCURSOR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:10098:1 */
    pub fn SetThreadDesktop(_: ::windef::HDESK) -> ::minwindef::BOOL; /* winuser.h:1440:1 */
    pub fn SetTimer(_: ::windef::HWND, _: ::basetsd::UINT_PTR, _: ::minwindef::UINT, _: ::winuser::TIMERPROC) -> ::basetsd::UINT_PTR; /* winuser.h:6642:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetUserObjectInformationW as SetUserObjectInformation; /* winuser.h:1659:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetUserObjectInformationA(_: ::winnt::HANDLE, _: ::libc::c_int, _: ::winnt::PVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:1645:1 */
    pub fn SetUserObjectInformationW(_: ::winnt::HANDLE, _: ::libc::c_int, _: ::winnt::PVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:1653:1 */
    pub fn SetUserObjectSecurity(_: ::winnt::HANDLE, _: ::winnt::PSECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* winuser.h:1582:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetWindowLongW as SetWindowLong; /* winuser.h:9313:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetWindowLongA(_: ::windef::HWND, _: ::libc::c_int, _: ::winnt::LONG) -> ::winnt::LONG; /* winuser.h:9301:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetWindowLongPtrW as SetWindowLongPtr; /* winuser.h:9371:9, winuser.h:9353:9, winuser.h:9371:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetWindowLongW(_: ::windef::HWND, _: ::libc::c_int, _: ::winnt::LONG) -> ::winnt::LONG; /* winuser.h:9308:1 */
    pub fn SetWindowPlacement(_: ::windef::HWND, _: *const ::winuser::WINDOWPLACEMENT) -> ::minwindef::BOOL; /* winuser.h:4550:1 */
    pub fn SetWindowPos(_: ::windef::HWND, _: ::windef::HWND, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:4531:1 */
    pub fn SetWindowRgn(_: ::windef::HWND, _: ::minwindef::HRGN, _: ::minwindef::BOOL) -> ::libc::c_int; /* winuser.h:8145:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetWindowTextW as SetWindowText; /* winuser.h:8486:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetWindowTextA(_: ::windef::HWND, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winuser.h:8476:1 */
    pub fn SetWindowTextW(_: ::windef::HWND, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winuser.h:8482:1 */
    pub fn SetWindowWord(_: ::windef::HWND, _: ::libc::c_int, _: ::minwindef::WORD) -> ::minwindef::WORD; /* winuser.h:9275:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetWindowsHookW as SetWindowsHook; /* winuser.h:9739:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetWindowsHookA(_: ::libc::c_int, _: ::winuser::HOOKPROC) -> ::windef::HHOOK; /* winuser.h:9729:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetWindowsHookExW as SetWindowsHookEx; /* winuser.h:9790:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetWindowsHookExA(_: ::libc::c_int, _: ::winuser::HOOKPROC, _: ::minwindef::HINSTANCE, _: ::minwindef::DWORD) -> ::windef::HHOOK; /* winuser.h:9776:1 */
    pub fn SetWindowsHookExW(_: ::libc::c_int, _: ::winuser::HOOKPROC, _: ::minwindef::HINSTANCE, _: ::minwindef::DWORD) -> ::windef::HHOOK; /* winuser.h:9784:1 */
    pub fn SetWindowsHookW(_: ::libc::c_int, _: ::winuser::HOOKPROC) -> ::windef::HHOOK; /* winuser.h:9735:1 */
    pub fn ShowCaret(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:8933:1 */
    pub fn ShowCursor(_: ::minwindef::BOOL) -> ::libc::c_int; /* winuser.h:8839:1 */
    pub fn ShowOwnedPopups(_: ::windef::HWND, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:4501:1 */
    pub fn ShowScrollBar(_: ::windef::HWND, _: ::libc::c_int, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:8347:1 */
    pub fn ShowWindow(_: ::windef::HWND, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:4322:1 */
    pub fn ShutdownBlockReasonCreate(_: ::windef::HWND, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winuser.h:14961:1 */
    pub fn ShutdownBlockReasonDestroy(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:14976:1 */
    pub fn ShutdownBlockReasonQuery(_: ::windef::HWND, _: ::winnt::LPWSTR, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winuser.h:14968:1 */
    pub fn SubtractRect(_: ::windef::LPRECT, _: *const ::windef::RECT, _: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9230:1 */
    pub fn SwapMouseButton(_: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:3503:1 */
    pub fn SwitchDesktop(_: ::windef::HDESK) -> ::minwindef::BOOL; /* winuser.h:1433:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SystemParametersInfoW as SystemParametersInfo; /* winuser.h:12555:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SystemParametersInfoA(_: ::minwindef::UINT, _: ::minwindef::UINT, _: ::winnt::PVOID, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:12540:1 */
    pub fn SystemParametersInfoW(_: ::minwindef::UINT, _: ::minwindef::UINT, _: ::winnt::PVOID, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:12549:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::TabbedTextOutW as TabbedTextOut; /* winuser.h:7966:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn TabbedTextOutA(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::winnt::LPCSTR, _: ::libc::c_int, _: ::libc::c_int, _: *const ::libc::c_int, _: ::libc::c_int) -> ::winnt::LONG; /* winuser.h:7944:1 */
    pub fn TabbedTextOutW(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::libc::c_int, _: *const ::libc::c_int, _: ::libc::c_int) -> ::winnt::LONG; /* winuser.h:7956:1 */
    pub fn ToAscii(_: ::minwindef::UINT, _: ::minwindef::UINT, _: *const ::libc::c_uchar, _: ::minwindef::LPWORD, _: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:5669:1 */
    pub fn ToUnicode(_: ::minwindef::UINT, _: ::minwindef::UINT, _: *const ::libc::c_uchar, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:5692:1 */
    pub fn TrackPopupMenu(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HWND, _: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:7255:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::TranslateAcceleratorW as TranslateAccelerator; /* winuser.h:6773:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn TranslateAcceleratorA(_: ::windef::HWND, _: ::windef::HACCEL, _: ::winuser::LPMSG) -> ::libc::c_int; /* winuser.h:6761:1 */
    pub fn TranslateAcceleratorW(_: ::windef::HWND, _: ::windef::HACCEL, _: ::winuser::LPMSG) -> ::libc::c_int; /* winuser.h:6768:1 */
    pub fn TranslateMDISysAccel(_: ::windef::HWND, _: ::winuser::LPMSG) -> ::minwindef::BOOL; /* winuser.h:11526:1 */
    pub fn TranslateMessage(_: *const ::winuser::MSG) -> ::minwindef::BOOL; /* winuser.h:3327:1 */
    pub fn UnhookWindowsHook(_: ::libc::c_int, _: ::winuser::HOOKPROC) -> ::minwindef::BOOL; /* winuser.h:9769:1 */
    pub fn UnhookWindowsHookEx(_: ::windef::HHOOK) -> ::minwindef::BOOL; /* winuser.h:9798:1 */
    pub fn UnionRect(_: ::windef::LPRECT, _: *const ::windef::RECT, _: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:9222:1 */
    pub fn UnloadKeyboardLayout(_: ::minwindef::HKL) -> ::minwindef::BOOL; /* winuser.h:1202:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::UnregisterClassW as UnregisterClass; /* winuser.h:4149:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn UnregisterClassA(_: ::winnt::LPCSTR, _: ::minwindef::HINSTANCE) -> ::minwindef::BOOL; /* winuser.h:4139:1 */
    pub fn UnregisterClassW(_: ::winnt::LPCWSTR, _: ::minwindef::HINSTANCE) -> ::minwindef::BOOL; /* winuser.h:4145:1 */
    pub fn UnregisterHotKey(_: ::windef::HWND, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:3428:1 */
    pub fn UpdateWindow(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:7998:1 */
    pub fn ValidateRect(_: ::windef::HWND, _: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:8193:1 */
    pub fn ValidateRgn(_: ::windef::HWND, _: ::minwindef::HRGN) -> ::minwindef::BOOL; /* winuser.h:8208:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VkKeyScanW as VkKeyScan; /* winuser.h:5717:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VkKeyScanA(_: ::winnt::CHAR) -> ::winnt::SHORT; /* winuser.h:5709:1 */
    pub fn VkKeyScanW(_: ::winnt::WCHAR) -> ::winnt::SHORT; /* winuser.h:5714:1 */
    pub fn WaitForInputIdle(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:3975:1 */
    pub fn WaitMessage() -> ::minwindef::BOOL; /* winuser.h:3965:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WinHelpW as WinHelp; /* winuser.h:11722:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WinHelpA(_: ::windef::HWND, _: ::winnt::LPCSTR, _: ::minwindef::UINT, _: ::basetsd::ULONG_PTR) -> ::minwindef::BOOL; /* winuser.h:11708:1 */
    pub fn WinHelpW(_: ::windef::HWND, _: ::winnt::LPCWSTR, _: ::minwindef::UINT, _: ::basetsd::ULONG_PTR) -> ::minwindef::BOOL; /* winuser.h:11716:1 */
    pub fn WindowFromDC(_: ::windef::HDC) -> ::windef::HWND; /* winuser.h:8059:1 */
    pub fn WindowFromPoint(_: ::windef::POINT) -> ::windef::HWND; /* winuser.h:9009:1 */
    pub fn keybd_event(_: ::minwindef::BYTE, _: ::minwindef::BYTE, _: ::minwindef::DWORD, _: ::basetsd::ULONG_PTR); /* winuser.h:5751:1 */
    pub fn mouse_event(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::basetsd::ULONG_PTR); /* winuser.h:5785:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::wsprintfW as wsprintf; /* winuser.h:307:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::wvsprintfW as wvsprintf; /* winuser.h:287:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn wvsprintfA(_: ::winnt::LPSTR, _: ::winnt::LPCSTR, _: ::libc::c_int) -> ::libc::c_int; /* winuser.h:275:1 */
    pub fn wvsprintfW(_: ::winnt::LPWSTR, _: ::winnt::LPCWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winuser.h:282:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn ActivateKeyboardLayout(_: ::minwindef::HKL, _: ::minwindef::UINT) -> ::minwindef::HKL; /* winuser.h:1173:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::BroadcastSystemMessageW as BroadcastSystemMessage; /* winuser.h:3735:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn BroadcastSystemMessageA(_: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::libc::c_long; /* winuser.h:3719:1 */
    pub fn BroadcastSystemMessageW(_: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM) -> ::libc::c_long; /* winuser.h:3728:1 */
    pub fn CascadeWindows(_: ::windef::HWND, _: ::minwindef::UINT, _: *const ::windef::RECT, _: ::minwindef::UINT, _: *const *mut ::windef::HWND__) -> ::minwindef::WORD; /* winuser.h:11585:8 */
    pub fn CharNextExA(_: ::minwindef::WORD, _: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::winnt::LPSTR; /* winuser.h:5485:1 */
    pub fn CharPrevExA(_: ::minwindef::WORD, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::winnt::LPSTR; /* winuser.h:5493:1 */
    pub fn CheckMenuRadioItem(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:9908:1 */
    pub fn ChildWindowFromPointEx(_: ::windef::HWND, _: ::windef::POINT, _: ::minwindef::UINT) -> ::windef::HWND; /* winuser.h:9042:1 */
    pub fn CopyImage(_: ::winnt::HANDLE, _: ::minwindef::UINT, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT) -> ::winnt::HANDLE; /* winuser.h:10287:1 */
    pub fn CreateIconFromResourceEx(_: ::minwindef::PBYTE, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::DWORD, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT) -> ::windef::HICON; /* winuser.h:10210:1 */
    pub fn DragDetect(_: ::windef::HWND, _: ::windef::POINT) -> ::minwindef::BOOL; /* winuser.h:7651:1 */
    pub fn DragObject(_: ::windef::HWND, _: ::windef::HWND, _: ::minwindef::UINT, _: ::basetsd::ULONG_PTR, _: ::windef::HCURSOR) -> ::minwindef::DWORD; /* winuser.h:7641:1 */
    pub fn DrawAnimatedRects(_: ::windef::HWND, _: ::libc::c_int, _: *const ::windef::RECT, _: *const ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:2981:1 */
    pub fn DrawCaption(_: ::windef::HWND, _: ::windef::HDC, _: *const ::windef::RECT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:2962:1 */
    pub fn DrawEdge(_: ::windef::HDC, _: ::windef::LPRECT, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:2869:1 */
    pub fn DrawFrameControl(_: ::windef::HDC, _: ::windef::LPRECT, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:2932:1 */
    pub fn DrawIconEx(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HICON, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT, _: ::windef::HBRUSH, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:10303:24 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::DrawStateW as DrawState; /* winuser.h:7928:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn DrawStateA(_: ::windef::HDC, _: ::windef::HBRUSH, _: ::winuser::DRAWSTATEPROC, _: ::minwindef::LPARAM, _: ::minwindef::WPARAM, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7902:1 */
    pub fn DrawStateW(_: ::windef::HDC, _: ::windef::HBRUSH, _: ::winuser::DRAWSTATEPROC, _: ::minwindef::LPARAM, _: ::minwindef::WPARAM, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7916:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::DrawTextExW as DrawTextEx; /* winuser.h:7827:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn DrawTextExA(_: ::windef::HDC, _: ::winnt::LPSTR, _: ::libc::c_int, _: ::windef::LPRECT, _: ::minwindef::UINT, _: ::winuser::LPDRAWTEXTPARAMS) -> ::libc::c_int; /* winuser.h:7802:1 */
    pub fn DrawTextExW(_: ::windef::HDC, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::windef::LPRECT, _: ::minwindef::UINT, _: ::winuser::LPDRAWTEXTPARAMS) -> ::libc::c_int; /* winuser.h:7816:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::FindWindowExW as FindWindowEx; /* winuser.h:9572:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn FindWindowExA(_: ::windef::HWND, _: ::windef::HWND, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::windef::HWND; /* winuser.h:9558:1 */
    pub fn FindWindowExW(_: ::windef::HWND, _: ::windef::HWND, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::windef::HWND; /* winuser.h:9566:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::GetClassInfoExW as GetClassInfoEx; /* winuser.h:4210:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn GetClassInfoExA(_: ::minwindef::HINSTANCE, _: ::winnt::LPCSTR, _: ::winuser::LPWNDCLASSEXA) -> ::minwindef::BOOL; /* winuser.h:4197:1 */
    pub fn GetClassInfoExW(_: ::minwindef::HINSTANCE, _: ::winnt::LPCWSTR, _: ::winuser::LPWNDCLASSEXW) -> ::minwindef::BOOL; /* winuser.h:4205:1 */
    pub fn GetKeyboardLayout(_: ::minwindef::DWORD) -> ::minwindef::HKL; /* winuser.h:1232:1 */
    pub fn GetKeyboardLayoutList(_: ::libc::c_int, _: *mut *mut ::minwindef::HKL__) -> ::libc::c_int; /* winuser.h:1225:1 */
    pub fn GetMenuContextHelpId(_: ::windef::HMENU) -> ::minwindef::DWORD; /* winuser.h:8604:1 */
    pub fn GetMenuDefaultItem(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:7530:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::GetMenuItemInfoW as GetMenuItemInfo; /* winuser.h:7496:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn GetMenuItemInfoA(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::BOOL, _: ::winuser::LPMENUITEMINFOA) -> ::minwindef::BOOL; /* winuser.h:7482:1 */
    pub fn GetMenuItemInfoW(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::BOOL, _: ::winuser::LPMENUITEMINFOW) -> ::minwindef::BOOL; /* winuser.h:7490:1 */
    pub fn GetMenuItemRect(_: ::windef::HWND, _: ::windef::HMENU, _: ::minwindef::UINT, _: ::windef::LPRECT) -> ::minwindef::BOOL; /* winuser.h:7546:1 */
    pub fn GetScrollInfo(_: ::windef::HWND, _: ::libc::c_int, _: ::winuser::LPSCROLLINFO) -> ::minwindef::BOOL; /* winuser.h:11397:1 */
    pub fn GetShellWindow() -> ::windef::HWND; /* winuser.h:9580:1 */
    pub fn GetSysColorBrush(_: ::libc::c_int) -> ::windef::HBRUSH; /* winuser.h:9128:1 */
    pub fn GetWindowContextHelpId(_: ::windef::HWND) -> ::minwindef::DWORD; /* winuser.h:8591:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::InsertMenuItemW as InsertMenuItem; /* winuser.h:7474:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn InsertMenuItemA(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::BOOL, _: ::winuser::LPCMENUITEMINFOA) -> ::minwindef::BOOL; /* winuser.h:7460:1 */
    pub fn InsertMenuItemW(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::BOOL, _: ::winuser::LPCMENUITEMINFOW) -> ::minwindef::BOOL; /* winuser.h:7468:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::LoadImageW as LoadImage; /* winuser.h:10279:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn LoadImageA(_: ::minwindef::HINSTANCE, _: ::winnt::LPCSTR, _: ::minwindef::UINT, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT) -> ::winnt::HANDLE; /* winuser.h:10261:1 */
    pub fn LoadImageW(_: ::minwindef::HINSTANCE, _: ::winnt::LPCWSTR, _: ::minwindef::UINT, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT) -> ::winnt::HANDLE; /* winuser.h:10271:1 */
    pub fn LookupIconIdFromDirectoryEx(_: ::minwindef::PBYTE, _: ::minwindef::BOOL, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT) -> ::libc::c_int; /* winuser.h:10189:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::MapVirtualKeyExW as MapVirtualKeyEx; /* winuser.h:6497:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn MapVirtualKeyExA(_: ::minwindef::UINT, _: ::minwindef::UINT, _: ::minwindef::HKL) -> ::minwindef::UINT; /* winuser.h:6485:1 */
    pub fn MapVirtualKeyExW(_: ::minwindef::UINT, _: ::minwindef::UINT, _: ::minwindef::HKL) -> ::minwindef::UINT; /* winuser.h:6492:1 */
    pub fn MenuItemFromPoint(_: ::windef::HWND, _: ::windef::HMENU, _: ::windef::POINT) -> ::libc::c_int; /* winuser.h:7555:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::MessageBoxIndirectW as MessageBoxIndirect; /* winuser.h:8807:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn MessageBoxIndirectA(_: *const ::winuser::MSGBOXPARAMSA) -> ::libc::c_int; /* winuser.h:8799:1 */
    pub fn MessageBoxIndirectW(_: *const ::winuser::MSGBOXPARAMSW) -> ::libc::c_int; /* winuser.h:8804:1 */
    pub fn PaintDesktop(_: ::windef::HDC) -> ::minwindef::BOOL; /* winuser.h:8018:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::RegisterClassExW as RegisterClassEx; /* winuser.h:4188:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn RegisterClassExA(_: *const ::winuser::WNDCLASSEXA) -> ::minwindef::ATOM; /* winuser.h:4180:1 */
    pub fn RegisterClassExW(_: *const ::winuser::WNDCLASSEXW) -> ::minwindef::ATOM; /* winuser.h:4185:1 */
    pub fn SetMenuContextHelpId(_: ::windef::HMENU, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:8597:1 */
    pub fn SetMenuDefaultItem(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:7538:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::SetMenuItemInfoW as SetMenuItemInfo; /* winuser.h:7518:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn SetMenuItemInfoA(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::BOOL, _: ::winuser::LPCMENUITEMINFOA) -> ::minwindef::BOOL; /* winuser.h:7504:1 */
    pub fn SetMenuItemInfoW(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::minwindef::BOOL, _: ::winuser::LPCMENUITEMINFOW) -> ::minwindef::BOOL; /* winuser.h:7512:1 */
    pub fn SetMessageExtraInfo(_: ::minwindef::LPARAM) -> ::minwindef::LPARAM; /* winuser.h:3544:1 */
    pub fn SetScrollInfo(_: ::windef::HWND, _: ::libc::c_int, _: ::winuser::LPCSCROLLINFO, _: ::minwindef::BOOL) -> ::libc::c_int; /* winuser.h:11388:1 */
    pub fn SetWindowContextHelpId(_: ::windef::HWND, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:8584:1 */
    pub fn ShowWindowAsync(_: ::windef::HWND, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:4462:1 */
    pub fn SwitchToThisWindow(_: ::windef::HWND, _: ::minwindef::BOOL); /* winuser.h:8024:1 */
    pub fn TileWindows(_: ::windef::HWND, _: ::minwindef::UINT, _: *const ::windef::RECT, _: ::minwindef::UINT, _: *const *mut ::windef::HWND__) -> ::minwindef::WORD; /* winuser.h:11576:1 */
    pub fn ToAsciiEx(_: ::minwindef::UINT, _: ::minwindef::UINT, _: *const ::libc::c_uchar, _: ::minwindef::LPWORD, _: ::minwindef::UINT, _: ::minwindef::HKL) -> ::libc::c_int; /* winuser.h:5680:1 */
    pub fn ToUnicodeEx(_: ::minwindef::UINT, _: ::minwindef::UINT, _: *const ::libc::c_uchar, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::minwindef::UINT, _: ::minwindef::HKL) -> ::libc::c_int; /* winuser.h:1189:1 */
    pub fn TrackMouseEvent(_: ::winuser::LPTRACKMOUSEEVENT) -> ::minwindef::BOOL; /* winuser.h:2658:1 */
    pub fn TrackPopupMenuEx(_: ::windef::HMENU, _: ::minwindef::UINT, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HWND, _: ::winuser::LPTPMPARAMS) -> ::minwindef::BOOL; /* winuser.h:7281:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::VkKeyScanExW as VkKeyScanEx; /* winuser.h:5736:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn VkKeyScanExA(_: ::winnt::CHAR, _: ::minwindef::HKL) -> ::winnt::SHORT; /* winuser.h:5726:1 */
    pub fn VkKeyScanExW(_: ::winnt::WCHAR, _: ::minwindef::HKL) -> ::winnt::SHORT; /* winuser.h:5732:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn AllowSetForegroundWindow(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:8040:1 */
    pub fn AnimateWindow(_: ::windef::HWND, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:4330:1 */
    pub fn BlockInput(_: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:13706:1 */
    pub fn EndMenu() -> ::minwindef::BOOL; /* winuser.h:7348:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumDisplayDevicesW as EnumDisplayDevices; /* winuser.h:12475:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumDisplayDevicesA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::wingdi::PDISPLAY_DEVICEA, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:12461:1 */
    pub fn EnumDisplayDevicesW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::wingdi::PDISPLAY_DEVICEW, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:12469:1 */
    pub fn EnumDisplayMonitors(_: ::windef::HDC, _: ::windef::LPCRECT, _: ::winuser::MONITORENUMPROC, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winuser.h:12976:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumDisplaySettingsExW as EnumDisplaySettingsEx; /* winuser.h:12449:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumDisplaySettingsExA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: *mut ::wingdi::DEVMODEA, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:12435:1 */
    pub fn EnumDisplaySettingsExW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: *mut ::wingdi::DEVMODEW, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:12443:1 */
    pub fn FlashWindowEx(_: ::winuser::PFLASHWINFO) -> ::minwindef::BOOL; /* winuser.h:4486:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::GetAltTabInfoW as GetAltTabInfo; /* winuser.h:14029:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn GetAltTabInfoA(_: ::windef::HWND, _: ::libc::c_int, _: ::winuser::PALTTABINFO, _: ::winnt::LPSTR, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:14013:1 */
    pub fn GetAltTabInfoW(_: ::windef::HWND, _: ::libc::c_int, _: ::winuser::PALTTABINFO, _: ::winnt::LPWSTR, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:14022:1 */
    pub fn GetAncestor(_: ::windef::HWND, _: ::minwindef::UINT) -> ::windef::HWND; /* winuser.h:13947:1 */
    pub fn GetClipboardSequenceNumber() -> ::minwindef::DWORD; /* winuser.h:5157:1 */
    pub fn GetComboBoxInfo(_: ::windef::HWND, _: ::winuser::PCOMBOBOXINFO) -> ::minwindef::BOOL; /* winuser.h:13927:1 */
    pub fn GetCursorInfo(_: ::winuser::PCURSORINFO) -> ::minwindef::BOOL; /* winuser.h:13810:1 */
    pub fn GetGUIThreadInfo(_: ::minwindef::DWORD, _: ::winuser::PGUITHREADINFO) -> ::minwindef::BOOL; /* winuser.h:13699:1 */
    pub fn GetGuiResources(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:11753:1 */
    pub fn GetLastInputInfo(_: ::winuser::PLASTINPUTINFO) -> ::minwindef::BOOL; /* winuser.h:6459:1 */
    pub fn GetListBoxInfo(_: ::windef::HWND) -> ::minwindef::DWORD; /* winuser.h:14041:1 */
    pub fn GetMenuBarInfo(_: ::windef::HWND, _: ::winnt::LONG, _: ::winnt::LONG, _: ::winuser::PMENUBARINFO) -> ::minwindef::BOOL; /* winuser.h:13882:1 */
    pub fn GetMenuInfo(_: ::windef::HMENU, _: ::winuser::LPMENUINFO) -> ::minwindef::BOOL; /* winuser.h:7334:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::GetMonitorInfoW as GetMonitorInfo; /* winuser.h:12966:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn GetMonitorInfoA(_: ::windef::HMONITOR, _: ::winuser::LPMONITORINFO) -> ::minwindef::BOOL; /* winuser.h:12956:1 */
    pub fn GetMonitorInfoW(_: ::windef::HMONITOR, _: ::winuser::LPMONITORINFO) -> ::minwindef::BOOL; /* winuser.h:12962:1 */
    pub fn GetMouseMovePointsEx(_: ::minwindef::UINT, _: ::winuser::LPMOUSEMOVEPOINT, _: ::winuser::LPMOUSEMOVEPOINT, _: ::libc::c_int, _: ::minwindef::DWORD) -> ::libc::c_int; /* winuser.h:1268:1 */
    pub fn GetProcessDefaultLayout(_: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winuser.h:9497:1 */
    pub fn GetScrollBarInfo(_: ::windef::HWND, _: ::winnt::LONG, _: ::winuser::PSCROLLBARINFO) -> ::minwindef::BOOL; /* winuser.h:13905:1 */
    pub fn GetTitleBarInfo(_: ::windef::HWND, _: ::winuser::PTITLEBARINFO) -> ::minwindef::BOOL; /* winuser.h:13852:1 */
    pub fn GetWindowInfo(_: ::windef::HWND, _: ::winuser::PWINDOWINFO) -> ::minwindef::BOOL; /* winuser.h:13835:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::GetWindowModuleFileNameW as GetWindowModuleFileName; /* winuser.h:13742:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn GetWindowModuleFileNameA(_: ::windef::HWND, _: ::winnt::LPSTR, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:13730:1 */
    pub fn GetWindowModuleFileNameW(_: ::windef::HWND, _: ::winnt::LPWSTR, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:13737:1 */
    pub fn InSendMessageEx(_: ::minwindef::LPVOID) -> ::minwindef::DWORD; /* winuser.h:4088:1 */
    pub fn LockSetForegroundWindow(_: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:8048:1 */
    pub fn LockWorkStation() -> ::minwindef::BOOL; /* winuser.h:14058:1 */
    pub fn MonitorFromPoint(_: ::windef::POINT, _: ::minwindef::DWORD) -> ::windef::HMONITOR; /* winuser.h:12879:1 */
    pub fn MonitorFromRect(_: ::windef::LPCRECT, _: ::minwindef::DWORD) -> ::windef::HMONITOR; /* winuser.h:12886:1 */
    pub fn MonitorFromWindow(_: ::windef::HWND, _: ::minwindef::DWORD) -> ::windef::HMONITOR; /* winuser.h:12893:1 */
    pub fn NotifyWinEvent(_: ::minwindef::DWORD, _: ::windef::HWND, _: ::winnt::LONG, _: ::winnt::LONG); /* winuser.h:12997:1 */
    pub fn RealChildWindowFromPoint(_: ::windef::HWND, _: ::windef::POINT) -> ::windef::HWND; /* winuser.h:13961:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::RealGetWindowClassW as RealGetWindowClass; /* winuser.h:13989:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn RealGetWindowClassA(_: ::windef::HWND, _: ::winnt::LPSTR, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:13973:1 */
    pub fn RealGetWindowClassW(_: ::windef::HWND, _: ::winnt::LPWSTR, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:13984:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::RegisterDeviceNotificationW as RegisterDeviceNotification; /* winuser.h:3814:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn RegisterDeviceNotificationA(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::winuser::HDEVNOTIFY; /* winuser.h:3802:1 */
    pub fn RegisterDeviceNotificationW(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::winuser::HDEVNOTIFY; /* winuser.h:3809:1 */
    pub fn SendInput(_: ::minwindef::UINT, _: ::winuser::LPINPUT, _: ::libc::c_int) -> ::minwindef::UINT; /* winuser.h:5841:1 */
    pub fn SetLayeredWindowAttributes(_: ::windef::HWND, _: ::windef::COLORREF, _: ::minwindef::BYTE, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:4432:1 */
    pub fn SetMenuInfo(_: ::windef::HMENU, _: ::winuser::LPCMENUINFO) -> ::minwindef::BOOL; /* winuser.h:7341:1 */
    pub fn SetProcessDefaultLayout(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:9503:1 */
    pub fn SetWinEventHook(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::HMODULE, _: ::winuser::WINEVENTPROC, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::windef::HWINEVENTHOOK; /* winuser.h:13015:1 */
    pub fn UnhookWinEvent(_: ::windef::HWINEVENTHOOK) -> ::minwindef::BOOL; /* winuser.h:13049:1 */
    pub fn UnregisterDeviceNotification(_: ::winuser::HDEVNOTIFY) -> ::minwindef::BOOL; /* winuser.h:3822:1 */
    pub fn UpdateLayeredWindow(_: ::windef::HWND, _: ::windef::HDC, _: *mut ::windef::POINT, _: *mut ::windef::SIZE, _: ::windef::HDC, _: *mut ::windef::POINT, _: ::windef::COLORREF, _: *mut ::wingdi::BLENDFUNCTION, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:4348:1 */
    pub fn UpdateLayeredWindowIndirect(_: ::windef::HWND, _: *const ::winuser::UPDATELAYEREDWINDOWINFO) -> ::minwindef::BOOL; /* winuser.h:4383:1 */
    pub fn UserHandleGrantAccess(_: ::winnt::HANDLE, _: ::winnt::HANDLE, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:14067:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::BroadcastSystemMessageExW as BroadcastSystemMessageEx; /* winuser.h:3701:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn BroadcastSystemMessageExA(_: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM, _: ::winuser::PBSMINFO) -> ::libc::c_long; /* winuser.h:3683:1 */
    pub fn BroadcastSystemMessageExW(_: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::UINT, _: ::minwindef::WPARAM, _: ::minwindef::LPARAM, _: ::winuser::PBSMINFO) -> ::libc::c_long; /* winuser.h:3693:1 */
    pub fn DefRawInputProc(_: *mut *mut ::winuser::RAWINPUT, _: ::winnt::INT, _: ::minwindef::UINT) -> ::minwindef::LRESULT; /* winuser.h:14506:1 */
    pub fn DisableProcessWindowsGhosting(); /* winuser.h:1765:1 */
    pub fn GetLayeredWindowAttributes(_: ::windef::HWND, _: *mut ::libc::c_ulong, _: *mut ::libc::c_uchar, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winuser.h:4400:1 */
    pub fn GetRawInputBuffer(_: ::winuser::PRAWINPUT, _: ::minwindef::PUINT, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:14420:1 */
    pub fn GetRawInputData(_: ::winuser::HRAWINPUT, _: ::minwindef::UINT, _: ::minwindef::LPVOID, _: ::minwindef::PUINT, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:14333:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetRawInputDeviceInfoW as GetRawInputDeviceInfo; /* winuser.h:14408:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetRawInputDeviceInfoA(_: ::winnt::HANDLE, _: ::minwindef::UINT, _: ::minwindef::LPVOID, _: ::minwindef::PUINT) -> ::minwindef::UINT; /* winuser.h:14394:1 */
    pub fn GetRawInputDeviceInfoW(_: ::winnt::HANDLE, _: ::minwindef::UINT, _: ::minwindef::LPVOID, _: ::minwindef::PUINT) -> ::minwindef::UINT; /* winuser.h:14402:1 */
    pub fn GetRawInputDeviceList(_: ::winuser::PRAWINPUTDEVICELIST, _: ::minwindef::PUINT, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:14498:1 */
    pub fn GetRegisteredRawInputDevices(_: ::winuser::PRAWINPUTDEVICE, _: ::minwindef::PUINT, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winuser.h:14484:1 */
    pub fn GetWindowRgnBox(_: ::windef::HWND, _: ::windef::LPRECT) -> ::libc::c_int; /* winuser.h:8169:1 */
    pub fn IsGUIThread(_: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:9683:1 */
    pub fn IsWinEventHookInstalled(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:13028:1 */
    pub fn IsWow64Message() -> ::minwindef::BOOL; /* winuser.h:3536:1 */
    pub fn PrintWindow(_: ::windef::HWND, _: ::windef::HDC, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:4416:1 */
    pub fn RegisterRawInputDevices(_: ::winuser::PCRAWINPUTDEVICE, _: ::minwindef::UINT, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:14476:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] 
extern "system" {
    pub fn RegisterPowerSettingNotification(_: ::winnt::HANDLE, _: ::guiddef::LPCGUID, _: ::minwindef::DWORD) -> ::winuser::HPOWERNOTIFY; /* winuser.h:3840:1 */
    pub fn RegisterSuspendResumeNotification(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::winuser::HPOWERNOTIFY; /* winuser.h:3856:1 */
    pub fn UnregisterPowerSettingNotification(_: ::winuser::HPOWERNOTIFY) -> ::minwindef::BOOL; /* winuser.h:3849:1 */
    pub fn UnregisterSuspendResumeNotification(_: ::winuser::HPOWERNOTIFY) -> ::minwindef::BOOL; /* winuser.h:3864:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn AddClipboardFormatListener(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:5277:1 */
    pub fn ChangeWindowMessageFilter(_: ::minwindef::UINT, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:14659:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetIconInfoExW as GetIconInfoEx; /* winuser.h:10385:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetIconInfoExA(_: ::windef::HICON, _: ::winuser::PICONINFOEXA) -> ::minwindef::BOOL; /* winuser.h:10375:1 */
    pub fn GetIconInfoExW(_: ::windef::HICON, _: ::winuser::PICONINFOEXW) -> ::minwindef::BOOL; /* winuser.h:10381:1 */
    pub fn GetPhysicalCursorPos(_: ::windef::LPPOINT) -> ::minwindef::BOOL; /* winuser.h:8874:1 */
    pub fn GetUpdatedClipboardFormats(_: ::minwindef::PUINT, _: ::minwindef::UINT, _: ::minwindef::PUINT) -> ::minwindef::BOOL; /* winuser.h:5289:1 */
    pub fn IsProcessDPIAware() -> ::minwindef::BOOL; /* winuser.h:13722:1 */
    pub fn LogicalToPhysicalPoint(_: ::windef::HWND, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* winuser.h:8967:1 */
    pub fn PhysicalToLogicalPoint(_: ::windef::HWND, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* winuser.h:8974:1 */
    pub fn RemoveClipboardFormatListener(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:5283:1 */
    pub fn SetPhysicalCursorPos(_: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:8853:1 */
    pub fn SetProcessDPIAware() -> ::minwindef::BOOL; /* winuser.h:13716:1 */
    pub fn SoundSentry() -> ::minwindef::BOOL; /* winuser.h:12769:1 */
    pub fn WindowFromPhysicalPoint(_: ::windef::POINT) -> ::windef::HWND; /* winuser.h:9016:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn CalculatePopupWindowPosition(_: *const ::windef::POINT, _: *const ::windef::SIZE, _: ::minwindef::UINT, _: *mut ::windef::RECT, _: *mut ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:7294:1 */
    pub fn ChangeWindowMessageFilterEx(_: ::windef::HWND, _: ::minwindef::UINT, _: ::minwindef::DWORD, _: ::winuser::PCHANGEFILTERSTRUCT) -> ::minwindef::BOOL; /* winuser.h:14702:1 */
    pub fn CloseGestureInfoHandle(_: ::winuser::HGESTUREINFO) -> ::minwindef::BOOL; /* winuser.h:14838:1 */
    pub fn CloseTouchInputHandle(_: ::winuser::HTOUCHINPUT) -> ::minwindef::BOOL; /* winuser.h:5922:1 */
    pub fn DisplayConfigGetDeviceInfo(_: *mut ::wingdi::DISPLAYCONFIG_DEVICE_INFO_HEADER) -> ::winnt::LONG; /* winuser.h:12521:1 */
    pub fn DisplayConfigSetDeviceInfo(_: *mut ::wingdi::DISPLAYCONFIG_DEVICE_INFO_HEADER) -> ::winnt::LONG; /* winuser.h:12527:1 */
    pub fn GetAutoRotationState(_: ::winuser::PAR_STATE) -> ::minwindef::BOOL; /* winuser.h:15093:1 */
    pub fn GetCIMSSM(_: *mut ::winuser::INPUT_MESSAGE_SOURCE) -> ::minwindef::BOOL; /* winuser.h:15030:1 */
    pub fn GetCurrentInputMessageSource(_: *mut ::winuser::INPUT_MESSAGE_SOURCE) -> ::minwindef::BOOL; /* winuser.h:15024:1 */
    pub fn GetDisplayAutoRotationPreferences(_: *mut ::winuser::ORIENTATION_PREFERENCE) -> ::minwindef::BOOL; /* winuser.h:15099:1 */
    pub fn GetDisplayConfigBufferSizes(_: ::basetsd::UINT32, _: *mut ::libc::c_uint, _: *mut ::libc::c_uint) -> ::winnt::LONG; /* winuser.h:12490:1 */
    pub fn GetGestureConfig(_: ::windef::HWND, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::PUINT, _: ::winuser::PGESTURECONFIG, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:14921:1 */
    pub fn GetGestureExtraArgs(_: ::winuser::HGESTUREINFO, _: ::minwindef::UINT, _: ::minwindef::PBYTE) -> ::minwindef::BOOL; /* winuser.h:14820:1 */
    pub fn GetGestureInfo(_: ::winuser::HGESTUREINFO, _: ::winuser::PGESTUREINFO) -> ::minwindef::BOOL; /* winuser.h:14807:1 */
    pub fn GetTouchInputInfo(_: ::winuser::HTOUCHINPUT, _: ::minwindef::UINT, _: ::winuser::PTOUCHINPUT, _: ::libc::c_int) -> ::minwindef::BOOL; /* winuser.h:5913:1 */
    pub fn GetWindowDisplayAffinity(_: ::windef::HWND, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winuser.h:4562:1 */
    pub fn IsImmersiveProcess(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winuser.h:15130:1 */
    pub fn IsTouchWindow(_: ::windef::HWND, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winuser.h:5962:1 */
    pub fn QueryDisplayConfig(_: ::basetsd::UINT32, _: *mut ::libc::c_uint, _: *mut ::wingdi::DISPLAYCONFIG_PATH_INFO, _: *mut ::libc::c_uint, _: *mut ::wingdi::DISPLAYCONFIG_MODE_INFO, _: *mut ::wingdi::DISPLAYCONFIG_TOPOLOGY_ID) -> ::winnt::LONG; /* winuser.h:12508:1 */
    pub fn RegisterTouchWindow(_: ::windef::HWND, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winuser.h:5949:1 */
    pub fn SetCoalescableTimer(_: ::windef::HWND, _: ::basetsd::UINT_PTR, _: ::minwindef::UINT, _: ::winuser::TIMERPROC, _: ::minwindef::ULONG) -> ::basetsd::UINT_PTR; /* winuser.h:6659:1 */
    pub fn SetDisplayAutoRotationPreferences(_: ::winuser::ORIENTATION_PREFERENCE) -> ::minwindef::BOOL; /* winuser.h:15113:1 */
    pub fn SetDisplayConfig(_: ::basetsd::UINT32, _: *mut ::wingdi::DISPLAYCONFIG_PATH_INFO, _: ::basetsd::UINT32, _: *mut ::wingdi::DISPLAYCONFIG_MODE_INFO, _: ::basetsd::UINT32) -> ::winnt::LONG; /* winuser.h:12498:1 */
    pub fn SetGestureConfig(_: ::windef::HWND, _: ::minwindef::DWORD, _: ::minwindef::UINT, _: ::winuser::PGESTURECONFIG, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winuser.h:14906:1 */
    pub fn SetProcessRestrictionExemption(_: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:15136:1 */
    pub fn SetWindowDisplayAffinity(_: ::windef::HWND, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:4569:1 */
    pub fn UnregisterTouchWindow(_: ::windef::HWND) -> ::minwindef::BOOL; /* winuser.h:5956:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn EnableMouseInPointer(_: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:6297:1 */
    pub fn EvaluateProximityToPolygon(_: ::basetsd::UINT32, _: *const ::windef::POINT, _: *const ::winuser::TOUCH_HIT_TESTING_INPUT, _: *mut ::winuser::TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> ::minwindef::BOOL; /* winuser.h:6352:1 */
    pub fn EvaluateProximityToRect(_: *const ::windef::RECT, _: *const ::winuser::TOUCH_HIT_TESTING_INPUT, _: *mut ::winuser::TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> ::minwindef::BOOL; /* winuser.h:6344:1 */
    pub fn GetPointerCursorId(_: ::basetsd::UINT32, _: *mut ::libc::c_uint) -> ::minwindef::BOOL; /* winuser.h:6173:1 */
    pub fn GetPointerDevice(_: ::winnt::HANDLE, _: *mut ::winuser::POINTER_DEVICE_INFO) -> ::minwindef::BOOL; /* winuser.h:14592:1 */
    pub fn GetPointerDeviceCursors(_: ::winnt::HANDLE, _: *mut ::libc::c_uint, _: *mut ::winuser::POINTER_DEVICE_CURSOR_INFO) -> ::minwindef::BOOL; /* winuser.h:14622:1 */
    pub fn GetPointerDeviceProperties(_: ::winnt::HANDLE, _: *mut ::libc::c_uint, _: *mut ::winuser::POINTER_DEVICE_PROPERTY) -> ::minwindef::BOOL; /* winuser.h:14599:1 */
    pub fn GetPointerDeviceRects(_: ::winnt::HANDLE, _: *mut ::windef::RECT, _: *mut ::windef::RECT) -> ::minwindef::BOOL; /* winuser.h:14614:1 */
    pub fn GetPointerDevices(_: *mut ::libc::c_uint, _: *mut ::winuser::POINTER_DEVICE_INFO) -> ::minwindef::BOOL; /* winuser.h:14585:1 */
    pub fn GetPointerFrameInfo(_: ::basetsd::UINT32, _: *mut ::libc::c_uint, _: *mut ::winuser::POINTER_INFO) -> ::minwindef::BOOL; /* winuser.h:6195:1 */
    pub fn GetPointerFrameInfoHistory(_: ::basetsd::UINT32, _: *mut ::libc::c_uint, _: *mut ::libc::c_uint, _: *mut ::winuser::POINTER_INFO) -> ::minwindef::BOOL; /* winuser.h:6203:1 */
    pub fn GetPointerFramePenInfo(_: ::basetsd::UINT32, _: *mut ::libc::c_uint, _: *mut ::winuser::POINTER_PEN_INFO) -> ::minwindef::BOOL; /* winuser.h:6259:1 */
    pub fn GetPointerFramePenInfoHistory(_: ::basetsd::UINT32, _: *mut ::libc::c_uint, _: *mut ::libc::c_uint, _: *mut ::winuser::POINTER_PEN_INFO) -> ::minwindef::BOOL; /* winuser.h:6267:1 */
    pub fn GetPointerFrameTouchInfo(_: ::basetsd::UINT32, _: *mut ::libc::c_uint, _: *mut ::winuser::POINTER_TOUCH_INFO) -> ::minwindef::BOOL; /* winuser.h:6227:1 */
    pub fn GetPointerFrameTouchInfoHistory(_: ::basetsd::UINT32, _: *mut ::libc::c_uint, _: *mut ::libc::c_uint, _: *mut ::winuser::POINTER_TOUCH_INFO) -> ::minwindef::BOOL; /* winuser.h:6235:1 */
    pub fn GetPointerInfo(_: ::basetsd::UINT32, _: *mut ::winuser::POINTER_INFO) -> ::minwindef::BOOL; /* winuser.h:6180:1 */
    pub fn GetPointerInfoHistory(_: ::basetsd::UINT32, _: *mut ::libc::c_uint, _: *mut ::winuser::POINTER_INFO) -> ::minwindef::BOOL; /* winuser.h:6187:1 */
    pub fn GetPointerPenInfo(_: ::basetsd::UINT32, _: *mut ::winuser::POINTER_PEN_INFO) -> ::minwindef::BOOL; /* winuser.h:6244:1 */
    pub fn GetPointerPenInfoHistory(_: ::basetsd::UINT32, _: *mut ::libc::c_uint, _: *mut ::winuser::POINTER_PEN_INFO) -> ::minwindef::BOOL; /* winuser.h:6251:1 */
    pub fn GetPointerTouchInfo(_: ::basetsd::UINT32, _: *mut ::winuser::POINTER_TOUCH_INFO) -> ::minwindef::BOOL; /* winuser.h:6212:1 */
    pub fn GetPointerTouchInfoHistory(_: ::basetsd::UINT32, _: *mut ::libc::c_uint, _: *mut ::winuser::POINTER_TOUCH_INFO) -> ::minwindef::BOOL; /* winuser.h:6219:1 */
    pub fn GetPointerType(_: ::basetsd::UINT32, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winuser.h:6166:1 */
    pub fn GetRawPointerDeviceData(_: ::basetsd::UINT32, _: ::basetsd::UINT32, _: ::basetsd::UINT32, _: *mut ::winuser::POINTER_DEVICE_PROPERTY, _: *mut ::libc::c_long) -> ::minwindef::BOOL; /* winuser.h:14630:1 */
    pub fn GetUnpredictedMessagePos() -> ::minwindef::DWORD; /* winuser.h:3528:1 */
    pub fn GetWindowFeedbackSetting(_: ::windef::HWND, _: ::winuser::FEEDBACK_TYPE, _: ::minwindef::DWORD, _: *mut ::libc::c_uint, _: *mut ::libc::c_void) -> ::minwindef::BOOL; /* winuser.h:6387:1 */
    pub fn InitializeTouchInjection(_: ::basetsd::UINT32, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winuser.h:6152:1 */
    pub fn InjectTouchInput(_: ::basetsd::UINT32, _: *const ::winuser::POINTER_TOUCH_INFO) -> ::minwindef::BOOL; /* winuser.h:6159:1 */
    pub fn IsMouseInPointerEnabled() -> ::minwindef::BOOL; /* winuser.h:6303:1 */
    pub fn PackTouchHitTestingProximityEvaluation(_: *const ::winuser::TOUCH_HIT_TESTING_INPUT, _: *const ::winuser::TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> ::minwindef::LRESULT; /* winuser.h:6361:1 */
    pub fn RegisterPointerDeviceNotifications(_: ::windef::HWND, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winuser.h:14607:1 */
    pub fn RegisterPointerInputTarget(_: ::windef::HWND, _: ::winuser::POINTER_INPUT_TYPE) -> ::minwindef::BOOL; /* winuser.h:6282:1 */
    pub fn RegisterTouchHitTestingWindow(_: ::windef::HWND, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winuser.h:6314:1 */
    pub fn SetWindowFeedbackSetting(_: ::windef::HWND, _: ::winuser::FEEDBACK_TYPE, _: ::minwindef::DWORD, _: ::basetsd::UINT32, _: *const ::libc::c_void) -> ::minwindef::BOOL; /* winuser.h:6397:1 */
    pub fn SkipPointerFrameMessages(_: ::basetsd::UINT32) -> ::minwindef::BOOL; /* winuser.h:6276:1 */
    pub fn UnregisterPointerInputTarget(_: ::windef::HWND, _: ::winuser::POINTER_INPUT_TYPE) -> ::minwindef::BOOL; /* winuser.h:6289:1 */
}
