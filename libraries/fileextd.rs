#[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn ReadTimeStampCounter() -> ::basetsd::DWORD64; /* winnt.h:4478:1 */
}
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetModuleHandleA(lpModuleName: ::winnt::LPCSTR) -> ::minwindef::HMODULE; /* libloaderapi.h:276:1 */
    pub fn TlsGetValue(dwTlsIndex: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* processthreadsapi.h:468:1 */
    pub fn UnhandledExceptionFilter(ExceptionInfo: *mut ::winnt::EXCEPTION_POINTERS) -> ::winnt::LONG; /* errhandlingapi.h:92:1 */
}
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn GetProcAddress(hModule: ::minwindef::HMODULE, lpProcName: ::winnt::LPCSTR) -> ::minwindef::FARPROC; /* libloaderapi.h:360:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn SetFileInformationByHandle(hFile: ::winnt::HANDLE, FileInformationClass: ::minwinbase::FILE_INFO_BY_HANDLE_CLASS, lpFileInformation: ::minwindef::LPVOID, dwBufferSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1037:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_0a000000"))] 
extern "system" {
    pub fn SetProtectedPolicy(PolicyGuid: ::guiddef::LPCGUID, PolicyValue: ::basetsd::ULONG_PTR, OldPolicyValue: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* processthreadsapi.h:1175:1 */
}
