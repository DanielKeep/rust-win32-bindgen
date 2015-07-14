#[repr(C)] pub struct SECURITY_ATTRIBUTES { nLength: ::minwindef::DWORD, lpSecurityDescriptor: ::minwindef::LPVOID, bInheritHandle: ::minwindef::BOOL } /* minwinbase.h:45:16, minwinbase.h:45:16, minwinbase.h:45:16 */
pub type PSECURITY_ATTRIBUTES = *mut ::minwinbase::SECURITY_ATTRIBUTES; /* minwinbase.h:49:25, minwinbase.h:49:25, minwinbase.h:49:25 */
pub type LPSECURITY_ATTRIBUTES = *mut ::minwinbase::SECURITY_ATTRIBUTES; /* minwinbase.h:49:48, minwinbase.h:49:48, minwinbase.h:49:48 */
#[repr(C)] pub struct OVERLAPPED { Internal: ::basetsd::ULONG_PTR, InternalHigh: ::basetsd::ULONG_PTR, u: ::minwinbase::OVERLAPPED_Child_2, hEvent: ::winnt::HANDLE } /* minwinbase.h:51:16, minwinbase.h:51:16, minwinbase.h:51:16 */
#[repr(C)] pub /*union*/ struct OVERLAPPED_Child_2; /* STUB! */ /* minwinbase.h:54:5, minwinbase.h:54:5, minwinbase.h:54:5 */
pub type LPOVERLAPPED = *mut ::minwinbase::OVERLAPPED; /* minwinbase.h:63:16, minwinbase.h:63:16, minwinbase.h:63:16 */
#[repr(C)] pub struct OVERLAPPED_ENTRY { lpCompletionKey: ::basetsd::ULONG_PTR, lpOverlapped: ::minwinbase::LPOVERLAPPED, Internal: ::basetsd::ULONG_PTR, dwNumberOfBytesTransferred: ::minwindef::DWORD } /* minwinbase.h:65:16, minwinbase.h:65:16, minwinbase.h:65:16 */
pub type LPOVERLAPPED_ENTRY = *mut ::minwinbase::OVERLAPPED_ENTRY; /* minwinbase.h:70:22, minwinbase.h:70:22, minwinbase.h:70:22 */
#[repr(C)] pub struct SYSTEMTIME { wYear: ::minwindef::WORD, wMonth: ::minwindef::WORD, wDayOfWeek: ::minwindef::WORD, wDay: ::minwindef::WORD, wHour: ::minwindef::WORD, wMinute: ::minwindef::WORD, wSecond: ::minwindef::WORD, wMilliseconds: ::minwindef::WORD } /* minwinbase.h:89:16, minwinbase.h:89:16, minwinbase.h:89:16 */
pub type PSYSTEMTIME = *mut ::minwinbase::SYSTEMTIME; /* minwinbase.h:98:16, minwinbase.h:98:16, minwinbase.h:98:16 */
pub type LPSYSTEMTIME = *mut ::minwinbase::SYSTEMTIME; /* minwinbase.h:98:30, minwinbase.h:98:30, minwinbase.h:98:30 */
#[repr(C)] pub struct WIN32_FIND_DATAA { dwFileAttributes: ::minwindef::DWORD, ftCreationTime: ::minwindef::FILETIME, ftLastAccessTime: ::minwindef::FILETIME, ftLastWriteTime: ::minwindef::FILETIME, nFileSizeHigh: ::minwindef::DWORD, nFileSizeLow: ::minwindef::DWORD, dwReserved0: ::minwindef::DWORD, dwReserved1: ::minwindef::DWORD, cFileName: *mut [::winnt::CHAR; 260], cAlternateFileName: *mut [::winnt::CHAR; 14] } /* minwinbase.h:101:16, minwinbase.h:101:16, minwinbase.h:101:16 */
pub type PWIN32_FIND_DATAA = *mut ::minwinbase::WIN32_FIND_DATAA; /* minwinbase.h:117:22, minwinbase.h:117:22, minwinbase.h:117:22 */
pub type LPWIN32_FIND_DATAA = *mut ::minwinbase::WIN32_FIND_DATAA; /* minwinbase.h:117:42, minwinbase.h:117:42, minwinbase.h:117:42 */
#[repr(C)] pub struct WIN32_FIND_DATAW { dwFileAttributes: ::minwindef::DWORD, ftCreationTime: ::minwindef::FILETIME, ftLastAccessTime: ::minwindef::FILETIME, ftLastWriteTime: ::minwindef::FILETIME, nFileSizeHigh: ::minwindef::DWORD, nFileSizeLow: ::minwindef::DWORD, dwReserved0: ::minwindef::DWORD, dwReserved1: ::minwindef::DWORD, cFileName: *mut [::winnt::WCHAR; 260], cAlternateFileName: *mut [::winnt::WCHAR; 14] } /* minwinbase.h:118:16, minwinbase.h:118:16, minwinbase.h:118:16 */
pub type PWIN32_FIND_DATAW = *mut ::minwinbase::WIN32_FIND_DATAW; /* minwinbase.h:134:22, minwinbase.h:134:22, minwinbase.h:134:22 */
pub type LPWIN32_FIND_DATAW = *mut ::minwinbase::WIN32_FIND_DATAW; /* minwinbase.h:134:42, minwinbase.h:134:42, minwinbase.h:134:42 */
pub type WIN32_FIND_DATA = ::minwinbase::WIN32_FIND_DATAW; /* minwinbase.h:136:26, minwinbase.h:136:26, minwinbase.h:136:26 */
pub type PWIN32_FIND_DATA = ::minwinbase::PWIN32_FIND_DATAW; /* minwinbase.h:137:27, minwinbase.h:137:27, minwinbase.h:137:27 */
pub type LPWIN32_FIND_DATA = ::minwinbase::LPWIN32_FIND_DATAW; /* minwinbase.h:138:28, minwinbase.h:138:28, minwinbase.h:138:28 */
#[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub enum FINDEX_INFO_LEVELS {FindExInfoStandard = 0, FindExInfoBasic = 1, FindExInfoMaxInfoLevel = 2} pub use self::FINDEX_INFO_LEVELS::{FindExInfoStandard, FindExInfoBasic, FindExInfoMaxInfoLevel}; /* minwinbase.h:147:14, minwinbase.h:147:14, minwinbase.h:147:14 */
#[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub enum FINDEX_SEARCH_OPS {FindExSearchNameMatch = 0, FindExSearchLimitToDirectories = 1, FindExSearchLimitToDevices = 2, FindExSearchMaxSearchOp = 3} pub use self::FINDEX_SEARCH_OPS::{FindExSearchNameMatch, FindExSearchLimitToDirectories, FindExSearchLimitToDevices, FindExSearchMaxSearchOp}; /* minwinbase.h:156:14, minwinbase.h:156:14, minwinbase.h:156:14 */
#[repr(C)] pub enum GET_FILEEX_INFO_LEVELS {GetFileExInfoStandard = 0, GetFileExMaxInfoLevel = 1} pub use self::GET_FILEEX_INFO_LEVELS::{GetFileExInfoStandard, GetFileExMaxInfoLevel}; /* minwinbase.h:164:14, minwinbase.h:164:14, minwinbase.h:164:14 */
#[cfg(any(feature="winapi_ver_06000000"))] #[repr(C)] pub enum FILE_INFO_BY_HANDLE_CLASS {FileBasicInfo = 0, FileStandardInfo = 1, FileNameInfo = 2, FileRenameInfo = 3, FileDispositionInfo = 4, FileAllocationInfo = 5, FileEndOfFileInfo = 6, FileStreamInfo = 7, FileCompressionInfo = 8, FileAttributeTagInfo = 9, FileIdBothDirectoryInfo = 10, FileIdBothDirectoryRestartInfo = 11, FileIoPriorityHintInfo = 12, FileRemoteProtocolInfo = 13, FileFullDirectoryInfo = 14, FileFullDirectoryRestartInfo = 15, FileStorageInfo = 16, FileAlignmentInfo = 17, FileIdInfo = 18, FileIdExtdDirectoryInfo = 19, FileIdExtdDirectoryRestartInfo = 20, MaximumFileInfoByHandleClass = 21} pub use self::FILE_INFO_BY_HANDLE_CLASS::{FileBasicInfo, FileStandardInfo, FileNameInfo, FileRenameInfo, FileDispositionInfo, FileAllocationInfo, FileEndOfFileInfo, FileStreamInfo, FileCompressionInfo, FileAttributeTagInfo, FileIdBothDirectoryInfo, FileIdBothDirectoryRestartInfo, FileIoPriorityHintInfo, FileRemoteProtocolInfo, FileFullDirectoryInfo, FileFullDirectoryRestartInfo, FileStorageInfo, FileAlignmentInfo, FileIdInfo, FileIdExtdDirectoryInfo, FileIdExtdDirectoryRestartInfo, MaximumFileInfoByHandleClass}; /* minwinbase.h:170:14, minwinbase.h:170:14, minwinbase.h:170:14 */
#[cfg(any(feature="winapi_ver_06000000"))] pub type PFILE_INFO_BY_HANDLE_CLASS = *mut ::minwinbase::FILE_INFO_BY_HANDLE_CLASS; /* minwinbase.h:195:31, minwinbase.h:195:31, minwinbase.h:195:31 */
pub type CRITICAL_SECTION = ::winnt::RTL_CRITICAL_SECTION; /* minwinbase.h:198:30, minwinbase.h:198:30, minwinbase.h:198:30 */
pub type PCRITICAL_SECTION = ::winnt::PRTL_CRITICAL_SECTION; /* minwinbase.h:199:31, minwinbase.h:199:31, minwinbase.h:199:31 */
pub type LPCRITICAL_SECTION = ::winnt::PRTL_CRITICAL_SECTION; /* minwinbase.h:200:31, minwinbase.h:200:31, minwinbase.h:200:31 */
pub type CRITICAL_SECTION_DEBUG = ::winnt::RTL_CRITICAL_SECTION_DEBUG; /* minwinbase.h:202:36, minwinbase.h:202:36, minwinbase.h:202:36 */
pub type PCRITICAL_SECTION_DEBUG = ::winnt::PRTL_CRITICAL_SECTION_DEBUG; /* minwinbase.h:203:37, minwinbase.h:203:37, minwinbase.h:203:37 */
pub type LPCRITICAL_SECTION_DEBUG = ::winnt::PRTL_CRITICAL_SECTION_DEBUG; /* minwinbase.h:204:37, minwinbase.h:204:37, minwinbase.h:204:37 */
pub type LPOVERLAPPED_COMPLETION_ROUTINE = extern "system" fn(::libc::c_ulong, ::libc::c_ulong, *mut ::minwinbase::OVERLAPPED); /* minwinbase.h:208:10, minwinbase.h:208:10, minwinbase.h:208:10 */
#[repr(C)] pub struct PROCESS_HEAP_ENTRY { lpData: ::winnt::PVOID, cbData: ::minwindef::DWORD, cbOverhead: ::minwindef::BYTE, iRegionIndex: ::minwindef::BYTE, wFlags: ::minwindef::WORD, u: ::minwinbase::PROCESS_HEAP_ENTRY_Child_5 } /* minwinbase.h:217:16, minwinbase.h:217:16, minwinbase.h:217:16 */
#[repr(C)] pub /*union*/ struct PROCESS_HEAP_ENTRY_Child_5; /* STUB! */ /* minwinbase.h:223:5, minwinbase.h:223:5, minwinbase.h:223:5 */
pub type LPPROCESS_HEAP_ENTRY = *mut ::minwinbase::PROCESS_HEAP_ENTRY; /* minwinbase.h:235:24, minwinbase.h:235:24, minwinbase.h:235:24 */
pub type PPROCESS_HEAP_ENTRY = *mut ::minwinbase::PROCESS_HEAP_ENTRY; /* minwinbase.h:235:47, minwinbase.h:235:47, minwinbase.h:235:47 */
#[repr(C)] pub struct REASON_CONTEXT { Version: ::minwindef::ULONG, Flags: ::minwindef::DWORD, Reason: ::minwinbase::REASON_CONTEXT_Child_2 } /* minwinbase.h:244:16, minwinbase.h:244:16, minwinbase.h:244:16 */
#[repr(C)] pub /*union*/ struct REASON_CONTEXT_Child_2; /* STUB! */ /* minwinbase.h:247:5, minwinbase.h:247:5, minwinbase.h:247:5 */
pub type PREASON_CONTEXT = *mut ::minwinbase::REASON_CONTEXT; /* minwinbase.h:258:20, minwinbase.h:258:20, minwinbase.h:258:20 */
pub type PTHREAD_START_ROUTINE = extern "system" fn(*mut ::libc::c_void) -> ::libc::c_ulong; /* minwinbase.h:273:24, minwinbase.h:273:24, minwinbase.h:273:24 */
pub type LPTHREAD_START_ROUTINE = ::minwinbase::PTHREAD_START_ROUTINE; /* minwinbase.h:276:31, minwinbase.h:276:31, minwinbase.h:276:31 */
#[repr(C)] pub struct EXCEPTION_DEBUG_INFO { ExceptionRecord: ::winnt::EXCEPTION_RECORD, dwFirstChance: ::minwindef::DWORD } /* minwinbase.h:278:16, minwinbase.h:278:16, minwinbase.h:278:16 */
pub type LPEXCEPTION_DEBUG_INFO = *mut ::minwinbase::EXCEPTION_DEBUG_INFO; /* minwinbase.h:281:26, minwinbase.h:281:26, minwinbase.h:281:26 */
#[repr(C)] pub struct CREATE_THREAD_DEBUG_INFO { hThread: ::winnt::HANDLE, lpThreadLocalBase: ::minwindef::LPVOID, lpStartAddress: ::minwinbase::LPTHREAD_START_ROUTINE } /* minwinbase.h:283:16, minwinbase.h:283:16, minwinbase.h:283:16 */
pub type LPCREATE_THREAD_DEBUG_INFO = *mut ::minwinbase::CREATE_THREAD_DEBUG_INFO; /* minwinbase.h:287:30, minwinbase.h:287:30, minwinbase.h:287:30 */
#[repr(C)] pub struct CREATE_PROCESS_DEBUG_INFO { hFile: ::winnt::HANDLE, hProcess: ::winnt::HANDLE, hThread: ::winnt::HANDLE, lpBaseOfImage: ::minwindef::LPVOID, dwDebugInfoFileOffset: ::minwindef::DWORD, nDebugInfoSize: ::minwindef::DWORD, lpThreadLocalBase: ::minwindef::LPVOID, lpStartAddress: ::minwinbase::LPTHREAD_START_ROUTINE, lpImageName: ::minwindef::LPVOID, fUnicode: ::minwindef::WORD } /* minwinbase.h:289:16, minwinbase.h:289:16, minwinbase.h:289:16 */
pub type LPCREATE_PROCESS_DEBUG_INFO = *mut ::minwinbase::CREATE_PROCESS_DEBUG_INFO; /* minwinbase.h:300:31, minwinbase.h:300:31, minwinbase.h:300:31 */
#[repr(C)] pub struct EXIT_THREAD_DEBUG_INFO { dwExitCode: ::minwindef::DWORD } /* minwinbase.h:302:16, minwinbase.h:302:16, minwinbase.h:302:16 */
pub type LPEXIT_THREAD_DEBUG_INFO = *mut ::minwinbase::EXIT_THREAD_DEBUG_INFO; /* minwinbase.h:304:28, minwinbase.h:304:28, minwinbase.h:304:28 */
#[repr(C)] pub struct EXIT_PROCESS_DEBUG_INFO { dwExitCode: ::minwindef::DWORD } /* minwinbase.h:306:16, minwinbase.h:306:16, minwinbase.h:306:16 */
pub type LPEXIT_PROCESS_DEBUG_INFO = *mut ::minwinbase::EXIT_PROCESS_DEBUG_INFO; /* minwinbase.h:308:29, minwinbase.h:308:29, minwinbase.h:308:29 */
#[repr(C)] pub struct LOAD_DLL_DEBUG_INFO { hFile: ::winnt::HANDLE, lpBaseOfDll: ::minwindef::LPVOID, dwDebugInfoFileOffset: ::minwindef::DWORD, nDebugInfoSize: ::minwindef::DWORD, lpImageName: ::minwindef::LPVOID, fUnicode: ::minwindef::WORD } /* minwinbase.h:310:16, minwinbase.h:310:16, minwinbase.h:310:16 */
pub type LPLOAD_DLL_DEBUG_INFO = *mut ::minwinbase::LOAD_DLL_DEBUG_INFO; /* minwinbase.h:317:25, minwinbase.h:317:25, minwinbase.h:317:25 */
#[repr(C)] pub struct UNLOAD_DLL_DEBUG_INFO { lpBaseOfDll: ::minwindef::LPVOID } /* minwinbase.h:319:16, minwinbase.h:319:16, minwinbase.h:319:16 */
pub type LPUNLOAD_DLL_DEBUG_INFO = *mut ::minwinbase::UNLOAD_DLL_DEBUG_INFO; /* minwinbase.h:321:27, minwinbase.h:321:27, minwinbase.h:321:27 */
#[repr(C)] pub struct OUTPUT_DEBUG_STRING_INFO { lpDebugStringData: ::winnt::LPSTR, fUnicode: ::minwindef::WORD, nDebugStringLength: ::minwindef::WORD } /* minwinbase.h:323:16, minwinbase.h:323:16, minwinbase.h:323:16 */
pub type LPOUTPUT_DEBUG_STRING_INFO = *mut ::minwinbase::OUTPUT_DEBUG_STRING_INFO; /* minwinbase.h:327:30, minwinbase.h:327:30, minwinbase.h:327:30 */
#[repr(C)] pub struct RIP_INFO { dwError: ::minwindef::DWORD, dwType: ::minwindef::DWORD } /* minwinbase.h:329:16, minwinbase.h:329:16, minwinbase.h:329:16 */
pub type LPRIP_INFO = *mut ::minwinbase::RIP_INFO; /* minwinbase.h:332:14, minwinbase.h:332:14, minwinbase.h:332:14 */
#[repr(C)] pub struct DEBUG_EVENT { dwDebugEventCode: ::minwindef::DWORD, dwProcessId: ::minwindef::DWORD, dwThreadId: ::minwindef::DWORD, u: ::minwinbase::DEBUG_EVENT_Child_3 } /* minwinbase.h:335:16, minwinbase.h:335:16, minwinbase.h:335:16 */
#[repr(C)] pub /*union*/ struct DEBUG_EVENT_Child_3; /* STUB! */ /* minwinbase.h:339:5, minwinbase.h:339:5, minwinbase.h:339:5 */
pub type LPDEBUG_EVENT = *mut ::minwinbase::DEBUG_EVENT; /* minwinbase.h:350:17, minwinbase.h:350:17, minwinbase.h:350:17 */
pub type LPCONTEXT = ::winnt::PCONTEXT; /* minwinbase.h:358:18, minwinbase.h:358:18, minwinbase.h:358:18 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FIND_FIRST_EX_CASE_SENSITIVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* minwinbase.h:153:9, minwinbase.h:153:9, minwinbase.h:153:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FIND_FIRST_EX_LARGE_FETCH: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* minwinbase.h:154:9, minwinbase.h:154:9, minwinbase.h:154:9 */
pub const LOCKFILE_FAIL_IMMEDIATELY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* minwinbase.h:214:9, minwinbase.h:214:9, minwinbase.h:214:9 */
pub const LOCKFILE_EXCLUSIVE_LOCK: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* minwinbase.h:215:9, minwinbase.h:215:9, minwinbase.h:215:9 */
pub const PROCESS_HEAP_REGION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* minwinbase.h:237:9, minwinbase.h:237:9, minwinbase.h:237:9 */
pub const PROCESS_HEAP_UNCOMMITTED_RANGE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* minwinbase.h:238:9, minwinbase.h:238:9, minwinbase.h:238:9 */
pub const PROCESS_HEAP_ENTRY_BUSY: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* minwinbase.h:239:9, minwinbase.h:239:9, minwinbase.h:239:9 */
pub const PROCESS_HEAP_SEG_ALLOC: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* minwinbase.h:240:9, minwinbase.h:240:9, minwinbase.h:240:9 */
pub const PROCESS_HEAP_ENTRY_MOVEABLE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* minwinbase.h:241:9, minwinbase.h:241:9, minwinbase.h:241:9 */
pub const PROCESS_HEAP_ENTRY_DDESHARE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* minwinbase.h:242:9, minwinbase.h:242:9, minwinbase.h:242:9 */
pub const EXCEPTION_DEBUG_EVENT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* minwinbase.h:263:9, minwinbase.h:263:9, minwinbase.h:263:9 */
pub const CREATE_THREAD_DEBUG_EVENT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* minwinbase.h:264:9, minwinbase.h:264:9, minwinbase.h:264:9 */
pub const CREATE_PROCESS_DEBUG_EVENT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* minwinbase.h:265:9, minwinbase.h:265:9, minwinbase.h:265:9 */
pub const EXIT_THREAD_DEBUG_EVENT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* minwinbase.h:266:9, minwinbase.h:266:9, minwinbase.h:266:9 */
pub const EXIT_PROCESS_DEBUG_EVENT: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* minwinbase.h:267:9, minwinbase.h:267:9, minwinbase.h:267:9 */
pub const LOAD_DLL_DEBUG_EVENT: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* minwinbase.h:268:9, minwinbase.h:268:9, minwinbase.h:268:9 */
pub const UNLOAD_DLL_DEBUG_EVENT: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* minwinbase.h:269:9, minwinbase.h:269:9, minwinbase.h:269:9 */
pub const OUTPUT_DEBUG_STRING_EVENT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* minwinbase.h:270:9, minwinbase.h:270:9, minwinbase.h:270:9 */
pub const RIP_EVENT: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* minwinbase.h:271:9, minwinbase.h:271:9, minwinbase.h:271:9 */
pub const LMEM_FIXED: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* minwinbase.h:393:9, minwinbase.h:393:9, minwinbase.h:393:9 */
pub const LMEM_MOVEABLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* minwinbase.h:394:9, minwinbase.h:394:9, minwinbase.h:394:9 */
pub const LMEM_NOCOMPACT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* minwinbase.h:395:9, minwinbase.h:395:9, minwinbase.h:395:9 */
pub const LMEM_NODISCARD: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* minwinbase.h:396:9, minwinbase.h:396:9, minwinbase.h:396:9 */
pub const LMEM_ZEROINIT: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* minwinbase.h:397:9, minwinbase.h:397:9, minwinbase.h:397:9 */
pub const LMEM_MODIFY: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* minwinbase.h:398:9, minwinbase.h:398:9, minwinbase.h:398:9 */
pub const LMEM_DISCARDABLE: i32 = 0xf00i32; /* Integer(3840, Yes, Unknown) */ /* minwinbase.h:399:9, minwinbase.h:399:9, minwinbase.h:399:9 */
pub const LMEM_VALID_FLAGS: i32 = 0xf72i32; /* Integer(3954, Yes, Unknown) */ /* minwinbase.h:400:9, minwinbase.h:400:9, minwinbase.h:400:9 */
pub const LMEM_INVALID_HANDLE: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* minwinbase.h:401:9, minwinbase.h:401:9, minwinbase.h:401:9 */
pub const LMEM_DISCARDED: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* minwinbase.h:412:9, minwinbase.h:412:9, minwinbase.h:412:9 */
pub const LMEM_LOCKCOUNT: i32 = 0xffi32; /* Integer(255, Yes, Unknown) */ /* minwinbase.h:413:9, minwinbase.h:413:9, minwinbase.h:413:9 */
