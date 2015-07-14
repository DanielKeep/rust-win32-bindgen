#[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn ReadTimeStampCounter() -> ::basetsd::DWORD64; /* winnt.h:4478:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn RtlVirtualUnwind(_: ::minwindef::DWORD, _: ::basetsd::DWORD64, _: ::basetsd::DWORD64, _: ::winnt::PRUNTIME_FUNCTION, _: ::winnt::PCONTEXT, _: *mut *mut ::libc::c_void, _: ::basetsd::PDWORD64, _: ::winnt::PKNONVOLATILE_CONTEXT_POINTERS) -> ::winnt::PEXCEPTION_ROUTINE; /* winnt.h:17104:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn RtlVirtualUnwind(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PRUNTIME_FUNCTION, _: ::winnt::PCONTEXT, _: *mut *mut ::libc::c_void, _: ::minwindef::PDWORD, _: ::winnt::PKNONVOLATILE_CONTEXT_POINTERS) -> ::winnt::PEXCEPTION_ROUTINE; /* winnt.h:17262:1 */
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
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn RtlLookupFunctionEntry(_: ::basetsd::DWORD64, _: ::basetsd::PDWORD64, _: ::winnt::PUNWIND_HISTORY_TABLE) -> ::winnt::PRUNTIME_FUNCTION; /* winnt.h:17051:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn RtlLookupFunctionEntry(_: ::basetsd::ULONG_PTR, _: ::minwindef::PDWORD, _: ::winnt::PUNWIND_HISTORY_TABLE) -> ::winnt::PRUNTIME_FUNCTION; /* winnt.h:17209:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000100"))] 
extern "system" {
    pub fn RtlCaptureContext(_: ::winnt::PCONTEXT); /* winnt.h:16934:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RtlGetProductInfo(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::winnt::BOOLEAN; /* winnt.h:18011:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[cfg(any(target_arch="x86_64", target_arch="arm"))] 
extern "system" {
    pub fn RtlAddGrowableFunctionTable(_: *mut *mut ::libc::c_void, _: ::winnt::PRUNTIME_FUNCTION, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::basetsd::ULONG_PTR, _: ::basetsd::ULONG_PTR) -> ::minwindef::DWORD; /* winnt.h:17010:1, winnt.h:17168:1 */
    pub fn RtlDeleteGrowableFunctionTable(_: ::winnt::PVOID); /* winnt.h:17032:1, winnt.h:17190:1 */
    pub fn RtlGrowFunctionTable(_: ::winnt::PVOID, _: ::minwindef::DWORD); /* winnt.h:17023:1, winnt.h:17181:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn RtlCrc32(_: *const ::libc::c_void, _: ::libc::c_int, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnt.h:18120:1 */
    pub fn RtlCrc64(_: *const ::libc::c_void, _: ::libc::c_int, _: ::winnt::ULONGLONG) -> ::winnt::ULONGLONG; /* winnt.h:18129:1 */
}
