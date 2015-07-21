#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct HIMC__ { unused: ::libc::c_int } /* imm.h:26:1, imm.h:26:1, imm.h:26:1 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub type HIMC = *mut ::imm::HIMC__; /* imm.h:26:1, imm.h:26:1, imm.h:26:1 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct HIMCC__ { unused: ::libc::c_int } /* imm.h:27:1, imm.h:27:1, imm.h:27:1 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub type HIMCC = *mut ::imm::HIMCC__; /* imm.h:27:1, imm.h:27:1, imm.h:27:1 */
#[cfg(feature="winapi_desktop")] pub type LPHKL = *mut *mut ::minwindef::HKL__; /* imm.h:39:19, imm.h:39:19, imm.h:39:19 */
#[cfg(feature="winapi_desktop")] pub type LPUINT = *mut ::libc::c_uint; /* imm.h:40:19, imm.h:40:19, imm.h:40:19 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct COMPOSITIONFORM { dwStyle: ::minwindef::DWORD, ptCurrentPos: ::windef::POINT, rcArea: ::windef::RECT } /* imm.h:48:16, imm.h:48:16, imm.h:48:16 */
#[cfg(feature="winapi_app")] pub type PCOMPOSITIONFORM = *mut ::imm::COMPOSITIONFORM; /* imm.h:52:21, imm.h:52:21, imm.h:52:21 */
#[cfg(feature="winapi_app")] pub type NPCOMPOSITIONFORM = *mut ::imm::COMPOSITIONFORM; /* imm.h:52:45, imm.h:52:45, imm.h:52:45 */
#[cfg(feature="winapi_app")] pub type LPCOMPOSITIONFORM = *mut ::imm::COMPOSITIONFORM; /* imm.h:52:69, imm.h:52:69, imm.h:52:69 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct CANDIDATEFORM { dwIndex: ::minwindef::DWORD, dwStyle: ::minwindef::DWORD, ptCurrentPos: ::windef::POINT, rcArea: ::windef::RECT } /* imm.h:55:16, imm.h:55:16, imm.h:55:16 */
#[cfg(feature="winapi_app")] pub type PCANDIDATEFORM = *mut ::imm::CANDIDATEFORM; /* imm.h:60:19, imm.h:60:19, imm.h:60:19 */
#[cfg(feature="winapi_app")] pub type NPCANDIDATEFORM = *mut ::imm::CANDIDATEFORM; /* imm.h:60:41, imm.h:60:41, imm.h:60:41 */
#[cfg(feature="winapi_app")] pub type LPCANDIDATEFORM = *mut ::imm::CANDIDATEFORM; /* imm.h:60:63, imm.h:60:63, imm.h:60:63 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct CANDIDATELIST { dwSize: ::minwindef::DWORD, dwStyle: ::minwindef::DWORD, dwCount: ::minwindef::DWORD, dwSelection: ::minwindef::DWORD, dwPageStart: ::minwindef::DWORD, dwPageSize: ::minwindef::DWORD, dwOffset: *mut [::minwindef::DWORD; 1] } /* imm.h:70:16, imm.h:70:16, imm.h:70:16 */
#[cfg(feature="winapi_desktop")] pub type PCANDIDATELIST = *mut ::imm::CANDIDATELIST; /* imm.h:78:19, imm.h:78:19, imm.h:78:19 */
#[cfg(feature="winapi_desktop")] pub type NPCANDIDATELIST = *mut ::imm::CANDIDATELIST; /* imm.h:78:41, imm.h:78:41, imm.h:78:41 */
#[cfg(feature="winapi_desktop")] pub type LPCANDIDATELIST = *mut ::imm::CANDIDATELIST; /* imm.h:78:63, imm.h:78:63, imm.h:78:63 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct REGISTERWORDA { lpReading: ::winnt::LPSTR, lpWord: ::winnt::LPSTR } /* imm.h:87:16, imm.h:87:16, imm.h:87:16 */
#[cfg(feature="winapi_app")] pub type PREGISTERWORDA = *mut ::imm::REGISTERWORDA; /* imm.h:90:19, imm.h:90:19, imm.h:90:19 */
#[cfg(feature="winapi_app")] pub type NPREGISTERWORDA = *mut ::imm::REGISTERWORDA; /* imm.h:90:41, imm.h:90:41, imm.h:90:41 */
#[cfg(feature="winapi_app")] pub type LPREGISTERWORDA = *mut ::imm::REGISTERWORDA; /* imm.h:90:63, imm.h:90:63, imm.h:90:63 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct REGISTERWORDW { lpReading: ::winnt::LPWSTR, lpWord: ::winnt::LPWSTR } /* imm.h:91:16, imm.h:91:16, imm.h:91:16 */
#[cfg(feature="winapi_app")] pub type PREGISTERWORDW = *mut ::imm::REGISTERWORDW; /* imm.h:94:19, imm.h:94:19, imm.h:94:19 */
#[cfg(feature="winapi_app")] pub type NPREGISTERWORDW = *mut ::imm::REGISTERWORDW; /* imm.h:94:41, imm.h:94:41, imm.h:94:41 */
#[cfg(feature="winapi_app")] pub type LPREGISTERWORDW = *mut ::imm::REGISTERWORDW; /* imm.h:94:63, imm.h:94:63, imm.h:94:63 */
#[cfg(feature="winapi_app")] pub type REGISTERWORD = ::imm::REGISTERWORDW; /* imm.h:96:23, imm.h:96:23, imm.h:96:23 */
#[cfg(feature="winapi_app")] pub type PREGISTERWORD = ::imm::PREGISTERWORDW; /* imm.h:97:24, imm.h:97:24, imm.h:97:24 */
#[cfg(feature="winapi_app")] pub type NPREGISTERWORD = ::imm::NPREGISTERWORDW; /* imm.h:98:25, imm.h:98:25, imm.h:98:25 */
#[cfg(feature="winapi_app")] pub type LPREGISTERWORD = ::imm::LPREGISTERWORDW; /* imm.h:99:25, imm.h:99:25, imm.h:99:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct RECONVERTSTRING { dwSize: ::minwindef::DWORD, dwVersion: ::minwindef::DWORD, dwStrLen: ::minwindef::DWORD, dwStrOffset: ::minwindef::DWORD, dwCompStrLen: ::minwindef::DWORD, dwCompStrOffset: ::minwindef::DWORD, dwTargetStrLen: ::minwindef::DWORD, dwTargetStrOffset: ::minwindef::DWORD } /* imm.h:115:16, imm.h:115:16, imm.h:115:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PRECONVERTSTRING = *mut ::imm::RECONVERTSTRING; /* imm.h:124:21, imm.h:124:21, imm.h:124:21 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type NPRECONVERTSTRING = *mut ::imm::RECONVERTSTRING; /* imm.h:124:45, imm.h:124:45, imm.h:124:45 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPRECONVERTSTRING = *mut ::imm::RECONVERTSTRING; /* imm.h:124:69, imm.h:124:69, imm.h:124:69 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct STYLEBUFA { dwStyle: ::minwindef::DWORD, szDescription: *mut [::winnt::CHAR; 32] } /* imm.h:136:16, imm.h:136:16, imm.h:136:16 */
#[cfg(feature="winapi_app")] pub type PSTYLEBUFA = *mut ::imm::STYLEBUFA; /* imm.h:139:15, imm.h:139:15, imm.h:139:15 */
#[cfg(feature="winapi_app")] pub type NPSTYLEBUFA = *mut ::imm::STYLEBUFA; /* imm.h:139:33, imm.h:139:33, imm.h:139:33 */
#[cfg(feature="winapi_app")] pub type LPSTYLEBUFA = *mut ::imm::STYLEBUFA; /* imm.h:139:51, imm.h:139:51, imm.h:139:51 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct STYLEBUFW { dwStyle: ::minwindef::DWORD, szDescription: *mut [::winnt::WCHAR; 32] } /* imm.h:140:16, imm.h:140:16, imm.h:140:16 */
#[cfg(feature="winapi_app")] pub type PSTYLEBUFW = *mut ::imm::STYLEBUFW; /* imm.h:143:15, imm.h:143:15, imm.h:143:15 */
#[cfg(feature="winapi_app")] pub type NPSTYLEBUFW = *mut ::imm::STYLEBUFW; /* imm.h:143:33, imm.h:143:33, imm.h:143:33 */
#[cfg(feature="winapi_app")] pub type LPSTYLEBUFW = *mut ::imm::STYLEBUFW; /* imm.h:143:51, imm.h:143:51, imm.h:143:51 */
#[cfg(feature="winapi_app")] pub type STYLEBUF = ::imm::STYLEBUFW; /* imm.h:145:19, imm.h:145:19, imm.h:145:19 */
#[cfg(feature="winapi_app")] pub type PSTYLEBUF = ::imm::PSTYLEBUFW; /* imm.h:146:20, imm.h:146:20, imm.h:146:20 */
#[cfg(feature="winapi_app")] pub type NPSTYLEBUF = ::imm::NPSTYLEBUFW; /* imm.h:147:21, imm.h:147:21, imm.h:147:21 */
#[cfg(feature="winapi_app")] pub type LPSTYLEBUF = ::imm::LPSTYLEBUFW; /* imm.h:148:21, imm.h:148:21, imm.h:148:21 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct IMEMENUITEMINFOA { cbSize: ::minwindef::UINT, fType: ::minwindef::UINT, fState: ::minwindef::UINT, wID: ::minwindef::UINT, hbmpChecked: ::windef::HBITMAP, hbmpUnchecked: ::windef::HBITMAP, dwItemData: ::minwindef::DWORD, szString: *mut [::winnt::CHAR; 80], hbmpItem: ::windef::HBITMAP } /* imm.h:173:16, imm.h:173:16, imm.h:173:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PIMEMENUITEMINFOA = *mut ::imm::IMEMENUITEMINFOA; /* imm.h:183:22, imm.h:183:22, imm.h:183:22 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type NPIMEMENUITEMINFOA = *mut ::imm::IMEMENUITEMINFOA; /* imm.h:183:47, imm.h:183:47, imm.h:183:47 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPIMEMENUITEMINFOA = *mut ::imm::IMEMENUITEMINFOA; /* imm.h:183:72, imm.h:183:72, imm.h:183:72 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct IMEMENUITEMINFOW { cbSize: ::minwindef::UINT, fType: ::minwindef::UINT, fState: ::minwindef::UINT, wID: ::minwindef::UINT, hbmpChecked: ::windef::HBITMAP, hbmpUnchecked: ::windef::HBITMAP, dwItemData: ::minwindef::DWORD, szString: *mut [::winnt::WCHAR; 80], hbmpItem: ::windef::HBITMAP } /* imm.h:184:16, imm.h:184:16, imm.h:184:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PIMEMENUITEMINFOW = *mut ::imm::IMEMENUITEMINFOW; /* imm.h:194:22, imm.h:194:22, imm.h:194:22 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type NPIMEMENUITEMINFOW = *mut ::imm::IMEMENUITEMINFOW; /* imm.h:194:47, imm.h:194:47, imm.h:194:47 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPIMEMENUITEMINFOW = *mut ::imm::IMEMENUITEMINFOW; /* imm.h:194:72, imm.h:194:72, imm.h:194:72 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type IMEMENUITEMINFO = ::imm::IMEMENUITEMINFOW; /* imm.h:196:26, imm.h:196:26, imm.h:196:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PIMEMENUITEMINFO = ::imm::PIMEMENUITEMINFOW; /* imm.h:197:27, imm.h:197:27, imm.h:197:27 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type NPIMEMENUITEMINFO = ::imm::NPIMEMENUITEMINFOW; /* imm.h:198:28, imm.h:198:28, imm.h:198:28 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPIMEMENUITEMINFO = ::imm::LPIMEMENUITEMINFOW; /* imm.h:199:28, imm.h:199:28, imm.h:199:28 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct IMECHARPOSITION { dwSize: ::minwindef::DWORD, dwCharPos: ::minwindef::DWORD, pt: ::windef::POINT, cLineHeight: ::minwindef::UINT, rcDocument: ::windef::RECT } /* imm.h:207:16, imm.h:207:16, imm.h:207:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PIMECHARPOSITION = *mut ::imm::IMECHARPOSITION; /* imm.h:213:21, imm.h:213:21, imm.h:213:21 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type NPIMECHARPOSITION = *mut ::imm::IMECHARPOSITION; /* imm.h:213:45, imm.h:213:45, imm.h:213:45 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPIMECHARPOSITION = *mut ::imm::IMECHARPOSITION; /* imm.h:213:69, imm.h:213:69, imm.h:213:69 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type IMCENUMPROC = Option<extern "system" fn(*mut ::imm::HIMC__, ::libc::c_long) -> ::libc::c_int>; /* imm.h:215:28, imm.h:215:28 */
#[cfg(feature="winapi_desktop")] pub type REGISTERWORDENUMPROCA = Option<extern "system" fn(*const ::libc::c_schar, ::libc::c_ulong, *const ::libc::c_schar, *mut ::libc::c_void) -> ::libc::c_int>; /* imm.h:377:24, imm.h:377:24, imm.h:377:24 */
#[cfg(feature="winapi_desktop")] pub type REGISTERWORDENUMPROCW = Option<extern "system" fn(*const ::libc::c_ushort, ::libc::c_ulong, *const ::libc::c_ushort, *mut ::libc::c_void) -> ::libc::c_int>; /* imm.h:378:24, imm.h:378:24, imm.h:378:24 */
pub const STYLE_DESCRIPTION_SIZE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* imm.h:131:9, imm.h:131:9, imm.h:131:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IMEMENUITEM_STRING_SIZE: i32 = 0x50i32; /* Integer(80, Yes, Unknown) */ /* imm.h:168:9, imm.h:168:9, imm.h:168:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::imm::REGISTERWORDENUMPROCW as REGISTERWORDENUMPROC; /* imm.h:380:9, imm.h:380:9, imm.h:380:9 */
pub const IMC_GETCANDIDATEPOS: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* imm.h:439:9, imm.h:439:9, imm.h:439:9 */
pub const IMC_SETCANDIDATEPOS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* imm.h:440:9, imm.h:440:9, imm.h:440:9 */
pub const IMC_GETCOMPOSITIONFONT: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* imm.h:441:9, imm.h:441:9, imm.h:441:9 */
pub const IMC_SETCOMPOSITIONFONT: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* imm.h:442:9, imm.h:442:9, imm.h:442:9 */
pub const IMC_GETCOMPOSITIONWINDOW: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* imm.h:443:9, imm.h:443:9, imm.h:443:9 */
pub const IMC_SETCOMPOSITIONWINDOW: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* imm.h:444:9, imm.h:444:9, imm.h:444:9 */
pub const IMC_GETSTATUSWINDOWPOS: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* imm.h:445:9, imm.h:445:9, imm.h:445:9 */
pub const IMC_SETSTATUSWINDOWPOS: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* imm.h:446:9, imm.h:446:9, imm.h:446:9 */
pub const IMC_CLOSESTATUSWINDOW: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* imm.h:447:9, imm.h:447:9, imm.h:447:9 */
pub const IMC_OPENSTATUSWINDOW: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* imm.h:448:9, imm.h:448:9, imm.h:448:9 */
pub const NI_OPENCANDIDATE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* imm.h:452:9, imm.h:452:9, imm.h:452:9 */
pub const NI_CLOSECANDIDATE: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* imm.h:453:9, imm.h:453:9, imm.h:453:9 */
pub const NI_SELECTCANDIDATESTR: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* imm.h:454:9, imm.h:454:9, imm.h:454:9 */
pub const NI_CHANGECANDIDATELIST: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* imm.h:455:9, imm.h:455:9, imm.h:455:9 */
pub const NI_FINALIZECONVERSIONRESULT: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* imm.h:456:9, imm.h:456:9, imm.h:456:9 */
pub const NI_COMPOSITIONSTR: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* imm.h:457:9, imm.h:457:9, imm.h:457:9 */
pub const NI_SETCANDIDATE_PAGESTART: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* imm.h:458:9, imm.h:458:9, imm.h:458:9 */
pub const NI_SETCANDIDATE_PAGESIZE: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* imm.h:459:9, imm.h:459:9, imm.h:459:9 */
pub const NI_IMEMENUSELECTED: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* imm.h:460:9, imm.h:460:9, imm.h:460:9 */
pub const ISC_SHOWUICANDIDATEWINDOW: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:463:9, imm.h:463:9, imm.h:463:9 */
pub const ISC_SHOWUICOMPOSITIONWINDOW: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* imm.h:464:9, imm.h:464:9, imm.h:464:9 */
pub const ISC_SHOWUIGUIDELINE: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* imm.h:465:9, imm.h:465:9, imm.h:465:9 */
pub const ISC_SHOWUIALLCANDIDATEWINDOW: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* imm.h:466:9, imm.h:466:9, imm.h:466:9 */
pub const ISC_SHOWUIALL: i32 = 0xc000000fi32; /* Integer(3221225487, Yes, Unknown) */ /* imm.h:467:9, imm.h:467:9, imm.h:467:9 */
pub const CPS_COMPLETE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:471:9, imm.h:471:9, imm.h:471:9 */
pub const CPS_CONVERT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:472:9, imm.h:472:9, imm.h:472:9 */
pub const CPS_REVERT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* imm.h:473:9, imm.h:473:9, imm.h:473:9 */
pub const CPS_CANCEL: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:474:9, imm.h:474:9, imm.h:474:9 */
pub const MOD_ALT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:477:9, imm.h:477:9, imm.h:477:9 */
pub const MOD_CONTROL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:478:9, imm.h:478:9, imm.h:478:9 */
pub const MOD_SHIFT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:479:9, imm.h:479:9, imm.h:479:9 */
pub const MOD_LEFT: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* imm.h:481:9, imm.h:481:9, imm.h:481:9 */
pub const MOD_RIGHT: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* imm.h:482:9, imm.h:482:9, imm.h:482:9 */
pub const MOD_ON_KEYUP: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* imm.h:484:9, imm.h:484:9, imm.h:484:9 */
pub const MOD_IGNORE_ALL_MODIFIER: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* imm.h:485:9, imm.h:485:9, imm.h:485:9 */
pub const IME_CHOTKEY_IME_NONIME_TOGGLE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* imm.h:489:9, imm.h:489:9, imm.h:489:9 */
pub const IME_CHOTKEY_SHAPE_TOGGLE: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* imm.h:490:9, imm.h:490:9, imm.h:490:9 */
pub const IME_CHOTKEY_SYMBOL_TOGGLE: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* imm.h:491:9, imm.h:491:9, imm.h:491:9 */
pub const IME_JHOTKEY_CLOSE_OPEN: i32 = 0x30i32; /* Integer(48, Yes, Unknown) */ /* imm.h:494:9, imm.h:494:9, imm.h:494:9 */
pub const IME_KHOTKEY_SHAPE_TOGGLE: i32 = 0x50i32; /* Integer(80, Yes, Unknown) */ /* imm.h:497:9, imm.h:497:9, imm.h:497:9 */
pub const IME_KHOTKEY_HANJACONVERT: i32 = 0x51i32; /* Integer(81, Yes, Unknown) */ /* imm.h:498:9, imm.h:498:9, imm.h:498:9 */
pub const IME_KHOTKEY_ENGLISH: i32 = 0x52i32; /* Integer(82, Yes, Unknown) */ /* imm.h:499:9, imm.h:499:9, imm.h:499:9 */
pub const IME_THOTKEY_IME_NONIME_TOGGLE: i32 = 0x70i32; /* Integer(112, Yes, Unknown) */ /* imm.h:502:9, imm.h:502:9, imm.h:502:9 */
pub const IME_THOTKEY_SHAPE_TOGGLE: i32 = 0x71i32; /* Integer(113, Yes, Unknown) */ /* imm.h:503:9, imm.h:503:9, imm.h:503:9 */
pub const IME_THOTKEY_SYMBOL_TOGGLE: i32 = 0x72i32; /* Integer(114, Yes, Unknown) */ /* imm.h:504:9, imm.h:504:9, imm.h:504:9 */
pub const IME_HOTKEY_DSWITCH_FIRST: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* imm.h:507:9, imm.h:507:9, imm.h:507:9 */
pub const IME_HOTKEY_DSWITCH_LAST: i32 = 0x11fi32; /* Integer(287, Yes, Unknown) */ /* imm.h:508:9, imm.h:508:9, imm.h:508:9 */
pub const IME_HOTKEY_PRIVATE_FIRST: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* imm.h:511:9, imm.h:511:9, imm.h:511:9 */
pub const IME_ITHOTKEY_RESEND_RESULTSTR: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* imm.h:512:9, imm.h:512:9, imm.h:512:9 */
pub const IME_ITHOTKEY_PREVIOUS_COMPOSITION: i32 = 0x201i32; /* Integer(513, Yes, Unknown) */ /* imm.h:513:9, imm.h:513:9, imm.h:513:9 */
pub const IME_ITHOTKEY_UISTYLE_TOGGLE: i32 = 0x202i32; /* Integer(514, Yes, Unknown) */ /* imm.h:514:9, imm.h:514:9, imm.h:514:9 */
pub const IME_ITHOTKEY_RECONVERTSTRING: i32 = 0x203i32; /* Integer(515, Yes, Unknown) */ /* imm.h:515:9, imm.h:515:9, imm.h:515:9 */
pub const IME_HOTKEY_PRIVATE_LAST: i32 = 0x21fi32; /* Integer(543, Yes, Unknown) */ /* imm.h:516:9, imm.h:516:9, imm.h:516:9 */
pub const GCS_COMPREADSTR: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:520:9, imm.h:520:9, imm.h:520:9 */
pub const GCS_COMPREADATTR: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:521:9, imm.h:521:9, imm.h:521:9 */
pub const GCS_COMPREADCLAUSE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:522:9, imm.h:522:9, imm.h:522:9 */
pub const GCS_COMPSTR: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* imm.h:523:9, imm.h:523:9, imm.h:523:9 */
pub const GCS_COMPATTR: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* imm.h:524:9, imm.h:524:9, imm.h:524:9 */
pub const GCS_COMPCLAUSE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* imm.h:525:9, imm.h:525:9, imm.h:525:9 */
pub const GCS_CURSORPOS: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* imm.h:526:9, imm.h:526:9, imm.h:526:9 */
pub const GCS_DELTASTART: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* imm.h:527:9, imm.h:527:9, imm.h:527:9 */
pub const GCS_RESULTREADSTR: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* imm.h:528:9, imm.h:528:9, imm.h:528:9 */
pub const GCS_RESULTREADCLAUSE: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* imm.h:529:9, imm.h:529:9, imm.h:529:9 */
pub const GCS_RESULTSTR: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* imm.h:530:9, imm.h:530:9, imm.h:530:9 */
pub const GCS_RESULTCLAUSE: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* imm.h:531:9, imm.h:531:9, imm.h:531:9 */
pub const CS_INSERTCHAR: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* imm.h:534:9, imm.h:534:9, imm.h:534:9 */
pub const CS_NOMOVECARET: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* imm.h:535:9, imm.h:535:9, imm.h:535:9 */
pub const IMEVER_0310: i32 = 0x3000ai32; /* Integer(196618, Yes, Unknown) */ /* imm.h:538:9, imm.h:538:9, imm.h:538:9 */
pub const IMEVER_0400: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* imm.h:539:9, imm.h:539:9, imm.h:539:9 */
pub const IME_PROP_AT_CARET: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* imm.h:543:9, imm.h:543:9, imm.h:543:9 */
pub const IME_PROP_SPECIAL_UI: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* imm.h:544:9, imm.h:544:9, imm.h:544:9 */
pub const IME_PROP_CANDLIST_START_FROM_1: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* imm.h:545:9, imm.h:545:9, imm.h:545:9 */
pub const IME_PROP_UNICODE: i32 = 0x80000i32; /* Integer(524288, Yes, Unknown) */ /* imm.h:546:9, imm.h:546:9, imm.h:546:9 */
pub const IME_PROP_COMPLETE_ON_UNSELECT: i32 = 0x100000i32; /* Integer(1048576, Yes, Unknown) */ /* imm.h:547:9, imm.h:547:9, imm.h:547:9 */
pub const UI_CAP_2700: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:551:9, imm.h:551:9, imm.h:551:9 */
pub const UI_CAP_ROT90: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:552:9, imm.h:552:9, imm.h:552:9 */
pub const UI_CAP_ROTANY: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:553:9, imm.h:553:9, imm.h:553:9 */
pub const SCS_CAP_COMPSTR: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:556:9, imm.h:556:9, imm.h:556:9 */
pub const SCS_CAP_MAKEREAD: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:557:9, imm.h:557:9, imm.h:557:9 */
pub const SCS_CAP_SETRECONVERTSTRING: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:558:9, imm.h:558:9, imm.h:558:9 */
pub const SELECT_CAP_CONVERSION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:562:9, imm.h:562:9, imm.h:562:9 */
pub const SELECT_CAP_SENTENCE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:563:9, imm.h:563:9, imm.h:563:9 */
pub const GGL_LEVEL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:567:9, imm.h:567:9, imm.h:567:9 */
pub const GGL_INDEX: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:568:9, imm.h:568:9, imm.h:568:9 */
pub const GGL_STRING: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* imm.h:569:9, imm.h:569:9, imm.h:569:9 */
pub const GGL_PRIVATE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:570:9, imm.h:570:9, imm.h:570:9 */
pub const GL_LEVEL_NOGUIDELINE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* imm.h:574:9, imm.h:574:9, imm.h:574:9 */
pub const GL_LEVEL_FATAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:575:9, imm.h:575:9, imm.h:575:9 */
pub const GL_LEVEL_ERROR: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:576:9, imm.h:576:9, imm.h:576:9 */
pub const GL_LEVEL_WARNING: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* imm.h:577:9, imm.h:577:9, imm.h:577:9 */
pub const GL_LEVEL_INFORMATION: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:578:9, imm.h:578:9, imm.h:578:9 */
pub const GL_ID_UNKNOWN: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* imm.h:582:9, imm.h:582:9, imm.h:582:9 */
pub const GL_ID_NOMODULE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:583:9, imm.h:583:9, imm.h:583:9 */
pub const GL_ID_NODICTIONARY: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* imm.h:584:9, imm.h:584:9, imm.h:584:9 */
pub const GL_ID_CANNOTSAVE: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* imm.h:585:9, imm.h:585:9, imm.h:585:9 */
pub const GL_ID_NOCONVERT: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* imm.h:586:9, imm.h:586:9, imm.h:586:9 */
pub const GL_ID_TYPINGERROR: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* imm.h:587:9, imm.h:587:9, imm.h:587:9 */
pub const GL_ID_TOOMANYSTROKE: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* imm.h:588:9, imm.h:588:9, imm.h:588:9 */
pub const GL_ID_READINGCONFLICT: i32 = 0x23i32; /* Integer(35, Yes, Unknown) */ /* imm.h:589:9, imm.h:589:9, imm.h:589:9 */
pub const GL_ID_INPUTREADING: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* imm.h:590:9, imm.h:590:9, imm.h:590:9 */
pub const GL_ID_INPUTRADICAL: i32 = 0x25i32; /* Integer(37, Yes, Unknown) */ /* imm.h:591:9, imm.h:591:9, imm.h:591:9 */
pub const GL_ID_INPUTCODE: i32 = 0x26i32; /* Integer(38, Yes, Unknown) */ /* imm.h:592:9, imm.h:592:9, imm.h:592:9 */
pub const GL_ID_INPUTSYMBOL: i32 = 0x27i32; /* Integer(39, Yes, Unknown) */ /* imm.h:593:9, imm.h:593:9, imm.h:593:9 */
pub const GL_ID_CHOOSECANDIDATE: i32 = 0x28i32; /* Integer(40, Yes, Unknown) */ /* imm.h:594:9, imm.h:594:9, imm.h:594:9 */
pub const GL_ID_REVERSECONVERSION: i32 = 0x29i32; /* Integer(41, Yes, Unknown) */ /* imm.h:595:9, imm.h:595:9, imm.h:595:9 */
pub const GL_ID_PRIVATE_FIRST: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* imm.h:596:9, imm.h:596:9, imm.h:596:9 */
pub const GL_ID_PRIVATE_LAST: i32 = 0xffffi32; /* Integer(65535, Yes, Unknown) */ /* imm.h:597:9, imm.h:597:9, imm.h:597:9 */
pub const IGP_PROPERTY: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:602:9, imm.h:602:9, imm.h:602:9 */
pub const IGP_CONVERSION: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* imm.h:603:9, imm.h:603:9, imm.h:603:9 */
pub const IGP_SENTENCE: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* imm.h:604:9, imm.h:604:9, imm.h:604:9 */
pub const IGP_UI: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* imm.h:605:9, imm.h:605:9, imm.h:605:9 */
pub const IGP_SETCOMPSTR: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* imm.h:606:9, imm.h:606:9, imm.h:606:9 */
pub const IGP_SELECT: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* imm.h:607:9, imm.h:607:9, imm.h:607:9 */
pub const SCS_SETRECONVERTSTRING: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* imm.h:613:9, imm.h:613:9, imm.h:613:9 */
pub const SCS_QUERYRECONVERTSTRING: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* imm.h:614:9, imm.h:614:9, imm.h:614:9 */
pub const ATTR_INPUT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* imm.h:617:9, imm.h:617:9, imm.h:617:9 */
pub const ATTR_TARGET_CONVERTED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:618:9, imm.h:618:9, imm.h:618:9 */
pub const ATTR_CONVERTED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:619:9, imm.h:619:9, imm.h:619:9 */
pub const ATTR_TARGET_NOTCONVERTED: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* imm.h:620:9, imm.h:620:9, imm.h:620:9 */
pub const ATTR_INPUT_ERROR: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:621:9, imm.h:621:9, imm.h:621:9 */
pub const ATTR_FIXEDCONVERTED: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* imm.h:622:9, imm.h:622:9, imm.h:622:9 */
pub const CFS_DEFAULT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* imm.h:625:9, imm.h:625:9, imm.h:625:9 */
pub const CFS_RECT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:626:9, imm.h:626:9, imm.h:626:9 */
pub const CFS_POINT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:627:9, imm.h:627:9, imm.h:627:9 */
pub const CFS_FORCE_POSITION: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* imm.h:628:9, imm.h:628:9, imm.h:628:9 */
pub const CFS_CANDIDATEPOS: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* imm.h:629:9, imm.h:629:9, imm.h:629:9 */
pub const CFS_EXCLUDE: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* imm.h:630:9, imm.h:630:9, imm.h:630:9 */
pub const GCL_CONVERSION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:633:9, imm.h:633:9, imm.h:633:9 */
pub const GCL_REVERSECONVERSION: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:634:9, imm.h:634:9, imm.h:634:9 */
pub const GCL_REVERSE_LENGTH: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* imm.h:635:9, imm.h:635:9, imm.h:635:9 */
#[doc(inline)] pub use ::ime_cmodes::IME_CMODE_NATIVE as IME_CMODE_HANGEUL; /* imm.h:642:9, imm.h:642:9, imm.h:642:9 */
pub const IME_CMODE_SOFTKBD: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* imm.h:643:9, imm.h:643:9, imm.h:643:9 */
pub const IME_CMODE_NOCONVERSION: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* imm.h:644:9, imm.h:644:9, imm.h:644:9 */
pub const IME_CMODE_EUDC: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* imm.h:645:9, imm.h:645:9, imm.h:645:9 */
pub const IME_CMODE_SYMBOL: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* imm.h:646:9, imm.h:646:9, imm.h:646:9 */
pub const IME_CMODE_FIXED: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* imm.h:647:9, imm.h:647:9, imm.h:647:9 */
pub const IME_CMODE_RESERVED: i32 = 0xf0000000i32; /* Integer(4026531840, Yes, Unknown) */ /* imm.h:648:9, imm.h:648:9, imm.h:648:9 */
pub const IME_SMODE_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* imm.h:651:9, imm.h:651:9, imm.h:651:9 */
pub const IME_SMODE_PLAURALCLAUSE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:652:9, imm.h:652:9, imm.h:652:9 */
pub const IME_SMODE_SINGLECONVERT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:653:9, imm.h:653:9, imm.h:653:9 */
pub const IME_SMODE_AUTOMATIC: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:654:9, imm.h:654:9, imm.h:654:9 */
pub const IME_SMODE_PHRASEPREDICT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* imm.h:655:9, imm.h:655:9, imm.h:655:9 */
pub const IME_SMODE_CONVERSATION: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* imm.h:656:9, imm.h:656:9, imm.h:656:9 */
pub const IME_SMODE_RESERVED: i32 = 0xf000i32; /* Integer(61440, Yes, Unknown) */ /* imm.h:657:9, imm.h:657:9, imm.h:657:9 */
pub const IME_CAND_UNKNOWN: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* imm.h:661:9, imm.h:661:9, imm.h:661:9 */
pub const IME_CAND_READ: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:662:9, imm.h:662:9, imm.h:662:9 */
pub const IME_CAND_CODE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:663:9, imm.h:663:9, imm.h:663:9 */
pub const IME_CAND_MEANING: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* imm.h:664:9, imm.h:664:9, imm.h:664:9 */
pub const IME_CAND_RADICAL: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:665:9, imm.h:665:9, imm.h:665:9 */
pub const IME_CAND_STROKE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* imm.h:666:9, imm.h:666:9, imm.h:666:9 */
pub const IMN_CLOSESTATUSWINDOW: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:669:9, imm.h:669:9, imm.h:669:9 */
pub const IMN_OPENSTATUSWINDOW: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:670:9, imm.h:670:9, imm.h:670:9 */
pub const IMN_CHANGECANDIDATE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* imm.h:671:9, imm.h:671:9, imm.h:671:9 */
pub const IMN_CLOSECANDIDATE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:672:9, imm.h:672:9, imm.h:672:9 */
pub const IMN_OPENCANDIDATE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* imm.h:673:9, imm.h:673:9, imm.h:673:9 */
pub const IMN_SETCONVERSIONMODE: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* imm.h:674:9, imm.h:674:9, imm.h:674:9 */
pub const IMN_SETSENTENCEMODE: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* imm.h:675:9, imm.h:675:9, imm.h:675:9 */
pub const IMN_SETOPENSTATUS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* imm.h:676:9, imm.h:676:9, imm.h:676:9 */
pub const IMN_SETCANDIDATEPOS: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* imm.h:677:9, imm.h:677:9, imm.h:677:9 */
pub const IMN_SETCOMPOSITIONFONT: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* imm.h:678:9, imm.h:678:9, imm.h:678:9 */
pub const IMN_SETCOMPOSITIONWINDOW: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* imm.h:679:9, imm.h:679:9, imm.h:679:9 */
pub const IMN_SETSTATUSWINDOWPOS: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* imm.h:680:9, imm.h:680:9, imm.h:680:9 */
pub const IMN_GUIDELINE: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* imm.h:681:9, imm.h:681:9, imm.h:681:9 */
pub const IMN_PRIVATE: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* imm.h:682:9, imm.h:682:9, imm.h:682:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IMR_COMPOSITIONWINDOW: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:686:9, imm.h:686:9, imm.h:686:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IMR_CANDIDATEWINDOW: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:687:9, imm.h:687:9, imm.h:687:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IMR_COMPOSITIONFONT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* imm.h:688:9, imm.h:688:9, imm.h:688:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IMR_RECONVERTSTRING: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:689:9, imm.h:689:9, imm.h:689:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IMR_CONFIRMRECONVERTSTRING: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* imm.h:690:9, imm.h:690:9, imm.h:690:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IMR_QUERYCHARPOSITION: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* imm.h:691:9, imm.h:691:9, imm.h:691:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IMR_DOCUMENTFEED: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* imm.h:692:9, imm.h:692:9, imm.h:692:9 */
pub const IME_CONFIG_GENERAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:701:9, imm.h:701:9, imm.h:701:9 */
pub const IME_CONFIG_REGISTERWORD: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:702:9, imm.h:702:9, imm.h:702:9 */
pub const IME_CONFIG_SELECTDICTIONARY: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* imm.h:703:9, imm.h:703:9, imm.h:703:9 */
pub const IME_ESC_QUERY_SUPPORT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* imm.h:707:9, imm.h:707:9, imm.h:707:9 */
pub const IME_ESC_RESERVED_FIRST: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:708:9, imm.h:708:9, imm.h:708:9 */
pub const IME_ESC_RESERVED_LAST: i32 = 0x7ffi32; /* Integer(2047, Yes, Unknown) */ /* imm.h:709:9, imm.h:709:9, imm.h:709:9 */
pub const IME_ESC_PRIVATE_FIRST: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* imm.h:710:9, imm.h:710:9, imm.h:710:9 */
pub const IME_ESC_PRIVATE_LAST: i32 = 0xfffi32; /* Integer(4095, Yes, Unknown) */ /* imm.h:711:9, imm.h:711:9, imm.h:711:9 */
pub const IME_ESC_SEQUENCE_TO_INTERNAL: i32 = 0x1001i32; /* Integer(4097, Yes, Unknown) */ /* imm.h:713:9, imm.h:713:9, imm.h:713:9 */
pub const IME_ESC_GET_EUDC_DICTIONARY: i32 = 0x1003i32; /* Integer(4099, Yes, Unknown) */ /* imm.h:714:9, imm.h:714:9, imm.h:714:9 */
pub const IME_ESC_SET_EUDC_DICTIONARY: i32 = 0x1004i32; /* Integer(4100, Yes, Unknown) */ /* imm.h:715:9, imm.h:715:9, imm.h:715:9 */
pub const IME_ESC_MAX_KEY: i32 = 0x1005i32; /* Integer(4101, Yes, Unknown) */ /* imm.h:716:9, imm.h:716:9, imm.h:716:9 */
pub const IME_ESC_IME_NAME: i32 = 0x1006i32; /* Integer(4102, Yes, Unknown) */ /* imm.h:717:9, imm.h:717:9, imm.h:717:9 */
pub const IME_ESC_SYNC_HOTKEY: i32 = 0x1007i32; /* Integer(4103, Yes, Unknown) */ /* imm.h:718:9, imm.h:718:9, imm.h:718:9 */
pub const IME_ESC_HANJA_MODE: i32 = 0x1008i32; /* Integer(4104, Yes, Unknown) */ /* imm.h:719:9, imm.h:719:9, imm.h:719:9 */
pub const IME_ESC_AUTOMATA: i32 = 0x1009i32; /* Integer(4105, Yes, Unknown) */ /* imm.h:720:9, imm.h:720:9, imm.h:720:9 */
pub const IME_ESC_PRIVATE_HOTKEY: i32 = 0x100ai32; /* Integer(4106, Yes, Unknown) */ /* imm.h:721:9, imm.h:721:9, imm.h:721:9 */
pub const IME_ESC_GETHELPFILENAME: i32 = 0x100bi32; /* Integer(4107, Yes, Unknown) */ /* imm.h:722:9, imm.h:722:9, imm.h:722:9 */
pub const IME_REGWORD_STYLE_EUDC: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:725:9, imm.h:725:9, imm.h:725:9 */
pub const IME_REGWORD_STYLE_USER_FIRST: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* imm.h:726:9, imm.h:726:9, imm.h:726:9 */
pub const IME_REGWORD_STYLE_USER_LAST: i32 = 0xffffffffi32; /* Integer(4294967295, Yes, Unknown) */ /* imm.h:727:9, imm.h:727:9, imm.h:727:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IACE_CHILDREN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:733:9, imm.h:733:9, imm.h:733:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IACE_DEFAULT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* imm.h:734:9, imm.h:734:9, imm.h:734:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IACE_IGNORENOCONTEXT: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* imm.h:735:9, imm.h:735:9, imm.h:735:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IGIMIF_RIGHTMENU: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:738:9, imm.h:738:9, imm.h:738:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IGIMII_CMODE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:741:9, imm.h:741:9, imm.h:741:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IGIMII_SMODE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:742:9, imm.h:742:9, imm.h:742:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IGIMII_CONFIGURE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:743:9, imm.h:743:9, imm.h:743:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IGIMII_TOOLS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* imm.h:744:9, imm.h:744:9, imm.h:744:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IGIMII_HELP: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* imm.h:745:9, imm.h:745:9, imm.h:745:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IGIMII_OTHER: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* imm.h:746:9, imm.h:746:9, imm.h:746:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IGIMII_INPUTTOOLS: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* imm.h:747:9, imm.h:747:9, imm.h:747:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IMFT_RADIOCHECK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:750:9, imm.h:750:9, imm.h:750:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IMFT_SEPARATOR: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:751:9, imm.h:751:9, imm.h:751:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const IMFT_SUBMENU: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* imm.h:752:9, imm.h:752:9, imm.h:752:9 */
#[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use ::winuser::MFS_GRAYED as IMFS_GRAYED; /* imm.h:755:9, imm.h:755:9, imm.h:755:9 */
#[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use ::winuser::MFS_DISABLED as IMFS_DISABLED; /* imm.h:756:9, imm.h:756:9, imm.h:756:9 */
#[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use ::winuser::MFS_CHECKED as IMFS_CHECKED; /* imm.h:757:9, imm.h:757:9, imm.h:757:9 */
#[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use ::winuser::MFS_HILITE as IMFS_HILITE; /* imm.h:758:9, imm.h:758:9, imm.h:758:9 */
#[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use ::winuser::MFS_ENABLED as IMFS_ENABLED; /* imm.h:759:9, imm.h:759:9, imm.h:759:9 */
#[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use ::winuser::MFS_UNCHECKED as IMFS_UNCHECKED; /* imm.h:760:9, imm.h:760:9, imm.h:760:9 */
#[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use ::winuser::MFS_UNHILITE as IMFS_UNHILITE; /* imm.h:761:9, imm.h:761:9, imm.h:761:9 */
#[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use ::winuser::MFS_DEFAULT as IMFS_DEFAULT; /* imm.h:762:9, imm.h:762:9, imm.h:762:9 */
pub const SOFTKEYBOARD_TYPE_T1: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* imm.h:768:9, imm.h:768:9, imm.h:768:9 */
pub const SOFTKEYBOARD_TYPE_C1: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* imm.h:770:9, imm.h:770:9, imm.h:770:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[cfg(any(target_arch="x86_64"))] pub type IMCENUMPROC = Option<extern "system" fn(*mut ::imm::HIMC__, ::libc::c_longlong) -> ::libc::c_int>; /* imm.h:215:28 */
