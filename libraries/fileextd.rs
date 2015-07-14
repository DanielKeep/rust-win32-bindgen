#[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn ReadTimeStampCounter() -> ::basetsd::DWORD64; /* winnt.h:4478:1 */
}
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetModuleHandleA(_: ::winnt::LPCSTR) -> ::minwindef::HMODULE; /* libloaderapi.h:276:1 */
    pub fn TlsGetValue(_: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* processthreadsapi.h:468:1 */
    pub fn UnhandledExceptionFilter(_: *mut ::winnt::EXCEPTION_POINTERS) -> ::winnt::LONG; /* errhandlingapi.h:92:1 */
}
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn GetProcAddress(_: ::minwindef::HMODULE, _: ::winnt::LPCSTR) -> ::minwindef::FARPROC; /* libloaderapi.h:360:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn SetFileInformationByHandle(_: ::winnt::HANDLE, _: ::minwinbase::FILE_INFO_BY_HANDLE_CLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1037:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_0a000000"))] 
extern "system" {
    pub fn SetProtectedPolicy(_: ::guiddef::LPCGUID, _: ::basetsd::ULONG_PTR, _: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* processthreadsapi.h:1175:1 */
}
