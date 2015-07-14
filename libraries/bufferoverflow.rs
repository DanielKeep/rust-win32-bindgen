#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn RtlVirtualUnwind(_: ::minwindef::DWORD, _: ::basetsd::DWORD64, _: ::basetsd::DWORD64, _: ::winnt::PRUNTIME_FUNCTION, _: ::winnt::PCONTEXT, _: *mut *mut ::libc::c_void, _: ::basetsd::PDWORD64, _: ::winnt::PKNONVOLATILE_CONTEXT_POINTERS) -> ::winnt::PEXCEPTION_ROUTINE; /* winnt.h:17104:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn RtlVirtualUnwind(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PRUNTIME_FUNCTION, _: ::winnt::PCONTEXT, _: *mut *mut ::libc::c_void, _: ::minwindef::PDWORD, _: ::winnt::PKNONVOLATILE_CONTEXT_POINTERS) -> ::winnt::PEXCEPTION_ROUTINE; /* winnt.h:17262:1 */
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
