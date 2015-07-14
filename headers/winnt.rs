pub type PVOID = *mut ::libc::c_void; /* winnt.h:341:15, winnt.h:341:15, winnt.h:341:15 */
pub type PVOID64 = *mut ::libc::c_void; /* winnt.h:342:27, winnt.h:342:27, winnt.h:342:27 */
pub type CHAR = ::libc::c_schar; /* winnt.h:383:14, winnt.h:383:14, winnt.h:383:14 */
pub type SHORT = ::libc::c_short; /* winnt.h:384:15, winnt.h:384:15, winnt.h:384:15 */
pub type LONG = ::libc::c_long; /* winnt.h:385:14, winnt.h:385:14, winnt.h:385:14 */
pub type INT = ::libc::c_int; /* winnt.h:387:13, winnt.h:387:13, winnt.h:387:13 */
pub type WCHAR = ::libc::c_ushort; /* winnt.h:396:17, winnt.h:396:17, winnt.h:396:17 */
pub type PWCHAR = *mut ::libc::c_ushort; /* winnt.h:402:16, winnt.h:402:16, winnt.h:402:16 */
pub type LPWCH = *mut ::libc::c_ushort; /* winnt.h:402:25, winnt.h:402:25, winnt.h:402:25 */
pub type PWCH = *mut ::libc::c_ushort; /* winnt.h:402:33, winnt.h:402:33, winnt.h:402:33 */
pub type LPCWCH = *const ::libc::c_ushort; /* winnt.h:403:22, winnt.h:403:22, winnt.h:403:22 */
pub type PCWCH = *const ::libc::c_ushort; /* winnt.h:403:31, winnt.h:403:31, winnt.h:403:31 */
pub type NWPSTR = *mut ::libc::c_ushort; /* winnt.h:405:34, winnt.h:405:34, winnt.h:405:34 */
pub type LPWSTR = *mut ::libc::c_ushort; /* winnt.h:405:43, winnt.h:405:43, winnt.h:405:43 */
pub type PWSTR = *mut ::libc::c_ushort; /* winnt.h:405:52, winnt.h:405:52, winnt.h:405:52 */
pub type PZPWSTR = *mut *mut ::libc::c_ushort; /* winnt.h:406:34, winnt.h:406:34, winnt.h:406:34 */
pub type PCZPWSTR = *const *mut ::libc::c_ushort; /* winnt.h:407:40, winnt.h:407:40, winnt.h:407:40 */
pub type LPUWSTR = *mut ::libc::c_ushort; /* winnt.h:408:44, winnt.h:408:44, winnt.h:408:44 */
pub type PUWSTR = *mut ::libc::c_ushort; /* winnt.h:408:54, winnt.h:408:54, winnt.h:408:54 */
pub type LPCWSTR = *const ::libc::c_ushort; /* winnt.h:409:40, winnt.h:409:40, winnt.h:409:40 */
pub type PCWSTR = *const ::libc::c_ushort; /* winnt.h:409:50, winnt.h:409:50, winnt.h:409:50 */
pub type PZPCWSTR = *mut *const ::libc::c_ushort; /* winnt.h:410:35, winnt.h:410:35, winnt.h:410:35 */
pub type PCZPCWSTR = *const *const ::libc::c_ushort; /* winnt.h:411:41, winnt.h:411:41, winnt.h:411:41 */
pub type LPCUWSTR = *const ::libc::c_ushort; /* winnt.h:412:50, winnt.h:412:50, winnt.h:412:50 */
pub type PCUWSTR = *const ::libc::c_ushort; /* winnt.h:412:61, winnt.h:412:61, winnt.h:412:61 */
pub type PZZWSTR = *mut ::libc::c_ushort; /* winnt.h:414:38, winnt.h:414:38, winnt.h:414:38 */
pub type PCZZWSTR = *const ::libc::c_ushort; /* winnt.h:415:44, winnt.h:415:44, winnt.h:415:44 */
pub type PUZZWSTR = *mut ::libc::c_ushort; /* winnt.h:416:48, winnt.h:416:48, winnt.h:416:48 */
pub type PCUZZWSTR = *const ::libc::c_ushort; /* winnt.h:417:54, winnt.h:417:54, winnt.h:417:54 */
pub type PNZWCH = *mut ::libc::c_ushort; /* winnt.h:419:17, winnt.h:419:17, winnt.h:419:17 */
pub type PCNZWCH = *const ::libc::c_ushort; /* winnt.h:420:23, winnt.h:420:23, winnt.h:420:23 */
pub type PUNZWCH = *mut ::libc::c_ushort; /* winnt.h:421:27, winnt.h:421:27, winnt.h:421:27 */
pub type PCUNZWCH = *const ::libc::c_ushort; /* winnt.h:422:33, winnt.h:422:33, winnt.h:422:33 */
#[cfg(any(feature="winapi_ver_06000000"))] pub type LPCWCHAR = *const ::libc::c_ushort; /* winnt.h:426:22, winnt.h:426:22, winnt.h:426:22 */
#[cfg(any(feature="winapi_ver_06000000"))] pub type PCWCHAR = *const ::libc::c_ushort; /* winnt.h:426:33, winnt.h:426:33, winnt.h:426:33 */
#[cfg(any(feature="winapi_ver_06000000"))] pub type LPCUWCHAR = *const ::libc::c_ushort; /* winnt.h:427:32, winnt.h:427:32, winnt.h:427:32 */
#[cfg(any(feature="winapi_ver_06000000"))] pub type PCUWCHAR = *const ::libc::c_ushort; /* winnt.h:427:44, winnt.h:427:44, winnt.h:427:44 */
#[cfg(any(feature="winapi_ver_06000000"))] pub type UCSCHAR = ::libc::c_ulong; /* winnt.h:433:23, winnt.h:433:23, winnt.h:433:23 */
#[cfg(any(feature="winapi_ver_06000000"))] pub type PUCSCHAR = *mut ::libc::c_ulong; /* winnt.h:453:18, winnt.h:453:18, winnt.h:453:18 */
#[cfg(any(feature="winapi_ver_06000000"))] pub type PCUCSCHAR = *const ::libc::c_ulong; /* winnt.h:454:24, winnt.h:454:24, winnt.h:454:24 */
#[cfg(any(feature="winapi_ver_06000000"))] pub type PUCSSTR = *mut ::libc::c_ulong; /* winnt.h:456:18, winnt.h:456:18, winnt.h:456:18 */
#[cfg(any(feature="winapi_ver_06000000"))] pub type PUUCSSTR = *mut ::libc::c_ulong; /* winnt.h:457:28, winnt.h:457:28, winnt.h:457:28 */
#[cfg(any(feature="winapi_ver_06000000"))] pub type PCUCSSTR = *const ::libc::c_ulong; /* winnt.h:459:24, winnt.h:459:24, winnt.h:459:24 */
#[cfg(any(feature="winapi_ver_06000000"))] pub type PCUUCSSTR = *const ::libc::c_ulong; /* winnt.h:460:34, winnt.h:460:34, winnt.h:460:34 */
#[cfg(any(feature="winapi_ver_06000000"))] pub type PUUCSCHAR = *mut ::libc::c_ulong; /* winnt.h:462:28, winnt.h:462:28, winnt.h:462:28 */
#[cfg(any(feature="winapi_ver_06000000"))] pub type PCUUCSCHAR = *const ::libc::c_ulong; /* winnt.h:463:34, winnt.h:463:34, winnt.h:463:34 */
pub type PCHAR = *mut ::libc::c_schar; /* winnt.h:471:15, winnt.h:471:15, winnt.h:471:15 */
pub type LPCH = *mut ::libc::c_schar; /* winnt.h:471:23, winnt.h:471:23, winnt.h:471:23 */
pub type PCH = *mut ::libc::c_schar; /* winnt.h:471:30, winnt.h:471:30, winnt.h:471:30 */
pub type LPCCH = *const ::libc::c_schar; /* winnt.h:472:21, winnt.h:472:21, winnt.h:472:21 */
pub type PCCH = *const ::libc::c_schar; /* winnt.h:472:29, winnt.h:472:29, winnt.h:472:29 */
pub type NPSTR = *mut ::libc::c_schar; /* winnt.h:474:33, winnt.h:474:33, winnt.h:474:33 */
pub type LPSTR = *mut ::libc::c_schar; /* winnt.h:474:41, winnt.h:474:41, winnt.h:474:41 */
pub type PSTR = *mut ::libc::c_schar; /* winnt.h:474:49, winnt.h:474:49, winnt.h:474:49 */
pub type PZPSTR = *mut *mut ::libc::c_schar; /* winnt.h:475:33, winnt.h:475:33, winnt.h:475:33 */
pub type PCZPSTR = *const *mut ::libc::c_schar; /* winnt.h:476:39, winnt.h:476:39, winnt.h:476:39 */
pub type LPCSTR = *const ::libc::c_schar; /* winnt.h:477:39, winnt.h:477:39, winnt.h:477:39 */
pub type PCSTR = *const ::libc::c_schar; /* winnt.h:477:48, winnt.h:477:48, winnt.h:477:48 */
pub type PZPCSTR = *mut *const ::libc::c_schar; /* winnt.h:478:34, winnt.h:478:34, winnt.h:478:34 */
pub type PCZPCSTR = *const *const ::libc::c_schar; /* winnt.h:479:40, winnt.h:479:40, winnt.h:479:40 */
pub type PZZSTR = *mut ::libc::c_schar; /* winnt.h:481:37, winnt.h:481:37, winnt.h:481:37 */
pub type PCZZSTR = *const ::libc::c_schar; /* winnt.h:482:43, winnt.h:482:43, winnt.h:482:43 */
pub type PNZCH = *mut ::libc::c_schar; /* winnt.h:484:16, winnt.h:484:16, winnt.h:484:16 */
pub type PCNZCH = *const ::libc::c_schar; /* winnt.h:485:22, winnt.h:485:22, winnt.h:485:22 */
pub type TCHAR = ::winnt::WCHAR; /* winnt.h:493:15, winnt.h:493:15, winnt.h:493:15 */
pub type PTCHAR = *mut ::libc::c_ushort; /* winnt.h:493:23, winnt.h:493:23, winnt.h:493:23 */
pub type TBYTE = ::winnt::WCHAR; /* winnt.h:494:15, winnt.h:494:15, winnt.h:494:15 */
pub type PTBYTE = *mut ::libc::c_ushort; /* winnt.h:494:24, winnt.h:494:24, winnt.h:494:24 */
pub type LPTCH = ::winnt::LPWCH; /* winnt.h:498:15, winnt.h:498:15, winnt.h:498:15 */
pub type PTCH = ::winnt::LPWCH; /* winnt.h:498:22, winnt.h:498:22, winnt.h:498:22 */
pub type LPCTCH = ::winnt::LPCWCH; /* winnt.h:499:16, winnt.h:499:16, winnt.h:499:16 */
pub type PCTCH = ::winnt::LPCWCH; /* winnt.h:499:24, winnt.h:499:24, winnt.h:499:24 */
pub type PTSTR = ::winnt::LPWSTR; /* winnt.h:500:16, winnt.h:500:16, winnt.h:500:16 */
pub type LPTSTR = ::winnt::LPWSTR; /* winnt.h:500:23, winnt.h:500:23, winnt.h:500:23 */
pub type PCTSTR = ::winnt::LPCWSTR; /* winnt.h:501:17, winnt.h:501:17, winnt.h:501:17 */
pub type LPCTSTR = ::winnt::LPCWSTR; /* winnt.h:501:25, winnt.h:501:25, winnt.h:501:25 */
pub type PUTSTR = ::winnt::LPUWSTR; /* winnt.h:502:17, winnt.h:502:17, winnt.h:502:17 */
pub type LPUTSTR = ::winnt::LPUWSTR; /* winnt.h:502:25, winnt.h:502:25, winnt.h:502:25 */
pub type PCUTSTR = ::winnt::LPCUWSTR; /* winnt.h:503:18, winnt.h:503:18, winnt.h:503:18 */
pub type LPCUTSTR = ::winnt::LPCUWSTR; /* winnt.h:503:27, winnt.h:503:27, winnt.h:503:27 */
pub type LP = ::winnt::LPWSTR; /* winnt.h:504:16, winnt.h:504:16, winnt.h:504:16 */
pub type PZZTSTR = ::winnt::PZZWSTR; /* winnt.h:505:17, winnt.h:505:17, winnt.h:505:17 */
pub type PCZZTSTR = ::winnt::PCZZWSTR; /* winnt.h:506:18, winnt.h:506:18, winnt.h:506:18 */
pub type PUZZTSTR = ::winnt::PUZZWSTR; /* winnt.h:507:18, winnt.h:507:18, winnt.h:507:18 */
pub type PCUZZTSTR = ::winnt::PCUZZWSTR; /* winnt.h:508:19, winnt.h:508:19, winnt.h:508:19 */
pub type PZPTSTR = ::winnt::PZPWSTR; /* winnt.h:509:17, winnt.h:509:17, winnt.h:509:17 */
pub type PNZTCH = ::winnt::PNZWCH; /* winnt.h:510:16, winnt.h:510:16, winnt.h:510:16 */
pub type PCNZTCH = ::winnt::PCNZWCH; /* winnt.h:511:17, winnt.h:511:17, winnt.h:511:17 */
pub type PUNZTCH = ::winnt::PUNZWCH; /* winnt.h:512:17, winnt.h:512:17, winnt.h:512:17 */
pub type PCUNZTCH = ::winnt::PCUNZWCH; /* winnt.h:513:18, winnt.h:513:18, winnt.h:513:18 */
pub type PSHORT = *mut ::libc::c_short; /* winnt.h:539:16, winnt.h:539:16, winnt.h:539:16 */
pub type PLONG = *mut ::libc::c_long; /* winnt.h:540:15, winnt.h:540:15, winnt.h:540:15 */
#[repr(C)] pub struct PROCESSOR_NUMBER { Group: ::minwindef::WORD, Number: ::minwindef::BYTE, Reserved: ::minwindef::BYTE } /* winnt.h:549:16, winnt.h:549:16, winnt.h:549:16 */
pub type PPROCESSOR_NUMBER = *mut ::winnt::PROCESSOR_NUMBER; /* winnt.h:553:22, winnt.h:553:22, winnt.h:553:22 */
#[repr(C)] pub struct GROUP_AFFINITY { Mask: ::basetsd::KAFFINITY, Group: ::minwindef::WORD, Reserved: *mut [::minwindef::WORD; 3] } /* winnt.h:560:16, winnt.h:560:16, winnt.h:560:16 */
pub type PGROUP_AFFINITY = *mut ::winnt::GROUP_AFFINITY; /* winnt.h:564:20, winnt.h:564:20, winnt.h:564:20 */
pub type HANDLE = *mut ::libc::c_void; /* winnt.h:573:15, winnt.h:573:15, winnt.h:573:15 */
pub type PHANDLE = *mut *mut ::libc::c_void; /* winnt.h:583:17, winnt.h:583:17, winnt.h:583:17 */
pub type FCHAR = ::minwindef::BYTE; /* winnt.h:591:16, winnt.h:591:16, winnt.h:591:16 */
pub type FSHORT = ::minwindef::WORD; /* winnt.h:592:16, winnt.h:592:16, winnt.h:592:16 */
pub type FLONG = ::minwindef::DWORD; /* winnt.h:593:16, winnt.h:593:16, winnt.h:593:16 */
pub type HRESULT = ::libc::c_long; /* winnt.h:604:49, winnt.h:604:49, winnt.h:604:49 */
pub type CCHAR = ::libc::c_schar; /* winnt.h:667:14, winnt.h:667:14, winnt.h:667:14 */
pub type LCID = ::minwindef::DWORD; /* winnt.h:668:15, winnt.h:668:15, winnt.h:668:15 */
pub type PLCID = ::minwindef::PDWORD; /* winnt.h:669:16, winnt.h:669:16, winnt.h:669:16 */
pub type LANGID = ::minwindef::WORD; /* winnt.h:670:16, winnt.h:670:16, winnt.h:670:16 */
#[repr(C)] pub enum COMPARTMENT_ID {UNSPECIFIED_COMPARTMENT_ID = 0, DEFAULT_COMPARTMENT_ID = 1} pub use self::COMPARTMENT_ID::{UNSPECIFIED_COMPARTMENT_ID, DEFAULT_COMPARTMENT_ID}; /* winnt.h:679:9, winnt.h:679:9, winnt.h:679:9 */
pub type PCOMPARTMENT_ID = *mut ::winnt::COMPARTMENT_ID; /* winnt.h:682:20, winnt.h:682:20, winnt.h:682:20 */
#[repr(C)] pub struct FLOAT128 { LowPart: ::libc::c_longlong, HighPart: ::libc::c_longlong } /* winnt.h:710:16, winnt.h:710:16, winnt.h:710:16 */
pub type PFLOAT128 = *mut ::winnt::FLOAT128; /* winnt.h:715:19, winnt.h:715:19, winnt.h:715:19 */
#[cfg(any(target_arch="x86"))] pub type LONGLONG = ::libc::c_double; /* winnt.h:741:16 */
#[cfg(any(target_arch="x86"))] pub type ULONGLONG = ::libc::c_double; /* winnt.h:742:16 */
#[cfg(any(target_arch="x86"))] pub type PLONGLONG = *mut ::libc::c_double; /* winnt.h:747:19 */
#[cfg(any(target_arch="x86"))] pub type PULONGLONG = *mut ::libc::c_double; /* winnt.h:748:20 */
pub type USN = ::winnt::LONGLONG; /* winnt.h:752:18, winnt.h:752:18, winnt.h:752:18 */
#[repr(C)] pub /*union*/ struct LARGE_INTEGER; /* STUB! */ /* winnt.h:757:15, winnt.h:757:15, winnt.h:757:15 */
pub type PLARGE_INTEGER = *mut ::winnt::LARGE_INTEGER; /* winnt.h:770:24, winnt.h:770:24, winnt.h:770:24 */
#[repr(C)] pub /*union*/ struct ULARGE_INTEGER; /* STUB! */ /* winnt.h:775:15, winnt.h:775:15, winnt.h:775:15 */
pub type PULARGE_INTEGER = *mut ::winnt::ULARGE_INTEGER; /* winnt.h:788:25, winnt.h:788:25, winnt.h:788:25 */
pub type RTL_REFERENCE_COUNT = ::basetsd::LONG_PTR; /* winnt.h:794:18, winnt.h:794:18, winnt.h:794:18 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PRTL_REFERENCE_COUNT = *mut ::libc::c_long; /* winnt.h:794:40, winnt.h:794:40 */
#[repr(C)] pub struct LUID { LowPart: ::minwindef::DWORD, HighPart: ::winnt::LONG } /* winnt.h:804:16, winnt.h:804:16, winnt.h:804:16 */
pub type PLUID = *mut ::winnt::LUID; /* winnt.h:807:10, winnt.h:807:10, winnt.h:807:10 */
pub type DWORDLONG = ::winnt::ULONGLONG; /* winnt.h:810:20, winnt.h:810:20, winnt.h:810:20 */
#[cfg(any(target_arch="x86"))] pub type PDWORDLONG = *mut ::libc::c_double; /* winnt.h:811:20 */
pub type BOOLEAN = ::minwindef::BYTE; /* winnt.h:1042:15, winnt.h:1042:15, winnt.h:1042:15 */
pub type PBOOLEAN = *mut ::libc::c_uchar; /* winnt.h:1043:18, winnt.h:1043:18, winnt.h:1043:18 */
#[repr(C)] pub struct LIST_ENTRY { Flink: *mut ::winnt::LIST_ENTRY, Blink: *mut ::winnt::LIST_ENTRY } /* winnt.h:1049:16, winnt.h:1049:16, winnt.h:1049:16 */
pub type PLIST_ENTRY = *mut ::winnt::LIST_ENTRY; /* winnt.h:1052:16, winnt.h:1052:16, winnt.h:1052:16 */
pub type PRLIST_ENTRY = *mut ::winnt::LIST_ENTRY; /* winnt.h:1052:49, winnt.h:1052:49, winnt.h:1052:49 */
#[repr(C)] pub struct SINGLE_LIST_ENTRY { Next: *mut ::winnt::SINGLE_LIST_ENTRY } /* winnt.h:1059:16, winnt.h:1059:16, winnt.h:1059:16 */
pub type PSINGLE_LIST_ENTRY = *mut ::winnt::SINGLE_LIST_ENTRY; /* winnt.h:1061:23, winnt.h:1061:23, winnt.h:1061:23 */
#[repr(C)] pub struct LIST_ENTRY32 { Flink: ::minwindef::DWORD, Blink: ::minwindef::DWORD } /* winnt.h:1070:16, winnt.h:1070:16, winnt.h:1070:16 */
pub type PLIST_ENTRY32 = *mut ::winnt::LIST_ENTRY32; /* winnt.h:1074:23, winnt.h:1074:23, winnt.h:1074:23 */
#[repr(C)] pub struct LIST_ENTRY64 { Flink: ::winnt::ULONGLONG, Blink: ::winnt::ULONGLONG } /* winnt.h:1076:16, winnt.h:1076:16, winnt.h:1076:16 */
pub type PLIST_ENTRY64 = *mut ::winnt::LIST_ENTRY64; /* winnt.h:1080:23, winnt.h:1080:23, winnt.h:1080:23 */
#[repr(C)] pub struct OBJECTID { Lineage: ::guiddef::GUID, Uniquifier: ::minwindef::DWORD } /* winnt.h:1088:17, winnt.h:1088:17, winnt.h:1088:17 */
pub type PEXCEPTION_ROUTINE = *mut ::libc::c_int; /* winnt.h:1318:28, winnt.h:1318:28, winnt.h:1318:28 */
pub type KSPIN_LOCK = ::basetsd::ULONG_PTR; /* winnt.h:2277:19, winnt.h:2277:19, winnt.h:2277:19 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PKSPIN_LOCK = *mut ::libc::c_ulong; /* winnt.h:2278:21, winnt.h:2278:21 */
#[repr(C)] pub struct M128A { Low: ::winnt::ULONGLONG, High: ::winnt::LONGLONG } /* winnt.h:2286:35, winnt.h:2286:35, winnt.h:2286:35 */
pub type PM128A = *mut ::winnt::M128A; /* winnt.h:2289:11, winnt.h:2289:11, winnt.h:2289:11 */
#[cfg(any(target_arch="x86", target_arch="arm"))] #[repr(C)] pub struct XSAVE_FORMAT { ControlWord: ::minwindef::WORD, StatusWord: ::minwindef::WORD, TagWord: ::minwindef::BYTE, Reserved1: ::minwindef::BYTE, ErrorOpcode: ::minwindef::WORD, ErrorOffset: ::minwindef::DWORD, ErrorSelector: ::minwindef::WORD, Reserved2: ::minwindef::WORD, DataOffset: ::minwindef::DWORD, DataSelector: ::minwindef::WORD, Reserved3: ::minwindef::WORD, MxCsr: ::minwindef::DWORD, MxCsr_Mask: ::minwindef::DWORD, FloatRegisters: *mut [::winnt::M128A; 8], XmmRegisters: *mut [::winnt::M128A; 8], Reserved4: *mut [::minwindef::BYTE; 224] } /* winnt.h:2295:35, winnt.h:2295:35 */
pub type PXSAVE_FORMAT = *mut ::winnt::XSAVE_FORMAT; /* winnt.h:2323:18, winnt.h:2323:18, winnt.h:2323:18 */
#[repr(C)] pub struct XSAVE_AREA_HEADER { Mask: ::basetsd::DWORD64, Reserved: *mut [::basetsd::DWORD64; 7] } /* winnt.h:2327:34, winnt.h:2327:34, winnt.h:2327:34 */
pub type PXSAVE_AREA_HEADER = *mut ::winnt::XSAVE_AREA_HEADER; /* winnt.h:2330:23, winnt.h:2330:23, winnt.h:2330:23 */
#[repr(C)] pub struct XSAVE_AREA { LegacyState: ::winnt::XSAVE_FORMAT, Header: ::winnt::XSAVE_AREA_HEADER } /* winnt.h:2332:35, winnt.h:2332:35, winnt.h:2332:35 */
pub type PXSAVE_AREA = *mut ::winnt::XSAVE_AREA; /* winnt.h:2335:16, winnt.h:2335:16, winnt.h:2335:16 */
#[cfg(any(target_arch="x86"))] #[repr(C)] pub struct XSTATE_CONTEXT { Mask: ::basetsd::DWORD64, Length: ::minwindef::DWORD, Reserved1: ::minwindef::DWORD, Area: ::winnt::PXSAVE_AREA, Reserved2: ::minwindef::DWORD, Buffer: ::winnt::PVOID, Reserved3: ::minwindef::DWORD } /* winnt.h:2337:16 */
pub type PXSTATE_CONTEXT = *mut ::winnt::XSTATE_CONTEXT; /* winnt.h:2353:20, winnt.h:2353:20, winnt.h:2353:20 */
#[repr(C)] pub struct SCOPE_TABLE_AMD64 { Count: ::minwindef::DWORD, ScopeRecord: *mut [::winnt::SCOPE_TABLE_AMD64_Child_1; 1] } /* winnt.h:2359:16, winnt.h:2359:16, winnt.h:2359:16 */
#[repr(C)] pub struct SCOPE_TABLE_AMD64_Child_1 { BeginAddress: ::minwindef::DWORD, EndAddress: ::minwindef::DWORD, HandlerAddress: ::minwindef::DWORD, JumpTarget: ::minwindef::DWORD } /* winnt.h:2361:5, winnt.h:2361:5, winnt.h:2361:5 */
pub type PSCOPE_TABLE_AMD64 = *mut ::winnt::SCOPE_TABLE_AMD64; /* winnt.h:2367:23, winnt.h:2367:23, winnt.h:2367:23 */
#[repr(C)] pub struct SCOPE_TABLE_ARM { Count: ::minwindef::DWORD, ScopeRecord: *mut [::winnt::SCOPE_TABLE_ARM_Child_1; 1] } /* winnt.h:3847:16, winnt.h:3847:16, winnt.h:3847:16 */
#[repr(C)] pub struct SCOPE_TABLE_ARM_Child_1 { BeginAddress: ::minwindef::DWORD, EndAddress: ::minwindef::DWORD, HandlerAddress: ::minwindef::DWORD, JumpTarget: ::minwindef::DWORD } /* winnt.h:3849:5, winnt.h:3849:5, winnt.h:3849:5 */
pub type PSCOPE_TABLE_ARM = *mut ::winnt::SCOPE_TABLE_ARM; /* winnt.h:3856:21, winnt.h:3856:21, winnt.h:3856:21 */
#[cfg(any(target_arch="x86"))] #[repr(C)] pub struct FLOATING_SAVE_AREA { ControlWord: ::minwindef::DWORD, StatusWord: ::minwindef::DWORD, TagWord: ::minwindef::DWORD, ErrorOffset: ::minwindef::DWORD, ErrorSelector: ::minwindef::DWORD, DataOffset: ::minwindef::DWORD, DataSelector: ::minwindef::DWORD, RegisterArea: *mut [::minwindef::BYTE; 80], Spare0: ::minwindef::DWORD } /* winnt.h:5908:16 */
#[cfg(any(target_arch="x86"))] pub type PFLOATING_SAVE_AREA = *mut ::winnt::FLOATING_SAVE_AREA; /* winnt.h:5920:29 */
#[cfg(any(target_arch="x86"))] #[repr(C)] pub struct CONTEXT { ContextFlags: ::minwindef::DWORD, Dr0: ::minwindef::DWORD, Dr1: ::minwindef::DWORD, Dr2: ::minwindef::DWORD, Dr3: ::minwindef::DWORD, Dr6: ::minwindef::DWORD, Dr7: ::minwindef::DWORD, FloatSave: ::winnt::FLOATING_SAVE_AREA, SegGs: ::minwindef::DWORD, SegFs: ::minwindef::DWORD, SegEs: ::minwindef::DWORD, SegDs: ::minwindef::DWORD, Edi: ::minwindef::DWORD, Esi: ::minwindef::DWORD, Ebx: ::minwindef::DWORD, Edx: ::minwindef::DWORD, Ecx: ::minwindef::DWORD, Eax: ::minwindef::DWORD, Ebp: ::minwindef::DWORD, Eip: ::minwindef::DWORD, SegCs: ::minwindef::DWORD, EFlags: ::minwindef::DWORD, Esp: ::minwindef::DWORD, SegSs: ::minwindef::DWORD, ExtendedRegisters: *mut [::minwindef::BYTE; 512] } /* winnt.h:5949:16 */
pub type PCONTEXT = *mut ::winnt::CONTEXT; /* winnt.h:6035:18, winnt.h:3680:13, winnt.h:4669:13 */
#[repr(C)] pub struct LDT_ENTRY { LimitLow: ::minwindef::WORD, BaseLow: ::minwindef::WORD, HighWord: ::winnt::LDT_ENTRY_Child_2 } /* winnt.h:6047:16, winnt.h:6047:16, winnt.h:6047:16 */
#[repr(C)] pub /*union*/ struct LDT_ENTRY_Child_2; /* STUB! */ /* winnt.h:6050:5, winnt.h:6050:5, winnt.h:6050:5 */
pub type PLDT_ENTRY = *mut ::winnt::LDT_ENTRY; /* winnt.h:6070:15, winnt.h:6070:15, winnt.h:6070:15 */
#[repr(C)] pub struct WOW64_FLOATING_SAVE_AREA { ControlWord: ::minwindef::DWORD, StatusWord: ::minwindef::DWORD, TagWord: ::minwindef::DWORD, ErrorOffset: ::minwindef::DWORD, ErrorSelector: ::minwindef::DWORD, DataOffset: ::minwindef::DWORD, DataSelector: ::minwindef::DWORD, RegisterArea: *mut [::minwindef::BYTE; 80], Cr0NpxState: ::minwindef::DWORD } /* winnt.h:8821:16, winnt.h:8821:16, winnt.h:8821:16 */
pub type PWOW64_FLOATING_SAVE_AREA = *mut ::winnt::WOW64_FLOATING_SAVE_AREA; /* winnt.h:8833:35, winnt.h:8833:35, winnt.h:8833:35 */
#[repr(C)] pub struct WOW64_CONTEXT { ContextFlags: ::minwindef::DWORD, Dr0: ::minwindef::DWORD, Dr1: ::minwindef::DWORD, Dr2: ::minwindef::DWORD, Dr3: ::minwindef::DWORD, Dr6: ::minwindef::DWORD, Dr7: ::minwindef::DWORD, FloatSave: ::winnt::WOW64_FLOATING_SAVE_AREA, SegGs: ::minwindef::DWORD, SegFs: ::minwindef::DWORD, SegEs: ::minwindef::DWORD, SegDs: ::minwindef::DWORD, Edi: ::minwindef::DWORD, Esi: ::minwindef::DWORD, Ebx: ::minwindef::DWORD, Edx: ::minwindef::DWORD, Ecx: ::minwindef::DWORD, Eax: ::minwindef::DWORD, Ebp: ::minwindef::DWORD, Eip: ::minwindef::DWORD, SegCs: ::minwindef::DWORD, EFlags: ::minwindef::DWORD, Esp: ::minwindef::DWORD, SegSs: ::minwindef::DWORD, ExtendedRegisters: *mut [::minwindef::BYTE; 512] } /* winnt.h:8847:16, winnt.h:8847:16, winnt.h:8847:16 */
pub type PWOW64_CONTEXT = *mut ::winnt::WOW64_CONTEXT; /* winnt.h:8933:24, winnt.h:8933:24, winnt.h:8933:24 */
#[repr(C)] pub struct WOW64_LDT_ENTRY { LimitLow: ::minwindef::WORD, BaseLow: ::minwindef::WORD, HighWord: ::winnt::WOW64_LDT_ENTRY_Child_2 } /* winnt.h:8938:16, winnt.h:8938:16, winnt.h:8938:16 */
#[repr(C)] pub /*union*/ struct WOW64_LDT_ENTRY_Child_2; /* STUB! */ /* winnt.h:8941:5, winnt.h:8941:5, winnt.h:8941:5 */
pub type PWOW64_LDT_ENTRY = *mut ::winnt::WOW64_LDT_ENTRY; /* winnt.h:8961:21, winnt.h:8961:21, winnt.h:8961:21 */
#[repr(C)] pub struct WOW64_DESCRIPTOR_TABLE_ENTRY { Selector: ::minwindef::DWORD, Descriptor: ::winnt::WOW64_LDT_ENTRY } /* winnt.h:8963:16, winnt.h:8963:16, winnt.h:8963:16 */
pub type PWOW64_DESCRIPTOR_TABLE_ENTRY = *mut ::winnt::WOW64_DESCRIPTOR_TABLE_ENTRY; /* winnt.h:8966:34, winnt.h:8966:34, winnt.h:8966:34 */
#[repr(C)] pub struct EXCEPTION_RECORD { ExceptionCode: ::minwindef::DWORD, ExceptionFlags: ::minwindef::DWORD, ExceptionRecord: *mut ::winnt::EXCEPTION_RECORD, ExceptionAddress: ::winnt::PVOID, NumberParameters: ::minwindef::DWORD, ExceptionInformation: *mut [::basetsd::ULONG_PTR; 15] } /* winnt.h:8989:16, winnt.h:8989:16, winnt.h:8989:16 */
pub type PEXCEPTION_RECORD = *mut ::winnt::EXCEPTION_RECORD; /* winnt.h:8998:27, winnt.h:8998:27, winnt.h:8998:27 */
#[repr(C)] pub struct EXCEPTION_RECORD32 { ExceptionCode: ::minwindef::DWORD, ExceptionFlags: ::minwindef::DWORD, ExceptionRecord: ::minwindef::DWORD, ExceptionAddress: ::minwindef::DWORD, NumberParameters: ::minwindef::DWORD, ExceptionInformation: *mut [::minwindef::DWORD; 15] } /* winnt.h:9000:16, winnt.h:9000:16, winnt.h:9000:16 */
pub type PEXCEPTION_RECORD32 = *mut ::winnt::EXCEPTION_RECORD32; /* winnt.h:9007:24, winnt.h:9007:24, winnt.h:9007:24 */
#[repr(C)] pub struct EXCEPTION_RECORD64 { ExceptionCode: ::minwindef::DWORD, ExceptionFlags: ::minwindef::DWORD, ExceptionRecord: ::basetsd::DWORD64, ExceptionAddress: ::basetsd::DWORD64, NumberParameters: ::minwindef::DWORD, __unusedAlignment: ::minwindef::DWORD, ExceptionInformation: *mut [::basetsd::DWORD64; 15] } /* winnt.h:9009:16, winnt.h:9009:16, winnt.h:9009:16 */
pub type PEXCEPTION_RECORD64 = *mut ::winnt::EXCEPTION_RECORD64; /* winnt.h:9017:24, winnt.h:9017:24, winnt.h:9017:24 */
#[repr(C)] pub struct EXCEPTION_POINTERS { ExceptionRecord: ::winnt::PEXCEPTION_RECORD, ContextRecord: ::winnt::PCONTEXT } /* winnt.h:9023:16, winnt.h:9023:16, winnt.h:9023:16 */
pub type PEXCEPTION_POINTERS = *mut ::winnt::EXCEPTION_POINTERS; /* winnt.h:9026:24, winnt.h:9026:24, winnt.h:9026:24 */
pub type PACCESS_TOKEN = ::winnt::PVOID; /* winnt.h:9047:15, winnt.h:9047:15, winnt.h:9047:15 */
pub type PSECURITY_DESCRIPTOR = ::winnt::PVOID; /* winnt.h:9048:15, winnt.h:9048:15, winnt.h:9048:15 */
pub type PSID = ::winnt::PVOID; /* winnt.h:9049:15, winnt.h:9049:15, winnt.h:9049:15 */
pub type PCLAIMS_BLOB = ::winnt::PVOID; /* winnt.h:9050:15, winnt.h:9050:15, winnt.h:9050:15 */
pub type ACCESS_MASK = ::minwindef::DWORD; /* winnt.h:9091:15, winnt.h:9091:15, winnt.h:9091:15 */
pub type PACCESS_MASK = *mut ::libc::c_ulong; /* winnt.h:9092:22, winnt.h:9092:22, winnt.h:9092:22 */
#[repr(C)] pub struct GENERIC_MAPPING { GenericRead: ::winnt::ACCESS_MASK, GenericWrite: ::winnt::ACCESS_MASK, GenericExecute: ::winnt::ACCESS_MASK, GenericAll: ::winnt::ACCESS_MASK } /* winnt.h:9149:16, winnt.h:9149:16, winnt.h:9149:16 */
pub type PGENERIC_MAPPING = *mut ::winnt::GENERIC_MAPPING; /* winnt.h:9155:26, winnt.h:9155:26, winnt.h:9155:26 */
#[repr(C)] pub struct LUID_AND_ATTRIBUTES { Luid: ::winnt::LUID, Attributes: ::minwindef::DWORD } /* winnt.h:9170:16, winnt.h:9170:16, winnt.h:9170:16 */
pub type PLUID_AND_ATTRIBUTES = *mut ::winnt::LUID_AND_ATTRIBUTES; /* winnt.h:9173:30, winnt.h:9173:30, winnt.h:9173:30 */
pub type LUID_AND_ATTRIBUTES_ARRAY = *mut [::winnt::LUID_AND_ATTRIBUTES; 1]; /* winnt.h:9174:29, winnt.h:9174:29, winnt.h:9174:29 */
pub type PLUID_AND_ATTRIBUTES_ARRAY = *mut *mut [::winnt::LUID_AND_ATTRIBUTES; 1]; /* winnt.h:9175:36, winnt.h:9175:36, winnt.h:9175:36 */
#[repr(C)] pub struct SID_IDENTIFIER_AUTHORITY { Value: *mut [::minwindef::BYTE; 6] } /* winnt.h:9212:16, winnt.h:9212:16, winnt.h:9212:16 */
pub type PSID_IDENTIFIER_AUTHORITY = *mut ::winnt::SID_IDENTIFIER_AUTHORITY; /* winnt.h:9214:30, winnt.h:9214:30, winnt.h:9214:30 */
#[repr(C)] pub struct SID { Revision: ::minwindef::BYTE, SubAuthorityCount: ::minwindef::BYTE, IdentifierAuthority: ::winnt::SID_IDENTIFIER_AUTHORITY, SubAuthority: *mut [::minwindef::DWORD; 1] } /* winnt.h:9220:16, winnt.h:9220:16, winnt.h:9220:16 */
pub type PISID = *mut ::winnt::SID; /* winnt.h:9229:9, winnt.h:9229:9, winnt.h:9229:9 */
#[repr(C)] pub enum SID_NAME_USE {SidTypeUser = 1, SidTypeGroup = 2, SidTypeDomain = 3, SidTypeAlias = 4, SidTypeWellKnownGroup = 5, SidTypeDeletedAccount = 6, SidTypeInvalid = 7, SidTypeUnknown = 8, SidTypeComputer = 9, SidTypeLabel = 10} pub use self::SID_NAME_USE::{SidTypeUser, SidTypeGroup, SidTypeDomain, SidTypeAlias, SidTypeWellKnownGroup, SidTypeDeletedAccount, SidTypeInvalid, SidTypeUnknown, SidTypeComputer, SidTypeLabel}; /* winnt.h:9243:14, winnt.h:9243:14, winnt.h:9243:14 */
pub type PSID_NAME_USE = *mut ::winnt::SID_NAME_USE; /* winnt.h:9254:18, winnt.h:9254:18, winnt.h:9254:18 */
#[repr(C)] pub struct SID_AND_ATTRIBUTES { Sid: ::winnt::PSID, Attributes: ::minwindef::DWORD } /* winnt.h:9256:16, winnt.h:9256:16, winnt.h:9256:16 */
pub type PSID_AND_ATTRIBUTES = *mut ::winnt::SID_AND_ATTRIBUTES; /* winnt.h:9263:29, winnt.h:9263:29, winnt.h:9263:29 */
pub type SID_AND_ATTRIBUTES_ARRAY = *mut [::winnt::SID_AND_ATTRIBUTES; 1]; /* winnt.h:9265:28, winnt.h:9265:28, winnt.h:9265:28 */
pub type PSID_AND_ATTRIBUTES_ARRAY = *mut *mut [::winnt::SID_AND_ATTRIBUTES; 1]; /* winnt.h:9266:35, winnt.h:9266:35, winnt.h:9266:35 */
pub type SID_HASH_ENTRY = ::basetsd::ULONG_PTR; /* winnt.h:9269:19, winnt.h:9269:19, winnt.h:9269:19 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PSID_HASH_ENTRY = *mut ::libc::c_ulong; /* winnt.h:9269:36, winnt.h:9269:36 */
#[repr(C)] pub struct SID_AND_ATTRIBUTES_HASH { SidCount: ::minwindef::DWORD, SidAttr: ::winnt::PSID_AND_ATTRIBUTES, Hash: *mut [::winnt::SID_HASH_ENTRY; 32] } /* winnt.h:9271:16, winnt.h:9271:16, winnt.h:9271:16 */
pub type PSID_AND_ATTRIBUTES_HASH = *mut ::winnt::SID_AND_ATTRIBUTES_HASH; /* winnt.h:9275:29, winnt.h:9275:29, winnt.h:9275:29 */
#[repr(C)] pub enum WELL_KNOWN_SID_TYPE {WinNullSid = 0, WinWorldSid = 1, WinLocalSid = 2, WinCreatorOwnerSid = 3, WinCreatorGroupSid = 4, WinCreatorOwnerServerSid = 5, WinCreatorGroupServerSid = 6, WinNtAuthoritySid = 7, WinDialupSid = 8, WinNetworkSid = 9, WinBatchSid = 10, WinInteractiveSid = 11, WinServiceSid = 12, WinAnonymousSid = 13, WinProxySid = 14, WinEnterpriseControllersSid = 15, WinSelfSid = 16, WinAuthenticatedUserSid = 17, WinRestrictedCodeSid = 18, WinTerminalServerSid = 19, WinRemoteLogonIdSid = 20, WinLogonIdsSid = 21, WinLocalSystemSid = 22, WinLocalServiceSid = 23, WinNetworkServiceSid = 24, WinBuiltinDomainSid = 25, WinBuiltinAdministratorsSid = 26, WinBuiltinUsersSid = 27, WinBuiltinGuestsSid = 28, WinBuiltinPowerUsersSid = 29, WinBuiltinAccountOperatorsSid = 30, WinBuiltinSystemOperatorsSid = 31, WinBuiltinPrintOperatorsSid = 32, WinBuiltinBackupOperatorsSid = 33, WinBuiltinReplicatorSid = 34, WinBuiltinPreWindows2000CompatibleAccessSid = 35, WinBuiltinRemoteDesktopUsersSid = 36, WinBuiltinNetworkConfigurationOperatorsSid = 37, WinAccountAdministratorSid = 38, WinAccountGuestSid = 39, WinAccountKrbtgtSid = 40, WinAccountDomainAdminsSid = 41, WinAccountDomainUsersSid = 42, WinAccountDomainGuestsSid = 43, WinAccountComputersSid = 44, WinAccountControllersSid = 45, WinAccountCertAdminsSid = 46, WinAccountSchemaAdminsSid = 47, WinAccountEnterpriseAdminsSid = 48, WinAccountPolicyAdminsSid = 49, WinAccountRasAndIasServersSid = 50, WinNTLMAuthenticationSid = 51, WinDigestAuthenticationSid = 52, WinSChannelAuthenticationSid = 53, WinThisOrganizationSid = 54, WinOtherOrganizationSid = 55, WinBuiltinIncomingForestTrustBuildersSid = 56, WinBuiltinPerfMonitoringUsersSid = 57, WinBuiltinPerfLoggingUsersSid = 58, WinBuiltinAuthorizationAccessSid = 59, WinBuiltinTerminalServerLicenseServersSid = 60, WinBuiltinDCOMUsersSid = 61, WinBuiltinIUsersSid = 62, WinIUserSid = 63, WinBuiltinCryptoOperatorsSid = 64, WinUntrustedLabelSid = 65, WinLowLabelSid = 66, WinMediumLabelSid = 67, WinHighLabelSid = 68, WinSystemLabelSid = 69, WinWriteRestrictedCodeSid = 70, WinCreatorOwnerRightsSid = 71, WinCacheablePrincipalsGroupSid = 72, WinNonCacheablePrincipalsGroupSid = 73, WinEnterpriseReadonlyControllersSid = 74, WinAccountReadonlyControllersSid = 75, WinBuiltinEventLogReadersGroup = 76, WinNewEnterpriseReadonlyControllersSid = 77, WinBuiltinCertSvcDComAccessGroup = 78, WinMediumPlusLabelSid = 79, WinLocalLogonSid = 80, WinConsoleLogonSid = 81, WinThisOrganizationCertificateSid = 82, WinApplicationPackageAuthoritySid = 83, WinBuiltinAnyPackageSid = 84, WinCapabilityInternetClientSid = 85, WinCapabilityInternetClientServerSid = 86, WinCapabilityPrivateNetworkClientServerSid = 87, WinCapabilityPicturesLibrarySid = 88, WinCapabilityVideosLibrarySid = 89, WinCapabilityMusicLibrarySid = 90, WinCapabilityDocumentsLibrarySid = 91, WinCapabilitySharedUserCertificatesSid = 92, WinCapabilityEnterpriseAuthenticationSid = 93, WinCapabilityRemovableStorageSid = 94, WinBuiltinRDSRemoteAccessServersSid = 95, WinBuiltinRDSEndpointServersSid = 96, WinBuiltinRDSManagementServersSid = 97, WinUserModeDriversSid = 98, WinBuiltinHyperVAdminsSid = 99, WinAccountCloneableControllersSid = 100, WinBuiltinAccessControlAssistanceOperatorsSid = 101, WinBuiltinRemoteManagementUsersSid = 102, WinAuthenticationAuthorityAssertedSid = 103, WinAuthenticationServiceAssertedSid = 104, WinLocalAccountSid = 105, WinLocalAccountAndAdministratorSid = 106, WinAccountProtectedUsersSid = 107} pub use self::WELL_KNOWN_SID_TYPE::{WinNullSid, WinWorldSid, WinLocalSid, WinCreatorOwnerSid, WinCreatorGroupSid, WinCreatorOwnerServerSid, WinCreatorGroupServerSid, WinNtAuthoritySid, WinDialupSid, WinNetworkSid, WinBatchSid, WinInteractiveSid, WinServiceSid, WinAnonymousSid, WinProxySid, WinEnterpriseControllersSid, WinSelfSid, WinAuthenticatedUserSid, WinRestrictedCodeSid, WinTerminalServerSid, WinRemoteLogonIdSid, WinLogonIdsSid, WinLocalSystemSid, WinLocalServiceSid, WinNetworkServiceSid, WinBuiltinDomainSid, WinBuiltinAdministratorsSid, WinBuiltinUsersSid, WinBuiltinGuestsSid, WinBuiltinPowerUsersSid, WinBuiltinAccountOperatorsSid, WinBuiltinSystemOperatorsSid, WinBuiltinPrintOperatorsSid, WinBuiltinBackupOperatorsSid, WinBuiltinReplicatorSid, WinBuiltinPreWindows2000CompatibleAccessSid, WinBuiltinRemoteDesktopUsersSid, WinBuiltinNetworkConfigurationOperatorsSid, WinAccountAdministratorSid, WinAccountGuestSid, WinAccountKrbtgtSid, WinAccountDomainAdminsSid, WinAccountDomainUsersSid, WinAccountDomainGuestsSid, WinAccountComputersSid, WinAccountControllersSid, WinAccountCertAdminsSid, WinAccountSchemaAdminsSid, WinAccountEnterpriseAdminsSid, WinAccountPolicyAdminsSid, WinAccountRasAndIasServersSid, WinNTLMAuthenticationSid, WinDigestAuthenticationSid, WinSChannelAuthenticationSid, WinThisOrganizationSid, WinOtherOrganizationSid, WinBuiltinIncomingForestTrustBuildersSid, WinBuiltinPerfMonitoringUsersSid, WinBuiltinPerfLoggingUsersSid, WinBuiltinAuthorizationAccessSid, WinBuiltinTerminalServerLicenseServersSid, WinBuiltinDCOMUsersSid, WinBuiltinIUsersSid, WinIUserSid, WinBuiltinCryptoOperatorsSid, WinUntrustedLabelSid, WinLowLabelSid, WinMediumLabelSid, WinHighLabelSid, WinSystemLabelSid, WinWriteRestrictedCodeSid, WinCreatorOwnerRightsSid, WinCacheablePrincipalsGroupSid, WinNonCacheablePrincipalsGroupSid, WinEnterpriseReadonlyControllersSid, WinAccountReadonlyControllersSid, WinBuiltinEventLogReadersGroup, WinNewEnterpriseReadonlyControllersSid, WinBuiltinCertSvcDComAccessGroup, WinMediumPlusLabelSid, WinLocalLogonSid, WinConsoleLogonSid, WinThisOrganizationCertificateSid, WinApplicationPackageAuthoritySid, WinBuiltinAnyPackageSid, WinCapabilityInternetClientSid, WinCapabilityInternetClientServerSid, WinCapabilityPrivateNetworkClientServerSid, WinCapabilityPicturesLibrarySid, WinCapabilityVideosLibrarySid, WinCapabilityMusicLibrarySid, WinCapabilityDocumentsLibrarySid, WinCapabilitySharedUserCertificatesSid, WinCapabilityEnterpriseAuthenticationSid, WinCapabilityRemovableStorageSid, WinBuiltinRDSRemoteAccessServersSid, WinBuiltinRDSEndpointServersSid, WinBuiltinRDSManagementServersSid, WinUserModeDriversSid, WinBuiltinHyperVAdminsSid, WinAccountCloneableControllersSid, WinBuiltinAccessControlAssistanceOperatorsSid, WinBuiltinRemoteManagementUsersSid, WinAuthenticationAuthorityAssertedSid, WinAuthenticationServiceAssertedSid, WinLocalAccountSid, WinLocalAccountAndAdministratorSid, WinAccountProtectedUsersSid}; /* winnt.h:9638:9, winnt.h:9638:9, winnt.h:9638:9 */
#[repr(C)] pub struct ACL { AclRevision: ::minwindef::BYTE, Sbz1: ::minwindef::BYTE, AclSize: ::minwindef::WORD, AceCount: ::minwindef::WORD, Sbz2: ::minwindef::WORD } /* winnt.h:9845:16, winnt.h:9845:16, winnt.h:9845:16 */
pub type PACL = *mut ::winnt::ACL; /* winnt.h:9852:14, winnt.h:9852:14, winnt.h:9852:14 */
#[repr(C)] pub struct ACE_HEADER { AceType: ::minwindef::BYTE, AceFlags: ::minwindef::BYTE, AceSize: ::minwindef::WORD } /* winnt.h:9875:16, winnt.h:9875:16, winnt.h:9875:16 */
pub type PACE_HEADER = *mut ::winnt::ACE_HEADER; /* winnt.h:9880:21, winnt.h:9880:21, winnt.h:9880:21 */
#[repr(C)] pub struct ACCESS_ALLOWED_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, SidStart: ::minwindef::DWORD } /* winnt.h:9989:16, winnt.h:9989:16, winnt.h:9989:16 */
pub type PACCESS_ALLOWED_ACE = *mut ::winnt::ACCESS_ALLOWED_ACE; /* winnt.h:9995:29, winnt.h:9995:29, winnt.h:9995:29 */
#[repr(C)] pub struct ACCESS_DENIED_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, SidStart: ::minwindef::DWORD } /* winnt.h:9997:16, winnt.h:9997:16, winnt.h:9997:16 */
pub type PACCESS_DENIED_ACE = *mut ::winnt::ACCESS_DENIED_ACE; /* winnt.h:10002:28, winnt.h:10002:28, winnt.h:10002:28 */
#[repr(C)] pub struct SYSTEM_AUDIT_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, SidStart: ::minwindef::DWORD } /* winnt.h:10004:16, winnt.h:10004:16, winnt.h:10004:16 */
pub type PSYSTEM_AUDIT_ACE = *mut ::winnt::SYSTEM_AUDIT_ACE; /* winnt.h:10009:27, winnt.h:10009:27, winnt.h:10009:27 */
#[repr(C)] pub struct SYSTEM_ALARM_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, SidStart: ::minwindef::DWORD } /* winnt.h:10011:16, winnt.h:10011:16, winnt.h:10011:16 */
pub type PSYSTEM_ALARM_ACE = *mut ::winnt::SYSTEM_ALARM_ACE; /* winnt.h:10016:27, winnt.h:10016:27, winnt.h:10016:27 */
#[repr(C)] pub struct SYSTEM_RESOURCE_ATTRIBUTE_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, SidStart: ::minwindef::DWORD } /* winnt.h:10018:16, winnt.h:10018:16, winnt.h:10018:16 */
pub type PSYSTEM_RESOURCE_ATTRIBUTE_ACE = *mut ::winnt::SYSTEM_RESOURCE_ATTRIBUTE_ACE; /* winnt.h:10023:35, winnt.h:10023:35, winnt.h:10023:35 */
#[repr(C)] pub struct SYSTEM_SCOPED_POLICY_ID_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, SidStart: ::minwindef::DWORD } /* winnt.h:10025:16, winnt.h:10025:16, winnt.h:10025:16 */
pub type PSYSTEM_SCOPED_POLICY_ID_ACE = *mut ::winnt::SYSTEM_SCOPED_POLICY_ID_ACE; /* winnt.h:10029:33, winnt.h:10029:33, winnt.h:10029:33 */
#[repr(C)] pub struct SYSTEM_MANDATORY_LABEL_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, SidStart: ::minwindef::DWORD } /* winnt.h:10031:16, winnt.h:10031:16, winnt.h:10031:16 */
pub type PSYSTEM_MANDATORY_LABEL_ACE = *mut ::winnt::SYSTEM_MANDATORY_LABEL_ACE; /* winnt.h:10035:32, winnt.h:10035:32, winnt.h:10035:32 */
#[repr(C)] pub struct SYSTEM_PROCESS_TRUST_LABEL_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, SidStart: ::minwindef::DWORD } /* winnt.h:10037:16, winnt.h:10037:16, winnt.h:10037:16 */
pub type PSYSTEM_PROCESS_TRUST_LABEL_ACE = *mut ::winnt::SYSTEM_PROCESS_TRUST_LABEL_ACE; /* winnt.h:10041:36, winnt.h:10041:36, winnt.h:10041:36 */
#[repr(C)] pub struct ACCESS_ALLOWED_OBJECT_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, Flags: ::minwindef::DWORD, ObjectType: ::guiddef::GUID, InheritedObjectType: ::guiddef::GUID, SidStart: ::minwindef::DWORD } /* winnt.h:10057:16, winnt.h:10057:16, winnt.h:10057:16 */
pub type PACCESS_ALLOWED_OBJECT_ACE = *mut ::winnt::ACCESS_ALLOWED_OBJECT_ACE; /* winnt.h:10064:31, winnt.h:10064:31, winnt.h:10064:31 */
#[repr(C)] pub struct ACCESS_DENIED_OBJECT_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, Flags: ::minwindef::DWORD, ObjectType: ::guiddef::GUID, InheritedObjectType: ::guiddef::GUID, SidStart: ::minwindef::DWORD } /* winnt.h:10066:16, winnt.h:10066:16, winnt.h:10066:16 */
pub type PACCESS_DENIED_OBJECT_ACE = *mut ::winnt::ACCESS_DENIED_OBJECT_ACE; /* winnt.h:10073:30, winnt.h:10073:30, winnt.h:10073:30 */
#[repr(C)] pub struct SYSTEM_AUDIT_OBJECT_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, Flags: ::minwindef::DWORD, ObjectType: ::guiddef::GUID, InheritedObjectType: ::guiddef::GUID, SidStart: ::minwindef::DWORD } /* winnt.h:10075:16, winnt.h:10075:16, winnt.h:10075:16 */
pub type PSYSTEM_AUDIT_OBJECT_ACE = *mut ::winnt::SYSTEM_AUDIT_OBJECT_ACE; /* winnt.h:10082:29, winnt.h:10082:29, winnt.h:10082:29 */
#[repr(C)] pub struct SYSTEM_ALARM_OBJECT_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, Flags: ::minwindef::DWORD, ObjectType: ::guiddef::GUID, InheritedObjectType: ::guiddef::GUID, SidStart: ::minwindef::DWORD } /* winnt.h:10084:16, winnt.h:10084:16, winnt.h:10084:16 */
pub type PSYSTEM_ALARM_OBJECT_ACE = *mut ::winnt::SYSTEM_ALARM_OBJECT_ACE; /* winnt.h:10091:29, winnt.h:10091:29, winnt.h:10091:29 */
#[repr(C)] pub struct ACCESS_ALLOWED_CALLBACK_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, SidStart: ::minwindef::DWORD } /* winnt.h:10098:16, winnt.h:10098:16, winnt.h:10098:16 */
pub type PACCESS_ALLOWED_CALLBACK_ACE = *mut ::winnt::ACCESS_ALLOWED_CALLBACK_ACE; /* winnt.h:10103:33, winnt.h:10103:33, winnt.h:10103:33 */
#[repr(C)] pub struct ACCESS_DENIED_CALLBACK_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, SidStart: ::minwindef::DWORD } /* winnt.h:10105:16, winnt.h:10105:16, winnt.h:10105:16 */
pub type PACCESS_DENIED_CALLBACK_ACE = *mut ::winnt::ACCESS_DENIED_CALLBACK_ACE; /* winnt.h:10110:32, winnt.h:10110:32, winnt.h:10110:32 */
#[repr(C)] pub struct SYSTEM_AUDIT_CALLBACK_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, SidStart: ::minwindef::DWORD } /* winnt.h:10112:16, winnt.h:10112:16, winnt.h:10112:16 */
pub type PSYSTEM_AUDIT_CALLBACK_ACE = *mut ::winnt::SYSTEM_AUDIT_CALLBACK_ACE; /* winnt.h:10117:31, winnt.h:10117:31, winnt.h:10117:31 */
#[repr(C)] pub struct SYSTEM_ALARM_CALLBACK_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, SidStart: ::minwindef::DWORD } /* winnt.h:10119:16, winnt.h:10119:16, winnt.h:10119:16 */
pub type PSYSTEM_ALARM_CALLBACK_ACE = *mut ::winnt::SYSTEM_ALARM_CALLBACK_ACE; /* winnt.h:10124:31, winnt.h:10124:31, winnt.h:10124:31 */
#[repr(C)] pub struct ACCESS_ALLOWED_CALLBACK_OBJECT_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, Flags: ::minwindef::DWORD, ObjectType: ::guiddef::GUID, InheritedObjectType: ::guiddef::GUID, SidStart: ::minwindef::DWORD } /* winnt.h:10126:16, winnt.h:10126:16, winnt.h:10126:16 */
pub type PACCESS_ALLOWED_CALLBACK_OBJECT_ACE = *mut ::winnt::ACCESS_ALLOWED_CALLBACK_OBJECT_ACE; /* winnt.h:10134:40, winnt.h:10134:40, winnt.h:10134:40 */
#[repr(C)] pub struct ACCESS_DENIED_CALLBACK_OBJECT_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, Flags: ::minwindef::DWORD, ObjectType: ::guiddef::GUID, InheritedObjectType: ::guiddef::GUID, SidStart: ::minwindef::DWORD } /* winnt.h:10136:16, winnt.h:10136:16, winnt.h:10136:16 */
pub type PACCESS_DENIED_CALLBACK_OBJECT_ACE = *mut ::winnt::ACCESS_DENIED_CALLBACK_OBJECT_ACE; /* winnt.h:10144:39, winnt.h:10144:39, winnt.h:10144:39 */
#[repr(C)] pub struct SYSTEM_AUDIT_CALLBACK_OBJECT_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, Flags: ::minwindef::DWORD, ObjectType: ::guiddef::GUID, InheritedObjectType: ::guiddef::GUID, SidStart: ::minwindef::DWORD } /* winnt.h:10146:16, winnt.h:10146:16, winnt.h:10146:16 */
pub type PSYSTEM_AUDIT_CALLBACK_OBJECT_ACE = *mut ::winnt::SYSTEM_AUDIT_CALLBACK_OBJECT_ACE; /* winnt.h:10154:38, winnt.h:10154:38, winnt.h:10154:38 */
#[repr(C)] pub struct SYSTEM_ALARM_CALLBACK_OBJECT_ACE { Header: ::winnt::ACE_HEADER, Mask: ::winnt::ACCESS_MASK, Flags: ::minwindef::DWORD, ObjectType: ::guiddef::GUID, InheritedObjectType: ::guiddef::GUID, SidStart: ::minwindef::DWORD } /* winnt.h:10156:16, winnt.h:10156:16, winnt.h:10156:16 */
pub type PSYSTEM_ALARM_CALLBACK_OBJECT_ACE = *mut ::winnt::SYSTEM_ALARM_CALLBACK_OBJECT_ACE; /* winnt.h:10164:38, winnt.h:10164:38, winnt.h:10164:38 */
#[repr(C)] pub enum ACL_INFORMATION_CLASS {AclRevisionInformation = 1, AclSizeInformation = 2} pub use self::ACL_INFORMATION_CLASS::{AclRevisionInformation, AclSizeInformation}; /* winnt.h:10180:14, winnt.h:10180:14, winnt.h:10180:14 */
#[repr(C)] pub struct ACL_REVISION_INFORMATION { AclRevision: ::minwindef::DWORD } /* winnt.h:10190:16, winnt.h:10190:16, winnt.h:10190:16 */
pub type PACL_REVISION_INFORMATION = *mut ::winnt::ACL_REVISION_INFORMATION; /* winnt.h:10193:35, winnt.h:10193:35, winnt.h:10193:35 */
#[repr(C)] pub struct ACL_SIZE_INFORMATION { AceCount: ::minwindef::DWORD, AclBytesInUse: ::minwindef::DWORD, AclBytesFree: ::minwindef::DWORD } /* winnt.h:10199:16, winnt.h:10199:16, winnt.h:10199:16 */
pub type PACL_SIZE_INFORMATION = *mut ::winnt::ACL_SIZE_INFORMATION; /* winnt.h:10204:31, winnt.h:10204:31, winnt.h:10204:31 */
pub type SECURITY_DESCRIPTOR_CONTROL = ::minwindef::WORD; /* winnt.h:10231:16, winnt.h:10231:16, winnt.h:10231:16 */
pub type PSECURITY_DESCRIPTOR_CONTROL = *mut ::libc::c_ushort; /* winnt.h:10231:46, winnt.h:10231:46, winnt.h:10231:46 */
#[repr(C)] pub struct SECURITY_DESCRIPTOR_RELATIVE { Revision: ::minwindef::BYTE, Sbz1: ::minwindef::BYTE, Control: ::winnt::SECURITY_DESCRIPTOR_CONTROL, Owner: ::minwindef::DWORD, Group: ::minwindef::DWORD, Sacl: ::minwindef::DWORD, Dacl: ::minwindef::DWORD } /* winnt.h:10321:16, winnt.h:10321:16, winnt.h:10321:16 */
pub type PISECURITY_DESCRIPTOR_RELATIVE = *mut ::winnt::SECURITY_DESCRIPTOR_RELATIVE; /* winnt.h:10329:38, winnt.h:10329:38, winnt.h:10329:38 */
#[repr(C)] pub struct SECURITY_DESCRIPTOR { Revision: ::minwindef::BYTE, Sbz1: ::minwindef::BYTE, Control: ::winnt::SECURITY_DESCRIPTOR_CONTROL, Owner: ::winnt::PSID, Group: ::winnt::PSID, Sacl: ::winnt::PACL, Dacl: ::winnt::PACL } /* winnt.h:10331:16, winnt.h:10331:16, winnt.h:10331:16 */
pub type PISECURITY_DESCRIPTOR = *mut ::winnt::SECURITY_DESCRIPTOR; /* winnt.h:10340:28, winnt.h:10340:28, winnt.h:10340:28 */
#[repr(C)] pub struct SECURITY_OBJECT_AI_PARAMS { Size: ::minwindef::DWORD, ConstraintMask: ::minwindef::DWORD } /* winnt.h:10343:16, winnt.h:10343:16, winnt.h:10343:16 */
pub type PSECURITY_OBJECT_AI_PARAMS = *mut ::winnt::SECURITY_OBJECT_AI_PARAMS; /* winnt.h:10346:31, winnt.h:10346:31, winnt.h:10346:31 */
#[repr(C)] pub struct OBJECT_TYPE_LIST { Level: ::minwindef::WORD, Sbz: ::minwindef::WORD, ObjectType: *mut ::guiddef::GUID } /* winnt.h:10398:16, winnt.h:10398:16, winnt.h:10398:16 */
pub type POBJECT_TYPE_LIST = *mut ::winnt::OBJECT_TYPE_LIST; /* winnt.h:10402:22, winnt.h:10402:22, winnt.h:10402:22 */
#[repr(C)] pub enum AUDIT_EVENT_TYPE {AuditEventObjectAccess = 0, AuditEventDirectoryServiceAccess = 1} pub use self::AUDIT_EVENT_TYPE::{AuditEventObjectAccess, AuditEventDirectoryServiceAccess}; /* winnt.h:10418:14, winnt.h:10418:14, winnt.h:10418:14 */
pub type PAUDIT_EVENT_TYPE = *mut ::winnt::AUDIT_EVENT_TYPE; /* winnt.h:10421:22, winnt.h:10421:22, winnt.h:10421:22 */
#[repr(C)] pub struct PRIVILEGE_SET { PrivilegeCount: ::minwindef::DWORD, Control: ::minwindef::DWORD, Privilege: *mut [::winnt::LUID_AND_ATTRIBUTES; 1] } /* winnt.h:10472:16, winnt.h:10472:16, winnt.h:10472:16 */
pub type PPRIVILEGE_SET = *mut ::winnt::PRIVILEGE_SET; /* winnt.h:10476:24, winnt.h:10476:24, winnt.h:10476:24 */
#[repr(C)] pub enum ACCESS_REASON_TYPE {AccessReasonNone = 0, AccessReasonAllowedAce = 65536, AccessReasonDeniedAce = 131072, AccessReasonAllowedParentAce = 196608, AccessReasonDeniedParentAce = 262144, AccessReasonNotGrantedByCape = 327680, AccessReasonNotGrantedByParentCape = 393216, AccessReasonNotGrantedToAppContainer = 458752, AccessReasonMissingPrivilege = 1048576, AccessReasonFromPrivilege = 2097152, AccessReasonIntegrityLevel = 3145728, AccessReasonOwnership = 4194304, AccessReasonNullDacl = 5242880, AccessReasonEmptyDacl = 6291456, AccessReasonNoSD = 7340032, AccessReasonNoGrant = 8388608, AccessReasonTrustLabel = 9437184} pub use self::ACCESS_REASON_TYPE::{AccessReasonNone, AccessReasonAllowedAce, AccessReasonDeniedAce, AccessReasonAllowedParentAce, AccessReasonDeniedParentAce, AccessReasonNotGrantedByCape, AccessReasonNotGrantedByParentCape, AccessReasonNotGrantedToAppContainer, AccessReasonMissingPrivilege, AccessReasonFromPrivilege, AccessReasonIntegrityLevel, AccessReasonOwnership, AccessReasonNullDacl, AccessReasonEmptyDacl, AccessReasonNoSD, AccessReasonNoGrant, AccessReasonTrustLabel}; /* winnt.h:10493:14, winnt.h:10493:14, winnt.h:10493:14 */
pub type ACCESS_REASON = ::minwindef::DWORD; /* winnt.h:10544:15, winnt.h:10544:15, winnt.h:10544:15 */
#[repr(C)] pub struct ACCESS_REASONS { Data: *mut [::winnt::ACCESS_REASON; 32] } /* winnt.h:10546:16, winnt.h:10546:16, winnt.h:10546:16 */
pub type PACCESS_REASONS = *mut ::winnt::ACCESS_REASONS; /* winnt.h:10548:20, winnt.h:10548:20, winnt.h:10548:20 */
#[repr(C)] pub struct SE_SECURITY_DESCRIPTOR { Size: ::minwindef::DWORD, Flags: ::minwindef::DWORD, SecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR } /* winnt.h:10573:16, winnt.h:10573:16, winnt.h:10573:16 */
pub type PSE_SECURITY_DESCRIPTOR = *mut ::winnt::SE_SECURITY_DESCRIPTOR; /* winnt.h:10578:28, winnt.h:10578:28, winnt.h:10578:28 */
#[repr(C)] pub struct SE_ACCESS_REQUEST { Size: ::minwindef::DWORD, SeSecurityDescriptor: ::winnt::PSE_SECURITY_DESCRIPTOR, DesiredAccess: ::winnt::ACCESS_MASK, PreviouslyGrantedAccess: ::winnt::ACCESS_MASK, PrincipalSelfSid: ::winnt::PSID, GenericMapping: ::winnt::PGENERIC_MAPPING, ObjectTypeListCount: ::minwindef::DWORD, ObjectTypeList: ::winnt::POBJECT_TYPE_LIST } /* winnt.h:10580:16, winnt.h:10580:16, winnt.h:10580:16 */
pub type PSE_ACCESS_REQUEST = *mut ::winnt::SE_ACCESS_REQUEST; /* winnt.h:10590:23, winnt.h:10590:23, winnt.h:10590:23 */
#[repr(C)] pub struct SE_ACCESS_REPLY { Size: ::minwindef::DWORD, ResultListCount: ::minwindef::DWORD, GrantedAccess: ::winnt::PACCESS_MASK, AccessStatus: ::minwindef::PDWORD, AccessReason: ::winnt::PACCESS_REASONS, Privileges: *mut *mut ::winnt::PRIVILEGE_SET } /* winnt.h:10593:16, winnt.h:10593:16, winnt.h:10593:16 */
pub type PSE_ACCESS_REPLY = *mut ::winnt::SE_ACCESS_REPLY; /* winnt.h:10601:21, winnt.h:10601:21, winnt.h:10601:21 */
#[repr(C)] pub enum SECURITY_IMPERSONATION_LEVEL {SecurityAnonymous = 0, SecurityIdentification = 1, SecurityImpersonation = 2, SecurityDelegation = 3} pub use self::SECURITY_IMPERSONATION_LEVEL::{SecurityAnonymous, SecurityIdentification, SecurityImpersonation, SecurityDelegation}; /* winnt.h:10664:14, winnt.h:10664:14, winnt.h:10664:14 */
pub type PSECURITY_IMPERSONATION_LEVEL = *mut ::winnt::SECURITY_IMPERSONATION_LEVEL; /* winnt.h:10669:39, winnt.h:10669:39, winnt.h:10669:39 */
#[repr(C)] pub enum TOKEN_TYPE {TokenPrimary = 1, TokenImpersonation = 2} pub use self::TOKEN_TYPE::{TokenPrimary, TokenImpersonation}; /* winnt.h:10736:14, winnt.h:10736:14, winnt.h:10736:14 */
pub type PTOKEN_TYPE = *mut ::winnt::TOKEN_TYPE; /* winnt.h:10740:21, winnt.h:10740:21, winnt.h:10740:21 */
#[repr(C)] pub enum TOKEN_ELEVATION_TYPE {TokenElevationTypeDefault = 1, TokenElevationTypeFull = 2, TokenElevationTypeLimited = 3} pub use self::TOKEN_ELEVATION_TYPE::{TokenElevationTypeDefault, TokenElevationTypeFull, TokenElevationTypeLimited}; /* winnt.h:10748:14, winnt.h:10748:14, winnt.h:10748:14 */
pub type PTOKEN_ELEVATION_TYPE = *mut ::winnt::TOKEN_ELEVATION_TYPE; /* winnt.h:10752:26, winnt.h:10752:26, winnt.h:10752:26 */
#[repr(C)] pub enum TOKEN_INFORMATION_CLASS {TokenUser = 1, TokenGroups = 2, TokenPrivileges = 3, TokenOwner = 4, TokenPrimaryGroup = 5, TokenDefaultDacl = 6, TokenSource = 7, TokenType = 8, TokenImpersonationLevel = 9, TokenStatistics = 10, TokenRestrictedSids = 11, TokenSessionId = 12, TokenGroupsAndPrivileges = 13, TokenSessionReference = 14, TokenSandBoxInert = 15, TokenAuditPolicy = 16, TokenOrigin = 17, TokenElevationType = 18, TokenLinkedToken = 19, TokenElevation = 20, TokenHasRestrictions = 21, TokenAccessInformation = 22, TokenVirtualizationAllowed = 23, TokenVirtualizationEnabled = 24, TokenIntegrityLevel = 25, TokenUIAccess = 26, TokenMandatoryPolicy = 27, TokenLogonSid = 28, TokenIsAppContainer = 29, TokenCapabilities = 30, TokenAppContainerSid = 31, TokenAppContainerNumber = 32, TokenUserClaimAttributes = 33, TokenDeviceClaimAttributes = 34, TokenRestrictedUserClaimAttributes = 35, TokenRestrictedDeviceClaimAttributes = 36, TokenDeviceGroups = 37, TokenRestrictedDeviceGroups = 38, TokenSecurityAttributes = 39, TokenIsRestricted = 40, TokenProcessTrustLevel = 41, MaxTokenInfoClass = 42} pub use self::TOKEN_INFORMATION_CLASS::{TokenUser, TokenGroups, TokenPrivileges, TokenOwner, TokenPrimaryGroup, TokenDefaultDacl, TokenSource, TokenType, TokenImpersonationLevel, TokenStatistics, TokenRestrictedSids, TokenSessionId, TokenGroupsAndPrivileges, TokenSessionReference, TokenSandBoxInert, TokenAuditPolicy, TokenOrigin, TokenElevationType, TokenLinkedToken, TokenElevation, TokenHasRestrictions, TokenAccessInformation, TokenVirtualizationAllowed, TokenVirtualizationEnabled, TokenIntegrityLevel, TokenUIAccess, TokenMandatoryPolicy, TokenLogonSid, TokenIsAppContainer, TokenCapabilities, TokenAppContainerSid, TokenAppContainerNumber, TokenUserClaimAttributes, TokenDeviceClaimAttributes, TokenRestrictedUserClaimAttributes, TokenRestrictedDeviceClaimAttributes, TokenDeviceGroups, TokenRestrictedDeviceGroups, TokenSecurityAttributes, TokenIsRestricted, TokenProcessTrustLevel, MaxTokenInfoClass}; /* winnt.h:10759:14, winnt.h:10759:14, winnt.h:10759:14 */
pub type PTOKEN_INFORMATION_CLASS = *mut ::winnt::TOKEN_INFORMATION_CLASS; /* winnt.h:10802:29, winnt.h:10802:29, winnt.h:10802:29 */
#[repr(C)] pub struct TOKEN_USER { User: ::winnt::SID_AND_ATTRIBUTES } /* winnt.h:10809:16, winnt.h:10809:16, winnt.h:10809:16 */
pub type PTOKEN_USER = *mut ::winnt::TOKEN_USER; /* winnt.h:10811:16, winnt.h:10811:16, winnt.h:10811:16 */
#[repr(C)] pub struct TOKEN_GROUPS { GroupCount: ::minwindef::DWORD, Groups: *mut [::winnt::SID_AND_ATTRIBUTES; 1] } /* winnt.h:10813:16, winnt.h:10813:16, winnt.h:10813:16 */
pub type PTOKEN_GROUPS = *mut ::winnt::TOKEN_GROUPS; /* winnt.h:10820:18, winnt.h:10820:18, winnt.h:10820:18 */
#[repr(C)] pub struct TOKEN_PRIVILEGES { PrivilegeCount: ::minwindef::DWORD, Privileges: *mut [::winnt::LUID_AND_ATTRIBUTES; 1] } /* winnt.h:10823:16, winnt.h:10823:16, winnt.h:10823:16 */
pub type PTOKEN_PRIVILEGES = *mut ::winnt::TOKEN_PRIVILEGES; /* winnt.h:10826:22, winnt.h:10826:22, winnt.h:10826:22 */
#[repr(C)] pub struct TOKEN_OWNER { Owner: ::winnt::PSID } /* winnt.h:10829:16, winnt.h:10829:16, winnt.h:10829:16 */
pub type PTOKEN_OWNER = *mut ::winnt::TOKEN_OWNER; /* winnt.h:10831:17, winnt.h:10831:17, winnt.h:10831:17 */
#[repr(C)] pub struct TOKEN_PRIMARY_GROUP { PrimaryGroup: ::winnt::PSID } /* winnt.h:10834:16, winnt.h:10834:16, winnt.h:10834:16 */
pub type PTOKEN_PRIMARY_GROUP = *mut ::winnt::TOKEN_PRIMARY_GROUP; /* winnt.h:10836:25, winnt.h:10836:25, winnt.h:10836:25 */
#[repr(C)] pub struct TOKEN_DEFAULT_DACL { DefaultDacl: ::winnt::PACL } /* winnt.h:10839:16, winnt.h:10839:16, winnt.h:10839:16 */
pub type PTOKEN_DEFAULT_DACL = *mut ::winnt::TOKEN_DEFAULT_DACL; /* winnt.h:10841:24, winnt.h:10841:24, winnt.h:10841:24 */
#[repr(C)] pub struct TOKEN_USER_CLAIMS { UserClaims: ::winnt::PCLAIMS_BLOB } /* winnt.h:10843:16, winnt.h:10843:16, winnt.h:10843:16 */
pub type PTOKEN_USER_CLAIMS = *mut ::winnt::TOKEN_USER_CLAIMS; /* winnt.h:10845:23, winnt.h:10845:23, winnt.h:10845:23 */
#[repr(C)] pub struct TOKEN_DEVICE_CLAIMS { DeviceClaims: ::winnt::PCLAIMS_BLOB } /* winnt.h:10847:16, winnt.h:10847:16, winnt.h:10847:16 */
pub type PTOKEN_DEVICE_CLAIMS = *mut ::winnt::TOKEN_DEVICE_CLAIMS; /* winnt.h:10849:25, winnt.h:10849:25, winnt.h:10849:25 */
#[repr(C)] pub struct TOKEN_GROUPS_AND_PRIVILEGES { SidCount: ::minwindef::DWORD, SidLength: ::minwindef::DWORD, Sids: ::winnt::PSID_AND_ATTRIBUTES, RestrictedSidCount: ::minwindef::DWORD, RestrictedSidLength: ::minwindef::DWORD, RestrictedSids: ::winnt::PSID_AND_ATTRIBUTES, PrivilegeCount: ::minwindef::DWORD, PrivilegeLength: ::minwindef::DWORD, Privileges: ::winnt::PLUID_AND_ATTRIBUTES, AuthenticationId: ::winnt::LUID } /* winnt.h:10851:16, winnt.h:10851:16, winnt.h:10851:16 */
pub type PTOKEN_GROUPS_AND_PRIVILEGES = *mut ::winnt::TOKEN_GROUPS_AND_PRIVILEGES; /* winnt.h:10862:33, winnt.h:10862:33, winnt.h:10862:33 */
#[repr(C)] pub struct TOKEN_LINKED_TOKEN { LinkedToken: ::winnt::HANDLE } /* winnt.h:10864:16, winnt.h:10864:16, winnt.h:10864:16 */
pub type PTOKEN_LINKED_TOKEN = *mut ::winnt::TOKEN_LINKED_TOKEN; /* winnt.h:10866:24, winnt.h:10866:24, winnt.h:10866:24 */
#[repr(C)] pub struct TOKEN_ELEVATION { TokenIsElevated: ::minwindef::DWORD } /* winnt.h:10868:16, winnt.h:10868:16, winnt.h:10868:16 */
pub type PTOKEN_ELEVATION = *mut ::winnt::TOKEN_ELEVATION; /* winnt.h:10870:21, winnt.h:10870:21, winnt.h:10870:21 */
#[repr(C)] pub struct TOKEN_MANDATORY_LABEL { Label: ::winnt::SID_AND_ATTRIBUTES } /* winnt.h:10872:16, winnt.h:10872:16, winnt.h:10872:16 */
pub type PTOKEN_MANDATORY_LABEL = *mut ::winnt::TOKEN_MANDATORY_LABEL; /* winnt.h:10874:27, winnt.h:10874:27, winnt.h:10874:27 */
#[repr(C)] pub struct TOKEN_MANDATORY_POLICY { Policy: ::minwindef::DWORD } /* winnt.h:10883:16, winnt.h:10883:16, winnt.h:10883:16 */
pub type PTOKEN_MANDATORY_POLICY = *mut ::winnt::TOKEN_MANDATORY_POLICY; /* winnt.h:10885:28, winnt.h:10885:28, winnt.h:10885:28 */
#[repr(C)] pub struct TOKEN_ACCESS_INFORMATION { SidHash: ::winnt::PSID_AND_ATTRIBUTES_HASH, RestrictedSidHash: ::winnt::PSID_AND_ATTRIBUTES_HASH, Privileges: ::winnt::PTOKEN_PRIVILEGES, AuthenticationId: ::winnt::LUID, TokenType: ::winnt::TOKEN_TYPE, ImpersonationLevel: ::winnt::SECURITY_IMPERSONATION_LEVEL, MandatoryPolicy: ::winnt::TOKEN_MANDATORY_POLICY, Flags: ::minwindef::DWORD, AppContainerNumber: ::minwindef::DWORD, PackageSid: ::winnt::PSID, CapabilitiesHash: ::winnt::PSID_AND_ATTRIBUTES_HASH, TrustLevelSid: ::winnt::PSID } /* winnt.h:10887:16, winnt.h:10887:16, winnt.h:10887:16 */
pub type PTOKEN_ACCESS_INFORMATION = *mut ::winnt::TOKEN_ACCESS_INFORMATION; /* winnt.h:10900:30, winnt.h:10900:30, winnt.h:10900:30 */
#[repr(C)] pub struct TOKEN_AUDIT_POLICY { PerUserPolicy: *mut [::minwindef::BYTE; 29] } /* winnt.h:10908:16, winnt.h:10908:16, winnt.h:10908:16 */
pub type PTOKEN_AUDIT_POLICY = *mut ::winnt::TOKEN_AUDIT_POLICY; /* winnt.h:10910:24, winnt.h:10910:24, winnt.h:10910:24 */
#[repr(C)] pub struct TOKEN_SOURCE { SourceName: *mut [::winnt::CHAR; 8], SourceIdentifier: ::winnt::LUID } /* winnt.h:10914:16, winnt.h:10914:16, winnt.h:10914:16 */
pub type PTOKEN_SOURCE = *mut ::winnt::TOKEN_SOURCE; /* winnt.h:10917:18, winnt.h:10917:18, winnt.h:10917:18 */
#[repr(C)] pub struct TOKEN_STATISTICS { TokenId: ::winnt::LUID, AuthenticationId: ::winnt::LUID, ExpirationTime: ::winnt::LARGE_INTEGER, TokenType: ::winnt::TOKEN_TYPE, ImpersonationLevel: ::winnt::SECURITY_IMPERSONATION_LEVEL, DynamicCharged: ::minwindef::DWORD, DynamicAvailable: ::minwindef::DWORD, GroupCount: ::minwindef::DWORD, PrivilegeCount: ::minwindef::DWORD, ModifiedId: ::winnt::LUID } /* winnt.h:10920:16, winnt.h:10920:16, winnt.h:10920:16 */
pub type PTOKEN_STATISTICS = *mut ::winnt::TOKEN_STATISTICS; /* winnt.h:10931:22, winnt.h:10931:22, winnt.h:10931:22 */
#[repr(C)] pub struct TOKEN_CONTROL { TokenId: ::winnt::LUID, AuthenticationId: ::winnt::LUID, ModifiedId: ::winnt::LUID, TokenSource: ::winnt::TOKEN_SOURCE } /* winnt.h:10935:16, winnt.h:10935:16, winnt.h:10935:16 */
pub type PTOKEN_CONTROL = *mut ::winnt::TOKEN_CONTROL; /* winnt.h:10940:19, winnt.h:10940:19, winnt.h:10940:19 */
#[repr(C)] pub struct TOKEN_ORIGIN { OriginatingLogonSession: ::winnt::LUID } /* winnt.h:10942:16, winnt.h:10942:16, winnt.h:10942:16 */
pub type PTOKEN_ORIGIN = *mut ::winnt::TOKEN_ORIGIN; /* winnt.h:10944:19, winnt.h:10944:19, winnt.h:10944:19 */
#[repr(C)] pub enum MANDATORY_LEVEL {MandatoryLevelUntrusted = 0, MandatoryLevelLow = 1, MandatoryLevelMedium = 2, MandatoryLevelHigh = 3, MandatoryLevelSystem = 4, MandatoryLevelSecureProcess = 5, MandatoryLevelCount = 6} pub use self::MANDATORY_LEVEL::{MandatoryLevelUntrusted, MandatoryLevelLow, MandatoryLevelMedium, MandatoryLevelHigh, MandatoryLevelSystem, MandatoryLevelSecureProcess, MandatoryLevelCount}; /* winnt.h:10947:14, winnt.h:10947:14, winnt.h:10947:14 */
pub type PMANDATORY_LEVEL = *mut ::winnt::MANDATORY_LEVEL; /* winnt.h:10955:21, winnt.h:10955:21, winnt.h:10955:21 */
#[repr(C)] pub struct TOKEN_APPCONTAINER_INFORMATION { TokenAppContainer: ::winnt::PSID } /* winnt.h:10957:16, winnt.h:10957:16, winnt.h:10957:16 */
pub type PTOKEN_APPCONTAINER_INFORMATION = *mut ::winnt::TOKEN_APPCONTAINER_INFORMATION; /* winnt.h:10959:36, winnt.h:10959:36, winnt.h:10959:36 */
#[repr(C)] pub struct TOKEN_SID_INFORMATION { Sid: ::winnt::PSID } /* winnt.h:10961:16, winnt.h:10961:16, winnt.h:10961:16 */
pub type PTOKEN_SID_INFORMATION = *mut ::winnt::TOKEN_SID_INFORMATION; /* winnt.h:10963:27, winnt.h:10963:27, winnt.h:10963:27 */
#[repr(C)] pub struct CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE { Version: ::basetsd::DWORD64, Name: ::winnt::PWSTR } /* winnt.h:10994:16, winnt.h:10994:16, winnt.h:10994:16 */
pub type PCLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE = *mut ::winnt::CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE; /* winnt.h:10997:41, winnt.h:10997:41, winnt.h:10997:41 */
#[repr(C)] pub struct CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE { pValue: ::winnt::PVOID, ValueLength: ::minwindef::DWORD } /* winnt.h:11006:16, winnt.h:11006:16, winnt.h:11006:16 */
pub type PCLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE = *mut ::winnt::CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE; /* winnt.h:11010:6, winnt.h:11010:6, winnt.h:11010:6 */
#[repr(C)] pub struct CLAIM_SECURITY_ATTRIBUTE_V1 { Name: ::winnt::PWSTR, ValueType: ::minwindef::WORD, Reserved: ::minwindef::WORD, Flags: ::minwindef::DWORD, ValueCount: ::minwindef::DWORD, Values: ::winnt::CLAIM_SECURITY_ATTRIBUTE_V1_Child_5 } /* winnt.h:11078:16, winnt.h:11078:16, winnt.h:11078:16 */
#[repr(C)] pub /*union*/ struct CLAIM_SECURITY_ATTRIBUTE_V1_Child_5; /* STUB! */ /* winnt.h:11116:5, winnt.h:11116:5, winnt.h:11116:5 */
pub type PCLAIM_SECURITY_ATTRIBUTE_V1 = *mut ::winnt::CLAIM_SECURITY_ATTRIBUTE_V1; /* winnt.h:11123:33, winnt.h:11123:33, winnt.h:11123:33 */
#[repr(C)] pub struct CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 { Name: ::minwindef::DWORD, ValueType: ::minwindef::WORD, Reserved: ::minwindef::WORD, Flags: ::minwindef::DWORD, ValueCount: ::minwindef::DWORD, Values: ::winnt::CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_Child_5 } /* winnt.h:11130:16, winnt.h:11130:16, winnt.h:11130:16 */
#[repr(C)] pub /*union*/ struct CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_Child_5; /* STUB! */ /* winnt.h:11168:5, winnt.h:11168:5, winnt.h:11168:5 */
pub type PCLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 = *mut ::winnt::CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1; /* winnt.h:11175:42, winnt.h:11175:42, winnt.h:11175:42 */
#[repr(C)] pub struct CLAIM_SECURITY_ATTRIBUTES_INFORMATION { Version: ::minwindef::WORD, Reserved: ::minwindef::WORD, AttributeCount: ::minwindef::DWORD, Attribute: ::winnt::CLAIM_SECURITY_ATTRIBUTES_INFORMATION_Child_3 } /* winnt.h:11197:16, winnt.h:11197:16, winnt.h:11197:16 */
#[repr(C)] pub /*union*/ struct CLAIM_SECURITY_ATTRIBUTES_INFORMATION_Child_3; /* STUB! */ /* winnt.h:11212:5, winnt.h:11212:5, winnt.h:11212:5 */
pub type PCLAIM_SECURITY_ATTRIBUTES_INFORMATION = *mut ::winnt::CLAIM_SECURITY_ATTRIBUTES_INFORMATION; /* winnt.h:11215:43, winnt.h:11215:43, winnt.h:11215:43 */
pub type SECURITY_CONTEXT_TRACKING_MODE = ::winnt::BOOLEAN; /* winnt.h:11224:17, winnt.h:11224:17, winnt.h:11224:17 */
pub type PSECURITY_CONTEXT_TRACKING_MODE = *mut ::libc::c_uchar; /* winnt.h:11225:23, winnt.h:11225:23, winnt.h:11225:23 */
#[repr(C)] pub struct SECURITY_QUALITY_OF_SERVICE { Length: ::minwindef::DWORD, ImpersonationLevel: ::winnt::SECURITY_IMPERSONATION_LEVEL, ContextTrackingMode: ::winnt::SECURITY_CONTEXT_TRACKING_MODE, EffectiveOnly: ::winnt::BOOLEAN } /* winnt.h:11233:16, winnt.h:11233:16, winnt.h:11233:16 */
pub type PSECURITY_QUALITY_OF_SERVICE = *mut ::winnt::SECURITY_QUALITY_OF_SERVICE; /* winnt.h:11238:38, winnt.h:11238:38, winnt.h:11238:38 */
#[repr(C)] pub struct SE_IMPERSONATION_STATE { Token: ::winnt::PACCESS_TOKEN, CopyOnOpen: ::winnt::BOOLEAN, EffectiveOnly: ::winnt::BOOLEAN, Level: ::winnt::SECURITY_IMPERSONATION_LEVEL } /* winnt.h:11245:16, winnt.h:11245:16, winnt.h:11245:16 */
pub type PSE_IMPERSONATION_STATE = *mut ::winnt::SE_IMPERSONATION_STATE; /* winnt.h:11250:28, winnt.h:11250:28, winnt.h:11250:28 */
pub type SECURITY_INFORMATION = ::minwindef::DWORD; /* winnt.h:11257:15, winnt.h:11257:15, winnt.h:11257:15 */
pub type PSECURITY_INFORMATION = *mut ::libc::c_ulong; /* winnt.h:11257:38, winnt.h:11257:38, winnt.h:11257:38 */
#[repr(C)] pub enum SE_LEARNING_MODE_DATA_TYPE {SeLearningModeInvalidType = 0, SeLearningModeSettings = 1, SeLearningModeMax = 2} pub use self::SE_LEARNING_MODE_DATA_TYPE::{SeLearningModeInvalidType, SeLearningModeSettings, SeLearningModeMax}; /* winnt.h:11279:14, winnt.h:11279:14, winnt.h:11279:14 */
#[repr(C)] pub struct SECURITY_CAPABILITIES { AppContainerSid: ::winnt::PSID, Capabilities: ::winnt::PSID_AND_ATTRIBUTES, CapabilityCount: ::minwindef::DWORD, Reserved: ::minwindef::DWORD } /* winnt.h:11287:16, winnt.h:11287:16, winnt.h:11287:16 */
pub type PSECURITY_CAPABILITIES = *mut ::winnt::SECURITY_CAPABILITIES; /* winnt.h:11292:27, winnt.h:11292:27, winnt.h:11292:27 */
pub type LPSECURITY_CAPABILITIES = *mut ::winnt::SECURITY_CAPABILITIES; /* winnt.h:11292:52, winnt.h:11292:52, winnt.h:11292:52 */
#[repr(C)] pub struct JOB_SET_ARRAY { JobHandle: ::winnt::HANDLE, MemberLevel: ::minwindef::DWORD, Flags: ::minwindef::DWORD } /* winnt.h:11356:16, winnt.h:11356:16, winnt.h:11356:16 */
pub type PJOB_SET_ARRAY = *mut ::winnt::JOB_SET_ARRAY; /* winnt.h:11360:19, winnt.h:11360:19, winnt.h:11360:19 */
#[repr(C)] pub struct EXCEPTION_REGISTRATION_RECORD { Next: *mut ::winnt::EXCEPTION_REGISTRATION_RECORD, Handler: ::winnt::PEXCEPTION_ROUTINE } /* winnt.h:11365:16, winnt.h:11365:16, winnt.h:11365:16 */
pub type PEXCEPTION_REGISTRATION_RECORD = *mut ::winnt::EXCEPTION_REGISTRATION_RECORD; /* winnt.h:11370:40, winnt.h:11370:40, winnt.h:11370:40 */
#[repr(C)] pub struct NT_TIB { ExceptionList: *mut ::winnt::EXCEPTION_REGISTRATION_RECORD, StackBase: ::winnt::PVOID, StackLimit: ::winnt::PVOID, SubSystemTib: ::winnt::PVOID, FiberData: ::winnt::PVOID, ArbitraryUserPointer: ::winnt::PVOID, Self_: *mut ::winnt::NT_TIB } /* winnt.h:11372:16, winnt.h:11372:16, winnt.h:11372:16 */
pub type PNT_TIB = *mut ::winnt::NT_TIB; /* winnt.h:11388:17, winnt.h:11388:17, winnt.h:11388:17 */
#[repr(C)] pub struct NT_TIB32 { ExceptionList: ::minwindef::DWORD, StackBase: ::minwindef::DWORD, StackLimit: ::minwindef::DWORD, SubSystemTib: ::minwindef::DWORD, FiberData: ::minwindef::DWORD, ArbitraryUserPointer: ::minwindef::DWORD, Self_: ::minwindef::DWORD } /* winnt.h:11393:16, winnt.h:11393:16, winnt.h:11393:16 */
pub type PNT_TIB32 = *mut ::winnt::NT_TIB32; /* winnt.h:11410:14, winnt.h:11410:14, winnt.h:11410:14 */
#[repr(C)] pub struct NT_TIB64 { ExceptionList: ::basetsd::DWORD64, StackBase: ::basetsd::DWORD64, StackLimit: ::basetsd::DWORD64, SubSystemTib: ::basetsd::DWORD64, FiberData: ::basetsd::DWORD64, ArbitraryUserPointer: ::basetsd::DWORD64, Self_: ::basetsd::DWORD64 } /* winnt.h:11412:16, winnt.h:11412:16, winnt.h:11412:16 */
pub type PNT_TIB64 = *mut ::winnt::NT_TIB64; /* winnt.h:11430:14, winnt.h:11430:14, winnt.h:11430:14 */
#[repr(C)] pub struct UMS_CREATE_THREAD_ATTRIBUTES { UmsVersion: ::minwindef::DWORD, UmsContext: ::winnt::PVOID, UmsCompletionList: ::winnt::PVOID } /* winnt.h:11439:16, winnt.h:11439:16, winnt.h:11439:16 */
pub type PUMS_CREATE_THREAD_ATTRIBUTES = *mut ::winnt::UMS_CREATE_THREAD_ATTRIBUTES; /* winnt.h:11443:34, winnt.h:11443:34, winnt.h:11443:34 */
#[repr(C)] pub struct QUOTA_LIMITS { PagedPoolLimit: ::basetsd::SIZE_T, NonPagedPoolLimit: ::basetsd::SIZE_T, MinimumWorkingSetSize: ::basetsd::SIZE_T, MaximumWorkingSetSize: ::basetsd::SIZE_T, PagefileLimit: ::basetsd::SIZE_T, TimeLimit: ::winnt::LARGE_INTEGER } /* winnt.h:11445:16, winnt.h:11445:16, winnt.h:11445:16 */
pub type PQUOTA_LIMITS = *mut ::winnt::QUOTA_LIMITS; /* winnt.h:11452:18, winnt.h:11452:18, winnt.h:11452:18 */
#[repr(C)] pub /*union*/ struct RATE_QUOTA_LIMIT; /* STUB! */ /* winnt.h:11460:15, winnt.h:11460:15, winnt.h:11460:15 */
pub type PRATE_QUOTA_LIMIT = *mut ::winnt::RATE_QUOTA_LIMIT; /* winnt.h:11466:22, winnt.h:11466:22, winnt.h:11466:22 */
#[repr(C)] pub struct QUOTA_LIMITS_EX { PagedPoolLimit: ::basetsd::SIZE_T, NonPagedPoolLimit: ::basetsd::SIZE_T, MinimumWorkingSetSize: ::basetsd::SIZE_T, MaximumWorkingSetSize: ::basetsd::SIZE_T, PagefileLimit: ::basetsd::SIZE_T, TimeLimit: ::winnt::LARGE_INTEGER, WorkingSetLimit: ::basetsd::SIZE_T, Reserved2: ::basetsd::SIZE_T, Reserved3: ::basetsd::SIZE_T, Reserved4: ::basetsd::SIZE_T, Flags: ::minwindef::DWORD, CpuRateLimit: ::winnt::RATE_QUOTA_LIMIT } /* winnt.h:11468:16, winnt.h:11468:16, winnt.h:11468:16 */
pub type PQUOTA_LIMITS_EX = *mut ::winnt::QUOTA_LIMITS_EX; /* winnt.h:11481:21, winnt.h:11481:21, winnt.h:11481:21 */
#[repr(C)] pub struct IO_COUNTERS { ReadOperationCount: ::winnt::ULONGLONG, WriteOperationCount: ::winnt::ULONGLONG, OtherOperationCount: ::winnt::ULONGLONG, ReadTransferCount: ::winnt::ULONGLONG, WriteTransferCount: ::winnt::ULONGLONG, OtherTransferCount: ::winnt::ULONGLONG } /* winnt.h:11483:16, winnt.h:11483:16, winnt.h:11483:16 */
pub type PIO_COUNTERS = *mut ::winnt::IO_COUNTERS; /* winnt.h:11491:22, winnt.h:11491:22, winnt.h:11491:22 */
#[repr(C)] pub enum HARDWARE_COUNTER_TYPE {PMCCounter = 0, MaxHardwareCounterType = 1} pub use self::HARDWARE_COUNTER_TYPE::{PMCCounter, MaxHardwareCounterType}; /* winnt.h:11496:14, winnt.h:11496:14, winnt.h:11496:14 */
pub type PHARDWARE_COUNTER_TYPE = *mut ::winnt::HARDWARE_COUNTER_TYPE; /* winnt.h:11499:27, winnt.h:11499:27, winnt.h:11499:27 */
#[repr(C)] pub enum PROCESS_MITIGATION_POLICY {ProcessDEPPolicy = 0, ProcessASLRPolicy = 1, ProcessDynamicCodePolicy = 2, ProcessStrictHandleCheckPolicy = 3, ProcessSystemCallDisablePolicy = 4, ProcessMitigationOptionsMask = 5, ProcessExtensionPointDisablePolicy = 6, ProcessReserved1Policy = 7, ProcessSignaturePolicy = 8, MaxProcessMitigationPolicy = 9} pub use self::PROCESS_MITIGATION_POLICY::{ProcessDEPPolicy, ProcessASLRPolicy, ProcessDynamicCodePolicy, ProcessStrictHandleCheckPolicy, ProcessSystemCallDisablePolicy, ProcessMitigationOptionsMask, ProcessExtensionPointDisablePolicy, ProcessReserved1Policy, ProcessSignaturePolicy, MaxProcessMitigationPolicy}; /* winnt.h:11500:14, winnt.h:11500:14, winnt.h:11500:14 */
pub type PPROCESS_MITIGATION_POLICY = *mut ::winnt::PROCESS_MITIGATION_POLICY; /* winnt.h:11511:31, winnt.h:11511:31, winnt.h:11511:31 */
#[repr(C)] pub struct PROCESS_MITIGATION_ASLR_POLICY; /* winnt.h:11518:16, winnt.h:11518:16, winnt.h:11518:16 */
#[repr(C)] pub /*union*/ struct PROCESS_MITIGATION_ASLR_POLICY_Child_0; /* STUB! */ /* winnt.h:11519:5, winnt.h:11519:5, winnt.h:11519:5 */
pub type PPROCESS_MITIGATION_ASLR_POLICY = *mut ::winnt::PROCESS_MITIGATION_ASLR_POLICY; /* winnt.h:11529:36, winnt.h:11529:36, winnt.h:11529:36 */
#[repr(C)] pub struct PROCESS_MITIGATION_DEP_POLICY { Permanent: ::winnt::BOOLEAN } /* winnt.h:11531:16, winnt.h:11531:16, winnt.h:11531:16 */
#[repr(C)] pub /*union*/ struct PROCESS_MITIGATION_DEP_POLICY_Child_0; /* STUB! */ /* winnt.h:11532:5, winnt.h:11532:5, winnt.h:11532:5 */
pub type PPROCESS_MITIGATION_DEP_POLICY = *mut ::winnt::PROCESS_MITIGATION_DEP_POLICY; /* winnt.h:11541:35, winnt.h:11541:35, winnt.h:11541:35 */
#[repr(C)] pub struct PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY; /* winnt.h:11543:16, winnt.h:11543:16, winnt.h:11543:16 */
#[repr(C)] pub /*union*/ struct PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_Child_0; /* STUB! */ /* winnt.h:11544:5, winnt.h:11544:5, winnt.h:11544:5 */
pub type PPROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY = *mut ::winnt::PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY; /* winnt.h:11552:51, winnt.h:11552:51, winnt.h:11552:51 */
#[repr(C)] pub struct PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY; /* winnt.h:11554:16, winnt.h:11554:16, winnt.h:11554:16 */
#[repr(C)] pub /*union*/ struct PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_Child_0; /* STUB! */ /* winnt.h:11555:5, winnt.h:11555:5, winnt.h:11555:5 */
pub type PPROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY = *mut ::winnt::PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY; /* winnt.h:11562:51, winnt.h:11562:51, winnt.h:11562:51 */
#[repr(C)] pub struct PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY; /* winnt.h:11564:16, winnt.h:11564:16, winnt.h:11564:16 */
#[repr(C)] pub /*union*/ struct PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_Child_0; /* STUB! */ /* winnt.h:11565:5, winnt.h:11565:5, winnt.h:11565:5 */
pub type PPROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY = *mut ::winnt::PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY; /* winnt.h:11572:55, winnt.h:11572:55, winnt.h:11572:55 */
#[repr(C)] pub struct PROCESS_MITIGATION_DYNAMIC_CODE_POLICY; /* winnt.h:11574:16, winnt.h:11574:16, winnt.h:11574:16 */
#[repr(C)] pub /*union*/ struct PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_Child_0; /* STUB! */ /* winnt.h:11575:5, winnt.h:11575:5, winnt.h:11575:5 */
pub type PPROCESS_MITIGATION_DYNAMIC_CODE_POLICY = *mut ::winnt::PROCESS_MITIGATION_DYNAMIC_CODE_POLICY; /* winnt.h:11582:44, winnt.h:11582:44, winnt.h:11582:44 */
#[repr(C)] pub struct PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY; /* winnt.h:11585:16, winnt.h:11585:16, winnt.h:11585:16 */
#[repr(C)] pub /*union*/ struct PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_Child_0; /* STUB! */ /* winnt.h:11586:5, winnt.h:11586:5, winnt.h:11586:5 */
pub type PPROCESS_MITIGATION_BINARY_SIGNATURE_POLICY = *mut ::winnt::PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY; /* winnt.h:11593:48, winnt.h:11593:48, winnt.h:11593:48 */
#[repr(C)] pub struct JOBOBJECT_BASIC_ACCOUNTING_INFORMATION { TotalUserTime: ::winnt::LARGE_INTEGER, TotalKernelTime: ::winnt::LARGE_INTEGER, ThisPeriodTotalUserTime: ::winnt::LARGE_INTEGER, ThisPeriodTotalKernelTime: ::winnt::LARGE_INTEGER, TotalPageFaultCount: ::minwindef::DWORD, TotalProcesses: ::minwindef::DWORD, ActiveProcesses: ::minwindef::DWORD, TotalTerminatedProcesses: ::minwindef::DWORD } /* winnt.h:11596:16, winnt.h:11596:16, winnt.h:11596:16 */
pub type PJOBOBJECT_BASIC_ACCOUNTING_INFORMATION = *mut ::winnt::JOBOBJECT_BASIC_ACCOUNTING_INFORMATION; /* winnt.h:11605:44, winnt.h:11605:44, winnt.h:11605:44 */
#[repr(C)] pub struct JOBOBJECT_BASIC_LIMIT_INFORMATION { PerProcessUserTimeLimit: ::winnt::LARGE_INTEGER, PerJobUserTimeLimit: ::winnt::LARGE_INTEGER, LimitFlags: ::minwindef::DWORD, MinimumWorkingSetSize: ::basetsd::SIZE_T, MaximumWorkingSetSize: ::basetsd::SIZE_T, ActiveProcessLimit: ::minwindef::DWORD, Affinity: ::basetsd::ULONG_PTR, PriorityClass: ::minwindef::DWORD, SchedulingClass: ::minwindef::DWORD } /* winnt.h:11607:16, winnt.h:11607:16, winnt.h:11607:16 */
pub type PJOBOBJECT_BASIC_LIMIT_INFORMATION = *mut ::winnt::JOBOBJECT_BASIC_LIMIT_INFORMATION; /* winnt.h:11617:39, winnt.h:11617:39, winnt.h:11617:39 */
#[repr(C)] pub struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION { BasicLimitInformation: ::winnt::JOBOBJECT_BASIC_LIMIT_INFORMATION, IoInfo: ::winnt::IO_COUNTERS, ProcessMemoryLimit: ::basetsd::SIZE_T, JobMemoryLimit: ::basetsd::SIZE_T, PeakProcessMemoryUsed: ::basetsd::SIZE_T, PeakJobMemoryUsed: ::basetsd::SIZE_T } /* winnt.h:11619:16, winnt.h:11619:16, winnt.h:11619:16 */
pub type PJOBOBJECT_EXTENDED_LIMIT_INFORMATION = *mut ::winnt::JOBOBJECT_EXTENDED_LIMIT_INFORMATION; /* winnt.h:11626:42, winnt.h:11626:42, winnt.h:11626:42 */
#[repr(C)] pub struct JOBOBJECT_BASIC_PROCESS_ID_LIST { NumberOfAssignedProcesses: ::minwindef::DWORD, NumberOfProcessIdsInList: ::minwindef::DWORD, ProcessIdList: *mut [::basetsd::ULONG_PTR; 1] } /* winnt.h:11628:16, winnt.h:11628:16, winnt.h:11628:16 */
pub type PJOBOBJECT_BASIC_PROCESS_ID_LIST = *mut ::winnt::JOBOBJECT_BASIC_PROCESS_ID_LIST; /* winnt.h:11632:37, winnt.h:11632:37, winnt.h:11632:37 */
#[repr(C)] pub struct JOBOBJECT_BASIC_UI_RESTRICTIONS { UIRestrictionsClass: ::minwindef::DWORD } /* winnt.h:11634:16, winnt.h:11634:16, winnt.h:11634:16 */
pub type PJOBOBJECT_BASIC_UI_RESTRICTIONS = *mut ::winnt::JOBOBJECT_BASIC_UI_RESTRICTIONS; /* winnt.h:11636:37, winnt.h:11636:37, winnt.h:11636:37 */
#[repr(C)] pub struct JOBOBJECT_SECURITY_LIMIT_INFORMATION { SecurityLimitFlags: ::minwindef::DWORD, JobToken: ::winnt::HANDLE, SidsToDisable: ::winnt::PTOKEN_GROUPS, PrivilegesToDelete: ::winnt::PTOKEN_PRIVILEGES, RestrictedSids: ::winnt::PTOKEN_GROUPS } /* winnt.h:11642:16, winnt.h:11642:16, winnt.h:11642:16 */
pub type PJOBOBJECT_SECURITY_LIMIT_INFORMATION = *mut ::winnt::JOBOBJECT_SECURITY_LIMIT_INFORMATION; /* winnt.h:11648:42, winnt.h:11648:42, winnt.h:11648:42 */
#[repr(C)] pub struct JOBOBJECT_END_OF_JOB_TIME_INFORMATION { EndOfJobTimeAction: ::minwindef::DWORD } /* winnt.h:11650:16, winnt.h:11650:16, winnt.h:11650:16 */
pub type PJOBOBJECT_END_OF_JOB_TIME_INFORMATION = *mut ::winnt::JOBOBJECT_END_OF_JOB_TIME_INFORMATION; /* winnt.h:11652:43, winnt.h:11652:43, winnt.h:11652:43 */
#[repr(C)] pub struct JOBOBJECT_ASSOCIATE_COMPLETION_PORT { CompletionKey: ::winnt::PVOID, CompletionPort: ::winnt::HANDLE } /* winnt.h:11654:16, winnt.h:11654:16, winnt.h:11654:16 */
pub type PJOBOBJECT_ASSOCIATE_COMPLETION_PORT = *mut ::winnt::JOBOBJECT_ASSOCIATE_COMPLETION_PORT; /* winnt.h:11657:41, winnt.h:11657:41, winnt.h:11657:41 */
#[repr(C)] pub struct JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION { BasicInfo: ::winnt::JOBOBJECT_BASIC_ACCOUNTING_INFORMATION, IoInfo: ::winnt::IO_COUNTERS } /* winnt.h:11659:16, winnt.h:11659:16, winnt.h:11659:16 */
pub type PJOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION = *mut ::winnt::JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION; /* winnt.h:11662:51, winnt.h:11662:51, winnt.h:11662:51 */
#[repr(C)] pub struct JOBOBJECT_JOBSET_INFORMATION { MemberLevel: ::minwindef::DWORD } /* winnt.h:11664:16, winnt.h:11664:16, winnt.h:11664:16 */
pub type PJOBOBJECT_JOBSET_INFORMATION = *mut ::winnt::JOBOBJECT_JOBSET_INFORMATION; /* winnt.h:11666:34, winnt.h:11666:34, winnt.h:11666:34 */
#[repr(C)] pub enum JOBOBJECT_RATE_CONTROL_TOLERANCE {ToleranceLow = 1, ToleranceMedium = 2, ToleranceHigh = 3} pub use self::JOBOBJECT_RATE_CONTROL_TOLERANCE::{ToleranceLow, ToleranceMedium, ToleranceHigh}; /* winnt.h:11668:14, winnt.h:11668:14, winnt.h:11668:14 */
#[repr(C)] pub enum JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL {ToleranceIntervalShort = 1, ToleranceIntervalMedium = 2, ToleranceIntervalLong = 3} pub use self::JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL::{ToleranceIntervalShort, ToleranceIntervalMedium, ToleranceIntervalLong}; /* winnt.h:11674:14, winnt.h:11674:14, winnt.h:11674:14 */
#[repr(C)] pub struct JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION { IoReadBytesLimit: ::basetsd::DWORD64, IoWriteBytesLimit: ::basetsd::DWORD64, PerJobUserTimeLimit: ::winnt::LARGE_INTEGER, JobMemoryLimit: ::basetsd::DWORD64, RateControlTolerance: ::winnt::JOBOBJECT_RATE_CONTROL_TOLERANCE, RateControlToleranceInterval: ::winnt::JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL, LimitFlags: ::minwindef::DWORD } /* winnt.h:11680:16, winnt.h:11680:16, winnt.h:11680:16 */
pub type PJOBOBJECT_NOTIFICATION_LIMIT_INFORMATION = *mut ::winnt::JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION; /* winnt.h:11688:46, winnt.h:11688:46, winnt.h:11688:46 */
#[repr(C)] pub struct JOBOBJECT_LIMIT_VIOLATION_INFORMATION { LimitFlags: ::minwindef::DWORD, ViolationLimitFlags: ::minwindef::DWORD, IoReadBytes: ::basetsd::DWORD64, IoReadBytesLimit: ::basetsd::DWORD64, IoWriteBytes: ::basetsd::DWORD64, IoWriteBytesLimit: ::basetsd::DWORD64, PerJobUserTime: ::winnt::LARGE_INTEGER, PerJobUserTimeLimit: ::winnt::LARGE_INTEGER, JobMemory: ::basetsd::DWORD64, JobMemoryLimit: ::basetsd::DWORD64, RateControlTolerance: ::winnt::JOBOBJECT_RATE_CONTROL_TOLERANCE, RateControlToleranceLimit: ::winnt::JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL } /* winnt.h:11690:16, winnt.h:11690:16, winnt.h:11690:16 */
pub type PJOBOBJECT_LIMIT_VIOLATION_INFORMATION = *mut ::winnt::JOBOBJECT_LIMIT_VIOLATION_INFORMATION; /* winnt.h:11703:43, winnt.h:11703:43, winnt.h:11703:43 */
#[repr(C)] pub struct JOBOBJECT_CPU_RATE_CONTROL_INFORMATION { ControlFlags: ::minwindef::DWORD } /* winnt.h:11705:16, winnt.h:11705:16, winnt.h:11705:16 */
#[repr(C)] pub /*union*/ struct JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_Child_1; /* STUB! */ /* winnt.h:11707:5, winnt.h:11707:5, winnt.h:11707:5 */
pub type PJOBOBJECT_CPU_RATE_CONTROL_INFORMATION = *mut ::winnt::JOBOBJECT_CPU_RATE_CONTROL_INFORMATION; /* winnt.h:11711:44, winnt.h:11711:44, winnt.h:11711:44 */
#[repr(C)] pub enum JOBOBJECTINFOCLASS {JobObjectBasicAccountingInformation = 1, JobObjectBasicLimitInformation = 2, JobObjectBasicProcessIdList = 3, JobObjectBasicUIRestrictions = 4, JobObjectSecurityLimitInformation = 5, JobObjectEndOfJobTimeInformation = 6, JobObjectAssociateCompletionPortInformation = 7, JobObjectBasicAndIoAccountingInformation = 8, JobObjectExtendedLimitInformation = 9, JobObjectJobSetInformation = 10, JobObjectGroupInformation = 11, JobObjectNotificationLimitInformation = 12, JobObjectLimitViolationInformation = 13, JobObjectGroupInformationEx = 14, JobObjectCpuRateControlInformation = 15, JobObjectCompletionFilter = 16, JobObjectCompletionCounter = 17, JobObjectReserved1Information = 18, JobObjectReserved2Information = 19, JobObjectReserved3Information = 20, JobObjectReserved4Information = 21, JobObjectReserved5Information = 22, JobObjectReserved6Information = 23, JobObjectReserved7Information = 24, JobObjectReserved8Information = 25, JobObjectReserved9Information = 26, MaxJobObjectInfoClass = 27} pub use self::JOBOBJECTINFOCLASS::{JobObjectBasicAccountingInformation, JobObjectBasicLimitInformation, JobObjectBasicProcessIdList, JobObjectBasicUIRestrictions, JobObjectSecurityLimitInformation, JobObjectEndOfJobTimeInformation, JobObjectAssociateCompletionPortInformation, JobObjectBasicAndIoAccountingInformation, JobObjectExtendedLimitInformation, JobObjectJobSetInformation, JobObjectGroupInformation, JobObjectNotificationLimitInformation, JobObjectLimitViolationInformation, JobObjectGroupInformationEx, JobObjectCpuRateControlInformation, JobObjectCompletionFilter, JobObjectCompletionCounter, JobObjectReserved1Information, JobObjectReserved2Information, JobObjectReserved3Information, JobObjectReserved4Information, JobObjectReserved5Information, JobObjectReserved6Information, JobObjectReserved7Information, JobObjectReserved8Information, JobObjectReserved9Information, MaxJobObjectInfoClass}; /* winnt.h:11828:14, winnt.h:11828:14, winnt.h:11828:14 */
#[repr(C)] pub enum FIRMWARE_TYPE {FirmwareTypeUnknown = 0, FirmwareTypeBios = 1, FirmwareTypeUefi = 2, FirmwareTypeMax = 3} pub use self::FIRMWARE_TYPE::{FirmwareTypeUnknown, FirmwareTypeBios, FirmwareTypeUefi, FirmwareTypeMax}; /* winnt.h:11859:14, winnt.h:11859:14, winnt.h:11859:14 */
pub type PFIRMWARE_TYPE = *mut ::winnt::FIRMWARE_TYPE; /* winnt.h:11864:19, winnt.h:11864:19, winnt.h:11864:19 */
#[repr(C)] pub enum LOGICAL_PROCESSOR_RELATIONSHIP {RelationProcessorCore = 0, RelationNumaNode = 1, RelationCache = 2, RelationProcessorPackage = 3, RelationGroup = 4, RelationAll = 65535} pub use self::LOGICAL_PROCESSOR_RELATIONSHIP::{RelationProcessorCore, RelationNumaNode, RelationCache, RelationProcessorPackage, RelationGroup, RelationAll}; /* winnt.h:11895:14, winnt.h:11895:14, winnt.h:11895:14 */
#[repr(C)] pub enum PROCESSOR_CACHE_TYPE {CacheUnified = 0, CacheInstruction = 1, CacheData = 2, CacheTrace = 3} pub use self::PROCESSOR_CACHE_TYPE::{CacheUnified, CacheInstruction, CacheData, CacheTrace}; /* winnt.h:11906:14, winnt.h:11906:14, winnt.h:11906:14 */
#[repr(C)] pub struct CACHE_DESCRIPTOR { Level: ::minwindef::BYTE, Associativity: ::minwindef::BYTE, LineSize: ::minwindef::WORD, Size: ::minwindef::DWORD, Type: ::winnt::PROCESSOR_CACHE_TYPE } /* winnt.h:11915:16, winnt.h:11915:16, winnt.h:11915:16 */
pub type PCACHE_DESCRIPTOR = *mut ::winnt::CACHE_DESCRIPTOR; /* winnt.h:11921:22, winnt.h:11921:22, winnt.h:11921:22 */
#[repr(C)] pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION { ProcessorMask: ::basetsd::ULONG_PTR, Relationship: ::winnt::LOGICAL_PROCESSOR_RELATIONSHIP, u: ::winnt::SYSTEM_LOGICAL_PROCESSOR_INFORMATION_Child_2 } /* winnt.h:11923:16, winnt.h:11923:16, winnt.h:11923:16 */
#[repr(C)] pub /*union*/ struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_Child_2; /* STUB! */ /* winnt.h:11926:5, winnt.h:11926:5, winnt.h:11926:5 */
pub type PSYSTEM_LOGICAL_PROCESSOR_INFORMATION = *mut ::winnt::SYSTEM_LOGICAL_PROCESSOR_INFORMATION; /* winnt.h:11936:42, winnt.h:11936:42, winnt.h:11936:42 */
#[repr(C)] pub struct PROCESSOR_RELATIONSHIP { Flags: ::minwindef::BYTE, Reserved: *mut [::minwindef::BYTE; 21], GroupCount: ::minwindef::WORD, GroupMask: *mut [::winnt::GROUP_AFFINITY; 1] } /* winnt.h:11938:16, winnt.h:11938:16, winnt.h:11938:16 */
pub type PPROCESSOR_RELATIONSHIP = *mut ::winnt::PROCESSOR_RELATIONSHIP; /* winnt.h:11943:28, winnt.h:11943:28, winnt.h:11943:28 */
#[repr(C)] pub struct NUMA_NODE_RELATIONSHIP { NodeNumber: ::minwindef::DWORD, Reserved: *mut [::minwindef::BYTE; 20], GroupMask: ::winnt::GROUP_AFFINITY } /* winnt.h:11945:16, winnt.h:11945:16, winnt.h:11945:16 */
pub type PNUMA_NODE_RELATIONSHIP = *mut ::winnt::NUMA_NODE_RELATIONSHIP; /* winnt.h:11949:28, winnt.h:11949:28, winnt.h:11949:28 */
#[repr(C)] pub struct CACHE_RELATIONSHIP { Level: ::minwindef::BYTE, Associativity: ::minwindef::BYTE, LineSize: ::minwindef::WORD, CacheSize: ::minwindef::DWORD, Type: ::winnt::PROCESSOR_CACHE_TYPE, Reserved: *mut [::minwindef::BYTE; 20], GroupMask: ::winnt::GROUP_AFFINITY } /* winnt.h:11951:16, winnt.h:11951:16, winnt.h:11951:16 */
pub type PCACHE_RELATIONSHIP = *mut ::winnt::CACHE_RELATIONSHIP; /* winnt.h:11959:24, winnt.h:11959:24, winnt.h:11959:24 */
#[repr(C)] pub struct PROCESSOR_GROUP_INFO { MaximumProcessorCount: ::minwindef::BYTE, ActiveProcessorCount: ::minwindef::BYTE, Reserved: *mut [::minwindef::BYTE; 38], ActiveProcessorMask: ::basetsd::KAFFINITY } /* winnt.h:11961:16, winnt.h:11961:16, winnt.h:11961:16 */
pub type PPROCESSOR_GROUP_INFO = *mut ::winnt::PROCESSOR_GROUP_INFO; /* winnt.h:11966:26, winnt.h:11966:26, winnt.h:11966:26 */
#[repr(C)] pub struct GROUP_RELATIONSHIP { MaximumGroupCount: ::minwindef::WORD, ActiveGroupCount: ::minwindef::WORD, Reserved: *mut [::minwindef::BYTE; 20], GroupInfo: *mut [::winnt::PROCESSOR_GROUP_INFO; 1] } /* winnt.h:11968:16, winnt.h:11968:16, winnt.h:11968:16 */
pub type PGROUP_RELATIONSHIP = *mut ::winnt::GROUP_RELATIONSHIP; /* winnt.h:11973:24, winnt.h:11973:24, winnt.h:11973:24 */
#[repr(C)] pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX { Relationship: ::winnt::LOGICAL_PROCESSOR_RELATIONSHIP, Size: ::minwindef::DWORD, u: ::winnt::SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_Child_2 } /* winnt.h:11975:34, winnt.h:11975:34, winnt.h:11975:34 */
#[repr(C)] pub /*union*/ struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_Child_2; /* STUB! */ /* winnt.h:11978:5, winnt.h:11978:5, winnt.h:11978:5 */
pub type PSYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX = *mut ::winnt::SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX; /* winnt.h:11986:99, winnt.h:11986:99, winnt.h:11986:99 */
#[repr(C)] pub struct SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION { CycleTime: ::basetsd::DWORD64 } /* winnt.h:11990:16, winnt.h:11990:16, winnt.h:11990:16 */
pub type PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION = *mut ::winnt::SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION; /* winnt.h:11992:45, winnt.h:11992:45, winnt.h:11992:45 */
#[repr(C)] pub struct XSTATE_FEATURE { Offset: ::minwindef::DWORD, Size: ::minwindef::DWORD } /* winnt.h:12085:16, winnt.h:12085:16, winnt.h:12085:16 */
pub type PXSTATE_FEATURE = *mut ::winnt::XSTATE_FEATURE; /* winnt.h:12088:20, winnt.h:12088:20, winnt.h:12088:20 */
#[repr(C)] pub struct XSTATE_CONFIGURATION { EnabledFeatures: ::basetsd::DWORD64, EnabledVolatileFeatures: ::basetsd::DWORD64, Size: ::minwindef::DWORD, OptimizedSave: ::minwindef::DWORD, Features: *mut [::winnt::XSTATE_FEATURE; 64] } /* winnt.h:12090:16, winnt.h:12090:16, winnt.h:12090:16 */
pub type PXSTATE_CONFIGURATION = *mut ::winnt::XSTATE_CONFIGURATION; /* winnt.h:12105:26, winnt.h:12105:26, winnt.h:12105:26 */
#[repr(C)] pub struct MEMORY_BASIC_INFORMATION { BaseAddress: ::winnt::PVOID, AllocationBase: ::winnt::PVOID, AllocationProtect: ::minwindef::DWORD, RegionSize: ::basetsd::SIZE_T, State: ::minwindef::DWORD, Protect: ::minwindef::DWORD, Type: ::minwindef::DWORD } /* winnt.h:12108:16, winnt.h:12108:16, winnt.h:12108:16 */
pub type PMEMORY_BASIC_INFORMATION = *mut ::winnt::MEMORY_BASIC_INFORMATION; /* winnt.h:12116:30, winnt.h:12116:30, winnt.h:12116:30 */
#[repr(C)] pub struct MEMORY_BASIC_INFORMATION32 { BaseAddress: ::minwindef::DWORD, AllocationBase: ::minwindef::DWORD, AllocationProtect: ::minwindef::DWORD, RegionSize: ::minwindef::DWORD, State: ::minwindef::DWORD, Protect: ::minwindef::DWORD, Type: ::minwindef::DWORD } /* winnt.h:12118:16, winnt.h:12118:16, winnt.h:12118:16 */
pub type PMEMORY_BASIC_INFORMATION32 = *mut ::winnt::MEMORY_BASIC_INFORMATION32; /* winnt.h:12126:32, winnt.h:12126:32, winnt.h:12126:32 */
#[repr(C)] pub struct MEMORY_BASIC_INFORMATION64 { BaseAddress: ::winnt::ULONGLONG, AllocationBase: ::winnt::ULONGLONG, AllocationProtect: ::minwindef::DWORD, __alignment1: ::minwindef::DWORD, RegionSize: ::winnt::ULONGLONG, State: ::minwindef::DWORD, Protect: ::minwindef::DWORD, Type: ::minwindef::DWORD, __alignment2: ::minwindef::DWORD } /* winnt.h:12128:35, winnt.h:12128:35, winnt.h:12128:35 */
pub type PMEMORY_BASIC_INFORMATION64 = *mut ::winnt::MEMORY_BASIC_INFORMATION64; /* winnt.h:12138:32, winnt.h:12138:32, winnt.h:12138:32 */
#[repr(C)] pub struct FILE_ID_128 { Identifier: *mut [::minwindef::BYTE; 16] } /* winnt.h:12321:16, winnt.h:12321:16, winnt.h:12321:16 */
pub type PFILE_ID_128 = *mut ::winnt::FILE_ID_128; /* winnt.h:12323:17, winnt.h:12323:17, winnt.h:12323:17 */
#[repr(C)] pub struct FILE_NOTIFY_INFORMATION { NextEntryOffset: ::minwindef::DWORD, Action: ::minwindef::DWORD, FileNameLength: ::minwindef::DWORD, FileName: *mut [::winnt::WCHAR; 1] } /* winnt.h:12329:16, winnt.h:12329:16, winnt.h:12329:16 */
pub type PFILE_NOTIFY_INFORMATION = *mut ::winnt::FILE_NOTIFY_INFORMATION; /* winnt.h:12334:29, winnt.h:12334:29, winnt.h:12334:29 */
#[repr(C)] pub /*union*/ struct FILE_SEGMENT_ELEMENT; /* STUB! */ /* winnt.h:12341:15, winnt.h:12341:15, winnt.h:12341:15 */
pub type PFILE_SEGMENT_ELEMENT = *mut ::winnt::FILE_SEGMENT_ELEMENT; /* winnt.h:12344:25, winnt.h:12344:25, winnt.h:12344:25 */
#[repr(C)] pub struct REPARSE_GUID_DATA_BUFFER { ReparseTag: ::minwindef::DWORD, ReparseDataLength: ::minwindef::WORD, Reserved: ::minwindef::WORD, ReparseGuid: ::guiddef::GUID, GenericReparseBuffer: ::winnt::REPARSE_GUID_DATA_BUFFER_Child_4 } /* winnt.h:12401:16, winnt.h:12401:16, winnt.h:12401:16 */
#[repr(C)] pub struct REPARSE_GUID_DATA_BUFFER_Child_4 { DataBuffer: *mut [::minwindef::BYTE; 1] } /* winnt.h:12406:5, winnt.h:12406:5, winnt.h:12406:5 */
pub type PREPARSE_GUID_DATA_BUFFER = *mut ::winnt::REPARSE_GUID_DATA_BUFFER; /* winnt.h:12409:30, winnt.h:12409:30, winnt.h:12409:30 */
#[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub struct SCRUB_DATA_INPUT { Size: ::minwindef::DWORD, Flags: ::minwindef::DWORD, MaximumIos: ::minwindef::DWORD, Reserved: *mut [::minwindef::DWORD; 17], ResumeContext: *mut [::minwindef::BYTE; 816] } /* winnt.h:12505:16, winnt.h:12505:16, winnt.h:12505:16 */
#[cfg(any(feature="winapi_ver_06020000"))] pub type PSCRUB_DATA_INPUT = *mut ::winnt::SCRUB_DATA_INPUT; /* winnt.h:12544:22, winnt.h:12544:22, winnt.h:12544:22 */
#[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub struct SCRUB_DATA_OUTPUT { Size: ::minwindef::DWORD, Flags: ::minwindef::DWORD, Status: ::minwindef::DWORD, ErrorFileOffset: ::winnt::ULONGLONG, ErrorLength: ::winnt::ULONGLONG, NumberOfBytesRepaired: ::winnt::ULONGLONG, NumberOfBytesFailed: ::winnt::ULONGLONG, InternalFileReference: ::winnt::ULONGLONG, Reserved: *mut [::minwindef::DWORD; 6], ResumeContext: *mut [::minwindef::BYTE; 816] } /* winnt.h:12592:16, winnt.h:12592:16, winnt.h:12592:16 */
#[cfg(any(feature="winapi_ver_06020000"))] pub type PSCRUB_DATA_OUTPUT = *mut ::winnt::SCRUB_DATA_OUTPUT; /* winnt.h:12698:23, winnt.h:12698:23, winnt.h:12698:23 */
#[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub enum SharedVirtualDiskSupportType {SharedVirtualDisksUnsupported = 0, SharedVirtualDisksSupported = 1} pub use self::SharedVirtualDiskSupportType::{SharedVirtualDisksUnsupported, SharedVirtualDisksSupported}; /* winnt.h:12713:14, winnt.h:12713:14, winnt.h:12713:14 */
#[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub enum SharedVirtualDiskHandleState {SharedVirtualDiskHandleStateNone = 0, SharedVirtualDiskHandleStateFileShared = 1, SharedVirtualDiskHandleStateHandleShared = 3} pub use self::SharedVirtualDiskHandleState::{SharedVirtualDiskHandleStateNone, SharedVirtualDiskHandleStateFileShared, SharedVirtualDiskHandleStateHandleShared}; /* winnt.h:12726:14, winnt.h:12726:14, winnt.h:12726:14 */
#[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub struct SHARED_VIRTUAL_DISK_SUPPORT { SharedVirtualDiskSupport: ::winnt::SharedVirtualDiskSupportType, HandleState: ::winnt::SharedVirtualDiskHandleState } /* winnt.h:12750:16, winnt.h:12750:16, winnt.h:12750:16 */
#[cfg(any(feature="winapi_ver_06020000"))] pub type PSHARED_VIRTUAL_DISK_SUPPORT = *mut ::winnt::SHARED_VIRTUAL_DISK_SUPPORT; /* winnt.h:12762:33, winnt.h:12762:33, winnt.h:12762:33 */
#[repr(C)] pub enum SYSTEM_POWER_STATE {PowerSystemUnspecified = 0, PowerSystemWorking = 1, PowerSystemSleeping1 = 2, PowerSystemSleeping2 = 3, PowerSystemSleeping3 = 4, PowerSystemHibernate = 5, PowerSystemShutdown = 6, PowerSystemMaximum = 7} pub use self::SYSTEM_POWER_STATE::{PowerSystemUnspecified, PowerSystemWorking, PowerSystemSleeping1, PowerSystemSleeping2, PowerSystemSleeping3, PowerSystemHibernate, PowerSystemShutdown, PowerSystemMaximum}; /* winnt.h:13835:14, winnt.h:13835:14, winnt.h:13835:14 */
pub type PSYSTEM_POWER_STATE = *mut ::winnt::SYSTEM_POWER_STATE; /* winnt.h:13844:24, winnt.h:13844:24, winnt.h:13844:24 */
#[repr(C)] pub enum POWER_ACTION {PowerActionNone = 0, PowerActionReserved = 1, PowerActionSleep = 2, PowerActionHibernate = 3, PowerActionShutdown = 4, PowerActionShutdownReset = 5, PowerActionShutdownOff = 6, PowerActionWarmEject = 7} pub use self::POWER_ACTION::{PowerActionNone, PowerActionReserved, PowerActionSleep, PowerActionHibernate, PowerActionShutdown, PowerActionShutdownReset, PowerActionShutdownOff, PowerActionWarmEject}; /* winnt.h:13848:9, winnt.h:13848:9, winnt.h:13848:9 */
pub type PPOWER_ACTION = *mut ::winnt::POWER_ACTION; /* winnt.h:13857:18, winnt.h:13857:18, winnt.h:13857:18 */
#[repr(C)] pub enum DEVICE_POWER_STATE {PowerDeviceUnspecified = 0, PowerDeviceD0 = 1, PowerDeviceD1 = 2, PowerDeviceD2 = 3, PowerDeviceD3 = 4, PowerDeviceMaximum = 5} pub use self::DEVICE_POWER_STATE::{PowerDeviceUnspecified, PowerDeviceD0, PowerDeviceD1, PowerDeviceD2, PowerDeviceD3, PowerDeviceMaximum}; /* winnt.h:13859:14, winnt.h:13859:14, winnt.h:13859:14 */
pub type PDEVICE_POWER_STATE = *mut ::winnt::DEVICE_POWER_STATE; /* winnt.h:13866:24, winnt.h:13866:24, winnt.h:13866:24 */
#[repr(C)] pub enum MONITOR_DISPLAY_STATE {PowerMonitorOff = 0, PowerMonitorOn = 1, PowerMonitorDim = 2} pub use self::MONITOR_DISPLAY_STATE::{PowerMonitorOff, PowerMonitorOn, PowerMonitorDim}; /* winnt.h:13868:14, winnt.h:13868:14, winnt.h:13868:14 */
pub type PMONITOR_DISPLAY_STATE = *mut ::winnt::MONITOR_DISPLAY_STATE; /* winnt.h:13872:27, winnt.h:13872:27, winnt.h:13872:27 */
#[repr(C)] pub enum USER_ACTIVITY_PRESENCE {PowerUserPresent = 0, PowerUserNotPresent = 1, PowerUserInactive = 2, PowerUserMaximum = 3} pub use self::USER_ACTIVITY_PRESENCE::{PowerUserPresent, PowerUserNotPresent, PowerUserInactive, PowerUserMaximum}; pub const PowerUserInvalid: USER_ACTIVITY_PRESENCE = USER_ACTIVITY_PRESENCE::PowerUserMaximum; /* winnt.h:13874:14, winnt.h:13874:14, winnt.h:13874:14 */
pub type PUSER_ACTIVITY_PRESENCE = *mut ::winnt::USER_ACTIVITY_PRESENCE; /* winnt.h:13880:28, winnt.h:13880:28, winnt.h:13880:28 */
pub type EXECUTION_STATE = ::minwindef::DWORD; /* winnt.h:13890:15, winnt.h:13890:15, winnt.h:13890:15 */
pub type PEXECUTION_STATE = *mut ::libc::c_ulong; /* winnt.h:13890:33, winnt.h:13890:33, winnt.h:13890:33 */
#[repr(C)] pub enum LATENCY_TIME {LT_DONT_CARE = 0, LT_LOWEST_LATENCY = 1} pub use self::LATENCY_TIME::{LT_DONT_CARE, LT_LOWEST_LATENCY}; /* winnt.h:13892:9, winnt.h:13892:9, winnt.h:13892:9 */
#[repr(C)] pub enum POWER_REQUEST_TYPE {PowerRequestDisplayRequired = 0, PowerRequestSystemRequired = 1, PowerRequestAwayModeRequired = 2, PowerRequestExecutionRequired = 3} pub use self::POWER_REQUEST_TYPE::{PowerRequestDisplayRequired, PowerRequestSystemRequired, PowerRequestAwayModeRequired, PowerRequestExecutionRequired}; /* winnt.h:13913:14, winnt.h:13913:14, winnt.h:13913:14 */
pub type PPOWER_REQUEST_TYPE = *mut ::winnt::POWER_REQUEST_TYPE; /* winnt.h:13918:24, winnt.h:13918:24, winnt.h:13918:24 */
#[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct CM_Power_Data_s { PD_Size: ::minwindef::DWORD, PD_MostRecentPowerState: ::winnt::DEVICE_POWER_STATE, PD_Capabilities: ::minwindef::DWORD, PD_D1Latency: ::minwindef::DWORD, PD_D2Latency: ::minwindef::DWORD, PD_D3Latency: ::minwindef::DWORD, PD_PowerStateMapping: *mut [::winnt::DEVICE_POWER_STATE; 7], PD_DeepestSystemWake: ::winnt::SYSTEM_POWER_STATE } /* winnt.h:13939:16, winnt.h:13939:16, winnt.h:13939:16 */
#[cfg(any(feature="winapi_ver_05010000"))] pub type CM_POWER_DATA = ::winnt::CM_Power_Data_s; /* winnt.h:13948:3, winnt.h:13948:3, winnt.h:13948:3 */
#[cfg(any(feature="winapi_ver_05010000"))] pub type PCM_POWER_DATA = *mut ::winnt::CM_Power_Data_s; /* winnt.h:13948:19, winnt.h:13948:19, winnt.h:13948:19 */
#[repr(C)] pub enum POWER_INFORMATION_LEVEL {SystemPowerPolicyAc = 0, SystemPowerPolicyDc = 1, VerifySystemPolicyAc = 2, VerifySystemPolicyDc = 3, SystemPowerCapabilities = 4, SystemBatteryState = 5, SystemPowerStateHandler = 6, ProcessorStateHandler = 7, SystemPowerPolicyCurrent = 8, AdministratorPowerPolicy = 9, SystemReserveHiberFile = 10, ProcessorInformation = 11, SystemPowerInformation = 12, ProcessorStateHandler2 = 13, LastWakeTime = 14, LastSleepTime = 15, SystemExecutionState = 16, SystemPowerStateNotifyHandler = 17, ProcessorPowerPolicyAc = 18, ProcessorPowerPolicyDc = 19, VerifyProcessorPowerPolicyAc = 20, VerifyProcessorPowerPolicyDc = 21, ProcessorPowerPolicyCurrent = 22, SystemPowerStateLogging = 23, SystemPowerLoggingEntry = 24, SetPowerSettingValue = 25, NotifyUserPowerSetting = 26, PowerInformationLevelUnused0 = 27, SystemMonitorHiberBootPowerOff = 28, SystemVideoState = 29, TraceApplicationPowerMessage = 30, TraceApplicationPowerMessageEnd = 31, ProcessorPerfStates = 32, ProcessorIdleStates = 33, ProcessorCap = 34, SystemWakeSource = 35, SystemHiberFileInformation = 36, TraceServicePowerMessage = 37, ProcessorLoad = 38, PowerShutdownNotification = 39, MonitorCapabilities = 40, SessionPowerInit = 41, SessionDisplayState = 42, PowerRequestCreate = 43, PowerRequestAction = 44, GetPowerRequestList = 45, ProcessorInformationEx = 46, NotifyUserModeLegacyPowerEvent = 47, GroupPark = 48, ProcessorIdleDomains = 49, WakeTimerList = 50, SystemHiberFileSize = 51, ProcessorIdleStatesHv = 52, ProcessorPerfStatesHv = 53, ProcessorPerfCapHv = 54, ProcessorSetIdle = 55, LogicalProcessorIdling = 56, UserPresence = 57, PowerSettingNotificationName = 58, GetPowerSettingValue = 59, IdleResiliency = 60, SessionRITState = 61, SessionConnectNotification = 62, SessionPowerCleanup = 63, SessionLockState = 64, SystemHiberbootState = 65, PlatformInformation = 66, PdcInvocation = 67, MonitorInvocation = 68, FirmwareTableInformationRegistered = 69, SetShutdownSelectedTime = 70, SuspendResumeInvocation = 71, PlmPowerRequestCreate = 72, ScreenOff = 73, CsDeviceNotification = 74, PlatformRole = 75, LastResumePerformance = 76, DisplayBurst = 77, ExitLatencySamplingPercentage = 78, RegisterSpmPowerSettings = 79, PlatformIdleStates = 80, ProcessorIdleVeto = 81, PlatformIdleVeto = 82, SystemBatteryStatePrecise = 83, ThermalEvent = 84, PowerInformationLevelMaximum = 85} pub use self::POWER_INFORMATION_LEVEL::{SystemPowerPolicyAc, SystemPowerPolicyDc, VerifySystemPolicyAc, VerifySystemPolicyDc, SystemPowerCapabilities, SystemBatteryState, SystemPowerStateHandler, ProcessorStateHandler, SystemPowerPolicyCurrent, AdministratorPowerPolicy, SystemReserveHiberFile, ProcessorInformation, SystemPowerInformation, ProcessorStateHandler2, LastWakeTime, LastSleepTime, SystemExecutionState, SystemPowerStateNotifyHandler, ProcessorPowerPolicyAc, ProcessorPowerPolicyDc, VerifyProcessorPowerPolicyAc, VerifyProcessorPowerPolicyDc, ProcessorPowerPolicyCurrent, SystemPowerStateLogging, SystemPowerLoggingEntry, SetPowerSettingValue, NotifyUserPowerSetting, PowerInformationLevelUnused0, SystemMonitorHiberBootPowerOff, SystemVideoState, TraceApplicationPowerMessage, TraceApplicationPowerMessageEnd, ProcessorPerfStates, ProcessorIdleStates, ProcessorCap, SystemWakeSource, SystemHiberFileInformation, TraceServicePowerMessage, ProcessorLoad, PowerShutdownNotification, MonitorCapabilities, SessionPowerInit, SessionDisplayState, PowerRequestCreate, PowerRequestAction, GetPowerRequestList, ProcessorInformationEx, NotifyUserModeLegacyPowerEvent, GroupPark, ProcessorIdleDomains, WakeTimerList, SystemHiberFileSize, ProcessorIdleStatesHv, ProcessorPerfStatesHv, ProcessorPerfCapHv, ProcessorSetIdle, LogicalProcessorIdling, UserPresence, PowerSettingNotificationName, GetPowerSettingValue, IdleResiliency, SessionRITState, SessionConnectNotification, SessionPowerCleanup, SessionLockState, SystemHiberbootState, PlatformInformation, PdcInvocation, MonitorInvocation, FirmwareTableInformationRegistered, SetShutdownSelectedTime, SuspendResumeInvocation, PlmPowerRequestCreate, ScreenOff, CsDeviceNotification, PlatformRole, LastResumePerformance, DisplayBurst, ExitLatencySamplingPercentage, RegisterSpmPowerSettings, PlatformIdleStates, ProcessorIdleVeto, PlatformIdleVeto, SystemBatteryStatePrecise, ThermalEvent, PowerInformationLevelMaximum}; /* winnt.h:13954:9, winnt.h:13954:9, winnt.h:13954:9 */
#[repr(C)] pub enum POWER_USER_PRESENCE_TYPE {UserNotPresent = 0, UserPresent = 1, UserUnknown = 255} pub use self::POWER_USER_PRESENCE_TYPE::{UserNotPresent, UserPresent, UserUnknown}; /* winnt.h:14047:9, winnt.h:14047:9, winnt.h:14047:9 */
pub type PPOWER_USER_PRESENCE_TYPE = *mut ::winnt::POWER_USER_PRESENCE_TYPE; /* winnt.h:14051:30, winnt.h:14051:30, winnt.h:14051:30 */
#[repr(C)] pub struct POWER_USER_PRESENCE { UserPresence: ::winnt::POWER_USER_PRESENCE_TYPE } /* winnt.h:14053:16, winnt.h:14053:16, winnt.h:14053:16 */
pub type PPOWER_USER_PRESENCE = *mut ::winnt::POWER_USER_PRESENCE; /* winnt.h:14055:25, winnt.h:14055:25, winnt.h:14055:25 */
#[repr(C)] pub struct POWER_SESSION_CONNECT { Connected: ::winnt::BOOLEAN, Console: ::winnt::BOOLEAN } /* winnt.h:14060:16, winnt.h:14060:16, winnt.h:14060:16 */
pub type PPOWER_SESSION_CONNECT = *mut ::winnt::POWER_SESSION_CONNECT; /* winnt.h:14063:27, winnt.h:14063:27, winnt.h:14063:27 */
#[repr(C)] pub struct POWER_SESSION_TIMEOUTS { InputTimeout: ::minwindef::DWORD, DisplayTimeout: ::minwindef::DWORD } /* winnt.h:14065:16, winnt.h:14065:16, winnt.h:14065:16 */
pub type PPOWER_SESSION_TIMEOUTS = *mut ::winnt::POWER_SESSION_TIMEOUTS; /* winnt.h:14068:28, winnt.h:14068:28, winnt.h:14068:28 */
#[repr(C)] pub struct POWER_SESSION_RIT_STATE { Active: ::winnt::BOOLEAN, LastInputTime: ::minwindef::DWORD } /* winnt.h:14073:16, winnt.h:14073:16, winnt.h:14073:16 */
pub type PPOWER_SESSION_RIT_STATE = *mut ::winnt::POWER_SESSION_RIT_STATE; /* winnt.h:14076:29, winnt.h:14076:29, winnt.h:14076:29 */
#[repr(C)] pub struct POWER_SESSION_WINLOGON { SessionId: ::minwindef::DWORD, Console: ::winnt::BOOLEAN, Locked: ::winnt::BOOLEAN } /* winnt.h:14081:16, winnt.h:14081:16, winnt.h:14081:16 */
pub type PPOWER_SESSION_WINLOGON = *mut ::winnt::POWER_SESSION_WINLOGON; /* winnt.h:14085:28, winnt.h:14085:28, winnt.h:14085:28 */
#[repr(C)] pub struct POWER_IDLE_RESILIENCY { CoalescingTimeout: ::minwindef::DWORD, IdleResiliencyPeriod: ::minwindef::DWORD } /* winnt.h:14090:16, winnt.h:14090:16, winnt.h:14090:16 */
pub type PPOWER_IDLE_RESILIENCY = *mut ::winnt::POWER_IDLE_RESILIENCY; /* winnt.h:14093:27, winnt.h:14093:27, winnt.h:14093:27 */
#[repr(C)] pub enum POWER_MONITOR_REQUEST_REASON {MonitorRequestReasonUnknown = 0, MonitorRequestReasonPowerButton = 1, MonitorRequestReasonRemoteConnection = 2, MonitorRequestReasonScMonitorpower = 3, MonitorRequestReasonUserInput = 4, MonitorRequestReasonAcDcDisplayBurst = 5, MonitorRequestReasonUserDisplayBurst = 6, MonitorRequestReasonPoSetSystemState = 7, MonitorRequestReasonSetThreadExecutionState = 8, MonitorRequestReasonFullWake = 9, MonitorRequestReasonSessionUnlock = 10, MonitorRequestReasonScreenOffRequest = 11, MonitorRequestReasonIdleTimeout = 12, MonitorRequestReasonPolicyChange = 13, MonitorRequestReasonSleepButton = 14, MonitorRequestReasonLid = 15, MonitorRequestReasonBatteryCountChange = 16, MonitorRequestReasonMax = 17} pub use self::POWER_MONITOR_REQUEST_REASON::{MonitorRequestReasonUnknown, MonitorRequestReasonPowerButton, MonitorRequestReasonRemoteConnection, MonitorRequestReasonScMonitorpower, MonitorRequestReasonUserInput, MonitorRequestReasonAcDcDisplayBurst, MonitorRequestReasonUserDisplayBurst, MonitorRequestReasonPoSetSystemState, MonitorRequestReasonSetThreadExecutionState, MonitorRequestReasonFullWake, MonitorRequestReasonSessionUnlock, MonitorRequestReasonScreenOffRequest, MonitorRequestReasonIdleTimeout, MonitorRequestReasonPolicyChange, MonitorRequestReasonSleepButton, MonitorRequestReasonLid, MonitorRequestReasonBatteryCountChange, MonitorRequestReasonMax}; /* winnt.h:14098:9, winnt.h:14098:9, winnt.h:14098:9 */
#[repr(C)] pub struct POWER_MONITOR_INVOCATION { On: ::winnt::BOOLEAN, Console: ::winnt::BOOLEAN, RequestReason: ::winnt::POWER_MONITOR_REQUEST_REASON } /* winnt.h:14122:16, winnt.h:14122:16, winnt.h:14122:16 */
pub type PPOWER_MONITOR_INVOCATION = *mut ::winnt::POWER_MONITOR_INVOCATION; /* winnt.h:14126:30, winnt.h:14126:30, winnt.h:14126:30 */
#[repr(C)] pub struct RESUME_PERFORMANCE { PostTimeMs: ::minwindef::DWORD, TotalResumeTimeMs: ::winnt::ULONGLONG, ResumeCompleteTimestamp: ::winnt::ULONGLONG } /* winnt.h:14132:16, winnt.h:14132:16, winnt.h:14132:16 */
pub type PRESUME_PERFORMANCE = *mut ::winnt::RESUME_PERFORMANCE; /* winnt.h:14136:24, winnt.h:14136:24, winnt.h:14136:24 */
#[repr(C)] pub enum SYSTEM_POWER_CONDITION {PoAc = 0, PoDc = 1, PoHot = 2, PoConditionMaximum = 3} pub use self::SYSTEM_POWER_CONDITION::{PoAc, PoDc, PoHot, PoConditionMaximum}; /* winnt.h:14142:9, winnt.h:14142:9, winnt.h:14142:9 */
#[repr(C)] pub struct SET_POWER_SETTING_VALUE { Version: ::minwindef::DWORD, Guid: ::guiddef::GUID, PowerCondition: ::winnt::SYSTEM_POWER_CONDITION, DataLength: ::minwindef::DWORD, Data: *mut [::minwindef::BYTE; 1] } /* winnt.h:14149:9, winnt.h:14149:9, winnt.h:14149:9 */
pub type PSET_POWER_SETTING_VALUE = *mut ::winnt::SET_POWER_SETTING_VALUE; /* winnt.h:14179:29, winnt.h:14179:29, winnt.h:14179:29 */
#[repr(C)] pub struct NOTIFY_USER_POWER_SETTING { Guid: ::guiddef::GUID } /* winnt.h:14183:9, winnt.h:14183:9, winnt.h:14183:9 */
pub type PNOTIFY_USER_POWER_SETTING = *mut ::winnt::NOTIFY_USER_POWER_SETTING; /* winnt.h:14185:31, winnt.h:14185:31, winnt.h:14185:31 */
#[repr(C)] pub struct APPLICATIONLAUNCH_SETTING_VALUE { ActivationTime: ::winnt::LARGE_INTEGER, Flags: ::minwindef::DWORD, ButtonInstanceID: ::minwindef::DWORD } /* winnt.h:14192:16, winnt.h:14192:16, winnt.h:14192:16 */
pub type PAPPLICATIONLAUNCH_SETTING_VALUE = *mut ::winnt::APPLICATIONLAUNCH_SETTING_VALUE; /* winnt.h:14211:37, winnt.h:14211:37, winnt.h:14211:37 */
#[repr(C)] pub enum POWER_PLATFORM_ROLE {PlatformRoleUnspecified = 0, PlatformRoleDesktop = 1, PlatformRoleMobile = 2, PlatformRoleWorkstation = 3, PlatformRoleEnterpriseServer = 4, PlatformRoleSOHOServer = 5, PlatformRoleAppliancePC = 6, PlatformRolePerformanceServer = 7, PlatformRoleSlate = 8, PlatformRoleMaximum = 9} pub use self::POWER_PLATFORM_ROLE::{PlatformRoleUnspecified, PlatformRoleDesktop, PlatformRoleMobile, PlatformRoleWorkstation, PlatformRoleEnterpriseServer, PlatformRoleSOHOServer, PlatformRoleAppliancePC, PlatformRolePerformanceServer, PlatformRoleSlate, PlatformRoleMaximum}; /* winnt.h:14217:14, winnt.h:14217:14, winnt.h:14217:14 */
pub type PPOWER_PLATFORM_ROLE = *mut ::winnt::POWER_PLATFORM_ROLE; /* winnt.h:14228:25, winnt.h:14228:25, winnt.h:14228:25 */
#[repr(C)] pub struct POWER_PLATFORM_INFORMATION { AoAc: ::winnt::BOOLEAN } /* winnt.h:14248:16, winnt.h:14248:16, winnt.h:14248:16 */
pub type PPOWER_PLATFORM_INFORMATION = *mut ::winnt::POWER_PLATFORM_INFORMATION; /* winnt.h:14250:32, winnt.h:14250:32, winnt.h:14250:32 */
#[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct BATTERY_REPORTING_SCALE { Granularity: ::minwindef::DWORD, Capacity: ::minwindef::DWORD } /* winnt.h:14257:9, winnt.h:14257:9, winnt.h:14257:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub type PBATTERY_REPORTING_SCALE = *mut ::winnt::BATTERY_REPORTING_SCALE; /* winnt.h:14260:29, winnt.h:14260:29, winnt.h:14260:29 */
#[repr(C)] pub struct PPM_WMI_LEGACY_PERFSTATE { Frequency: ::minwindef::DWORD, Flags: ::minwindef::DWORD, PercentFrequency: ::minwindef::DWORD } /* winnt.h:14265:9, winnt.h:14265:9, winnt.h:14265:9 */
pub type PPPM_WMI_LEGACY_PERFSTATE = *mut ::winnt::PPM_WMI_LEGACY_PERFSTATE; /* winnt.h:14269:30, winnt.h:14269:30, winnt.h:14269:30 */
#[repr(C)] pub struct PPM_WMI_IDLE_STATE { Latency: ::minwindef::DWORD, Power: ::minwindef::DWORD, TimeCheck: ::minwindef::DWORD, PromotePercent: ::minwindef::BYTE, DemotePercent: ::minwindef::BYTE, StateType: ::minwindef::BYTE, Reserved: ::minwindef::BYTE, StateFlags: ::minwindef::DWORD, Context: ::minwindef::DWORD, IdleHandler: ::minwindef::DWORD, Reserved1: ::minwindef::DWORD } /* winnt.h:14271:9, winnt.h:14271:9, winnt.h:14271:9 */
pub type PPPM_WMI_IDLE_STATE = *mut ::winnt::PPM_WMI_IDLE_STATE; /* winnt.h:14283:24, winnt.h:14283:24, winnt.h:14283:24 */
#[repr(C)] pub struct PPM_WMI_IDLE_STATES { Type: ::minwindef::DWORD, Count: ::minwindef::DWORD, TargetState: ::minwindef::DWORD, OldState: ::minwindef::DWORD, TargetProcessors: ::basetsd::DWORD64, State: *mut [::winnt::PPM_WMI_IDLE_STATE; 1] } /* winnt.h:14285:9, winnt.h:14285:9, winnt.h:14285:9 */
pub type PPPM_WMI_IDLE_STATES = *mut ::winnt::PPM_WMI_IDLE_STATES; /* winnt.h:14292:25, winnt.h:14292:25, winnt.h:14292:25 */
#[repr(C)] pub struct PPM_WMI_IDLE_STATES_EX { Type: ::minwindef::DWORD, Count: ::minwindef::DWORD, TargetState: ::minwindef::DWORD, OldState: ::minwindef::DWORD, TargetProcessors: ::winnt::PVOID, State: *mut [::winnt::PPM_WMI_IDLE_STATE; 1] } /* winnt.h:14294:9, winnt.h:14294:9, winnt.h:14294:9 */
pub type PPPM_WMI_IDLE_STATES_EX = *mut ::winnt::PPM_WMI_IDLE_STATES_EX; /* winnt.h:14301:28, winnt.h:14301:28, winnt.h:14301:28 */
#[repr(C)] pub struct PPM_WMI_PERF_STATE { Frequency: ::minwindef::DWORD, Power: ::minwindef::DWORD, PercentFrequency: ::minwindef::BYTE, IncreaseLevel: ::minwindef::BYTE, DecreaseLevel: ::minwindef::BYTE, Type: ::minwindef::BYTE, IncreaseTime: ::minwindef::DWORD, DecreaseTime: ::minwindef::DWORD, Control: ::basetsd::DWORD64, Status: ::basetsd::DWORD64, HitCount: ::minwindef::DWORD, Reserved1: ::minwindef::DWORD, Reserved2: ::basetsd::DWORD64, Reserved3: ::basetsd::DWORD64 } /* winnt.h:14303:9, winnt.h:14303:9, winnt.h:14303:9 */
pub type PPPM_WMI_PERF_STATE = *mut ::winnt::PPM_WMI_PERF_STATE; /* winnt.h:14318:24, winnt.h:14318:24, winnt.h:14318:24 */
#[repr(C)] pub struct PPM_WMI_PERF_STATES { Count: ::minwindef::DWORD, MaxFrequency: ::minwindef::DWORD, CurrentState: ::minwindef::DWORD, MaxPerfState: ::minwindef::DWORD, MinPerfState: ::minwindef::DWORD, LowestPerfState: ::minwindef::DWORD, ThermalConstraint: ::minwindef::DWORD, BusyAdjThreshold: ::minwindef::BYTE, PolicyType: ::minwindef::BYTE, Type: ::minwindef::BYTE, Reserved: ::minwindef::BYTE, TimerInterval: ::minwindef::DWORD, TargetProcessors: ::basetsd::DWORD64, PStateHandler: ::minwindef::DWORD, PStateContext: ::minwindef::DWORD, TStateHandler: ::minwindef::DWORD, TStateContext: ::minwindef::DWORD, FeedbackHandler: ::minwindef::DWORD, Reserved1: ::minwindef::DWORD, Reserved2: ::basetsd::DWORD64, State: *mut [::winnt::PPM_WMI_PERF_STATE; 1] } /* winnt.h:14320:9, winnt.h:14320:9, winnt.h:14320:9 */
pub type PPPM_WMI_PERF_STATES = *mut ::winnt::PPM_WMI_PERF_STATES; /* winnt.h:14342:25, winnt.h:14342:25, winnt.h:14342:25 */
#[repr(C)] pub struct PPM_WMI_PERF_STATES_EX { Count: ::minwindef::DWORD, MaxFrequency: ::minwindef::DWORD, CurrentState: ::minwindef::DWORD, MaxPerfState: ::minwindef::DWORD, MinPerfState: ::minwindef::DWORD, LowestPerfState: ::minwindef::DWORD, ThermalConstraint: ::minwindef::DWORD, BusyAdjThreshold: ::minwindef::BYTE, PolicyType: ::minwindef::BYTE, Type: ::minwindef::BYTE, Reserved: ::minwindef::BYTE, TimerInterval: ::minwindef::DWORD, TargetProcessors: ::winnt::PVOID, PStateHandler: ::minwindef::DWORD, PStateContext: ::minwindef::DWORD, TStateHandler: ::minwindef::DWORD, TStateContext: ::minwindef::DWORD, FeedbackHandler: ::minwindef::DWORD, Reserved1: ::minwindef::DWORD, Reserved2: ::basetsd::DWORD64, State: *mut [::winnt::PPM_WMI_PERF_STATE; 1] } /* winnt.h:14344:9, winnt.h:14344:9, winnt.h:14344:9 */
pub type PPPM_WMI_PERF_STATES_EX = *mut ::winnt::PPM_WMI_PERF_STATES_EX; /* winnt.h:14366:28, winnt.h:14366:28, winnt.h:14366:28 */
#[repr(C)] pub struct PPM_IDLE_STATE_ACCOUNTING { IdleTransitions: ::minwindef::DWORD, FailedTransitions: ::minwindef::DWORD, InvalidBucketIndex: ::minwindef::DWORD, TotalTime: ::basetsd::DWORD64, IdleTimeBuckets: *mut [::minwindef::DWORD; 6] } /* winnt.h:14374:9, winnt.h:14374:9, winnt.h:14374:9 */
pub type PPPM_IDLE_STATE_ACCOUNTING = *mut ::winnt::PPM_IDLE_STATE_ACCOUNTING; /* winnt.h:14380:31, winnt.h:14380:31, winnt.h:14380:31 */
#[repr(C)] pub struct PPM_IDLE_ACCOUNTING { StateCount: ::minwindef::DWORD, TotalTransitions: ::minwindef::DWORD, ResetCount: ::minwindef::DWORD, StartTime: ::basetsd::DWORD64, State: *mut [::winnt::PPM_IDLE_STATE_ACCOUNTING; 1] } /* winnt.h:14382:9, winnt.h:14382:9, winnt.h:14382:9 */
pub type PPPM_IDLE_ACCOUNTING = *mut ::winnt::PPM_IDLE_ACCOUNTING; /* winnt.h:14388:25, winnt.h:14388:25, winnt.h:14388:25 */
#[repr(C)] pub struct PPM_IDLE_STATE_BUCKET_EX { TotalTimeUs: ::basetsd::DWORD64, MinTimeUs: ::minwindef::DWORD, MaxTimeUs: ::minwindef::DWORD, Count: ::minwindef::DWORD } /* winnt.h:14396:9, winnt.h:14396:9, winnt.h:14396:9 */
pub type PPPM_IDLE_STATE_BUCKET_EX = *mut ::winnt::PPM_IDLE_STATE_BUCKET_EX; /* winnt.h:14401:30, winnt.h:14401:30, winnt.h:14401:30 */
#[repr(C)] pub struct PPM_IDLE_STATE_ACCOUNTING_EX { TotalTime: ::basetsd::DWORD64, IdleTransitions: ::minwindef::DWORD, FailedTransitions: ::minwindef::DWORD, InvalidBucketIndex: ::minwindef::DWORD, MinTimeUs: ::minwindef::DWORD, MaxTimeUs: ::minwindef::DWORD, CancelledTransitions: ::minwindef::DWORD, IdleTimeBuckets: *mut [::winnt::PPM_IDLE_STATE_BUCKET_EX; 16] } /* winnt.h:14403:9, winnt.h:14403:9, winnt.h:14403:9 */
pub type PPPM_IDLE_STATE_ACCOUNTING_EX = *mut ::winnt::PPM_IDLE_STATE_ACCOUNTING_EX; /* winnt.h:14412:34, winnt.h:14412:34, winnt.h:14412:34 */
#[repr(C)] pub struct PPM_IDLE_ACCOUNTING_EX { StateCount: ::minwindef::DWORD, TotalTransitions: ::minwindef::DWORD, ResetCount: ::minwindef::DWORD, AbortCount: ::minwindef::DWORD, StartTime: ::basetsd::DWORD64, State: *mut [::winnt::PPM_IDLE_STATE_ACCOUNTING_EX; 1] } /* winnt.h:14414:9, winnt.h:14414:9, winnt.h:14414:9 */
pub type PPPM_IDLE_ACCOUNTING_EX = *mut ::winnt::PPM_IDLE_ACCOUNTING_EX; /* winnt.h:14421:28, winnt.h:14421:28, winnt.h:14421:28 */
#[repr(C)] pub struct PPM_PERFSTATE_EVENT { State: ::minwindef::DWORD, Status: ::minwindef::DWORD, Latency: ::minwindef::DWORD, Speed: ::minwindef::DWORD, Processor: ::minwindef::DWORD } /* winnt.h:14522:9, winnt.h:14522:9, winnt.h:14522:9 */
pub type PPPM_PERFSTATE_EVENT = *mut ::winnt::PPM_PERFSTATE_EVENT; /* winnt.h:14528:25, winnt.h:14528:25, winnt.h:14528:25 */
#[repr(C)] pub struct PPM_PERFSTATE_DOMAIN_EVENT { State: ::minwindef::DWORD, Latency: ::minwindef::DWORD, Speed: ::minwindef::DWORD, Processors: ::basetsd::DWORD64 } /* winnt.h:14530:9, winnt.h:14530:9, winnt.h:14530:9 */
pub type PPPM_PERFSTATE_DOMAIN_EVENT = *mut ::winnt::PPM_PERFSTATE_DOMAIN_EVENT; /* winnt.h:14535:32, winnt.h:14535:32, winnt.h:14535:32 */
#[repr(C)] pub struct PPM_IDLESTATE_EVENT { NewState: ::minwindef::DWORD, OldState: ::minwindef::DWORD, Processors: ::basetsd::DWORD64 } /* winnt.h:14537:9, winnt.h:14537:9, winnt.h:14537:9 */
pub type PPPM_IDLESTATE_EVENT = *mut ::winnt::PPM_IDLESTATE_EVENT; /* winnt.h:14541:25, winnt.h:14541:25, winnt.h:14541:25 */
#[repr(C)] pub struct PPM_THERMALCHANGE_EVENT { ThermalConstraint: ::minwindef::DWORD, Processors: ::basetsd::DWORD64 } /* winnt.h:14543:9, winnt.h:14543:9, winnt.h:14543:9 */
pub type PPPM_THERMALCHANGE_EVENT = *mut ::winnt::PPM_THERMALCHANGE_EVENT; /* winnt.h:14546:29, winnt.h:14546:29, winnt.h:14546:29 */
#[repr(C)] pub struct PPM_THERMAL_POLICY_EVENT { Mode: ::minwindef::BYTE, Processors: ::basetsd::DWORD64 } /* winnt.h:14551:9, winnt.h:14551:9, winnt.h:14551:9 */
pub type PPPM_THERMAL_POLICY_EVENT = *mut ::winnt::PPM_THERMAL_POLICY_EVENT; /* winnt.h:14554:30, winnt.h:14554:30, winnt.h:14554:30 */
#[repr(C)] pub struct POWER_ACTION_POLICY { Action: ::winnt::POWER_ACTION, Flags: ::minwindef::DWORD, EventCode: ::minwindef::DWORD } /* winnt.h:14561:9, winnt.h:14561:9, winnt.h:14561:9 */
pub type PPOWER_ACTION_POLICY = *mut ::winnt::POWER_ACTION_POLICY; /* winnt.h:14565:25, winnt.h:14565:25, winnt.h:14565:25 */
#[repr(C)] pub struct SYSTEM_POWER_LEVEL { Enable: ::winnt::BOOLEAN, Spare: *mut [::minwindef::BYTE; 3], BatteryLevel: ::minwindef::DWORD, PowerPolicy: ::winnt::POWER_ACTION_POLICY, MinSystemState: ::winnt::SYSTEM_POWER_STATE } /* winnt.h:14598:9, winnt.h:14598:9, winnt.h:14598:9 */
pub type PSYSTEM_POWER_LEVEL = *mut ::winnt::SYSTEM_POWER_LEVEL; /* winnt.h:14604:24, winnt.h:14604:24, winnt.h:14604:24 */
#[repr(C)] pub struct SYSTEM_POWER_POLICY { Revision: ::minwindef::DWORD, PowerButton: ::winnt::POWER_ACTION_POLICY, SleepButton: ::winnt::POWER_ACTION_POLICY, LidClose: ::winnt::POWER_ACTION_POLICY, LidOpenWake: ::winnt::SYSTEM_POWER_STATE, Reserved: ::minwindef::DWORD, Idle: ::winnt::POWER_ACTION_POLICY, IdleTimeout: ::minwindef::DWORD, IdleSensitivity: ::minwindef::BYTE, DynamicThrottle: ::minwindef::BYTE, Spare2: *mut [::minwindef::BYTE; 2], MinSleep: ::winnt::SYSTEM_POWER_STATE, MaxSleep: ::winnt::SYSTEM_POWER_STATE, ReducedLatencySleep: ::winnt::SYSTEM_POWER_STATE, WinLogonFlags: ::minwindef::DWORD, Spare3: ::minwindef::DWORD, DozeS4Timeout: ::minwindef::DWORD, BroadcastCapacityResolution: ::minwindef::DWORD, DischargePolicy: *mut [::winnt::SYSTEM_POWER_LEVEL; 4], VideoTimeout: ::minwindef::DWORD, VideoDimDisplay: ::winnt::BOOLEAN, VideoReserved: *mut [::minwindef::DWORD; 3], SpindownTimeout: ::minwindef::DWORD, OptimizeForPower: ::winnt::BOOLEAN, FanThrottleTolerance: ::minwindef::BYTE, ForcedThrottle: ::minwindef::BYTE, MinThrottle: ::minwindef::BYTE, OverThrottled: ::winnt::POWER_ACTION_POLICY } /* winnt.h:14613:16, winnt.h:14613:16, winnt.h:14613:16 */
pub type PSYSTEM_POWER_POLICY = *mut ::winnt::SYSTEM_POWER_POLICY; /* winnt.h:14662:25, winnt.h:14662:25, winnt.h:14662:25 */
#[repr(C)] pub struct PROCESSOR_IDLESTATE_INFO { TimeCheck: ::minwindef::DWORD, DemotePercent: ::minwindef::BYTE, PromotePercent: ::minwindef::BYTE, Spare: *mut [::minwindef::BYTE; 2] } /* winnt.h:14673:9, winnt.h:14673:9, winnt.h:14673:9 */
pub type PPROCESSOR_IDLESTATE_INFO = *mut ::winnt::PROCESSOR_IDLESTATE_INFO; /* winnt.h:14678:30, winnt.h:14678:30, winnt.h:14678:30 */
#[repr(C)] pub struct PROCESSOR_IDLESTATE_POLICY { Revision: ::minwindef::WORD, Flags: ::winnt::PROCESSOR_IDLESTATE_POLICY_Child_1, PolicyCount: ::minwindef::DWORD, Policy: *mut [::winnt::PROCESSOR_IDLESTATE_INFO; 3] } /* winnt.h:14680:9, winnt.h:14680:9, winnt.h:14680:9 */
#[repr(C)] pub /*union*/ struct PROCESSOR_IDLESTATE_POLICY_Child_1; /* STUB! */ /* winnt.h:14682:5, winnt.h:14682:5, winnt.h:14682:5 */
pub type PPROCESSOR_IDLESTATE_POLICY = *mut ::winnt::PROCESSOR_IDLESTATE_POLICY; /* winnt.h:14693:32, winnt.h:14693:32, winnt.h:14693:32 */
#[repr(C)] pub struct PROCESSOR_POWER_POLICY_INFO { TimeCheck: ::minwindef::DWORD, DemoteLimit: ::minwindef::DWORD, PromoteLimit: ::minwindef::DWORD, DemotePercent: ::minwindef::BYTE, PromotePercent: ::minwindef::BYTE, Spare: *mut [::minwindef::BYTE; 2], AllowDemotion: ::minwindef::DWORD, AllowPromotion: ::minwindef::DWORD, Reserved: ::minwindef::DWORD } /* winnt.h:14708:16, winnt.h:14708:16, winnt.h:14708:16 */
pub type PPROCESSOR_POWER_POLICY_INFO = *mut ::winnt::PROCESSOR_POWER_POLICY_INFO; /* winnt.h:14725:33, winnt.h:14725:33, winnt.h:14725:33 */
#[repr(C)] pub struct PROCESSOR_POWER_POLICY { Revision: ::minwindef::DWORD, DynamicThrottle: ::minwindef::BYTE, Spare: *mut [::minwindef::BYTE; 3], DisableCStates: ::minwindef::DWORD, Reserved: ::minwindef::DWORD, PolicyCount: ::minwindef::DWORD, Policy: *mut [::winnt::PROCESSOR_POWER_POLICY_INFO; 3] } /* winnt.h:14728:16, winnt.h:14728:16, winnt.h:14728:16 */
pub type PPROCESSOR_POWER_POLICY = *mut ::winnt::PROCESSOR_POWER_POLICY; /* winnt.h:14745:28, winnt.h:14745:28, winnt.h:14745:28 */
#[repr(C)] pub struct PROCESSOR_PERFSTATE_POLICY { Revision: ::minwindef::DWORD, MaxThrottle: ::minwindef::BYTE, MinThrottle: ::minwindef::BYTE, BusyAdjThreshold: ::minwindef::BYTE, u: ::winnt::PROCESSOR_PERFSTATE_POLICY_Child_4, TimeCheck: ::minwindef::DWORD, IncreaseTime: ::minwindef::DWORD, DecreaseTime: ::minwindef::DWORD, IncreasePercent: ::minwindef::DWORD, DecreasePercent: ::minwindef::DWORD } /* winnt.h:14751:9, winnt.h:14751:9, winnt.h:14751:9 */
#[repr(C)] pub /*union*/ struct PROCESSOR_PERFSTATE_POLICY_Child_4; /* STUB! */ /* winnt.h:14756:5, winnt.h:14756:5, winnt.h:14756:5 */
pub type PPROCESSOR_PERFSTATE_POLICY = *mut ::winnt::PROCESSOR_PERFSTATE_POLICY; /* winnt.h:14774:32, winnt.h:14774:32, winnt.h:14774:32 */
#[repr(C)] pub struct ADMINISTRATOR_POWER_POLICY { MinSleep: ::winnt::SYSTEM_POWER_STATE, MaxSleep: ::winnt::SYSTEM_POWER_STATE, MinVideoTimeout: ::minwindef::DWORD, MaxVideoTimeout: ::minwindef::DWORD, MinSpindownTimeout: ::minwindef::DWORD, MaxSpindownTimeout: ::minwindef::DWORD } /* winnt.h:14777:16, winnt.h:14777:16, winnt.h:14777:16 */
pub type PADMINISTRATOR_POWER_POLICY = *mut ::winnt::ADMINISTRATOR_POWER_POLICY; /* winnt.h:14790:32, winnt.h:14790:32, winnt.h:14790:32 */
#[repr(C)] pub struct SYSTEM_POWER_CAPABILITIES { PowerButtonPresent: ::winnt::BOOLEAN, SleepButtonPresent: ::winnt::BOOLEAN, LidPresent: ::winnt::BOOLEAN, SystemS1: ::winnt::BOOLEAN, SystemS2: ::winnt::BOOLEAN, SystemS3: ::winnt::BOOLEAN, SystemS4: ::winnt::BOOLEAN, SystemS5: ::winnt::BOOLEAN, HiberFilePresent: ::winnt::BOOLEAN, FullWake: ::winnt::BOOLEAN, VideoDimPresent: ::winnt::BOOLEAN, ApmPresent: ::winnt::BOOLEAN, UpsPresent: ::winnt::BOOLEAN, ThermalControl: ::winnt::BOOLEAN, ProcessorThrottle: ::winnt::BOOLEAN, ProcessorMinThrottle: ::minwindef::BYTE, ProcessorMaxThrottle: ::minwindef::BYTE, FastSystemS4: ::winnt::BOOLEAN, Hiberboot: ::winnt::BOOLEAN, WakeAlarmPresent: ::winnt::BOOLEAN, AoAc: ::winnt::BOOLEAN, DiskSpinDown: ::winnt::BOOLEAN, spare3: *mut [::minwindef::BYTE; 8], SystemBatteriesPresent: ::winnt::BOOLEAN, BatteriesAreShortTerm: ::winnt::BOOLEAN, BatteryScale: *mut [::winnt::BATTERY_REPORTING_SCALE; 3], AcOnLineWake: ::winnt::SYSTEM_POWER_STATE, SoftLidWake: ::winnt::SYSTEM_POWER_STATE, RtcWake: ::winnt::SYSTEM_POWER_STATE, MinDeviceWakeState: ::winnt::SYSTEM_POWER_STATE, DefaultLowLatencyWake: ::winnt::SYSTEM_POWER_STATE } /* winnt.h:14793:9, winnt.h:14793:9, winnt.h:14793:9 */
pub type PSYSTEM_POWER_CAPABILITIES = *mut ::winnt::SYSTEM_POWER_CAPABILITIES; /* winnt.h:14840:31, winnt.h:14840:31, winnt.h:14840:31 */
#[repr(C)] pub struct SYSTEM_BATTERY_STATE { AcOnLine: ::winnt::BOOLEAN, BatteryPresent: ::winnt::BOOLEAN, Charging: ::winnt::BOOLEAN, Discharging: ::winnt::BOOLEAN, Spare1: *mut [::winnt::BOOLEAN; 3], Tag: ::minwindef::BYTE, MaxCapacity: ::minwindef::DWORD, RemainingCapacity: ::minwindef::DWORD, Rate: ::minwindef::DWORD, EstimatedTime: ::minwindef::DWORD, DefaultAlert1: ::minwindef::DWORD, DefaultAlert2: ::minwindef::DWORD } /* winnt.h:14842:9, winnt.h:14842:9, winnt.h:14842:9 */
pub type PSYSTEM_BATTERY_STATE = *mut ::winnt::SYSTEM_BATTERY_STATE; /* winnt.h:14858:26, winnt.h:14858:26, winnt.h:14858:26 */
#[repr(C)] pub struct IMAGE_DOS_HEADER { e_magic: ::minwindef::WORD, e_cblp: ::minwindef::WORD, e_cp: ::minwindef::WORD, e_crlc: ::minwindef::WORD, e_cparhdr: ::minwindef::WORD, e_minalloc: ::minwindef::WORD, e_maxalloc: ::minwindef::WORD, e_ss: ::minwindef::WORD, e_sp: ::minwindef::WORD, e_csum: ::minwindef::WORD, e_ip: ::minwindef::WORD, e_cs: ::minwindef::WORD, e_lfarlc: ::minwindef::WORD, e_ovno: ::minwindef::WORD, e_res: *mut [::minwindef::WORD; 4], e_oemid: ::minwindef::WORD, e_oeminfo: ::minwindef::WORD, e_res2: *mut [::minwindef::WORD; 10], e_lfanew: ::winnt::LONG } /* winnt.h:14889:16, winnt.h:14889:16, winnt.h:14889:16 */
pub type PIMAGE_DOS_HEADER = *mut ::winnt::IMAGE_DOS_HEADER; /* winnt.h:14909:24, winnt.h:14909:24, winnt.h:14909:24 */
#[repr(C)] pub struct IMAGE_OS2_HEADER { ne_magic: ::minwindef::WORD, ne_ver: ::winnt::CHAR, ne_rev: ::winnt::CHAR, ne_enttab: ::minwindef::WORD, ne_cbenttab: ::minwindef::WORD, ne_crc: ::winnt::LONG, ne_flags: ::minwindef::WORD, ne_autodata: ::minwindef::WORD, ne_heap: ::minwindef::WORD, ne_stack: ::minwindef::WORD, ne_csip: ::winnt::LONG, ne_sssp: ::winnt::LONG, ne_cseg: ::minwindef::WORD, ne_cmod: ::minwindef::WORD, ne_cbnrestab: ::minwindef::WORD, ne_segtab: ::minwindef::WORD, ne_rsrctab: ::minwindef::WORD, ne_restab: ::minwindef::WORD, ne_modtab: ::minwindef::WORD, ne_imptab: ::minwindef::WORD, ne_nrestab: ::winnt::LONG, ne_cmovent: ::minwindef::WORD, ne_align: ::minwindef::WORD, ne_cres: ::minwindef::WORD, ne_exetyp: ::minwindef::BYTE, ne_flagsothers: ::minwindef::BYTE, ne_pretthunks: ::minwindef::WORD, ne_psegrefbytes: ::minwindef::WORD, ne_swaparea: ::minwindef::WORD, ne_expver: ::minwindef::WORD } /* winnt.h:14911:16, winnt.h:14911:16, winnt.h:14911:16 */
pub type PIMAGE_OS2_HEADER = *mut ::winnt::IMAGE_OS2_HEADER; /* winnt.h:14942:24, winnt.h:14942:24, winnt.h:14942:24 */
#[repr(C)] pub struct IMAGE_VXD_HEADER { e32_magic: ::minwindef::WORD, e32_border: ::minwindef::BYTE, e32_worder: ::minwindef::BYTE, e32_level: ::minwindef::DWORD, e32_cpu: ::minwindef::WORD, e32_os: ::minwindef::WORD, e32_ver: ::minwindef::DWORD, e32_mflags: ::minwindef::DWORD, e32_mpages: ::minwindef::DWORD, e32_startobj: ::minwindef::DWORD, e32_eip: ::minwindef::DWORD, e32_stackobj: ::minwindef::DWORD, e32_esp: ::minwindef::DWORD, e32_pagesize: ::minwindef::DWORD, e32_lastpagesize: ::minwindef::DWORD, e32_fixupsize: ::minwindef::DWORD, e32_fixupsum: ::minwindef::DWORD, e32_ldrsize: ::minwindef::DWORD, e32_ldrsum: ::minwindef::DWORD, e32_objtab: ::minwindef::DWORD, e32_objcnt: ::minwindef::DWORD, e32_objmap: ::minwindef::DWORD, e32_itermap: ::minwindef::DWORD, e32_rsrctab: ::minwindef::DWORD, e32_rsrccnt: ::minwindef::DWORD, e32_restab: ::minwindef::DWORD, e32_enttab: ::minwindef::DWORD, e32_dirtab: ::minwindef::DWORD, e32_dircnt: ::minwindef::DWORD, e32_fpagetab: ::minwindef::DWORD, e32_frectab: ::minwindef::DWORD, e32_impmod: ::minwindef::DWORD, e32_impmodcnt: ::minwindef::DWORD, e32_impproc: ::minwindef::DWORD, e32_pagesum: ::minwindef::DWORD, e32_datapage: ::minwindef::DWORD, e32_preload: ::minwindef::DWORD, e32_nrestab: ::minwindef::DWORD, e32_cbnrestab: ::minwindef::DWORD, e32_nressum: ::minwindef::DWORD, e32_autodata: ::minwindef::DWORD, e32_debuginfo: ::minwindef::DWORD, e32_debuglen: ::minwindef::DWORD, e32_instpreload: ::minwindef::DWORD, e32_instdemand: ::minwindef::DWORD, e32_heapsize: ::minwindef::DWORD, e32_res3: *mut [::minwindef::BYTE; 12], e32_winresoff: ::minwindef::DWORD, e32_winreslen: ::minwindef::DWORD, e32_devid: ::minwindef::WORD, e32_ddkver: ::minwindef::WORD } /* winnt.h:14944:16, winnt.h:14944:16, winnt.h:14944:16 */
pub type PIMAGE_VXD_HEADER = *mut ::winnt::IMAGE_VXD_HEADER; /* winnt.h:14996:24, winnt.h:14996:24, winnt.h:14996:24 */
#[repr(C)] pub struct IMAGE_FILE_HEADER { Machine: ::minwindef::WORD, NumberOfSections: ::minwindef::WORD, TimeDateStamp: ::minwindef::DWORD, PointerToSymbolTable: ::minwindef::DWORD, NumberOfSymbols: ::minwindef::DWORD, SizeOfOptionalHeader: ::minwindef::WORD, Characteristics: ::minwindef::WORD } /* winnt.h:15006:16, winnt.h:15006:16, winnt.h:15006:16 */
pub type PIMAGE_FILE_HEADER = *mut ::winnt::IMAGE_FILE_HEADER; /* winnt.h:15014:23, winnt.h:15014:23, winnt.h:15014:23 */
#[repr(C)] pub struct IMAGE_DATA_DIRECTORY { VirtualAddress: ::minwindef::DWORD, Size: ::minwindef::DWORD } /* winnt.h:15069:16, winnt.h:15069:16, winnt.h:15069:16 */
pub type PIMAGE_DATA_DIRECTORY = *mut ::winnt::IMAGE_DATA_DIRECTORY; /* winnt.h:15072:26, winnt.h:15072:26, winnt.h:15072:26 */
#[repr(C)] pub struct IMAGE_OPTIONAL_HEADER32 { Magic: ::minwindef::WORD, MajorLinkerVersion: ::minwindef::BYTE, MinorLinkerVersion: ::minwindef::BYTE, SizeOfCode: ::minwindef::DWORD, SizeOfInitializedData: ::minwindef::DWORD, SizeOfUninitializedData: ::minwindef::DWORD, AddressOfEntryPoint: ::minwindef::DWORD, BaseOfCode: ::minwindef::DWORD, BaseOfData: ::minwindef::DWORD, ImageBase: ::minwindef::DWORD, SectionAlignment: ::minwindef::DWORD, FileAlignment: ::minwindef::DWORD, MajorOperatingSystemVersion: ::minwindef::WORD, MinorOperatingSystemVersion: ::minwindef::WORD, MajorImageVersion: ::minwindef::WORD, MinorImageVersion: ::minwindef::WORD, MajorSubsystemVersion: ::minwindef::WORD, MinorSubsystemVersion: ::minwindef::WORD, Win32VersionValue: ::minwindef::DWORD, SizeOfImage: ::minwindef::DWORD, SizeOfHeaders: ::minwindef::DWORD, CheckSum: ::minwindef::DWORD, Subsystem: ::minwindef::WORD, DllCharacteristics: ::minwindef::WORD, SizeOfStackReserve: ::minwindef::DWORD, SizeOfStackCommit: ::minwindef::DWORD, SizeOfHeapReserve: ::minwindef::DWORD, SizeOfHeapCommit: ::minwindef::DWORD, LoaderFlags: ::minwindef::DWORD, NumberOfRvaAndSizes: ::minwindef::DWORD, DataDirectory: *mut [::winnt::IMAGE_DATA_DIRECTORY; 16] } /* winnt.h:15080:16, winnt.h:15080:16, winnt.h:15080:16 */
pub type PIMAGE_OPTIONAL_HEADER32 = *mut ::winnt::IMAGE_OPTIONAL_HEADER32; /* winnt.h:15121:29, winnt.h:15121:29, winnt.h:15121:29 */
#[repr(C)] pub struct IMAGE_ROM_OPTIONAL_HEADER { Magic: ::minwindef::WORD, MajorLinkerVersion: ::minwindef::BYTE, MinorLinkerVersion: ::minwindef::BYTE, SizeOfCode: ::minwindef::DWORD, SizeOfInitializedData: ::minwindef::DWORD, SizeOfUninitializedData: ::minwindef::DWORD, AddressOfEntryPoint: ::minwindef::DWORD, BaseOfCode: ::minwindef::DWORD, BaseOfData: ::minwindef::DWORD, BaseOfBss: ::minwindef::DWORD, GprMask: ::minwindef::DWORD, CprMask: *mut [::minwindef::DWORD; 4], GpValue: ::minwindef::DWORD } /* winnt.h:15123:16, winnt.h:15123:16, winnt.h:15123:16 */
pub type PIMAGE_ROM_OPTIONAL_HEADER = *mut ::winnt::IMAGE_ROM_OPTIONAL_HEADER; /* winnt.h:15137:31, winnt.h:15137:31, winnt.h:15137:31 */
#[repr(C)] pub struct IMAGE_OPTIONAL_HEADER64 { Magic: ::minwindef::WORD, MajorLinkerVersion: ::minwindef::BYTE, MinorLinkerVersion: ::minwindef::BYTE, SizeOfCode: ::minwindef::DWORD, SizeOfInitializedData: ::minwindef::DWORD, SizeOfUninitializedData: ::minwindef::DWORD, AddressOfEntryPoint: ::minwindef::DWORD, BaseOfCode: ::minwindef::DWORD, ImageBase: ::winnt::ULONGLONG, SectionAlignment: ::minwindef::DWORD, FileAlignment: ::minwindef::DWORD, MajorOperatingSystemVersion: ::minwindef::WORD, MinorOperatingSystemVersion: ::minwindef::WORD, MajorImageVersion: ::minwindef::WORD, MinorImageVersion: ::minwindef::WORD, MajorSubsystemVersion: ::minwindef::WORD, MinorSubsystemVersion: ::minwindef::WORD, Win32VersionValue: ::minwindef::DWORD, SizeOfImage: ::minwindef::DWORD, SizeOfHeaders: ::minwindef::DWORD, CheckSum: ::minwindef::DWORD, Subsystem: ::minwindef::WORD, DllCharacteristics: ::minwindef::WORD, SizeOfStackReserve: ::winnt::ULONGLONG, SizeOfStackCommit: ::winnt::ULONGLONG, SizeOfHeapReserve: ::winnt::ULONGLONG, SizeOfHeapCommit: ::winnt::ULONGLONG, LoaderFlags: ::minwindef::DWORD, NumberOfRvaAndSizes: ::minwindef::DWORD, DataDirectory: *mut [::winnt::IMAGE_DATA_DIRECTORY; 16] } /* winnt.h:15139:16, winnt.h:15139:16, winnt.h:15139:16 */
pub type PIMAGE_OPTIONAL_HEADER64 = *mut ::winnt::IMAGE_OPTIONAL_HEADER64; /* winnt.h:15170:29, winnt.h:15170:29, winnt.h:15170:29 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type IMAGE_OPTIONAL_HEADER = ::winnt::IMAGE_OPTIONAL_HEADER32; /* winnt.h:15181:45, winnt.h:15181:45 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PIMAGE_OPTIONAL_HEADER = ::winnt::PIMAGE_OPTIONAL_HEADER32; /* winnt.h:15182:45, winnt.h:15182:45 */
#[repr(C)] pub struct IMAGE_NT_HEADERS64 { Signature: ::minwindef::DWORD, FileHeader: ::winnt::IMAGE_FILE_HEADER, OptionalHeader: ::winnt::IMAGE_OPTIONAL_HEADER64 } /* winnt.h:15186:16, winnt.h:15186:16, winnt.h:15186:16 */
pub type PIMAGE_NT_HEADERS64 = *mut ::winnt::IMAGE_NT_HEADERS64; /* winnt.h:15190:24, winnt.h:15190:24, winnt.h:15190:24 */
#[repr(C)] pub struct IMAGE_NT_HEADERS32 { Signature: ::minwindef::DWORD, FileHeader: ::winnt::IMAGE_FILE_HEADER, OptionalHeader: ::winnt::IMAGE_OPTIONAL_HEADER32 } /* winnt.h:15192:16, winnt.h:15192:16, winnt.h:15192:16 */
pub type PIMAGE_NT_HEADERS32 = *mut ::winnt::IMAGE_NT_HEADERS32; /* winnt.h:15196:24, winnt.h:15196:24, winnt.h:15196:24 */
#[repr(C)] pub struct IMAGE_ROM_HEADERS { FileHeader: ::winnt::IMAGE_FILE_HEADER, OptionalHeader: ::winnt::IMAGE_ROM_OPTIONAL_HEADER } /* winnt.h:15198:16, winnt.h:15198:16, winnt.h:15198:16 */
pub type PIMAGE_ROM_HEADERS = *mut ::winnt::IMAGE_ROM_HEADERS; /* winnt.h:15201:23, winnt.h:15201:23, winnt.h:15201:23 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type IMAGE_NT_HEADERS = ::winnt::IMAGE_NT_HEADERS32; /* winnt.h:15207:45, winnt.h:15207:45 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PIMAGE_NT_HEADERS = ::winnt::PIMAGE_NT_HEADERS32; /* winnt.h:15208:45, winnt.h:15208:45 */
#[repr(C)] pub struct ANON_OBJECT_HEADER { Sig1: ::minwindef::WORD, Sig2: ::minwindef::WORD, Version: ::minwindef::WORD, Machine: ::minwindef::WORD, TimeDateStamp: ::minwindef::DWORD, ClassID: ::guiddef::CLSID, SizeOfData: ::minwindef::DWORD } /* winnt.h:15277:16, winnt.h:15277:16, winnt.h:15277:16 */
#[repr(C)] pub struct ANON_OBJECT_HEADER_V2 { Sig1: ::minwindef::WORD, Sig2: ::minwindef::WORD, Version: ::minwindef::WORD, Machine: ::minwindef::WORD, TimeDateStamp: ::minwindef::DWORD, ClassID: ::guiddef::CLSID, SizeOfData: ::minwindef::DWORD, Flags: ::minwindef::DWORD, MetaDataSize: ::minwindef::DWORD, MetaDataOffset: ::minwindef::DWORD } /* winnt.h:15287:16, winnt.h:15287:16, winnt.h:15287:16 */
#[repr(C)] pub struct ANON_OBJECT_HEADER_BIGOBJ { Sig1: ::minwindef::WORD, Sig2: ::minwindef::WORD, Version: ::minwindef::WORD, Machine: ::minwindef::WORD, TimeDateStamp: ::minwindef::DWORD, ClassID: ::guiddef::CLSID, SizeOfData: ::minwindef::DWORD, Flags: ::minwindef::DWORD, MetaDataSize: ::minwindef::DWORD, MetaDataOffset: ::minwindef::DWORD, NumberOfSections: ::minwindef::DWORD, PointerToSymbolTable: ::minwindef::DWORD, NumberOfSymbols: ::minwindef::DWORD } /* winnt.h:15300:16, winnt.h:15300:16, winnt.h:15300:16 */
#[repr(C)] pub struct IMAGE_SECTION_HEADER { Name: *mut [::minwindef::BYTE; 8], Misc: ::winnt::IMAGE_SECTION_HEADER_Child_2, VirtualAddress: ::minwindef::DWORD, SizeOfRawData: ::minwindef::DWORD, PointerToRawData: ::minwindef::DWORD, PointerToRelocations: ::minwindef::DWORD, PointerToLinenumbers: ::minwindef::DWORD, NumberOfRelocations: ::minwindef::WORD, NumberOfLinenumbers: ::minwindef::WORD, Characteristics: ::minwindef::DWORD } /* winnt.h:15325:16, winnt.h:15325:16, winnt.h:15325:16 */
#[repr(C)] pub /*union*/ struct IMAGE_SECTION_HEADER_Child_2; /* STUB! */ /* winnt.h:15327:5, winnt.h:15327:5, winnt.h:15327:5 */
pub type PIMAGE_SECTION_HEADER = *mut ::winnt::IMAGE_SECTION_HEADER; /* winnt.h:15339:26, winnt.h:15339:26, winnt.h:15339:26 */
#[repr(C)] pub struct IMAGE_SYMBOL { N: ::winnt::IMAGE_SYMBOL_Child_1, Value: ::minwindef::DWORD, SectionNumber: ::winnt::SHORT, Type: ::minwindef::WORD, StorageClass: ::minwindef::BYTE, NumberOfAuxSymbols: ::minwindef::BYTE } /* winnt.h:15412:16, winnt.h:15412:16, winnt.h:15412:16 */
#[repr(C)] pub /*union*/ struct IMAGE_SYMBOL_Child_1; /* STUB! */ /* winnt.h:15413:5, winnt.h:15413:5, winnt.h:15413:5 */
pub type PIMAGE_SYMBOL = *mut ::winnt::IMAGE_SYMBOL; /* winnt.h:15427:33, winnt.h:15427:33, winnt.h:15427:33 */
#[repr(C)] pub struct IMAGE_SYMBOL_EX { N: ::winnt::IMAGE_SYMBOL_EX_Child_1, Value: ::minwindef::DWORD, SectionNumber: ::winnt::LONG, Type: ::minwindef::WORD, StorageClass: ::minwindef::BYTE, NumberOfAuxSymbols: ::minwindef::BYTE } /* winnt.h:15431:16, winnt.h:15431:16, winnt.h:15431:16 */
#[repr(C)] pub /*union*/ struct IMAGE_SYMBOL_EX_Child_1; /* STUB! */ /* winnt.h:15432:5, winnt.h:15432:5, winnt.h:15432:5 */
pub type PIMAGE_SYMBOL_EX = *mut ::winnt::IMAGE_SYMBOL_EX; /* winnt.h:15446:36, winnt.h:15446:36, winnt.h:15446:36 */
#[repr(C)] pub struct IMAGE_AUX_SYMBOL_TOKEN_DEF { bAuxType: ::minwindef::BYTE, bReserved: ::minwindef::BYTE, SymbolTableIndex: ::minwindef::DWORD, rgbReserved: *mut [::minwindef::BYTE; 12] } /* winnt.h:15570:16, winnt.h:15570:16, winnt.h:15570:16 */
pub type PIMAGE_AUX_SYMBOL_TOKEN_DEF = *mut ::winnt::IMAGE_AUX_SYMBOL_TOKEN_DEF; /* winnt.h:15577:47, winnt.h:15577:47, winnt.h:15577:47 */
#[repr(C)] pub /*union*/ struct IMAGE_AUX_SYMBOL; /* STUB! */ /* winnt.h:15585:15, winnt.h:15585:15, winnt.h:15585:15 */
pub type PIMAGE_AUX_SYMBOL = *mut ::winnt::IMAGE_AUX_SYMBOL; /* winnt.h:15625:37, winnt.h:15625:37, winnt.h:15625:37 */
#[repr(C)] pub /*union*/ struct IMAGE_AUX_SYMBOL_EX; /* STUB! */ /* winnt.h:15627:15, winnt.h:15627:15, winnt.h:15627:15 */
pub type PIMAGE_AUX_SYMBOL_EX = *mut ::winnt::IMAGE_AUX_SYMBOL_EX; /* winnt.h:15656:40, winnt.h:15656:40, winnt.h:15656:40 */
#[repr(C)] pub enum IMAGE_AUX_SYMBOL_TYPE {IMAGE_AUX_SYMBOL_TYPE_TOKEN_DEF = 1, __SeeGhIssue10292} pub use self::IMAGE_AUX_SYMBOL_TYPE::{IMAGE_AUX_SYMBOL_TYPE_TOKEN_DEF}; /* winnt.h:15658:14, winnt.h:15658:14, winnt.h:15658:14 */
#[repr(C)] pub struct IMAGE_RELOCATION { u: ::winnt::IMAGE_RELOCATION_Child_1, SymbolTableIndex: ::minwindef::DWORD, Type: ::minwindef::WORD } /* winnt.h:15683:16, winnt.h:15683:16, winnt.h:15683:16 */
#[repr(C)] pub /*union*/ struct IMAGE_RELOCATION_Child_1; /* STUB! */ /* winnt.h:15684:5, winnt.h:15684:5, winnt.h:15684:5 */
pub type PIMAGE_RELOCATION = *mut ::winnt::IMAGE_RELOCATION; /* winnt.h:15691:37, winnt.h:15691:37, winnt.h:15691:37 */
#[repr(C)] pub struct IMAGE_LINENUMBER { Type: ::winnt::IMAGE_LINENUMBER_Child_1, Linenumber: ::minwindef::WORD } /* winnt.h:16064:16, winnt.h:16064:16, winnt.h:16064:16 */
#[repr(C)] pub /*union*/ struct IMAGE_LINENUMBER_Child_1; /* STUB! */ /* winnt.h:16065:5, winnt.h:16065:5, winnt.h:16065:5 */
pub type PIMAGE_LINENUMBER = *mut ::winnt::IMAGE_LINENUMBER; /* winnt.h:16071:37, winnt.h:16071:37, winnt.h:16071:37 */
#[repr(C)] pub struct IMAGE_BASE_RELOCATION { VirtualAddress: ::minwindef::DWORD, SizeOfBlock: ::minwindef::DWORD } /* winnt.h:16081:16, winnt.h:16081:16, winnt.h:16081:16 */
pub type PIMAGE_BASE_RELOCATION = *mut ::winnt::IMAGE_BASE_RELOCATION; /* winnt.h:16086:43, winnt.h:16086:43, winnt.h:16086:43 */
#[repr(C)] pub struct IMAGE_ARCHIVE_MEMBER_HEADER { Name: *mut [::minwindef::BYTE; 16], Date: *mut [::minwindef::BYTE; 12], UserID: *mut [::minwindef::BYTE; 6], GroupID: *mut [::minwindef::BYTE; 6], Mode: *mut [::minwindef::BYTE; 8], Size: *mut [::minwindef::BYTE; 10], EndHeader: *mut [::minwindef::BYTE; 2] } /* winnt.h:16128:16, winnt.h:16128:16, winnt.h:16128:16 */
pub type PIMAGE_ARCHIVE_MEMBER_HEADER = *mut ::winnt::IMAGE_ARCHIVE_MEMBER_HEADER; /* winnt.h:16136:33, winnt.h:16136:33, winnt.h:16136:33 */
#[repr(C)] pub struct IMAGE_EXPORT_DIRECTORY { Characteristics: ::minwindef::DWORD, TimeDateStamp: ::minwindef::DWORD, MajorVersion: ::minwindef::WORD, MinorVersion: ::minwindef::WORD, Name: ::minwindef::DWORD, Base: ::minwindef::DWORD, NumberOfFunctions: ::minwindef::DWORD, NumberOfNames: ::minwindef::DWORD, AddressOfFunctions: ::minwindef::DWORD, AddressOfNames: ::minwindef::DWORD, AddressOfNameOrdinals: ::minwindef::DWORD } /* winnt.h:16148:16, winnt.h:16148:16, winnt.h:16148:16 */
pub type PIMAGE_EXPORT_DIRECTORY = *mut ::winnt::IMAGE_EXPORT_DIRECTORY; /* winnt.h:16160:28, winnt.h:16160:28, winnt.h:16160:28 */
#[repr(C)] pub struct IMAGE_IMPORT_BY_NAME { Hint: ::minwindef::WORD, Name: *mut [::winnt::CHAR; 1] } /* winnt.h:16166:16, winnt.h:16166:16, winnt.h:16166:16 */
pub type PIMAGE_IMPORT_BY_NAME = *mut ::winnt::IMAGE_IMPORT_BY_NAME; /* winnt.h:16169:26, winnt.h:16169:26, winnt.h:16169:26 */
#[repr(C)] pub struct IMAGE_THUNK_DATA64 { u1: ::winnt::IMAGE_THUNK_DATA64_Child_1 } /* winnt.h:16173:16, winnt.h:16173:16, winnt.h:16173:16 */
#[repr(C)] pub /*union*/ struct IMAGE_THUNK_DATA64_Child_1; /* STUB! */ /* winnt.h:16174:5, winnt.h:16174:5, winnt.h:16174:5 */
pub type PIMAGE_THUNK_DATA64 = *mut ::winnt::IMAGE_THUNK_DATA64; /* winnt.h:16181:30, winnt.h:16181:30, winnt.h:16181:30 */
#[repr(C)] pub struct IMAGE_THUNK_DATA32 { u1: ::winnt::IMAGE_THUNK_DATA32_Child_1 } /* winnt.h:16185:16, winnt.h:16185:16, winnt.h:16185:16 */
#[repr(C)] pub /*union*/ struct IMAGE_THUNK_DATA32_Child_1; /* STUB! */ /* winnt.h:16186:5, winnt.h:16186:5, winnt.h:16186:5 */
pub type PIMAGE_THUNK_DATA32 = *mut ::winnt::IMAGE_THUNK_DATA32; /* winnt.h:16193:30, winnt.h:16193:30, winnt.h:16193:30 */
pub type PIMAGE_TLS_CALLBACK = extern "system" fn(*mut ::libc::c_void, ::libc::c_ulong, *mut ::libc::c_void); /* winnt.h:16207:9, winnt.h:16207:9, winnt.h:16207:9 */
#[repr(C)] pub struct IMAGE_TLS_DIRECTORY64 { StartAddressOfRawData: ::winnt::ULONGLONG, EndAddressOfRawData: ::winnt::ULONGLONG, AddressOfIndex: ::winnt::ULONGLONG, AddressOfCallBacks: ::winnt::ULONGLONG, SizeOfZeroFill: ::minwindef::DWORD, u: ::winnt::IMAGE_TLS_DIRECTORY64_Child_6 } /* winnt.h:16213:16, winnt.h:16213:16, winnt.h:16213:16 */
#[repr(C)] pub /*union*/ struct IMAGE_TLS_DIRECTORY64_Child_6; /* STUB! */ /* winnt.h:16219:5, winnt.h:16219:5, winnt.h:16219:5 */
pub type PIMAGE_TLS_DIRECTORY64 = *mut ::winnt::IMAGE_TLS_DIRECTORY64; /* winnt.h:16230:33, winnt.h:16230:33, winnt.h:16230:33 */
#[repr(C)] pub struct IMAGE_TLS_DIRECTORY32 { StartAddressOfRawData: ::minwindef::DWORD, EndAddressOfRawData: ::minwindef::DWORD, AddressOfIndex: ::minwindef::DWORD, AddressOfCallBacks: ::minwindef::DWORD, SizeOfZeroFill: ::minwindef::DWORD, u: ::winnt::IMAGE_TLS_DIRECTORY32_Child_6 } /* winnt.h:16232:16, winnt.h:16232:16, winnt.h:16232:16 */
#[repr(C)] pub /*union*/ struct IMAGE_TLS_DIRECTORY32_Child_6; /* STUB! */ /* winnt.h:16238:5, winnt.h:16238:5, winnt.h:16238:5 */
pub type PIMAGE_TLS_DIRECTORY32 = *mut ::winnt::IMAGE_TLS_DIRECTORY32; /* winnt.h:16248:33, winnt.h:16248:33, winnt.h:16248:33 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type IMAGE_THUNK_DATA = ::winnt::IMAGE_THUNK_DATA32; /* winnt.h:16261:41, winnt.h:16261:41 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PIMAGE_THUNK_DATA = ::winnt::PIMAGE_THUNK_DATA32; /* winnt.h:16262:41, winnt.h:16262:41 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type IMAGE_TLS_DIRECTORY = ::winnt::IMAGE_TLS_DIRECTORY32; /* winnt.h:16264:41, winnt.h:16264:41 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PIMAGE_TLS_DIRECTORY = ::winnt::PIMAGE_TLS_DIRECTORY32; /* winnt.h:16265:41, winnt.h:16265:41 */
#[repr(C)] pub struct IMAGE_IMPORT_DESCRIPTOR { u: ::winnt::IMAGE_IMPORT_DESCRIPTOR_Child_1, TimeDateStamp: ::minwindef::DWORD, ForwarderChain: ::minwindef::DWORD, Name: ::minwindef::DWORD, FirstThunk: ::minwindef::DWORD } /* winnt.h:16268:16, winnt.h:16268:16, winnt.h:16268:16 */
#[repr(C)] pub /*union*/ struct IMAGE_IMPORT_DESCRIPTOR_Child_1; /* STUB! */ /* winnt.h:16269:5, winnt.h:16269:5, winnt.h:16269:5 */
pub type PIMAGE_IMPORT_DESCRIPTOR = *mut ::winnt::IMAGE_IMPORT_DESCRIPTOR; /* winnt.h:16282:44, winnt.h:16282:44, winnt.h:16282:44 */
#[repr(C)] pub struct IMAGE_BOUND_IMPORT_DESCRIPTOR { TimeDateStamp: ::minwindef::DWORD, OffsetModuleName: ::minwindef::WORD, NumberOfModuleForwarderRefs: ::minwindef::WORD } /* winnt.h:16288:16, winnt.h:16288:16, winnt.h:16288:16 */
pub type PIMAGE_BOUND_IMPORT_DESCRIPTOR = *mut ::winnt::IMAGE_BOUND_IMPORT_DESCRIPTOR; /* winnt.h:16293:36, winnt.h:16293:36, winnt.h:16293:36 */
#[repr(C)] pub struct IMAGE_BOUND_FORWARDER_REF { TimeDateStamp: ::minwindef::DWORD, OffsetModuleName: ::minwindef::WORD, Reserved: ::minwindef::WORD } /* winnt.h:16295:16, winnt.h:16295:16, winnt.h:16295:16 */
pub type PIMAGE_BOUND_FORWARDER_REF = *mut ::winnt::IMAGE_BOUND_FORWARDER_REF; /* winnt.h:16299:31, winnt.h:16299:31, winnt.h:16299:31 */
#[repr(C)] pub struct IMAGE_DELAYLOAD_DESCRIPTOR { Attributes: ::winnt::IMAGE_DELAYLOAD_DESCRIPTOR_Child_1, DllNameRVA: ::minwindef::DWORD, ModuleHandleRVA: ::minwindef::DWORD, ImportAddressTableRVA: ::minwindef::DWORD, ImportNameTableRVA: ::minwindef::DWORD, BoundImportAddressTableRVA: ::minwindef::DWORD, UnloadInformationTableRVA: ::minwindef::DWORD, TimeDateStamp: ::minwindef::DWORD } /* winnt.h:16301:16, winnt.h:16301:16, winnt.h:16301:16 */
#[repr(C)] pub /*union*/ struct IMAGE_DELAYLOAD_DESCRIPTOR_Child_1; /* STUB! */ /* winnt.h:16302:5, winnt.h:16302:5, winnt.h:16302:5 */
pub type PIMAGE_DELAYLOAD_DESCRIPTOR = *mut ::winnt::IMAGE_DELAYLOAD_DESCRIPTOR; /* winnt.h:16319:32, winnt.h:16319:32, winnt.h:16319:32 */
pub type PCIMAGE_DELAYLOAD_DESCRIPTOR = *const ::winnt::IMAGE_DELAYLOAD_DESCRIPTOR; /* winnt.h:16321:43, winnt.h:16321:43, winnt.h:16321:43 */
#[repr(C)] pub struct IMAGE_RESOURCE_DIRECTORY { Characteristics: ::minwindef::DWORD, TimeDateStamp: ::minwindef::DWORD, MajorVersion: ::minwindef::WORD, MinorVersion: ::minwindef::WORD, NumberOfNamedEntries: ::minwindef::WORD, NumberOfIdEntries: ::minwindef::WORD } /* winnt.h:16341:16, winnt.h:16341:16, winnt.h:16341:16 */
pub type PIMAGE_RESOURCE_DIRECTORY = *mut ::winnt::IMAGE_RESOURCE_DIRECTORY; /* winnt.h:16349:30, winnt.h:16349:30, winnt.h:16349:30 */
#[repr(C)] pub struct IMAGE_RESOURCE_DIRECTORY_ENTRY { u: ::winnt::IMAGE_RESOURCE_DIRECTORY_ENTRY_Child_1, u2: ::winnt::IMAGE_RESOURCE_DIRECTORY_ENTRY_Child_3 } /* winnt.h:16368:16, winnt.h:16368:16, winnt.h:16368:16 */
#[repr(C)] pub /*union*/ struct IMAGE_RESOURCE_DIRECTORY_ENTRY_Child_1; /* STUB! */ /* winnt.h:16369:5, winnt.h:16369:5, winnt.h:16369:5 */
#[repr(C)] pub /*union*/ struct IMAGE_RESOURCE_DIRECTORY_ENTRY_Child_3; /* STUB! */ /* winnt.h:16377:5, winnt.h:16377:5, winnt.h:16377:5 */
pub type PIMAGE_RESOURCE_DIRECTORY_ENTRY = *mut ::winnt::IMAGE_RESOURCE_DIRECTORY_ENTRY; /* winnt.h:16384:36, winnt.h:16384:36, winnt.h:16384:36 */
#[repr(C)] pub struct IMAGE_RESOURCE_DIRECTORY_STRING { Length: ::minwindef::WORD, NameString: *mut [::winnt::CHAR; 1] } /* winnt.h:16395:16, winnt.h:16395:16, winnt.h:16395:16 */
pub type PIMAGE_RESOURCE_DIRECTORY_STRING = *mut ::winnt::IMAGE_RESOURCE_DIRECTORY_STRING; /* winnt.h:16398:37, winnt.h:16398:37, winnt.h:16398:37 */
#[repr(C)] pub struct IMAGE_RESOURCE_DIR_STRING_U { Length: ::minwindef::WORD, NameString: *mut [::winnt::WCHAR; 1] } /* winnt.h:16401:16, winnt.h:16401:16, winnt.h:16401:16 */
pub type PIMAGE_RESOURCE_DIR_STRING_U = *mut ::winnt::IMAGE_RESOURCE_DIR_STRING_U; /* winnt.h:16404:33, winnt.h:16404:33, winnt.h:16404:33 */
#[repr(C)] pub struct IMAGE_RESOURCE_DATA_ENTRY { OffsetToData: ::minwindef::DWORD, Size: ::minwindef::DWORD, CodePage: ::minwindef::DWORD, Reserved: ::minwindef::DWORD } /* winnt.h:16416:16, winnt.h:16416:16, winnt.h:16416:16 */
pub type PIMAGE_RESOURCE_DATA_ENTRY = *mut ::winnt::IMAGE_RESOURCE_DATA_ENTRY; /* winnt.h:16421:31, winnt.h:16421:31, winnt.h:16421:31 */
#[repr(C)] pub struct IMAGE_LOAD_CONFIG_DIRECTORY32 { Size: ::minwindef::DWORD, TimeDateStamp: ::minwindef::DWORD, MajorVersion: ::minwindef::WORD, MinorVersion: ::minwindef::WORD, GlobalFlagsClear: ::minwindef::DWORD, GlobalFlagsSet: ::minwindef::DWORD, CriticalSectionDefaultTimeout: ::minwindef::DWORD, DeCommitFreeBlockThreshold: ::minwindef::DWORD, DeCommitTotalFreeThreshold: ::minwindef::DWORD, LockPrefixTable: ::minwindef::DWORD, MaximumAllocationSize: ::minwindef::DWORD, VirtualMemoryThreshold: ::minwindef::DWORD, ProcessHeapFlags: ::minwindef::DWORD, ProcessAffinityMask: ::minwindef::DWORD, CSDVersion: ::minwindef::WORD, Reserved1: ::minwindef::WORD, EditList: ::minwindef::DWORD, SecurityCookie: ::minwindef::DWORD, SEHandlerTable: ::minwindef::DWORD, SEHandlerCount: ::minwindef::DWORD, GuardCFCheckFunctionPointer: ::minwindef::DWORD, Reserved2: ::minwindef::DWORD, GuardCFFunctionTable: ::minwindef::DWORD, GuardCFFunctionCount: ::minwindef::DWORD, GuardFlags: ::minwindef::DWORD } /* winnt.h:16427:16, winnt.h:16427:16, winnt.h:16427:16 */
pub type PIMAGE_LOAD_CONFIG_DIRECTORY32 = *mut ::winnt::IMAGE_LOAD_CONFIG_DIRECTORY32; /* winnt.h:16453:35, winnt.h:16453:35, winnt.h:16453:35 */
#[repr(C)] pub struct IMAGE_LOAD_CONFIG_DIRECTORY64 { Size: ::minwindef::DWORD, TimeDateStamp: ::minwindef::DWORD, MajorVersion: ::minwindef::WORD, MinorVersion: ::minwindef::WORD, GlobalFlagsClear: ::minwindef::DWORD, GlobalFlagsSet: ::minwindef::DWORD, CriticalSectionDefaultTimeout: ::minwindef::DWORD, DeCommitFreeBlockThreshold: ::winnt::ULONGLONG, DeCommitTotalFreeThreshold: ::winnt::ULONGLONG, LockPrefixTable: ::winnt::ULONGLONG, MaximumAllocationSize: ::winnt::ULONGLONG, VirtualMemoryThreshold: ::winnt::ULONGLONG, ProcessAffinityMask: ::winnt::ULONGLONG, ProcessHeapFlags: ::minwindef::DWORD, CSDVersion: ::minwindef::WORD, Reserved1: ::minwindef::WORD, EditList: ::winnt::ULONGLONG, SecurityCookie: ::winnt::ULONGLONG, SEHandlerTable: ::winnt::ULONGLONG, SEHandlerCount: ::winnt::ULONGLONG, GuardCFCheckFunctionPointer: ::winnt::ULONGLONG, Reserved2: ::winnt::ULONGLONG, GuardCFFunctionTable: ::winnt::ULONGLONG, GuardCFFunctionCount: ::winnt::ULONGLONG, GuardFlags: ::minwindef::DWORD } /* winnt.h:16455:16, winnt.h:16455:16, winnt.h:16455:16 */
pub type PIMAGE_LOAD_CONFIG_DIRECTORY64 = *mut ::winnt::IMAGE_LOAD_CONFIG_DIRECTORY64; /* winnt.h:16481:35, winnt.h:16481:35, winnt.h:16481:35 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type IMAGE_LOAD_CONFIG_DIRECTORY = ::winnt::IMAGE_LOAD_CONFIG_DIRECTORY32; /* winnt.h:16487:43, winnt.h:16487:43 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PIMAGE_LOAD_CONFIG_DIRECTORY = ::winnt::PIMAGE_LOAD_CONFIG_DIRECTORY32; /* winnt.h:16488:43, winnt.h:16488:43 */
#[repr(C)] pub struct IMAGE_CE_RUNTIME_FUNCTION_ENTRY { FuncStart: ::minwindef::DWORD, PrologLen: ::minwindef::DWORD, FuncLen: ::minwindef::DWORD, ThirtyTwoBit: ::minwindef::DWORD, ExceptionFlag: ::minwindef::DWORD } /* winnt.h:16509:16, winnt.h:16509:16, winnt.h:16509:16 */
pub type PIMAGE_CE_RUNTIME_FUNCTION_ENTRY = *mut ::winnt::IMAGE_CE_RUNTIME_FUNCTION_ENTRY; /* winnt.h:16515:38, winnt.h:16515:38, winnt.h:16515:38 */
#[cfg(any(target_arch="x86", target_arch="x86_64"))] #[repr(C)] pub struct IMAGE_ARM_RUNTIME_FUNCTION_ENTRY { BeginAddress: ::minwindef::DWORD, u: ::winnt::IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_Child_2 } /* winnt.h:16517:16, winnt.h:16517:16 */
#[cfg(any(target_arch="x86", target_arch="x86_64"))] #[repr(C)] pub /*union*/ struct IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_Child_2; /* STUB! */ /* winnt.h:16519:5, winnt.h:16519:5 */
#[cfg(any(target_arch="x86", target_arch="x86_64"))] pub type PIMAGE_ARM_RUNTIME_FUNCTION_ENTRY = *mut ::winnt::IMAGE_ARM_RUNTIME_FUNCTION_ENTRY; /* winnt.h:16533:39, winnt.h:16533:39 */
#[repr(C)] pub struct IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY { BeginAddress: ::winnt::ULONGLONG, EndAddress: ::winnt::ULONGLONG, ExceptionHandler: ::winnt::ULONGLONG, HandlerData: ::winnt::ULONGLONG, PrologEndAddress: ::winnt::ULONGLONG } /* winnt.h:16535:16, winnt.h:16535:16, winnt.h:16535:16 */
pub type PIMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY = *mut ::winnt::IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY; /* winnt.h:16541:42, winnt.h:16541:42, winnt.h:16541:42 */
#[repr(C)] pub struct IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY { BeginAddress: ::minwindef::DWORD, EndAddress: ::minwindef::DWORD, ExceptionHandler: ::minwindef::DWORD, HandlerData: ::minwindef::DWORD, PrologEndAddress: ::minwindef::DWORD } /* winnt.h:16543:16, winnt.h:16543:16, winnt.h:16543:16 */
pub type PIMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY = *mut ::winnt::IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY; /* winnt.h:16549:40, winnt.h:16549:40, winnt.h:16549:40 */
#[cfg(any(target_arch="x86", target_arch="arm"))] #[repr(C)] pub struct _IMAGE_RUNTIME_FUNCTION_ENTRY { BeginAddress: ::minwindef::DWORD, EndAddress: ::minwindef::DWORD, u: ::winnt::_IMAGE_RUNTIME_FUNCTION_ENTRY_Child_3 } /* winnt.h:16551:16, winnt.h:16551:16 */
#[cfg(any(target_arch="x86", target_arch="arm"))] #[repr(C)] pub /*union*/ struct _IMAGE_RUNTIME_FUNCTION_ENTRY_Child_3; /* STUB! */ /* winnt.h:16554:5, winnt.h:16554:5 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type _PIMAGE_RUNTIME_FUNCTION_ENTRY = *mut ::winnt::_IMAGE_RUNTIME_FUNCTION_ENTRY; /* winnt.h:16558:35, winnt.h:16558:35 */
pub type IMAGE_IA64_RUNTIME_FUNCTION_ENTRY = ::winnt::_IMAGE_RUNTIME_FUNCTION_ENTRY; /* winnt.h:16560:41, winnt.h:16560:41, winnt.h:16560:41 */
pub type PIMAGE_IA64_RUNTIME_FUNCTION_ENTRY = ::winnt::_PIMAGE_RUNTIME_FUNCTION_ENTRY; /* winnt.h:16561:40, winnt.h:16561:40, winnt.h:16561:40 */
#[cfg(any(target_arch="x86", target_arch="x86_64"))] pub type IMAGE_RUNTIME_FUNCTION_ENTRY = ::winnt::_IMAGE_RUNTIME_FUNCTION_ENTRY; /* winnt.h:16582:41, winnt.h:16582:41 */
#[cfg(any(target_arch="x86", target_arch="x86_64"))] pub type PIMAGE_RUNTIME_FUNCTION_ENTRY = ::winnt::_PIMAGE_RUNTIME_FUNCTION_ENTRY; /* winnt.h:16583:40, winnt.h:16583:40 */
#[repr(C)] pub struct IMAGE_DEBUG_DIRECTORY { Characteristics: ::minwindef::DWORD, TimeDateStamp: ::minwindef::DWORD, MajorVersion: ::minwindef::WORD, MinorVersion: ::minwindef::WORD, Type: ::minwindef::DWORD, SizeOfData: ::minwindef::DWORD, AddressOfRawData: ::minwindef::DWORD, PointerToRawData: ::minwindef::DWORD } /* winnt.h:16591:16, winnt.h:16591:16, winnt.h:16591:16 */
pub type PIMAGE_DEBUG_DIRECTORY = *mut ::winnt::IMAGE_DEBUG_DIRECTORY; /* winnt.h:16600:27, winnt.h:16600:27, winnt.h:16600:27 */
#[repr(C)] pub struct IMAGE_COFF_SYMBOLS_HEADER { NumberOfSymbols: ::minwindef::DWORD, LvaToFirstSymbol: ::minwindef::DWORD, NumberOfLinenumbers: ::minwindef::DWORD, LvaToFirstLinenumber: ::minwindef::DWORD, RvaToFirstByteOfCode: ::minwindef::DWORD, RvaToLastByteOfCode: ::minwindef::DWORD, RvaToFirstByteOfData: ::minwindef::DWORD, RvaToLastByteOfData: ::minwindef::DWORD } /* winnt.h:16616:16, winnt.h:16616:16, winnt.h:16616:16 */
pub type PIMAGE_COFF_SYMBOLS_HEADER = *mut ::winnt::IMAGE_COFF_SYMBOLS_HEADER; /* winnt.h:16625:31, winnt.h:16625:31, winnt.h:16625:31 */
#[repr(C)] pub struct FPO_DATA { ulOffStart: ::minwindef::DWORD, cbProcSize: ::minwindef::DWORD, cdwLocals: ::minwindef::DWORD, cdwParams: ::minwindef::WORD, cbProlog: ::minwindef::WORD, cbRegs: ::minwindef::WORD, fHasSEH: ::minwindef::WORD, fUseBP: ::minwindef::WORD, reserved: ::minwindef::WORD, cbFrame: ::minwindef::WORD } /* winnt.h:16632:16, winnt.h:16632:16, winnt.h:16632:16 */
pub type PFPO_DATA = *mut ::winnt::FPO_DATA; /* winnt.h:16643:14, winnt.h:16643:14, winnt.h:16643:14 */
#[repr(C)] pub struct IMAGE_DEBUG_MISC { DataType: ::minwindef::DWORD, Length: ::minwindef::DWORD, Unicode: ::winnt::BOOLEAN, Reserved: *mut [::minwindef::BYTE; 3], Data: *mut [::minwindef::BYTE; 1] } /* winnt.h:16649:16, winnt.h:16649:16, winnt.h:16649:16 */
pub type PIMAGE_DEBUG_MISC = *mut ::winnt::IMAGE_DEBUG_MISC; /* winnt.h:16656:22, winnt.h:16656:22, winnt.h:16656:22 */
#[repr(C)] pub struct IMAGE_FUNCTION_ENTRY { StartingAddress: ::minwindef::DWORD, EndingAddress: ::minwindef::DWORD, EndOfPrologue: ::minwindef::DWORD } /* winnt.h:16665:16, winnt.h:16665:16, winnt.h:16665:16 */
pub type PIMAGE_FUNCTION_ENTRY = *mut ::winnt::IMAGE_FUNCTION_ENTRY; /* winnt.h:16669:26, winnt.h:16669:26, winnt.h:16669:26 */
#[repr(C)] pub struct IMAGE_FUNCTION_ENTRY64 { StartingAddress: ::winnt::ULONGLONG, EndingAddress: ::winnt::ULONGLONG, u: ::winnt::IMAGE_FUNCTION_ENTRY64_Child_3 } /* winnt.h:16671:16, winnt.h:16671:16, winnt.h:16671:16 */
#[repr(C)] pub /*union*/ struct IMAGE_FUNCTION_ENTRY64_Child_3; /* STUB! */ /* winnt.h:16674:5, winnt.h:16674:5, winnt.h:16674:5 */
pub type PIMAGE_FUNCTION_ENTRY64 = *mut ::winnt::IMAGE_FUNCTION_ENTRY64; /* winnt.h:16678:28, winnt.h:16678:28, winnt.h:16678:28 */
#[repr(C)] pub struct IMAGE_SEPARATE_DEBUG_HEADER { Signature: ::minwindef::WORD, Flags: ::minwindef::WORD, Machine: ::minwindef::WORD, Characteristics: ::minwindef::WORD, TimeDateStamp: ::minwindef::DWORD, CheckSum: ::minwindef::DWORD, ImageBase: ::minwindef::DWORD, SizeOfImage: ::minwindef::DWORD, NumberOfSections: ::minwindef::DWORD, ExportedNamesSize: ::minwindef::DWORD, DebugDirectorySize: ::minwindef::DWORD, SectionAlignment: ::minwindef::DWORD, Reserved: *mut [::minwindef::DWORD; 2] } /* winnt.h:16700:16, winnt.h:16700:16, winnt.h:16700:16 */
pub type PIMAGE_SEPARATE_DEBUG_HEADER = *mut ::winnt::IMAGE_SEPARATE_DEBUG_HEADER; /* winnt.h:16714:33, winnt.h:16714:33, winnt.h:16714:33 */
#[repr(C)] pub struct NON_PAGED_DEBUG_INFO { Signature: ::minwindef::WORD, Flags: ::minwindef::WORD, Size: ::minwindef::DWORD, Machine: ::minwindef::WORD, Characteristics: ::minwindef::WORD, TimeDateStamp: ::minwindef::DWORD, CheckSum: ::minwindef::DWORD, SizeOfImage: ::minwindef::DWORD, ImageBase: ::winnt::ULONGLONG } /* winnt.h:16716:16, winnt.h:16716:16, winnt.h:16716:16 */
pub type PNON_PAGED_DEBUG_INFO = *mut ::winnt::NON_PAGED_DEBUG_INFO; /* winnt.h:16728:26, winnt.h:16728:26, winnt.h:16728:26 */
#[repr(C)] pub struct IMAGE_ARCHITECTURE_HEADER { AmaskValue: ::libc::c_uint, _field1: ::libc::c_int, AmaskShift: ::libc::c_uint, _field3: ::libc::c_int, FirstEntryRVA: ::minwindef::DWORD } /* winnt.h:16750:16, winnt.h:16750:16, winnt.h:16750:16 */
pub type PIMAGE_ARCHITECTURE_HEADER = *mut ::winnt::IMAGE_ARCHITECTURE_HEADER; /* winnt.h:16757:31, winnt.h:16757:31, winnt.h:16757:31 */
#[repr(C)] pub struct IMAGE_ARCHITECTURE_ENTRY { FixupInstRVA: ::minwindef::DWORD, NewInst: ::minwindef::DWORD } /* winnt.h:16759:16, winnt.h:16759:16, winnt.h:16759:16 */
pub type PIMAGE_ARCHITECTURE_ENTRY = *mut ::winnt::IMAGE_ARCHITECTURE_ENTRY; /* winnt.h:16762:30, winnt.h:16762:30, winnt.h:16762:30 */
#[repr(C)] pub struct IMPORT_OBJECT_HEADER { Sig1: ::minwindef::WORD, Sig2: ::minwindef::WORD, Version: ::minwindef::WORD, Machine: ::minwindef::WORD, TimeDateStamp: ::minwindef::DWORD, SizeOfData: ::minwindef::DWORD, u: ::winnt::IMPORT_OBJECT_HEADER_Child_6, Type: ::minwindef::WORD, NameType: ::minwindef::WORD, Reserved: ::minwindef::WORD } /* winnt.h:16773:16, winnt.h:16773:16, winnt.h:16773:16 */
#[repr(C)] pub /*union*/ struct IMPORT_OBJECT_HEADER_Child_6; /* STUB! */ /* winnt.h:16781:5, winnt.h:16781:5, winnt.h:16781:5 */
#[repr(C)] pub enum IMPORT_OBJECT_TYPE {IMPORT_OBJECT_CODE = 0, IMPORT_OBJECT_DATA = 1, IMPORT_OBJECT_CONST = 2} pub use self::IMPORT_OBJECT_TYPE::{IMPORT_OBJECT_CODE, IMPORT_OBJECT_DATA, IMPORT_OBJECT_CONST}; /* winnt.h:16791:14, winnt.h:16791:14, winnt.h:16791:14 */
#[repr(C)] pub enum IMPORT_OBJECT_NAME_TYPE {IMPORT_OBJECT_ORDINAL = 0, IMPORT_OBJECT_NAME = 1, IMPORT_OBJECT_NAME_NO_PREFIX = 2, IMPORT_OBJECT_NAME_UNDECORATE = 3} pub use self::IMPORT_OBJECT_NAME_TYPE::{IMPORT_OBJECT_ORDINAL, IMPORT_OBJECT_NAME, IMPORT_OBJECT_NAME_NO_PREFIX, IMPORT_OBJECT_NAME_UNDECORATE}; /* winnt.h:16798:14, winnt.h:16798:14, winnt.h:16798:14 */
#[repr(C)] pub enum ReplacesCorHdrNumericDefines {COMIMAGE_FLAGS_ILONLY = 1, COMIMAGE_FLAGS_32BITREQUIRED = 2, COMIMAGE_FLAGS_IL_LIBRARY = 4, COMIMAGE_FLAGS_STRONGNAMESIGNED = 8, COMIMAGE_FLAGS_NATIVE_ENTRYPOINT = 16, COMIMAGE_FLAGS_TRACKDEBUGDATA = 65536, COR_VERSION_MINOR = 5, COR_ILMETHOD_SECT_SMALL_MAX_DATASIZE = 255, IMAGE_COR_EATJ_THUNK_SIZE = 32, MAX_CLASS_NAME = 1024} pub use self::ReplacesCorHdrNumericDefines::{COMIMAGE_FLAGS_ILONLY, COMIMAGE_FLAGS_32BITREQUIRED, COMIMAGE_FLAGS_IL_LIBRARY, COMIMAGE_FLAGS_STRONGNAMESIGNED, COMIMAGE_FLAGS_NATIVE_ENTRYPOINT, COMIMAGE_FLAGS_TRACKDEBUGDATA, COR_VERSION_MINOR, COR_ILMETHOD_SECT_SMALL_MAX_DATASIZE, IMAGE_COR_EATJ_THUNK_SIZE, MAX_CLASS_NAME}; pub const COR_VERSION_MAJOR_V2: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines::COMIMAGE_FLAGS_32BITREQUIRED; pub const COR_VERSION_MAJOR: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines::COMIMAGE_FLAGS_32BITREQUIRED; pub const COR_DELETED_NAME_LENGTH: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines::COMIMAGE_FLAGS_STRONGNAMESIGNED; pub const COR_VTABLEGAP_NAME_LENGTH: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines::COMIMAGE_FLAGS_STRONGNAMESIGNED; pub const NATIVE_TYPE_MAX_CB: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines::COMIMAGE_FLAGS_ILONLY; pub const IMAGE_COR_MIH_METHODRVA: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines::COMIMAGE_FLAGS_ILONLY; pub const IMAGE_COR_MIH_EHRVA: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines::COMIMAGE_FLAGS_32BITREQUIRED; pub const IMAGE_COR_MIH_BASICBLOCK: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines::COMIMAGE_FLAGS_STRONGNAMESIGNED; pub const COR_VTABLE_32BIT: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines::COMIMAGE_FLAGS_ILONLY; pub const COR_VTABLE_64BIT: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines::COMIMAGE_FLAGS_32BITREQUIRED; pub const COR_VTABLE_FROM_UNMANAGED: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines::COMIMAGE_FLAGS_IL_LIBRARY; pub const COR_VTABLE_FROM_UNMANAGED_RETAIN_APPDOMAIN: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines::COMIMAGE_FLAGS_STRONGNAMESIGNED; pub const COR_VTABLE_CALL_MOST_DERIVED: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines::COMIMAGE_FLAGS_NATIVE_ENTRYPOINT; pub const MAX_PACKAGE_NAME: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines::MAX_CLASS_NAME; /* winnt.h:16811:14, winnt.h:16811:14, winnt.h:16811:14 */
#[repr(C)] pub struct IMAGE_COR20_HEADER { cb: ::minwindef::DWORD, MajorRuntimeVersion: ::minwindef::WORD, MinorRuntimeVersion: ::minwindef::WORD, MetaData: ::winnt::IMAGE_DATA_DIRECTORY, Flags: ::minwindef::DWORD, u: ::winnt::IMAGE_COR20_HEADER_Child_5, Resources: ::winnt::IMAGE_DATA_DIRECTORY, StrongNameSignature: ::winnt::IMAGE_DATA_DIRECTORY, CodeManagerTable: ::winnt::IMAGE_DATA_DIRECTORY, VTableFixups: ::winnt::IMAGE_DATA_DIRECTORY, ExportAddressTableJumps: ::winnt::IMAGE_DATA_DIRECTORY, ManagedNativeHeader: ::winnt::IMAGE_DATA_DIRECTORY } /* winnt.h:16854:16, winnt.h:16854:16, winnt.h:16854:16 */
#[repr(C)] pub /*union*/ struct IMAGE_COR20_HEADER_Child_5; /* STUB! */ /* winnt.h:16867:5, winnt.h:16867:5, winnt.h:16867:5 */
pub type PIMAGE_COR20_HEADER = *mut ::winnt::IMAGE_COR20_HEADER; /* winnt.h:16884:24, winnt.h:16884:24, winnt.h:16884:24 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type SLIST_ENTRY = ::winnt::SINGLE_LIST_ENTRY; /* winnt.h:17493:35, winnt.h:17493:35 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PSLIST_ENTRY = *mut ::winnt::SINGLE_LIST_ENTRY; /* winnt.h:17493:49, winnt.h:17493:49 */
#[repr(C)] pub /*union*/ struct SLIST_HEADER; /* STUB! */ /* winnt.h:17514:15, winnt.h:17499:34, winnt.h:17525:15 */
pub type PSLIST_HEADER = *mut ::winnt::SLIST_HEADER; /* winnt.h:17521:18, winnt.h:17510:18, winnt.h:17532:18 */
#[repr(C)] pub /*union*/ struct RTL_RUN_ONCE; /* STUB! */ /* winnt.h:17624:15, winnt.h:17624:15, winnt.h:17624:15 */
pub type PRTL_RUN_ONCE = *mut ::winnt::RTL_RUN_ONCE; /* winnt.h:17626:18, winnt.h:17626:18, winnt.h:17626:18 */
#[repr(C)] pub struct RTL_BARRIER { Reserved1: ::minwindef::DWORD, Reserved2: ::minwindef::DWORD, Reserved3: *mut [::basetsd::ULONG_PTR; 2], Reserved4: ::minwindef::DWORD, Reserved5: ::minwindef::DWORD } /* winnt.h:17630:16, winnt.h:17630:16, winnt.h:17630:16 */
pub type PRTL_BARRIER = *mut ::winnt::RTL_BARRIER; /* winnt.h:17636:17, winnt.h:17636:17, winnt.h:17636:17 */
#[repr(C)] pub struct MESSAGE_RESOURCE_ENTRY { Length: ::minwindef::WORD, Flags: ::minwindef::WORD, Text: *mut [::minwindef::BYTE; 1] } /* winnt.h:17838:16, winnt.h:17838:16, winnt.h:17838:16 */
pub type PMESSAGE_RESOURCE_ENTRY = *mut ::winnt::MESSAGE_RESOURCE_ENTRY; /* winnt.h:17842:28, winnt.h:17842:28, winnt.h:17842:28 */
#[repr(C)] pub struct MESSAGE_RESOURCE_BLOCK { LowId: ::minwindef::DWORD, HighId: ::minwindef::DWORD, OffsetToEntries: ::minwindef::DWORD } /* winnt.h:17846:16, winnt.h:17846:16, winnt.h:17846:16 */
pub type PMESSAGE_RESOURCE_BLOCK = *mut ::winnt::MESSAGE_RESOURCE_BLOCK; /* winnt.h:17850:28, winnt.h:17850:28, winnt.h:17850:28 */
#[repr(C)] pub struct MESSAGE_RESOURCE_DATA { NumberOfBlocks: ::minwindef::DWORD, Blocks: *mut [::winnt::MESSAGE_RESOURCE_BLOCK; 1] } /* winnt.h:17852:16, winnt.h:17852:16, winnt.h:17852:16 */
pub type PMESSAGE_RESOURCE_DATA = *mut ::winnt::MESSAGE_RESOURCE_DATA; /* winnt.h:17855:27, winnt.h:17855:27, winnt.h:17855:27 */
#[repr(C)] pub struct OSVERSIONINFOA { dwOSVersionInfoSize: ::minwindef::DWORD, dwMajorVersion: ::minwindef::DWORD, dwMinorVersion: ::minwindef::DWORD, dwBuildNumber: ::minwindef::DWORD, dwPlatformId: ::minwindef::DWORD, szCSDVersion: *mut [::winnt::CHAR; 128] } /* winnt.h:17857:16, winnt.h:17857:16, winnt.h:17857:16 */
pub type POSVERSIONINFOA = *mut ::winnt::OSVERSIONINFOA; /* winnt.h:17864:20, winnt.h:17864:20, winnt.h:17864:20 */
pub type LPOSVERSIONINFOA = *mut ::winnt::OSVERSIONINFOA; /* winnt.h:17864:38, winnt.h:17864:38, winnt.h:17864:38 */
#[repr(C)] pub struct OSVERSIONINFOW { dwOSVersionInfoSize: ::minwindef::DWORD, dwMajorVersion: ::minwindef::DWORD, dwMinorVersion: ::minwindef::DWORD, dwBuildNumber: ::minwindef::DWORD, dwPlatformId: ::minwindef::DWORD, szCSDVersion: *mut [::winnt::WCHAR; 128] } /* winnt.h:17866:16, winnt.h:17866:16, winnt.h:17866:16 */
pub type POSVERSIONINFOW = *mut ::winnt::OSVERSIONINFOW; /* winnt.h:17873:20, winnt.h:17873:20, winnt.h:17873:20 */
pub type LPOSVERSIONINFOW = *mut ::winnt::OSVERSIONINFOW; /* winnt.h:17873:38, winnt.h:17873:38, winnt.h:17873:38 */
pub type RTL_OSVERSIONINFOW = ::winnt::OSVERSIONINFOW; /* winnt.h:17873:56, winnt.h:17873:56, winnt.h:17873:56 */
pub type PRTL_OSVERSIONINFOW = *mut ::winnt::OSVERSIONINFOW; /* winnt.h:17873:77, winnt.h:17873:77, winnt.h:17873:77 */
pub type OSVERSIONINFO = ::winnt::OSVERSIONINFOW; /* winnt.h:17875:24, winnt.h:17875:24, winnt.h:17875:24 */
pub type POSVERSIONINFO = ::winnt::POSVERSIONINFOW; /* winnt.h:17876:25, winnt.h:17876:25, winnt.h:17876:25 */
pub type LPOSVERSIONINFO = ::winnt::LPOSVERSIONINFOW; /* winnt.h:17877:26, winnt.h:17877:26, winnt.h:17877:26 */
#[repr(C)] pub struct OSVERSIONINFOEXA { dwOSVersionInfoSize: ::minwindef::DWORD, dwMajorVersion: ::minwindef::DWORD, dwMinorVersion: ::minwindef::DWORD, dwBuildNumber: ::minwindef::DWORD, dwPlatformId: ::minwindef::DWORD, szCSDVersion: *mut [::winnt::CHAR; 128], wServicePackMajor: ::minwindef::WORD, wServicePackMinor: ::minwindef::WORD, wSuiteMask: ::minwindef::WORD, wProductType: ::minwindef::BYTE, wReserved: ::minwindef::BYTE } /* winnt.h:17884:16, winnt.h:17884:16, winnt.h:17884:16 */
pub type POSVERSIONINFOEXA = *mut ::winnt::OSVERSIONINFOEXA; /* winnt.h:17896:22, winnt.h:17896:22, winnt.h:17896:22 */
pub type LPOSVERSIONINFOEXA = *mut ::winnt::OSVERSIONINFOEXA; /* winnt.h:17896:42, winnt.h:17896:42, winnt.h:17896:42 */
#[repr(C)] pub struct OSVERSIONINFOEXW { dwOSVersionInfoSize: ::minwindef::DWORD, dwMajorVersion: ::minwindef::DWORD, dwMinorVersion: ::minwindef::DWORD, dwBuildNumber: ::minwindef::DWORD, dwPlatformId: ::minwindef::DWORD, szCSDVersion: *mut [::winnt::WCHAR; 128], wServicePackMajor: ::minwindef::WORD, wServicePackMinor: ::minwindef::WORD, wSuiteMask: ::minwindef::WORD, wProductType: ::minwindef::BYTE, wReserved: ::minwindef::BYTE } /* winnt.h:17897:16, winnt.h:17897:16, winnt.h:17897:16 */
pub type POSVERSIONINFOEXW = *mut ::winnt::OSVERSIONINFOEXW; /* winnt.h:17909:22, winnt.h:17909:22, winnt.h:17909:22 */
pub type LPOSVERSIONINFOEXW = *mut ::winnt::OSVERSIONINFOEXW; /* winnt.h:17909:42, winnt.h:17909:42, winnt.h:17909:42 */
pub type RTL_OSVERSIONINFOEXW = ::winnt::OSVERSIONINFOEXW; /* winnt.h:17909:62, winnt.h:17909:62, winnt.h:17909:62 */
pub type PRTL_OSVERSIONINFOEXW = *mut ::winnt::OSVERSIONINFOEXW; /* winnt.h:17909:85, winnt.h:17909:85, winnt.h:17909:85 */
pub type OSVERSIONINFOEX = ::winnt::OSVERSIONINFOEXW; /* winnt.h:17911:26, winnt.h:17911:26, winnt.h:17911:26 */
pub type POSVERSIONINFOEX = ::winnt::POSVERSIONINFOEXW; /* winnt.h:17912:27, winnt.h:17912:27, winnt.h:17912:27 */
pub type LPOSVERSIONINFOEX = ::winnt::LPOSVERSIONINFOEXW; /* winnt.h:17913:28, winnt.h:17913:28, winnt.h:17913:28 */
#[repr(C)] pub enum RTL_UMS_THREAD_INFO_CLASS {UmsThreadInvalidInfoClass = 0, UmsThreadUserContext = 1, UmsThreadPriority = 2, UmsThreadAffinity = 3, UmsThreadTeb = 4, UmsThreadIsSuspended = 5, UmsThreadIsTerminated = 6, UmsThreadMaxInfoClass = 7} pub use self::RTL_UMS_THREAD_INFO_CLASS::{UmsThreadInvalidInfoClass, UmsThreadUserContext, UmsThreadPriority, UmsThreadAffinity, UmsThreadTeb, UmsThreadIsSuspended, UmsThreadIsTerminated, UmsThreadMaxInfoClass}; /* winnt.h:18028:14, winnt.h:18028:14, winnt.h:18028:14 */
pub type PRTL_UMS_THREAD_INFO_CLASS = *mut ::winnt::RTL_UMS_THREAD_INFO_CLASS; /* winnt.h:18037:31, winnt.h:18037:31, winnt.h:18037:31 */
#[repr(C)] pub enum RTL_UMS_SCHEDULER_REASON {UmsSchedulerStartup = 0, UmsSchedulerThreadBlocked = 1, UmsSchedulerThreadYield = 2} pub use self::RTL_UMS_SCHEDULER_REASON::{UmsSchedulerStartup, UmsSchedulerThreadBlocked, UmsSchedulerThreadYield}; /* winnt.h:18039:14, winnt.h:18039:14, winnt.h:18039:14 */
pub type PRTL_UMS_SCHEDULER_REASON = *mut ::winnt::RTL_UMS_SCHEDULER_REASON; /* winnt.h:18043:30, winnt.h:18043:30, winnt.h:18043:30 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type RTL_UMS_SCHEDULER_ENTRY_POINT = extern "system" fn(::winnt::RTL_UMS_SCHEDULER_REASON, ::libc::c_ulong, *mut ::libc::c_void); /* winnt.h:18049:1, winnt.h:18049:1 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PRTL_UMS_SCHEDULER_ENTRY_POINT = extern "system" fn(::winnt::RTL_UMS_SCHEDULER_REASON, ::libc::c_ulong, *mut ::libc::c_void); /* winnt.h:18055:40, winnt.h:18055:40 */
#[repr(C)] pub struct RTL_CRITICAL_SECTION_DEBUG { Type: ::minwindef::WORD, CreatorBackTraceIndex: ::minwindef::WORD, CriticalSection: *mut ::winnt::RTL_CRITICAL_SECTION, ProcessLocksList: ::winnt::LIST_ENTRY, EntryCount: ::minwindef::DWORD, ContentionCount: ::minwindef::DWORD, Flags: ::minwindef::DWORD, CreatorBackTraceIndexHigh: ::minwindef::WORD, SpareWORD: ::minwindef::WORD } /* winnt.h:18145:16, winnt.h:18145:16, winnt.h:18145:16 */
pub type PRTL_CRITICAL_SECTION_DEBUG = *mut ::winnt::RTL_CRITICAL_SECTION_DEBUG; /* winnt.h:18155:32, winnt.h:18155:32, winnt.h:18155:32 */
pub type RTL_RESOURCE_DEBUG = ::winnt::RTL_CRITICAL_SECTION_DEBUG; /* winnt.h:18155:61, winnt.h:18155:61, winnt.h:18155:61 */
pub type PRTL_RESOURCE_DEBUG = *mut ::winnt::RTL_CRITICAL_SECTION_DEBUG; /* winnt.h:18155:82, winnt.h:18155:82, winnt.h:18155:82 */
#[repr(C)] pub struct RTL_CRITICAL_SECTION { DebugInfo: ::winnt::PRTL_CRITICAL_SECTION_DEBUG, LockCount: ::winnt::LONG, RecursionCount: ::winnt::LONG, OwningThread: ::winnt::HANDLE, LockSemaphore: ::winnt::HANDLE, SpinCount: ::basetsd::ULONG_PTR } /* winnt.h:18178:16, winnt.h:18178:16, winnt.h:18178:16 */
pub type PRTL_CRITICAL_SECTION = *mut ::winnt::RTL_CRITICAL_SECTION; /* winnt.h:18191:26, winnt.h:18191:26, winnt.h:18191:26 */
#[repr(C)] pub struct RTL_SRWLOCK { Ptr: ::winnt::PVOID } /* winnt.h:18195:16, winnt.h:18195:16, winnt.h:18195:16 */
pub type PRTL_SRWLOCK = *mut ::winnt::RTL_SRWLOCK; /* winnt.h:18197:17, winnt.h:18197:17, winnt.h:18197:17 */
#[repr(C)] pub struct RTL_CONDITION_VARIABLE { Ptr: ::winnt::PVOID } /* winnt.h:18199:16, winnt.h:18199:16, winnt.h:18199:16 */
pub type PRTL_CONDITION_VARIABLE = *mut ::winnt::RTL_CONDITION_VARIABLE; /* winnt.h:18201:28, winnt.h:18201:28, winnt.h:18201:28 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PAPCFUNC = extern "system" fn(::libc::c_ulong); /* winnt.h:18206:9, winnt.h:18206:9 */
pub type PVECTORED_EXCEPTION_HANDLER = extern "system" fn(*mut ::winnt::EXCEPTION_POINTERS) -> ::libc::c_long; /* winnt.h:18209:22, winnt.h:18209:22, winnt.h:18209:22 */
#[repr(C)] pub enum HEAP_INFORMATION_CLASS {HeapCompatibilityInformation = 0, HeapEnableTerminationOnCorruption = 1} pub use self::HEAP_INFORMATION_CLASS::{HeapCompatibilityInformation, HeapEnableTerminationOnCorruption}; /* winnt.h:18213:14, winnt.h:18213:14, winnt.h:18213:14 */
pub type WAITORTIMERCALLBACKFUNC = extern "system" fn(*mut ::libc::c_void, ::libc::c_uchar); /* winnt.h:18254:23, winnt.h:18254:23, winnt.h:18254:23 */
pub type WORKERCALLBACKFUNC = extern "system" fn(*mut ::libc::c_void); /* winnt.h:18255:23, winnt.h:18255:23, winnt.h:18255:23 */
pub type APC_CALLBACK_FUNCTION = extern "system" fn(::libc::c_ulong, *mut ::libc::c_void, *mut ::libc::c_void); /* winnt.h:18256:23, winnt.h:18256:23, winnt.h:18256:23 */
pub type WAITORTIMERCALLBACK = ::winnt::WAITORTIMERCALLBACKFUNC; /* winnt.h:18257:33, winnt.h:18257:33, winnt.h:18257:33 */
pub type PFLS_CALLBACK_FUNCTION = extern "system" fn(*mut ::libc::c_void); /* winnt.h:18260:9, winnt.h:18260:9, winnt.h:18260:9 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PSECURE_MEMORY_CACHE_CALLBACK = extern "system" fn(*mut ::libc::c_void, ::libc::c_ulong) -> ::libc::c_uchar; /* winnt.h:18266:9, winnt.h:18266:9 */
#[repr(C)] pub enum ACTIVATION_CONTEXT_INFO_CLASS {ActivationContextBasicInformation = 1, ActivationContextDetailedInformation = 2, AssemblyDetailedInformationInActivationContext = 3, FileInformationInAssemblyOfAssemblyInActivationContext = 4, RunlevelInformationInActivationContext = 5, CompatibilityInformationInActivationContext = 6, ActivationContextManifestResourceName = 7, MaxActivationContextInfoClass = 8} pub use self::ACTIVATION_CONTEXT_INFO_CLASS::{ActivationContextBasicInformation, ActivationContextDetailedInformation, AssemblyDetailedInformationInActivationContext, FileInformationInAssemblyOfAssemblyInActivationContext, RunlevelInformationInActivationContext, CompatibilityInformationInActivationContext, ActivationContextManifestResourceName, MaxActivationContextInfoClass}; pub const AssemblyDetailedInformationInActivationContxt: ACTIVATION_CONTEXT_INFO_CLASS = ACTIVATION_CONTEXT_INFO_CLASS::AssemblyDetailedInformationInActivationContext; pub const FileInformationInAssemblyOfAssemblyInActivationContxt: ACTIVATION_CONTEXT_INFO_CLASS = ACTIVATION_CONTEXT_INFO_CLASS::FileInformationInAssemblyOfAssemblyInActivationContext; /* winnt.h:18274:14, winnt.h:18274:14, winnt.h:18274:14 */
#[repr(C)] pub struct ACTIVATION_CONTEXT_QUERY_INDEX { ulAssemblyIndex: ::minwindef::DWORD, ulFileIndexInAssembly: ::minwindef::DWORD } /* winnt.h:18294:16, winnt.h:18294:16, winnt.h:18294:16 */
pub type PACTIVATION_CONTEXT_QUERY_INDEX = *mut ::winnt::ACTIVATION_CONTEXT_QUERY_INDEX; /* winnt.h:18297:37, winnt.h:18297:37, winnt.h:18297:37 */
pub type PCACTIVATION_CONTEXT_QUERY_INDEX = *const ::winnt::ACTIVATION_CONTEXT_QUERY_INDEX; /* winnt.h:18299:56, winnt.h:18299:56, winnt.h:18299:56 */
#[repr(C)] pub struct ASSEMBLY_FILE_DETAILED_INFORMATION { ulFlags: ::minwindef::DWORD, ulFilenameLength: ::minwindef::DWORD, ulPathLength: ::minwindef::DWORD, lpFileName: ::winnt::PCWSTR, lpFilePath: ::winnt::PCWSTR } /* winnt.h:18307:16, winnt.h:18307:16, winnt.h:18307:16 */
pub type PASSEMBLY_FILE_DETAILED_INFORMATION = *mut ::winnt::ASSEMBLY_FILE_DETAILED_INFORMATION; /* winnt.h:18314:40, winnt.h:18314:40, winnt.h:18314:40 */
pub type PCASSEMBLY_FILE_DETAILED_INFORMATION = *const ::winnt::ASSEMBLY_FILE_DETAILED_INFORMATION; /* winnt.h:18315:51, winnt.h:18315:51, winnt.h:18315:51 */
#[repr(C)] pub struct ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION { ulFlags: ::minwindef::DWORD, ulEncodedAssemblyIdentityLength: ::minwindef::DWORD, ulManifestPathType: ::minwindef::DWORD, ulManifestPathLength: ::minwindef::DWORD, liManifestLastWriteTime: ::winnt::LARGE_INTEGER, ulPolicyPathType: ::minwindef::DWORD, ulPolicyPathLength: ::minwindef::DWORD, liPolicyLastWriteTime: ::winnt::LARGE_INTEGER, ulMetadataSatelliteRosterIndex: ::minwindef::DWORD, ulManifestVersionMajor: ::minwindef::DWORD, ulManifestVersionMinor: ::minwindef::DWORD, ulPolicyVersionMajor: ::minwindef::DWORD, ulPolicyVersionMinor: ::minwindef::DWORD, ulAssemblyDirectoryNameLength: ::minwindef::DWORD, lpAssemblyEncodedAssemblyIdentity: ::winnt::PCWSTR, lpAssemblyManifestPath: ::winnt::PCWSTR, lpAssemblyPolicyPath: ::winnt::PCWSTR, lpAssemblyDirectoryName: ::winnt::PCWSTR, ulFileCount: ::minwindef::DWORD } /* winnt.h:18326:16, winnt.h:18326:16, winnt.h:18326:16 */
pub type PACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION = *mut ::winnt::ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION; /* winnt.h:18349:55, winnt.h:18349:55, winnt.h:18349:55 */
pub type PCACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION = *const ::winnt::ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION; /* winnt.h:18351:74, winnt.h:18351:74, winnt.h:18351:74 */
#[repr(C)] pub enum ACTCTX_REQUESTED_RUN_LEVEL {ACTCTX_RUN_LEVEL_UNSPECIFIED = 0, ACTCTX_RUN_LEVEL_AS_INVOKER = 1, ACTCTX_RUN_LEVEL_HIGHEST_AVAILABLE = 2, ACTCTX_RUN_LEVEL_REQUIRE_ADMIN = 3, ACTCTX_RUN_LEVEL_NUMBERS = 4} pub use self::ACTCTX_REQUESTED_RUN_LEVEL::{ACTCTX_RUN_LEVEL_UNSPECIFIED, ACTCTX_RUN_LEVEL_AS_INVOKER, ACTCTX_RUN_LEVEL_HIGHEST_AVAILABLE, ACTCTX_RUN_LEVEL_REQUIRE_ADMIN, ACTCTX_RUN_LEVEL_NUMBERS}; /* winnt.h:18353:9, winnt.h:18353:9, winnt.h:18353:9 */
#[repr(C)] pub struct ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION { ulFlags: ::minwindef::DWORD, RunLevel: ::winnt::ACTCTX_REQUESTED_RUN_LEVEL, UiAccess: ::minwindef::DWORD } /* winnt.h:18362:16, winnt.h:18362:16, winnt.h:18362:16 */
pub type PACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION = *mut ::winnt::ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION; /* winnt.h:18366:47, winnt.h:18366:47, winnt.h:18366:47 */
pub type PCACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION = *const ::winnt::ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION; /* winnt.h:18368:66, winnt.h:18368:66, winnt.h:18368:66 */
#[repr(C)] pub enum ACTCTX_COMPATIBILITY_ELEMENT_TYPE {ACTCTX_COMPATIBILITY_ELEMENT_TYPE_UNKNOWN = 0, ACTCTX_COMPATIBILITY_ELEMENT_TYPE_OS = 1, ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MITIGATION = 2} pub use self::ACTCTX_COMPATIBILITY_ELEMENT_TYPE::{ACTCTX_COMPATIBILITY_ELEMENT_TYPE_UNKNOWN, ACTCTX_COMPATIBILITY_ELEMENT_TYPE_OS, ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MITIGATION}; /* winnt.h:18370:9, winnt.h:18370:9, winnt.h:18370:9 */
#[repr(C)] pub struct COMPATIBILITY_CONTEXT_ELEMENT { Id: ::guiddef::GUID, Type: ::winnt::ACTCTX_COMPATIBILITY_ELEMENT_TYPE } /* winnt.h:18377:16, winnt.h:18377:16, winnt.h:18377:16 */
pub type PCOMPATIBILITY_CONTEXT_ELEMENT = *mut ::winnt::COMPATIBILITY_CONTEXT_ELEMENT; /* winnt.h:18380:35, winnt.h:18380:35, winnt.h:18380:35 */
pub type PCCOMPATIBILITY_CONTEXT_ELEMENT = *const ::winnt::COMPATIBILITY_CONTEXT_ELEMENT; /* winnt.h:18382:54, winnt.h:18382:54, winnt.h:18382:54 */
#[repr(C)] pub struct ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION { ElementCount: ::minwindef::DWORD, Elements: *mut ::winnt::COMPATIBILITY_CONTEXT_ELEMENT } /* winnt.h:18387:16, winnt.h:18387:16, winnt.h:18387:16 */
pub type PACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION = *mut ::winnt::ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION; /* winnt.h:18390:51, winnt.h:18390:51, winnt.h:18390:51 */
pub type PCACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION = *const ::winnt::ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION; /* winnt.h:18394:70, winnt.h:18394:70, winnt.h:18394:70 */
#[repr(C)] pub struct SUPPORTED_OS_INFO { OsCount: ::minwindef::WORD, MitigationExist: ::minwindef::WORD, OsList: *mut [::minwindef::WORD; 4] } /* winnt.h:18399:16, winnt.h:18399:16, winnt.h:18399:16 */
pub type PSUPPORTED_OS_INFO = *mut ::winnt::SUPPORTED_OS_INFO; /* winnt.h:18403:23, winnt.h:18403:23, winnt.h:18403:23 */
#[repr(C)] pub struct ACTIVATION_CONTEXT_DETAILED_INFORMATION { dwFlags: ::minwindef::DWORD, ulFormatVersion: ::minwindef::DWORD, ulAssemblyCount: ::minwindef::DWORD, ulRootManifestPathType: ::minwindef::DWORD, ulRootManifestPathChars: ::minwindef::DWORD, ulRootConfigurationPathType: ::minwindef::DWORD, ulRootConfigurationPathChars: ::minwindef::DWORD, ulAppDirPathType: ::minwindef::DWORD, ulAppDirPathChars: ::minwindef::DWORD, lpRootManifestPath: ::winnt::PCWSTR, lpRootConfigurationPath: ::winnt::PCWSTR, lpAppDirPath: ::winnt::PCWSTR } /* winnt.h:18405:16, winnt.h:18405:16, winnt.h:18405:16 */
pub type PACTIVATION_CONTEXT_DETAILED_INFORMATION = *mut ::winnt::ACTIVATION_CONTEXT_DETAILED_INFORMATION; /* winnt.h:18418:45, winnt.h:18418:45, winnt.h:18418:45 */
pub type PCACTIVATION_CONTEXT_DETAILED_INFORMATION = *const ::winnt::ACTIVATION_CONTEXT_DETAILED_INFORMATION; /* winnt.h:18420:64, winnt.h:18420:64, winnt.h:18420:64 */
#[repr(C)] pub struct HARDWARE_COUNTER_DATA { Type: ::winnt::HARDWARE_COUNTER_TYPE, Reserved: ::minwindef::DWORD, Value: ::basetsd::DWORD64 } /* winnt.h:18425:16, winnt.h:18425:16, winnt.h:18425:16 */
pub type PHARDWARE_COUNTER_DATA = *mut ::winnt::HARDWARE_COUNTER_DATA; /* winnt.h:18429:27, winnt.h:18429:27, winnt.h:18429:27 */
#[repr(C)] pub struct PERFORMANCE_DATA { Size: ::minwindef::WORD, Version: ::minwindef::BYTE, HwCountersCount: ::minwindef::BYTE, ContextSwitchCount: ::minwindef::DWORD, WaitReasonBitMap: ::basetsd::DWORD64, CycleTime: ::basetsd::DWORD64, RetryCount: ::minwindef::DWORD, Reserved: ::minwindef::DWORD, HwCounters: *mut [::winnt::HARDWARE_COUNTER_DATA; 16] } /* winnt.h:18433:16, winnt.h:18433:16, winnt.h:18433:16 */
pub type PPERFORMANCE_DATA = *mut ::winnt::PERFORMANCE_DATA; /* winnt.h:18443:22, winnt.h:18443:22, winnt.h:18443:22 */
#[repr(C)] pub struct EVENTLOGRECORD { Length: ::minwindef::DWORD, Reserved: ::minwindef::DWORD, RecordNumber: ::minwindef::DWORD, TimeGenerated: ::minwindef::DWORD, TimeWritten: ::minwindef::DWORD, EventID: ::minwindef::DWORD, EventType: ::minwindef::WORD, NumStrings: ::minwindef::WORD, EventCategory: ::minwindef::WORD, ReservedFlags: ::minwindef::WORD, ClosingRecordNumber: ::minwindef::DWORD, StringOffset: ::minwindef::DWORD, UserSidLength: ::minwindef::DWORD, UserSidOffset: ::minwindef::DWORD, DataLength: ::minwindef::DWORD, DataOffset: ::minwindef::DWORD } /* winnt.h:18490:16, winnt.h:18490:16, winnt.h:18490:16 */
pub type PEVENTLOGRECORD = *mut ::winnt::EVENTLOGRECORD; /* winnt.h:18518:20, winnt.h:18518:20, winnt.h:18518:20 */
#[repr(C)] pub struct EVENTSFORLOGFILE; /* winnt.h:18529:8, winnt.h:18529:8, winnt.h:18529:8 */
pub type PEVENTSFORLOGFILE = *mut ::libc::c_void; /* winnt.h:18530:53, winnt.h:18530:53, winnt.h:18530:53 */
#[repr(C)] pub struct PACKEDEVENTINFO; /* winnt.h:18532:8, winnt.h:18532:8, winnt.h:18532:8 */
pub type PPACKEDEVENTINFO = *mut ::libc::c_void; /* winnt.h:18533:51, winnt.h:18533:51, winnt.h:18533:51 */
#[repr(C)] pub enum SERVICE_NODE_TYPE {DriverType = 1, FileSystemType = 2, Win32ServiceOwnProcess = 16, Win32ServiceShareProcess = 32, AdapterType = 4, RecognizerType = 8} pub use self::SERVICE_NODE_TYPE::{DriverType, FileSystemType, Win32ServiceOwnProcess, Win32ServiceShareProcess, AdapterType, RecognizerType}; /* winnt.h:18767:14, winnt.h:18767:14, winnt.h:18767:14 */
#[repr(C)] pub enum SERVICE_LOAD_TYPE {BootLoad = 0, SystemLoad = 1, AutoLoad = 2, DemandLoad = 3, DisableLoad = 4} pub use self::SERVICE_LOAD_TYPE::{BootLoad, SystemLoad, AutoLoad, DemandLoad, DisableLoad}; /* winnt.h:18776:14, winnt.h:18776:14, winnt.h:18776:14 */
#[repr(C)] pub enum SERVICE_ERROR_TYPE {IgnoreError = 0, NormalError = 1, SevereError = 2, CriticalError = 3} pub use self::SERVICE_ERROR_TYPE::{IgnoreError, NormalError, SevereError, CriticalError}; /* winnt.h:18784:14, winnt.h:18784:14, winnt.h:18784:14 */
#[repr(C)] pub struct TAPE_ERASE { Type: ::minwindef::DWORD, Immediate: ::winnt::BOOLEAN } /* winnt.h:18855:16, winnt.h:18855:16, winnt.h:18855:16 */
pub type PTAPE_ERASE = *mut ::winnt::TAPE_ERASE; /* winnt.h:18858:16, winnt.h:18858:16, winnt.h:18858:16 */
#[repr(C)] pub struct TAPE_PREPARE { Operation: ::minwindef::DWORD, Immediate: ::winnt::BOOLEAN } /* winnt.h:18871:16, winnt.h:18871:16, winnt.h:18871:16 */
pub type PTAPE_PREPARE = *mut ::winnt::TAPE_PREPARE; /* winnt.h:18874:18, winnt.h:18874:18, winnt.h:18874:18 */
#[repr(C)] pub struct TAPE_WRITE_MARKS { Type: ::minwindef::DWORD, Count: ::minwindef::DWORD, Immediate: ::winnt::BOOLEAN } /* winnt.h:18885:16, winnt.h:18885:16, winnt.h:18885:16 */
pub type PTAPE_WRITE_MARKS = *mut ::winnt::TAPE_WRITE_MARKS; /* winnt.h:18889:22, winnt.h:18889:22, winnt.h:18889:22 */
#[repr(C)] pub struct TAPE_GET_POSITION { Type: ::minwindef::DWORD, Partition: ::minwindef::DWORD, Offset: ::winnt::LARGE_INTEGER } /* winnt.h:18899:16, winnt.h:18899:16, winnt.h:18899:16 */
pub type PTAPE_GET_POSITION = *mut ::winnt::TAPE_GET_POSITION; /* winnt.h:18903:23, winnt.h:18903:23, winnt.h:18903:23 */
#[repr(C)] pub struct TAPE_SET_POSITION { Method: ::minwindef::DWORD, Partition: ::minwindef::DWORD, Offset: ::winnt::LARGE_INTEGER, Immediate: ::winnt::BOOLEAN } /* winnt.h:18920:16, winnt.h:18920:16, winnt.h:18920:16 */
pub type PTAPE_SET_POSITION = *mut ::winnt::TAPE_SET_POSITION; /* winnt.h:18925:23, winnt.h:18925:23, winnt.h:18925:23 */
#[repr(C)] pub struct TAPE_GET_DRIVE_PARAMETERS { ECC: ::winnt::BOOLEAN, Compression: ::winnt::BOOLEAN, DataPadding: ::winnt::BOOLEAN, ReportSetmarks: ::winnt::BOOLEAN, DefaultBlockSize: ::minwindef::DWORD, MaximumBlockSize: ::minwindef::DWORD, MinimumBlockSize: ::minwindef::DWORD, MaximumPartitionCount: ::minwindef::DWORD, FeaturesLow: ::minwindef::DWORD, FeaturesHigh: ::minwindef::DWORD, EOTWarningZoneSize: ::minwindef::DWORD } /* winnt.h:19013:16, winnt.h:19013:16, winnt.h:19013:16 */
pub type PTAPE_GET_DRIVE_PARAMETERS = *mut ::winnt::TAPE_GET_DRIVE_PARAMETERS; /* winnt.h:19025:31, winnt.h:19025:31, winnt.h:19025:31 */
#[repr(C)] pub struct TAPE_SET_DRIVE_PARAMETERS { ECC: ::winnt::BOOLEAN, Compression: ::winnt::BOOLEAN, DataPadding: ::winnt::BOOLEAN, ReportSetmarks: ::winnt::BOOLEAN, EOTWarningZoneSize: ::minwindef::DWORD } /* winnt.h:19031:16, winnt.h:19031:16, winnt.h:19031:16 */
pub type PTAPE_SET_DRIVE_PARAMETERS = *mut ::winnt::TAPE_SET_DRIVE_PARAMETERS; /* winnt.h:19037:31, winnt.h:19037:31, winnt.h:19037:31 */
#[repr(C)] pub struct TAPE_GET_MEDIA_PARAMETERS { Capacity: ::winnt::LARGE_INTEGER, Remaining: ::winnt::LARGE_INTEGER, BlockSize: ::minwindef::DWORD, PartitionCount: ::minwindef::DWORD, WriteProtected: ::winnt::BOOLEAN } /* winnt.h:19043:16, winnt.h:19043:16, winnt.h:19043:16 */
pub type PTAPE_GET_MEDIA_PARAMETERS = *mut ::winnt::TAPE_GET_MEDIA_PARAMETERS; /* winnt.h:19049:31, winnt.h:19049:31, winnt.h:19049:31 */
#[repr(C)] pub struct TAPE_SET_MEDIA_PARAMETERS { BlockSize: ::minwindef::DWORD } /* winnt.h:19055:16, winnt.h:19055:16, winnt.h:19055:16 */
pub type PTAPE_SET_MEDIA_PARAMETERS = *mut ::winnt::TAPE_SET_MEDIA_PARAMETERS; /* winnt.h:19057:31, winnt.h:19057:31, winnt.h:19057:31 */
#[repr(C)] pub struct TAPE_CREATE_PARTITION { Method: ::minwindef::DWORD, Count: ::minwindef::DWORD, Size: ::minwindef::DWORD } /* winnt.h:19067:16, winnt.h:19067:16, winnt.h:19067:16 */
pub type PTAPE_CREATE_PARTITION = *mut ::winnt::TAPE_CREATE_PARTITION; /* winnt.h:19071:27, winnt.h:19071:27, winnt.h:19071:27 */
#[repr(C)] pub struct TAPE_WMI_OPERATIONS { Method: ::minwindef::DWORD, DataBufferSize: ::minwindef::DWORD, DataBuffer: ::winnt::PVOID } /* winnt.h:19083:16, winnt.h:19083:16, winnt.h:19083:16 */
pub type PTAPE_WMI_OPERATIONS = *mut ::winnt::TAPE_WMI_OPERATIONS; /* winnt.h:19087:25, winnt.h:19087:25, winnt.h:19087:25 */
#[repr(C)] pub enum TAPE_DRIVE_PROBLEM_TYPE {TapeDriveProblemNone = 0, TapeDriveReadWriteWarning = 1, TapeDriveReadWriteError = 2, TapeDriveReadWarning = 3, TapeDriveWriteWarning = 4, TapeDriveReadError = 5, TapeDriveWriteError = 6, TapeDriveHardwareError = 7, TapeDriveUnsupportedMedia = 8, TapeDriveScsiConnectionError = 9, TapeDriveTimetoClean = 10, TapeDriveCleanDriveNow = 11, TapeDriveMediaLifeExpired = 12, TapeDriveSnappedTape = 13} pub use self::TAPE_DRIVE_PROBLEM_TYPE::{TapeDriveProblemNone, TapeDriveReadWriteWarning, TapeDriveReadWriteError, TapeDriveReadWarning, TapeDriveWriteWarning, TapeDriveReadError, TapeDriveWriteError, TapeDriveHardwareError, TapeDriveUnsupportedMedia, TapeDriveScsiConnectionError, TapeDriveTimetoClean, TapeDriveCleanDriveNow, TapeDriveMediaLifeExpired, TapeDriveSnappedTape}; /* winnt.h:19092:14, winnt.h:19092:14, winnt.h:19092:14 */
#[repr(C)] pub enum TRANSACTION_OUTCOME {TransactionOutcomeUndetermined = 1, TransactionOutcomeCommitted = 2, TransactionOutcomeAborted = 3} pub use self::TRANSACTION_OUTCOME::{TransactionOutcomeUndetermined, TransactionOutcomeCommitted, TransactionOutcomeAborted}; /* winnt.h:19281:14, winnt.h:19281:14, winnt.h:19281:14 */
#[repr(C)] pub enum TRANSACTION_STATE {TransactionStateNormal = 1, TransactionStateIndoubt = 2, TransactionStateCommittedNotify = 3} pub use self::TRANSACTION_STATE::{TransactionStateNormal, TransactionStateIndoubt, TransactionStateCommittedNotify}; /* winnt.h:19288:14, winnt.h:19288:14, winnt.h:19288:14 */
#[repr(C)] pub struct TRANSACTION_BASIC_INFORMATION { TransactionId: ::guiddef::GUID, State: ::minwindef::DWORD, Outcome: ::minwindef::DWORD } /* winnt.h:19295:16, winnt.h:19295:16, winnt.h:19295:16 */
pub type PTRANSACTION_BASIC_INFORMATION = *mut ::winnt::TRANSACTION_BASIC_INFORMATION; /* winnt.h:19299:35, winnt.h:19299:35, winnt.h:19299:35 */
#[repr(C)] pub struct TRANSACTIONMANAGER_BASIC_INFORMATION { TmIdentity: ::guiddef::GUID, VirtualClock: ::winnt::LARGE_INTEGER } /* winnt.h:19301:16, winnt.h:19301:16, winnt.h:19301:16 */
pub type PTRANSACTIONMANAGER_BASIC_INFORMATION = *mut ::winnt::TRANSACTIONMANAGER_BASIC_INFORMATION; /* winnt.h:19304:42, winnt.h:19304:42, winnt.h:19304:42 */
#[repr(C)] pub struct TRANSACTIONMANAGER_LOG_INFORMATION { LogIdentity: ::guiddef::GUID } /* winnt.h:19306:16, winnt.h:19306:16, winnt.h:19306:16 */
pub type PTRANSACTIONMANAGER_LOG_INFORMATION = *mut ::winnt::TRANSACTIONMANAGER_LOG_INFORMATION; /* winnt.h:19308:40, winnt.h:19308:40, winnt.h:19308:40 */
#[repr(C)] pub struct TRANSACTIONMANAGER_LOGPATH_INFORMATION { LogPathLength: ::minwindef::DWORD, LogPath: *mut [::winnt::WCHAR; 1] } /* winnt.h:19310:16, winnt.h:19310:16, winnt.h:19310:16 */
pub type PTRANSACTIONMANAGER_LOGPATH_INFORMATION = *mut ::winnt::TRANSACTIONMANAGER_LOGPATH_INFORMATION; /* winnt.h:19314:44, winnt.h:19314:44, winnt.h:19314:44 */
#[repr(C)] pub struct TRANSACTIONMANAGER_RECOVERY_INFORMATION { LastRecoveredLsn: ::winnt::ULONGLONG } /* winnt.h:19316:16, winnt.h:19316:16, winnt.h:19316:16 */
pub type PTRANSACTIONMANAGER_RECOVERY_INFORMATION = *mut ::winnt::TRANSACTIONMANAGER_RECOVERY_INFORMATION; /* winnt.h:19318:45, winnt.h:19318:45, winnt.h:19318:45 */
#[repr(C)] pub struct TRANSACTIONMANAGER_OLDEST_INFORMATION { OldestTransactionGuid: ::guiddef::GUID } /* winnt.h:19322:16, winnt.h:19322:16, winnt.h:19322:16 */
pub type PTRANSACTIONMANAGER_OLDEST_INFORMATION = *mut ::winnt::TRANSACTIONMANAGER_OLDEST_INFORMATION; /* winnt.h:19324:43, winnt.h:19324:43, winnt.h:19324:43 */
#[repr(C)] pub struct TRANSACTION_PROPERTIES_INFORMATION { IsolationLevel: ::minwindef::DWORD, IsolationFlags: ::minwindef::DWORD, Timeout: ::winnt::LARGE_INTEGER, Outcome: ::minwindef::DWORD, DescriptionLength: ::minwindef::DWORD, Description: *mut [::winnt::WCHAR; 1] } /* winnt.h:19328:16, winnt.h:19328:16, winnt.h:19328:16 */
pub type PTRANSACTION_PROPERTIES_INFORMATION = *mut ::winnt::TRANSACTION_PROPERTIES_INFORMATION; /* winnt.h:19336:40, winnt.h:19336:40, winnt.h:19336:40 */
#[repr(C)] pub struct TRANSACTION_BIND_INFORMATION { TmHandle: ::winnt::HANDLE } /* winnt.h:19340:16, winnt.h:19340:16, winnt.h:19340:16 */
pub type PTRANSACTION_BIND_INFORMATION = *mut ::winnt::TRANSACTION_BIND_INFORMATION; /* winnt.h:19342:34, winnt.h:19342:34, winnt.h:19342:34 */
#[repr(C)] pub struct TRANSACTION_ENLISTMENT_PAIR { EnlistmentId: ::guiddef::GUID, ResourceManagerId: ::guiddef::GUID } /* winnt.h:19344:16, winnt.h:19344:16, winnt.h:19344:16 */
pub type PTRANSACTION_ENLISTMENT_PAIR = *mut ::winnt::TRANSACTION_ENLISTMENT_PAIR; /* winnt.h:19347:33, winnt.h:19347:33, winnt.h:19347:33 */
#[repr(C)] pub struct TRANSACTION_ENLISTMENTS_INFORMATION { NumberOfEnlistments: ::minwindef::DWORD, EnlistmentPair: *mut [::winnt::TRANSACTION_ENLISTMENT_PAIR; 1] } /* winnt.h:19349:16, winnt.h:19349:16, winnt.h:19349:16 */
pub type PTRANSACTION_ENLISTMENTS_INFORMATION = *mut ::winnt::TRANSACTION_ENLISTMENTS_INFORMATION; /* winnt.h:19352:41, winnt.h:19352:41, winnt.h:19352:41 */
#[repr(C)] pub struct TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION { SuperiorEnlistmentPair: ::winnt::TRANSACTION_ENLISTMENT_PAIR } /* winnt.h:19354:16, winnt.h:19354:16, winnt.h:19354:16 */
pub type PTRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION = *mut ::winnt::TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION; /* winnt.h:19356:49, winnt.h:19356:49, winnt.h:19356:49 */
#[repr(C)] pub struct RESOURCEMANAGER_BASIC_INFORMATION { ResourceManagerId: ::guiddef::GUID, DescriptionLength: ::minwindef::DWORD, Description: *mut [::winnt::WCHAR; 1] } /* winnt.h:19359:16, winnt.h:19359:16, winnt.h:19359:16 */
pub type PRESOURCEMANAGER_BASIC_INFORMATION = *mut ::winnt::RESOURCEMANAGER_BASIC_INFORMATION; /* winnt.h:19363:39, winnt.h:19363:39, winnt.h:19363:39 */
#[repr(C)] pub struct RESOURCEMANAGER_COMPLETION_INFORMATION { IoCompletionPortHandle: ::winnt::HANDLE, CompletionKey: ::basetsd::ULONG_PTR } /* winnt.h:19365:16, winnt.h:19365:16, winnt.h:19365:16 */
pub type PRESOURCEMANAGER_COMPLETION_INFORMATION = *mut ::winnt::RESOURCEMANAGER_COMPLETION_INFORMATION; /* winnt.h:19368:44, winnt.h:19368:44, winnt.h:19368:44 */
#[repr(C)] pub enum TRANSACTION_INFORMATION_CLASS {TransactionBasicInformation = 0, TransactionPropertiesInformation = 1, TransactionEnlistmentInformation = 2, TransactionSuperiorEnlistmentInformation = 3, TransactionBindInformation = 4, TransactionDTCPrivateInformation = 5} pub use self::TRANSACTION_INFORMATION_CLASS::{TransactionBasicInformation, TransactionPropertiesInformation, TransactionEnlistmentInformation, TransactionSuperiorEnlistmentInformation, TransactionBindInformation, TransactionDTCPrivateInformation}; /* winnt.h:19373:14, winnt.h:19373:14, winnt.h:19373:14 */
#[repr(C)] pub enum TRANSACTIONMANAGER_INFORMATION_CLASS {TransactionManagerBasicInformation = 0, TransactionManagerLogInformation = 1, TransactionManagerLogPathInformation = 2, TransactionManagerRecoveryInformation = 4, TransactionManagerOnlineProbeInformation = 3, TransactionManagerOldestTransactionInformation = 5} pub use self::TRANSACTIONMANAGER_INFORMATION_CLASS::{TransactionManagerBasicInformation, TransactionManagerLogInformation, TransactionManagerLogPathInformation, TransactionManagerRecoveryInformation, TransactionManagerOnlineProbeInformation, TransactionManagerOldestTransactionInformation}; /* winnt.h:19389:14, winnt.h:19389:14, winnt.h:19389:14 */
#[repr(C)] pub enum RESOURCEMANAGER_INFORMATION_CLASS {ResourceManagerBasicInformation = 0, ResourceManagerCompletionInformation = 1} pub use self::RESOURCEMANAGER_INFORMATION_CLASS::{ResourceManagerBasicInformation, ResourceManagerCompletionInformation}; /* winnt.h:19408:14, winnt.h:19408:14, winnt.h:19408:14 */
#[repr(C)] pub struct ENLISTMENT_BASIC_INFORMATION { EnlistmentId: ::guiddef::GUID, TransactionId: ::guiddef::GUID, ResourceManagerId: ::guiddef::GUID } /* winnt.h:19414:16, winnt.h:19414:16, winnt.h:19414:16 */
pub type PENLISTMENT_BASIC_INFORMATION = *mut ::winnt::ENLISTMENT_BASIC_INFORMATION; /* winnt.h:19418:34, winnt.h:19418:34, winnt.h:19418:34 */
#[repr(C)] pub struct ENLISTMENT_CRM_INFORMATION { CrmTransactionManagerId: ::guiddef::GUID, CrmResourceManagerId: ::guiddef::GUID, CrmEnlistmentId: ::guiddef::GUID } /* winnt.h:19420:16, winnt.h:19420:16, winnt.h:19420:16 */
pub type PENLISTMENT_CRM_INFORMATION = *mut ::winnt::ENLISTMENT_CRM_INFORMATION; /* winnt.h:19424:32, winnt.h:19424:32, winnt.h:19424:32 */
#[repr(C)] pub enum ENLISTMENT_INFORMATION_CLASS {EnlistmentBasicInformation = 0, EnlistmentRecoveryInformation = 1, EnlistmentCrmInformation = 2} pub use self::ENLISTMENT_INFORMATION_CLASS::{EnlistmentBasicInformation, EnlistmentRecoveryInformation, EnlistmentCrmInformation}; /* winnt.h:19428:14, winnt.h:19428:14, winnt.h:19428:14 */
#[repr(C)] pub struct TRANSACTION_LIST_ENTRY { UOW: ::ktmtypes::UOW } /* winnt.h:19434:16, winnt.h:19434:16, winnt.h:19434:16 */
pub type PTRANSACTION_LIST_ENTRY = *mut ::winnt::TRANSACTION_LIST_ENTRY; /* winnt.h:19436:28, winnt.h:19436:28, winnt.h:19436:28 */
#[repr(C)] pub struct TRANSACTION_LIST_INFORMATION { NumberOfTransactions: ::minwindef::DWORD, TransactionInformation: *mut [::winnt::TRANSACTION_LIST_ENTRY; 1] } /* winnt.h:19438:16, winnt.h:19438:16, winnt.h:19438:16 */
pub type PTRANSACTION_LIST_INFORMATION = *mut ::winnt::TRANSACTION_LIST_INFORMATION; /* winnt.h:19441:34, winnt.h:19441:34, winnt.h:19441:34 */
#[repr(C)] pub enum KTMOBJECT_TYPE {KTMOBJECT_TRANSACTION = 0, KTMOBJECT_TRANSACTION_MANAGER = 1, KTMOBJECT_RESOURCE_MANAGER = 2, KTMOBJECT_ENLISTMENT = 3, KTMOBJECT_INVALID = 4} pub use self::KTMOBJECT_TYPE::{KTMOBJECT_TRANSACTION, KTMOBJECT_TRANSACTION_MANAGER, KTMOBJECT_RESOURCE_MANAGER, KTMOBJECT_ENLISTMENT, KTMOBJECT_INVALID}; /* winnt.h:19448:14, winnt.h:19448:14, winnt.h:19448:14 */
pub type PKTMOBJECT_TYPE = *mut ::winnt::KTMOBJECT_TYPE; /* winnt.h:19456:20, winnt.h:19456:20, winnt.h:19456:20 */
#[repr(C)] pub struct KTMOBJECT_CURSOR { LastQuery: ::guiddef::GUID, ObjectIdCount: ::minwindef::DWORD, ObjectIds: *mut [::guiddef::GUID; 1] } /* winnt.h:19466:16, winnt.h:19466:16, winnt.h:19466:16 */
pub type PKTMOBJECT_CURSOR = *mut ::winnt::KTMOBJECT_CURSOR; /* winnt.h:19486:22, winnt.h:19486:22, winnt.h:19486:22 */
pub type TP_VERSION = ::minwindef::DWORD; /* winnt.h:19495:15, winnt.h:19495:15, winnt.h:19495:15 */
pub type PTP_VERSION = *mut ::libc::c_ulong; /* winnt.h:19495:28, winnt.h:19495:28, winnt.h:19495:28 */
#[repr(C)] pub struct TP_CALLBACK_INSTANCE; /* winnt.h:19497:16, winnt.h:19497:16, winnt.h:19497:16 */
pub type PTP_CALLBACK_INSTANCE = *mut ::libc::c_void; /* winnt.h:19497:61, winnt.h:19497:61, winnt.h:19497:61 */
pub type PTP_SIMPLE_CALLBACK = extern "system" fn(*mut ::libc::c_void, *mut ::libc::c_void); /* winnt.h:19499:22, winnt.h:19499:22, winnt.h:19499:22 */
#[repr(C)] pub struct TP_POOL; /* winnt.h:19504:16, winnt.h:19504:16, winnt.h:19504:16 */
pub type PTP_POOL = *mut ::libc::c_void; /* winnt.h:19504:35, winnt.h:19504:35, winnt.h:19504:35 */
#[repr(C)] pub enum TP_CALLBACK_PRIORITY {TP_CALLBACK_PRIORITY_HIGH = 0, TP_CALLBACK_PRIORITY_NORMAL = 1, TP_CALLBACK_PRIORITY_LOW = 2, TP_CALLBACK_PRIORITY_INVALID = 3} pub use self::TP_CALLBACK_PRIORITY::{TP_CALLBACK_PRIORITY_HIGH, TP_CALLBACK_PRIORITY_NORMAL, TP_CALLBACK_PRIORITY_LOW, TP_CALLBACK_PRIORITY_INVALID}; pub const TP_CALLBACK_PRIORITY_COUNT: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY::TP_CALLBACK_PRIORITY_INVALID; /* winnt.h:19506:14, winnt.h:19506:14, winnt.h:19506:14 */
#[repr(C)] pub struct TP_POOL_STACK_INFORMATION { StackReserve: ::basetsd::SIZE_T, StackCommit: ::basetsd::SIZE_T } /* winnt.h:19514:16, winnt.h:19514:16, winnt.h:19514:16 */
pub type PTP_POOL_STACK_INFORMATION = *mut ::winnt::TP_POOL_STACK_INFORMATION; /* winnt.h:19517:30, winnt.h:19517:30, winnt.h:19517:30 */
#[repr(C)] pub struct TP_CLEANUP_GROUP; /* winnt.h:19519:16, winnt.h:19519:16, winnt.h:19519:16 */
pub type PTP_CLEANUP_GROUP = *mut ::libc::c_void; /* winnt.h:19519:53, winnt.h:19519:53, winnt.h:19519:53 */
pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK = extern "system" fn(*mut ::libc::c_void, *mut ::libc::c_void); /* winnt.h:19521:22, winnt.h:19521:22, winnt.h:19521:22 */
#[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct TP_CALLBACK_ENVIRON_V3 { Version: ::winnt::TP_VERSION, Pool: ::winnt::PTP_POOL, CleanupGroup: ::winnt::PTP_CLEANUP_GROUP, CleanupGroupCancelCallback: ::winnt::PTP_CLEANUP_GROUP_CANCEL_CALLBACK, RaceDll: ::winnt::PVOID, ActivationContext: *mut ::libc::c_void, FinalizationCallback: ::winnt::PTP_SIMPLE_CALLBACK, u: ::winnt::TP_CALLBACK_ENVIRON_V3_Child_8, CallbackPriority: ::winnt::TP_CALLBACK_PRIORITY, Size: ::minwindef::DWORD } /* winnt.h:19533:16, winnt.h:19533:16, winnt.h:19533:16 */
#[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub /*union*/ struct TP_CALLBACK_ENVIRON_V3_Child_8; /* STUB! */ /* winnt.h:19541:5, winnt.h:19541:5, winnt.h:19541:5 */
#[cfg(any(feature="winapi_ver_06010000"))] pub type TP_CALLBACK_ENVIRON = ::winnt::TP_CALLBACK_ENVIRON_V3; /* winnt.h:19553:32, winnt.h:19553:32, winnt.h:19553:32 */
#[cfg(any(feature="winapi_ver_06010000"))] pub type PTP_CALLBACK_ENVIRON = *mut ::winnt::TP_CALLBACK_ENVIRON_V3; /* winnt.h:19553:54, winnt.h:19553:54, winnt.h:19553:54 */
#[repr(C)] pub struct TP_WORK; /* winnt.h:19727:16, winnt.h:19727:16, winnt.h:19727:16 */
pub type PTP_WORK = *mut ::libc::c_void; /* winnt.h:19727:35, winnt.h:19727:35, winnt.h:19727:35 */
pub type PTP_WORK_CALLBACK = extern "system" fn(*mut ::libc::c_void, *mut ::libc::c_void, *mut ::libc::c_void); /* winnt.h:19729:22, winnt.h:19729:22, winnt.h:19729:22 */
#[repr(C)] pub struct TP_TIMER; /* winnt.h:19735:16, winnt.h:19735:16, winnt.h:19735:16 */
pub type PTP_TIMER = *mut ::libc::c_void; /* winnt.h:19735:37, winnt.h:19735:37, winnt.h:19735:37 */
pub type PTP_TIMER_CALLBACK = extern "system" fn(*mut ::libc::c_void, *mut ::libc::c_void, *mut ::libc::c_void); /* winnt.h:19737:22, winnt.h:19737:22, winnt.h:19737:22 */
pub type TP_WAIT_RESULT = ::minwindef::DWORD; /* winnt.h:19743:18, winnt.h:19743:18, winnt.h:19743:18 */
#[repr(C)] pub struct TP_WAIT; /* winnt.h:19745:16, winnt.h:19745:16, winnt.h:19745:16 */
pub type PTP_WAIT = *mut ::libc::c_void; /* winnt.h:19745:35, winnt.h:19745:35, winnt.h:19745:35 */
pub type PTP_WAIT_CALLBACK = extern "system" fn(*mut ::libc::c_void, *mut ::libc::c_void, *mut ::libc::c_void, ::libc::c_ulong); /* winnt.h:19747:22, winnt.h:19747:22, winnt.h:19747:22 */
#[repr(C)] pub struct TP_IO; /* winnt.h:19754:16, winnt.h:19754:16, winnt.h:19754:16 */
pub type PTP_IO = *mut ::libc::c_void; /* winnt.h:19754:31, winnt.h:19754:31, winnt.h:19754:31 */
pub const ANYSIZE_ARRAY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:33:9, winnt.h:33:9, winnt.h:33:9 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub const MEMORY_ALLOCATION_ALIGNMENT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:119:9, winnt.h:119:9 */
#[cfg(any(target_arch="x86", target_arch="x86_64"))] pub const SYSTEM_CACHE_ALIGNMENT_SIZE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:208:9, winnt.h:208:9 */
pub const PRAGMA_DEPRECATED_DDK: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:332:9, winnt.h:332:9, winnt.h:332:9 */
pub const ALL_PROCESSOR_GROUPS: i32 = 0xffffi32; /* Integer(65535, Yes, Unknown) */ /* winnt.h:542:9, winnt.h:542:9, winnt.h:542:9 */
pub const APPLICATION_ERROR_MASK: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* winnt.h:686:9, winnt.h:686:9, winnt.h:686:9 */
pub const ERROR_SEVERITY_SUCCESS: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:687:9, winnt.h:687:9, winnt.h:687:9 */
pub const ERROR_SEVERITY_INFORMATIONAL: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* winnt.h:688:9, winnt.h:688:9, winnt.h:688:9 */
pub const ERROR_SEVERITY_WARNING: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:689:9, winnt.h:689:9, winnt.h:689:9 */
pub const ERROR_SEVERITY_ERROR: i32 = 0xc0000000i32; /* Integer(3221225472, Yes, Unknown) */ /* winnt.h:690:9, winnt.h:690:9, winnt.h:690:9 */
pub const MINCHAR: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:1094:9, winnt.h:1094:9, winnt.h:1094:9 */
pub const MAXCHAR: i32 = 0x7fi32; /* Integer(127, Yes, Unknown) */ /* winnt.h:1095:9, winnt.h:1095:9, winnt.h:1095:9 */
pub const MINSHORT: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:1096:9, winnt.h:1096:9, winnt.h:1096:9 */
pub const MAXSHORT: i32 = 0x7fffi32; /* Integer(32767, Yes, Unknown) */ /* winnt.h:1097:9, winnt.h:1097:9, winnt.h:1097:9 */
pub const MINLONG: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:1098:9, winnt.h:1098:9, winnt.h:1098:9 */
pub const MAXLONG: i32 = 0x7fffffffi32; /* Integer(2147483647, Yes, Unknown) */ /* winnt.h:1099:9, winnt.h:1099:9, winnt.h:1099:9 */
pub const MAXBYTE: i32 = 0xffi32; /* Integer(255, Yes, Unknown) */ /* winnt.h:1100:9, winnt.h:1100:9, winnt.h:1100:9 */
pub const MAXWORD: i32 = 0xffffi32; /* Integer(65535, Yes, Unknown) */ /* winnt.h:1101:9, winnt.h:1101:9, winnt.h:1101:9 */
pub const MAXDWORD: i32 = 0xffffffffi32; /* Integer(4294967295, Yes, Unknown) */ /* winnt.h:1102:9, winnt.h:1102:9, winnt.h:1102:9 */
pub const VER_SERVER_NT: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:1321:9, winnt.h:1321:9, winnt.h:1321:9 */
pub const VER_WORKSTATION_NT: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* winnt.h:1322:9, winnt.h:1322:9, winnt.h:1322:9 */
pub const VER_SUITE_SMALLBUSINESS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1323:9, winnt.h:1323:9, winnt.h:1323:9 */
pub const VER_SUITE_ENTERPRISE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1324:9, winnt.h:1324:9, winnt.h:1324:9 */
pub const VER_SUITE_BACKOFFICE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1325:9, winnt.h:1325:9, winnt.h:1325:9 */
pub const VER_SUITE_COMMUNICATIONS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:1326:9, winnt.h:1326:9, winnt.h:1326:9 */
pub const VER_SUITE_TERMINAL: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:1327:9, winnt.h:1327:9, winnt.h:1327:9 */
pub const VER_SUITE_SMALLBUSINESS_RESTRICTED: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:1328:9, winnt.h:1328:9, winnt.h:1328:9 */
pub const VER_SUITE_EMBEDDEDNT: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:1329:9, winnt.h:1329:9, winnt.h:1329:9 */
pub const VER_SUITE_DATACENTER: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:1330:9, winnt.h:1330:9, winnt.h:1330:9 */
pub const VER_SUITE_SINGLEUSERTS: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:1331:9, winnt.h:1331:9, winnt.h:1331:9 */
pub const VER_SUITE_PERSONAL: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:1332:9, winnt.h:1332:9, winnt.h:1332:9 */
pub const VER_SUITE_BLADE: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnt.h:1333:9, winnt.h:1333:9, winnt.h:1333:9 */
pub const VER_SUITE_EMBEDDED_RESTRICTED: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winnt.h:1334:9, winnt.h:1334:9, winnt.h:1334:9 */
pub const VER_SUITE_SECURITY_APPLIANCE: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winnt.h:1335:9, winnt.h:1335:9, winnt.h:1335:9 */
pub const VER_SUITE_STORAGE_SERVER: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winnt.h:1336:9, winnt.h:1336:9, winnt.h:1336:9 */
pub const VER_SUITE_COMPUTE_SERVER: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winnt.h:1337:9, winnt.h:1337:9, winnt.h:1337:9 */
pub const VER_SUITE_WH_SERVER: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:1338:9, winnt.h:1338:9, winnt.h:1338:9 */
pub const PRODUCT_UNDEFINED: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:1355:9, winnt.h:1355:9, winnt.h:1355:9 */
pub const PRODUCT_ULTIMATE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1357:9, winnt.h:1357:9, winnt.h:1357:9 */
pub const PRODUCT_HOME_BASIC: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1358:9, winnt.h:1358:9, winnt.h:1358:9 */
pub const PRODUCT_HOME_PREMIUM: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:1359:9, winnt.h:1359:9, winnt.h:1359:9 */
pub const PRODUCT_ENTERPRISE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1360:9, winnt.h:1360:9, winnt.h:1360:9 */
pub const PRODUCT_HOME_BASIC_N: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:1361:9, winnt.h:1361:9, winnt.h:1361:9 */
pub const PRODUCT_BUSINESS: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:1362:9, winnt.h:1362:9, winnt.h:1362:9 */
pub const PRODUCT_STANDARD_SERVER: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:1363:9, winnt.h:1363:9, winnt.h:1363:9 */
pub const PRODUCT_DATACENTER_SERVER: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:1364:9, winnt.h:1364:9, winnt.h:1364:9 */
pub const PRODUCT_SMALLBUSINESS_SERVER: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:1365:9, winnt.h:1365:9, winnt.h:1365:9 */
pub const PRODUCT_ENTERPRISE_SERVER: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:1366:9, winnt.h:1366:9, winnt.h:1366:9 */
pub const PRODUCT_STARTER: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:1367:9, winnt.h:1367:9, winnt.h:1367:9 */
pub const PRODUCT_DATACENTER_SERVER_CORE: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:1368:9, winnt.h:1368:9, winnt.h:1368:9 */
pub const PRODUCT_STANDARD_SERVER_CORE: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:1369:9, winnt.h:1369:9, winnt.h:1369:9 */
pub const PRODUCT_ENTERPRISE_SERVER_CORE: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:1370:9, winnt.h:1370:9, winnt.h:1370:9 */
pub const PRODUCT_ENTERPRISE_SERVER_IA64: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:1371:9, winnt.h:1371:9, winnt.h:1371:9 */
pub const PRODUCT_BUSINESS_N: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:1372:9, winnt.h:1372:9, winnt.h:1372:9 */
pub const PRODUCT_WEB_SERVER: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winnt.h:1373:9, winnt.h:1373:9, winnt.h:1373:9 */
pub const PRODUCT_CLUSTER_SERVER: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:1374:9, winnt.h:1374:9, winnt.h:1374:9 */
pub const PRODUCT_HOME_SERVER: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winnt.h:1375:9, winnt.h:1375:9, winnt.h:1375:9 */
pub const PRODUCT_STORAGE_EXPRESS_SERVER: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnt.h:1376:9, winnt.h:1376:9, winnt.h:1376:9 */
pub const PRODUCT_STORAGE_STANDARD_SERVER: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winnt.h:1377:9, winnt.h:1377:9, winnt.h:1377:9 */
pub const PRODUCT_STORAGE_WORKGROUP_SERVER: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winnt.h:1378:9, winnt.h:1378:9, winnt.h:1378:9 */
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winnt.h:1379:9, winnt.h:1379:9, winnt.h:1379:9 */
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winnt.h:1380:9, winnt.h:1380:9, winnt.h:1380:9 */
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* winnt.h:1381:9, winnt.h:1381:9, winnt.h:1381:9 */
pub const PRODUCT_HOME_PREMIUM_N: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* winnt.h:1382:9, winnt.h:1382:9, winnt.h:1382:9 */
pub const PRODUCT_ENTERPRISE_N: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* winnt.h:1383:9, winnt.h:1383:9, winnt.h:1383:9 */
pub const PRODUCT_ULTIMATE_N: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* winnt.h:1384:9, winnt.h:1384:9, winnt.h:1384:9 */
pub const PRODUCT_WEB_SERVER_CORE: i32 = 0x1di32; /* Integer(29, Yes, Unknown) */ /* winnt.h:1385:9, winnt.h:1385:9, winnt.h:1385:9 */
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* winnt.h:1386:9, winnt.h:1386:9, winnt.h:1386:9 */
pub const PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* winnt.h:1387:9, winnt.h:1387:9, winnt.h:1387:9 */
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:1388:9, winnt.h:1388:9, winnt.h:1388:9 */
pub const PRODUCT_SERVER_FOUNDATION: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* winnt.h:1389:9, winnt.h:1389:9, winnt.h:1389:9 */
pub const PRODUCT_HOME_PREMIUM_SERVER: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* winnt.h:1390:9, winnt.h:1390:9, winnt.h:1390:9 */
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS_V: i32 = 0x23i32; /* Integer(35, Yes, Unknown) */ /* winnt.h:1391:9, winnt.h:1391:9, winnt.h:1391:9 */
pub const PRODUCT_STANDARD_SERVER_V: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* winnt.h:1392:9, winnt.h:1392:9, winnt.h:1392:9 */
pub const PRODUCT_DATACENTER_SERVER_V: i32 = 0x25i32; /* Integer(37, Yes, Unknown) */ /* winnt.h:1393:9, winnt.h:1393:9, winnt.h:1393:9 */
pub const PRODUCT_ENTERPRISE_SERVER_V: i32 = 0x26i32; /* Integer(38, Yes, Unknown) */ /* winnt.h:1394:9, winnt.h:1394:9, winnt.h:1394:9 */
pub const PRODUCT_DATACENTER_SERVER_CORE_V: i32 = 0x27i32; /* Integer(39, Yes, Unknown) */ /* winnt.h:1395:9, winnt.h:1395:9, winnt.h:1395:9 */
pub const PRODUCT_STANDARD_SERVER_CORE_V: i32 = 0x28i32; /* Integer(40, Yes, Unknown) */ /* winnt.h:1396:9, winnt.h:1396:9, winnt.h:1396:9 */
pub const PRODUCT_ENTERPRISE_SERVER_CORE_V: i32 = 0x29i32; /* Integer(41, Yes, Unknown) */ /* winnt.h:1397:9, winnt.h:1397:9, winnt.h:1397:9 */
pub const PRODUCT_HYPERV: i32 = 0x2ai32; /* Integer(42, Yes, Unknown) */ /* winnt.h:1398:9, winnt.h:1398:9, winnt.h:1398:9 */
pub const PRODUCT_STORAGE_EXPRESS_SERVER_CORE: i32 = 0x2bi32; /* Integer(43, Yes, Unknown) */ /* winnt.h:1399:9, winnt.h:1399:9, winnt.h:1399:9 */
pub const PRODUCT_STORAGE_STANDARD_SERVER_CORE: i32 = 0x2ci32; /* Integer(44, Yes, Unknown) */ /* winnt.h:1400:9, winnt.h:1400:9, winnt.h:1400:9 */
pub const PRODUCT_STORAGE_WORKGROUP_SERVER_CORE: i32 = 0x2di32; /* Integer(45, Yes, Unknown) */ /* winnt.h:1401:9, winnt.h:1401:9, winnt.h:1401:9 */
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE: i32 = 0x2ei32; /* Integer(46, Yes, Unknown) */ /* winnt.h:1402:9, winnt.h:1402:9, winnt.h:1402:9 */
pub const PRODUCT_STARTER_N: i32 = 0x2fi32; /* Integer(47, Yes, Unknown) */ /* winnt.h:1403:9, winnt.h:1403:9, winnt.h:1403:9 */
pub const PRODUCT_PROFESSIONAL: i32 = 0x30i32; /* Integer(48, Yes, Unknown) */ /* winnt.h:1404:9, winnt.h:1404:9, winnt.h:1404:9 */
pub const PRODUCT_PROFESSIONAL_N: i32 = 0x31i32; /* Integer(49, Yes, Unknown) */ /* winnt.h:1405:9, winnt.h:1405:9, winnt.h:1405:9 */
pub const PRODUCT_SB_SOLUTION_SERVER: i32 = 0x32i32; /* Integer(50, Yes, Unknown) */ /* winnt.h:1406:9, winnt.h:1406:9, winnt.h:1406:9 */
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS: i32 = 0x33i32; /* Integer(51, Yes, Unknown) */ /* winnt.h:1407:9, winnt.h:1407:9, winnt.h:1407:9 */
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS: i32 = 0x34i32; /* Integer(52, Yes, Unknown) */ /* winnt.h:1408:9, winnt.h:1408:9, winnt.h:1408:9 */
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE: i32 = 0x35i32; /* Integer(53, Yes, Unknown) */ /* winnt.h:1409:9, winnt.h:1409:9, winnt.h:1409:9 */
pub const PRODUCT_SB_SOLUTION_SERVER_EM: i32 = 0x36i32; /* Integer(54, Yes, Unknown) */ /* winnt.h:1410:9, winnt.h:1410:9, winnt.h:1410:9 */
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM: i32 = 0x37i32; /* Integer(55, Yes, Unknown) */ /* winnt.h:1411:9, winnt.h:1411:9, winnt.h:1411:9 */
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER: i32 = 0x38i32; /* Integer(56, Yes, Unknown) */ /* winnt.h:1412:9, winnt.h:1412:9, winnt.h:1412:9 */
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER_CORE: i32 = 0x39i32; /* Integer(57, Yes, Unknown) */ /* winnt.h:1413:9, winnt.h:1413:9, winnt.h:1413:9 */
pub const PRODUCT_PROFESSIONAL_EMBEDDED: i32 = 0x3ai32; /* Integer(58, Yes, Unknown) */ /* winnt.h:1414:9, winnt.h:1414:9, winnt.h:1414:9 */
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT: i32 = 0x3bi32; /* Integer(59, Yes, Unknown) */ /* winnt.h:1415:9, winnt.h:1415:9, winnt.h:1415:9 */
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL: i32 = 0x3ci32; /* Integer(60, Yes, Unknown) */ /* winnt.h:1416:9, winnt.h:1416:9, winnt.h:1416:9 */
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC: i32 = 0x3di32; /* Integer(61, Yes, Unknown) */ /* winnt.h:1417:9, winnt.h:1417:9, winnt.h:1417:9 */
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC: i32 = 0x3ei32; /* Integer(62, Yes, Unknown) */ /* winnt.h:1418:9, winnt.h:1418:9, winnt.h:1418:9 */
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE: i32 = 0x3fi32; /* Integer(63, Yes, Unknown) */ /* winnt.h:1419:9, winnt.h:1419:9, winnt.h:1419:9 */
pub const PRODUCT_CLUSTER_SERVER_V: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:1420:9, winnt.h:1420:9, winnt.h:1420:9 */
pub const PRODUCT_EMBEDDED: i32 = 0x41i32; /* Integer(65, Yes, Unknown) */ /* winnt.h:1421:9, winnt.h:1421:9, winnt.h:1421:9 */
pub const PRODUCT_STARTER_E: i32 = 0x42i32; /* Integer(66, Yes, Unknown) */ /* winnt.h:1422:9, winnt.h:1422:9, winnt.h:1422:9 */
pub const PRODUCT_HOME_BASIC_E: i32 = 0x43i32; /* Integer(67, Yes, Unknown) */ /* winnt.h:1423:9, winnt.h:1423:9, winnt.h:1423:9 */
pub const PRODUCT_HOME_PREMIUM_E: i32 = 0x44i32; /* Integer(68, Yes, Unknown) */ /* winnt.h:1424:9, winnt.h:1424:9, winnt.h:1424:9 */
pub const PRODUCT_PROFESSIONAL_E: i32 = 0x45i32; /* Integer(69, Yes, Unknown) */ /* winnt.h:1425:9, winnt.h:1425:9, winnt.h:1425:9 */
pub const PRODUCT_ENTERPRISE_E: i32 = 0x46i32; /* Integer(70, Yes, Unknown) */ /* winnt.h:1426:9, winnt.h:1426:9, winnt.h:1426:9 */
pub const PRODUCT_ULTIMATE_E: i32 = 0x47i32; /* Integer(71, Yes, Unknown) */ /* winnt.h:1427:9, winnt.h:1427:9, winnt.h:1427:9 */
pub const PRODUCT_ENTERPRISE_EVALUATION: i32 = 0x48i32; /* Integer(72, Yes, Unknown) */ /* winnt.h:1428:9, winnt.h:1428:9, winnt.h:1428:9 */
pub const PRODUCT_MULTIPOINT_STANDARD_SERVER: i32 = 0x4ci32; /* Integer(76, Yes, Unknown) */ /* winnt.h:1429:9, winnt.h:1429:9, winnt.h:1429:9 */
pub const PRODUCT_MULTIPOINT_PREMIUM_SERVER: i32 = 0x4di32; /* Integer(77, Yes, Unknown) */ /* winnt.h:1430:9, winnt.h:1430:9, winnt.h:1430:9 */
pub const PRODUCT_STANDARD_EVALUATION_SERVER: i32 = 0x4fi32; /* Integer(79, Yes, Unknown) */ /* winnt.h:1431:9, winnt.h:1431:9, winnt.h:1431:9 */
pub const PRODUCT_DATACENTER_EVALUATION_SERVER: i32 = 0x50i32; /* Integer(80, Yes, Unknown) */ /* winnt.h:1432:9, winnt.h:1432:9, winnt.h:1432:9 */
pub const PRODUCT_ENTERPRISE_N_EVALUATION: i32 = 0x54i32; /* Integer(84, Yes, Unknown) */ /* winnt.h:1433:9, winnt.h:1433:9, winnt.h:1433:9 */
pub const PRODUCT_EMBEDDED_AUTOMOTIVE: i32 = 0x55i32; /* Integer(85, Yes, Unknown) */ /* winnt.h:1434:9, winnt.h:1434:9, winnt.h:1434:9 */
pub const PRODUCT_EMBEDDED_INDUSTRY_A: i32 = 0x56i32; /* Integer(86, Yes, Unknown) */ /* winnt.h:1435:9, winnt.h:1435:9, winnt.h:1435:9 */
pub const PRODUCT_THINPC: i32 = 0x57i32; /* Integer(87, Yes, Unknown) */ /* winnt.h:1436:9, winnt.h:1436:9, winnt.h:1436:9 */
pub const PRODUCT_EMBEDDED_A: i32 = 0x58i32; /* Integer(88, Yes, Unknown) */ /* winnt.h:1437:9, winnt.h:1437:9, winnt.h:1437:9 */
pub const PRODUCT_EMBEDDED_INDUSTRY: i32 = 0x59i32; /* Integer(89, Yes, Unknown) */ /* winnt.h:1438:9, winnt.h:1438:9, winnt.h:1438:9 */
pub const PRODUCT_EMBEDDED_E: i32 = 0x5ai32; /* Integer(90, Yes, Unknown) */ /* winnt.h:1439:9, winnt.h:1439:9, winnt.h:1439:9 */
pub const PRODUCT_EMBEDDED_INDUSTRY_E: i32 = 0x5bi32; /* Integer(91, Yes, Unknown) */ /* winnt.h:1440:9, winnt.h:1440:9, winnt.h:1440:9 */
pub const PRODUCT_EMBEDDED_INDUSTRY_A_E: i32 = 0x5ci32; /* Integer(92, Yes, Unknown) */ /* winnt.h:1441:9, winnt.h:1441:9, winnt.h:1441:9 */
pub const PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER: i32 = 0x5fi32; /* Integer(95, Yes, Unknown) */ /* winnt.h:1442:9, winnt.h:1442:9, winnt.h:1442:9 */
pub const PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER: i32 = 0x60i32; /* Integer(96, Yes, Unknown) */ /* winnt.h:1443:9, winnt.h:1443:9, winnt.h:1443:9 */
pub const PRODUCT_CORE_ARM: i32 = 0x61i32; /* Integer(97, Yes, Unknown) */ /* winnt.h:1444:9, winnt.h:1444:9, winnt.h:1444:9 */
pub const PRODUCT_CORE_N: i32 = 0x62i32; /* Integer(98, Yes, Unknown) */ /* winnt.h:1445:9, winnt.h:1445:9, winnt.h:1445:9 */
pub const PRODUCT_CORE_COUNTRYSPECIFIC: i32 = 0x63i32; /* Integer(99, Yes, Unknown) */ /* winnt.h:1446:9, winnt.h:1446:9, winnt.h:1446:9 */
pub const PRODUCT_CORE_SINGLELANGUAGE: i32 = 0x64i32; /* Integer(100, Yes, Unknown) */ /* winnt.h:1447:9, winnt.h:1447:9, winnt.h:1447:9 */
pub const PRODUCT_CORE: i32 = 0x65i32; /* Integer(101, Yes, Unknown) */ /* winnt.h:1448:9, winnt.h:1448:9, winnt.h:1448:9 */
pub const PRODUCT_PROFESSIONAL_WMC: i32 = 0x67i32; /* Integer(103, Yes, Unknown) */ /* winnt.h:1449:9, winnt.h:1449:9, winnt.h:1449:9 */
pub const PRODUCT_MOBILE_CORE: i32 = 0x68i32; /* Integer(104, Yes, Unknown) */ /* winnt.h:1450:9, winnt.h:1450:9, winnt.h:1450:9 */
pub const PRODUCT_EMBEDDED_INDUSTRY_EVAL: i32 = 0x69i32; /* Integer(105, Yes, Unknown) */ /* winnt.h:1451:9, winnt.h:1451:9, winnt.h:1451:9 */
pub const PRODUCT_EMBEDDED_INDUSTRY_E_EVAL: i32 = 0x6ai32; /* Integer(106, Yes, Unknown) */ /* winnt.h:1452:9, winnt.h:1452:9, winnt.h:1452:9 */
pub const PRODUCT_EMBEDDED_EVAL: i32 = 0x6bi32; /* Integer(107, Yes, Unknown) */ /* winnt.h:1453:9, winnt.h:1453:9, winnt.h:1453:9 */
pub const PRODUCT_EMBEDDED_E_EVAL: i32 = 0x6ci32; /* Integer(108, Yes, Unknown) */ /* winnt.h:1454:9, winnt.h:1454:9, winnt.h:1454:9 */
pub const PRODUCT_CORE_SERVER: i32 = 0x6di32; /* Integer(109, Yes, Unknown) */ /* winnt.h:1455:9, winnt.h:1455:9, winnt.h:1455:9 */
pub const PRODUCT_CLOUD_STORAGE_SERVER: i32 = 0x6ei32; /* Integer(110, Yes, Unknown) */ /* winnt.h:1456:9, winnt.h:1456:9, winnt.h:1456:9 */
pub const PRODUCT_CORE_CONNECTED: i32 = 0x6fi32; /* Integer(111, Yes, Unknown) */ /* winnt.h:1457:9, winnt.h:1457:9, winnt.h:1457:9 */
pub const PRODUCT_PROFESSIONAL_STUDENT: i32 = 0x70i32; /* Integer(112, Yes, Unknown) */ /* winnt.h:1458:9, winnt.h:1458:9, winnt.h:1458:9 */
pub const PRODUCT_CORE_CONNECTED_N: i32 = 0x71i32; /* Integer(113, Yes, Unknown) */ /* winnt.h:1459:9, winnt.h:1459:9, winnt.h:1459:9 */
pub const PRODUCT_PROFESSIONAL_STUDENT_N: i32 = 0x72i32; /* Integer(114, Yes, Unknown) */ /* winnt.h:1460:9, winnt.h:1460:9, winnt.h:1460:9 */
pub const PRODUCT_CORE_CONNECTED_SINGLELANGUAGE: i32 = 0x73i32; /* Integer(115, Yes, Unknown) */ /* winnt.h:1461:9, winnt.h:1461:9, winnt.h:1461:9 */
pub const PRODUCT_CORE_CONNECTED_COUNTRYSPECIFIC: i32 = 0x74i32; /* Integer(116, Yes, Unknown) */ /* winnt.h:1462:9, winnt.h:1462:9, winnt.h:1462:9 */
pub const PRODUCT_UNLICENSED: i32 = 0xabcdabcdi32; /* Integer(2882382797, Yes, Unknown) */ /* winnt.h:1464:9, winnt.h:1464:9, winnt.h:1464:9 */
pub const LANG_NEUTRAL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:1498:9, winnt.h:1498:9, winnt.h:1498:9 */
pub const LANG_INVARIANT: i32 = 0x7fi32; /* Integer(127, Yes, Unknown) */ /* winnt.h:1499:9, winnt.h:1499:9, winnt.h:1499:9 */
pub const LANG_AFRIKAANS: i32 = 0x36i32; /* Integer(54, Yes, Unknown) */ /* winnt.h:1501:9, winnt.h:1501:9, winnt.h:1501:9 */
pub const LANG_ALBANIAN: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* winnt.h:1502:9, winnt.h:1502:9, winnt.h:1502:9 */
pub const LANG_ALSATIAN: i32 = 0x84i32; /* Integer(132, Yes, Unknown) */ /* winnt.h:1503:9, winnt.h:1503:9, winnt.h:1503:9 */
pub const LANG_AMHARIC: i32 = 0x5ei32; /* Integer(94, Yes, Unknown) */ /* winnt.h:1504:9, winnt.h:1504:9, winnt.h:1504:9 */
pub const LANG_ARABIC: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1505:9, winnt.h:1505:9, winnt.h:1505:9 */
pub const LANG_ARMENIAN: i32 = 0x2bi32; /* Integer(43, Yes, Unknown) */ /* winnt.h:1506:9, winnt.h:1506:9, winnt.h:1506:9 */
pub const LANG_ASSAMESE: i32 = 0x4di32; /* Integer(77, Yes, Unknown) */ /* winnt.h:1507:9, winnt.h:1507:9, winnt.h:1507:9 */
pub const LANG_AZERI: i32 = 0x2ci32; /* Integer(44, Yes, Unknown) */ /* winnt.h:1508:9, winnt.h:1508:9, winnt.h:1508:9 */
pub const LANG_AZERBAIJANI: i32 = 0x2ci32; /* Integer(44, Yes, Unknown) */ /* winnt.h:1509:9, winnt.h:1509:9, winnt.h:1509:9 */
pub const LANG_BANGLA: i32 = 0x45i32; /* Integer(69, Yes, Unknown) */ /* winnt.h:1510:9, winnt.h:1510:9, winnt.h:1510:9 */
pub const LANG_BASHKIR: i32 = 0x6di32; /* Integer(109, Yes, Unknown) */ /* winnt.h:1511:9, winnt.h:1511:9, winnt.h:1511:9 */
pub const LANG_BASQUE: i32 = 0x2di32; /* Integer(45, Yes, Unknown) */ /* winnt.h:1512:9, winnt.h:1512:9, winnt.h:1512:9 */
pub const LANG_BELARUSIAN: i32 = 0x23i32; /* Integer(35, Yes, Unknown) */ /* winnt.h:1513:9, winnt.h:1513:9, winnt.h:1513:9 */
pub const LANG_BENGALI: i32 = 0x45i32; /* Integer(69, Yes, Unknown) */ /* winnt.h:1514:9, winnt.h:1514:9, winnt.h:1514:9 */
pub const LANG_BRETON: i32 = 0x7ei32; /* Integer(126, Yes, Unknown) */ /* winnt.h:1515:9, winnt.h:1515:9, winnt.h:1515:9 */
pub const LANG_BOSNIAN: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* winnt.h:1516:9, winnt.h:1516:9, winnt.h:1516:9 */
pub const LANG_BOSNIAN_NEUTRAL: i32 = 0x781ai32; /* Integer(30746, Yes, Unknown) */ /* winnt.h:1517:9, winnt.h:1517:9, winnt.h:1517:9 */
pub const LANG_BULGARIAN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1518:9, winnt.h:1518:9, winnt.h:1518:9 */
pub const LANG_CATALAN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:1519:9, winnt.h:1519:9, winnt.h:1519:9 */
pub const LANG_CENTRAL_KURDISH: i32 = 0x92i32; /* Integer(146, Yes, Unknown) */ /* winnt.h:1520:9, winnt.h:1520:9, winnt.h:1520:9 */
pub const LANG_CHEROKEE: i32 = 0x5ci32; /* Integer(92, Yes, Unknown) */ /* winnt.h:1521:9, winnt.h:1521:9, winnt.h:1521:9 */
pub const LANG_CHINESE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1522:9, winnt.h:1522:9, winnt.h:1522:9 */
pub const LANG_CHINESE_SIMPLIFIED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1523:9, winnt.h:1523:9, winnt.h:1523:9 */
pub const LANG_CHINESE_TRADITIONAL: i32 = 0x7c04i32; /* Integer(31748, Yes, Unknown) */ /* winnt.h:1524:9, winnt.h:1524:9, winnt.h:1524:9 */
pub const LANG_CORSICAN: i32 = 0x83i32; /* Integer(131, Yes, Unknown) */ /* winnt.h:1525:9, winnt.h:1525:9, winnt.h:1525:9 */
pub const LANG_CROATIAN: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* winnt.h:1526:9, winnt.h:1526:9, winnt.h:1526:9 */
pub const LANG_CZECH: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:1527:9, winnt.h:1527:9, winnt.h:1527:9 */
pub const LANG_DANISH: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:1528:9, winnt.h:1528:9, winnt.h:1528:9 */
pub const LANG_DARI: i32 = 0x8ci32; /* Integer(140, Yes, Unknown) */ /* winnt.h:1529:9, winnt.h:1529:9, winnt.h:1529:9 */
pub const LANG_DIVEHI: i32 = 0x65i32; /* Integer(101, Yes, Unknown) */ /* winnt.h:1530:9, winnt.h:1530:9, winnt.h:1530:9 */
pub const LANG_DUTCH: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winnt.h:1531:9, winnt.h:1531:9, winnt.h:1531:9 */
pub const LANG_ENGLISH: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:1532:9, winnt.h:1532:9, winnt.h:1532:9 */
pub const LANG_ESTONIAN: i32 = 0x25i32; /* Integer(37, Yes, Unknown) */ /* winnt.h:1533:9, winnt.h:1533:9, winnt.h:1533:9 */
pub const LANG_FAEROESE: i32 = 0x38i32; /* Integer(56, Yes, Unknown) */ /* winnt.h:1534:9, winnt.h:1534:9, winnt.h:1534:9 */
pub const LANG_FARSI: i32 = 0x29i32; /* Integer(41, Yes, Unknown) */ /* winnt.h:1535:9, winnt.h:1535:9, winnt.h:1535:9 */
pub const LANG_FILIPINO: i32 = 0x64i32; /* Integer(100, Yes, Unknown) */ /* winnt.h:1536:9, winnt.h:1536:9, winnt.h:1536:9 */
pub const LANG_FINNISH: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:1537:9, winnt.h:1537:9, winnt.h:1537:9 */
pub const LANG_FRENCH: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:1538:9, winnt.h:1538:9, winnt.h:1538:9 */
pub const LANG_FRISIAN: i32 = 0x62i32; /* Integer(98, Yes, Unknown) */ /* winnt.h:1539:9, winnt.h:1539:9, winnt.h:1539:9 */
pub const LANG_FULAH: i32 = 0x67i32; /* Integer(103, Yes, Unknown) */ /* winnt.h:1540:9, winnt.h:1540:9, winnt.h:1540:9 */
pub const LANG_GALICIAN: i32 = 0x56i32; /* Integer(86, Yes, Unknown) */ /* winnt.h:1541:9, winnt.h:1541:9, winnt.h:1541:9 */
pub const LANG_GEORGIAN: i32 = 0x37i32; /* Integer(55, Yes, Unknown) */ /* winnt.h:1542:9, winnt.h:1542:9, winnt.h:1542:9 */
pub const LANG_GERMAN: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:1543:9, winnt.h:1543:9, winnt.h:1543:9 */
pub const LANG_GREEK: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:1544:9, winnt.h:1544:9, winnt.h:1544:9 */
pub const LANG_GREENLANDIC: i32 = 0x6fi32; /* Integer(111, Yes, Unknown) */ /* winnt.h:1545:9, winnt.h:1545:9, winnt.h:1545:9 */
pub const LANG_GUJARATI: i32 = 0x47i32; /* Integer(71, Yes, Unknown) */ /* winnt.h:1546:9, winnt.h:1546:9, winnt.h:1546:9 */
pub const LANG_HAUSA: i32 = 0x68i32; /* Integer(104, Yes, Unknown) */ /* winnt.h:1547:9, winnt.h:1547:9, winnt.h:1547:9 */
pub const LANG_HAWAIIAN: i32 = 0x75i32; /* Integer(117, Yes, Unknown) */ /* winnt.h:1548:9, winnt.h:1548:9, winnt.h:1548:9 */
pub const LANG_HEBREW: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:1549:9, winnt.h:1549:9, winnt.h:1549:9 */
pub const LANG_HINDI: i32 = 0x39i32; /* Integer(57, Yes, Unknown) */ /* winnt.h:1550:9, winnt.h:1550:9, winnt.h:1550:9 */
pub const LANG_HUNGARIAN: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:1551:9, winnt.h:1551:9, winnt.h:1551:9 */
pub const LANG_ICELANDIC: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:1552:9, winnt.h:1552:9, winnt.h:1552:9 */
pub const LANG_IGBO: i32 = 0x70i32; /* Integer(112, Yes, Unknown) */ /* winnt.h:1553:9, winnt.h:1553:9, winnt.h:1553:9 */
pub const LANG_INDONESIAN: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* winnt.h:1554:9, winnt.h:1554:9, winnt.h:1554:9 */
pub const LANG_INUKTITUT: i32 = 0x5di32; /* Integer(93, Yes, Unknown) */ /* winnt.h:1555:9, winnt.h:1555:9, winnt.h:1555:9 */
pub const LANG_IRISH: i32 = 0x3ci32; /* Integer(60, Yes, Unknown) */ /* winnt.h:1556:9, winnt.h:1556:9, winnt.h:1556:9 */
pub const LANG_ITALIAN: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:1557:9, winnt.h:1557:9, winnt.h:1557:9 */
pub const LANG_JAPANESE: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winnt.h:1558:9, winnt.h:1558:9, winnt.h:1558:9 */
pub const LANG_KANNADA: i32 = 0x4bi32; /* Integer(75, Yes, Unknown) */ /* winnt.h:1559:9, winnt.h:1559:9, winnt.h:1559:9 */
pub const LANG_KASHMIRI: i32 = 0x60i32; /* Integer(96, Yes, Unknown) */ /* winnt.h:1560:9, winnt.h:1560:9, winnt.h:1560:9 */
pub const LANG_KAZAK: i32 = 0x3fi32; /* Integer(63, Yes, Unknown) */ /* winnt.h:1561:9, winnt.h:1561:9, winnt.h:1561:9 */
pub const LANG_KHMER: i32 = 0x53i32; /* Integer(83, Yes, Unknown) */ /* winnt.h:1562:9, winnt.h:1562:9, winnt.h:1562:9 */
pub const LANG_KICHE: i32 = 0x86i32; /* Integer(134, Yes, Unknown) */ /* winnt.h:1563:9, winnt.h:1563:9, winnt.h:1563:9 */
pub const LANG_KINYARWANDA: i32 = 0x87i32; /* Integer(135, Yes, Unknown) */ /* winnt.h:1564:9, winnt.h:1564:9, winnt.h:1564:9 */
pub const LANG_KONKANI: i32 = 0x57i32; /* Integer(87, Yes, Unknown) */ /* winnt.h:1565:9, winnt.h:1565:9, winnt.h:1565:9 */
pub const LANG_KOREAN: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:1566:9, winnt.h:1566:9, winnt.h:1566:9 */
pub const LANG_KYRGYZ: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:1567:9, winnt.h:1567:9, winnt.h:1567:9 */
pub const LANG_LAO: i32 = 0x54i32; /* Integer(84, Yes, Unknown) */ /* winnt.h:1568:9, winnt.h:1568:9, winnt.h:1568:9 */
pub const LANG_LATVIAN: i32 = 0x26i32; /* Integer(38, Yes, Unknown) */ /* winnt.h:1569:9, winnt.h:1569:9, winnt.h:1569:9 */
pub const LANG_LITHUANIAN: i32 = 0x27i32; /* Integer(39, Yes, Unknown) */ /* winnt.h:1570:9, winnt.h:1570:9, winnt.h:1570:9 */
pub const LANG_LOWER_SORBIAN: i32 = 0x2ei32; /* Integer(46, Yes, Unknown) */ /* winnt.h:1571:9, winnt.h:1571:9, winnt.h:1571:9 */
pub const LANG_LUXEMBOURGISH: i32 = 0x6ei32; /* Integer(110, Yes, Unknown) */ /* winnt.h:1572:9, winnt.h:1572:9, winnt.h:1572:9 */
pub const LANG_MACEDONIAN: i32 = 0x2fi32; /* Integer(47, Yes, Unknown) */ /* winnt.h:1573:9, winnt.h:1573:9, winnt.h:1573:9 */
pub const LANG_MALAY: i32 = 0x3ei32; /* Integer(62, Yes, Unknown) */ /* winnt.h:1574:9, winnt.h:1574:9, winnt.h:1574:9 */
pub const LANG_MALAYALAM: i32 = 0x4ci32; /* Integer(76, Yes, Unknown) */ /* winnt.h:1575:9, winnt.h:1575:9, winnt.h:1575:9 */
pub const LANG_MALTESE: i32 = 0x3ai32; /* Integer(58, Yes, Unknown) */ /* winnt.h:1576:9, winnt.h:1576:9, winnt.h:1576:9 */
pub const LANG_MANIPURI: i32 = 0x58i32; /* Integer(88, Yes, Unknown) */ /* winnt.h:1577:9, winnt.h:1577:9, winnt.h:1577:9 */
pub const LANG_MAORI: i32 = 0x81i32; /* Integer(129, Yes, Unknown) */ /* winnt.h:1578:9, winnt.h:1578:9, winnt.h:1578:9 */
pub const LANG_MAPUDUNGUN: i32 = 0x7ai32; /* Integer(122, Yes, Unknown) */ /* winnt.h:1579:9, winnt.h:1579:9, winnt.h:1579:9 */
pub const LANG_MARATHI: i32 = 0x4ei32; /* Integer(78, Yes, Unknown) */ /* winnt.h:1580:9, winnt.h:1580:9, winnt.h:1580:9 */
pub const LANG_MOHAWK: i32 = 0x7ci32; /* Integer(124, Yes, Unknown) */ /* winnt.h:1581:9, winnt.h:1581:9, winnt.h:1581:9 */
pub const LANG_MONGOLIAN: i32 = 0x50i32; /* Integer(80, Yes, Unknown) */ /* winnt.h:1582:9, winnt.h:1582:9, winnt.h:1582:9 */
pub const LANG_NEPALI: i32 = 0x61i32; /* Integer(97, Yes, Unknown) */ /* winnt.h:1583:9, winnt.h:1583:9, winnt.h:1583:9 */
pub const LANG_NORWEGIAN: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnt.h:1584:9, winnt.h:1584:9, winnt.h:1584:9 */
pub const LANG_OCCITAN: i32 = 0x82i32; /* Integer(130, Yes, Unknown) */ /* winnt.h:1585:9, winnt.h:1585:9, winnt.h:1585:9 */
pub const LANG_ODIA: i32 = 0x48i32; /* Integer(72, Yes, Unknown) */ /* winnt.h:1586:9, winnt.h:1586:9, winnt.h:1586:9 */
pub const LANG_ORIYA: i32 = 0x48i32; /* Integer(72, Yes, Unknown) */ /* winnt.h:1587:9, winnt.h:1587:9, winnt.h:1587:9 */
pub const LANG_PASHTO: i32 = 0x63i32; /* Integer(99, Yes, Unknown) */ /* winnt.h:1588:9, winnt.h:1588:9, winnt.h:1588:9 */
pub const LANG_PERSIAN: i32 = 0x29i32; /* Integer(41, Yes, Unknown) */ /* winnt.h:1589:9, winnt.h:1589:9, winnt.h:1589:9 */
pub const LANG_POLISH: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winnt.h:1590:9, winnt.h:1590:9, winnt.h:1590:9 */
pub const LANG_PORTUGUESE: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winnt.h:1591:9, winnt.h:1591:9, winnt.h:1591:9 */
pub const LANG_PULAR: i32 = 0x67i32; /* Integer(103, Yes, Unknown) */ /* winnt.h:1592:9, winnt.h:1592:9, winnt.h:1592:9 */
pub const LANG_PUNJABI: i32 = 0x46i32; /* Integer(70, Yes, Unknown) */ /* winnt.h:1593:9, winnt.h:1593:9, winnt.h:1593:9 */
pub const LANG_QUECHUA: i32 = 0x6bi32; /* Integer(107, Yes, Unknown) */ /* winnt.h:1594:9, winnt.h:1594:9, winnt.h:1594:9 */
pub const LANG_ROMANIAN: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winnt.h:1595:9, winnt.h:1595:9, winnt.h:1595:9 */
pub const LANG_ROMANSH: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winnt.h:1596:9, winnt.h:1596:9, winnt.h:1596:9 */
pub const LANG_RUSSIAN: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* winnt.h:1597:9, winnt.h:1597:9, winnt.h:1597:9 */
pub const LANG_SAKHA: i32 = 0x85i32; /* Integer(133, Yes, Unknown) */ /* winnt.h:1598:9, winnt.h:1598:9, winnt.h:1598:9 */
pub const LANG_SAMI: i32 = 0x3bi32; /* Integer(59, Yes, Unknown) */ /* winnt.h:1599:9, winnt.h:1599:9, winnt.h:1599:9 */
pub const LANG_SANSKRIT: i32 = 0x4fi32; /* Integer(79, Yes, Unknown) */ /* winnt.h:1600:9, winnt.h:1600:9, winnt.h:1600:9 */
pub const LANG_SCOTTISH_GAELIC: i32 = 0x91i32; /* Integer(145, Yes, Unknown) */ /* winnt.h:1601:9, winnt.h:1601:9, winnt.h:1601:9 */
pub const LANG_SERBIAN: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* winnt.h:1602:9, winnt.h:1602:9, winnt.h:1602:9 */
pub const LANG_SERBIAN_NEUTRAL: i32 = 0x7c1ai32; /* Integer(31770, Yes, Unknown) */ /* winnt.h:1603:9, winnt.h:1603:9, winnt.h:1603:9 */
pub const LANG_SINDHI: i32 = 0x59i32; /* Integer(89, Yes, Unknown) */ /* winnt.h:1604:9, winnt.h:1604:9, winnt.h:1604:9 */
pub const LANG_SINHALESE: i32 = 0x5bi32; /* Integer(91, Yes, Unknown) */ /* winnt.h:1605:9, winnt.h:1605:9, winnt.h:1605:9 */
pub const LANG_SLOVAK: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* winnt.h:1606:9, winnt.h:1606:9, winnt.h:1606:9 */
pub const LANG_SLOVENIAN: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* winnt.h:1607:9, winnt.h:1607:9, winnt.h:1607:9 */
pub const LANG_SOTHO: i32 = 0x6ci32; /* Integer(108, Yes, Unknown) */ /* winnt.h:1608:9, winnt.h:1608:9, winnt.h:1608:9 */
pub const LANG_SPANISH: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:1609:9, winnt.h:1609:9, winnt.h:1609:9 */
pub const LANG_SWAHILI: i32 = 0x41i32; /* Integer(65, Yes, Unknown) */ /* winnt.h:1610:9, winnt.h:1610:9, winnt.h:1610:9 */
pub const LANG_SWEDISH: i32 = 0x1di32; /* Integer(29, Yes, Unknown) */ /* winnt.h:1611:9, winnt.h:1611:9, winnt.h:1611:9 */
pub const LANG_SYRIAC: i32 = 0x5ai32; /* Integer(90, Yes, Unknown) */ /* winnt.h:1612:9, winnt.h:1612:9, winnt.h:1612:9 */
pub const LANG_TAJIK: i32 = 0x28i32; /* Integer(40, Yes, Unknown) */ /* winnt.h:1613:9, winnt.h:1613:9, winnt.h:1613:9 */
pub const LANG_TAMAZIGHT: i32 = 0x5fi32; /* Integer(95, Yes, Unknown) */ /* winnt.h:1614:9, winnt.h:1614:9, winnt.h:1614:9 */
pub const LANG_TAMIL: i32 = 0x49i32; /* Integer(73, Yes, Unknown) */ /* winnt.h:1615:9, winnt.h:1615:9, winnt.h:1615:9 */
pub const LANG_TATAR: i32 = 0x44i32; /* Integer(68, Yes, Unknown) */ /* winnt.h:1616:9, winnt.h:1616:9, winnt.h:1616:9 */
pub const LANG_TELUGU: i32 = 0x4ai32; /* Integer(74, Yes, Unknown) */ /* winnt.h:1617:9, winnt.h:1617:9, winnt.h:1617:9 */
pub const LANG_THAI: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* winnt.h:1618:9, winnt.h:1618:9, winnt.h:1618:9 */
pub const LANG_TIBETAN: i32 = 0x51i32; /* Integer(81, Yes, Unknown) */ /* winnt.h:1619:9, winnt.h:1619:9, winnt.h:1619:9 */
pub const LANG_TIGRIGNA: i32 = 0x73i32; /* Integer(115, Yes, Unknown) */ /* winnt.h:1620:9, winnt.h:1620:9, winnt.h:1620:9 */
pub const LANG_TIGRINYA: i32 = 0x73i32; /* Integer(115, Yes, Unknown) */ /* winnt.h:1621:9, winnt.h:1621:9, winnt.h:1621:9 */
pub const LANG_TSWANA: i32 = 0x32i32; /* Integer(50, Yes, Unknown) */ /* winnt.h:1622:9, winnt.h:1622:9, winnt.h:1622:9 */
pub const LANG_TURKISH: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* winnt.h:1623:9, winnt.h:1623:9, winnt.h:1623:9 */
pub const LANG_TURKMEN: i32 = 0x42i32; /* Integer(66, Yes, Unknown) */ /* winnt.h:1624:9, winnt.h:1624:9, winnt.h:1624:9 */
pub const LANG_UIGHUR: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:1625:9, winnt.h:1625:9, winnt.h:1625:9 */
pub const LANG_UKRAINIAN: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* winnt.h:1626:9, winnt.h:1626:9, winnt.h:1626:9 */
pub const LANG_UPPER_SORBIAN: i32 = 0x2ei32; /* Integer(46, Yes, Unknown) */ /* winnt.h:1627:9, winnt.h:1627:9, winnt.h:1627:9 */
pub const LANG_URDU: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:1628:9, winnt.h:1628:9, winnt.h:1628:9 */
pub const LANG_UZBEK: i32 = 0x43i32; /* Integer(67, Yes, Unknown) */ /* winnt.h:1629:9, winnt.h:1629:9, winnt.h:1629:9 */
pub const LANG_VALENCIAN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:1630:9, winnt.h:1630:9, winnt.h:1630:9 */
pub const LANG_VIETNAMESE: i32 = 0x2ai32; /* Integer(42, Yes, Unknown) */ /* winnt.h:1631:9, winnt.h:1631:9, winnt.h:1631:9 */
pub const LANG_WELSH: i32 = 0x52i32; /* Integer(82, Yes, Unknown) */ /* winnt.h:1632:9, winnt.h:1632:9, winnt.h:1632:9 */
pub const LANG_WOLOF: i32 = 0x88i32; /* Integer(136, Yes, Unknown) */ /* winnt.h:1633:9, winnt.h:1633:9, winnt.h:1633:9 */
pub const LANG_XHOSA: i32 = 0x34i32; /* Integer(52, Yes, Unknown) */ /* winnt.h:1634:9, winnt.h:1634:9, winnt.h:1634:9 */
pub const LANG_YAKUT: i32 = 0x85i32; /* Integer(133, Yes, Unknown) */ /* winnt.h:1635:9, winnt.h:1635:9, winnt.h:1635:9 */
pub const LANG_YI: i32 = 0x78i32; /* Integer(120, Yes, Unknown) */ /* winnt.h:1636:9, winnt.h:1636:9, winnt.h:1636:9 */
pub const LANG_YORUBA: i32 = 0x6ai32; /* Integer(106, Yes, Unknown) */ /* winnt.h:1637:9, winnt.h:1637:9, winnt.h:1637:9 */
pub const LANG_ZULU: i32 = 0x35i32; /* Integer(53, Yes, Unknown) */ /* winnt.h:1638:9, winnt.h:1638:9, winnt.h:1638:9 */
pub const SUBLANG_NEUTRAL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:1651:9, winnt.h:1651:9, winnt.h:1651:9 */
pub const SUBLANG_DEFAULT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1652:9, winnt.h:1652:9, winnt.h:1652:9 */
pub const SUBLANG_SYS_DEFAULT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1653:9, winnt.h:1653:9, winnt.h:1653:9 */
pub const SUBLANG_CUSTOM_DEFAULT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:1654:9, winnt.h:1654:9, winnt.h:1654:9 */
pub const SUBLANG_CUSTOM_UNSPECIFIED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1655:9, winnt.h:1655:9, winnt.h:1655:9 */
pub const SUBLANG_UI_CUSTOM_DEFAULT: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:1656:9, winnt.h:1656:9, winnt.h:1656:9 */
pub const SUBLANG_AFRIKAANS_SOUTH_AFRICA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1659:9, winnt.h:1659:9, winnt.h:1659:9 */
pub const SUBLANG_ALBANIAN_ALBANIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1660:9, winnt.h:1660:9, winnt.h:1660:9 */
pub const SUBLANG_ALSATIAN_FRANCE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1661:9, winnt.h:1661:9, winnt.h:1661:9 */
pub const SUBLANG_AMHARIC_ETHIOPIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1662:9, winnt.h:1662:9, winnt.h:1662:9 */
pub const SUBLANG_ARABIC_SAUDI_ARABIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1663:9, winnt.h:1663:9, winnt.h:1663:9 */
pub const SUBLANG_ARABIC_IRAQ: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1664:9, winnt.h:1664:9, winnt.h:1664:9 */
pub const SUBLANG_ARABIC_EGYPT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:1665:9, winnt.h:1665:9, winnt.h:1665:9 */
pub const SUBLANG_ARABIC_LIBYA: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1666:9, winnt.h:1666:9, winnt.h:1666:9 */
pub const SUBLANG_ARABIC_ALGERIA: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:1667:9, winnt.h:1667:9, winnt.h:1667:9 */
pub const SUBLANG_ARABIC_MOROCCO: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:1668:9, winnt.h:1668:9, winnt.h:1668:9 */
pub const SUBLANG_ARABIC_TUNISIA: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:1669:9, winnt.h:1669:9, winnt.h:1669:9 */
pub const SUBLANG_ARABIC_OMAN: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:1670:9, winnt.h:1670:9, winnt.h:1670:9 */
pub const SUBLANG_ARABIC_YEMEN: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:1671:9, winnt.h:1671:9, winnt.h:1671:9 */
pub const SUBLANG_ARABIC_SYRIA: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:1672:9, winnt.h:1672:9, winnt.h:1672:9 */
pub const SUBLANG_ARABIC_JORDAN: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:1673:9, winnt.h:1673:9, winnt.h:1673:9 */
pub const SUBLANG_ARABIC_LEBANON: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:1674:9, winnt.h:1674:9, winnt.h:1674:9 */
pub const SUBLANG_ARABIC_KUWAIT: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:1675:9, winnt.h:1675:9, winnt.h:1675:9 */
pub const SUBLANG_ARABIC_UAE: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:1676:9, winnt.h:1676:9, winnt.h:1676:9 */
pub const SUBLANG_ARABIC_BAHRAIN: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:1677:9, winnt.h:1677:9, winnt.h:1677:9 */
pub const SUBLANG_ARABIC_QATAR: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:1678:9, winnt.h:1678:9, winnt.h:1678:9 */
pub const SUBLANG_ARMENIAN_ARMENIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1679:9, winnt.h:1679:9, winnt.h:1679:9 */
pub const SUBLANG_ASSAMESE_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1680:9, winnt.h:1680:9, winnt.h:1680:9 */
pub const SUBLANG_AZERI_LATIN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1681:9, winnt.h:1681:9, winnt.h:1681:9 */
pub const SUBLANG_AZERI_CYRILLIC: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1682:9, winnt.h:1682:9, winnt.h:1682:9 */
pub const SUBLANG_AZERBAIJANI_AZERBAIJAN_LATIN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1683:9, winnt.h:1683:9, winnt.h:1683:9 */
pub const SUBLANG_AZERBAIJANI_AZERBAIJAN_CYRILLIC: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1684:9, winnt.h:1684:9, winnt.h:1684:9 */
pub const SUBLANG_BANGLA_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1685:9, winnt.h:1685:9, winnt.h:1685:9 */
pub const SUBLANG_BANGLA_BANGLADESH: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1686:9, winnt.h:1686:9, winnt.h:1686:9 */
pub const SUBLANG_BASHKIR_RUSSIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1687:9, winnt.h:1687:9, winnt.h:1687:9 */
pub const SUBLANG_BASQUE_BASQUE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1688:9, winnt.h:1688:9, winnt.h:1688:9 */
pub const SUBLANG_BELARUSIAN_BELARUS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1689:9, winnt.h:1689:9, winnt.h:1689:9 */
pub const SUBLANG_BENGALI_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1690:9, winnt.h:1690:9, winnt.h:1690:9 */
pub const SUBLANG_BENGALI_BANGLADESH: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1691:9, winnt.h:1691:9, winnt.h:1691:9 */
pub const SUBLANG_BOSNIAN_BOSNIA_HERZEGOVINA_LATIN: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:1692:9, winnt.h:1692:9, winnt.h:1692:9 */
pub const SUBLANG_BOSNIAN_BOSNIA_HERZEGOVINA_CYRILLIC: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:1693:9, winnt.h:1693:9, winnt.h:1693:9 */
pub const SUBLANG_BRETON_FRANCE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1694:9, winnt.h:1694:9, winnt.h:1694:9 */
pub const SUBLANG_BULGARIAN_BULGARIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1695:9, winnt.h:1695:9, winnt.h:1695:9 */
pub const SUBLANG_CATALAN_CATALAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1696:9, winnt.h:1696:9, winnt.h:1696:9 */
pub const SUBLANG_CENTRAL_KURDISH_IRAQ: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1697:9, winnt.h:1697:9, winnt.h:1697:9 */
pub const SUBLANG_CHEROKEE_CHEROKEE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1698:9, winnt.h:1698:9, winnt.h:1698:9 */
pub const SUBLANG_CHINESE_TRADITIONAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1699:9, winnt.h:1699:9, winnt.h:1699:9 */
pub const SUBLANG_CHINESE_SIMPLIFIED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1700:9, winnt.h:1700:9, winnt.h:1700:9 */
pub const SUBLANG_CHINESE_HONGKONG: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:1701:9, winnt.h:1701:9, winnt.h:1701:9 */
pub const SUBLANG_CHINESE_SINGAPORE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1702:9, winnt.h:1702:9, winnt.h:1702:9 */
pub const SUBLANG_CHINESE_MACAU: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:1703:9, winnt.h:1703:9, winnt.h:1703:9 */
pub const SUBLANG_CORSICAN_FRANCE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1704:9, winnt.h:1704:9, winnt.h:1704:9 */
pub const SUBLANG_CZECH_CZECH_REPUBLIC: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1705:9, winnt.h:1705:9, winnt.h:1705:9 */
pub const SUBLANG_CROATIAN_CROATIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1706:9, winnt.h:1706:9, winnt.h:1706:9 */
pub const SUBLANG_CROATIAN_BOSNIA_HERZEGOVINA_LATIN: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1707:9, winnt.h:1707:9, winnt.h:1707:9 */
pub const SUBLANG_DANISH_DENMARK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1708:9, winnt.h:1708:9, winnt.h:1708:9 */
pub const SUBLANG_DARI_AFGHANISTAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1709:9, winnt.h:1709:9, winnt.h:1709:9 */
pub const SUBLANG_DIVEHI_MALDIVES: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1710:9, winnt.h:1710:9, winnt.h:1710:9 */
pub const SUBLANG_DUTCH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1711:9, winnt.h:1711:9, winnt.h:1711:9 */
pub const SUBLANG_DUTCH_BELGIAN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1712:9, winnt.h:1712:9, winnt.h:1712:9 */
pub const SUBLANG_ENGLISH_US: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1713:9, winnt.h:1713:9, winnt.h:1713:9 */
pub const SUBLANG_ENGLISH_UK: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1714:9, winnt.h:1714:9, winnt.h:1714:9 */
pub const SUBLANG_ENGLISH_AUS: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:1715:9, winnt.h:1715:9, winnt.h:1715:9 */
pub const SUBLANG_ENGLISH_CAN: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1716:9, winnt.h:1716:9, winnt.h:1716:9 */
pub const SUBLANG_ENGLISH_NZ: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:1717:9, winnt.h:1717:9, winnt.h:1717:9 */
pub const SUBLANG_ENGLISH_EIRE: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:1718:9, winnt.h:1718:9, winnt.h:1718:9 */
pub const SUBLANG_ENGLISH_SOUTH_AFRICA: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:1719:9, winnt.h:1719:9, winnt.h:1719:9 */
pub const SUBLANG_ENGLISH_JAMAICA: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:1720:9, winnt.h:1720:9, winnt.h:1720:9 */
pub const SUBLANG_ENGLISH_CARIBBEAN: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:1721:9, winnt.h:1721:9, winnt.h:1721:9 */
pub const SUBLANG_ENGLISH_BELIZE: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:1722:9, winnt.h:1722:9, winnt.h:1722:9 */
pub const SUBLANG_ENGLISH_TRINIDAD: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:1723:9, winnt.h:1723:9, winnt.h:1723:9 */
pub const SUBLANG_ENGLISH_ZIMBABWE: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:1724:9, winnt.h:1724:9, winnt.h:1724:9 */
pub const SUBLANG_ENGLISH_PHILIPPINES: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:1725:9, winnt.h:1725:9, winnt.h:1725:9 */
pub const SUBLANG_ENGLISH_INDIA: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:1726:9, winnt.h:1726:9, winnt.h:1726:9 */
pub const SUBLANG_ENGLISH_MALAYSIA: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winnt.h:1727:9, winnt.h:1727:9, winnt.h:1727:9 */
pub const SUBLANG_ENGLISH_SINGAPORE: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:1728:9, winnt.h:1728:9, winnt.h:1728:9 */
pub const SUBLANG_ESTONIAN_ESTONIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1729:9, winnt.h:1729:9, winnt.h:1729:9 */
pub const SUBLANG_FAEROESE_FAROE_ISLANDS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1730:9, winnt.h:1730:9, winnt.h:1730:9 */
pub const SUBLANG_FILIPINO_PHILIPPINES: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1731:9, winnt.h:1731:9, winnt.h:1731:9 */
pub const SUBLANG_FINNISH_FINLAND: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1732:9, winnt.h:1732:9, winnt.h:1732:9 */
pub const SUBLANG_FRENCH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1733:9, winnt.h:1733:9, winnt.h:1733:9 */
pub const SUBLANG_FRENCH_BELGIAN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1734:9, winnt.h:1734:9, winnt.h:1734:9 */
pub const SUBLANG_FRENCH_CANADIAN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:1735:9, winnt.h:1735:9, winnt.h:1735:9 */
pub const SUBLANG_FRENCH_SWISS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1736:9, winnt.h:1736:9, winnt.h:1736:9 */
pub const SUBLANG_FRENCH_LUXEMBOURG: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:1737:9, winnt.h:1737:9, winnt.h:1737:9 */
pub const SUBLANG_FRENCH_MONACO: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:1738:9, winnt.h:1738:9, winnt.h:1738:9 */
pub const SUBLANG_FRISIAN_NETHERLANDS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1739:9, winnt.h:1739:9, winnt.h:1739:9 */
pub const SUBLANG_FULAH_SENEGAL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1740:9, winnt.h:1740:9, winnt.h:1740:9 */
pub const SUBLANG_GALICIAN_GALICIAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1741:9, winnt.h:1741:9, winnt.h:1741:9 */
pub const SUBLANG_GEORGIAN_GEORGIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1742:9, winnt.h:1742:9, winnt.h:1742:9 */
pub const SUBLANG_GERMAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1743:9, winnt.h:1743:9, winnt.h:1743:9 */
pub const SUBLANG_GERMAN_SWISS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1744:9, winnt.h:1744:9, winnt.h:1744:9 */
pub const SUBLANG_GERMAN_AUSTRIAN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:1745:9, winnt.h:1745:9, winnt.h:1745:9 */
pub const SUBLANG_GERMAN_LUXEMBOURG: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1746:9, winnt.h:1746:9, winnt.h:1746:9 */
pub const SUBLANG_GERMAN_LIECHTENSTEIN: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:1747:9, winnt.h:1747:9, winnt.h:1747:9 */
pub const SUBLANG_GREEK_GREECE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1748:9, winnt.h:1748:9, winnt.h:1748:9 */
pub const SUBLANG_GREENLANDIC_GREENLAND: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1749:9, winnt.h:1749:9, winnt.h:1749:9 */
pub const SUBLANG_GUJARATI_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1750:9, winnt.h:1750:9, winnt.h:1750:9 */
pub const SUBLANG_HAUSA_NIGERIA_LATIN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1751:9, winnt.h:1751:9, winnt.h:1751:9 */
pub const SUBLANG_HAWAIIAN_US: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1752:9, winnt.h:1752:9, winnt.h:1752:9 */
pub const SUBLANG_HEBREW_ISRAEL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1753:9, winnt.h:1753:9, winnt.h:1753:9 */
pub const SUBLANG_HINDI_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1754:9, winnt.h:1754:9, winnt.h:1754:9 */
pub const SUBLANG_HUNGARIAN_HUNGARY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1755:9, winnt.h:1755:9, winnt.h:1755:9 */
pub const SUBLANG_ICELANDIC_ICELAND: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1756:9, winnt.h:1756:9, winnt.h:1756:9 */
pub const SUBLANG_IGBO_NIGERIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1757:9, winnt.h:1757:9, winnt.h:1757:9 */
pub const SUBLANG_INDONESIAN_INDONESIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1758:9, winnt.h:1758:9, winnt.h:1758:9 */
pub const SUBLANG_INUKTITUT_CANADA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1759:9, winnt.h:1759:9, winnt.h:1759:9 */
pub const SUBLANG_INUKTITUT_CANADA_LATIN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1760:9, winnt.h:1760:9, winnt.h:1760:9 */
pub const SUBLANG_IRISH_IRELAND: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1761:9, winnt.h:1761:9, winnt.h:1761:9 */
pub const SUBLANG_ITALIAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1762:9, winnt.h:1762:9, winnt.h:1762:9 */
pub const SUBLANG_ITALIAN_SWISS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1763:9, winnt.h:1763:9, winnt.h:1763:9 */
pub const SUBLANG_JAPANESE_JAPAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1764:9, winnt.h:1764:9, winnt.h:1764:9 */
pub const SUBLANG_KANNADA_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1765:9, winnt.h:1765:9, winnt.h:1765:9 */
pub const SUBLANG_KASHMIRI_SASIA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1766:9, winnt.h:1766:9, winnt.h:1766:9 */
pub const SUBLANG_KASHMIRI_INDIA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1767:9, winnt.h:1767:9, winnt.h:1767:9 */
pub const SUBLANG_KAZAK_KAZAKHSTAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1768:9, winnt.h:1768:9, winnt.h:1768:9 */
pub const SUBLANG_KHMER_CAMBODIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1769:9, winnt.h:1769:9, winnt.h:1769:9 */
pub const SUBLANG_KICHE_GUATEMALA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1770:9, winnt.h:1770:9, winnt.h:1770:9 */
pub const SUBLANG_KINYARWANDA_RWANDA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1771:9, winnt.h:1771:9, winnt.h:1771:9 */
pub const SUBLANG_KONKANI_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1772:9, winnt.h:1772:9, winnt.h:1772:9 */
pub const SUBLANG_KOREAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1773:9, winnt.h:1773:9, winnt.h:1773:9 */
pub const SUBLANG_KYRGYZ_KYRGYZSTAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1774:9, winnt.h:1774:9, winnt.h:1774:9 */
pub const SUBLANG_LAO_LAO: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1775:9, winnt.h:1775:9, winnt.h:1775:9 */
pub const SUBLANG_LATVIAN_LATVIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1776:9, winnt.h:1776:9, winnt.h:1776:9 */
pub const SUBLANG_LITHUANIAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1777:9, winnt.h:1777:9, winnt.h:1777:9 */
pub const SUBLANG_LOWER_SORBIAN_GERMANY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1778:9, winnt.h:1778:9, winnt.h:1778:9 */
pub const SUBLANG_LUXEMBOURGISH_LUXEMBOURG: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1779:9, winnt.h:1779:9, winnt.h:1779:9 */
pub const SUBLANG_MACEDONIAN_MACEDONIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1780:9, winnt.h:1780:9, winnt.h:1780:9 */
pub const SUBLANG_MALAY_MALAYSIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1781:9, winnt.h:1781:9, winnt.h:1781:9 */
pub const SUBLANG_MALAY_BRUNEI_DARUSSALAM: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1782:9, winnt.h:1782:9, winnt.h:1782:9 */
pub const SUBLANG_MALAYALAM_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1783:9, winnt.h:1783:9, winnt.h:1783:9 */
pub const SUBLANG_MALTESE_MALTA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1784:9, winnt.h:1784:9, winnt.h:1784:9 */
pub const SUBLANG_MAORI_NEW_ZEALAND: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1785:9, winnt.h:1785:9, winnt.h:1785:9 */
pub const SUBLANG_MAPUDUNGUN_CHILE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1786:9, winnt.h:1786:9, winnt.h:1786:9 */
pub const SUBLANG_MARATHI_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1787:9, winnt.h:1787:9, winnt.h:1787:9 */
pub const SUBLANG_MOHAWK_MOHAWK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1788:9, winnt.h:1788:9, winnt.h:1788:9 */
pub const SUBLANG_MONGOLIAN_CYRILLIC_MONGOLIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1789:9, winnt.h:1789:9, winnt.h:1789:9 */
pub const SUBLANG_MONGOLIAN_PRC: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1790:9, winnt.h:1790:9, winnt.h:1790:9 */
pub const SUBLANG_NEPALI_INDIA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1791:9, winnt.h:1791:9, winnt.h:1791:9 */
pub const SUBLANG_NEPALI_NEPAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1792:9, winnt.h:1792:9, winnt.h:1792:9 */
pub const SUBLANG_NORWEGIAN_BOKMAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1793:9, winnt.h:1793:9, winnt.h:1793:9 */
pub const SUBLANG_NORWEGIAN_NYNORSK: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1794:9, winnt.h:1794:9, winnt.h:1794:9 */
pub const SUBLANG_OCCITAN_FRANCE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1795:9, winnt.h:1795:9, winnt.h:1795:9 */
pub const SUBLANG_ODIA_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1796:9, winnt.h:1796:9, winnt.h:1796:9 */
pub const SUBLANG_ORIYA_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1797:9, winnt.h:1797:9, winnt.h:1797:9 */
pub const SUBLANG_PASHTO_AFGHANISTAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1798:9, winnt.h:1798:9, winnt.h:1798:9 */
pub const SUBLANG_PERSIAN_IRAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1799:9, winnt.h:1799:9, winnt.h:1799:9 */
pub const SUBLANG_POLISH_POLAND: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1800:9, winnt.h:1800:9, winnt.h:1800:9 */
pub const SUBLANG_PORTUGUESE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1801:9, winnt.h:1801:9, winnt.h:1801:9 */
pub const SUBLANG_PORTUGUESE_BRAZILIAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1802:9, winnt.h:1802:9, winnt.h:1802:9 */
pub const SUBLANG_PULAR_SENEGAL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1803:9, winnt.h:1803:9, winnt.h:1803:9 */
pub const SUBLANG_PUNJABI_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1804:9, winnt.h:1804:9, winnt.h:1804:9 */
pub const SUBLANG_PUNJABI_PAKISTAN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1805:9, winnt.h:1805:9, winnt.h:1805:9 */
pub const SUBLANG_QUECHUA_BOLIVIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1806:9, winnt.h:1806:9, winnt.h:1806:9 */
pub const SUBLANG_QUECHUA_ECUADOR: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1807:9, winnt.h:1807:9, winnt.h:1807:9 */
pub const SUBLANG_QUECHUA_PERU: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:1808:9, winnt.h:1808:9, winnt.h:1808:9 */
pub const SUBLANG_ROMANIAN_ROMANIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1809:9, winnt.h:1809:9, winnt.h:1809:9 */
pub const SUBLANG_ROMANSH_SWITZERLAND: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1810:9, winnt.h:1810:9, winnt.h:1810:9 */
pub const SUBLANG_RUSSIAN_RUSSIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1811:9, winnt.h:1811:9, winnt.h:1811:9 */
pub const SUBLANG_SAKHA_RUSSIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1812:9, winnt.h:1812:9, winnt.h:1812:9 */
pub const SUBLANG_SAMI_NORTHERN_NORWAY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1813:9, winnt.h:1813:9, winnt.h:1813:9 */
pub const SUBLANG_SAMI_NORTHERN_SWEDEN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1814:9, winnt.h:1814:9, winnt.h:1814:9 */
pub const SUBLANG_SAMI_NORTHERN_FINLAND: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:1815:9, winnt.h:1815:9, winnt.h:1815:9 */
pub const SUBLANG_SAMI_LULE_NORWAY: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1816:9, winnt.h:1816:9, winnt.h:1816:9 */
pub const SUBLANG_SAMI_LULE_SWEDEN: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:1817:9, winnt.h:1817:9, winnt.h:1817:9 */
pub const SUBLANG_SAMI_SOUTHERN_NORWAY: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:1818:9, winnt.h:1818:9, winnt.h:1818:9 */
pub const SUBLANG_SAMI_SOUTHERN_SWEDEN: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:1819:9, winnt.h:1819:9, winnt.h:1819:9 */
pub const SUBLANG_SAMI_SKOLT_FINLAND: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:1820:9, winnt.h:1820:9, winnt.h:1820:9 */
pub const SUBLANG_SAMI_INARI_FINLAND: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:1821:9, winnt.h:1821:9, winnt.h:1821:9 */
pub const SUBLANG_SANSKRIT_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1822:9, winnt.h:1822:9, winnt.h:1822:9 */
pub const SUBLANG_SCOTTISH_GAELIC: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1823:9, winnt.h:1823:9, winnt.h:1823:9 */
pub const SUBLANG_SERBIAN_BOSNIA_HERZEGOVINA_LATIN: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:1824:9, winnt.h:1824:9, winnt.h:1824:9 */
pub const SUBLANG_SERBIAN_BOSNIA_HERZEGOVINA_CYRILLIC: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:1825:9, winnt.h:1825:9, winnt.h:1825:9 */
pub const SUBLANG_SERBIAN_MONTENEGRO_LATIN: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:1826:9, winnt.h:1826:9, winnt.h:1826:9 */
pub const SUBLANG_SERBIAN_MONTENEGRO_CYRILLIC: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:1827:9, winnt.h:1827:9, winnt.h:1827:9 */
pub const SUBLANG_SERBIAN_SERBIA_LATIN: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:1828:9, winnt.h:1828:9, winnt.h:1828:9 */
pub const SUBLANG_SERBIAN_SERBIA_CYRILLIC: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:1829:9, winnt.h:1829:9, winnt.h:1829:9 */
pub const SUBLANG_SERBIAN_CROATIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1830:9, winnt.h:1830:9, winnt.h:1830:9 */
pub const SUBLANG_SERBIAN_LATIN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1831:9, winnt.h:1831:9, winnt.h:1831:9 */
pub const SUBLANG_SERBIAN_CYRILLIC: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:1832:9, winnt.h:1832:9, winnt.h:1832:9 */
pub const SUBLANG_SINDHI_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1833:9, winnt.h:1833:9, winnt.h:1833:9 */
pub const SUBLANG_SINDHI_PAKISTAN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1834:9, winnt.h:1834:9, winnt.h:1834:9 */
pub const SUBLANG_SINDHI_AFGHANISTAN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1835:9, winnt.h:1835:9, winnt.h:1835:9 */
pub const SUBLANG_SINHALESE_SRI_LANKA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1836:9, winnt.h:1836:9, winnt.h:1836:9 */
pub const SUBLANG_SOTHO_NORTHERN_SOUTH_AFRICA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1837:9, winnt.h:1837:9, winnt.h:1837:9 */
pub const SUBLANG_SLOVAK_SLOVAKIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1838:9, winnt.h:1838:9, winnt.h:1838:9 */
pub const SUBLANG_SLOVENIAN_SLOVENIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1839:9, winnt.h:1839:9, winnt.h:1839:9 */
pub const SUBLANG_SPANISH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1840:9, winnt.h:1840:9, winnt.h:1840:9 */
pub const SUBLANG_SPANISH_MEXICAN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1841:9, winnt.h:1841:9, winnt.h:1841:9 */
pub const SUBLANG_SPANISH_MODERN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:1842:9, winnt.h:1842:9, winnt.h:1842:9 */
pub const SUBLANG_SPANISH_GUATEMALA: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1843:9, winnt.h:1843:9, winnt.h:1843:9 */
pub const SUBLANG_SPANISH_COSTA_RICA: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:1844:9, winnt.h:1844:9, winnt.h:1844:9 */
pub const SUBLANG_SPANISH_PANAMA: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:1845:9, winnt.h:1845:9, winnt.h:1845:9 */
pub const SUBLANG_SPANISH_DOMINICAN_REPUBLIC: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:1846:9, winnt.h:1846:9, winnt.h:1846:9 */
pub const SUBLANG_SPANISH_VENEZUELA: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:1847:9, winnt.h:1847:9, winnt.h:1847:9 */
pub const SUBLANG_SPANISH_COLOMBIA: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:1848:9, winnt.h:1848:9, winnt.h:1848:9 */
pub const SUBLANG_SPANISH_PERU: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:1849:9, winnt.h:1849:9, winnt.h:1849:9 */
pub const SUBLANG_SPANISH_ARGENTINA: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:1850:9, winnt.h:1850:9, winnt.h:1850:9 */
pub const SUBLANG_SPANISH_ECUADOR: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:1851:9, winnt.h:1851:9, winnt.h:1851:9 */
pub const SUBLANG_SPANISH_CHILE: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:1852:9, winnt.h:1852:9, winnt.h:1852:9 */
pub const SUBLANG_SPANISH_URUGUAY: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:1853:9, winnt.h:1853:9, winnt.h:1853:9 */
pub const SUBLANG_SPANISH_PARAGUAY: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:1854:9, winnt.h:1854:9, winnt.h:1854:9 */
pub const SUBLANG_SPANISH_BOLIVIA: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:1855:9, winnt.h:1855:9, winnt.h:1855:9 */
pub const SUBLANG_SPANISH_EL_SALVADOR: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winnt.h:1856:9, winnt.h:1856:9, winnt.h:1856:9 */
pub const SUBLANG_SPANISH_HONDURAS: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:1857:9, winnt.h:1857:9, winnt.h:1857:9 */
pub const SUBLANG_SPANISH_NICARAGUA: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winnt.h:1858:9, winnt.h:1858:9, winnt.h:1858:9 */
pub const SUBLANG_SPANISH_PUERTO_RICO: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnt.h:1859:9, winnt.h:1859:9, winnt.h:1859:9 */
pub const SUBLANG_SPANISH_US: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winnt.h:1860:9, winnt.h:1860:9, winnt.h:1860:9 */
pub const SUBLANG_SWAHILI_KENYA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1861:9, winnt.h:1861:9, winnt.h:1861:9 */
pub const SUBLANG_SWEDISH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1862:9, winnt.h:1862:9, winnt.h:1862:9 */
pub const SUBLANG_SWEDISH_FINLAND: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1863:9, winnt.h:1863:9, winnt.h:1863:9 */
pub const SUBLANG_SYRIAC_SYRIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1864:9, winnt.h:1864:9, winnt.h:1864:9 */
pub const SUBLANG_TAJIK_TAJIKISTAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1865:9, winnt.h:1865:9, winnt.h:1865:9 */
pub const SUBLANG_TAMAZIGHT_ALGERIA_LATIN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1866:9, winnt.h:1866:9, winnt.h:1866:9 */
pub const SUBLANG_TAMAZIGHT_MOROCCO_TIFINAGH: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1867:9, winnt.h:1867:9, winnt.h:1867:9 */
pub const SUBLANG_TAMIL_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1868:9, winnt.h:1868:9, winnt.h:1868:9 */
pub const SUBLANG_TAMIL_SRI_LANKA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1869:9, winnt.h:1869:9, winnt.h:1869:9 */
pub const SUBLANG_TATAR_RUSSIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1870:9, winnt.h:1870:9, winnt.h:1870:9 */
pub const SUBLANG_TELUGU_INDIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1871:9, winnt.h:1871:9, winnt.h:1871:9 */
pub const SUBLANG_THAI_THAILAND: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1872:9, winnt.h:1872:9, winnt.h:1872:9 */
pub const SUBLANG_TIBETAN_PRC: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1873:9, winnt.h:1873:9, winnt.h:1873:9 */
pub const SUBLANG_TIGRIGNA_ERITREA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1874:9, winnt.h:1874:9, winnt.h:1874:9 */
pub const SUBLANG_TIGRINYA_ERITREA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1875:9, winnt.h:1875:9, winnt.h:1875:9 */
pub const SUBLANG_TIGRINYA_ETHIOPIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1876:9, winnt.h:1876:9, winnt.h:1876:9 */
pub const SUBLANG_TSWANA_BOTSWANA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1877:9, winnt.h:1877:9, winnt.h:1877:9 */
pub const SUBLANG_TSWANA_SOUTH_AFRICA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1878:9, winnt.h:1878:9, winnt.h:1878:9 */
pub const SUBLANG_TURKISH_TURKEY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1879:9, winnt.h:1879:9, winnt.h:1879:9 */
pub const SUBLANG_TURKMEN_TURKMENISTAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1880:9, winnt.h:1880:9, winnt.h:1880:9 */
pub const SUBLANG_UIGHUR_PRC: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1881:9, winnt.h:1881:9, winnt.h:1881:9 */
pub const SUBLANG_UKRAINIAN_UKRAINE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1882:9, winnt.h:1882:9, winnt.h:1882:9 */
pub const SUBLANG_UPPER_SORBIAN_GERMANY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1883:9, winnt.h:1883:9, winnt.h:1883:9 */
pub const SUBLANG_URDU_PAKISTAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1884:9, winnt.h:1884:9, winnt.h:1884:9 */
pub const SUBLANG_URDU_INDIA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1885:9, winnt.h:1885:9, winnt.h:1885:9 */
pub const SUBLANG_UZBEK_LATIN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1886:9, winnt.h:1886:9, winnt.h:1886:9 */
pub const SUBLANG_UZBEK_CYRILLIC: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1887:9, winnt.h:1887:9, winnt.h:1887:9 */
pub const SUBLANG_VALENCIAN_VALENCIA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1888:9, winnt.h:1888:9, winnt.h:1888:9 */
pub const SUBLANG_VIETNAMESE_VIETNAM: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1889:9, winnt.h:1889:9, winnt.h:1889:9 */
pub const SUBLANG_WELSH_UNITED_KINGDOM: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1890:9, winnt.h:1890:9, winnt.h:1890:9 */
pub const SUBLANG_WOLOF_SENEGAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1891:9, winnt.h:1891:9, winnt.h:1891:9 */
pub const SUBLANG_XHOSA_SOUTH_AFRICA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1892:9, winnt.h:1892:9, winnt.h:1892:9 */
pub const SUBLANG_YAKUT_RUSSIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1893:9, winnt.h:1893:9, winnt.h:1893:9 */
pub const SUBLANG_YI_PRC: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1894:9, winnt.h:1894:9, winnt.h:1894:9 */
pub const SUBLANG_YORUBA_NIGERIA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1895:9, winnt.h:1895:9, winnt.h:1895:9 */
pub const SUBLANG_ZULU_SOUTH_AFRICA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1896:9, winnt.h:1896:9, winnt.h:1896:9 */
pub const SORT_DEFAULT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:1907:9, winnt.h:1907:9, winnt.h:1907:9 */
pub const SORT_INVARIANT_MATH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1909:9, winnt.h:1909:9, winnt.h:1909:9 */
pub const SORT_JAPANESE_XJIS: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:1911:9, winnt.h:1911:9, winnt.h:1911:9 */
pub const SORT_JAPANESE_UNICODE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1912:9, winnt.h:1912:9, winnt.h:1912:9 */
pub const SORT_JAPANESE_RADICALSTROKE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1913:9, winnt.h:1913:9, winnt.h:1913:9 */
pub const SORT_CHINESE_BIG5: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:1915:9, winnt.h:1915:9, winnt.h:1915:9 */
pub const SORT_CHINESE_PRCP: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:1916:9, winnt.h:1916:9, winnt.h:1916:9 */
pub const SORT_CHINESE_UNICODE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1917:9, winnt.h:1917:9, winnt.h:1917:9 */
pub const SORT_CHINESE_PRC: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:1918:9, winnt.h:1918:9, winnt.h:1918:9 */
pub const SORT_CHINESE_BOPOMOFO: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:1919:9, winnt.h:1919:9, winnt.h:1919:9 */
pub const SORT_CHINESE_RADICALSTROKE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:1920:9, winnt.h:1920:9, winnt.h:1920:9 */
pub const SORT_KOREAN_KSC: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:1922:9, winnt.h:1922:9, winnt.h:1922:9 */
pub const SORT_KOREAN_UNICODE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1923:9, winnt.h:1923:9, winnt.h:1923:9 */
pub const SORT_GERMAN_PHONE_BOOK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1925:9, winnt.h:1925:9, winnt.h:1925:9 */
pub const SORT_HUNGARIAN_DEFAULT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:1927:9, winnt.h:1927:9, winnt.h:1927:9 */
pub const SORT_HUNGARIAN_TECHNICAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1928:9, winnt.h:1928:9, winnt.h:1928:9 */
pub const SORT_GEORGIAN_TRADITIONAL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:1930:9, winnt.h:1930:9, winnt.h:1930:9 */
pub const SORT_GEORGIAN_MODERN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:1931:9, winnt.h:1931:9, winnt.h:1931:9 */
pub const NLS_VALID_LOCALE_MASK: i32 = 0xfffffi32; /* Integer(1048575, Yes, Unknown) */ /* winnt.h:1994:9, winnt.h:1994:9, winnt.h:1994:9 */
pub const LOCALE_NAME_MAX_LENGTH: i32 = 0x55i32; /* Integer(85, Yes, Unknown) */ /* winnt.h:2011:9, winnt.h:2011:9, winnt.h:2011:9 */
pub const LOCALE_TRANSIENT_KEYBOARD1: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winnt.h:2047:9, winnt.h:2047:9, winnt.h:2047:9 */
pub const LOCALE_TRANSIENT_KEYBOARD2: i32 = 0x2400i32; /* Integer(9216, Yes, Unknown) */ /* winnt.h:2048:9, winnt.h:2048:9, winnt.h:2048:9 */
pub const LOCALE_TRANSIENT_KEYBOARD3: i32 = 0x2800i32; /* Integer(10240, Yes, Unknown) */ /* winnt.h:2049:9, winnt.h:2049:9, winnt.h:2049:9 */
pub const LOCALE_TRANSIENT_KEYBOARD4: i32 = 0x2c00i32; /* Integer(11264, Yes, Unknown) */ /* winnt.h:2050:9, winnt.h:2050:9, winnt.h:2050:9 */
pub const MAXIMUM_WAIT_OBJECTS: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:2273:9, winnt.h:2273:9, winnt.h:2273:9 */
#[doc(inline)] pub use ::winnt::MAXCHAR as MAXIMUM_SUSPEND_COUNT; /* winnt.h:2275:9, winnt.h:2275:9, winnt.h:2275:9 */
pub const EXCEPTION_READ_FAULT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:5858:9, winnt.h:3476:9, winnt.h:4517:9 */
pub const EXCEPTION_WRITE_FAULT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:5859:9, winnt.h:3477:9, winnt.h:4518:9 */
pub const EXCEPTION_EXECUTE_FAULT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:5860:9, winnt.h:3478:9, winnt.h:4519:9 */
#[cfg(any(target_arch="x86"))] pub const SIZE_OF_80387_REGISTERS: i32 = 0x50i32; /* Integer(80, Yes, Unknown) */ /* winnt.h:5870:9 */
#[cfg(any(target_arch="x86"))] pub const CONTEXT_i386: i64 = 0x10000i64; /* Integer(65536, Yes, Long) */ /* winnt.h:5878:9 */
#[cfg(any(target_arch="x86"))] pub const CONTEXT_i486: i64 = 0x10000i64; /* Integer(65536, Yes, Long) */ /* winnt.h:5879:9 */
pub const CONTEXT_EXCEPTION_ACTIVE: i64 = 0x8000000i64; /* Integer(134217728, Yes, Long) */ /* winnt.h:5899:9, winnt.h:3506:9, winnt.h:4541:9 */
pub const CONTEXT_SERVICE_ACTIVE: i64 = 0x10000000i64; /* Integer(268435456, Yes, Long) */ /* winnt.h:5900:9, winnt.h:3507:9, winnt.h:4542:9 */
pub const CONTEXT_EXCEPTION_REQUEST: i64 = 0x40000000i64; /* Integer(1073741824, Yes, Long) */ /* winnt.h:5901:9, winnt.h:3508:9, winnt.h:4543:9 */
pub const CONTEXT_EXCEPTION_REPORTING: i64 = 0x80000000i64; /* Integer(2147483648, Yes, Long) */ /* winnt.h:5902:9, winnt.h:3509:9, winnt.h:4544:9 */
#[cfg(any(target_arch="x86"))] pub const MAXIMUM_SUPPORTED_EXTENSION: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:5926:9 */
pub const WOW64_CONTEXT_i386: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winnt.h:8788:9, winnt.h:8788:9, winnt.h:8788:9 */
pub const WOW64_CONTEXT_i486: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winnt.h:8789:9, winnt.h:8789:9, winnt.h:8789:9 */
pub const WOW64_CONTEXT_EXCEPTION_ACTIVE: i32 = 0x8000000i32; /* Integer(134217728, Yes, Unknown) */ /* winnt.h:8806:9, winnt.h:8806:9, winnt.h:8806:9 */
pub const WOW64_CONTEXT_SERVICE_ACTIVE: i32 = 0x10000000i32; /* Integer(268435456, Yes, Unknown) */ /* winnt.h:8807:9, winnt.h:8807:9, winnt.h:8807:9 */
pub const WOW64_CONTEXT_EXCEPTION_REQUEST: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* winnt.h:8808:9, winnt.h:8808:9, winnt.h:8808:9 */
pub const WOW64_CONTEXT_EXCEPTION_REPORTING: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:8809:9, winnt.h:8809:9, winnt.h:8809:9 */
pub const WOW64_SIZE_OF_80387_REGISTERS: i32 = 0x50i32; /* Integer(80, Yes, Unknown) */ /* winnt.h:8817:9, winnt.h:8817:9, winnt.h:8817:9 */
pub const WOW64_MAXIMUM_SUPPORTED_EXTENSION: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:8819:9, winnt.h:8819:9, winnt.h:8819:9 */
pub const EXCEPTION_NONCONTINUABLE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:8968:9, winnt.h:8968:9, winnt.h:8968:9 */
pub const EXCEPTION_UNWINDING: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:8969:9, winnt.h:8969:9, winnt.h:8969:9 */
pub const EXCEPTION_EXIT_UNWIND: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:8970:9, winnt.h:8970:9, winnt.h:8970:9 */
pub const EXCEPTION_STACK_INVALID: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:8971:9, winnt.h:8971:9, winnt.h:8971:9 */
pub const EXCEPTION_NESTED_CALL: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:8972:9, winnt.h:8972:9, winnt.h:8972:9 */
pub const EXCEPTION_TARGET_UNWIND: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:8973:9, winnt.h:8973:9, winnt.h:8973:9 */
pub const EXCEPTION_COLLIDED_UNWIND: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:8974:9, winnt.h:8974:9, winnt.h:8974:9 */
pub const EXCEPTION_MAXIMUM_PARAMETERS: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:8983:9, winnt.h:8983:9, winnt.h:8983:9 */
pub const SID_HASH_SIZE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:9268:9, winnt.h:9268:9, winnt.h:9268:9 */
pub const SECURITY_TRUSTED_INSTALLER_RID1: i32 = 0x38fb89b5i32; /* Integer(956008885, Yes, Unknown) */ /* winnt.h:9624:9, winnt.h:9624:9, winnt.h:9624:9 */
pub const SECURITY_TRUSTED_INSTALLER_RID2: i32 = 0xcbc28419i32; /* Integer(3418522649, Yes, Unknown) */ /* winnt.h:9625:9, winnt.h:9625:9, winnt.h:9625:9 */
pub const SECURITY_TRUSTED_INSTALLER_RID3: i32 = 0x6d236c5ci32; /* Integer(1831038044, Yes, Unknown) */ /* winnt.h:9626:9, winnt.h:9626:9, winnt.h:9626:9 */
pub const SECURITY_TRUSTED_INSTALLER_RID4: i32 = 0x6e770057i32; /* Integer(1853292631, Yes, Unknown) */ /* winnt.h:9627:9, winnt.h:9627:9, winnt.h:9627:9 */
pub const SECURITY_TRUSTED_INSTALLER_RID5: i32 = 0x876402c0i32; /* Integer(2271478464, Yes, Unknown) */ /* winnt.h:9628:9, winnt.h:9628:9, winnt.h:9628:9 */
pub const SYSTEM_MANDATORY_LABEL_NO_WRITE_UP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:10043:9, winnt.h:10043:9, winnt.h:10043:9 */
pub const SYSTEM_MANDATORY_LABEL_NO_READ_UP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:10044:9, winnt.h:10044:9, winnt.h:10044:9 */
pub const SYSTEM_MANDATORY_LABEL_NO_EXECUTE_UP: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:10045:9, winnt.h:10045:9, winnt.h:10045:9 */
pub const SYSTEM_PROCESS_TRUST_LABEL_VALID_MASK: i32 = 0xffffffi32; /* Integer(16777215, Yes, Unknown) */ /* winnt.h:10052:9, winnt.h:10052:9, winnt.h:10052:9 */
pub const SYSTEM_PROCESS_TRUST_NOCONSTRAINT_MASK: i32 = 0xffffffffi32; /* Integer(4294967295, Yes, Unknown) */ /* winnt.h:10053:9, winnt.h:10053:9, winnt.h:10053:9 */
pub const ACE_OBJECT_TYPE_PRESENT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:10170:9, winnt.h:10170:9, winnt.h:10170:9 */
pub const ACE_INHERITED_OBJECT_TYPE_PRESENT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:10171:9, winnt.h:10171:9, winnt.h:10171:9 */
pub const ACCESS_OBJECT_GUID: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:10408:9, winnt.h:10408:9, winnt.h:10408:9 */
pub const ACCESS_PROPERTY_SET_GUID: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:10409:9, winnt.h:10409:9, winnt.h:10409:9 */
pub const ACCESS_PROPERTY_GUID: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:10410:9, winnt.h:10410:9, winnt.h:10410:9 */
pub const ACCESS_MAX_LEVEL: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:10412:9, winnt.h:10412:9, winnt.h:10412:9 */
pub const AUDIT_ALLOW_NO_PRIVILEGE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:10423:9, winnt.h:10423:9, winnt.h:10423:9 */
pub const ACCESS_DS_SOURCE_A: &'static str = "DS"; /* String("DS", Narrow) */ /* winnt.h:10429:9, winnt.h:10429:9, winnt.h:10429:9 */
pub const ACCESS_DS_SOURCE_W: &'static str = "DS"; /* String("DS", Wide) */ /* winnt.h:10430:9, winnt.h:10430:9, winnt.h:10430:9 */
pub const ACCESS_DS_OBJECT_TYPE_NAME_A: &'static str = "Directory Service Object"; /* String("Directory Service Object", Narrow) */ /* winnt.h:10431:9, winnt.h:10431:9, winnt.h:10431:9 */
pub const ACCESS_DS_OBJECT_TYPE_NAME_W: &'static str = "Directory Service Object"; /* String("Directory Service Object", Wide) */ /* winnt.h:10432:9, winnt.h:10432:9, winnt.h:10432:9 */
pub const ACCESS_REASON_TYPE_MASK: i32 = 0xff0000i32; /* Integer(16711680, Yes, Unknown) */ /* winnt.h:10487:9, winnt.h:10487:9, winnt.h:10487:9 */
pub const ACCESS_REASON_DATA_MASK: i32 = 0xffffi32; /* Integer(65535, Yes, Unknown) */ /* winnt.h:10488:9, winnt.h:10488:9, winnt.h:10488:9 */
pub const ACCESS_REASON_STAGING_MASK: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:10490:9, winnt.h:10490:9, winnt.h:10490:9 */
pub const ACCESS_REASON_EXDATA_MASK: i32 = 0x7f000000i32; /* Integer(2130706432, Yes, Unknown) */ /* winnt.h:10491:9, winnt.h:10491:9, winnt.h:10491:9 */
pub const SE_SECURITY_DESCRIPTOR_FLAG_NO_OWNER_ACE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:10569:9, winnt.h:10569:9, winnt.h:10569:9 */
pub const SE_SECURITY_DESCRIPTOR_FLAG_NO_LABEL_ACE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:10570:9, winnt.h:10570:9, winnt.h:10570:9 */
pub const SE_SECURITY_DESCRIPTOR_VALID_FLAGS: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:10571:9, winnt.h:10571:9, winnt.h:10571:9 */
pub const SE_CREATE_TOKEN_NAME: &'static str = "SeCreateTokenPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeCreateTokenPrivilege", Narrow)] } */ /* winnt.h:10610:9, winnt.h:10610:9, winnt.h:10610:9 */
pub const SE_ASSIGNPRIMARYTOKEN_NAME: &'static str = "SeAssignPrimaryTokenPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeAssignPrimaryTokenPrivilege", Narrow)] } */ /* winnt.h:10611:9, winnt.h:10611:9, winnt.h:10611:9 */
pub const SE_LOCK_MEMORY_NAME: &'static str = "SeLockMemoryPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeLockMemoryPrivilege", Narrow)] } */ /* winnt.h:10612:9, winnt.h:10612:9, winnt.h:10612:9 */
pub const SE_INCREASE_QUOTA_NAME: &'static str = "SeIncreaseQuotaPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeIncreaseQuotaPrivilege", Narrow)] } */ /* winnt.h:10613:9, winnt.h:10613:9, winnt.h:10613:9 */
pub const SE_UNSOLICITED_INPUT_NAME: &'static str = "SeUnsolicitedInputPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeUnsolicitedInputPrivilege", Narrow)] } */ /* winnt.h:10614:9, winnt.h:10614:9, winnt.h:10614:9 */
pub const SE_MACHINE_ACCOUNT_NAME: &'static str = "SeMachineAccountPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeMachineAccountPrivilege", Narrow)] } */ /* winnt.h:10615:9, winnt.h:10615:9, winnt.h:10615:9 */
pub const SE_TCB_NAME: &'static str = "SeTcbPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeTcbPrivilege", Narrow)] } */ /* winnt.h:10616:9, winnt.h:10616:9, winnt.h:10616:9 */
pub const SE_SECURITY_NAME: &'static str = "SeSecurityPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeSecurityPrivilege", Narrow)] } */ /* winnt.h:10617:9, winnt.h:10617:9, winnt.h:10617:9 */
pub const SE_TAKE_OWNERSHIP_NAME: &'static str = "SeTakeOwnershipPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeTakeOwnershipPrivilege", Narrow)] } */ /* winnt.h:10618:9, winnt.h:10618:9, winnt.h:10618:9 */
pub const SE_LOAD_DRIVER_NAME: &'static str = "SeLoadDriverPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeLoadDriverPrivilege", Narrow)] } */ /* winnt.h:10619:9, winnt.h:10619:9, winnt.h:10619:9 */
pub const SE_SYSTEM_PROFILE_NAME: &'static str = "SeSystemProfilePrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeSystemProfilePrivilege", Narrow)] } */ /* winnt.h:10620:9, winnt.h:10620:9, winnt.h:10620:9 */
pub const SE_SYSTEMTIME_NAME: &'static str = "SeSystemtimePrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeSystemtimePrivilege", Narrow)] } */ /* winnt.h:10621:9, winnt.h:10621:9, winnt.h:10621:9 */
pub const SE_PROF_SINGLE_PROCESS_NAME: &'static str = "SeProfileSingleProcessPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeProfileSingleProcessPrivilege", Narrow)] } */ /* winnt.h:10622:9, winnt.h:10622:9, winnt.h:10622:9 */
pub const SE_INC_BASE_PRIORITY_NAME: &'static str = "SeIncreaseBasePriorityPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeIncreaseBasePriorityPrivilege", Narrow)] } */ /* winnt.h:10623:9, winnt.h:10623:9, winnt.h:10623:9 */
pub const SE_CREATE_PAGEFILE_NAME: &'static str = "SeCreatePagefilePrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeCreatePagefilePrivilege", Narrow)] } */ /* winnt.h:10624:9, winnt.h:10624:9, winnt.h:10624:9 */
pub const SE_CREATE_PERMANENT_NAME: &'static str = "SeCreatePermanentPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeCreatePermanentPrivilege", Narrow)] } */ /* winnt.h:10625:9, winnt.h:10625:9, winnt.h:10625:9 */
pub const SE_BACKUP_NAME: &'static str = "SeBackupPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeBackupPrivilege", Narrow)] } */ /* winnt.h:10626:9, winnt.h:10626:9, winnt.h:10626:9 */
pub const SE_RESTORE_NAME: &'static str = "SeRestorePrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeRestorePrivilege", Narrow)] } */ /* winnt.h:10627:9, winnt.h:10627:9, winnt.h:10627:9 */
pub const SE_SHUTDOWN_NAME: &'static str = "SeShutdownPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeShutdownPrivilege", Narrow)] } */ /* winnt.h:10628:9, winnt.h:10628:9, winnt.h:10628:9 */
pub const SE_DEBUG_NAME: &'static str = "SeDebugPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeDebugPrivilege", Narrow)] } */ /* winnt.h:10629:9, winnt.h:10629:9, winnt.h:10629:9 */
pub const SE_AUDIT_NAME: &'static str = "SeAuditPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeAuditPrivilege", Narrow)] } */ /* winnt.h:10630:9, winnt.h:10630:9, winnt.h:10630:9 */
pub const SE_SYSTEM_ENVIRONMENT_NAME: &'static str = "SeSystemEnvironmentPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeSystemEnvironmentPrivilege", Narrow)] } */ /* winnt.h:10631:9, winnt.h:10631:9, winnt.h:10631:9 */
pub const SE_CHANGE_NOTIFY_NAME: &'static str = "SeChangeNotifyPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeChangeNotifyPrivilege", Narrow)] } */ /* winnt.h:10632:9, winnt.h:10632:9, winnt.h:10632:9 */
pub const SE_REMOTE_SHUTDOWN_NAME: &'static str = "SeRemoteShutdownPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeRemoteShutdownPrivilege", Narrow)] } */ /* winnt.h:10633:9, winnt.h:10633:9, winnt.h:10633:9 */
pub const SE_UNDOCK_NAME: &'static str = "SeUndockPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeUndockPrivilege", Narrow)] } */ /* winnt.h:10634:9, winnt.h:10634:9, winnt.h:10634:9 */
pub const SE_SYNC_AGENT_NAME: &'static str = "SeSyncAgentPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeSyncAgentPrivilege", Narrow)] } */ /* winnt.h:10635:9, winnt.h:10635:9, winnt.h:10635:9 */
pub const SE_ENABLE_DELEGATION_NAME: &'static str = "SeEnableDelegationPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeEnableDelegationPrivilege", Narrow)] } */ /* winnt.h:10636:9, winnt.h:10636:9, winnt.h:10636:9 */
pub const SE_MANAGE_VOLUME_NAME: &'static str = "SeManageVolumePrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeManageVolumePrivilege", Narrow)] } */ /* winnt.h:10637:9, winnt.h:10637:9, winnt.h:10637:9 */
pub const SE_IMPERSONATE_NAME: &'static str = "SeImpersonatePrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeImpersonatePrivilege", Narrow)] } */ /* winnt.h:10638:9, winnt.h:10638:9, winnt.h:10638:9 */
pub const SE_CREATE_GLOBAL_NAME: &'static str = "SeCreateGlobalPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeCreateGlobalPrivilege", Narrow)] } */ /* winnt.h:10639:9, winnt.h:10639:9, winnt.h:10639:9 */
pub const SE_TRUSTED_CREDMAN_ACCESS_NAME: &'static str = "SeTrustedCredManAccessPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeTrustedCredManAccessPrivilege", Narrow)] } */ /* winnt.h:10640:9, winnt.h:10640:9, winnt.h:10640:9 */
pub const SE_RELABEL_NAME: &'static str = "SeRelabelPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeRelabelPrivilege", Narrow)] } */ /* winnt.h:10641:9, winnt.h:10641:9, winnt.h:10641:9 */
pub const SE_INC_WORKING_SET_NAME: &'static str = "SeIncreaseWorkingSetPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeIncreaseWorkingSetPrivilege", Narrow)] } */ /* winnt.h:10642:9, winnt.h:10642:9, winnt.h:10642:9 */
pub const SE_TIME_ZONE_NAME: &'static str = "SeTimeZonePrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeTimeZonePrivilege", Narrow)] } */ /* winnt.h:10643:9, winnt.h:10643:9, winnt.h:10643:9 */
pub const SE_CREATE_SYMBOLIC_LINK_NAME: &'static str = "SeCreateSymbolicLinkPrivilege"; /* Call { subject: Ident("TEXT"), args: [String("SeCreateSymbolicLinkPrivilege", Narrow)] } */ /* winnt.h:10644:9, winnt.h:10644:9, winnt.h:10644:9 */
pub const TOKEN_MANDATORY_POLICY_OFF: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:10876:9, winnt.h:10876:9, winnt.h:10876:9 */
pub const TOKEN_MANDATORY_POLICY_NO_WRITE_UP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:10877:9, winnt.h:10877:9, winnt.h:10877:9 */
pub const TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:10878:9, winnt.h:10878:9, winnt.h:10878:9 */
pub const TOKEN_SOURCE_LENGTH: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:10912:9, winnt.h:10912:9, winnt.h:10912:9 */
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INVALID: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:10977:9, winnt.h:10977:9, winnt.h:10977:9 */
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INT64: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:10979:9, winnt.h:10979:9, winnt.h:10979:9 */
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_UINT64: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:10980:9, winnt.h:10980:9, winnt.h:10980:9 */
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_STRING: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:10988:9, winnt.h:10988:9, winnt.h:10988:9 */
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_FQBN: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:10999:9, winnt.h:10999:9, winnt.h:10999:9 */
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_SID: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:11001:9, winnt.h:11001:9, winnt.h:11001:9 */
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:11003:9, winnt.h:11003:9, winnt.h:11003:9 */
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:11012:9, winnt.h:11012:9, winnt.h:11012:9 */
pub const CLAIM_SECURITY_ATTRIBUTE_NON_INHERITABLE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11022:9, winnt.h:11022:9, winnt.h:11022:9 */
pub const CLAIM_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:11031:9, winnt.h:11031:9, winnt.h:11031:9 */
pub const CLAIM_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:11037:9, winnt.h:11037:9, winnt.h:11037:9 */
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:11043:9, winnt.h:11043:9, winnt.h:11043:9 */
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:11049:9, winnt.h:11049:9, winnt.h:11049:9 */
pub const CLAIM_SECURITY_ATTRIBUTE_MANDATORY: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:11055:9, winnt.h:11055:9, winnt.h:11055:9 */
pub const CLAIM_SECURITY_ATTRIBUTE_CUSTOM_FLAGS: i32 = 0xffff0000i32; /* Integer(4294901760, Yes, Unknown) */ /* winnt.h:11071:9, winnt.h:11071:9, winnt.h:11071:9 */
pub const CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11190:9, winnt.h:11190:9, winnt.h:11190:9 */
#[doc(inline)] pub use ::winnt::CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1 as CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION; /* winnt.h:11192:9, winnt.h:11192:9, winnt.h:11192:9 */
pub const DISABLE_MAX_PRIVILEGE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11252:9, winnt.h:11252:9, winnt.h:11252:9 */
pub const SANDBOX_INERT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:11253:9, winnt.h:11253:9, winnt.h:11253:9 */
pub const LUA_TOKEN: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:11254:9, winnt.h:11254:9, winnt.h:11254:9 */
pub const WRITE_RESTRICTED: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:11255:9, winnt.h:11255:9, winnt.h:11255:9 */
pub const SE_LEARNING_MODE_FLAG_PERMISSIVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11285:9, winnt.h:11285:9, winnt.h:11285:9 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub const MAXIMUM_PROC_PER_GROUP: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:11322:9, winnt.h:11322:9 */
#[doc(inline)] pub use ::winnt::MAXIMUM_PROC_PER_GROUP as MAXIMUM_PROCESSORS; /* winnt.h:11326:9, winnt.h:11326:9, winnt.h:11326:9 */
pub const FLS_MAXIMUM_AVAILABLE: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:11362:9, winnt.h:11362:9, winnt.h:11362:9 */
pub const TLS_MINIMUM_AVAILABLE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:11363:9, winnt.h:11363:9, winnt.h:11363:9 */
pub const THREAD_BASE_PRIORITY_LOWRT: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:11434:9, winnt.h:11434:9, winnt.h:11434:9 */
pub const THREAD_BASE_PRIORITY_MAX: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:11435:9, winnt.h:11435:9, winnt.h:11435:9 */
pub const QUOTA_LIMITS_HARDWS_MIN_ENABLE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11454:9, winnt.h:11454:9, winnt.h:11454:9 */
pub const QUOTA_LIMITS_HARDWS_MIN_DISABLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:11455:9, winnt.h:11455:9, winnt.h:11455:9 */
pub const QUOTA_LIMITS_HARDWS_MAX_ENABLE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:11456:9, winnt.h:11456:9, winnt.h:11456:9 */
pub const QUOTA_LIMITS_HARDWS_MAX_DISABLE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:11457:9, winnt.h:11457:9, winnt.h:11457:9 */
pub const QUOTA_LIMITS_USE_DEFAULT_LIMITS: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:11458:9, winnt.h:11458:9, winnt.h:11458:9 */
pub const MAX_HW_COUNTERS: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:11493:9, winnt.h:11493:9, winnt.h:11493:9 */
pub const THREAD_PROFILING_FLAG_DISPATCH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11494:9, winnt.h:11494:9, winnt.h:11494:9 */
pub const JOB_OBJECT_TERMINATE_AT_END_OF_JOB: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:11713:9, winnt.h:11713:9, winnt.h:11713:9 */
pub const JOB_OBJECT_POST_AT_END_OF_JOB: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11714:9, winnt.h:11714:9, winnt.h:11714:9 */
pub const JOB_OBJECT_MSG_END_OF_JOB_TIME: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11722:9, winnt.h:11722:9, winnt.h:11722:9 */
pub const JOB_OBJECT_MSG_END_OF_PROCESS_TIME: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:11723:9, winnt.h:11723:9, winnt.h:11723:9 */
pub const JOB_OBJECT_MSG_ACTIVE_PROCESS_LIMIT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:11724:9, winnt.h:11724:9, winnt.h:11724:9 */
pub const JOB_OBJECT_MSG_ACTIVE_PROCESS_ZERO: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:11725:9, winnt.h:11725:9, winnt.h:11725:9 */
pub const JOB_OBJECT_MSG_NEW_PROCESS: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:11726:9, winnt.h:11726:9, winnt.h:11726:9 */
pub const JOB_OBJECT_MSG_EXIT_PROCESS: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:11727:9, winnt.h:11727:9, winnt.h:11727:9 */
pub const JOB_OBJECT_MSG_ABNORMAL_EXIT_PROCESS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:11728:9, winnt.h:11728:9, winnt.h:11728:9 */
pub const JOB_OBJECT_MSG_PROCESS_MEMORY_LIMIT: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:11729:9, winnt.h:11729:9, winnt.h:11729:9 */
pub const JOB_OBJECT_MSG_JOB_MEMORY_LIMIT: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:11730:9, winnt.h:11730:9, winnt.h:11730:9 */
pub const JOB_OBJECT_MSG_NOTIFICATION_LIMIT: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:11731:9, winnt.h:11731:9, winnt.h:11731:9 */
pub const JOB_OBJECT_MSG_JOB_CYCLE_TIME_LIMIT: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:11732:9, winnt.h:11732:9, winnt.h:11732:9 */
pub const JOB_OBJECT_MSG_MINIMUM: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11738:9, winnt.h:11738:9, winnt.h:11738:9 */
pub const JOB_OBJECT_MSG_MAXIMUM: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:11739:9, winnt.h:11739:9, winnt.h:11739:9 */
pub const JOB_OBJECT_LIMIT_WORKINGSET: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11748:9, winnt.h:11748:9, winnt.h:11748:9 */
pub const JOB_OBJECT_LIMIT_PROCESS_TIME: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:11749:9, winnt.h:11749:9, winnt.h:11749:9 */
pub const JOB_OBJECT_LIMIT_JOB_TIME: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:11750:9, winnt.h:11750:9, winnt.h:11750:9 */
pub const JOB_OBJECT_LIMIT_ACTIVE_PROCESS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:11751:9, winnt.h:11751:9, winnt.h:11751:9 */
pub const JOB_OBJECT_LIMIT_AFFINITY: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:11752:9, winnt.h:11752:9, winnt.h:11752:9 */
pub const JOB_OBJECT_LIMIT_PRIORITY_CLASS: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:11753:9, winnt.h:11753:9, winnt.h:11753:9 */
pub const JOB_OBJECT_LIMIT_PRESERVE_JOB_TIME: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:11754:9, winnt.h:11754:9, winnt.h:11754:9 */
pub const JOB_OBJECT_LIMIT_SCHEDULING_CLASS: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:11755:9, winnt.h:11755:9, winnt.h:11755:9 */
pub const JOB_OBJECT_LIMIT_PROCESS_MEMORY: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:11760:9, winnt.h:11760:9, winnt.h:11760:9 */
pub const JOB_OBJECT_LIMIT_JOB_MEMORY: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:11761:9, winnt.h:11761:9, winnt.h:11761:9 */
pub const JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnt.h:11762:9, winnt.h:11762:9, winnt.h:11762:9 */
pub const JOB_OBJECT_LIMIT_BREAKAWAY_OK: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winnt.h:11763:9, winnt.h:11763:9, winnt.h:11763:9 */
pub const JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winnt.h:11764:9, winnt.h:11764:9, winnt.h:11764:9 */
pub const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winnt.h:11765:9, winnt.h:11765:9, winnt.h:11765:9 */
pub const JOB_OBJECT_LIMIT_SUBSET_AFFINITY: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winnt.h:11766:9, winnt.h:11766:9, winnt.h:11766:9 */
pub const JOB_OBJECT_LIMIT_JOB_READ_BYTES: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winnt.h:11772:9, winnt.h:11772:9, winnt.h:11772:9 */
pub const JOB_OBJECT_LIMIT_JOB_WRITE_BYTES: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winnt.h:11773:9, winnt.h:11773:9, winnt.h:11773:9 */
pub const JOB_OBJECT_LIMIT_RATE_CONTROL: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* winnt.h:11774:9, winnt.h:11774:9, winnt.h:11774:9 */
pub const JOB_OBJECT_LIMIT_RESERVED3: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:11780:9, winnt.h:11780:9, winnt.h:11780:9 */
pub const JOB_OBJECT_LIMIT_VALID_FLAGS: i32 = 0x7ffffi32; /* Integer(524287, Yes, Unknown) */ /* winnt.h:11786:9, winnt.h:11786:9, winnt.h:11786:9 */
pub const JOB_OBJECT_BASIC_LIMIT_VALID_FLAGS: i32 = 0xffi32; /* Integer(255, Yes, Unknown) */ /* winnt.h:11787:9, winnt.h:11787:9, winnt.h:11787:9 */
pub const JOB_OBJECT_EXTENDED_LIMIT_VALID_FLAGS: i32 = 0x7fffi32; /* Integer(32767, Yes, Unknown) */ /* winnt.h:11788:9, winnt.h:11788:9, winnt.h:11788:9 */
pub const JOB_OBJECT_NOTIFICATION_LIMIT_VALID_FLAGS: i32 = 0x70204i32; /* Integer(459268, Yes, Unknown) */ /* winnt.h:11789:9, winnt.h:11789:9, winnt.h:11789:9 */
pub const JOB_OBJECT_RESERVED_LIMIT_VALID_FLAGS: i32 = 0x7ffffi32; /* Integer(524287, Yes, Unknown) */ /* winnt.h:11790:9, winnt.h:11790:9, winnt.h:11790:9 */
pub const JOB_OBJECT_UILIMIT_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:11796:9, winnt.h:11796:9, winnt.h:11796:9 */
pub const JOB_OBJECT_UILIMIT_HANDLES: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11798:9, winnt.h:11798:9, winnt.h:11798:9 */
pub const JOB_OBJECT_UILIMIT_READCLIPBOARD: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:11799:9, winnt.h:11799:9, winnt.h:11799:9 */
pub const JOB_OBJECT_UILIMIT_WRITECLIPBOARD: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:11800:9, winnt.h:11800:9, winnt.h:11800:9 */
pub const JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:11801:9, winnt.h:11801:9, winnt.h:11801:9 */
pub const JOB_OBJECT_UILIMIT_DISPLAYSETTINGS: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:11802:9, winnt.h:11802:9, winnt.h:11802:9 */
pub const JOB_OBJECT_UILIMIT_GLOBALATOMS: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:11803:9, winnt.h:11803:9, winnt.h:11803:9 */
pub const JOB_OBJECT_UILIMIT_DESKTOP: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:11804:9, winnt.h:11804:9, winnt.h:11804:9 */
pub const JOB_OBJECT_UILIMIT_EXITWINDOWS: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:11805:9, winnt.h:11805:9, winnt.h:11805:9 */
pub const JOB_OBJECT_UILIMIT_ALL: i32 = 0xffi32; /* Integer(255, Yes, Unknown) */ /* winnt.h:11807:9, winnt.h:11807:9, winnt.h:11807:9 */
pub const JOB_OBJECT_UI_VALID_FLAGS: i32 = 0xffi32; /* Integer(255, Yes, Unknown) */ /* winnt.h:11809:9, winnt.h:11809:9, winnt.h:11809:9 */
pub const JOB_OBJECT_SECURITY_NO_ADMIN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11811:9, winnt.h:11811:9, winnt.h:11811:9 */
pub const JOB_OBJECT_SECURITY_RESTRICTED_TOKEN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:11812:9, winnt.h:11812:9, winnt.h:11812:9 */
pub const JOB_OBJECT_SECURITY_ONLY_TOKEN: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:11813:9, winnt.h:11813:9, winnt.h:11813:9 */
pub const JOB_OBJECT_SECURITY_FILTER_TOKENS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:11814:9, winnt.h:11814:9, winnt.h:11814:9 */
pub const JOB_OBJECT_SECURITY_VALID_FLAGS: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:11816:9, winnt.h:11816:9, winnt.h:11816:9 */
pub const JOB_OBJECT_CPU_RATE_CONTROL_ENABLE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11822:9, winnt.h:11822:9, winnt.h:11822:9 */
pub const JOB_OBJECT_CPU_RATE_CONTROL_WEIGHT_BASED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:11823:9, winnt.h:11823:9, winnt.h:11823:9 */
pub const JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:11824:9, winnt.h:11824:9, winnt.h:11824:9 */
pub const JOB_OBJECT_CPU_RATE_CONTROL_NOTIFY: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:11825:9, winnt.h:11825:9, winnt.h:11825:9 */
pub const JOB_OBJECT_CPU_RATE_CONTROL_VALID_FLAGS: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:11826:9, winnt.h:11826:9, winnt.h:11826:9 */
pub const EVENT_MODIFY_STATE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:11866:9, winnt.h:11866:9, winnt.h:11866:9 */
pub const MUTANT_QUERY_STATE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11869:9, winnt.h:11869:9, winnt.h:11869:9 */
pub const SEMAPHORE_MODIFY_STATE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:11874:9, winnt.h:11874:9, winnt.h:11874:9 */
pub const TIMER_QUERY_STATE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11881:9, winnt.h:11881:9, winnt.h:11881:9 */
pub const TIMER_MODIFY_STATE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:11882:9, winnt.h:11882:9, winnt.h:11882:9 */
pub const TIME_ZONE_ID_UNKNOWN: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:11889:9, winnt.h:11889:9, winnt.h:11889:9 */
pub const TIME_ZONE_ID_STANDARD: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11890:9, winnt.h:11890:9, winnt.h:11890:9 */
pub const TIME_ZONE_ID_DAYLIGHT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:11891:9, winnt.h:11891:9, winnt.h:11891:9 */
pub const LTP_PC_SMT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:11904:9, winnt.h:11904:9, winnt.h:11904:9 */
pub const CACHE_FULLY_ASSOCIATIVE: i32 = 0xffi32; /* Integer(255, Yes, Unknown) */ /* winnt.h:11913:9, winnt.h:11913:9, winnt.h:11913:9 */
pub const PROCESSOR_INTEL_386: i32 = 0x182i32; /* Integer(386, Yes, Unknown) */ /* winnt.h:11995:9, winnt.h:11995:9, winnt.h:11995:9 */
pub const PROCESSOR_INTEL_486: i32 = 0x1e6i32; /* Integer(486, Yes, Unknown) */ /* winnt.h:11996:9, winnt.h:11996:9, winnt.h:11996:9 */
pub const PROCESSOR_INTEL_PENTIUM: i32 = 0x24ai32; /* Integer(586, Yes, Unknown) */ /* winnt.h:11997:9, winnt.h:11997:9, winnt.h:11997:9 */
pub const PROCESSOR_INTEL_IA64: i32 = 0x898i32; /* Integer(2200, Yes, Unknown) */ /* winnt.h:11998:9, winnt.h:11998:9, winnt.h:11998:9 */
pub const PROCESSOR_AMD_X8664: i32 = 0x21d8i32; /* Integer(8664, Yes, Unknown) */ /* winnt.h:11999:9, winnt.h:11999:9, winnt.h:11999:9 */
pub const PROCESSOR_MIPS_R4000: i32 = 0xfa0i32; /* Integer(4000, Yes, Unknown) */ /* winnt.h:12000:9, winnt.h:12000:9, winnt.h:12000:9 */
pub const PROCESSOR_ALPHA_21064: i32 = 0x5248i32; /* Integer(21064, Yes, Unknown) */ /* winnt.h:12001:9, winnt.h:12001:9, winnt.h:12001:9 */
pub const PROCESSOR_PPC_601: i32 = 0x259i32; /* Integer(601, Yes, Unknown) */ /* winnt.h:12002:9, winnt.h:12002:9, winnt.h:12002:9 */
pub const PROCESSOR_PPC_603: i32 = 0x25bi32; /* Integer(603, Yes, Unknown) */ /* winnt.h:12003:9, winnt.h:12003:9, winnt.h:12003:9 */
pub const PROCESSOR_PPC_604: i32 = 0x25ci32; /* Integer(604, Yes, Unknown) */ /* winnt.h:12004:9, winnt.h:12004:9, winnt.h:12004:9 */
pub const PROCESSOR_PPC_620: i32 = 0x26ci32; /* Integer(620, Yes, Unknown) */ /* winnt.h:12005:9, winnt.h:12005:9, winnt.h:12005:9 */
pub const PROCESSOR_HITACHI_SH3: i32 = 0x2713i32; /* Integer(10003, Yes, Unknown) */ /* winnt.h:12006:9, winnt.h:12006:9, winnt.h:12006:9 */
pub const PROCESSOR_HITACHI_SH3E: i32 = 0x2714i32; /* Integer(10004, Yes, Unknown) */ /* winnt.h:12007:9, winnt.h:12007:9, winnt.h:12007:9 */
pub const PROCESSOR_HITACHI_SH4: i32 = 0x2715i32; /* Integer(10005, Yes, Unknown) */ /* winnt.h:12008:9, winnt.h:12008:9, winnt.h:12008:9 */
pub const PROCESSOR_MOTOROLA_821: i32 = 0x335i32; /* Integer(821, Yes, Unknown) */ /* winnt.h:12009:9, winnt.h:12009:9, winnt.h:12009:9 */
pub const PROCESSOR_SHx_SH3: i32 = 0x67i32; /* Integer(103, Yes, Unknown) */ /* winnt.h:12010:9, winnt.h:12010:9, winnt.h:12010:9 */
pub const PROCESSOR_SHx_SH4: i32 = 0x68i32; /* Integer(104, Yes, Unknown) */ /* winnt.h:12011:9, winnt.h:12011:9, winnt.h:12011:9 */
pub const PROCESSOR_STRONGARM: i32 = 0xa11i32; /* Integer(2577, Yes, Unknown) */ /* winnt.h:12012:9, winnt.h:12012:9, winnt.h:12012:9 */
pub const PROCESSOR_ARM720: i32 = 0x720i32; /* Integer(1824, Yes, Unknown) */ /* winnt.h:12013:9, winnt.h:12013:9, winnt.h:12013:9 */
pub const PROCESSOR_ARM820: i32 = 0x820i32; /* Integer(2080, Yes, Unknown) */ /* winnt.h:12014:9, winnt.h:12014:9, winnt.h:12014:9 */
pub const PROCESSOR_ARM920: i32 = 0x920i32; /* Integer(2336, Yes, Unknown) */ /* winnt.h:12015:9, winnt.h:12015:9, winnt.h:12015:9 */
pub const PROCESSOR_ARM_7TDMI: i32 = 0x11171i32; /* Integer(70001, Yes, Unknown) */ /* winnt.h:12016:9, winnt.h:12016:9, winnt.h:12016:9 */
pub const PROCESSOR_OPTIL: i32 = 0x494fi32; /* Integer(18767, Yes, Unknown) */ /* winnt.h:12017:9, winnt.h:12017:9, winnt.h:12017:9 */
pub const PROCESSOR_ARCHITECTURE_INTEL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:12019:9, winnt.h:12019:9, winnt.h:12019:9 */
pub const PROCESSOR_ARCHITECTURE_MIPS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12020:9, winnt.h:12020:9, winnt.h:12020:9 */
pub const PROCESSOR_ARCHITECTURE_ALPHA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:12021:9, winnt.h:12021:9, winnt.h:12021:9 */
pub const PROCESSOR_ARCHITECTURE_PPC: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:12022:9, winnt.h:12022:9, winnt.h:12022:9 */
pub const PROCESSOR_ARCHITECTURE_SHX: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:12023:9, winnt.h:12023:9, winnt.h:12023:9 */
pub const PROCESSOR_ARCHITECTURE_ARM: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:12024:9, winnt.h:12024:9, winnt.h:12024:9 */
pub const PROCESSOR_ARCHITECTURE_IA64: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:12025:9, winnt.h:12025:9, winnt.h:12025:9 */
pub const PROCESSOR_ARCHITECTURE_ALPHA64: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:12026:9, winnt.h:12026:9, winnt.h:12026:9 */
pub const PROCESSOR_ARCHITECTURE_MSIL: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:12027:9, winnt.h:12027:9, winnt.h:12027:9 */
pub const PROCESSOR_ARCHITECTURE_AMD64: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:12028:9, winnt.h:12028:9, winnt.h:12028:9 */
pub const PROCESSOR_ARCHITECTURE_IA32_ON_WIN64: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:12029:9, winnt.h:12029:9, winnt.h:12029:9 */
pub const PROCESSOR_ARCHITECTURE_NEUTRAL: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:12030:9, winnt.h:12030:9, winnt.h:12030:9 */
pub const PROCESSOR_ARCHITECTURE_UNKNOWN: i32 = 0xffffi32; /* Integer(65535, Yes, Unknown) */ /* winnt.h:12032:9, winnt.h:12032:9, winnt.h:12032:9 */
pub const PF_FLOATING_POINT_PRECISION_ERRATA: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:12034:9, winnt.h:12034:9, winnt.h:12034:9 */
pub const PF_FLOATING_POINT_EMULATED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12035:9, winnt.h:12035:9, winnt.h:12035:9 */
pub const PF_COMPARE_EXCHANGE_DOUBLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:12036:9, winnt.h:12036:9, winnt.h:12036:9 */
pub const PF_MMX_INSTRUCTIONS_AVAILABLE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:12037:9, winnt.h:12037:9, winnt.h:12037:9 */
pub const PF_PPC_MOVEMEM_64BIT_OK: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:12038:9, winnt.h:12038:9, winnt.h:12038:9 */
pub const PF_ALPHA_BYTE_INSTRUCTIONS: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:12039:9, winnt.h:12039:9, winnt.h:12039:9 */
pub const PF_XMMI_INSTRUCTIONS_AVAILABLE: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:12040:9, winnt.h:12040:9, winnt.h:12040:9 */
pub const PF_3DNOW_INSTRUCTIONS_AVAILABLE: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:12041:9, winnt.h:12041:9, winnt.h:12041:9 */
pub const PF_RDTSC_INSTRUCTION_AVAILABLE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:12042:9, winnt.h:12042:9, winnt.h:12042:9 */
pub const PF_PAE_ENABLED: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:12043:9, winnt.h:12043:9, winnt.h:12043:9 */
pub const PF_XMMI64_INSTRUCTIONS_AVAILABLE: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:12044:9, winnt.h:12044:9, winnt.h:12044:9 */
pub const PF_SSE_DAZ_MODE_AVAILABLE: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:12045:9, winnt.h:12045:9, winnt.h:12045:9 */
pub const PF_NX_ENABLED: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:12046:9, winnt.h:12046:9, winnt.h:12046:9 */
pub const PF_SSE3_INSTRUCTIONS_AVAILABLE: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:12047:9, winnt.h:12047:9, winnt.h:12047:9 */
pub const PF_COMPARE_EXCHANGE128: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:12048:9, winnt.h:12048:9, winnt.h:12048:9 */
pub const PF_COMPARE64_EXCHANGE128: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:12049:9, winnt.h:12049:9, winnt.h:12049:9 */
pub const PF_CHANNELS_ENABLED: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:12050:9, winnt.h:12050:9, winnt.h:12050:9 */
pub const PF_XSAVE_ENABLED: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winnt.h:12051:9, winnt.h:12051:9, winnt.h:12051:9 */
pub const PF_ARM_VFP_32_REGISTERS_AVAILABLE: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:12052:9, winnt.h:12052:9, winnt.h:12052:9 */
pub const PF_ARM_NEON_INSTRUCTIONS_AVAILABLE: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winnt.h:12053:9, winnt.h:12053:9, winnt.h:12053:9 */
pub const PF_SECOND_LEVEL_ADDRESS_TRANSLATION: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnt.h:12054:9, winnt.h:12054:9, winnt.h:12054:9 */
pub const PF_VIRT_FIRMWARE_ENABLED: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winnt.h:12055:9, winnt.h:12055:9, winnt.h:12055:9 */
pub const PF_RDWRFSGSBASE_AVAILABLE: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winnt.h:12056:9, winnt.h:12056:9, winnt.h:12056:9 */
pub const PF_FASTFAIL_AVAILABLE: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winnt.h:12057:9, winnt.h:12057:9, winnt.h:12057:9 */
pub const PF_ARM_DIVIDE_INSTRUCTION_AVAILABLE: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winnt.h:12058:9, winnt.h:12058:9, winnt.h:12058:9 */
pub const PF_ARM_64BIT_LOADSTORE_ATOMIC: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* winnt.h:12059:9, winnt.h:12059:9, winnt.h:12059:9 */
pub const PF_ARM_EXTERNAL_CACHE_AVAILABLE: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* winnt.h:12060:9, winnt.h:12060:9, winnt.h:12060:9 */
pub const PF_ARM_FMAC_INSTRUCTIONS_AVAILABLE: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* winnt.h:12061:9, winnt.h:12061:9, winnt.h:12061:9 */
pub const PF_RDRAND_INSTRUCTION_AVAILABLE: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* winnt.h:12062:9, winnt.h:12062:9, winnt.h:12062:9 */
pub const SECTION_QUERY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12140:9, winnt.h:12140:9, winnt.h:12140:9 */
pub const SECTION_MAP_WRITE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:12141:9, winnt.h:12141:9, winnt.h:12141:9 */
pub const SECTION_MAP_READ: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:12142:9, winnt.h:12142:9, winnt.h:12142:9 */
pub const SECTION_MAP_EXECUTE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:12143:9, winnt.h:12143:9, winnt.h:12143:9 */
pub const SECTION_EXTEND_SIZE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:12144:9, winnt.h:12144:9, winnt.h:12144:9 */
pub const SECTION_MAP_EXECUTE_EXPLICIT: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:12145:9, winnt.h:12145:9, winnt.h:12145:9 */
pub const SESSION_QUERY_ACCESS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12153:9, winnt.h:12153:9, winnt.h:12153:9 */
pub const SESSION_MODIFY_ACCESS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:12154:9, winnt.h:12154:9, winnt.h:12154:9 */
pub const PAGE_NOACCESS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12160:9, winnt.h:12160:9, winnt.h:12160:9 */
pub const PAGE_READONLY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:12161:9, winnt.h:12161:9, winnt.h:12161:9 */
pub const PAGE_READWRITE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:12162:9, winnt.h:12162:9, winnt.h:12162:9 */
pub const PAGE_WRITECOPY: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:12163:9, winnt.h:12163:9, winnt.h:12163:9 */
#[cfg(feature="winapi_desktop")] pub const PAGE_EXECUTE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:12166:9, winnt.h:12166:9, winnt.h:12166:9 */
#[cfg(feature="winapi_desktop")] pub const PAGE_EXECUTE_READ: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:12167:9, winnt.h:12167:9, winnt.h:12167:9 */
#[cfg(feature="winapi_desktop")] pub const PAGE_EXECUTE_READWRITE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:12168:9, winnt.h:12168:9, winnt.h:12168:9 */
#[cfg(feature="winapi_desktop")] pub const PAGE_EXECUTE_WRITECOPY: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:12169:9, winnt.h:12169:9, winnt.h:12169:9 */
pub const PAGE_GUARD: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:12172:9, winnt.h:12172:9, winnt.h:12172:9 */
pub const PAGE_NOCACHE: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:12173:9, winnt.h:12173:9, winnt.h:12173:9 */
pub const PAGE_WRITECOMBINE: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnt.h:12174:9, winnt.h:12174:9, winnt.h:12174:9 */
pub const PAGE_REVERT_TO_FILE_MAP: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:12175:9, winnt.h:12175:9, winnt.h:12175:9 */
pub const MEM_COMMIT: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winnt.h:12176:9, winnt.h:12176:9, winnt.h:12176:9 */
pub const MEM_RESERVE: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winnt.h:12177:9, winnt.h:12177:9, winnt.h:12177:9 */
pub const MEM_DECOMMIT: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winnt.h:12178:9, winnt.h:12178:9, winnt.h:12178:9 */
pub const MEM_RELEASE: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:12179:9, winnt.h:12179:9, winnt.h:12179:9 */
pub const MEM_FREE: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winnt.h:12180:9, winnt.h:12180:9, winnt.h:12180:9 */
pub const MEM_PRIVATE: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winnt.h:12181:9, winnt.h:12181:9, winnt.h:12181:9 */
pub const MEM_MAPPED: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* winnt.h:12182:9, winnt.h:12182:9, winnt.h:12182:9 */
pub const MEM_RESET: i32 = 0x80000i32; /* Integer(524288, Yes, Unknown) */ /* winnt.h:12183:9, winnt.h:12183:9, winnt.h:12183:9 */
pub const MEM_TOP_DOWN: i32 = 0x100000i32; /* Integer(1048576, Yes, Unknown) */ /* winnt.h:12184:9, winnt.h:12184:9, winnt.h:12184:9 */
pub const MEM_WRITE_WATCH: i32 = 0x200000i32; /* Integer(2097152, Yes, Unknown) */ /* winnt.h:12185:9, winnt.h:12185:9, winnt.h:12185:9 */
pub const MEM_PHYSICAL: i32 = 0x400000i32; /* Integer(4194304, Yes, Unknown) */ /* winnt.h:12186:9, winnt.h:12186:9, winnt.h:12186:9 */
pub const MEM_ROTATE: i32 = 0x800000i32; /* Integer(8388608, Yes, Unknown) */ /* winnt.h:12187:9, winnt.h:12187:9, winnt.h:12187:9 */
pub const MEM_DIFFERENT_IMAGE_BASE_OK: i32 = 0x800000i32; /* Integer(8388608, Yes, Unknown) */ /* winnt.h:12188:9, winnt.h:12188:9, winnt.h:12188:9 */
pub const MEM_RESET_UNDO: i32 = 0x1000000i32; /* Integer(16777216, Yes, Unknown) */ /* winnt.h:12189:9, winnt.h:12189:9, winnt.h:12189:9 */
pub const MEM_LARGE_PAGES: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* winnt.h:12190:9, winnt.h:12190:9, winnt.h:12190:9 */
pub const MEM_4MB_PAGES: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:12191:9, winnt.h:12191:9, winnt.h:12191:9 */
pub const SEC_FILE: i32 = 0x800000i32; /* Integer(8388608, Yes, Unknown) */ /* winnt.h:12192:9, winnt.h:12192:9, winnt.h:12192:9 */
pub const SEC_IMAGE: i32 = 0x1000000i32; /* Integer(16777216, Yes, Unknown) */ /* winnt.h:12193:9, winnt.h:12193:9, winnt.h:12193:9 */
pub const SEC_PROTECTED_IMAGE: i32 = 0x2000000i32; /* Integer(33554432, Yes, Unknown) */ /* winnt.h:12194:9, winnt.h:12194:9, winnt.h:12194:9 */
pub const SEC_RESERVE: i32 = 0x4000000i32; /* Integer(67108864, Yes, Unknown) */ /* winnt.h:12195:9, winnt.h:12195:9, winnt.h:12195:9 */
pub const SEC_COMMIT: i32 = 0x8000000i32; /* Integer(134217728, Yes, Unknown) */ /* winnt.h:12196:9, winnt.h:12196:9, winnt.h:12196:9 */
pub const SEC_NOCACHE: i32 = 0x10000000i32; /* Integer(268435456, Yes, Unknown) */ /* winnt.h:12197:9, winnt.h:12197:9, winnt.h:12197:9 */
pub const SEC_WRITECOMBINE: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* winnt.h:12198:9, winnt.h:12198:9, winnt.h:12198:9 */
pub const SEC_LARGE_PAGES: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:12199:9, winnt.h:12199:9, winnt.h:12199:9 */
#[doc(inline)] pub use ::winnt::SEC_IMAGE as MEM_IMAGE; /* winnt.h:12201:9, winnt.h:12201:9, winnt.h:12201:9 */
pub const WRITE_WATCH_FLAG_RESET: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12202:9, winnt.h:12202:9, winnt.h:12202:9 */
pub const MEM_UNMAP_WITH_TRANSIENT_BOOST: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12203:9, winnt.h:12203:9, winnt.h:12203:9 */
pub const FILE_SHARE_READ: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12263:9, winnt.h:12263:9, winnt.h:12263:9 */
pub const FILE_SHARE_WRITE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:12264:9, winnt.h:12264:9, winnt.h:12264:9 */
pub const FILE_SHARE_DELETE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:12265:9, winnt.h:12265:9, winnt.h:12265:9 */
pub const FILE_ATTRIBUTE_READONLY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12266:9, winnt.h:12266:9, winnt.h:12266:9 */
pub const FILE_ATTRIBUTE_HIDDEN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:12267:9, winnt.h:12267:9, winnt.h:12267:9 */
pub const FILE_ATTRIBUTE_SYSTEM: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:12268:9, winnt.h:12268:9, winnt.h:12268:9 */
pub const FILE_ATTRIBUTE_DIRECTORY: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:12269:9, winnt.h:12269:9, winnt.h:12269:9 */
pub const FILE_ATTRIBUTE_ARCHIVE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:12270:9, winnt.h:12270:9, winnt.h:12270:9 */
pub const FILE_ATTRIBUTE_DEVICE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:12271:9, winnt.h:12271:9, winnt.h:12271:9 */
pub const FILE_ATTRIBUTE_NORMAL: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:12272:9, winnt.h:12272:9, winnt.h:12272:9 */
pub const FILE_ATTRIBUTE_TEMPORARY: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:12273:9, winnt.h:12273:9, winnt.h:12273:9 */
pub const FILE_ATTRIBUTE_SPARSE_FILE: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:12274:9, winnt.h:12274:9, winnt.h:12274:9 */
pub const FILE_ATTRIBUTE_REPARSE_POINT: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnt.h:12275:9, winnt.h:12275:9, winnt.h:12275:9 */
pub const FILE_ATTRIBUTE_COMPRESSED: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winnt.h:12276:9, winnt.h:12276:9, winnt.h:12276:9 */
pub const FILE_ATTRIBUTE_OFFLINE: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winnt.h:12277:9, winnt.h:12277:9, winnt.h:12277:9 */
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winnt.h:12278:9, winnt.h:12278:9, winnt.h:12278:9 */
pub const FILE_ATTRIBUTE_ENCRYPTED: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winnt.h:12279:9, winnt.h:12279:9, winnt.h:12279:9 */
pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:12280:9, winnt.h:12280:9, winnt.h:12280:9 */
pub const FILE_ATTRIBUTE_VIRTUAL: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winnt.h:12281:9, winnt.h:12281:9, winnt.h:12281:9 */
pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winnt.h:12282:9, winnt.h:12282:9, winnt.h:12282:9 */
pub const FILE_ATTRIBUTE_EA: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* winnt.h:12283:9, winnt.h:12283:9, winnt.h:12283:9 */
pub const FILE_NOTIFY_CHANGE_FILE_NAME: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12284:9, winnt.h:12284:9, winnt.h:12284:9 */
pub const FILE_NOTIFY_CHANGE_DIR_NAME: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:12285:9, winnt.h:12285:9, winnt.h:12285:9 */
pub const FILE_NOTIFY_CHANGE_ATTRIBUTES: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:12286:9, winnt.h:12286:9, winnt.h:12286:9 */
pub const FILE_NOTIFY_CHANGE_SIZE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:12287:9, winnt.h:12287:9, winnt.h:12287:9 */
pub const FILE_NOTIFY_CHANGE_LAST_WRITE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:12288:9, winnt.h:12288:9, winnt.h:12288:9 */
pub const FILE_NOTIFY_CHANGE_LAST_ACCESS: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:12289:9, winnt.h:12289:9, winnt.h:12289:9 */
pub const FILE_NOTIFY_CHANGE_CREATION: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:12290:9, winnt.h:12290:9, winnt.h:12290:9 */
pub const FILE_NOTIFY_CHANGE_SECURITY: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:12291:9, winnt.h:12291:9, winnt.h:12291:9 */
pub const FILE_ACTION_ADDED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12292:9, winnt.h:12292:9, winnt.h:12292:9 */
pub const FILE_ACTION_REMOVED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:12293:9, winnt.h:12293:9, winnt.h:12293:9 */
pub const FILE_ACTION_MODIFIED: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:12294:9, winnt.h:12294:9, winnt.h:12294:9 */
pub const FILE_ACTION_RENAMED_OLD_NAME: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:12295:9, winnt.h:12295:9, winnt.h:12295:9 */
pub const FILE_ACTION_RENAMED_NEW_NAME: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:12296:9, winnt.h:12296:9, winnt.h:12296:9 */
pub const FILE_CASE_SENSITIVE_SEARCH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12299:9, winnt.h:12299:9, winnt.h:12299:9 */
pub const FILE_CASE_PRESERVED_NAMES: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:12300:9, winnt.h:12300:9, winnt.h:12300:9 */
pub const FILE_UNICODE_ON_DISK: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:12301:9, winnt.h:12301:9, winnt.h:12301:9 */
pub const FILE_PERSISTENT_ACLS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:12302:9, winnt.h:12302:9, winnt.h:12302:9 */
pub const FILE_FILE_COMPRESSION: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:12303:9, winnt.h:12303:9, winnt.h:12303:9 */
pub const FILE_VOLUME_QUOTAS: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:12304:9, winnt.h:12304:9, winnt.h:12304:9 */
pub const FILE_SUPPORTS_SPARSE_FILES: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:12305:9, winnt.h:12305:9, winnt.h:12305:9 */
pub const FILE_SUPPORTS_REPARSE_POINTS: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:12306:9, winnt.h:12306:9, winnt.h:12306:9 */
pub const FILE_SUPPORTS_REMOTE_STORAGE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:12307:9, winnt.h:12307:9, winnt.h:12307:9 */
pub const FILE_VOLUME_IS_COMPRESSED: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:12308:9, winnt.h:12308:9, winnt.h:12308:9 */
pub const FILE_SUPPORTS_OBJECT_IDS: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winnt.h:12309:9, winnt.h:12309:9, winnt.h:12309:9 */
pub const FILE_SUPPORTS_ENCRYPTION: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winnt.h:12310:9, winnt.h:12310:9, winnt.h:12310:9 */
pub const FILE_NAMED_STREAMS: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* winnt.h:12311:9, winnt.h:12311:9, winnt.h:12311:9 */
pub const FILE_READ_ONLY_VOLUME: i32 = 0x80000i32; /* Integer(524288, Yes, Unknown) */ /* winnt.h:12312:9, winnt.h:12312:9, winnt.h:12312:9 */
pub const FILE_SEQUENTIAL_WRITE_ONCE: i32 = 0x100000i32; /* Integer(1048576, Yes, Unknown) */ /* winnt.h:12313:9, winnt.h:12313:9, winnt.h:12313:9 */
pub const FILE_SUPPORTS_TRANSACTIONS: i32 = 0x200000i32; /* Integer(2097152, Yes, Unknown) */ /* winnt.h:12314:9, winnt.h:12314:9, winnt.h:12314:9 */
pub const FILE_SUPPORTS_HARD_LINKS: i32 = 0x400000i32; /* Integer(4194304, Yes, Unknown) */ /* winnt.h:12315:9, winnt.h:12315:9, winnt.h:12315:9 */
pub const FILE_SUPPORTS_EXTENDED_ATTRIBUTES: i32 = 0x800000i32; /* Integer(8388608, Yes, Unknown) */ /* winnt.h:12316:9, winnt.h:12316:9, winnt.h:12316:9 */
pub const FILE_SUPPORTS_OPEN_BY_FILE_ID: i32 = 0x1000000i32; /* Integer(16777216, Yes, Unknown) */ /* winnt.h:12317:9, winnt.h:12317:9, winnt.h:12317:9 */
pub const FILE_SUPPORTS_USN_JOURNAL: i32 = 0x2000000i32; /* Integer(33554432, Yes, Unknown) */ /* winnt.h:12318:9, winnt.h:12318:9, winnt.h:12318:9 */
pub const FILE_SUPPORTS_INTEGRITY_STREAMS: i32 = 0x4000000i32; /* Integer(67108864, Yes, Unknown) */ /* winnt.h:12319:9, winnt.h:12319:9, winnt.h:12319:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const FLUSH_FLAGS_FILE_DATA_ONLY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12379:9, winnt.h:12379:9, winnt.h:12379:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const FLUSH_FLAGS_NO_SYNC: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:12388:9, winnt.h:12388:9, winnt.h:12388:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SCRUB_DATA_INPUT_FLAG_RESUME: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12493:9, winnt.h:12493:9, winnt.h:12493:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SCRUB_DATA_INPUT_FLAG_SKIP_IN_SYNC: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:12494:9, winnt.h:12494:9, winnt.h:12494:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SCRUB_DATA_INPUT_FLAG_SKIP_NON_INTEGRITY_DATA: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:12495:9, winnt.h:12495:9, winnt.h:12495:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SCRUB_DATA_OUTPUT_FLAG_INCOMPLETE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12497:9, winnt.h:12497:9, winnt.h:12497:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const SCRUB_DATA_OUTPUT_FLAG_NON_USER_DATA_RANGE: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winnt.h:12499:9, winnt.h:12499:9, winnt.h:12499:9 */
pub const IO_COMPLETION_MODIFY_STATE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:12781:9, winnt.h:12781:9, winnt.h:12781:9 */
pub const DUPLICATE_CLOSE_SOURCE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:12788:9, winnt.h:12788:9, winnt.h:12788:9 */
pub const DUPLICATE_SAME_ACCESS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:12789:9, winnt.h:12789:9, winnt.h:12789:9 */
pub const POWERBUTTON_ACTION_INDEX_NOTHING: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:13179:9, winnt.h:13179:9, winnt.h:13179:9 */
pub const POWERBUTTON_ACTION_INDEX_SLEEP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:13180:9, winnt.h:13180:9, winnt.h:13180:9 */
pub const POWERBUTTON_ACTION_INDEX_HIBERNATE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:13181:9, winnt.h:13181:9, winnt.h:13181:9 */
pub const POWERBUTTON_ACTION_INDEX_SHUTDOWN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:13182:9, winnt.h:13182:9, winnt.h:13182:9 */
pub const POWERBUTTON_ACTION_VALUE_NOTHING: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:13188:9, winnt.h:13188:9, winnt.h:13188:9 */
pub const POWERBUTTON_ACTION_VALUE_SLEEP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:13189:9, winnt.h:13189:9, winnt.h:13189:9 */
pub const POWERBUTTON_ACTION_VALUE_HIBERNATE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:13190:9, winnt.h:13190:9, winnt.h:13190:9 */
pub const POWERBUTTON_ACTION_VALUE_SHUTDOWN: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:13191:9, winnt.h:13191:9, winnt.h:13191:9 */
pub const PERFSTATE_POLICY_CHANGE_IDEAL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:13272:9, winnt.h:13272:9, winnt.h:13272:9 */
pub const PERFSTATE_POLICY_CHANGE_SINGLE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:13273:9, winnt.h:13273:9, winnt.h:13273:9 */
pub const PERFSTATE_POLICY_CHANGE_ROCKET: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:13274:9, winnt.h:13274:9, winnt.h:13274:9 */
pub const PERFSTATE_POLICY_CHANGE_IDEAL_AGGRESSIVE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:13275:9, winnt.h:13275:9, winnt.h:13275:9 */
#[doc(inline)] pub use ::winnt::PERFSTATE_POLICY_CHANGE_ROCKET as PERFSTATE_POLICY_CHANGE_DECREASE_MAX; /* winnt.h:13277:9, winnt.h:13277:9, winnt.h:13277:9 */
#[doc(inline)] pub use ::winnt::PERFSTATE_POLICY_CHANGE_IDEAL_AGGRESSIVE as PERFSTATE_POLICY_CHANGE_INCREASE_MAX; /* winnt.h:13278:9, winnt.h:13278:9, winnt.h:13278:9 */
pub const PROCESSOR_PERF_BOOST_POLICY_DISABLED: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:13383:9, winnt.h:13383:9, winnt.h:13383:9 */
pub const PROCESSOR_PERF_BOOST_POLICY_MAX: i32 = 0x64i32; /* Integer(100, Yes, Unknown) */ /* winnt.h:13384:9, winnt.h:13384:9, winnt.h:13384:9 */
pub const PROCESSOR_PERF_BOOST_MODE_DISABLED: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:13395:9, winnt.h:13395:9, winnt.h:13395:9 */
pub const PROCESSOR_PERF_BOOST_MODE_ENABLED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:13396:9, winnt.h:13396:9, winnt.h:13396:9 */
pub const PROCESSOR_PERF_BOOST_MODE_AGGRESSIVE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:13397:9, winnt.h:13397:9, winnt.h:13397:9 */
pub const PROCESSOR_PERF_BOOST_MODE_EFFICIENT_ENABLED: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:13398:9, winnt.h:13398:9, winnt.h:13398:9 */
pub const PROCESSOR_PERF_BOOST_MODE_EFFICIENT_AGGRESSIVE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:13399:9, winnt.h:13399:9, winnt.h:13399:9 */
#[doc(inline)] pub use ::winnt::PROCESSOR_PERF_BOOST_MODE_EFFICIENT_AGGRESSIVE as PROCESSOR_PERF_BOOST_MODE_MAX; /* winnt.h:13400:9, winnt.h:13400:9, winnt.h:13400:9 */
pub const CORE_PARKING_POLICY_CHANGE_IDEAL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:13473:9, winnt.h:13473:9, winnt.h:13473:9 */
pub const CORE_PARKING_POLICY_CHANGE_SINGLE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:13474:9, winnt.h:13474:9, winnt.h:13474:9 */
pub const CORE_PARKING_POLICY_CHANGE_ROCKET: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:13475:9, winnt.h:13475:9, winnt.h:13475:9 */
pub const CORE_PARKING_POLICY_CHANGE_MULTISTEP: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:13476:9, winnt.h:13476:9, winnt.h:13476:9 */
#[doc(inline)] pub use ::winnt::CORE_PARKING_POLICY_CHANGE_MULTISTEP as CORE_PARKING_POLICY_CHANGE_MAX; /* winnt.h:13477:9, winnt.h:13477:9, winnt.h:13477:9 */
pub const POWER_DEVICE_IDLE_POLICY_PERFORMANCE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:13681:9, winnt.h:13681:9, winnt.h:13681:9 */
pub const POWER_DEVICE_IDLE_POLICY_CONSERVATIVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:13682:9, winnt.h:13682:9, winnt.h:13682:9 */
pub const POWER_SYSTEM_MAXIMUM: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:13846:9, winnt.h:13846:9, winnt.h:13846:9 */
pub const DIAGNOSTIC_REASON_VERSION: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:13897:9, winnt.h:13897:9, winnt.h:13897:9 */
pub const DIAGNOSTIC_REASON_SIMPLE_STRING: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:13899:9, winnt.h:13899:9, winnt.h:13899:9 */
pub const DIAGNOSTIC_REASON_DETAILED_STRING: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:13900:9, winnt.h:13900:9, winnt.h:13900:9 */
pub const DIAGNOSTIC_REASON_NOT_SPECIFIED: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:13901:9, winnt.h:13901:9, winnt.h:13901:9 */
pub const POWER_REQUEST_CONTEXT_VERSION: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:13908:9, winnt.h:13908:9, winnt.h:13908:9 */
pub const POWER_REQUEST_CONTEXT_SIMPLE_STRING: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:13910:9, winnt.h:13910:9, winnt.h:13910:9 */
pub const POWER_REQUEST_CONTEXT_DETAILED_STRING: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:13911:9, winnt.h:13911:9, winnt.h:13911:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const PDCAP_D0_SUPPORTED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:13929:9, winnt.h:13929:9, winnt.h:13929:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const PDCAP_D1_SUPPORTED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:13930:9, winnt.h:13930:9, winnt.h:13930:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const PDCAP_D2_SUPPORTED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:13931:9, winnt.h:13931:9, winnt.h:13931:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const PDCAP_D3_SUPPORTED: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:13932:9, winnt.h:13932:9, winnt.h:13932:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const PDCAP_WAKE_FROM_D0_SUPPORTED: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:13933:9, winnt.h:13933:9, winnt.h:13933:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const PDCAP_WAKE_FROM_D1_SUPPORTED: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:13934:9, winnt.h:13934:9, winnt.h:13934:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const PDCAP_WAKE_FROM_D2_SUPPORTED: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:13935:9, winnt.h:13935:9, winnt.h:13935:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const PDCAP_WAKE_FROM_D3_SUPPORTED: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:13936:9, winnt.h:13936:9, winnt.h:13936:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const PDCAP_WARM_EJECT_SUPPORTED: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:13937:9, winnt.h:13937:9, winnt.h:13937:9 */
pub const PROC_IDLE_BUCKET_COUNT: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:14372:9, winnt.h:14372:9, winnt.h:14372:9 */
pub const PROC_IDLE_BUCKET_COUNT_EX: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:14394:9, winnt.h:14394:9, winnt.h:14394:9 */
pub const ACPI_PPM_SOFTWARE_ALL: i32 = 0xfci32; /* Integer(252, Yes, Unknown) */ /* winnt.h:14428:9, winnt.h:14428:9, winnt.h:14428:9 */
pub const ACPI_PPM_SOFTWARE_ANY: i32 = 0xfdi32; /* Integer(253, Yes, Unknown) */ /* winnt.h:14429:9, winnt.h:14429:9, winnt.h:14429:9 */
pub const ACPI_PPM_HARDWARE_ALL: i32 = 0xfei32; /* Integer(254, Yes, Unknown) */ /* winnt.h:14430:9, winnt.h:14430:9, winnt.h:14430:9 */
pub const MS_PPM_SOFTWARE_ALL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:14436:9, winnt.h:14436:9, winnt.h:14436:9 */
pub const PPM_FIRMWARE_ACPI1C2: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:14442:9, winnt.h:14442:9, winnt.h:14442:9 */
pub const PPM_FIRMWARE_ACPI1C3: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:14443:9, winnt.h:14443:9, winnt.h:14443:9 */
pub const PPM_FIRMWARE_ACPI1TSTATES: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:14444:9, winnt.h:14444:9, winnt.h:14444:9 */
pub const PPM_FIRMWARE_CST: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:14445:9, winnt.h:14445:9, winnt.h:14445:9 */
pub const PPM_FIRMWARE_CSD: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:14446:9, winnt.h:14446:9, winnt.h:14446:9 */
pub const PPM_FIRMWARE_PCT: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:14447:9, winnt.h:14447:9, winnt.h:14447:9 */
pub const PPM_FIRMWARE_PSS: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:14448:9, winnt.h:14448:9, winnt.h:14448:9 */
pub const PPM_FIRMWARE_XPSS: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:14449:9, winnt.h:14449:9, winnt.h:14449:9 */
pub const PPM_FIRMWARE_PPC: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:14450:9, winnt.h:14450:9, winnt.h:14450:9 */
pub const PPM_FIRMWARE_PSD: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:14451:9, winnt.h:14451:9, winnt.h:14451:9 */
pub const PPM_FIRMWARE_PTC: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnt.h:14452:9, winnt.h:14452:9, winnt.h:14452:9 */
pub const PPM_FIRMWARE_TSS: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winnt.h:14453:9, winnt.h:14453:9, winnt.h:14453:9 */
pub const PPM_FIRMWARE_TPC: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winnt.h:14454:9, winnt.h:14454:9, winnt.h:14454:9 */
pub const PPM_FIRMWARE_TSD: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winnt.h:14455:9, winnt.h:14455:9, winnt.h:14455:9 */
pub const PPM_FIRMWARE_PCCH: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winnt.h:14456:9, winnt.h:14456:9, winnt.h:14456:9 */
pub const PPM_FIRMWARE_PCCP: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:14457:9, winnt.h:14457:9, winnt.h:14457:9 */
pub const PPM_FIRMWARE_OSC: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winnt.h:14458:9, winnt.h:14458:9, winnt.h:14458:9 */
pub const PPM_FIRMWARE_PDC: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winnt.h:14459:9, winnt.h:14459:9, winnt.h:14459:9 */
pub const PPM_FIRMWARE_CPC: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* winnt.h:14460:9, winnt.h:14460:9, winnt.h:14460:9 */
pub const PPM_PERFORMANCE_IMPLEMENTATION_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:14466:9, winnt.h:14466:9, winnt.h:14466:9 */
pub const PPM_PERFORMANCE_IMPLEMENTATION_PSTATES: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:14467:9, winnt.h:14467:9, winnt.h:14467:9 */
pub const PPM_PERFORMANCE_IMPLEMENTATION_PCCV1: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:14468:9, winnt.h:14468:9, winnt.h:14468:9 */
pub const PPM_PERFORMANCE_IMPLEMENTATION_CPPC: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:14469:9, winnt.h:14469:9, winnt.h:14469:9 */
pub const PPM_PERFORMANCE_IMPLEMENTATION_PEP: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:14470:9, winnt.h:14470:9, winnt.h:14470:9 */
pub const PPM_IDLE_IMPLEMENTATION_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:14472:9, winnt.h:14472:9, winnt.h:14472:9 */
pub const PPM_IDLE_IMPLEMENTATION_CSTATES: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:14473:9, winnt.h:14473:9, winnt.h:14473:9 */
pub const PPM_IDLE_IMPLEMENTATION_PEP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:14474:9, winnt.h:14474:9, winnt.h:14474:9 */
pub const PPM_IDLE_IMPLEMENTATION_MICROPEP: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:14475:9, winnt.h:14475:9, winnt.h:14475:9 */
pub const POWER_ACTION_QUERY_ALLOWED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:14568:9, winnt.h:14568:9, winnt.h:14568:9 */
pub const POWER_ACTION_UI_ALLOWED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:14569:9, winnt.h:14569:9, winnt.h:14569:9 */
pub const POWER_ACTION_OVERRIDE_APPS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:14570:9, winnt.h:14570:9, winnt.h:14570:9 */
pub const POWER_ACTION_HIBERBOOT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:14571:9, winnt.h:14571:9, winnt.h:14571:9 */
pub const POWER_ACTION_PSEUDO_TRANSITION: i32 = 0x8000000i32; /* Integer(134217728, Yes, Unknown) */ /* winnt.h:14572:9, winnt.h:14572:9, winnt.h:14572:9 */
pub const POWER_ACTION_LIGHTEST_FIRST: i32 = 0x10000000i32; /* Integer(268435456, Yes, Unknown) */ /* winnt.h:14573:9, winnt.h:14573:9, winnt.h:14573:9 */
pub const POWER_ACTION_LOCK_CONSOLE: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* winnt.h:14574:9, winnt.h:14574:9, winnt.h:14574:9 */
pub const POWER_ACTION_DISABLE_WAKES: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* winnt.h:14575:9, winnt.h:14575:9, winnt.h:14575:9 */
pub const POWER_ACTION_CRITICAL: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:14576:9, winnt.h:14576:9, winnt.h:14576:9 */
pub const POWER_LEVEL_USER_NOTIFY_TEXT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:14579:9, winnt.h:14579:9, winnt.h:14579:9 */
pub const POWER_LEVEL_USER_NOTIFY_SOUND: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:14580:9, winnt.h:14580:9, winnt.h:14580:9 */
pub const POWER_LEVEL_USER_NOTIFY_EXEC: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:14581:9, winnt.h:14581:9, winnt.h:14581:9 */
pub const POWER_USER_NOTIFY_BUTTON: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:14582:9, winnt.h:14582:9, winnt.h:14582:9 */
pub const POWER_USER_NOTIFY_SHUTDOWN: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:14583:9, winnt.h:14583:9, winnt.h:14583:9 */
pub const POWER_USER_NOTIFY_FORCED_SHUTDOWN: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:14584:9, winnt.h:14584:9, winnt.h:14584:9 */
pub const POWER_FORCE_TRIGGER_RESET: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:14585:9, winnt.h:14585:9, winnt.h:14585:9 */
pub const BATTERY_DISCHARGE_FLAGS_EVENTCODE_MASK: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:14594:9, winnt.h:14594:9, winnt.h:14594:9 */
pub const BATTERY_DISCHARGE_FLAGS_ENABLE: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:14595:9, winnt.h:14595:9, winnt.h:14595:9 */
pub const NUM_DISCHARGE_POLICIES: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:14607:9, winnt.h:14607:9, winnt.h:14607:9 */
pub const DISCHARGE_POLICY_CRITICAL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:14608:9, winnt.h:14608:9, winnt.h:14608:9 */
pub const DISCHARGE_POLICY_LOW: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:14609:9, winnt.h:14609:9, winnt.h:14609:9 */
pub const PROCESSOR_IDLESTATE_POLICY_COUNT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:14671:9, winnt.h:14671:9, winnt.h:14671:9 */
pub const PO_THROTTLE_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:14701:9, winnt.h:14701:9, winnt.h:14701:9 */
pub const PO_THROTTLE_CONSTANT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:14702:9, winnt.h:14702:9, winnt.h:14702:9 */
pub const PO_THROTTLE_DEGRADE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:14703:9, winnt.h:14703:9, winnt.h:14703:9 */
pub const PO_THROTTLE_ADAPTIVE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:14704:9, winnt.h:14704:9, winnt.h:14704:9 */
pub const PO_THROTTLE_MAXIMUM: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:14705:9, winnt.h:14705:9, winnt.h:14705:9 */
pub const IMAGE_DOS_SIGNATURE: i32 = 0x5a4di32; /* Integer(23117, Yes, Unknown) */ /* winnt.h:14871:9, winnt.h:14871:9, winnt.h:14871:9 */
pub const IMAGE_OS2_SIGNATURE: i32 = 0x454ei32; /* Integer(17742, Yes, Unknown) */ /* winnt.h:14872:9, winnt.h:14872:9, winnt.h:14872:9 */
pub const IMAGE_OS2_SIGNATURE_LE: i32 = 0x454ci32; /* Integer(17740, Yes, Unknown) */ /* winnt.h:14873:9, winnt.h:14873:9, winnt.h:14873:9 */
pub const IMAGE_VXD_SIGNATURE: i32 = 0x454ci32; /* Integer(17740, Yes, Unknown) */ /* winnt.h:14874:9, winnt.h:14874:9, winnt.h:14874:9 */
pub const IMAGE_NT_SIGNATURE: i32 = 0x4550i32; /* Integer(17744, Yes, Unknown) */ /* winnt.h:14875:9, winnt.h:14875:9, winnt.h:14875:9 */
pub const IMAGE_SIZEOF_FILE_HEADER: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnt.h:15016:9, winnt.h:15016:9, winnt.h:15016:9 */
pub const IMAGE_FILE_RELOCS_STRIPPED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15018:9, winnt.h:15018:9, winnt.h:15018:9 */
pub const IMAGE_FILE_EXECUTABLE_IMAGE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15019:9, winnt.h:15019:9, winnt.h:15019:9 */
pub const IMAGE_FILE_LINE_NUMS_STRIPPED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15020:9, winnt.h:15020:9, winnt.h:15020:9 */
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15021:9, winnt.h:15021:9, winnt.h:15021:9 */
pub const IMAGE_FILE_AGGRESIVE_WS_TRIM: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:15022:9, winnt.h:15022:9, winnt.h:15022:9 */
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:15023:9, winnt.h:15023:9, winnt.h:15023:9 */
pub const IMAGE_FILE_BYTES_REVERSED_LO: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:15024:9, winnt.h:15024:9, winnt.h:15024:9 */
pub const IMAGE_FILE_32BIT_MACHINE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:15025:9, winnt.h:15025:9, winnt.h:15025:9 */
pub const IMAGE_FILE_DEBUG_STRIPPED: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:15026:9, winnt.h:15026:9, winnt.h:15026:9 */
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnt.h:15027:9, winnt.h:15027:9, winnt.h:15027:9 */
pub const IMAGE_FILE_NET_RUN_FROM_SWAP: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winnt.h:15028:9, winnt.h:15028:9, winnt.h:15028:9 */
pub const IMAGE_FILE_SYSTEM: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winnt.h:15029:9, winnt.h:15029:9, winnt.h:15029:9 */
pub const IMAGE_FILE_DLL: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winnt.h:15030:9, winnt.h:15030:9, winnt.h:15030:9 */
pub const IMAGE_FILE_UP_SYSTEM_ONLY: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winnt.h:15031:9, winnt.h:15031:9, winnt.h:15031:9 */
pub const IMAGE_FILE_BYTES_REVERSED_HI: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:15032:9, winnt.h:15032:9, winnt.h:15032:9 */
pub const IMAGE_FILE_MACHINE_UNKNOWN: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15034:9, winnt.h:15034:9, winnt.h:15034:9 */
pub const IMAGE_FILE_MACHINE_I386: i32 = 0x14ci32; /* Integer(332, Yes, Unknown) */ /* winnt.h:15035:9, winnt.h:15035:9, winnt.h:15035:9 */
pub const IMAGE_FILE_MACHINE_R3000: i32 = 0x162i32; /* Integer(354, Yes, Unknown) */ /* winnt.h:15036:9, winnt.h:15036:9, winnt.h:15036:9 */
pub const IMAGE_FILE_MACHINE_R4000: i32 = 0x166i32; /* Integer(358, Yes, Unknown) */ /* winnt.h:15037:9, winnt.h:15037:9, winnt.h:15037:9 */
pub const IMAGE_FILE_MACHINE_R10000: i32 = 0x168i32; /* Integer(360, Yes, Unknown) */ /* winnt.h:15038:9, winnt.h:15038:9, winnt.h:15038:9 */
pub const IMAGE_FILE_MACHINE_WCEMIPSV2: i32 = 0x169i32; /* Integer(361, Yes, Unknown) */ /* winnt.h:15039:9, winnt.h:15039:9, winnt.h:15039:9 */
pub const IMAGE_FILE_MACHINE_ALPHA: i32 = 0x184i32; /* Integer(388, Yes, Unknown) */ /* winnt.h:15040:9, winnt.h:15040:9, winnt.h:15040:9 */
pub const IMAGE_FILE_MACHINE_SH3: i32 = 0x1a2i32; /* Integer(418, Yes, Unknown) */ /* winnt.h:15041:9, winnt.h:15041:9, winnt.h:15041:9 */
pub const IMAGE_FILE_MACHINE_SH3DSP: i32 = 0x1a3i32; /* Integer(419, Yes, Unknown) */ /* winnt.h:15042:9, winnt.h:15042:9, winnt.h:15042:9 */
pub const IMAGE_FILE_MACHINE_SH3E: i32 = 0x1a4i32; /* Integer(420, Yes, Unknown) */ /* winnt.h:15043:9, winnt.h:15043:9, winnt.h:15043:9 */
pub const IMAGE_FILE_MACHINE_SH4: i32 = 0x1a6i32; /* Integer(422, Yes, Unknown) */ /* winnt.h:15044:9, winnt.h:15044:9, winnt.h:15044:9 */
pub const IMAGE_FILE_MACHINE_SH5: i32 = 0x1a8i32; /* Integer(424, Yes, Unknown) */ /* winnt.h:15045:9, winnt.h:15045:9, winnt.h:15045:9 */
pub const IMAGE_FILE_MACHINE_ARM: i32 = 0x1c0i32; /* Integer(448, Yes, Unknown) */ /* winnt.h:15046:9, winnt.h:15046:9, winnt.h:15046:9 */
pub const IMAGE_FILE_MACHINE_THUMB: i32 = 0x1c2i32; /* Integer(450, Yes, Unknown) */ /* winnt.h:15047:9, winnt.h:15047:9, winnt.h:15047:9 */
pub const IMAGE_FILE_MACHINE_ARMNT: i32 = 0x1c4i32; /* Integer(452, Yes, Unknown) */ /* winnt.h:15048:9, winnt.h:15048:9, winnt.h:15048:9 */
pub const IMAGE_FILE_MACHINE_AM33: i32 = 0x1d3i32; /* Integer(467, Yes, Unknown) */ /* winnt.h:15049:9, winnt.h:15049:9, winnt.h:15049:9 */
pub const IMAGE_FILE_MACHINE_POWERPC: i32 = 0x1f0i32; /* Integer(496, Yes, Unknown) */ /* winnt.h:15050:9, winnt.h:15050:9, winnt.h:15050:9 */
pub const IMAGE_FILE_MACHINE_POWERPCFP: i32 = 0x1f1i32; /* Integer(497, Yes, Unknown) */ /* winnt.h:15051:9, winnt.h:15051:9, winnt.h:15051:9 */
pub const IMAGE_FILE_MACHINE_IA64: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:15052:9, winnt.h:15052:9, winnt.h:15052:9 */
pub const IMAGE_FILE_MACHINE_MIPS16: i32 = 0x266i32; /* Integer(614, Yes, Unknown) */ /* winnt.h:15053:9, winnt.h:15053:9, winnt.h:15053:9 */
pub const IMAGE_FILE_MACHINE_ALPHA64: i32 = 0x284i32; /* Integer(644, Yes, Unknown) */ /* winnt.h:15054:9, winnt.h:15054:9, winnt.h:15054:9 */
pub const IMAGE_FILE_MACHINE_MIPSFPU: i32 = 0x366i32; /* Integer(870, Yes, Unknown) */ /* winnt.h:15055:9, winnt.h:15055:9, winnt.h:15055:9 */
pub const IMAGE_FILE_MACHINE_MIPSFPU16: i32 = 0x466i32; /* Integer(1126, Yes, Unknown) */ /* winnt.h:15056:9, winnt.h:15056:9, winnt.h:15056:9 */
#[doc(inline)] pub use ::winnt::IMAGE_FILE_MACHINE_ALPHA64 as IMAGE_FILE_MACHINE_AXP64; /* winnt.h:15057:9, winnt.h:15057:9, winnt.h:15057:9 */
pub const IMAGE_FILE_MACHINE_TRICORE: i32 = 0x520i32; /* Integer(1312, Yes, Unknown) */ /* winnt.h:15058:9, winnt.h:15058:9, winnt.h:15058:9 */
pub const IMAGE_FILE_MACHINE_CEF: i32 = 0xcefi32; /* Integer(3311, Yes, Unknown) */ /* winnt.h:15059:9, winnt.h:15059:9, winnt.h:15059:9 */
pub const IMAGE_FILE_MACHINE_EBC: i32 = 0xebci32; /* Integer(3772, Yes, Unknown) */ /* winnt.h:15060:9, winnt.h:15060:9, winnt.h:15060:9 */
pub const IMAGE_FILE_MACHINE_AMD64: i32 = 0x8664i32; /* Integer(34404, Yes, Unknown) */ /* winnt.h:15061:9, winnt.h:15061:9, winnt.h:15061:9 */
pub const IMAGE_FILE_MACHINE_M32R: i32 = 0x9041i32; /* Integer(36929, Yes, Unknown) */ /* winnt.h:15062:9, winnt.h:15062:9, winnt.h:15062:9 */
pub const IMAGE_FILE_MACHINE_CEE: i32 = 0xc0eei32; /* Integer(49390, Yes, Unknown) */ /* winnt.h:15063:9, winnt.h:15063:9, winnt.h:15063:9 */
pub const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:15074:9, winnt.h:15074:9, winnt.h:15074:9 */
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: i32 = 0x10bi32; /* Integer(267, Yes, Unknown) */ /* winnt.h:15172:9, winnt.h:15172:9, winnt.h:15172:9 */
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: i32 = 0x20bi32; /* Integer(523, Yes, Unknown) */ /* winnt.h:15173:9, winnt.h:15173:9, winnt.h:15173:9 */
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: i32 = 0x107i32; /* Integer(263, Yes, Unknown) */ /* winnt.h:15174:9, winnt.h:15174:9, winnt.h:15174:9 */
#[cfg(any(target_arch="x86", target_arch="arm"))] #[doc(inline)] pub use ::winnt::IMAGE_NT_OPTIONAL_HDR32_MAGIC as IMAGE_NT_OPTIONAL_HDR_MAGIC; /* winnt.h:15183:9, winnt.h:15183:9 */
pub const IMAGE_SUBSYSTEM_UNKNOWN: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15221:9, winnt.h:15221:9, winnt.h:15221:9 */
pub const IMAGE_SUBSYSTEM_NATIVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15222:9, winnt.h:15222:9, winnt.h:15222:9 */
pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15223:9, winnt.h:15223:9, winnt.h:15223:9 */
pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15224:9, winnt.h:15224:9, winnt.h:15224:9 */
pub const IMAGE_SUBSYSTEM_OS2_CUI: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15225:9, winnt.h:15225:9, winnt.h:15225:9 */
pub const IMAGE_SUBSYSTEM_POSIX_CUI: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15226:9, winnt.h:15226:9, winnt.h:15226:9 */
pub const IMAGE_SUBSYSTEM_NATIVE_WINDOWS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15227:9, winnt.h:15227:9, winnt.h:15227:9 */
pub const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:15228:9, winnt.h:15228:9, winnt.h:15228:9 */
pub const IMAGE_SUBSYSTEM_EFI_APPLICATION: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:15229:9, winnt.h:15229:9, winnt.h:15229:9 */
pub const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:15230:9, winnt.h:15230:9, winnt.h:15230:9 */
pub const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:15231:9, winnt.h:15231:9, winnt.h:15231:9 */
pub const IMAGE_SUBSYSTEM_EFI_ROM: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:15232:9, winnt.h:15232:9, winnt.h:15232:9 */
pub const IMAGE_SUBSYSTEM_XBOX: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:15233:9, winnt.h:15233:9, winnt.h:15233:9 */
pub const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:15234:9, winnt.h:15234:9, winnt.h:15234:9 */
pub const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:15242:9, winnt.h:15242:9, winnt.h:15242:9 */
pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:15243:9, winnt.h:15243:9, winnt.h:15243:9 */
pub const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:15244:9, winnt.h:15244:9, winnt.h:15244:9 */
pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:15245:9, winnt.h:15245:9, winnt.h:15245:9 */
pub const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:15246:9, winnt.h:15246:9, winnt.h:15246:9 */
pub const IMAGE_DLLCHARACTERISTICS_NO_SEH: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnt.h:15247:9, winnt.h:15247:9, winnt.h:15247:9 */
pub const IMAGE_DLLCHARACTERISTICS_NO_BIND: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winnt.h:15248:9, winnt.h:15248:9, winnt.h:15248:9 */
pub const IMAGE_DLLCHARACTERISTICS_APPCONTAINER: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winnt.h:15249:9, winnt.h:15249:9, winnt.h:15249:9 */
pub const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winnt.h:15250:9, winnt.h:15250:9, winnt.h:15250:9 */
pub const IMAGE_DLLCHARACTERISTICS_GUARD_CF: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winnt.h:15251:9, winnt.h:15251:9, winnt.h:15251:9 */
pub const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:15252:9, winnt.h:15252:9, winnt.h:15252:9 */
pub const IMAGE_DIRECTORY_ENTRY_EXPORT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15256:9, winnt.h:15256:9, winnt.h:15256:9 */
pub const IMAGE_DIRECTORY_ENTRY_IMPORT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15257:9, winnt.h:15257:9, winnt.h:15257:9 */
pub const IMAGE_DIRECTORY_ENTRY_RESOURCE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15258:9, winnt.h:15258:9, winnt.h:15258:9 */
pub const IMAGE_DIRECTORY_ENTRY_EXCEPTION: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15259:9, winnt.h:15259:9, winnt.h:15259:9 */
pub const IMAGE_DIRECTORY_ENTRY_SECURITY: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15260:9, winnt.h:15260:9, winnt.h:15260:9 */
pub const IMAGE_DIRECTORY_ENTRY_BASERELOC: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15261:9, winnt.h:15261:9, winnt.h:15261:9 */
pub const IMAGE_DIRECTORY_ENTRY_DEBUG: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15262:9, winnt.h:15262:9, winnt.h:15262:9 */
pub const IMAGE_DIRECTORY_ENTRY_ARCHITECTURE: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15264:9, winnt.h:15264:9, winnt.h:15264:9 */
pub const IMAGE_DIRECTORY_ENTRY_GLOBALPTR: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15265:9, winnt.h:15265:9, winnt.h:15265:9 */
pub const IMAGE_DIRECTORY_ENTRY_TLS: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:15266:9, winnt.h:15266:9, winnt.h:15266:9 */
pub const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:15267:9, winnt.h:15267:9, winnt.h:15267:9 */
pub const IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:15268:9, winnt.h:15268:9, winnt.h:15268:9 */
pub const IMAGE_DIRECTORY_ENTRY_IAT: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:15269:9, winnt.h:15269:9, winnt.h:15269:9 */
pub const IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:15270:9, winnt.h:15270:9, winnt.h:15270:9 */
pub const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:15271:9, winnt.h:15271:9, winnt.h:15271:9 */
pub const IMAGE_SIZEOF_SHORT_NAME: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15323:9, winnt.h:15323:9, winnt.h:15323:9 */
pub const IMAGE_SIZEOF_SECTION_HEADER: i32 = 0x28i32; /* Integer(40, Yes, Unknown) */ /* winnt.h:15341:9, winnt.h:15341:9, winnt.h:15341:9 */
pub const IMAGE_SCN_TYPE_NO_PAD: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15350:9, winnt.h:15350:9, winnt.h:15350:9 */
pub const IMAGE_SCN_CNT_CODE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:15353:9, winnt.h:15353:9, winnt.h:15353:9 */
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:15354:9, winnt.h:15354:9, winnt.h:15354:9 */
pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:15355:9, winnt.h:15355:9, winnt.h:15355:9 */
pub const IMAGE_SCN_LNK_OTHER: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:15357:9, winnt.h:15357:9, winnt.h:15357:9 */
pub const IMAGE_SCN_LNK_INFO: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:15358:9, winnt.h:15358:9, winnt.h:15358:9 */
pub const IMAGE_SCN_LNK_REMOVE: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winnt.h:15360:9, winnt.h:15360:9, winnt.h:15360:9 */
pub const IMAGE_SCN_LNK_COMDAT: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winnt.h:15361:9, winnt.h:15361:9, winnt.h:15361:9 */
pub const IMAGE_SCN_NO_DEFER_SPEC_EXC: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* winnt.h:15364:9, winnt.h:15364:9, winnt.h:15364:9 */
pub const IMAGE_SCN_GPREL: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:15365:9, winnt.h:15365:9, winnt.h:15365:9 */
pub const IMAGE_SCN_MEM_FARDATA: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:15366:9, winnt.h:15366:9, winnt.h:15366:9 */
pub const IMAGE_SCN_MEM_PURGEABLE: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winnt.h:15368:9, winnt.h:15368:9, winnt.h:15368:9 */
pub const IMAGE_SCN_MEM_16BIT: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winnt.h:15369:9, winnt.h:15369:9, winnt.h:15369:9 */
pub const IMAGE_SCN_MEM_LOCKED: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* winnt.h:15370:9, winnt.h:15370:9, winnt.h:15370:9 */
pub const IMAGE_SCN_MEM_PRELOAD: i32 = 0x80000i32; /* Integer(524288, Yes, Unknown) */ /* winnt.h:15371:9, winnt.h:15371:9, winnt.h:15371:9 */
pub const IMAGE_SCN_ALIGN_1BYTES: i32 = 0x100000i32; /* Integer(1048576, Yes, Unknown) */ /* winnt.h:15373:9, winnt.h:15373:9, winnt.h:15373:9 */
pub const IMAGE_SCN_ALIGN_2BYTES: i32 = 0x200000i32; /* Integer(2097152, Yes, Unknown) */ /* winnt.h:15374:9, winnt.h:15374:9, winnt.h:15374:9 */
pub const IMAGE_SCN_ALIGN_4BYTES: i32 = 0x300000i32; /* Integer(3145728, Yes, Unknown) */ /* winnt.h:15375:9, winnt.h:15375:9, winnt.h:15375:9 */
pub const IMAGE_SCN_ALIGN_8BYTES: i32 = 0x400000i32; /* Integer(4194304, Yes, Unknown) */ /* winnt.h:15376:9, winnt.h:15376:9, winnt.h:15376:9 */
pub const IMAGE_SCN_ALIGN_16BYTES: i32 = 0x500000i32; /* Integer(5242880, Yes, Unknown) */ /* winnt.h:15377:9, winnt.h:15377:9, winnt.h:15377:9 */
pub const IMAGE_SCN_ALIGN_32BYTES: i32 = 0x600000i32; /* Integer(6291456, Yes, Unknown) */ /* winnt.h:15378:9, winnt.h:15378:9, winnt.h:15378:9 */
pub const IMAGE_SCN_ALIGN_64BYTES: i32 = 0x700000i32; /* Integer(7340032, Yes, Unknown) */ /* winnt.h:15379:9, winnt.h:15379:9, winnt.h:15379:9 */
pub const IMAGE_SCN_ALIGN_128BYTES: i32 = 0x800000i32; /* Integer(8388608, Yes, Unknown) */ /* winnt.h:15380:9, winnt.h:15380:9, winnt.h:15380:9 */
pub const IMAGE_SCN_ALIGN_256BYTES: i32 = 0x900000i32; /* Integer(9437184, Yes, Unknown) */ /* winnt.h:15381:9, winnt.h:15381:9, winnt.h:15381:9 */
pub const IMAGE_SCN_ALIGN_512BYTES: i32 = 0xa00000i32; /* Integer(10485760, Yes, Unknown) */ /* winnt.h:15382:9, winnt.h:15382:9, winnt.h:15382:9 */
pub const IMAGE_SCN_ALIGN_1024BYTES: i32 = 0xb00000i32; /* Integer(11534336, Yes, Unknown) */ /* winnt.h:15383:9, winnt.h:15383:9, winnt.h:15383:9 */
pub const IMAGE_SCN_ALIGN_2048BYTES: i32 = 0xc00000i32; /* Integer(12582912, Yes, Unknown) */ /* winnt.h:15384:9, winnt.h:15384:9, winnt.h:15384:9 */
pub const IMAGE_SCN_ALIGN_4096BYTES: i32 = 0xd00000i32; /* Integer(13631488, Yes, Unknown) */ /* winnt.h:15385:9, winnt.h:15385:9, winnt.h:15385:9 */
pub const IMAGE_SCN_ALIGN_8192BYTES: i32 = 0xe00000i32; /* Integer(14680064, Yes, Unknown) */ /* winnt.h:15386:9, winnt.h:15386:9, winnt.h:15386:9 */
pub const IMAGE_SCN_ALIGN_MASK: i32 = 0xf00000i32; /* Integer(15728640, Yes, Unknown) */ /* winnt.h:15388:9, winnt.h:15388:9, winnt.h:15388:9 */
pub const IMAGE_SCN_LNK_NRELOC_OVFL: i32 = 0x1000000i32; /* Integer(16777216, Yes, Unknown) */ /* winnt.h:15390:9, winnt.h:15390:9, winnt.h:15390:9 */
pub const IMAGE_SCN_MEM_DISCARDABLE: i32 = 0x2000000i32; /* Integer(33554432, Yes, Unknown) */ /* winnt.h:15391:9, winnt.h:15391:9, winnt.h:15391:9 */
pub const IMAGE_SCN_MEM_NOT_CACHED: i32 = 0x4000000i32; /* Integer(67108864, Yes, Unknown) */ /* winnt.h:15392:9, winnt.h:15392:9, winnt.h:15392:9 */
pub const IMAGE_SCN_MEM_NOT_PAGED: i32 = 0x8000000i32; /* Integer(134217728, Yes, Unknown) */ /* winnt.h:15393:9, winnt.h:15393:9, winnt.h:15393:9 */
pub const IMAGE_SCN_MEM_SHARED: i32 = 0x10000000i32; /* Integer(268435456, Yes, Unknown) */ /* winnt.h:15394:9, winnt.h:15394:9, winnt.h:15394:9 */
pub const IMAGE_SCN_MEM_EXECUTE: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* winnt.h:15395:9, winnt.h:15395:9, winnt.h:15395:9 */
pub const IMAGE_SCN_MEM_READ: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* winnt.h:15396:9, winnt.h:15396:9, winnt.h:15396:9 */
pub const IMAGE_SCN_MEM_WRITE: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:15397:9, winnt.h:15397:9, winnt.h:15397:9 */
pub const IMAGE_SCN_SCALE_INDEX: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15402:9, winnt.h:15402:9, winnt.h:15402:9 */
pub const IMAGE_SIZEOF_SYMBOL: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:15429:9, winnt.h:15429:9, winnt.h:15429:9 */
pub const IMAGE_SYM_UNDEFINED: ::winnt::SHORT = 0x0i32 as ::winnt::SHORT; /* Cast { value: Integer(0, Yes, Unknown), ty: Type("SHORT", false) } */ /* winnt.h:15455:9, winnt.h:15455:9, winnt.h:15455:9 */
pub const IMAGE_SYM_ABSOLUTE: ::winnt::SHORT = -0x1i32 as ::winnt::SHORT; /* Cast { value: Unary(Neg, Integer(1, Yes, Unknown)), ty: Type("SHORT", false) } */ /* winnt.h:15456:9, winnt.h:15456:9, winnt.h:15456:9 */
pub const IMAGE_SYM_DEBUG: ::winnt::SHORT = -0x2i32 as ::winnt::SHORT; /* Cast { value: Unary(Neg, Integer(2, Yes, Unknown)), ty: Type("SHORT", false) } */ /* winnt.h:15457:9, winnt.h:15457:9, winnt.h:15457:9 */
pub const IMAGE_SYM_SECTION_MAX: i32 = 0xfeffi32; /* Integer(65279, Yes, Unknown) */ /* winnt.h:15458:9, winnt.h:15458:9, winnt.h:15458:9 */
#[doc(inline)] pub use ::winnt::MAXLONG as IMAGE_SYM_SECTION_MAX_EX; /* winnt.h:15459:9, winnt.h:15459:9, winnt.h:15459:9 */
pub const IMAGE_SYM_TYPE_NULL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15465:9, winnt.h:15465:9, winnt.h:15465:9 */
pub const IMAGE_SYM_TYPE_VOID: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15466:9, winnt.h:15466:9, winnt.h:15466:9 */
pub const IMAGE_SYM_TYPE_CHAR: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15467:9, winnt.h:15467:9, winnt.h:15467:9 */
pub const IMAGE_SYM_TYPE_SHORT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15468:9, winnt.h:15468:9, winnt.h:15468:9 */
pub const IMAGE_SYM_TYPE_INT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15469:9, winnt.h:15469:9, winnt.h:15469:9 */
pub const IMAGE_SYM_TYPE_LONG: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15470:9, winnt.h:15470:9, winnt.h:15470:9 */
pub const IMAGE_SYM_TYPE_FLOAT: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15471:9, winnt.h:15471:9, winnt.h:15471:9 */
pub const IMAGE_SYM_TYPE_DOUBLE: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15472:9, winnt.h:15472:9, winnt.h:15472:9 */
pub const IMAGE_SYM_TYPE_STRUCT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15473:9, winnt.h:15473:9, winnt.h:15473:9 */
pub const IMAGE_SYM_TYPE_UNION: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:15474:9, winnt.h:15474:9, winnt.h:15474:9 */
pub const IMAGE_SYM_TYPE_ENUM: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:15475:9, winnt.h:15475:9, winnt.h:15475:9 */
pub const IMAGE_SYM_TYPE_MOE: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:15476:9, winnt.h:15476:9, winnt.h:15476:9 */
pub const IMAGE_SYM_TYPE_BYTE: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:15477:9, winnt.h:15477:9, winnt.h:15477:9 */
pub const IMAGE_SYM_TYPE_WORD: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:15478:9, winnt.h:15478:9, winnt.h:15478:9 */
pub const IMAGE_SYM_TYPE_UINT: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:15479:9, winnt.h:15479:9, winnt.h:15479:9 */
pub const IMAGE_SYM_TYPE_DWORD: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:15480:9, winnt.h:15480:9, winnt.h:15480:9 */
pub const IMAGE_SYM_TYPE_PCODE: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:15481:9, winnt.h:15481:9, winnt.h:15481:9 */
pub const IMAGE_SYM_DTYPE_NULL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15486:9, winnt.h:15486:9, winnt.h:15486:9 */
pub const IMAGE_SYM_DTYPE_POINTER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15487:9, winnt.h:15487:9, winnt.h:15487:9 */
pub const IMAGE_SYM_DTYPE_FUNCTION: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15488:9, winnt.h:15488:9, winnt.h:15488:9 */
pub const IMAGE_SYM_DTYPE_ARRAY: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15489:9, winnt.h:15489:9, winnt.h:15489:9 */
pub const IMAGE_SYM_CLASS_END_OF_FUNCTION: ::minwindef::BYTE = -0x1i32 as ::minwindef::BYTE; /* Cast { value: Unary(Neg, Integer(1, Yes, Unknown)), ty: Type("BYTE", false) } */ /* winnt.h:15494:9, winnt.h:15494:9, winnt.h:15494:9 */
pub const IMAGE_SYM_CLASS_NULL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15495:9, winnt.h:15495:9, winnt.h:15495:9 */
pub const IMAGE_SYM_CLASS_AUTOMATIC: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15496:9, winnt.h:15496:9, winnt.h:15496:9 */
pub const IMAGE_SYM_CLASS_EXTERNAL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15497:9, winnt.h:15497:9, winnt.h:15497:9 */
pub const IMAGE_SYM_CLASS_STATIC: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15498:9, winnt.h:15498:9, winnt.h:15498:9 */
pub const IMAGE_SYM_CLASS_REGISTER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15499:9, winnt.h:15499:9, winnt.h:15499:9 */
pub const IMAGE_SYM_CLASS_EXTERNAL_DEF: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15500:9, winnt.h:15500:9, winnt.h:15500:9 */
pub const IMAGE_SYM_CLASS_LABEL: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15501:9, winnt.h:15501:9, winnt.h:15501:9 */
pub const IMAGE_SYM_CLASS_UNDEFINED_LABEL: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15502:9, winnt.h:15502:9, winnt.h:15502:9 */
pub const IMAGE_SYM_CLASS_MEMBER_OF_STRUCT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15503:9, winnt.h:15503:9, winnt.h:15503:9 */
pub const IMAGE_SYM_CLASS_ARGUMENT: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:15504:9, winnt.h:15504:9, winnt.h:15504:9 */
pub const IMAGE_SYM_CLASS_STRUCT_TAG: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:15505:9, winnt.h:15505:9, winnt.h:15505:9 */
pub const IMAGE_SYM_CLASS_MEMBER_OF_UNION: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:15506:9, winnt.h:15506:9, winnt.h:15506:9 */
pub const IMAGE_SYM_CLASS_UNION_TAG: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:15507:9, winnt.h:15507:9, winnt.h:15507:9 */
pub const IMAGE_SYM_CLASS_TYPE_DEFINITION: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:15508:9, winnt.h:15508:9, winnt.h:15508:9 */
pub const IMAGE_SYM_CLASS_UNDEFINED_STATIC: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:15509:9, winnt.h:15509:9, winnt.h:15509:9 */
pub const IMAGE_SYM_CLASS_ENUM_TAG: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:15510:9, winnt.h:15510:9, winnt.h:15510:9 */
pub const IMAGE_SYM_CLASS_MEMBER_OF_ENUM: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:15511:9, winnt.h:15511:9, winnt.h:15511:9 */
pub const IMAGE_SYM_CLASS_REGISTER_PARAM: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winnt.h:15512:9, winnt.h:15512:9, winnt.h:15512:9 */
pub const IMAGE_SYM_CLASS_BIT_FIELD: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:15513:9, winnt.h:15513:9, winnt.h:15513:9 */
pub const IMAGE_SYM_CLASS_FAR_EXTERNAL: i32 = 0x44i32; /* Integer(68, Yes, Unknown) */ /* winnt.h:15515:9, winnt.h:15515:9, winnt.h:15515:9 */
pub const IMAGE_SYM_CLASS_BLOCK: i32 = 0x64i32; /* Integer(100, Yes, Unknown) */ /* winnt.h:15517:9, winnt.h:15517:9, winnt.h:15517:9 */
pub const IMAGE_SYM_CLASS_FUNCTION: i32 = 0x65i32; /* Integer(101, Yes, Unknown) */ /* winnt.h:15518:9, winnt.h:15518:9, winnt.h:15518:9 */
pub const IMAGE_SYM_CLASS_END_OF_STRUCT: i32 = 0x66i32; /* Integer(102, Yes, Unknown) */ /* winnt.h:15519:9, winnt.h:15519:9, winnt.h:15519:9 */
pub const IMAGE_SYM_CLASS_FILE: i32 = 0x67i32; /* Integer(103, Yes, Unknown) */ /* winnt.h:15520:9, winnt.h:15520:9, winnt.h:15520:9 */
pub const IMAGE_SYM_CLASS_SECTION: i32 = 0x68i32; /* Integer(104, Yes, Unknown) */ /* winnt.h:15522:9, winnt.h:15522:9, winnt.h:15522:9 */
pub const IMAGE_SYM_CLASS_WEAK_EXTERNAL: i32 = 0x69i32; /* Integer(105, Yes, Unknown) */ /* winnt.h:15523:9, winnt.h:15523:9, winnt.h:15523:9 */
pub const IMAGE_SYM_CLASS_CLR_TOKEN: i32 = 0x6bi32; /* Integer(107, Yes, Unknown) */ /* winnt.h:15525:9, winnt.h:15525:9, winnt.h:15525:9 */
pub const N_BTMASK: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:15529:9, winnt.h:15529:9, winnt.h:15529:9 */
pub const N_TMASK: i32 = 0x30i32; /* Integer(48, Yes, Unknown) */ /* winnt.h:15530:9, winnt.h:15530:9, winnt.h:15530:9 */
pub const N_TMASK1: i32 = 0xc0i32; /* Integer(192, Yes, Unknown) */ /* winnt.h:15531:9, winnt.h:15531:9, winnt.h:15531:9 */
pub const N_TMASK2: i32 = 0xf0i32; /* Integer(240, Yes, Unknown) */ /* winnt.h:15532:9, winnt.h:15532:9, winnt.h:15532:9 */
pub const N_BTSHFT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15533:9, winnt.h:15533:9, winnt.h:15533:9 */
pub const N_TSHIFT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15534:9, winnt.h:15534:9, winnt.h:15534:9 */
pub const IMAGE_COMDAT_SELECT_NODUPLICATES: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15667:9, winnt.h:15667:9, winnt.h:15667:9 */
pub const IMAGE_COMDAT_SELECT_ANY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15668:9, winnt.h:15668:9, winnt.h:15668:9 */
pub const IMAGE_COMDAT_SELECT_SAME_SIZE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15669:9, winnt.h:15669:9, winnt.h:15669:9 */
pub const IMAGE_COMDAT_SELECT_EXACT_MATCH: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15670:9, winnt.h:15670:9, winnt.h:15670:9 */
pub const IMAGE_COMDAT_SELECT_ASSOCIATIVE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15671:9, winnt.h:15671:9, winnt.h:15671:9 */
pub const IMAGE_COMDAT_SELECT_LARGEST: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15672:9, winnt.h:15672:9, winnt.h:15672:9 */
pub const IMAGE_COMDAT_SELECT_NEWEST: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15673:9, winnt.h:15673:9, winnt.h:15673:9 */
pub const IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15675:9, winnt.h:15675:9, winnt.h:15675:9 */
pub const IMAGE_WEAK_EXTERN_SEARCH_LIBRARY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15676:9, winnt.h:15676:9, winnt.h:15676:9 */
pub const IMAGE_WEAK_EXTERN_SEARCH_ALIAS: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15677:9, winnt.h:15677:9, winnt.h:15677:9 */
pub const IMAGE_REL_I386_ABSOLUTE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15696:9, winnt.h:15696:9, winnt.h:15696:9 */
pub const IMAGE_REL_I386_DIR16: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15697:9, winnt.h:15697:9, winnt.h:15697:9 */
pub const IMAGE_REL_I386_REL16: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15698:9, winnt.h:15698:9, winnt.h:15698:9 */
pub const IMAGE_REL_I386_DIR32: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15699:9, winnt.h:15699:9, winnt.h:15699:9 */
pub const IMAGE_REL_I386_DIR32NB: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15700:9, winnt.h:15700:9, winnt.h:15700:9 */
pub const IMAGE_REL_I386_SEG12: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:15701:9, winnt.h:15701:9, winnt.h:15701:9 */
pub const IMAGE_REL_I386_SECTION: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:15702:9, winnt.h:15702:9, winnt.h:15702:9 */
pub const IMAGE_REL_I386_SECREL: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:15703:9, winnt.h:15703:9, winnt.h:15703:9 */
pub const IMAGE_REL_I386_TOKEN: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:15704:9, winnt.h:15704:9, winnt.h:15704:9 */
pub const IMAGE_REL_I386_SECREL7: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:15705:9, winnt.h:15705:9, winnt.h:15705:9 */
pub const IMAGE_REL_I386_REL32: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnt.h:15706:9, winnt.h:15706:9, winnt.h:15706:9 */
pub const IMAGE_REL_MIPS_ABSOLUTE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15711:9, winnt.h:15711:9, winnt.h:15711:9 */
pub const IMAGE_REL_MIPS_REFHALF: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15712:9, winnt.h:15712:9, winnt.h:15712:9 */
pub const IMAGE_REL_MIPS_REFWORD: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15713:9, winnt.h:15713:9, winnt.h:15713:9 */
pub const IMAGE_REL_MIPS_JMPADDR: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15714:9, winnt.h:15714:9, winnt.h:15714:9 */
pub const IMAGE_REL_MIPS_REFHI: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15715:9, winnt.h:15715:9, winnt.h:15715:9 */
pub const IMAGE_REL_MIPS_REFLO: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15716:9, winnt.h:15716:9, winnt.h:15716:9 */
pub const IMAGE_REL_MIPS_GPREL: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15717:9, winnt.h:15717:9, winnt.h:15717:9 */
pub const IMAGE_REL_MIPS_LITERAL: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15718:9, winnt.h:15718:9, winnt.h:15718:9 */
pub const IMAGE_REL_MIPS_SECTION: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:15719:9, winnt.h:15719:9, winnt.h:15719:9 */
pub const IMAGE_REL_MIPS_SECREL: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:15720:9, winnt.h:15720:9, winnt.h:15720:9 */
pub const IMAGE_REL_MIPS_SECRELLO: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:15721:9, winnt.h:15721:9, winnt.h:15721:9 */
pub const IMAGE_REL_MIPS_SECRELHI: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:15722:9, winnt.h:15722:9, winnt.h:15722:9 */
pub const IMAGE_REL_MIPS_TOKEN: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:15723:9, winnt.h:15723:9, winnt.h:15723:9 */
pub const IMAGE_REL_MIPS_JMPADDR16: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:15724:9, winnt.h:15724:9, winnt.h:15724:9 */
pub const IMAGE_REL_MIPS_REFWORDNB: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* winnt.h:15725:9, winnt.h:15725:9, winnt.h:15725:9 */
pub const IMAGE_REL_MIPS_PAIR: i32 = 0x25i32; /* Integer(37, Yes, Unknown) */ /* winnt.h:15726:9, winnt.h:15726:9, winnt.h:15726:9 */
pub const IMAGE_REL_ALPHA_ABSOLUTE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15731:9, winnt.h:15731:9, winnt.h:15731:9 */
pub const IMAGE_REL_ALPHA_REFLONG: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15732:9, winnt.h:15732:9, winnt.h:15732:9 */
pub const IMAGE_REL_ALPHA_REFQUAD: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15733:9, winnt.h:15733:9, winnt.h:15733:9 */
pub const IMAGE_REL_ALPHA_GPREL32: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15734:9, winnt.h:15734:9, winnt.h:15734:9 */
pub const IMAGE_REL_ALPHA_LITERAL: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15735:9, winnt.h:15735:9, winnt.h:15735:9 */
pub const IMAGE_REL_ALPHA_LITUSE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15736:9, winnt.h:15736:9, winnt.h:15736:9 */
pub const IMAGE_REL_ALPHA_GPDISP: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15737:9, winnt.h:15737:9, winnt.h:15737:9 */
pub const IMAGE_REL_ALPHA_BRADDR: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15738:9, winnt.h:15738:9, winnt.h:15738:9 */
pub const IMAGE_REL_ALPHA_HINT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15739:9, winnt.h:15739:9, winnt.h:15739:9 */
pub const IMAGE_REL_ALPHA_INLINE_REFLONG: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:15740:9, winnt.h:15740:9, winnt.h:15740:9 */
pub const IMAGE_REL_ALPHA_REFHI: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:15741:9, winnt.h:15741:9, winnt.h:15741:9 */
pub const IMAGE_REL_ALPHA_REFLO: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:15742:9, winnt.h:15742:9, winnt.h:15742:9 */
pub const IMAGE_REL_ALPHA_PAIR: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:15743:9, winnt.h:15743:9, winnt.h:15743:9 */
pub const IMAGE_REL_ALPHA_MATCH: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:15744:9, winnt.h:15744:9, winnt.h:15744:9 */
pub const IMAGE_REL_ALPHA_SECTION: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:15745:9, winnt.h:15745:9, winnt.h:15745:9 */
pub const IMAGE_REL_ALPHA_SECREL: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:15746:9, winnt.h:15746:9, winnt.h:15746:9 */
pub const IMAGE_REL_ALPHA_REFLONGNB: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:15747:9, winnt.h:15747:9, winnt.h:15747:9 */
pub const IMAGE_REL_ALPHA_SECRELLO: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winnt.h:15748:9, winnt.h:15748:9, winnt.h:15748:9 */
pub const IMAGE_REL_ALPHA_SECRELHI: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:15749:9, winnt.h:15749:9, winnt.h:15749:9 */
pub const IMAGE_REL_ALPHA_REFQ3: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winnt.h:15750:9, winnt.h:15750:9, winnt.h:15750:9 */
pub const IMAGE_REL_ALPHA_REFQ2: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnt.h:15751:9, winnt.h:15751:9, winnt.h:15751:9 */
pub const IMAGE_REL_ALPHA_REFQ1: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winnt.h:15752:9, winnt.h:15752:9, winnt.h:15752:9 */
pub const IMAGE_REL_ALPHA_GPRELLO: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winnt.h:15753:9, winnt.h:15753:9, winnt.h:15753:9 */
pub const IMAGE_REL_ALPHA_GPRELHI: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winnt.h:15754:9, winnt.h:15754:9, winnt.h:15754:9 */
pub const IMAGE_REL_PPC_ABSOLUTE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15759:9, winnt.h:15759:9, winnt.h:15759:9 */
pub const IMAGE_REL_PPC_ADDR64: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15760:9, winnt.h:15760:9, winnt.h:15760:9 */
pub const IMAGE_REL_PPC_ADDR32: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15761:9, winnt.h:15761:9, winnt.h:15761:9 */
pub const IMAGE_REL_PPC_ADDR24: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15762:9, winnt.h:15762:9, winnt.h:15762:9 */
pub const IMAGE_REL_PPC_ADDR16: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15763:9, winnt.h:15763:9, winnt.h:15763:9 */
pub const IMAGE_REL_PPC_ADDR14: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15764:9, winnt.h:15764:9, winnt.h:15764:9 */
pub const IMAGE_REL_PPC_REL24: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15765:9, winnt.h:15765:9, winnt.h:15765:9 */
pub const IMAGE_REL_PPC_REL14: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15766:9, winnt.h:15766:9, winnt.h:15766:9 */
pub const IMAGE_REL_PPC_TOCREL16: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15767:9, winnt.h:15767:9, winnt.h:15767:9 */
pub const IMAGE_REL_PPC_TOCREL14: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:15768:9, winnt.h:15768:9, winnt.h:15768:9 */
pub const IMAGE_REL_PPC_ADDR32NB: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:15770:9, winnt.h:15770:9, winnt.h:15770:9 */
pub const IMAGE_REL_PPC_SECREL: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:15771:9, winnt.h:15771:9, winnt.h:15771:9 */
pub const IMAGE_REL_PPC_SECTION: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:15772:9, winnt.h:15772:9, winnt.h:15772:9 */
pub const IMAGE_REL_PPC_IFGLUE: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:15773:9, winnt.h:15773:9, winnt.h:15773:9 */
pub const IMAGE_REL_PPC_IMGLUE: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:15774:9, winnt.h:15774:9, winnt.h:15774:9 */
pub const IMAGE_REL_PPC_SECREL16: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:15775:9, winnt.h:15775:9, winnt.h:15775:9 */
pub const IMAGE_REL_PPC_REFHI: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:15776:9, winnt.h:15776:9, winnt.h:15776:9 */
pub const IMAGE_REL_PPC_REFLO: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winnt.h:15777:9, winnt.h:15777:9, winnt.h:15777:9 */
pub const IMAGE_REL_PPC_PAIR: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:15778:9, winnt.h:15778:9, winnt.h:15778:9 */
pub const IMAGE_REL_PPC_SECRELLO: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winnt.h:15779:9, winnt.h:15779:9, winnt.h:15779:9 */
pub const IMAGE_REL_PPC_SECRELHI: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnt.h:15780:9, winnt.h:15780:9, winnt.h:15780:9 */
pub const IMAGE_REL_PPC_GPREL: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winnt.h:15781:9, winnt.h:15781:9, winnt.h:15781:9 */
pub const IMAGE_REL_PPC_TOKEN: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winnt.h:15782:9, winnt.h:15782:9, winnt.h:15782:9 */
pub const IMAGE_REL_PPC_TYPEMASK: i32 = 0xffi32; /* Integer(255, Yes, Unknown) */ /* winnt.h:15784:9, winnt.h:15784:9, winnt.h:15784:9 */
pub const IMAGE_REL_PPC_NEG: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:15788:9, winnt.h:15788:9, winnt.h:15788:9 */
pub const IMAGE_REL_PPC_BRTAKEN: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:15789:9, winnt.h:15789:9, winnt.h:15789:9 */
pub const IMAGE_REL_PPC_BRNTAKEN: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnt.h:15790:9, winnt.h:15790:9, winnt.h:15790:9 */
pub const IMAGE_REL_PPC_TOCDEFN: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winnt.h:15791:9, winnt.h:15791:9, winnt.h:15791:9 */
pub const IMAGE_REL_SH3_ABSOLUTE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15796:9, winnt.h:15796:9, winnt.h:15796:9 */
pub const IMAGE_REL_SH3_DIRECT16: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15797:9, winnt.h:15797:9, winnt.h:15797:9 */
pub const IMAGE_REL_SH3_DIRECT32: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15798:9, winnt.h:15798:9, winnt.h:15798:9 */
pub const IMAGE_REL_SH3_DIRECT8: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15799:9, winnt.h:15799:9, winnt.h:15799:9 */
pub const IMAGE_REL_SH3_DIRECT8_WORD: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15800:9, winnt.h:15800:9, winnt.h:15800:9 */
pub const IMAGE_REL_SH3_DIRECT8_LONG: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15801:9, winnt.h:15801:9, winnt.h:15801:9 */
pub const IMAGE_REL_SH3_DIRECT4: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15802:9, winnt.h:15802:9, winnt.h:15802:9 */
pub const IMAGE_REL_SH3_DIRECT4_WORD: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15803:9, winnt.h:15803:9, winnt.h:15803:9 */
pub const IMAGE_REL_SH3_DIRECT4_LONG: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15804:9, winnt.h:15804:9, winnt.h:15804:9 */
pub const IMAGE_REL_SH3_PCREL8_WORD: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:15805:9, winnt.h:15805:9, winnt.h:15805:9 */
pub const IMAGE_REL_SH3_PCREL8_LONG: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:15806:9, winnt.h:15806:9, winnt.h:15806:9 */
pub const IMAGE_REL_SH3_PCREL12_WORD: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:15807:9, winnt.h:15807:9, winnt.h:15807:9 */
pub const IMAGE_REL_SH3_STARTOF_SECTION: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:15808:9, winnt.h:15808:9, winnt.h:15808:9 */
pub const IMAGE_REL_SH3_SIZEOF_SECTION: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:15809:9, winnt.h:15809:9, winnt.h:15809:9 */
pub const IMAGE_REL_SH3_SECTION: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:15810:9, winnt.h:15810:9, winnt.h:15810:9 */
pub const IMAGE_REL_SH3_SECREL: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:15811:9, winnt.h:15811:9, winnt.h:15811:9 */
pub const IMAGE_REL_SH3_DIRECT32_NB: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:15812:9, winnt.h:15812:9, winnt.h:15812:9 */
pub const IMAGE_REL_SH3_GPREL4_LONG: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winnt.h:15813:9, winnt.h:15813:9, winnt.h:15813:9 */
pub const IMAGE_REL_SH3_TOKEN: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:15814:9, winnt.h:15814:9, winnt.h:15814:9 */
pub const IMAGE_REL_SHM_PCRELPT: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winnt.h:15815:9, winnt.h:15815:9, winnt.h:15815:9 */
pub const IMAGE_REL_SHM_REFLO: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnt.h:15820:9, winnt.h:15820:9, winnt.h:15820:9 */
pub const IMAGE_REL_SHM_REFHALF: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winnt.h:15821:9, winnt.h:15821:9, winnt.h:15821:9 */
pub const IMAGE_REL_SHM_RELLO: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winnt.h:15822:9, winnt.h:15822:9, winnt.h:15822:9 */
pub const IMAGE_REL_SHM_RELHALF: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winnt.h:15823:9, winnt.h:15823:9, winnt.h:15823:9 */
pub const IMAGE_REL_SHM_PAIR: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winnt.h:15824:9, winnt.h:15824:9, winnt.h:15824:9 */
pub const IMAGE_REL_SH_NOMODE: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:15826:9, winnt.h:15826:9, winnt.h:15826:9 */
pub const IMAGE_REL_ARM_ABSOLUTE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15829:9, winnt.h:15829:9, winnt.h:15829:9 */
pub const IMAGE_REL_ARM_ADDR32: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15830:9, winnt.h:15830:9, winnt.h:15830:9 */
pub const IMAGE_REL_ARM_ADDR32NB: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15831:9, winnt.h:15831:9, winnt.h:15831:9 */
pub const IMAGE_REL_ARM_BRANCH24: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15832:9, winnt.h:15832:9, winnt.h:15832:9 */
pub const IMAGE_REL_ARM_BRANCH11: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15833:9, winnt.h:15833:9, winnt.h:15833:9 */
pub const IMAGE_REL_ARM_TOKEN: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15834:9, winnt.h:15834:9, winnt.h:15834:9 */
pub const IMAGE_REL_ARM_GPREL12: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15835:9, winnt.h:15835:9, winnt.h:15835:9 */
pub const IMAGE_REL_ARM_GPREL7: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15836:9, winnt.h:15836:9, winnt.h:15836:9 */
pub const IMAGE_REL_ARM_BLX24: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15837:9, winnt.h:15837:9, winnt.h:15837:9 */
pub const IMAGE_REL_ARM_BLX11: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:15838:9, winnt.h:15838:9, winnt.h:15838:9 */
pub const IMAGE_REL_ARM_SECTION: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:15839:9, winnt.h:15839:9, winnt.h:15839:9 */
pub const IMAGE_REL_ARM_SECREL: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:15840:9, winnt.h:15840:9, winnt.h:15840:9 */
pub const IMAGE_REL_ARM_MOV32A: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:15841:9, winnt.h:15841:9, winnt.h:15841:9 */
pub const IMAGE_REL_ARM_MOV32: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:15842:9, winnt.h:15842:9, winnt.h:15842:9 */
pub const IMAGE_REL_ARM_MOV32T: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winnt.h:15843:9, winnt.h:15843:9, winnt.h:15843:9 */
pub const IMAGE_REL_THUMB_MOV32: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winnt.h:15844:9, winnt.h:15844:9, winnt.h:15844:9 */
pub const IMAGE_REL_ARM_BRANCH20T: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:15845:9, winnt.h:15845:9, winnt.h:15845:9 */
pub const IMAGE_REL_THUMB_BRANCH20: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:15846:9, winnt.h:15846:9, winnt.h:15846:9 */
pub const IMAGE_REL_ARM_BRANCH24T: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnt.h:15847:9, winnt.h:15847:9, winnt.h:15847:9 */
pub const IMAGE_REL_THUMB_BRANCH24: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnt.h:15848:9, winnt.h:15848:9, winnt.h:15848:9 */
pub const IMAGE_REL_ARM_BLX23T: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winnt.h:15849:9, winnt.h:15849:9, winnt.h:15849:9 */
pub const IMAGE_REL_THUMB_BLX23: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winnt.h:15850:9, winnt.h:15850:9, winnt.h:15850:9 */
pub const IMAGE_REL_AM_ABSOLUTE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15852:9, winnt.h:15852:9, winnt.h:15852:9 */
pub const IMAGE_REL_AM_ADDR32: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15853:9, winnt.h:15853:9, winnt.h:15853:9 */
pub const IMAGE_REL_AM_ADDR32NB: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15854:9, winnt.h:15854:9, winnt.h:15854:9 */
pub const IMAGE_REL_AM_CALL32: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15855:9, winnt.h:15855:9, winnt.h:15855:9 */
pub const IMAGE_REL_AM_FUNCINFO: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15856:9, winnt.h:15856:9, winnt.h:15856:9 */
pub const IMAGE_REL_AM_REL32_1: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15857:9, winnt.h:15857:9, winnt.h:15857:9 */
pub const IMAGE_REL_AM_REL32_2: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15858:9, winnt.h:15858:9, winnt.h:15858:9 */
pub const IMAGE_REL_AM_SECREL: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15859:9, winnt.h:15859:9, winnt.h:15859:9 */
pub const IMAGE_REL_AM_SECTION: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15860:9, winnt.h:15860:9, winnt.h:15860:9 */
pub const IMAGE_REL_AM_TOKEN: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:15861:9, winnt.h:15861:9, winnt.h:15861:9 */
pub const IMAGE_REL_AMD64_ABSOLUTE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15866:9, winnt.h:15866:9, winnt.h:15866:9 */
pub const IMAGE_REL_AMD64_ADDR64: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15867:9, winnt.h:15867:9, winnt.h:15867:9 */
pub const IMAGE_REL_AMD64_ADDR32: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15868:9, winnt.h:15868:9, winnt.h:15868:9 */
pub const IMAGE_REL_AMD64_ADDR32NB: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15869:9, winnt.h:15869:9, winnt.h:15869:9 */
pub const IMAGE_REL_AMD64_REL32: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15870:9, winnt.h:15870:9, winnt.h:15870:9 */
pub const IMAGE_REL_AMD64_REL32_1: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15871:9, winnt.h:15871:9, winnt.h:15871:9 */
pub const IMAGE_REL_AMD64_REL32_2: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15872:9, winnt.h:15872:9, winnt.h:15872:9 */
pub const IMAGE_REL_AMD64_REL32_3: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15873:9, winnt.h:15873:9, winnt.h:15873:9 */
pub const IMAGE_REL_AMD64_REL32_4: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15874:9, winnt.h:15874:9, winnt.h:15874:9 */
pub const IMAGE_REL_AMD64_REL32_5: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:15875:9, winnt.h:15875:9, winnt.h:15875:9 */
pub const IMAGE_REL_AMD64_SECTION: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:15876:9, winnt.h:15876:9, winnt.h:15876:9 */
pub const IMAGE_REL_AMD64_SECREL: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:15877:9, winnt.h:15877:9, winnt.h:15877:9 */
pub const IMAGE_REL_AMD64_SECREL7: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:15878:9, winnt.h:15878:9, winnt.h:15878:9 */
pub const IMAGE_REL_AMD64_TOKEN: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:15879:9, winnt.h:15879:9, winnt.h:15879:9 */
pub const IMAGE_REL_AMD64_SREL32: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:15880:9, winnt.h:15880:9, winnt.h:15880:9 */
pub const IMAGE_REL_AMD64_PAIR: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:15881:9, winnt.h:15881:9, winnt.h:15881:9 */
pub const IMAGE_REL_AMD64_SSPAN32: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:15882:9, winnt.h:15882:9, winnt.h:15882:9 */
pub const IMAGE_REL_IA64_ABSOLUTE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15887:9, winnt.h:15887:9, winnt.h:15887:9 */
pub const IMAGE_REL_IA64_IMM14: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15888:9, winnt.h:15888:9, winnt.h:15888:9 */
pub const IMAGE_REL_IA64_IMM22: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15889:9, winnt.h:15889:9, winnt.h:15889:9 */
pub const IMAGE_REL_IA64_IMM64: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15890:9, winnt.h:15890:9, winnt.h:15890:9 */
pub const IMAGE_REL_IA64_DIR32: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15891:9, winnt.h:15891:9, winnt.h:15891:9 */
pub const IMAGE_REL_IA64_DIR64: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15892:9, winnt.h:15892:9, winnt.h:15892:9 */
pub const IMAGE_REL_IA64_PCREL21B: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15893:9, winnt.h:15893:9, winnt.h:15893:9 */
pub const IMAGE_REL_IA64_PCREL21M: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15894:9, winnt.h:15894:9, winnt.h:15894:9 */
pub const IMAGE_REL_IA64_PCREL21F: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15895:9, winnt.h:15895:9, winnt.h:15895:9 */
pub const IMAGE_REL_IA64_GPREL22: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:15896:9, winnt.h:15896:9, winnt.h:15896:9 */
pub const IMAGE_REL_IA64_LTOFF22: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:15897:9, winnt.h:15897:9, winnt.h:15897:9 */
pub const IMAGE_REL_IA64_SECTION: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:15898:9, winnt.h:15898:9, winnt.h:15898:9 */
pub const IMAGE_REL_IA64_SECREL22: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:15899:9, winnt.h:15899:9, winnt.h:15899:9 */
pub const IMAGE_REL_IA64_SECREL64I: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:15900:9, winnt.h:15900:9, winnt.h:15900:9 */
pub const IMAGE_REL_IA64_SECREL32: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:15901:9, winnt.h:15901:9, winnt.h:15901:9 */
pub const IMAGE_REL_IA64_DIR32NB: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:15903:9, winnt.h:15903:9, winnt.h:15903:9 */
pub const IMAGE_REL_IA64_SREL14: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winnt.h:15904:9, winnt.h:15904:9, winnt.h:15904:9 */
pub const IMAGE_REL_IA64_SREL22: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:15905:9, winnt.h:15905:9, winnt.h:15905:9 */
pub const IMAGE_REL_IA64_SREL32: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winnt.h:15906:9, winnt.h:15906:9, winnt.h:15906:9 */
pub const IMAGE_REL_IA64_UREL32: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnt.h:15907:9, winnt.h:15907:9, winnt.h:15907:9 */
pub const IMAGE_REL_IA64_PCREL60X: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winnt.h:15908:9, winnt.h:15908:9, winnt.h:15908:9 */
pub const IMAGE_REL_IA64_PCREL60B: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winnt.h:15909:9, winnt.h:15909:9, winnt.h:15909:9 */
pub const IMAGE_REL_IA64_PCREL60F: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winnt.h:15910:9, winnt.h:15910:9, winnt.h:15910:9 */
pub const IMAGE_REL_IA64_PCREL60I: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winnt.h:15911:9, winnt.h:15911:9, winnt.h:15911:9 */
pub const IMAGE_REL_IA64_PCREL60M: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* winnt.h:15912:9, winnt.h:15912:9, winnt.h:15912:9 */
pub const IMAGE_REL_IA64_IMMGPREL64: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* winnt.h:15913:9, winnt.h:15913:9, winnt.h:15913:9 */
pub const IMAGE_REL_IA64_TOKEN: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* winnt.h:15914:9, winnt.h:15914:9, winnt.h:15914:9 */
pub const IMAGE_REL_IA64_GPREL32: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* winnt.h:15915:9, winnt.h:15915:9, winnt.h:15915:9 */
pub const IMAGE_REL_IA64_ADDEND: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* winnt.h:15916:9, winnt.h:15916:9, winnt.h:15916:9 */
pub const IMAGE_REL_CEF_ABSOLUTE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15921:9, winnt.h:15921:9, winnt.h:15921:9 */
pub const IMAGE_REL_CEF_ADDR32: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15922:9, winnt.h:15922:9, winnt.h:15922:9 */
pub const IMAGE_REL_CEF_ADDR64: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15923:9, winnt.h:15923:9, winnt.h:15923:9 */
pub const IMAGE_REL_CEF_ADDR32NB: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15924:9, winnt.h:15924:9, winnt.h:15924:9 */
pub const IMAGE_REL_CEF_SECTION: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15925:9, winnt.h:15925:9, winnt.h:15925:9 */
pub const IMAGE_REL_CEF_SECREL: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15926:9, winnt.h:15926:9, winnt.h:15926:9 */
pub const IMAGE_REL_CEF_TOKEN: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15927:9, winnt.h:15927:9, winnt.h:15927:9 */
pub const IMAGE_REL_CEE_ABSOLUTE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15932:9, winnt.h:15932:9, winnt.h:15932:9 */
pub const IMAGE_REL_CEE_ADDR32: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15933:9, winnt.h:15933:9, winnt.h:15933:9 */
pub const IMAGE_REL_CEE_ADDR64: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15934:9, winnt.h:15934:9, winnt.h:15934:9 */
pub const IMAGE_REL_CEE_ADDR32NB: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15935:9, winnt.h:15935:9, winnt.h:15935:9 */
pub const IMAGE_REL_CEE_SECTION: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15936:9, winnt.h:15936:9, winnt.h:15936:9 */
pub const IMAGE_REL_CEE_SECREL: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15937:9, winnt.h:15937:9, winnt.h:15937:9 */
pub const IMAGE_REL_CEE_TOKEN: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15938:9, winnt.h:15938:9, winnt.h:15938:9 */
pub const IMAGE_REL_M32R_ABSOLUTE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15941:9, winnt.h:15941:9, winnt.h:15941:9 */
pub const IMAGE_REL_M32R_ADDR32: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15942:9, winnt.h:15942:9, winnt.h:15942:9 */
pub const IMAGE_REL_M32R_ADDR32NB: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15943:9, winnt.h:15943:9, winnt.h:15943:9 */
pub const IMAGE_REL_M32R_ADDR24: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15944:9, winnt.h:15944:9, winnt.h:15944:9 */
pub const IMAGE_REL_M32R_GPREL16: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15945:9, winnt.h:15945:9, winnt.h:15945:9 */
pub const IMAGE_REL_M32R_PCREL24: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15946:9, winnt.h:15946:9, winnt.h:15946:9 */
pub const IMAGE_REL_M32R_PCREL16: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:15947:9, winnt.h:15947:9, winnt.h:15947:9 */
pub const IMAGE_REL_M32R_PCREL8: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15948:9, winnt.h:15948:9, winnt.h:15948:9 */
pub const IMAGE_REL_M32R_REFHALF: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15949:9, winnt.h:15949:9, winnt.h:15949:9 */
pub const IMAGE_REL_M32R_REFHI: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:15950:9, winnt.h:15950:9, winnt.h:15950:9 */
pub const IMAGE_REL_M32R_REFLO: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:15951:9, winnt.h:15951:9, winnt.h:15951:9 */
pub const IMAGE_REL_M32R_PAIR: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:15952:9, winnt.h:15952:9, winnt.h:15952:9 */
pub const IMAGE_REL_M32R_SECTION: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:15953:9, winnt.h:15953:9, winnt.h:15953:9 */
pub const IMAGE_REL_M32R_SECREL32: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:15954:9, winnt.h:15954:9, winnt.h:15954:9 */
pub const IMAGE_REL_M32R_TOKEN: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:15955:9, winnt.h:15955:9, winnt.h:15955:9 */
pub const IMAGE_REL_EBC_ABSOLUTE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15957:9, winnt.h:15957:9, winnt.h:15957:9 */
pub const IMAGE_REL_EBC_ADDR32NB: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15958:9, winnt.h:15958:9, winnt.h:15958:9 */
pub const IMAGE_REL_EBC_REL32: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:15959:9, winnt.h:15959:9, winnt.h:15959:9 */
pub const IMAGE_REL_EBC_SECTION: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15960:9, winnt.h:15960:9, winnt.h:15960:9 */
pub const IMAGE_REL_EBC_SECREL: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15961:9, winnt.h:15961:9, winnt.h:15961:9 */
pub const EMARCH_ENC_I17_IMM7B_INST_WORD_X: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15970:9, winnt.h:15970:9, winnt.h:15970:9 */
pub const EMARCH_ENC_I17_IMM7B_SIZE_X: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15971:9, winnt.h:15971:9, winnt.h:15971:9 */
pub const EMARCH_ENC_I17_IMM7B_INST_WORD_POS_X: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:15972:9, winnt.h:15972:9, winnt.h:15972:9 */
pub const EMARCH_ENC_I17_IMM7B_VAL_POS_X: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:15973:9, winnt.h:15973:9, winnt.h:15973:9 */
pub const EMARCH_ENC_I17_IMM9D_INST_WORD_X: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15975:9, winnt.h:15975:9, winnt.h:15975:9 */
pub const EMARCH_ENC_I17_IMM9D_SIZE_X: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:15976:9, winnt.h:15976:9, winnt.h:15976:9 */
pub const EMARCH_ENC_I17_IMM9D_INST_WORD_POS_X: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:15977:9, winnt.h:15977:9, winnt.h:15977:9 */
pub const EMARCH_ENC_I17_IMM9D_VAL_POS_X: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:15978:9, winnt.h:15978:9, winnt.h:15978:9 */
pub const EMARCH_ENC_I17_IMM5C_INST_WORD_X: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15980:9, winnt.h:15980:9, winnt.h:15980:9 */
pub const EMARCH_ENC_I17_IMM5C_SIZE_X: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:15981:9, winnt.h:15981:9, winnt.h:15981:9 */
pub const EMARCH_ENC_I17_IMM5C_INST_WORD_POS_X: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:15982:9, winnt.h:15982:9, winnt.h:15982:9 */
pub const EMARCH_ENC_I17_IMM5C_VAL_POS_X: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:15983:9, winnt.h:15983:9, winnt.h:15983:9 */
pub const EMARCH_ENC_I17_IC_INST_WORD_X: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:15985:9, winnt.h:15985:9, winnt.h:15985:9 */
pub const EMARCH_ENC_I17_IC_SIZE_X: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15986:9, winnt.h:15986:9, winnt.h:15986:9 */
pub const EMARCH_ENC_I17_IC_INST_WORD_POS_X: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:15987:9, winnt.h:15987:9, winnt.h:15987:9 */
pub const EMARCH_ENC_I17_IC_VAL_POS_X: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winnt.h:15988:9, winnt.h:15988:9, winnt.h:15988:9 */
pub const EMARCH_ENC_I17_IMM41a_INST_WORD_X: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15990:9, winnt.h:15990:9, winnt.h:15990:9 */
pub const EMARCH_ENC_I17_IMM41a_SIZE_X: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:15991:9, winnt.h:15991:9, winnt.h:15991:9 */
pub const EMARCH_ENC_I17_IMM41a_INST_WORD_POS_X: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:15992:9, winnt.h:15992:9, winnt.h:15992:9 */
pub const EMARCH_ENC_I17_IMM41a_VAL_POS_X: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winnt.h:15993:9, winnt.h:15993:9, winnt.h:15993:9 */
pub const EMARCH_ENC_I17_IMM41b_INST_WORD_X: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:15995:9, winnt.h:15995:9, winnt.h:15995:9 */
pub const EMARCH_ENC_I17_IMM41b_SIZE_X: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:15996:9, winnt.h:15996:9, winnt.h:15996:9 */
pub const EMARCH_ENC_I17_IMM41b_INST_WORD_POS_X: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winnt.h:15997:9, winnt.h:15997:9, winnt.h:15997:9 */
pub const EMARCH_ENC_I17_IMM41b_VAL_POS_X: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:15998:9, winnt.h:15998:9, winnt.h:15998:9 */
pub const EMARCH_ENC_I17_IMM41c_INST_WORD_X: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:16000:9, winnt.h:16000:9, winnt.h:16000:9 */
pub const EMARCH_ENC_I17_IMM41c_SIZE_X: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winnt.h:16001:9, winnt.h:16001:9, winnt.h:16001:9 */
pub const EMARCH_ENC_I17_IMM41c_INST_WORD_POS_X: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:16002:9, winnt.h:16002:9, winnt.h:16002:9 */
pub const EMARCH_ENC_I17_IMM41c_VAL_POS_X: i32 = 0x28i32; /* Integer(40, Yes, Unknown) */ /* winnt.h:16003:9, winnt.h:16003:9, winnt.h:16003:9 */
pub const EMARCH_ENC_I17_SIGN_INST_WORD_X: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:16005:9, winnt.h:16005:9, winnt.h:16005:9 */
pub const EMARCH_ENC_I17_SIGN_SIZE_X: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:16006:9, winnt.h:16006:9, winnt.h:16006:9 */
pub const EMARCH_ENC_I17_SIGN_INST_WORD_POS_X: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* winnt.h:16007:9, winnt.h:16007:9, winnt.h:16007:9 */
pub const EMARCH_ENC_I17_SIGN_VAL_POS_X: i32 = 0x3fi32; /* Integer(63, Yes, Unknown) */ /* winnt.h:16008:9, winnt.h:16008:9, winnt.h:16008:9 */
pub const X3_OPCODE_INST_WORD_X: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:16010:9, winnt.h:16010:9, winnt.h:16010:9 */
pub const X3_OPCODE_SIZE_X: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:16011:9, winnt.h:16011:9, winnt.h:16011:9 */
pub const X3_OPCODE_INST_WORD_POS_X: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* winnt.h:16012:9, winnt.h:16012:9, winnt.h:16012:9 */
pub const X3_OPCODE_SIGN_VAL_POS_X: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:16013:9, winnt.h:16013:9, winnt.h:16013:9 */
pub const X3_I_INST_WORD_X: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:16015:9, winnt.h:16015:9, winnt.h:16015:9 */
pub const X3_I_SIZE_X: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:16016:9, winnt.h:16016:9, winnt.h:16016:9 */
pub const X3_I_INST_WORD_POS_X: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* winnt.h:16017:9, winnt.h:16017:9, winnt.h:16017:9 */
pub const X3_I_SIGN_VAL_POS_X: i32 = 0x3bi32; /* Integer(59, Yes, Unknown) */ /* winnt.h:16018:9, winnt.h:16018:9, winnt.h:16018:9 */
pub const X3_D_WH_INST_WORD_X: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:16020:9, winnt.h:16020:9, winnt.h:16020:9 */
pub const X3_D_WH_SIZE_X: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:16021:9, winnt.h:16021:9, winnt.h:16021:9 */
pub const X3_D_WH_INST_WORD_POS_X: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winnt.h:16022:9, winnt.h:16022:9, winnt.h:16022:9 */
pub const X3_D_WH_SIGN_VAL_POS_X: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:16023:9, winnt.h:16023:9, winnt.h:16023:9 */
pub const X3_IMM20_INST_WORD_X: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:16025:9, winnt.h:16025:9, winnt.h:16025:9 */
pub const X3_IMM20_SIZE_X: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnt.h:16026:9, winnt.h:16026:9, winnt.h:16026:9 */
pub const X3_IMM20_INST_WORD_POS_X: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:16027:9, winnt.h:16027:9, winnt.h:16027:9 */
pub const X3_IMM20_SIGN_VAL_POS_X: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:16028:9, winnt.h:16028:9, winnt.h:16028:9 */
pub const X3_IMM39_1_INST_WORD_X: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:16030:9, winnt.h:16030:9, winnt.h:16030:9 */
pub const X3_IMM39_1_SIZE_X: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winnt.h:16031:9, winnt.h:16031:9, winnt.h:16031:9 */
pub const X3_IMM39_1_INST_WORD_POS_X: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:16032:9, winnt.h:16032:9, winnt.h:16032:9 */
pub const X3_IMM39_1_SIGN_VAL_POS_X: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* winnt.h:16033:9, winnt.h:16033:9, winnt.h:16033:9 */
pub const X3_IMM39_2_INST_WORD_X: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:16035:9, winnt.h:16035:9, winnt.h:16035:9 */
pub const X3_IMM39_2_SIZE_X: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:16036:9, winnt.h:16036:9, winnt.h:16036:9 */
pub const X3_IMM39_2_INST_WORD_POS_X: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:16037:9, winnt.h:16037:9, winnt.h:16037:9 */
pub const X3_IMM39_2_SIGN_VAL_POS_X: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnt.h:16038:9, winnt.h:16038:9, winnt.h:16038:9 */
pub const X3_P_INST_WORD_X: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:16040:9, winnt.h:16040:9, winnt.h:16040:9 */
pub const X3_P_SIZE_X: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:16041:9, winnt.h:16041:9, winnt.h:16041:9 */
pub const X3_P_INST_WORD_POS_X: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:16042:9, winnt.h:16042:9, winnt.h:16042:9 */
pub const X3_P_SIGN_VAL_POS_X: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:16043:9, winnt.h:16043:9, winnt.h:16043:9 */
pub const X3_TMPLT_INST_WORD_X: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:16045:9, winnt.h:16045:9, winnt.h:16045:9 */
pub const X3_TMPLT_SIZE_X: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:16046:9, winnt.h:16046:9, winnt.h:16046:9 */
pub const X3_TMPLT_INST_WORD_POS_X: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:16047:9, winnt.h:16047:9, winnt.h:16047:9 */
pub const X3_TMPLT_SIGN_VAL_POS_X: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:16048:9, winnt.h:16048:9, winnt.h:16048:9 */
pub const X3_BTYPE_QP_INST_WORD_X: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:16050:9, winnt.h:16050:9, winnt.h:16050:9 */
pub const X3_BTYPE_QP_SIZE_X: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:16051:9, winnt.h:16051:9, winnt.h:16051:9 */
pub const X3_BTYPE_QP_INST_WORD_POS_X: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winnt.h:16052:9, winnt.h:16052:9, winnt.h:16052:9 */
pub const X3_BTYPE_QP_INST_VAL_POS_X: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:16053:9, winnt.h:16053:9, winnt.h:16053:9 */
pub const X3_EMPTY_INST_WORD_X: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:16055:9, winnt.h:16055:9, winnt.h:16055:9 */
pub const X3_EMPTY_SIZE_X: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:16056:9, winnt.h:16056:9, winnt.h:16056:9 */
pub const X3_EMPTY_INST_WORD_POS_X: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:16057:9, winnt.h:16057:9, winnt.h:16057:9 */
pub const X3_EMPTY_INST_VAL_POS_X: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:16058:9, winnt.h:16058:9, winnt.h:16058:9 */
pub const IMAGE_REL_BASED_ABSOLUTE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:16092:9, winnt.h:16092:9, winnt.h:16092:9 */
pub const IMAGE_REL_BASED_HIGH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:16093:9, winnt.h:16093:9, winnt.h:16093:9 */
pub const IMAGE_REL_BASED_LOW: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:16094:9, winnt.h:16094:9, winnt.h:16094:9 */
pub const IMAGE_REL_BASED_HIGHLOW: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:16095:9, winnt.h:16095:9, winnt.h:16095:9 */
pub const IMAGE_REL_BASED_HIGHADJ: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:16096:9, winnt.h:16096:9, winnt.h:16096:9 */
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_5: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:16097:9, winnt.h:16097:9, winnt.h:16097:9 */
pub const IMAGE_REL_BASED_RESERVED: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:16098:9, winnt.h:16098:9, winnt.h:16098:9 */
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_7: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:16099:9, winnt.h:16099:9, winnt.h:16099:9 */
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_8: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:16100:9, winnt.h:16100:9, winnt.h:16100:9 */
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_9: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:16101:9, winnt.h:16101:9, winnt.h:16101:9 */
pub const IMAGE_REL_BASED_DIR64: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:16102:9, winnt.h:16102:9, winnt.h:16102:9 */
pub const IMAGE_REL_BASED_IA64_IMM64: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:16108:9, winnt.h:16108:9, winnt.h:16108:9 */
pub const IMAGE_REL_BASED_MIPS_JMPADDR: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:16110:9, winnt.h:16110:9, winnt.h:16110:9 */
pub const IMAGE_REL_BASED_MIPS_JMPADDR16: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:16111:9, winnt.h:16111:9, winnt.h:16111:9 */
pub const IMAGE_REL_BASED_ARM_MOV32: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:16113:9, winnt.h:16113:9, winnt.h:16113:9 */
pub const IMAGE_REL_BASED_THUMB_MOV32: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:16114:9, winnt.h:16114:9, winnt.h:16114:9 */
pub const IMAGE_ARCHIVE_START_SIZE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:16121:9, winnt.h:16121:9, winnt.h:16121:9 */
pub const IMAGE_ARCHIVE_START: &'static str = "!<arch>\n"; /* String("!<arch>\\n", Narrow) */ /* winnt.h:16122:9, winnt.h:16122:9, winnt.h:16122:9 */
pub const IMAGE_ARCHIVE_END: &'static str = "`\n"; /* String("`\\n", Narrow) */ /* winnt.h:16123:9, winnt.h:16123:9, winnt.h:16123:9 */
pub const IMAGE_ARCHIVE_PAD: &'static str = "\n"; /* String("\\n", Narrow) */ /* winnt.h:16124:9, winnt.h:16124:9, winnt.h:16124:9 */
pub const IMAGE_ARCHIVE_LINKER_MEMBER: &'static str = "/               "; /* String("/               ", Narrow) */ /* winnt.h:16125:9, winnt.h:16125:9, winnt.h:16125:9 */
pub const IMAGE_ARCHIVE_LONGNAMES_MEMBER: &'static str = "//              "; /* String("//              ", Narrow) */ /* winnt.h:16126:9, winnt.h:16126:9, winnt.h:16126:9 */
pub const IMAGE_SIZEOF_ARCHIVE_MEMBER_HDR: i32 = 0x3ci32; /* Integer(60, Yes, Unknown) */ /* winnt.h:16138:9, winnt.h:16138:9, winnt.h:16138:9 */
pub const IMAGE_ORDINAL_FLAG64: i32 = 0x8000000000000000i32; /* Integer(9223372036854775808, Yes, Unknown) */ /* winnt.h:16195:9, winnt.h:16195:9, winnt.h:16195:9 */
pub const IMAGE_ORDINAL_FLAG32: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:16196:9, winnt.h:16196:9, winnt.h:16196:9 */
#[cfg(any(target_arch="x86", target_arch="arm"))] #[doc(inline)] pub use ::winnt::IMAGE_ORDINAL_FLAG32 as IMAGE_ORDINAL_FLAG; /* winnt.h:16259:9, winnt.h:16259:9 */
pub const IMAGE_RESOURCE_NAME_IS_STRING: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:16351:9, winnt.h:16351:9, winnt.h:16351:9 */
pub const IMAGE_RESOURCE_DATA_IS_DIRECTORY: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:16352:9, winnt.h:16352:9, winnt.h:16352:9 */
pub const IMAGE_GUARD_CF_INSTRUMENTED: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:16491:9, winnt.h:16491:9, winnt.h:16491:9 */
pub const IMAGE_GUARD_CFW_INSTRUMENTED: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:16492:9, winnt.h:16492:9, winnt.h:16492:9 */
pub const IMAGE_GUARD_CF_FUNCTION_TABLE_PRESENT: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnt.h:16493:9, winnt.h:16493:9, winnt.h:16493:9 */
pub const IMAGE_GUARD_SECURITY_COOKIE_UNUSED: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winnt.h:16494:9, winnt.h:16494:9, winnt.h:16494:9 */
pub const IMAGE_GUARD_PROTECT_DELAYLOAD_IAT: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winnt.h:16495:9, winnt.h:16495:9, winnt.h:16495:9 */
pub const IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winnt.h:16496:9, winnt.h:16496:9, winnt.h:16496:9 */
pub const IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK: i32 = 0xf0000000i32; /* Integer(4026531840, Yes, Unknown) */ /* winnt.h:16497:9, winnt.h:16497:9, winnt.h:16497:9 */
pub const IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_SHIFT: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* winnt.h:16498:9, winnt.h:16498:9, winnt.h:16498:9 */
pub const IMAGE_DEBUG_TYPE_UNKNOWN: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:16602:9, winnt.h:16602:9, winnt.h:16602:9 */
pub const IMAGE_DEBUG_TYPE_COFF: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:16603:9, winnt.h:16603:9, winnt.h:16603:9 */
pub const IMAGE_DEBUG_TYPE_CODEVIEW: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:16604:9, winnt.h:16604:9, winnt.h:16604:9 */
pub const IMAGE_DEBUG_TYPE_FPO: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:16605:9, winnt.h:16605:9, winnt.h:16605:9 */
pub const IMAGE_DEBUG_TYPE_MISC: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:16606:9, winnt.h:16606:9, winnt.h:16606:9 */
pub const IMAGE_DEBUG_TYPE_EXCEPTION: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:16607:9, winnt.h:16607:9, winnt.h:16607:9 */
pub const IMAGE_DEBUG_TYPE_FIXUP: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:16608:9, winnt.h:16608:9, winnt.h:16608:9 */
pub const IMAGE_DEBUG_TYPE_OMAP_TO_SRC: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:16609:9, winnt.h:16609:9, winnt.h:16609:9 */
pub const IMAGE_DEBUG_TYPE_OMAP_FROM_SRC: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:16610:9, winnt.h:16610:9, winnt.h:16610:9 */
pub const IMAGE_DEBUG_TYPE_BORLAND: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:16611:9, winnt.h:16611:9, winnt.h:16611:9 */
pub const IMAGE_DEBUG_TYPE_RESERVED10: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:16612:9, winnt.h:16612:9, winnt.h:16612:9 */
pub const IMAGE_DEBUG_TYPE_CLSID: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:16613:9, winnt.h:16613:9, winnt.h:16613:9 */
pub const FRAME_FPO: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:16627:9, winnt.h:16627:9, winnt.h:16627:9 */
pub const FRAME_TRAP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:16628:9, winnt.h:16628:9, winnt.h:16628:9 */
pub const FRAME_TSS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:16629:9, winnt.h:16629:9, winnt.h:16629:9 */
pub const FRAME_NONFPO: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:16630:9, winnt.h:16630:9, winnt.h:16630:9 */
pub const SIZEOF_RFPO_DATA: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:16644:9, winnt.h:16644:9, winnt.h:16644:9 */
pub const IMAGE_DEBUG_MISC_EXENAME: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:16647:9, winnt.h:16647:9, winnt.h:16647:9 */
pub const IMAGE_SEPARATE_DEBUG_SIGNATURE: i32 = 0x4944i32; /* Integer(18756, Yes, Unknown) */ /* winnt.h:16731:9, winnt.h:16731:9, winnt.h:16731:9 */
pub const NON_PAGED_DEBUG_SIGNATURE: i32 = 0x494ei32; /* Integer(18766, Yes, Unknown) */ /* winnt.h:16732:9, winnt.h:16732:9, winnt.h:16732:9 */
pub const IMAGE_SEPARATE_DEBUG_FLAGS_MASK: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:16738:9, winnt.h:16738:9, winnt.h:16738:9 */
pub const IMAGE_SEPARATE_DEBUG_MISMATCH: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:16739:9, winnt.h:16739:9, winnt.h:16739:9 */
pub const IMPORT_OBJECT_HDR_SIG2: i32 = 0xffffi32; /* Integer(65535, Yes, Unknown) */ /* winnt.h:16771:9, winnt.h:16771:9, winnt.h:16771:9 */
pub const RTL_RUN_ONCE_CHECK_ONLY: u64 = 0x1u64; /* Integer(1, No, Long) */ /* winnt.h:17613:9, winnt.h:17613:9, winnt.h:17613:9 */
pub const RTL_RUN_ONCE_ASYNC: u64 = 0x2u64; /* Integer(2, No, Long) */ /* winnt.h:17614:9, winnt.h:17614:9, winnt.h:17614:9 */
pub const RTL_RUN_ONCE_INIT_FAILED: u64 = 0x4u64; /* Integer(4, No, Long) */ /* winnt.h:17615:9, winnt.h:17615:9, winnt.h:17615:9 */
pub const RTL_RUN_ONCE_CTX_RESERVED_BITS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:17622:9, winnt.h:17622:9, winnt.h:17622:9 */
pub const FAST_FAIL_LEGACY_GS_VIOLATION: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:17647:9, winnt.h:17647:9, winnt.h:17647:9 */
pub const FAST_FAIL_VTGUARD_CHECK_FAILURE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:17648:9, winnt.h:17648:9, winnt.h:17648:9 */
pub const FAST_FAIL_STACK_COOKIE_CHECK_FAILURE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:17649:9, winnt.h:17649:9, winnt.h:17649:9 */
pub const FAST_FAIL_CORRUPT_LIST_ENTRY: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:17650:9, winnt.h:17650:9, winnt.h:17650:9 */
pub const FAST_FAIL_INCORRECT_STACK: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:17651:9, winnt.h:17651:9, winnt.h:17651:9 */
pub const FAST_FAIL_INVALID_ARG: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:17652:9, winnt.h:17652:9, winnt.h:17652:9 */
pub const FAST_FAIL_GS_COOKIE_INIT: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:17653:9, winnt.h:17653:9, winnt.h:17653:9 */
pub const FAST_FAIL_FATAL_APP_EXIT: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:17654:9, winnt.h:17654:9, winnt.h:17654:9 */
pub const FAST_FAIL_RANGE_CHECK_FAILURE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:17655:9, winnt.h:17655:9, winnt.h:17655:9 */
pub const FAST_FAIL_UNSAFE_REGISTRY_ACCESS: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnt.h:17656:9, winnt.h:17656:9, winnt.h:17656:9 */
pub const FAST_FAIL_GUARD_ICALL_CHECK_FAILURE: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnt.h:17657:9, winnt.h:17657:9, winnt.h:17657:9 */
pub const FAST_FAIL_GUARD_WRITE_CHECK_FAILURE: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnt.h:17658:9, winnt.h:17658:9, winnt.h:17658:9 */
pub const FAST_FAIL_INVALID_FIBER_SWITCH: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:17659:9, winnt.h:17659:9, winnt.h:17659:9 */
pub const FAST_FAIL_INVALID_SET_OF_CONTEXT: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnt.h:17660:9, winnt.h:17660:9, winnt.h:17660:9 */
pub const FAST_FAIL_INVALID_REFERENCE_COUNT: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnt.h:17661:9, winnt.h:17661:9, winnt.h:17661:9 */
pub const FAST_FAIL_INVALID_JUMP_BUFFER: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:17662:9, winnt.h:17662:9, winnt.h:17662:9 */
pub const FAST_FAIL_MRDATA_MODIFIED: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winnt.h:17663:9, winnt.h:17663:9, winnt.h:17663:9 */
pub const FAST_FAIL_CERTIFICATION_FAILURE: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnt.h:17664:9, winnt.h:17664:9, winnt.h:17664:9 */
pub const FAST_FAIL_INVALID_EXCEPTION_CHAIN: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winnt.h:17665:9, winnt.h:17665:9, winnt.h:17665:9 */
pub const FAST_FAIL_CRYPTO_LIBRARY: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winnt.h:17666:9, winnt.h:17666:9, winnt.h:17666:9 */
pub const FAST_FAIL_INVALID_CALL_IN_DLL_CALLOUT: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winnt.h:17667:9, winnt.h:17667:9, winnt.h:17667:9 */
pub const FAST_FAIL_INVALID_IMAGE_BASE: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winnt.h:17668:9, winnt.h:17668:9, winnt.h:17668:9 */
pub const FAST_FAIL_DLOAD_PROTECTION_FAILURE: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* winnt.h:17669:9, winnt.h:17669:9, winnt.h:17669:9 */
pub const FAST_FAIL_UNSAFE_EXTENSION_CALL: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* winnt.h:17670:9, winnt.h:17670:9, winnt.h:17670:9 */
pub const FAST_FAIL_INVALID_FAST_FAIL_CODE: i32 = 0xffffffffi32; /* Integer(4294967295, Yes, Unknown) */ /* winnt.h:17671:9, winnt.h:17671:9, winnt.h:17671:9 */
pub const HEAP_NO_SERIALIZE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:17685:9, winnt.h:17685:9, winnt.h:17685:9 */
pub const HEAP_GROWABLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:17686:9, winnt.h:17686:9, winnt.h:17686:9 */
pub const HEAP_GENERATE_EXCEPTIONS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:17687:9, winnt.h:17687:9, winnt.h:17687:9 */
pub const HEAP_ZERO_MEMORY: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:17688:9, winnt.h:17688:9, winnt.h:17688:9 */
pub const HEAP_REALLOC_IN_PLACE_ONLY: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:17689:9, winnt.h:17689:9, winnt.h:17689:9 */
pub const HEAP_TAIL_CHECKING_ENABLED: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:17690:9, winnt.h:17690:9, winnt.h:17690:9 */
pub const HEAP_FREE_CHECKING_ENABLED: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:17691:9, winnt.h:17691:9, winnt.h:17691:9 */
pub const HEAP_DISABLE_COALESCE_ON_FREE: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:17692:9, winnt.h:17692:9, winnt.h:17692:9 */
pub const HEAP_CREATE_ALIGN_16: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winnt.h:17693:9, winnt.h:17693:9, winnt.h:17693:9 */
pub const HEAP_CREATE_ENABLE_TRACING: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winnt.h:17694:9, winnt.h:17694:9, winnt.h:17694:9 */
pub const HEAP_CREATE_ENABLE_EXECUTE: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* winnt.h:17695:9, winnt.h:17695:9, winnt.h:17695:9 */
pub const HEAP_MAXIMUM_TAG: i32 = 0xfffi32; /* Integer(4095, Yes, Unknown) */ /* winnt.h:17696:9, winnt.h:17696:9, winnt.h:17696:9 */
pub const HEAP_PSEUDO_TAG_FLAG: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnt.h:17697:9, winnt.h:17697:9, winnt.h:17697:9 */
pub const HEAP_TAG_SHIFT: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnt.h:17698:9, winnt.h:17698:9, winnt.h:17698:9 */
pub const IS_TEXT_UNICODE_ASCII16: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:17716:9, winnt.h:17716:9, winnt.h:17716:9 */
pub const IS_TEXT_UNICODE_REVERSE_ASCII16: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:17717:9, winnt.h:17717:9, winnt.h:17717:9 */
pub const IS_TEXT_UNICODE_STATISTICS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:17719:9, winnt.h:17719:9, winnt.h:17719:9 */
pub const IS_TEXT_UNICODE_REVERSE_STATISTICS: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:17720:9, winnt.h:17720:9, winnt.h:17720:9 */
pub const IS_TEXT_UNICODE_CONTROLS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:17722:9, winnt.h:17722:9, winnt.h:17722:9 */
pub const IS_TEXT_UNICODE_REVERSE_CONTROLS: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:17723:9, winnt.h:17723:9, winnt.h:17723:9 */
pub const IS_TEXT_UNICODE_SIGNATURE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:17725:9, winnt.h:17725:9, winnt.h:17725:9 */
pub const IS_TEXT_UNICODE_REVERSE_SIGNATURE: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:17726:9, winnt.h:17726:9, winnt.h:17726:9 */
pub const IS_TEXT_UNICODE_ILLEGAL_CHARS: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:17728:9, winnt.h:17728:9, winnt.h:17728:9 */
pub const IS_TEXT_UNICODE_ODD_LENGTH: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:17729:9, winnt.h:17729:9, winnt.h:17729:9 */
pub const IS_TEXT_UNICODE_DBCS_LEADBYTE: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnt.h:17730:9, winnt.h:17730:9, winnt.h:17730:9 */
pub const IS_TEXT_UNICODE_NULL_BYTES: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winnt.h:17731:9, winnt.h:17731:9, winnt.h:17731:9 */
pub const IS_TEXT_UNICODE_UNICODE_MASK: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnt.h:17733:9, winnt.h:17733:9, winnt.h:17733:9 */
pub const IS_TEXT_UNICODE_REVERSE_MASK: i32 = 0xf0i32; /* Integer(240, Yes, Unknown) */ /* winnt.h:17734:9, winnt.h:17734:9, winnt.h:17734:9 */
pub const IS_TEXT_UNICODE_NOT_UNICODE_MASK: i32 = 0xf00i32; /* Integer(3840, Yes, Unknown) */ /* winnt.h:17735:9, winnt.h:17735:9, winnt.h:17735:9 */
pub const IS_TEXT_UNICODE_NOT_ASCII_MASK: i32 = 0xf000i32; /* Integer(61440, Yes, Unknown) */ /* winnt.h:17736:9, winnt.h:17736:9, winnt.h:17736:9 */
pub const SEF_DACL_AUTO_INHERIT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:17820:9, winnt.h:17820:9, winnt.h:17820:9 */
pub const SEF_SACL_AUTO_INHERIT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:17821:9, winnt.h:17821:9, winnt.h:17821:9 */
pub const SEF_DEFAULT_DESCRIPTOR_FOR_OBJECT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:17822:9, winnt.h:17822:9, winnt.h:17822:9 */
pub const SEF_AVOID_PRIVILEGE_CHECK: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:17823:9, winnt.h:17823:9, winnt.h:17823:9 */
pub const SEF_AVOID_OWNER_CHECK: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:17824:9, winnt.h:17824:9, winnt.h:17824:9 */
pub const SEF_DEFAULT_OWNER_FROM_PARENT: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:17825:9, winnt.h:17825:9, winnt.h:17825:9 */
pub const SEF_DEFAULT_GROUP_FROM_PARENT: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:17826:9, winnt.h:17826:9, winnt.h:17826:9 */
pub const SEF_MACL_NO_WRITE_UP: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:17827:9, winnt.h:17827:9, winnt.h:17827:9 */
pub const SEF_MACL_NO_READ_UP: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:17828:9, winnt.h:17828:9, winnt.h:17828:9 */
pub const SEF_MACL_NO_EXECUTE_UP: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnt.h:17829:9, winnt.h:17829:9, winnt.h:17829:9 */
pub const SEF_AI_USE_EXTRA_PARAMS: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winnt.h:17830:9, winnt.h:17830:9, winnt.h:17830:9 */
pub const SEF_AVOID_OWNER_RESTRICTION: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winnt.h:17831:9, winnt.h:17831:9, winnt.h:17831:9 */
pub const MESSAGE_RESOURCE_UNICODE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:17844:9, winnt.h:17844:9, winnt.h:17844:9 */
pub const VER_EQUAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:17924:9, winnt.h:17924:9, winnt.h:17924:9 */
pub const VER_GREATER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:17925:9, winnt.h:17925:9, winnt.h:17925:9 */
pub const VER_GREATER_EQUAL: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:17926:9, winnt.h:17926:9, winnt.h:17926:9 */
pub const VER_LESS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:17927:9, winnt.h:17927:9, winnt.h:17927:9 */
pub const VER_LESS_EQUAL: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnt.h:17928:9, winnt.h:17928:9, winnt.h:17928:9 */
pub const VER_AND: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnt.h:17929:9, winnt.h:17929:9, winnt.h:17929:9 */
pub const VER_OR: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:17930:9, winnt.h:17930:9, winnt.h:17930:9 */
pub const VER_CONDITION_MASK: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnt.h:17932:9, winnt.h:17932:9, winnt.h:17932:9 */
pub const VER_NUM_BITS_PER_CONDITION_MASK: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:17933:9, winnt.h:17933:9, winnt.h:17933:9 */
pub const VER_MINORVERSION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:17939:9, winnt.h:17939:9, winnt.h:17939:9 */
pub const VER_MAJORVERSION: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:17940:9, winnt.h:17940:9, winnt.h:17940:9 */
pub const VER_BUILDNUMBER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:17941:9, winnt.h:17941:9, winnt.h:17941:9 */
pub const VER_PLATFORMID: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:17942:9, winnt.h:17942:9, winnt.h:17942:9 */
pub const VER_SERVICEPACKMINOR: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:17943:9, winnt.h:17943:9, winnt.h:17943:9 */
pub const VER_SERVICEPACKMAJOR: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:17944:9, winnt.h:17944:9, winnt.h:17944:9 */
pub const VER_SUITENAME: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:17945:9, winnt.h:17945:9, winnt.h:17945:9 */
pub const VER_PRODUCT_TYPE: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:17946:9, winnt.h:17946:9, winnt.h:17946:9 */
pub const VER_NT_WORKSTATION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:17952:9, winnt.h:17952:9, winnt.h:17952:9 */
pub const VER_NT_DOMAIN_CONTROLLER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:17953:9, winnt.h:17953:9, winnt.h:17953:9 */
pub const VER_NT_SERVER: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:17954:9, winnt.h:17954:9, winnt.h:17954:9 */
pub const VER_PLATFORM_WIN32s: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:17960:9, winnt.h:17960:9, winnt.h:17960:9 */
pub const VER_PLATFORM_WIN32_WINDOWS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:17961:9, winnt.h:17961:9, winnt.h:17961:9 */
pub const VER_PLATFORM_WIN32_NT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:17962:9, winnt.h:17962:9, winnt.h:17962:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const CTMF_INCLUDE_APPCONTAINER: u64 = 0x1u64; /* Integer(1, No, Long) */ /* winnt.h:18098:9, winnt.h:18098:9, winnt.h:18098:9 */
pub const RTL_CRITSECT_TYPE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:18157:9, winnt.h:18157:9, winnt.h:18157:9 */
pub const RTL_RESOURCE_TYPE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18158:9, winnt.h:18158:9, winnt.h:18158:9 */
pub const RTL_CRITICAL_SECTION_FLAG_NO_DEBUG_INFO: i32 = 0x1000000i32; /* Integer(16777216, Yes, Unknown) */ /* winnt.h:18163:9, winnt.h:18163:9, winnt.h:18163:9 */
pub const RTL_CRITICAL_SECTION_FLAG_DYNAMIC_SPIN: i32 = 0x2000000i32; /* Integer(33554432, Yes, Unknown) */ /* winnt.h:18164:9, winnt.h:18164:9, winnt.h:18164:9 */
pub const RTL_CRITICAL_SECTION_FLAG_STATIC_INIT: i32 = 0x4000000i32; /* Integer(67108864, Yes, Unknown) */ /* winnt.h:18165:9, winnt.h:18165:9, winnt.h:18165:9 */
pub const RTL_CRITICAL_SECTION_FLAG_RESOURCE_TYPE: i32 = 0x8000000i32; /* Integer(134217728, Yes, Unknown) */ /* winnt.h:18166:9, winnt.h:18166:9, winnt.h:18166:9 */
pub const RTL_CRITICAL_SECTION_FLAG_FORCE_DEBUG_INFO: i32 = 0x10000000i32; /* Integer(268435456, Yes, Unknown) */ /* winnt.h:18167:9, winnt.h:18167:9, winnt.h:18167:9 */
pub const RTL_CRITICAL_SECTION_ALL_FLAG_BITS: i32 = 0xff000000i32; /* Integer(4278190080, Yes, Unknown) */ /* winnt.h:18168:9, winnt.h:18168:9, winnt.h:18168:9 */
pub const RTL_CRITICAL_SECTION_DEBUG_FLAG_STATIC_INIT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18174:9, winnt.h:18174:9, winnt.h:18174:9 */
pub const RTL_CONDITION_VARIABLE_LOCKMODE_SHARED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18203:9, winnt.h:18203:9, winnt.h:18203:9 */
pub const WT_EXECUTEDEFAULT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:18243:9, winnt.h:18243:9, winnt.h:18243:9 */
pub const WT_EXECUTEINIOTHREAD: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18244:9, winnt.h:18244:9, winnt.h:18244:9 */
pub const WT_EXECUTEINUITHREAD: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:18245:9, winnt.h:18245:9, winnt.h:18245:9 */
pub const WT_EXECUTEINWAITTHREAD: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:18246:9, winnt.h:18246:9, winnt.h:18246:9 */
pub const WT_EXECUTEONLYONCE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:18247:9, winnt.h:18247:9, winnt.h:18247:9 */
pub const WT_EXECUTEINTIMERTHREAD: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:18248:9, winnt.h:18248:9, winnt.h:18248:9 */
pub const WT_EXECUTELONGFUNCTION: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:18249:9, winnt.h:18249:9, winnt.h:18249:9 */
pub const WT_EXECUTEINPERSISTENTIOTHREAD: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:18250:9, winnt.h:18250:9, winnt.h:18250:9 */
pub const WT_EXECUTEINPERSISTENTTHREAD: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:18251:9, winnt.h:18251:9, winnt.h:18251:9 */
pub const WT_TRANSFER_IMPERSONATION: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:18252:9, winnt.h:18252:9, winnt.h:18252:9 */
pub const WT_EXECUTEINLONGTHREAD: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:18271:9, winnt.h:18271:9, winnt.h:18271:9 */
pub const WT_EXECUTEDELETEWAIT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:18272:9, winnt.h:18272:9, winnt.h:18272:9 */
#[doc(inline)] pub use ::winnt::ACTIVATION_CONTEXT_INFO_CLASS as ACTIVATIONCONTEXTINFOCLASS; /* winnt.h:18291:9, winnt.h:18291:9, winnt.h:18291:9 */
#[doc(inline)] pub use ::winnt::ASSEMBLY_FILE_DETAILED_INFORMATION as ASSEMBLY_DLL_REDIRECTION_DETAILED_INFORMATION; /* winnt.h:18322:11, winnt.h:18322:11, winnt.h:18322:11 */
#[doc(inline)] pub use ::winnt::PASSEMBLY_FILE_DETAILED_INFORMATION as PASSEMBLY_DLL_REDIRECTION_DETAILED_INFORMATION; /* winnt.h:18323:10, winnt.h:18323:10, winnt.h:18323:10 */
#[doc(inline)] pub use ::winnt::PCASSEMBLY_FILE_DETAILED_INFORMATION as PCASSEMBLY_DLL_REDIRECTION_DETAILED_INFORMATION; /* winnt.h:18324:9, winnt.h:18324:9, winnt.h:18324:9 */
pub const CREATE_BOUNDARY_DESCRIPTOR_ADD_APPCONTAINER_SID: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18423:9, winnt.h:18423:9, winnt.h:18423:9 */
pub const PERFORMANCE_DATA_VERSION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18431:9, winnt.h:18431:9, winnt.h:18431:9 */
pub const READ_THREAD_PROFILING_FLAG_DISPATCHING: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18445:9, winnt.h:18445:9, winnt.h:18445:9 */
pub const READ_THREAD_PROFILING_FLAG_HARDWARE_COUNTERS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:18446:9, winnt.h:18446:9, winnt.h:18446:9 */
pub const DLL_PROCESS_ATTACH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18447:9, winnt.h:18447:9, winnt.h:18447:9 */
pub const DLL_THREAD_ATTACH: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:18448:9, winnt.h:18448:9, winnt.h:18448:9 */
pub const DLL_THREAD_DETACH: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:18449:9, winnt.h:18449:9, winnt.h:18449:9 */
pub const DLL_PROCESS_DETACH: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:18450:9, winnt.h:18450:9, winnt.h:18450:9 */
pub const EVENTLOG_SEQUENTIAL_READ: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18455:9, winnt.h:18455:9, winnt.h:18455:9 */
pub const EVENTLOG_SEEK_READ: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:18456:9, winnt.h:18456:9, winnt.h:18456:9 */
pub const EVENTLOG_FORWARDS_READ: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:18457:9, winnt.h:18457:9, winnt.h:18457:9 */
pub const EVENTLOG_BACKWARDS_READ: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:18458:9, winnt.h:18458:9, winnt.h:18458:9 */
pub const EVENTLOG_SUCCESS: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:18463:9, winnt.h:18463:9, winnt.h:18463:9 */
pub const EVENTLOG_ERROR_TYPE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18464:9, winnt.h:18464:9, winnt.h:18464:9 */
pub const EVENTLOG_WARNING_TYPE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:18465:9, winnt.h:18465:9, winnt.h:18465:9 */
pub const EVENTLOG_INFORMATION_TYPE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:18466:9, winnt.h:18466:9, winnt.h:18466:9 */
pub const EVENTLOG_AUDIT_SUCCESS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:18467:9, winnt.h:18467:9, winnt.h:18467:9 */
pub const EVENTLOG_AUDIT_FAILURE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:18468:9, winnt.h:18468:9, winnt.h:18468:9 */
pub const EVENTLOG_START_PAIRED_EVENT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18475:9, winnt.h:18475:9, winnt.h:18475:9 */
pub const EVENTLOG_END_PAIRED_EVENT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:18476:9, winnt.h:18476:9, winnt.h:18476:9 */
pub const EVENTLOG_END_ALL_PAIRED_EVENTS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:18477:9, winnt.h:18477:9, winnt.h:18477:9 */
pub const EVENTLOG_PAIRED_EVENT_ACTIVE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:18478:9, winnt.h:18478:9, winnt.h:18478:9 */
pub const EVENTLOG_PAIRED_EVENT_INACTIVE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:18479:9, winnt.h:18479:9, winnt.h:18479:9 */
pub const MAXLOGICALLOGNAMESIZE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:18522:9, winnt.h:18522:9, winnt.h:18522:9 */
pub const REG_STANDARD_FORMAT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18649:9, winnt.h:18649:9, winnt.h:18649:9 */
pub const REG_LATEST_FORMAT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:18650:9, winnt.h:18650:9, winnt.h:18650:9 */
pub const REG_NO_COMPRESSION: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:18651:9, winnt.h:18651:9, winnt.h:18651:9 */
pub const REG_FORCE_UNLOAD: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18674:9, winnt.h:18674:9, winnt.h:18674:9 */
pub const SERVICE_KERNEL_DRIVER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18723:9, winnt.h:18723:9, winnt.h:18723:9 */
pub const SERVICE_FILE_SYSTEM_DRIVER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:18724:9, winnt.h:18724:9, winnt.h:18724:9 */
pub const SERVICE_ADAPTER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:18725:9, winnt.h:18725:9, winnt.h:18725:9 */
pub const SERVICE_RECOGNIZER_DRIVER: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:18726:9, winnt.h:18726:9, winnt.h:18726:9 */
pub const SERVICE_WIN32_OWN_PROCESS: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:18732:9, winnt.h:18732:9, winnt.h:18732:9 */
pub const SERVICE_WIN32_SHARE_PROCESS: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:18733:9, winnt.h:18733:9, winnt.h:18733:9 */
pub const SERVICE_INTERACTIVE_PROCESS: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:18737:9, winnt.h:18737:9, winnt.h:18737:9 */
pub const SERVICE_BOOT_START: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:18748:9, winnt.h:18748:9, winnt.h:18748:9 */
pub const SERVICE_SYSTEM_START: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18749:9, winnt.h:18749:9, winnt.h:18749:9 */
pub const SERVICE_AUTO_START: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:18750:9, winnt.h:18750:9, winnt.h:18750:9 */
pub const SERVICE_DEMAND_START: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:18751:9, winnt.h:18751:9, winnt.h:18751:9 */
pub const SERVICE_DISABLED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:18752:9, winnt.h:18752:9, winnt.h:18752:9 */
pub const SERVICE_ERROR_IGNORE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:18757:9, winnt.h:18757:9, winnt.h:18757:9 */
pub const SERVICE_ERROR_NORMAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18758:9, winnt.h:18758:9, winnt.h:18758:9 */
pub const SERVICE_ERROR_SEVERE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:18759:9, winnt.h:18759:9, winnt.h:18759:9 */
pub const SERVICE_ERROR_CRITICAL: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:18760:9, winnt.h:18760:9, winnt.h:18760:9 */
pub const CM_SERVICE_NETWORK_BOOT_LOAD: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18822:9, winnt.h:18822:9, winnt.h:18822:9 */
pub const CM_SERVICE_VIRTUAL_DISK_BOOT_LOAD: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:18823:9, winnt.h:18823:9, winnt.h:18823:9 */
pub const CM_SERVICE_USB_DISK_BOOT_LOAD: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:18824:9, winnt.h:18824:9, winnt.h:18824:9 */
pub const CM_SERVICE_SD_DISK_BOOT_LOAD: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:18825:9, winnt.h:18825:9, winnt.h:18825:9 */
pub const CM_SERVICE_USB3_DISK_BOOT_LOAD: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:18826:9, winnt.h:18826:9, winnt.h:18826:9 */
pub const CM_SERVICE_MEASURED_BOOT_LOAD: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:18827:9, winnt.h:18827:9, winnt.h:18827:9 */
pub const CM_SERVICE_VERIFIER_BOOT_LOAD: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:18828:9, winnt.h:18828:9, winnt.h:18828:9 */
pub const CM_SERVICE_WINPE_BOOT_LOAD: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:18829:9, winnt.h:18829:9, winnt.h:18829:9 */
pub const TAPE_ERASE_SHORT: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winnt.h:18852:9, winnt.h:18852:9, winnt.h:18852:9 */
pub const TAPE_ERASE_LONG: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winnt.h:18853:9, winnt.h:18853:9, winnt.h:18853:9 */
pub const TAPE_LOAD: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winnt.h:18864:9, winnt.h:18864:9, winnt.h:18864:9 */
pub const TAPE_UNLOAD: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winnt.h:18865:9, winnt.h:18865:9, winnt.h:18865:9 */
pub const TAPE_TENSION: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winnt.h:18866:9, winnt.h:18866:9, winnt.h:18866:9 */
pub const TAPE_LOCK: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* winnt.h:18867:9, winnt.h:18867:9, winnt.h:18867:9 */
pub const TAPE_UNLOCK: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winnt.h:18868:9, winnt.h:18868:9, winnt.h:18868:9 */
pub const TAPE_FORMAT: i64 = 0x5i64; /* Integer(5, Yes, Long) */ /* winnt.h:18869:9, winnt.h:18869:9, winnt.h:18869:9 */
pub const TAPE_SETMARKS: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winnt.h:18880:9, winnt.h:18880:9, winnt.h:18880:9 */
pub const TAPE_FILEMARKS: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winnt.h:18881:9, winnt.h:18881:9, winnt.h:18881:9 */
pub const TAPE_SHORT_FILEMARKS: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winnt.h:18882:9, winnt.h:18882:9, winnt.h:18882:9 */
pub const TAPE_LONG_FILEMARKS: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* winnt.h:18883:9, winnt.h:18883:9, winnt.h:18883:9 */
pub const TAPE_ABSOLUTE_POSITION: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winnt.h:18895:9, winnt.h:18895:9, winnt.h:18895:9 */
pub const TAPE_LOGICAL_POSITION: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winnt.h:18896:9, winnt.h:18896:9, winnt.h:18896:9 */
pub const TAPE_PSEUDO_LOGICAL_POSITION: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winnt.h:18897:9, winnt.h:18897:9, winnt.h:18897:9 */
pub const TAPE_REWIND: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winnt.h:18909:9, winnt.h:18909:9, winnt.h:18909:9 */
pub const TAPE_ABSOLUTE_BLOCK: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winnt.h:18910:9, winnt.h:18910:9, winnt.h:18910:9 */
pub const TAPE_LOGICAL_BLOCK: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winnt.h:18911:9, winnt.h:18911:9, winnt.h:18911:9 */
pub const TAPE_PSEUDO_LOGICAL_BLOCK: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* winnt.h:18912:9, winnt.h:18912:9, winnt.h:18912:9 */
pub const TAPE_SPACE_END_OF_DATA: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winnt.h:18913:9, winnt.h:18913:9, winnt.h:18913:9 */
pub const TAPE_SPACE_RELATIVE_BLOCKS: i64 = 0x5i64; /* Integer(5, Yes, Long) */ /* winnt.h:18914:9, winnt.h:18914:9, winnt.h:18914:9 */
pub const TAPE_SPACE_FILEMARKS: i64 = 0x6i64; /* Integer(6, Yes, Long) */ /* winnt.h:18915:9, winnt.h:18915:9, winnt.h:18915:9 */
pub const TAPE_SPACE_SEQUENTIAL_FMKS: i64 = 0x7i64; /* Integer(7, Yes, Long) */ /* winnt.h:18916:9, winnt.h:18916:9, winnt.h:18916:9 */
pub const TAPE_SPACE_SETMARKS: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* winnt.h:18917:9, winnt.h:18917:9, winnt.h:18917:9 */
pub const TAPE_SPACE_SEQUENTIAL_SMKS: i64 = 0x9i64; /* Integer(9, Yes, Long) */ /* winnt.h:18918:9, winnt.h:18918:9, winnt.h:18918:9 */
pub const TAPE_DRIVE_FIXED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:18935:9, winnt.h:18935:9, winnt.h:18935:9 */
pub const TAPE_DRIVE_SELECT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:18936:9, winnt.h:18936:9, winnt.h:18936:9 */
pub const TAPE_DRIVE_INITIATOR: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:18937:9, winnt.h:18937:9, winnt.h:18937:9 */
pub const TAPE_DRIVE_ERASE_SHORT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:18939:9, winnt.h:18939:9, winnt.h:18939:9 */
pub const TAPE_DRIVE_ERASE_LONG: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnt.h:18940:9, winnt.h:18940:9, winnt.h:18940:9 */
pub const TAPE_DRIVE_ERASE_BOP_ONLY: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:18941:9, winnt.h:18941:9, winnt.h:18941:9 */
pub const TAPE_DRIVE_ERASE_IMMEDIATE: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:18942:9, winnt.h:18942:9, winnt.h:18942:9 */
pub const TAPE_DRIVE_TAPE_CAPACITY: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnt.h:18944:9, winnt.h:18944:9, winnt.h:18944:9 */
pub const TAPE_DRIVE_TAPE_REMAINING: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnt.h:18945:9, winnt.h:18945:9, winnt.h:18945:9 */
pub const TAPE_DRIVE_FIXED_BLOCK: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnt.h:18946:9, winnt.h:18946:9, winnt.h:18946:9 */
pub const TAPE_DRIVE_VARIABLE_BLOCK: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winnt.h:18947:9, winnt.h:18947:9, winnt.h:18947:9 */
pub const TAPE_DRIVE_WRITE_PROTECT: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winnt.h:18949:9, winnt.h:18949:9, winnt.h:18949:9 */
pub const TAPE_DRIVE_EOT_WZ_SIZE: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winnt.h:18950:9, winnt.h:18950:9, winnt.h:18950:9 */
pub const TAPE_DRIVE_ECC: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winnt.h:18952:9, winnt.h:18952:9, winnt.h:18952:9 */
pub const TAPE_DRIVE_COMPRESSION: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winnt.h:18953:9, winnt.h:18953:9, winnt.h:18953:9 */
pub const TAPE_DRIVE_PADDING: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* winnt.h:18954:9, winnt.h:18954:9, winnt.h:18954:9 */
pub const TAPE_DRIVE_REPORT_SMKS: i32 = 0x80000i32; /* Integer(524288, Yes, Unknown) */ /* winnt.h:18955:9, winnt.h:18955:9, winnt.h:18955:9 */
pub const TAPE_DRIVE_GET_ABSOLUTE_BLK: i32 = 0x100000i32; /* Integer(1048576, Yes, Unknown) */ /* winnt.h:18957:9, winnt.h:18957:9, winnt.h:18957:9 */
pub const TAPE_DRIVE_GET_LOGICAL_BLK: i32 = 0x200000i32; /* Integer(2097152, Yes, Unknown) */ /* winnt.h:18958:9, winnt.h:18958:9, winnt.h:18958:9 */
pub const TAPE_DRIVE_SET_EOT_WZ_SIZE: i32 = 0x400000i32; /* Integer(4194304, Yes, Unknown) */ /* winnt.h:18959:9, winnt.h:18959:9, winnt.h:18959:9 */
pub const TAPE_DRIVE_EJECT_MEDIA: i32 = 0x1000000i32; /* Integer(16777216, Yes, Unknown) */ /* winnt.h:18961:9, winnt.h:18961:9, winnt.h:18961:9 */
pub const TAPE_DRIVE_CLEAN_REQUESTS: i32 = 0x2000000i32; /* Integer(33554432, Yes, Unknown) */ /* winnt.h:18962:9, winnt.h:18962:9, winnt.h:18962:9 */
pub const TAPE_DRIVE_SET_CMP_BOP_ONLY: i32 = 0x4000000i32; /* Integer(67108864, Yes, Unknown) */ /* winnt.h:18963:9, winnt.h:18963:9, winnt.h:18963:9 */
pub const TAPE_DRIVE_RESERVED_BIT: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:18965:9, winnt.h:18965:9, winnt.h:18965:9 */
pub const TAPE_DRIVE_LOAD_UNLOAD: i32 = 0x80000001i32; /* Integer(2147483649, Yes, Unknown) */ /* winnt.h:18973:9, winnt.h:18973:9, winnt.h:18973:9 */
pub const TAPE_DRIVE_TENSION: i32 = 0x80000002i32; /* Integer(2147483650, Yes, Unknown) */ /* winnt.h:18974:9, winnt.h:18974:9, winnt.h:18974:9 */
pub const TAPE_DRIVE_LOCK_UNLOCK: i32 = 0x80000004i32; /* Integer(2147483652, Yes, Unknown) */ /* winnt.h:18975:9, winnt.h:18975:9, winnt.h:18975:9 */
pub const TAPE_DRIVE_REWIND_IMMEDIATE: i32 = 0x80000008i32; /* Integer(2147483656, Yes, Unknown) */ /* winnt.h:18976:9, winnt.h:18976:9, winnt.h:18976:9 */
pub const TAPE_DRIVE_SET_BLOCK_SIZE: i32 = 0x80000010i32; /* Integer(2147483664, Yes, Unknown) */ /* winnt.h:18978:9, winnt.h:18978:9, winnt.h:18978:9 */
pub const TAPE_DRIVE_LOAD_UNLD_IMMED: i32 = 0x80000020i32; /* Integer(2147483680, Yes, Unknown) */ /* winnt.h:18979:9, winnt.h:18979:9, winnt.h:18979:9 */
pub const TAPE_DRIVE_TENSION_IMMED: i32 = 0x80000040i32; /* Integer(2147483712, Yes, Unknown) */ /* winnt.h:18980:9, winnt.h:18980:9, winnt.h:18980:9 */
pub const TAPE_DRIVE_LOCK_UNLK_IMMED: i32 = 0x80000080i32; /* Integer(2147483776, Yes, Unknown) */ /* winnt.h:18981:9, winnt.h:18981:9, winnt.h:18981:9 */
pub const TAPE_DRIVE_SET_ECC: i32 = 0x80000100i32; /* Integer(2147483904, Yes, Unknown) */ /* winnt.h:18983:9, winnt.h:18983:9, winnt.h:18983:9 */
pub const TAPE_DRIVE_SET_COMPRESSION: i32 = 0x80000200i32; /* Integer(2147484160, Yes, Unknown) */ /* winnt.h:18984:9, winnt.h:18984:9, winnt.h:18984:9 */
pub const TAPE_DRIVE_SET_PADDING: i32 = 0x80000400i32; /* Integer(2147484672, Yes, Unknown) */ /* winnt.h:18985:9, winnt.h:18985:9, winnt.h:18985:9 */
pub const TAPE_DRIVE_SET_REPORT_SMKS: i32 = 0x80000800i32; /* Integer(2147485696, Yes, Unknown) */ /* winnt.h:18986:9, winnt.h:18986:9, winnt.h:18986:9 */
pub const TAPE_DRIVE_ABSOLUTE_BLK: i32 = 0x80001000i32; /* Integer(2147487744, Yes, Unknown) */ /* winnt.h:18988:9, winnt.h:18988:9, winnt.h:18988:9 */
pub const TAPE_DRIVE_ABS_BLK_IMMED: i32 = 0x80002000i32; /* Integer(2147491840, Yes, Unknown) */ /* winnt.h:18989:9, winnt.h:18989:9, winnt.h:18989:9 */
pub const TAPE_DRIVE_LOGICAL_BLK: i32 = 0x80004000i32; /* Integer(2147500032, Yes, Unknown) */ /* winnt.h:18990:9, winnt.h:18990:9, winnt.h:18990:9 */
pub const TAPE_DRIVE_LOG_BLK_IMMED: i32 = 0x80008000i32; /* Integer(2147516416, Yes, Unknown) */ /* winnt.h:18991:9, winnt.h:18991:9, winnt.h:18991:9 */
pub const TAPE_DRIVE_END_OF_DATA: i32 = 0x80010000i32; /* Integer(2147549184, Yes, Unknown) */ /* winnt.h:18993:9, winnt.h:18993:9, winnt.h:18993:9 */
pub const TAPE_DRIVE_RELATIVE_BLKS: i32 = 0x80020000i32; /* Integer(2147614720, Yes, Unknown) */ /* winnt.h:18994:9, winnt.h:18994:9, winnt.h:18994:9 */
pub const TAPE_DRIVE_FILEMARKS: i32 = 0x80040000i32; /* Integer(2147745792, Yes, Unknown) */ /* winnt.h:18995:9, winnt.h:18995:9, winnt.h:18995:9 */
pub const TAPE_DRIVE_SEQUENTIAL_FMKS: i32 = 0x80080000i32; /* Integer(2148007936, Yes, Unknown) */ /* winnt.h:18996:9, winnt.h:18996:9, winnt.h:18996:9 */
pub const TAPE_DRIVE_SETMARKS: i32 = 0x80100000i32; /* Integer(2148532224, Yes, Unknown) */ /* winnt.h:18998:9, winnt.h:18998:9, winnt.h:18998:9 */
pub const TAPE_DRIVE_SEQUENTIAL_SMKS: i32 = 0x80200000i32; /* Integer(2149580800, Yes, Unknown) */ /* winnt.h:18999:9, winnt.h:18999:9, winnt.h:18999:9 */
pub const TAPE_DRIVE_REVERSE_POSITION: i32 = 0x80400000i32; /* Integer(2151677952, Yes, Unknown) */ /* winnt.h:19000:9, winnt.h:19000:9, winnt.h:19000:9 */
pub const TAPE_DRIVE_SPACE_IMMEDIATE: i32 = 0x80800000i32; /* Integer(2155872256, Yes, Unknown) */ /* winnt.h:19001:9, winnt.h:19001:9, winnt.h:19001:9 */
pub const TAPE_DRIVE_WRITE_SETMARKS: i32 = 0x81000000i32; /* Integer(2164260864, Yes, Unknown) */ /* winnt.h:19003:9, winnt.h:19003:9, winnt.h:19003:9 */
pub const TAPE_DRIVE_WRITE_FILEMARKS: i32 = 0x82000000i32; /* Integer(2181038080, Yes, Unknown) */ /* winnt.h:19004:9, winnt.h:19004:9, winnt.h:19004:9 */
pub const TAPE_DRIVE_WRITE_SHORT_FMKS: i32 = 0x84000000i32; /* Integer(2214592512, Yes, Unknown) */ /* winnt.h:19005:9, winnt.h:19005:9, winnt.h:19005:9 */
pub const TAPE_DRIVE_WRITE_LONG_FMKS: i32 = 0x88000000i32; /* Integer(2281701376, Yes, Unknown) */ /* winnt.h:19006:9, winnt.h:19006:9, winnt.h:19006:9 */
pub const TAPE_DRIVE_WRITE_MARK_IMMED: i32 = 0x90000000i32; /* Integer(2415919104, Yes, Unknown) */ /* winnt.h:19008:9, winnt.h:19008:9, winnt.h:19008:9 */
pub const TAPE_DRIVE_FORMAT: i32 = 0xa0000000i32; /* Integer(2684354560, Yes, Unknown) */ /* winnt.h:19009:9, winnt.h:19009:9, winnt.h:19009:9 */
pub const TAPE_DRIVE_FORMAT_IMMEDIATE: i32 = 0xc0000000i32; /* Integer(3221225472, Yes, Unknown) */ /* winnt.h:19010:9, winnt.h:19010:9, winnt.h:19010:9 */
pub const TAPE_DRIVE_HIGH_FEATURES: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnt.h:19011:9, winnt.h:19011:9, winnt.h:19011:9 */
pub const TAPE_FIXED_PARTITIONS: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winnt.h:19063:9, winnt.h:19063:9, winnt.h:19063:9 */
pub const TAPE_SELECT_PARTITIONS: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winnt.h:19064:9, winnt.h:19064:9, winnt.h:19064:9 */
pub const TAPE_INITIATOR_PARTITIONS: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winnt.h:19065:9, winnt.h:19065:9, winnt.h:19065:9 */
pub const TAPE_QUERY_DRIVE_PARAMETERS: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* winnt.h:19077:9, winnt.h:19077:9, winnt.h:19077:9 */
pub const TAPE_QUERY_MEDIA_CAPACITY: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* winnt.h:19078:9, winnt.h:19078:9, winnt.h:19078:9 */
pub const TAPE_CHECK_FOR_DRIVE_PROBLEM: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* winnt.h:19079:9, winnt.h:19079:9, winnt.h:19079:9 */
pub const TAPE_QUERY_IO_ERROR_DATA: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* winnt.h:19080:9, winnt.h:19080:9, winnt.h:19080:9 */
pub const TAPE_QUERY_DEVICE_ERROR_DATA: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* winnt.h:19081:9, winnt.h:19081:9, winnt.h:19081:9 */
#[cfg(any(target_arch="x86"))] pub const PcTeb: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winnt.h:19828:9 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub type LONGLONG = ::libc::c_longlong; /* winnt.h:725:17, winnt.h:725:17 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub type ULONGLONG = ::libc::c_ulonglong; /* winnt.h:726:26, winnt.h:726:26 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub type PLONGLONG = *mut ::libc::c_longlong; /* winnt.h:747:19, winnt.h:747:19 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub type PULONGLONG = *mut ::libc::c_ulonglong; /* winnt.h:748:20, winnt.h:748:20 */
#[cfg(any(target_arch="x86_64"))] pub type PRTL_REFERENCE_COUNT = *mut ::libc::c_longlong; /* winnt.h:794:40 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub type PDWORDLONG = *mut ::libc::c_ulonglong; /* winnt.h:811:20, winnt.h:811:20 */
#[cfg(any(target_arch="x86_64"))] pub type PKSPIN_LOCK = *mut ::libc::c_ulonglong; /* winnt.h:2278:21 */
#[cfg(any(target_arch="x86_64"))] #[repr(C)] pub struct XSAVE_FORMAT { ControlWord: ::minwindef::WORD, StatusWord: ::minwindef::WORD, TagWord: ::minwindef::BYTE, Reserved1: ::minwindef::BYTE, ErrorOpcode: ::minwindef::WORD, ErrorOffset: ::minwindef::DWORD, ErrorSelector: ::minwindef::WORD, Reserved2: ::minwindef::WORD, DataOffset: ::minwindef::DWORD, DataSelector: ::minwindef::WORD, Reserved3: ::minwindef::WORD, MxCsr: ::minwindef::DWORD, MxCsr_Mask: ::minwindef::DWORD, FloatRegisters: *mut [::winnt::M128A; 8], XmmRegisters: *mut [::winnt::M128A; 16], Reserved4: *mut [::minwindef::BYTE; 96] } /* winnt.h:2295:35 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] #[repr(C)] pub struct XSTATE_CONTEXT { Mask: ::basetsd::DWORD64, Length: ::minwindef::DWORD, Reserved1: ::minwindef::DWORD, Area: ::winnt::PXSAVE_AREA, Buffer: ::winnt::PVOID } /* winnt.h:2337:16, winnt.h:2337:16 */
#[cfg(any(target_arch="x86_64"))] pub type XMM_SAVE_AREA32 = ::winnt::XSAVE_FORMAT; /* winnt.h:3526:22 */
#[cfg(any(target_arch="x86_64"))] pub type PXMM_SAVE_AREA32 = *mut ::winnt::XSAVE_FORMAT; /* winnt.h:3526:40 */
#[cfg(any(target_arch="x86_64"))] #[repr(C)] pub struct CONTEXT { P1Home: ::basetsd::DWORD64, P2Home: ::basetsd::DWORD64, P3Home: ::basetsd::DWORD64, P4Home: ::basetsd::DWORD64, P5Home: ::basetsd::DWORD64, P6Home: ::basetsd::DWORD64, ContextFlags: ::minwindef::DWORD, MxCsr: ::minwindef::DWORD, SegCs: ::minwindef::WORD, SegDs: ::minwindef::WORD, SegEs: ::minwindef::WORD, SegFs: ::minwindef::WORD, SegGs: ::minwindef::WORD, SegSs: ::minwindef::WORD, EFlags: ::minwindef::DWORD, Dr0: ::basetsd::DWORD64, Dr1: ::basetsd::DWORD64, Dr2: ::basetsd::DWORD64, Dr3: ::basetsd::DWORD64, Dr6: ::basetsd::DWORD64, Dr7: ::basetsd::DWORD64, Rax: ::basetsd::DWORD64, Rcx: ::basetsd::DWORD64, Rdx: ::basetsd::DWORD64, Rbx: ::basetsd::DWORD64, Rsp: ::basetsd::DWORD64, Rbp: ::basetsd::DWORD64, Rsi: ::basetsd::DWORD64, Rdi: ::basetsd::DWORD64, R8: ::basetsd::DWORD64, R9: ::basetsd::DWORD64, R10: ::basetsd::DWORD64, R11: ::basetsd::DWORD64, R12: ::basetsd::DWORD64, R13: ::basetsd::DWORD64, R14: ::basetsd::DWORD64, R15: ::basetsd::DWORD64, Rip: ::basetsd::DWORD64, u: ::winnt::CONTEXT_Child_39, VectorRegister: *mut [::winnt::M128A; 26], VectorControl: ::basetsd::DWORD64, DebugControl: ::basetsd::DWORD64, LastBranchToRip: ::basetsd::DWORD64, LastBranchFromRip: ::basetsd::DWORD64, LastExceptionToRip: ::basetsd::DWORD64, LastExceptionFromRip: ::basetsd::DWORD64 } /* winnt.h:3563:35 */
#[cfg(any(target_arch="x86_64"))] #[repr(C)] pub /*union*/ struct CONTEXT_Child_39; /* STUB! */ /* winnt.h:3640:5 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub type PRUNTIME_FUNCTION = *mut ::winnt::RUNTIME_FUNCTION; /* winnt.h:3687:65, winnt.h:4675:69 */
#[cfg(any(target_arch="x86_64"))] pub type SCOPE_TABLE = ::winnt::SCOPE_TABLE_AMD64; /* winnt.h:3688:27 */
#[cfg(any(target_arch="x86_64"))] pub type PSCOPE_TABLE = *mut ::winnt::SCOPE_TABLE_AMD64; /* winnt.h:3688:41 */
#[cfg(any(target_arch="x86_64"))] #[repr(C)] pub struct UNWIND_HISTORY_TABLE_ENTRY { ImageBase: ::basetsd::DWORD64, FunctionEntry: ::winnt::PRUNTIME_FUNCTION } /* winnt.h:3709:16 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub type PUNWIND_HISTORY_TABLE_ENTRY = *mut ::winnt::UNWIND_HISTORY_TABLE_ENTRY; /* winnt.h:3712:32, winnt.h:4695:32 */
#[cfg(any(target_arch="x86_64"))] #[repr(C)] pub struct UNWIND_HISTORY_TABLE { Count: ::minwindef::DWORD, LocalHint: ::minwindef::BYTE, GlobalHint: ::minwindef::BYTE, Search: ::minwindef::BYTE, Once: ::minwindef::BYTE, LowAddress: ::basetsd::DWORD64, HighAddress: ::basetsd::DWORD64, Entry: *mut [::winnt::UNWIND_HISTORY_TABLE_ENTRY; 12] } /* winnt.h:3714:16 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub type PUNWIND_HISTORY_TABLE = *mut ::winnt::UNWIND_HISTORY_TABLE; /* winnt.h:3723:26, winnt.h:4706:26 */
#[cfg(any(target_arch="x86_64"))] pub type GET_RUNTIME_FUNCTION_CALLBACK = extern "system" fn(::basetsd::DWORD64, ::winnt::PVOID) -> ::winnt::PRUNTIME_FUNCTION; /* winnt.h:3732:1 */
#[cfg(any(target_arch="x86_64"))] pub type PGET_RUNTIME_FUNCTION_CALLBACK = extern "system" fn(::libc::c_ulonglong, *mut ::libc::c_void) -> *mut ::winnt::RUNTIME_FUNCTION; /* winnt.h:3736:40 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub type OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK = extern "system" fn(::winnt::HANDLE, ::winnt::PVOID, ::minwindef::PDWORD, *mut *mut ::winnt::RUNTIME_FUNCTION) -> ::minwindef::DWORD; /* winnt.h:3741:1, winnt.h:4764:1 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub type POUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK = extern "system" fn(*mut ::libc::c_void, *mut ::libc::c_void, *mut ::libc::c_ulong, *mut *mut ::winnt::RUNTIME_FUNCTION) -> ::libc::c_ulong; /* winnt.h:3747:49, winnt.h:4770:49 */
#[cfg(any(target_arch="x86_64"))] #[repr(C)] pub struct DISPATCHER_CONTEXT { ControlPc: ::basetsd::DWORD64, ImageBase: ::basetsd::DWORD64, FunctionEntry: ::winnt::PRUNTIME_FUNCTION, EstablisherFrame: ::basetsd::DWORD64, TargetIp: ::basetsd::DWORD64, ContextRecord: ::winnt::PCONTEXT, LanguageHandler: ::winnt::PEXCEPTION_ROUTINE, HandlerData: ::winnt::PVOID, HistoryTable: ::winnt::PUNWIND_HISTORY_TABLE, ScopeIndex: ::minwindef::DWORD, Fill0: ::minwindef::DWORD } /* winnt.h:3756:16 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub type PDISPATCHER_CONTEXT = *mut ::winnt::DISPATCHER_CONTEXT; /* winnt.h:3768:24, winnt.h:4726:24 */
#[cfg(any(target_arch="x86_64"))] pub type PEXCEPTION_FILTER = extern "system" fn(*mut ::winnt::EXCEPTION_POINTERS, *mut ::libc::c_void) -> ::libc::c_long; /* winnt.h:3777:3 */
#[cfg(any(target_arch="x86_64"))] pub type PTERMINATION_HANDLER = extern "system" fn(::libc::c_uchar, *mut ::libc::c_void); /* winnt.h:3784:3 */
#[cfg(any(target_arch="x86_64"))] #[repr(C)] pub struct KNONVOLATILE_CONTEXT_POINTERS { u: ::winnt::KNONVOLATILE_CONTEXT_POINTERS_Child_0, u2: ::winnt::KNONVOLATILE_CONTEXT_POINTERS_Child_2 } /* winnt.h:3794:16 */
#[cfg(any(target_arch="x86_64"))] #[repr(C)] pub /*union*/ struct KNONVOLATILE_CONTEXT_POINTERS_Child_0; /* STUB! */ /* winnt.h:3795:5 */
#[cfg(any(target_arch="x86_64"))] #[repr(C)] pub /*union*/ struct KNONVOLATILE_CONTEXT_POINTERS_Child_2; /* STUB! */ /* winnt.h:3817:5 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub type PKNONVOLATILE_CONTEXT_POINTERS = *mut ::winnt::KNONVOLATILE_CONTEXT_POINTERS; /* winnt.h:3839:35, winnt.h:4801:35 */
#[cfg(any(target_arch="x86_64"))] pub type PSID_HASH_ENTRY = *mut ::libc::c_ulonglong; /* winnt.h:9269:36 */
#[cfg(any(target_arch="x86_64"))] pub type IMAGE_OPTIONAL_HEADER = ::winnt::IMAGE_OPTIONAL_HEADER64; /* winnt.h:15177:45 */
#[cfg(any(target_arch="x86_64"))] pub type PIMAGE_OPTIONAL_HEADER = ::winnt::PIMAGE_OPTIONAL_HEADER64; /* winnt.h:15178:45 */
#[cfg(any(target_arch="x86_64"))] pub type IMAGE_NT_HEADERS = ::winnt::IMAGE_NT_HEADERS64; /* winnt.h:15204:45 */
#[cfg(any(target_arch="x86_64"))] pub type PIMAGE_NT_HEADERS = ::winnt::PIMAGE_NT_HEADERS64; /* winnt.h:15205:45 */
#[cfg(any(target_arch="x86_64"))] pub type IMAGE_THUNK_DATA = ::winnt::IMAGE_THUNK_DATA64; /* winnt.h:16253:41 */
#[cfg(any(target_arch="x86_64"))] pub type PIMAGE_THUNK_DATA = ::winnt::PIMAGE_THUNK_DATA64; /* winnt.h:16254:41 */
#[cfg(any(target_arch="x86_64"))] pub type IMAGE_TLS_DIRECTORY = ::winnt::IMAGE_TLS_DIRECTORY64; /* winnt.h:16256:41 */
#[cfg(any(target_arch="x86_64"))] pub type PIMAGE_TLS_DIRECTORY = ::winnt::PIMAGE_TLS_DIRECTORY64; /* winnt.h:16257:41 */
#[cfg(any(target_arch="x86_64"))] pub type IMAGE_LOAD_CONFIG_DIRECTORY = ::winnt::IMAGE_LOAD_CONFIG_DIRECTORY64; /* winnt.h:16484:43 */
#[cfg(any(target_arch="x86_64"))] pub type PIMAGE_LOAD_CONFIG_DIRECTORY = ::winnt::PIMAGE_LOAD_CONFIG_DIRECTORY64; /* winnt.h:16485:43 */
#[cfg(any(target_arch="x86_64"))] #[repr(C)] pub struct RUNTIME_FUNCTION { BeginAddress: ::minwindef::DWORD, EndAddress: ::minwindef::DWORD, u: ::winnt::RUNTIME_FUNCTION_Child_3 } /* winnt.h:16551:16 */
#[cfg(any(target_arch="x86_64"))] #[repr(C)] pub /*union*/ struct RUNTIME_FUNCTION_Child_3; /* STUB! */ /* winnt.h:16554:5 */
#[cfg(any(target_arch="x86_64"))] pub type _PIMAGE_RUNTIME_FUNCTION_ENTRY = *mut ::winnt::RUNTIME_FUNCTION; /* winnt.h:16558:35 */
#[cfg(any(target_arch="x86_64"))] #[repr(C)] pub struct SLIST_ENTRY { Next: *mut ::winnt::SLIST_ENTRY } /* winnt.h:17485:35 */
#[cfg(any(target_arch="x86_64"))] pub type PSLIST_ENTRY = *mut ::winnt::SLIST_ENTRY; /* winnt.h:17487:17 */
#[cfg(any(target_arch="x86_64"))] pub type RTL_UMS_SCHEDULER_ENTRY_POINT = extern "system" fn(::winnt::RTL_UMS_SCHEDULER_REASON, ::libc::c_ulonglong, *mut ::libc::c_void); /* winnt.h:18049:1 */
#[cfg(any(target_arch="x86_64"))] pub type PRTL_UMS_SCHEDULER_ENTRY_POINT = extern "system" fn(::winnt::RTL_UMS_SCHEDULER_REASON, ::libc::c_ulonglong, *mut ::libc::c_void); /* winnt.h:18055:40 */
#[cfg(any(target_arch="x86_64"))] pub type PAPCFUNC = extern "system" fn(::libc::c_ulonglong); /* winnt.h:18206:9 */
#[cfg(any(target_arch="x86_64"))] pub type PSECURE_MEMORY_CACHE_CALLBACK = extern "system" fn(*mut ::libc::c_void, ::libc::c_ulonglong) -> ::libc::c_uchar; /* winnt.h:18266:9 */
#[cfg(any(target_arch="x86_64"))] pub const MEMORY_ALLOCATION_ALIGNMENT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:116:9 */
#[cfg(any(target_arch="x86_64"))] pub const CONTEXT_AMD64: i64 = 0x100000i64; /* Integer(1048576, Yes, Long) */ /* winnt.h:3487:9 */
#[cfg(any(target_arch="x86_64"))] pub const INITIAL_MXCSR: i32 = 0x1f80i32; /* Integer(8064, Yes, Unknown) */ /* winnt.h:3519:9 */
#[cfg(any(target_arch="x86_64"))] pub const INITIAL_FPCSR: i32 = 0x27fi32; /* Integer(639, Yes, Unknown) */ /* winnt.h:3520:9 */
#[cfg(any(target_arch="x86_64"))] pub const RUNTIME_FUNCTION_INDIRECT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:3690:9 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub const UNW_FLAG_NHANDLER: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:3696:9, winnt.h:4682:9 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub const UNW_FLAG_EHANDLER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:3697:9, winnt.h:4683:9 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub const UNW_FLAG_UHANDLER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:3698:9, winnt.h:4684:9 */
#[cfg(any(target_arch="x86_64"))] pub const UNW_FLAG_CHAININFO: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnt.h:3699:9 */
#[cfg(any(target_arch="x86_64"))] pub const UNW_FLAG_NO_EPILOGUE: u64 = 0x80000000u64; /* Integer(2147483648, No, Long) */ /* winnt.h:3701:9 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub const UNWIND_HISTORY_TABLE_SIZE: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnt.h:3707:9, winnt.h:4690:9 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub const OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK_EXPORT_NAME: &'static str = "OutOfProcessFunctionTableCallback"; /* String("OutOfProcessFunctionTableCallback", Narrow) */ /* winnt.h:3749:9, winnt.h:4772:9 */
#[cfg(any(target_arch="x86_64"))] pub const MAXIMUM_PROC_PER_GROUP: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnt.h:11318:9 */
#[cfg(any(target_arch="x86_64"))] #[doc(inline)] pub use ::winnt::IMAGE_NT_OPTIONAL_HDR64_MAGIC as IMAGE_NT_OPTIONAL_HDR_MAGIC; /* winnt.h:15179:9 */
#[cfg(any(target_arch="x86_64"))] #[doc(inline)] pub use ::winnt::IMAGE_ORDINAL_FLAG64 as IMAGE_ORDINAL_FLAG; /* winnt.h:16251:9 */
#[cfg(any(target_arch="arm"))] #[repr(C)] pub struct NEON128 { Low: ::winnt::ULONGLONG, High: ::winnt::LONGLONG } /* winnt.h:4606:16 */
#[cfg(any(target_arch="arm"))] pub type PNEON128 = *mut ::winnt::NEON128; /* winnt.h:4609:13 */
#[cfg(any(target_arch="arm"))] #[repr(C)] pub struct CONTEXT { ContextFlags: ::minwindef::DWORD, R0: ::minwindef::DWORD, R1: ::minwindef::DWORD, R2: ::minwindef::DWORD, R3: ::minwindef::DWORD, R4: ::minwindef::DWORD, R5: ::minwindef::DWORD, R6: ::minwindef::DWORD, R7: ::minwindef::DWORD, R8: ::minwindef::DWORD, R9: ::minwindef::DWORD, R10: ::minwindef::DWORD, R11: ::minwindef::DWORD, R12: ::minwindef::DWORD, Sp: ::minwindef::DWORD, Lr: ::minwindef::DWORD, Pc: ::minwindef::DWORD, Cpsr: ::minwindef::DWORD, Fpscr: ::minwindef::DWORD, Padding: ::minwindef::DWORD, u: ::winnt::CONTEXT_Child_21, Bvr: *mut [::minwindef::DWORD; 8], Bcr: *mut [::minwindef::DWORD; 8], Wvr: *mut [::minwindef::DWORD; 1], Wcr: *mut [::minwindef::DWORD; 1], Padding2: *mut [::minwindef::DWORD; 2] } /* winnt.h:4611:34 */
#[cfg(any(target_arch="arm"))] #[repr(C)] pub /*union*/ struct CONTEXT_Child_21; /* STUB! */ /* winnt.h:4652:5 */
#[cfg(any(target_arch="arm"))] pub type SCOPE_TABLE = ::winnt::SCOPE_TABLE_ARM; /* winnt.h:4676:25 */
#[cfg(any(target_arch="arm"))] pub type PSCOPE_TABLE = *mut ::winnt::SCOPE_TABLE_ARM; /* winnt.h:4676:39 */
#[cfg(any(target_arch="arm"))] #[repr(C)] pub struct UNWIND_HISTORY_TABLE_ENTRY { ImageBase: ::minwindef::DWORD, FunctionEntry: ::winnt::PRUNTIME_FUNCTION } /* winnt.h:4692:16 */
#[cfg(any(target_arch="arm"))] #[repr(C)] pub struct UNWIND_HISTORY_TABLE { Count: ::minwindef::DWORD, LocalHint: ::minwindef::BYTE, GlobalHint: ::minwindef::BYTE, Search: ::minwindef::BYTE, Once: ::minwindef::BYTE, LowAddress: ::minwindef::DWORD, HighAddress: ::minwindef::DWORD, Entry: *mut [::winnt::UNWIND_HISTORY_TABLE_ENTRY; 12] } /* winnt.h:4697:16 */
#[cfg(any(target_arch="arm"))] #[repr(C)] pub struct DISPATCHER_CONTEXT { ControlPc: ::minwindef::DWORD, ImageBase: ::minwindef::DWORD, FunctionEntry: ::winnt::PRUNTIME_FUNCTION, EstablisherFrame: ::minwindef::DWORD, TargetPc: ::minwindef::DWORD, ContextRecord: ::winnt::PCONTEXT, LanguageHandler: ::winnt::PEXCEPTION_ROUTINE, HandlerData: ::winnt::PVOID, HistoryTable: ::winnt::PUNWIND_HISTORY_TABLE, ScopeIndex: ::minwindef::DWORD, ControlPcIsUnwound: ::winnt::BOOLEAN, NonVolatileRegisters: ::minwindef::PBYTE, Reserved: ::minwindef::DWORD } /* winnt.h:4712:16 */
#[cfg(any(target_arch="arm"))] pub type PEXCEPTION_FILTER = extern "system" fn(*mut ::winnt::EXCEPTION_POINTERS, ::libc::c_ulong) -> ::libc::c_long; /* winnt.h:4736:3 */
#[cfg(any(target_arch="arm"))] pub type PTERMINATION_HANDLER = extern "system" fn(::libc::c_uchar, ::libc::c_ulong); /* winnt.h:4743:3 */
#[cfg(any(target_arch="arm"))] pub type GET_RUNTIME_FUNCTION_CALLBACK = extern "system" fn(::minwindef::DWORD, ::winnt::PVOID) -> ::winnt::PRUNTIME_FUNCTION; /* winnt.h:4755:1 */
#[cfg(any(target_arch="arm"))] pub type PGET_RUNTIME_FUNCTION_CALLBACK = extern "system" fn(::libc::c_ulong, *mut ::libc::c_void) -> *mut ::winnt::RUNTIME_FUNCTION; /* winnt.h:4759:40 */
#[cfg(any(target_arch="arm"))] #[repr(C)] pub struct KNONVOLATILE_CONTEXT_POINTERS { R4: ::minwindef::PDWORD, R5: ::minwindef::PDWORD, R6: ::minwindef::PDWORD, R7: ::minwindef::PDWORD, R8: ::minwindef::PDWORD, R9: ::minwindef::PDWORD, R10: ::minwindef::PDWORD, R11: ::minwindef::PDWORD, Lr: ::minwindef::PDWORD, D8: ::winnt::PULONGLONG, D9: ::winnt::PULONGLONG, D10: ::winnt::PULONGLONG, D11: ::winnt::PULONGLONG, D12: ::winnt::PULONGLONG, D13: ::winnt::PULONGLONG, D14: ::winnt::PULONGLONG, D15: ::winnt::PULONGLONG } /* winnt.h:4780:16 */
#[cfg(any(target_arch="arm"))] #[repr(C)] pub struct RUNTIME_FUNCTION { BeginAddress: ::minwindef::DWORD, u: ::winnt::RUNTIME_FUNCTION_Child_2 } /* winnt.h:16517:16 */
#[cfg(any(target_arch="arm"))] #[repr(C)] pub /*union*/ struct RUNTIME_FUNCTION_Child_2; /* STUB! */ /* winnt.h:16519:5 */
#[cfg(any(target_arch="arm"))] pub type IMAGE_ARM_RUNTIME_FUNCTION_ENTRY = ::winnt::RUNTIME_FUNCTION; /* winnt.h:16533:3 */
#[cfg(any(target_arch="arm"))] pub type PIMAGE_ARM_RUNTIME_FUNCTION_ENTRY = *mut ::winnt::RUNTIME_FUNCTION; /* winnt.h:16533:39 */
#[cfg(any(target_arch="arm"))] pub type IMAGE_RUNTIME_FUNCTION_ENTRY = ::winnt::IMAGE_ARM_RUNTIME_FUNCTION_ENTRY; /* winnt.h:16577:44 */
#[cfg(any(target_arch="arm"))] pub type PIMAGE_RUNTIME_FUNCTION_ENTRY = ::winnt::PIMAGE_ARM_RUNTIME_FUNCTION_ENTRY; /* winnt.h:16578:43 */
#[cfg(any(target_arch="arm"))] pub const SYSTEM_CACHE_ALIGNMENT_SIZE: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnt.h:210:9 */
#[cfg(any(target_arch="arm"))] pub const PF_TEMPORAL_LEVEL_1: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:4467:9 */
#[cfg(any(target_arch="arm"))] pub const PF_TEMPORAL_LEVEL_2: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:4468:9 */
#[cfg(any(target_arch="arm"))] pub const PF_TEMPORAL_LEVEL_3: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnt.h:4469:9 */
#[cfg(any(target_arch="arm"))] pub const PF_NON_TEMPORAL_LEVEL_ALL: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnt.h:4470:9 */
#[cfg(any(target_arch="arm"))] pub const CONTEXT_ARM: i64 = 0x200000i64; /* Integer(2097152, Yes, Long) */ /* winnt.h:4528:9 */
#[cfg(any(target_arch="arm"))] pub const CONTEXT_UNWOUND_TO_CALL: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* winnt.h:4553:9 */
#[cfg(any(target_arch="arm"))] pub const INITIAL_CPSR: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnt.h:4563:9 */
#[cfg(any(target_arch="arm"))] pub const INITIAL_FPSCR: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnt.h:4564:9 */
#[cfg(any(target_arch="arm"))] pub const ARM_MAX_BREAKPOINTS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnt.h:4572:9 */
#[cfg(any(target_arch="arm"))] pub const ARM_MAX_WATCHPOINTS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnt.h:4573:9 */
