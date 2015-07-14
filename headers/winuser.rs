#[cfg(feature="winapi_desktop")] pub type HDWP = ::winnt::HANDLE; /* winuser.h:62:16, winuser.h:62:16, winuser.h:62:16 */
#[cfg(feature="winapi_desktop")] pub type MENUTEMPLATEA = ::libc::c_void; /* winuser.h:63:14, winuser.h:63:14, winuser.h:63:14 */
#[cfg(feature="winapi_desktop")] pub type MENUTEMPLATEW = ::libc::c_void; /* winuser.h:64:14, winuser.h:64:14, winuser.h:64:14 */
#[cfg(feature="winapi_desktop")] pub type MENUTEMPLATE = ::winuser::MENUTEMPLATEW; /* winuser.h:66:23, winuser.h:66:23, winuser.h:66:23 */
#[cfg(feature="winapi_desktop")] pub type LPMENUTEMPLATEA = ::winnt::PVOID; /* winuser.h:70:15, winuser.h:70:15, winuser.h:70:15 */
#[cfg(feature="winapi_desktop")] pub type LPMENUTEMPLATEW = ::winnt::PVOID; /* winuser.h:71:15, winuser.h:71:15, winuser.h:71:15 */
#[cfg(feature="winapi_desktop")] pub type LPMENUTEMPLATE = ::winuser::LPMENUTEMPLATEW; /* winuser.h:73:25, winuser.h:73:25, winuser.h:73:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type WNDPROC = extern "system" fn(*mut ::windef::HWND__, ::libc::c_uint, ::libc::c_uint, ::libc::c_long) -> ::libc::c_long; /* winuser.h:78:28, winuser.h:78:28 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type DLGPROC = extern "system" fn(*mut ::windef::HWND__, ::libc::c_uint, ::libc::c_uint, ::libc::c_long) -> ::libc::c_int; /* winuser.h:88:28, winuser.h:88:28 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type TIMERPROC = extern "system" fn(*mut ::windef::HWND__, ::libc::c_uint, ::libc::c_uint, ::libc::c_ulong); /* winuser.h:96:25, winuser.h:96:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type GRAYSTRINGPROC = extern "system" fn(*mut ::windef::HDC__, ::libc::c_long, ::libc::c_int) -> ::libc::c_int; /* winuser.h:97:25, winuser.h:97:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type WNDENUMPROC = extern "system" fn(*mut ::windef::HWND__, ::libc::c_long) -> ::libc::c_int; /* winuser.h:98:25, winuser.h:98:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type HOOKPROC = extern "system" fn(::libc::c_int, ::libc::c_uint, ::libc::c_long) -> ::libc::c_long; /* winuser.h:99:28, winuser.h:99:28 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type SENDASYNCPROC = extern "system" fn(*mut ::windef::HWND__, ::libc::c_uint, ::libc::c_ulong, ::libc::c_long); /* winuser.h:100:25, winuser.h:100:25 */
#[cfg(feature="winapi_desktop")] pub type PROPENUMPROCA = extern "system" fn(*mut ::windef::HWND__, *const ::libc::c_schar, *mut ::libc::c_void) -> ::libc::c_int; /* winuser.h:102:25, winuser.h:102:25, winuser.h:102:25 */
#[cfg(feature="winapi_desktop")] pub type PROPENUMPROCW = extern "system" fn(*mut ::windef::HWND__, *const ::libc::c_ushort, *mut ::libc::c_void) -> ::libc::c_int; /* winuser.h:103:25, winuser.h:103:25, winuser.h:103:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type PROPENUMPROCEXA = extern "system" fn(*mut ::windef::HWND__, *mut ::libc::c_schar, *mut ::libc::c_void, ::libc::c_ulong) -> ::libc::c_int; /* winuser.h:105:25, winuser.h:105:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type PROPENUMPROCEXW = extern "system" fn(*mut ::windef::HWND__, *mut ::libc::c_ushort, *mut ::libc::c_void, ::libc::c_ulong) -> ::libc::c_int; /* winuser.h:106:25, winuser.h:106:25 */
#[cfg(feature="winapi_desktop")] pub type EDITWORDBREAKPROCA = extern "system" fn(*mut ::libc::c_schar, ::libc::c_int, ::libc::c_int, ::libc::c_int) -> ::libc::c_int; /* winuser.h:108:24, winuser.h:108:24, winuser.h:108:24 */
#[cfg(feature="winapi_desktop")] pub type EDITWORDBREAKPROCW = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_int, ::libc::c_int, ::libc::c_int) -> ::libc::c_int; /* winuser.h:109:24, winuser.h:109:24, winuser.h:109:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type DRAWSTATEPROC = extern "system" fn(*mut ::windef::HDC__, ::libc::c_long, ::libc::c_uint, ::libc::c_int, ::libc::c_int) -> ::libc::c_int; /* winuser.h:112:25, winuser.h:112:25 */
#[cfg(feature="winapi_desktop")] pub type PROPENUMPROC = ::winuser::PROPENUMPROCW; /* winuser.h:161:30, winuser.h:161:30, winuser.h:161:30 */
#[cfg(feature="winapi_desktop")] pub type PROPENUMPROCEX = ::winuser::PROPENUMPROCEXW; /* winuser.h:162:30, winuser.h:162:30, winuser.h:162:30 */
#[cfg(feature="winapi_desktop")] pub type EDITWORDBREAKPROC = ::winuser::EDITWORDBREAKPROCW; /* winuser.h:163:30, winuser.h:163:30, winuser.h:163:30 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type NAMEENUMPROCA = extern "system" fn(*mut ::libc::c_schar, ::libc::c_long) -> ::libc::c_int; /* winuser.h:172:25, winuser.h:172:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type NAMEENUMPROCW = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_long) -> ::libc::c_int; /* winuser.h:173:25, winuser.h:173:25 */
#[cfg(feature="winapi_desktop")] pub type WINSTAENUMPROCA = ::winuser::NAMEENUMPROCA; /* winuser.h:175:25, winuser.h:175:25, winuser.h:175:25 */
#[cfg(feature="winapi_desktop")] pub type DESKTOPENUMPROCA = ::winuser::NAMEENUMPROCA; /* winuser.h:176:25, winuser.h:176:25, winuser.h:176:25 */
#[cfg(feature="winapi_desktop")] pub type WINSTAENUMPROCW = ::winuser::NAMEENUMPROCW; /* winuser.h:177:25, winuser.h:177:25, winuser.h:177:25 */
#[cfg(feature="winapi_desktop")] pub type DESKTOPENUMPROCW = ::winuser::NAMEENUMPROCW; /* winuser.h:178:25, winuser.h:178:25, winuser.h:178:25 */
#[cfg(feature="winapi_desktop")] pub type WINSTAENUMPROC = ::winuser::WINSTAENUMPROCW; /* winuser.h:194:29, winuser.h:194:29, winuser.h:194:29 */
#[cfg(feature="winapi_desktop")] pub type DESKTOPENUMPROC = ::winuser::DESKTOPENUMPROCW; /* winuser.h:195:29, winuser.h:195:29, winuser.h:195:29 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct CBT_CREATEWNDA { lpcs: *mut ::winuser::CREATESTRUCTA, hwndInsertAfter: ::windef::HWND } /* winuser.h:772:16, winuser.h:772:16, winuser.h:772:16 */
#[cfg(feature="winapi_desktop")] pub type LPCBT_CREATEWNDA = *mut ::winuser::CBT_CREATEWNDA; /* winuser.h:776:20, winuser.h:776:20, winuser.h:776:20 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct CBT_CREATEWNDW { lpcs: *mut ::winuser::CREATESTRUCTW, hwndInsertAfter: ::windef::HWND } /* winuser.h:780:16, winuser.h:780:16, winuser.h:780:16 */
#[cfg(feature="winapi_desktop")] pub type LPCBT_CREATEWNDW = *mut ::winuser::CBT_CREATEWNDW; /* winuser.h:784:20, winuser.h:784:20, winuser.h:784:20 */
#[cfg(feature="winapi_desktop")] pub type CBT_CREATEWND = ::winuser::CBT_CREATEWNDW; /* winuser.h:786:24, winuser.h:786:24, winuser.h:786:24 */
#[cfg(feature="winapi_desktop")] pub type LPCBT_CREATEWND = ::winuser::LPCBT_CREATEWNDW; /* winuser.h:787:26, winuser.h:787:26, winuser.h:787:26 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct CBTACTIVATESTRUCT { fMouse: ::minwindef::BOOL, hWndActive: ::windef::HWND } /* winuser.h:796:16, winuser.h:796:16, winuser.h:796:16 */
#[cfg(feature="winapi_desktop")] pub type LPCBTACTIVATESTRUCT = *mut ::winuser::CBTACTIVATESTRUCT; /* winuser.h:800:23, winuser.h:800:23, winuser.h:800:23 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct WTSSESSION_NOTIFICATION { cbSize: ::minwindef::DWORD, dwSessionId: ::minwindef::DWORD } /* winuser.h:813:16, winuser.h:813:16, winuser.h:813:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PWTSSESSION_NOTIFICATION = *mut ::winuser::WTSSESSION_NOTIFICATION; /* winuser.h:818:29, winuser.h:818:29, winuser.h:818:29 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SHELLHOOKINFO { hwnd: ::windef::HWND, rc: ::windef::RECT } /* winuser.h:964:9, winuser.h:964:9, winuser.h:964:9 */
#[cfg(feature="winapi_desktop")] pub type LPSHELLHOOKINFO = *mut ::winuser::SHELLHOOKINFO; /* winuser.h:968:19, winuser.h:968:19, winuser.h:968:19 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EVENTMSG { message: ::minwindef::UINT, paramL: ::minwindef::UINT, paramH: ::minwindef::UINT, time: ::minwindef::DWORD, hwnd: ::windef::HWND } /* winuser.h:973:16, winuser.h:973:16, winuser.h:973:16 */
#[cfg(feature="winapi_desktop")] pub type PEVENTMSGMSG = *mut ::winuser::EVENTMSG; /* winuser.h:979:14, winuser.h:979:14, winuser.h:979:14 */
#[cfg(feature="winapi_desktop")] pub type NPEVENTMSGMSG = *mut ::winuser::EVENTMSG; /* winuser.h:979:34, winuser.h:979:34, winuser.h:979:34 */
#[cfg(feature="winapi_desktop")] pub type LPEVENTMSGMSG = *mut ::winuser::EVENTMSG; /* winuser.h:979:54, winuser.h:979:54, winuser.h:979:54 */
#[cfg(feature="winapi_desktop")] pub type PEVENTMSG = *mut ::winuser::EVENTMSG; /* winuser.h:981:29, winuser.h:981:29, winuser.h:981:29 */
#[cfg(feature="winapi_desktop")] pub type NPEVENTMSG = *mut ::winuser::EVENTMSG; /* winuser.h:981:46, winuser.h:981:46, winuser.h:981:46 */
#[cfg(feature="winapi_desktop")] pub type LPEVENTMSG = *mut ::winuser::EVENTMSG; /* winuser.h:981:63, winuser.h:981:63, winuser.h:981:63 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct CWPSTRUCT { lParam: ::minwindef::LPARAM, wParam: ::minwindef::WPARAM, message: ::minwindef::UINT, hwnd: ::windef::HWND } /* winuser.h:986:16, winuser.h:986:16, winuser.h:986:16 */
#[cfg(feature="winapi_desktop")] pub type PCWPSTRUCT = *mut ::winuser::CWPSTRUCT; /* winuser.h:991:15, winuser.h:991:15, winuser.h:991:15 */
#[cfg(feature="winapi_desktop")] pub type NPCWPSTRUCT = *mut ::winuser::CWPSTRUCT; /* winuser.h:991:33, winuser.h:991:33, winuser.h:991:33 */
#[cfg(feature="winapi_desktop")] pub type LPCWPSTRUCT = *mut ::winuser::CWPSTRUCT; /* winuser.h:991:51, winuser.h:991:51, winuser.h:991:51 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct CWPRETSTRUCT { lResult: ::minwindef::LRESULT, lParam: ::minwindef::LPARAM, wParam: ::minwindef::WPARAM, message: ::minwindef::UINT, hwnd: ::windef::HWND } /* winuser.h:997:16, winuser.h:997:16, winuser.h:997:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PCWPRETSTRUCT = *mut ::winuser::CWPRETSTRUCT; /* winuser.h:1003:18, winuser.h:1003:18, winuser.h:1003:18 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type NPCWPRETSTRUCT = *mut ::winuser::CWPRETSTRUCT; /* winuser.h:1003:39, winuser.h:1003:39, winuser.h:1003:39 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPCWPRETSTRUCT = *mut ::winuser::CWPRETSTRUCT; /* winuser.h:1003:60, winuser.h:1003:60, winuser.h:1003:60 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct KBDLLHOOKSTRUCT { vkCode: ::minwindef::DWORD, scanCode: ::minwindef::DWORD, flags: ::minwindef::DWORD, time: ::minwindef::DWORD, dwExtraInfo: ::basetsd::ULONG_PTR } /* winuser.h:1031:16, winuser.h:1031:16, winuser.h:1031:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPKBDLLHOOKSTRUCT = *mut ::winuser::KBDLLHOOKSTRUCT; /* winuser.h:1037:25, winuser.h:1037:25, winuser.h:1037:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PKBDLLHOOKSTRUCT = *mut ::winuser::KBDLLHOOKSTRUCT; /* winuser.h:1037:45, winuser.h:1037:45, winuser.h:1037:45 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct MSLLHOOKSTRUCT { pt: ::windef::POINT, mouseData: ::minwindef::DWORD, flags: ::minwindef::DWORD, time: ::minwindef::DWORD, dwExtraInfo: ::basetsd::ULONG_PTR } /* winuser.h:1042:16, winuser.h:1042:16, winuser.h:1042:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPMSLLHOOKSTRUCT = *mut ::winuser::MSLLHOOKSTRUCT; /* winuser.h:1048:24, winuser.h:1048:24, winuser.h:1048:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PMSLLHOOKSTRUCT = *mut ::winuser::MSLLHOOKSTRUCT; /* winuser.h:1048:43, winuser.h:1048:43, winuser.h:1048:43 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct DEBUGHOOKINFO { idThread: ::minwindef::DWORD, idThreadInstaller: ::minwindef::DWORD, lParam: ::minwindef::LPARAM, wParam: ::minwindef::WPARAM, code: ::libc::c_int } /* winuser.h:1061:16, winuser.h:1061:16, winuser.h:1061:16 */
#[cfg(feature="winapi_desktop")] pub type PDEBUGHOOKINFO = *mut ::winuser::DEBUGHOOKINFO; /* winuser.h:1068:19, winuser.h:1068:19, winuser.h:1068:19 */
#[cfg(feature="winapi_desktop")] pub type NPDEBUGHOOKINFO = *mut ::winuser::DEBUGHOOKINFO; /* winuser.h:1068:41, winuser.h:1068:41, winuser.h:1068:41 */
#[cfg(feature="winapi_desktop")] pub type LPDEBUGHOOKINFO = *mut ::winuser::DEBUGHOOKINFO; /* winuser.h:1068:63, winuser.h:1068:63, winuser.h:1068:63 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct MOUSEHOOKSTRUCT { pt: ::windef::POINT, hwnd: ::windef::HWND, wHitTestCode: ::minwindef::UINT, dwExtraInfo: ::basetsd::ULONG_PTR } /* winuser.h:1073:16, winuser.h:1073:16, winuser.h:1073:16 */
#[cfg(feature="winapi_desktop")] pub type LPMOUSEHOOKSTRUCT = *mut ::winuser::MOUSEHOOKSTRUCT; /* winuser.h:1078:25, winuser.h:1078:25, winuser.h:1078:25 */
#[cfg(feature="winapi_desktop")] pub type PMOUSEHOOKSTRUCT = *mut ::winuser::MOUSEHOOKSTRUCT; /* winuser.h:1078:45, winuser.h:1078:45, winuser.h:1078:45 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct MOUSEHOOKSTRUCTEX { mouseData: ::minwindef::DWORD } /* winuser.h:1087:16, winuser.h:1087:16, winuser.h:1087:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPMOUSEHOOKSTRUCTEX = *mut ::winuser::MOUSEHOOKSTRUCTEX; /* winuser.h:1091:23, winuser.h:1091:23, winuser.h:1091:23 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PMOUSEHOOKSTRUCTEX = *mut ::winuser::MOUSEHOOKSTRUCTEX; /* winuser.h:1091:45, winuser.h:1091:45, winuser.h:1091:45 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct HARDWAREHOOKSTRUCT { hwnd: ::windef::HWND, message: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM } /* winuser.h:1099:16, winuser.h:1099:16, winuser.h:1099:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPHARDWAREHOOKSTRUCT = *mut ::winuser::HARDWAREHOOKSTRUCT; /* winuser.h:1104:28, winuser.h:1104:28, winuser.h:1104:28 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PHARDWAREHOOKSTRUCT = *mut ::winuser::HARDWAREHOOKSTRUCT; /* winuser.h:1104:51, winuser.h:1104:51, winuser.h:1104:51 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct MOUSEMOVEPOINT { x: ::libc::c_int, y: ::libc::c_int, time: ::minwindef::DWORD, dwExtraInfo: ::basetsd::ULONG_PTR } /* winuser.h:1246:16, winuser.h:1246:16, winuser.h:1246:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PMOUSEMOVEPOINT = *mut ::winuser::MOUSEMOVEPOINT; /* winuser.h:1251:20, winuser.h:1251:20, winuser.h:1251:20 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPMOUSEMOVEPOINT = *mut ::winuser::MOUSEMOVEPOINT; /* winuser.h:1251:42, winuser.h:1251:42, winuser.h:1251:42 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct USEROBJECTFLAGS { fInherit: ::minwindef::BOOL, fReserved: ::minwindef::BOOL, dwFlags: ::minwindef::DWORD } /* winuser.h:1612:16, winuser.h:1612:16, winuser.h:1612:16 */
#[cfg(feature="winapi_desktop")] pub type PUSEROBJECTFLAGS = *mut ::winuser::USEROBJECTFLAGS; /* winuser.h:1616:21, winuser.h:1616:21, winuser.h:1616:21 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct WNDCLASSEXA { cbSize: ::minwindef::UINT, style: ::minwindef::UINT, lpfnWndProc: ::winuser::WNDPROC, cbClsExtra: ::libc::c_int, cbWndExtra: ::libc::c_int, hInstance: ::minwindef::HINSTANCE, hIcon: ::windef::HICON, hCursor: ::windef::HCURSOR, hbrBackground: ::windef::HBRUSH, lpszMenuName: ::winnt::LPCSTR, lpszClassName: ::winnt::LPCSTR, hIconSm: ::windef::HICON } /* winuser.h:1673:16, winuser.h:1673:16, winuser.h:1673:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PWNDCLASSEXA = *mut ::winuser::WNDCLASSEXA; /* winuser.h:1688:17, winuser.h:1688:17, winuser.h:1688:17 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type NPWNDCLASSEXA = *mut ::winuser::WNDCLASSEXA; /* winuser.h:1688:37, winuser.h:1688:37, winuser.h:1688:37 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPWNDCLASSEXA = *mut ::winuser::WNDCLASSEXA; /* winuser.h:1688:57, winuser.h:1688:57, winuser.h:1688:57 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct WNDCLASSEXW { cbSize: ::minwindef::UINT, style: ::minwindef::UINT, lpfnWndProc: ::winuser::WNDPROC, cbClsExtra: ::libc::c_int, cbWndExtra: ::libc::c_int, hInstance: ::minwindef::HINSTANCE, hIcon: ::windef::HICON, hCursor: ::windef::HCURSOR, hbrBackground: ::windef::HBRUSH, lpszMenuName: ::winnt::LPCWSTR, lpszClassName: ::winnt::LPCWSTR, hIconSm: ::windef::HICON } /* winuser.h:1689:16, winuser.h:1689:16, winuser.h:1689:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PWNDCLASSEXW = *mut ::winuser::WNDCLASSEXW; /* winuser.h:1704:17, winuser.h:1704:17, winuser.h:1704:17 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type NPWNDCLASSEXW = *mut ::winuser::WNDCLASSEXW; /* winuser.h:1704:37, winuser.h:1704:37, winuser.h:1704:37 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPWNDCLASSEXW = *mut ::winuser::WNDCLASSEXW; /* winuser.h:1704:57, winuser.h:1704:57, winuser.h:1704:57 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type WNDCLASSEX = ::winuser::WNDCLASSEXW; /* winuser.h:1706:21, winuser.h:1706:21, winuser.h:1706:21 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PWNDCLASSEX = ::winuser::PWNDCLASSEXW; /* winuser.h:1707:22, winuser.h:1707:22, winuser.h:1707:22 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type NPWNDCLASSEX = ::winuser::NPWNDCLASSEXW; /* winuser.h:1708:23, winuser.h:1708:23, winuser.h:1708:23 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPWNDCLASSEX = ::winuser::LPWNDCLASSEXW; /* winuser.h:1709:23, winuser.h:1709:23, winuser.h:1709:23 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct WNDCLASSA { style: ::minwindef::UINT, lpfnWndProc: ::winuser::WNDPROC, cbClsExtra: ::libc::c_int, cbWndExtra: ::libc::c_int, hInstance: ::minwindef::HINSTANCE, hIcon: ::windef::HICON, hCursor: ::windef::HCURSOR, hbrBackground: ::windef::HBRUSH, lpszMenuName: ::winnt::LPCSTR, lpszClassName: ::winnt::LPCSTR } /* winuser.h:1718:16, winuser.h:1718:16, winuser.h:1718:16 */
#[cfg(feature="winapi_desktop")] pub type PWNDCLASSA = *mut ::winuser::WNDCLASSA; /* winuser.h:1729:15, winuser.h:1729:15, winuser.h:1729:15 */
#[cfg(feature="winapi_desktop")] pub type NPWNDCLASSA = *mut ::winuser::WNDCLASSA; /* winuser.h:1729:33, winuser.h:1729:33, winuser.h:1729:33 */
#[cfg(feature="winapi_desktop")] pub type LPWNDCLASSA = *mut ::winuser::WNDCLASSA; /* winuser.h:1729:51, winuser.h:1729:51, winuser.h:1729:51 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct WNDCLASSW { style: ::minwindef::UINT, lpfnWndProc: ::winuser::WNDPROC, cbClsExtra: ::libc::c_int, cbWndExtra: ::libc::c_int, hInstance: ::minwindef::HINSTANCE, hIcon: ::windef::HICON, hCursor: ::windef::HCURSOR, hbrBackground: ::windef::HBRUSH, lpszMenuName: ::winnt::LPCWSTR, lpszClassName: ::winnt::LPCWSTR } /* winuser.h:1730:16, winuser.h:1730:16, winuser.h:1730:16 */
#[cfg(feature="winapi_desktop")] pub type PWNDCLASSW = *mut ::winuser::WNDCLASSW; /* winuser.h:1741:15, winuser.h:1741:15, winuser.h:1741:15 */
#[cfg(feature="winapi_desktop")] pub type NPWNDCLASSW = *mut ::winuser::WNDCLASSW; /* winuser.h:1741:33, winuser.h:1741:33, winuser.h:1741:33 */
#[cfg(feature="winapi_desktop")] pub type LPWNDCLASSW = *mut ::winuser::WNDCLASSW; /* winuser.h:1741:51, winuser.h:1741:51, winuser.h:1741:51 */
#[cfg(feature="winapi_desktop")] pub type WNDCLASS = ::winuser::WNDCLASSW; /* winuser.h:1743:19, winuser.h:1743:19, winuser.h:1743:19 */
#[cfg(feature="winapi_desktop")] pub type PWNDCLASS = ::winuser::PWNDCLASSW; /* winuser.h:1744:20, winuser.h:1744:20, winuser.h:1744:20 */
#[cfg(feature="winapi_desktop")] pub type NPWNDCLASS = ::winuser::NPWNDCLASSW; /* winuser.h:1745:21, winuser.h:1745:21, winuser.h:1745:21 */
#[cfg(feature="winapi_desktop")] pub type LPWNDCLASS = ::winuser::LPWNDCLASSW; /* winuser.h:1746:21, winuser.h:1746:21, winuser.h:1746:21 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct MSG { hwnd: ::windef::HWND, message: ::minwindef::UINT, wParam: ::minwindef::WPARAM, lParam: ::minwindef::LPARAM, time: ::minwindef::DWORD, pt: ::windef::POINT } /* winuser.h:1781:16, winuser.h:1781:16, winuser.h:1781:16 */
#[cfg(feature="winapi_app")] pub type PMSG = *mut ::winuser::MSG; /* winuser.h:1791:9, winuser.h:1791:9, winuser.h:1791:9 */
#[cfg(feature="winapi_app")] pub type NPMSG = *mut ::winuser::MSG; /* winuser.h:1791:21, winuser.h:1791:21, winuser.h:1791:21 */
#[cfg(feature="winapi_app")] pub type LPMSG = *mut ::winuser::MSG; /* winuser.h:1791:33, winuser.h:1791:33, winuser.h:1791:33 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct MINMAXINFO { ptReserved: ::windef::POINT, ptMaxSize: ::windef::POINT, ptMaxPosition: ::windef::POINT, ptMinTrackSize: ::windef::POINT, ptMaxTrackSize: ::windef::POINT } /* winuser.h:1939:16, winuser.h:1939:16, winuser.h:1939:16 */
#[cfg(feature="winapi_desktop")] pub type PMINMAXINFO = *mut ::winuser::MINMAXINFO; /* winuser.h:1945:16, winuser.h:1945:16, winuser.h:1945:16 */
#[cfg(feature="winapi_desktop")] pub type LPMINMAXINFO = *mut ::winuser::MINMAXINFO; /* winuser.h:1945:30, winuser.h:1945:30, winuser.h:1945:30 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct COPYDATASTRUCT { dwData: ::basetsd::ULONG_PTR, cbData: ::minwindef::DWORD, lpData: ::winnt::PVOID } /* winuser.h:1995:16, winuser.h:1995:16, winuser.h:1995:16 */
#[cfg(feature="winapi_desktop")] pub type PCOPYDATASTRUCT = *mut ::winuser::COPYDATASTRUCT; /* winuser.h:1999:20, winuser.h:1999:20, winuser.h:1999:20 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct MDINEXTMENU { hmenuIn: ::windef::HMENU, hmenuNext: ::windef::HMENU, hwndNext: ::windef::HWND } /* winuser.h:2002:16, winuser.h:2002:16, winuser.h:2002:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PMDINEXTMENU = *mut ::winuser::MDINEXTMENU; /* winuser.h:2007:18, winuser.h:2007:18, winuser.h:2007:18 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPMDINEXTMENU = *mut ::winuser::MDINEXTMENU; /* winuser.h:2007:38, winuser.h:2007:38, winuser.h:2007:38 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] #[repr(C)] pub struct POWERBROADCAST_SETTING { PowerSetting: ::guiddef::GUID, DataLength: ::minwindef::DWORD, Data: *mut [::minwindef::UCHAR; 1] } /* winuser.h:2257:9, winuser.h:2257:9, winuser.h:2257:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] pub type PPOWERBROADCAST_SETTING = *mut ::winuser::POWERBROADCAST_SETTING; /* winuser.h:2261:28, winuser.h:2261:28, winuser.h:2261:28 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct WINDOWPOS { hwnd: ::windef::HWND, hwndInsertAfter: ::windef::HWND, x: ::libc::c_int, y: ::libc::c_int, cx: ::libc::c_int, cy: ::libc::c_int, flags: ::minwindef::UINT } /* winuser.h:2575:16, winuser.h:2575:16, winuser.h:2575:16 */
#[cfg(feature="winapi_desktop")] pub type LPWINDOWPOS = *mut ::winuser::WINDOWPOS; /* winuser.h:2583:15, winuser.h:2583:15, winuser.h:2583:15 */
#[cfg(feature="winapi_desktop")] pub type PWINDOWPOS = *mut ::winuser::WINDOWPOS; /* winuser.h:2583:29, winuser.h:2583:29, winuser.h:2583:29 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct NCCALCSIZE_PARAMS { rgrc: *mut [::windef::RECT; 3], lppos: ::winuser::PWINDOWPOS } /* winuser.h:2588:16, winuser.h:2588:16, winuser.h:2588:16 */
#[cfg(feature="winapi_desktop")] pub type LPNCCALCSIZE_PARAMS = *mut ::winuser::NCCALCSIZE_PARAMS; /* winuser.h:2591:23, winuser.h:2591:23, winuser.h:2591:23 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct TRACKMOUSEEVENT { cbSize: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD, hwndTrack: ::windef::HWND, dwHoverTime: ::minwindef::DWORD } /* winuser.h:2648:16, winuser.h:2648:16, winuser.h:2648:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPTRACKMOUSEEVENT = *mut ::winuser::TRACKMOUSEEVENT; /* winuser.h:2653:21, winuser.h:2653:21, winuser.h:2653:21 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct ACCEL { fVirt: ::minwindef::BYTE, key: ::minwindef::WORD, cmd: ::minwindef::WORD } /* winuser.h:3061:16, winuser.h:3061:16, winuser.h:3061:16 */
#[cfg(feature="winapi_desktop")] pub type LPACCEL = *mut ::winuser::ACCEL; /* winuser.h:3071:11, winuser.h:3071:11, winuser.h:3071:11 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct PAINTSTRUCT { hdc: ::windef::HDC, fErase: ::minwindef::BOOL, rcPaint: ::windef::RECT, fRestore: ::minwindef::BOOL, fIncUpdate: ::minwindef::BOOL, rgbReserved: *mut [::minwindef::BYTE; 32] } /* winuser.h:3073:16, winuser.h:3073:16, winuser.h:3073:16 */
#[cfg(feature="winapi_desktop")] pub type PPAINTSTRUCT = *mut ::winuser::PAINTSTRUCT; /* winuser.h:3080:17, winuser.h:3080:17, winuser.h:3080:17 */
#[cfg(feature="winapi_desktop")] pub type NPPAINTSTRUCT = *mut ::winuser::PAINTSTRUCT; /* winuser.h:3080:32, winuser.h:3080:32, winuser.h:3080:32 */
#[cfg(feature="winapi_desktop")] pub type LPPAINTSTRUCT = *mut ::winuser::PAINTSTRUCT; /* winuser.h:3080:48, winuser.h:3080:48, winuser.h:3080:48 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct CREATESTRUCTA { lpCreateParams: ::minwindef::LPVOID, hInstance: ::minwindef::HINSTANCE, hMenu: ::windef::HMENU, hwndParent: ::windef::HWND, cy: ::libc::c_int, cx: ::libc::c_int, y: ::libc::c_int, x: ::libc::c_int, style: ::winnt::LONG, lpszName: ::winnt::LPCSTR, lpszClass: ::winnt::LPCSTR, dwExStyle: ::minwindef::DWORD } /* winuser.h:3082:16, winuser.h:3082:16, winuser.h:3082:16 */
#[cfg(feature="winapi_desktop")] pub type LPCREATESTRUCTA = *mut ::winuser::CREATESTRUCTA; /* winuser.h:3095:19, winuser.h:3095:19, winuser.h:3095:19 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct CREATESTRUCTW { lpCreateParams: ::minwindef::LPVOID, hInstance: ::minwindef::HINSTANCE, hMenu: ::windef::HMENU, hwndParent: ::windef::HWND, cy: ::libc::c_int, cx: ::libc::c_int, y: ::libc::c_int, x: ::libc::c_int, style: ::winnt::LONG, lpszName: ::winnt::LPCWSTR, lpszClass: ::winnt::LPCWSTR, dwExStyle: ::minwindef::DWORD } /* winuser.h:3096:16, winuser.h:3096:16, winuser.h:3096:16 */
#[cfg(feature="winapi_desktop")] pub type LPCREATESTRUCTW = *mut ::winuser::CREATESTRUCTW; /* winuser.h:3109:19, winuser.h:3109:19, winuser.h:3109:19 */
#[cfg(feature="winapi_desktop")] pub type CREATESTRUCT = ::winuser::CREATESTRUCTW; /* winuser.h:3111:23, winuser.h:3111:23, winuser.h:3111:23 */
#[cfg(feature="winapi_desktop")] pub type LPCREATESTRUCT = ::winuser::LPCREATESTRUCTW; /* winuser.h:3112:25, winuser.h:3112:25, winuser.h:3112:25 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct WINDOWPLACEMENT { length: ::minwindef::UINT, flags: ::minwindef::UINT, showCmd: ::minwindef::UINT, ptMinPosition: ::windef::POINT, ptMaxPosition: ::windef::POINT, rcNormalPosition: ::windef::RECT } /* winuser.h:3118:16, winuser.h:3118:16, winuser.h:3118:16 */
#[cfg(feature="winapi_desktop")] pub type PWINDOWPLACEMENT = *mut ::winuser::WINDOWPLACEMENT; /* winuser.h:3129:26, winuser.h:3129:26, winuser.h:3129:26 */
#[cfg(feature="winapi_desktop")] pub type LPWINDOWPLACEMENT = *mut ::winuser::WINDOWPLACEMENT; /* winuser.h:3129:45, winuser.h:3129:45, winuser.h:3129:45 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct NMHDR { hwndFrom: ::windef::HWND, idFrom: ::basetsd::UINT_PTR, code: ::minwindef::UINT } /* winuser.h:3145:16, winuser.h:3145:16, winuser.h:3145:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPNMHDR = *mut ::winuser::NMHDR; /* winuser.h:3158:21, winuser.h:3158:21, winuser.h:3158:21 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct STYLESTRUCT { styleOld: ::minwindef::DWORD, styleNew: ::minwindef::DWORD } /* winuser.h:3160:16, winuser.h:3160:16, winuser.h:3160:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPSTYLESTRUCT = *mut ::winuser::STYLESTRUCT; /* winuser.h:3164:18, winuser.h:3164:18, winuser.h:3164:18 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct MEASUREITEMSTRUCT { CtlType: ::minwindef::UINT, CtlID: ::minwindef::UINT, itemID: ::minwindef::UINT, itemWidth: ::minwindef::UINT, itemHeight: ::minwindef::UINT, itemData: ::basetsd::ULONG_PTR } /* winuser.h:3216:16, winuser.h:3216:16, winuser.h:3216:16 */
#[cfg(feature="winapi_desktop")] pub type PMEASUREITEMSTRUCT = *mut ::winuser::MEASUREITEMSTRUCT; /* winuser.h:3223:28, winuser.h:3223:28, winuser.h:3223:28 */
#[cfg(feature="winapi_desktop")] pub type LPMEASUREITEMSTRUCT = *mut ::winuser::MEASUREITEMSTRUCT; /* winuser.h:3223:53, winuser.h:3223:53, winuser.h:3223:53 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct DRAWITEMSTRUCT { CtlType: ::minwindef::UINT, CtlID: ::minwindef::UINT, itemID: ::minwindef::UINT, itemAction: ::minwindef::UINT, itemState: ::minwindef::UINT, hwndItem: ::windef::HWND, hDC: ::windef::HDC, rcItem: ::windef::RECT, itemData: ::basetsd::ULONG_PTR } /* winuser.h:3228:16, winuser.h:3228:16, winuser.h:3228:16 */
#[cfg(feature="winapi_desktop")] pub type PDRAWITEMSTRUCT = *mut ::winuser::DRAWITEMSTRUCT; /* winuser.h:3238:25, winuser.h:3238:25, winuser.h:3238:25 */
#[cfg(feature="winapi_desktop")] pub type LPDRAWITEMSTRUCT = *mut ::winuser::DRAWITEMSTRUCT; /* winuser.h:3238:47, winuser.h:3238:47, winuser.h:3238:47 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct DELETEITEMSTRUCT { CtlType: ::minwindef::UINT, CtlID: ::minwindef::UINT, itemID: ::minwindef::UINT, hwndItem: ::windef::HWND, itemData: ::basetsd::ULONG_PTR } /* winuser.h:3243:16, winuser.h:3243:16, winuser.h:3243:16 */
#[cfg(feature="winapi_desktop")] pub type PDELETEITEMSTRUCT = *mut ::winuser::DELETEITEMSTRUCT; /* winuser.h:3249:27, winuser.h:3249:27, winuser.h:3249:27 */
#[cfg(feature="winapi_desktop")] pub type LPDELETEITEMSTRUCT = *mut ::winuser::DELETEITEMSTRUCT; /* winuser.h:3249:51, winuser.h:3249:51, winuser.h:3249:51 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct COMPAREITEMSTRUCT { CtlType: ::minwindef::UINT, CtlID: ::minwindef::UINT, hwndItem: ::windef::HWND, itemID1: ::minwindef::UINT, itemData1: ::basetsd::ULONG_PTR, itemID2: ::minwindef::UINT, itemData2: ::basetsd::ULONG_PTR, dwLocaleId: ::minwindef::DWORD } /* winuser.h:3254:16, winuser.h:3254:16, winuser.h:3254:16 */
#[cfg(feature="winapi_desktop")] pub type PCOMPAREITEMSTRUCT = *mut ::winuser::COMPAREITEMSTRUCT; /* winuser.h:3263:28, winuser.h:3263:28, winuser.h:3263:28 */
#[cfg(feature="winapi_desktop")] pub type LPCOMPAREITEMSTRUCT = *mut ::winuser::COMPAREITEMSTRUCT; /* winuser.h:3263:53, winuser.h:3263:53, winuser.h:3263:53 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct BSMINFO { cbSize: ::minwindef::UINT, hdesk: ::windef::HDESK, hwnd: ::windef::HWND, luid: ::winnt::LUID } /* winuser.h:3673:9, winuser.h:3673:9, winuser.h:3673:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PBSMINFO = *mut ::winuser::BSMINFO; /* winuser.h:3678:13, winuser.h:3678:13, winuser.h:3678:13 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type HDEVNOTIFY = ::winnt::PVOID; /* winuser.h:3790:26, winuser.h:3790:26, winuser.h:3790:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PHDEVNOTIFY = *mut *mut ::libc::c_void; /* winuser.h:3791:26, winuser.h:3791:26, winuser.h:3791:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] pub type HPOWERNOTIFY = ::winnt::PVOID; /* winuser.h:3832:26, winuser.h:3832:26, winuser.h:3832:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] pub type PHPOWERNOTIFY = *mut *mut ::libc::c_void; /* winuser.h:3833:26, winuser.h:3833:26, winuser.h:3833:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PREGISTERCLASSNAMEW = extern "system" fn(*const ::libc::c_ushort) -> ::libc::c_uchar; /* winuser.h:4231:27, winuser.h:4231:27, winuser.h:4231:27 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct UPDATELAYEREDWINDOWINFO { cbSize: ::minwindef::DWORD, hdcDst: ::windef::HDC, pptDst: *const ::windef::POINT, psize: *const ::windef::SIZE, hdcSrc: ::windef::HDC, pptSrc: *const ::windef::POINT, crKey: ::windef::COLORREF, pblend: *const ::wingdi::BLENDFUNCTION, dwFlags: ::minwindef::DWORD, prcDirty: *const ::windef::RECT } /* winuser.h:4362:16, winuser.h:4362:16, winuser.h:4362:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PUPDATELAYEREDWINDOWINFO = *mut ::winuser::UPDATELAYEREDWINDOWINFO; /* winuser.h:4374:29, winuser.h:4374:29, winuser.h:4374:29 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct FLASHWINFO { cbSize: ::minwindef::UINT, hwnd: ::windef::HWND, dwFlags: ::minwindef::DWORD, uCount: ::minwindef::UINT, dwTimeout: ::minwindef::DWORD } /* winuser.h:4475:9, winuser.h:4475:9, winuser.h:4475:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PFLASHWINFO = *mut ::winuser::FLASHWINFO; /* winuser.h:4481:16, winuser.h:4481:16, winuser.h:4481:16 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct DLGTEMPLATE { style: ::minwindef::DWORD, dwExtendedStyle: ::minwindef::DWORD, cdit: ::minwindef::WORD, x: ::libc::c_short, y: ::libc::c_short, cx: ::libc::c_short, cy: ::libc::c_short } /* winuser.h:4694:9, winuser.h:4694:9, winuser.h:4694:9 */
#[cfg(feature="winapi_desktop")] pub type LPDLGTEMPLATEA = *mut ::winuser::DLGTEMPLATE; /* winuser.h:4710:22, winuser.h:4710:22, winuser.h:4710:22 */
#[cfg(feature="winapi_desktop")] pub type LPDLGTEMPLATEW = *mut ::winuser::DLGTEMPLATE; /* winuser.h:4711:22, winuser.h:4711:22, winuser.h:4711:22 */
#[cfg(feature="winapi_desktop")] pub type LPDLGTEMPLATE = ::winuser::LPDLGTEMPLATEW; /* winuser.h:4713:24, winuser.h:4713:24, winuser.h:4713:24 */
#[cfg(feature="winapi_app")] pub type LPCDLGTEMPLATEA = *const ::winuser::DLGTEMPLATE; /* winuser.h:4724:28, winuser.h:4724:28, winuser.h:4724:28 */
#[cfg(feature="winapi_app")] pub type LPCDLGTEMPLATEW = *const ::winuser::DLGTEMPLATE; /* winuser.h:4725:28, winuser.h:4725:28, winuser.h:4725:28 */
#[cfg(feature="winapi_app")] pub type LPCDLGTEMPLATE = ::winuser::LPCDLGTEMPLATEW; /* winuser.h:4727:25, winuser.h:4727:25, winuser.h:4727:25 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct DLGITEMTEMPLATE { style: ::minwindef::DWORD, dwExtendedStyle: ::minwindef::DWORD, x: ::libc::c_short, y: ::libc::c_short, cx: ::libc::c_short, cy: ::libc::c_short, id: ::minwindef::WORD } /* winuser.h:4742:9, winuser.h:4742:9, winuser.h:4742:9 */
#[cfg(feature="winapi_desktop")] pub type PDLGITEMTEMPLATEA = *mut ::winuser::DLGITEMTEMPLATE; /* winuser.h:4751:26, winuser.h:4751:26, winuser.h:4751:26 */
#[cfg(feature="winapi_desktop")] pub type PDLGITEMTEMPLATEW = *mut ::winuser::DLGITEMTEMPLATE; /* winuser.h:4752:26, winuser.h:4752:26, winuser.h:4752:26 */
#[cfg(feature="winapi_desktop")] pub type PDLGITEMTEMPLATE = ::winuser::PDLGITEMTEMPLATEW; /* winuser.h:4754:27, winuser.h:4754:27, winuser.h:4754:27 */
#[cfg(feature="winapi_desktop")] pub type LPDLGITEMTEMPLATEA = *mut ::winuser::DLGITEMTEMPLATE; /* winuser.h:4758:26, winuser.h:4758:26, winuser.h:4758:26 */
#[cfg(feature="winapi_desktop")] pub type LPDLGITEMTEMPLATEW = *mut ::winuser::DLGITEMTEMPLATE; /* winuser.h:4759:26, winuser.h:4759:26, winuser.h:4759:26 */
#[cfg(feature="winapi_desktop")] pub type LPDLGITEMTEMPLATE = ::winuser::LPDLGITEMTEMPLATEW; /* winuser.h:4761:28, winuser.h:4761:28, winuser.h:4761:28 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct MOUSEINPUT { dx: ::winnt::LONG, dy: ::winnt::LONG, mouseData: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD, time: ::minwindef::DWORD, dwExtraInfo: ::basetsd::ULONG_PTR } /* winuser.h:5800:16, winuser.h:5800:16, winuser.h:5800:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PMOUSEINPUT = *mut ::winuser::MOUSEINPUT; /* winuser.h:5807:16, winuser.h:5807:16, winuser.h:5807:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPMOUSEINPUT = *mut ::winuser::MOUSEINPUT; /* winuser.h:5807:34, winuser.h:5807:34, winuser.h:5807:34 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct KEYBDINPUT { wVk: ::minwindef::WORD, wScan: ::minwindef::WORD, dwFlags: ::minwindef::DWORD, time: ::minwindef::DWORD, dwExtraInfo: ::basetsd::ULONG_PTR } /* winuser.h:5809:16, winuser.h:5809:16, winuser.h:5809:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PKEYBDINPUT = *mut ::winuser::KEYBDINPUT; /* winuser.h:5815:16, winuser.h:5815:16, winuser.h:5815:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPKEYBDINPUT = *mut ::winuser::KEYBDINPUT; /* winuser.h:5815:34, winuser.h:5815:34, winuser.h:5815:34 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct HARDWAREINPUT { uMsg: ::minwindef::DWORD, wParamL: ::minwindef::WORD, wParamH: ::minwindef::WORD } /* winuser.h:5817:16, winuser.h:5817:16, winuser.h:5817:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PHARDWAREINPUT = *mut ::winuser::HARDWAREINPUT; /* winuser.h:5821:19, winuser.h:5821:19, winuser.h:5821:19 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPHARDWAREINPUT = *mut ::winuser::HARDWAREINPUT; /* winuser.h:5821:40, winuser.h:5821:40, winuser.h:5821:40 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct INPUT { type_: ::minwindef::DWORD } /* winuser.h:5827:16, winuser.h:5827:16, winuser.h:5827:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub /*union*/ struct INPUT_Child_1; /* STUB! */ /* winuser.h:5830:5, winuser.h:5830:5, winuser.h:5830:5 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PINPUT = *mut ::winuser::INPUT; /* winuser.h:5836:11, winuser.h:5836:11, winuser.h:5836:11 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPINPUT = *mut ::winuser::INPUT; /* winuser.h:5836:24, winuser.h:5836:24, winuser.h:5836:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct HTOUCHINPUT__ { unused: ::libc::c_int } /* winuser.h:5863:1, winuser.h:5863:1, winuser.h:5863:1 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub type HTOUCHINPUT = *mut ::winuser::HTOUCHINPUT__; /* winuser.h:5863:1, winuser.h:5863:1, winuser.h:5863:1 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct TOUCHINPUT { x: ::winnt::LONG, y: ::winnt::LONG, hSource: ::winnt::HANDLE, dwID: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD, dwMask: ::minwindef::DWORD, dwTime: ::minwindef::DWORD, dwExtraInfo: ::basetsd::ULONG_PTR, cxContact: ::minwindef::DWORD, cyContact: ::minwindef::DWORD } /* winuser.h:5865:16, winuser.h:5865:16, winuser.h:5865:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub type PTOUCHINPUT = *mut ::winuser::TOUCHINPUT; /* winuser.h:5876:16, winuser.h:5876:16, winuser.h:5876:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub type PCTOUCHINPUT = *const ::winuser::TOUCHINPUT; /* winuser.h:5877:28, winuser.h:5877:28, winuser.h:5877:28 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub enum tagPOINTER_INPUT_TYPE {PT_POINTER = 1, PT_TOUCH = 2, PT_PEN = 3, PT_MOUSE = 4} pub use self::tagPOINTER_INPUT_TYPE::{PT_POINTER, PT_TOUCH, PT_PEN, PT_MOUSE}; /* winuser.h:5976:6, winuser.h:5976:6, winuser.h:5976:6 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub type POINTER_INPUT_TYPE = ::minwindef::DWORD; /* winuser.h:5985:15, winuser.h:5985:15, winuser.h:5985:15 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub type POINTER_FLAGS = ::basetsd::UINT32; /* winuser.h:5987:16, winuser.h:5987:16, winuser.h:5987:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub enum POINTER_BUTTON_CHANGE_TYPE {POINTER_CHANGE_NONE = 0, POINTER_CHANGE_FIRSTBUTTON_DOWN = 1, POINTER_CHANGE_FIRSTBUTTON_UP = 2, POINTER_CHANGE_SECONDBUTTON_DOWN = 3, POINTER_CHANGE_SECONDBUTTON_UP = 4, POINTER_CHANGE_THIRDBUTTON_DOWN = 5, POINTER_CHANGE_THIRDBUTTON_UP = 6, POINTER_CHANGE_FOURTHBUTTON_DOWN = 7, POINTER_CHANGE_FOURTHBUTTON_UP = 8, POINTER_CHANGE_FIFTHBUTTON_DOWN = 9, POINTER_CHANGE_FIFTHBUTTON_UP = 10} pub use self::POINTER_BUTTON_CHANGE_TYPE::{POINTER_CHANGE_NONE, POINTER_CHANGE_FIRSTBUTTON_DOWN, POINTER_CHANGE_FIRSTBUTTON_UP, POINTER_CHANGE_SECONDBUTTON_DOWN, POINTER_CHANGE_SECONDBUTTON_UP, POINTER_CHANGE_THIRDBUTTON_DOWN, POINTER_CHANGE_THIRDBUTTON_UP, POINTER_CHANGE_FOURTHBUTTON_DOWN, POINTER_CHANGE_FOURTHBUTTON_UP, POINTER_CHANGE_FIFTHBUTTON_DOWN, POINTER_CHANGE_FIFTHBUTTON_UP}; /* winuser.h:6022:14, winuser.h:6022:14, winuser.h:6022:14 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub struct POINTER_INFO { pointerType: ::winuser::POINTER_INPUT_TYPE, pointerId: ::basetsd::UINT32, frameId: ::basetsd::UINT32, pointerFlags: ::winuser::POINTER_FLAGS, sourceDevice: ::winnt::HANDLE, hwndTarget: ::windef::HWND, ptPixelLocation: ::windef::POINT, ptHimetricLocation: ::windef::POINT, ptPixelLocationRaw: ::windef::POINT, ptHimetricLocationRaw: ::windef::POINT, dwTime: ::minwindef::DWORD, historyCount: ::basetsd::UINT32, InputData: ::basetsd::INT32, dwKeyStates: ::minwindef::DWORD, PerformanceCount: ::basetsd::UINT64, ButtonChangeType: ::winuser::POINTER_BUTTON_CHANGE_TYPE } /* winuser.h:6036:16, winuser.h:6036:16, winuser.h:6036:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub type TOUCH_FLAGS = ::basetsd::UINT32; /* winuser.h:6056:16, winuser.h:6056:16, winuser.h:6056:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub type TOUCH_MASK = ::basetsd::UINT32; /* winuser.h:6059:16, winuser.h:6059:16, winuser.h:6059:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub struct POINTER_TOUCH_INFO { pointerInfo: ::winuser::POINTER_INFO, touchFlags: ::winuser::TOUCH_FLAGS, touchMask: ::winuser::TOUCH_MASK, rcContact: ::windef::RECT, rcContactRaw: ::windef::RECT, orientation: ::basetsd::UINT32, pressure: ::basetsd::UINT32 } /* winuser.h:6065:16, winuser.h:6065:16, winuser.h:6065:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub type PEN_FLAGS = ::basetsd::UINT32; /* winuser.h:6075:16, winuser.h:6075:16, winuser.h:6075:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub type PEN_MASK = ::basetsd::UINT32; /* winuser.h:6081:16, winuser.h:6081:16, winuser.h:6081:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub struct POINTER_PEN_INFO { pointerInfo: ::winuser::POINTER_INFO, penFlags: ::winuser::PEN_FLAGS, penMask: ::winuser::PEN_MASK, pressure: ::basetsd::UINT32, rotation: ::basetsd::UINT32, tiltX: ::basetsd::INT32, tiltY: ::basetsd::INT32 } /* winuser.h:6088:16, winuser.h:6088:16, winuser.h:6088:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub struct TOUCH_HIT_TESTING_PROXIMITY_EVALUATION { score: ::basetsd::UINT16, adjustedPoint: ::windef::POINT } /* winuser.h:6318:16, winuser.h:6318:16, winuser.h:6318:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub type PTOUCH_HIT_TESTING_PROXIMITY_EVALUATION = *mut ::winuser::TOUCH_HIT_TESTING_PROXIMITY_EVALUATION; /* winuser.h:6322:44, winuser.h:6322:44, winuser.h:6322:44 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub struct TOUCH_HIT_TESTING_INPUT { pointerId: ::basetsd::UINT32, point: ::windef::POINT, boundingBox: ::windef::RECT, nonOccludedBoundingBox: ::windef::RECT, orientation: ::basetsd::UINT32 } /* winuser.h:6328:16, winuser.h:6328:16, winuser.h:6328:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub type PTOUCH_HIT_TESTING_INPUT = *mut ::winuser::TOUCH_HIT_TESTING_INPUT; /* winuser.h:6335:29, winuser.h:6335:29, winuser.h:6335:29 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub enum FEEDBACK_TYPE {FEEDBACK_TOUCH_CONTACTVISUALIZATION = 1, FEEDBACK_PEN_BARRELVISUALIZATION = 2, FEEDBACK_PEN_TAP = 3, FEEDBACK_PEN_DOUBLETAP = 4, FEEDBACK_PEN_PRESSANDHOLD = 5, FEEDBACK_PEN_RIGHTTAP = 6, FEEDBACK_TOUCH_TAP = 7, FEEDBACK_TOUCH_DOUBLETAP = 8, FEEDBACK_TOUCH_PRESSANDHOLD = 9, FEEDBACK_TOUCH_RIGHTTAP = 10, FEEDBACK_GESTURE_PRESSANDTAP = 11, FEEDBACK_MAX = -1} pub use self::FEEDBACK_TYPE::{FEEDBACK_TOUCH_CONTACTVISUALIZATION, FEEDBACK_PEN_BARRELVISUALIZATION, FEEDBACK_PEN_TAP, FEEDBACK_PEN_DOUBLETAP, FEEDBACK_PEN_PRESSANDHOLD, FEEDBACK_PEN_RIGHTTAP, FEEDBACK_TOUCH_TAP, FEEDBACK_TOUCH_DOUBLETAP, FEEDBACK_TOUCH_PRESSANDHOLD, FEEDBACK_TOUCH_RIGHTTAP, FEEDBACK_GESTURE_PRESSANDTAP, FEEDBACK_MAX}; /* winuser.h:6365:14, winuser.h:6365:14, winuser.h:6365:14 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct LASTINPUTINFO { cbSize: ::minwindef::UINT, dwTime: ::minwindef::DWORD } /* winuser.h:6451:16, winuser.h:6451:16, winuser.h:6451:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PLASTINPUTINFO = *mut ::winuser::LASTINPUTINFO; /* winuser.h:6454:20, winuser.h:6454:20, winuser.h:6454:20 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct TPMPARAMS { cbSize: ::minwindef::UINT, rcExclude: ::windef::RECT } /* winuser.h:7271:16, winuser.h:7271:16, winuser.h:7271:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPTPMPARAMS = *mut ::winuser::TPMPARAMS; /* winuser.h:7276:24, winuser.h:7276:24, winuser.h:7276:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct MENUINFO { cbSize: ::minwindef::DWORD, fMask: ::minwindef::DWORD, dwStyle: ::minwindef::DWORD, cyMax: ::minwindef::UINT, hbrBack: ::windef::HBRUSH, dwContextHelpID: ::minwindef::DWORD, dwMenuData: ::basetsd::ULONG_PTR } /* winuser.h:7319:16, winuser.h:7319:16, winuser.h:7319:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPMENUINFO = *mut ::winuser::MENUINFO; /* winuser.h:7328:20, winuser.h:7328:20, winuser.h:7328:20 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPCMENUINFO = *const ::winuser::MENUINFO; /* winuser.h:7329:29, winuser.h:7329:29, winuser.h:7329:29 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct MENUGETOBJECTINFO { dwFlags: ::minwindef::DWORD, uPos: ::minwindef::UINT, hmenu: ::windef::HMENU, riid: ::winnt::PVOID, pvObj: ::winnt::PVOID } /* winuser.h:7357:16, winuser.h:7357:16, winuser.h:7357:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PMENUGETOBJECTINFO = *mut ::winuser::MENUGETOBJECTINFO; /* winuser.h:7364:24, winuser.h:7364:24, winuser.h:7364:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct MENUITEMINFOA { cbSize: ::minwindef::UINT, fMask: ::minwindef::UINT, fType: ::minwindef::UINT, fState: ::minwindef::UINT, wID: ::minwindef::UINT, hSubMenu: ::windef::HMENU, hbmpChecked: ::windef::HBITMAP, hbmpUnchecked: ::windef::HBITMAP, dwItemData: ::basetsd::ULONG_PTR, dwTypeData: ::winnt::LPSTR, cch: ::minwindef::UINT, hbmpItem: ::windef::HBITMAP } /* winuser.h:7407:16, winuser.h:7407:16, winuser.h:7407:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPMENUITEMINFOA = *mut ::winuser::MENUITEMINFOA; /* winuser.h:7423:25, winuser.h:7423:25, winuser.h:7423:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct MENUITEMINFOW { cbSize: ::minwindef::UINT, fMask: ::minwindef::UINT, fType: ::minwindef::UINT, fState: ::minwindef::UINT, wID: ::minwindef::UINT, hSubMenu: ::windef::HMENU, hbmpChecked: ::windef::HBITMAP, hbmpUnchecked: ::windef::HBITMAP, dwItemData: ::basetsd::ULONG_PTR, dwTypeData: ::winnt::LPWSTR, cch: ::minwindef::UINT, hbmpItem: ::windef::HBITMAP } /* winuser.h:7424:16, winuser.h:7424:16, winuser.h:7424:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPMENUITEMINFOW = *mut ::winuser::MENUITEMINFOW; /* winuser.h:7440:25, winuser.h:7440:25, winuser.h:7440:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type MENUITEMINFO = ::winuser::MENUITEMINFOW; /* winuser.h:7442:23, winuser.h:7442:23, winuser.h:7442:23 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPMENUITEMINFO = ::winuser::LPMENUITEMINFOW; /* winuser.h:7443:25, winuser.h:7443:25, winuser.h:7443:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPCMENUITEMINFOA = *const ::winuser::MENUITEMINFOA; /* winuser.h:7448:34, winuser.h:7448:34, winuser.h:7448:34 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPCMENUITEMINFOW = *const ::winuser::MENUITEMINFOW; /* winuser.h:7449:34, winuser.h:7449:34, winuser.h:7449:34 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPCMENUITEMINFO = ::winuser::LPCMENUITEMINFOW; /* winuser.h:7451:26, winuser.h:7451:26, winuser.h:7451:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct DROPSTRUCT { hwndSource: ::windef::HWND, hwndSink: ::windef::HWND, wFmt: ::minwindef::DWORD, dwData: ::basetsd::ULONG_PTR, ptDrop: ::windef::POINT, dwControlData: ::minwindef::DWORD } /* winuser.h:7612:16, winuser.h:7612:16, winuser.h:7612:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PDROPSTRUCT = *mut ::winuser::DROPSTRUCT; /* winuser.h:7620:16, winuser.h:7620:16, winuser.h:7620:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPDROPSTRUCT = *mut ::winuser::DROPSTRUCT; /* winuser.h:7620:30, winuser.h:7620:30, winuser.h:7620:30 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct DRAWTEXTPARAMS { cbSize: ::minwindef::UINT, iTabLength: ::libc::c_int, iLeftMargin: ::libc::c_int, iRightMargin: ::libc::c_int, uiLengthDrawn: ::minwindef::UINT } /* winuser.h:7714:16, winuser.h:7714:16, winuser.h:7714:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPDRAWTEXTPARAMS = *mut ::winuser::DRAWTEXTPARAMS; /* winuser.h:7721:24, winuser.h:7721:24, winuser.h:7721:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct HELPINFO { cbSize: ::minwindef::UINT, iContextType: ::libc::c_int, iCtrlId: ::libc::c_int, hItemHandle: ::winnt::HANDLE, dwContextId: ::basetsd::DWORD_PTR, MousePos: ::windef::POINT } /* winuser.h:8571:16, winuser.h:8571:16, winuser.h:8571:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPHELPINFO = *mut ::winuser::HELPINFO; /* winuser.h:8579:19, winuser.h:8579:19, winuser.h:8579:19 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type MSGBOXCALLBACK = extern "system" fn(*mut ::winuser::HELPINFO); /* winuser.h:8758:25, winuser.h:8758:25, winuser.h:8758:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct MSGBOXPARAMSA { cbSize: ::minwindef::UINT, hwndOwner: ::windef::HWND, hInstance: ::minwindef::HINSTANCE, lpszText: ::winnt::LPCSTR, lpszCaption: ::winnt::LPCSTR, dwStyle: ::minwindef::DWORD, lpszIcon: ::winnt::LPCSTR, dwContextHelpId: ::basetsd::DWORD_PTR, lpfnMsgBoxCallback: ::winuser::MSGBOXCALLBACK, dwLanguageId: ::minwindef::DWORD } /* winuser.h:8760:16, winuser.h:8760:16, winuser.h:8760:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PMSGBOXPARAMSA = *mut ::winuser::MSGBOXPARAMSA; /* winuser.h:8772:19, winuser.h:8772:19, winuser.h:8772:19 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPMSGBOXPARAMSA = *mut ::winuser::MSGBOXPARAMSA; /* winuser.h:8772:36, winuser.h:8772:36, winuser.h:8772:36 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct MSGBOXPARAMSW { cbSize: ::minwindef::UINT, hwndOwner: ::windef::HWND, hInstance: ::minwindef::HINSTANCE, lpszText: ::winnt::LPCWSTR, lpszCaption: ::winnt::LPCWSTR, dwStyle: ::minwindef::DWORD, lpszIcon: ::winnt::LPCWSTR, dwContextHelpId: ::basetsd::DWORD_PTR, lpfnMsgBoxCallback: ::winuser::MSGBOXCALLBACK, dwLanguageId: ::minwindef::DWORD } /* winuser.h:8773:16, winuser.h:8773:16, winuser.h:8773:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PMSGBOXPARAMSW = *mut ::winuser::MSGBOXPARAMSW; /* winuser.h:8785:19, winuser.h:8785:19, winuser.h:8785:19 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPMSGBOXPARAMSW = *mut ::winuser::MSGBOXPARAMSW; /* winuser.h:8785:36, winuser.h:8785:36, winuser.h:8785:36 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type MSGBOXPARAMS = ::winuser::MSGBOXPARAMSW; /* winuser.h:8787:23, winuser.h:8787:23, winuser.h:8787:23 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PMSGBOXPARAMS = ::winuser::PMSGBOXPARAMSW; /* winuser.h:8788:24, winuser.h:8788:24, winuser.h:8788:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPMSGBOXPARAMS = ::winuser::LPMSGBOXPARAMSW; /* winuser.h:8789:25, winuser.h:8789:25, winuser.h:8789:25 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct MENUITEMTEMPLATEHEADER { versionNumber: ::minwindef::WORD, offset: ::minwindef::WORD } /* winuser.h:9919:9, winuser.h:9919:9, winuser.h:9919:9 */
#[cfg(feature="winapi_desktop")] pub type PMENUITEMTEMPLATEHEADER = *mut ::winuser::MENUITEMTEMPLATEHEADER; /* winuser.h:9922:28, winuser.h:9922:28, winuser.h:9922:28 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct MENUITEMTEMPLATE { mtOption: ::minwindef::WORD, mtID: ::minwindef::WORD, mtString: *mut [::winnt::WCHAR; 1] } /* winuser.h:9924:9, winuser.h:9924:9, winuser.h:9924:9 */
#[cfg(feature="winapi_desktop")] pub type PMENUITEMTEMPLATE = *mut ::winuser::MENUITEMTEMPLATE; /* winuser.h:9928:22, winuser.h:9928:22, winuser.h:9928:22 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct ICONINFO { fIcon: ::minwindef::BOOL, xHotspot: ::minwindef::DWORD, yHotspot: ::minwindef::DWORD, hbmMask: ::windef::HBITMAP, hbmColor: ::windef::HBITMAP } /* winuser.h:10102:16, winuser.h:10102:16, winuser.h:10102:16 */
#[cfg(feature="winapi_desktop")] pub type PICONINFO = *mut ::winuser::ICONINFO; /* winuser.h:10109:19, winuser.h:10109:19, winuser.h:10109:19 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct CURSORSHAPE { xHotSpot: ::libc::c_int, yHotSpot: ::libc::c_int, cx: ::libc::c_int, cy: ::libc::c_int, cbWidth: ::libc::c_int, Planes: ::minwindef::BYTE, BitsPixel: ::minwindef::BYTE } /* winuser.h:10220:16, winuser.h:10220:16, winuser.h:10220:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPCURSORSHAPE = *mut ::winuser::CURSORSHAPE; /* winuser.h:10229:21, winuser.h:10229:21, winuser.h:10229:21 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[repr(C)] pub struct ICONINFOEXA { cbSize: ::minwindef::DWORD, fIcon: ::minwindef::BOOL, xHotspot: ::minwindef::DWORD, yHotspot: ::minwindef::DWORD, hbmMask: ::windef::HBITMAP, hbmColor: ::windef::HBITMAP, wResID: ::minwindef::WORD, szModName: *mut [::winnt::CHAR; 260], szResName: *mut [::winnt::CHAR; 260] } /* winuser.h:10342:16, winuser.h:10342:16, winuser.h:10342:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] pub type PICONINFOEXA = *mut ::winuser::ICONINFOEXA; /* winuser.h:10352:17, winuser.h:10352:17, winuser.h:10352:17 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[repr(C)] pub struct ICONINFOEXW { cbSize: ::minwindef::DWORD, fIcon: ::minwindef::BOOL, xHotspot: ::minwindef::DWORD, yHotspot: ::minwindef::DWORD, hbmMask: ::windef::HBITMAP, hbmColor: ::windef::HBITMAP, wResID: ::minwindef::WORD, szModName: *mut [::winnt::WCHAR; 260], szResName: *mut [::winnt::WCHAR; 260] } /* winuser.h:10353:16, winuser.h:10353:16, winuser.h:10353:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] pub type PICONINFOEXW = *mut ::winuser::ICONINFOEXW; /* winuser.h:10363:17, winuser.h:10363:17, winuser.h:10363:17 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] pub type ICONINFOEX = ::winuser::ICONINFOEXW; /* winuser.h:10365:21, winuser.h:10365:21, winuser.h:10365:21 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] pub type PICONINFOEX = ::winuser::PICONINFOEXW; /* winuser.h:10366:22, winuser.h:10366:22, winuser.h:10366:22 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct SCROLLINFO { cbSize: ::minwindef::UINT, fMask: ::minwindef::UINT, nMin: ::libc::c_int, nMax: ::libc::c_int, nPage: ::minwindef::UINT, nPos: ::libc::c_int, nTrackPos: ::libc::c_int } /* winuser.h:11373:16, winuser.h:11373:16, winuser.h:11373:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPSCROLLINFO = *mut ::winuser::SCROLLINFO; /* winuser.h:11382:22, winuser.h:11382:22, winuser.h:11382:22 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPCSCROLLINFO = *const ::winuser::SCROLLINFO; /* winuser.h:11383:31, winuser.h:11383:31, winuser.h:11383:31 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct MDICREATESTRUCTA { szClass: ::winnt::LPCSTR, szTitle: ::winnt::LPCSTR, hOwner: ::winnt::HANDLE, x: ::libc::c_int, y: ::libc::c_int, cx: ::libc::c_int, cy: ::libc::c_int, style: ::minwindef::DWORD, lParam: ::minwindef::LPARAM } /* winuser.h:11430:16, winuser.h:11430:16, winuser.h:11430:16 */
#[cfg(feature="winapi_desktop")] pub type LPMDICREATESTRUCTA = *mut ::winuser::MDICREATESTRUCTA; /* winuser.h:11440:22, winuser.h:11440:22, winuser.h:11440:22 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct MDICREATESTRUCTW { szClass: ::winnt::LPCWSTR, szTitle: ::winnt::LPCWSTR, hOwner: ::winnt::HANDLE, x: ::libc::c_int, y: ::libc::c_int, cx: ::libc::c_int, cy: ::libc::c_int, style: ::minwindef::DWORD, lParam: ::minwindef::LPARAM } /* winuser.h:11441:16, winuser.h:11441:16, winuser.h:11441:16 */
#[cfg(feature="winapi_desktop")] pub type LPMDICREATESTRUCTW = *mut ::winuser::MDICREATESTRUCTW; /* winuser.h:11451:22, winuser.h:11451:22, winuser.h:11451:22 */
#[cfg(feature="winapi_desktop")] pub type MDICREATESTRUCT = ::winuser::MDICREATESTRUCTW; /* winuser.h:11453:26, winuser.h:11453:26, winuser.h:11453:26 */
#[cfg(feature="winapi_desktop")] pub type LPMDICREATESTRUCT = ::winuser::LPMDICREATESTRUCTW; /* winuser.h:11454:28, winuser.h:11454:28, winuser.h:11454:28 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct CLIENTCREATESTRUCT { hWindowMenu: ::winnt::HANDLE, idFirstChild: ::minwindef::UINT } /* winuser.h:11460:16, winuser.h:11460:16, winuser.h:11460:16 */
#[cfg(feature="winapi_desktop")] pub type LPCLIENTCREATESTRUCT = *mut ::winuser::CLIENTCREATESTRUCT; /* winuser.h:11463:24, winuser.h:11463:24, winuser.h:11463:24 */
#[cfg(feature="winapi_desktop")] pub type HELPPOLY = ::minwindef::DWORD; /* winuser.h:11608:15, winuser.h:11608:15, winuser.h:11608:15 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct MULTIKEYHELPA { mkSize: ::minwindef::DWORD, mkKeylist: ::winnt::CHAR, szKeyphrase: *mut [::winnt::CHAR; 1] } /* winuser.h:11609:16, winuser.h:11609:16, winuser.h:11609:16 */
#[cfg(feature="winapi_desktop")] pub type PMULTIKEYHELPA = *mut ::winuser::MULTIKEYHELPA; /* winuser.h:11617:19, winuser.h:11617:19, winuser.h:11617:19 */
#[cfg(feature="winapi_desktop")] pub type LPMULTIKEYHELPA = *mut ::winuser::MULTIKEYHELPA; /* winuser.h:11617:36, winuser.h:11617:36, winuser.h:11617:36 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct MULTIKEYHELPW { mkSize: ::minwindef::DWORD, mkKeylist: ::winnt::WCHAR, szKeyphrase: *mut [::winnt::WCHAR; 1] } /* winuser.h:11618:16, winuser.h:11618:16, winuser.h:11618:16 */
#[cfg(feature="winapi_desktop")] pub type PMULTIKEYHELPW = *mut ::winuser::MULTIKEYHELPW; /* winuser.h:11626:19, winuser.h:11626:19, winuser.h:11626:19 */
#[cfg(feature="winapi_desktop")] pub type LPMULTIKEYHELPW = *mut ::winuser::MULTIKEYHELPW; /* winuser.h:11626:36, winuser.h:11626:36, winuser.h:11626:36 */
#[cfg(feature="winapi_desktop")] pub type MULTIKEYHELP = ::winuser::MULTIKEYHELPW; /* winuser.h:11628:23, winuser.h:11628:23, winuser.h:11628:23 */
#[cfg(feature="winapi_desktop")] pub type PMULTIKEYHELP = ::winuser::PMULTIKEYHELPW; /* winuser.h:11629:24, winuser.h:11629:24, winuser.h:11629:24 */
#[cfg(feature="winapi_desktop")] pub type LPMULTIKEYHELP = ::winuser::LPMULTIKEYHELPW; /* winuser.h:11630:25, winuser.h:11630:25, winuser.h:11630:25 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct HELPWININFOA { wStructSize: ::libc::c_int, x: ::libc::c_int, y: ::libc::c_int, dx: ::libc::c_int, dy: ::libc::c_int, wMax: ::libc::c_int, rgchMember: *mut [::winnt::CHAR; 2] } /* winuser.h:11637:16, winuser.h:11637:16, winuser.h:11637:16 */
#[cfg(feature="winapi_desktop")] pub type PHELPWININFOA = *mut ::winuser::HELPWININFOA; /* winuser.h:11645:18, winuser.h:11645:18, winuser.h:11645:18 */
#[cfg(feature="winapi_desktop")] pub type LPHELPWININFOA = *mut ::winuser::HELPWININFOA; /* winuser.h:11645:34, winuser.h:11645:34, winuser.h:11645:34 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct HELPWININFOW { wStructSize: ::libc::c_int, x: ::libc::c_int, y: ::libc::c_int, dx: ::libc::c_int, dy: ::libc::c_int, wMax: ::libc::c_int, rgchMember: *mut [::winnt::WCHAR; 2] } /* winuser.h:11646:16, winuser.h:11646:16, winuser.h:11646:16 */
#[cfg(feature="winapi_desktop")] pub type PHELPWININFOW = *mut ::winuser::HELPWININFOW; /* winuser.h:11654:18, winuser.h:11654:18, winuser.h:11654:18 */
#[cfg(feature="winapi_desktop")] pub type LPHELPWININFOW = *mut ::winuser::HELPWININFOW; /* winuser.h:11654:34, winuser.h:11654:34, winuser.h:11654:34 */
#[cfg(feature="winapi_desktop")] pub type HELPWININFO = ::winuser::HELPWININFOW; /* winuser.h:11656:22, winuser.h:11656:22, winuser.h:11656:22 */
#[cfg(feature="winapi_desktop")] pub type PHELPWININFO = ::winuser::PHELPWININFOW; /* winuser.h:11657:23, winuser.h:11657:23, winuser.h:11657:23 */
#[cfg(feature="winapi_desktop")] pub type LPHELPWININFO = ::winuser::LPHELPWININFOW; /* winuser.h:11658:24, winuser.h:11658:24, winuser.h:11658:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub struct TOUCHPREDICTIONPARAMETERS { cbSize: ::minwindef::UINT, dwLatency: ::minwindef::UINT, dwSampleTime: ::minwindef::UINT, bUseHWTimeStamp: ::minwindef::UINT } /* winuser.h:11948:16, winuser.h:11948:16, winuser.h:11948:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub type PTOUCHPREDICTIONPARAMETERS = *mut ::winuser::TOUCHPREDICTIONPARAMETERS; /* winuser.h:11954:31, winuser.h:11954:31, winuser.h:11954:31 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct NONCLIENTMETRICSA { cbSize: ::minwindef::UINT, iBorderWidth: ::libc::c_int, iScrollWidth: ::libc::c_int, iScrollHeight: ::libc::c_int, iCaptionWidth: ::libc::c_int, iCaptionHeight: ::libc::c_int, lfCaptionFont: ::wingdi::LOGFONTA, iSmCaptionWidth: ::libc::c_int, iSmCaptionHeight: ::libc::c_int, lfSmCaptionFont: ::wingdi::LOGFONTA, iMenuWidth: ::libc::c_int, iMenuHeight: ::libc::c_int, lfMenuFont: ::wingdi::LOGFONTA, lfStatusFont: ::wingdi::LOGFONTA, lfMessageFont: ::wingdi::LOGFONTA, iPaddedBorderWidth: ::libc::c_int } /* winuser.h:12141:16, winuser.h:12141:16, winuser.h:12141:16 */
#[cfg(feature="winapi_desktop")] pub type PNONCLIENTMETRICSA = *mut ::winuser::NONCLIENTMETRICSA; /* winuser.h:12161:25, winuser.h:12161:25, winuser.h:12161:25 */
#[cfg(feature="winapi_desktop")] pub type LPNONCLIENTMETRICSA = *mut ::winuser::NONCLIENTMETRICSA; /* winuser.h:12161:50, winuser.h:12161:50, winuser.h:12161:50 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct NONCLIENTMETRICSW { cbSize: ::minwindef::UINT, iBorderWidth: ::libc::c_int, iScrollWidth: ::libc::c_int, iScrollHeight: ::libc::c_int, iCaptionWidth: ::libc::c_int, iCaptionHeight: ::libc::c_int, lfCaptionFont: ::wingdi::LOGFONTW, iSmCaptionWidth: ::libc::c_int, iSmCaptionHeight: ::libc::c_int, lfSmCaptionFont: ::wingdi::LOGFONTW, iMenuWidth: ::libc::c_int, iMenuHeight: ::libc::c_int, lfMenuFont: ::wingdi::LOGFONTW, lfStatusFont: ::wingdi::LOGFONTW, lfMessageFont: ::wingdi::LOGFONTW, iPaddedBorderWidth: ::libc::c_int } /* winuser.h:12162:16, winuser.h:12162:16, winuser.h:12162:16 */
#[cfg(feature="winapi_desktop")] pub type PNONCLIENTMETRICSW = *mut ::winuser::NONCLIENTMETRICSW; /* winuser.h:12182:25, winuser.h:12182:25, winuser.h:12182:25 */
#[cfg(feature="winapi_desktop")] pub type LPNONCLIENTMETRICSW = *mut ::winuser::NONCLIENTMETRICSW; /* winuser.h:12182:50, winuser.h:12182:50, winuser.h:12182:50 */
#[cfg(feature="winapi_desktop")] pub type NONCLIENTMETRICS = ::winuser::NONCLIENTMETRICSW; /* winuser.h:12184:27, winuser.h:12184:27, winuser.h:12184:27 */
#[cfg(feature="winapi_desktop")] pub type PNONCLIENTMETRICS = ::winuser::PNONCLIENTMETRICSW; /* winuser.h:12185:28, winuser.h:12185:28, winuser.h:12185:28 */
#[cfg(feature="winapi_desktop")] pub type LPNONCLIENTMETRICS = ::winuser::LPNONCLIENTMETRICSW; /* winuser.h:12186:29, winuser.h:12186:29, winuser.h:12186:29 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct MINIMIZEDMETRICS { cbSize: ::minwindef::UINT, iWidth: ::libc::c_int, iHorzGap: ::libc::c_int, iVertGap: ::libc::c_int, iArrange: ::libc::c_int } /* winuser.h:12216:16, winuser.h:12216:16, winuser.h:12216:16 */
#[cfg(feature="winapi_desktop")] pub type PMINIMIZEDMETRICS = *mut ::winuser::MINIMIZEDMETRICS; /* winuser.h:12223:24, winuser.h:12223:24, winuser.h:12223:24 */
#[cfg(feature="winapi_desktop")] pub type LPMINIMIZEDMETRICS = *mut ::winuser::MINIMIZEDMETRICS; /* winuser.h:12223:44, winuser.h:12223:44, winuser.h:12223:44 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct ICONMETRICSA { cbSize: ::minwindef::UINT, iHorzSpacing: ::libc::c_int, iVertSpacing: ::libc::c_int, iTitleWrap: ::libc::c_int, lfFont: ::wingdi::LOGFONTA } /* winuser.h:12227:16, winuser.h:12227:16, winuser.h:12227:16 */
#[cfg(feature="winapi_desktop")] pub type PICONMETRICSA = *mut ::winuser::ICONMETRICSA; /* winuser.h:12234:20, winuser.h:12234:20, winuser.h:12234:20 */
#[cfg(feature="winapi_desktop")] pub type LPICONMETRICSA = *mut ::winuser::ICONMETRICSA; /* winuser.h:12234:36, winuser.h:12234:36, winuser.h:12234:36 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct ICONMETRICSW { cbSize: ::minwindef::UINT, iHorzSpacing: ::libc::c_int, iVertSpacing: ::libc::c_int, iTitleWrap: ::libc::c_int, lfFont: ::wingdi::LOGFONTW } /* winuser.h:12235:16, winuser.h:12235:16, winuser.h:12235:16 */
#[cfg(feature="winapi_desktop")] pub type PICONMETRICSW = *mut ::winuser::ICONMETRICSW; /* winuser.h:12242:20, winuser.h:12242:20, winuser.h:12242:20 */
#[cfg(feature="winapi_desktop")] pub type LPICONMETRICSW = *mut ::winuser::ICONMETRICSW; /* winuser.h:12242:36, winuser.h:12242:36, winuser.h:12242:36 */
#[cfg(feature="winapi_desktop")] pub type ICONMETRICS = ::winuser::ICONMETRICSW; /* winuser.h:12244:22, winuser.h:12244:22, winuser.h:12244:22 */
#[cfg(feature="winapi_desktop")] pub type PICONMETRICS = ::winuser::PICONMETRICSW; /* winuser.h:12245:23, winuser.h:12245:23, winuser.h:12245:23 */
#[cfg(feature="winapi_desktop")] pub type LPICONMETRICS = ::winuser::LPICONMETRICSW; /* winuser.h:12246:24, winuser.h:12246:24, winuser.h:12246:24 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct ANIMATIONINFO { cbSize: ::minwindef::UINT, iMinAnimate: ::libc::c_int } /* winuser.h:12255:16, winuser.h:12255:16, winuser.h:12255:16 */
#[cfg(feature="winapi_desktop")] pub type LPANIMATIONINFO = *mut ::winuser::ANIMATIONINFO; /* winuser.h:12259:21, winuser.h:12259:21, winuser.h:12259:21 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERIALKEYSA { cbSize: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, lpszActivePort: ::winnt::LPSTR, lpszPort: ::winnt::LPSTR, iBaudRate: ::minwindef::UINT, iPortState: ::minwindef::UINT, iActive: ::minwindef::UINT } /* winuser.h:12261:16, winuser.h:12261:16, winuser.h:12261:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERIALKEYSA = *mut ::winuser::SERIALKEYSA; /* winuser.h:12270:19, winuser.h:12270:19, winuser.h:12270:19 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERIALKEYSW { cbSize: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, lpszActivePort: ::winnt::LPWSTR, lpszPort: ::winnt::LPWSTR, iBaudRate: ::minwindef::UINT, iPortState: ::minwindef::UINT, iActive: ::minwindef::UINT } /* winuser.h:12271:16, winuser.h:12271:16, winuser.h:12271:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERIALKEYSW = *mut ::winuser::SERIALKEYSW; /* winuser.h:12280:19, winuser.h:12280:19, winuser.h:12280:19 */
#[cfg(feature="winapi_desktop")] pub type SERIALKEYS = ::winuser::SERIALKEYSW; /* winuser.h:12282:21, winuser.h:12282:21, winuser.h:12282:21 */
#[cfg(feature="winapi_desktop")] pub type LPSERIALKEYS = ::winuser::LPSERIALKEYSW; /* winuser.h:12283:23, winuser.h:12283:23, winuser.h:12283:23 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct HIGHCONTRASTA { cbSize: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, lpszDefaultScheme: ::winnt::LPSTR } /* winuser.h:12295:16, winuser.h:12295:16, winuser.h:12295:16 */
#[cfg(feature="winapi_desktop")] pub type LPHIGHCONTRASTA = *mut ::winuser::HIGHCONTRASTA; /* winuser.h:12300:21, winuser.h:12300:21, winuser.h:12300:21 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct HIGHCONTRASTW { cbSize: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, lpszDefaultScheme: ::winnt::LPWSTR } /* winuser.h:12301:16, winuser.h:12301:16, winuser.h:12301:16 */
#[cfg(feature="winapi_desktop")] pub type LPHIGHCONTRASTW = *mut ::winuser::HIGHCONTRASTW; /* winuser.h:12306:21, winuser.h:12306:21, winuser.h:12306:21 */
#[cfg(feature="winapi_desktop")] pub type HIGHCONTRAST = ::winuser::HIGHCONTRASTW; /* winuser.h:12308:23, winuser.h:12308:23, winuser.h:12308:23 */
#[cfg(feature="winapi_desktop")] pub type LPHIGHCONTRAST = ::winuser::LPHIGHCONTRASTW; /* winuser.h:12309:25, winuser.h:12309:25, winuser.h:12309:25 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct FILTERKEYS { cbSize: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, iWaitMSec: ::minwindef::DWORD, iDelayMSec: ::minwindef::DWORD, iRepeatMSec: ::minwindef::DWORD, iBounceMSec: ::minwindef::DWORD } /* winuser.h:12572:16, winuser.h:12572:16, winuser.h:12572:16 */
#[cfg(feature="winapi_desktop")] pub type LPFILTERKEYS = *mut ::winuser::FILTERKEYS; /* winuser.h:12580:16, winuser.h:12580:16, winuser.h:12580:16 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct STICKYKEYS { cbSize: ::minwindef::UINT, dwFlags: ::minwindef::DWORD } /* winuser.h:12599:16, winuser.h:12599:16, winuser.h:12599:16 */
#[cfg(feature="winapi_desktop")] pub type LPSTICKYKEYS = *mut ::winuser::STICKYKEYS; /* winuser.h:12603:16, winuser.h:12603:16, winuser.h:12603:16 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct MOUSEKEYS { cbSize: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, iMaxSpeed: ::minwindef::DWORD, iTimeToMaxSpeed: ::minwindef::DWORD, iCtrlSpeed: ::minwindef::DWORD, dwReserved1: ::minwindef::DWORD, dwReserved2: ::minwindef::DWORD } /* winuser.h:12642:16, winuser.h:12642:16, winuser.h:12642:16 */
#[cfg(feature="winapi_desktop")] pub type LPMOUSEKEYS = *mut ::winuser::MOUSEKEYS; /* winuser.h:12651:15, winuser.h:12651:15, winuser.h:12651:15 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct ACCESSTIMEOUT { cbSize: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, iTimeOutMSec: ::minwindef::DWORD } /* winuser.h:12678:16, winuser.h:12678:16, winuser.h:12678:16 */
#[cfg(feature="winapi_desktop")] pub type LPACCESSTIMEOUT = *mut ::winuser::ACCESSTIMEOUT; /* winuser.h:12683:19, winuser.h:12683:19, winuser.h:12683:19 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SOUNDSENTRYA { cbSize: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, iFSTextEffect: ::minwindef::DWORD, iFSTextEffectMSec: ::minwindef::DWORD, iFSTextEffectColorBits: ::minwindef::DWORD, iFSGrafEffect: ::minwindef::DWORD, iFSGrafEffectMSec: ::minwindef::DWORD, iFSGrafEffectColor: ::minwindef::DWORD, iWindowsEffect: ::minwindef::DWORD, iWindowsEffectMSec: ::minwindef::DWORD, lpszWindowsEffectDLL: ::winnt::LPSTR, iWindowsEffectOrdinal: ::minwindef::DWORD } /* winuser.h:12714:16, winuser.h:12714:16, winuser.h:12714:16 */
#[cfg(feature="winapi_desktop")] pub type LPSOUNDSENTRYA = *mut ::winuser::SOUNDSENTRYA; /* winuser.h:12728:18, winuser.h:12728:18, winuser.h:12728:18 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SOUNDSENTRYW { cbSize: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, iFSTextEffect: ::minwindef::DWORD, iFSTextEffectMSec: ::minwindef::DWORD, iFSTextEffectColorBits: ::minwindef::DWORD, iFSGrafEffect: ::minwindef::DWORD, iFSGrafEffectMSec: ::minwindef::DWORD, iFSGrafEffectColor: ::minwindef::DWORD, iWindowsEffect: ::minwindef::DWORD, iWindowsEffectMSec: ::minwindef::DWORD, lpszWindowsEffectDLL: ::winnt::LPWSTR, iWindowsEffectOrdinal: ::minwindef::DWORD } /* winuser.h:12729:16, winuser.h:12729:16, winuser.h:12729:16 */
#[cfg(feature="winapi_desktop")] pub type LPSOUNDSENTRYW = *mut ::winuser::SOUNDSENTRYW; /* winuser.h:12743:18, winuser.h:12743:18, winuser.h:12743:18 */
#[cfg(feature="winapi_desktop")] pub type SOUNDSENTRY = ::winuser::SOUNDSENTRYW; /* winuser.h:12745:22, winuser.h:12745:22, winuser.h:12745:22 */
#[cfg(feature="winapi_desktop")] pub type LPSOUNDSENTRY = ::winuser::LPSOUNDSENTRYW; /* winuser.h:12746:24, winuser.h:12746:24, winuser.h:12746:24 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct TOGGLEKEYS { cbSize: ::minwindef::UINT, dwFlags: ::minwindef::DWORD } /* winuser.h:12772:16, winuser.h:12772:16, winuser.h:12772:16 */
#[cfg(feature="winapi_desktop")] pub type LPTOGGLEKEYS = *mut ::winuser::TOGGLEKEYS; /* winuser.h:12776:16, winuser.h:12776:16, winuser.h:12776:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[repr(C)] pub struct AUDIODESCRIPTION { cbSize: ::minwindef::UINT, Enabled: ::minwindef::BOOL, Locale: ::winnt::LCID } /* winuser.h:12795:16, winuser.h:12795:16, winuser.h:12795:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] pub type LPAUDIODESCRIPTION = *mut ::winuser::AUDIODESCRIPTION; /* winuser.h:12799:22, winuser.h:12799:22, winuser.h:12799:22 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct MONITORINFO { cbSize: ::minwindef::DWORD, rcMonitor: ::windef::RECT, rcWork: ::windef::RECT, dwFlags: ::minwindef::DWORD } /* winuser.h:12909:16, winuser.h:12909:16, winuser.h:12909:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPMONITORINFO = *mut ::winuser::MONITORINFO; /* winuser.h:12915:17, winuser.h:12915:17, winuser.h:12915:17 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct MONITORINFOEXA { szDevice: *mut [::winnt::CHAR; 32] } /* winuser.h:12934:16, winuser.h:12934:16, winuser.h:12934:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPMONITORINFOEXA = *mut ::winuser::MONITORINFOEXA; /* winuser.h:12938:20, winuser.h:12938:20, winuser.h:12938:20 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct MONITORINFOEXW { szDevice: *mut [::winnt::WCHAR; 32] } /* winuser.h:12939:16, winuser.h:12939:16, winuser.h:12939:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPMONITORINFOEXW = *mut ::winuser::MONITORINFOEXW; /* winuser.h:12943:20, winuser.h:12943:20, winuser.h:12943:20 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type MONITORINFOEX = ::winuser::MONITORINFOEXW; /* winuser.h:12945:24, winuser.h:12945:24, winuser.h:12945:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPMONITORINFOEX = ::winuser::LPMONITORINFOEXW; /* winuser.h:12946:26, winuser.h:12946:26, winuser.h:12946:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type MONITORENUMPROC = extern "system" fn(*mut ::windef::HMONITOR__, *mut ::windef::HDC__, *mut ::windef::RECT, ::libc::c_long) -> ::libc::c_int; /* winuser.h:12971:25, winuser.h:12971:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type WINEVENTPROC = extern "system" fn(*mut ::windef::HWINEVENTHOOK__, ::libc::c_ulong, *mut ::windef::HWND__, ::libc::c_long, ::libc::c_long, ::libc::c_ulong, ::libc::c_ulong); /* winuser.h:13003:25, winuser.h:13003:25, winuser.h:13003:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct GUITHREADINFO { cbSize: ::minwindef::DWORD, flags: ::minwindef::DWORD, hwndActive: ::windef::HWND, hwndFocus: ::windef::HWND, hwndCapture: ::windef::HWND, hwndMenuOwner: ::windef::HWND, hwndMoveSize: ::windef::HWND, hwndCaret: ::windef::HWND, rcCaret: ::windef::RECT } /* winuser.h:13664:16, winuser.h:13664:16, winuser.h:13664:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PGUITHREADINFO = *mut ::winuser::GUITHREADINFO; /* winuser.h:13675:19, winuser.h:13675:19, winuser.h:13675:19 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPGUITHREADINFO = *mut ::winuser::GUITHREADINFO; /* winuser.h:13675:41, winuser.h:13675:41, winuser.h:13675:41 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct CURSORINFO { cbSize: ::minwindef::DWORD, flags: ::minwindef::DWORD, hCursor: ::windef::HCURSOR, ptScreenPos: ::windef::POINT } /* winuser.h:13794:16, winuser.h:13794:16, winuser.h:13794:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PCURSORINFO = *mut ::winuser::CURSORINFO; /* winuser.h:13800:16, winuser.h:13800:16, winuser.h:13800:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPCURSORINFO = *mut ::winuser::CURSORINFO; /* winuser.h:13800:30, winuser.h:13800:30, winuser.h:13800:30 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct WINDOWINFO { cbSize: ::minwindef::DWORD, rcWindow: ::windef::RECT, rcClient: ::windef::RECT, dwStyle: ::minwindef::DWORD, dwExStyle: ::minwindef::DWORD, dwWindowStatus: ::minwindef::DWORD, cxWindowBorders: ::minwindef::UINT, cyWindowBorders: ::minwindef::UINT, atomWindowType: ::minwindef::ATOM, wCreatorVersion: ::minwindef::WORD } /* winuser.h:13816:16, winuser.h:13816:16, winuser.h:13816:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PWINDOWINFO = *mut ::winuser::WINDOWINFO; /* winuser.h:13828:16, winuser.h:13828:16, winuser.h:13828:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPWINDOWINFO = *mut ::winuser::WINDOWINFO; /* winuser.h:13828:30, winuser.h:13828:30, winuser.h:13828:30 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct TITLEBARINFO { cbSize: ::minwindef::DWORD, rcTitleBar: ::windef::RECT, rgstate: *mut [::minwindef::DWORD; 6] } /* winuser.h:13842:16, winuser.h:13842:16, winuser.h:13842:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PTITLEBARINFO = *mut ::winuser::TITLEBARINFO; /* winuser.h:13847:18, winuser.h:13847:18, winuser.h:13847:18 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPTITLEBARINFO = *mut ::winuser::TITLEBARINFO; /* winuser.h:13847:34, winuser.h:13847:34, winuser.h:13847:34 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[repr(C)] pub struct TITLEBARINFOEX { cbSize: ::minwindef::DWORD, rcTitleBar: ::windef::RECT, rgstate: *mut [::minwindef::DWORD; 6], rgrect: *mut [::windef::RECT; 6] } /* winuser.h:13857:16, winuser.h:13857:16, winuser.h:13857:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] pub type PTITLEBARINFOEX = *mut ::winuser::TITLEBARINFOEX; /* winuser.h:13863:20, winuser.h:13863:20, winuser.h:13863:20 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] pub type LPTITLEBARINFOEX = *mut ::winuser::TITLEBARINFOEX; /* winuser.h:13863:38, winuser.h:13863:38, winuser.h:13863:38 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct MENUBARINFO { cbSize: ::minwindef::DWORD, rcBar: ::windef::RECT, hMenu: ::windef::HMENU, hwndMenu: ::windef::HWND, fBarFocused: ::minwindef::BOOL, fFocused: ::minwindef::BOOL } /* winuser.h:13869:16, winuser.h:13869:16, winuser.h:13869:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PMENUBARINFO = *mut ::winuser::MENUBARINFO; /* winuser.h:13877:17, winuser.h:13877:17, winuser.h:13877:17 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPMENUBARINFO = *mut ::winuser::MENUBARINFO; /* winuser.h:13877:32, winuser.h:13877:32, winuser.h:13877:32 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct SCROLLBARINFO { cbSize: ::minwindef::DWORD, rcScrollBar: ::windef::RECT, dxyLineButton: ::libc::c_int, xyThumbTop: ::libc::c_int, xyThumbBottom: ::libc::c_int, reserved: ::libc::c_int, rgstate: *mut [::minwindef::DWORD; 6] } /* winuser.h:13891:16, winuser.h:13891:16, winuser.h:13891:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PSCROLLBARINFO = *mut ::winuser::SCROLLBARINFO; /* winuser.h:13900:19, winuser.h:13900:19, winuser.h:13900:19 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPSCROLLBARINFO = *mut ::winuser::SCROLLBARINFO; /* winuser.h:13900:36, winuser.h:13900:36, winuser.h:13900:36 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct COMBOBOXINFO { cbSize: ::minwindef::DWORD, rcItem: ::windef::RECT, rcButton: ::windef::RECT, stateButton: ::minwindef::DWORD, hwndCombo: ::windef::HWND, hwndItem: ::windef::HWND, hwndList: ::windef::HWND } /* winuser.h:13913:16, winuser.h:13913:16, winuser.h:13913:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PCOMBOBOXINFO = *mut ::winuser::COMBOBOXINFO; /* winuser.h:13922:18, winuser.h:13922:18, winuser.h:13922:18 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPCOMBOBOXINFO = *mut ::winuser::COMBOBOXINFO; /* winuser.h:13922:34, winuser.h:13922:34, winuser.h:13922:34 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct ALTTABINFO { cbSize: ::minwindef::DWORD, cItems: ::libc::c_int, cColumns: ::libc::c_int, cRows: ::libc::c_int, iColFocus: ::libc::c_int, iRowFocus: ::libc::c_int, cxItem: ::libc::c_int, cyItem: ::libc::c_int, ptStart: ::windef::POINT } /* winuser.h:13997:16, winuser.h:13997:16, winuser.h:13997:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PALTTABINFO = *mut ::winuser::ALTTABINFO; /* winuser.h:14008:16, winuser.h:14008:16, winuser.h:14008:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPALTTABINFO = *mut ::winuser::ALTTABINFO; /* winuser.h:14008:30, winuser.h:14008:30, winuser.h:14008:30 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct HRAWINPUT__ { unused: ::libc::c_int } /* winuser.h:14086:1, winuser.h:14086:1, winuser.h:14086:1 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type HRAWINPUT = *mut ::winuser::HRAWINPUT__; /* winuser.h:14086:1, winuser.h:14086:1, winuser.h:14086:1 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct RAWINPUTHEADER { dwType: ::minwindef::DWORD, dwSize: ::minwindef::DWORD, hDevice: ::winnt::HANDLE, wParam: ::minwindef::WPARAM } /* winuser.h:14119:16, winuser.h:14119:16, winuser.h:14119:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PRAWINPUTHEADER = *mut ::winuser::RAWINPUTHEADER; /* winuser.h:14124:20, winuser.h:14124:20, winuser.h:14124:20 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type LPRAWINPUTHEADER = *mut ::winuser::RAWINPUTHEADER; /* winuser.h:14124:38, winuser.h:14124:38, winuser.h:14124:38 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct RAWMOUSE { usFlags: ::minwindef::USHORT, ulRawButtons: ::minwindef::ULONG, lLastX: ::winnt::LONG, lLastY: ::winnt::LONG, ulExtraInformation: ::minwindef::ULONG } /* winuser.h:14142:16, winuser.h:14142:16, winuser.h:14142:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub /*union*/ struct RAWMOUSE_Child_1; /* STUB! */ /* winuser.h:14151:5, winuser.h:14151:5, winuser.h:14151:5 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PRAWMOUSE = *mut ::winuser::RAWMOUSE; /* winuser.h:14180:14, winuser.h:14180:14, winuser.h:14180:14 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type LPRAWMOUSE = *mut ::winuser::RAWMOUSE; /* winuser.h:14180:26, winuser.h:14180:26, winuser.h:14180:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct RAWKEYBOARD { MakeCode: ::minwindef::USHORT, Flags: ::minwindef::USHORT, Reserved: ::minwindef::USHORT, VKey: ::minwindef::USHORT, Message: ::minwindef::UINT, ExtraInformation: ::minwindef::ULONG } /* winuser.h:14231:16, winuser.h:14231:16, winuser.h:14231:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PRAWKEYBOARD = *mut ::winuser::RAWKEYBOARD; /* winuser.h:14257:17, winuser.h:14257:17, winuser.h:14257:17 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type LPRAWKEYBOARD = *mut ::winuser::RAWKEYBOARD; /* winuser.h:14257:32, winuser.h:14257:32, winuser.h:14257:32 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct RAWHID { dwSizeHid: ::minwindef::DWORD, dwCount: ::minwindef::DWORD, bRawData: *mut [::minwindef::BYTE; 1] } /* winuser.h:14284:16, winuser.h:14284:16, winuser.h:14284:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PRAWHID = *mut ::winuser::RAWHID; /* winuser.h:14288:12, winuser.h:14288:12, winuser.h:14288:12 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type LPRAWHID = *mut ::winuser::RAWHID; /* winuser.h:14288:22, winuser.h:14288:22, winuser.h:14288:22 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct RAWINPUT { header: ::winuser::RAWINPUTHEADER, data: ::winuser::RAWINPUT_Child_1 } /* winuser.h:14300:16, winuser.h:14300:16, winuser.h:14300:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub /*union*/ struct RAWINPUT_Child_1; /* STUB! */ /* winuser.h:14302:5, winuser.h:14302:5, winuser.h:14302:5 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PRAWINPUT = *mut ::winuser::RAWINPUT; /* winuser.h:14307:14, winuser.h:14307:14, winuser.h:14307:14 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type LPRAWINPUT = *mut ::winuser::RAWINPUT; /* winuser.h:14307:26, winuser.h:14307:26, winuser.h:14307:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct RID_DEVICE_INFO_MOUSE { dwId: ::minwindef::DWORD, dwNumberOfButtons: ::minwindef::DWORD, dwSampleRate: ::minwindef::DWORD, fHasHorizontalWheel: ::minwindef::BOOL } /* winuser.h:14353:16, winuser.h:14353:16, winuser.h:14353:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PRID_DEVICE_INFO_MOUSE = *mut ::winuser::RID_DEVICE_INFO_MOUSE; /* winuser.h:14358:27, winuser.h:14358:27, winuser.h:14358:27 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct RID_DEVICE_INFO_KEYBOARD { dwType: ::minwindef::DWORD, dwSubType: ::minwindef::DWORD, dwKeyboardMode: ::minwindef::DWORD, dwNumberOfFunctionKeys: ::minwindef::DWORD, dwNumberOfIndicators: ::minwindef::DWORD, dwNumberOfKeysTotal: ::minwindef::DWORD } /* winuser.h:14360:16, winuser.h:14360:16, winuser.h:14360:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PRID_DEVICE_INFO_KEYBOARD = *mut ::winuser::RID_DEVICE_INFO_KEYBOARD; /* winuser.h:14367:30, winuser.h:14367:30, winuser.h:14367:30 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct RID_DEVICE_INFO_HID { dwVendorId: ::minwindef::DWORD, dwProductId: ::minwindef::DWORD, dwVersionNumber: ::minwindef::DWORD, usUsagePage: ::minwindef::USHORT, usUsage: ::minwindef::USHORT } /* winuser.h:14369:16, winuser.h:14369:16, winuser.h:14369:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PRID_DEVICE_INFO_HID = *mut ::winuser::RID_DEVICE_INFO_HID; /* winuser.h:14379:25, winuser.h:14379:25, winuser.h:14379:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct RID_DEVICE_INFO { cbSize: ::minwindef::DWORD, dwType: ::minwindef::DWORD } /* winuser.h:14381:16, winuser.h:14381:16, winuser.h:14381:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub /*union*/ struct RID_DEVICE_INFO_Child_2; /* STUB! */ /* winuser.h:14384:5, winuser.h:14384:5, winuser.h:14384:5 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PRID_DEVICE_INFO = *mut ::winuser::RID_DEVICE_INFO; /* winuser.h:14389:21, winuser.h:14389:21, winuser.h:14389:21 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type LPRID_DEVICE_INFO = *mut ::winuser::RID_DEVICE_INFO; /* winuser.h:14389:40, winuser.h:14389:40, winuser.h:14389:40 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct RAWINPUTDEVICE { usUsagePage: ::minwindef::USHORT, usUsage: ::minwindef::USHORT, dwFlags: ::minwindef::DWORD, hwndTarget: ::windef::HWND } /* winuser.h:14428:16, winuser.h:14428:16, winuser.h:14428:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PRAWINPUTDEVICE = *mut ::winuser::RAWINPUTDEVICE; /* winuser.h:14433:20, winuser.h:14433:20, winuser.h:14433:20 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type LPRAWINPUTDEVICE = *mut ::winuser::RAWINPUTDEVICE; /* winuser.h:14433:38, winuser.h:14433:38, winuser.h:14433:38 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PCRAWINPUTDEVICE = *const ::winuser::RAWINPUTDEVICE; /* winuser.h:14435:31, winuser.h:14435:31, winuser.h:14435:31 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct RAWINPUTDEVICELIST { hDevice: ::winnt::HANDLE, dwType: ::minwindef::DWORD } /* winuser.h:14490:16, winuser.h:14490:16, winuser.h:14490:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PRAWINPUTDEVICELIST = *mut ::winuser::RAWINPUTDEVICELIST; /* winuser.h:14493:24, winuser.h:14493:24, winuser.h:14493:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub enum POINTER_DEVICE_TYPE {POINTER_DEVICE_TYPE_INTEGRATED_PEN = 1, POINTER_DEVICE_TYPE_EXTERNAL_PEN = 2, POINTER_DEVICE_TYPE_TOUCH = 3, POINTER_DEVICE_TYPE_MAX = -1} pub use self::POINTER_DEVICE_TYPE::{POINTER_DEVICE_TYPE_INTEGRATED_PEN, POINTER_DEVICE_TYPE_EXTERNAL_PEN, POINTER_DEVICE_TYPE_TOUCH, POINTER_DEVICE_TYPE_MAX}; /* winuser.h:14539:14, winuser.h:14539:14, winuser.h:14539:14 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub struct POINTER_DEVICE_INFO { displayOrientation: ::minwindef::DWORD, device: ::winnt::HANDLE, pointerDeviceType: ::winuser::POINTER_DEVICE_TYPE, monitor: ::windef::HMONITOR, startingCursorId: ::minwindef::ULONG, maxActiveContacts: ::minwindef::USHORT, productString: *mut [::winnt::WCHAR; 520] } /* winuser.h:14549:16, winuser.h:14549:16, winuser.h:14549:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub struct POINTER_DEVICE_PROPERTY { logicalMin: ::basetsd::INT32, logicalMax: ::basetsd::INT32, physicalMin: ::basetsd::INT32, physicalMax: ::basetsd::INT32, unit: ::basetsd::UINT32, unitExponent: ::basetsd::UINT32, usagePageId: ::minwindef::USHORT, usageId: ::minwindef::USHORT } /* winuser.h:14559:16, winuser.h:14559:16, winuser.h:14559:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub enum POINTER_DEVICE_CURSOR_TYPE {POINTER_DEVICE_CURSOR_TYPE_UNKNOWN = 0, POINTER_DEVICE_CURSOR_TYPE_TIP = 1, POINTER_DEVICE_CURSOR_TYPE_ERASER = 2, POINTER_DEVICE_CURSOR_TYPE_MAX = -1} pub use self::POINTER_DEVICE_CURSOR_TYPE::{POINTER_DEVICE_CURSOR_TYPE_UNKNOWN, POINTER_DEVICE_CURSOR_TYPE_TIP, POINTER_DEVICE_CURSOR_TYPE_ERASER, POINTER_DEVICE_CURSOR_TYPE_MAX}; /* winuser.h:14570:14, winuser.h:14570:14, winuser.h:14570:14 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub struct POINTER_DEVICE_CURSOR_INFO { cursorId: ::basetsd::UINT32, cursor: ::winuser::POINTER_DEVICE_CURSOR_TYPE } /* winuser.h:14577:16, winuser.h:14577:16, winuser.h:14577:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct CHANGEFILTERSTRUCT { cbSize: ::minwindef::DWORD, ExtStatus: ::minwindef::DWORD } /* winuser.h:14681:16, winuser.h:14681:16, winuser.h:14681:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub type PCHANGEFILTERSTRUCT = *mut ::winuser::CHANGEFILTERSTRUCT; /* winuser.h:14684:24, winuser.h:14684:24, winuser.h:14684:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct HGESTUREINFO__ { unused: ::libc::c_int } /* winuser.h:14728:1, winuser.h:14728:1, winuser.h:14728:1 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub type HGESTUREINFO = *mut ::winuser::HGESTUREINFO__; /* winuser.h:14728:1, winuser.h:14728:1, winuser.h:14728:1 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct GESTUREINFO { cbSize: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, dwID: ::minwindef::DWORD, hwndTarget: ::windef::HWND, ptsLocation: ::windef::POINTS, dwInstanceID: ::minwindef::DWORD, dwSequenceID: ::minwindef::DWORD, ullArguments: ::winnt::ULONGLONG, cbExtraArgs: ::minwindef::UINT } /* winuser.h:14763:16, winuser.h:14763:16, winuser.h:14763:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub type PGESTUREINFO = *mut ::winuser::GESTUREINFO; /* winuser.h:14773:17, winuser.h:14773:17, winuser.h:14773:17 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub type PCGESTUREINFO = *const ::winuser::GESTUREINFO; /* winuser.h:14774:29, winuser.h:14774:29, winuser.h:14774:29 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct GESTURENOTIFYSTRUCT { cbSize: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, hwndTarget: ::windef::HWND, ptsLocation: ::windef::POINTS, dwInstanceID: ::minwindef::DWORD } /* winuser.h:14784:16, winuser.h:14784:16, winuser.h:14784:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub type PGESTURENOTIFYSTRUCT = *mut ::winuser::GESTURENOTIFYSTRUCT; /* winuser.h:14790:25, winuser.h:14790:25, winuser.h:14790:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct GESTURECONFIG { dwID: ::minwindef::DWORD, dwWant: ::minwindef::DWORD, dwBlock: ::minwindef::DWORD } /* winuser.h:14849:16, winuser.h:14849:16, winuser.h:14849:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub type PGESTURECONFIG = *mut ::winuser::GESTURECONFIG; /* winuser.h:14853:19, winuser.h:14853:19, winuser.h:14853:19 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub enum INPUT_MESSAGE_DEVICE_TYPE {IMDT_UNAVAILABLE = 0, IMDT_KEYBOARD = 1, IMDT_MOUSE = 2, IMDT_TOUCH = 4, IMDT_PEN = 8} pub use self::INPUT_MESSAGE_DEVICE_TYPE::{IMDT_UNAVAILABLE, IMDT_KEYBOARD, IMDT_MOUSE, IMDT_TOUCH, IMDT_PEN}; /* winuser.h:14991:14, winuser.h:14991:14, winuser.h:14991:14 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub enum INPUT_MESSAGE_ORIGIN_ID {IMO_UNAVAILABLE = 0, IMO_HARDWARE = 1, IMO_INJECTED = 2, IMO_SYSTEM = 4} pub use self::INPUT_MESSAGE_ORIGIN_ID::{IMO_UNAVAILABLE, IMO_HARDWARE, IMO_INJECTED, IMO_SYSTEM}; /* winuser.h:15002:14, winuser.h:15002:14, winuser.h:15002:14 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct INPUT_MESSAGE_SOURCE { deviceType: ::winuser::INPUT_MESSAGE_DEVICE_TYPE, originId: ::winuser::INPUT_MESSAGE_ORIGIN_ID } /* winuser.h:15012:17, winuser.h:15012:17, winuser.h:15012:17 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub enum AR_STATE {AR_ENABLED = 0, AR_DISABLED = 1, AR_SUPPRESSED = 2, AR_REMOTESESSION = 4, AR_MULTIMON = 8, AR_NOSENSOR = 16, AR_NOT_SUPPORTED = 32, AR_DOCKED = 64, AR_LAPTOP = 128} pub use self::AR_STATE::{AR_ENABLED, AR_DISABLED, AR_SUPPRESSED, AR_REMOTESESSION, AR_MULTIMON, AR_NOSENSOR, AR_NOT_SUPPORTED, AR_DOCKED, AR_LAPTOP}; /* winuser.h:15046:14, winuser.h:15046:14, winuser.h:15046:14 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub type PAR_STATE = *mut ::winuser::AR_STATE; /* winuser.h:15056:14, winuser.h:15056:14, winuser.h:15056:14 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub enum ORIENTATION_PREFERENCE {ORIENTATION_PREFERENCE_NONE = 0, ORIENTATION_PREFERENCE_LANDSCAPE = 1, ORIENTATION_PREFERENCE_PORTRAIT = 2, ORIENTATION_PREFERENCE_LANDSCAPE_FLIPPED = 4, ORIENTATION_PREFERENCE_PORTRAIT_FLIPPED = 8} pub use self::ORIENTATION_PREFERENCE::{ORIENTATION_PREFERENCE_NONE, ORIENTATION_PREFERENCE_LANDSCAPE, ORIENTATION_PREFERENCE_PORTRAIT, ORIENTATION_PREFERENCE_LANDSCAPE_FLIPPED, ORIENTATION_PREFERENCE_PORTRAIT_FLIPPED}; /* winuser.h:15075:14, winuser.h:15075:14, winuser.h:15075:14 */
pub const DIFFERENCE: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:233:9, winuser.h:233:9, winuser.h:233:9 */
pub const SB_HORZ: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:330:9, winuser.h:330:9, winuser.h:330:9 */
pub const SB_VERT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:331:9, winuser.h:331:9, winuser.h:331:9 */
pub const SB_CTL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:332:9, winuser.h:332:9, winuser.h:332:9 */
pub const SB_BOTH: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:333:9, winuser.h:333:9, winuser.h:333:9 */
pub const SB_LINEUP: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:338:9, winuser.h:338:9, winuser.h:338:9 */
pub const SB_LINELEFT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:339:9, winuser.h:339:9, winuser.h:339:9 */
pub const SB_LINEDOWN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:340:9, winuser.h:340:9, winuser.h:340:9 */
pub const SB_LINERIGHT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:341:9, winuser.h:341:9, winuser.h:341:9 */
pub const SB_PAGEUP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:342:9, winuser.h:342:9, winuser.h:342:9 */
pub const SB_PAGELEFT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:343:9, winuser.h:343:9, winuser.h:343:9 */
pub const SB_PAGEDOWN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:344:9, winuser.h:344:9, winuser.h:344:9 */
pub const SB_PAGERIGHT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:345:9, winuser.h:345:9, winuser.h:345:9 */
pub const SB_THUMBPOSITION: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:346:9, winuser.h:346:9, winuser.h:346:9 */
pub const SB_THUMBTRACK: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:347:9, winuser.h:347:9, winuser.h:347:9 */
pub const SB_TOP: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:348:9, winuser.h:348:9, winuser.h:348:9 */
pub const SB_LEFT: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:349:9, winuser.h:349:9, winuser.h:349:9 */
pub const SB_BOTTOM: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:350:9, winuser.h:350:9, winuser.h:350:9 */
pub const SB_RIGHT: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:351:9, winuser.h:351:9, winuser.h:351:9 */
pub const SB_ENDSCROLL: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:352:9, winuser.h:352:9, winuser.h:352:9 */
pub const SW_HIDE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:362:9, winuser.h:362:9, winuser.h:362:9 */
pub const SW_SHOWNORMAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:363:9, winuser.h:363:9, winuser.h:363:9 */
pub const SW_NORMAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:364:9, winuser.h:364:9, winuser.h:364:9 */
pub const SW_SHOWMINIMIZED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:365:9, winuser.h:365:9, winuser.h:365:9 */
pub const SW_SHOWMAXIMIZED: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:366:9, winuser.h:366:9, winuser.h:366:9 */
pub const SW_MAXIMIZE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:367:9, winuser.h:367:9, winuser.h:367:9 */
pub const SW_SHOWNOACTIVATE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:368:9, winuser.h:368:9, winuser.h:368:9 */
pub const SW_SHOW: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:369:9, winuser.h:369:9, winuser.h:369:9 */
pub const SW_MINIMIZE: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:370:9, winuser.h:370:9, winuser.h:370:9 */
pub const SW_SHOWMINNOACTIVE: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:371:9, winuser.h:371:9, winuser.h:371:9 */
pub const SW_SHOWNA: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:372:9, winuser.h:372:9, winuser.h:372:9 */
pub const SW_RESTORE: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:373:9, winuser.h:373:9, winuser.h:373:9 */
pub const SW_SHOWDEFAULT: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:374:9, winuser.h:374:9, winuser.h:374:9 */
pub const SW_FORCEMINIMIZE: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:375:9, winuser.h:375:9, winuser.h:375:9 */
pub const SW_MAX: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:376:9, winuser.h:376:9, winuser.h:376:9 */
pub const HIDE_WINDOW: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:382:9, winuser.h:382:9, winuser.h:382:9 */
pub const SHOW_OPENWINDOW: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:383:9, winuser.h:383:9, winuser.h:383:9 */
pub const SHOW_ICONWINDOW: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:384:9, winuser.h:384:9, winuser.h:384:9 */
pub const SHOW_FULLSCREEN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:385:9, winuser.h:385:9, winuser.h:385:9 */
pub const SHOW_OPENNOACTIVATE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:386:9, winuser.h:386:9, winuser.h:386:9 */
pub const SW_PARENTCLOSING: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:391:9, winuser.h:391:9, winuser.h:391:9 */
pub const SW_OTHERZOOM: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:392:9, winuser.h:392:9, winuser.h:392:9 */
pub const SW_PARENTOPENING: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:393:9, winuser.h:393:9, winuser.h:393:9 */
pub const SW_OTHERUNZOOM: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:394:9, winuser.h:394:9, winuser.h:394:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const AW_HOR_POSITIVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:403:9, winuser.h:403:9, winuser.h:403:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const AW_HOR_NEGATIVE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:404:9, winuser.h:404:9, winuser.h:404:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const AW_VER_POSITIVE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:405:9, winuser.h:405:9, winuser.h:405:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const AW_VER_NEGATIVE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:406:9, winuser.h:406:9, winuser.h:406:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const AW_CENTER: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:407:9, winuser.h:407:9, winuser.h:407:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const AW_HIDE: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winuser.h:408:9, winuser.h:408:9, winuser.h:408:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const AW_ACTIVATE: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winuser.h:409:9, winuser.h:409:9, winuser.h:409:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const AW_SLIDE: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* winuser.h:410:9, winuser.h:410:9, winuser.h:410:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const AW_BLEND: i32 = 0x80000i32; /* Integer(524288, Yes, Unknown) */ /* winuser.h:411:9, winuser.h:411:9, winuser.h:411:9 */
pub const KF_EXTENDED: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:419:9, winuser.h:419:9, winuser.h:419:9 */
pub const KF_DLGMODE: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winuser.h:420:9, winuser.h:420:9, winuser.h:420:9 */
pub const KF_MENUMODE: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:421:9, winuser.h:421:9, winuser.h:421:9 */
pub const KF_ALTDOWN: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:422:9, winuser.h:422:9, winuser.h:422:9 */
pub const KF_REPEAT: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winuser.h:423:9, winuser.h:423:9, winuser.h:423:9 */
pub const KF_UP: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:424:9, winuser.h:424:9, winuser.h:424:9 */
pub const VK_LBUTTON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:432:9, winuser.h:432:9, winuser.h:432:9 */
pub const VK_RBUTTON: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:433:9, winuser.h:433:9, winuser.h:433:9 */
pub const VK_CANCEL: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:434:9, winuser.h:434:9, winuser.h:434:9 */
pub const VK_MBUTTON: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:435:9, winuser.h:435:9, winuser.h:435:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_XBUTTON1: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:438:9, winuser.h:438:9, winuser.h:438:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_XBUTTON2: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:439:9, winuser.h:439:9, winuser.h:439:9 */
pub const VK_BACK: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:446:9, winuser.h:446:9, winuser.h:446:9 */
pub const VK_TAB: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:447:9, winuser.h:447:9, winuser.h:447:9 */
pub const VK_CLEAR: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winuser.h:453:9, winuser.h:453:9, winuser.h:453:9 */
pub const VK_RETURN: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winuser.h:454:9, winuser.h:454:9, winuser.h:454:9 */
pub const VK_SHIFT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:456:9, winuser.h:456:9, winuser.h:456:9 */
pub const VK_CONTROL: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winuser.h:457:9, winuser.h:457:9, winuser.h:457:9 */
pub const VK_MENU: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winuser.h:458:9, winuser.h:458:9, winuser.h:458:9 */
pub const VK_PAUSE: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winuser.h:459:9, winuser.h:459:9, winuser.h:459:9 */
pub const VK_CAPITAL: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winuser.h:460:9, winuser.h:460:9, winuser.h:460:9 */
pub const VK_KANA: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winuser.h:462:9, winuser.h:462:9, winuser.h:462:9 */
pub const VK_HANGEUL: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winuser.h:463:9, winuser.h:463:9, winuser.h:463:9 */
pub const VK_HANGUL: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winuser.h:464:9, winuser.h:464:9, winuser.h:464:9 */
pub const VK_JUNJA: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winuser.h:465:9, winuser.h:465:9, winuser.h:465:9 */
pub const VK_FINAL: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winuser.h:466:9, winuser.h:466:9, winuser.h:466:9 */
pub const VK_HANJA: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* winuser.h:467:9, winuser.h:467:9, winuser.h:467:9 */
pub const VK_KANJI: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* winuser.h:468:9, winuser.h:468:9, winuser.h:468:9 */
pub const VK_ESCAPE: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* winuser.h:470:9, winuser.h:470:9, winuser.h:470:9 */
pub const VK_CONVERT: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* winuser.h:472:9, winuser.h:472:9, winuser.h:472:9 */
pub const VK_NONCONVERT: i32 = 0x1di32; /* Integer(29, Yes, Unknown) */ /* winuser.h:473:9, winuser.h:473:9, winuser.h:473:9 */
pub const VK_ACCEPT: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* winuser.h:474:9, winuser.h:474:9, winuser.h:474:9 */
pub const VK_MODECHANGE: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* winuser.h:475:9, winuser.h:475:9, winuser.h:475:9 */
pub const VK_SPACE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:477:9, winuser.h:477:9, winuser.h:477:9 */
pub const VK_PRIOR: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* winuser.h:478:9, winuser.h:478:9, winuser.h:478:9 */
pub const VK_NEXT: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* winuser.h:479:9, winuser.h:479:9, winuser.h:479:9 */
pub const VK_END: i32 = 0x23i32; /* Integer(35, Yes, Unknown) */ /* winuser.h:480:9, winuser.h:480:9, winuser.h:480:9 */
pub const VK_HOME: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* winuser.h:481:9, winuser.h:481:9, winuser.h:481:9 */
pub const VK_LEFT: i32 = 0x25i32; /* Integer(37, Yes, Unknown) */ /* winuser.h:482:9, winuser.h:482:9, winuser.h:482:9 */
pub const VK_UP: i32 = 0x26i32; /* Integer(38, Yes, Unknown) */ /* winuser.h:483:9, winuser.h:483:9, winuser.h:483:9 */
pub const VK_RIGHT: i32 = 0x27i32; /* Integer(39, Yes, Unknown) */ /* winuser.h:484:9, winuser.h:484:9, winuser.h:484:9 */
pub const VK_DOWN: i32 = 0x28i32; /* Integer(40, Yes, Unknown) */ /* winuser.h:485:9, winuser.h:485:9, winuser.h:485:9 */
pub const VK_SELECT: i32 = 0x29i32; /* Integer(41, Yes, Unknown) */ /* winuser.h:486:9, winuser.h:486:9, winuser.h:486:9 */
pub const VK_PRINT: i32 = 0x2ai32; /* Integer(42, Yes, Unknown) */ /* winuser.h:487:9, winuser.h:487:9, winuser.h:487:9 */
pub const VK_EXECUTE: i32 = 0x2bi32; /* Integer(43, Yes, Unknown) */ /* winuser.h:488:9, winuser.h:488:9, winuser.h:488:9 */
pub const VK_SNAPSHOT: i32 = 0x2ci32; /* Integer(44, Yes, Unknown) */ /* winuser.h:489:9, winuser.h:489:9, winuser.h:489:9 */
pub const VK_INSERT: i32 = 0x2di32; /* Integer(45, Yes, Unknown) */ /* winuser.h:490:9, winuser.h:490:9, winuser.h:490:9 */
pub const VK_DELETE: i32 = 0x2ei32; /* Integer(46, Yes, Unknown) */ /* winuser.h:491:9, winuser.h:491:9, winuser.h:491:9 */
pub const VK_HELP: i32 = 0x2fi32; /* Integer(47, Yes, Unknown) */ /* winuser.h:492:9, winuser.h:492:9, winuser.h:492:9 */
pub const VK_LWIN: i32 = 0x5bi32; /* Integer(91, Yes, Unknown) */ /* winuser.h:500:9, winuser.h:500:9, winuser.h:500:9 */
pub const VK_RWIN: i32 = 0x5ci32; /* Integer(92, Yes, Unknown) */ /* winuser.h:501:9, winuser.h:501:9, winuser.h:501:9 */
pub const VK_APPS: i32 = 0x5di32; /* Integer(93, Yes, Unknown) */ /* winuser.h:502:9, winuser.h:502:9, winuser.h:502:9 */
pub const VK_SLEEP: i32 = 0x5fi32; /* Integer(95, Yes, Unknown) */ /* winuser.h:508:9, winuser.h:508:9, winuser.h:508:9 */
pub const VK_NUMPAD0: i32 = 0x60i32; /* Integer(96, Yes, Unknown) */ /* winuser.h:510:9, winuser.h:510:9, winuser.h:510:9 */
pub const VK_NUMPAD1: i32 = 0x61i32; /* Integer(97, Yes, Unknown) */ /* winuser.h:511:9, winuser.h:511:9, winuser.h:511:9 */
pub const VK_NUMPAD2: i32 = 0x62i32; /* Integer(98, Yes, Unknown) */ /* winuser.h:512:9, winuser.h:512:9, winuser.h:512:9 */
pub const VK_NUMPAD3: i32 = 0x63i32; /* Integer(99, Yes, Unknown) */ /* winuser.h:513:9, winuser.h:513:9, winuser.h:513:9 */
pub const VK_NUMPAD4: i32 = 0x64i32; /* Integer(100, Yes, Unknown) */ /* winuser.h:514:9, winuser.h:514:9, winuser.h:514:9 */
pub const VK_NUMPAD5: i32 = 0x65i32; /* Integer(101, Yes, Unknown) */ /* winuser.h:515:9, winuser.h:515:9, winuser.h:515:9 */
pub const VK_NUMPAD6: i32 = 0x66i32; /* Integer(102, Yes, Unknown) */ /* winuser.h:516:9, winuser.h:516:9, winuser.h:516:9 */
pub const VK_NUMPAD7: i32 = 0x67i32; /* Integer(103, Yes, Unknown) */ /* winuser.h:517:9, winuser.h:517:9, winuser.h:517:9 */
pub const VK_NUMPAD8: i32 = 0x68i32; /* Integer(104, Yes, Unknown) */ /* winuser.h:518:9, winuser.h:518:9, winuser.h:518:9 */
pub const VK_NUMPAD9: i32 = 0x69i32; /* Integer(105, Yes, Unknown) */ /* winuser.h:519:9, winuser.h:519:9, winuser.h:519:9 */
pub const VK_MULTIPLY: i32 = 0x6ai32; /* Integer(106, Yes, Unknown) */ /* winuser.h:520:9, winuser.h:520:9, winuser.h:520:9 */
pub const VK_ADD: i32 = 0x6bi32; /* Integer(107, Yes, Unknown) */ /* winuser.h:521:9, winuser.h:521:9, winuser.h:521:9 */
pub const VK_SEPARATOR: i32 = 0x6ci32; /* Integer(108, Yes, Unknown) */ /* winuser.h:522:9, winuser.h:522:9, winuser.h:522:9 */
pub const VK_SUBTRACT: i32 = 0x6di32; /* Integer(109, Yes, Unknown) */ /* winuser.h:523:9, winuser.h:523:9, winuser.h:523:9 */
pub const VK_DECIMAL: i32 = 0x6ei32; /* Integer(110, Yes, Unknown) */ /* winuser.h:524:9, winuser.h:524:9, winuser.h:524:9 */
pub const VK_DIVIDE: i32 = 0x6fi32; /* Integer(111, Yes, Unknown) */ /* winuser.h:525:9, winuser.h:525:9, winuser.h:525:9 */
pub const VK_F1: i32 = 0x70i32; /* Integer(112, Yes, Unknown) */ /* winuser.h:526:9, winuser.h:526:9, winuser.h:526:9 */
pub const VK_F2: i32 = 0x71i32; /* Integer(113, Yes, Unknown) */ /* winuser.h:527:9, winuser.h:527:9, winuser.h:527:9 */
pub const VK_F3: i32 = 0x72i32; /* Integer(114, Yes, Unknown) */ /* winuser.h:528:9, winuser.h:528:9, winuser.h:528:9 */
pub const VK_F4: i32 = 0x73i32; /* Integer(115, Yes, Unknown) */ /* winuser.h:529:9, winuser.h:529:9, winuser.h:529:9 */
pub const VK_F5: i32 = 0x74i32; /* Integer(116, Yes, Unknown) */ /* winuser.h:530:9, winuser.h:530:9, winuser.h:530:9 */
pub const VK_F6: i32 = 0x75i32; /* Integer(117, Yes, Unknown) */ /* winuser.h:531:9, winuser.h:531:9, winuser.h:531:9 */
pub const VK_F7: i32 = 0x76i32; /* Integer(118, Yes, Unknown) */ /* winuser.h:532:9, winuser.h:532:9, winuser.h:532:9 */
pub const VK_F8: i32 = 0x77i32; /* Integer(119, Yes, Unknown) */ /* winuser.h:533:9, winuser.h:533:9, winuser.h:533:9 */
pub const VK_F9: i32 = 0x78i32; /* Integer(120, Yes, Unknown) */ /* winuser.h:534:9, winuser.h:534:9, winuser.h:534:9 */
pub const VK_F10: i32 = 0x79i32; /* Integer(121, Yes, Unknown) */ /* winuser.h:535:9, winuser.h:535:9, winuser.h:535:9 */
pub const VK_F11: i32 = 0x7ai32; /* Integer(122, Yes, Unknown) */ /* winuser.h:536:9, winuser.h:536:9, winuser.h:536:9 */
pub const VK_F12: i32 = 0x7bi32; /* Integer(123, Yes, Unknown) */ /* winuser.h:537:9, winuser.h:537:9, winuser.h:537:9 */
pub const VK_F13: i32 = 0x7ci32; /* Integer(124, Yes, Unknown) */ /* winuser.h:538:9, winuser.h:538:9, winuser.h:538:9 */
pub const VK_F14: i32 = 0x7di32; /* Integer(125, Yes, Unknown) */ /* winuser.h:539:9, winuser.h:539:9, winuser.h:539:9 */
pub const VK_F15: i32 = 0x7ei32; /* Integer(126, Yes, Unknown) */ /* winuser.h:540:9, winuser.h:540:9, winuser.h:540:9 */
pub const VK_F16: i32 = 0x7fi32; /* Integer(127, Yes, Unknown) */ /* winuser.h:541:9, winuser.h:541:9, winuser.h:541:9 */
pub const VK_F17: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:542:9, winuser.h:542:9, winuser.h:542:9 */
pub const VK_F18: i32 = 0x81i32; /* Integer(129, Yes, Unknown) */ /* winuser.h:543:9, winuser.h:543:9, winuser.h:543:9 */
pub const VK_F19: i32 = 0x82i32; /* Integer(130, Yes, Unknown) */ /* winuser.h:544:9, winuser.h:544:9, winuser.h:544:9 */
pub const VK_F20: i32 = 0x83i32; /* Integer(131, Yes, Unknown) */ /* winuser.h:545:9, winuser.h:545:9, winuser.h:545:9 */
pub const VK_F21: i32 = 0x84i32; /* Integer(132, Yes, Unknown) */ /* winuser.h:546:9, winuser.h:546:9, winuser.h:546:9 */
pub const VK_F22: i32 = 0x85i32; /* Integer(133, Yes, Unknown) */ /* winuser.h:547:9, winuser.h:547:9, winuser.h:547:9 */
pub const VK_F23: i32 = 0x86i32; /* Integer(134, Yes, Unknown) */ /* winuser.h:548:9, winuser.h:548:9, winuser.h:548:9 */
pub const VK_F24: i32 = 0x87i32; /* Integer(135, Yes, Unknown) */ /* winuser.h:549:9, winuser.h:549:9, winuser.h:549:9 */
pub const VK_NUMLOCK: i32 = 0x90i32; /* Integer(144, Yes, Unknown) */ /* winuser.h:555:9, winuser.h:555:9, winuser.h:555:9 */
pub const VK_SCROLL: i32 = 0x91i32; /* Integer(145, Yes, Unknown) */ /* winuser.h:556:9, winuser.h:556:9, winuser.h:556:9 */
pub const VK_OEM_NEC_EQUAL: i32 = 0x92i32; /* Integer(146, Yes, Unknown) */ /* winuser.h:561:9, winuser.h:561:9, winuser.h:561:9 */
pub const VK_OEM_FJ_JISHO: i32 = 0x92i32; /* Integer(146, Yes, Unknown) */ /* winuser.h:566:9, winuser.h:566:9, winuser.h:566:9 */
pub const VK_OEM_FJ_MASSHOU: i32 = 0x93i32; /* Integer(147, Yes, Unknown) */ /* winuser.h:567:9, winuser.h:567:9, winuser.h:567:9 */
pub const VK_OEM_FJ_TOUROKU: i32 = 0x94i32; /* Integer(148, Yes, Unknown) */ /* winuser.h:568:9, winuser.h:568:9, winuser.h:568:9 */
pub const VK_OEM_FJ_LOYA: i32 = 0x95i32; /* Integer(149, Yes, Unknown) */ /* winuser.h:569:9, winuser.h:569:9, winuser.h:569:9 */
pub const VK_OEM_FJ_ROYA: i32 = 0x96i32; /* Integer(150, Yes, Unknown) */ /* winuser.h:570:9, winuser.h:570:9, winuser.h:570:9 */
pub const VK_LSHIFT: i32 = 0xa0i32; /* Integer(160, Yes, Unknown) */ /* winuser.h:581:9, winuser.h:581:9, winuser.h:581:9 */
pub const VK_RSHIFT: i32 = 0xa1i32; /* Integer(161, Yes, Unknown) */ /* winuser.h:582:9, winuser.h:582:9, winuser.h:582:9 */
pub const VK_LCONTROL: i32 = 0xa2i32; /* Integer(162, Yes, Unknown) */ /* winuser.h:583:9, winuser.h:583:9, winuser.h:583:9 */
pub const VK_RCONTROL: i32 = 0xa3i32; /* Integer(163, Yes, Unknown) */ /* winuser.h:584:9, winuser.h:584:9, winuser.h:584:9 */
pub const VK_LMENU: i32 = 0xa4i32; /* Integer(164, Yes, Unknown) */ /* winuser.h:585:9, winuser.h:585:9, winuser.h:585:9 */
pub const VK_RMENU: i32 = 0xa5i32; /* Integer(165, Yes, Unknown) */ /* winuser.h:586:9, winuser.h:586:9, winuser.h:586:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_BROWSER_BACK: i32 = 0xa6i32; /* Integer(166, Yes, Unknown) */ /* winuser.h:589:9, winuser.h:589:9, winuser.h:589:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_BROWSER_FORWARD: i32 = 0xa7i32; /* Integer(167, Yes, Unknown) */ /* winuser.h:590:9, winuser.h:590:9, winuser.h:590:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_BROWSER_REFRESH: i32 = 0xa8i32; /* Integer(168, Yes, Unknown) */ /* winuser.h:591:9, winuser.h:591:9, winuser.h:591:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_BROWSER_STOP: i32 = 0xa9i32; /* Integer(169, Yes, Unknown) */ /* winuser.h:592:9, winuser.h:592:9, winuser.h:592:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_BROWSER_SEARCH: i32 = 0xaai32; /* Integer(170, Yes, Unknown) */ /* winuser.h:593:9, winuser.h:593:9, winuser.h:593:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_BROWSER_FAVORITES: i32 = 0xabi32; /* Integer(171, Yes, Unknown) */ /* winuser.h:594:9, winuser.h:594:9, winuser.h:594:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_BROWSER_HOME: i32 = 0xaci32; /* Integer(172, Yes, Unknown) */ /* winuser.h:595:9, winuser.h:595:9, winuser.h:595:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_VOLUME_MUTE: i32 = 0xadi32; /* Integer(173, Yes, Unknown) */ /* winuser.h:597:9, winuser.h:597:9, winuser.h:597:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_VOLUME_DOWN: i32 = 0xaei32; /* Integer(174, Yes, Unknown) */ /* winuser.h:598:9, winuser.h:598:9, winuser.h:598:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_VOLUME_UP: i32 = 0xafi32; /* Integer(175, Yes, Unknown) */ /* winuser.h:599:9, winuser.h:599:9, winuser.h:599:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_MEDIA_NEXT_TRACK: i32 = 0xb0i32; /* Integer(176, Yes, Unknown) */ /* winuser.h:600:9, winuser.h:600:9, winuser.h:600:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_MEDIA_PREV_TRACK: i32 = 0xb1i32; /* Integer(177, Yes, Unknown) */ /* winuser.h:601:9, winuser.h:601:9, winuser.h:601:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_MEDIA_STOP: i32 = 0xb2i32; /* Integer(178, Yes, Unknown) */ /* winuser.h:602:9, winuser.h:602:9, winuser.h:602:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_MEDIA_PLAY_PAUSE: i32 = 0xb3i32; /* Integer(179, Yes, Unknown) */ /* winuser.h:603:9, winuser.h:603:9, winuser.h:603:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_LAUNCH_MAIL: i32 = 0xb4i32; /* Integer(180, Yes, Unknown) */ /* winuser.h:604:9, winuser.h:604:9, winuser.h:604:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_LAUNCH_MEDIA_SELECT: i32 = 0xb5i32; /* Integer(181, Yes, Unknown) */ /* winuser.h:605:9, winuser.h:605:9, winuser.h:605:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_LAUNCH_APP1: i32 = 0xb6i32; /* Integer(182, Yes, Unknown) */ /* winuser.h:606:9, winuser.h:606:9, winuser.h:606:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_LAUNCH_APP2: i32 = 0xb7i32; /* Integer(183, Yes, Unknown) */ /* winuser.h:607:9, winuser.h:607:9, winuser.h:607:9 */
pub const VK_OEM_1: i32 = 0xbai32; /* Integer(186, Yes, Unknown) */ /* winuser.h:615:9, winuser.h:615:9, winuser.h:615:9 */
pub const VK_OEM_PLUS: i32 = 0xbbi32; /* Integer(187, Yes, Unknown) */ /* winuser.h:616:9, winuser.h:616:9, winuser.h:616:9 */
pub const VK_OEM_COMMA: i32 = 0xbci32; /* Integer(188, Yes, Unknown) */ /* winuser.h:617:9, winuser.h:617:9, winuser.h:617:9 */
pub const VK_OEM_MINUS: i32 = 0xbdi32; /* Integer(189, Yes, Unknown) */ /* winuser.h:618:9, winuser.h:618:9, winuser.h:618:9 */
pub const VK_OEM_PERIOD: i32 = 0xbei32; /* Integer(190, Yes, Unknown) */ /* winuser.h:619:9, winuser.h:619:9, winuser.h:619:9 */
pub const VK_OEM_2: i32 = 0xbfi32; /* Integer(191, Yes, Unknown) */ /* winuser.h:620:9, winuser.h:620:9, winuser.h:620:9 */
pub const VK_OEM_3: i32 = 0xc0i32; /* Integer(192, Yes, Unknown) */ /* winuser.h:621:9, winuser.h:621:9, winuser.h:621:9 */
pub const VK_OEM_4: i32 = 0xdbi32; /* Integer(219, Yes, Unknown) */ /* winuser.h:631:9, winuser.h:631:9, winuser.h:631:9 */
pub const VK_OEM_5: i32 = 0xdci32; /* Integer(220, Yes, Unknown) */ /* winuser.h:632:9, winuser.h:632:9, winuser.h:632:9 */
pub const VK_OEM_6: i32 = 0xddi32; /* Integer(221, Yes, Unknown) */ /* winuser.h:633:9, winuser.h:633:9, winuser.h:633:9 */
pub const VK_OEM_7: i32 = 0xdei32; /* Integer(222, Yes, Unknown) */ /* winuser.h:634:9, winuser.h:634:9, winuser.h:634:9 */
pub const VK_OEM_8: i32 = 0xdfi32; /* Integer(223, Yes, Unknown) */ /* winuser.h:635:9, winuser.h:635:9, winuser.h:635:9 */
pub const VK_OEM_AX: i32 = 0xe1i32; /* Integer(225, Yes, Unknown) */ /* winuser.h:644:9, winuser.h:644:9, winuser.h:644:9 */
pub const VK_OEM_102: i32 = 0xe2i32; /* Integer(226, Yes, Unknown) */ /* winuser.h:645:9, winuser.h:645:9, winuser.h:645:9 */
pub const VK_ICO_HELP: i32 = 0xe3i32; /* Integer(227, Yes, Unknown) */ /* winuser.h:646:9, winuser.h:646:9, winuser.h:646:9 */
pub const VK_ICO_00: i32 = 0xe4i32; /* Integer(228, Yes, Unknown) */ /* winuser.h:647:9, winuser.h:647:9, winuser.h:647:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const VK_PROCESSKEY: i32 = 0xe5i32; /* Integer(229, Yes, Unknown) */ /* winuser.h:650:9, winuser.h:650:9, winuser.h:650:9 */
pub const VK_ICO_CLEAR: i32 = 0xe6i32; /* Integer(230, Yes, Unknown) */ /* winuser.h:653:9, winuser.h:653:9, winuser.h:653:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const VK_PACKET: i32 = 0xe7i32; /* Integer(231, Yes, Unknown) */ /* winuser.h:657:9, winuser.h:657:9, winuser.h:657:9 */
pub const VK_OEM_RESET: i32 = 0xe9i32; /* Integer(233, Yes, Unknown) */ /* winuser.h:667:9, winuser.h:667:9, winuser.h:667:9 */
pub const VK_OEM_JUMP: i32 = 0xeai32; /* Integer(234, Yes, Unknown) */ /* winuser.h:668:9, winuser.h:668:9, winuser.h:668:9 */
pub const VK_OEM_PA1: i32 = 0xebi32; /* Integer(235, Yes, Unknown) */ /* winuser.h:669:9, winuser.h:669:9, winuser.h:669:9 */
pub const VK_OEM_PA2: i32 = 0xeci32; /* Integer(236, Yes, Unknown) */ /* winuser.h:670:9, winuser.h:670:9, winuser.h:670:9 */
pub const VK_OEM_PA3: i32 = 0xedi32; /* Integer(237, Yes, Unknown) */ /* winuser.h:671:9, winuser.h:671:9, winuser.h:671:9 */
pub const VK_OEM_WSCTRL: i32 = 0xeei32; /* Integer(238, Yes, Unknown) */ /* winuser.h:672:9, winuser.h:672:9, winuser.h:672:9 */
pub const VK_OEM_CUSEL: i32 = 0xefi32; /* Integer(239, Yes, Unknown) */ /* winuser.h:673:9, winuser.h:673:9, winuser.h:673:9 */
pub const VK_OEM_ATTN: i32 = 0xf0i32; /* Integer(240, Yes, Unknown) */ /* winuser.h:674:9, winuser.h:674:9, winuser.h:674:9 */
pub const VK_OEM_FINISH: i32 = 0xf1i32; /* Integer(241, Yes, Unknown) */ /* winuser.h:675:9, winuser.h:675:9, winuser.h:675:9 */
pub const VK_OEM_COPY: i32 = 0xf2i32; /* Integer(242, Yes, Unknown) */ /* winuser.h:676:9, winuser.h:676:9, winuser.h:676:9 */
pub const VK_OEM_AUTO: i32 = 0xf3i32; /* Integer(243, Yes, Unknown) */ /* winuser.h:677:9, winuser.h:677:9, winuser.h:677:9 */
pub const VK_OEM_ENLW: i32 = 0xf4i32; /* Integer(244, Yes, Unknown) */ /* winuser.h:678:9, winuser.h:678:9, winuser.h:678:9 */
pub const VK_OEM_BACKTAB: i32 = 0xf5i32; /* Integer(245, Yes, Unknown) */ /* winuser.h:679:9, winuser.h:679:9, winuser.h:679:9 */
pub const VK_ATTN: i32 = 0xf6i32; /* Integer(246, Yes, Unknown) */ /* winuser.h:681:9, winuser.h:681:9, winuser.h:681:9 */
pub const VK_CRSEL: i32 = 0xf7i32; /* Integer(247, Yes, Unknown) */ /* winuser.h:682:9, winuser.h:682:9, winuser.h:682:9 */
pub const VK_EXSEL: i32 = 0xf8i32; /* Integer(248, Yes, Unknown) */ /* winuser.h:683:9, winuser.h:683:9, winuser.h:683:9 */
pub const VK_EREOF: i32 = 0xf9i32; /* Integer(249, Yes, Unknown) */ /* winuser.h:684:9, winuser.h:684:9, winuser.h:684:9 */
pub const VK_PLAY: i32 = 0xfai32; /* Integer(250, Yes, Unknown) */ /* winuser.h:685:9, winuser.h:685:9, winuser.h:685:9 */
pub const VK_ZOOM: i32 = 0xfbi32; /* Integer(251, Yes, Unknown) */ /* winuser.h:686:9, winuser.h:686:9, winuser.h:686:9 */
pub const VK_NONAME: i32 = 0xfci32; /* Integer(252, Yes, Unknown) */ /* winuser.h:687:9, winuser.h:687:9, winuser.h:687:9 */
pub const VK_PA1: i32 = 0xfdi32; /* Integer(253, Yes, Unknown) */ /* winuser.h:688:9, winuser.h:688:9, winuser.h:688:9 */
pub const VK_OEM_CLEAR: i32 = 0xfei32; /* Integer(254, Yes, Unknown) */ /* winuser.h:689:9, winuser.h:689:9, winuser.h:689:9 */
pub const WH_JOURNALRECORD: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:705:9, winuser.h:705:9, winuser.h:705:9 */
pub const WH_JOURNALPLAYBACK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:706:9, winuser.h:706:9, winuser.h:706:9 */
pub const WH_KEYBOARD: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:707:9, winuser.h:707:9, winuser.h:707:9 */
pub const WH_GETMESSAGE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:708:9, winuser.h:708:9, winuser.h:708:9 */
pub const WH_CALLWNDPROC: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:709:9, winuser.h:709:9, winuser.h:709:9 */
pub const WH_CBT: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:710:9, winuser.h:710:9, winuser.h:710:9 */
pub const WH_SYSMSGFILTER: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:711:9, winuser.h:711:9, winuser.h:711:9 */
pub const WH_MOUSE: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:712:9, winuser.h:712:9, winuser.h:712:9 */
pub const WH_DEBUG: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:716:9, winuser.h:716:9, winuser.h:716:9 */
pub const WH_SHELL: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:717:9, winuser.h:717:9, winuser.h:717:9 */
pub const WH_FOREGROUNDIDLE: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:718:9, winuser.h:718:9, winuser.h:718:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WH_CALLWNDPROCRET: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winuser.h:720:9, winuser.h:720:9, winuser.h:720:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WH_KEYBOARD_LL: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winuser.h:724:9, winuser.h:724:9, winuser.h:724:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WH_MOUSE_LL: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winuser.h:725:9, winuser.h:725:9, winuser.h:725:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WH_MAX: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winuser.h:730:9, winuser.h:730:9, winuser.h:730:9 */
#[doc(inline)] pub use ::winuser::WH_MAX as WH_MAXHOOK; /* winuser.h:739:9, winuser.h:739:9, winuser.h:739:9 */
pub const HC_ACTION: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:744:9, winuser.h:744:9, winuser.h:744:9 */
pub const HC_GETNEXT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:745:9, winuser.h:745:9, winuser.h:745:9 */
pub const HC_SKIP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:746:9, winuser.h:746:9, winuser.h:746:9 */
pub const HC_NOREMOVE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:747:9, winuser.h:747:9, winuser.h:747:9 */
#[doc(inline)] pub use ::winuser::HC_NOREMOVE as HC_NOREM; /* winuser.h:748:9, winuser.h:748:9, winuser.h:748:9 */
pub const HC_SYSMODALON: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:749:9, winuser.h:749:9, winuser.h:749:9 */
pub const HC_SYSMODALOFF: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:750:9, winuser.h:750:9, winuser.h:750:9 */
pub const HCBT_MOVESIZE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:755:9, winuser.h:755:9, winuser.h:755:9 */
pub const HCBT_MINMAX: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:756:9, winuser.h:756:9, winuser.h:756:9 */
pub const HCBT_QS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:757:9, winuser.h:757:9, winuser.h:757:9 */
pub const HCBT_CREATEWND: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:758:9, winuser.h:758:9, winuser.h:758:9 */
pub const HCBT_DESTROYWND: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:759:9, winuser.h:759:9, winuser.h:759:9 */
pub const HCBT_ACTIVATE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:760:9, winuser.h:760:9, winuser.h:760:9 */
pub const HCBT_CLICKSKIPPED: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:761:9, winuser.h:761:9, winuser.h:761:9 */
pub const HCBT_KEYSKIPPED: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:762:9, winuser.h:762:9, winuser.h:762:9 */
pub const HCBT_SYSCOMMAND: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:763:9, winuser.h:763:9, winuser.h:763:9 */
pub const HCBT_SETFOCUS: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:764:9, winuser.h:764:9, winuser.h:764:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WTS_CONSOLE_CONNECT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:827:9, winuser.h:827:9, winuser.h:827:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WTS_CONSOLE_DISCONNECT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:828:9, winuser.h:828:9, winuser.h:828:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WTS_REMOTE_CONNECT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:829:9, winuser.h:829:9, winuser.h:829:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WTS_REMOTE_DISCONNECT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:830:9, winuser.h:830:9, winuser.h:830:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WTS_SESSION_LOGON: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:831:9, winuser.h:831:9, winuser.h:831:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WTS_SESSION_LOGOFF: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:832:9, winuser.h:832:9, winuser.h:832:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WTS_SESSION_LOCK: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:833:9, winuser.h:833:9, winuser.h:833:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WTS_SESSION_UNLOCK: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:834:9, winuser.h:834:9, winuser.h:834:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WTS_SESSION_REMOTE_CONTROL: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:835:9, winuser.h:835:9, winuser.h:835:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WTS_SESSION_CREATE: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:836:9, winuser.h:836:9, winuser.h:836:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WTS_SESSION_TERMINATE: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:837:9, winuser.h:837:9, winuser.h:837:9 */
pub const MSGF_DIALOGBOX: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:844:9, winuser.h:844:9, winuser.h:844:9 */
pub const MSGF_MESSAGEBOX: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:845:9, winuser.h:845:9, winuser.h:845:9 */
pub const MSGF_MENU: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:846:9, winuser.h:846:9, winuser.h:846:9 */
pub const MSGF_SCROLLBAR: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:847:9, winuser.h:847:9, winuser.h:847:9 */
pub const MSGF_NEXTWINDOW: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:848:9, winuser.h:848:9, winuser.h:848:9 */
pub const MSGF_MAX: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:849:9, winuser.h:849:9, winuser.h:849:9 */
pub const MSGF_USER: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:850:9, winuser.h:850:9, winuser.h:850:9 */
pub const HSHELL_WINDOWCREATED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:855:9, winuser.h:855:9, winuser.h:855:9 */
pub const HSHELL_WINDOWDESTROYED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:856:9, winuser.h:856:9, winuser.h:856:9 */
pub const HSHELL_ACTIVATESHELLWINDOW: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:857:9, winuser.h:857:9, winuser.h:857:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const HSHELL_WINDOWACTIVATED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:860:9, winuser.h:860:9, winuser.h:860:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const HSHELL_GETMINRECT: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:861:9, winuser.h:861:9, winuser.h:861:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const HSHELL_REDRAW: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:862:9, winuser.h:862:9, winuser.h:862:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const HSHELL_TASKMAN: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:863:9, winuser.h:863:9, winuser.h:863:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const HSHELL_LANGUAGE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:864:9, winuser.h:864:9, winuser.h:864:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const HSHELL_SYSMENU: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:865:9, winuser.h:865:9, winuser.h:865:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const HSHELL_ENDTASK: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:866:9, winuser.h:866:9, winuser.h:866:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const HSHELL_ACCESSIBILITYSTATE: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:869:9, winuser.h:869:9, winuser.h:869:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const HSHELL_APPCOMMAND: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winuser.h:870:9, winuser.h:870:9, winuser.h:870:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const HSHELL_WINDOWREPLACED: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winuser.h:874:9, winuser.h:874:9, winuser.h:874:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const HSHELL_WINDOWREPLACING: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winuser.h:875:9, winuser.h:875:9, winuser.h:875:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const HSHELL_MONITORCHANGED: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:880:9, winuser.h:880:9, winuser.h:880:9 */
pub const HSHELL_HIGHBIT: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:884:9, winuser.h:884:9, winuser.h:884:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_BROWSER_BACKWARD: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:890:9, winuser.h:890:9, winuser.h:890:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_BROWSER_FORWARD: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:891:9, winuser.h:891:9, winuser.h:891:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_BROWSER_REFRESH: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:892:9, winuser.h:892:9, winuser.h:892:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_BROWSER_STOP: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:893:9, winuser.h:893:9, winuser.h:893:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_BROWSER_SEARCH: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:894:9, winuser.h:894:9, winuser.h:894:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_BROWSER_FAVORITES: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:895:9, winuser.h:895:9, winuser.h:895:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_BROWSER_HOME: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:896:9, winuser.h:896:9, winuser.h:896:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_VOLUME_MUTE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:897:9, winuser.h:897:9, winuser.h:897:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_VOLUME_DOWN: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:898:9, winuser.h:898:9, winuser.h:898:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_VOLUME_UP: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:899:9, winuser.h:899:9, winuser.h:899:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_MEDIA_NEXTTRACK: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:900:9, winuser.h:900:9, winuser.h:900:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_MEDIA_PREVIOUSTRACK: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winuser.h:901:9, winuser.h:901:9, winuser.h:901:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_MEDIA_STOP: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winuser.h:902:9, winuser.h:902:9, winuser.h:902:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_MEDIA_PLAY_PAUSE: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winuser.h:903:9, winuser.h:903:9, winuser.h:903:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_LAUNCH_MAIL: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winuser.h:904:9, winuser.h:904:9, winuser.h:904:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_LAUNCH_MEDIA_SELECT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:905:9, winuser.h:905:9, winuser.h:905:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_LAUNCH_APP1: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winuser.h:906:9, winuser.h:906:9, winuser.h:906:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_LAUNCH_APP2: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winuser.h:907:9, winuser.h:907:9, winuser.h:907:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_BASS_DOWN: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winuser.h:908:9, winuser.h:908:9, winuser.h:908:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_BASS_BOOST: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winuser.h:909:9, winuser.h:909:9, winuser.h:909:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_BASS_UP: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winuser.h:910:9, winuser.h:910:9, winuser.h:910:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_TREBLE_DOWN: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winuser.h:911:9, winuser.h:911:9, winuser.h:911:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const APPCOMMAND_TREBLE_UP: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winuser.h:912:9, winuser.h:912:9, winuser.h:912:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_MICROPHONE_VOLUME_MUTE: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winuser.h:914:9, winuser.h:914:9, winuser.h:914:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_MICROPHONE_VOLUME_DOWN: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* winuser.h:915:9, winuser.h:915:9, winuser.h:915:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_MICROPHONE_VOLUME_UP: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* winuser.h:916:9, winuser.h:916:9, winuser.h:916:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_HELP: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* winuser.h:917:9, winuser.h:917:9, winuser.h:917:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_FIND: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* winuser.h:918:9, winuser.h:918:9, winuser.h:918:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_NEW: i32 = 0x1di32; /* Integer(29, Yes, Unknown) */ /* winuser.h:919:9, winuser.h:919:9, winuser.h:919:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_OPEN: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* winuser.h:920:9, winuser.h:920:9, winuser.h:920:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_CLOSE: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* winuser.h:921:9, winuser.h:921:9, winuser.h:921:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_SAVE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:922:9, winuser.h:922:9, winuser.h:922:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_PRINT: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* winuser.h:923:9, winuser.h:923:9, winuser.h:923:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_UNDO: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* winuser.h:924:9, winuser.h:924:9, winuser.h:924:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_REDO: i32 = 0x23i32; /* Integer(35, Yes, Unknown) */ /* winuser.h:925:9, winuser.h:925:9, winuser.h:925:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_COPY: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* winuser.h:926:9, winuser.h:926:9, winuser.h:926:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_CUT: i32 = 0x25i32; /* Integer(37, Yes, Unknown) */ /* winuser.h:927:9, winuser.h:927:9, winuser.h:927:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_PASTE: i32 = 0x26i32; /* Integer(38, Yes, Unknown) */ /* winuser.h:928:9, winuser.h:928:9, winuser.h:928:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_REPLY_TO_MAIL: i32 = 0x27i32; /* Integer(39, Yes, Unknown) */ /* winuser.h:929:9, winuser.h:929:9, winuser.h:929:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_FORWARD_MAIL: i32 = 0x28i32; /* Integer(40, Yes, Unknown) */ /* winuser.h:930:9, winuser.h:930:9, winuser.h:930:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_SEND_MAIL: i32 = 0x29i32; /* Integer(41, Yes, Unknown) */ /* winuser.h:931:9, winuser.h:931:9, winuser.h:931:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_SPELL_CHECK: i32 = 0x2ai32; /* Integer(42, Yes, Unknown) */ /* winuser.h:932:9, winuser.h:932:9, winuser.h:932:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_DICTATE_OR_COMMAND_CONTROL_TOGGLE: i32 = 0x2bi32; /* Integer(43, Yes, Unknown) */ /* winuser.h:933:9, winuser.h:933:9, winuser.h:933:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_MIC_ON_OFF_TOGGLE: i32 = 0x2ci32; /* Integer(44, Yes, Unknown) */ /* winuser.h:934:9, winuser.h:934:9, winuser.h:934:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_CORRECTION_LIST: i32 = 0x2di32; /* Integer(45, Yes, Unknown) */ /* winuser.h:935:9, winuser.h:935:9, winuser.h:935:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_MEDIA_PLAY: i32 = 0x2ei32; /* Integer(46, Yes, Unknown) */ /* winuser.h:936:9, winuser.h:936:9, winuser.h:936:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_MEDIA_PAUSE: i32 = 0x2fi32; /* Integer(47, Yes, Unknown) */ /* winuser.h:937:9, winuser.h:937:9, winuser.h:937:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_MEDIA_RECORD: i32 = 0x30i32; /* Integer(48, Yes, Unknown) */ /* winuser.h:938:9, winuser.h:938:9, winuser.h:938:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_MEDIA_FAST_FORWARD: i32 = 0x31i32; /* Integer(49, Yes, Unknown) */ /* winuser.h:939:9, winuser.h:939:9, winuser.h:939:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_MEDIA_REWIND: i32 = 0x32i32; /* Integer(50, Yes, Unknown) */ /* winuser.h:940:9, winuser.h:940:9, winuser.h:940:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_MEDIA_CHANNEL_UP: i32 = 0x33i32; /* Integer(51, Yes, Unknown) */ /* winuser.h:941:9, winuser.h:941:9, winuser.h:941:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const APPCOMMAND_MEDIA_CHANNEL_DOWN: i32 = 0x34i32; /* Integer(52, Yes, Unknown) */ /* winuser.h:942:9, winuser.h:942:9, winuser.h:942:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const APPCOMMAND_DELETE: i32 = 0x35i32; /* Integer(53, Yes, Unknown) */ /* winuser.h:945:9, winuser.h:945:9, winuser.h:945:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const APPCOMMAND_DWM_FLIP3D: i32 = 0x36i32; /* Integer(54, Yes, Unknown) */ /* winuser.h:946:9, winuser.h:946:9, winuser.h:946:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const FAPPCOMMAND_MOUSE: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:949:9, winuser.h:949:9, winuser.h:949:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const FAPPCOMMAND_KEY: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:950:9, winuser.h:950:9, winuser.h:950:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const FAPPCOMMAND_OEM: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:951:9, winuser.h:951:9, winuser.h:951:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const FAPPCOMMAND_MASK: i32 = 0xf000i32; /* Integer(61440, Yes, Unknown) */ /* winuser.h:952:9, winuser.h:952:9, winuser.h:952:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LLKHF_INJECTED: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:1017:9, winuser.h:1017:9, winuser.h:1017:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LLKHF_LOWER_IL_INJECTED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:1020:9, winuser.h:1020:9, winuser.h:1020:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LLMHF_INJECTED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:1022:9, winuser.h:1022:9, winuser.h:1022:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LLMHF_LOWER_IL_INJECTED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:1023:9, winuser.h:1023:9, winuser.h:1023:9 */
pub const HKL_PREV: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:1115:9, winuser.h:1115:9, winuser.h:1115:9 */
pub const HKL_NEXT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:1116:9, winuser.h:1116:9, winuser.h:1116:9 */
pub const KLF_ACTIVATE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:1119:9, winuser.h:1119:9, winuser.h:1119:9 */
pub const KLF_SUBSTITUTE_OK: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:1120:9, winuser.h:1120:9, winuser.h:1120:9 */
pub const KLF_REORDER: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:1121:9, winuser.h:1121:9, winuser.h:1121:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const KLF_REPLACELANG: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:1123:9, winuser.h:1123:9, winuser.h:1123:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const KLF_NOTELLSHELL: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:1124:9, winuser.h:1124:9, winuser.h:1124:9 */
pub const KLF_SETFORPROCESS: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:1126:9, winuser.h:1126:9, winuser.h:1126:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const KLF_SHIFTLOCK: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winuser.h:1128:9, winuser.h:1128:9, winuser.h:1128:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const KLF_RESET: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* winuser.h:1129:9, winuser.h:1129:9, winuser.h:1129:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const INPUTLANGCHANGE_SYSCHARSET: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:1137:9, winuser.h:1137:9, winuser.h:1137:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const INPUTLANGCHANGE_FORWARD: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:1138:9, winuser.h:1138:9, winuser.h:1138:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const INPUTLANGCHANGE_BACKWARD: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:1139:9, winuser.h:1139:9, winuser.h:1139:9 */
pub const KL_NAMELENGTH: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:1145:9, winuser.h:1145:9, winuser.h:1145:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const GMMP_USE_DISPLAY_POINTS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:1259:9, winuser.h:1259:9, winuser.h:1259:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const GMMP_USE_HIGH_RESOLUTION_POINTS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:1260:9, winuser.h:1260:9, winuser.h:1260:9 */
pub const DESKTOP_READOBJECTS: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:1284:9, winuser.h:1284:9, winuser.h:1284:9 */
pub const DESKTOP_CREATEWINDOW: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:1285:9, winuser.h:1285:9, winuser.h:1285:9 */
pub const DESKTOP_CREATEMENU: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:1286:9, winuser.h:1286:9, winuser.h:1286:9 */
pub const DESKTOP_HOOKCONTROL: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winuser.h:1287:9, winuser.h:1287:9, winuser.h:1287:9 */
pub const DESKTOP_JOURNALRECORD: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* winuser.h:1288:9, winuser.h:1288:9, winuser.h:1288:9 */
pub const DESKTOP_JOURNALPLAYBACK: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* winuser.h:1289:9, winuser.h:1289:9, winuser.h:1289:9 */
pub const DESKTOP_ENUMERATE: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* winuser.h:1290:9, winuser.h:1290:9, winuser.h:1290:9 */
pub const DESKTOP_WRITEOBJECTS: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* winuser.h:1291:9, winuser.h:1291:9, winuser.h:1291:9 */
pub const DESKTOP_SWITCHDESKTOP: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* winuser.h:1292:9, winuser.h:1292:9, winuser.h:1292:9 */
pub const DF_ALLOWOTHERACCOUNTHOOK: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:1297:9, winuser.h:1297:9, winuser.h:1297:9 */
pub const WINSTA_ENUMDESKTOPS: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:1464:9, winuser.h:1464:9, winuser.h:1464:9 */
pub const WINSTA_READATTRIBUTES: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:1465:9, winuser.h:1465:9, winuser.h:1465:9 */
pub const WINSTA_ACCESSCLIPBOARD: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:1466:9, winuser.h:1466:9, winuser.h:1466:9 */
pub const WINSTA_CREATEDESKTOP: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winuser.h:1467:9, winuser.h:1467:9, winuser.h:1467:9 */
pub const WINSTA_WRITEATTRIBUTES: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* winuser.h:1468:9, winuser.h:1468:9, winuser.h:1468:9 */
pub const WINSTA_ACCESSGLOBALATOMS: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* winuser.h:1469:9, winuser.h:1469:9, winuser.h:1469:9 */
pub const WINSTA_EXITWINDOWS: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* winuser.h:1470:9, winuser.h:1470:9, winuser.h:1470:9 */
pub const WINSTA_ENUMERATE: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* winuser.h:1471:9, winuser.h:1471:9, winuser.h:1471:9 */
pub const WINSTA_READSCREEN: i64 = 0x200i64; /* Integer(512, Yes, Long) */ /* winuser.h:1472:9, winuser.h:1472:9, winuser.h:1472:9 */
pub const CWF_CREATE_ONLY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:1481:9, winuser.h:1481:9, winuser.h:1481:9 */
pub const WSF_VISIBLE: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:1486:9, winuser.h:1486:9, winuser.h:1486:9 */
pub const UOI_FLAGS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:1600:9, winuser.h:1600:9, winuser.h:1600:9 */
pub const UOI_NAME: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:1601:9, winuser.h:1601:9, winuser.h:1601:9 */
pub const UOI_TYPE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:1602:9, winuser.h:1602:9, winuser.h:1602:9 */
pub const UOI_USER_SID: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:1603:9, winuser.h:1603:9, winuser.h:1603:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const UOI_HEAPSIZE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:1605:9, winuser.h:1605:9, winuser.h:1605:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const UOI_IO: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:1606:9, winuser.h:1606:9, winuser.h:1606:9 */
pub const WM_NULL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:1883:9, winuser.h:1883:9, winuser.h:1883:9 */
pub const WM_CREATE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:1884:9, winuser.h:1884:9, winuser.h:1884:9 */
pub const WM_DESTROY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:1885:9, winuser.h:1885:9, winuser.h:1885:9 */
pub const WM_MOVE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:1886:9, winuser.h:1886:9, winuser.h:1886:9 */
pub const WM_SIZE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:1887:9, winuser.h:1887:9, winuser.h:1887:9 */
pub const WM_ACTIVATE: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:1889:9, winuser.h:1889:9, winuser.h:1889:9 */
pub const WA_INACTIVE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:1893:13, winuser.h:1893:13, winuser.h:1893:13 */
pub const WA_ACTIVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:1894:13, winuser.h:1894:13, winuser.h:1894:13 */
pub const WA_CLICKACTIVE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:1895:13, winuser.h:1895:13, winuser.h:1895:13 */
pub const WM_SETFOCUS: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:1897:9, winuser.h:1897:9, winuser.h:1897:9 */
pub const WM_KILLFOCUS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:1898:9, winuser.h:1898:9, winuser.h:1898:9 */
pub const WM_ENABLE: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:1899:9, winuser.h:1899:9, winuser.h:1899:9 */
pub const WM_SETREDRAW: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:1900:9, winuser.h:1900:9, winuser.h:1900:9 */
pub const WM_SETTEXT: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winuser.h:1901:9, winuser.h:1901:9, winuser.h:1901:9 */
pub const WM_GETTEXT: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winuser.h:1902:9, winuser.h:1902:9, winuser.h:1902:9 */
pub const WM_GETTEXTLENGTH: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winuser.h:1903:9, winuser.h:1903:9, winuser.h:1903:9 */
pub const WM_PAINT: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winuser.h:1904:9, winuser.h:1904:9, winuser.h:1904:9 */
pub const WM_CLOSE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:1905:9, winuser.h:1905:9, winuser.h:1905:9 */
pub const WM_QUERYENDSESSION: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winuser.h:1907:9, winuser.h:1907:9, winuser.h:1907:9 */
pub const WM_QUERYOPEN: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winuser.h:1908:9, winuser.h:1908:9, winuser.h:1908:9 */
pub const WM_ENDSESSION: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winuser.h:1909:9, winuser.h:1909:9, winuser.h:1909:9 */
pub const WM_QUIT: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winuser.h:1911:9, winuser.h:1911:9, winuser.h:1911:9 */
pub const WM_ERASEBKGND: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winuser.h:1912:9, winuser.h:1912:9, winuser.h:1912:9 */
pub const WM_SYSCOLORCHANGE: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winuser.h:1913:9, winuser.h:1913:9, winuser.h:1913:9 */
pub const WM_SHOWWINDOW: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winuser.h:1914:9, winuser.h:1914:9, winuser.h:1914:9 */
pub const WM_WININICHANGE: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* winuser.h:1915:9, winuser.h:1915:9, winuser.h:1915:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::WM_WININICHANGE as WM_SETTINGCHANGE; /* winuser.h:1917:9, winuser.h:1917:9, winuser.h:1917:9 */
pub const WM_DEVMODECHANGE: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* winuser.h:1921:9, winuser.h:1921:9, winuser.h:1921:9 */
pub const WM_ACTIVATEAPP: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* winuser.h:1922:9, winuser.h:1922:9, winuser.h:1922:9 */
pub const WM_FONTCHANGE: i32 = 0x1di32; /* Integer(29, Yes, Unknown) */ /* winuser.h:1923:9, winuser.h:1923:9, winuser.h:1923:9 */
pub const WM_TIMECHANGE: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* winuser.h:1924:9, winuser.h:1924:9, winuser.h:1924:9 */
pub const WM_CANCELMODE: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* winuser.h:1925:9, winuser.h:1925:9, winuser.h:1925:9 */
pub const WM_SETCURSOR: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:1926:9, winuser.h:1926:9, winuser.h:1926:9 */
pub const WM_MOUSEACTIVATE: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* winuser.h:1927:9, winuser.h:1927:9, winuser.h:1927:9 */
pub const WM_CHILDACTIVATE: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* winuser.h:1928:9, winuser.h:1928:9, winuser.h:1928:9 */
pub const WM_QUEUESYNC: i32 = 0x23i32; /* Integer(35, Yes, Unknown) */ /* winuser.h:1929:9, winuser.h:1929:9, winuser.h:1929:9 */
pub const WM_GETMINMAXINFO: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* winuser.h:1931:9, winuser.h:1931:9, winuser.h:1931:9 */
pub const WM_PAINTICON: i32 = 0x26i32; /* Integer(38, Yes, Unknown) */ /* winuser.h:1950:9, winuser.h:1950:9, winuser.h:1950:9 */
pub const WM_ICONERASEBKGND: i32 = 0x27i32; /* Integer(39, Yes, Unknown) */ /* winuser.h:1951:9, winuser.h:1951:9, winuser.h:1951:9 */
pub const WM_NEXTDLGCTL: i32 = 0x28i32; /* Integer(40, Yes, Unknown) */ /* winuser.h:1952:9, winuser.h:1952:9, winuser.h:1952:9 */
pub const WM_SPOOLERSTATUS: i32 = 0x2ai32; /* Integer(42, Yes, Unknown) */ /* winuser.h:1953:9, winuser.h:1953:9, winuser.h:1953:9 */
pub const WM_DRAWITEM: i32 = 0x2bi32; /* Integer(43, Yes, Unknown) */ /* winuser.h:1954:9, winuser.h:1954:9, winuser.h:1954:9 */
pub const WM_MEASUREITEM: i32 = 0x2ci32; /* Integer(44, Yes, Unknown) */ /* winuser.h:1955:9, winuser.h:1955:9, winuser.h:1955:9 */
pub const WM_DELETEITEM: i32 = 0x2di32; /* Integer(45, Yes, Unknown) */ /* winuser.h:1956:9, winuser.h:1956:9, winuser.h:1956:9 */
pub const WM_VKEYTOITEM: i32 = 0x2ei32; /* Integer(46, Yes, Unknown) */ /* winuser.h:1957:9, winuser.h:1957:9, winuser.h:1957:9 */
pub const WM_CHARTOITEM: i32 = 0x2fi32; /* Integer(47, Yes, Unknown) */ /* winuser.h:1958:9, winuser.h:1958:9, winuser.h:1958:9 */
pub const WM_SETFONT: i32 = 0x30i32; /* Integer(48, Yes, Unknown) */ /* winuser.h:1959:9, winuser.h:1959:9, winuser.h:1959:9 */
pub const WM_GETFONT: i32 = 0x31i32; /* Integer(49, Yes, Unknown) */ /* winuser.h:1960:9, winuser.h:1960:9, winuser.h:1960:9 */
pub const WM_SETHOTKEY: i32 = 0x32i32; /* Integer(50, Yes, Unknown) */ /* winuser.h:1961:9, winuser.h:1961:9, winuser.h:1961:9 */
pub const WM_GETHOTKEY: i32 = 0x33i32; /* Integer(51, Yes, Unknown) */ /* winuser.h:1962:9, winuser.h:1962:9, winuser.h:1962:9 */
pub const WM_QUERYDRAGICON: i32 = 0x37i32; /* Integer(55, Yes, Unknown) */ /* winuser.h:1963:9, winuser.h:1963:9, winuser.h:1963:9 */
pub const WM_COMPAREITEM: i32 = 0x39i32; /* Integer(57, Yes, Unknown) */ /* winuser.h:1964:9, winuser.h:1964:9, winuser.h:1964:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_GETOBJECT: i32 = 0x3di32; /* Integer(61, Yes, Unknown) */ /* winuser.h:1967:9, winuser.h:1967:9, winuser.h:1967:9 */
pub const WM_COMPACTING: i32 = 0x41i32; /* Integer(65, Yes, Unknown) */ /* winuser.h:1970:9, winuser.h:1970:9, winuser.h:1970:9 */
pub const WM_COMMNOTIFY: i32 = 0x44i32; /* Integer(68, Yes, Unknown) */ /* winuser.h:1971:9, winuser.h:1971:9, winuser.h:1971:9 */
pub const WM_WINDOWPOSCHANGING: i32 = 0x46i32; /* Integer(70, Yes, Unknown) */ /* winuser.h:1972:9, winuser.h:1972:9, winuser.h:1972:9 */
pub const WM_WINDOWPOSCHANGED: i32 = 0x47i32; /* Integer(71, Yes, Unknown) */ /* winuser.h:1973:9, winuser.h:1973:9, winuser.h:1973:9 */
pub const WM_POWER: i32 = 0x48i32; /* Integer(72, Yes, Unknown) */ /* winuser.h:1975:9, winuser.h:1975:9, winuser.h:1975:9 */
pub const PWR_OK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:1979:9, winuser.h:1979:9, winuser.h:1979:9 */
pub const PWR_SUSPENDREQUEST: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:1981:9, winuser.h:1981:9, winuser.h:1981:9 */
pub const PWR_SUSPENDRESUME: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:1982:9, winuser.h:1982:9, winuser.h:1982:9 */
pub const PWR_CRITICALRESUME: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:1983:9, winuser.h:1983:9, winuser.h:1983:9 */
pub const WM_COPYDATA: i32 = 0x4ai32; /* Integer(74, Yes, Unknown) */ /* winuser.h:1985:9, winuser.h:1985:9, winuser.h:1985:9 */
pub const WM_CANCELJOURNAL: i32 = 0x4bi32; /* Integer(75, Yes, Unknown) */ /* winuser.h:1986:9, winuser.h:1986:9, winuser.h:1986:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_NOTIFY: i32 = 0x4ei32; /* Integer(78, Yes, Unknown) */ /* winuser.h:2015:9, winuser.h:2015:9, winuser.h:2015:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_INPUTLANGCHANGEREQUEST: i32 = 0x50i32; /* Integer(80, Yes, Unknown) */ /* winuser.h:2016:9, winuser.h:2016:9, winuser.h:2016:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_INPUTLANGCHANGE: i32 = 0x51i32; /* Integer(81, Yes, Unknown) */ /* winuser.h:2017:9, winuser.h:2017:9, winuser.h:2017:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_TCARD: i32 = 0x52i32; /* Integer(82, Yes, Unknown) */ /* winuser.h:2018:9, winuser.h:2018:9, winuser.h:2018:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_HELP: i32 = 0x53i32; /* Integer(83, Yes, Unknown) */ /* winuser.h:2019:9, winuser.h:2019:9, winuser.h:2019:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_USERCHANGED: i32 = 0x54i32; /* Integer(84, Yes, Unknown) */ /* winuser.h:2020:9, winuser.h:2020:9, winuser.h:2020:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_NOTIFYFORMAT: i32 = 0x55i32; /* Integer(85, Yes, Unknown) */ /* winuser.h:2021:9, winuser.h:2021:9, winuser.h:2021:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const NFR_ANSI: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2023:9, winuser.h:2023:9, winuser.h:2023:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const NFR_UNICODE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2024:9, winuser.h:2024:9, winuser.h:2024:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const NF_QUERY: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:2025:9, winuser.h:2025:9, winuser.h:2025:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const NF_REQUERY: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:2026:9, winuser.h:2026:9, winuser.h:2026:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_CONTEXTMENU: i32 = 0x7bi32; /* Integer(123, Yes, Unknown) */ /* winuser.h:2028:9, winuser.h:2028:9, winuser.h:2028:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_STYLECHANGING: i32 = 0x7ci32; /* Integer(124, Yes, Unknown) */ /* winuser.h:2029:9, winuser.h:2029:9, winuser.h:2029:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_STYLECHANGED: i32 = 0x7di32; /* Integer(125, Yes, Unknown) */ /* winuser.h:2030:9, winuser.h:2030:9, winuser.h:2030:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_DISPLAYCHANGE: i32 = 0x7ei32; /* Integer(126, Yes, Unknown) */ /* winuser.h:2031:9, winuser.h:2031:9, winuser.h:2031:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_GETICON: i32 = 0x7fi32; /* Integer(127, Yes, Unknown) */ /* winuser.h:2032:9, winuser.h:2032:9, winuser.h:2032:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_SETICON: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:2033:9, winuser.h:2033:9, winuser.h:2033:9 */
pub const WM_NCCREATE: i32 = 0x81i32; /* Integer(129, Yes, Unknown) */ /* winuser.h:2036:9, winuser.h:2036:9, winuser.h:2036:9 */
pub const WM_NCDESTROY: i32 = 0x82i32; /* Integer(130, Yes, Unknown) */ /* winuser.h:2037:9, winuser.h:2037:9, winuser.h:2037:9 */
pub const WM_NCCALCSIZE: i32 = 0x83i32; /* Integer(131, Yes, Unknown) */ /* winuser.h:2038:9, winuser.h:2038:9, winuser.h:2038:9 */
pub const WM_NCHITTEST: i32 = 0x84i32; /* Integer(132, Yes, Unknown) */ /* winuser.h:2039:9, winuser.h:2039:9, winuser.h:2039:9 */
pub const WM_NCPAINT: i32 = 0x85i32; /* Integer(133, Yes, Unknown) */ /* winuser.h:2040:9, winuser.h:2040:9, winuser.h:2040:9 */
pub const WM_NCACTIVATE: i32 = 0x86i32; /* Integer(134, Yes, Unknown) */ /* winuser.h:2041:9, winuser.h:2041:9, winuser.h:2041:9 */
pub const WM_GETDLGCODE: i32 = 0x87i32; /* Integer(135, Yes, Unknown) */ /* winuser.h:2042:9, winuser.h:2042:9, winuser.h:2042:9 */
pub const WM_SYNCPAINT: i32 = 0x88i32; /* Integer(136, Yes, Unknown) */ /* winuser.h:2044:9, winuser.h:2044:9, winuser.h:2044:9 */
pub const WM_NCMOUSEMOVE: i32 = 0xa0i32; /* Integer(160, Yes, Unknown) */ /* winuser.h:2046:9, winuser.h:2046:9, winuser.h:2046:9 */
pub const WM_NCLBUTTONDOWN: i32 = 0xa1i32; /* Integer(161, Yes, Unknown) */ /* winuser.h:2047:9, winuser.h:2047:9, winuser.h:2047:9 */
pub const WM_NCLBUTTONUP: i32 = 0xa2i32; /* Integer(162, Yes, Unknown) */ /* winuser.h:2048:9, winuser.h:2048:9, winuser.h:2048:9 */
pub const WM_NCLBUTTONDBLCLK: i32 = 0xa3i32; /* Integer(163, Yes, Unknown) */ /* winuser.h:2049:9, winuser.h:2049:9, winuser.h:2049:9 */
pub const WM_NCRBUTTONDOWN: i32 = 0xa4i32; /* Integer(164, Yes, Unknown) */ /* winuser.h:2050:9, winuser.h:2050:9, winuser.h:2050:9 */
pub const WM_NCRBUTTONUP: i32 = 0xa5i32; /* Integer(165, Yes, Unknown) */ /* winuser.h:2051:9, winuser.h:2051:9, winuser.h:2051:9 */
pub const WM_NCRBUTTONDBLCLK: i32 = 0xa6i32; /* Integer(166, Yes, Unknown) */ /* winuser.h:2052:9, winuser.h:2052:9, winuser.h:2052:9 */
pub const WM_NCMBUTTONDOWN: i32 = 0xa7i32; /* Integer(167, Yes, Unknown) */ /* winuser.h:2053:9, winuser.h:2053:9, winuser.h:2053:9 */
pub const WM_NCMBUTTONUP: i32 = 0xa8i32; /* Integer(168, Yes, Unknown) */ /* winuser.h:2054:9, winuser.h:2054:9, winuser.h:2054:9 */
pub const WM_NCMBUTTONDBLCLK: i32 = 0xa9i32; /* Integer(169, Yes, Unknown) */ /* winuser.h:2055:9, winuser.h:2055:9, winuser.h:2055:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_NCXBUTTONDOWN: i32 = 0xabi32; /* Integer(171, Yes, Unknown) */ /* winuser.h:2060:9, winuser.h:2060:9, winuser.h:2060:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_NCXBUTTONUP: i32 = 0xaci32; /* Integer(172, Yes, Unknown) */ /* winuser.h:2061:9, winuser.h:2061:9, winuser.h:2061:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_NCXBUTTONDBLCLK: i32 = 0xadi32; /* Integer(173, Yes, Unknown) */ /* winuser.h:2062:9, winuser.h:2062:9, winuser.h:2062:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WM_INPUT_DEVICE_CHANGE: i32 = 0xfei32; /* Integer(254, Yes, Unknown) */ /* winuser.h:2067:9, winuser.h:2067:9, winuser.h:2067:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WM_INPUT: i32 = 0xffi32; /* Integer(255, Yes, Unknown) */ /* winuser.h:2071:9, winuser.h:2071:9, winuser.h:2071:9 */
pub const WM_KEYFIRST: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:2074:9, winuser.h:2074:9, winuser.h:2074:9 */
pub const WM_KEYDOWN: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:2075:9, winuser.h:2075:9, winuser.h:2075:9 */
pub const WM_KEYUP: i32 = 0x101i32; /* Integer(257, Yes, Unknown) */ /* winuser.h:2076:9, winuser.h:2076:9, winuser.h:2076:9 */
pub const WM_CHAR: i32 = 0x102i32; /* Integer(258, Yes, Unknown) */ /* winuser.h:2077:9, winuser.h:2077:9, winuser.h:2077:9 */
pub const WM_DEADCHAR: i32 = 0x103i32; /* Integer(259, Yes, Unknown) */ /* winuser.h:2078:9, winuser.h:2078:9, winuser.h:2078:9 */
pub const WM_SYSKEYDOWN: i32 = 0x104i32; /* Integer(260, Yes, Unknown) */ /* winuser.h:2079:9, winuser.h:2079:9, winuser.h:2079:9 */
pub const WM_SYSKEYUP: i32 = 0x105i32; /* Integer(261, Yes, Unknown) */ /* winuser.h:2080:9, winuser.h:2080:9, winuser.h:2080:9 */
pub const WM_SYSCHAR: i32 = 0x106i32; /* Integer(262, Yes, Unknown) */ /* winuser.h:2081:9, winuser.h:2081:9, winuser.h:2081:9 */
pub const WM_SYSDEADCHAR: i32 = 0x107i32; /* Integer(263, Yes, Unknown) */ /* winuser.h:2082:9, winuser.h:2082:9, winuser.h:2082:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WM_UNICHAR: i32 = 0x109i32; /* Integer(265, Yes, Unknown) */ /* winuser.h:2084:9, winuser.h:2084:9, winuser.h:2084:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WM_KEYLAST: i32 = 0x109i32; /* Integer(265, Yes, Unknown) */ /* winuser.h:2085:9, winuser.h:2085:9, winuser.h:2085:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const UNICODE_NOCHAR: i32 = 0xffffi32; /* Integer(65535, Yes, Unknown) */ /* winuser.h:2086:9, winuser.h:2086:9, winuser.h:2086:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_IME_STARTCOMPOSITION: i32 = 0x10di32; /* Integer(269, Yes, Unknown) */ /* winuser.h:2092:9, winuser.h:2092:9, winuser.h:2092:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_IME_ENDCOMPOSITION: i32 = 0x10ei32; /* Integer(270, Yes, Unknown) */ /* winuser.h:2093:9, winuser.h:2093:9, winuser.h:2093:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_IME_COMPOSITION: i32 = 0x10fi32; /* Integer(271, Yes, Unknown) */ /* winuser.h:2094:9, winuser.h:2094:9, winuser.h:2094:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_IME_KEYLAST: i32 = 0x10fi32; /* Integer(271, Yes, Unknown) */ /* winuser.h:2095:9, winuser.h:2095:9, winuser.h:2095:9 */
pub const WM_INITDIALOG: i32 = 0x110i32; /* Integer(272, Yes, Unknown) */ /* winuser.h:2098:9, winuser.h:2098:9, winuser.h:2098:9 */
pub const WM_COMMAND: i32 = 0x111i32; /* Integer(273, Yes, Unknown) */ /* winuser.h:2099:9, winuser.h:2099:9, winuser.h:2099:9 */
pub const WM_SYSCOMMAND: i32 = 0x112i32; /* Integer(274, Yes, Unknown) */ /* winuser.h:2100:9, winuser.h:2100:9, winuser.h:2100:9 */
pub const WM_TIMER: i32 = 0x113i32; /* Integer(275, Yes, Unknown) */ /* winuser.h:2101:9, winuser.h:2101:9, winuser.h:2101:9 */
pub const WM_HSCROLL: i32 = 0x114i32; /* Integer(276, Yes, Unknown) */ /* winuser.h:2102:9, winuser.h:2102:9, winuser.h:2102:9 */
pub const WM_VSCROLL: i32 = 0x115i32; /* Integer(277, Yes, Unknown) */ /* winuser.h:2103:9, winuser.h:2103:9, winuser.h:2103:9 */
pub const WM_INITMENU: i32 = 0x116i32; /* Integer(278, Yes, Unknown) */ /* winuser.h:2104:9, winuser.h:2104:9, winuser.h:2104:9 */
pub const WM_INITMENUPOPUP: i32 = 0x117i32; /* Integer(279, Yes, Unknown) */ /* winuser.h:2105:9, winuser.h:2105:9, winuser.h:2105:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const WM_GESTURE: i32 = 0x119i32; /* Integer(281, Yes, Unknown) */ /* winuser.h:2107:9, winuser.h:2107:9, winuser.h:2107:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const WM_GESTURENOTIFY: i32 = 0x11ai32; /* Integer(282, Yes, Unknown) */ /* winuser.h:2108:9, winuser.h:2108:9, winuser.h:2108:9 */
pub const WM_MENUSELECT: i32 = 0x11fi32; /* Integer(287, Yes, Unknown) */ /* winuser.h:2110:9, winuser.h:2110:9, winuser.h:2110:9 */
pub const WM_MENUCHAR: i32 = 0x120i32; /* Integer(288, Yes, Unknown) */ /* winuser.h:2111:9, winuser.h:2111:9, winuser.h:2111:9 */
pub const WM_ENTERIDLE: i32 = 0x121i32; /* Integer(289, Yes, Unknown) */ /* winuser.h:2112:9, winuser.h:2112:9, winuser.h:2112:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_MENURBUTTONUP: i32 = 0x122i32; /* Integer(290, Yes, Unknown) */ /* winuser.h:2115:9, winuser.h:2115:9, winuser.h:2115:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_MENUDRAG: i32 = 0x123i32; /* Integer(291, Yes, Unknown) */ /* winuser.h:2116:9, winuser.h:2116:9, winuser.h:2116:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_MENUGETOBJECT: i32 = 0x124i32; /* Integer(292, Yes, Unknown) */ /* winuser.h:2117:9, winuser.h:2117:9, winuser.h:2117:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_UNINITMENUPOPUP: i32 = 0x125i32; /* Integer(293, Yes, Unknown) */ /* winuser.h:2118:9, winuser.h:2118:9, winuser.h:2118:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_MENUCOMMAND: i32 = 0x126i32; /* Integer(294, Yes, Unknown) */ /* winuser.h:2119:9, winuser.h:2119:9, winuser.h:2119:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_CHANGEUISTATE: i32 = 0x127i32; /* Integer(295, Yes, Unknown) */ /* winuser.h:2123:9, winuser.h:2123:9, winuser.h:2123:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_UPDATEUISTATE: i32 = 0x128i32; /* Integer(296, Yes, Unknown) */ /* winuser.h:2124:9, winuser.h:2124:9, winuser.h:2124:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_QUERYUISTATE: i32 = 0x129i32; /* Integer(297, Yes, Unknown) */ /* winuser.h:2125:9, winuser.h:2125:9, winuser.h:2125:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const UIS_SET: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2130:9, winuser.h:2130:9, winuser.h:2130:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const UIS_CLEAR: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2131:9, winuser.h:2131:9, winuser.h:2131:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const UIS_INITIALIZE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:2132:9, winuser.h:2132:9, winuser.h:2132:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const UISF_HIDEFOCUS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2137:9, winuser.h:2137:9, winuser.h:2137:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const UISF_HIDEACCEL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2138:9, winuser.h:2138:9, winuser.h:2138:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const UISF_ACTIVE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:2140:9, winuser.h:2140:9, winuser.h:2140:9 */
pub const WM_CTLCOLORMSGBOX: i32 = 0x132i32; /* Integer(306, Yes, Unknown) */ /* winuser.h:2148:9, winuser.h:2148:9, winuser.h:2148:9 */
pub const WM_CTLCOLOREDIT: i32 = 0x133i32; /* Integer(307, Yes, Unknown) */ /* winuser.h:2149:9, winuser.h:2149:9, winuser.h:2149:9 */
pub const WM_CTLCOLORLISTBOX: i32 = 0x134i32; /* Integer(308, Yes, Unknown) */ /* winuser.h:2150:9, winuser.h:2150:9, winuser.h:2150:9 */
pub const WM_CTLCOLORBTN: i32 = 0x135i32; /* Integer(309, Yes, Unknown) */ /* winuser.h:2151:9, winuser.h:2151:9, winuser.h:2151:9 */
pub const WM_CTLCOLORDLG: i32 = 0x136i32; /* Integer(310, Yes, Unknown) */ /* winuser.h:2152:9, winuser.h:2152:9, winuser.h:2152:9 */
pub const WM_CTLCOLORSCROLLBAR: i32 = 0x137i32; /* Integer(311, Yes, Unknown) */ /* winuser.h:2153:9, winuser.h:2153:9, winuser.h:2153:9 */
pub const WM_CTLCOLORSTATIC: i32 = 0x138i32; /* Integer(312, Yes, Unknown) */ /* winuser.h:2154:9, winuser.h:2154:9, winuser.h:2154:9 */
pub const MN_GETHMENU: i32 = 0x1e1i32; /* Integer(481, Yes, Unknown) */ /* winuser.h:2155:9, winuser.h:2155:9, winuser.h:2155:9 */
pub const WM_MOUSEFIRST: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:2157:9, winuser.h:2157:9, winuser.h:2157:9 */
pub const WM_MOUSEMOVE: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:2158:9, winuser.h:2158:9, winuser.h:2158:9 */
pub const WM_LBUTTONDOWN: i32 = 0x201i32; /* Integer(513, Yes, Unknown) */ /* winuser.h:2159:9, winuser.h:2159:9, winuser.h:2159:9 */
pub const WM_LBUTTONUP: i32 = 0x202i32; /* Integer(514, Yes, Unknown) */ /* winuser.h:2160:9, winuser.h:2160:9, winuser.h:2160:9 */
pub const WM_LBUTTONDBLCLK: i32 = 0x203i32; /* Integer(515, Yes, Unknown) */ /* winuser.h:2161:9, winuser.h:2161:9, winuser.h:2161:9 */
pub const WM_RBUTTONDOWN: i32 = 0x204i32; /* Integer(516, Yes, Unknown) */ /* winuser.h:2162:9, winuser.h:2162:9, winuser.h:2162:9 */
pub const WM_RBUTTONUP: i32 = 0x205i32; /* Integer(517, Yes, Unknown) */ /* winuser.h:2163:9, winuser.h:2163:9, winuser.h:2163:9 */
pub const WM_RBUTTONDBLCLK: i32 = 0x206i32; /* Integer(518, Yes, Unknown) */ /* winuser.h:2164:9, winuser.h:2164:9, winuser.h:2164:9 */
pub const WM_MBUTTONDOWN: i32 = 0x207i32; /* Integer(519, Yes, Unknown) */ /* winuser.h:2165:9, winuser.h:2165:9, winuser.h:2165:9 */
pub const WM_MBUTTONUP: i32 = 0x208i32; /* Integer(520, Yes, Unknown) */ /* winuser.h:2166:9, winuser.h:2166:9, winuser.h:2166:9 */
pub const WM_MBUTTONDBLCLK: i32 = 0x209i32; /* Integer(521, Yes, Unknown) */ /* winuser.h:2167:9, winuser.h:2167:9, winuser.h:2167:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_MOUSEWHEEL: i32 = 0x20ai32; /* Integer(522, Yes, Unknown) */ /* winuser.h:2169:9, winuser.h:2169:9, winuser.h:2169:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_XBUTTONDOWN: i32 = 0x20bi32; /* Integer(523, Yes, Unknown) */ /* winuser.h:2172:9, winuser.h:2172:9, winuser.h:2172:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_XBUTTONUP: i32 = 0x20ci32; /* Integer(524, Yes, Unknown) */ /* winuser.h:2173:9, winuser.h:2173:9, winuser.h:2173:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_XBUTTONDBLCLK: i32 = 0x20di32; /* Integer(525, Yes, Unknown) */ /* winuser.h:2174:9, winuser.h:2174:9, winuser.h:2174:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const WM_MOUSEHWHEEL: i32 = 0x20ei32; /* Integer(526, Yes, Unknown) */ /* winuser.h:2177:9, winuser.h:2177:9, winuser.h:2177:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const WM_MOUSELAST: i32 = 0x20ei32; /* Integer(526, Yes, Unknown) */ /* winuser.h:2181:9, winuser.h:2181:9, winuser.h:2181:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WHEEL_DELTA: i32 = 0x78i32; /* Integer(120, Yes, Unknown) */ /* winuser.h:2193:9, winuser.h:2193:9, winuser.h:2193:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const XBUTTON1: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2206:9, winuser.h:2206:9, winuser.h:2206:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const XBUTTON2: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2207:9, winuser.h:2207:9, winuser.h:2207:9 */
pub const WM_PARENTNOTIFY: i32 = 0x210i32; /* Integer(528, Yes, Unknown) */ /* winuser.h:2211:9, winuser.h:2211:9, winuser.h:2211:9 */
pub const WM_ENTERMENULOOP: i32 = 0x211i32; /* Integer(529, Yes, Unknown) */ /* winuser.h:2212:9, winuser.h:2212:9, winuser.h:2212:9 */
pub const WM_EXITMENULOOP: i32 = 0x212i32; /* Integer(530, Yes, Unknown) */ /* winuser.h:2213:9, winuser.h:2213:9, winuser.h:2213:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_NEXTMENU: i32 = 0x213i32; /* Integer(531, Yes, Unknown) */ /* winuser.h:2216:9, winuser.h:2216:9, winuser.h:2216:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_SIZING: i32 = 0x214i32; /* Integer(532, Yes, Unknown) */ /* winuser.h:2217:9, winuser.h:2217:9, winuser.h:2217:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_CAPTURECHANGED: i32 = 0x215i32; /* Integer(533, Yes, Unknown) */ /* winuser.h:2218:9, winuser.h:2218:9, winuser.h:2218:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_MOVING: i32 = 0x216i32; /* Integer(534, Yes, Unknown) */ /* winuser.h:2219:9, winuser.h:2219:9, winuser.h:2219:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_POWERBROADCAST: i32 = 0x218i32; /* Integer(536, Yes, Unknown) */ /* winuser.h:2225:9, winuser.h:2225:9, winuser.h:2225:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PBT_APMQUERYSUSPEND: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:2228:9, winuser.h:2228:9, winuser.h:2228:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PBT_APMQUERYSTANDBY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2229:9, winuser.h:2229:9, winuser.h:2229:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PBT_APMQUERYSUSPENDFAILED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2231:9, winuser.h:2231:9, winuser.h:2231:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PBT_APMQUERYSTANDBYFAILED: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:2232:9, winuser.h:2232:9, winuser.h:2232:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PBT_APMSUSPEND: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:2234:9, winuser.h:2234:9, winuser.h:2234:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PBT_APMSTANDBY: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:2235:9, winuser.h:2235:9, winuser.h:2235:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PBT_APMRESUMECRITICAL: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:2237:9, winuser.h:2237:9, winuser.h:2237:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PBT_APMRESUMESUSPEND: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:2238:9, winuser.h:2238:9, winuser.h:2238:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PBT_APMRESUMESTANDBY: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:2239:9, winuser.h:2239:9, winuser.h:2239:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PBTF_APMRESUMEFROMFAILURE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2241:9, winuser.h:2241:9, winuser.h:2241:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PBT_APMBATTERYLOW: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:2243:9, winuser.h:2243:9, winuser.h:2243:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PBT_APMPOWERSTATUSCHANGE: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:2244:9, winuser.h:2244:9, winuser.h:2244:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PBT_APMOEMEVENT: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:2246:9, winuser.h:2246:9, winuser.h:2246:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PBT_APMRESUMEAUTOMATIC: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winuser.h:2249:9, winuser.h:2249:9, winuser.h:2249:9 */
#[cfg(any(feature="winapi_ver_05020000"))] pub const PBT_POWERSETTINGCHANGE: i32 = 0x8013i32; /* Integer(32787, Yes, Unknown) */ /* winuser.h:2252:9, winuser.h:2252:9, winuser.h:2252:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_DEVICECHANGE: i32 = 0x219i32; /* Integer(537, Yes, Unknown) */ /* winuser.h:2275:9, winuser.h:2275:9, winuser.h:2275:9 */
pub const WM_MDICREATE: i32 = 0x220i32; /* Integer(544, Yes, Unknown) */ /* winuser.h:2278:9, winuser.h:2278:9, winuser.h:2278:9 */
pub const WM_MDIDESTROY: i32 = 0x221i32; /* Integer(545, Yes, Unknown) */ /* winuser.h:2279:9, winuser.h:2279:9, winuser.h:2279:9 */
pub const WM_MDIACTIVATE: i32 = 0x222i32; /* Integer(546, Yes, Unknown) */ /* winuser.h:2280:9, winuser.h:2280:9, winuser.h:2280:9 */
pub const WM_MDIRESTORE: i32 = 0x223i32; /* Integer(547, Yes, Unknown) */ /* winuser.h:2281:9, winuser.h:2281:9, winuser.h:2281:9 */
pub const WM_MDINEXT: i32 = 0x224i32; /* Integer(548, Yes, Unknown) */ /* winuser.h:2282:9, winuser.h:2282:9, winuser.h:2282:9 */
pub const WM_MDIMAXIMIZE: i32 = 0x225i32; /* Integer(549, Yes, Unknown) */ /* winuser.h:2283:9, winuser.h:2283:9, winuser.h:2283:9 */
pub const WM_MDITILE: i32 = 0x226i32; /* Integer(550, Yes, Unknown) */ /* winuser.h:2284:9, winuser.h:2284:9, winuser.h:2284:9 */
pub const WM_MDICASCADE: i32 = 0x227i32; /* Integer(551, Yes, Unknown) */ /* winuser.h:2285:9, winuser.h:2285:9, winuser.h:2285:9 */
pub const WM_MDIICONARRANGE: i32 = 0x228i32; /* Integer(552, Yes, Unknown) */ /* winuser.h:2286:9, winuser.h:2286:9, winuser.h:2286:9 */
pub const WM_MDIGETACTIVE: i32 = 0x229i32; /* Integer(553, Yes, Unknown) */ /* winuser.h:2287:9, winuser.h:2287:9, winuser.h:2287:9 */
pub const WM_MDISETMENU: i32 = 0x230i32; /* Integer(560, Yes, Unknown) */ /* winuser.h:2290:9, winuser.h:2290:9, winuser.h:2290:9 */
pub const WM_ENTERSIZEMOVE: i32 = 0x231i32; /* Integer(561, Yes, Unknown) */ /* winuser.h:2291:9, winuser.h:2291:9, winuser.h:2291:9 */
pub const WM_EXITSIZEMOVE: i32 = 0x232i32; /* Integer(562, Yes, Unknown) */ /* winuser.h:2292:9, winuser.h:2292:9, winuser.h:2292:9 */
pub const WM_DROPFILES: i32 = 0x233i32; /* Integer(563, Yes, Unknown) */ /* winuser.h:2293:9, winuser.h:2293:9, winuser.h:2293:9 */
pub const WM_MDIREFRESHMENU: i32 = 0x234i32; /* Integer(564, Yes, Unknown) */ /* winuser.h:2294:9, winuser.h:2294:9, winuser.h:2294:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_POINTERDEVICECHANGE: i32 = 0x238i32; /* Integer(568, Yes, Unknown) */ /* winuser.h:2297:9, winuser.h:2297:9, winuser.h:2297:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_POINTERDEVICEINRANGE: i32 = 0x239i32; /* Integer(569, Yes, Unknown) */ /* winuser.h:2298:9, winuser.h:2298:9, winuser.h:2298:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_POINTERDEVICEOUTOFRANGE: i32 = 0x23ai32; /* Integer(570, Yes, Unknown) */ /* winuser.h:2299:9, winuser.h:2299:9, winuser.h:2299:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const WM_TOUCH: i32 = 0x240i32; /* Integer(576, Yes, Unknown) */ /* winuser.h:2304:9, winuser.h:2304:9, winuser.h:2304:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_NCPOINTERUPDATE: i32 = 0x241i32; /* Integer(577, Yes, Unknown) */ /* winuser.h:2308:9, winuser.h:2308:9, winuser.h:2308:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_NCPOINTERDOWN: i32 = 0x242i32; /* Integer(578, Yes, Unknown) */ /* winuser.h:2309:9, winuser.h:2309:9, winuser.h:2309:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_NCPOINTERUP: i32 = 0x243i32; /* Integer(579, Yes, Unknown) */ /* winuser.h:2310:9, winuser.h:2310:9, winuser.h:2310:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_POINTERUPDATE: i32 = 0x245i32; /* Integer(581, Yes, Unknown) */ /* winuser.h:2311:9, winuser.h:2311:9, winuser.h:2311:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_POINTERDOWN: i32 = 0x246i32; /* Integer(582, Yes, Unknown) */ /* winuser.h:2312:9, winuser.h:2312:9, winuser.h:2312:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_POINTERUP: i32 = 0x247i32; /* Integer(583, Yes, Unknown) */ /* winuser.h:2313:9, winuser.h:2313:9, winuser.h:2313:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_POINTERENTER: i32 = 0x249i32; /* Integer(585, Yes, Unknown) */ /* winuser.h:2314:9, winuser.h:2314:9, winuser.h:2314:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_POINTERLEAVE: i32 = 0x24ai32; /* Integer(586, Yes, Unknown) */ /* winuser.h:2315:9, winuser.h:2315:9, winuser.h:2315:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_POINTERACTIVATE: i32 = 0x24bi32; /* Integer(587, Yes, Unknown) */ /* winuser.h:2316:9, winuser.h:2316:9, winuser.h:2316:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_POINTERCAPTURECHANGED: i32 = 0x24ci32; /* Integer(588, Yes, Unknown) */ /* winuser.h:2317:9, winuser.h:2317:9, winuser.h:2317:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_TOUCHHITTESTING: i32 = 0x24di32; /* Integer(589, Yes, Unknown) */ /* winuser.h:2318:9, winuser.h:2318:9, winuser.h:2318:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_POINTERWHEEL: i32 = 0x24ei32; /* Integer(590, Yes, Unknown) */ /* winuser.h:2319:9, winuser.h:2319:9, winuser.h:2319:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WM_POINTERHWHEEL: i32 = 0x24fi32; /* Integer(591, Yes, Unknown) */ /* winuser.h:2320:9, winuser.h:2320:9, winuser.h:2320:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const DM_POINTERHITTEST: i32 = 0x250i32; /* Integer(592, Yes, Unknown) */ /* winuser.h:2321:9, winuser.h:2321:9, winuser.h:2321:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_IME_SETCONTEXT: i32 = 0x281i32; /* Integer(641, Yes, Unknown) */ /* winuser.h:2326:9, winuser.h:2326:9, winuser.h:2326:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_IME_NOTIFY: i32 = 0x282i32; /* Integer(642, Yes, Unknown) */ /* winuser.h:2327:9, winuser.h:2327:9, winuser.h:2327:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_IME_CONTROL: i32 = 0x283i32; /* Integer(643, Yes, Unknown) */ /* winuser.h:2328:9, winuser.h:2328:9, winuser.h:2328:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_IME_COMPOSITIONFULL: i32 = 0x284i32; /* Integer(644, Yes, Unknown) */ /* winuser.h:2329:9, winuser.h:2329:9, winuser.h:2329:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_IME_SELECT: i32 = 0x285i32; /* Integer(645, Yes, Unknown) */ /* winuser.h:2330:9, winuser.h:2330:9, winuser.h:2330:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_IME_CHAR: i32 = 0x286i32; /* Integer(646, Yes, Unknown) */ /* winuser.h:2331:9, winuser.h:2331:9, winuser.h:2331:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_IME_REQUEST: i32 = 0x288i32; /* Integer(648, Yes, Unknown) */ /* winuser.h:2334:9, winuser.h:2334:9, winuser.h:2334:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_IME_KEYDOWN: i32 = 0x290i32; /* Integer(656, Yes, Unknown) */ /* winuser.h:2337:9, winuser.h:2337:9, winuser.h:2337:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_IME_KEYUP: i32 = 0x291i32; /* Integer(657, Yes, Unknown) */ /* winuser.h:2338:9, winuser.h:2338:9, winuser.h:2338:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_MOUSEHOVER: i32 = 0x2a1i32; /* Integer(673, Yes, Unknown) */ /* winuser.h:2342:9, winuser.h:2342:9, winuser.h:2342:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_MOUSELEAVE: i32 = 0x2a3i32; /* Integer(675, Yes, Unknown) */ /* winuser.h:2343:9, winuser.h:2343:9, winuser.h:2343:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_NCMOUSEHOVER: i32 = 0x2a0i32; /* Integer(672, Yes, Unknown) */ /* winuser.h:2346:9, winuser.h:2346:9, winuser.h:2346:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_NCMOUSELEAVE: i32 = 0x2a2i32; /* Integer(674, Yes, Unknown) */ /* winuser.h:2347:9, winuser.h:2347:9, winuser.h:2347:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WM_WTSSESSION_CHANGE: i32 = 0x2b1i32; /* Integer(689, Yes, Unknown) */ /* winuser.h:2351:9, winuser.h:2351:9, winuser.h:2351:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WM_TABLET_FIRST: i32 = 0x2c0i32; /* Integer(704, Yes, Unknown) */ /* winuser.h:2353:9, winuser.h:2353:9, winuser.h:2353:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WM_TABLET_LAST: i32 = 0x2dfi32; /* Integer(735, Yes, Unknown) */ /* winuser.h:2354:9, winuser.h:2354:9, winuser.h:2354:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const WM_DPICHANGED: i32 = 0x2e0i32; /* Integer(736, Yes, Unknown) */ /* winuser.h:2358:9, winuser.h:2358:9, winuser.h:2358:9 */
pub const WM_CUT: i32 = 0x300i32; /* Integer(768, Yes, Unknown) */ /* winuser.h:2361:9, winuser.h:2361:9, winuser.h:2361:9 */
pub const WM_COPY: i32 = 0x301i32; /* Integer(769, Yes, Unknown) */ /* winuser.h:2362:9, winuser.h:2362:9, winuser.h:2362:9 */
pub const WM_PASTE: i32 = 0x302i32; /* Integer(770, Yes, Unknown) */ /* winuser.h:2363:9, winuser.h:2363:9, winuser.h:2363:9 */
pub const WM_CLEAR: i32 = 0x303i32; /* Integer(771, Yes, Unknown) */ /* winuser.h:2364:9, winuser.h:2364:9, winuser.h:2364:9 */
pub const WM_UNDO: i32 = 0x304i32; /* Integer(772, Yes, Unknown) */ /* winuser.h:2365:9, winuser.h:2365:9, winuser.h:2365:9 */
pub const WM_RENDERFORMAT: i32 = 0x305i32; /* Integer(773, Yes, Unknown) */ /* winuser.h:2366:9, winuser.h:2366:9, winuser.h:2366:9 */
pub const WM_RENDERALLFORMATS: i32 = 0x306i32; /* Integer(774, Yes, Unknown) */ /* winuser.h:2367:9, winuser.h:2367:9, winuser.h:2367:9 */
pub const WM_DESTROYCLIPBOARD: i32 = 0x307i32; /* Integer(775, Yes, Unknown) */ /* winuser.h:2368:9, winuser.h:2368:9, winuser.h:2368:9 */
pub const WM_DRAWCLIPBOARD: i32 = 0x308i32; /* Integer(776, Yes, Unknown) */ /* winuser.h:2369:9, winuser.h:2369:9, winuser.h:2369:9 */
pub const WM_PAINTCLIPBOARD: i32 = 0x309i32; /* Integer(777, Yes, Unknown) */ /* winuser.h:2370:9, winuser.h:2370:9, winuser.h:2370:9 */
pub const WM_VSCROLLCLIPBOARD: i32 = 0x30ai32; /* Integer(778, Yes, Unknown) */ /* winuser.h:2371:9, winuser.h:2371:9, winuser.h:2371:9 */
pub const WM_SIZECLIPBOARD: i32 = 0x30bi32; /* Integer(779, Yes, Unknown) */ /* winuser.h:2372:9, winuser.h:2372:9, winuser.h:2372:9 */
pub const WM_ASKCBFORMATNAME: i32 = 0x30ci32; /* Integer(780, Yes, Unknown) */ /* winuser.h:2373:9, winuser.h:2373:9, winuser.h:2373:9 */
pub const WM_CHANGECBCHAIN: i32 = 0x30di32; /* Integer(781, Yes, Unknown) */ /* winuser.h:2374:9, winuser.h:2374:9, winuser.h:2374:9 */
pub const WM_HSCROLLCLIPBOARD: i32 = 0x30ei32; /* Integer(782, Yes, Unknown) */ /* winuser.h:2375:9, winuser.h:2375:9, winuser.h:2375:9 */
pub const WM_QUERYNEWPALETTE: i32 = 0x30fi32; /* Integer(783, Yes, Unknown) */ /* winuser.h:2376:9, winuser.h:2376:9, winuser.h:2376:9 */
pub const WM_PALETTEISCHANGING: i32 = 0x310i32; /* Integer(784, Yes, Unknown) */ /* winuser.h:2377:9, winuser.h:2377:9, winuser.h:2377:9 */
pub const WM_PALETTECHANGED: i32 = 0x311i32; /* Integer(785, Yes, Unknown) */ /* winuser.h:2378:9, winuser.h:2378:9, winuser.h:2378:9 */
pub const WM_HOTKEY: i32 = 0x312i32; /* Integer(786, Yes, Unknown) */ /* winuser.h:2379:9, winuser.h:2379:9, winuser.h:2379:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_PRINT: i32 = 0x317i32; /* Integer(791, Yes, Unknown) */ /* winuser.h:2382:9, winuser.h:2382:9, winuser.h:2382:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_PRINTCLIENT: i32 = 0x318i32; /* Integer(792, Yes, Unknown) */ /* winuser.h:2383:9, winuser.h:2383:9, winuser.h:2383:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WM_APPCOMMAND: i32 = 0x319i32; /* Integer(793, Yes, Unknown) */ /* winuser.h:2387:9, winuser.h:2387:9, winuser.h:2387:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WM_THEMECHANGED: i32 = 0x31ai32; /* Integer(794, Yes, Unknown) */ /* winuser.h:2391:9, winuser.h:2391:9, winuser.h:2391:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WM_CLIPBOARDUPDATE: i32 = 0x31di32; /* Integer(797, Yes, Unknown) */ /* winuser.h:2396:9, winuser.h:2396:9, winuser.h:2396:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const WM_DWMCOMPOSITIONCHANGED: i32 = 0x31ei32; /* Integer(798, Yes, Unknown) */ /* winuser.h:2400:9, winuser.h:2400:9, winuser.h:2400:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const WM_DWMNCRENDERINGCHANGED: i32 = 0x31fi32; /* Integer(799, Yes, Unknown) */ /* winuser.h:2401:9, winuser.h:2401:9, winuser.h:2401:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const WM_DWMCOLORIZATIONCOLORCHANGED: i32 = 0x320i32; /* Integer(800, Yes, Unknown) */ /* winuser.h:2402:9, winuser.h:2402:9, winuser.h:2402:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const WM_DWMWINDOWMAXIMIZEDCHANGE: i32 = 0x321i32; /* Integer(801, Yes, Unknown) */ /* winuser.h:2403:9, winuser.h:2403:9, winuser.h:2403:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const WM_DWMSENDICONICTHUMBNAIL: i32 = 0x323i32; /* Integer(803, Yes, Unknown) */ /* winuser.h:2407:9, winuser.h:2407:9, winuser.h:2407:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const WM_DWMSENDICONICLIVEPREVIEWBITMAP: i32 = 0x326i32; /* Integer(806, Yes, Unknown) */ /* winuser.h:2408:9, winuser.h:2408:9, winuser.h:2408:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const WM_GETTITLEBARINFOEX: i32 = 0x33fi32; /* Integer(831, Yes, Unknown) */ /* winuser.h:2413:9, winuser.h:2413:9, winuser.h:2413:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_HANDHELDFIRST: i32 = 0x358i32; /* Integer(856, Yes, Unknown) */ /* winuser.h:2418:9, winuser.h:2418:9, winuser.h:2418:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_HANDHELDLAST: i32 = 0x35fi32; /* Integer(863, Yes, Unknown) */ /* winuser.h:2419:9, winuser.h:2419:9, winuser.h:2419:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_AFXFIRST: i32 = 0x360i32; /* Integer(864, Yes, Unknown) */ /* winuser.h:2421:9, winuser.h:2421:9, winuser.h:2421:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_AFXLAST: i32 = 0x37fi32; /* Integer(895, Yes, Unknown) */ /* winuser.h:2422:9, winuser.h:2422:9, winuser.h:2422:9 */
pub const WM_PENWINFIRST: i32 = 0x380i32; /* Integer(896, Yes, Unknown) */ /* winuser.h:2425:9, winuser.h:2425:9, winuser.h:2425:9 */
pub const WM_PENWINLAST: i32 = 0x38fi32; /* Integer(911, Yes, Unknown) */ /* winuser.h:2426:9, winuser.h:2426:9, winuser.h:2426:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WM_APP: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:2430:9, winuser.h:2430:9, winuser.h:2430:9 */
pub const WM_USER: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winuser.h:2439:9, winuser.h:2439:9, winuser.h:2439:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WMSZ_LEFT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2444:9, winuser.h:2444:9, winuser.h:2444:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WMSZ_RIGHT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2445:9, winuser.h:2445:9, winuser.h:2445:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WMSZ_TOP: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:2446:9, winuser.h:2446:9, winuser.h:2446:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WMSZ_TOPLEFT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:2447:9, winuser.h:2447:9, winuser.h:2447:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WMSZ_TOPRIGHT: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:2448:9, winuser.h:2448:9, winuser.h:2448:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WMSZ_BOTTOM: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:2449:9, winuser.h:2449:9, winuser.h:2449:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WMSZ_BOTTOMLEFT: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:2450:9, winuser.h:2450:9, winuser.h:2450:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WMSZ_BOTTOMRIGHT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:2451:9, winuser.h:2451:9, winuser.h:2451:9 */
pub const HTNOWHERE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:2461:9, winuser.h:2461:9, winuser.h:2461:9 */
pub const HTCLIENT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2462:9, winuser.h:2462:9, winuser.h:2462:9 */
pub const HTCAPTION: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2463:9, winuser.h:2463:9, winuser.h:2463:9 */
pub const HTSYSMENU: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:2464:9, winuser.h:2464:9, winuser.h:2464:9 */
pub const HTGROWBOX: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:2465:9, winuser.h:2465:9, winuser.h:2465:9 */
#[doc(inline)] pub use ::winuser::HTGROWBOX as HTSIZE; /* winuser.h:2466:9, winuser.h:2466:9, winuser.h:2466:9 */
pub const HTMENU: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:2467:9, winuser.h:2467:9, winuser.h:2467:9 */
pub const HTHSCROLL: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:2468:9, winuser.h:2468:9, winuser.h:2468:9 */
pub const HTVSCROLL: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:2469:9, winuser.h:2469:9, winuser.h:2469:9 */
pub const HTMINBUTTON: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:2470:9, winuser.h:2470:9, winuser.h:2470:9 */
pub const HTMAXBUTTON: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:2471:9, winuser.h:2471:9, winuser.h:2471:9 */
pub const HTLEFT: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:2472:9, winuser.h:2472:9, winuser.h:2472:9 */
pub const HTRIGHT: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:2473:9, winuser.h:2473:9, winuser.h:2473:9 */
pub const HTTOP: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winuser.h:2474:9, winuser.h:2474:9, winuser.h:2474:9 */
pub const HTTOPLEFT: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winuser.h:2475:9, winuser.h:2475:9, winuser.h:2475:9 */
pub const HTTOPRIGHT: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winuser.h:2476:9, winuser.h:2476:9, winuser.h:2476:9 */
pub const HTBOTTOM: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winuser.h:2477:9, winuser.h:2477:9, winuser.h:2477:9 */
pub const HTBOTTOMLEFT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:2478:9, winuser.h:2478:9, winuser.h:2478:9 */
pub const HTBOTTOMRIGHT: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winuser.h:2479:9, winuser.h:2479:9, winuser.h:2479:9 */
pub const HTBORDER: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winuser.h:2480:9, winuser.h:2480:9, winuser.h:2480:9 */
#[doc(inline)] pub use ::winuser::HTMINBUTTON as HTREDUCE; /* winuser.h:2481:9, winuser.h:2481:9, winuser.h:2481:9 */
#[doc(inline)] pub use ::winuser::HTMAXBUTTON as HTZOOM; /* winuser.h:2482:9, winuser.h:2482:9, winuser.h:2482:9 */
#[doc(inline)] pub use ::winuser::HTLEFT as HTSIZEFIRST; /* winuser.h:2483:9, winuser.h:2483:9, winuser.h:2483:9 */
#[doc(inline)] pub use ::winuser::HTBOTTOMRIGHT as HTSIZELAST; /* winuser.h:2484:9, winuser.h:2484:9, winuser.h:2484:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const HTOBJECT: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winuser.h:2486:9, winuser.h:2486:9, winuser.h:2486:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const HTCLOSE: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winuser.h:2487:9, winuser.h:2487:9, winuser.h:2487:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const HTHELP: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winuser.h:2488:9, winuser.h:2488:9, winuser.h:2488:9 */
pub const SMTO_NORMAL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:2495:9, winuser.h:2495:9, winuser.h:2495:9 */
pub const SMTO_BLOCK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2496:9, winuser.h:2496:9, winuser.h:2496:9 */
pub const SMTO_ABORTIFHUNG: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2497:9, winuser.h:2497:9, winuser.h:2497:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SMTO_NOTIMEOUTIFNOTHUNG: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:2499:9, winuser.h:2499:9, winuser.h:2499:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SMTO_ERRORONEXIT: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:2502:9, winuser.h:2502:9, winuser.h:2502:9 */
pub const MA_ACTIVATE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2512:9, winuser.h:2512:9, winuser.h:2512:9 */
pub const MA_ACTIVATEANDEAT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2513:9, winuser.h:2513:9, winuser.h:2513:9 */
pub const MA_NOACTIVATE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:2514:9, winuser.h:2514:9, winuser.h:2514:9 */
pub const MA_NOACTIVATEANDEAT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:2515:9, winuser.h:2515:9, winuser.h:2515:9 */
pub const ICON_SMALL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:2520:9, winuser.h:2520:9, winuser.h:2520:9 */
pub const ICON_BIG: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2521:9, winuser.h:2521:9, winuser.h:2521:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const ICON_SMALL2: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2523:9, winuser.h:2523:9, winuser.h:2523:9 */
pub const SIZE_RESTORED: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:2553:9, winuser.h:2553:9, winuser.h:2553:9 */
pub const SIZE_MINIMIZED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2554:9, winuser.h:2554:9, winuser.h:2554:9 */
pub const SIZE_MAXIMIZED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2555:9, winuser.h:2555:9, winuser.h:2555:9 */
pub const SIZE_MAXSHOW: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:2556:9, winuser.h:2556:9, winuser.h:2556:9 */
pub const SIZE_MAXHIDE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:2557:9, winuser.h:2557:9, winuser.h:2557:9 */
#[doc(inline)] pub use ::winuser::SIZE_RESTORED as SIZENORMAL; /* winuser.h:2562:9, winuser.h:2562:9, winuser.h:2562:9 */
#[doc(inline)] pub use ::winuser::SIZE_MINIMIZED as SIZEICONIC; /* winuser.h:2563:9, winuser.h:2563:9, winuser.h:2563:9 */
#[doc(inline)] pub use ::winuser::SIZE_MAXIMIZED as SIZEFULLSCREEN; /* winuser.h:2564:9, winuser.h:2564:9, winuser.h:2564:9 */
#[doc(inline)] pub use ::winuser::SIZE_MAXSHOW as SIZEZOOMSHOW; /* winuser.h:2565:9, winuser.h:2565:9, winuser.h:2565:9 */
#[doc(inline)] pub use ::winuser::SIZE_MAXHIDE as SIZEZOOMHIDE; /* winuser.h:2566:9, winuser.h:2566:9, winuser.h:2566:9 */
pub const WVR_ALIGNTOP: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:2599:9, winuser.h:2599:9, winuser.h:2599:9 */
pub const WVR_ALIGNLEFT: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:2600:9, winuser.h:2600:9, winuser.h:2600:9 */
pub const WVR_ALIGNBOTTOM: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:2601:9, winuser.h:2601:9, winuser.h:2601:9 */
pub const WVR_ALIGNRIGHT: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:2602:9, winuser.h:2602:9, winuser.h:2602:9 */
pub const WVR_HREDRAW: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:2603:9, winuser.h:2603:9, winuser.h:2603:9 */
pub const WVR_VREDRAW: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:2604:9, winuser.h:2604:9, winuser.h:2604:9 */
pub const WVR_VALIDRECTS: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winuser.h:2607:9, winuser.h:2607:9, winuser.h:2607:9 */
pub const MK_LBUTTON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2615:9, winuser.h:2615:9, winuser.h:2615:9 */
pub const MK_RBUTTON: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2616:9, winuser.h:2616:9, winuser.h:2616:9 */
pub const MK_SHIFT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:2617:9, winuser.h:2617:9, winuser.h:2617:9 */
pub const MK_CONTROL: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:2618:9, winuser.h:2618:9, winuser.h:2618:9 */
pub const MK_MBUTTON: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:2619:9, winuser.h:2619:9, winuser.h:2619:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const MK_XBUTTON1: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:2621:9, winuser.h:2621:9, winuser.h:2621:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const MK_XBUTTON2: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:2622:9, winuser.h:2622:9, winuser.h:2622:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const TME_HOVER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2631:9, winuser.h:2631:9, winuser.h:2631:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const TME_LEAVE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2632:9, winuser.h:2632:9, winuser.h:2632:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const TME_NONCLIENT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:2634:9, winuser.h:2634:9, winuser.h:2634:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const TME_QUERY: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* winuser.h:2636:9, winuser.h:2636:9, winuser.h:2636:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const TME_CANCEL: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winuser.h:2637:9, winuser.h:2637:9, winuser.h:2637:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const HOVER_DEFAULT: i32 = 0xffffffffi32; /* Integer(4294967295, Yes, Unknown) */ /* winuser.h:2640:9, winuser.h:2640:9, winuser.h:2640:9 */
pub const WS_OVERLAPPED: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:2680:9, winuser.h:2680:9, winuser.h:2680:9 */
pub const WS_POPUP: i64 = 0x80000000i64; /* Integer(2147483648, Yes, Long) */ /* winuser.h:2681:9, winuser.h:2681:9, winuser.h:2681:9 */
pub const WS_CHILD: i64 = 0x40000000i64; /* Integer(1073741824, Yes, Long) */ /* winuser.h:2682:9, winuser.h:2682:9, winuser.h:2682:9 */
pub const WS_MINIMIZE: i64 = 0x20000000i64; /* Integer(536870912, Yes, Long) */ /* winuser.h:2683:9, winuser.h:2683:9, winuser.h:2683:9 */
pub const WS_VISIBLE: i64 = 0x10000000i64; /* Integer(268435456, Yes, Long) */ /* winuser.h:2684:9, winuser.h:2684:9, winuser.h:2684:9 */
pub const WS_DISABLED: i64 = 0x8000000i64; /* Integer(134217728, Yes, Long) */ /* winuser.h:2685:9, winuser.h:2685:9, winuser.h:2685:9 */
pub const WS_CLIPSIBLINGS: i64 = 0x4000000i64; /* Integer(67108864, Yes, Long) */ /* winuser.h:2686:9, winuser.h:2686:9, winuser.h:2686:9 */
pub const WS_CLIPCHILDREN: i64 = 0x2000000i64; /* Integer(33554432, Yes, Long) */ /* winuser.h:2687:9, winuser.h:2687:9, winuser.h:2687:9 */
pub const WS_MAXIMIZE: i64 = 0x1000000i64; /* Integer(16777216, Yes, Long) */ /* winuser.h:2688:9, winuser.h:2688:9, winuser.h:2688:9 */
pub const WS_CAPTION: i64 = 0xc00000i64; /* Integer(12582912, Yes, Long) */ /* winuser.h:2689:9, winuser.h:2689:9, winuser.h:2689:9 */
pub const WS_BORDER: i64 = 0x800000i64; /* Integer(8388608, Yes, Long) */ /* winuser.h:2690:9, winuser.h:2690:9, winuser.h:2690:9 */
pub const WS_DLGFRAME: i64 = 0x400000i64; /* Integer(4194304, Yes, Long) */ /* winuser.h:2691:9, winuser.h:2691:9, winuser.h:2691:9 */
pub const WS_VSCROLL: i64 = 0x200000i64; /* Integer(2097152, Yes, Long) */ /* winuser.h:2692:9, winuser.h:2692:9, winuser.h:2692:9 */
pub const WS_HSCROLL: i64 = 0x100000i64; /* Integer(1048576, Yes, Long) */ /* winuser.h:2693:9, winuser.h:2693:9, winuser.h:2693:9 */
pub const WS_SYSMENU: i64 = 0x80000i64; /* Integer(524288, Yes, Long) */ /* winuser.h:2694:9, winuser.h:2694:9, winuser.h:2694:9 */
pub const WS_THICKFRAME: i64 = 0x40000i64; /* Integer(262144, Yes, Long) */ /* winuser.h:2695:9, winuser.h:2695:9, winuser.h:2695:9 */
pub const WS_GROUP: i64 = 0x20000i64; /* Integer(131072, Yes, Long) */ /* winuser.h:2696:9, winuser.h:2696:9, winuser.h:2696:9 */
pub const WS_TABSTOP: i64 = 0x10000i64; /* Integer(65536, Yes, Long) */ /* winuser.h:2697:9, winuser.h:2697:9, winuser.h:2697:9 */
pub const WS_MINIMIZEBOX: i64 = 0x20000i64; /* Integer(131072, Yes, Long) */ /* winuser.h:2699:9, winuser.h:2699:9, winuser.h:2699:9 */
pub const WS_MAXIMIZEBOX: i64 = 0x10000i64; /* Integer(65536, Yes, Long) */ /* winuser.h:2700:9, winuser.h:2700:9, winuser.h:2700:9 */
#[doc(inline)] pub use ::winuser::WS_OVERLAPPED as WS_TILED; /* winuser.h:2703:9, winuser.h:2703:9, winuser.h:2703:9 */
#[doc(inline)] pub use ::winuser::WS_MINIMIZE as WS_ICONIC; /* winuser.h:2704:9, winuser.h:2704:9, winuser.h:2704:9 */
#[doc(inline)] pub use ::winuser::WS_THICKFRAME as WS_SIZEBOX; /* winuser.h:2705:9, winuser.h:2705:9, winuser.h:2705:9 */
pub const WS_EX_DLGMODALFRAME: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:2727:9, winuser.h:2727:9, winuser.h:2727:9 */
pub const WS_EX_NOPARENTNOTIFY: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:2728:9, winuser.h:2728:9, winuser.h:2728:9 */
pub const WS_EX_TOPMOST: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winuser.h:2729:9, winuser.h:2729:9, winuser.h:2729:9 */
pub const WS_EX_ACCEPTFILES: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* winuser.h:2730:9, winuser.h:2730:9, winuser.h:2730:9 */
pub const WS_EX_TRANSPARENT: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* winuser.h:2731:9, winuser.h:2731:9, winuser.h:2731:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WS_EX_MDICHILD: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* winuser.h:2733:9, winuser.h:2733:9, winuser.h:2733:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WS_EX_TOOLWINDOW: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* winuser.h:2734:9, winuser.h:2734:9, winuser.h:2734:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WS_EX_WINDOWEDGE: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* winuser.h:2735:9, winuser.h:2735:9, winuser.h:2735:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WS_EX_CLIENTEDGE: i64 = 0x200i64; /* Integer(512, Yes, Long) */ /* winuser.h:2736:9, winuser.h:2736:9, winuser.h:2736:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WS_EX_CONTEXTHELP: i64 = 0x400i64; /* Integer(1024, Yes, Long) */ /* winuser.h:2737:9, winuser.h:2737:9, winuser.h:2737:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WS_EX_RIGHT: i64 = 0x1000i64; /* Integer(4096, Yes, Long) */ /* winuser.h:2742:9, winuser.h:2742:9, winuser.h:2742:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WS_EX_LEFT: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:2743:9, winuser.h:2743:9, winuser.h:2743:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WS_EX_RTLREADING: i64 = 0x2000i64; /* Integer(8192, Yes, Long) */ /* winuser.h:2744:9, winuser.h:2744:9, winuser.h:2744:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WS_EX_LTRREADING: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:2745:9, winuser.h:2745:9, winuser.h:2745:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WS_EX_LEFTSCROLLBAR: i64 = 0x4000i64; /* Integer(16384, Yes, Long) */ /* winuser.h:2746:9, winuser.h:2746:9, winuser.h:2746:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WS_EX_RIGHTSCROLLBAR: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:2747:9, winuser.h:2747:9, winuser.h:2747:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WS_EX_CONTROLPARENT: i64 = 0x10000i64; /* Integer(65536, Yes, Long) */ /* winuser.h:2749:9, winuser.h:2749:9, winuser.h:2749:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WS_EX_STATICEDGE: i64 = 0x20000i64; /* Integer(131072, Yes, Long) */ /* winuser.h:2750:9, winuser.h:2750:9, winuser.h:2750:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const WS_EX_APPWINDOW: i64 = 0x40000i64; /* Integer(262144, Yes, Long) */ /* winuser.h:2751:9, winuser.h:2751:9, winuser.h:2751:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WS_EX_LAYERED: i32 = 0x80000i32; /* Integer(524288, Yes, Unknown) */ /* winuser.h:2760:9, winuser.h:2760:9, winuser.h:2760:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WS_EX_NOINHERITLAYOUT: i64 = 0x100000i64; /* Integer(1048576, Yes, Long) */ /* winuser.h:2766:9, winuser.h:2766:9, winuser.h:2766:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const WS_EX_NOREDIRECTIONBITMAP: i64 = 0x200000i64; /* Integer(2097152, Yes, Long) */ /* winuser.h:2770:9, winuser.h:2770:9, winuser.h:2770:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WS_EX_LAYOUTRTL: i64 = 0x400000i64; /* Integer(4194304, Yes, Long) */ /* winuser.h:2774:9, winuser.h:2774:9, winuser.h:2774:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const WS_EX_COMPOSITED: i64 = 0x2000000i64; /* Integer(33554432, Yes, Long) */ /* winuser.h:2778:9, winuser.h:2778:9, winuser.h:2778:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WS_EX_NOACTIVATE: i64 = 0x8000000i64; /* Integer(134217728, Yes, Long) */ /* winuser.h:2781:9, winuser.h:2781:9, winuser.h:2781:9 */
pub const CS_VREDRAW: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2788:9, winuser.h:2788:9, winuser.h:2788:9 */
pub const CS_HREDRAW: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2789:9, winuser.h:2789:9, winuser.h:2789:9 */
pub const CS_DBLCLKS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:2790:9, winuser.h:2790:9, winuser.h:2790:9 */
pub const CS_OWNDC: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:2791:9, winuser.h:2791:9, winuser.h:2791:9 */
pub const CS_CLASSDC: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:2792:9, winuser.h:2792:9, winuser.h:2792:9 */
pub const CS_PARENTDC: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:2793:9, winuser.h:2793:9, winuser.h:2793:9 */
pub const CS_NOCLOSE: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:2794:9, winuser.h:2794:9, winuser.h:2794:9 */
pub const CS_SAVEBITS: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winuser.h:2795:9, winuser.h:2795:9, winuser.h:2795:9 */
pub const CS_BYTEALIGNCLIENT: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:2796:9, winuser.h:2796:9, winuser.h:2796:9 */
pub const CS_BYTEALIGNWINDOW: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:2797:9, winuser.h:2797:9, winuser.h:2797:9 */
pub const CS_GLOBALCLASS: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winuser.h:2798:9, winuser.h:2798:9, winuser.h:2798:9 */
pub const CS_IME: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winuser.h:2800:9, winuser.h:2800:9, winuser.h:2800:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const CS_DROPSHADOW: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winuser.h:2802:9, winuser.h:2802:9, winuser.h:2802:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PRF_CHECKVISIBLE: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:2810:9, winuser.h:2810:9, winuser.h:2810:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PRF_NONCLIENT: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:2811:9, winuser.h:2811:9, winuser.h:2811:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PRF_CLIENT: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:2812:9, winuser.h:2812:9, winuser.h:2812:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PRF_ERASEBKGND: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winuser.h:2813:9, winuser.h:2813:9, winuser.h:2813:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PRF_CHILDREN: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* winuser.h:2814:9, winuser.h:2814:9, winuser.h:2814:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const PRF_OWNED: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* winuser.h:2815:9, winuser.h:2815:9, winuser.h:2815:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BDR_RAISEDOUTER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2818:9, winuser.h:2818:9, winuser.h:2818:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BDR_SUNKENOUTER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2819:9, winuser.h:2819:9, winuser.h:2819:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BDR_RAISEDINNER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:2820:9, winuser.h:2820:9, winuser.h:2820:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BDR_SUNKENINNER: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:2821:9, winuser.h:2821:9, winuser.h:2821:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BF_LEFT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2835:9, winuser.h:2835:9, winuser.h:2835:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BF_TOP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2836:9, winuser.h:2836:9, winuser.h:2836:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BF_RIGHT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:2837:9, winuser.h:2837:9, winuser.h:2837:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BF_BOTTOM: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:2838:9, winuser.h:2838:9, winuser.h:2838:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BF_DIAGONAL: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:2846:9, winuser.h:2846:9, winuser.h:2846:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BF_MIDDLE: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winuser.h:2856:9, winuser.h:2856:9, winuser.h:2856:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BF_SOFT: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:2857:9, winuser.h:2857:9, winuser.h:2857:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BF_ADJUST: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:2858:9, winuser.h:2858:9, winuser.h:2858:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BF_FLAT: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winuser.h:2859:9, winuser.h:2859:9, winuser.h:2859:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BF_MONO: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:2860:9, winuser.h:2860:9, winuser.h:2860:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFC_CAPTION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2880:9, winuser.h:2880:9, winuser.h:2880:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFC_MENU: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2881:9, winuser.h:2881:9, winuser.h:2881:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFC_SCROLL: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:2882:9, winuser.h:2882:9, winuser.h:2882:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFC_BUTTON: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:2883:9, winuser.h:2883:9, winuser.h:2883:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DFC_POPUPMENU: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:2885:9, winuser.h:2885:9, winuser.h:2885:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_CAPTIONCLOSE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:2888:9, winuser.h:2888:9, winuser.h:2888:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_CAPTIONMIN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2889:9, winuser.h:2889:9, winuser.h:2889:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_CAPTIONMAX: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2890:9, winuser.h:2890:9, winuser.h:2890:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_CAPTIONRESTORE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:2891:9, winuser.h:2891:9, winuser.h:2891:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_CAPTIONHELP: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:2892:9, winuser.h:2892:9, winuser.h:2892:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_MENUARROW: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:2894:9, winuser.h:2894:9, winuser.h:2894:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_MENUCHECK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2895:9, winuser.h:2895:9, winuser.h:2895:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_MENUBULLET: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2896:9, winuser.h:2896:9, winuser.h:2896:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_MENUARROWRIGHT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:2897:9, winuser.h:2897:9, winuser.h:2897:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_SCROLLUP: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:2898:9, winuser.h:2898:9, winuser.h:2898:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_SCROLLDOWN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2899:9, winuser.h:2899:9, winuser.h:2899:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_SCROLLLEFT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2900:9, winuser.h:2900:9, winuser.h:2900:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_SCROLLRIGHT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:2901:9, winuser.h:2901:9, winuser.h:2901:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_SCROLLCOMBOBOX: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:2902:9, winuser.h:2902:9, winuser.h:2902:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_SCROLLSIZEGRIP: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:2903:9, winuser.h:2903:9, winuser.h:2903:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_SCROLLSIZEGRIPRIGHT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:2904:9, winuser.h:2904:9, winuser.h:2904:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_BUTTONCHECK: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:2906:9, winuser.h:2906:9, winuser.h:2906:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_BUTTONRADIOIMAGE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2907:9, winuser.h:2907:9, winuser.h:2907:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_BUTTONRADIOMASK: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2908:9, winuser.h:2908:9, winuser.h:2908:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_BUTTONRADIO: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:2909:9, winuser.h:2909:9, winuser.h:2909:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_BUTTON3STATE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:2910:9, winuser.h:2910:9, winuser.h:2910:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_BUTTONPUSH: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:2911:9, winuser.h:2911:9, winuser.h:2911:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_INACTIVE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:2913:9, winuser.h:2913:9, winuser.h:2913:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_PUSHED: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:2914:9, winuser.h:2914:9, winuser.h:2914:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_CHECKED: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winuser.h:2915:9, winuser.h:2915:9, winuser.h:2915:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DFCS_TRANSPARENT: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winuser.h:2918:9, winuser.h:2918:9, winuser.h:2918:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DFCS_HOT: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:2919:9, winuser.h:2919:9, winuser.h:2919:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_ADJUSTRECT: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:2922:9, winuser.h:2922:9, winuser.h:2922:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_FLAT: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winuser.h:2923:9, winuser.h:2923:9, winuser.h:2923:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DFCS_MONO: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:2924:9, winuser.h:2924:9, winuser.h:2924:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DC_ACTIVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2944:9, winuser.h:2944:9, winuser.h:2944:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DC_SMALLCAP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2945:9, winuser.h:2945:9, winuser.h:2945:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DC_ICON: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:2946:9, winuser.h:2946:9, winuser.h:2946:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DC_TEXT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:2947:9, winuser.h:2947:9, winuser.h:2947:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DC_INBUTTON: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:2948:9, winuser.h:2948:9, winuser.h:2948:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DC_GRADIENT: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:2950:9, winuser.h:2950:9, winuser.h:2950:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const DC_BUTTONS: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:2953:9, winuser.h:2953:9, winuser.h:2953:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const IDANI_OPEN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2972:9, winuser.h:2972:9, winuser.h:2972:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const IDANI_CAPTION: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:2973:9, winuser.h:2973:9, winuser.h:2973:9 */
pub const CF_TEXT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:2998:9, winuser.h:2998:9, winuser.h:2998:9 */
pub const CF_BITMAP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:2999:9, winuser.h:2999:9, winuser.h:2999:9 */
pub const CF_METAFILEPICT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:3000:9, winuser.h:3000:9, winuser.h:3000:9 */
pub const CF_SYLK: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:3001:9, winuser.h:3001:9, winuser.h:3001:9 */
pub const CF_DIF: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:3002:9, winuser.h:3002:9, winuser.h:3002:9 */
pub const CF_TIFF: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:3003:9, winuser.h:3003:9, winuser.h:3003:9 */
pub const CF_OEMTEXT: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:3004:9, winuser.h:3004:9, winuser.h:3004:9 */
pub const CF_DIB: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:3005:9, winuser.h:3005:9, winuser.h:3005:9 */
pub const CF_PALETTE: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:3006:9, winuser.h:3006:9, winuser.h:3006:9 */
pub const CF_PENDATA: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:3007:9, winuser.h:3007:9, winuser.h:3007:9 */
pub const CF_RIFF: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:3008:9, winuser.h:3008:9, winuser.h:3008:9 */
pub const CF_WAVE: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winuser.h:3009:9, winuser.h:3009:9, winuser.h:3009:9 */
pub const CF_UNICODETEXT: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winuser.h:3010:9, winuser.h:3010:9, winuser.h:3010:9 */
pub const CF_ENHMETAFILE: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winuser.h:3011:9, winuser.h:3011:9, winuser.h:3011:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CF_HDROP: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winuser.h:3013:9, winuser.h:3013:9, winuser.h:3013:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CF_LOCALE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:3014:9, winuser.h:3014:9, winuser.h:3014:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CF_DIBV5: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winuser.h:3017:9, winuser.h:3017:9, winuser.h:3017:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CF_MAX: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winuser.h:3021:9, winuser.h:3021:9, winuser.h:3021:9 */
pub const CF_OWNERDISPLAY: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:3028:9, winuser.h:3028:9, winuser.h:3028:9 */
pub const CF_DSPTEXT: i32 = 0x81i32; /* Integer(129, Yes, Unknown) */ /* winuser.h:3029:9, winuser.h:3029:9, winuser.h:3029:9 */
pub const CF_DSPBITMAP: i32 = 0x82i32; /* Integer(130, Yes, Unknown) */ /* winuser.h:3030:9, winuser.h:3030:9, winuser.h:3030:9 */
pub const CF_DSPMETAFILEPICT: i32 = 0x83i32; /* Integer(131, Yes, Unknown) */ /* winuser.h:3031:9, winuser.h:3031:9, winuser.h:3031:9 */
pub const CF_DSPENHMETAFILE: i32 = 0x8ei32; /* Integer(142, Yes, Unknown) */ /* winuser.h:3032:9, winuser.h:3032:9, winuser.h:3032:9 */
pub const CF_PRIVATEFIRST: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:3037:9, winuser.h:3037:9, winuser.h:3037:9 */
pub const CF_PRIVATELAST: i32 = 0x2ffi32; /* Integer(767, Yes, Unknown) */ /* winuser.h:3038:9, winuser.h:3038:9, winuser.h:3038:9 */
pub const CF_GDIOBJFIRST: i32 = 0x300i32; /* Integer(768, Yes, Unknown) */ /* winuser.h:3043:9, winuser.h:3043:9, winuser.h:3043:9 */
pub const CF_GDIOBJLAST: i32 = 0x3ffi32; /* Integer(1023, Yes, Unknown) */ /* winuser.h:3044:9, winuser.h:3044:9, winuser.h:3044:9 */
#[doc(inline)] pub use ::minwindef::TRUE as FVIRTKEY; /* winuser.h:3052:9, winuser.h:3052:9, winuser.h:3052:9 */
pub const FNOINVERT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:3053:9, winuser.h:3053:9, winuser.h:3053:9 */
pub const FSHIFT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:3054:9, winuser.h:3054:9, winuser.h:3054:9 */
pub const FCONTROL: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:3055:9, winuser.h:3055:9, winuser.h:3055:9 */
pub const FALT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:3056:9, winuser.h:3056:9, winuser.h:3056:9 */
#[cfg(feature="winapi_desktop")] pub const WPF_SETMINPOSITION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:3131:9, winuser.h:3131:9, winuser.h:3131:9 */
#[cfg(feature="winapi_desktop")] pub const WPF_RESTORETOMAXIMIZED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:3132:9, winuser.h:3132:9, winuser.h:3132:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const WPF_ASYNCWINDOWPLACEMENT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:3134:9, winuser.h:3134:9, winuser.h:3134:9 */
pub const ODT_MENU: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:3174:9, winuser.h:3174:9, winuser.h:3174:9 */
pub const ODT_LISTBOX: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:3175:9, winuser.h:3175:9, winuser.h:3175:9 */
pub const ODT_COMBOBOX: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:3176:9, winuser.h:3176:9, winuser.h:3176:9 */
pub const ODT_BUTTON: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:3177:9, winuser.h:3177:9, winuser.h:3177:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const ODT_STATIC: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:3179:9, winuser.h:3179:9, winuser.h:3179:9 */
pub const ODA_DRAWENTIRE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:3185:9, winuser.h:3185:9, winuser.h:3185:9 */
pub const ODA_SELECT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:3186:9, winuser.h:3186:9, winuser.h:3186:9 */
pub const ODA_FOCUS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:3187:9, winuser.h:3187:9, winuser.h:3187:9 */
pub const ODS_SELECTED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:3192:9, winuser.h:3192:9, winuser.h:3192:9 */
pub const ODS_GRAYED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:3193:9, winuser.h:3193:9, winuser.h:3193:9 */
pub const ODS_DISABLED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:3194:9, winuser.h:3194:9, winuser.h:3194:9 */
pub const ODS_CHECKED: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:3195:9, winuser.h:3195:9, winuser.h:3195:9 */
pub const ODS_FOCUS: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:3196:9, winuser.h:3196:9, winuser.h:3196:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const ODS_DEFAULT: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:3198:9, winuser.h:3198:9, winuser.h:3198:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const ODS_COMBOBOXEDIT: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:3199:9, winuser.h:3199:9, winuser.h:3199:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ODS_HOTLIGHT: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:3202:9, winuser.h:3202:9, winuser.h:3202:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ODS_INACTIVE: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:3203:9, winuser.h:3203:9, winuser.h:3203:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ODS_NOACCEL: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:3205:9, winuser.h:3205:9, winuser.h:3205:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ODS_NOFOCUSRECT: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:3206:9, winuser.h:3206:9, winuser.h:3206:9 */
pub const PM_NOREMOVE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:3400:9, winuser.h:3400:9, winuser.h:3400:9 */
pub const PM_REMOVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:3401:9, winuser.h:3401:9, winuser.h:3401:9 */
pub const PM_NOYIELD: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:3402:9, winuser.h:3402:9, winuser.h:3402:9 */
pub const MOD_WIN: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:3438:9, winuser.h:3438:9, winuser.h:3438:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const MOD_NOREPEAT: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winuser.h:3440:9, winuser.h:3440:9, winuser.h:3440:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const ENDSESSION_CLOSEAPP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:3460:9, winuser.h:3460:9, winuser.h:3460:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const ENDSESSION_CRITICAL: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* winuser.h:3463:9, winuser.h:3463:9, winuser.h:3463:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const ENDSESSION_LOGOFF: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winuser.h:3466:9, winuser.h:3466:9, winuser.h:3466:9 */
pub const EWX_LOGOFF: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:3469:9, winuser.h:3469:9, winuser.h:3469:9 */
pub const EWX_SHUTDOWN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:3470:9, winuser.h:3470:9, winuser.h:3470:9 */
pub const EWX_REBOOT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:3471:9, winuser.h:3471:9, winuser.h:3471:9 */
pub const EWX_FORCE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:3472:9, winuser.h:3472:9, winuser.h:3472:9 */
pub const EWX_POWEROFF: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:3473:9, winuser.h:3473:9, winuser.h:3473:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EWX_FORCEIFHUNG: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:3475:9, winuser.h:3475:9, winuser.h:3475:9 */
pub const EWX_QUICKRESOLVE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:3477:9, winuser.h:3477:9, winuser.h:3477:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const EWX_RESTARTAPPS: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:3479:9, winuser.h:3479:9, winuser.h:3479:9 */
pub const EWX_HYBRID_SHUTDOWN: i32 = 0x400000i32; /* Integer(4194304, Yes, Unknown) */ /* winuser.h:3481:9, winuser.h:3481:9, winuser.h:3481:9 */
pub const EWX_BOOTOPTIONS: i32 = 0x1000000i32; /* Integer(16777216, Yes, Unknown) */ /* winuser.h:3482:9, winuser.h:3482:9, winuser.h:3482:9 */
/* #[cfg(feature="winapi_desktop")] // #define ExitWindows(dwReserved,Code) Call { subject: Ident("ExitWindowsEx"), args: [Ident("EWX_LOGOFF"), Integer(4294967295, Yes, Unknown)] } */ /* winuser.h:3488:9, winuser.h:3488:9, winuser.h:3488:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BSM_ALLCOMPONENTS: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:3757:9, winuser.h:3757:9, winuser.h:3757:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BSM_VXDS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:3758:9, winuser.h:3758:9, winuser.h:3758:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BSM_NETDRIVER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:3759:9, winuser.h:3759:9, winuser.h:3759:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BSM_INSTALLABLEDRIVERS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:3760:9, winuser.h:3760:9, winuser.h:3760:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BSM_APPLICATIONS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:3761:9, winuser.h:3761:9, winuser.h:3761:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BSM_ALLDESKTOPS: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:3762:9, winuser.h:3762:9, winuser.h:3762:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BSF_QUERY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:3765:9, winuser.h:3765:9, winuser.h:3765:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BSF_IGNORECURRENTTASK: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:3766:9, winuser.h:3766:9, winuser.h:3766:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BSF_FLUSHDISK: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:3767:9, winuser.h:3767:9, winuser.h:3767:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BSF_NOHANG: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:3768:9, winuser.h:3768:9, winuser.h:3768:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BSF_POSTMESSAGE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:3769:9, winuser.h:3769:9, winuser.h:3769:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BSF_FORCEIFHUNG: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:3770:9, winuser.h:3770:9, winuser.h:3770:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BSF_NOTIMEOUTIFNOTHUNG: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:3771:9, winuser.h:3771:9, winuser.h:3771:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const BSF_ALLOWSFW: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:3773:9, winuser.h:3773:9, winuser.h:3773:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const BSF_SENDNOTIFYMESSAGE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:3774:9, winuser.h:3774:9, winuser.h:3774:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const BSF_RETURNHDESK: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:3777:9, winuser.h:3777:9, winuser.h:3777:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const BSF_LUID: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winuser.h:3778:9, winuser.h:3778:9, winuser.h:3778:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BROADCAST_QUERY_DENY: i32 = 0x424d5144i32; /* Integer(1112363332, Yes, Unknown) */ /* winuser.h:3781:9, winuser.h:3781:9, winuser.h:3781:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const DEVICE_NOTIFY_WINDOW_HANDLE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:3793:9, winuser.h:3793:9, winuser.h:3793:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const DEVICE_NOTIFY_SERVICE_HANDLE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:3794:9, winuser.h:3794:9, winuser.h:3794:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub const DEVICE_NOTIFY_ALL_INTERFACE_CLASSES: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:3796:9, winuser.h:3796:9, winuser.h:3796:9 */
/* // #define PostAppMessageA(idThread,wMsg,wParam,lParam) Call { subject: Ident("PostThreadMessageA"), args: [Cast { value: Ident("idThread"), ty: Type("DWORD", false) }, Ident("wMsg"), Ident("wParam"), Ident("lParam")] } */ /* winuser.h:3925:9, winuser.h:3925:9, winuser.h:3925:9 */
/* // #define PostAppMessageW(idThread,wMsg,wParam,lParam) Call { subject: Ident("PostThreadMessageW"), args: [Cast { value: Ident("idThread"), ty: Type("DWORD", false) }, Ident("wMsg"), Ident("wParam"), Ident("lParam")] } */ /* winuser.h:3927:9, winuser.h:3927:9, winuser.h:3927:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ISMEX_NOSEND: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:4097:9, winuser.h:4097:9, winuser.h:4097:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ISMEX_SEND: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:4098:9, winuser.h:4098:9, winuser.h:4098:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ISMEX_NOTIFY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:4099:9, winuser.h:4099:9, winuser.h:4099:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ISMEX_CALLBACK: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:4100:9, winuser.h:4100:9, winuser.h:4100:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ISMEX_REPLIED: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:4101:9, winuser.h:4101:9, winuser.h:4101:9 */
/* // #define CreateWindowA(lpClassName,lpWindowName,dwStyle,x,y,nWidth,nHeight,hWndParent,hMenu,hInstance,lpParam) Call { subject: Ident("CreateWindowExA"), args: [Integer(0, Yes, Long), Ident("lpClassName"), Ident("lpWindowName"), Ident("dwStyle"), Ident("x"), Ident("y"), Ident("nWidth"), Ident("nHeight"), Ident("hWndParent"), Ident("hMenu"), Ident("hInstance"), Ident("lpParam")] } */ /* winuser.h:4275:9, winuser.h:4275:9, winuser.h:4275:9 */
/* // #define CreateWindowW(lpClassName,lpWindowName,dwStyle,x,y,nWidth,nHeight,hWndParent,hMenu,hInstance,lpParam) Call { subject: Ident("CreateWindowExW"), args: [Integer(0, Yes, Long), Ident("lpClassName"), Ident("lpWindowName"), Ident("dwStyle"), Ident("x"), Ident("y"), Ident("nWidth"), Ident("nHeight"), Ident("hWndParent"), Ident("hMenu"), Ident("hInstance"), Ident("lpParam")] } */ /* winuser.h:4279:9, winuser.h:4279:9, winuser.h:4279:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub const PW_CLIENTONLY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:4406:9, winuser.h:4406:9, winuser.h:4406:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const LWA_COLORKEY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:4441:9, winuser.h:4441:9, winuser.h:4441:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const LWA_ALPHA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:4442:9, winuser.h:4442:9, winuser.h:4442:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ULW_COLORKEY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:4445:9, winuser.h:4445:9, winuser.h:4445:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ULW_ALPHA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:4446:9, winuser.h:4446:9, winuser.h:4446:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ULW_OPAQUE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:4447:9, winuser.h:4447:9, winuser.h:4447:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ULW_EX_NORESIZE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:4449:9, winuser.h:4449:9, winuser.h:4449:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const FLASHW_STOP: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:4489:9, winuser.h:4489:9, winuser.h:4489:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const FLASHW_CAPTION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:4490:9, winuser.h:4490:9, winuser.h:4490:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const FLASHW_TRAY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:4491:9, winuser.h:4491:9, winuser.h:4491:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const FLASHW_TIMER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:4493:9, winuser.h:4493:9, winuser.h:4493:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const FLASHW_TIMERNOFG: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winuser.h:4494:9, winuser.h:4494:9, winuser.h:4494:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub const WDA_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:4555:9, winuser.h:4555:9, winuser.h:4555:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub const WDA_MONITOR: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:4556:9, winuser.h:4556:9, winuser.h:4556:9 */
pub const SWP_NOSIZE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:4653:9, winuser.h:4653:9, winuser.h:4653:9 */
pub const SWP_NOMOVE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:4654:9, winuser.h:4654:9, winuser.h:4654:9 */
pub const SWP_NOZORDER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:4655:9, winuser.h:4655:9, winuser.h:4655:9 */
pub const SWP_NOREDRAW: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:4656:9, winuser.h:4656:9, winuser.h:4656:9 */
pub const SWP_NOACTIVATE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:4657:9, winuser.h:4657:9, winuser.h:4657:9 */
pub const SWP_FRAMECHANGED: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:4658:9, winuser.h:4658:9, winuser.h:4658:9 */
pub const SWP_SHOWWINDOW: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:4659:9, winuser.h:4659:9, winuser.h:4659:9 */
pub const SWP_HIDEWINDOW: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:4660:9, winuser.h:4660:9, winuser.h:4660:9 */
pub const SWP_NOCOPYBITS: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:4661:9, winuser.h:4661:9, winuser.h:4661:9 */
pub const SWP_NOOWNERZORDER: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:4662:9, winuser.h:4662:9, winuser.h:4662:9 */
pub const SWP_NOSENDCHANGING: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winuser.h:4663:9, winuser.h:4663:9, winuser.h:4663:9 */
#[doc(inline)] pub use ::winuser::SWP_FRAMECHANGED as SWP_DRAWFRAME; /* winuser.h:4665:9, winuser.h:4665:9, winuser.h:4665:9 */
#[doc(inline)] pub use ::winuser::SWP_NOOWNERZORDER as SWP_NOREPOSITION; /* winuser.h:4666:9, winuser.h:4666:9, winuser.h:4666:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SWP_DEFERERASE: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:4669:9, winuser.h:4669:9, winuser.h:4669:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SWP_ASYNCWINDOWPOS: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winuser.h:4670:9, winuser.h:4670:9, winuser.h:4670:9 */
/* #[cfg(feature="winapi_desktop")] // #define CreateDialogA(hInstance,lpName,hWndParent,lpDialogFunc) Call { subject: Ident("CreateDialogParamA"), args: [Ident("hInstance"), Ident("lpName"), Ident("hWndParent"), Ident("lpDialogFunc"), Integer(0, Yes, Long)] } */ /* winuser.h:4823:9, winuser.h:4823:9, winuser.h:4823:9 */
/* #[cfg(feature="winapi_desktop")] // #define CreateDialogW(hInstance,lpName,hWndParent,lpDialogFunc) Call { subject: Ident("CreateDialogParamW"), args: [Ident("hInstance"), Ident("lpName"), Ident("hWndParent"), Ident("lpDialogFunc"), Integer(0, Yes, Long)] } */ /* winuser.h:4825:9, winuser.h:4825:9, winuser.h:4825:9 */
/* #[cfg(feature="winapi_desktop")] // #define CreateDialogIndirectA(hInstance,lpTemplate,hWndParent,lpDialogFunc) Call { subject: Ident("CreateDialogIndirectParamA"), args: [Ident("hInstance"), Ident("lpTemplate"), Ident("hWndParent"), Ident("lpDialogFunc"), Integer(0, Yes, Long)] } */ /* winuser.h:4833:9, winuser.h:4833:9, winuser.h:4833:9 */
/* #[cfg(feature="winapi_desktop")] // #define CreateDialogIndirectW(hInstance,lpTemplate,hWndParent,lpDialogFunc) Call { subject: Ident("CreateDialogIndirectParamW"), args: [Ident("hInstance"), Ident("lpTemplate"), Ident("hWndParent"), Ident("lpDialogFunc"), Integer(0, Yes, Long)] } */ /* winuser.h:4835:9, winuser.h:4835:9, winuser.h:4835:9 */
/* #[cfg(feature="winapi_desktop")] // #define DialogBoxA(hInstance,lpTemplate,hWndParent,lpDialogFunc) Call { subject: Ident("DialogBoxParamA"), args: [Ident("hInstance"), Ident("lpTemplate"), Ident("hWndParent"), Ident("lpDialogFunc"), Integer(0, Yes, Long)] } */ /* winuser.h:4891:9, winuser.h:4891:9, winuser.h:4891:9 */
/* #[cfg(feature="winapi_desktop")] // #define DialogBoxW(hInstance,lpTemplate,hWndParent,lpDialogFunc) Call { subject: Ident("DialogBoxParamW"), args: [Ident("hInstance"), Ident("lpTemplate"), Ident("hWndParent"), Ident("lpDialogFunc"), Integer(0, Yes, Long)] } */ /* winuser.h:4893:9, winuser.h:4893:9, winuser.h:4893:9 */
/* #[cfg(feature="winapi_desktop")] // #define DialogBoxIndirectA(hInstance,lpTemplate,hWndParent,lpDialogFunc) Call { subject: Ident("DialogBoxIndirectParamA"), args: [Ident("hInstance"), Ident("lpTemplate"), Ident("hWndParent"), Ident("lpDialogFunc"), Integer(0, Yes, Long)] } */ /* winuser.h:4901:9, winuser.h:4901:9, winuser.h:4901:9 */
/* #[cfg(feature="winapi_desktop")] // #define DialogBoxIndirectW(hInstance,lpTemplate,hWndParent,lpDialogFunc) Call { subject: Ident("DialogBoxIndirectParamW"), args: [Ident("hInstance"), Ident("lpTemplate"), Ident("hWndParent"), Ident("lpDialogFunc"), Integer(0, Yes, Long)] } */ /* winuser.h:4903:9, winuser.h:4903:9, winuser.h:4903:9 */
pub const DLGWINDOWEXTRA: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* winuser.h:5101:9, winuser.h:5101:9, winuser.h:5101:9 */
#[cfg(feature="winapi_desktop")] pub const KEYEVENTF_EXTENDEDKEY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:5741:9, winuser.h:5741:9, winuser.h:5741:9 */
#[cfg(feature="winapi_desktop")] pub const KEYEVENTF_KEYUP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:5742:9, winuser.h:5742:9, winuser.h:5742:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const KEYEVENTF_UNICODE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:5744:9, winuser.h:5744:9, winuser.h:5744:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const KEYEVENTF_SCANCODE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:5745:9, winuser.h:5745:9, winuser.h:5745:9 */
pub const MOUSEEVENTF_MOVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:5760:9, winuser.h:5760:9, winuser.h:5760:9 */
pub const MOUSEEVENTF_LEFTDOWN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:5761:9, winuser.h:5761:9, winuser.h:5761:9 */
pub const MOUSEEVENTF_LEFTUP: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:5762:9, winuser.h:5762:9, winuser.h:5762:9 */
pub const MOUSEEVENTF_RIGHTDOWN: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:5763:9, winuser.h:5763:9, winuser.h:5763:9 */
pub const MOUSEEVENTF_RIGHTUP: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:5764:9, winuser.h:5764:9, winuser.h:5764:9 */
pub const MOUSEEVENTF_MIDDLEDOWN: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:5765:9, winuser.h:5765:9, winuser.h:5765:9 */
pub const MOUSEEVENTF_MIDDLEUP: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:5766:9, winuser.h:5766:9, winuser.h:5766:9 */
pub const MOUSEEVENTF_XDOWN: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:5767:9, winuser.h:5767:9, winuser.h:5767:9 */
pub const MOUSEEVENTF_XUP: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:5768:9, winuser.h:5768:9, winuser.h:5768:9 */
pub const MOUSEEVENTF_WHEEL: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winuser.h:5769:9, winuser.h:5769:9, winuser.h:5769:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const MOUSEEVENTF_HWHEEL: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:5771:9, winuser.h:5771:9, winuser.h:5771:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const MOUSEEVENTF_MOVE_NOCOALESCE: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:5774:9, winuser.h:5774:9, winuser.h:5774:9 */
pub const MOUSEEVENTF_VIRTUALDESK: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winuser.h:5776:9, winuser.h:5776:9, winuser.h:5776:9 */
pub const MOUSEEVENTF_ABSOLUTE: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:5777:9, winuser.h:5777:9, winuser.h:5777:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const INPUT_MOUSE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:5823:9, winuser.h:5823:9, winuser.h:5823:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const INPUT_KEYBOARD: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:5824:9, winuser.h:5824:9, winuser.h:5824:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const INPUT_HARDWARE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:5825:9, winuser.h:5825:9, winuser.h:5825:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const TOUCHEVENTF_MOVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:5891:9, winuser.h:5891:9, winuser.h:5891:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const TOUCHEVENTF_DOWN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:5892:9, winuser.h:5892:9, winuser.h:5892:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const TOUCHEVENTF_UP: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:5893:9, winuser.h:5893:9, winuser.h:5893:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const TOUCHEVENTF_INRANGE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:5894:9, winuser.h:5894:9, winuser.h:5894:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const TOUCHEVENTF_PRIMARY: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:5895:9, winuser.h:5895:9, winuser.h:5895:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const TOUCHEVENTF_NOCOALESCE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:5896:9, winuser.h:5896:9, winuser.h:5896:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const TOUCHEVENTF_PEN: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:5897:9, winuser.h:5897:9, winuser.h:5897:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const TOUCHEVENTF_PALM: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:5898:9, winuser.h:5898:9, winuser.h:5898:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:5903:9, winuser.h:5903:9, winuser.h:5903:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const TOUCHINPUTMASKF_EXTRAINFO: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:5904:9, winuser.h:5904:9, winuser.h:5904:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const TOUCHINPUTMASKF_CONTACTAREA: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:5905:9, winuser.h:5905:9, winuser.h:5905:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:5992:9, winuser.h:5992:9, winuser.h:5992:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_NEW: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:5993:9, winuser.h:5993:9, winuser.h:5993:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_INRANGE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:5994:9, winuser.h:5994:9, winuser.h:5994:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_INCONTACT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:5995:9, winuser.h:5995:9, winuser.h:5995:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_FIRSTBUTTON: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:5996:9, winuser.h:5996:9, winuser.h:5996:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_SECONDBUTTON: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:5997:9, winuser.h:5997:9, winuser.h:5997:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_THIRDBUTTON: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:5998:9, winuser.h:5998:9, winuser.h:5998:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_FOURTHBUTTON: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:5999:9, winuser.h:5999:9, winuser.h:5999:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_FIFTHBUTTON: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:6000:9, winuser.h:6000:9, winuser.h:6000:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_PRIMARY: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:6001:9, winuser.h:6001:9, winuser.h:6001:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_CONFIDENCE: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winuser.h:6002:9, winuser.h:6002:9, winuser.h:6002:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_CANCELED: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:6003:9, winuser.h:6003:9, winuser.h:6003:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_DOWN: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winuser.h:6004:9, winuser.h:6004:9, winuser.h:6004:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_UPDATE: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winuser.h:6005:9, winuser.h:6005:9, winuser.h:6005:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_UP: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* winuser.h:6006:9, winuser.h:6006:9, winuser.h:6006:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_WHEEL: i32 = 0x80000i32; /* Integer(524288, Yes, Unknown) */ /* winuser.h:6007:9, winuser.h:6007:9, winuser.h:6007:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_HWHEEL: i32 = 0x100000i32; /* Integer(1048576, Yes, Unknown) */ /* winuser.h:6008:9, winuser.h:6008:9, winuser.h:6008:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_CAPTURECHANGED: i32 = 0x200000i32; /* Integer(2097152, Yes, Unknown) */ /* winuser.h:6009:9, winuser.h:6009:9, winuser.h:6009:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_FLAG_HASTRANSFORM: i32 = 0x400000i32; /* Integer(4194304, Yes, Unknown) */ /* winuser.h:6010:9, winuser.h:6010:9, winuser.h:6010:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCH_FLAG_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:6057:9, winuser.h:6057:9, winuser.h:6057:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCH_MASK_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:6060:9, winuser.h:6060:9, winuser.h:6060:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCH_MASK_CONTACTAREA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:6061:9, winuser.h:6061:9, winuser.h:6061:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCH_MASK_ORIENTATION: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:6062:9, winuser.h:6062:9, winuser.h:6062:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCH_MASK_PRESSURE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:6063:9, winuser.h:6063:9, winuser.h:6063:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const PEN_FLAG_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:6076:9, winuser.h:6076:9, winuser.h:6076:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const PEN_FLAG_BARREL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:6077:9, winuser.h:6077:9, winuser.h:6077:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const PEN_FLAG_INVERTED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:6078:9, winuser.h:6078:9, winuser.h:6078:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const PEN_FLAG_ERASER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:6079:9, winuser.h:6079:9, winuser.h:6079:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const PEN_MASK_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:6082:9, winuser.h:6082:9, winuser.h:6082:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const PEN_MASK_PRESSURE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:6083:9, winuser.h:6083:9, winuser.h:6083:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const PEN_MASK_ROTATION: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:6084:9, winuser.h:6084:9, winuser.h:6084:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const PEN_MASK_TILT_X: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:6085:9, winuser.h:6085:9, winuser.h:6085:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const PEN_MASK_TILT_Y: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:6086:9, winuser.h:6086:9, winuser.h:6086:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_MESSAGE_FLAG_NEW: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:6104:9, winuser.h:6104:9, winuser.h:6104:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_MESSAGE_FLAG_INRANGE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:6105:9, winuser.h:6105:9, winuser.h:6105:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_MESSAGE_FLAG_INCONTACT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:6106:9, winuser.h:6106:9, winuser.h:6106:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_MESSAGE_FLAG_FIRSTBUTTON: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:6107:9, winuser.h:6107:9, winuser.h:6107:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_MESSAGE_FLAG_SECONDBUTTON: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:6108:9, winuser.h:6108:9, winuser.h:6108:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_MESSAGE_FLAG_THIRDBUTTON: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:6109:9, winuser.h:6109:9, winuser.h:6109:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_MESSAGE_FLAG_FOURTHBUTTON: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:6110:9, winuser.h:6110:9, winuser.h:6110:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_MESSAGE_FLAG_FIFTHBUTTON: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:6111:9, winuser.h:6111:9, winuser.h:6111:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_MESSAGE_FLAG_PRIMARY: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:6112:9, winuser.h:6112:9, winuser.h:6112:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_MESSAGE_FLAG_CONFIDENCE: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winuser.h:6113:9, winuser.h:6113:9, winuser.h:6113:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_MESSAGE_FLAG_CANCELED: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:6114:9, winuser.h:6114:9, winuser.h:6114:9 */
#[cfg(any(feature="winapi_ver_06020000"))] #[doc(inline)] pub use ::winuser::MA_ACTIVATE as PA_ACTIVATE; /* winuser.h:6136:9, winuser.h:6136:9, winuser.h:6136:9 */
#[cfg(any(feature="winapi_ver_06020000"))] #[doc(inline)] pub use ::winuser::MA_NOACTIVATE as PA_NOACTIVATE; /* winuser.h:6137:9, winuser.h:6137:9, winuser.h:6137:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const MAX_TOUCH_COUNT: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:6140:9, winuser.h:6140:9, winuser.h:6140:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCH_FEEDBACK_DEFAULT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:6142:9, winuser.h:6142:9, winuser.h:6142:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCH_FEEDBACK_INDIRECT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:6143:9, winuser.h:6143:9, winuser.h:6143:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCH_FEEDBACK_NONE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:6144:9, winuser.h:6144:9, winuser.h:6144:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCH_HIT_TESTING_DEFAULT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:6307:9, winuser.h:6307:9, winuser.h:6307:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCH_HIT_TESTING_CLIENT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:6308:9, winuser.h:6308:9, winuser.h:6308:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCH_HIT_TESTING_NONE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:6309:9, winuser.h:6309:9, winuser.h:6309:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCH_HIT_TESTING_PROXIMITY_CLOSEST: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:6338:9, winuser.h:6338:9, winuser.h:6338:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCH_HIT_TESTING_PROXIMITY_FARTHEST: i32 = 0xfffi32; /* Integer(4095, Yes, Unknown) */ /* winuser.h:6339:9, winuser.h:6339:9, winuser.h:6339:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const GWFS_INCLUDE_ANCESTORS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:6381:9, winuser.h:6381:9, winuser.h:6381:9 */
pub const MWMO_WAITALL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:6566:9, winuser.h:6566:9, winuser.h:6566:9 */
pub const MWMO_ALERTABLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:6567:9, winuser.h:6567:9, winuser.h:6567:9 */
pub const MWMO_INPUTAVAILABLE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:6568:9, winuser.h:6568:9, winuser.h:6568:9 */
pub const QS_KEY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:6573:9, winuser.h:6573:9, winuser.h:6573:9 */
pub const QS_MOUSEMOVE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:6574:9, winuser.h:6574:9, winuser.h:6574:9 */
pub const QS_MOUSEBUTTON: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:6575:9, winuser.h:6575:9, winuser.h:6575:9 */
pub const QS_POSTMESSAGE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:6576:9, winuser.h:6576:9, winuser.h:6576:9 */
pub const QS_TIMER: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:6577:9, winuser.h:6577:9, winuser.h:6577:9 */
pub const QS_PAINT: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:6578:9, winuser.h:6578:9, winuser.h:6578:9 */
pub const QS_SENDMESSAGE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:6579:9, winuser.h:6579:9, winuser.h:6579:9 */
pub const QS_HOTKEY: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:6580:9, winuser.h:6580:9, winuser.h:6580:9 */
pub const QS_ALLPOSTMESSAGE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:6581:9, winuser.h:6581:9, winuser.h:6581:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const QS_RAWINPUT: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winuser.h:6584:9, winuser.h:6584:9, winuser.h:6584:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const QS_TOUCH: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winuser.h:6588:9, winuser.h:6588:9, winuser.h:6588:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const QS_POINTER: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:6589:9, winuser.h:6589:9, winuser.h:6589:9 */
pub const USER_TIMER_MAXIMUM: i32 = 0x7fffffffi32; /* Integer(2147483647, Yes, Unknown) */ /* winuser.h:6629:9, winuser.h:6629:9, winuser.h:6629:9 */
pub const USER_TIMER_MINIMUM: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:6630:9, winuser.h:6630:9, winuser.h:6630:9 */
pub const SM_CXSCREEN: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:6789:9, winuser.h:6789:9, winuser.h:6789:9 */
pub const SM_CYSCREEN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:6790:9, winuser.h:6790:9, winuser.h:6790:9 */
pub const SM_CXVSCROLL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:6791:9, winuser.h:6791:9, winuser.h:6791:9 */
pub const SM_CYHSCROLL: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:6792:9, winuser.h:6792:9, winuser.h:6792:9 */
pub const SM_CYCAPTION: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:6793:9, winuser.h:6793:9, winuser.h:6793:9 */
pub const SM_CXBORDER: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:6794:9, winuser.h:6794:9, winuser.h:6794:9 */
pub const SM_CYBORDER: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:6795:9, winuser.h:6795:9, winuser.h:6795:9 */
pub const SM_CXDLGFRAME: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:6796:9, winuser.h:6796:9, winuser.h:6796:9 */
pub const SM_CYDLGFRAME: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:6797:9, winuser.h:6797:9, winuser.h:6797:9 */
pub const SM_CYVTHUMB: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:6798:9, winuser.h:6798:9, winuser.h:6798:9 */
pub const SM_CXHTHUMB: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:6799:9, winuser.h:6799:9, winuser.h:6799:9 */
pub const SM_CXICON: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:6800:9, winuser.h:6800:9, winuser.h:6800:9 */
pub const SM_CYICON: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winuser.h:6801:9, winuser.h:6801:9, winuser.h:6801:9 */
pub const SM_CXCURSOR: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winuser.h:6802:9, winuser.h:6802:9, winuser.h:6802:9 */
pub const SM_CYCURSOR: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winuser.h:6803:9, winuser.h:6803:9, winuser.h:6803:9 */
pub const SM_CYMENU: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winuser.h:6804:9, winuser.h:6804:9, winuser.h:6804:9 */
pub const SM_CXFULLSCREEN: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:6805:9, winuser.h:6805:9, winuser.h:6805:9 */
pub const SM_CYFULLSCREEN: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winuser.h:6806:9, winuser.h:6806:9, winuser.h:6806:9 */
pub const SM_CYKANJIWINDOW: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winuser.h:6807:9, winuser.h:6807:9, winuser.h:6807:9 */
pub const SM_MOUSEPRESENT: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winuser.h:6808:9, winuser.h:6808:9, winuser.h:6808:9 */
pub const SM_CYVSCROLL: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winuser.h:6809:9, winuser.h:6809:9, winuser.h:6809:9 */
pub const SM_CXHSCROLL: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winuser.h:6810:9, winuser.h:6810:9, winuser.h:6810:9 */
pub const SM_DEBUG: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winuser.h:6811:9, winuser.h:6811:9, winuser.h:6811:9 */
pub const SM_SWAPBUTTON: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winuser.h:6812:9, winuser.h:6812:9, winuser.h:6812:9 */
pub const SM_RESERVED1: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winuser.h:6813:9, winuser.h:6813:9, winuser.h:6813:9 */
pub const SM_RESERVED2: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* winuser.h:6814:9, winuser.h:6814:9, winuser.h:6814:9 */
pub const SM_RESERVED3: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* winuser.h:6815:9, winuser.h:6815:9, winuser.h:6815:9 */
pub const SM_RESERVED4: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* winuser.h:6816:9, winuser.h:6816:9, winuser.h:6816:9 */
pub const SM_CXMIN: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* winuser.h:6817:9, winuser.h:6817:9, winuser.h:6817:9 */
pub const SM_CYMIN: i32 = 0x1di32; /* Integer(29, Yes, Unknown) */ /* winuser.h:6818:9, winuser.h:6818:9, winuser.h:6818:9 */
pub const SM_CXSIZE: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* winuser.h:6819:9, winuser.h:6819:9, winuser.h:6819:9 */
pub const SM_CYSIZE: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* winuser.h:6820:9, winuser.h:6820:9, winuser.h:6820:9 */
pub const SM_CXFRAME: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:6821:9, winuser.h:6821:9, winuser.h:6821:9 */
pub const SM_CYFRAME: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* winuser.h:6822:9, winuser.h:6822:9, winuser.h:6822:9 */
pub const SM_CXMINTRACK: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* winuser.h:6823:9, winuser.h:6823:9, winuser.h:6823:9 */
pub const SM_CYMINTRACK: i32 = 0x23i32; /* Integer(35, Yes, Unknown) */ /* winuser.h:6824:9, winuser.h:6824:9, winuser.h:6824:9 */
pub const SM_CXDOUBLECLK: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* winuser.h:6825:9, winuser.h:6825:9, winuser.h:6825:9 */
pub const SM_CYDOUBLECLK: i32 = 0x25i32; /* Integer(37, Yes, Unknown) */ /* winuser.h:6826:9, winuser.h:6826:9, winuser.h:6826:9 */
pub const SM_CXICONSPACING: i32 = 0x26i32; /* Integer(38, Yes, Unknown) */ /* winuser.h:6827:9, winuser.h:6827:9, winuser.h:6827:9 */
pub const SM_CYICONSPACING: i32 = 0x27i32; /* Integer(39, Yes, Unknown) */ /* winuser.h:6828:9, winuser.h:6828:9, winuser.h:6828:9 */
pub const SM_MENUDROPALIGNMENT: i32 = 0x28i32; /* Integer(40, Yes, Unknown) */ /* winuser.h:6829:9, winuser.h:6829:9, winuser.h:6829:9 */
pub const SM_PENWINDOWS: i32 = 0x29i32; /* Integer(41, Yes, Unknown) */ /* winuser.h:6830:9, winuser.h:6830:9, winuser.h:6830:9 */
pub const SM_DBCSENABLED: i32 = 0x2ai32; /* Integer(42, Yes, Unknown) */ /* winuser.h:6831:9, winuser.h:6831:9, winuser.h:6831:9 */
pub const SM_CMOUSEBUTTONS: i32 = 0x2bi32; /* Integer(43, Yes, Unknown) */ /* winuser.h:6832:9, winuser.h:6832:9, winuser.h:6832:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::SM_CXDLGFRAME as SM_CXFIXEDFRAME; /* winuser.h:6835:9, winuser.h:6835:9, winuser.h:6835:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::SM_CYDLGFRAME as SM_CYFIXEDFRAME; /* winuser.h:6836:9, winuser.h:6836:9, winuser.h:6836:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::SM_CXFRAME as SM_CXSIZEFRAME; /* winuser.h:6837:9, winuser.h:6837:9, winuser.h:6837:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::SM_CYFRAME as SM_CYSIZEFRAME; /* winuser.h:6838:9, winuser.h:6838:9, winuser.h:6838:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_SECURE: i32 = 0x2ci32; /* Integer(44, Yes, Unknown) */ /* winuser.h:6840:9, winuser.h:6840:9, winuser.h:6840:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CXEDGE: i32 = 0x2di32; /* Integer(45, Yes, Unknown) */ /* winuser.h:6841:9, winuser.h:6841:9, winuser.h:6841:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CYEDGE: i32 = 0x2ei32; /* Integer(46, Yes, Unknown) */ /* winuser.h:6842:9, winuser.h:6842:9, winuser.h:6842:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CXMINSPACING: i32 = 0x2fi32; /* Integer(47, Yes, Unknown) */ /* winuser.h:6843:9, winuser.h:6843:9, winuser.h:6843:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CYMINSPACING: i32 = 0x30i32; /* Integer(48, Yes, Unknown) */ /* winuser.h:6844:9, winuser.h:6844:9, winuser.h:6844:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CXSMICON: i32 = 0x31i32; /* Integer(49, Yes, Unknown) */ /* winuser.h:6845:9, winuser.h:6845:9, winuser.h:6845:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CYSMICON: i32 = 0x32i32; /* Integer(50, Yes, Unknown) */ /* winuser.h:6846:9, winuser.h:6846:9, winuser.h:6846:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CYSMCAPTION: i32 = 0x33i32; /* Integer(51, Yes, Unknown) */ /* winuser.h:6847:9, winuser.h:6847:9, winuser.h:6847:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CXSMSIZE: i32 = 0x34i32; /* Integer(52, Yes, Unknown) */ /* winuser.h:6848:9, winuser.h:6848:9, winuser.h:6848:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CYSMSIZE: i32 = 0x35i32; /* Integer(53, Yes, Unknown) */ /* winuser.h:6849:9, winuser.h:6849:9, winuser.h:6849:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CXMENUSIZE: i32 = 0x36i32; /* Integer(54, Yes, Unknown) */ /* winuser.h:6850:9, winuser.h:6850:9, winuser.h:6850:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CYMENUSIZE: i32 = 0x37i32; /* Integer(55, Yes, Unknown) */ /* winuser.h:6851:9, winuser.h:6851:9, winuser.h:6851:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_ARRANGE: i32 = 0x38i32; /* Integer(56, Yes, Unknown) */ /* winuser.h:6852:9, winuser.h:6852:9, winuser.h:6852:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CXMINIMIZED: i32 = 0x39i32; /* Integer(57, Yes, Unknown) */ /* winuser.h:6853:9, winuser.h:6853:9, winuser.h:6853:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CYMINIMIZED: i32 = 0x3ai32; /* Integer(58, Yes, Unknown) */ /* winuser.h:6854:9, winuser.h:6854:9, winuser.h:6854:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CXMAXTRACK: i32 = 0x3bi32; /* Integer(59, Yes, Unknown) */ /* winuser.h:6855:9, winuser.h:6855:9, winuser.h:6855:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CYMAXTRACK: i32 = 0x3ci32; /* Integer(60, Yes, Unknown) */ /* winuser.h:6856:9, winuser.h:6856:9, winuser.h:6856:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CXMAXIMIZED: i32 = 0x3di32; /* Integer(61, Yes, Unknown) */ /* winuser.h:6857:9, winuser.h:6857:9, winuser.h:6857:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CYMAXIMIZED: i32 = 0x3ei32; /* Integer(62, Yes, Unknown) */ /* winuser.h:6858:9, winuser.h:6858:9, winuser.h:6858:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_NETWORK: i32 = 0x3fi32; /* Integer(63, Yes, Unknown) */ /* winuser.h:6859:9, winuser.h:6859:9, winuser.h:6859:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CLEANBOOT: i32 = 0x43i32; /* Integer(67, Yes, Unknown) */ /* winuser.h:6860:9, winuser.h:6860:9, winuser.h:6860:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CXDRAG: i32 = 0x44i32; /* Integer(68, Yes, Unknown) */ /* winuser.h:6861:9, winuser.h:6861:9, winuser.h:6861:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CYDRAG: i32 = 0x45i32; /* Integer(69, Yes, Unknown) */ /* winuser.h:6862:9, winuser.h:6862:9, winuser.h:6862:9 */
pub const SM_SHOWSOUNDS: i32 = 0x46i32; /* Integer(70, Yes, Unknown) */ /* winuser.h:6864:9, winuser.h:6864:9, winuser.h:6864:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CXMENUCHECK: i32 = 0x47i32; /* Integer(71, Yes, Unknown) */ /* winuser.h:6866:9, winuser.h:6866:9, winuser.h:6866:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_CYMENUCHECK: i32 = 0x48i32; /* Integer(72, Yes, Unknown) */ /* winuser.h:6867:9, winuser.h:6867:9, winuser.h:6867:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_SLOWMACHINE: i32 = 0x49i32; /* Integer(73, Yes, Unknown) */ /* winuser.h:6868:9, winuser.h:6868:9, winuser.h:6868:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_MIDEASTENABLED: i32 = 0x4ai32; /* Integer(74, Yes, Unknown) */ /* winuser.h:6869:9, winuser.h:6869:9, winuser.h:6869:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SM_MOUSEWHEELPRESENT: i32 = 0x4bi32; /* Integer(75, Yes, Unknown) */ /* winuser.h:6873:9, winuser.h:6873:9, winuser.h:6873:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SM_XVIRTUALSCREEN: i32 = 0x4ci32; /* Integer(76, Yes, Unknown) */ /* winuser.h:6876:9, winuser.h:6876:9, winuser.h:6876:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SM_YVIRTUALSCREEN: i32 = 0x4di32; /* Integer(77, Yes, Unknown) */ /* winuser.h:6877:9, winuser.h:6877:9, winuser.h:6877:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SM_CXVIRTUALSCREEN: i32 = 0x4ei32; /* Integer(78, Yes, Unknown) */ /* winuser.h:6878:9, winuser.h:6878:9, winuser.h:6878:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SM_CYVIRTUALSCREEN: i32 = 0x4fi32; /* Integer(79, Yes, Unknown) */ /* winuser.h:6879:9, winuser.h:6879:9, winuser.h:6879:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SM_CMONITORS: i32 = 0x50i32; /* Integer(80, Yes, Unknown) */ /* winuser.h:6880:9, winuser.h:6880:9, winuser.h:6880:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SM_SAMEDISPLAYFORMAT: i32 = 0x51i32; /* Integer(81, Yes, Unknown) */ /* winuser.h:6881:9, winuser.h:6881:9, winuser.h:6881:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SM_IMMENABLED: i32 = 0x52i32; /* Integer(82, Yes, Unknown) */ /* winuser.h:6884:9, winuser.h:6884:9, winuser.h:6884:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SM_CXFOCUSBORDER: i32 = 0x53i32; /* Integer(83, Yes, Unknown) */ /* winuser.h:6887:9, winuser.h:6887:9, winuser.h:6887:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SM_CYFOCUSBORDER: i32 = 0x54i32; /* Integer(84, Yes, Unknown) */ /* winuser.h:6888:9, winuser.h:6888:9, winuser.h:6888:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SM_TABLETPC: i32 = 0x56i32; /* Integer(86, Yes, Unknown) */ /* winuser.h:6892:9, winuser.h:6892:9, winuser.h:6892:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SM_MEDIACENTER: i32 = 0x57i32; /* Integer(87, Yes, Unknown) */ /* winuser.h:6893:9, winuser.h:6893:9, winuser.h:6893:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SM_STARTER: i32 = 0x58i32; /* Integer(88, Yes, Unknown) */ /* winuser.h:6894:9, winuser.h:6894:9, winuser.h:6894:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SM_SERVERR2: i32 = 0x59i32; /* Integer(89, Yes, Unknown) */ /* winuser.h:6895:9, winuser.h:6895:9, winuser.h:6895:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SM_MOUSEHORIZONTALWHEELPRESENT: i32 = 0x5bi32; /* Integer(91, Yes, Unknown) */ /* winuser.h:6899:9, winuser.h:6899:9, winuser.h:6899:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SM_CXPADDEDBORDER: i32 = 0x5ci32; /* Integer(92, Yes, Unknown) */ /* winuser.h:6900:9, winuser.h:6900:9, winuser.h:6900:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SM_DIGITIZER: i32 = 0x5ei32; /* Integer(94, Yes, Unknown) */ /* winuser.h:6905:9, winuser.h:6905:9, winuser.h:6905:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SM_MAXIMUMTOUCHES: i32 = 0x5fi32; /* Integer(95, Yes, Unknown) */ /* winuser.h:6906:9, winuser.h:6906:9, winuser.h:6906:9 */
pub const SM_CMETRICS: i32 = 0x61i32; /* Integer(97, Yes, Unknown) */ /* winuser.h:6918:9, winuser.h:6918:9, winuser.h:6918:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SM_REMOTESESSION: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:6922:9, winuser.h:6922:9, winuser.h:6922:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SM_SHUTTINGDOWN: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:6926:9, winuser.h:6926:9, winuser.h:6926:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SM_REMOTECONTROL: i32 = 0x2001i32; /* Integer(8193, Yes, Unknown) */ /* winuser.h:6930:9, winuser.h:6930:9, winuser.h:6930:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SM_CARETBLINKINGENABLED: i32 = 0x2002i32; /* Integer(8194, Yes, Unknown) */ /* winuser.h:6934:9, winuser.h:6934:9, winuser.h:6934:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SM_CONVERTIBLESLATEMODE: i32 = 0x2003i32; /* Integer(8195, Yes, Unknown) */ /* winuser.h:6938:9, winuser.h:6938:9, winuser.h:6938:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SM_SYSTEMDOCKED: i32 = 0x2004i32; /* Integer(8196, Yes, Unknown) */ /* winuser.h:6939:9, winuser.h:6939:9, winuser.h:6939:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub const PMB_ACTIVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:7084:9, winuser.h:7084:9, winuser.h:7084:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const MNC_IGNORE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:7266:9, winuser.h:7266:9, winuser.h:7266:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const MNC_CLOSE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:7267:9, winuser.h:7267:9, winuser.h:7267:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const MNC_EXECUTE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:7268:9, winuser.h:7268:9, winuser.h:7268:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const MNC_SELECT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:7269:9, winuser.h:7269:9, winuser.h:7269:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MNS_NOCHECK: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winuser.h:7305:9, winuser.h:7305:9, winuser.h:7305:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MNS_MODELESS: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* winuser.h:7306:9, winuser.h:7306:9, winuser.h:7306:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MNS_DRAGDROP: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* winuser.h:7307:9, winuser.h:7307:9, winuser.h:7307:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MNS_AUTODISMISS: i32 = 0x10000000i32; /* Integer(268435456, Yes, Unknown) */ /* winuser.h:7308:9, winuser.h:7308:9, winuser.h:7308:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MNS_NOTIFYBYPOS: i32 = 0x8000000i32; /* Integer(134217728, Yes, Unknown) */ /* winuser.h:7309:9, winuser.h:7309:9, winuser.h:7309:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MNS_CHECKORBMP: i32 = 0x4000000i32; /* Integer(67108864, Yes, Unknown) */ /* winuser.h:7310:9, winuser.h:7310:9, winuser.h:7310:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MIM_MAXHEIGHT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:7312:9, winuser.h:7312:9, winuser.h:7312:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MIM_BACKGROUND: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:7313:9, winuser.h:7313:9, winuser.h:7313:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MIM_HELPID: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:7314:9, winuser.h:7314:9, winuser.h:7314:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MIM_MENUDATA: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:7315:9, winuser.h:7315:9, winuser.h:7315:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MIM_STYLE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:7316:9, winuser.h:7316:9, winuser.h:7316:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MIM_APPLYTOSUBMENUS: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winuser.h:7317:9, winuser.h:7317:9, winuser.h:7317:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MND_CONTINUE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:7354:9, winuser.h:7354:9, winuser.h:7354:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MND_ENDMENU: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:7355:9, winuser.h:7355:9, winuser.h:7355:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MNGOF_TOPGAP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:7369:9, winuser.h:7369:9, winuser.h:7369:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MNGOF_BOTTOMGAP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:7370:9, winuser.h:7370:9, winuser.h:7370:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MNGO_NOINTERFACE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:7375:9, winuser.h:7375:9, winuser.h:7375:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MNGO_NOERROR: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:7376:9, winuser.h:7376:9, winuser.h:7376:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const MIIM_STATE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:7380:9, winuser.h:7380:9, winuser.h:7380:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const MIIM_ID: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:7381:9, winuser.h:7381:9, winuser.h:7381:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const MIIM_SUBMENU: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:7382:9, winuser.h:7382:9, winuser.h:7382:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const MIIM_CHECKMARKS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:7383:9, winuser.h:7383:9, winuser.h:7383:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const MIIM_TYPE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:7384:9, winuser.h:7384:9, winuser.h:7384:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const MIIM_DATA: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:7385:9, winuser.h:7385:9, winuser.h:7385:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MIIM_STRING: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:7389:9, winuser.h:7389:9, winuser.h:7389:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MIIM_BITMAP: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:7390:9, winuser.h:7390:9, winuser.h:7390:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MIIM_FTYPE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:7391:9, winuser.h:7391:9, winuser.h:7391:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const GMDI_USEDISABLED: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:7524:9, winuser.h:7524:9, winuser.h:7524:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const GMDI_GOINTOPOPUPS: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:7525:9, winuser.h:7525:9, winuser.h:7525:9 */
#[cfg(feature="winapi_desktop")] pub const TPM_LEFTBUTTON: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:7564:9, winuser.h:7564:9, winuser.h:7564:9 */
#[cfg(feature="winapi_desktop")] pub const TPM_RIGHTBUTTON: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:7565:9, winuser.h:7565:9, winuser.h:7565:9 */
#[cfg(feature="winapi_desktop")] pub const TPM_LEFTALIGN: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:7566:9, winuser.h:7566:9, winuser.h:7566:9 */
#[cfg(feature="winapi_desktop")] pub const TPM_CENTERALIGN: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:7567:9, winuser.h:7567:9, winuser.h:7567:9 */
#[cfg(feature="winapi_desktop")] pub const TPM_RIGHTALIGN: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winuser.h:7568:9, winuser.h:7568:9, winuser.h:7568:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const TPM_TOPALIGN: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:7570:9, winuser.h:7570:9, winuser.h:7570:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const TPM_VCENTERALIGN: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* winuser.h:7571:9, winuser.h:7571:9, winuser.h:7571:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const TPM_BOTTOMALIGN: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* winuser.h:7572:9, winuser.h:7572:9, winuser.h:7572:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const TPM_HORIZONTAL: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:7574:9, winuser.h:7574:9, winuser.h:7574:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const TPM_VERTICAL: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* winuser.h:7575:9, winuser.h:7575:9, winuser.h:7575:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const TPM_NONOTIFY: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* winuser.h:7576:9, winuser.h:7576:9, winuser.h:7576:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const TPM_RETURNCMD: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* winuser.h:7577:9, winuser.h:7577:9, winuser.h:7577:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const TPM_RECURSE: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:7580:9, winuser.h:7580:9, winuser.h:7580:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const TPM_HORPOSANIMATION: i64 = 0x400i64; /* Integer(1024, Yes, Long) */ /* winuser.h:7581:9, winuser.h:7581:9, winuser.h:7581:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const TPM_HORNEGANIMATION: i64 = 0x800i64; /* Integer(2048, Yes, Long) */ /* winuser.h:7582:9, winuser.h:7582:9, winuser.h:7582:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const TPM_VERPOSANIMATION: i64 = 0x1000i64; /* Integer(4096, Yes, Long) */ /* winuser.h:7583:9, winuser.h:7583:9, winuser.h:7583:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const TPM_VERNEGANIMATION: i64 = 0x2000i64; /* Integer(8192, Yes, Long) */ /* winuser.h:7584:9, winuser.h:7584:9, winuser.h:7584:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const TPM_NOANIMATION: i64 = 0x4000i64; /* Integer(16384, Yes, Long) */ /* winuser.h:7586:9, winuser.h:7586:9, winuser.h:7586:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub const TPM_LAYOUTRTL: i64 = 0x8000i64; /* Integer(32768, Yes, Long) */ /* winuser.h:7589:9, winuser.h:7589:9, winuser.h:7589:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub const TPM_WORKAREA: i64 = 0x10000i64; /* Integer(65536, Yes, Long) */ /* winuser.h:7593:9, winuser.h:7593:9, winuser.h:7593:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DOF_EXECUTABLE: i32 = 0x8001i32; /* Integer(32769, Yes, Unknown) */ /* winuser.h:7625:9, winuser.h:7625:9, winuser.h:7625:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DOF_DOCUMENT: i32 = 0x8002i32; /* Integer(32770, Yes, Unknown) */ /* winuser.h:7626:9, winuser.h:7626:9, winuser.h:7626:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DOF_DIRECTORY: i32 = 0x8003i32; /* Integer(32771, Yes, Unknown) */ /* winuser.h:7627:9, winuser.h:7627:9, winuser.h:7627:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DOF_MULTIPLE: i32 = 0x8004i32; /* Integer(32772, Yes, Unknown) */ /* winuser.h:7628:9, winuser.h:7628:9, winuser.h:7628:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DOF_PROGMAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:7629:9, winuser.h:7629:9, winuser.h:7629:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DOF_SHELLDATA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:7630:9, winuser.h:7630:9, winuser.h:7630:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DO_DROPFILE: i64 = 0x454c4946i64; /* Integer(1162627398, Yes, Long) */ /* winuser.h:7632:9, winuser.h:7632:9, winuser.h:7632:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DO_PRINTFILE: i64 = 0x544e5250i64; /* Integer(1414419024, Yes, Long) */ /* winuser.h:7633:9, winuser.h:7633:9, winuser.h:7633:9 */
pub const DT_TOP: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:7680:9, winuser.h:7680:9, winuser.h:7680:9 */
pub const DT_LEFT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:7681:9, winuser.h:7681:9, winuser.h:7681:9 */
pub const DT_CENTER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:7682:9, winuser.h:7682:9, winuser.h:7682:9 */
pub const DT_RIGHT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:7683:9, winuser.h:7683:9, winuser.h:7683:9 */
pub const DT_VCENTER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:7684:9, winuser.h:7684:9, winuser.h:7684:9 */
pub const DT_BOTTOM: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:7685:9, winuser.h:7685:9, winuser.h:7685:9 */
pub const DT_WORDBREAK: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:7686:9, winuser.h:7686:9, winuser.h:7686:9 */
pub const DT_SINGLELINE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:7687:9, winuser.h:7687:9, winuser.h:7687:9 */
pub const DT_EXPANDTABS: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:7688:9, winuser.h:7688:9, winuser.h:7688:9 */
pub const DT_TABSTOP: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:7689:9, winuser.h:7689:9, winuser.h:7689:9 */
pub const DT_NOCLIP: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:7690:9, winuser.h:7690:9, winuser.h:7690:9 */
pub const DT_EXTERNALLEADING: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:7691:9, winuser.h:7691:9, winuser.h:7691:9 */
pub const DT_CALCRECT: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winuser.h:7692:9, winuser.h:7692:9, winuser.h:7692:9 */
pub const DT_NOPREFIX: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winuser.h:7693:9, winuser.h:7693:9, winuser.h:7693:9 */
pub const DT_INTERNAL: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:7694:9, winuser.h:7694:9, winuser.h:7694:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DT_EDITCONTROL: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:7697:9, winuser.h:7697:9, winuser.h:7697:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DT_PATH_ELLIPSIS: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winuser.h:7698:9, winuser.h:7698:9, winuser.h:7698:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DT_END_ELLIPSIS: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:7699:9, winuser.h:7699:9, winuser.h:7699:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DT_MODIFYSTRING: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winuser.h:7700:9, winuser.h:7700:9, winuser.h:7700:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DT_RTLREADING: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winuser.h:7701:9, winuser.h:7701:9, winuser.h:7701:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DT_WORD_ELLIPSIS: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* winuser.h:7702:9, winuser.h:7702:9, winuser.h:7702:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DT_NOFULLWIDTHCHARBREAK: i32 = 0x80000i32; /* Integer(524288, Yes, Unknown) */ /* winuser.h:7704:9, winuser.h:7704:9, winuser.h:7704:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DT_HIDEPREFIX: i32 = 0x100000i32; /* Integer(1048576, Yes, Unknown) */ /* winuser.h:7706:9, winuser.h:7706:9, winuser.h:7706:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DT_PREFIXONLY: i32 = 0x200000i32; /* Integer(2097152, Yes, Unknown) */ /* winuser.h:7707:9, winuser.h:7707:9, winuser.h:7707:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DST_COMPLEX: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:7879:9, winuser.h:7879:9, winuser.h:7879:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DST_TEXT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:7880:9, winuser.h:7880:9, winuser.h:7880:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DST_PREFIXTEXT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:7881:9, winuser.h:7881:9, winuser.h:7881:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DST_ICON: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:7882:9, winuser.h:7882:9, winuser.h:7882:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DST_BITMAP: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:7883:9, winuser.h:7883:9, winuser.h:7883:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DSS_NORMAL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:7886:9, winuser.h:7886:9, winuser.h:7886:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DSS_UNION: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:7887:9, winuser.h:7887:9, winuser.h:7887:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DSS_DISABLED: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:7888:9, winuser.h:7888:9, winuser.h:7888:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DSS_MONO: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:7889:9, winuser.h:7889:9, winuser.h:7889:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DSS_HIDEPREFIX: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:7891:9, winuser.h:7891:9, winuser.h:7891:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DSS_PREFIXONLY: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winuser.h:7892:9, winuser.h:7892:9, winuser.h:7892:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DSS_RIGHT: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:7894:9, winuser.h:7894:9, winuser.h:7894:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const LSFW_LOCK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:8051:9, winuser.h:8051:9, winuser.h:8051:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const LSFW_UNLOCK: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:8052:9, winuser.h:8052:9, winuser.h:8052:9 */
pub const DCX_WINDOW: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:8082:9, winuser.h:8082:9, winuser.h:8082:9 */
pub const DCX_CACHE: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:8083:9, winuser.h:8083:9, winuser.h:8083:9 */
pub const DCX_NORESETATTRS: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:8084:9, winuser.h:8084:9, winuser.h:8084:9 */
pub const DCX_CLIPCHILDREN: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winuser.h:8085:9, winuser.h:8085:9, winuser.h:8085:9 */
pub const DCX_CLIPSIBLINGS: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* winuser.h:8086:9, winuser.h:8086:9, winuser.h:8086:9 */
pub const DCX_PARENTCLIP: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* winuser.h:8087:9, winuser.h:8087:9, winuser.h:8087:9 */
pub const DCX_EXCLUDERGN: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* winuser.h:8088:9, winuser.h:8088:9, winuser.h:8088:9 */
pub const DCX_INTERSECTRGN: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* winuser.h:8089:9, winuser.h:8089:9, winuser.h:8089:9 */
pub const DCX_EXCLUDEUPDATE: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* winuser.h:8090:9, winuser.h:8090:9, winuser.h:8090:9 */
pub const DCX_INTERSECTUPDATE: i64 = 0x200i64; /* Integer(512, Yes, Long) */ /* winuser.h:8091:9, winuser.h:8091:9, winuser.h:8091:9 */
pub const DCX_LOCKWINDOWUPDATE: i64 = 0x400i64; /* Integer(1024, Yes, Long) */ /* winuser.h:8092:9, winuser.h:8092:9, winuser.h:8092:9 */
pub const DCX_VALIDATE: i64 = 0x200000i64; /* Integer(2097152, Yes, Long) */ /* winuser.h:8094:9, winuser.h:8094:9, winuser.h:8094:9 */
pub const RDW_INVALIDATE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:8228:9, winuser.h:8228:9, winuser.h:8228:9 */
pub const RDW_INTERNALPAINT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:8229:9, winuser.h:8229:9, winuser.h:8229:9 */
pub const RDW_ERASE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:8230:9, winuser.h:8230:9, winuser.h:8230:9 */
pub const RDW_VALIDATE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:8232:9, winuser.h:8232:9, winuser.h:8232:9 */
pub const RDW_NOINTERNALPAINT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:8233:9, winuser.h:8233:9, winuser.h:8233:9 */
pub const RDW_NOERASE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:8234:9, winuser.h:8234:9, winuser.h:8234:9 */
pub const RDW_NOCHILDREN: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:8236:9, winuser.h:8236:9, winuser.h:8236:9 */
pub const RDW_ALLCHILDREN: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:8237:9, winuser.h:8237:9, winuser.h:8237:9 */
pub const RDW_UPDATENOW: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:8239:9, winuser.h:8239:9, winuser.h:8239:9 */
pub const RDW_ERASENOW: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:8240:9, winuser.h:8240:9, winuser.h:8240:9 */
pub const RDW_FRAME: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winuser.h:8242:9, winuser.h:8242:9, winuser.h:8242:9 */
pub const RDW_NOFRAME: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winuser.h:8243:9, winuser.h:8243:9, winuser.h:8243:9 */
pub const SW_SCROLLCHILDREN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:8297:9, winuser.h:8297:9, winuser.h:8297:9 */
pub const SW_INVALIDATE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:8298:9, winuser.h:8298:9, winuser.h:8298:9 */
pub const SW_ERASE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:8299:9, winuser.h:8299:9, winuser.h:8299:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SW_SMOOTHSCROLL: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:8301:9, winuser.h:8301:9, winuser.h:8301:9 */
#[cfg(feature="winapi_desktop")] pub const ESB_ENABLE_BOTH: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:8364:9, winuser.h:8364:9, winuser.h:8364:9 */
#[cfg(feature="winapi_desktop")] pub const ESB_DISABLE_BOTH: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:8365:9, winuser.h:8365:9, winuser.h:8365:9 */
#[cfg(feature="winapi_desktop")] pub const ESB_DISABLE_LEFT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:8367:9, winuser.h:8367:9, winuser.h:8367:9 */
#[cfg(feature="winapi_desktop")] pub const ESB_DISABLE_RIGHT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:8368:9, winuser.h:8368:9, winuser.h:8368:9 */
#[cfg(feature="winapi_desktop")] pub const ESB_DISABLE_UP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:8370:9, winuser.h:8370:9, winuser.h:8370:9 */
#[cfg(feature="winapi_desktop")] pub const ESB_DISABLE_DOWN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:8371:9, winuser.h:8371:9, winuser.h:8371:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winuser::ESB_DISABLE_LEFT as ESB_DISABLE_LTUP; /* winuser.h:8373:9, winuser.h:8373:9, winuser.h:8373:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winuser::ESB_DISABLE_RIGHT as ESB_DISABLE_RTDN; /* winuser.h:8374:9, winuser.h:8374:9, winuser.h:8374:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const HELPINFO_WINDOW: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:8565:9, winuser.h:8565:9, winuser.h:8565:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const HELPINFO_MENUITEM: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:8566:9, winuser.h:8566:9, winuser.h:8566:9 */
pub const MB_OK: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:8618:9, winuser.h:8618:9, winuser.h:8618:9 */
pub const MB_OKCANCEL: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:8619:9, winuser.h:8619:9, winuser.h:8619:9 */
pub const MB_ABORTRETRYIGNORE: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:8620:9, winuser.h:8620:9, winuser.h:8620:9 */
pub const MB_YESNOCANCEL: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* winuser.h:8621:9, winuser.h:8621:9, winuser.h:8621:9 */
pub const MB_YESNO: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:8622:9, winuser.h:8622:9, winuser.h:8622:9 */
pub const MB_RETRYCANCEL: i64 = 0x5i64; /* Integer(5, Yes, Long) */ /* winuser.h:8623:9, winuser.h:8623:9, winuser.h:8623:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const MB_CANCELTRYCONTINUE: i64 = 0x6i64; /* Integer(6, Yes, Long) */ /* winuser.h:8625:9, winuser.h:8625:9, winuser.h:8625:9 */
pub const MB_ICONHAND: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* winuser.h:8629:9, winuser.h:8629:9, winuser.h:8629:9 */
pub const MB_ICONQUESTION: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* winuser.h:8630:9, winuser.h:8630:9, winuser.h:8630:9 */
pub const MB_ICONEXCLAMATION: i64 = 0x30i64; /* Integer(48, Yes, Long) */ /* winuser.h:8631:9, winuser.h:8631:9, winuser.h:8631:9 */
pub const MB_ICONASTERISK: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* winuser.h:8632:9, winuser.h:8632:9, winuser.h:8632:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const MB_USERICON: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* winuser.h:8635:9, winuser.h:8635:9, winuser.h:8635:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MB_ICONEXCLAMATION as MB_ICONWARNING; /* winuser.h:8636:9, winuser.h:8636:9, winuser.h:8636:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MB_ICONHAND as MB_ICONERROR; /* winuser.h:8637:9, winuser.h:8637:9, winuser.h:8637:9 */
#[doc(inline)] pub use ::winuser::MB_ICONASTERISK as MB_ICONINFORMATION; /* winuser.h:8640:9, winuser.h:8640:9, winuser.h:8640:9 */
#[doc(inline)] pub use ::winuser::MB_ICONHAND as MB_ICONSTOP; /* winuser.h:8641:9, winuser.h:8641:9, winuser.h:8641:9 */
pub const MB_DEFBUTTON1: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:8643:9, winuser.h:8643:9, winuser.h:8643:9 */
pub const MB_DEFBUTTON2: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* winuser.h:8644:9, winuser.h:8644:9, winuser.h:8644:9 */
pub const MB_DEFBUTTON3: i64 = 0x200i64; /* Integer(512, Yes, Long) */ /* winuser.h:8645:9, winuser.h:8645:9, winuser.h:8645:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const MB_DEFBUTTON4: i64 = 0x300i64; /* Integer(768, Yes, Long) */ /* winuser.h:8647:9, winuser.h:8647:9, winuser.h:8647:9 */
pub const MB_APPLMODAL: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:8650:9, winuser.h:8650:9, winuser.h:8650:9 */
pub const MB_SYSTEMMODAL: i64 = 0x1000i64; /* Integer(4096, Yes, Long) */ /* winuser.h:8651:9, winuser.h:8651:9, winuser.h:8651:9 */
pub const MB_TASKMODAL: i64 = 0x2000i64; /* Integer(8192, Yes, Long) */ /* winuser.h:8652:9, winuser.h:8652:9, winuser.h:8652:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const MB_HELP: i64 = 0x4000i64; /* Integer(16384, Yes, Long) */ /* winuser.h:8654:9, winuser.h:8654:9, winuser.h:8654:9 */
pub const MB_NOFOCUS: i64 = 0x8000i64; /* Integer(32768, Yes, Long) */ /* winuser.h:8657:9, winuser.h:8657:9, winuser.h:8657:9 */
pub const MB_SETFOREGROUND: i64 = 0x10000i64; /* Integer(65536, Yes, Long) */ /* winuser.h:8658:9, winuser.h:8658:9, winuser.h:8658:9 */
pub const MB_DEFAULT_DESKTOP_ONLY: i64 = 0x20000i64; /* Integer(131072, Yes, Long) */ /* winuser.h:8659:9, winuser.h:8659:9, winuser.h:8659:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const MB_TOPMOST: i64 = 0x40000i64; /* Integer(262144, Yes, Long) */ /* winuser.h:8662:9, winuser.h:8662:9, winuser.h:8662:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const MB_RIGHT: i64 = 0x80000i64; /* Integer(524288, Yes, Long) */ /* winuser.h:8663:9, winuser.h:8663:9, winuser.h:8663:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const MB_RTLREADING: i64 = 0x100000i64; /* Integer(1048576, Yes, Long) */ /* winuser.h:8664:9, winuser.h:8664:9, winuser.h:8664:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const MB_SERVICE_NOTIFICATION: i64 = 0x200000i64; /* Integer(2097152, Yes, Long) */ /* winuser.h:8670:9, winuser.h:8670:9, winuser.h:8670:9 */
pub const MB_SERVICE_NOTIFICATION_NT3X: i64 = 0x40000i64; /* Integer(262144, Yes, Long) */ /* winuser.h:8674:9, winuser.h:8674:9, winuser.h:8674:9 */
pub const MB_TYPEMASK: i64 = 0xfi64; /* Integer(15, Yes, Long) */ /* winuser.h:8677:9, winuser.h:8677:9, winuser.h:8677:9 */
pub const MB_ICONMASK: i64 = 0xf0i64; /* Integer(240, Yes, Long) */ /* winuser.h:8678:9, winuser.h:8678:9, winuser.h:8678:9 */
pub const MB_DEFMASK: i64 = 0xf00i64; /* Integer(3840, Yes, Long) */ /* winuser.h:8679:9, winuser.h:8679:9, winuser.h:8679:9 */
pub const MB_MODEMASK: i64 = 0x3000i64; /* Integer(12288, Yes, Long) */ /* winuser.h:8680:9, winuser.h:8680:9, winuser.h:8680:9 */
pub const MB_MISCMASK: i64 = 0xc000i64; /* Integer(49152, Yes, Long) */ /* winuser.h:8681:9, winuser.h:8681:9, winuser.h:8681:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CWP_ALL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:9031:9, winuser.h:9031:9, winuser.h:9031:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CWP_SKIPINVISIBLE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:9032:9, winuser.h:9032:9, winuser.h:9032:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CWP_SKIPDISABLED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:9033:9, winuser.h:9033:9, winuser.h:9033:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CWP_SKIPTRANSPARENT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:9034:9, winuser.h:9034:9, winuser.h:9034:9 */
pub const CTLCOLOR_MSGBOX: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:9057:9, winuser.h:9057:9, winuser.h:9057:9 */
pub const CTLCOLOR_EDIT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:9058:9, winuser.h:9058:9, winuser.h:9058:9 */
pub const CTLCOLOR_LISTBOX: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:9059:9, winuser.h:9059:9, winuser.h:9059:9 */
pub const CTLCOLOR_BTN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:9060:9, winuser.h:9060:9, winuser.h:9060:9 */
pub const CTLCOLOR_DLG: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:9061:9, winuser.h:9061:9, winuser.h:9061:9 */
pub const CTLCOLOR_SCROLLBAR: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:9062:9, winuser.h:9062:9, winuser.h:9062:9 */
pub const CTLCOLOR_STATIC: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:9063:9, winuser.h:9063:9, winuser.h:9063:9 */
pub const CTLCOLOR_MAX: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:9064:9, winuser.h:9064:9, winuser.h:9064:9 */
pub const COLOR_SCROLLBAR: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:9066:9, winuser.h:9066:9, winuser.h:9066:9 */
pub const COLOR_BACKGROUND: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:9067:9, winuser.h:9067:9, winuser.h:9067:9 */
pub const COLOR_ACTIVECAPTION: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:9068:9, winuser.h:9068:9, winuser.h:9068:9 */
pub const COLOR_INACTIVECAPTION: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:9069:9, winuser.h:9069:9, winuser.h:9069:9 */
pub const COLOR_MENU: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:9070:9, winuser.h:9070:9, winuser.h:9070:9 */
pub const COLOR_WINDOW: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:9071:9, winuser.h:9071:9, winuser.h:9071:9 */
pub const COLOR_WINDOWFRAME: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:9072:9, winuser.h:9072:9, winuser.h:9072:9 */
pub const COLOR_MENUTEXT: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:9073:9, winuser.h:9073:9, winuser.h:9073:9 */
pub const COLOR_WINDOWTEXT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:9074:9, winuser.h:9074:9, winuser.h:9074:9 */
pub const COLOR_CAPTIONTEXT: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:9075:9, winuser.h:9075:9, winuser.h:9075:9 */
pub const COLOR_ACTIVEBORDER: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:9076:9, winuser.h:9076:9, winuser.h:9076:9 */
pub const COLOR_INACTIVEBORDER: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:9077:9, winuser.h:9077:9, winuser.h:9077:9 */
pub const COLOR_APPWORKSPACE: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winuser.h:9078:9, winuser.h:9078:9, winuser.h:9078:9 */
pub const COLOR_HIGHLIGHT: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winuser.h:9079:9, winuser.h:9079:9, winuser.h:9079:9 */
pub const COLOR_HIGHLIGHTTEXT: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winuser.h:9080:9, winuser.h:9080:9, winuser.h:9080:9 */
pub const COLOR_BTNFACE: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winuser.h:9081:9, winuser.h:9081:9, winuser.h:9081:9 */
pub const COLOR_BTNSHADOW: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:9082:9, winuser.h:9082:9, winuser.h:9082:9 */
pub const COLOR_GRAYTEXT: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winuser.h:9083:9, winuser.h:9083:9, winuser.h:9083:9 */
pub const COLOR_BTNTEXT: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winuser.h:9084:9, winuser.h:9084:9, winuser.h:9084:9 */
pub const COLOR_INACTIVECAPTIONTEXT: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winuser.h:9085:9, winuser.h:9085:9, winuser.h:9085:9 */
pub const COLOR_BTNHIGHLIGHT: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winuser.h:9086:9, winuser.h:9086:9, winuser.h:9086:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const COLOR_3DDKSHADOW: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winuser.h:9089:9, winuser.h:9089:9, winuser.h:9089:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const COLOR_3DLIGHT: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winuser.h:9090:9, winuser.h:9090:9, winuser.h:9090:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const COLOR_INFOTEXT: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winuser.h:9091:9, winuser.h:9091:9, winuser.h:9091:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const COLOR_INFOBK: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winuser.h:9092:9, winuser.h:9092:9, winuser.h:9092:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const COLOR_HOTLIGHT: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* winuser.h:9096:9, winuser.h:9096:9, winuser.h:9096:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const COLOR_GRADIENTACTIVECAPTION: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* winuser.h:9097:9, winuser.h:9097:9, winuser.h:9097:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const COLOR_GRADIENTINACTIVECAPTION: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* winuser.h:9098:9, winuser.h:9098:9, winuser.h:9098:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const COLOR_MENUHILIGHT: i32 = 0x1di32; /* Integer(29, Yes, Unknown) */ /* winuser.h:9100:9, winuser.h:9100:9, winuser.h:9100:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const COLOR_MENUBAR: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* winuser.h:9101:9, winuser.h:9101:9, winuser.h:9101:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::COLOR_BACKGROUND as COLOR_DESKTOP; /* winuser.h:9106:9, winuser.h:9106:9, winuser.h:9106:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::COLOR_BTNFACE as COLOR_3DFACE; /* winuser.h:9107:9, winuser.h:9107:9, winuser.h:9107:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::COLOR_BTNSHADOW as COLOR_3DSHADOW; /* winuser.h:9108:9, winuser.h:9108:9, winuser.h:9108:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::COLOR_BTNHIGHLIGHT as COLOR_3DHIGHLIGHT; /* winuser.h:9109:9, winuser.h:9109:9, winuser.h:9109:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::COLOR_BTNHIGHLIGHT as COLOR_3DHILIGHT; /* winuser.h:9110:9, winuser.h:9110:9, winuser.h:9110:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::COLOR_BTNHIGHLIGHT as COLOR_BTNHILIGHT; /* winuser.h:9111:9, winuser.h:9111:9, winuser.h:9111:9 */
/* #[cfg(feature="winapi_desktop")] // #define EnumTaskWindows(hTask,lpfn,lParam) Call { subject: Ident("EnumThreadWindows"), args: [Call { subject: Ident("HandleToUlong"), args: [Ident("hTask")] }, Ident("lpfn"), Ident("lParam")] } */ /* winuser.h:9614:9, winuser.h:9614:9, winuser.h:9614:9 */
/* #[cfg(feature="winapi_desktop")] // #define GetNextWindow(hWnd,wCmd) Call { subject: Ident("GetWindow"), args: [Ident("hWnd"), Ident("wCmd")] } */ /* winuser.h:9668:9, winuser.h:9668:9, winuser.h:9668:9 */
#[cfg(feature="winapi_desktop")] pub const GW_HWNDFIRST: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:9701:9, winuser.h:9701:9, winuser.h:9701:9 */
#[cfg(feature="winapi_desktop")] pub const GW_HWNDLAST: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:9702:9, winuser.h:9702:9, winuser.h:9702:9 */
#[cfg(feature="winapi_desktop")] pub const GW_HWNDNEXT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:9703:9, winuser.h:9703:9, winuser.h:9703:9 */
#[cfg(feature="winapi_desktop")] pub const GW_HWNDPREV: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:9704:9, winuser.h:9704:9, winuser.h:9704:9 */
#[cfg(feature="winapi_desktop")] pub const GW_OWNER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:9705:9, winuser.h:9705:9, winuser.h:9705:9 */
#[cfg(feature="winapi_desktop")] pub const GW_CHILD: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:9706:9, winuser.h:9706:9, winuser.h:9706:9 */
#[cfg(feature="winapi_desktop")] pub const GW_ENABLEDPOPUP: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:9710:9, winuser.h:9710:9, winuser.h:9710:9 */
#[cfg(feature="winapi_desktop")] pub const GW_MAX: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:9711:9, winuser.h:9711:9, winuser.h:9711:9 */
pub const MF_INSERT: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:9832:9, winuser.h:9832:9, winuser.h:9832:9 */
pub const MF_CHANGE: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* winuser.h:9833:9, winuser.h:9833:9, winuser.h:9833:9 */
pub const MF_APPEND: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* winuser.h:9834:9, winuser.h:9834:9, winuser.h:9834:9 */
pub const MF_DELETE: i64 = 0x200i64; /* Integer(512, Yes, Long) */ /* winuser.h:9835:9, winuser.h:9835:9, winuser.h:9835:9 */
pub const MF_REMOVE: i64 = 0x1000i64; /* Integer(4096, Yes, Long) */ /* winuser.h:9836:9, winuser.h:9836:9, winuser.h:9836:9 */
pub const MF_BYCOMMAND: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:9838:9, winuser.h:9838:9, winuser.h:9838:9 */
pub const MF_BYPOSITION: i64 = 0x400i64; /* Integer(1024, Yes, Long) */ /* winuser.h:9839:9, winuser.h:9839:9, winuser.h:9839:9 */
pub const MF_SEPARATOR: i64 = 0x800i64; /* Integer(2048, Yes, Long) */ /* winuser.h:9841:9, winuser.h:9841:9, winuser.h:9841:9 */
pub const MF_ENABLED: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:9843:9, winuser.h:9843:9, winuser.h:9843:9 */
pub const MF_GRAYED: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:9844:9, winuser.h:9844:9, winuser.h:9844:9 */
pub const MF_DISABLED: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:9845:9, winuser.h:9845:9, winuser.h:9845:9 */
pub const MF_UNCHECKED: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:9847:9, winuser.h:9847:9, winuser.h:9847:9 */
pub const MF_CHECKED: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winuser.h:9848:9, winuser.h:9848:9, winuser.h:9848:9 */
pub const MF_USECHECKBITMAPS: i64 = 0x200i64; /* Integer(512, Yes, Long) */ /* winuser.h:9849:9, winuser.h:9849:9, winuser.h:9849:9 */
pub const MF_STRING: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:9851:9, winuser.h:9851:9, winuser.h:9851:9 */
pub const MF_BITMAP: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:9852:9, winuser.h:9852:9, winuser.h:9852:9 */
pub const MF_OWNERDRAW: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* winuser.h:9853:9, winuser.h:9853:9, winuser.h:9853:9 */
pub const MF_POPUP: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* winuser.h:9855:9, winuser.h:9855:9, winuser.h:9855:9 */
pub const MF_MENUBARBREAK: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* winuser.h:9856:9, winuser.h:9856:9, winuser.h:9856:9 */
pub const MF_MENUBREAK: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* winuser.h:9857:9, winuser.h:9857:9, winuser.h:9857:9 */
pub const MF_UNHILITE: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:9859:9, winuser.h:9859:9, winuser.h:9859:9 */
pub const MF_HILITE: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* winuser.h:9860:9, winuser.h:9860:9, winuser.h:9860:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const MF_DEFAULT: i64 = 0x1000i64; /* Integer(4096, Yes, Long) */ /* winuser.h:9863:9, winuser.h:9863:9, winuser.h:9863:9 */
pub const MF_SYSMENU: i64 = 0x2000i64; /* Integer(8192, Yes, Long) */ /* winuser.h:9865:9, winuser.h:9865:9, winuser.h:9865:9 */
pub const MF_HELP: i64 = 0x4000i64; /* Integer(16384, Yes, Long) */ /* winuser.h:9866:9, winuser.h:9866:9, winuser.h:9866:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const MF_RIGHTJUSTIFY: i64 = 0x4000i64; /* Integer(16384, Yes, Long) */ /* winuser.h:9868:9, winuser.h:9868:9, winuser.h:9868:9 */
pub const MF_MOUSESELECT: i64 = 0x8000i64; /* Integer(32768, Yes, Long) */ /* winuser.h:9871:9, winuser.h:9871:9, winuser.h:9871:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const MF_END: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* winuser.h:9873:9, winuser.h:9873:9, winuser.h:9873:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MF_STRING as MFT_STRING; /* winuser.h:9878:9, winuser.h:9878:9, winuser.h:9878:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MF_BITMAP as MFT_BITMAP; /* winuser.h:9879:9, winuser.h:9879:9, winuser.h:9879:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MF_MENUBARBREAK as MFT_MENUBARBREAK; /* winuser.h:9880:9, winuser.h:9880:9, winuser.h:9880:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MF_MENUBREAK as MFT_MENUBREAK; /* winuser.h:9881:9, winuser.h:9881:9, winuser.h:9881:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MF_OWNERDRAW as MFT_OWNERDRAW; /* winuser.h:9882:9, winuser.h:9882:9, winuser.h:9882:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const MFT_RADIOCHECK: i64 = 0x200i64; /* Integer(512, Yes, Long) */ /* winuser.h:9883:9, winuser.h:9883:9, winuser.h:9883:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MF_SEPARATOR as MFT_SEPARATOR; /* winuser.h:9884:9, winuser.h:9884:9, winuser.h:9884:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const MFT_RIGHTORDER: i64 = 0x2000i64; /* Integer(8192, Yes, Long) */ /* winuser.h:9885:9, winuser.h:9885:9, winuser.h:9885:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MF_RIGHTJUSTIFY as MFT_RIGHTJUSTIFY; /* winuser.h:9886:9, winuser.h:9886:9, winuser.h:9886:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const MFS_GRAYED: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* winuser.h:9889:9, winuser.h:9889:9, winuser.h:9889:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MFS_GRAYED as MFS_DISABLED; /* winuser.h:9890:9, winuser.h:9890:9, winuser.h:9890:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MF_CHECKED as MFS_CHECKED; /* winuser.h:9891:9, winuser.h:9891:9, winuser.h:9891:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MF_HILITE as MFS_HILITE; /* winuser.h:9892:9, winuser.h:9892:9, winuser.h:9892:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MF_ENABLED as MFS_ENABLED; /* winuser.h:9893:9, winuser.h:9893:9, winuser.h:9893:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MF_UNCHECKED as MFS_UNCHECKED; /* winuser.h:9894:9, winuser.h:9894:9, winuser.h:9894:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MF_UNHILITE as MFS_UNHILITE; /* winuser.h:9895:9, winuser.h:9895:9, winuser.h:9895:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::MF_DEFAULT as MFS_DEFAULT; /* winuser.h:9896:9, winuser.h:9896:9, winuser.h:9896:9 */
pub const SC_SIZE: i32 = 0xf000i32; /* Integer(61440, Yes, Unknown) */ /* winuser.h:9941:9, winuser.h:9941:9, winuser.h:9941:9 */
pub const SC_MOVE: i32 = 0xf010i32; /* Integer(61456, Yes, Unknown) */ /* winuser.h:9942:9, winuser.h:9942:9, winuser.h:9942:9 */
pub const SC_MINIMIZE: i32 = 0xf020i32; /* Integer(61472, Yes, Unknown) */ /* winuser.h:9943:9, winuser.h:9943:9, winuser.h:9943:9 */
pub const SC_MAXIMIZE: i32 = 0xf030i32; /* Integer(61488, Yes, Unknown) */ /* winuser.h:9944:9, winuser.h:9944:9, winuser.h:9944:9 */
pub const SC_NEXTWINDOW: i32 = 0xf040i32; /* Integer(61504, Yes, Unknown) */ /* winuser.h:9945:9, winuser.h:9945:9, winuser.h:9945:9 */
pub const SC_PREVWINDOW: i32 = 0xf050i32; /* Integer(61520, Yes, Unknown) */ /* winuser.h:9946:9, winuser.h:9946:9, winuser.h:9946:9 */
pub const SC_CLOSE: i32 = 0xf060i32; /* Integer(61536, Yes, Unknown) */ /* winuser.h:9947:9, winuser.h:9947:9, winuser.h:9947:9 */
pub const SC_VSCROLL: i32 = 0xf070i32; /* Integer(61552, Yes, Unknown) */ /* winuser.h:9948:9, winuser.h:9948:9, winuser.h:9948:9 */
pub const SC_HSCROLL: i32 = 0xf080i32; /* Integer(61568, Yes, Unknown) */ /* winuser.h:9949:9, winuser.h:9949:9, winuser.h:9949:9 */
pub const SC_MOUSEMENU: i32 = 0xf090i32; /* Integer(61584, Yes, Unknown) */ /* winuser.h:9950:9, winuser.h:9950:9, winuser.h:9950:9 */
pub const SC_KEYMENU: i32 = 0xf100i32; /* Integer(61696, Yes, Unknown) */ /* winuser.h:9951:9, winuser.h:9951:9, winuser.h:9951:9 */
pub const SC_ARRANGE: i32 = 0xf110i32; /* Integer(61712, Yes, Unknown) */ /* winuser.h:9952:9, winuser.h:9952:9, winuser.h:9952:9 */
pub const SC_RESTORE: i32 = 0xf120i32; /* Integer(61728, Yes, Unknown) */ /* winuser.h:9953:9, winuser.h:9953:9, winuser.h:9953:9 */
pub const SC_TASKLIST: i32 = 0xf130i32; /* Integer(61744, Yes, Unknown) */ /* winuser.h:9954:9, winuser.h:9954:9, winuser.h:9954:9 */
pub const SC_SCREENSAVE: i32 = 0xf140i32; /* Integer(61760, Yes, Unknown) */ /* winuser.h:9955:9, winuser.h:9955:9, winuser.h:9955:9 */
pub const SC_HOTKEY: i32 = 0xf150i32; /* Integer(61776, Yes, Unknown) */ /* winuser.h:9956:9, winuser.h:9956:9, winuser.h:9956:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SC_DEFAULT: i32 = 0xf160i32; /* Integer(61792, Yes, Unknown) */ /* winuser.h:9958:9, winuser.h:9958:9, winuser.h:9958:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SC_MONITORPOWER: i32 = 0xf170i32; /* Integer(61808, Yes, Unknown) */ /* winuser.h:9959:9, winuser.h:9959:9, winuser.h:9959:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SC_CONTEXTHELP: i32 = 0xf180i32; /* Integer(61824, Yes, Unknown) */ /* winuser.h:9960:9, winuser.h:9960:9, winuser.h:9960:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SC_SEPARATOR: i32 = 0xf00fi32; /* Integer(61455, Yes, Unknown) */ /* winuser.h:9961:9, winuser.h:9961:9, winuser.h:9961:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SCF_ISSECURE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:9965:9, winuser.h:9965:9, winuser.h:9965:9 */
#[doc(inline)] pub use ::winuser::SC_MINIMIZE as SC_ICON; /* winuser.h:9973:9, winuser.h:9973:9, winuser.h:9973:9 */
#[doc(inline)] pub use ::winuser::SC_MAXIMIZE as SC_ZOOM; /* winuser.h:9974:9, winuser.h:9974:9, winuser.h:9974:9 */
pub const IMAGE_BITMAP: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:10235:9, winuser.h:10235:9, winuser.h:10235:9 */
pub const IMAGE_ICON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:10236:9, winuser.h:10236:9, winuser.h:10236:9 */
pub const IMAGE_CURSOR: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:10237:9, winuser.h:10237:9, winuser.h:10237:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const IMAGE_ENHMETAFILE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:10239:9, winuser.h:10239:9, winuser.h:10239:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LR_DEFAULTCOLOR: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:10241:9, winuser.h:10241:9, winuser.h:10241:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LR_MONOCHROME: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:10242:9, winuser.h:10242:9, winuser.h:10242:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LR_COLOR: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:10243:9, winuser.h:10243:9, winuser.h:10243:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LR_COPYRETURNORG: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:10244:9, winuser.h:10244:9, winuser.h:10244:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LR_COPYDELETEORG: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:10245:9, winuser.h:10245:9, winuser.h:10245:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LR_LOADFROMFILE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:10246:9, winuser.h:10246:9, winuser.h:10246:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LR_LOADTRANSPARENT: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:10247:9, winuser.h:10247:9, winuser.h:10247:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LR_DEFAULTSIZE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:10248:9, winuser.h:10248:9, winuser.h:10248:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LR_VGACOLOR: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:10249:9, winuser.h:10249:9, winuser.h:10249:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LR_LOADMAP3DCOLORS: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:10250:9, winuser.h:10250:9, winuser.h:10250:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LR_CREATEDIBSECTION: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:10251:9, winuser.h:10251:9, winuser.h:10251:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LR_COPYFROMRESOURCE: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winuser.h:10252:9, winuser.h:10252:9, winuser.h:10252:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LR_SHARED: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:10253:9, winuser.h:10253:9, winuser.h:10253:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const DI_MASK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:10294:9, winuser.h:10294:9, winuser.h:10294:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const DI_IMAGE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:10295:9, winuser.h:10295:9, winuser.h:10295:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const DI_NORMAL: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:10296:9, winuser.h:10296:9, winuser.h:10296:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const DI_COMPAT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:10297:9, winuser.h:10297:9, winuser.h:10297:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const DI_DEFAULTSIZE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:10298:9, winuser.h:10298:9, winuser.h:10298:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] pub const DI_NOMIRROR: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:10300:9, winuser.h:10300:9, winuser.h:10300:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const RES_ICON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:10392:9, winuser.h:10392:9, winuser.h:10392:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const RES_CURSOR: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:10393:9, winuser.h:10393:9, winuser.h:10393:9 */
pub const ORD_LANGDRIVER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:10483:9, winuser.h:10483:9, winuser.h:10483:9 */
pub const IDOK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:10564:9, winuser.h:10564:9, winuser.h:10564:9 */
pub const IDCANCEL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:10565:9, winuser.h:10565:9, winuser.h:10565:9 */
pub const IDABORT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:10566:9, winuser.h:10566:9, winuser.h:10566:9 */
pub const IDRETRY: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:10567:9, winuser.h:10567:9, winuser.h:10567:9 */
pub const IDIGNORE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:10568:9, winuser.h:10568:9, winuser.h:10568:9 */
pub const IDYES: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:10569:9, winuser.h:10569:9, winuser.h:10569:9 */
pub const IDNO: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:10570:9, winuser.h:10570:9, winuser.h:10570:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const IDCLOSE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:10572:9, winuser.h:10572:9, winuser.h:10572:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const IDHELP: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:10573:9, winuser.h:10573:9, winuser.h:10573:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IDTRYAGAIN: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:10577:9, winuser.h:10577:9, winuser.h:10577:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IDCONTINUE: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:10578:9, winuser.h:10578:9, winuser.h:10578:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const IDTIMEOUT: i32 = 0x7d00i32; /* Integer(32000, Yes, Unknown) */ /* winuser.h:10583:9, winuser.h:10583:9, winuser.h:10583:9 */
pub const ES_LEFT: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:10600:9, winuser.h:10600:9, winuser.h:10600:9 */
pub const ES_CENTER: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:10601:9, winuser.h:10601:9, winuser.h:10601:9 */
pub const ES_RIGHT: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:10602:9, winuser.h:10602:9, winuser.h:10602:9 */
pub const ES_MULTILINE: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:10603:9, winuser.h:10603:9, winuser.h:10603:9 */
pub const ES_UPPERCASE: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winuser.h:10604:9, winuser.h:10604:9, winuser.h:10604:9 */
pub const ES_LOWERCASE: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* winuser.h:10605:9, winuser.h:10605:9, winuser.h:10605:9 */
pub const ES_PASSWORD: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* winuser.h:10606:9, winuser.h:10606:9, winuser.h:10606:9 */
pub const ES_AUTOVSCROLL: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* winuser.h:10607:9, winuser.h:10607:9, winuser.h:10607:9 */
pub const ES_AUTOHSCROLL: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* winuser.h:10608:9, winuser.h:10608:9, winuser.h:10608:9 */
pub const ES_NOHIDESEL: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* winuser.h:10609:9, winuser.h:10609:9, winuser.h:10609:9 */
pub const ES_OEMCONVERT: i64 = 0x400i64; /* Integer(1024, Yes, Long) */ /* winuser.h:10610:9, winuser.h:10610:9, winuser.h:10610:9 */
pub const ES_READONLY: i64 = 0x800i64; /* Integer(2048, Yes, Long) */ /* winuser.h:10611:9, winuser.h:10611:9, winuser.h:10611:9 */
pub const ES_WANTRETURN: i64 = 0x1000i64; /* Integer(4096, Yes, Long) */ /* winuser.h:10612:9, winuser.h:10612:9, winuser.h:10612:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const ES_NUMBER: i64 = 0x2000i64; /* Integer(8192, Yes, Long) */ /* winuser.h:10614:9, winuser.h:10614:9, winuser.h:10614:9 */
pub const EN_SETFOCUS: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:10623:9, winuser.h:10623:9, winuser.h:10623:9 */
pub const EN_KILLFOCUS: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:10624:9, winuser.h:10624:9, winuser.h:10624:9 */
pub const EN_CHANGE: i32 = 0x300i32; /* Integer(768, Yes, Unknown) */ /* winuser.h:10625:9, winuser.h:10625:9, winuser.h:10625:9 */
pub const EN_UPDATE: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winuser.h:10626:9, winuser.h:10626:9, winuser.h:10626:9 */
pub const EN_ERRSPACE: i32 = 0x500i32; /* Integer(1280, Yes, Unknown) */ /* winuser.h:10627:9, winuser.h:10627:9, winuser.h:10627:9 */
pub const EN_MAXTEXT: i32 = 0x501i32; /* Integer(1281, Yes, Unknown) */ /* winuser.h:10628:9, winuser.h:10628:9, winuser.h:10628:9 */
pub const EN_HSCROLL: i32 = 0x601i32; /* Integer(1537, Yes, Unknown) */ /* winuser.h:10629:9, winuser.h:10629:9, winuser.h:10629:9 */
pub const EN_VSCROLL: i32 = 0x602i32; /* Integer(1538, Yes, Unknown) */ /* winuser.h:10630:9, winuser.h:10630:9, winuser.h:10630:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EN_ALIGN_LTR_EC: i32 = 0x700i32; /* Integer(1792, Yes, Unknown) */ /* winuser.h:10633:9, winuser.h:10633:9, winuser.h:10633:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EN_ALIGN_RTL_EC: i32 = 0x701i32; /* Integer(1793, Yes, Unknown) */ /* winuser.h:10634:9, winuser.h:10634:9, winuser.h:10634:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const EC_LEFTMARGIN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:10639:9, winuser.h:10639:9, winuser.h:10639:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const EC_RIGHTMARGIN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:10640:9, winuser.h:10640:9, winuser.h:10640:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const EC_USEFONTINFO: i32 = 0xffffi32; /* Integer(65535, Yes, Unknown) */ /* winuser.h:10641:9, winuser.h:10641:9, winuser.h:10641:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EMSIS_COMPOSITIONSTRING: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:10646:9, winuser.h:10646:9, winuser.h:10646:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EIMES_GETCOMPSTRATONCE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:10649:9, winuser.h:10649:9, winuser.h:10649:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EIMES_CANCELCOMPSTRINFOCUS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:10650:9, winuser.h:10650:9, winuser.h:10650:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EIMES_COMPLETECOMPSTRKILLFOCUS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:10651:9, winuser.h:10651:9, winuser.h:10651:9 */
pub const EM_GETSEL: i32 = 0xb0i32; /* Integer(176, Yes, Unknown) */ /* winuser.h:10660:9, winuser.h:10660:9, winuser.h:10660:9 */
pub const EM_SETSEL: i32 = 0xb1i32; /* Integer(177, Yes, Unknown) */ /* winuser.h:10661:9, winuser.h:10661:9, winuser.h:10661:9 */
pub const EM_GETRECT: i32 = 0xb2i32; /* Integer(178, Yes, Unknown) */ /* winuser.h:10662:9, winuser.h:10662:9, winuser.h:10662:9 */
pub const EM_SETRECT: i32 = 0xb3i32; /* Integer(179, Yes, Unknown) */ /* winuser.h:10663:9, winuser.h:10663:9, winuser.h:10663:9 */
pub const EM_SETRECTNP: i32 = 0xb4i32; /* Integer(180, Yes, Unknown) */ /* winuser.h:10664:9, winuser.h:10664:9, winuser.h:10664:9 */
pub const EM_SCROLL: i32 = 0xb5i32; /* Integer(181, Yes, Unknown) */ /* winuser.h:10665:9, winuser.h:10665:9, winuser.h:10665:9 */
pub const EM_LINESCROLL: i32 = 0xb6i32; /* Integer(182, Yes, Unknown) */ /* winuser.h:10666:9, winuser.h:10666:9, winuser.h:10666:9 */
pub const EM_SCROLLCARET: i32 = 0xb7i32; /* Integer(183, Yes, Unknown) */ /* winuser.h:10667:9, winuser.h:10667:9, winuser.h:10667:9 */
pub const EM_GETMODIFY: i32 = 0xb8i32; /* Integer(184, Yes, Unknown) */ /* winuser.h:10668:9, winuser.h:10668:9, winuser.h:10668:9 */
pub const EM_SETMODIFY: i32 = 0xb9i32; /* Integer(185, Yes, Unknown) */ /* winuser.h:10669:9, winuser.h:10669:9, winuser.h:10669:9 */
pub const EM_GETLINECOUNT: i32 = 0xbai32; /* Integer(186, Yes, Unknown) */ /* winuser.h:10670:9, winuser.h:10670:9, winuser.h:10670:9 */
pub const EM_LINEINDEX: i32 = 0xbbi32; /* Integer(187, Yes, Unknown) */ /* winuser.h:10671:9, winuser.h:10671:9, winuser.h:10671:9 */
pub const EM_SETHANDLE: i32 = 0xbci32; /* Integer(188, Yes, Unknown) */ /* winuser.h:10672:9, winuser.h:10672:9, winuser.h:10672:9 */
pub const EM_GETHANDLE: i32 = 0xbdi32; /* Integer(189, Yes, Unknown) */ /* winuser.h:10673:9, winuser.h:10673:9, winuser.h:10673:9 */
pub const EM_GETTHUMB: i32 = 0xbei32; /* Integer(190, Yes, Unknown) */ /* winuser.h:10674:9, winuser.h:10674:9, winuser.h:10674:9 */
pub const EM_LINELENGTH: i32 = 0xc1i32; /* Integer(193, Yes, Unknown) */ /* winuser.h:10675:9, winuser.h:10675:9, winuser.h:10675:9 */
pub const EM_REPLACESEL: i32 = 0xc2i32; /* Integer(194, Yes, Unknown) */ /* winuser.h:10676:9, winuser.h:10676:9, winuser.h:10676:9 */
pub const EM_GETLINE: i32 = 0xc4i32; /* Integer(196, Yes, Unknown) */ /* winuser.h:10677:9, winuser.h:10677:9, winuser.h:10677:9 */
pub const EM_LIMITTEXT: i32 = 0xc5i32; /* Integer(197, Yes, Unknown) */ /* winuser.h:10678:9, winuser.h:10678:9, winuser.h:10678:9 */
pub const EM_CANUNDO: i32 = 0xc6i32; /* Integer(198, Yes, Unknown) */ /* winuser.h:10679:9, winuser.h:10679:9, winuser.h:10679:9 */
pub const EM_UNDO: i32 = 0xc7i32; /* Integer(199, Yes, Unknown) */ /* winuser.h:10680:9, winuser.h:10680:9, winuser.h:10680:9 */
pub const EM_FMTLINES: i32 = 0xc8i32; /* Integer(200, Yes, Unknown) */ /* winuser.h:10681:9, winuser.h:10681:9, winuser.h:10681:9 */
pub const EM_LINEFROMCHAR: i32 = 0xc9i32; /* Integer(201, Yes, Unknown) */ /* winuser.h:10682:9, winuser.h:10682:9, winuser.h:10682:9 */
pub const EM_SETTABSTOPS: i32 = 0xcbi32; /* Integer(203, Yes, Unknown) */ /* winuser.h:10683:9, winuser.h:10683:9, winuser.h:10683:9 */
pub const EM_SETPASSWORDCHAR: i32 = 0xcci32; /* Integer(204, Yes, Unknown) */ /* winuser.h:10684:9, winuser.h:10684:9, winuser.h:10684:9 */
pub const EM_EMPTYUNDOBUFFER: i32 = 0xcdi32; /* Integer(205, Yes, Unknown) */ /* winuser.h:10685:9, winuser.h:10685:9, winuser.h:10685:9 */
pub const EM_GETFIRSTVISIBLELINE: i32 = 0xcei32; /* Integer(206, Yes, Unknown) */ /* winuser.h:10686:9, winuser.h:10686:9, winuser.h:10686:9 */
pub const EM_SETREADONLY: i32 = 0xcfi32; /* Integer(207, Yes, Unknown) */ /* winuser.h:10687:9, winuser.h:10687:9, winuser.h:10687:9 */
pub const EM_SETWORDBREAKPROC: i32 = 0xd0i32; /* Integer(208, Yes, Unknown) */ /* winuser.h:10688:9, winuser.h:10688:9, winuser.h:10688:9 */
pub const EM_GETWORDBREAKPROC: i32 = 0xd1i32; /* Integer(209, Yes, Unknown) */ /* winuser.h:10689:9, winuser.h:10689:9, winuser.h:10689:9 */
pub const EM_GETPASSWORDCHAR: i32 = 0xd2i32; /* Integer(210, Yes, Unknown) */ /* winuser.h:10690:9, winuser.h:10690:9, winuser.h:10690:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const EM_SETMARGINS: i32 = 0xd3i32; /* Integer(211, Yes, Unknown) */ /* winuser.h:10692:9, winuser.h:10692:9, winuser.h:10692:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const EM_GETMARGINS: i32 = 0xd4i32; /* Integer(212, Yes, Unknown) */ /* winuser.h:10693:9, winuser.h:10693:9, winuser.h:10693:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::EM_LIMITTEXT as EM_SETLIMITTEXT; /* winuser.h:10694:9, winuser.h:10694:9, winuser.h:10694:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const EM_GETLIMITTEXT: i32 = 0xd5i32; /* Integer(213, Yes, Unknown) */ /* winuser.h:10695:9, winuser.h:10695:9, winuser.h:10695:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const EM_POSFROMCHAR: i32 = 0xd6i32; /* Integer(214, Yes, Unknown) */ /* winuser.h:10696:9, winuser.h:10696:9, winuser.h:10696:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const EM_CHARFROMPOS: i32 = 0xd7i32; /* Integer(215, Yes, Unknown) */ /* winuser.h:10697:9, winuser.h:10697:9, winuser.h:10697:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EM_SETIMESTATUS: i32 = 0xd8i32; /* Integer(216, Yes, Unknown) */ /* winuser.h:10701:9, winuser.h:10701:9, winuser.h:10701:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EM_GETIMESTATUS: i32 = 0xd9i32; /* Integer(217, Yes, Unknown) */ /* winuser.h:10702:9, winuser.h:10702:9, winuser.h:10702:9 */
pub const WB_LEFT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:10711:9, winuser.h:10711:9, winuser.h:10711:9 */
pub const WB_RIGHT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:10712:9, winuser.h:10712:9, winuser.h:10712:9 */
pub const WB_ISDELIMITER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:10713:9, winuser.h:10713:9, winuser.h:10713:9 */
pub const BS_PUSHBUTTON: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:10719:9, winuser.h:10719:9, winuser.h:10719:9 */
pub const BS_DEFPUSHBUTTON: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:10720:9, winuser.h:10720:9, winuser.h:10720:9 */
pub const BS_CHECKBOX: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:10721:9, winuser.h:10721:9, winuser.h:10721:9 */
pub const BS_AUTOCHECKBOX: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* winuser.h:10722:9, winuser.h:10722:9, winuser.h:10722:9 */
pub const BS_RADIOBUTTON: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:10723:9, winuser.h:10723:9, winuser.h:10723:9 */
pub const BS_3STATE: i64 = 0x5i64; /* Integer(5, Yes, Long) */ /* winuser.h:10724:9, winuser.h:10724:9, winuser.h:10724:9 */
pub const BS_AUTO3STATE: i64 = 0x6i64; /* Integer(6, Yes, Long) */ /* winuser.h:10725:9, winuser.h:10725:9, winuser.h:10725:9 */
pub const BS_GROUPBOX: i64 = 0x7i64; /* Integer(7, Yes, Long) */ /* winuser.h:10726:9, winuser.h:10726:9, winuser.h:10726:9 */
pub const BS_USERBUTTON: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winuser.h:10727:9, winuser.h:10727:9, winuser.h:10727:9 */
pub const BS_AUTORADIOBUTTON: i64 = 0x9i64; /* Integer(9, Yes, Long) */ /* winuser.h:10728:9, winuser.h:10728:9, winuser.h:10728:9 */
pub const BS_PUSHBOX: i64 = 0xai64; /* Integer(10, Yes, Long) */ /* winuser.h:10729:9, winuser.h:10729:9, winuser.h:10729:9 */
pub const BS_OWNERDRAW: i64 = 0xbi64; /* Integer(11, Yes, Long) */ /* winuser.h:10730:9, winuser.h:10730:9, winuser.h:10730:9 */
pub const BS_TYPEMASK: i64 = 0xfi64; /* Integer(15, Yes, Long) */ /* winuser.h:10731:9, winuser.h:10731:9, winuser.h:10731:9 */
pub const BS_LEFTTEXT: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* winuser.h:10732:9, winuser.h:10732:9, winuser.h:10732:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BS_TEXT: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:10734:9, winuser.h:10734:9, winuser.h:10734:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BS_ICON: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* winuser.h:10735:9, winuser.h:10735:9, winuser.h:10735:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BS_BITMAP: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* winuser.h:10736:9, winuser.h:10736:9, winuser.h:10736:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BS_LEFT: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* winuser.h:10737:9, winuser.h:10737:9, winuser.h:10737:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BS_RIGHT: i64 = 0x200i64; /* Integer(512, Yes, Long) */ /* winuser.h:10738:9, winuser.h:10738:9, winuser.h:10738:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BS_CENTER: i64 = 0x300i64; /* Integer(768, Yes, Long) */ /* winuser.h:10739:9, winuser.h:10739:9, winuser.h:10739:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BS_TOP: i64 = 0x400i64; /* Integer(1024, Yes, Long) */ /* winuser.h:10740:9, winuser.h:10740:9, winuser.h:10740:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BS_BOTTOM: i64 = 0x800i64; /* Integer(2048, Yes, Long) */ /* winuser.h:10741:9, winuser.h:10741:9, winuser.h:10741:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BS_VCENTER: i64 = 0xc00i64; /* Integer(3072, Yes, Long) */ /* winuser.h:10742:9, winuser.h:10742:9, winuser.h:10742:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BS_PUSHLIKE: i64 = 0x1000i64; /* Integer(4096, Yes, Long) */ /* winuser.h:10743:9, winuser.h:10743:9, winuser.h:10743:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BS_MULTILINE: i64 = 0x2000i64; /* Integer(8192, Yes, Long) */ /* winuser.h:10744:9, winuser.h:10744:9, winuser.h:10744:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BS_NOTIFY: i64 = 0x4000i64; /* Integer(16384, Yes, Long) */ /* winuser.h:10745:9, winuser.h:10745:9, winuser.h:10745:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BS_FLAT: i64 = 0x8000i64; /* Integer(32768, Yes, Long) */ /* winuser.h:10746:9, winuser.h:10746:9, winuser.h:10746:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::BS_LEFTTEXT as BS_RIGHTBUTTON; /* winuser.h:10747:9, winuser.h:10747:9, winuser.h:10747:9 */
pub const BN_CLICKED: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:10753:9, winuser.h:10753:9, winuser.h:10753:9 */
pub const BN_PAINT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:10754:9, winuser.h:10754:9, winuser.h:10754:9 */
pub const BN_HILITE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:10755:9, winuser.h:10755:9, winuser.h:10755:9 */
pub const BN_UNHILITE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:10756:9, winuser.h:10756:9, winuser.h:10756:9 */
pub const BN_DISABLE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:10757:9, winuser.h:10757:9, winuser.h:10757:9 */
pub const BN_DOUBLECLICKED: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:10758:9, winuser.h:10758:9, winuser.h:10758:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::BN_HILITE as BN_PUSHED; /* winuser.h:10760:9, winuser.h:10760:9, winuser.h:10760:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::BN_UNHILITE as BN_UNPUSHED; /* winuser.h:10761:9, winuser.h:10761:9, winuser.h:10761:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::BN_DOUBLECLICKED as BN_DBLCLK; /* winuser.h:10762:9, winuser.h:10762:9, winuser.h:10762:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BN_SETFOCUS: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:10763:9, winuser.h:10763:9, winuser.h:10763:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BN_KILLFOCUS: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:10764:9, winuser.h:10764:9, winuser.h:10764:9 */
pub const BM_GETCHECK: i32 = 0xf0i32; /* Integer(240, Yes, Unknown) */ /* winuser.h:10770:9, winuser.h:10770:9, winuser.h:10770:9 */
pub const BM_SETCHECK: i32 = 0xf1i32; /* Integer(241, Yes, Unknown) */ /* winuser.h:10771:9, winuser.h:10771:9, winuser.h:10771:9 */
pub const BM_GETSTATE: i32 = 0xf2i32; /* Integer(242, Yes, Unknown) */ /* winuser.h:10772:9, winuser.h:10772:9, winuser.h:10772:9 */
pub const BM_SETSTATE: i32 = 0xf3i32; /* Integer(243, Yes, Unknown) */ /* winuser.h:10773:9, winuser.h:10773:9, winuser.h:10773:9 */
pub const BM_SETSTYLE: i32 = 0xf4i32; /* Integer(244, Yes, Unknown) */ /* winuser.h:10774:9, winuser.h:10774:9, winuser.h:10774:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BM_CLICK: i32 = 0xf5i32; /* Integer(245, Yes, Unknown) */ /* winuser.h:10776:9, winuser.h:10776:9, winuser.h:10776:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BM_GETIMAGE: i32 = 0xf6i32; /* Integer(246, Yes, Unknown) */ /* winuser.h:10777:9, winuser.h:10777:9, winuser.h:10777:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BM_SETIMAGE: i32 = 0xf7i32; /* Integer(247, Yes, Unknown) */ /* winuser.h:10778:9, winuser.h:10778:9, winuser.h:10778:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const BM_SETDONTCLICK: i32 = 0xf8i32; /* Integer(248, Yes, Unknown) */ /* winuser.h:10781:9, winuser.h:10781:9, winuser.h:10781:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BST_UNCHECKED: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:10785:9, winuser.h:10785:9, winuser.h:10785:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BST_CHECKED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:10786:9, winuser.h:10786:9, winuser.h:10786:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BST_INDETERMINATE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:10787:9, winuser.h:10787:9, winuser.h:10787:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BST_PUSHED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:10788:9, winuser.h:10788:9, winuser.h:10788:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BST_FOCUS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:10789:9, winuser.h:10789:9, winuser.h:10789:9 */
pub const SS_LEFT: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:10795:9, winuser.h:10795:9, winuser.h:10795:9 */
pub const SS_CENTER: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:10796:9, winuser.h:10796:9, winuser.h:10796:9 */
pub const SS_RIGHT: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:10797:9, winuser.h:10797:9, winuser.h:10797:9 */
pub const SS_ICON: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* winuser.h:10798:9, winuser.h:10798:9, winuser.h:10798:9 */
pub const SS_BLACKRECT: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:10799:9, winuser.h:10799:9, winuser.h:10799:9 */
pub const SS_GRAYRECT: i64 = 0x5i64; /* Integer(5, Yes, Long) */ /* winuser.h:10800:9, winuser.h:10800:9, winuser.h:10800:9 */
pub const SS_WHITERECT: i64 = 0x6i64; /* Integer(6, Yes, Long) */ /* winuser.h:10801:9, winuser.h:10801:9, winuser.h:10801:9 */
pub const SS_BLACKFRAME: i64 = 0x7i64; /* Integer(7, Yes, Long) */ /* winuser.h:10802:9, winuser.h:10802:9, winuser.h:10802:9 */
pub const SS_GRAYFRAME: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winuser.h:10803:9, winuser.h:10803:9, winuser.h:10803:9 */
pub const SS_WHITEFRAME: i64 = 0x9i64; /* Integer(9, Yes, Long) */ /* winuser.h:10804:9, winuser.h:10804:9, winuser.h:10804:9 */
pub const SS_USERITEM: i64 = 0xai64; /* Integer(10, Yes, Long) */ /* winuser.h:10805:9, winuser.h:10805:9, winuser.h:10805:9 */
pub const SS_SIMPLE: i64 = 0xbi64; /* Integer(11, Yes, Long) */ /* winuser.h:10806:9, winuser.h:10806:9, winuser.h:10806:9 */
pub const SS_LEFTNOWORDWRAP: i64 = 0xci64; /* Integer(12, Yes, Long) */ /* winuser.h:10807:9, winuser.h:10807:9, winuser.h:10807:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_OWNERDRAW: i64 = 0xdi64; /* Integer(13, Yes, Long) */ /* winuser.h:10809:9, winuser.h:10809:9, winuser.h:10809:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_BITMAP: i64 = 0xei64; /* Integer(14, Yes, Long) */ /* winuser.h:10810:9, winuser.h:10810:9, winuser.h:10810:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_ENHMETAFILE: i64 = 0xfi64; /* Integer(15, Yes, Long) */ /* winuser.h:10811:9, winuser.h:10811:9, winuser.h:10811:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_ETCHEDHORZ: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* winuser.h:10812:9, winuser.h:10812:9, winuser.h:10812:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_ETCHEDVERT: i64 = 0x11i64; /* Integer(17, Yes, Long) */ /* winuser.h:10813:9, winuser.h:10813:9, winuser.h:10813:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_ETCHEDFRAME: i64 = 0x12i64; /* Integer(18, Yes, Long) */ /* winuser.h:10814:9, winuser.h:10814:9, winuser.h:10814:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_TYPEMASK: i64 = 0x1fi64; /* Integer(31, Yes, Long) */ /* winuser.h:10815:9, winuser.h:10815:9, winuser.h:10815:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SS_REALSIZECONTROL: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* winuser.h:10818:9, winuser.h:10818:9, winuser.h:10818:9 */
pub const SS_NOPREFIX: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* winuser.h:10820:9, winuser.h:10820:9, winuser.h:10820:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_NOTIFY: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* winuser.h:10822:9, winuser.h:10822:9, winuser.h:10822:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_CENTERIMAGE: i64 = 0x200i64; /* Integer(512, Yes, Long) */ /* winuser.h:10823:9, winuser.h:10823:9, winuser.h:10823:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_RIGHTJUST: i64 = 0x400i64; /* Integer(1024, Yes, Long) */ /* winuser.h:10824:9, winuser.h:10824:9, winuser.h:10824:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_REALSIZEIMAGE: i64 = 0x800i64; /* Integer(2048, Yes, Long) */ /* winuser.h:10825:9, winuser.h:10825:9, winuser.h:10825:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_SUNKEN: i64 = 0x1000i64; /* Integer(4096, Yes, Long) */ /* winuser.h:10826:9, winuser.h:10826:9, winuser.h:10826:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_EDITCONTROL: i64 = 0x2000i64; /* Integer(8192, Yes, Long) */ /* winuser.h:10827:9, winuser.h:10827:9, winuser.h:10827:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_ENDELLIPSIS: i64 = 0x4000i64; /* Integer(16384, Yes, Long) */ /* winuser.h:10828:9, winuser.h:10828:9, winuser.h:10828:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_PATHELLIPSIS: i64 = 0x8000i64; /* Integer(32768, Yes, Long) */ /* winuser.h:10829:9, winuser.h:10829:9, winuser.h:10829:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_WORDELLIPSIS: i64 = 0xc000i64; /* Integer(49152, Yes, Long) */ /* winuser.h:10830:9, winuser.h:10830:9, winuser.h:10830:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SS_ELLIPSISMASK: i64 = 0xc000i64; /* Integer(49152, Yes, Long) */ /* winuser.h:10831:9, winuser.h:10831:9, winuser.h:10831:9 */
pub const STM_SETICON: i32 = 0x170i32; /* Integer(368, Yes, Unknown) */ /* winuser.h:10840:9, winuser.h:10840:9, winuser.h:10840:9 */
pub const STM_GETICON: i32 = 0x171i32; /* Integer(369, Yes, Unknown) */ /* winuser.h:10841:9, winuser.h:10841:9, winuser.h:10841:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const STM_SETIMAGE: i32 = 0x172i32; /* Integer(370, Yes, Unknown) */ /* winuser.h:10843:9, winuser.h:10843:9, winuser.h:10843:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const STM_GETIMAGE: i32 = 0x173i32; /* Integer(371, Yes, Unknown) */ /* winuser.h:10844:9, winuser.h:10844:9, winuser.h:10844:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const STN_CLICKED: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:10845:9, winuser.h:10845:9, winuser.h:10845:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const STN_DBLCLK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:10846:9, winuser.h:10846:9, winuser.h:10846:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const STN_ENABLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:10847:9, winuser.h:10847:9, winuser.h:10847:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const STN_DISABLE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:10848:9, winuser.h:10848:9, winuser.h:10848:9 */
pub const STM_MSGMAX: i32 = 0x174i32; /* Integer(372, Yes, Unknown) */ /* winuser.h:10850:9, winuser.h:10850:9, winuser.h:10850:9 */
pub const DWL_MSGRESULT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:10861:9, winuser.h:10861:9, winuser.h:10861:9 */
pub const DWL_DLGPROC: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:10862:9, winuser.h:10862:9, winuser.h:10862:9 */
pub const DWL_USER: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:10863:9, winuser.h:10863:9, winuser.h:10863:9 */
pub const DWLP_MSGRESULT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:10873:9, winuser.h:10873:9, winuser.h:10873:9 */
/* // #define DWLP_DLGPROC Binary(Add, Ident("DWLP_MSGRESULT"), Call { subject: Ident("sizeof"), args: [Ident("LRESULT")] }) */ /* winuser.h:10874:9, winuser.h:10874:9, winuser.h:10874:9 */
/* // #define DWLP_USER Binary(Add, Ident("DWLP_DLGPROC"), Call { subject: Ident("sizeof"), args: [Ident("DLGPROC")] }) */ /* winuser.h:10875:9, winuser.h:10875:9, winuser.h:10875:9 */
pub const DDL_READWRITE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:10943:9, winuser.h:10943:9, winuser.h:10943:9 */
pub const DDL_READONLY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:10944:9, winuser.h:10944:9, winuser.h:10944:9 */
pub const DDL_HIDDEN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:10945:9, winuser.h:10945:9, winuser.h:10945:9 */
pub const DDL_SYSTEM: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:10946:9, winuser.h:10946:9, winuser.h:10946:9 */
pub const DDL_DIRECTORY: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:10947:9, winuser.h:10947:9, winuser.h:10947:9 */
pub const DDL_ARCHIVE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:10948:9, winuser.h:10948:9, winuser.h:10948:9 */
pub const DDL_POSTMSGS: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:10950:9, winuser.h:10950:9, winuser.h:10950:9 */
pub const DDL_DRIVES: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winuser.h:10951:9, winuser.h:10951:9, winuser.h:10951:9 */
pub const DDL_EXCLUSIVE: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:10952:9, winuser.h:10952:9, winuser.h:10952:9 */
pub const DS_ABSALIGN: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:11033:9, winuser.h:11033:9, winuser.h:11033:9 */
pub const DS_SYSMODAL: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:11034:9, winuser.h:11034:9, winuser.h:11034:9 */
pub const DS_LOCALEDIT: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* winuser.h:11035:9, winuser.h:11035:9, winuser.h:11035:9 */
pub const DS_SETFONT: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* winuser.h:11036:9, winuser.h:11036:9, winuser.h:11036:9 */
pub const DS_MODALFRAME: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* winuser.h:11037:9, winuser.h:11037:9, winuser.h:11037:9 */
pub const DS_NOIDLEMSG: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* winuser.h:11038:9, winuser.h:11038:9, winuser.h:11038:9 */
pub const DS_SETFOREGROUND: i64 = 0x200i64; /* Integer(512, Yes, Long) */ /* winuser.h:11039:9, winuser.h:11039:9, winuser.h:11039:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DS_3DLOOK: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:11043:9, winuser.h:11043:9, winuser.h:11043:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DS_FIXEDSYS: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winuser.h:11044:9, winuser.h:11044:9, winuser.h:11044:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DS_NOFAILCREATE: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* winuser.h:11045:9, winuser.h:11045:9, winuser.h:11045:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DS_CONTROL: i64 = 0x400i64; /* Integer(1024, Yes, Long) */ /* winuser.h:11046:9, winuser.h:11046:9, winuser.h:11046:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DS_CENTER: i64 = 0x800i64; /* Integer(2048, Yes, Long) */ /* winuser.h:11047:9, winuser.h:11047:9, winuser.h:11047:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DS_CENTERMOUSE: i64 = 0x1000i64; /* Integer(4096, Yes, Long) */ /* winuser.h:11048:9, winuser.h:11048:9, winuser.h:11048:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DS_CONTEXTHELP: i64 = 0x2000i64; /* Integer(8192, Yes, Long) */ /* winuser.h:11049:9, winuser.h:11049:9, winuser.h:11049:9 */
pub const DC_HASDEFID: i32 = 0x534bi32; /* Integer(21323, Yes, Unknown) */ /* winuser.h:11068:9, winuser.h:11068:9, winuser.h:11068:9 */
pub const DLGC_WANTARROWS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:11073:9, winuser.h:11073:9, winuser.h:11073:9 */
pub const DLGC_WANTTAB: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:11074:9, winuser.h:11074:9, winuser.h:11074:9 */
pub const DLGC_WANTALLKEYS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:11075:9, winuser.h:11075:9, winuser.h:11075:9 */
pub const DLGC_WANTMESSAGE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:11076:9, winuser.h:11076:9, winuser.h:11076:9 */
pub const DLGC_HASSETSEL: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:11077:9, winuser.h:11077:9, winuser.h:11077:9 */
pub const DLGC_DEFPUSHBUTTON: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:11078:9, winuser.h:11078:9, winuser.h:11078:9 */
pub const DLGC_UNDEFPUSHBUTTON: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:11079:9, winuser.h:11079:9, winuser.h:11079:9 */
pub const DLGC_RADIOBUTTON: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:11080:9, winuser.h:11080:9, winuser.h:11080:9 */
pub const DLGC_WANTCHARS: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:11081:9, winuser.h:11081:9, winuser.h:11081:9 */
pub const DLGC_STATIC: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:11082:9, winuser.h:11082:9, winuser.h:11082:9 */
pub const DLGC_BUTTON: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:11083:9, winuser.h:11083:9, winuser.h:11083:9 */
pub const LB_CTLCODE: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:11085:9, winuser.h:11085:9, winuser.h:11085:9 */
pub const LB_OKAY: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:11090:9, winuser.h:11090:9, winuser.h:11090:9 */
pub const LBN_SELCHANGE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:11106:9, winuser.h:11106:9, winuser.h:11106:9 */
pub const LBN_DBLCLK: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:11107:9, winuser.h:11107:9, winuser.h:11107:9 */
pub const LBN_SELCANCEL: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:11108:9, winuser.h:11108:9, winuser.h:11108:9 */
pub const LBN_SETFOCUS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:11109:9, winuser.h:11109:9, winuser.h:11109:9 */
pub const LBN_KILLFOCUS: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:11110:9, winuser.h:11110:9, winuser.h:11110:9 */
pub const LB_ADDSTRING: i32 = 0x180i32; /* Integer(384, Yes, Unknown) */ /* winuser.h:11119:9, winuser.h:11119:9, winuser.h:11119:9 */
pub const LB_INSERTSTRING: i32 = 0x181i32; /* Integer(385, Yes, Unknown) */ /* winuser.h:11120:9, winuser.h:11120:9, winuser.h:11120:9 */
pub const LB_DELETESTRING: i32 = 0x182i32; /* Integer(386, Yes, Unknown) */ /* winuser.h:11121:9, winuser.h:11121:9, winuser.h:11121:9 */
pub const LB_SELITEMRANGEEX: i32 = 0x183i32; /* Integer(387, Yes, Unknown) */ /* winuser.h:11122:9, winuser.h:11122:9, winuser.h:11122:9 */
pub const LB_RESETCONTENT: i32 = 0x184i32; /* Integer(388, Yes, Unknown) */ /* winuser.h:11123:9, winuser.h:11123:9, winuser.h:11123:9 */
pub const LB_SETSEL: i32 = 0x185i32; /* Integer(389, Yes, Unknown) */ /* winuser.h:11124:9, winuser.h:11124:9, winuser.h:11124:9 */
pub const LB_SETCURSEL: i32 = 0x186i32; /* Integer(390, Yes, Unknown) */ /* winuser.h:11125:9, winuser.h:11125:9, winuser.h:11125:9 */
pub const LB_GETSEL: i32 = 0x187i32; /* Integer(391, Yes, Unknown) */ /* winuser.h:11126:9, winuser.h:11126:9, winuser.h:11126:9 */
pub const LB_GETCURSEL: i32 = 0x188i32; /* Integer(392, Yes, Unknown) */ /* winuser.h:11127:9, winuser.h:11127:9, winuser.h:11127:9 */
pub const LB_GETTEXT: i32 = 0x189i32; /* Integer(393, Yes, Unknown) */ /* winuser.h:11128:9, winuser.h:11128:9, winuser.h:11128:9 */
pub const LB_GETTEXTLEN: i32 = 0x18ai32; /* Integer(394, Yes, Unknown) */ /* winuser.h:11129:9, winuser.h:11129:9, winuser.h:11129:9 */
pub const LB_GETCOUNT: i32 = 0x18bi32; /* Integer(395, Yes, Unknown) */ /* winuser.h:11130:9, winuser.h:11130:9, winuser.h:11130:9 */
pub const LB_SELECTSTRING: i32 = 0x18ci32; /* Integer(396, Yes, Unknown) */ /* winuser.h:11131:9, winuser.h:11131:9, winuser.h:11131:9 */
pub const LB_DIR: i32 = 0x18di32; /* Integer(397, Yes, Unknown) */ /* winuser.h:11132:9, winuser.h:11132:9, winuser.h:11132:9 */
pub const LB_GETTOPINDEX: i32 = 0x18ei32; /* Integer(398, Yes, Unknown) */ /* winuser.h:11133:9, winuser.h:11133:9, winuser.h:11133:9 */
pub const LB_FINDSTRING: i32 = 0x18fi32; /* Integer(399, Yes, Unknown) */ /* winuser.h:11134:9, winuser.h:11134:9, winuser.h:11134:9 */
pub const LB_GETSELCOUNT: i32 = 0x190i32; /* Integer(400, Yes, Unknown) */ /* winuser.h:11135:9, winuser.h:11135:9, winuser.h:11135:9 */
pub const LB_GETSELITEMS: i32 = 0x191i32; /* Integer(401, Yes, Unknown) */ /* winuser.h:11136:9, winuser.h:11136:9, winuser.h:11136:9 */
pub const LB_SETTABSTOPS: i32 = 0x192i32; /* Integer(402, Yes, Unknown) */ /* winuser.h:11137:9, winuser.h:11137:9, winuser.h:11137:9 */
pub const LB_GETHORIZONTALEXTENT: i32 = 0x193i32; /* Integer(403, Yes, Unknown) */ /* winuser.h:11138:9, winuser.h:11138:9, winuser.h:11138:9 */
pub const LB_SETHORIZONTALEXTENT: i32 = 0x194i32; /* Integer(404, Yes, Unknown) */ /* winuser.h:11139:9, winuser.h:11139:9, winuser.h:11139:9 */
pub const LB_SETCOLUMNWIDTH: i32 = 0x195i32; /* Integer(405, Yes, Unknown) */ /* winuser.h:11140:9, winuser.h:11140:9, winuser.h:11140:9 */
pub const LB_ADDFILE: i32 = 0x196i32; /* Integer(406, Yes, Unknown) */ /* winuser.h:11141:9, winuser.h:11141:9, winuser.h:11141:9 */
pub const LB_SETTOPINDEX: i32 = 0x197i32; /* Integer(407, Yes, Unknown) */ /* winuser.h:11142:9, winuser.h:11142:9, winuser.h:11142:9 */
pub const LB_GETITEMRECT: i32 = 0x198i32; /* Integer(408, Yes, Unknown) */ /* winuser.h:11143:9, winuser.h:11143:9, winuser.h:11143:9 */
pub const LB_GETITEMDATA: i32 = 0x199i32; /* Integer(409, Yes, Unknown) */ /* winuser.h:11144:9, winuser.h:11144:9, winuser.h:11144:9 */
pub const LB_SETITEMDATA: i32 = 0x19ai32; /* Integer(410, Yes, Unknown) */ /* winuser.h:11145:9, winuser.h:11145:9, winuser.h:11145:9 */
pub const LB_SELITEMRANGE: i32 = 0x19bi32; /* Integer(411, Yes, Unknown) */ /* winuser.h:11146:9, winuser.h:11146:9, winuser.h:11146:9 */
pub const LB_SETANCHORINDEX: i32 = 0x19ci32; /* Integer(412, Yes, Unknown) */ /* winuser.h:11147:9, winuser.h:11147:9, winuser.h:11147:9 */
pub const LB_GETANCHORINDEX: i32 = 0x19di32; /* Integer(413, Yes, Unknown) */ /* winuser.h:11148:9, winuser.h:11148:9, winuser.h:11148:9 */
pub const LB_SETCARETINDEX: i32 = 0x19ei32; /* Integer(414, Yes, Unknown) */ /* winuser.h:11149:9, winuser.h:11149:9, winuser.h:11149:9 */
pub const LB_GETCARETINDEX: i32 = 0x19fi32; /* Integer(415, Yes, Unknown) */ /* winuser.h:11150:9, winuser.h:11150:9, winuser.h:11150:9 */
pub const LB_SETITEMHEIGHT: i32 = 0x1a0i32; /* Integer(416, Yes, Unknown) */ /* winuser.h:11151:9, winuser.h:11151:9, winuser.h:11151:9 */
pub const LB_GETITEMHEIGHT: i32 = 0x1a1i32; /* Integer(417, Yes, Unknown) */ /* winuser.h:11152:9, winuser.h:11152:9, winuser.h:11152:9 */
pub const LB_FINDSTRINGEXACT: i32 = 0x1a2i32; /* Integer(418, Yes, Unknown) */ /* winuser.h:11153:9, winuser.h:11153:9, winuser.h:11153:9 */
pub const LB_SETLOCALE: i32 = 0x1a5i32; /* Integer(421, Yes, Unknown) */ /* winuser.h:11154:9, winuser.h:11154:9, winuser.h:11154:9 */
pub const LB_GETLOCALE: i32 = 0x1a6i32; /* Integer(422, Yes, Unknown) */ /* winuser.h:11155:9, winuser.h:11155:9, winuser.h:11155:9 */
pub const LB_SETCOUNT: i32 = 0x1a7i32; /* Integer(423, Yes, Unknown) */ /* winuser.h:11156:9, winuser.h:11156:9, winuser.h:11156:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LB_INITSTORAGE: i32 = 0x1a8i32; /* Integer(424, Yes, Unknown) */ /* winuser.h:11158:9, winuser.h:11158:9, winuser.h:11158:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LB_ITEMFROMPOINT: i32 = 0x1a9i32; /* Integer(425, Yes, Unknown) */ /* winuser.h:11159:9, winuser.h:11159:9, winuser.h:11159:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const LB_GETLISTBOXINFO: i32 = 0x1b2i32; /* Integer(434, Yes, Unknown) */ /* winuser.h:11167:9, winuser.h:11167:9, winuser.h:11167:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const LB_MSGMAX: i32 = 0x1b3i32; /* Integer(435, Yes, Unknown) */ /* winuser.h:11171:9, winuser.h:11171:9, winuser.h:11171:9 */
pub const LBS_NOTIFY: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:11188:9, winuser.h:11188:9, winuser.h:11188:9 */
pub const LBS_SORT: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:11189:9, winuser.h:11189:9, winuser.h:11189:9 */
pub const LBS_NOREDRAW: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:11190:9, winuser.h:11190:9, winuser.h:11190:9 */
pub const LBS_MULTIPLESEL: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winuser.h:11191:9, winuser.h:11191:9, winuser.h:11191:9 */
pub const LBS_OWNERDRAWFIXED: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* winuser.h:11192:9, winuser.h:11192:9, winuser.h:11192:9 */
pub const LBS_OWNERDRAWVARIABLE: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* winuser.h:11193:9, winuser.h:11193:9, winuser.h:11193:9 */
pub const LBS_HASSTRINGS: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* winuser.h:11194:9, winuser.h:11194:9, winuser.h:11194:9 */
pub const LBS_USETABSTOPS: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* winuser.h:11195:9, winuser.h:11195:9, winuser.h:11195:9 */
pub const LBS_NOINTEGRALHEIGHT: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* winuser.h:11196:9, winuser.h:11196:9, winuser.h:11196:9 */
pub const LBS_MULTICOLUMN: i64 = 0x200i64; /* Integer(512, Yes, Long) */ /* winuser.h:11197:9, winuser.h:11197:9, winuser.h:11197:9 */
pub const LBS_WANTKEYBOARDINPUT: i64 = 0x400i64; /* Integer(1024, Yes, Long) */ /* winuser.h:11198:9, winuser.h:11198:9, winuser.h:11198:9 */
pub const LBS_EXTENDEDSEL: i64 = 0x800i64; /* Integer(2048, Yes, Long) */ /* winuser.h:11199:9, winuser.h:11199:9, winuser.h:11199:9 */
pub const LBS_DISABLENOSCROLL: i64 = 0x1000i64; /* Integer(4096, Yes, Long) */ /* winuser.h:11200:9, winuser.h:11200:9, winuser.h:11200:9 */
pub const LBS_NODATA: i64 = 0x2000i64; /* Integer(8192, Yes, Long) */ /* winuser.h:11201:9, winuser.h:11201:9, winuser.h:11201:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const LBS_NOSEL: i64 = 0x4000i64; /* Integer(16384, Yes, Long) */ /* winuser.h:11203:9, winuser.h:11203:9, winuser.h:11203:9 */
pub const LBS_COMBOBOX: i64 = 0x8000i64; /* Integer(32768, Yes, Long) */ /* winuser.h:11205:9, winuser.h:11205:9, winuser.h:11205:9 */
pub const CB_OKAY: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:11216:9, winuser.h:11216:9, winuser.h:11216:9 */
pub const CBN_SELCHANGE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:11225:9, winuser.h:11225:9, winuser.h:11225:9 */
pub const CBN_DBLCLK: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:11226:9, winuser.h:11226:9, winuser.h:11226:9 */
pub const CBN_SETFOCUS: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:11227:9, winuser.h:11227:9, winuser.h:11227:9 */
pub const CBN_KILLFOCUS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:11228:9, winuser.h:11228:9, winuser.h:11228:9 */
pub const CBN_EDITCHANGE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:11229:9, winuser.h:11229:9, winuser.h:11229:9 */
pub const CBN_EDITUPDATE: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:11230:9, winuser.h:11230:9, winuser.h:11230:9 */
pub const CBN_DROPDOWN: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:11231:9, winuser.h:11231:9, winuser.h:11231:9 */
pub const CBN_CLOSEUP: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:11232:9, winuser.h:11232:9, winuser.h:11232:9 */
pub const CBN_SELENDOK: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:11233:9, winuser.h:11233:9, winuser.h:11233:9 */
pub const CBN_SELENDCANCEL: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:11234:9, winuser.h:11234:9, winuser.h:11234:9 */
pub const CBS_SIMPLE: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:11241:9, winuser.h:11241:9, winuser.h:11241:9 */
pub const CBS_DROPDOWN: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:11242:9, winuser.h:11242:9, winuser.h:11242:9 */
pub const CBS_DROPDOWNLIST: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* winuser.h:11243:9, winuser.h:11243:9, winuser.h:11243:9 */
pub const CBS_OWNERDRAWFIXED: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* winuser.h:11244:9, winuser.h:11244:9, winuser.h:11244:9 */
pub const CBS_OWNERDRAWVARIABLE: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* winuser.h:11245:9, winuser.h:11245:9, winuser.h:11245:9 */
pub const CBS_AUTOHSCROLL: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* winuser.h:11246:9, winuser.h:11246:9, winuser.h:11246:9 */
pub const CBS_OEMCONVERT: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* winuser.h:11247:9, winuser.h:11247:9, winuser.h:11247:9 */
pub const CBS_SORT: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* winuser.h:11248:9, winuser.h:11248:9, winuser.h:11248:9 */
pub const CBS_HASSTRINGS: i64 = 0x200i64; /* Integer(512, Yes, Long) */ /* winuser.h:11249:9, winuser.h:11249:9, winuser.h:11249:9 */
pub const CBS_NOINTEGRALHEIGHT: i64 = 0x400i64; /* Integer(1024, Yes, Long) */ /* winuser.h:11250:9, winuser.h:11250:9, winuser.h:11250:9 */
pub const CBS_DISABLENOSCROLL: i64 = 0x800i64; /* Integer(2048, Yes, Long) */ /* winuser.h:11251:9, winuser.h:11251:9, winuser.h:11251:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CBS_UPPERCASE: i64 = 0x2000i64; /* Integer(8192, Yes, Long) */ /* winuser.h:11253:9, winuser.h:11253:9, winuser.h:11253:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CBS_LOWERCASE: i64 = 0x4000i64; /* Integer(16384, Yes, Long) */ /* winuser.h:11254:9, winuser.h:11254:9, winuser.h:11254:9 */
pub const CB_GETEDITSEL: i32 = 0x140i32; /* Integer(320, Yes, Unknown) */ /* winuser.h:11264:9, winuser.h:11264:9, winuser.h:11264:9 */
pub const CB_LIMITTEXT: i32 = 0x141i32; /* Integer(321, Yes, Unknown) */ /* winuser.h:11265:9, winuser.h:11265:9, winuser.h:11265:9 */
pub const CB_SETEDITSEL: i32 = 0x142i32; /* Integer(322, Yes, Unknown) */ /* winuser.h:11266:9, winuser.h:11266:9, winuser.h:11266:9 */
pub const CB_ADDSTRING: i32 = 0x143i32; /* Integer(323, Yes, Unknown) */ /* winuser.h:11267:9, winuser.h:11267:9, winuser.h:11267:9 */
pub const CB_DELETESTRING: i32 = 0x144i32; /* Integer(324, Yes, Unknown) */ /* winuser.h:11268:9, winuser.h:11268:9, winuser.h:11268:9 */
pub const CB_DIR: i32 = 0x145i32; /* Integer(325, Yes, Unknown) */ /* winuser.h:11269:9, winuser.h:11269:9, winuser.h:11269:9 */
pub const CB_GETCOUNT: i32 = 0x146i32; /* Integer(326, Yes, Unknown) */ /* winuser.h:11270:9, winuser.h:11270:9, winuser.h:11270:9 */
pub const CB_GETCURSEL: i32 = 0x147i32; /* Integer(327, Yes, Unknown) */ /* winuser.h:11271:9, winuser.h:11271:9, winuser.h:11271:9 */
pub const CB_GETLBTEXT: i32 = 0x148i32; /* Integer(328, Yes, Unknown) */ /* winuser.h:11272:9, winuser.h:11272:9, winuser.h:11272:9 */
pub const CB_GETLBTEXTLEN: i32 = 0x149i32; /* Integer(329, Yes, Unknown) */ /* winuser.h:11273:9, winuser.h:11273:9, winuser.h:11273:9 */
pub const CB_INSERTSTRING: i32 = 0x14ai32; /* Integer(330, Yes, Unknown) */ /* winuser.h:11274:9, winuser.h:11274:9, winuser.h:11274:9 */
pub const CB_RESETCONTENT: i32 = 0x14bi32; /* Integer(331, Yes, Unknown) */ /* winuser.h:11275:9, winuser.h:11275:9, winuser.h:11275:9 */
pub const CB_FINDSTRING: i32 = 0x14ci32; /* Integer(332, Yes, Unknown) */ /* winuser.h:11276:9, winuser.h:11276:9, winuser.h:11276:9 */
pub const CB_SELECTSTRING: i32 = 0x14di32; /* Integer(333, Yes, Unknown) */ /* winuser.h:11277:9, winuser.h:11277:9, winuser.h:11277:9 */
pub const CB_SETCURSEL: i32 = 0x14ei32; /* Integer(334, Yes, Unknown) */ /* winuser.h:11278:9, winuser.h:11278:9, winuser.h:11278:9 */
pub const CB_SHOWDROPDOWN: i32 = 0x14fi32; /* Integer(335, Yes, Unknown) */ /* winuser.h:11279:9, winuser.h:11279:9, winuser.h:11279:9 */
pub const CB_GETITEMDATA: i32 = 0x150i32; /* Integer(336, Yes, Unknown) */ /* winuser.h:11280:9, winuser.h:11280:9, winuser.h:11280:9 */
pub const CB_SETITEMDATA: i32 = 0x151i32; /* Integer(337, Yes, Unknown) */ /* winuser.h:11281:9, winuser.h:11281:9, winuser.h:11281:9 */
pub const CB_GETDROPPEDCONTROLRECT: i32 = 0x152i32; /* Integer(338, Yes, Unknown) */ /* winuser.h:11282:9, winuser.h:11282:9, winuser.h:11282:9 */
pub const CB_SETITEMHEIGHT: i32 = 0x153i32; /* Integer(339, Yes, Unknown) */ /* winuser.h:11283:9, winuser.h:11283:9, winuser.h:11283:9 */
pub const CB_GETITEMHEIGHT: i32 = 0x154i32; /* Integer(340, Yes, Unknown) */ /* winuser.h:11284:9, winuser.h:11284:9, winuser.h:11284:9 */
pub const CB_SETEXTENDEDUI: i32 = 0x155i32; /* Integer(341, Yes, Unknown) */ /* winuser.h:11285:9, winuser.h:11285:9, winuser.h:11285:9 */
pub const CB_GETEXTENDEDUI: i32 = 0x156i32; /* Integer(342, Yes, Unknown) */ /* winuser.h:11286:9, winuser.h:11286:9, winuser.h:11286:9 */
pub const CB_GETDROPPEDSTATE: i32 = 0x157i32; /* Integer(343, Yes, Unknown) */ /* winuser.h:11287:9, winuser.h:11287:9, winuser.h:11287:9 */
pub const CB_FINDSTRINGEXACT: i32 = 0x158i32; /* Integer(344, Yes, Unknown) */ /* winuser.h:11288:9, winuser.h:11288:9, winuser.h:11288:9 */
pub const CB_SETLOCALE: i32 = 0x159i32; /* Integer(345, Yes, Unknown) */ /* winuser.h:11289:9, winuser.h:11289:9, winuser.h:11289:9 */
pub const CB_GETLOCALE: i32 = 0x15ai32; /* Integer(346, Yes, Unknown) */ /* winuser.h:11290:9, winuser.h:11290:9, winuser.h:11290:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CB_GETTOPINDEX: i32 = 0x15bi32; /* Integer(347, Yes, Unknown) */ /* winuser.h:11292:9, winuser.h:11292:9, winuser.h:11292:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CB_SETTOPINDEX: i32 = 0x15ci32; /* Integer(348, Yes, Unknown) */ /* winuser.h:11293:9, winuser.h:11293:9, winuser.h:11293:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CB_GETHORIZONTALEXTENT: i32 = 0x15di32; /* Integer(349, Yes, Unknown) */ /* winuser.h:11294:9, winuser.h:11294:9, winuser.h:11294:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CB_SETHORIZONTALEXTENT: i32 = 0x15ei32; /* Integer(350, Yes, Unknown) */ /* winuser.h:11295:9, winuser.h:11295:9, winuser.h:11295:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CB_GETDROPPEDWIDTH: i32 = 0x15fi32; /* Integer(351, Yes, Unknown) */ /* winuser.h:11296:9, winuser.h:11296:9, winuser.h:11296:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CB_SETDROPPEDWIDTH: i32 = 0x160i32; /* Integer(352, Yes, Unknown) */ /* winuser.h:11297:9, winuser.h:11297:9, winuser.h:11297:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CB_INITSTORAGE: i32 = 0x161i32; /* Integer(353, Yes, Unknown) */ /* winuser.h:11298:9, winuser.h:11298:9, winuser.h:11298:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const CB_GETCOMBOBOXINFO: i32 = 0x164i32; /* Integer(356, Yes, Unknown) */ /* winuser.h:11305:9, winuser.h:11305:9, winuser.h:11305:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const CB_MSGMAX: i32 = 0x165i32; /* Integer(357, Yes, Unknown) */ /* winuser.h:11309:9, winuser.h:11309:9, winuser.h:11309:9 */
pub const SBS_HORZ: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:11327:9, winuser.h:11327:9, winuser.h:11327:9 */
pub const SBS_VERT: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:11328:9, winuser.h:11328:9, winuser.h:11328:9 */
pub const SBS_TOPALIGN: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:11329:9, winuser.h:11329:9, winuser.h:11329:9 */
pub const SBS_LEFTALIGN: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:11330:9, winuser.h:11330:9, winuser.h:11330:9 */
pub const SBS_BOTTOMALIGN: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:11331:9, winuser.h:11331:9, winuser.h:11331:9 */
pub const SBS_RIGHTALIGN: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:11332:9, winuser.h:11332:9, winuser.h:11332:9 */
pub const SBS_SIZEBOXTOPLEFTALIGN: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:11333:9, winuser.h:11333:9, winuser.h:11333:9 */
pub const SBS_SIZEBOXBOTTOMRIGHTALIGN: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:11334:9, winuser.h:11334:9, winuser.h:11334:9 */
pub const SBS_SIZEBOX: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winuser.h:11335:9, winuser.h:11335:9, winuser.h:11335:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SBS_SIZEGRIP: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* winuser.h:11337:9, winuser.h:11337:9, winuser.h:11337:9 */
pub const SBM_SETPOS: i32 = 0xe0i32; /* Integer(224, Yes, Unknown) */ /* winuser.h:11347:9, winuser.h:11347:9, winuser.h:11347:9 */
pub const SBM_GETPOS: i32 = 0xe1i32; /* Integer(225, Yes, Unknown) */ /* winuser.h:11348:9, winuser.h:11348:9, winuser.h:11348:9 */
pub const SBM_SETRANGE: i32 = 0xe2i32; /* Integer(226, Yes, Unknown) */ /* winuser.h:11349:9, winuser.h:11349:9, winuser.h:11349:9 */
pub const SBM_SETRANGEREDRAW: i32 = 0xe6i32; /* Integer(230, Yes, Unknown) */ /* winuser.h:11350:9, winuser.h:11350:9, winuser.h:11350:9 */
pub const SBM_GETRANGE: i32 = 0xe3i32; /* Integer(227, Yes, Unknown) */ /* winuser.h:11351:9, winuser.h:11351:9, winuser.h:11351:9 */
pub const SBM_ENABLE_ARROWS: i32 = 0xe4i32; /* Integer(228, Yes, Unknown) */ /* winuser.h:11352:9, winuser.h:11352:9, winuser.h:11352:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SBM_SETSCROLLINFO: i32 = 0xe9i32; /* Integer(233, Yes, Unknown) */ /* winuser.h:11354:9, winuser.h:11354:9, winuser.h:11354:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SBM_GETSCROLLINFO: i32 = 0xeai32; /* Integer(234, Yes, Unknown) */ /* winuser.h:11355:9, winuser.h:11355:9, winuser.h:11355:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SBM_GETSCROLLBARINFO: i32 = 0xebi32; /* Integer(235, Yes, Unknown) */ /* winuser.h:11359:9, winuser.h:11359:9, winuser.h:11359:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SIF_RANGE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:11363:9, winuser.h:11363:9, winuser.h:11363:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SIF_PAGE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:11364:9, winuser.h:11364:9, winuser.h:11364:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SIF_POS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:11365:9, winuser.h:11365:9, winuser.h:11365:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SIF_DISABLENOSCROLL: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:11366:9, winuser.h:11366:9, winuser.h:11366:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SIF_TRACKPOS: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:11367:9, winuser.h:11367:9, winuser.h:11367:9 */
pub const MDIS_ALLCHILDSTYLES: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:11415:9, winuser.h:11415:9, winuser.h:11415:9 */
pub const MDITILE_VERTICAL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:11420:9, winuser.h:11420:9, winuser.h:11420:9 */
pub const MDITILE_HORIZONTAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:11421:9, winuser.h:11421:9, winuser.h:11421:9 */
pub const MDITILE_SKIPDISABLED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:11422:9, winuser.h:11422:9, winuser.h:11422:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const MDITILE_ZORDER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:11424:9, winuser.h:11424:9, winuser.h:11424:9 */
#[cfg(feature="winapi_desktop")] pub const HELP_CONTEXT: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:11669:9, winuser.h:11669:9, winuser.h:11669:9 */
#[cfg(feature="winapi_desktop")] pub const HELP_QUIT: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:11670:9, winuser.h:11670:9, winuser.h:11670:9 */
#[cfg(feature="winapi_desktop")] pub const HELP_INDEX: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* winuser.h:11671:9, winuser.h:11671:9, winuser.h:11671:9 */
#[cfg(feature="winapi_desktop")] pub const HELP_CONTENTS: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* winuser.h:11672:9, winuser.h:11672:9, winuser.h:11672:9 */
#[cfg(feature="winapi_desktop")] pub const HELP_HELPONHELP: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:11673:9, winuser.h:11673:9, winuser.h:11673:9 */
#[cfg(feature="winapi_desktop")] pub const HELP_SETINDEX: i64 = 0x5i64; /* Integer(5, Yes, Long) */ /* winuser.h:11674:9, winuser.h:11674:9, winuser.h:11674:9 */
#[cfg(feature="winapi_desktop")] pub const HELP_SETCONTENTS: i64 = 0x5i64; /* Integer(5, Yes, Long) */ /* winuser.h:11675:9, winuser.h:11675:9, winuser.h:11675:9 */
#[cfg(feature="winapi_desktop")] pub const HELP_CONTEXTPOPUP: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winuser.h:11676:9, winuser.h:11676:9, winuser.h:11676:9 */
#[cfg(feature="winapi_desktop")] pub const HELP_FORCEFILE: i64 = 0x9i64; /* Integer(9, Yes, Long) */ /* winuser.h:11677:9, winuser.h:11677:9, winuser.h:11677:9 */
#[cfg(feature="winapi_desktop")] pub const HELP_KEY: i64 = 0x101i64; /* Integer(257, Yes, Long) */ /* winuser.h:11678:9, winuser.h:11678:9, winuser.h:11678:9 */
#[cfg(feature="winapi_desktop")] pub const HELP_COMMAND: i64 = 0x102i64; /* Integer(258, Yes, Long) */ /* winuser.h:11679:9, winuser.h:11679:9, winuser.h:11679:9 */
#[cfg(feature="winapi_desktop")] pub const HELP_PARTIALKEY: i64 = 0x105i64; /* Integer(261, Yes, Long) */ /* winuser.h:11680:9, winuser.h:11680:9, winuser.h:11680:9 */
#[cfg(feature="winapi_desktop")] pub const HELP_MULTIKEY: i64 = 0x201i64; /* Integer(513, Yes, Long) */ /* winuser.h:11681:9, winuser.h:11681:9, winuser.h:11681:9 */
#[cfg(feature="winapi_desktop")] pub const HELP_SETWINPOS: i64 = 0x203i64; /* Integer(515, Yes, Long) */ /* winuser.h:11682:9, winuser.h:11682:9, winuser.h:11682:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const HELP_CONTEXTMENU: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:11684:9, winuser.h:11684:9, winuser.h:11684:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const HELP_FINDER: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:11685:9, winuser.h:11685:9, winuser.h:11685:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const HELP_WM_HELP: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winuser.h:11686:9, winuser.h:11686:9, winuser.h:11686:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const HELP_SETPOPUP_POS: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winuser.h:11687:9, winuser.h:11687:9, winuser.h:11687:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const HELP_TCARD: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:11689:9, winuser.h:11689:9, winuser.h:11689:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const HELP_TCARD_DATA: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:11690:9, winuser.h:11690:9, winuser.h:11690:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const HELP_TCARD_OTHER_CALLER: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winuser.h:11691:9, winuser.h:11691:9, winuser.h:11691:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const IDH_NO_HELP: i32 = 0x6f18i32; /* Integer(28440, Yes, Unknown) */ /* winuser.h:11694:9, winuser.h:11694:9, winuser.h:11694:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const IDH_MISSING_CONTEXT: i32 = 0x6f19i32; /* Integer(28441, Yes, Unknown) */ /* winuser.h:11695:9, winuser.h:11695:9, winuser.h:11695:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const IDH_GENERIC_HELP_BUTTON: i32 = 0x6f1ai32; /* Integer(28442, Yes, Unknown) */ /* winuser.h:11696:9, winuser.h:11696:9, winuser.h:11696:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const IDH_OK: i32 = 0x6f1bi32; /* Integer(28443, Yes, Unknown) */ /* winuser.h:11697:9, winuser.h:11697:9, winuser.h:11697:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const IDH_CANCEL: i32 = 0x6f1ci32; /* Integer(28444, Yes, Unknown) */ /* winuser.h:11698:9, winuser.h:11698:9, winuser.h:11698:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const IDH_HELP: i32 = 0x6f1di32; /* Integer(28445, Yes, Unknown) */ /* winuser.h:11699:9, winuser.h:11699:9, winuser.h:11699:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const GR_GDIOBJECTS: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:11734:9, winuser.h:11734:9, winuser.h:11734:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const GR_USEROBJECTS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:11735:9, winuser.h:11735:9, winuser.h:11735:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GR_GDIOBJECTS_PEAK: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:11738:9, winuser.h:11738:9, winuser.h:11738:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GR_USEROBJECTS_PEAK: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:11739:9, winuser.h:11739:9, winuser.h:11739:9 */
pub const SPI_GETBEEP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:11776:9, winuser.h:11776:9, winuser.h:11776:9 */
pub const SPI_SETBEEP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:11777:9, winuser.h:11777:9, winuser.h:11777:9 */
pub const SPI_GETMOUSE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:11778:9, winuser.h:11778:9, winuser.h:11778:9 */
pub const SPI_SETMOUSE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:11779:9, winuser.h:11779:9, winuser.h:11779:9 */
pub const SPI_GETBORDER: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:11780:9, winuser.h:11780:9, winuser.h:11780:9 */
pub const SPI_SETBORDER: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:11781:9, winuser.h:11781:9, winuser.h:11781:9 */
pub const SPI_GETKEYBOARDSPEED: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:11782:9, winuser.h:11782:9, winuser.h:11782:9 */
pub const SPI_SETKEYBOARDSPEED: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:11783:9, winuser.h:11783:9, winuser.h:11783:9 */
pub const SPI_LANGDRIVER: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winuser.h:11784:9, winuser.h:11784:9, winuser.h:11784:9 */
pub const SPI_ICONHORIZONTALSPACING: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winuser.h:11785:9, winuser.h:11785:9, winuser.h:11785:9 */
pub const SPI_GETSCREENSAVETIMEOUT: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winuser.h:11786:9, winuser.h:11786:9, winuser.h:11786:9 */
pub const SPI_SETSCREENSAVETIMEOUT: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winuser.h:11787:9, winuser.h:11787:9, winuser.h:11787:9 */
pub const SPI_GETSCREENSAVEACTIVE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:11788:9, winuser.h:11788:9, winuser.h:11788:9 */
pub const SPI_SETSCREENSAVEACTIVE: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winuser.h:11789:9, winuser.h:11789:9, winuser.h:11789:9 */
pub const SPI_GETGRIDGRANULARITY: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winuser.h:11790:9, winuser.h:11790:9, winuser.h:11790:9 */
pub const SPI_SETGRIDGRANULARITY: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winuser.h:11791:9, winuser.h:11791:9, winuser.h:11791:9 */
pub const SPI_SETDESKWALLPAPER: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winuser.h:11792:9, winuser.h:11792:9, winuser.h:11792:9 */
pub const SPI_SETDESKPATTERN: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winuser.h:11793:9, winuser.h:11793:9, winuser.h:11793:9 */
pub const SPI_GETKEYBOARDDELAY: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winuser.h:11794:9, winuser.h:11794:9, winuser.h:11794:9 */
pub const SPI_SETKEYBOARDDELAY: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winuser.h:11795:9, winuser.h:11795:9, winuser.h:11795:9 */
pub const SPI_ICONVERTICALSPACING: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winuser.h:11796:9, winuser.h:11796:9, winuser.h:11796:9 */
pub const SPI_GETICONTITLEWRAP: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* winuser.h:11797:9, winuser.h:11797:9, winuser.h:11797:9 */
pub const SPI_SETICONTITLEWRAP: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* winuser.h:11798:9, winuser.h:11798:9, winuser.h:11798:9 */
pub const SPI_GETMENUDROPALIGNMENT: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* winuser.h:11799:9, winuser.h:11799:9, winuser.h:11799:9 */
pub const SPI_SETMENUDROPALIGNMENT: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* winuser.h:11800:9, winuser.h:11800:9, winuser.h:11800:9 */
pub const SPI_SETDOUBLECLKWIDTH: i32 = 0x1di32; /* Integer(29, Yes, Unknown) */ /* winuser.h:11801:9, winuser.h:11801:9, winuser.h:11801:9 */
pub const SPI_SETDOUBLECLKHEIGHT: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* winuser.h:11802:9, winuser.h:11802:9, winuser.h:11802:9 */
pub const SPI_GETICONTITLELOGFONT: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* winuser.h:11803:9, winuser.h:11803:9, winuser.h:11803:9 */
pub const SPI_SETDOUBLECLICKTIME: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:11804:9, winuser.h:11804:9, winuser.h:11804:9 */
pub const SPI_SETMOUSEBUTTONSWAP: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* winuser.h:11805:9, winuser.h:11805:9, winuser.h:11805:9 */
pub const SPI_SETICONTITLELOGFONT: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* winuser.h:11806:9, winuser.h:11806:9, winuser.h:11806:9 */
pub const SPI_GETFASTTASKSWITCH: i32 = 0x23i32; /* Integer(35, Yes, Unknown) */ /* winuser.h:11807:9, winuser.h:11807:9, winuser.h:11807:9 */
pub const SPI_SETFASTTASKSWITCH: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* winuser.h:11808:9, winuser.h:11808:9, winuser.h:11808:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETDRAGFULLWINDOWS: i32 = 0x25i32; /* Integer(37, Yes, Unknown) */ /* winuser.h:11810:9, winuser.h:11810:9, winuser.h:11810:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETDRAGFULLWINDOWS: i32 = 0x26i32; /* Integer(38, Yes, Unknown) */ /* winuser.h:11811:9, winuser.h:11811:9, winuser.h:11811:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETNONCLIENTMETRICS: i32 = 0x29i32; /* Integer(41, Yes, Unknown) */ /* winuser.h:11812:9, winuser.h:11812:9, winuser.h:11812:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETNONCLIENTMETRICS: i32 = 0x2ai32; /* Integer(42, Yes, Unknown) */ /* winuser.h:11813:9, winuser.h:11813:9, winuser.h:11813:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETMINIMIZEDMETRICS: i32 = 0x2bi32; /* Integer(43, Yes, Unknown) */ /* winuser.h:11814:9, winuser.h:11814:9, winuser.h:11814:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETMINIMIZEDMETRICS: i32 = 0x2ci32; /* Integer(44, Yes, Unknown) */ /* winuser.h:11815:9, winuser.h:11815:9, winuser.h:11815:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETICONMETRICS: i32 = 0x2di32; /* Integer(45, Yes, Unknown) */ /* winuser.h:11816:9, winuser.h:11816:9, winuser.h:11816:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETICONMETRICS: i32 = 0x2ei32; /* Integer(46, Yes, Unknown) */ /* winuser.h:11817:9, winuser.h:11817:9, winuser.h:11817:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETWORKAREA: i32 = 0x2fi32; /* Integer(47, Yes, Unknown) */ /* winuser.h:11818:9, winuser.h:11818:9, winuser.h:11818:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETWORKAREA: i32 = 0x30i32; /* Integer(48, Yes, Unknown) */ /* winuser.h:11819:9, winuser.h:11819:9, winuser.h:11819:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETPENWINDOWS: i32 = 0x31i32; /* Integer(49, Yes, Unknown) */ /* winuser.h:11820:9, winuser.h:11820:9, winuser.h:11820:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETHIGHCONTRAST: i32 = 0x42i32; /* Integer(66, Yes, Unknown) */ /* winuser.h:11822:9, winuser.h:11822:9, winuser.h:11822:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETHIGHCONTRAST: i32 = 0x43i32; /* Integer(67, Yes, Unknown) */ /* winuser.h:11823:9, winuser.h:11823:9, winuser.h:11823:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETKEYBOARDPREF: i32 = 0x44i32; /* Integer(68, Yes, Unknown) */ /* winuser.h:11824:9, winuser.h:11824:9, winuser.h:11824:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETKEYBOARDPREF: i32 = 0x45i32; /* Integer(69, Yes, Unknown) */ /* winuser.h:11825:9, winuser.h:11825:9, winuser.h:11825:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETSCREENREADER: i32 = 0x46i32; /* Integer(70, Yes, Unknown) */ /* winuser.h:11826:9, winuser.h:11826:9, winuser.h:11826:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETSCREENREADER: i32 = 0x47i32; /* Integer(71, Yes, Unknown) */ /* winuser.h:11827:9, winuser.h:11827:9, winuser.h:11827:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETANIMATION: i32 = 0x48i32; /* Integer(72, Yes, Unknown) */ /* winuser.h:11828:9, winuser.h:11828:9, winuser.h:11828:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETANIMATION: i32 = 0x49i32; /* Integer(73, Yes, Unknown) */ /* winuser.h:11829:9, winuser.h:11829:9, winuser.h:11829:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETFONTSMOOTHING: i32 = 0x4ai32; /* Integer(74, Yes, Unknown) */ /* winuser.h:11830:9, winuser.h:11830:9, winuser.h:11830:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETFONTSMOOTHING: i32 = 0x4bi32; /* Integer(75, Yes, Unknown) */ /* winuser.h:11831:9, winuser.h:11831:9, winuser.h:11831:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETDRAGWIDTH: i32 = 0x4ci32; /* Integer(76, Yes, Unknown) */ /* winuser.h:11832:9, winuser.h:11832:9, winuser.h:11832:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETDRAGHEIGHT: i32 = 0x4di32; /* Integer(77, Yes, Unknown) */ /* winuser.h:11833:9, winuser.h:11833:9, winuser.h:11833:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETHANDHELD: i32 = 0x4ei32; /* Integer(78, Yes, Unknown) */ /* winuser.h:11834:9, winuser.h:11834:9, winuser.h:11834:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETLOWPOWERTIMEOUT: i32 = 0x4fi32; /* Integer(79, Yes, Unknown) */ /* winuser.h:11835:9, winuser.h:11835:9, winuser.h:11835:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETPOWEROFFTIMEOUT: i32 = 0x50i32; /* Integer(80, Yes, Unknown) */ /* winuser.h:11836:9, winuser.h:11836:9, winuser.h:11836:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETLOWPOWERTIMEOUT: i32 = 0x51i32; /* Integer(81, Yes, Unknown) */ /* winuser.h:11837:9, winuser.h:11837:9, winuser.h:11837:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETPOWEROFFTIMEOUT: i32 = 0x52i32; /* Integer(82, Yes, Unknown) */ /* winuser.h:11838:9, winuser.h:11838:9, winuser.h:11838:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETLOWPOWERACTIVE: i32 = 0x53i32; /* Integer(83, Yes, Unknown) */ /* winuser.h:11839:9, winuser.h:11839:9, winuser.h:11839:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETPOWEROFFACTIVE: i32 = 0x54i32; /* Integer(84, Yes, Unknown) */ /* winuser.h:11840:9, winuser.h:11840:9, winuser.h:11840:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETLOWPOWERACTIVE: i32 = 0x55i32; /* Integer(85, Yes, Unknown) */ /* winuser.h:11841:9, winuser.h:11841:9, winuser.h:11841:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETPOWEROFFACTIVE: i32 = 0x56i32; /* Integer(86, Yes, Unknown) */ /* winuser.h:11842:9, winuser.h:11842:9, winuser.h:11842:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETCURSORS: i32 = 0x57i32; /* Integer(87, Yes, Unknown) */ /* winuser.h:11843:9, winuser.h:11843:9, winuser.h:11843:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETICONS: i32 = 0x58i32; /* Integer(88, Yes, Unknown) */ /* winuser.h:11844:9, winuser.h:11844:9, winuser.h:11844:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETDEFAULTINPUTLANG: i32 = 0x59i32; /* Integer(89, Yes, Unknown) */ /* winuser.h:11845:9, winuser.h:11845:9, winuser.h:11845:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETDEFAULTINPUTLANG: i32 = 0x5ai32; /* Integer(90, Yes, Unknown) */ /* winuser.h:11846:9, winuser.h:11846:9, winuser.h:11846:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETLANGTOGGLE: i32 = 0x5bi32; /* Integer(91, Yes, Unknown) */ /* winuser.h:11847:9, winuser.h:11847:9, winuser.h:11847:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETWINDOWSEXTENSION: i32 = 0x5ci32; /* Integer(92, Yes, Unknown) */ /* winuser.h:11848:9, winuser.h:11848:9, winuser.h:11848:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETMOUSETRAILS: i32 = 0x5di32; /* Integer(93, Yes, Unknown) */ /* winuser.h:11849:9, winuser.h:11849:9, winuser.h:11849:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETMOUSETRAILS: i32 = 0x5ei32; /* Integer(94, Yes, Unknown) */ /* winuser.h:11850:9, winuser.h:11850:9, winuser.h:11850:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETSCREENSAVERRUNNING: i32 = 0x61i32; /* Integer(97, Yes, Unknown) */ /* winuser.h:11851:9, winuser.h:11851:9, winuser.h:11851:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winuser::SPI_SETSCREENSAVERRUNNING as SPI_SCREENSAVERRUNNING; /* winuser.h:11852:9, winuser.h:11852:9, winuser.h:11852:9 */
pub const SPI_GETFILTERKEYS: i32 = 0x32i32; /* Integer(50, Yes, Unknown) */ /* winuser.h:11854:9, winuser.h:11854:9, winuser.h:11854:9 */
pub const SPI_SETFILTERKEYS: i32 = 0x33i32; /* Integer(51, Yes, Unknown) */ /* winuser.h:11855:9, winuser.h:11855:9, winuser.h:11855:9 */
pub const SPI_GETTOGGLEKEYS: i32 = 0x34i32; /* Integer(52, Yes, Unknown) */ /* winuser.h:11856:9, winuser.h:11856:9, winuser.h:11856:9 */
pub const SPI_SETTOGGLEKEYS: i32 = 0x35i32; /* Integer(53, Yes, Unknown) */ /* winuser.h:11857:9, winuser.h:11857:9, winuser.h:11857:9 */
pub const SPI_GETMOUSEKEYS: i32 = 0x36i32; /* Integer(54, Yes, Unknown) */ /* winuser.h:11858:9, winuser.h:11858:9, winuser.h:11858:9 */
pub const SPI_SETMOUSEKEYS: i32 = 0x37i32; /* Integer(55, Yes, Unknown) */ /* winuser.h:11859:9, winuser.h:11859:9, winuser.h:11859:9 */
pub const SPI_GETSHOWSOUNDS: i32 = 0x38i32; /* Integer(56, Yes, Unknown) */ /* winuser.h:11860:9, winuser.h:11860:9, winuser.h:11860:9 */
pub const SPI_SETSHOWSOUNDS: i32 = 0x39i32; /* Integer(57, Yes, Unknown) */ /* winuser.h:11861:9, winuser.h:11861:9, winuser.h:11861:9 */
pub const SPI_GETSTICKYKEYS: i32 = 0x3ai32; /* Integer(58, Yes, Unknown) */ /* winuser.h:11862:9, winuser.h:11862:9, winuser.h:11862:9 */
pub const SPI_SETSTICKYKEYS: i32 = 0x3bi32; /* Integer(59, Yes, Unknown) */ /* winuser.h:11863:9, winuser.h:11863:9, winuser.h:11863:9 */
pub const SPI_GETACCESSTIMEOUT: i32 = 0x3ci32; /* Integer(60, Yes, Unknown) */ /* winuser.h:11864:9, winuser.h:11864:9, winuser.h:11864:9 */
pub const SPI_SETACCESSTIMEOUT: i32 = 0x3di32; /* Integer(61, Yes, Unknown) */ /* winuser.h:11865:9, winuser.h:11865:9, winuser.h:11865:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETSERIALKEYS: i32 = 0x3ei32; /* Integer(62, Yes, Unknown) */ /* winuser.h:11867:9, winuser.h:11867:9, winuser.h:11867:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETSERIALKEYS: i32 = 0x3fi32; /* Integer(63, Yes, Unknown) */ /* winuser.h:11868:9, winuser.h:11868:9, winuser.h:11868:9 */
pub const SPI_GETSOUNDSENTRY: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:11870:9, winuser.h:11870:9, winuser.h:11870:9 */
pub const SPI_SETSOUNDSENTRY: i32 = 0x41i32; /* Integer(65, Yes, Unknown) */ /* winuser.h:11871:9, winuser.h:11871:9, winuser.h:11871:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETSNAPTODEFBUTTON: i32 = 0x5fi32; /* Integer(95, Yes, Unknown) */ /* winuser.h:11873:9, winuser.h:11873:9, winuser.h:11873:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETSNAPTODEFBUTTON: i32 = 0x60i32; /* Integer(96, Yes, Unknown) */ /* winuser.h:11874:9, winuser.h:11874:9, winuser.h:11874:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETMOUSEHOVERWIDTH: i32 = 0x62i32; /* Integer(98, Yes, Unknown) */ /* winuser.h:11877:9, winuser.h:11877:9, winuser.h:11877:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETMOUSEHOVERWIDTH: i32 = 0x63i32; /* Integer(99, Yes, Unknown) */ /* winuser.h:11878:9, winuser.h:11878:9, winuser.h:11878:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETMOUSEHOVERHEIGHT: i32 = 0x64i32; /* Integer(100, Yes, Unknown) */ /* winuser.h:11879:9, winuser.h:11879:9, winuser.h:11879:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETMOUSEHOVERHEIGHT: i32 = 0x65i32; /* Integer(101, Yes, Unknown) */ /* winuser.h:11880:9, winuser.h:11880:9, winuser.h:11880:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETMOUSEHOVERTIME: i32 = 0x66i32; /* Integer(102, Yes, Unknown) */ /* winuser.h:11881:9, winuser.h:11881:9, winuser.h:11881:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETMOUSEHOVERTIME: i32 = 0x67i32; /* Integer(103, Yes, Unknown) */ /* winuser.h:11882:9, winuser.h:11882:9, winuser.h:11882:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETWHEELSCROLLLINES: i32 = 0x68i32; /* Integer(104, Yes, Unknown) */ /* winuser.h:11883:9, winuser.h:11883:9, winuser.h:11883:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETWHEELSCROLLLINES: i32 = 0x69i32; /* Integer(105, Yes, Unknown) */ /* winuser.h:11884:9, winuser.h:11884:9, winuser.h:11884:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETMENUSHOWDELAY: i32 = 0x6ai32; /* Integer(106, Yes, Unknown) */ /* winuser.h:11885:9, winuser.h:11885:9, winuser.h:11885:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETMENUSHOWDELAY: i32 = 0x6bi32; /* Integer(107, Yes, Unknown) */ /* winuser.h:11886:9, winuser.h:11886:9, winuser.h:11886:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_GETWHEELSCROLLCHARS: i32 = 0x6ci32; /* Integer(108, Yes, Unknown) */ /* winuser.h:11889:9, winuser.h:11889:9, winuser.h:11889:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_SETWHEELSCROLLCHARS: i32 = 0x6di32; /* Integer(109, Yes, Unknown) */ /* winuser.h:11890:9, winuser.h:11890:9, winuser.h:11890:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_GETSHOWIMEUI: i32 = 0x6ei32; /* Integer(110, Yes, Unknown) */ /* winuser.h:11893:9, winuser.h:11893:9, winuser.h:11893:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const SPI_SETSHOWIMEUI: i32 = 0x6fi32; /* Integer(111, Yes, Unknown) */ /* winuser.h:11894:9, winuser.h:11894:9, winuser.h:11894:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETMOUSESPEED: i32 = 0x70i32; /* Integer(112, Yes, Unknown) */ /* winuser.h:11899:9, winuser.h:11899:9, winuser.h:11899:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETMOUSESPEED: i32 = 0x71i32; /* Integer(113, Yes, Unknown) */ /* winuser.h:11900:9, winuser.h:11900:9, winuser.h:11900:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETSCREENSAVERRUNNING: i32 = 0x72i32; /* Integer(114, Yes, Unknown) */ /* winuser.h:11901:9, winuser.h:11901:9, winuser.h:11901:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETDESKWALLPAPER: i32 = 0x73i32; /* Integer(115, Yes, Unknown) */ /* winuser.h:11902:9, winuser.h:11902:9, winuser.h:11902:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_GETAUDIODESCRIPTION: i32 = 0x74i32; /* Integer(116, Yes, Unknown) */ /* winuser.h:11906:9, winuser.h:11906:9, winuser.h:11906:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_SETAUDIODESCRIPTION: i32 = 0x75i32; /* Integer(117, Yes, Unknown) */ /* winuser.h:11907:9, winuser.h:11907:9, winuser.h:11907:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_GETSCREENSAVESECURE: i32 = 0x76i32; /* Integer(118, Yes, Unknown) */ /* winuser.h:11909:9, winuser.h:11909:9, winuser.h:11909:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_SETSCREENSAVESECURE: i32 = 0x77i32; /* Integer(119, Yes, Unknown) */ /* winuser.h:11910:9, winuser.h:11910:9, winuser.h:11910:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETHUNGAPPTIMEOUT: i32 = 0x78i32; /* Integer(120, Yes, Unknown) */ /* winuser.h:11914:9, winuser.h:11914:9, winuser.h:11914:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETHUNGAPPTIMEOUT: i32 = 0x79i32; /* Integer(121, Yes, Unknown) */ /* winuser.h:11915:9, winuser.h:11915:9, winuser.h:11915:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETWAITTOKILLTIMEOUT: i32 = 0x7ai32; /* Integer(122, Yes, Unknown) */ /* winuser.h:11916:9, winuser.h:11916:9, winuser.h:11916:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETWAITTOKILLTIMEOUT: i32 = 0x7bi32; /* Integer(123, Yes, Unknown) */ /* winuser.h:11917:9, winuser.h:11917:9, winuser.h:11917:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETWAITTOKILLSERVICETIMEOUT: i32 = 0x7ci32; /* Integer(124, Yes, Unknown) */ /* winuser.h:11918:9, winuser.h:11918:9, winuser.h:11918:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETWAITTOKILLSERVICETIMEOUT: i32 = 0x7di32; /* Integer(125, Yes, Unknown) */ /* winuser.h:11919:9, winuser.h:11919:9, winuser.h:11919:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETMOUSEDOCKTHRESHOLD: i32 = 0x7ei32; /* Integer(126, Yes, Unknown) */ /* winuser.h:11920:9, winuser.h:11920:9, winuser.h:11920:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETMOUSEDOCKTHRESHOLD: i32 = 0x7fi32; /* Integer(127, Yes, Unknown) */ /* winuser.h:11921:9, winuser.h:11921:9, winuser.h:11921:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETPENDOCKTHRESHOLD: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:11922:9, winuser.h:11922:9, winuser.h:11922:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETPENDOCKTHRESHOLD: i32 = 0x81i32; /* Integer(129, Yes, Unknown) */ /* winuser.h:11923:9, winuser.h:11923:9, winuser.h:11923:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETWINARRANGING: i32 = 0x82i32; /* Integer(130, Yes, Unknown) */ /* winuser.h:11924:9, winuser.h:11924:9, winuser.h:11924:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETWINARRANGING: i32 = 0x83i32; /* Integer(131, Yes, Unknown) */ /* winuser.h:11925:9, winuser.h:11925:9, winuser.h:11925:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETMOUSEDRAGOUTTHRESHOLD: i32 = 0x84i32; /* Integer(132, Yes, Unknown) */ /* winuser.h:11926:9, winuser.h:11926:9, winuser.h:11926:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETMOUSEDRAGOUTTHRESHOLD: i32 = 0x85i32; /* Integer(133, Yes, Unknown) */ /* winuser.h:11927:9, winuser.h:11927:9, winuser.h:11927:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETPENDRAGOUTTHRESHOLD: i32 = 0x86i32; /* Integer(134, Yes, Unknown) */ /* winuser.h:11928:9, winuser.h:11928:9, winuser.h:11928:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETPENDRAGOUTTHRESHOLD: i32 = 0x87i32; /* Integer(135, Yes, Unknown) */ /* winuser.h:11929:9, winuser.h:11929:9, winuser.h:11929:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETMOUSESIDEMOVETHRESHOLD: i32 = 0x88i32; /* Integer(136, Yes, Unknown) */ /* winuser.h:11930:9, winuser.h:11930:9, winuser.h:11930:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETMOUSESIDEMOVETHRESHOLD: i32 = 0x89i32; /* Integer(137, Yes, Unknown) */ /* winuser.h:11931:9, winuser.h:11931:9, winuser.h:11931:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETPENSIDEMOVETHRESHOLD: i32 = 0x8ai32; /* Integer(138, Yes, Unknown) */ /* winuser.h:11932:9, winuser.h:11932:9, winuser.h:11932:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETPENSIDEMOVETHRESHOLD: i32 = 0x8bi32; /* Integer(139, Yes, Unknown) */ /* winuser.h:11933:9, winuser.h:11933:9, winuser.h:11933:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETDRAGFROMMAXIMIZE: i32 = 0x8ci32; /* Integer(140, Yes, Unknown) */ /* winuser.h:11934:9, winuser.h:11934:9, winuser.h:11934:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETDRAGFROMMAXIMIZE: i32 = 0x8di32; /* Integer(141, Yes, Unknown) */ /* winuser.h:11935:9, winuser.h:11935:9, winuser.h:11935:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETSNAPSIZING: i32 = 0x8ei32; /* Integer(142, Yes, Unknown) */ /* winuser.h:11936:9, winuser.h:11936:9, winuser.h:11936:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETSNAPSIZING: i32 = 0x8fi32; /* Integer(143, Yes, Unknown) */ /* winuser.h:11937:9, winuser.h:11937:9, winuser.h:11937:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETDOCKMOVING: i32 = 0x90i32; /* Integer(144, Yes, Unknown) */ /* winuser.h:11938:9, winuser.h:11938:9, winuser.h:11938:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETDOCKMOVING: i32 = 0x91i32; /* Integer(145, Yes, Unknown) */ /* winuser.h:11939:9, winuser.h:11939:9, winuser.h:11939:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const MAX_TOUCH_PREDICTION_FILTER_TAPS: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:11943:9, winuser.h:11943:9, winuser.h:11943:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_LATENCY: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:11956:9, winuser.h:11956:9, winuser.h:11956:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_SAMPLETIME: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:11957:9, winuser.h:11957:9, winuser.h:11957:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const TOUCHPREDICTIONPARAMETERS_DEFAULT_USE_HW_TIMESTAMP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:11958:9, winuser.h:11958:9, winuser.h:11958:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SPI_GETTOUCHPREDICTIONPARAMETERS: i32 = 0x9ci32; /* Integer(156, Yes, Unknown) */ /* winuser.h:11968:9, winuser.h:11968:9, winuser.h:11968:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SPI_SETTOUCHPREDICTIONPARAMETERS: i32 = 0x9di32; /* Integer(157, Yes, Unknown) */ /* winuser.h:11969:9, winuser.h:11969:9, winuser.h:11969:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const MAX_LOGICALDPIOVERRIDE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:11971:9, winuser.h:11971:9, winuser.h:11971:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const MIN_LOGICALDPIOVERRIDE: i32 = -0x2i32; /* Unary(Neg, Integer(2, Yes, Unknown)) */ /* winuser.h:11972:9, winuser.h:11972:9, winuser.h:11972:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SPI_GETLOGICALDPIOVERRIDE: i32 = 0x9ei32; /* Integer(158, Yes, Unknown) */ /* winuser.h:11974:9, winuser.h:11974:9, winuser.h:11974:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SPI_SETLOGICALDPIOVERRIDE: i32 = 0x9fi32; /* Integer(159, Yes, Unknown) */ /* winuser.h:11975:9, winuser.h:11975:9, winuser.h:11975:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SPI_GETMOUSECORNERCLIPLENGTH: i32 = 0xa0i32; /* Integer(160, Yes, Unknown) */ /* winuser.h:11977:9, winuser.h:11977:9, winuser.h:11977:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SPI_SETMOUSECORNERCLIPLENGTH: i32 = 0xa1i32; /* Integer(161, Yes, Unknown) */ /* winuser.h:11978:9, winuser.h:11978:9, winuser.h:11978:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SPI_GETMENURECT: i32 = 0xa2i32; /* Integer(162, Yes, Unknown) */ /* winuser.h:11980:9, winuser.h:11980:9, winuser.h:11980:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SPI_SETMENURECT: i32 = 0xa3i32; /* Integer(163, Yes, Unknown) */ /* winuser.h:11981:9, winuser.h:11981:9, winuser.h:11981:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETACTIVEWINDOWTRACKING: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:11987:9, winuser.h:11987:9, winuser.h:11987:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETACTIVEWINDOWTRACKING: i32 = 0x1001i32; /* Integer(4097, Yes, Unknown) */ /* winuser.h:11988:9, winuser.h:11988:9, winuser.h:11988:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETMENUANIMATION: i32 = 0x1002i32; /* Integer(4098, Yes, Unknown) */ /* winuser.h:11989:9, winuser.h:11989:9, winuser.h:11989:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETMENUANIMATION: i32 = 0x1003i32; /* Integer(4099, Yes, Unknown) */ /* winuser.h:11990:9, winuser.h:11990:9, winuser.h:11990:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETCOMBOBOXANIMATION: i32 = 0x1004i32; /* Integer(4100, Yes, Unknown) */ /* winuser.h:11991:9, winuser.h:11991:9, winuser.h:11991:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETCOMBOBOXANIMATION: i32 = 0x1005i32; /* Integer(4101, Yes, Unknown) */ /* winuser.h:11992:9, winuser.h:11992:9, winuser.h:11992:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETLISTBOXSMOOTHSCROLLING: i32 = 0x1006i32; /* Integer(4102, Yes, Unknown) */ /* winuser.h:11993:9, winuser.h:11993:9, winuser.h:11993:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETLISTBOXSMOOTHSCROLLING: i32 = 0x1007i32; /* Integer(4103, Yes, Unknown) */ /* winuser.h:11994:9, winuser.h:11994:9, winuser.h:11994:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETGRADIENTCAPTIONS: i32 = 0x1008i32; /* Integer(4104, Yes, Unknown) */ /* winuser.h:11995:9, winuser.h:11995:9, winuser.h:11995:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETGRADIENTCAPTIONS: i32 = 0x1009i32; /* Integer(4105, Yes, Unknown) */ /* winuser.h:11996:9, winuser.h:11996:9, winuser.h:11996:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETKEYBOARDCUES: i32 = 0x100ai32; /* Integer(4106, Yes, Unknown) */ /* winuser.h:11997:9, winuser.h:11997:9, winuser.h:11997:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETKEYBOARDCUES: i32 = 0x100bi32; /* Integer(4107, Yes, Unknown) */ /* winuser.h:11998:9, winuser.h:11998:9, winuser.h:11998:9 */
#[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use ::winuser::SPI_GETKEYBOARDCUES as SPI_GETMENUUNDERLINES; /* winuser.h:11999:9, winuser.h:11999:9, winuser.h:11999:9 */
#[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use ::winuser::SPI_SETKEYBOARDCUES as SPI_SETMENUUNDERLINES; /* winuser.h:12000:9, winuser.h:12000:9, winuser.h:12000:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETACTIVEWNDTRKZORDER: i32 = 0x100ci32; /* Integer(4108, Yes, Unknown) */ /* winuser.h:12001:9, winuser.h:12001:9, winuser.h:12001:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETACTIVEWNDTRKZORDER: i32 = 0x100di32; /* Integer(4109, Yes, Unknown) */ /* winuser.h:12002:9, winuser.h:12002:9, winuser.h:12002:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETHOTTRACKING: i32 = 0x100ei32; /* Integer(4110, Yes, Unknown) */ /* winuser.h:12003:9, winuser.h:12003:9, winuser.h:12003:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETHOTTRACKING: i32 = 0x100fi32; /* Integer(4111, Yes, Unknown) */ /* winuser.h:12004:9, winuser.h:12004:9, winuser.h:12004:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETMENUFADE: i32 = 0x1012i32; /* Integer(4114, Yes, Unknown) */ /* winuser.h:12005:9, winuser.h:12005:9, winuser.h:12005:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETMENUFADE: i32 = 0x1013i32; /* Integer(4115, Yes, Unknown) */ /* winuser.h:12006:9, winuser.h:12006:9, winuser.h:12006:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETSELECTIONFADE: i32 = 0x1014i32; /* Integer(4116, Yes, Unknown) */ /* winuser.h:12007:9, winuser.h:12007:9, winuser.h:12007:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETSELECTIONFADE: i32 = 0x1015i32; /* Integer(4117, Yes, Unknown) */ /* winuser.h:12008:9, winuser.h:12008:9, winuser.h:12008:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETTOOLTIPANIMATION: i32 = 0x1016i32; /* Integer(4118, Yes, Unknown) */ /* winuser.h:12009:9, winuser.h:12009:9, winuser.h:12009:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETTOOLTIPANIMATION: i32 = 0x1017i32; /* Integer(4119, Yes, Unknown) */ /* winuser.h:12010:9, winuser.h:12010:9, winuser.h:12010:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETTOOLTIPFADE: i32 = 0x1018i32; /* Integer(4120, Yes, Unknown) */ /* winuser.h:12011:9, winuser.h:12011:9, winuser.h:12011:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETTOOLTIPFADE: i32 = 0x1019i32; /* Integer(4121, Yes, Unknown) */ /* winuser.h:12012:9, winuser.h:12012:9, winuser.h:12012:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETCURSORSHADOW: i32 = 0x101ai32; /* Integer(4122, Yes, Unknown) */ /* winuser.h:12013:9, winuser.h:12013:9, winuser.h:12013:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETCURSORSHADOW: i32 = 0x101bi32; /* Integer(4123, Yes, Unknown) */ /* winuser.h:12014:9, winuser.h:12014:9, winuser.h:12014:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_GETMOUSESONAR: i32 = 0x101ci32; /* Integer(4124, Yes, Unknown) */ /* winuser.h:12016:9, winuser.h:12016:9, winuser.h:12016:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_SETMOUSESONAR: i32 = 0x101di32; /* Integer(4125, Yes, Unknown) */ /* winuser.h:12017:9, winuser.h:12017:9, winuser.h:12017:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_GETMOUSECLICKLOCK: i32 = 0x101ei32; /* Integer(4126, Yes, Unknown) */ /* winuser.h:12018:9, winuser.h:12018:9, winuser.h:12018:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_SETMOUSECLICKLOCK: i32 = 0x101fi32; /* Integer(4127, Yes, Unknown) */ /* winuser.h:12019:9, winuser.h:12019:9, winuser.h:12019:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_GETMOUSEVANISH: i32 = 0x1020i32; /* Integer(4128, Yes, Unknown) */ /* winuser.h:12020:9, winuser.h:12020:9, winuser.h:12020:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_SETMOUSEVANISH: i32 = 0x1021i32; /* Integer(4129, Yes, Unknown) */ /* winuser.h:12021:9, winuser.h:12021:9, winuser.h:12021:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_GETFLATMENU: i32 = 0x1022i32; /* Integer(4130, Yes, Unknown) */ /* winuser.h:12022:9, winuser.h:12022:9, winuser.h:12022:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_SETFLATMENU: i32 = 0x1023i32; /* Integer(4131, Yes, Unknown) */ /* winuser.h:12023:9, winuser.h:12023:9, winuser.h:12023:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_GETDROPSHADOW: i32 = 0x1024i32; /* Integer(4132, Yes, Unknown) */ /* winuser.h:12024:9, winuser.h:12024:9, winuser.h:12024:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_SETDROPSHADOW: i32 = 0x1025i32; /* Integer(4133, Yes, Unknown) */ /* winuser.h:12025:9, winuser.h:12025:9, winuser.h:12025:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_GETBLOCKSENDINPUTRESETS: i32 = 0x1026i32; /* Integer(4134, Yes, Unknown) */ /* winuser.h:12026:9, winuser.h:12026:9, winuser.h:12026:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_SETBLOCKSENDINPUTRESETS: i32 = 0x1027i32; /* Integer(4135, Yes, Unknown) */ /* winuser.h:12027:9, winuser.h:12027:9, winuser.h:12027:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETUIEFFECTS: i32 = 0x103ei32; /* Integer(4158, Yes, Unknown) */ /* winuser.h:12030:9, winuser.h:12030:9, winuser.h:12030:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETUIEFFECTS: i32 = 0x103fi32; /* Integer(4159, Yes, Unknown) */ /* winuser.h:12031:9, winuser.h:12031:9, winuser.h:12031:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_GETDISABLEOVERLAPPEDCONTENT: i32 = 0x1040i32; /* Integer(4160, Yes, Unknown) */ /* winuser.h:12034:9, winuser.h:12034:9, winuser.h:12034:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_SETDISABLEOVERLAPPEDCONTENT: i32 = 0x1041i32; /* Integer(4161, Yes, Unknown) */ /* winuser.h:12035:9, winuser.h:12035:9, winuser.h:12035:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_GETCLIENTAREAANIMATION: i32 = 0x1042i32; /* Integer(4162, Yes, Unknown) */ /* winuser.h:12036:9, winuser.h:12036:9, winuser.h:12036:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_SETCLIENTAREAANIMATION: i32 = 0x1043i32; /* Integer(4163, Yes, Unknown) */ /* winuser.h:12037:9, winuser.h:12037:9, winuser.h:12037:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_GETCLEARTYPE: i32 = 0x1048i32; /* Integer(4168, Yes, Unknown) */ /* winuser.h:12038:9, winuser.h:12038:9, winuser.h:12038:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_SETCLEARTYPE: i32 = 0x1049i32; /* Integer(4169, Yes, Unknown) */ /* winuser.h:12039:9, winuser.h:12039:9, winuser.h:12039:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_GETSPEECHRECOGNITION: i32 = 0x104ai32; /* Integer(4170, Yes, Unknown) */ /* winuser.h:12040:9, winuser.h:12040:9, winuser.h:12040:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_SETSPEECHRECOGNITION: i32 = 0x104bi32; /* Integer(4171, Yes, Unknown) */ /* winuser.h:12041:9, winuser.h:12041:9, winuser.h:12041:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETCARETBROWSING: i32 = 0x104ci32; /* Integer(4172, Yes, Unknown) */ /* winuser.h:12045:9, winuser.h:12045:9, winuser.h:12045:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETCARETBROWSING: i32 = 0x104di32; /* Integer(4173, Yes, Unknown) */ /* winuser.h:12046:9, winuser.h:12046:9, winuser.h:12046:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETTHREADLOCALINPUTSETTINGS: i32 = 0x104ei32; /* Integer(4174, Yes, Unknown) */ /* winuser.h:12047:9, winuser.h:12047:9, winuser.h:12047:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETTHREADLOCALINPUTSETTINGS: i32 = 0x104fi32; /* Integer(4175, Yes, Unknown) */ /* winuser.h:12048:9, winuser.h:12048:9, winuser.h:12048:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_GETSYSTEMLANGUAGEBAR: i32 = 0x1050i32; /* Integer(4176, Yes, Unknown) */ /* winuser.h:12049:9, winuser.h:12049:9, winuser.h:12049:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SPI_SETSYSTEMLANGUAGEBAR: i32 = 0x1051i32; /* Integer(4177, Yes, Unknown) */ /* winuser.h:12050:9, winuser.h:12050:9, winuser.h:12050:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETFOREGROUNDLOCKTIMEOUT: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:12053:9, winuser.h:12053:9, winuser.h:12053:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETFOREGROUNDLOCKTIMEOUT: i32 = 0x2001i32; /* Integer(8193, Yes, Unknown) */ /* winuser.h:12054:9, winuser.h:12054:9, winuser.h:12054:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETACTIVEWNDTRKTIMEOUT: i32 = 0x2002i32; /* Integer(8194, Yes, Unknown) */ /* winuser.h:12055:9, winuser.h:12055:9, winuser.h:12055:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETACTIVEWNDTRKTIMEOUT: i32 = 0x2003i32; /* Integer(8195, Yes, Unknown) */ /* winuser.h:12056:9, winuser.h:12056:9, winuser.h:12056:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETFOREGROUNDFLASHCOUNT: i32 = 0x2004i32; /* Integer(8196, Yes, Unknown) */ /* winuser.h:12057:9, winuser.h:12057:9, winuser.h:12057:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETFOREGROUNDFLASHCOUNT: i32 = 0x2005i32; /* Integer(8197, Yes, Unknown) */ /* winuser.h:12058:9, winuser.h:12058:9, winuser.h:12058:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_GETCARETWIDTH: i32 = 0x2006i32; /* Integer(8198, Yes, Unknown) */ /* winuser.h:12059:9, winuser.h:12059:9, winuser.h:12059:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SPI_SETCARETWIDTH: i32 = 0x2007i32; /* Integer(8199, Yes, Unknown) */ /* winuser.h:12060:9, winuser.h:12060:9, winuser.h:12060:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_GETMOUSECLICKLOCKTIME: i32 = 0x2008i32; /* Integer(8200, Yes, Unknown) */ /* winuser.h:12063:9, winuser.h:12063:9, winuser.h:12063:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_SETMOUSECLICKLOCKTIME: i32 = 0x2009i32; /* Integer(8201, Yes, Unknown) */ /* winuser.h:12064:9, winuser.h:12064:9, winuser.h:12064:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_GETFONTSMOOTHINGTYPE: i32 = 0x200ai32; /* Integer(8202, Yes, Unknown) */ /* winuser.h:12065:9, winuser.h:12065:9, winuser.h:12065:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_SETFONTSMOOTHINGTYPE: i32 = 0x200bi32; /* Integer(8203, Yes, Unknown) */ /* winuser.h:12066:9, winuser.h:12066:9, winuser.h:12066:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const FE_FONTSMOOTHINGSTANDARD: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12069:9, winuser.h:12069:9, winuser.h:12069:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const FE_FONTSMOOTHINGCLEARTYPE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12070:9, winuser.h:12070:9, winuser.h:12070:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_GETFONTSMOOTHINGCONTRAST: i32 = 0x200ci32; /* Integer(8204, Yes, Unknown) */ /* winuser.h:12072:9, winuser.h:12072:9, winuser.h:12072:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_SETFONTSMOOTHINGCONTRAST: i32 = 0x200di32; /* Integer(8205, Yes, Unknown) */ /* winuser.h:12073:9, winuser.h:12073:9, winuser.h:12073:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_GETFOCUSBORDERWIDTH: i32 = 0x200ei32; /* Integer(8206, Yes, Unknown) */ /* winuser.h:12075:9, winuser.h:12075:9, winuser.h:12075:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_SETFOCUSBORDERWIDTH: i32 = 0x200fi32; /* Integer(8207, Yes, Unknown) */ /* winuser.h:12076:9, winuser.h:12076:9, winuser.h:12076:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_GETFOCUSBORDERHEIGHT: i32 = 0x2010i32; /* Integer(8208, Yes, Unknown) */ /* winuser.h:12077:9, winuser.h:12077:9, winuser.h:12077:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_SETFOCUSBORDERHEIGHT: i32 = 0x2011i32; /* Integer(8209, Yes, Unknown) */ /* winuser.h:12078:9, winuser.h:12078:9, winuser.h:12078:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_GETFONTSMOOTHINGORIENTATION: i32 = 0x2012i32; /* Integer(8210, Yes, Unknown) */ /* winuser.h:12080:9, winuser.h:12080:9, winuser.h:12080:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const SPI_SETFONTSMOOTHINGORIENTATION: i32 = 0x2013i32; /* Integer(8211, Yes, Unknown) */ /* winuser.h:12081:9, winuser.h:12081:9, winuser.h:12081:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const FE_FONTSMOOTHINGORIENTATIONBGR: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:12084:9, winuser.h:12084:9, winuser.h:12084:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const FE_FONTSMOOTHINGORIENTATIONRGB: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12085:9, winuser.h:12085:9, winuser.h:12085:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_GETMINIMUMHITRADIUS: i32 = 0x2014i32; /* Integer(8212, Yes, Unknown) */ /* winuser.h:12089:9, winuser.h:12089:9, winuser.h:12089:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_SETMINIMUMHITRADIUS: i32 = 0x2015i32; /* Integer(8213, Yes, Unknown) */ /* winuser.h:12090:9, winuser.h:12090:9, winuser.h:12090:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_GETMESSAGEDURATION: i32 = 0x2016i32; /* Integer(8214, Yes, Unknown) */ /* winuser.h:12091:9, winuser.h:12091:9, winuser.h:12091:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const SPI_SETMESSAGEDURATION: i32 = 0x2017i32; /* Integer(8215, Yes, Unknown) */ /* winuser.h:12092:9, winuser.h:12092:9, winuser.h:12092:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SPI_GETCONTACTVISUALIZATION: i32 = 0x2018i32; /* Integer(8216, Yes, Unknown) */ /* winuser.h:12096:9, winuser.h:12096:9, winuser.h:12096:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SPI_SETCONTACTVISUALIZATION: i32 = 0x2019i32; /* Integer(8217, Yes, Unknown) */ /* winuser.h:12097:9, winuser.h:12097:9, winuser.h:12097:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const CONTACTVISUALIZATION_OFF: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:12099:9, winuser.h:12099:9, winuser.h:12099:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const CONTACTVISUALIZATION_ON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12100:9, winuser.h:12100:9, winuser.h:12100:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const CONTACTVISUALIZATION_PRESENTATIONMODE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12101:9, winuser.h:12101:9, winuser.h:12101:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SPI_GETGESTUREVISUALIZATION: i32 = 0x201ai32; /* Integer(8218, Yes, Unknown) */ /* winuser.h:12103:9, winuser.h:12103:9, winuser.h:12103:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SPI_SETGESTUREVISUALIZATION: i32 = 0x201bi32; /* Integer(8219, Yes, Unknown) */ /* winuser.h:12104:9, winuser.h:12104:9, winuser.h:12104:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const GESTUREVISUALIZATION_OFF: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:12106:9, winuser.h:12106:9, winuser.h:12106:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const GESTUREVISUALIZATION_ON: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* winuser.h:12107:9, winuser.h:12107:9, winuser.h:12107:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const GESTUREVISUALIZATION_TAP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12108:9, winuser.h:12108:9, winuser.h:12108:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const GESTUREVISUALIZATION_DOUBLETAP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12109:9, winuser.h:12109:9, winuser.h:12109:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const GESTUREVISUALIZATION_PRESSANDTAP: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:12110:9, winuser.h:12110:9, winuser.h:12110:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const GESTUREVISUALIZATION_PRESSANDHOLD: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:12111:9, winuser.h:12111:9, winuser.h:12111:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const GESTUREVISUALIZATION_RIGHTTAP: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:12112:9, winuser.h:12112:9, winuser.h:12112:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SPI_GETMOUSEWHEELROUTING: i32 = 0x201ci32; /* Integer(8220, Yes, Unknown) */ /* winuser.h:12116:9, winuser.h:12116:9, winuser.h:12116:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SPI_SETMOUSEWHEELROUTING: i32 = 0x201di32; /* Integer(8221, Yes, Unknown) */ /* winuser.h:12117:9, winuser.h:12117:9, winuser.h:12117:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const MOUSEWHEEL_ROUTING_FOCUS: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:12119:13, winuser.h:12119:13, winuser.h:12119:13 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const MOUSEWHEEL_ROUTING_HYBRID: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12120:13, winuser.h:12120:13, winuser.h:12120:13 */
pub const SPIF_UPDATEINIFILE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12129:9, winuser.h:12129:9, winuser.h:12129:9 */
pub const SPIF_SENDWININICHANGE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12130:9, winuser.h:12130:9, winuser.h:12130:9 */
#[doc(inline)] pub use ::winuser::SPIF_SENDWININICHANGE as SPIF_SENDCHANGE; /* winuser.h:12131:9, winuser.h:12131:9, winuser.h:12131:9 */
pub const METRICS_USEDEFAULT: i32 = -0x1i32; /* Unary(Neg, Integer(1, Yes, Unknown)) */ /* winuser.h:12134:9, winuser.h:12134:9, winuser.h:12134:9 */
pub const ARW_BOTTOMLEFT: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:12199:9, winuser.h:12199:9, winuser.h:12199:9 */
pub const ARW_BOTTOMRIGHT: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:12200:9, winuser.h:12200:9, winuser.h:12200:9 */
pub const ARW_TOPLEFT: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:12201:9, winuser.h:12201:9, winuser.h:12201:9 */
pub const ARW_TOPRIGHT: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* winuser.h:12202:9, winuser.h:12202:9, winuser.h:12202:9 */
pub const ARW_STARTMASK: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* winuser.h:12203:9, winuser.h:12203:9, winuser.h:12203:9 */
pub const ARW_STARTRIGHT: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winuser.h:12204:9, winuser.h:12204:9, winuser.h:12204:9 */
pub const ARW_STARTTOP: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winuser.h:12205:9, winuser.h:12205:9, winuser.h:12205:9 */
pub const ARW_LEFT: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:12207:9, winuser.h:12207:9, winuser.h:12207:9 */
pub const ARW_RIGHT: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winuser.h:12208:9, winuser.h:12208:9, winuser.h:12208:9 */
pub const ARW_UP: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:12209:9, winuser.h:12209:9, winuser.h:12209:9 */
pub const ARW_DOWN: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winuser.h:12210:9, winuser.h:12210:9, winuser.h:12210:9 */
pub const ARW_HIDE: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winuser.h:12211:9, winuser.h:12211:9, winuser.h:12211:9 */
#[cfg(feature="winapi_desktop")] pub const SERKF_SERIALKEYSON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12290:9, winuser.h:12290:9, winuser.h:12290:9 */
#[cfg(feature="winapi_desktop")] pub const SERKF_AVAILABLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12291:9, winuser.h:12291:9, winuser.h:12291:9 */
#[cfg(feature="winapi_desktop")] pub const SERKF_INDICATOR: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:12292:9, winuser.h:12292:9, winuser.h:12292:9 */
pub const HCF_HIGHCONTRASTON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12319:9, winuser.h:12319:9, winuser.h:12319:9 */
pub const HCF_AVAILABLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12320:9, winuser.h:12320:9, winuser.h:12320:9 */
pub const HCF_HOTKEYACTIVE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:12321:9, winuser.h:12321:9, winuser.h:12321:9 */
pub const HCF_CONFIRMHOTKEY: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:12322:9, winuser.h:12322:9, winuser.h:12322:9 */
pub const HCF_HOTKEYSOUND: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:12323:9, winuser.h:12323:9, winuser.h:12323:9 */
pub const HCF_INDICATOR: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:12324:9, winuser.h:12324:9, winuser.h:12324:9 */
pub const HCF_HOTKEYAVAILABLE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:12325:9, winuser.h:12325:9, winuser.h:12325:9 */
pub const HCF_LOGONDESKTOP: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:12326:9, winuser.h:12326:9, winuser.h:12326:9 */
pub const HCF_DEFAULTDESKTOP: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:12327:9, winuser.h:12327:9, winuser.h:12327:9 */
pub const CDS_UPDATEREGISTRY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12330:9, winuser.h:12330:9, winuser.h:12330:9 */
pub const CDS_TEST: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12331:9, winuser.h:12331:9, winuser.h:12331:9 */
pub const CDS_FULLSCREEN: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:12332:9, winuser.h:12332:9, winuser.h:12332:9 */
pub const CDS_GLOBAL: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:12333:9, winuser.h:12333:9, winuser.h:12333:9 */
pub const CDS_SET_PRIMARY: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:12334:9, winuser.h:12334:9, winuser.h:12334:9 */
pub const CDS_VIDEOPARAMETERS: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:12335:9, winuser.h:12335:9, winuser.h:12335:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const CDS_ENABLE_UNSAFE_MODES: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:12337:9, winuser.h:12337:9, winuser.h:12337:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const CDS_DISABLE_UNSAFE_MODES: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:12338:9, winuser.h:12338:9, winuser.h:12338:9 */
pub const CDS_RESET: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* winuser.h:12340:9, winuser.h:12340:9, winuser.h:12340:9 */
pub const CDS_RESET_EX: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* winuser.h:12341:9, winuser.h:12341:9, winuser.h:12341:9 */
pub const CDS_NORESET: i32 = 0x10000000i32; /* Integer(268435456, Yes, Unknown) */ /* winuser.h:12342:9, winuser.h:12342:9, winuser.h:12342:9 */
pub const DISP_CHANGE_SUCCESSFUL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:12347:9, winuser.h:12347:9, winuser.h:12347:9 */
pub const DISP_CHANGE_RESTART: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12348:9, winuser.h:12348:9, winuser.h:12348:9 */
pub const DISP_CHANGE_FAILED: i32 = -0x1i32; /* Unary(Neg, Integer(1, Yes, Unknown)) */ /* winuser.h:12349:9, winuser.h:12349:9, winuser.h:12349:9 */
pub const DISP_CHANGE_BADMODE: i32 = -0x2i32; /* Unary(Neg, Integer(2, Yes, Unknown)) */ /* winuser.h:12350:9, winuser.h:12350:9, winuser.h:12350:9 */
pub const DISP_CHANGE_NOTUPDATED: i32 = -0x3i32; /* Unary(Neg, Integer(3, Yes, Unknown)) */ /* winuser.h:12351:9, winuser.h:12351:9, winuser.h:12351:9 */
pub const DISP_CHANGE_BADFLAGS: i32 = -0x4i32; /* Unary(Neg, Integer(4, Yes, Unknown)) */ /* winuser.h:12352:9, winuser.h:12352:9, winuser.h:12352:9 */
pub const DISP_CHANGE_BADPARAM: i32 = -0x5i32; /* Unary(Neg, Integer(5, Yes, Unknown)) */ /* winuser.h:12353:9, winuser.h:12353:9, winuser.h:12353:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const DISP_CHANGE_BADDUALVIEW: i32 = -0x6i32; /* Unary(Neg, Integer(6, Yes, Unknown)) */ /* winuser.h:12355:9, winuser.h:12355:9, winuser.h:12355:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EDS_RAWMODE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12455:9, winuser.h:12455:9, winuser.h:12455:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EDS_ROTATEDMODE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:12456:9, winuser.h:12456:9, winuser.h:12456:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EDD_GET_DEVICE_INTERFACE_NAME: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12481:9, winuser.h:12481:9, winuser.h:12481:9 */
pub const FKF_FILTERKEYSON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12588:9, winuser.h:12588:9, winuser.h:12588:9 */
pub const FKF_AVAILABLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12589:9, winuser.h:12589:9, winuser.h:12589:9 */
pub const FKF_HOTKEYACTIVE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:12590:9, winuser.h:12590:9, winuser.h:12590:9 */
pub const FKF_CONFIRMHOTKEY: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:12591:9, winuser.h:12591:9, winuser.h:12591:9 */
pub const FKF_HOTKEYSOUND: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:12592:9, winuser.h:12592:9, winuser.h:12592:9 */
pub const FKF_INDICATOR: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:12593:9, winuser.h:12593:9, winuser.h:12593:9 */
pub const FKF_CLICKON: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:12594:9, winuser.h:12594:9, winuser.h:12594:9 */
pub const SKF_STICKYKEYSON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12611:9, winuser.h:12611:9, winuser.h:12611:9 */
pub const SKF_AVAILABLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12612:9, winuser.h:12612:9, winuser.h:12612:9 */
pub const SKF_HOTKEYACTIVE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:12613:9, winuser.h:12613:9, winuser.h:12613:9 */
pub const SKF_CONFIRMHOTKEY: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:12614:9, winuser.h:12614:9, winuser.h:12614:9 */
pub const SKF_HOTKEYSOUND: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:12615:9, winuser.h:12615:9, winuser.h:12615:9 */
pub const SKF_INDICATOR: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:12616:9, winuser.h:12616:9, winuser.h:12616:9 */
pub const SKF_AUDIBLEFEEDBACK: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:12617:9, winuser.h:12617:9, winuser.h:12617:9 */
pub const SKF_TRISTATE: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:12618:9, winuser.h:12618:9, winuser.h:12618:9 */
pub const SKF_TWOKEYSOFF: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:12619:9, winuser.h:12619:9, winuser.h:12619:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_LALTLATCHED: i32 = 0x10000000i32; /* Integer(268435456, Yes, Unknown) */ /* winuser.h:12621:9, winuser.h:12621:9, winuser.h:12621:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_LCTLLATCHED: i32 = 0x4000000i32; /* Integer(67108864, Yes, Unknown) */ /* winuser.h:12622:9, winuser.h:12622:9, winuser.h:12622:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_LSHIFTLATCHED: i32 = 0x1000000i32; /* Integer(16777216, Yes, Unknown) */ /* winuser.h:12623:9, winuser.h:12623:9, winuser.h:12623:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_RALTLATCHED: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* winuser.h:12624:9, winuser.h:12624:9, winuser.h:12624:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_RCTLLATCHED: i32 = 0x8000000i32; /* Integer(134217728, Yes, Unknown) */ /* winuser.h:12625:9, winuser.h:12625:9, winuser.h:12625:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_RSHIFTLATCHED: i32 = 0x2000000i32; /* Integer(33554432, Yes, Unknown) */ /* winuser.h:12626:9, winuser.h:12626:9, winuser.h:12626:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_LWINLATCHED: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* winuser.h:12627:9, winuser.h:12627:9, winuser.h:12627:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_RWINLATCHED: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winuser.h:12628:9, winuser.h:12628:9, winuser.h:12628:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_LALTLOCKED: i32 = 0x100000i32; /* Integer(1048576, Yes, Unknown) */ /* winuser.h:12629:9, winuser.h:12629:9, winuser.h:12629:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_LCTLLOCKED: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* winuser.h:12630:9, winuser.h:12630:9, winuser.h:12630:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_LSHIFTLOCKED: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winuser.h:12631:9, winuser.h:12631:9, winuser.h:12631:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_RALTLOCKED: i32 = 0x200000i32; /* Integer(2097152, Yes, Unknown) */ /* winuser.h:12632:9, winuser.h:12632:9, winuser.h:12632:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_RCTLLOCKED: i32 = 0x80000i32; /* Integer(524288, Yes, Unknown) */ /* winuser.h:12633:9, winuser.h:12633:9, winuser.h:12633:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_RSHIFTLOCKED: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winuser.h:12634:9, winuser.h:12634:9, winuser.h:12634:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_LWINLOCKED: i32 = 0x400000i32; /* Integer(4194304, Yes, Unknown) */ /* winuser.h:12635:9, winuser.h:12635:9, winuser.h:12635:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SKF_RWINLOCKED: i32 = 0x800000i32; /* Integer(8388608, Yes, Unknown) */ /* winuser.h:12636:9, winuser.h:12636:9, winuser.h:12636:9 */
pub const MKF_MOUSEKEYSON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12659:9, winuser.h:12659:9, winuser.h:12659:9 */
pub const MKF_AVAILABLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12660:9, winuser.h:12660:9, winuser.h:12660:9 */
pub const MKF_HOTKEYACTIVE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:12661:9, winuser.h:12661:9, winuser.h:12661:9 */
pub const MKF_CONFIRMHOTKEY: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:12662:9, winuser.h:12662:9, winuser.h:12662:9 */
pub const MKF_HOTKEYSOUND: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:12663:9, winuser.h:12663:9, winuser.h:12663:9 */
pub const MKF_INDICATOR: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:12664:9, winuser.h:12664:9, winuser.h:12664:9 */
pub const MKF_MODIFIERS: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:12665:9, winuser.h:12665:9, winuser.h:12665:9 */
pub const MKF_REPLACENUMBERS: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:12666:9, winuser.h:12666:9, winuser.h:12666:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const MKF_LEFTBUTTONSEL: i32 = 0x10000000i32; /* Integer(268435456, Yes, Unknown) */ /* winuser.h:12668:9, winuser.h:12668:9, winuser.h:12668:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const MKF_RIGHTBUTTONSEL: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* winuser.h:12669:9, winuser.h:12669:9, winuser.h:12669:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const MKF_LEFTBUTTONDOWN: i32 = 0x1000000i32; /* Integer(16777216, Yes, Unknown) */ /* winuser.h:12670:9, winuser.h:12670:9, winuser.h:12670:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const MKF_RIGHTBUTTONDOWN: i32 = 0x2000000i32; /* Integer(33554432, Yes, Unknown) */ /* winuser.h:12671:9, winuser.h:12671:9, winuser.h:12671:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const MKF_MOUSEMODE: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winuser.h:12672:9, winuser.h:12672:9, winuser.h:12672:9 */
pub const ATF_TIMEOUTON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12691:9, winuser.h:12691:9, winuser.h:12691:9 */
pub const ATF_ONOFFFEEDBACK: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12692:9, winuser.h:12692:9, winuser.h:12692:9 */
pub const SSGF_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:12695:9, winuser.h:12695:9, winuser.h:12695:9 */
pub const SSGF_DISPLAY: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:12696:9, winuser.h:12696:9, winuser.h:12696:9 */
pub const SSTF_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:12699:9, winuser.h:12699:9, winuser.h:12699:9 */
pub const SSTF_CHARS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12700:9, winuser.h:12700:9, winuser.h:12700:9 */
pub const SSTF_BORDER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12701:9, winuser.h:12701:9, winuser.h:12701:9 */
pub const SSTF_DISPLAY: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:12702:9, winuser.h:12702:9, winuser.h:12702:9 */
pub const SSWF_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:12705:9, winuser.h:12705:9, winuser.h:12705:9 */
pub const SSWF_TITLE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12706:9, winuser.h:12706:9, winuser.h:12706:9 */
pub const SSWF_WINDOW: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12707:9, winuser.h:12707:9, winuser.h:12707:9 */
pub const SSWF_DISPLAY: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:12708:9, winuser.h:12708:9, winuser.h:12708:9 */
pub const SSWF_CUSTOM: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:12709:9, winuser.h:12709:9, winuser.h:12709:9 */
pub const SSF_SOUNDSENTRYON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12758:9, winuser.h:12758:9, winuser.h:12758:9 */
pub const SSF_AVAILABLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12759:9, winuser.h:12759:9, winuser.h:12759:9 */
pub const SSF_INDICATOR: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:12760:9, winuser.h:12760:9, winuser.h:12760:9 */
pub const TKF_TOGGLEKEYSON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12784:9, winuser.h:12784:9, winuser.h:12784:9 */
pub const TKF_AVAILABLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12785:9, winuser.h:12785:9, winuser.h:12785:9 */
pub const TKF_HOTKEYACTIVE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:12786:9, winuser.h:12786:9, winuser.h:12786:9 */
pub const TKF_CONFIRMHOTKEY: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:12787:9, winuser.h:12787:9, winuser.h:12787:9 */
pub const TKF_HOTKEYSOUND: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:12788:9, winuser.h:12788:9, winuser.h:12788:9 */
pub const TKF_INDICATOR: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:12789:9, winuser.h:12789:9, winuser.h:12789:9 */
pub const SLE_ERROR: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12820:9, winuser.h:12820:9, winuser.h:12820:9 */
pub const SLE_MINORERROR: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12821:9, winuser.h:12821:9, winuser.h:12821:9 */
pub const SLE_WARNING: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:12822:9, winuser.h:12822:9, winuser.h:12822:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const MONITOR_DEFAULTTONULL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:12869:9, winuser.h:12869:9, winuser.h:12869:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const MONITOR_DEFAULTTOPRIMARY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12870:9, winuser.h:12870:9, winuser.h:12870:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const MONITOR_DEFAULTTONEAREST: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:12871:9, winuser.h:12871:9, winuser.h:12871:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const MONITORINFOF_PRIMARY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:12900:9, winuser.h:12900:9, winuser.h:12900:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WINEVENT_OUTOFCONTEXT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:13038:9, winuser.h:13038:9, winuser.h:13038:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WINEVENT_SKIPOWNTHREAD: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:13039:9, winuser.h:13039:9, winuser.h:13039:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WINEVENT_SKIPOWNPROCESS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:13040:9, winuser.h:13040:9, winuser.h:13040:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const WINEVENT_INCONTEXT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:13041:9, winuser.h:13041:9, winuser.h:13041:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CHILDID_SELF: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:13074:13, winuser.h:13074:13, winuser.h:13074:13 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const INDEXID_OBJECT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:13075:13, winuser.h:13075:13, winuser.h:13075:13 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const INDEXID_CONTAINER: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:13076:13, winuser.h:13076:13, winuser.h:13076:13 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_MIN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:13099:9, winuser.h:13099:9, winuser.h:13099:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_MAX: i32 = 0x7fffffffi32; /* Integer(2147483647, Yes, Unknown) */ /* winuser.h:13100:9, winuser.h:13100:9, winuser.h:13100:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_SOUND: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:13123:9, winuser.h:13123:9, winuser.h:13123:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_ALERT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:13135:9, winuser.h:13135:9, winuser.h:13135:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_FOREGROUND: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:13145:9, winuser.h:13145:9, winuser.h:13145:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_MENUSTART: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:13161:9, winuser.h:13161:9, winuser.h:13161:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_MENUEND: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:13162:9, winuser.h:13162:9, winuser.h:13162:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_MENUPOPUPSTART: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:13176:9, winuser.h:13176:9, winuser.h:13176:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_MENUPOPUPEND: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:13177:9, winuser.h:13177:9, winuser.h:13177:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_CAPTURESTART: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:13185:9, winuser.h:13185:9, winuser.h:13185:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_CAPTUREEND: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:13186:9, winuser.h:13186:9, winuser.h:13186:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_MOVESIZESTART: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:13194:9, winuser.h:13194:9, winuser.h:13194:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_MOVESIZEEND: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:13195:9, winuser.h:13195:9, winuser.h:13195:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_CONTEXTHELPSTART: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winuser.h:13203:9, winuser.h:13203:9, winuser.h:13203:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_CONTEXTHELPEND: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winuser.h:13204:9, winuser.h:13204:9, winuser.h:13204:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_DRAGDROPSTART: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winuser.h:13216:9, winuser.h:13216:9, winuser.h:13216:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_DRAGDROPEND: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winuser.h:13217:9, winuser.h:13217:9, winuser.h:13217:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_DIALOGSTART: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:13227:9, winuser.h:13227:9, winuser.h:13227:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_DIALOGEND: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winuser.h:13228:9, winuser.h:13228:9, winuser.h:13228:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_SCROLLINGSTART: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winuser.h:13237:9, winuser.h:13237:9, winuser.h:13237:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_SCROLLINGEND: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winuser.h:13238:9, winuser.h:13238:9, winuser.h:13238:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_SWITCHSTART: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winuser.h:13247:9, winuser.h:13247:9, winuser.h:13247:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_SWITCHEND: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winuser.h:13248:9, winuser.h:13248:9, winuser.h:13248:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_MINIMIZESTART: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winuser.h:13255:9, winuser.h:13255:9, winuser.h:13255:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_SYSTEM_MINIMIZEEND: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winuser.h:13256:9, winuser.h:13256:9, winuser.h:13256:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const EVENT_SYSTEM_DESKTOPSWITCH: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:13260:9, winuser.h:13260:9, winuser.h:13260:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_SYSTEM_SWITCHER_APPGRABBED: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* winuser.h:13266:9, winuser.h:13266:9, winuser.h:13266:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_SYSTEM_SWITCHER_APPOVERTARGET: i32 = 0x25i32; /* Integer(37, Yes, Unknown) */ /* winuser.h:13273:9, winuser.h:13273:9, winuser.h:13273:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_SYSTEM_SWITCHER_APPDROPPED: i32 = 0x26i32; /* Integer(38, Yes, Unknown) */ /* winuser.h:13275:9, winuser.h:13275:9, winuser.h:13275:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_SYSTEM_SWITCHER_CANCELLED: i32 = 0x27i32; /* Integer(39, Yes, Unknown) */ /* winuser.h:13277:9, winuser.h:13277:9, winuser.h:13277:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_SYSTEM_IME_KEY_NOTIFICATION: i32 = 0x29i32; /* Integer(41, Yes, Unknown) */ /* winuser.h:13292:9, winuser.h:13292:9, winuser.h:13292:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const EVENT_SYSTEM_END: i32 = 0xffi32; /* Integer(255, Yes, Unknown) */ /* winuser.h:13298:9, winuser.h:13298:9, winuser.h:13298:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const EVENT_OEM_DEFINED_START: i32 = 0x101i32; /* Integer(257, Yes, Unknown) */ /* winuser.h:13300:9, winuser.h:13300:9, winuser.h:13300:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const EVENT_OEM_DEFINED_END: i32 = 0x1ffi32; /* Integer(511, Yes, Unknown) */ /* winuser.h:13301:9, winuser.h:13301:9, winuser.h:13301:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const EVENT_UIA_EVENTID_START: i32 = 0x4e00i32; /* Integer(19968, Yes, Unknown) */ /* winuser.h:13303:9, winuser.h:13303:9, winuser.h:13303:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const EVENT_UIA_EVENTID_END: i32 = 0x4effi32; /* Integer(20223, Yes, Unknown) */ /* winuser.h:13304:9, winuser.h:13304:9, winuser.h:13304:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const EVENT_UIA_PROPID_START: i32 = 0x7500i32; /* Integer(29952, Yes, Unknown) */ /* winuser.h:13306:9, winuser.h:13306:9, winuser.h:13306:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const EVENT_UIA_PROPID_END: i32 = 0x75ffi32; /* Integer(30207, Yes, Unknown) */ /* winuser.h:13307:9, winuser.h:13307:9, winuser.h:13307:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const EVENT_CONSOLE_CARET: i32 = 0x4001i32; /* Integer(16385, Yes, Unknown) */ /* winuser.h:13311:9, winuser.h:13311:9, winuser.h:13311:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const EVENT_CONSOLE_UPDATE_REGION: i32 = 0x4002i32; /* Integer(16386, Yes, Unknown) */ /* winuser.h:13312:9, winuser.h:13312:9, winuser.h:13312:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const EVENT_CONSOLE_UPDATE_SIMPLE: i32 = 0x4003i32; /* Integer(16387, Yes, Unknown) */ /* winuser.h:13313:9, winuser.h:13313:9, winuser.h:13313:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const EVENT_CONSOLE_UPDATE_SCROLL: i32 = 0x4004i32; /* Integer(16388, Yes, Unknown) */ /* winuser.h:13314:9, winuser.h:13314:9, winuser.h:13314:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const EVENT_CONSOLE_LAYOUT: i32 = 0x4005i32; /* Integer(16389, Yes, Unknown) */ /* winuser.h:13315:9, winuser.h:13315:9, winuser.h:13315:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const EVENT_CONSOLE_START_APPLICATION: i32 = 0x4006i32; /* Integer(16390, Yes, Unknown) */ /* winuser.h:13316:9, winuser.h:13316:9, winuser.h:13316:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const EVENT_CONSOLE_END_APPLICATION: i32 = 0x4007i32; /* Integer(16391, Yes, Unknown) */ /* winuser.h:13317:9, winuser.h:13317:9, winuser.h:13317:9 */
#[cfg(any(feature="winapi_ver_05010000"))] #[cfg(any(target_arch="x86", target_arch="arm"))] pub const CONSOLE_APPLICATION_16BIT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:13325:9, winuser.h:13325:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const CONSOLE_CARET_SELECTION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:13331:9, winuser.h:13331:9, winuser.h:13331:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const CONSOLE_CARET_VISIBLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:13332:9, winuser.h:13332:9, winuser.h:13332:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const EVENT_CONSOLE_END: i32 = 0x40ffi32; /* Integer(16639, Yes, Unknown) */ /* winuser.h:13336:9, winuser.h:13336:9, winuser.h:13336:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_CREATE: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:13417:9, winuser.h:13417:9, winuser.h:13417:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_DESTROY: i32 = 0x8001i32; /* Integer(32769, Yes, Unknown) */ /* winuser.h:13418:9, winuser.h:13418:9, winuser.h:13418:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_SHOW: i32 = 0x8002i32; /* Integer(32770, Yes, Unknown) */ /* winuser.h:13419:9, winuser.h:13419:9, winuser.h:13419:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_HIDE: i32 = 0x8003i32; /* Integer(32771, Yes, Unknown) */ /* winuser.h:13420:9, winuser.h:13420:9, winuser.h:13420:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_REORDER: i32 = 0x8004i32; /* Integer(32772, Yes, Unknown) */ /* winuser.h:13421:9, winuser.h:13421:9, winuser.h:13421:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_FOCUS: i32 = 0x8005i32; /* Integer(32773, Yes, Unknown) */ /* winuser.h:13433:9, winuser.h:13433:9, winuser.h:13433:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_SELECTION: i32 = 0x8006i32; /* Integer(32774, Yes, Unknown) */ /* winuser.h:13434:9, winuser.h:13434:9, winuser.h:13434:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_SELECTIONADD: i32 = 0x8007i32; /* Integer(32775, Yes, Unknown) */ /* winuser.h:13435:9, winuser.h:13435:9, winuser.h:13435:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_SELECTIONREMOVE: i32 = 0x8008i32; /* Integer(32776, Yes, Unknown) */ /* winuser.h:13436:9, winuser.h:13436:9, winuser.h:13436:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_SELECTIONWITHIN: i32 = 0x8009i32; /* Integer(32777, Yes, Unknown) */ /* winuser.h:13437:9, winuser.h:13437:9, winuser.h:13437:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_STATECHANGE: i32 = 0x800ai32; /* Integer(32778, Yes, Unknown) */ /* winuser.h:13475:9, winuser.h:13475:9, winuser.h:13475:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_LOCATIONCHANGE: i32 = 0x800bi32; /* Integer(32779, Yes, Unknown) */ /* winuser.h:13482:9, winuser.h:13482:9, winuser.h:13482:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_NAMECHANGE: i32 = 0x800ci32; /* Integer(32780, Yes, Unknown) */ /* winuser.h:13507:9, winuser.h:13507:9, winuser.h:13507:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_DESCRIPTIONCHANGE: i32 = 0x800di32; /* Integer(32781, Yes, Unknown) */ /* winuser.h:13508:9, winuser.h:13508:9, winuser.h:13508:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_VALUECHANGE: i32 = 0x800ei32; /* Integer(32782, Yes, Unknown) */ /* winuser.h:13509:9, winuser.h:13509:9, winuser.h:13509:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_PARENTCHANGE: i32 = 0x800fi32; /* Integer(32783, Yes, Unknown) */ /* winuser.h:13510:9, winuser.h:13510:9, winuser.h:13510:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_HELPCHANGE: i32 = 0x8010i32; /* Integer(32784, Yes, Unknown) */ /* winuser.h:13511:9, winuser.h:13511:9, winuser.h:13511:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_DEFACTIONCHANGE: i32 = 0x8011i32; /* Integer(32785, Yes, Unknown) */ /* winuser.h:13512:9, winuser.h:13512:9, winuser.h:13512:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const EVENT_OBJECT_ACCELERATORCHANGE: i32 = 0x8012i32; /* Integer(32786, Yes, Unknown) */ /* winuser.h:13513:9, winuser.h:13513:9, winuser.h:13513:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const EVENT_OBJECT_INVOKED: i32 = 0x8013i32; /* Integer(32787, Yes, Unknown) */ /* winuser.h:13516:9, winuser.h:13516:9, winuser.h:13516:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const EVENT_OBJECT_TEXTSELECTIONCHANGED: i32 = 0x8014i32; /* Integer(32788, Yes, Unknown) */ /* winuser.h:13517:9, winuser.h:13517:9, winuser.h:13517:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const EVENT_OBJECT_CONTENTSCROLLED: i32 = 0x8015i32; /* Integer(32789, Yes, Unknown) */ /* winuser.h:13535:9, winuser.h:13535:9, winuser.h:13535:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const EVENT_SYSTEM_ARRANGMENTPREVIEW: i32 = 0x8016i32; /* Integer(32790, Yes, Unknown) */ /* winuser.h:13539:9, winuser.h:13539:9, winuser.h:13539:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_OBJECT_CLOAKED: i32 = 0x8017i32; /* Integer(32791, Yes, Unknown) */ /* winuser.h:13550:9, winuser.h:13550:9, winuser.h:13550:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_OBJECT_UNCLOAKED: i32 = 0x8018i32; /* Integer(32792, Yes, Unknown) */ /* winuser.h:13551:9, winuser.h:13551:9, winuser.h:13551:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_OBJECT_LIVEREGIONCHANGED: i32 = 0x8019i32; /* Integer(32793, Yes, Unknown) */ /* winuser.h:13561:9, winuser.h:13561:9, winuser.h:13561:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_OBJECT_HOSTEDOBJECTSINVALIDATED: i32 = 0x8020i32; /* Integer(32800, Yes, Unknown) */ /* winuser.h:13577:9, winuser.h:13577:9, winuser.h:13577:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_OBJECT_DRAGSTART: i32 = 0x8021i32; /* Integer(32801, Yes, Unknown) */ /* winuser.h:13592:9, winuser.h:13592:9, winuser.h:13592:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_OBJECT_DRAGCANCEL: i32 = 0x8022i32; /* Integer(32802, Yes, Unknown) */ /* winuser.h:13593:9, winuser.h:13593:9, winuser.h:13593:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_OBJECT_DRAGCOMPLETE: i32 = 0x8023i32; /* Integer(32803, Yes, Unknown) */ /* winuser.h:13594:9, winuser.h:13594:9, winuser.h:13594:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_OBJECT_DRAGENTER: i32 = 0x8024i32; /* Integer(32804, Yes, Unknown) */ /* winuser.h:13596:9, winuser.h:13596:9, winuser.h:13596:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_OBJECT_DRAGLEAVE: i32 = 0x8025i32; /* Integer(32805, Yes, Unknown) */ /* winuser.h:13597:9, winuser.h:13597:9, winuser.h:13597:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_OBJECT_DRAGDROPPED: i32 = 0x8026i32; /* Integer(32806, Yes, Unknown) */ /* winuser.h:13598:9, winuser.h:13598:9, winuser.h:13598:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_OBJECT_IME_SHOW: i32 = 0x8027i32; /* Integer(32807, Yes, Unknown) */ /* winuser.h:13604:9, winuser.h:13604:9, winuser.h:13604:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_OBJECT_IME_HIDE: i32 = 0x8028i32; /* Integer(32808, Yes, Unknown) */ /* winuser.h:13605:9, winuser.h:13605:9, winuser.h:13605:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_OBJECT_IME_CHANGE: i32 = 0x8029i32; /* Integer(32809, Yes, Unknown) */ /* winuser.h:13611:9, winuser.h:13611:9, winuser.h:13611:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const EVENT_OBJECT_TEXTEDIT_CONVERSIONTARGETCHANGED: i32 = 0x8030i32; /* Integer(32816, Yes, Unknown) */ /* winuser.h:13613:9, winuser.h:13613:9, winuser.h:13613:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const EVENT_OBJECT_END: i32 = 0x80ffi32; /* Integer(33023, Yes, Unknown) */ /* winuser.h:13618:9, winuser.h:13618:9, winuser.h:13618:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const EVENT_AIA_START: i32 = 0xa000i32; /* Integer(40960, Yes, Unknown) */ /* winuser.h:13620:9, winuser.h:13620:9, winuser.h:13620:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const EVENT_AIA_END: i32 = 0xafffi32; /* Integer(45055, Yes, Unknown) */ /* winuser.h:13621:9, winuser.h:13621:9, winuser.h:13621:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_STARTUP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:13633:9, winuser.h:13633:9, winuser.h:13633:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_SHUTDOWN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:13634:9, winuser.h:13634:9, winuser.h:13634:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_BEEP: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:13635:9, winuser.h:13635:9, winuser.h:13635:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_ERROR: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:13636:9, winuser.h:13636:9, winuser.h:13636:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_QUESTION: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:13637:9, winuser.h:13637:9, winuser.h:13637:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_WARNING: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:13638:9, winuser.h:13638:9, winuser.h:13638:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_INFORMATION: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:13639:9, winuser.h:13639:9, winuser.h:13639:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_MAXIMIZE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:13640:9, winuser.h:13640:9, winuser.h:13640:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_MINIMIZE: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winuser.h:13641:9, winuser.h:13641:9, winuser.h:13641:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_RESTOREUP: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winuser.h:13642:9, winuser.h:13642:9, winuser.h:13642:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_RESTOREDOWN: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winuser.h:13643:9, winuser.h:13643:9, winuser.h:13643:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_APPSTART: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winuser.h:13644:9, winuser.h:13644:9, winuser.h:13644:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_FAULT: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winuser.h:13645:9, winuser.h:13645:9, winuser.h:13645:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_APPEND: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winuser.h:13646:9, winuser.h:13646:9, winuser.h:13646:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_MENUCOMMAND: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winuser.h:13647:9, winuser.h:13647:9, winuser.h:13647:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SOUND_SYSTEM_MENUPOPUP: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:13648:9, winuser.h:13648:9, winuser.h:13648:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CSOUND_SYSTEM: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:13649:9, winuser.h:13649:9, winuser.h:13649:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ALERT_SYSTEM_INFORMATIONAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:13654:9, winuser.h:13654:9, winuser.h:13654:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ALERT_SYSTEM_WARNING: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:13655:9, winuser.h:13655:9, winuser.h:13655:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ALERT_SYSTEM_ERROR: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:13656:9, winuser.h:13656:9, winuser.h:13656:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ALERT_SYSTEM_QUERY: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:13657:9, winuser.h:13657:9, winuser.h:13657:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ALERT_SYSTEM_CRITICAL: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:13658:9, winuser.h:13658:9, winuser.h:13658:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CALERT_SYSTEM: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:13659:9, winuser.h:13659:9, winuser.h:13659:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const GUI_CARETBLINKING: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:13680:9, winuser.h:13680:9, winuser.h:13680:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const GUI_INMOVESIZE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:13681:9, winuser.h:13681:9, winuser.h:13681:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const GUI_INMENUMODE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:13682:9, winuser.h:13682:9, winuser.h:13682:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const GUI_SYSTEMMENUMODE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:13683:9, winuser.h:13683:9, winuser.h:13683:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const GUI_POPUPMENUMODE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:13684:9, winuser.h:13684:9, winuser.h:13684:9 */
#[cfg(any(feature="winapi_ver_05010000"))] #[cfg(any(target_arch="x86", target_arch="arm"))] pub const GUI_16BITTASK: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:13689:9, winuser.h:13689:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] pub const USER_DEFAULT_SCREEN_DPI: i32 = 0x60i32; /* Integer(96, Yes, Unknown) */ /* winuser.h:13711:9, winuser.h:13711:9, winuser.h:13711:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_UNAVAILABLE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:13751:9, winuser.h:13751:9, winuser.h:13751:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_SELECTED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:13752:9, winuser.h:13752:9, winuser.h:13752:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_FOCUSED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:13753:9, winuser.h:13753:9, winuser.h:13753:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_PRESSED: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:13754:9, winuser.h:13754:9, winuser.h:13754:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_CHECKED: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:13755:9, winuser.h:13755:9, winuser.h:13755:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_MIXED: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:13756:9, winuser.h:13756:9, winuser.h:13756:9 */
#[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use ::winuser::STATE_SYSTEM_MIXED as STATE_SYSTEM_INDETERMINATE; /* winuser.h:13757:9, winuser.h:13757:9, winuser.h:13757:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_READONLY: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:13758:9, winuser.h:13758:9, winuser.h:13758:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_HOTTRACKED: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:13759:9, winuser.h:13759:9, winuser.h:13759:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_DEFAULT: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:13760:9, winuser.h:13760:9, winuser.h:13760:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_EXPANDED: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:13761:9, winuser.h:13761:9, winuser.h:13761:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_COLLAPSED: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winuser.h:13762:9, winuser.h:13762:9, winuser.h:13762:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_BUSY: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winuser.h:13763:9, winuser.h:13763:9, winuser.h:13763:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_FLOATING: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:13764:9, winuser.h:13764:9, winuser.h:13764:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_MARQUEED: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:13765:9, winuser.h:13765:9, winuser.h:13765:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_ANIMATED: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winuser.h:13766:9, winuser.h:13766:9, winuser.h:13766:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_INVISIBLE: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winuser.h:13767:9, winuser.h:13767:9, winuser.h:13767:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_OFFSCREEN: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winuser.h:13768:9, winuser.h:13768:9, winuser.h:13768:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_SIZEABLE: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winuser.h:13769:9, winuser.h:13769:9, winuser.h:13769:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_MOVEABLE: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* winuser.h:13770:9, winuser.h:13770:9, winuser.h:13770:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_SELFVOICING: i32 = 0x80000i32; /* Integer(524288, Yes, Unknown) */ /* winuser.h:13771:9, winuser.h:13771:9, winuser.h:13771:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_FOCUSABLE: i32 = 0x100000i32; /* Integer(1048576, Yes, Unknown) */ /* winuser.h:13772:9, winuser.h:13772:9, winuser.h:13772:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_SELECTABLE: i32 = 0x200000i32; /* Integer(2097152, Yes, Unknown) */ /* winuser.h:13773:9, winuser.h:13773:9, winuser.h:13773:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_LINKED: i32 = 0x400000i32; /* Integer(4194304, Yes, Unknown) */ /* winuser.h:13774:9, winuser.h:13774:9, winuser.h:13774:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_TRAVERSED: i32 = 0x800000i32; /* Integer(8388608, Yes, Unknown) */ /* winuser.h:13775:9, winuser.h:13775:9, winuser.h:13775:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_MULTISELECTABLE: i32 = 0x1000000i32; /* Integer(16777216, Yes, Unknown) */ /* winuser.h:13776:9, winuser.h:13776:9, winuser.h:13776:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_EXTSELECTABLE: i32 = 0x2000000i32; /* Integer(33554432, Yes, Unknown) */ /* winuser.h:13777:9, winuser.h:13777:9, winuser.h:13777:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_ALERT_LOW: i32 = 0x4000000i32; /* Integer(67108864, Yes, Unknown) */ /* winuser.h:13778:9, winuser.h:13778:9, winuser.h:13778:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_ALERT_MEDIUM: i32 = 0x8000000i32; /* Integer(134217728, Yes, Unknown) */ /* winuser.h:13779:9, winuser.h:13779:9, winuser.h:13779:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_ALERT_HIGH: i32 = 0x10000000i32; /* Integer(268435456, Yes, Unknown) */ /* winuser.h:13780:9, winuser.h:13780:9, winuser.h:13780:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_PROTECTED: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* winuser.h:13781:9, winuser.h:13781:9, winuser.h:13781:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STATE_SYSTEM_VALID: i32 = 0x3fffffffi32; /* Integer(1073741823, Yes, Unknown) */ /* winuser.h:13782:9, winuser.h:13782:9, winuser.h:13782:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CCHILDREN_TITLEBAR: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:13785:9, winuser.h:13785:9, winuser.h:13785:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CCHILDREN_SCROLLBAR: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:13786:9, winuser.h:13786:9, winuser.h:13786:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const CURSOR_SHOWING: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:13802:9, winuser.h:13802:9, winuser.h:13802:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] pub const CURSOR_SUPPRESSED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:13804:9, winuser.h:13804:9, winuser.h:13804:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const WS_ACTIVECAPTION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:13830:9, winuser.h:13830:9, winuser.h:13830:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const GA_PARENT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:13937:13, winuser.h:13937:13, winuser.h:13937:13 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const GA_ROOT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:13938:13, winuser.h:13938:13, winuser.h:13938:13 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const GA_ROOTOWNER: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:13939:13, winuser.h:13939:13, winuser.h:13939:13 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIM_INPUT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:14105:9, winuser.h:14105:9, winuser.h:14105:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIM_INPUTSINK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14111:9, winuser.h:14111:9, winuser.h:14111:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIM_TYPEMOUSE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:14132:9, winuser.h:14132:9, winuser.h:14132:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIM_TYPEKEYBOARD: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14133:9, winuser.h:14133:9, winuser.h:14133:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIM_TYPEHID: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:14134:9, winuser.h:14134:9, winuser.h:14134:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_MOUSE_LEFT_BUTTON_DOWN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14189:9, winuser.h:14189:9, winuser.h:14189:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_MOUSE_LEFT_BUTTON_UP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:14190:9, winuser.h:14190:9, winuser.h:14190:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_MOUSE_RIGHT_BUTTON_DOWN: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:14191:9, winuser.h:14191:9, winuser.h:14191:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_MOUSE_RIGHT_BUTTON_UP: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:14192:9, winuser.h:14192:9, winuser.h:14192:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_MOUSE_MIDDLE_BUTTON_DOWN: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:14193:9, winuser.h:14193:9, winuser.h:14193:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_MOUSE_MIDDLE_BUTTON_UP: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:14194:9, winuser.h:14194:9, winuser.h:14194:9 */
#[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use ::winuser::RI_MOUSE_LEFT_BUTTON_DOWN as RI_MOUSE_BUTTON_1_DOWN; /* winuser.h:14196:9, winuser.h:14196:9, winuser.h:14196:9 */
#[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use ::winuser::RI_MOUSE_LEFT_BUTTON_UP as RI_MOUSE_BUTTON_1_UP; /* winuser.h:14197:9, winuser.h:14197:9, winuser.h:14197:9 */
#[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use ::winuser::RI_MOUSE_RIGHT_BUTTON_DOWN as RI_MOUSE_BUTTON_2_DOWN; /* winuser.h:14198:9, winuser.h:14198:9, winuser.h:14198:9 */
#[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use ::winuser::RI_MOUSE_RIGHT_BUTTON_UP as RI_MOUSE_BUTTON_2_UP; /* winuser.h:14199:9, winuser.h:14199:9, winuser.h:14199:9 */
#[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use ::winuser::RI_MOUSE_MIDDLE_BUTTON_DOWN as RI_MOUSE_BUTTON_3_DOWN; /* winuser.h:14200:9, winuser.h:14200:9, winuser.h:14200:9 */
#[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use ::winuser::RI_MOUSE_MIDDLE_BUTTON_UP as RI_MOUSE_BUTTON_3_UP; /* winuser.h:14201:9, winuser.h:14201:9, winuser.h:14201:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_MOUSE_BUTTON_4_DOWN: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:14203:9, winuser.h:14203:9, winuser.h:14203:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_MOUSE_BUTTON_4_UP: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:14204:9, winuser.h:14204:9, winuser.h:14204:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_MOUSE_BUTTON_5_DOWN: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:14205:9, winuser.h:14205:9, winuser.h:14205:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_MOUSE_BUTTON_5_UP: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:14206:9, winuser.h:14206:9, winuser.h:14206:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_MOUSE_WHEEL: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winuser.h:14212:9, winuser.h:14212:9, winuser.h:14212:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const MOUSE_MOVE_RELATIVE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:14217:9, winuser.h:14217:9, winuser.h:14217:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const MOUSE_MOVE_ABSOLUTE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14218:9, winuser.h:14218:9, winuser.h:14218:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const MOUSE_VIRTUAL_DESKTOP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:14219:9, winuser.h:14219:9, winuser.h:14219:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const MOUSE_ATTRIBUTES_CHANGED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:14220:9, winuser.h:14220:9, winuser.h:14220:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const MOUSE_MOVE_NOCOALESCE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:14222:9, winuser.h:14222:9, winuser.h:14222:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const KEYBOARD_OVERRUN_MAKE_CODE: i32 = 0xffi32; /* Integer(255, Yes, Unknown) */ /* winuser.h:14266:9, winuser.h:14266:9, winuser.h:14266:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_KEY_MAKE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:14271:9, winuser.h:14271:9, winuser.h:14271:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_KEY_BREAK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14272:9, winuser.h:14272:9, winuser.h:14272:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_KEY_E0: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:14273:9, winuser.h:14273:9, winuser.h:14273:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_KEY_E1: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:14274:9, winuser.h:14274:9, winuser.h:14274:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_KEY_TERMSRV_SET_LED: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:14275:9, winuser.h:14275:9, winuser.h:14275:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RI_KEY_TERMSRV_SHADOW: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:14276:9, winuser.h:14276:9, winuser.h:14276:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RID_INPUT: i32 = 0x10000003i32; /* Integer(268435459, Yes, Unknown) */ /* winuser.h:14324:9, winuser.h:14324:9, winuser.h:14324:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RID_HEADER: i32 = 0x10000005i32; /* Integer(268435461, Yes, Unknown) */ /* winuser.h:14325:9, winuser.h:14325:9, winuser.h:14325:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIDI_PREPARSEDDATA: i32 = 0x20000005i32; /* Integer(536870917, Yes, Unknown) */ /* winuser.h:14346:9, winuser.h:14346:9, winuser.h:14346:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIDI_DEVICENAME: i32 = 0x20000007i32; /* Integer(536870919, Yes, Unknown) */ /* winuser.h:14347:9, winuser.h:14347:9, winuser.h:14347:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIDI_DEVICEINFO: i32 = 0x2000000bi32; /* Integer(536870923, Yes, Unknown) */ /* winuser.h:14348:9, winuser.h:14348:9, winuser.h:14348:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIDEV_REMOVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14440:9, winuser.h:14440:9, winuser.h:14440:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIDEV_EXCLUDE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:14441:9, winuser.h:14441:9, winuser.h:14441:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIDEV_PAGEONLY: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:14442:9, winuser.h:14442:9, winuser.h:14442:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIDEV_NOLEGACY: i32 = 0x30i32; /* Integer(48, Yes, Unknown) */ /* winuser.h:14443:9, winuser.h:14443:9, winuser.h:14443:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIDEV_INPUTSINK: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:14444:9, winuser.h:14444:9, winuser.h:14444:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIDEV_CAPTUREMOUSE: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:14445:9, winuser.h:14445:9, winuser.h:14445:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIDEV_NOHOTKEYS: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:14446:9, winuser.h:14446:9, winuser.h:14446:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIDEV_APPKEYS: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winuser.h:14447:9, winuser.h:14447:9, winuser.h:14447:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIDEV_EXINPUTSINK: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winuser.h:14449:9, winuser.h:14449:9, winuser.h:14449:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIDEV_DEVNOTIFY: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winuser.h:14450:9, winuser.h:14450:9, winuser.h:14450:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const RIDEV_EXMODEMASK: i32 = 0xf0i32; /* Integer(240, Yes, Unknown) */ /* winuser.h:14452:9, winuser.h:14452:9, winuser.h:14452:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const GIDC_ARRIVAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14460:9, winuser.h:14460:9, winuser.h:14460:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const GIDC_REMOVAL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:14461:9, winuser.h:14461:9, winuser.h:14461:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const POINTER_DEVICE_PRODUCT_STRING_MAX: i32 = 0x208i32; /* Integer(520, Yes, Unknown) */ /* winuser.h:14519:9, winuser.h:14519:9, winuser.h:14519:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const PDC_ARRIVAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14523:9, winuser.h:14523:9, winuser.h:14523:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const PDC_REMOVAL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:14524:9, winuser.h:14524:9, winuser.h:14524:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const PDC_ORIENTATION_0: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:14525:9, winuser.h:14525:9, winuser.h:14525:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const PDC_ORIENTATION_90: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:14526:9, winuser.h:14526:9, winuser.h:14526:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const PDC_ORIENTATION_180: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:14527:9, winuser.h:14527:9, winuser.h:14527:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const PDC_ORIENTATION_270: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winuser.h:14528:9, winuser.h:14528:9, winuser.h:14528:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const PDC_MODE_DEFAULT: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:14529:9, winuser.h:14529:9, winuser.h:14529:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const PDC_MODE_CENTERED: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:14530:9, winuser.h:14530:9, winuser.h:14530:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const PDC_MAPPING_CHANGE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:14531:9, winuser.h:14531:9, winuser.h:14531:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const PDC_RESOLUTION: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winuser.h:14532:9, winuser.h:14532:9, winuser.h:14532:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const PDC_ORIGIN: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winuser.h:14533:9, winuser.h:14533:9, winuser.h:14533:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const PDC_MODE_ASPECTRATIOPRESERVED: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winuser.h:14534:9, winuser.h:14534:9, winuser.h:14534:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const MSGFLT_ADD: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14650:9, winuser.h:14650:9, winuser.h:14650:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const MSGFLT_REMOVE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:14651:9, winuser.h:14651:9, winuser.h:14651:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GF_BEGIN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14736:9, winuser.h:14736:9, winuser.h:14736:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GF_INERTIA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:14737:9, winuser.h:14737:9, winuser.h:14737:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GF_END: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:14738:9, winuser.h:14738:9, winuser.h:14738:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GID_BEGIN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14743:9, winuser.h:14743:9, winuser.h:14743:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GID_END: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:14744:9, winuser.h:14744:9, winuser.h:14744:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GID_ZOOM: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winuser.h:14745:9, winuser.h:14745:9, winuser.h:14745:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GID_PAN: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:14746:9, winuser.h:14746:9, winuser.h:14746:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GID_ROTATE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winuser.h:14747:9, winuser.h:14747:9, winuser.h:14747:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GID_TWOFINGERTAP: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winuser.h:14748:9, winuser.h:14748:9, winuser.h:14748:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GID_PRESSANDTAP: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winuser.h:14749:9, winuser.h:14749:9, winuser.h:14749:9 */
#[cfg(any(feature="winapi_ver_06010000"))] #[doc(inline)] pub use ::winuser::GID_PRESSANDTAP as GID_ROLLOVER; /* winuser.h:14750:9, winuser.h:14750:9, winuser.h:14750:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GC_ALLGESTURES: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14865:9, winuser.h:14865:9, winuser.h:14865:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GC_ZOOM: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14870:9, winuser.h:14870:9, winuser.h:14870:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GC_PAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14875:9, winuser.h:14875:9, winuser.h:14875:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GC_PAN_WITH_SINGLE_FINGER_VERTICALLY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:14876:9, winuser.h:14876:9, winuser.h:14876:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GC_PAN_WITH_SINGLE_FINGER_HORIZONTALLY: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:14877:9, winuser.h:14877:9, winuser.h:14877:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GC_PAN_WITH_GUTTER: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:14878:9, winuser.h:14878:9, winuser.h:14878:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GC_PAN_WITH_INERTIA: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winuser.h:14879:9, winuser.h:14879:9, winuser.h:14879:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GC_ROTATE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14884:9, winuser.h:14884:9, winuser.h:14884:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GC_TWOFINGERTAP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14889:9, winuser.h:14889:9, winuser.h:14889:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GC_PRESSANDTAP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14894:9, winuser.h:14894:9, winuser.h:14894:9 */
#[cfg(any(feature="winapi_ver_06010000"))] #[doc(inline)] pub use ::winuser::GC_PRESSANDTAP as GC_ROLLOVER; /* winuser.h:14895:9, winuser.h:14895:9, winuser.h:14895:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const GESTURECONFIGMAXCOUNT: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:14897:9, winuser.h:14897:9, winuser.h:14897:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub const GCF_INCLUDE_ANCESTORS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14915:9, winuser.h:14915:9, winuser.h:14915:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const NID_INTEGRATED_TOUCH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winuser.h:14943:9, winuser.h:14943:9, winuser.h:14943:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const NID_EXTERNAL_TOUCH: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winuser.h:14944:9, winuser.h:14944:9, winuser.h:14944:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const NID_INTEGRATED_PEN: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winuser.h:14945:9, winuser.h:14945:9, winuser.h:14945:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const NID_EXTERNAL_PEN: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winuser.h:14946:9, winuser.h:14946:9, winuser.h:14946:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const NID_MULTI_INPUT: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winuser.h:14947:9, winuser.h:14947:9, winuser.h:14947:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const NID_READY: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winuser.h:14948:9, winuser.h:14948:9, winuser.h:14948:9 */
pub const MAX_STR_BLOCKREASON: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winuser.h:14953:9, winuser.h:14953:9, winuser.h:14953:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type WNDPROC = extern "system" fn(*mut ::windef::HWND__, ::libc::c_uint, ::libc::c_ulonglong, ::libc::c_longlong) -> ::libc::c_longlong; /* winuser.h:78:28 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] pub type DLGPROC = extern "system" fn(*mut ::windef::HWND__, ::libc::c_uint, ::libc::c_ulonglong, ::libc::c_longlong) -> ::libc::c_longlong; /* winuser.h:88:28 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type TIMERPROC = extern "system" fn(*mut ::windef::HWND__, ::libc::c_uint, ::libc::c_ulonglong, ::libc::c_ulong); /* winuser.h:96:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type GRAYSTRINGPROC = extern "system" fn(*mut ::windef::HDC__, ::libc::c_longlong, ::libc::c_int) -> ::libc::c_int; /* winuser.h:97:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type WNDENUMPROC = extern "system" fn(*mut ::windef::HWND__, ::libc::c_longlong) -> ::libc::c_int; /* winuser.h:98:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type HOOKPROC = extern "system" fn(::libc::c_int, ::libc::c_ulonglong, ::libc::c_longlong) -> ::libc::c_longlong; /* winuser.h:99:28 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type SENDASYNCPROC = extern "system" fn(*mut ::windef::HWND__, ::libc::c_uint, ::libc::c_ulonglong, ::libc::c_longlong); /* winuser.h:100:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type PROPENUMPROCEXA = extern "system" fn(*mut ::windef::HWND__, *mut ::libc::c_schar, *mut ::libc::c_void, ::libc::c_ulonglong) -> ::libc::c_int; /* winuser.h:105:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type PROPENUMPROCEXW = extern "system" fn(*mut ::windef::HWND__, *mut ::libc::c_ushort, *mut ::libc::c_void, ::libc::c_ulonglong) -> ::libc::c_int; /* winuser.h:106:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[cfg(any(target_arch="x86_64"))] pub type DRAWSTATEPROC = extern "system" fn(*mut ::windef::HDC__, ::libc::c_longlong, ::libc::c_ulonglong, ::libc::c_int, ::libc::c_int) -> ::libc::c_int; /* winuser.h:112:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type NAMEENUMPROCA = extern "system" fn(*mut ::libc::c_schar, ::libc::c_longlong) -> ::libc::c_int; /* winuser.h:172:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type NAMEENUMPROCW = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_longlong) -> ::libc::c_int; /* winuser.h:173:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[cfg(any(target_arch="x86_64"))] pub type MONITORENUMPROC = extern "system" fn(*mut ::windef::HMONITOR__, *mut ::windef::HDC__, *mut ::windef::RECT, ::libc::c_longlong) -> ::libc::c_int; /* winuser.h:12971:25 */
#[cfg(any(feature="winapi_ver_05010000"))] #[cfg(any(target_arch="x86_64"))] pub const CONSOLE_APPLICATION_16BIT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:13323:9 */
#[cfg(any(feature="winapi_ver_05010000"))] #[cfg(any(target_arch="x86_64"))] pub const GUI_16BITTASK: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winuser.h:13687:9 */
