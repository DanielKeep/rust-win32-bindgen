#[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn ReadTimeStampCounter() -> ::basetsd::DWORD64; /* winnt.h:4478:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn RtlVirtualUnwind(HandlerType: ::minwindef::DWORD, ImageBase: ::basetsd::DWORD64, ControlPc: ::basetsd::DWORD64, FunctionEntry: ::winnt::PRUNTIME_FUNCTION, ContextRecord: ::winnt::PCONTEXT, HandlerData: *mut *mut ::libc::c_void, EstablisherFrame: ::basetsd::PDWORD64, ContextPointers: ::winnt::PKNONVOLATILE_CONTEXT_POINTERS) -> ::winnt::PEXCEPTION_ROUTINE; /* winnt.h:17104:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn RtlVirtualUnwind(HandlerType: ::minwindef::DWORD, ImageBase: ::minwindef::DWORD, ControlPc: ::minwindef::DWORD, FunctionEntry: ::winnt::PRUNTIME_FUNCTION, ContextRecord: ::winnt::PCONTEXT, HandlerData: *mut *mut ::libc::c_void, EstablisherFrame: ::minwindef::PDWORD, ContextPointers: ::winnt::PKNONVOLATILE_CONTEXT_POINTERS) -> ::winnt::PEXCEPTION_ROUTINE; /* winnt.h:17262:1 */
}
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RtlFirstEntrySList(ListHead: *const ::winnt::SLIST_HEADER) -> ::winnt::PSLIST_ENTRY; /* winnt.h:17553:1 */
    pub fn RtlInitializeSListHead(ListHead: ::winnt::PSLIST_HEADER); /* winnt.h:17545:1 */
    pub fn RtlInterlockedFlushSList(ListHead: ::winnt::PSLIST_HEADER) -> ::winnt::PSLIST_ENTRY; /* winnt.h:17585:1 */
    pub fn RtlInterlockedPopEntrySList(ListHead: ::winnt::PSLIST_HEADER) -> ::winnt::PSLIST_ENTRY; /* winnt.h:17560:1 */
    pub fn RtlInterlockedPushEntrySList(ListHead: ::winnt::PSLIST_HEADER, ListEntry: ::winnt::PSLIST_ENTRY) -> ::winnt::PSLIST_ENTRY; /* winnt.h:17567:1 */
    pub fn RtlInterlockedPushListSListEx(ListHead: ::winnt::PSLIST_HEADER, List: ::winnt::PSLIST_ENTRY, ListEnd: ::winnt::PSLIST_ENTRY, Count: ::minwindef::DWORD) -> ::winnt::PSLIST_ENTRY; /* winnt.h:17575:1 */
    pub fn RtlQueryDepthSList(ListHead: ::winnt::PSLIST_HEADER) -> ::minwindef::WORD; /* winnt.h:17592:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn RtlLookupFunctionEntry(ControlPc: ::basetsd::DWORD64, ImageBase: ::basetsd::PDWORD64, HistoryTable: ::winnt::PUNWIND_HISTORY_TABLE) -> ::winnt::PRUNTIME_FUNCTION; /* winnt.h:17051:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn RtlLookupFunctionEntry(ControlPc: ::basetsd::ULONG_PTR, ImageBase: ::minwindef::PDWORD, HistoryTable: ::winnt::PUNWIND_HISTORY_TABLE) -> ::winnt::PRUNTIME_FUNCTION; /* winnt.h:17209:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000100"))] 
extern "system" {
    pub fn RtlCaptureContext(ContextRecord: ::winnt::PCONTEXT); /* winnt.h:16934:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RtlGetProductInfo(OSMajorVersion: ::minwindef::DWORD, OSMinorVersion: ::minwindef::DWORD, SpMajorVersion: ::minwindef::DWORD, SpMinorVersion: ::minwindef::DWORD, ReturnedProductType: ::minwindef::PDWORD) -> ::winnt::BOOLEAN; /* winnt.h:18011:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[cfg(any(target_arch="x86_64", target_arch="arm"))] 
extern "system" {
    pub fn RtlAddGrowableFunctionTable(DynamicTable: *mut *mut ::libc::c_void, FunctionTable: ::winnt::PRUNTIME_FUNCTION, EntryCount: ::minwindef::DWORD, MaximumEntryCount: ::minwindef::DWORD, RangeBase: ::basetsd::ULONG_PTR, RangeEnd: ::basetsd::ULONG_PTR) -> ::minwindef::DWORD; /* winnt.h:17010:1, winnt.h:17168:1 */
    pub fn RtlDeleteGrowableFunctionTable(DynamicTable: ::winnt::PVOID); /* winnt.h:17032:1, winnt.h:17190:1 */
    pub fn RtlGrowFunctionTable(DynamicTable: ::winnt::PVOID, NewEntryCount: ::minwindef::DWORD); /* winnt.h:17023:1, winnt.h:17181:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn RtlCrc32(Buffer: *const ::libc::c_void, Size: ::libc::c_int, InitialCrc: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnt.h:18120:1 */
    pub fn RtlCrc64(Buffer: *const ::libc::c_void, Size: ::libc::c_int, InitialCrc: ::winnt::ULONGLONG) -> ::winnt::ULONGLONG; /* winnt.h:18129:1 */
}
