#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn RtlAddFunctionTable(FunctionTable: ::winnt::PRUNTIME_FUNCTION, EntryCount: ::minwindef::DWORD, BaseAddress: ::basetsd::DWORD64) -> ::winnt::BOOLEAN; /* winnt.h:16974:1 */
    pub fn RtlInstallFunctionTableCallback(TableIdentifier: ::basetsd::DWORD64, BaseAddress: ::basetsd::DWORD64, Length: ::minwindef::DWORD, Callback: ::winnt::PGET_RUNTIME_FUNCTION_CALLBACK, Context: ::winnt::PVOID, OutOfProcessCallbackDll: ::winnt::PCWSTR) -> ::winnt::BOOLEAN; /* winnt.h:16992:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn RtlAddFunctionTable(FunctionTable: ::winnt::PRUNTIME_FUNCTION, EntryCount: ::minwindef::DWORD, BaseAddress: ::minwindef::DWORD) -> ::winnt::BOOLEAN; /* winnt.h:17132:1 */
    pub fn RtlInstallFunctionTableCallback(TableIdentifier: ::minwindef::DWORD, BaseAddress: ::minwindef::DWORD, Length: ::minwindef::DWORD, Callback: ::winnt::PGET_RUNTIME_FUNCTION_CALLBACK, Context: ::winnt::PVOID, OutOfProcessCallbackDll: ::winnt::PCWSTR) -> ::winnt::BOOLEAN; /* winnt.h:17150:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64", target_arch="arm"))] 
extern "system" {
    pub fn RtlDeleteFunctionTable(FunctionTable: ::winnt::PRUNTIME_FUNCTION) -> ::winnt::BOOLEAN; /* winnt.h:16984:1, winnt.h:17142:1 */
    pub fn RtlRestoreContext(ContextRecord: ::winnt::PCONTEXT, ExceptionRecord: *mut ::winnt::EXCEPTION_RECORD); /* winnt.h:17068:1, winnt.h:17226:1 */
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
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64", target_arch="arm"))] 
extern "system" {
    pub fn RtlUnwindEx(TargetFrame: ::winnt::PVOID, TargetIp: ::winnt::PVOID, ExceptionRecord: ::winnt::PEXCEPTION_RECORD, ReturnValue: ::winnt::PVOID, ContextRecord: ::winnt::PCONTEXT, HistoryTable: ::winnt::PUNWIND_HISTORY_TABLE); /* winnt.h:17084:1, winnt.h:17242:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::RtlCaptureStackBackTrace as CaptureStackBackTrace; /* winbase.h:112:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn RtlPcToFileHeader(PcValue: ::winnt::PVOID, BaseOfImage: *mut *mut ::libc::c_void) -> ::winnt::PVOID; /* winnt.h:17417:1 */
    pub fn RtlUnwind(TargetFrame: ::winnt::PVOID, TargetIp: ::winnt::PVOID, ExceptionRecord: ::winnt::PEXCEPTION_RECORD, ReturnValue: ::winnt::PVOID); /* winnt.h:16953:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn RtlCompareMemory(Source1: *const ::libc::c_void, Source2: *const ::libc::c_void, Length: ::basetsd::SIZE_T) -> ::basetsd::SIZE_T; /* winnt.h:17442:1 */
    pub fn VerSetConditionMask(ConditionMask: ::winnt::ULONGLONG, TypeMask: ::minwindef::DWORD, Condition: ::minwindef::BYTE) -> ::winnt::ULONGLONG; /* winnt.h:17986:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010100"))] 
extern "system" {
    pub fn RtlCaptureStackBackTrace(FramesToSkip: ::minwindef::DWORD, FramesToCapture: ::minwindef::DWORD, BackTrace: *mut *mut ::libc::c_void, BackTraceHash: ::minwindef::PDWORD) -> ::minwindef::WORD; /* winnt.h:16911:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RtlGetProductInfo(OSMajorVersion: ::minwindef::DWORD, OSMinorVersion: ::minwindef::DWORD, SpMajorVersion: ::minwindef::DWORD, SpMinorVersion: ::minwindef::DWORD, ReturnedProductType: ::minwindef::PDWORD) -> ::winnt::BOOLEAN; /* winnt.h:18011:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn RtlCrc32(Buffer: *const ::libc::c_void, Size: ::libc::c_int, InitialCrc: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnt.h:18120:1 */
    pub fn RtlCrc64(Buffer: *const ::libc::c_void, Size: ::libc::c_int, InitialCrc: ::winnt::ULONGLONG) -> ::winnt::ULONGLONG; /* winnt.h:18129:1 */
}
