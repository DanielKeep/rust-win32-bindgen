#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn RtlAddFunctionTable(_: ::winnt::PRUNTIME_FUNCTION, _: ::minwindef::DWORD, _: ::basetsd::DWORD64) -> ::winnt::BOOLEAN; /* winnt.h:16974:1 */
    pub fn RtlInstallFunctionTableCallback(_: ::basetsd::DWORD64, _: ::basetsd::DWORD64, _: ::minwindef::DWORD, _: ::winnt::PGET_RUNTIME_FUNCTION_CALLBACK, _: ::winnt::PVOID, _: ::winnt::PCWSTR) -> ::winnt::BOOLEAN; /* winnt.h:16992:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn RtlAddFunctionTable(_: ::winnt::PRUNTIME_FUNCTION, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winnt::BOOLEAN; /* winnt.h:17132:1 */
    pub fn RtlInstallFunctionTableCallback(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PGET_RUNTIME_FUNCTION_CALLBACK, _: ::winnt::PVOID, _: ::winnt::PCWSTR) -> ::winnt::BOOLEAN; /* winnt.h:17150:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64", target_arch="arm"))] 
extern "system" {
    pub fn RtlDeleteFunctionTable(_: ::winnt::PRUNTIME_FUNCTION) -> ::winnt::BOOLEAN; /* winnt.h:16984:1, winnt.h:17142:1 */
    pub fn RtlRestoreContext(_: ::winnt::PCONTEXT, _: *mut ::winnt::EXCEPTION_RECORD); /* winnt.h:17068:1, winnt.h:17226:1 */
}
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RtlFirstEntrySList(_: *const ::winnt::SLIST_HEADER) -> ::winnt::PSLIST_ENTRY; /* winnt.h:17553:1 */
    pub fn RtlInitializeSListHead(_: ::winnt::PSLIST_HEADER); /* winnt.h:17545:1 */
    pub fn RtlInterlockedFlushSList(_: ::winnt::PSLIST_HEADER) -> ::winnt::PSLIST_ENTRY; /* winnt.h:17585:1 */
    pub fn RtlInterlockedPopEntrySList(_: ::winnt::PSLIST_HEADER) -> ::winnt::PSLIST_ENTRY; /* winnt.h:17560:1 */
    pub fn RtlInterlockedPushEntrySList(_: ::winnt::PSLIST_HEADER, _: ::winnt::PSLIST_ENTRY) -> ::winnt::PSLIST_ENTRY; /* winnt.h:17567:1 */
    pub fn RtlInterlockedPushListSListEx(_: ::winnt::PSLIST_HEADER, _: ::winnt::PSLIST_ENTRY, _: ::winnt::PSLIST_ENTRY, _: ::minwindef::DWORD) -> ::winnt::PSLIST_ENTRY; /* winnt.h:17575:1 */
    pub fn RtlQueryDepthSList(_: ::winnt::PSLIST_HEADER) -> ::minwindef::WORD; /* winnt.h:17592:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64", target_arch="arm"))] 
extern "system" {
    pub fn RtlUnwindEx(_: ::winnt::PVOID, _: ::winnt::PVOID, _: ::winnt::PEXCEPTION_RECORD, _: ::winnt::PVOID, _: ::winnt::PCONTEXT, _: ::winnt::PUNWIND_HISTORY_TABLE); /* winnt.h:17084:1, winnt.h:17242:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::RtlCaptureStackBackTrace as CaptureStackBackTrace; /* winbase.h:112:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn RtlPcToFileHeader(_: ::winnt::PVOID, _: *mut *mut ::libc::c_void) -> ::winnt::PVOID; /* winnt.h:17417:1 */
    pub fn RtlUnwind(_: ::winnt::PVOID, _: ::winnt::PVOID, _: ::winnt::PEXCEPTION_RECORD, _: ::winnt::PVOID); /* winnt.h:16953:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn RtlCompareMemory(_: *const ::libc::c_void, _: *const ::libc::c_void, _: ::basetsd::SIZE_T) -> ::basetsd::SIZE_T; /* winnt.h:17442:1 */
    pub fn VerSetConditionMask(_: ::winnt::ULONGLONG, _: ::minwindef::DWORD, _: ::minwindef::BYTE) -> ::winnt::ULONGLONG; /* winnt.h:17986:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010100"))] 
extern "system" {
    pub fn RtlCaptureStackBackTrace(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *mut *mut ::libc::c_void, _: ::minwindef::PDWORD) -> ::minwindef::WORD; /* winnt.h:16911:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RtlGetProductInfo(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::winnt::BOOLEAN; /* winnt.h:18011:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn RtlCrc32(_: *const ::libc::c_void, _: ::libc::c_int, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnt.h:18120:1 */
    pub fn RtlCrc64(_: *const ::libc::c_void, _: ::libc::c_int, _: ::winnt::ULONGLONG) -> ::winnt::ULONGLONG; /* winnt.h:18129:1 */
}
