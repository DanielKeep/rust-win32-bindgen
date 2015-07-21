#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn RtlVirtualUnwind(HandlerType: ::minwindef::DWORD, ImageBase: ::basetsd::DWORD64, ControlPc: ::basetsd::DWORD64, FunctionEntry: ::winnt::PRUNTIME_FUNCTION, ContextRecord: ::winnt::PCONTEXT, HandlerData: *mut *mut ::libc::c_void, EstablisherFrame: ::basetsd::PDWORD64, ContextPointers: ::winnt::PKNONVOLATILE_CONTEXT_POINTERS) -> ::winnt::PEXCEPTION_ROUTINE; /* winnt.h:17104:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn RtlVirtualUnwind(HandlerType: ::minwindef::DWORD, ImageBase: ::minwindef::DWORD, ControlPc: ::minwindef::DWORD, FunctionEntry: ::winnt::PRUNTIME_FUNCTION, ContextRecord: ::winnt::PCONTEXT, HandlerData: *mut *mut ::libc::c_void, EstablisherFrame: ::minwindef::PDWORD, ContextPointers: ::winnt::PKNONVOLATILE_CONTEXT_POINTERS) -> ::winnt::PEXCEPTION_ROUTINE; /* winnt.h:17262:1 */
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
