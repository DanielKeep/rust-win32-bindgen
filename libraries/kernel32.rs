
extern "system" {
    pub fn InitializeSListHead(_: ::winnt::PSLIST_HEADER); /* interlockedapi.h:50:1 */
    pub fn InterlockedFlushSList(_: ::winnt::PSLIST_HEADER) -> ::winnt::PSLIST_ENTRY; /* interlockedapi.h:93:1 */
    pub fn InterlockedPopEntrySList(_: ::winnt::PSLIST_HEADER) -> ::winnt::PSLIST_ENTRY; /* interlockedapi.h:58:1 */
    pub fn InterlockedPushEntrySList(_: ::winnt::PSLIST_HEADER, _: ::winnt::PSLIST_ENTRY) -> ::winnt::PSLIST_ENTRY; /* interlockedapi.h:66:1 */
    pub fn QueryDepthSList(_: ::winnt::PSLIST_HEADER) -> ::minwindef::USHORT; /* interlockedapi.h:101:1 */
    pub fn QueryPerformanceCounter(_: *mut ::winnt::LARGE_INTEGER) -> ::minwindef::BOOL; /* profileapi.h:42:1 */
    pub fn QueryPerformanceFrequency(_: *mut ::winnt::LARGE_INTEGER) -> ::minwindef::BOOL; /* profileapi.h:50:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::lstrcmpW as ua_lstrcmpW; /* stralign.h:95:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::lstrcmpiW as ua_lstrcmpiW; /* stralign.h:94:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::lstrlenW as ua_lstrlenW; /* stralign.h:96:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn GetWriteWatch(_: ::minwindef::DWORD, _: ::winnt::PVOID, _: ::basetsd::SIZE_T, _: *mut *mut ::libc::c_void, _: *mut ::libc::c_ulonglong, _: ::minwindef::LPDWORD) -> ::minwindef::UINT; /* memoryapi.h:389:1 */
    pub fn ReadProcessMemory(_: ::winnt::HANDLE, _: ::minwindef::LPCVOID, _: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: *mut ::libc::c_ulonglong) -> ::minwindef::BOOL; /* memoryapi.h:217:1 */
    pub fn RtlAddFunctionTable(_: ::winnt::PRUNTIME_FUNCTION, _: ::minwindef::DWORD, _: ::basetsd::DWORD64) -> ::winnt::BOOLEAN; /* winnt.h:16974:1 */
    pub fn RtlInstallFunctionTableCallback(_: ::basetsd::DWORD64, _: ::basetsd::DWORD64, _: ::minwindef::DWORD, _: ::winnt::PGET_RUNTIME_FUNCTION_CALLBACK, _: ::winnt::PVOID, _: ::winnt::PCWSTR) -> ::winnt::BOOLEAN; /* winnt.h:16992:1 */
    pub fn RtlVirtualUnwind(_: ::minwindef::DWORD, _: ::basetsd::DWORD64, _: ::basetsd::DWORD64, _: ::winnt::PRUNTIME_FUNCTION, _: ::winnt::PCONTEXT, _: *mut *mut ::libc::c_void, _: ::basetsd::PDWORD64, _: ::winnt::PKNONVOLATILE_CONTEXT_POINTERS) -> ::winnt::PEXCEPTION_ROUTINE; /* winnt.h:17104:1 */
    pub fn WriteProcessMemory(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::minwindef::LPCVOID, _: ::basetsd::SIZE_T, _: *mut ::libc::c_ulonglong) -> ::minwindef::BOOL; /* memoryapi.h:230:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn RtlAddFunctionTable(_: ::winnt::PRUNTIME_FUNCTION, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winnt::BOOLEAN; /* winnt.h:17132:1 */
    pub fn RtlInstallFunctionTableCallback(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PGET_RUNTIME_FUNCTION_CALLBACK, _: ::winnt::PVOID, _: ::winnt::PCWSTR) -> ::winnt::BOOLEAN; /* winnt.h:17150:1 */
    pub fn RtlVirtualUnwind(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PRUNTIME_FUNCTION, _: ::winnt::PCONTEXT, _: *mut *mut ::libc::c_void, _: ::minwindef::PDWORD, _: ::winnt::PKNONVOLATILE_CONTEXT_POINTERS) -> ::winnt::PEXCEPTION_ROUTINE; /* winnt.h:17262:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] 
extern "system" {
    pub fn GetWriteWatch(_: ::minwindef::DWORD, _: ::winnt::PVOID, _: ::basetsd::SIZE_T, _: *mut *mut ::libc::c_void, _: *mut ::libc::c_ulong, _: ::minwindef::LPDWORD) -> ::minwindef::UINT; /* memoryapi.h:389:1 */
    pub fn ReadProcessMemory(_: ::winnt::HANDLE, _: ::minwindef::LPCVOID, _: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* memoryapi.h:217:1 */
    pub fn WriteProcessMemory(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::minwindef::LPCVOID, _: ::basetsd::SIZE_T, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* memoryapi.h:230:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64", target_arch="arm"))] 
extern "system" {
    pub fn RtlDeleteFunctionTable(_: ::winnt::PRUNTIME_FUNCTION) -> ::winnt::BOOLEAN; /* winnt.h:16984:1, winnt.h:17142:1 */
    pub fn RtlRestoreContext(_: ::winnt::PCONTEXT, _: *mut ::winnt::EXCEPTION_RECORD); /* winnt.h:17068:1, winnt.h:17226:1 */
    pub fn uaw_lstrcmpW(_: ::winnt::PCUWSTR, _: ::winnt::PCUWSTR) -> ::libc::c_int; /* stralign.h:159:1 */
    pub fn uaw_lstrcmpiW(_: ::winnt::PCUWSTR, _: ::winnt::PCUWSTR) -> ::libc::c_int; /* stralign.h:166:1 */
    pub fn uaw_lstrlenW(_: ::winnt::LPCUWSTR) -> ::libc::c_int; /* stralign.h:173:1 */
    pub fn uaw_wcschr(_: ::winnt::PCUWSTR, _: ::winnt::WCHAR) -> ::winnt::PUWSTR; /* stralign.h:179:1 */
    pub fn uaw_wcscpy(_: ::winnt::PUWSTR, _: ::winnt::PCUWSTR) -> ::winnt::PUWSTR; /* stralign.h:186:1 */
    pub fn uaw_wcsicmp(_: ::winnt::PCUWSTR, _: ::winnt::PCUWSTR) -> ::libc::c_int; /* stralign.h:193:1 */
    pub fn uaw_wcsrchr(_: ::winnt::PCUWSTR, _: ::winnt::WCHAR) -> ::winnt::PUWSTR; /* stralign.h:206:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AddAtomW as AddAtom; /* winbase.h:3933:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AddAtomA(_: ::winnt::LPCSTR) -> ::minwindef::ATOM; /* winbase.h:3923:1 */
    pub fn AddAtomW(_: ::winnt::LPCWSTR) -> ::minwindef::ATOM; /* winbase.h:3929:1 */
    pub fn AddDllDirectory(_: ::winnt::PCWSTR) -> ::libloaderapi::DLL_DIRECTORY_COOKIE; /* libloaderapi.h:498:1 */
    pub fn AddSIDToBoundaryDescriptor(_: *mut *mut ::libc::c_void, _: ::winnt::PSID) -> ::minwindef::BOOL; /* namespaceapi.h:82:1 */
    pub fn AllocConsole() -> ::minwindef::BOOL; /* consoleapi.h:44:1 */
    pub fn AreFileApisANSI() -> ::minwindef::BOOL; /* winbase.h:5820:1 */
    pub fn BackupRead(_: ::winnt::HANDLE, _: ::minwindef::LPBYTE, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::BOOL, _: ::minwindef::BOOL, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winbase.h:2862:1 */
    pub fn BackupSeek(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winbase.h:2875:1 */
    pub fn BackupWrite(_: ::winnt::HANDLE, _: ::minwindef::LPBYTE, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::BOOL, _: ::minwindef::BOOL, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winbase.h:2887:1 */
    pub fn Beep(_: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* utilapiset.h:85:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::BeginUpdateResourceW as BeginUpdateResource; /* winbase.h:3787:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn BeginUpdateResourceA(_: ::winnt::LPCSTR, _: ::minwindef::BOOL) -> ::winnt::HANDLE; /* winbase.h:3775:1 */
    pub fn BeginUpdateResourceW(_: ::winnt::LPCWSTR, _: ::minwindef::BOOL) -> ::winnt::HANDLE; /* winbase.h:3782:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::BuildCommDCBW as BuildCommDCB; /* winbase.h:6686:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn BuildCommDCBA(_: ::winnt::LPCSTR, _: ::winbase::LPDCB) -> ::minwindef::BOOL; /* winbase.h:6674:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::BuildCommDCBAndTimeoutsW as BuildCommDCBAndTimeouts; /* winbase.h:6708:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn BuildCommDCBAndTimeoutsA(_: ::winnt::LPCSTR, _: ::winbase::LPDCB, _: ::winbase::LPCOMMTIMEOUTS) -> ::minwindef::BOOL; /* winbase.h:6694:1 */
    pub fn BuildCommDCBAndTimeoutsW(_: ::winnt::LPCWSTR, _: ::winbase::LPDCB, _: ::winbase::LPCOMMTIMEOUTS) -> ::minwindef::BOOL; /* winbase.h:6702:1 */
    pub fn BuildCommDCBW(_: ::winnt::LPCWSTR, _: ::winbase::LPDCB) -> ::minwindef::BOOL; /* winbase.h:6681:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CallNamedPipeW as CallNamedPipe; /* winbase.h:5721:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CallNamedPipeA(_: ::winnt::LPCSTR, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5699:1 */
    pub fn CallNamedPipeW(_: ::winnt::LPCWSTR, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5711:1 */
    pub fn CancelDeviceWakeupRequest(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:1937:1 */
    pub fn CancelIo(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* ioapiset.h:178:1 */
    pub fn ChangeTimerQueueTimer(_: ::winnt::HANDLE, _: ::winnt::HANDLE, _: ::minwindef::ULONG, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* threadpoollegacyapiset.h:91:1 */
    pub fn ClearCommBreak(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:2004:1 */
    pub fn ClearCommError(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::winbase::LPCOMSTAT) -> ::minwindef::BOOL; /* winbase.h:2011:1 */
    pub fn ClosePrivateNamespace(_: ::winnt::HANDLE, _: ::minwindef::ULONG) -> ::winnt::BOOLEAN; /* namespaceapi.h:64:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CommConfigDialogW as CommConfigDialog; /* winbase.h:6730:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CommConfigDialogA(_: ::winnt::LPCSTR, _: ::windef::HWND, _: ::winbase::LPCOMMCONFIG) -> ::minwindef::BOOL; /* winbase.h:6716:1 */
    pub fn CommConfigDialogW(_: ::winnt::LPCWSTR, _: ::windef::HWND, _: ::winbase::LPCOMMCONFIG) -> ::minwindef::BOOL; /* winbase.h:6724:1 */
    pub fn CompareFileTime(_: *const ::minwindef::FILETIME, _: *const ::minwindef::FILETIME) -> ::winnt::LONG; /* fileapi.h:77:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CompareStringW as CompareString; /* stringapiset.h:93:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CompareStringA(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::PCNZCH, _: ::libc::c_int, _: ::winnt::PCNZCH, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1414:1 */
    pub fn CompareStringW(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::PCNZWCH, _: ::libc::c_int, _: ::winnt::PCNZWCH, _: ::libc::c_int) -> ::libc::c_int; /* stringapiset.h:83:1 */
    pub fn ConnectNamedPipe(_: ::winnt::HANDLE, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* namedpipeapi.h:61:1 */
    pub fn ContinueDebugEvent(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:117:1 */
    pub fn ConvertDefaultLocale(_: ::winnt::LCID) -> ::winnt::LCID; /* winnls.h:1957:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CopyFileW as CopyFile; /* winbase.h:5054:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CopyFileA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:5040:1 */
    pub fn CopyFileW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:5048:1 */
    pub fn CreateBoundaryDescriptorW(_: ::winnt::LPCWSTR, _: ::minwindef::ULONG) -> ::winnt::HANDLE; /* namespaceapi.h:73:1 */
    pub fn CreateConsoleScreenBuffer(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *const ::minwinbase::SECURITY_ATTRIBUTES, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::winnt::HANDLE; /* wincon.h:826:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateDirectoryExW as CreateDirectoryEx; /* winbase.h:4665:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateDirectoryExA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::BOOL; /* winbase.h:4651:1 */
    pub fn CreateDirectoryExW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::BOOL; /* winbase.h:4659:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateEventW as CreateEvent; /* synchapi.h:609:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateEventA(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::BOOL, _: ::minwindef::BOOL, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* synchapi.h:590:1 */
    pub fn CreateEventW(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::BOOL, _: ::minwindef::BOOL, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* synchapi.h:601:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateFileW as CreateFile; /* fileapi.h:146:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateFileA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::winnt::HANDLE; /* fileapi.h:122:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateFileMappingW as CreateFileMapping; /* memoryapi.h:254:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateFileMappingA(_: ::winnt::HANDLE, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:3116:1 */
    pub fn CreateFileMappingW(_: ::winnt::HANDLE, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* memoryapi.h:243:1 */
    pub fn CreateFileW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::winnt::HANDLE; /* fileapi.h:135:1 */
    pub fn CreateIoCompletionPort(_: ::winnt::HANDLE, _: ::winnt::HANDLE, _: ::basetsd::ULONG_PTR, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* ioapiset.h:64:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateMailslotW as CreateMailslot; /* winbase.h:2439:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateMailslotA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::winnt::HANDLE; /* winbase.h:2423:1 */
    pub fn CreateMailslotW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::winnt::HANDLE; /* winbase.h:2432:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateMutexW as CreateMutex; /* synchapi.h:552:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateMutexA(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::BOOL, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* synchapi.h:535:1 */
    pub fn CreateMutexW(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::BOOL, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* synchapi.h:545:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateNamedPipeW as CreateNamedPipe; /* namedpipeapi.h:129:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateNamedPipeA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::winnt::HANDLE; /* winbase.h:5652:1 */
    pub fn CreateNamedPipeW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::winnt::HANDLE; /* namedpipeapi.h:116:1 */
    pub fn CreatePipe(_: ::winnt::PHANDLE, _: ::winnt::PHANDLE, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:50:1 */
    pub fn CreatePrivateNamespaceW(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::LPVOID, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* namespaceapi.h:45:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateProcessW as CreateProcess; /* processthreadsapi.h:532:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateProcessA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::BOOL, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::winnt::LPCSTR, _: ::processthreadsapi::LPSTARTUPINFOA, _: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* processthreadsapi.h:502:1 */
    pub fn CreateProcessW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::BOOL, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::winnt::LPCWSTR, _: ::processthreadsapi::LPSTARTUPINFOW, _: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* processthreadsapi.h:518:1 */
    pub fn CreateRemoteThread(_: ::winnt::HANDLE, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::basetsd::SIZE_T, _: ::minwinbase::LPTHREAD_START_ROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:228:1 */
    pub fn CreateRemoteThreadEx(_: ::winnt::HANDLE, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::basetsd::SIZE_T, _: ::minwinbase::LPTHREAD_START_ROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::processthreadsapi::LPPROC_THREAD_ATTRIBUTE_LIST, _: ::minwindef::LPDWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:815:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateSemaphoreW as CreateSemaphore; /* winbase.h:3019:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateSemaphoreA(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::LONG, _: ::winnt::LONG, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:3002:1 */
    pub fn CreateSemaphoreW(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::LONG, _: ::winnt::LONG, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winbase.h:3012:1 */
    pub fn CreateTapePartition(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:2194:1 */
    pub fn CreateTimerQueue() -> ::winnt::HANDLE; /* threadpoollegacyapiset.h:68:1 */
    pub fn CreateTimerQueueTimer(_: ::winnt::PHANDLE, _: ::winnt::HANDLE, _: ::winnt::WAITORTIMERCALLBACK, _: ::winnt::PVOID, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* threadpoollegacyapiset.h:76:1 */
    pub fn DebugActiveProcess(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:136:1 */
    pub fn DebugActiveProcessStop(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:144:1 */
    pub fn DebugBreak(); /* debugapi.h:70:1 */
    pub fn DebugBreakProcess(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:1862:1 */
    pub fn DebugSetProcessKillOnExit(_: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:1855:1 */
    pub fn DecodeSystemPointer(_: ::winnt::PVOID) -> ::winnt::PVOID; /* utilapiset.h:77:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DefineDosDeviceW as DefineDosDevice; /* fileapi.h:162:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DefineDosDeviceA(_: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4755:1 */
    pub fn DefineDosDeviceW(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* fileapi.h:154:1 */
    pub fn DeleteAtom(_: ::minwindef::ATOM) -> ::minwindef::ATOM; /* winbase.h:1916:1 */
    pub fn DeleteBoundaryDescriptor(_: ::winnt::HANDLE); /* namespaceapi.h:91:1 */
    pub fn DeleteSynchronizationBarrier(_: ::synchapi::LPSYNCHRONIZATION_BARRIER) -> ::minwindef::BOOL; /* synchapi.h:893:1 */
    pub fn DeleteTimerQueueEx(_: ::winnt::HANDLE, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* threadpoollegacyapiset.h:114:1 */
    pub fn DeleteTimerQueueTimer(_: ::winnt::HANDLE, _: ::winnt::HANDLE, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* threadpoollegacyapiset.h:103:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DeleteVolumeMountPointW as DeleteVolumeMountPoint; /* fileapi.h:208:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DeleteVolumeMountPointW(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* fileapi.h:202:1 */
    pub fn DeviceIoControl(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* ioapiset.h:116:1 */
    pub fn DisconnectNamedPipe(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* namedpipeapi.h:70:1 */
    pub fn DnsHostnameToComputerNameExW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:602:1 */
    pub fn DosDateTimeToFileTime(_: ::minwindef::WORD, _: ::minwindef::WORD, _: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* winbase.h:2311:1 */
    pub fn EncodeSystemPointer(_: ::winnt::PVOID) -> ::winnt::PVOID; /* utilapiset.h:68:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EndUpdateResourceW as EndUpdateResource; /* winbase.h:3835:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EndUpdateResourceA(_: ::winnt::HANDLE, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:3823:1 */
    pub fn EndUpdateResourceW(_: ::winnt::HANDLE, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:3830:1 */
    pub fn EnterSynchronizationBarrier(_: ::synchapi::LPSYNCHRONIZATION_BARRIER, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:876:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumCalendarInfoW as EnumCalendarInfo; /* winnls.h:1763:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumCalendarInfoA(_: ::winnls::CALINFO_ENUMPROCA, _: ::winnt::LCID, _: ::winnls::CALID, _: ::winnls::CALTYPE) -> ::minwindef::BOOL; /* winnls.h:1748:1 */
    pub fn EnumCalendarInfoW(_: ::winnls::CALINFO_ENUMPROCW, _: ::winnt::LCID, _: ::winnls::CALID, _: ::winnls::CALTYPE) -> ::minwindef::BOOL; /* winnls.h:1757:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumDateFormatsW as EnumDateFormats; /* winnls.h:1833:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumDateFormatsA(_: ::winnls::DATEFMT_ENUMPROCA, _: ::winnt::LCID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1820:1 */
    pub fn EnumDateFormatsW(_: ::winnls::DATEFMT_ENUMPROCW, _: ::winnt::LCID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1828:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumResourceLanguagesW as EnumResourceLanguages; /* winbase.h:3767:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumResourceLanguagesA(_: ::minwindef::HMODULE, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::libloaderapi::ENUMRESLANGPROCA, _: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winbase.h:3749:1 */
    pub fn EnumResourceLanguagesW(_: ::minwindef::HMODULE, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::libloaderapi::ENUMRESLANGPROCW, _: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winbase.h:3759:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumResourceNamesW as EnumResourceNames; /* winbase.h:3741:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumResourceNamesA(_: ::minwindef::HMODULE, _: ::winnt::LPCSTR, _: ::libloaderapi::ENUMRESNAMEPROCA, _: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winbase.h:3725:1 */
    pub fn EnumResourceNamesW(_: ::minwindef::HMODULE, _: ::winnt::LPCWSTR, _: ::libloaderapi::ENUMRESNAMEPROCW, _: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winbase.h:3734:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumResourceTypesW as EnumResourceTypes; /* winbase.h:3717:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumResourceTypesA(_: ::minwindef::HMODULE, _: ::libloaderapi::ENUMRESTYPEPROCA, _: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winbase.h:3703:1 */
    pub fn EnumResourceTypesW(_: ::minwindef::HMODULE, _: ::libloaderapi::ENUMRESTYPEPROCW, _: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winbase.h:3711:1 */
    pub fn EnumSystemFirmwareTables(_: ::minwindef::DWORD, _: ::winnt::PVOID, _: ::minwindef::DWORD) -> ::minwindef::UINT; /* sysinfoapi.h:565:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumTimeFormatsW as EnumTimeFormats; /* winnls.h:1811:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumTimeFormatsA(_: ::winnls::TIMEFMT_ENUMPROCA, _: ::winnt::LCID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1798:1 */
    pub fn EnumTimeFormatsW(_: ::winnls::TIMEFMT_ENUMPROCW, _: ::winnt::LCID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1806:1 */
    pub fn EraseTape(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winbase.h:2185:1 */
    pub fn EscapeCommFunction(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2029:1 */
    pub fn ExitProcess(_: ::minwindef::UINT); /* processthreadsapi.h:169:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ExpandEnvironmentStringsW as ExpandEnvironmentStrings; /* processenv.h:248:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ExpandEnvironmentStringsA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processenv.h:231:1 */
    pub fn ExpandEnvironmentStringsW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processenv.h:241:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FatalAppExitW as FatalAppExit; /* winbase.h:3466:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FatalAppExitA(_: ::minwindef::UINT, _: ::winnt::LPCSTR); /* winbase.h:3454:1 */
    pub fn FatalAppExitW(_: ::minwindef::UINT, _: ::winnt::LPCWSTR); /* winbase.h:3461:1 */
    pub fn FatalExit(_: ::libc::c_int); /* winbase.h:1254:1 */
    pub fn FileTimeToDosDateTime(_: *const ::minwindef::FILETIME, _: ::minwindef::LPWORD, _: ::minwindef::LPWORD) -> ::minwindef::BOOL; /* winbase.h:2302:1 */
    pub fn FileTimeToLocalFileTime(_: *const ::minwindef::FILETIME, _: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* fileapi.h:214:1 */
    pub fn FillConsoleOutputAttribute(_: ::winnt::HANDLE, _: ::minwindef::WORD, _: ::minwindef::DWORD, _: ::wincon::COORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:522:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FillConsoleOutputCharacterW as FillConsoleOutputCharacter; /* wincon.h:514:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FillConsoleOutputCharacterA(_: ::winnt::HANDLE, _: ::winnt::CHAR, _: ::minwindef::DWORD, _: ::wincon::COORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:496:1 */
    pub fn FillConsoleOutputCharacterW(_: ::winnt::HANDLE, _: ::winnt::WCHAR, _: ::minwindef::DWORD, _: ::wincon::COORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:506:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindAtomW as FindAtom; /* winbase.h:3951:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindAtomA(_: ::winnt::LPCSTR) -> ::minwindef::ATOM; /* winbase.h:3941:1 */
    pub fn FindAtomW(_: ::winnt::LPCWSTR) -> ::minwindef::ATOM; /* winbase.h:3947:1 */
    pub fn FindCloseChangeNotification(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* fileapi.h:248:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindFirstChangeNotificationW as FindFirstChangeNotification; /* fileapi.h:272:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindFirstChangeNotificationA(_: ::winnt::LPCSTR, _: ::minwindef::BOOL, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* fileapi.h:256:1 */
    pub fn FindFirstChangeNotificationW(_: ::winnt::LPCWSTR, _: ::minwindef::BOOL, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* fileapi.h:265:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindFirstFileW as FindFirstFile; /* fileapi.h:294:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindFirstFileA(_: ::winnt::LPCSTR, _: ::minwinbase::LPWIN32_FIND_DATAA) -> ::winnt::HANDLE; /* fileapi.h:280:1 */
    pub fn FindFirstFileW(_: ::winnt::LPCWSTR, _: ::minwinbase::LPWIN32_FIND_DATAW) -> ::winnt::HANDLE; /* fileapi.h:288:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindFirstVolumeW as FindFirstVolume; /* fileapi.h:358:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindFirstVolumeW(_: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* fileapi.h:351:1 */
    pub fn FindNextChangeNotification(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* fileapi.h:364:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindNextVolumeW as FindNextVolume; /* fileapi.h:416:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindNextVolumeW(_: ::winnt::HANDLE, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:408:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindResourceW as FindResource; /* winbase.h:3681:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindResourceA(_: ::minwindef::HMODULE, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::HRSRC; /* winbase.h:3666:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindResourceExW as FindResourceEx; /* libloaderapi.h:182:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindResourceExA(_: ::minwindef::HMODULE, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::WORD) -> ::minwindef::HRSRC; /* winbase.h:3690:1 */
    pub fn FindResourceExW(_: ::minwindef::HMODULE, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::WORD) -> ::minwindef::HRSRC; /* libloaderapi.h:173:1 */
    pub fn FindResourceW(_: ::minwindef::HMODULE, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::HRSRC; /* winbase.h:3675:1 */
    pub fn FindVolumeClose(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* fileapi.h:422:1 */
    pub fn FlushConsoleInputBuffer(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* wincon.h:645:1 */
    pub fn FlushInstructionCache(_: ::winnt::HANDLE, _: ::minwindef::LPCVOID, _: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:916:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FoldStringW as FoldString; /* stringapiset.h:108:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FoldStringA(_: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::libc::c_int, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2170:1 */
    pub fn FoldStringW(_: ::minwindef::DWORD, _: ::winnt::LPCWCH, _: ::libc::c_int, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* stringapiset.h:99:1 */
    pub fn FreeConsole() -> ::minwindef::BOOL; /* wincon.h:726:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FreeEnvironmentStringsW as FreeEnvironmentStrings; /* processenv.h:100:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FreeEnvironmentStringsA(_: ::winnt::LPCH) -> ::minwindef::BOOL; /* processenv.h:88:1 */
    pub fn FreeEnvironmentStringsW(_: ::winnt::LPWCH) -> ::minwindef::BOOL; /* processenv.h:95:1 */
    pub fn FreeLibraryAndExitThread(_: ::minwindef::HMODULE, _: ::minwindef::DWORD); /* libloaderapi.h:229:1 */
    pub fn FreeResource(_: ::minwindef::HGLOBAL) -> ::minwindef::BOOL; /* libloaderapi.h:238:1 */
    pub fn GenerateConsoleCtrlEvent(_: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wincon.h:719:1 */
    pub fn GetACP() -> ::minwindef::UINT; /* winnls.h:1360:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetAtomNameW as GetAtomName; /* winbase.h:3973:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetAtomNameA(_: ::minwindef::ATOM, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::minwindef::UINT; /* winbase.h:3959:1 */
    pub fn GetAtomNameW(_: ::minwindef::ATOM, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::minwindef::UINT; /* winbase.h:3967:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetBinaryTypeW as GetBinaryType; /* winbase.h:1159:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetBinaryTypeA(_: ::winnt::LPCSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:1147:1 */
    pub fn GetBinaryTypeW(_: ::winnt::LPCWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:1154:1 */
    pub fn GetCommConfig(_: ::winnt::HANDLE, _: ::winbase::LPCOMMCONFIG, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2038:1 */
    pub fn GetCommMask(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2047:1 */
    pub fn GetCommModemStatus(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2063:1 */
    pub fn GetCommProperties(_: ::winnt::HANDLE, _: ::winbase::LPCOMMPROP) -> ::minwindef::BOOL; /* winbase.h:2055:1 */
    pub fn GetCommState(_: ::winnt::HANDLE, _: ::winbase::LPDCB) -> ::minwindef::BOOL; /* winbase.h:2071:1 */
    pub fn GetCommTimeouts(_: ::winnt::HANDLE, _: ::winbase::LPCOMMTIMEOUTS) -> ::minwindef::BOOL; /* winbase.h:2079:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetComputerNameW as GetComputerName; /* winbase.h:6802:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetComputerNameA(_: ::winnt::LPSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6789:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetComputerNameExW as GetComputerNameEx; /* sysinfoapi.h:377:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetComputerNameExA(_: ::sysinfoapi::COMPUTER_NAME_FORMAT, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:360:1 */
    pub fn GetComputerNameExW(_: ::sysinfoapi::COMPUTER_NAME_FORMAT, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:370:1 */
    pub fn GetComputerNameW(_: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6797:1 */
    pub fn GetConsoleCP() -> ::minwindef::UINT; /* consoleapi.h:52:1 */
    pub fn GetConsoleCursorInfo(_: ::winnt::HANDLE, _: ::wincon::PCONSOLE_CURSOR_INFO) -> ::minwindef::BOOL; /* wincon.h:565:1 */
    pub fn GetConsoleMode(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* consoleapi.h:60:1 */
    pub fn GetConsoleOutputCP() -> ::minwindef::UINT; /* consoleapi.h:69:1 */
    pub fn GetConsoleScreenBufferInfo(_: ::winnt::HANDLE, _: ::wincon::PCONSOLE_SCREEN_BUFFER_INFO) -> ::minwindef::BOOL; /* wincon.h:536:1 */
    pub fn GetConsoleScreenBufferInfoEx(_: ::winnt::HANDLE, _: ::wincon::PCONSOLE_SCREEN_BUFFER_INFOEX) -> ::minwindef::BOOL; /* wincon.h:544:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetConsoleTitleW as GetConsoleTitle; /* wincon.h:755:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetConsoleTitleA(_: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:743:1 */
    pub fn GetConsoleTitleW(_: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:750:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetCurrencyFormatW as GetCurrencyFormat; /* winnls.h:1739:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetCurrencyFormatA(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: *const ::winnls::CURRENCYFMTA, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1720:1 */
    pub fn GetCurrencyFormatW(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: *const ::winnls::CURRENCYFMTW, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1731:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetCurrentDirectoryW as GetCurrentDirectory; /* processenv.h:292:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetCurrentDirectoryA(_: ::minwindef::DWORD, _: ::winnt::LPSTR) -> ::minwindef::DWORD; /* processenv.h:277:1 */
    pub fn GetCurrentDirectoryW(_: ::minwindef::DWORD, _: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* processenv.h:286:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetDateFormatW as GetDateFormat; /* datetimeapi.h:76:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetDateFormatA(_: ::winnt::LCID, _: ::minwindef::DWORD, _: *const ::minwinbase::SYSTEMTIME, _: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* datetimeapi.h:53:1 */
    pub fn GetDateFormatW(_: ::winnt::LCID, _: ::minwindef::DWORD, _: *const ::minwinbase::SYSTEMTIME, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* datetimeapi.h:66:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetDefaultCommConfigW as GetDefaultCommConfig; /* winbase.h:6752:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetDefaultCommConfigA(_: ::winnt::LPCSTR, _: ::winbase::LPCOMMCONFIG, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6738:1 */
    pub fn GetDefaultCommConfigW(_: ::winnt::LPCWSTR, _: ::winbase::LPCOMMCONFIG, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6746:1 */
    pub fn GetDevicePowerState(_: ::winnt::HANDLE, _: *mut ::libc::c_int) -> ::minwindef::BOOL; /* winbase.h:1944:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetDiskFreeSpaceW as GetDiskFreeSpace; /* fileapi.h:472:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetDiskFreeSpaceA(_: ::winnt::LPCSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* fileapi.h:452:1 */
    pub fn GetDiskFreeSpaceW(_: ::winnt::LPCWSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* fileapi.h:463:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetDriveTypeW as GetDriveType; /* fileapi.h:532:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetDriveTypeA(_: ::winnt::LPCSTR) -> ::minwindef::UINT; /* fileapi.h:520:1 */
    pub fn GetDriveTypeW(_: ::winnt::LPCWSTR) -> ::minwindef::UINT; /* fileapi.h:527:1 */
    pub fn GetEnvironmentStrings() -> ::winnt::LPCH; /* processenv.h:54:1 */
    pub fn GetEnvironmentStringsW() -> ::winnt::LPWCH; /* processenv.h:63:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetEnvironmentVariableW as GetEnvironmentVariable; /* processenv.h:200:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetEnvironmentVariableA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processenv.h:183:1 */
    pub fn GetEnvironmentVariableW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processenv.h:193:1 */
    pub fn GetExitCodeProcess(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:186:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileAttributesW as GetFileAttributes; /* fileapi.h:552:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileAttributesA(_: ::winnt::LPCSTR) -> ::minwindef::DWORD; /* fileapi.h:540:1 */
    pub fn GetFileAttributesW(_: ::winnt::LPCWSTR) -> ::minwindef::DWORD; /* fileapi.h:547:1 */
    pub fn GetFileInformationByHandle(_: ::winnt::HANDLE, _: ::fileapi::LPBY_HANDLE_FILE_INFORMATION) -> ::minwindef::BOOL; /* fileapi.h:627:1 */
    pub fn GetFileSize(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* fileapi.h:636:1 */
    pub fn GetFileSizeEx(_: ::winnt::HANDLE, _: ::winnt::PLARGE_INTEGER) -> ::minwindef::BOOL; /* fileapi.h:645:1 */
    pub fn GetFileTime(_: ::winnt::HANDLE, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* fileapi.h:654:1 */
    pub fn GetFileType(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* fileapi.h:665:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFirmwareEnvironmentVariableW as GetFirmwareEnvironmentVariable; /* winbase.h:3554:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFirmwareEnvironmentVariableA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::PVOID, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:3538:1 */
    pub fn GetFirmwareEnvironmentVariableW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::PVOID, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:3547:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFullPathNameW as GetFullPathName; /* fileapi.h:724:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFullPathNameA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: *mut *mut ::libc::c_schar) -> ::minwindef::DWORD; /* fileapi.h:705:1 */
    pub fn GetFullPathNameW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: *mut *mut ::libc::c_ushort) -> ::minwindef::DWORD; /* fileapi.h:716:1 */
    pub fn GetHandleInformation(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* handleapi.h:79:1 */
    pub fn GetLargePageMinimum() -> ::basetsd::SIZE_T; /* memoryapi.h:339:1 */
    pub fn GetLargestConsoleWindowSize(_: ::winnt::HANDLE) -> ::wincon::COORD; /* wincon.h:558:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetLocaleInfoW as GetLocaleInfo; /* winnls.h:1512:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetLocaleInfoA(_: ::winnt::LCID, _: ::winnls::LCTYPE, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1518:1 */
    pub fn GetLocaleInfoW(_: ::winnt::LCID, _: ::winnls::LCTYPE, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1505:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetLogicalDriveStringsW as GetLogicalDriveStrings; /* fileapi.h:747:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetLogicalDriveStringsA(_: ::minwindef::DWORD, _: ::winnt::LPSTR) -> ::minwindef::DWORD; /* winbase.h:3166:1 */
    pub fn GetLogicalDriveStringsW(_: ::minwindef::DWORD, _: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* fileapi.h:740:1 */
    pub fn GetLogicalDrives() -> ::minwindef::DWORD; /* fileapi.h:732:1 */
    pub fn GetLogicalProcessorInformation(_: ::winnt::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:455:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetLongPathNameW as GetLongPathName; /* fileapi.h:771:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetLongPathNameA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:754:1 */
    pub fn GetLongPathNameW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:764:1 */
    pub fn GetMailslotInfo(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2447:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetModuleFileNameW as GetModuleFileName; /* libloaderapi.h:266:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetModuleFileNameA(_: ::minwindef::HMODULE, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* libloaderapi.h:248:1 */
    pub fn GetModuleFileNameW(_: ::minwindef::HMODULE, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* libloaderapi.h:259:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetModuleHandleW as GetModuleHandle; /* libloaderapi.h:290:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetModuleHandleA(_: ::winnt::LPCSTR) -> ::minwindef::HMODULE; /* libloaderapi.h:276:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetModuleHandleExW as GetModuleHandleEx; /* libloaderapi.h:343:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetModuleHandleExA(_: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: *mut *mut ::minwindef::HINSTANCE__) -> ::minwindef::BOOL; /* libloaderapi.h:327:1 */
    pub fn GetModuleHandleExW(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: *mut *mut ::minwindef::HINSTANCE__) -> ::minwindef::BOOL; /* libloaderapi.h:336:1 */
    pub fn GetModuleHandleW(_: ::winnt::LPCWSTR) -> ::minwindef::HMODULE; /* libloaderapi.h:285:1 */
    pub fn GetNLSVersion(_: ::winnls::NLS_FUNCTION, _: ::winnt::LCID, _: ::winnls::LPNLSVERSIONINFO) -> ::minwindef::BOOL; /* winnls.h:1875:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetNamedPipeClientComputerNameW as GetNamedPipeClientComputerName; /* namedpipeapi.h:161:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetNamedPipeHandleStateW as GetNamedPipeHandleState; /* winbase.h:5691:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetNamedPipeHandleStateA(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5669:1 */
    pub fn GetNamedPipeHandleStateW(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5681:1 */
    pub fn GetNamedPipeInfo(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2412:1 */
    pub fn GetNumaAvailableMemoryNode(_: ::minwindef::UCHAR, _: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* winbase.h:8044:1 */
    pub fn GetNumaHighestNodeNumber(_: ::minwindef::PULONG) -> ::minwindef::BOOL; /* systemtopologyapi.h:43:1 */
    pub fn GetNumaNodeProcessorMask(_: ::minwindef::UCHAR, _: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* winbase.h:8036:1 */
    pub fn GetNumaProcessorNode(_: ::minwindef::UCHAR, _: ::minwindef::PUCHAR) -> ::minwindef::BOOL; /* winbase.h:8004:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetNumberFormatW as GetNumberFormat; /* winnls.h:1711:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetNumberFormatA(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: *const ::winnls::NUMBERFMTA, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1692:1 */
    pub fn GetNumberFormatW(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: *const ::winnls::NUMBERFMTW, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1703:1 */
    pub fn GetNumberOfConsoleInputEvents(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* consoleapi.h:77:1 */
    pub fn GetNumberOfConsoleMouseButtons(_: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:631:1 */
    pub fn GetOEMCP() -> ::minwindef::UINT; /* winnls.h:1365:1 */
    pub fn GetOverlappedResult(_: ::winnt::HANDLE, _: ::minwinbase::LPOVERLAPPED, _: ::minwindef::LPDWORD, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* ioapiset.h:52:1 */
    pub fn GetPhysicallyInstalledSystemMemory(_: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* sysinfoapi.h:613:1 */
    pub fn GetPriorityClass(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:669:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetPrivateProfileIntW as GetPrivateProfileInt; /* winbase.h:4109:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetPrivateProfileIntA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::INT, _: ::winnt::LPCSTR) -> ::minwindef::UINT; /* winbase.h:4093:1 */
    pub fn GetPrivateProfileIntW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::INT, _: ::winnt::LPCWSTR) -> ::minwindef::UINT; /* winbase.h:4102:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetPrivateProfileSectionW as GetPrivateProfileSection; /* winbase.h:4237:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetPrivateProfileSectionA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::winnt::LPCSTR) -> ::minwindef::DWORD; /* winbase.h:4221:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetPrivateProfileSectionNamesW as GetPrivateProfileSectionNames; /* winbase.h:4306:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetPrivateProfileSectionNamesA(_: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::winnt::LPCSTR) -> ::minwindef::DWORD; /* winbase.h:4292:1 */
    pub fn GetPrivateProfileSectionNamesW(_: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR) -> ::minwindef::DWORD; /* winbase.h:4300:1 */
    pub fn GetPrivateProfileSectionW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR) -> ::minwindef::DWORD; /* winbase.h:4230:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetPrivateProfileStringW as GetPrivateProfileString; /* winbase.h:4161:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetPrivateProfileStringA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::winnt::LPCSTR) -> ::minwindef::DWORD; /* winbase.h:4141:1 */
    pub fn GetPrivateProfileStringW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR) -> ::minwindef::DWORD; /* winbase.h:4152:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetPrivateProfileStructW as GetPrivateProfileStruct; /* winbase.h:4354:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetPrivateProfileStructA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::LPVOID, _: ::minwindef::UINT, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4336:1 */
    pub fn GetPrivateProfileStructW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::LPVOID, _: ::minwindef::UINT, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:4346:1 */
    pub fn GetProcessAffinityMask(_: ::winnt::HANDLE, _: ::basetsd::PDWORD_PTR, _: ::basetsd::PDWORD_PTR) -> ::minwindef::BOOL; /* winbase.h:1210:1 */
    pub fn GetProcessHeaps(_: ::minwindef::DWORD, _: ::winnt::PHANDLE) -> ::minwindef::DWORD; /* heapapi.h:204:1 */
    pub fn GetProcessIoCounters(_: ::winnt::HANDLE, _: ::winnt::PIO_COUNTERS) -> ::minwindef::BOOL; /* winbase.h:1227:1 */
    pub fn GetProcessShutdownParameters(_: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:3446:1 */
    pub fn GetProcessTimes(_: ::winnt::HANDLE, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* processthreadsapi.h:126:1 */
    pub fn GetProcessVersion(_: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processthreadsapi.h:551:1 */
    pub fn GetProcessWorkingSetSize(_: ::winnt::HANDLE, _: ::basetsd::PSIZE_T, _: ::basetsd::PSIZE_T) -> ::minwindef::BOOL; /* winbase.h:1235:1 */
    pub fn GetProcessWorkingSetSizeEx(_: ::winnt::HANDLE, _: ::basetsd::PSIZE_T, _: ::basetsd::PSIZE_T, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:348:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetProfileIntW as GetProfileInt; /* winbase.h:3995:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetProfileIntA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::INT) -> ::minwindef::UINT; /* winbase.h:3981:1 */
    pub fn GetProfileIntW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::INT) -> ::minwindef::UINT; /* winbase.h:3989:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetProfileSectionW as GetProfileSection; /* winbase.h:4065:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetProfileSectionA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:4051:1 */
    pub fn GetProfileSectionW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:4059:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetProfileStringW as GetProfileString; /* winbase.h:4021:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetProfileStringA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:4003:1 */
    pub fn GetProfileStringW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:4013:1 */
    pub fn GetQueuedCompletionStatus(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::basetsd::PULONG_PTR, _: *mut *mut ::minwinbase::OVERLAPPED, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* ioapiset.h:75:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetShortPathNameW as GetShortPathName; /* fileapi.h:788:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetShortPathNameA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:1168:1 */
    pub fn GetShortPathNameW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:780:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetStartupInfoW as GetStartupInfo; /* processthreadsapi.h:564:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetStartupInfoA(_: ::processthreadsapi::LPSTARTUPINFOA); /* winbase.h:3474:1 */
    pub fn GetStartupInfoW(_: ::processthreadsapi::LPSTARTUPINFOW); /* processthreadsapi.h:559:1 */
    pub fn GetStdHandle(_: ::minwindef::DWORD) -> ::winnt::HANDLE; /* processenv.h:108:1 */
    pub fn GetStringTypeA(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::libc::c_int, _: ::minwindef::LPWORD) -> ::minwindef::BOOL; /* winnls.h:2160:1 */
    pub fn GetStringTypeExA(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::libc::c_int, _: ::minwindef::LPWORD) -> ::minwindef::BOOL; /* winnls.h:2136:1 */
    pub fn GetSystemDefaultLCID() -> ::winnt::LCID; /* winnls.h:1997:1 */
    pub fn GetSystemDefaultLangID() -> ::winnt::LANGID; /* winnls.h:1987:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetSystemDirectoryW as GetSystemDirectory; /* sysinfoapi.h:265:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetSystemDirectoryA(_: ::winnt::LPSTR, _: ::minwindef::UINT) -> ::minwindef::UINT; /* sysinfoapi.h:250:1 */
    pub fn GetSystemDirectoryW(_: ::winnt::LPWSTR, _: ::minwindef::UINT) -> ::minwindef::UINT; /* sysinfoapi.h:259:1 */
    pub fn GetSystemFirmwareTable(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PVOID, _: ::minwindef::DWORD) -> ::minwindef::UINT; /* sysinfoapi.h:575:1 */
    pub fn GetSystemInfo(_: ::sysinfoapi::LPSYSTEM_INFO); /* sysinfoapi.h:184:1 */
    pub fn GetSystemTimeAdjustment(_: ::minwindef::PDWORD, _: ::minwindef::PDWORD, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* sysinfoapi.h:239:1 */
    pub fn GetSystemTimePreciseAsFileTime(_: ::minwindef::LPFILETIME); /* sysinfoapi.h:557:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetSystemWindowsDirectoryW as GetSystemWindowsDirectory; /* sysinfoapi.h:315:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetSystemWindowsDirectoryA(_: ::winnt::LPSTR, _: ::minwindef::UINT) -> ::minwindef::UINT; /* sysinfoapi.h:300:1 */
    pub fn GetSystemWindowsDirectoryW(_: ::winnt::LPWSTR, _: ::minwindef::UINT) -> ::minwindef::UINT; /* sysinfoapi.h:309:1 */
    pub fn GetTapeParameters(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPVOID) -> ::minwindef::DWORD; /* winbase.h:2221:1 */
    pub fn GetTapePosition(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winbase.h:2165:1 */
    pub fn GetTapeStatus(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:2214:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTempFileNameW as GetTempFileName; /* fileapi.h:803:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTempFileNameA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::UINT, _: ::winnt::LPSTR) -> ::minwindef::UINT; /* winbase.h:4425:1 */
    pub fn GetTempFileNameW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::UINT, _: ::winnt::LPWSTR) -> ::minwindef::UINT; /* fileapi.h:794:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTempPathW as GetTempPath; /* fileapi.h:1213:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTempPathA(_: ::minwindef::DWORD, _: ::winnt::LPSTR) -> ::minwindef::DWORD; /* winbase.h:4414:1 */
    pub fn GetTempPathW(_: ::minwindef::DWORD, _: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* fileapi.h:1206:1 */
    pub fn GetThreadContext(_: ::winnt::HANDLE, _: ::minwinbase::LPCONTEXT) -> ::minwindef::BOOL; /* processthreadsapi.h:859:1 */
    pub fn GetThreadErrorMode() -> ::minwindef::DWORD; /* winbase.h:1792:1 */
    pub fn GetThreadLocale() -> ::winnt::LCID; /* winnls.h:1963:1 */
    pub fn GetThreadPriorityBoost(_: ::winnt::HANDLE, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:312:1 */
    pub fn GetThreadSelectorEntry(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winbase::LPLDT_ENTRY) -> ::minwindef::BOOL; /* winbase.h:1669:1 */
    pub fn GetThreadTimes(_: ::winnt::HANDLE, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* processthreadsapi.h:926:1 */
    pub fn GetTickCount() -> ::minwindef::DWORD; /* sysinfoapi.h:203:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTimeFormatW as GetTimeFormat; /* datetimeapi.h:109:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTimeFormatA(_: ::winnt::LCID, _: ::minwindef::DWORD, _: *const ::minwinbase::SYSTEMTIME, _: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* datetimeapi.h:86:1 */
    pub fn GetTimeFormatW(_: ::winnt::LCID, _: ::minwindef::DWORD, _: *const ::minwinbase::SYSTEMTIME, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* datetimeapi.h:99:1 */
    pub fn GetUserDefaultLCID() -> ::winnt::LCID; /* winnls.h:2002:1 */
    pub fn GetUserDefaultLangID() -> ::winnt::LANGID; /* winnls.h:1992:1 */
    pub fn GetVersion() -> ::minwindef::DWORD; /* sysinfoapi.h:110:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetVersionExW as GetVersionEx; /* sysinfoapi.h:447:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetVersionExA(_: ::winnt::LPOSVERSIONINFOA) -> ::minwindef::BOOL; /* sysinfoapi.h:433:1 */
    pub fn GetVersionExW(_: ::winnt::LPOSVERSIONINFOW) -> ::minwindef::BOOL; /* sysinfoapi.h:442:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetVolumeInformationW as GetVolumeInformation; /* fileapi.h:842:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetVolumeInformationA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5825:1 */
    pub fn GetVolumeInformationW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:829:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetVolumeNameForVolumeMountPointW as GetVolumeNameForVolumeMountPoint; /* fileapi.h:1227:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetVolumeNameForVolumeMountPointW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1219:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetVolumePathNameW as GetVolumePathName; /* fileapi.h:856:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetVolumePathNameW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:848:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetWindowsDirectoryW as GetWindowsDirectory; /* sysinfoapi.h:291:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetWindowsDirectoryA(_: ::winnt::LPSTR, _: ::minwindef::UINT) -> ::minwindef::UINT; /* sysinfoapi.h:275:1 */
    pub fn GetWindowsDirectoryW(_: ::winnt::LPWSTR, _: ::minwindef::UINT) -> ::minwindef::UINT; /* sysinfoapi.h:285:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GlobalAddAtomW as GlobalAddAtom; /* winbase.h:3855:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GlobalAddAtomA(_: ::winnt::LPCSTR) -> ::minwindef::ATOM; /* winbase.h:3845:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GlobalAddAtomExW as GlobalAddAtomEx; /* winbase.h:3875:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GlobalAddAtomExA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::minwindef::ATOM; /* winbase.h:3863:1 */
    pub fn GlobalAddAtomExW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::minwindef::ATOM; /* winbase.h:3870:1 */
    pub fn GlobalAddAtomW(_: ::winnt::LPCWSTR) -> ::minwindef::ATOM; /* winbase.h:3851:1 */
    pub fn GlobalAlloc(_: ::minwindef::UINT, _: ::basetsd::SIZE_T) -> ::minwindef::HGLOBAL; /* winbase.h:930:1 */
    pub fn GlobalCompact(_: ::minwindef::DWORD) -> ::basetsd::SIZE_T; /* winbase.h:994:1 */
    pub fn GlobalDeleteAtom(_: ::minwindef::ATOM) -> ::minwindef::ATOM; /* winbase.h:1902:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GlobalFindAtomW as GlobalFindAtom; /* winbase.h:3893:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GlobalFindAtomA(_: ::winnt::LPCSTR) -> ::minwindef::ATOM; /* winbase.h:3883:1 */
    pub fn GlobalFindAtomW(_: ::winnt::LPCWSTR) -> ::minwindef::ATOM; /* winbase.h:3889:1 */
    pub fn GlobalFix(_: ::minwindef::HGLOBAL); /* winbase.h:1001:1 */
    pub fn GlobalFlags(_: ::minwindef::HGLOBAL) -> ::minwindef::UINT; /* winbase.h:955:1 */
    pub fn GlobalFree(_: ::minwindef::HGLOBAL) -> ::minwindef::HGLOBAL; /* winbase.h:987:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GlobalGetAtomNameW as GlobalGetAtomName; /* winbase.h:3915:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GlobalGetAtomNameA(_: ::minwindef::ATOM, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::minwindef::UINT; /* winbase.h:3901:1 */
    pub fn GlobalGetAtomNameW(_: ::minwindef::ATOM, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::minwindef::UINT; /* winbase.h:3909:1 */
    pub fn GlobalHandle(_: ::minwindef::LPCVOID) -> ::minwindef::HGLOBAL; /* winbase.h:971:1 */
    pub fn GlobalLock(_: ::minwindef::HGLOBAL) -> ::minwindef::LPVOID; /* winbase.h:963:1 */
    pub fn GlobalMemoryStatus(_: ::winbase::LPMEMORYSTATUS); /* winbase.h:1030:1 */
    pub fn GlobalMemoryStatusEx(_: ::sysinfoapi::LPMEMORYSTATUSEX) -> ::minwindef::BOOL; /* sysinfoapi.h:130:1 */
    pub fn GlobalReAlloc(_: ::minwindef::HGLOBAL, _: ::basetsd::SIZE_T, _: ::minwindef::UINT) -> ::minwindef::HGLOBAL; /* winbase.h:939:1 */
    pub fn GlobalSize(_: ::minwindef::HGLOBAL) -> ::basetsd::SIZE_T; /* winbase.h:948:1 */
    pub fn GlobalUnWire(_: ::minwindef::HGLOBAL) -> ::minwindef::BOOL; /* winbase.h:1022:1 */
    pub fn GlobalUnfix(_: ::minwindef::HGLOBAL); /* winbase.h:1008:1 */
    pub fn GlobalUnlock(_: ::minwindef::HGLOBAL) -> ::minwindef::BOOL; /* winbase.h:978:1 */
    pub fn GlobalWire(_: ::minwindef::HGLOBAL) -> ::minwindef::LPVOID; /* winbase.h:1015:1 */
    pub fn HeapCompact(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::basetsd::SIZE_T; /* heapapi.h:159:1 */
    pub fn HeapCreate(_: ::minwindef::DWORD, _: ::basetsd::SIZE_T, _: ::basetsd::SIZE_T) -> ::winnt::HANDLE; /* heapapi.h:70:1 */
    pub fn HeapDestroy(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* heapapi.h:80:1 */
    pub fn HeapLock(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* heapapi.h:213:1 */
    pub fn HeapQueryInformation(_: ::winnt::HANDLE, _: ::winnt::HEAP_INFORMATION_CLASS, _: ::winnt::PVOID, _: ::basetsd::SIZE_T, _: ::basetsd::PSIZE_T) -> ::minwindef::BOOL; /* heapapi.h:249:1 */
    pub fn HeapSetInformation(_: ::winnt::HANDLE, _: ::winnt::HEAP_INFORMATION_CLASS, _: ::winnt::PVOID, _: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* heapapi.h:238:1 */
    pub fn HeapSummary(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::heapapi::LPHEAP_SUMMARY) -> ::minwindef::BOOL; /* heapapi.h:170:1 */
    pub fn HeapUnlock(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* heapapi.h:221:1 */
    pub fn HeapValidate(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPCVOID) -> ::minwindef::BOOL; /* heapapi.h:149:1 */
    pub fn HeapWalk(_: ::winnt::HANDLE, _: ::minwinbase::LPPROCESS_HEAP_ENTRY) -> ::minwindef::BOOL; /* heapapi.h:229:1 */
    pub fn InitAtomTable(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:1909:1 */
    pub fn InitializeCriticalSection(_: ::minwinbase::LPCRITICAL_SECTION); /* synchapi.h:162:1 */
    pub fn InitializeCriticalSectionAndSpinCount(_: ::minwinbase::LPCRITICAL_SECTION, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:203:1 */
    pub fn InitializeSynchronizationBarrier(_: ::synchapi::LPSYNCHRONIZATION_BARRIER, _: ::winnt::LONG, _: ::winnt::LONG) -> ::minwindef::BOOL; /* synchapi.h:884:1 */
    pub fn InstallELAMCertificateInfo(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* sysinfoapi.h:647:1 */
    pub fn IsBadCodePtr(_: ::minwindef::FARPROC) -> ::minwindef::BOOL; /* winbase.h:6433:1 */
    pub fn IsBadHugeReadPtr(_: *const ::libc::c_void, _: ::basetsd::UINT_PTR) -> ::minwindef::BOOL; /* winbase.h:6417:1 */
    pub fn IsBadHugeWritePtr(_: ::minwindef::LPVOID, _: ::basetsd::UINT_PTR) -> ::minwindef::BOOL; /* winbase.h:6425:1 */
    pub fn IsBadReadPtr(_: *const ::libc::c_void, _: ::basetsd::UINT_PTR) -> ::minwindef::BOOL; /* winbase.h:6401:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsBadStringPtrW as IsBadStringPtr; /* winbase.h:6452:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn IsBadStringPtrA(_: ::winnt::LPCSTR, _: ::basetsd::UINT_PTR) -> ::minwindef::BOOL; /* winbase.h:6440:1 */
    pub fn IsBadStringPtrW(_: ::winnt::LPCWSTR, _: ::basetsd::UINT_PTR) -> ::minwindef::BOOL; /* winbase.h:6447:1 */
    pub fn IsBadWritePtr(_: ::minwindef::LPVOID, _: ::basetsd::UINT_PTR) -> ::minwindef::BOOL; /* winbase.h:6409:1 */
    pub fn IsDBCSLeadByte(_: ::minwindef::BYTE) -> ::minwindef::BOOL; /* winnls.h:1630:1 */
    pub fn IsDBCSLeadByteEx(_: ::minwindef::UINT, _: ::minwindef::BYTE) -> ::minwindef::BOOL; /* winnls.h:1637:1 */
    pub fn IsNLSDefinedString(_: ::winnls::NLS_FUNCTION, _: ::minwindef::DWORD, _: ::winnls::LPNLSVERSIONINFO, _: ::winnt::LPCWSTR, _: ::winnt::INT) -> ::minwindef::BOOL; /* winnls.h:1883:1 */
    pub fn IsSystemResumeAutomatic() -> ::minwindef::BOOL; /* winbase.h:1662:1 */
    pub fn IsValidLocale(_: ::winnt::LCID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1894:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LCMapStringW as LCMapString; /* winnls.h:1483:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LCMapStringA(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::libc::c_int, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1489:1 */
    pub fn LCMapStringW(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1475:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadLibraryW as LoadLibrary; /* winbase.h:3190:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadLibraryA(_: ::winnt::LPCSTR) -> ::minwindef::HMODULE; /* winbase.h:3179:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadLibraryExW as LoadLibraryEx; /* libloaderapi.h:394:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadLibraryExA(_: ::winnt::LPCSTR, _: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::HMODULE; /* libloaderapi.h:377:1 */
    pub fn LoadLibraryExW(_: ::winnt::LPCWSTR, _: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::HMODULE; /* libloaderapi.h:387:1 */
    pub fn LoadLibraryW(_: ::winnt::LPCWSTR) -> ::minwindef::HMODULE; /* winbase.h:3186:1 */
    pub fn LoadModule(_: ::winnt::LPCSTR, _: ::minwindef::LPVOID) -> ::minwindef::DWORD; /* winbase.h:1986:1 */
    pub fn LoadResource(_: ::minwindef::HMODULE, _: ::minwindef::HRSRC) -> ::minwindef::HGLOBAL; /* libloaderapi.h:417:1 */
    pub fn LocalAlloc(_: ::minwindef::UINT, _: ::basetsd::SIZE_T) -> ::minwindef::HLOCAL; /* winbase.h:1039:1 */
    pub fn LocalCompact(_: ::minwindef::UINT) -> ::basetsd::SIZE_T; /* winbase.h:1111:1 */
    pub fn LocalFileTimeToFileTime(_: *const ::minwindef::FILETIME, _: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* fileapi.h:862:1 */
    pub fn LocalFlags(_: ::minwindef::HLOCAL) -> ::minwindef::UINT; /* winbase.h:1087:1 */
    pub fn LocalFree(_: ::minwindef::HLOCAL) -> ::minwindef::HLOCAL; /* winbase.h:1096:1 */
    pub fn LocalHandle(_: ::minwindef::LPCVOID) -> ::minwindef::HLOCAL; /* winbase.h:1066:1 */
    pub fn LocalLock(_: ::minwindef::HLOCAL) -> ::minwindef::LPVOID; /* winbase.h:1058:1 */
    pub fn LocalReAlloc(_: ::minwindef::HLOCAL, _: ::basetsd::SIZE_T, _: ::minwindef::UINT) -> ::minwindef::HLOCAL; /* winbase.h:1048:1 */
    pub fn LocalShrink(_: ::minwindef::HLOCAL, _: ::minwindef::UINT) -> ::basetsd::SIZE_T; /* winbase.h:1103:1 */
    pub fn LocalSize(_: ::minwindef::HLOCAL) -> ::basetsd::SIZE_T; /* winbase.h:1080:1 */
    pub fn LocalUnlock(_: ::minwindef::HLOCAL) -> ::minwindef::BOOL; /* winbase.h:1073:1 */
    pub fn LockFile(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:871:1 */
    pub fn LockResource(_: ::minwindef::HGLOBAL) -> ::minwindef::LPVOID; /* libloaderapi.h:470:1 */
    pub fn MapViewOfFile(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::basetsd::SIZE_T) -> ::minwindef::LPVOID; /* memoryapi.h:276:1 */
    pub fn MapViewOfFileEx(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::basetsd::SIZE_T, _: ::minwindef::LPVOID) -> ::minwindef::LPVOID; /* memoryapi.h:289:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::MoveFileW as MoveFile; /* winbase.h:5331:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn MoveFileA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:5319:1 */
    pub fn MoveFileW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:5326:1 */
    pub fn OpenFile(_: ::winnt::LPCSTR, _: ::winbase::LPOFSTRUCT, _: ::minwindef::UINT) -> ::minwindef::HFILE; /* winbase.h:2764:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenFileMappingW as OpenFileMapping; /* memoryapi.h:269:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenFileMappingA(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:3153:1 */
    pub fn OpenFileMappingW(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* memoryapi.h:261:1 */
    pub fn OpenMutexA(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:2989:1 */
    pub fn OpenPrivateNamespaceW(_: ::minwindef::LPVOID, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* namespaceapi.h:55:1 */
    pub fn OpenProcess(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:938:1 */
    pub fn OpenSemaphoreA(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:3028:1 */
    pub fn OpenThread(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:273:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PeekConsoleInputW as PeekConsoleInput; /* wincon.h:340:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PeekConsoleInputA(_: ::winnt::HANDLE, _: ::wincon::PINPUT_RECORD, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* consoleapi.h:86:1 */
    pub fn PeekConsoleInputW(_: ::winnt::HANDLE, _: ::wincon::PINPUT_RECORD, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:333:1 */
    pub fn PeekNamedPipe(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:89:1 */
    pub fn PostQueuedCompletionStatus(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::basetsd::ULONG_PTR, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* ioapiset.h:105:1 */
    pub fn PrepareTape(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winbase.h:2176:1 */
    pub fn ProcessIdToSessionId(_: ::minwindef::DWORD, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* processthreadsapi.h:677:1 */
    pub fn PulseEvent(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:1885:1 */
    pub fn PurgeComm(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2087:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::QueryDosDeviceW as QueryDosDevice; /* fileapi.h:918:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn QueryDosDeviceA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:4767:1 */
    pub fn QueryDosDeviceW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:910:1 */
    pub fn QueueUserWorkItem(_: ::minwinbase::LPTHREAD_START_ROUTINE, _: ::winnt::PVOID, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* threadpoollegacyapiset.h:47:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ReadConsoleW as ReadConsole; /* consoleapi.h:123:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ReadConsoleA(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::wincon::PCONSOLE_READCONSOLE_CONTROL) -> ::minwindef::BOOL; /* consoleapi.h:102:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ReadConsoleInputW as ReadConsoleInput; /* consoleapi.h:151:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ReadConsoleInputA(_: ::winnt::HANDLE, _: ::wincon::PINPUT_RECORD, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* consoleapi.h:132:1 */
    pub fn ReadConsoleInputW(_: ::winnt::HANDLE, _: ::wincon::PINPUT_RECORD, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* consoleapi.h:143:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ReadConsoleOutputW as ReadConsoleOutput; /* wincon.h:388:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ReadConsoleOutputA(_: ::winnt::HANDLE, _: ::wincon::PCHAR_INFO, _: ::wincon::COORD, _: ::wincon::COORD, _: ::wincon::PSMALL_RECT) -> ::minwindef::BOOL; /* wincon.h:370:1 */
    pub fn ReadConsoleOutputAttribute(_: ::winnt::HANDLE, _: ::minwindef::LPWORD, _: ::minwindef::DWORD, _: ::wincon::COORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:448:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ReadConsoleOutputCharacterW as ReadConsoleOutputCharacter; /* wincon.h:440:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ReadConsoleOutputCharacterA(_: ::winnt::HANDLE, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::wincon::COORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:422:1 */
    pub fn ReadConsoleOutputCharacterW(_: ::winnt::HANDLE, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::wincon::COORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:432:1 */
    pub fn ReadConsoleOutputW(_: ::winnt::HANDLE, _: ::wincon::PCHAR_INFO, _: ::wincon::COORD, _: ::wincon::COORD, _: ::wincon::PSMALL_RECT) -> ::minwindef::BOOL; /* wincon.h:380:1 */
    pub fn ReadConsoleW(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::wincon::PCONSOLE_READCONSOLE_CONTROL) -> ::minwindef::BOOL; /* consoleapi.h:114:1 */
    pub fn ReadFileEx(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwinbase::LPOVERLAPPED, _: ::minwinbase::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::minwindef::BOOL; /* fileapi.h:952:1 */
    pub fn ReadFileScatter(_: ::winnt::HANDLE, _: *mut ::winnt::FILE_SEGMENT_ELEMENT, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* fileapi.h:965:1 */
    pub fn RemoveDllDirectory(_: ::libloaderapi::DLL_DIRECTORY_COOKIE) -> ::minwindef::BOOL; /* libloaderapi.h:506:1 */
    pub fn RequestDeviceWakeup(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:1930:1 */
    pub fn RequestWakeupLatency(_: ::winnt::LATENCY_TIME) -> ::minwindef::BOOL; /* winbase.h:1655:1 */
    pub fn ResetWriteWatch(_: ::minwindef::LPVOID, _: ::basetsd::SIZE_T) -> ::minwindef::UINT; /* memoryapi.h:402:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ScrollConsoleScreenBufferW as ScrollConsoleScreenBuffer; /* wincon.h:694:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ScrollConsoleScreenBufferA(_: ::winnt::HANDLE, _: *const ::wincon::SMALL_RECT, _: *const ::wincon::SMALL_RECT, _: ::wincon::COORD, _: *const ::wincon::CHAR_INFO) -> ::minwindef::BOOL; /* wincon.h:676:1 */
    pub fn ScrollConsoleScreenBufferW(_: ::winnt::HANDLE, _: *const ::wincon::SMALL_RECT, _: *const ::wincon::SMALL_RECT, _: ::wincon::COORD, _: *const ::wincon::CHAR_INFO) -> ::minwindef::BOOL; /* wincon.h:686:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SearchPathW as SearchPath; /* processenv.h:311:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SearchPathA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: *mut *mut ::libc::c_schar) -> ::minwindef::DWORD; /* processenv.h:326:1 */
    pub fn SearchPathW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: *mut *mut ::libc::c_ushort) -> ::minwindef::DWORD; /* processenv.h:300:1 */
    pub fn SetCommBreak(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:2095:1 */
    pub fn SetCommConfig(_: ::winnt::HANDLE, _: ::winbase::LPCOMMCONFIG, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2102:1 */
    pub fn SetCommMask(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2111:1 */
    pub fn SetCommState(_: ::winnt::HANDLE, _: ::winbase::LPDCB) -> ::minwindef::BOOL; /* winbase.h:2119:1 */
    pub fn SetCommTimeouts(_: ::winnt::HANDLE, _: ::winbase::LPCOMMTIMEOUTS) -> ::minwindef::BOOL; /* winbase.h:2127:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetComputerNameW as SetComputerName; /* winbase.h:6820:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetComputerNameA(_: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:6810:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetComputerNameExW as SetComputerNameEx; /* sysinfoapi.h:405:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetComputerNameEx2W as SetComputerNameEx2; /* sysinfoapi.h:631:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetComputerNameEx2W(_: ::sysinfoapi::COMPUTER_NAME_FORMAT, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* sysinfoapi.h:623:1 */
    pub fn SetComputerNameExW(_: ::sysinfoapi::COMPUTER_NAME_FORMAT, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* sysinfoapi.h:398:1 */
    pub fn SetComputerNameW(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:6816:1 */
    pub fn SetConsoleActiveScreenBuffer(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* wincon.h:638:1 */
    pub fn SetConsoleCP(_: ::minwindef::UINT) -> ::minwindef::BOOL; /* wincon.h:837:1 */
    pub fn SetConsoleCtrlHandler(_: ::wincon::PHANDLER_ROUTINE, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* consoleapi.h:159:1 */
    pub fn SetConsoleCursorInfo(_: ::winnt::HANDLE, _: *const ::wincon::CONSOLE_CURSOR_INFO) -> ::minwindef::BOOL; /* wincon.h:668:1 */
    pub fn SetConsoleCursorPosition(_: ::winnt::HANDLE, _: ::wincon::COORD) -> ::minwindef::BOOL; /* wincon.h:660:1 */
    pub fn SetConsoleMode(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* consoleapi.h:168:1 */
    pub fn SetConsoleOutputCP(_: ::minwindef::UINT) -> ::minwindef::BOOL; /* wincon.h:844:1 */
    pub fn SetConsoleScreenBufferInfoEx(_: ::winnt::HANDLE, _: ::wincon::PCONSOLE_SCREEN_BUFFER_INFOEX) -> ::minwindef::BOOL; /* wincon.h:551:1 */
    pub fn SetConsoleScreenBufferSize(_: ::winnt::HANDLE, _: ::wincon::COORD) -> ::minwindef::BOOL; /* wincon.h:652:1 */
    pub fn SetConsoleTextAttribute(_: ::winnt::HANDLE, _: ::minwindef::WORD) -> ::minwindef::BOOL; /* wincon.h:711:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetConsoleTitleW as SetConsoleTitle; /* wincon.h:793:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetConsoleTitleA(_: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* wincon.h:783:1 */
    pub fn SetConsoleTitleW(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* wincon.h:789:1 */
    pub fn SetConsoleWindowInfo(_: ::winnt::HANDLE, _: ::minwindef::BOOL, _: *const ::wincon::SMALL_RECT) -> ::minwindef::BOOL; /* wincon.h:702:1 */
    pub fn SetCriticalSectionSpinCount(_: ::minwinbase::LPCRITICAL_SECTION, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* synchapi.h:241:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetCurrentDirectoryW as SetCurrentDirectory; /* processenv.h:268:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetCurrentDirectoryA(_: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* processenv.h:256:1 */
    pub fn SetCurrentDirectoryW(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* processenv.h:263:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetDefaultCommConfigW as SetDefaultCommConfig; /* winbase.h:6774:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetDefaultCommConfigA(_: ::winnt::LPCSTR, _: ::winbase::LPCOMMCONFIG, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:6760:1 */
    pub fn SetDefaultCommConfigW(_: ::winnt::LPCWSTR, _: ::winbase::LPCOMMCONFIG, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:6768:1 */
    pub fn SetDefaultDllDirectories(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* libloaderapi.h:514:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetEnvironmentStringsW as SetEnvironmentStrings; /* processenv.h:82:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetEnvironmentStringsA(_: ::winnt::LPCH) -> ::minwindef::BOOL; /* winbase.h:1261:1 */
    pub fn SetEnvironmentStringsW(_: ::winnt::LPWCH) -> ::minwindef::BOOL; /* processenv.h:77:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetEnvironmentVariableW as SetEnvironmentVariable; /* processenv.h:222:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetEnvironmentVariableA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* processenv.h:208:1 */
    pub fn SetEnvironmentVariableW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* processenv.h:216:1 */
    pub fn SetErrorMode(_: ::minwindef::UINT) -> ::minwindef::UINT; /* errhandlingapi.h:156:1 */
    pub fn SetFileApisToANSI(); /* winbase.h:5815:1 */
    pub fn SetFileApisToOEM(); /* winbase.h:5810:1 */
    pub fn SetFilePointer(_: ::winnt::HANDLE, _: ::winnt::LONG, _: ::winnt::PLONG, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:1057:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetFileShortNameW as SetFileShortName; /* winbase.h:1973:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetFileShortNameA(_: ::winnt::HANDLE, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:1961:1 */
    pub fn SetFileShortNameW(_: ::winnt::HANDLE, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:1968:1 */
    pub fn SetFileTime(_: ::winnt::HANDLE, _: *const ::minwindef::FILETIME, _: *const ::minwindef::FILETIME, _: *const ::minwindef::FILETIME) -> ::minwindef::BOOL; /* fileapi.h:1093:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetFirmwareEnvironmentVariableW as SetFirmwareEnvironmentVariable; /* winbase.h:3608:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetFirmwareEnvironmentVariableA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::PVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3592:1 */
    pub fn SetFirmwareEnvironmentVariableW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::PVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3601:1 */
    pub fn SetHandleCount(_: ::minwindef::UINT) -> ::minwindef::UINT; /* winbase.h:1923:1 */
    pub fn SetHandleInformation(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* handleapi.h:88:1 */
    pub fn SetLocalTime(_: *const ::minwinbase::SYSTEMTIME) -> ::minwindef::BOOL; /* sysinfoapi.h:176:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetLocaleInfoW as SetLocaleInfo; /* winnls.h:1544:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetLocaleInfoA(_: ::winnt::LCID, _: ::winnls::LCTYPE, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winnls.h:1532:1 */
    pub fn SetLocaleInfoW(_: ::winnt::LCID, _: ::winnls::LCTYPE, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winnls.h:1539:1 */
    pub fn SetMailslotInfo(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2458:1 */
    pub fn SetMessageWaitingIndicator(_: ::winnt::HANDLE, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winbase.h:1952:1 */
    pub fn SetNamedPipeHandleState(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:78:1 */
    pub fn SetPriorityClass(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:652:1 */
    pub fn SetProcessAffinityMask(_: ::winnt::HANDLE, _: ::basetsd::DWORD_PTR) -> ::minwindef::BOOL; /* winbase.h:1219:1 */
    pub fn SetProcessShutdownParameters(_: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:542:1 */
    pub fn SetProcessWorkingSetSize(_: ::winnt::HANDLE, _: ::basetsd::SIZE_T, _: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* winbase.h:1244:1 */
    pub fn SetProcessWorkingSetSizeEx(_: ::winnt::HANDLE, _: ::basetsd::SIZE_T, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:359:1 */
    pub fn SetSearchPathMode(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:4612:1 */
    pub fn SetStdHandle(_: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* processenv.h:116:1 */
    pub fn SetSystemTime(_: *const ::minwinbase::SYSTEMTIME) -> ::minwindef::BOOL; /* sysinfoapi.h:411:1 */
    pub fn SetSystemTimeAdjustment(_: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* sysinfoapi.h:638:1 */
    pub fn SetTapeParameters(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::DWORD; /* winbase.h:2234:1 */
    pub fn SetTapePosition(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winbase.h:2153:1 */
    pub fn SetThreadAffinityMask(_: ::winnt::HANDLE, _: ::basetsd::DWORD_PTR) -> ::basetsd::DWORD_PTR; /* winbase.h:1572:1 */
    pub fn SetThreadContext(_: ::winnt::HANDLE, _: *const ::winnt::CONTEXT) -> ::minwindef::BOOL; /* processthreadsapi.h:868:1 */
    pub fn SetThreadErrorMode(_: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:1799:1 */
    pub fn SetThreadExecutionState(_: ::winnt::EXECUTION_STATE) -> ::winnt::EXECUTION_STATE; /* winbase.h:1678:1 */
    pub fn SetThreadLocale(_: ::winnt::LCID) -> ::minwindef::BOOL; /* winnls.h:1968:1 */
    pub fn SetThreadPriorityBoost(_: ::winnt::HANDLE, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:303:1 */
    pub fn SetThreadStackGuarantee(_: ::minwindef::PULONG) -> ::minwindef::BOOL; /* processthreadsapi.h:661:1 */
    pub fn SetThreadUILanguage(_: ::winnt::LANGID) -> ::winnt::LANGID; /* winnls.h:2008:1 */
    pub fn SetTimeZoneInformation(_: *const ::timezoneapi::TIME_ZONE_INFORMATION) -> ::minwindef::BOOL; /* timezoneapi.h:130:1 */
    pub fn SetUnhandledExceptionFilter(_: ::errhandlingapi::LPTOP_LEVEL_EXCEPTION_FILTER) -> ::errhandlingapi::LPTOP_LEVEL_EXCEPTION_FILTER; /* errhandlingapi.h:100:1 */
    pub fn SetUserGeoID(_: ::winnls::GEOID) -> ::minwindef::BOOL; /* winnls.h:1951:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetVolumeLabelW as SetVolumeLabel; /* winbase.h:5802:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetVolumeLabelA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:5790:1 */
    pub fn SetVolumeLabelW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:5797:1 */
    pub fn SetupComm(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2020:1 */
    pub fn SizeofResource(_: ::minwindef::HMODULE, _: ::minwindef::HRSRC) -> ::minwindef::DWORD; /* libloaderapi.h:478:1 */
    pub fn SwitchToThread() -> ::minwindef::BOOL; /* processthreadsapi.h:195:1 */
    pub fn TerminateProcess(_: ::winnt::HANDLE, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* processthreadsapi.h:177:1 */
    pub fn TerminateThread(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:348:1 */
    pub fn TlsAlloc() -> ::minwindef::DWORD; /* processthreadsapi.h:460:1 */
    pub fn TlsFree(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:485:1 */
    pub fn TlsGetValue(_: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* processthreadsapi.h:468:1 */
    pub fn TlsSetValue(_: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* processthreadsapi.h:476:1 */
    pub fn TransactNamedPipe(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* namedpipeapi.h:102:1 */
    pub fn TransmitCommChar(_: ::winnt::HANDLE, _: ::libc::c_schar) -> ::minwindef::BOOL; /* winbase.h:2135:1 */
    pub fn UnhandledExceptionFilter(_: *mut ::winnt::EXCEPTION_POINTERS) -> ::winnt::LONG; /* errhandlingapi.h:92:1 */
    pub fn UnlockFile(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1118:1 */
    pub fn UnregisterWaitEx(_: ::winnt::HANDLE, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* threadpoollegacyapiset.h:58:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::UpdateResourceW as UpdateResource; /* winbase.h:3815:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn UpdateResourceA(_: ::winnt::HANDLE, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::WORD, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3795:1 */
    pub fn UpdateResourceW(_: ::winnt::HANDLE, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::WORD, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3806:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VerLanguageNameW as VerLanguageName; /* winver.h:178:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VerLanguageNameA(_: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winver.h:165:1 */
    pub fn VerLanguageNameW(_: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winver.h:172:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VerifyVersionInfoW as VerifyVersionInfo; /* winbase.h:7345:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VerifyVersionInfoA(_: ::winnt::LPOSVERSIONINFOEXA, _: ::minwindef::DWORD, _: ::winnt::DWORDLONG) -> ::minwindef::BOOL; /* winbase.h:7331:1 */
    pub fn VerifyVersionInfoW(_: ::winnt::LPOSVERSIONINFOEXW, _: ::minwindef::DWORD, _: ::winnt::DWORDLONG) -> ::minwindef::BOOL; /* winbase.h:7339:1 */
    pub fn VirtualAlloc(_: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* memoryapi.h:90:1 */
    pub fn VirtualAllocEx(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* memoryapi.h:157:1 */
    pub fn VirtualFree(_: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:110:1 */
    pub fn VirtualFreeEx(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:181:1 */
    pub fn VirtualLock(_: ::minwindef::LPVOID, _: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* memoryapi.h:370:1 */
    pub fn VirtualProtect(_: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:121:1 */
    pub fn VirtualProtectEx(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:193:1 */
    pub fn VirtualQueryEx(_: ::winnt::HANDLE, _: ::minwindef::LPCVOID, _: ::winnt::PMEMORY_BASIC_INFORMATION, _: ::basetsd::SIZE_T) -> ::basetsd::SIZE_T; /* memoryapi.h:205:1 */
    pub fn VirtualUnlock(_: ::minwindef::LPVOID, _: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* memoryapi.h:379:1 */
    pub fn WaitCommEvent(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* winbase.h:2143:1 */
    pub fn WaitForDebugEvent(_: ::minwinbase::LPDEBUG_EVENT, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:127:1 */
    pub fn WaitForMultipleObjects(_: ::minwindef::DWORD, _: *const *mut ::libc::c_void, _: ::minwindef::BOOL, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:1892:1 */
    pub fn WaitForSingleObject(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* synchapi.h:473:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WaitNamedPipeW as WaitNamedPipe; /* namedpipeapi.h:142:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WaitNamedPipeA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5729:1 */
    pub fn WaitNamedPipeW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:135:1 */
    pub fn WinExec(_: ::winnt::LPCSTR, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winbase.h:1996:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WriteConsoleW as WriteConsole; /* consoleapi.h:197:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WriteConsoleA(_: ::winnt::HANDLE, _: *const ::libc::c_void, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* consoleapi.h:177:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WriteConsoleInputW as WriteConsoleInput; /* wincon.h:362:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WriteConsoleInputA(_: ::winnt::HANDLE, _: *const ::wincon::INPUT_RECORD, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:346:1 */
    pub fn WriteConsoleInputW(_: ::winnt::HANDLE, _: *const ::wincon::INPUT_RECORD, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:355:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WriteConsoleOutputW as WriteConsoleOutput; /* wincon.h:414:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WriteConsoleOutputA(_: ::winnt::HANDLE, _: *const ::wincon::CHAR_INFO, _: ::wincon::COORD, _: ::wincon::COORD, _: ::wincon::PSMALL_RECT) -> ::minwindef::BOOL; /* wincon.h:396:1 */
    pub fn WriteConsoleOutputAttribute(_: ::winnt::HANDLE, _: *const ::libc::c_ushort, _: ::minwindef::DWORD, _: ::wincon::COORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:485:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WriteConsoleOutputCharacterW as WriteConsoleOutputCharacter; /* wincon.h:477:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WriteConsoleOutputCharacterA(_: ::winnt::HANDLE, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::wincon::COORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:459:1 */
    pub fn WriteConsoleOutputCharacterW(_: ::winnt::HANDLE, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::wincon::COORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:469:1 */
    pub fn WriteConsoleOutputW(_: ::winnt::HANDLE, _: *const ::wincon::CHAR_INFO, _: ::wincon::COORD, _: ::wincon::COORD, _: ::wincon::PSMALL_RECT) -> ::minwindef::BOOL; /* wincon.h:406:1 */
    pub fn WriteConsoleW(_: ::winnt::HANDLE, _: *const ::libc::c_void, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* consoleapi.h:188:1 */
    pub fn WriteFileEx(_: ::winnt::HANDLE, _: ::minwindef::LPCVOID, _: ::minwindef::DWORD, _: ::minwinbase::LPOVERLAPPED, _: ::minwinbase::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::minwindef::BOOL; /* fileapi.h:1168:1 */
    pub fn WriteFileGather(_: ::winnt::HANDLE, _: *mut ::winnt::FILE_SEGMENT_ELEMENT, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* fileapi.h:1180:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WritePrivateProfileSectionW as WritePrivateProfileSection; /* winbase.h:4283:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WritePrivateProfileSectionA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4269:1 */
    pub fn WritePrivateProfileSectionW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:4277:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WritePrivateProfileStringW as WritePrivateProfileString; /* winbase.h:4213:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WritePrivateProfileStringA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4197:1 */
    pub fn WritePrivateProfileStringW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:4206:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WritePrivateProfileStructW as WritePrivateProfileStruct; /* winbase.h:4406:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WritePrivateProfileStructA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::LPVOID, _: ::minwindef::UINT, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4388:1 */
    pub fn WritePrivateProfileStructW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::LPVOID, _: ::minwindef::UINT, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:4398:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WriteProfileSectionW as WriteProfileSection; /* winbase.h:4085:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WriteProfileSectionA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4073:1 */
    pub fn WriteProfileSectionW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:4080:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WriteProfileStringW as WriteProfileString; /* winbase.h:4043:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WriteProfileStringA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4029:1 */
    pub fn WriteProfileStringW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:4037:1 */
    pub fn WriteTapemark(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winbase.h:2204:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::lstrcatW as lstrcat; /* winbase.h:2734:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn lstrcatA(_: ::winnt::LPSTR, _: ::winnt::LPCSTR) -> ::winnt::LPSTR; /* winbase.h:2722:1 */
    pub fn lstrcatW(_: ::winnt::LPWSTR, _: ::winnt::LPCWSTR) -> ::winnt::LPWSTR; /* winbase.h:2729:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::lstrcmpW as lstrcmp; /* winbase.h:2639:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn lstrcmpA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::libc::c_int; /* winbase.h:2627:1 */
    pub fn lstrcmpW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::libc::c_int; /* winbase.h:2634:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::lstrcmpiW as lstrcmpi; /* winbase.h:2659:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn lstrcmpiA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::libc::c_int; /* winbase.h:2647:1 */
    pub fn lstrcmpiW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::libc::c_int; /* winbase.h:2654:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::lstrcpyW as lstrcpy; /* winbase.h:2714:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn lstrcpyA(_: ::winnt::LPSTR, _: ::winnt::LPCSTR) -> ::winnt::LPSTR; /* winbase.h:2702:1 */
    pub fn lstrcpyW(_: ::winnt::LPWSTR, _: ::winnt::LPCWSTR) -> ::winnt::LPWSTR; /* winbase.h:2709:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::lstrcpynW as lstrcpyn; /* winbase.h:2689:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn lstrcpynA(_: ::winnt::LPSTR, _: ::winnt::LPCSTR, _: ::libc::c_int) -> ::winnt::LPSTR; /* winbase.h:2671:1 */
    pub fn lstrcpynW(_: ::winnt::LPWSTR, _: ::winnt::LPCWSTR, _: ::libc::c_int) -> ::winnt::LPWSTR; /* winbase.h:2683:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::lstrlenW as lstrlen; /* winbase.h:2756:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn lstrlenA(_: ::winnt::LPCSTR) -> ::libc::c_int; /* winbase.h:2746:1 */
    pub fn lstrlenW(_: ::winnt::LPCWSTR) -> ::libc::c_int; /* winbase.h:2752:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn RaiseException(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *const ::libc::c_ulonglong); /* errhandlingapi.h:73:1 */
    pub fn RtlLookupFunctionEntry(_: ::basetsd::DWORD64, _: ::basetsd::PDWORD64, _: ::winnt::PUNWIND_HISTORY_TABLE) -> ::winnt::PRUNTIME_FUNCTION; /* winnt.h:17051:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn RtlLookupFunctionEntry(_: ::basetsd::ULONG_PTR, _: ::minwindef::PDWORD, _: ::winnt::PUNWIND_HISTORY_TABLE) -> ::winnt::PRUNTIME_FUNCTION; /* winnt.h:17209:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86", target_arch="arm"))] 
extern "system" {
    pub fn RaiseException(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *const ::libc::c_ulong); /* errhandlingapi.h:73:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64", target_arch="arm"))] 
extern "system" {
    pub fn RtlUnwindEx(_: ::winnt::PVOID, _: ::winnt::PVOID, _: ::winnt::PEXCEPTION_RECORD, _: ::winnt::PVOID, _: ::winnt::PCONTEXT, _: ::winnt::PUNWIND_HISTORY_TABLE); /* winnt.h:17084:1, winnt.h:17242:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::RtlCaptureStackBackTrace as CaptureStackBackTrace; /* winbase.h:112:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn CloseHandle(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* handleapi.h:50:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::CreateDirectoryW as CreateDirectory; /* fileapi.h:107:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn CreateDirectoryA(_: ::winnt::LPCSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::BOOL; /* fileapi.h:93:1 */
    pub fn CreateDirectoryW(_: ::winnt::LPCWSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::BOOL; /* fileapi.h:101:1 */
    pub fn CreateThread(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::basetsd::SIZE_T, _: ::minwinbase::LPTHREAD_START_ROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:209:1 */
    pub fn DecodePointer(_: ::winnt::PVOID) -> ::winnt::PVOID; /* utilapiset.h:52:1 */
    pub fn DeleteCriticalSection(_: ::minwinbase::LPCRITICAL_SECTION); /* synchapi.h:270:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::DeleteFileW as DeleteFile; /* fileapi.h:187:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn DeleteFileA(_: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* fileapi.h:175:1 */
    pub fn DeleteFileW(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* fileapi.h:182:1 */
    pub fn DisableThreadLibraryCalls(_: ::minwindef::HMODULE) -> ::minwindef::BOOL; /* libloaderapi.h:157:1 */
    pub fn DuplicateHandle(_: ::winnt::HANDLE, _: ::winnt::HANDLE, _: ::winnt::HANDLE, _: ::minwindef::LPHANDLE, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* handleapi.h:58:1 */
    pub fn EncodePointer(_: ::winnt::PVOID) -> ::winnt::PVOID; /* utilapiset.h:43:1 */
    pub fn EnterCriticalSection(_: ::minwinbase::LPCRITICAL_SECTION); /* synchapi.h:179:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::EnumSystemCodePagesW as EnumSystemCodePages; /* winnls.h:2283:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn EnumSystemCodePagesA(_: ::winnls::CODEPAGE_ENUMPROCA, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:2273:1 */
    pub fn EnumSystemCodePagesW(_: ::winnls::CODEPAGE_ENUMPROCW, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:2279:1 */
    pub fn EnumSystemGeoID(_: ::winnls::GEOCLASS, _: ::winnls::GEOID, _: ::winnls::GEO_ENUMPROC) -> ::minwindef::BOOL; /* winnls.h:1931:1 */
    pub fn ExitThread(_: ::minwindef::DWORD); /* processthreadsapi.h:335:1 */
    pub fn FileTimeToSystemTime(_: *const ::minwindef::FILETIME, _: ::minwinbase::LPSYSTEMTIME) -> ::minwindef::BOOL; /* timezoneapi.h:95:1 */
    pub fn FindClose(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* fileapi.h:233:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::FindNextFileW as FindNextFile; /* fileapi.h:393:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn FindNextFileA(_: ::winnt::HANDLE, _: ::minwinbase::LPWIN32_FIND_DATAA) -> ::minwindef::BOOL; /* fileapi.h:379:1 */
    pub fn FindNextFileW(_: ::winnt::HANDLE, _: ::minwinbase::LPWIN32_FIND_DATAW) -> ::minwindef::BOOL; /* fileapi.h:387:1 */
    pub fn FlushFileBuffers(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* fileapi.h:437:1 */
    pub fn FlushViewOfFile(_: ::minwindef::LPCVOID, _: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* memoryapi.h:309:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::FormatMessageW as FormatMessage; /* winbase.h:2352:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn FormatMessageA(_: ::minwindef::DWORD, _: ::minwindef::LPCVOID, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: *mut ::libc::c_int) -> ::minwindef::DWORD; /* winbase.h:2329:1 */
    pub fn FormatMessageW(_: ::minwindef::DWORD, _: ::minwindef::LPCVOID, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: *mut ::libc::c_int) -> ::minwindef::DWORD; /* winbase.h:2342:1 */
    pub fn FreeLibrary(_: ::minwindef::HMODULE) -> ::minwindef::BOOL; /* libloaderapi.h:213:1 */
    pub fn GetCPInfo(_: ::minwindef::UINT, _: ::winnls::LPCPINFO) -> ::minwindef::BOOL; /* winnls.h:1376:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::GetCPInfoExW as GetCPInfoEx; /* winnls.h:1395:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn GetCPInfoExA(_: ::minwindef::UINT, _: ::minwindef::DWORD, _: ::winnls::LPCPINFOEXA) -> ::minwindef::BOOL; /* winnls.h:1383:1 */
    pub fn GetCPInfoExW(_: ::minwindef::UINT, _: ::minwindef::DWORD, _: ::winnls::LPCPINFOEXW) -> ::minwindef::BOOL; /* winnls.h:1390:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::GetCommandLineW as GetCommandLine; /* processenv.h:163:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn GetCommandLineA() -> ::winnt::LPSTR; /* processenv.h:151:1 */
    pub fn GetCommandLineW() -> ::winnt::LPWSTR; /* processenv.h:158:1 */
    pub fn GetCurrentProcess() -> ::winnt::HANDLE; /* processthreadsapi.h:145:1 */
    pub fn GetCurrentProcessId() -> ::minwindef::DWORD; /* processthreadsapi.h:153:1 */
    pub fn GetCurrentThread() -> ::winnt::HANDLE; /* processthreadsapi.h:249:1 */
    pub fn GetCurrentThreadId() -> ::minwindef::DWORD; /* processthreadsapi.h:257:1 */
    pub fn GetDateFormatEx(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: *const ::minwinbase::SYSTEMTIME, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::winnt::LPCWSTR) -> ::libc::c_int; /* datetimeapi.h:144:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::GetDiskFreeSpaceExW as GetDiskFreeSpaceEx; /* fileapi.h:505:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn GetDiskFreeSpaceExA(_: ::winnt::LPCSTR, _: ::winnt::PULARGE_INTEGER, _: ::winnt::PULARGE_INTEGER, _: ::winnt::PULARGE_INTEGER) -> ::minwindef::BOOL; /* fileapi.h:487:1 */
    pub fn GetDiskFreeSpaceExW(_: ::winnt::LPCWSTR, _: ::winnt::PULARGE_INTEGER, _: ::winnt::PULARGE_INTEGER, _: ::winnt::PULARGE_INTEGER) -> ::minwindef::BOOL; /* fileapi.h:497:1 */
    pub fn GetExitCodeThread(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:364:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::GetFileAttributesExW as GetFileAttributesEx; /* fileapi.h:599:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn GetFileAttributesExA(_: ::winnt::LPCSTR, _: ::minwinbase::GET_FILEEX_INFO_LEVELS, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* fileapi.h:583:1 */
    pub fn GetFileAttributesExW(_: ::winnt::LPCWSTR, _: ::minwinbase::GET_FILEEX_INFO_LEVELS, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* fileapi.h:592:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::GetGeoInfoW as GetGeoInfo; /* winnls.h:1923:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn GetGeoInfoA(_: ::winnls::GEOID, _: ::winnls::GEOTYPE, _: ::winnt::LPSTR, _: ::libc::c_int, _: ::winnt::LANGID) -> ::libc::c_int; /* winnls.h:1907:1 */
    pub fn GetGeoInfoW(_: ::winnls::GEOID, _: ::winnls::GEOTYPE, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::winnt::LANGID) -> ::libc::c_int; /* winnls.h:1916:1 */
    pub fn GetLastError() -> ::minwindef::DWORD; /* errhandlingapi.h:118:1 */
    pub fn GetLocalTime(_: ::minwinbase::LPSYSTEMTIME); /* sysinfoapi.h:161:1 */
    pub fn GetOverlappedResultEx(_: ::winnt::HANDLE, _: ::minwinbase::LPOVERLAPPED, _: ::minwindef::LPDWORD, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* ioapiset.h:159:1 */
    pub fn GetProcAddress(_: ::minwindef::HMODULE, _: ::winnt::LPCSTR) -> ::minwindef::FARPROC; /* libloaderapi.h:360:1 */
    pub fn GetProcessHeap() -> ::winnt::HANDLE; /* heapapi.h:189:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::GetStringTypeExW as GetStringTypeEx; /* stringapiset.h:130:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn GetStringTypeExW(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::LPCWCH, _: ::libc::c_int, _: ::minwindef::LPWORD) -> ::minwindef::BOOL; /* stringapiset.h:121:1 */
    pub fn GetStringTypeW(_: ::minwindef::DWORD, _: ::winnt::LPCWCH, _: ::libc::c_int, _: ::minwindef::LPWORD) -> ::minwindef::BOOL; /* stringapiset.h:136:1 */
    pub fn GetSystemTime(_: ::minwinbase::LPSYSTEMTIME); /* sysinfoapi.h:145:1 */
    pub fn GetSystemTimeAsFileTime(_: ::minwindef::LPFILETIME); /* sysinfoapi.h:153:1 */
    pub fn GetThreadPriority(_: ::winnt::HANDLE) -> ::libc::c_int; /* processthreadsapi.h:326:1 */
    pub fn GetTimeFormatEx(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: *const ::minwinbase::SYSTEMTIME, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* datetimeapi.h:131:1 */
    pub fn GetTimeZoneInformation(_: ::timezoneapi::LPTIME_ZONE_INFORMATION) -> ::minwindef::DWORD; /* timezoneapi.h:115:1 */
    pub fn GetUserGeoID(_: ::winnls::GEOCLASS) -> ::winnls::GEOID; /* winnls.h:1939:1 */
    pub fn HeapAlloc(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::basetsd::SIZE_T) -> ::minwindef::LPVOID; /* heapapi.h:97:1 */
    pub fn HeapFree(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* heapapi.h:122:1 */
    pub fn HeapReAlloc(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::basetsd::SIZE_T) -> ::minwindef::LPVOID; /* heapapi.h:110:1 */
    pub fn HeapSize(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPCVOID) -> ::basetsd::SIZE_T; /* heapapi.h:132:1 */
    pub fn IsProcessorFeaturePresent(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:954:1 */
    pub fn IsValidCodePage(_: ::minwindef::UINT) -> ::minwindef::BOOL; /* winnls.h:1348:1 */
    pub fn LeaveCriticalSection(_: ::minwinbase::LPCRITICAL_SECTION); /* synchapi.h:187:1 */
    pub fn LockFileEx(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* fileapi.h:890:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::MoveFileExW as MoveFileEx; /* winbase.h:5379:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn MoveFileExA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5365:1 */
    pub fn MoveFileExW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5373:1 */
    pub fn MulDiv(_: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::libc::c_int; /* winbase.h:2252:1 */
    pub fn MultiByteToWideChar(_: ::minwindef::UINT, _: ::minwindef::DWORD, _: ::winnt::LPCCH, _: ::libc::c_int, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* stringapiset.h:154:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::OpenEventW as OpenEvent; /* synchapi.h:642:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn OpenEventA(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* synchapi.h:625:1 */
    pub fn OpenEventW(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* synchapi.h:635:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::OpenMutexW as OpenMutex; /* synchapi.h:576:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn OpenMutexW(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* synchapi.h:568:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::OpenSemaphoreW as OpenSemaphore; /* synchapi.h:659:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn OpenSemaphoreW(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* synchapi.h:651:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::OutputDebugStringW as OutputDebugString; /* debugapi.h:97:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn OutputDebugStringA(_: ::winnt::LPCSTR); /* debugapi.h:85:1 */
    pub fn OutputDebugStringW(_: ::winnt::LPCWSTR); /* debugapi.h:92:1 */
    pub fn RaiseFailFastException(_: ::winnt::PEXCEPTION_RECORD, _: ::winnt::PCONTEXT, _: ::minwindef::DWORD); /* winbase.h:1277:1 */
    pub fn ReadFile(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* fileapi.h:932:1 */
    pub fn ReleaseMutex(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* synchapi.h:458:1 */
    pub fn ReleaseSemaphore(_: ::winnt::HANDLE, _: ::winnt::LONG, _: ::minwindef::LPLONG) -> ::minwindef::BOOL; /* synchapi.h:448:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::RemoveDirectoryW as RemoveDirectory; /* fileapi.h:996:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn RemoveDirectoryA(_: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* fileapi.h:984:1 */
    pub fn RemoveDirectoryW(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* fileapi.h:991:1 */
    pub fn ResetEvent(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* synchapi.h:440:1 */
    pub fn ResumeThread(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:381:1 */
    pub fn RtlPcToFileHeader(_: ::winnt::PVOID, _: *mut *mut ::libc::c_void) -> ::winnt::PVOID; /* winnt.h:17417:1 */
    pub fn RtlUnwind(_: ::winnt::PVOID, _: ::winnt::PVOID, _: ::winnt::PEXCEPTION_RECORD, _: ::winnt::PVOID); /* winnt.h:16953:1 */
    pub fn SetEndOfFile(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* fileapi.h:1004:1 */
    pub fn SetEvent(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* synchapi.h:432:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::SetFileAttributesW as SetFileAttributes; /* fileapi.h:1026:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn SetFileAttributesA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1012:1 */
    pub fn SetFileAttributesW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1020:1 */
    pub fn SetFilePointerEx(_: ::winnt::HANDLE, _: ::winnt::LARGE_INTEGER, _: ::winnt::PLARGE_INTEGER, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1075:1 */
    pub fn SetLastError(_: ::minwindef::DWORD); /* errhandlingapi.h:128:1 */
    pub fn SetThreadPriority(_: ::winnt::HANDLE, _: ::libc::c_int) -> ::minwindef::BOOL; /* processthreadsapi.h:288:1 */
    pub fn Sleep(_: ::minwindef::DWORD); /* synchapi.h:908:1 */
    pub fn SleepEx(_: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* synchapi.h:489:1 */
    pub fn SuspendThread(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:373:1 */
    pub fn SystemTimeToFileTime(_: *const ::minwinbase::SYSTEMTIME, _: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* timezoneapi.h:105:1 */
    pub fn SystemTimeToTzSpecificLocalTime(_: *const ::timezoneapi::TIME_ZONE_INFORMATION, _: *const ::minwinbase::SYSTEMTIME, _: ::minwinbase::LPSYSTEMTIME) -> ::minwindef::BOOL; /* timezoneapi.h:73:1 */
    pub fn TzSpecificLocalTimeToSystemTime(_: *const ::timezoneapi::TIME_ZONE_INFORMATION, _: *const ::minwinbase::SYSTEMTIME, _: ::minwinbase::LPSYSTEMTIME) -> ::minwindef::BOOL; /* timezoneapi.h:84:1 */
    pub fn UnlockFileEx(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* fileapi.h:1137:1 */
    pub fn UnmapViewOfFile(_: ::minwindef::LPCVOID) -> ::minwindef::BOOL; /* memoryapi.h:318:1 */
    pub fn VirtualQuery(_: ::minwindef::LPCVOID, _: ::winnt::PMEMORY_BASIC_INFORMATION, _: ::basetsd::SIZE_T) -> ::basetsd::SIZE_T; /* memoryapi.h:139:1 */
    pub fn WaitForMultipleObjectsEx(_: ::minwindef::DWORD, _: *const *mut ::libc::c_void, _: ::minwindef::BOOL, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* synchapi.h:508:1 */
    pub fn WaitForSingleObjectEx(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* synchapi.h:498:1 */
    pub fn WideCharToMultiByte(_: ::minwindef::UINT, _: ::minwindef::DWORD, _: ::winnt::LPCWCH, _: ::libc::c_int, _: ::winnt::LPSTR, _: ::libc::c_int, _: ::winnt::LPCCH, _: ::minwindef::LPBOOL) -> ::libc::c_int; /* stringapiset.h:169:1 */
    pub fn WriteFile(_: ::winnt::HANDLE, _: ::minwindef::LPCVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* fileapi.h:1149:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn CancelWaitableTimer(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* synchapi.h:729:1 */
    pub fn ConvertThreadToFiber(_: ::minwindef::LPVOID) -> ::minwindef::LPVOID; /* winbase.h:1377:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::CopyFileExW as CopyFileEx; /* winbase.h:5128:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn CopyFileExA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winbase::LPPROGRESS_ROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::LPBOOL, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5106:1 */
    pub fn CopyFileExW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winbase::LPPROGRESS_ROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::LPBOOL, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5118:1 */
    pub fn CreateFiber(_: ::basetsd::SIZE_T, _: ::winbase::LPFIBER_START_ROUTINE, _: ::minwindef::LPVOID) -> ::minwindef::LPVOID; /* winbase.h:1367:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::CreateWaitableTimerW as CreateWaitableTimer; /* winbase.h:3058:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn CreateWaitableTimerA(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::BOOL, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:3043:1 */
    pub fn CreateWaitableTimerW(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::BOOL, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winbase.h:3052:1 */
    pub fn GetSystemPowerStatus(_: ::winbase::LPSYSTEM_POWER_STATUS) -> ::minwindef::BOOL; /* winbase.h:7404:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::OpenWaitableTimerW as OpenWaitableTimer; /* synchapi.h:692:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn OpenWaitableTimerA(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:3067:1 */
    pub fn OpenWaitableTimerW(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* synchapi.h:684:1 */
    pub fn QueueUserAPC(_: ::winnt::PAPCFUNC, _: ::winnt::HANDLE, _: ::basetsd::ULONG_PTR) -> ::minwindef::DWORD; /* processthreadsapi.h:114:1 */
    pub fn ReadDirectoryChangesW(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwinbase::LPOVERLAPPED, _: ::minwinbase::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::minwindef::BOOL; /* winbase.h:6368:1 */
    pub fn SetSystemPowerState(_: ::minwindef::BOOL, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:7411:1 */
    pub fn SetWaitableTimer(_: ::winnt::HANDLE, _: *const ::winnt::LARGE_INTEGER, _: ::winnt::LONG, _: ::synchapi::PTIMERAPCROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* synchapi.h:716:1 */
    pub fn SignalObjectAndWait(_: ::winnt::HANDLE, _: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winbase.h:2851:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn ConvertThreadToFiberEx(_: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* winbase.h:1352:1 */
    pub fn CreateFiberEx(_: ::basetsd::SIZE_T, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD, _: ::winbase::LPFIBER_START_ROUTINE, _: ::minwindef::LPVOID) -> ::minwindef::LPVOID; /* winbase.h:1340:1 */
    pub fn DeleteFiber(_: ::minwindef::LPVOID); /* winbase.h:1307:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::FindFirstFileExW as FindFirstFileEx; /* fileapi.h:334:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn FindFirstFileExA(_: ::winnt::LPCSTR, _: ::minwinbase::FINDEX_INFO_LEVELS, _: ::minwindef::LPVOID, _: ::minwinbase::FINDEX_SEARCH_OPS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* fileapi.h:312:1 */
    pub fn FindFirstFileExW(_: ::winnt::LPCWSTR, _: ::minwinbase::FINDEX_INFO_LEVELS, _: ::minwindef::LPVOID, _: ::minwinbase::FINDEX_SEARCH_OPS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* fileapi.h:324:1 */
    pub fn IsDebuggerPresent() -> ::minwindef::BOOL; /* debugapi.h:54:1 */
    pub fn SetThreadIdealProcessor(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:1585:1 */
    pub fn SwitchToFiber(_: ::minwindef::LPVOID); /* winbase.h:1300:1 */
    pub fn TryEnterCriticalSection(_: ::minwinbase::LPCRITICAL_SECTION) -> ::minwindef::BOOL; /* synchapi.h:260:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn ActivateActCtx(_: ::winnt::HANDLE, _: *mut ::libc::c_ulonglong) -> ::minwindef::BOOL; /* winbase.h:7767:1 */
    pub fn QueryActCtxW(_: ::minwindef::DWORD, _: ::winnt::HANDLE, _: ::winnt::PVOID, _: ::minwindef::ULONG, _: ::winnt::PVOID, _: ::basetsd::SIZE_T, _: *mut ::libc::c_ulonglong) -> ::minwindef::BOOL; /* winbase.h:7931:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[cfg(any(target_arch="x86", target_arch="arm"))] 
extern "system" {
    pub fn ActivateActCtx(_: ::winnt::HANDLE, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winbase.h:7767:1 */
    pub fn QueryActCtxW(_: ::minwindef::DWORD, _: ::winnt::HANDLE, _: ::winnt::PVOID, _: ::minwindef::ULONG, _: ::winnt::PVOID, _: ::basetsd::SIZE_T, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winbase.h:7931:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn AddRefActCtx(_: ::winnt::HANDLE); /* winbase.h:7743:1 */
    pub fn AssignProcessToJobObject(_: ::winnt::HANDLE, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:7481:1 */
    pub fn AttachConsole(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wincon.h:733:1 */
    pub fn BindIoCompletionCallback(_: ::winnt::HANDLE, _: ::minwinbase::LPOVERLAPPED_COMPLETION_ROUTINE, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winbase.h:7084:1 */
    pub fn CancelTimerQueueTimer(_: ::winnt::HANDLE, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:7106:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::CreateActCtxW as CreateActCtx; /* winbase.h:7735:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn CreateActCtxA(_: ::winbase::PCACTCTXA) -> ::winnt::HANDLE; /* winbase.h:7725:1 */
    pub fn CreateActCtxW(_: ::winbase::PCACTCTXW) -> ::winnt::HANDLE; /* winbase.h:7731:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::CreateHardLinkW as CreateHardLink; /* winbase.h:5524:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn CreateHardLinkA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::BOOL; /* winbase.h:5510:1 */
    pub fn CreateHardLinkW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::BOOL; /* winbase.h:5518:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::CreateJobObjectW as CreateJobObject; /* winbase.h:7449:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn CreateJobObjectA(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:7436:1 */
    pub fn CreateJobObjectW(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winbase.h:7444:1 */
    pub fn CreateJobSet(_: ::minwindef::ULONG, _: ::winnt::PJOB_SET_ARRAY, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winbase.h:7518:1 */
    pub fn DeactivateActCtx(_: ::minwindef::DWORD, _: ::basetsd::ULONG_PTR) -> ::minwindef::BOOL; /* winbase.h:7779:1 */
    pub fn DeleteTimerQueue(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:7115:1 */
    pub fn DeleteVolumeMountPointA(_: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:7620:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::DnsHostnameToComputerNameW as DnsHostnameToComputerName; /* winbase.h:6859:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn DnsHostnameToComputerNameA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6844:1 */
    pub fn DnsHostnameToComputerNameW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6853:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumCalendarInfoExW as EnumCalendarInfoEx; /* winnls.h:1788:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumCalendarInfoExA(_: ::winnls::CALINFO_ENUMPROCEXA, _: ::winnt::LCID, _: ::winnls::CALID, _: ::winnls::CALTYPE) -> ::minwindef::BOOL; /* winnls.h:1773:1 */
    pub fn EnumCalendarInfoExW(_: ::winnls::CALINFO_ENUMPROCEXW, _: ::winnt::LCID, _: ::winnls::CALID, _: ::winnls::CALTYPE) -> ::minwindef::BOOL; /* winnls.h:1782:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumDateFormatsExW as EnumDateFormatsEx; /* winnls.h:1856:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumDateFormatsExA(_: ::winnls::DATEFMT_ENUMPROCEXA, _: ::winnt::LCID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1843:1 */
    pub fn EnumDateFormatsExW(_: ::winnls::DATEFMT_ENUMPROCEXW, _: ::winnt::LCID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1851:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumLanguageGroupLocalesW as EnumLanguageGroupLocales; /* winnls.h:2238:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumLanguageGroupLocalesA(_: ::winnls::LANGGROUPLOCALE_ENUMPROCA, _: ::winnls::LGRPID, _: ::minwindef::DWORD, _: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winnls.h:2224:1 */
    pub fn EnumLanguageGroupLocalesW(_: ::winnls::LANGGROUPLOCALE_ENUMPROCW, _: ::winnls::LGRPID, _: ::minwindef::DWORD, _: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winnls.h:2232:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumSystemLanguageGroupsW as EnumSystemLanguageGroups; /* winnls.h:2216:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumSystemLanguageGroupsA(_: ::winnls::LANGUAGEGROUP_ENUMPROCA, _: ::minwindef::DWORD, _: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winnls.h:2204:1 */
    pub fn EnumSystemLanguageGroupsW(_: ::winnls::LANGUAGEGROUP_ENUMPROCW, _: ::minwindef::DWORD, _: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winnls.h:2211:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumSystemLocalesW as EnumSystemLocales; /* winnls.h:2195:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumSystemLocalesA(_: ::winnls::LOCALE_ENUMPROCA, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:2185:1 */
    pub fn EnumSystemLocalesW(_: ::winnls::LOCALE_ENUMPROCW, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:2191:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumUILanguagesW as EnumUILanguages; /* winnls.h:2258:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumUILanguagesA(_: ::winnls::UILANGUAGE_ENUMPROCA, _: ::minwindef::DWORD, _: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winnls.h:2246:1 */
    pub fn EnumUILanguagesW(_: ::winnls::UILANGUAGE_ENUMPROCW, _: ::minwindef::DWORD, _: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winnls.h:2253:1 */
    pub fn FindActCtxSectionGuid(_: ::minwindef::DWORD, _: *const ::guiddef::GUID, _: ::minwindef::ULONG, _: *const ::guiddef::GUID, _: ::winbase::PACTCTX_SECTION_KEYED_DATA) -> ::minwindef::BOOL; /* winbase.h:7868:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::FindActCtxSectionStringW as FindActCtxSectionString; /* winbase.h:7860:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn FindActCtxSectionStringA(_: ::minwindef::DWORD, _: *const ::guiddef::GUID, _: ::minwindef::ULONG, _: ::winnt::LPCSTR, _: ::winbase::PACTCTX_SECTION_KEYED_DATA) -> ::minwindef::BOOL; /* winbase.h:7841:1 */
    pub fn FindActCtxSectionStringW(_: ::minwindef::DWORD, _: *const ::guiddef::GUID, _: ::minwindef::ULONG, _: ::winnt::LPCWSTR, _: ::winbase::PACTCTX_SECTION_KEYED_DATA) -> ::minwindef::BOOL; /* winbase.h:7852:1 */
    pub fn FindFirstVolumeA(_: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:7526:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::FindFirstVolumeMountPointW as FindFirstVolumeMountPoint; /* winbase.h:7563:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn FindFirstVolumeMountPointA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:7549:1 */
    pub fn FindFirstVolumeMountPointW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:7557:1 */
    pub fn FindNextVolumeA(_: ::winnt::HANDLE, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:7537:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::FindNextVolumeMountPointW as FindNextVolumeMountPoint; /* winbase.h:7585:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn FindNextVolumeMountPointA(_: ::winnt::HANDLE, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:7571:1 */
    pub fn FindNextVolumeMountPointW(_: ::winnt::HANDLE, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:7579:1 */
    pub fn FindVolumeMountPointClose(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:7593:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::GetCalendarInfoW as GetCalendarInfo; /* winnls.h:1574:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn GetCalendarInfoA(_: ::winnt::LCID, _: ::winnls::CALID, _: ::winnls::CALTYPE, _: ::winnt::LPSTR, _: ::libc::c_int, _: ::minwindef::LPDWORD) -> ::libc::c_int; /* winnls.h:1555:1 */
    pub fn GetCalendarInfoW(_: ::winnt::LCID, _: ::winnls::CALID, _: ::winnls::CALTYPE, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::minwindef::LPDWORD) -> ::libc::c_int; /* winnls.h:1566:1 */
    pub fn GetConsoleDisplayMode(_: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:856:1 */
    pub fn GetConsoleFontSize(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::wincon::COORD; /* wincon.h:614:1 */
    pub fn GetConsoleHistoryInfo(_: ::wincon::PCONSOLE_HISTORY_INFO) -> ::minwindef::BOOL; /* wincon.h:602:1 */
    pub fn GetConsoleSelectionInfo(_: ::wincon::PCONSOLE_SELECTION_INFO) -> ::minwindef::BOOL; /* wincon.h:622:1 */
    pub fn GetConsoleWindow() -> ::windef::HWND; /* wincon.h:872:1 */
    pub fn GetCurrentActCtx(_: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winbase.h:7787:1 */
    pub fn GetCurrentConsoleFont(_: ::winnt::HANDLE, _: ::minwindef::BOOL, _: ::wincon::PCONSOLE_FONT_INFO) -> ::minwindef::BOOL; /* wincon.h:575:1 */
    pub fn GetCurrentConsoleFontEx(_: ::winnt::HANDLE, _: ::minwindef::BOOL, _: ::wincon::PCONSOLE_FONT_INFOEX) -> ::minwindef::BOOL; /* wincon.h:585:1 */
    pub fn GetSystemDefaultUILanguage() -> ::winnt::LANGID; /* winnls.h:1976:1 */
    pub fn GetUserDefaultUILanguage() -> ::winnt::LANGID; /* winnls.h:1981:1 */
    pub fn GetVolumeNameForVolumeMountPointA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:7630:1 */
    pub fn GetVolumePathNameA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:7642:1 */
    pub fn IsValidLanguageGroup(_: ::winnls::LGRPID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1866:1 */
    pub fn MapUserPhysicalPagesScatter(_: *mut *mut ::libc::c_void, _: ::basetsd::ULONG_PTR, _: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* winbase.h:7426:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::MoveFileWithProgressW as MoveFileWithProgress; /* winbase.h:5412:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn MoveFileWithProgressA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winbase::LPPROGRESS_ROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5394:1 */
    pub fn MoveFileWithProgressW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winbase::LPPROGRESS_ROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5404:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::OpenJobObjectW as OpenJobObject; /* winbase.h:7473:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn OpenJobObjectA(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:7458:1 */
    pub fn OpenJobObjectW(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winbase.h:7467:1 */
    pub fn QueryInformationJobObject(_: ::winnt::HANDLE, _: ::winnt::JOBOBJECTINFOCLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:7497:1 */
    pub fn RegisterWaitForSingleObject(_: ::winnt::PHANDLE, _: ::winnt::HANDLE, _: ::winnt::WAITORTIMERCALLBACK, _: ::winnt::PVOID, _: ::minwindef::ULONG, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winbase.h:7064:1 */
    pub fn ReleaseActCtx(_: ::winnt::HANDLE); /* winbase.h:7751:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::ReplaceFileW as ReplaceFile; /* winbase.h:5495:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn ReplaceFileA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winbase.h:5475:1 */
    pub fn ReplaceFileW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winbase.h:5486:1 */
    pub fn RtlCompareMemory(_: *const ::libc::c_void, _: *const ::libc::c_void, _: ::basetsd::SIZE_T) -> ::basetsd::SIZE_T; /* winnt.h:17442:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::SetCalendarInfoW as SetCalendarInfo; /* winnls.h:1596:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn SetCalendarInfoA(_: ::winnt::LCID, _: ::winnls::CALID, _: ::winnls::CALTYPE, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winnls.h:1582:1 */
    pub fn SetCalendarInfoW(_: ::winnt::LCID, _: ::winnls::CALID, _: ::winnls::CALTYPE, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winnls.h:1590:1 */
    pub fn SetComputerNameExA(_: ::sysinfoapi::COMPUTER_NAME_FORMAT, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:6831:1 */
    pub fn SetConsoleDisplayMode(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::wincon::PCOORD) -> ::minwindef::BOOL; /* wincon.h:864:1 */
    pub fn SetConsoleHistoryInfo(_: ::wincon::PCONSOLE_HISTORY_INFO) -> ::minwindef::BOOL; /* wincon.h:608:1 */
    pub fn SetCurrentConsoleFontEx(_: ::winnt::HANDLE, _: ::minwindef::BOOL, _: ::wincon::PCONSOLE_FONT_INFOEX) -> ::minwindef::BOOL; /* wincon.h:593:1 */
    pub fn SetInformationJobObject(_: ::winnt::HANDLE, _: ::winnt::JOBOBJECTINFOCLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:7508:1 */
    pub fn SetTimerQueueTimer(_: ::winnt::HANDLE, _: ::winnt::WAITORTIMERCALLBACK, _: ::winnt::PVOID, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::winnt::HANDLE; /* winbase.h:7093:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::SetVolumeMountPointW as SetVolumeMountPoint; /* winbase.h:7612:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn SetVolumeMountPointA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:7600:1 */
    pub fn SetVolumeMountPointW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:7607:1 */
    pub fn TerminateJobObject(_: ::winnt::HANDLE, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* winbase.h:7489:1 */
    pub fn UnregisterWait(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:7077:1 */
    pub fn VerSetConditionMask(_: ::winnt::ULONGLONG, _: ::minwindef::DWORD, _: ::minwindef::BYTE) -> ::winnt::ULONGLONG; /* winnt.h:17986:1 */
    pub fn ZombifyActCtx(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:7758:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000100"))] 
extern "system" {
    pub fn RtlCaptureContext(_: ::winnt::PCONTEXT); /* winnt.h:16934:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::AddConsoleAliasW as AddConsoleAlias; /* wincon.h:906:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn AddConsoleAliasA(_: ::winnt::LPSTR, _: ::winnt::LPSTR, _: ::winnt::LPSTR) -> ::minwindef::BOOL; /* wincon.h:894:1 */
    pub fn AddConsoleAliasW(_: ::winnt::LPWSTR, _: ::winnt::LPWSTR, _: ::winnt::LPWSTR) -> ::minwindef::BOOL; /* wincon.h:901:1 */
    pub fn AddVectoredContinueHandler(_: ::minwindef::ULONG, _: ::winnt::PVECTORED_EXCEPTION_HANDLER) -> ::winnt::PVOID; /* errhandlingapi.h:197:1 */
    pub fn AddVectoredExceptionHandler(_: ::minwindef::ULONG, _: ::winnt::PVECTORED_EXCEPTION_HANDLER) -> ::winnt::PVOID; /* errhandlingapi.h:179:1 */
    pub fn AllocateUserPhysicalPages(_: ::winnt::HANDLE, _: ::basetsd::PULONG_PTR, _: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* memoryapi.h:582:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::CheckNameLegalDOS8Dot3W as CheckNameLegalDOS8Dot3; /* winbase.h:4979:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn CheckNameLegalDOS8Dot3A(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::minwindef::PBOOL, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* winbase.h:4961:1 */
    pub fn CheckNameLegalDOS8Dot3W(_: ::winnt::LPCWSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::minwindef::PBOOL, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* winbase.h:4971:1 */
    pub fn CheckRemoteDebuggerPresent(_: ::winnt::HANDLE, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* debugapi.h:155:1 */
    pub fn CreateMemoryResourceNotification(_: ::memoryapi::MEMORY_RESOURCE_NOTIFICATION_TYPE) -> ::winnt::HANDLE; /* memoryapi.h:420:1 */
    pub fn FindFirstStreamW(_: ::winnt::LPCWSTR, _: ::winbase::STREAM_INFO_LEVELS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:5586:1 */
    pub fn FindNextStreamW(_: ::winnt::HANDLE, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winbase.h:5609:1 */
    pub fn FreeUserPhysicalPages(_: ::winnt::HANDLE, _: ::basetsd::PULONG_PTR, _: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* memoryapi.h:593:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetCompressedFileSizeW as GetCompressedFileSize; /* fileapi.h:1340:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetCompressedFileSizeA(_: ::winnt::LPCSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* fileapi.h:1326:1 */
    pub fn GetCompressedFileSizeW(_: ::winnt::LPCWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* fileapi.h:1334:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetConsoleAliasW as GetConsoleAlias; /* wincon.h:928:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetConsoleAliasA(_: ::winnt::LPSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::winnt::LPSTR) -> ::minwindef::DWORD; /* wincon.h:914:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetConsoleAliasExesW as GetConsoleAliasExes; /* wincon.h:998:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetConsoleAliasExesA(_: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:988:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetConsoleAliasExesLengthW as GetConsoleAliasExesLength; /* wincon.h:960:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetConsoleAliasExesLengthA() -> ::minwindef::DWORD; /* wincon.h:952:1 */
    pub fn GetConsoleAliasExesLengthW() -> ::minwindef::DWORD; /* wincon.h:957:1 */
    pub fn GetConsoleAliasExesW(_: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:994:1 */
    pub fn GetConsoleAliasW(_: ::winnt::LPWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* wincon.h:922:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetConsoleAliasesW as GetConsoleAliases; /* wincon.h:980:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetConsoleAliasesA(_: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::winnt::LPSTR) -> ::minwindef::DWORD; /* wincon.h:968:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetConsoleAliasesLengthW as GetConsoleAliasesLength; /* wincon.h:944:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetConsoleAliasesLengthA(_: ::winnt::LPSTR) -> ::minwindef::DWORD; /* wincon.h:936:1 */
    pub fn GetConsoleAliasesLengthW(_: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* wincon.h:941:1 */
    pub fn GetConsoleAliasesW(_: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* wincon.h:975:1 */
    pub fn GetConsoleProcessList(_: ::minwindef::LPDWORD, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:883:1 */
    pub fn GetProcessHandleCount(_: ::winnt::HANDLE, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:974:1 */
    pub fn GetProcessId(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:690:1 */
    pub fn GetProcessPriorityBoost(_: ::winnt::HANDLE, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:1067:1 */
    pub fn GetSystemRegistryQuota(_: ::minwindef::PDWORD, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:2288:1 */
    pub fn GetSystemTimes(_: ::minwindef::PFILETIME, _: ::minwindef::PFILETIME, _: ::minwindef::PFILETIME) -> ::minwindef::BOOL; /* processthreadsapi.h:1094:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetSystemWow64DirectoryW as GetSystemWow64Directory; /* winbase.h:4479:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetSystemWow64DirectoryA(_: ::winnt::LPSTR, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winbase.h:4466:1 */
    pub fn GetSystemWow64DirectoryW(_: ::winnt::LPWSTR, _: ::minwindef::UINT) -> ::minwindef::UINT; /* winbase.h:4474:1 */
    pub fn GetThreadIOPendingFlag(_: ::winnt::HANDLE, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:1085:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetVolumePathNamesForVolumeNameW as GetVolumePathNamesForVolumeName; /* fileapi.h:1245:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetVolumePathNamesForVolumeNameA(_: ::winnt::LPCSTR, _: ::winnt::LPCH, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:7658:1 */
    pub fn GetVolumePathNamesForVolumeNameW(_: ::winnt::LPCWSTR, _: ::winnt::LPWCH, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* fileapi.h:1236:1 */
    pub fn IsProcessInJob(_: ::winnt::HANDLE, _: ::winnt::HANDLE, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* jobapi.h:46:1 */
    pub fn IsWow64Process(_: ::winnt::HANDLE, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* wow64apiset.h:72:1 */
    pub fn MapUserPhysicalPages(_: ::winnt::PVOID, _: ::basetsd::ULONG_PTR, _: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* memoryapi.h:604:1 */
    pub fn QueryMemoryResourceNotification(_: ::winnt::HANDLE, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* memoryapi.h:429:1 */
    pub fn RemoveVectoredContinueHandler(_: ::winnt::PVOID) -> ::minwindef::ULONG; /* errhandlingapi.h:206:1 */
    pub fn RemoveVectoredExceptionHandler(_: ::winnt::PVOID) -> ::minwindef::ULONG; /* errhandlingapi.h:188:1 */
    pub fn SetFileValidData(_: ::winnt::HANDLE, _: ::winnt::LONGLONG) -> ::minwindef::BOOL; /* fileapi.h:1107:1 */
    pub fn SetProcessPriorityBoost(_: ::winnt::HANDLE, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:1076:1 */
    pub fn WTSGetActiveConsoleSessionId() -> ::minwindef::DWORD; /* winbase.h:7959:1 */
    pub fn Wow64DisableWow64FsRedirection(_: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* wow64apiset.h:49:1 */
    pub fn Wow64EnableWow64FsRedirection(_: ::winnt::BOOLEAN) -> ::winnt::BOOLEAN; /* winbase.h:4487:1 */
    pub fn Wow64RevertWow64FsRedirection(_: ::winnt::PVOID) -> ::minwindef::BOOL; /* wow64apiset.h:57:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn ConvertFiberToThread() -> ::minwindef::BOOL; /* winbase.h:1316:1 */
    pub fn GetNativeSystemInfo(_: ::sysinfoapi::LPSYSTEM_INFO); /* sysinfoapi.h:495:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010100"))] 
extern "system" {
    pub fn RtlCaptureStackBackTrace(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *mut *mut ::libc::c_void, _: ::minwindef::PDWORD) -> ::minwindef::WORD; /* winnt.h:16911:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010300"))] 
extern "system" {
    pub fn GetSystemDEPPolicy() -> ::winbase::DEP_SYSTEM_POLICY_TYPE; /* winbase.h:2277:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] 
extern "system" {
    pub fn GetCurrentProcessorNumber() -> ::minwindef::DWORD; /* processthreadsapi.h:995:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] #[doc(inline)] pub use self::GetDllDirectoryW as GetDllDirectory; /* winbase.h:4597:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] 
extern "system" {
    pub fn GetDllDirectoryA(_: ::minwindef::DWORD, _: ::winnt::LPSTR) -> ::minwindef::DWORD; /* winbase.h:4584:1 */
    pub fn GetDllDirectoryW(_: ::minwindef::DWORD, _: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* winbase.h:4592:1 */
    pub fn GetSystemFileCacheSize(_: ::basetsd::PSIZE_T, _: ::basetsd::PSIZE_T, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:450:1 */
    pub fn GetThreadId(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:703:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] #[doc(inline)] pub use self::NeedCurrentDirectoryForExePathW as NeedCurrentDirectoryForExePath; /* processenv.h:354:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] 
extern "system" {
    pub fn NeedCurrentDirectoryForExePathA(_: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* processenv.h:342:1 */
    pub fn NeedCurrentDirectoryForExePathW(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* processenv.h:349:1 */
    pub fn ReOpenFile(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:4824:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] #[doc(inline)] pub use self::SetDllDirectoryW as SetDllDirectory; /* winbase.h:4575:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] 
extern "system" {
    pub fn SetDllDirectoryA(_: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4565:1 */
    pub fn SetDllDirectoryW(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:4571:1 */
    pub fn SetSystemFileCacheSize(_: ::basetsd::SIZE_T, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:460:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn QueryActCtxSettingsW(_: ::minwindef::DWORD, _: ::winnt::HANDLE, _: ::winnt::PCWSTR, _: ::winnt::PCWSTR, _: ::winnt::PWSTR, _: ::basetsd::SIZE_T, _: *mut ::libc::c_ulonglong) -> ::minwindef::BOOL; /* winbase.h:8571:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[cfg(any(target_arch="x86", target_arch="arm"))] 
extern "system" {
    pub fn QueryActCtxSettingsW(_: ::minwindef::DWORD, _: ::winnt::HANDLE, _: ::winnt::PCWSTR, _: ::winnt::PCWSTR, _: ::winnt::PWSTR, _: ::basetsd::SIZE_T, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winbase.h:8571:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn AddIntegrityLabelToBoundaryDescriptor(_: *mut *mut ::libc::c_void, _: ::winnt::PSID) -> ::minwindef::BOOL; /* winbase.h:7265:1 */
    pub fn AddSecureMemoryCacheCallback(_: ::winnt::PSECURE_MEMORY_CACHE_CALLBACK) -> ::minwindef::BOOL; /* winbase.h:8602:1 */
    pub fn AllocateUserPhysicalPagesNuma(_: ::winnt::HANDLE, _: ::basetsd::PULONG_PTR, _: ::basetsd::PULONG_PTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:620:1 */
    pub fn ApplicationRecoveryFinished(_: ::minwindef::BOOL); /* winbase.h:8179:1 */
    pub fn ApplicationRecoveryInProgress(_: ::minwindef::PBOOL) -> ::winnt::HRESULT; /* winbase.h:8172:1 */
    pub fn CallbackMayRunLong(_: ::winnt::PTP_CALLBACK_INSTANCE) -> ::minwindef::BOOL; /* threadpoolapiset.h:190:1 */
    pub fn CancelIoEx(_: ::winnt::HANDLE, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* ioapiset.h:134:1 */
    pub fn CancelSynchronousIo(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* ioapiset.h:189:1 */
    pub fn CancelThreadpoolIo(_: ::winnt::PTP_IO); /* threadpoolapiset.h:358:1 */
    pub fn CloseThreadpool(_: ::winnt::PTP_POOL); /* threadpoolapiset.h:109:1 */
    pub fn CloseThreadpoolCleanupGroup(_: ::winnt::PTP_CLEANUP_GROUP); /* threadpoolapiset.h:136:1 */
    pub fn CloseThreadpoolCleanupGroupMembers(_: ::winnt::PTP_CLEANUP_GROUP, _: ::minwindef::BOOL, _: ::winnt::PVOID); /* threadpoolapiset.h:126:1 */
    pub fn CloseThreadpoolIo(_: ::winnt::PTP_IO); /* threadpoolapiset.h:375:1 */
    pub fn CloseThreadpoolTimer(_: ::winnt::PTP_TIMER); /* threadpoolapiset.h:292:1 */
    pub fn CloseThreadpoolWait(_: ::winnt::PTP_WAIT); /* threadpoolapiset.h:330:1 */
    pub fn CloseThreadpoolWork(_: ::winnt::PTP_WORK); /* threadpoolapiset.h:245:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CopyFileTransactedW as CopyFileTransacted; /* winbase.h:5160:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CopyFileTransactedA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winbase::LPPROGRESS_ROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::LPBOOL, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5138:1 */
    pub fn CopyFileTransactedW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winbase::LPPROGRESS_ROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::LPBOOL, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5150:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateBoundaryDescriptorW as CreateBoundaryDescriptor; /* winbase.h:7258:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateBoundaryDescriptorA(_: ::winnt::LPCSTR, _: ::minwindef::ULONG) -> ::winnt::HANDLE; /* winbase.h:7250:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateDirectoryTransactedW as CreateDirectoryTransacted; /* winbase.h:4691:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateDirectoryTransactedA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4675:1 */
    pub fn CreateDirectoryTransactedW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4684:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateFileMappingNumaW as CreateFileMappingNuma; /* memoryapi.h:488:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateFileMappingNumaA(_: ::winnt::HANDLE, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:3134:1 */
    pub fn CreateFileMappingNumaW(_: ::winnt::HANDLE, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* memoryapi.h:476:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateFileTransactedW as CreateFileTransacted; /* winbase.h:4811:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateFileTransactedA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::HANDLE, _: ::winnt::HANDLE, _: ::minwindef::PUSHORT, _: ::winnt::PVOID) -> ::winnt::HANDLE; /* winbase.h:4783:1 */
    pub fn CreateFileTransactedW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::HANDLE, _: ::winnt::HANDLE, _: ::minwindef::PUSHORT, _: ::winnt::PVOID) -> ::winnt::HANDLE; /* winbase.h:4798:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateHardLinkTransactedW as CreateHardLinkTransacted; /* winbase.h:5555:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateHardLinkTransactedA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5539:1 */
    pub fn CreateHardLinkTransactedW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5548:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreatePrivateNamespaceW as CreatePrivateNamespace; /* winbase.h:7223:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreatePrivateNamespaceA(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::LPVOID, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:7214:1 */
    pub fn CreateSemaphoreExA(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::LONG, _: ::winnt::LONG, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:3082:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateSymbolicLinkW as CreateSymbolicLink; /* winbase.h:8534:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateSymbolicLinkA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::winnt::BOOLEAN; /* winbase.h:8520:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateSymbolicLinkTransactedW as CreateSymbolicLinkTransacted; /* winbase.h:8558:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateSymbolicLinkTransactedA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::winnt::BOOLEAN; /* winbase.h:8542:1 */
    pub fn CreateSymbolicLinkTransactedW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::winnt::BOOLEAN; /* winbase.h:8551:1 */
    pub fn CreateSymbolicLinkW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::winnt::BOOLEAN; /* winbase.h:8528:1 */
    pub fn CreateThreadpool(_: ::winnt::PVOID) -> ::winnt::PTP_POOL; /* threadpoolapiset.h:65:1 */
    pub fn CreateThreadpoolCleanupGroup() -> ::winnt::PTP_CLEANUP_GROUP; /* threadpoolapiset.h:118:1 */
    pub fn CreateThreadpoolIo(_: ::winnt::HANDLE, _: ::threadpoolapiset::PTP_WIN32_IO_CALLBACK, _: ::winnt::PVOID, _: ::winnt::PTP_CALLBACK_ENVIRON) -> ::winnt::PTP_IO; /* threadpoolapiset.h:339:1 */
    pub fn CreateThreadpoolTimer(_: ::winnt::PTP_TIMER_CALLBACK, _: ::winnt::PVOID, _: ::winnt::PTP_CALLBACK_ENVIRON) -> ::winnt::PTP_TIMER; /* threadpoolapiset.h:254:1 */
    pub fn CreateThreadpoolWait(_: ::winnt::PTP_WAIT_CALLBACK, _: ::winnt::PVOID, _: ::winnt::PTP_CALLBACK_ENVIRON) -> ::winnt::PTP_WAIT; /* threadpoolapiset.h:301:1 */
    pub fn CreateThreadpoolWork(_: ::winnt::PTP_WORK_CALLBACK, _: ::winnt::PVOID, _: ::winnt::PTP_CALLBACK_ENVIRON) -> ::winnt::PTP_WORK; /* threadpoolapiset.h:218:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateWaitableTimerExW as CreateWaitableTimerEx; /* synchapi.h:845:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateWaitableTimerExA(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:3098:1 */
    pub fn CreateWaitableTimerExW(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* synchapi.h:836:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::DeleteFileTransactedW as DeleteFileTransacted; /* winbase.h:4919:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn DeleteFileTransactedA(_: ::winnt::LPCSTR, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4907:1 */
    pub fn DeleteFileTransactedW(_: ::winnt::LPCWSTR, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4914:1 */
    pub fn DeleteProcThreadAttributeList(_: ::processthreadsapi::LPPROC_THREAD_ATTRIBUTE_LIST); /* processthreadsapi.h:761:1 */
    pub fn DisassociateCurrentThreadFromCallback(_: ::winnt::PTP_CALLBACK_INSTANCE); /* threadpoolapiset.h:198:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::EnumResourceLanguagesExW as EnumResourceLanguagesEx; /* libloaderapi.h:561:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn EnumResourceLanguagesExA(_: ::minwindef::HMODULE, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::libloaderapi::ENUMRESLANGPROCA, _: ::basetsd::LONG_PTR, _: ::minwindef::DWORD, _: ::winnt::LANGID) -> ::minwindef::BOOL; /* libloaderapi.h:537:1 */
    pub fn EnumResourceLanguagesExW(_: ::minwindef::HMODULE, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::libloaderapi::ENUMRESLANGPROCW, _: ::basetsd::LONG_PTR, _: ::minwindef::DWORD, _: ::winnt::LANGID) -> ::minwindef::BOOL; /* libloaderapi.h:550:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::EnumResourceNamesExW as EnumResourceNamesEx; /* libloaderapi.h:591:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn EnumResourceNamesExA(_: ::minwindef::HMODULE, _: ::winnt::LPCSTR, _: ::libloaderapi::ENUMRESNAMEPROCA, _: ::basetsd::LONG_PTR, _: ::minwindef::DWORD, _: ::winnt::LANGID) -> ::minwindef::BOOL; /* libloaderapi.h:569:1 */
    pub fn EnumResourceNamesExW(_: ::minwindef::HMODULE, _: ::winnt::LPCWSTR, _: ::libloaderapi::ENUMRESNAMEPROCW, _: ::basetsd::LONG_PTR, _: ::minwindef::DWORD, _: ::winnt::LANGID) -> ::minwindef::BOOL; /* libloaderapi.h:581:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::EnumResourceTypesExW as EnumResourceTypesEx; /* libloaderapi.h:619:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn EnumResourceTypesExA(_: ::minwindef::HMODULE, _: ::libloaderapi::ENUMRESTYPEPROCA, _: ::basetsd::LONG_PTR, _: ::minwindef::DWORD, _: ::winnt::LANGID) -> ::minwindef::BOOL; /* libloaderapi.h:599:1 */
    pub fn EnumResourceTypesExW(_: ::minwindef::HMODULE, _: ::libloaderapi::ENUMRESTYPEPROCW, _: ::basetsd::LONG_PTR, _: ::minwindef::DWORD, _: ::winnt::LANGID) -> ::minwindef::BOOL; /* libloaderapi.h:610:1 */
    pub fn FindFirstFileNameTransactedW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::winnt::PWSTR, _: ::winnt::HANDLE) -> ::winnt::HANDLE; /* winbase.h:5639:1 */
    pub fn FindFirstFileNameW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::winnt::PWSTR) -> ::winnt::HANDLE; /* winbase.h:5620:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::FindFirstFileTransactedW as FindFirstFileTransacted; /* winbase.h:5021:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn FindFirstFileTransactedA(_: ::winnt::LPCSTR, _: ::minwinbase::FINDEX_INFO_LEVELS, _: ::minwindef::LPVOID, _: ::minwinbase::FINDEX_SEARCH_OPS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::winnt::HANDLE; /* winbase.h:4999:1 */
    pub fn FindFirstFileTransactedW(_: ::winnt::LPCWSTR, _: ::minwinbase::FINDEX_INFO_LEVELS, _: ::minwindef::LPVOID, _: ::minwinbase::FINDEX_SEARCH_OPS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::winnt::HANDLE; /* winbase.h:5011:1 */
    pub fn FindFirstStreamTransactedW(_: ::winnt::LPCWSTR, _: ::winbase::STREAM_INFO_LEVELS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::winnt::HANDLE; /* winbase.h:5597:1 */
    pub fn FindNLSString(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::minwindef::LPINT) -> ::libc::c_int; /* winnls.h:1460:1 */
    pub fn FindNextFileNameW(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::winnt::PWSTR) -> ::minwindef::BOOL; /* winbase.h:5630:1 */
    pub fn FreeLibraryWhenCallbackReturns(_: ::winnt::PTP_CALLBACK_INSTANCE, _: ::minwindef::HMODULE); /* threadpoolapiset.h:181:1 */
    pub fn GetApplicationRecoveryCallback(_: ::winnt::HANDLE, _: *mut extern "system" fn(*mut ::libc::c_void) -> ::libc::c_ulong, _: *mut *mut ::libc::c_void, _: ::minwindef::PDWORD, _: ::minwindef::PDWORD) -> ::winnt::HRESULT; /* winbase.h:8151:1 */
    pub fn GetApplicationRestartSettings(_: ::winnt::HANDLE, _: ::winnt::PWSTR, _: ::minwindef::PDWORD, _: ::minwindef::PDWORD) -> ::winnt::HRESULT; /* winbase.h:8162:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetCompressedFileSizeTransactedW as GetCompressedFileSizeTransacted; /* winbase.h:4899:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetCompressedFileSizeTransactedA(_: ::winnt::LPCSTR, _: ::minwindef::LPDWORD, _: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:4885:1 */
    pub fn GetCompressedFileSizeTransactedW(_: ::winnt::LPCWSTR, _: ::minwindef::LPDWORD, _: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:4893:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetConsoleOriginalTitleW as GetConsoleOriginalTitle; /* wincon.h:774:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetConsoleOriginalTitleA(_: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:764:1 */
    pub fn GetConsoleOriginalTitleW(_: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:770:1 */
    pub fn GetDurationFormat(_: ::winnt::LCID, _: ::minwindef::DWORD, _: *const ::minwinbase::SYSTEMTIME, _: ::winnt::ULONGLONG, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1678:1 */
    pub fn GetErrorMode() -> ::minwindef::UINT; /* errhandlingapi.h:146:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetFileAttributesTransactedW as GetFileAttributesTransacted; /* winbase.h:4877:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetFileAttributesTransactedA(_: ::winnt::LPCSTR, _: ::minwinbase::GET_FILEEX_INFO_LEVELS, _: ::minwindef::LPVOID, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4861:1 */
    pub fn GetFileAttributesTransactedW(_: ::winnt::LPCWSTR, _: ::minwinbase::GET_FILEEX_INFO_LEVELS, _: ::minwindef::LPVOID, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4870:1 */
    pub fn GetFileBandwidthReservation(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPBOOL, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:5857:1 */
    pub fn GetFileMUIInfo(_: ::minwindef::DWORD, _: ::winnt::PCWSTR, _: ::winnls::PFILEMUIINFO, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winnls.h:2084:1 */
    pub fn GetFileMUIPath(_: ::minwindef::DWORD, _: ::winnt::PCWSTR, _: ::winnt::PWSTR, _: ::minwindef::PULONG, _: ::winnt::PWSTR, _: ::minwindef::PULONG, _: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* winnls.h:2093:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetFinalPathNameByHandleW as GetFinalPathNameByHandle; /* fileapi.h:694:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetFinalPathNameByHandleA(_: ::winnt::HANDLE, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:676:1 */
    pub fn GetFinalPathNameByHandleW(_: ::winnt::HANDLE, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:686:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetFullPathNameTransactedW as GetFullPathNameTransacted; /* winbase.h:4739:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetFullPathNameTransactedA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: *mut *mut ::libc::c_schar, _: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:4720:1 */
    pub fn GetFullPathNameTransactedW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: *mut *mut ::libc::c_ushort, _: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:4731:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetLongPathNameTransactedW as GetLongPathNameTransacted; /* winbase.h:1200:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetLongPathNameTransactedA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:1183:1 */
    pub fn GetLongPathNameTransactedW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:1193:1 */
    pub fn GetNamedPipeClientComputerNameA(_: ::winnt::HANDLE, _: ::winnt::LPSTR, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winbase.h:5743:1 */
    pub fn GetNamedPipeClientComputerNameW(_: ::winnt::HANDLE, _: ::winnt::LPWSTR, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* namedpipeapi.h:151:1 */
    pub fn GetNamedPipeClientProcessId(_: ::winnt::HANDLE, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winbase.h:5756:1 */
    pub fn GetNamedPipeClientSessionId(_: ::winnt::HANDLE, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winbase.h:5764:1 */
    pub fn GetNamedPipeServerProcessId(_: ::winnt::HANDLE, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winbase.h:5772:1 */
    pub fn GetNamedPipeServerSessionId(_: ::winnt::HANDLE, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winbase.h:5780:1 */
    pub fn GetNumaProximityNode(_: ::minwindef::ULONG, _: ::minwindef::PUCHAR) -> ::minwindef::BOOL; /* winbase.h:8066:1 */
    pub fn GetProcessDEPPolicy(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* winbase.h:1644:1 */
    pub fn GetProcessIdOfThread(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:741:1 */
    pub fn GetProcessPreferredUILanguages(_: ::minwindef::DWORD, _: ::minwindef::PULONG, _: ::winnt::PZZWSTR, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:2021:1 */
    pub fn GetProductInfo(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:515:1 */
    pub fn GetQueuedCompletionStatusEx(_: ::winnt::HANDLE, _: ::minwinbase::LPOVERLAPPED_ENTRY, _: ::minwindef::ULONG, _: ::minwindef::PULONG, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* ioapiset.h:90:1 */
    pub fn GetSystemPreferredUILanguages(_: ::minwindef::DWORD, _: ::minwindef::PULONG, _: ::winnt::PZZWSTR, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:2052:1 */
    pub fn GetThreadPreferredUILanguages(_: ::minwindef::DWORD, _: ::minwindef::PULONG, _: ::winnt::PZZWSTR, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:2063:1 */
    pub fn GetThreadUILanguage() -> ::winnt::LANGID; /* winnls.h:2016:1 */
    pub fn GetUILanguageInfo(_: ::minwindef::DWORD, _: ::winnt::PCZZWSTR, _: ::winnt::PZZWSTR, _: ::minwindef::PDWORD, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winnls.h:2107:1 */
    pub fn GetUserPreferredUILanguages(_: ::minwindef::DWORD, _: ::minwindef::PULONG, _: ::winnt::PZZWSTR, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:2041:1 */
    pub fn GetVolumeInformationByHandleW(_: ::winnt::HANDLE, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:812:1 */
    pub fn InitializeProcThreadAttributeList(_: ::processthreadsapi::LPPROC_THREAD_ATTRIBUTE_LIST, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::basetsd::PSIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:750:1 */
    pub fn IsThreadpoolTimerSet(_: ::winnt::PTP_TIMER) -> ::minwindef::BOOL; /* threadpoolapiset.h:275:1 */
    pub fn LeaveCriticalSectionWhenCallbackReturns(_: ::winnt::PTP_CALLBACK_INSTANCE, _: ::minwinbase::PCRITICAL_SECTION); /* threadpoolapiset.h:172:1 */
    pub fn MapViewOfFileExNuma(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::basetsd::SIZE_T, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* winbase.h:6386:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::MoveFileTransactedW as MoveFileTransacted; /* winbase.h:5442:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn MoveFileTransactedA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winbase::LPPROGRESS_ROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5422:1 */
    pub fn MoveFileTransactedW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winbase::LPPROGRESS_ROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5433:1 */
    pub fn NotifyUILanguageChange(_: ::minwindef::DWORD, _: ::winnt::PCWSTR, _: ::winnt::PCWSTR, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winnls.h:2119:1 */
    pub fn OpenFileById(_: ::winnt::HANDLE, _: ::winbase::LPFILE_ID_DESCRIPTOR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:8490:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::OpenPrivateNamespaceW as OpenPrivateNamespace; /* winbase.h:7238:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn OpenPrivateNamespaceA(_: ::minwindef::LPVOID, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:7230:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::QueryFullProcessImageNameW as QueryFullProcessImageName; /* winbase.h:3243:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn QueryFullProcessImageNameA(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:3227:1 */
    pub fn QueryFullProcessImageNameW(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:3236:1 */
    pub fn QueryIdleProcessorCycleTime(_: ::minwindef::PULONG, _: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:63:1 */
    pub fn QueryProcessAffinityUpdateMode(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:780:1 */
    pub fn QueryProcessCycleTime(_: ::winnt::HANDLE, _: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:54:1 */
    pub fn QueryThreadCycleTime(_: ::winnt::HANDLE, _: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:45:1 */
    pub fn QueryThreadpoolStackInformation(_: ::winnt::PTP_POOL, _: ::winnt::PTP_POOL_STACK_INFORMATION) -> ::minwindef::BOOL; /* threadpoolapiset.h:100:1 */
    pub fn RegisterApplicationRecoveryCallback(_: ::winbase::APPLICATION_RECOVERY_CALLBACK, _: ::winnt::PVOID, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winnt::HRESULT; /* winbase.h:8123:1 */
    pub fn RegisterApplicationRestart(_: ::winnt::PCWSTR, _: ::minwindef::DWORD) -> ::winnt::HRESULT; /* winbase.h:8138:1 */
    pub fn ReleaseMutexWhenCallbackReturns(_: ::winnt::PTP_CALLBACK_INSTANCE, _: ::winnt::HANDLE); /* threadpoolapiset.h:163:1 */
    pub fn ReleaseSemaphoreWhenCallbackReturns(_: ::winnt::PTP_CALLBACK_INSTANCE, _: ::winnt::HANDLE, _: ::minwindef::DWORD); /* threadpoolapiset.h:153:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RemoveDirectoryTransactedW as RemoveDirectoryTransacted; /* winbase.h:4711:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RemoveDirectoryTransactedA(_: ::winnt::LPCSTR, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4699:1 */
    pub fn RemoveDirectoryTransactedW(_: ::winnt::LPCWSTR, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4706:1 */
    pub fn RemoveSecureMemoryCacheCallback(_: ::winnt::PSECURE_MEMORY_CACHE_CALLBACK) -> ::minwindef::BOOL; /* winbase.h:8609:1 */
    pub fn ReplacePartitionUnit(_: ::winnt::PWSTR, _: ::winnt::PWSTR, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winbase.h:8588:1 */
    pub fn SetDynamicTimeZoneInformation(_: *const ::timezoneapi::DYNAMIC_TIME_ZONE_INFORMATION) -> ::minwindef::BOOL; /* timezoneapi.h:141:1 */
    pub fn SetEventWhenCallbackReturns(_: ::winnt::PTP_CALLBACK_INSTANCE, _: ::winnt::HANDLE); /* threadpoolapiset.h:144:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::SetFileAttributesTransactedW as SetFileAttributesTransacted; /* winbase.h:4853:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn SetFileAttributesTransactedA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4839:1 */
    pub fn SetFileAttributesTransactedW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4847:1 */
    pub fn SetFileBandwidthReservation(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:5845:1 */
    pub fn SetFileCompletionNotificationModes(_: ::winnt::HANDLE, _: ::minwindef::UCHAR) -> ::minwindef::BOOL; /* winbase.h:1773:1 */
    pub fn SetFileIoOverlappedRange(_: ::winnt::HANDLE, _: ::minwindef::PUCHAR, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* fileapi.h:1304:1 */
    pub fn SetProcessAffinityUpdateMode(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:771:1 */
    pub fn SetProcessDEPPolicy(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:1637:1 */
    pub fn SetProcessPreferredUILanguages(_: ::minwindef::DWORD, _: ::winnt::PCZZWSTR, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:2032:1 */
    pub fn SetStdHandleEx(_: ::minwindef::DWORD, _: ::winnt::HANDLE, _: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* processenv.h:128:1 */
    pub fn SetThreadPreferredUILanguages(_: ::minwindef::DWORD, _: ::winnt::PCZZWSTR, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:2074:1 */
    pub fn SetThreadpoolStackInformation(_: ::winnt::PTP_POOL, _: ::winnt::PTP_POOL_STACK_INFORMATION) -> ::minwindef::BOOL; /* threadpoolapiset.h:91:1 */
    pub fn SetThreadpoolThreadMaximum(_: ::winnt::PTP_POOL, _: ::minwindef::DWORD); /* threadpoolapiset.h:73:1 */
    pub fn SetThreadpoolThreadMinimum(_: ::winnt::PTP_POOL, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* threadpoolapiset.h:82:1 */
    pub fn SetThreadpoolTimer(_: ::winnt::PTP_TIMER, _: ::minwindef::PFILETIME, _: ::minwindef::DWORD, _: ::minwindef::DWORD); /* threadpoolapiset.h:264:1 */
    pub fn SetThreadpoolTimerEx(_: ::winnt::PTP_TIMER, _: ::minwindef::PFILETIME, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* threadpoolapiset.h:386:1 */
    pub fn SetThreadpoolWait(_: ::winnt::PTP_WAIT, _: ::winnt::HANDLE, _: ::minwindef::PFILETIME); /* threadpoolapiset.h:311:1 */
    pub fn SetThreadpoolWaitEx(_: ::winnt::PTP_WAIT, _: ::winnt::HANDLE, _: ::minwindef::PFILETIME, _: ::winnt::PVOID) -> ::minwindef::BOOL; /* threadpoolapiset.h:397:1 */
    pub fn StartThreadpoolIo(_: ::winnt::PTP_IO); /* threadpoolapiset.h:350:1 */
    pub fn SubmitThreadpoolWork(_: ::winnt::PTP_WORK); /* threadpoolapiset.h:228:1 */
    pub fn TrySubmitThreadpoolCallback(_: ::winnt::PTP_SIMPLE_CALLBACK, _: ::winnt::PVOID, _: ::winnt::PTP_CALLBACK_ENVIRON) -> ::minwindef::BOOL; /* threadpoolapiset.h:207:1 */
    pub fn UnregisterApplicationRecoveryCallback() -> ::winnt::HRESULT; /* winbase.h:8133:1 */
    pub fn UnregisterApplicationRestart() -> ::winnt::HRESULT; /* winbase.h:8146:1 */
    pub fn UpdateProcThreadAttribute(_: ::processthreadsapi::LPPROC_THREAD_ATTRIBUTE_LIST, _: ::minwindef::DWORD, _: ::basetsd::DWORD_PTR, _: ::winnt::PVOID, _: ::basetsd::SIZE_T, _: ::winnt::PVOID, _: ::basetsd::PSIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:791:1 */
    pub fn VirtualAllocExNuma(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* memoryapi.h:632:1 */
    pub fn WaitForThreadpoolIoCallbacks(_: ::winnt::PTP_IO, _: ::minwindef::BOOL); /* threadpoolapiset.h:366:1 */
    pub fn WaitForThreadpoolTimerCallbacks(_: ::winnt::PTP_TIMER, _: ::minwindef::BOOL); /* threadpoolapiset.h:283:1 */
    pub fn WaitForThreadpoolWaitCallbacks(_: ::winnt::PTP_WAIT, _: ::minwindef::BOOL); /* threadpoolapiset.h:321:1 */
    pub fn WaitForThreadpoolWorkCallbacks(_: ::winnt::PTP_WORK, _: ::minwindef::BOOL); /* threadpoolapiset.h:236:1 */
    pub fn Wow64GetThreadContext(_: ::winnt::HANDLE, _: ::winnt::PWOW64_CONTEXT) -> ::minwindef::BOOL; /* winbase.h:1811:1 */
    pub fn Wow64SetThreadContext(_: ::winnt::HANDLE, _: *const ::winnt::WOW64_CONTEXT) -> ::minwindef::BOOL; /* winbase.h:1819:1 */
    pub fn Wow64SuspendThread(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:1846:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn AcquireSRWLockExclusive(_: ::synchapi::PSRWLOCK); /* synchapi.h:104:1 */
    pub fn AcquireSRWLockShared(_: ::synchapi::PSRWLOCK); /* synchapi.h:113:1 */
    pub fn CompareStringEx(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPCWCH, _: ::libc::c_int, _: ::winnt::LPCWCH, _: ::libc::c_int, _: ::winnls::LPNLSVERSIONINFO, _: ::minwindef::LPVOID, _: ::minwindef::LPARAM) -> ::libc::c_int; /* stringapiset.h:46:1 */
    pub fn CompareStringOrdinal(_: ::winnt::LPCWCH, _: ::libc::c_int, _: ::winnt::LPCWCH, _: ::libc::c_int, _: ::minwindef::BOOL) -> ::libc::c_int; /* stringapiset.h:62:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateEventExW as CreateEventEx; /* synchapi.h:800:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateEventExA(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* synchapi.h:781:1 */
    pub fn CreateEventExW(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* synchapi.h:792:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateMutexExW as CreateMutexEx; /* synchapi.h:769:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateMutexExA(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* synchapi.h:750:1 */
    pub fn CreateMutexExW(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* synchapi.h:761:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateSemaphoreExW as CreateSemaphoreEx; /* synchapi.h:820:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateSemaphoreExW(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::LONG, _: ::winnt::LONG, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* synchapi.h:809:1 */
    pub fn EnumCalendarInfoExEx(_: ::winnls::CALINFO_ENUMPROCEXEX, _: ::winnt::LPCWSTR, _: ::winnls::CALID, _: ::winnt::LPCWSTR, _: ::winnls::CALTYPE, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winnls.h:2516:1 */
    pub fn EnumDateFormatsExEx(_: ::winnls::DATEFMT_ENUMPROCEXEX, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winnls.h:2530:1 */
    pub fn EnumSystemLocalesEx(_: ::winnls::LOCALE_ENUMPROCEX, _: ::minwindef::DWORD, _: ::minwindef::LPARAM, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winnls.h:2554:1 */
    pub fn EnumTimeFormatsEx(_: ::winnls::TIMEFMT_ENUMPROCEX, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winnls.h:2542:1 */
    pub fn FindNLSStringEx(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::minwindef::LPINT, _: ::winnls::LPNLSVERSIONINFO, _: ::minwindef::LPVOID, _: ::minwindef::LPARAM) -> ::libc::c_int; /* winnls.h:2469:1 */
    pub fn FlsAlloc(_: ::winnt::PFLS_CALLBACK_FUNCTION) -> ::minwindef::DWORD; /* fibersapi.h:58:1 */
    pub fn FlsFree(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fibersapi.h:83:1 */
    pub fn FlsGetValue(_: ::minwindef::DWORD) -> ::winnt::PVOID; /* fibersapi.h:66:1 */
    pub fn FlsSetValue(_: ::minwindef::DWORD, _: ::winnt::PVOID) -> ::minwindef::BOOL; /* fibersapi.h:74:1 */
    pub fn FlushProcessWriteBuffers(); /* processthreadsapi.h:726:1 */
    pub fn GetCalendarInfoEx(_: ::winnt::LPCWSTR, _: ::winnls::CALID, _: ::winnt::LPCWSTR, _: ::winnls::CALTYPE, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::minwindef::LPDWORD) -> ::libc::c_int; /* winnls.h:2383:1 */
    pub fn GetCurrencyFormatEx(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: *const ::winnls::CURRENCYFMTW, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2421:1 */
    pub fn GetDurationFormatEx(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: *const ::minwinbase::SYSTEMTIME, _: ::winnt::ULONGLONG, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2396:1 */
    pub fn GetDynamicTimeZoneInformation(_: ::timezoneapi::PDYNAMIC_TIME_ZONE_INFORMATION) -> ::minwindef::DWORD; /* timezoneapi.h:162:1 */
    pub fn GetFileInformationByHandleEx(_: ::winnt::HANDLE, _: ::minwinbase::FILE_INFO_BY_HANDLE_CLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:8455:1 */
    pub fn GetLocaleInfoEx(_: ::winnt::LPCWSTR, _: ::winnls::LCTYPE, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2373:1 */
    pub fn GetNLSVersionEx(_: ::winnls::NLS_FUNCTION, _: ::winnt::LPCWSTR, _: ::winnls::LPNLSVERSIONINFOEX) -> ::minwindef::BOOL; /* winnls.h:2449:1 */
    pub fn GetNumberFormatEx(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: *const ::winnls::NUMBERFMTW, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2409:1 */
    pub fn GetStringScripts(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2345:8 */
    pub fn GetSystemDefaultLocaleName(_: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2441:1 */
    pub fn GetTickCount64() -> ::winnt::ULONGLONG; /* sysinfoapi.h:221:1 */
    pub fn GetUserDefaultLocaleName(_: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2433:1 */
    pub fn InitOnceBeginInitialize(_: ::synchapi::LPINIT_ONCE, _: ::minwindef::DWORD, _: ::minwindef::PBOOL, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* synchapi.h:337:1 */
    pub fn InitOnceComplete(_: ::synchapi::LPINIT_ONCE, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* synchapi.h:348:1 */
    pub fn InitOnceExecuteOnce(_: ::synchapi::PINIT_ONCE, _: ::synchapi::PINIT_ONCE_FN, _: ::winnt::PVOID, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* synchapi.h:326:1 */
    pub fn InitOnceInitialize(_: ::synchapi::PINIT_ONCE); /* synchapi.h:318:1 */
    pub fn InitializeConditionVariable(_: ::synchapi::PCONDITION_VARIABLE); /* synchapi.h:381:1 */
    pub fn InitializeCriticalSectionEx(_: ::minwinbase::LPCRITICAL_SECTION, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:222:1 */
    pub fn InitializeSRWLock(_: ::synchapi::PSRWLOCK); /* synchapi.h:77:1 */
    pub fn IsNormalizedString(_: ::winnls::NORM_FORM, _: ::winnt::LPCWSTR, _: ::libc::c_int) -> ::minwindef::BOOL; /* winnls.h:2303:8 */
    pub fn IsThreadAFiber() -> ::minwindef::BOOL; /* fibersapi.h:107:1 */
    pub fn IsValidLocaleName(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winnls.h:2507:1 */
    pub fn LCIDToLocaleName(_: ::winnt::LCID, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::minwindef::DWORD) -> ::libc::c_int; /* winnls.h:1652:1 */
    pub fn LCMapStringEx(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::winnls::LPNLSVERSIONINFO, _: ::minwindef::LPVOID, _: ::minwindef::LPARAM) -> ::libc::c_int; /* winnls.h:2492:1 */
    pub fn LocaleNameToLCID(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::winnt::LCID; /* winnls.h:1661:1 */
    pub fn NormalizeString(_: ::winnls::NORM_FORM, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2295:8 */
    pub fn ReleaseSRWLockExclusive(_: ::synchapi::PSRWLOCK); /* synchapi.h:86:1 */
    pub fn ReleaseSRWLockShared(_: ::synchapi::PSRWLOCK); /* synchapi.h:95:1 */
    pub fn SetFileInformationByHandle(_: ::winnt::HANDLE, _: ::minwinbase::FILE_INFO_BY_HANDLE_CLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1037:1 */
    pub fn SleepConditionVariableCS(_: ::synchapi::PCONDITION_VARIABLE, _: ::minwinbase::PCRITICAL_SECTION, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:405:1 */
    pub fn SleepConditionVariableSRW(_: ::synchapi::PCONDITION_VARIABLE, _: ::synchapi::PSRWLOCK, _: ::minwindef::DWORD, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* synchapi.h:415:1 */
    pub fn TryAcquireSRWLockExclusive(_: ::synchapi::PSRWLOCK) -> ::winnt::BOOLEAN; /* synchapi.h:122:1 */
    pub fn TryAcquireSRWLockShared(_: ::synchapi::PSRWLOCK) -> ::winnt::BOOLEAN; /* synchapi.h:131:1 */
    pub fn VerifyScripts(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPCWSTR, _: ::libc::c_int) -> ::minwindef::BOOL; /* winnls.h:2336:8 */
    pub fn WakeAllConditionVariable(_: ::synchapi::PCONDITION_VARIABLE); /* synchapi.h:397:1 */
    pub fn WakeConditionVariable(_: ::synchapi::PCONDITION_VARIABLE); /* synchapi.h:389:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn CreateUmsCompletionList(_: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winbase.h:1449:1 */
    pub fn CreateUmsThreadContext(_: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winbase.h:1536:1 */
    pub fn DeleteUmsCompletionList(_: ::winbase::PUMS_COMPLETION_LIST) -> ::minwindef::BOOL; /* winbase.h:1487:1 */
    pub fn DeleteUmsThreadContext(_: ::winbase::PUMS_CONTEXT) -> ::minwindef::BOOL; /* winbase.h:1529:1 */
    pub fn DequeueUmsCompletionListItems(_: ::winbase::PUMS_COMPLETION_LIST, _: ::minwindef::DWORD, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winbase.h:1456:1 */
    pub fn DisableThreadProfiling(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:8723:1 */
    pub fn EnableThreadProfiling(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::basetsd::DWORD64, _: *mut *mut ::libc::c_void) -> ::minwindef::DWORD; /* winbase.h:8713:1 */
    pub fn EnterUmsSchedulingMode(_: ::winbase::PUMS_SCHEDULER_STARTUP_INFO) -> ::minwindef::BOOL; /* winbase.h:1543:1 */
    pub fn ExecuteUmsThread(_: ::winbase::PUMS_CONTEXT) -> ::minwindef::BOOL; /* winbase.h:1473:1 */
    pub fn GetActiveProcessorCount(_: ::minwindef::WORD) -> ::minwindef::DWORD; /* winbase.h:7984:1 */
    pub fn GetActiveProcessorGroupCount() -> ::minwindef::WORD; /* winbase.h:7970:1 */
    pub fn GetCurrentProcessorNumberEx(_: ::winnt::PPROCESSOR_NUMBER); /* processthreadsapi.h:1041:1 */
    pub fn GetCurrentUmsThread() -> ::winbase::PUMS_CONTEXT; /* winbase.h:1494:1 */
    pub fn GetLogicalProcessorInformationEx(_: ::winnt::LOGICAL_PROCESSOR_RELATIONSHIP, _: ::winnt::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:467:1 */
    pub fn GetMaximumProcessorCount(_: ::minwindef::WORD) -> ::minwindef::DWORD; /* winbase.h:7991:1 */
    pub fn GetMaximumProcessorGroupCount() -> ::minwindef::WORD; /* winbase.h:7977:1 */
    pub fn GetNextUmsListItem(_: ::winbase::PUMS_CONTEXT) -> ::winbase::PUMS_CONTEXT; /* winbase.h:1501:1 */
    pub fn GetNumaAvailableMemoryNodeEx(_: ::minwindef::USHORT, _: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* winbase.h:8054:1 */
    pub fn GetNumaNodeNumberFromHandle(_: ::winnt::HANDLE, _: ::minwindef::PUSHORT) -> ::minwindef::BOOL; /* winbase.h:8014:1 */
    pub fn GetNumaNodeProcessorMaskEx(_: ::minwindef::USHORT, _: ::winnt::PGROUP_AFFINITY) -> ::minwindef::BOOL; /* systemtopologyapi.h:54:1 */
    pub fn GetNumaProcessorNodeEx(_: ::winnt::PPROCESSOR_NUMBER, _: ::minwindef::PUSHORT) -> ::minwindef::BOOL; /* winbase.h:8026:1 */
    pub fn GetNumaProximityNodeEx(_: ::minwindef::ULONG, _: ::minwindef::PUSHORT) -> ::minwindef::BOOL; /* winbase.h:8078:1 */
    pub fn GetProcessGroupAffinity(_: ::winnt::HANDLE, _: ::minwindef::PUSHORT, _: ::minwindef::PUSHORT) -> ::minwindef::BOOL; /* processtopologyapi.h:49:1 */
    pub fn GetProcessorSystemCycleTime(_: ::minwindef::USHORT, _: ::winnt::PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:1120:1 */
    pub fn GetThreadGroupAffinity(_: ::winnt::HANDLE, _: ::winnt::PGROUP_AFFINITY) -> ::minwindef::BOOL; /* processtopologyapi.h:72:1 */
    pub fn GetThreadIdealProcessorEx(_: ::winnt::HANDLE, _: ::winnt::PPROCESSOR_NUMBER) -> ::minwindef::BOOL; /* processthreadsapi.h:1032:1 */
    pub fn GetUmsCompletionListEvent(_: ::winbase::PUMS_COMPLETION_LIST, _: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* winbase.h:1465:1 */
    pub fn GetUmsSystemThreadInformation(_: ::winnt::HANDLE, _: ::winbase::PUMS_SYSTEM_THREAD_INFORMATION) -> ::minwindef::BOOL; /* winbase.h:1550:1 */
    pub fn PowerClearRequest(_: ::winnt::HANDLE, _: ::winnt::POWER_REQUEST_TYPE) -> ::minwindef::BOOL; /* winbase.h:1708:1 */
    pub fn PowerCreateRequest(_: ::minwinbase::PREASON_CONTEXT) -> ::winnt::HANDLE; /* winbase.h:1693:1 */
    pub fn PowerSetRequest(_: ::winnt::HANDLE, _: ::winnt::POWER_REQUEST_TYPE) -> ::minwindef::BOOL; /* winbase.h:1700:1 */
    pub fn QueryIdleProcessorCycleTimeEx(_: ::minwindef::USHORT, _: ::minwindef::PULONG, _: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:77:1 */
    pub fn QueryThreadProfiling(_: ::winnt::HANDLE, _: ::winnt::PBOOLEAN) -> ::minwindef::DWORD; /* winbase.h:8730:1 */
    pub fn QueryUmsThreadInformation(_: ::winbase::PUMS_CONTEXT, _: ::winbase::UMS_THREAD_INFO_CLASS, _: ::winnt::PVOID, _: ::minwindef::ULONG, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winbase.h:1508:1 */
    pub fn ReadThreadProfilingData(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winnt::PPERFORMANCE_DATA) -> ::minwindef::DWORD; /* winbase.h:8738:1 */
    pub fn SetThreadGroupAffinity(_: ::winnt::HANDLE, _: *const ::winnt::GROUP_AFFINITY, _: ::winnt::PGROUP_AFFINITY) -> ::minwindef::BOOL; /* processtopologyapi.h:81:1 */
    pub fn SetUmsThreadInformation(_: ::winbase::PUMS_CONTEXT, _: ::winbase::UMS_THREAD_INFO_CLASS, _: ::winnt::PVOID, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winbase.h:1519:1 */
    pub fn SetWaitableTimerEx(_: ::winnt::HANDLE, _: *const ::winnt::LARGE_INTEGER, _: ::winnt::LONG, _: ::synchapi::PTIMERAPCROUTINE, _: ::minwindef::LPVOID, _: ::minwinbase::PREASON_CONTEXT, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* synchapi.h:700:1 */
    pub fn UmsThreadYield(_: ::winnt::PVOID) -> ::minwindef::BOOL; /* winbase.h:1480:1 */
    pub fn Wow64GetThreadSelectorEntry(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winnt::PWOW64_LDT_ENTRY) -> ::minwindef::BOOL; /* winbase.h:1831:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn CopyFile2(_: ::winnt::PCWSTR, _: ::winnt::PCWSTR, _: *mut ::winbase::COPYFILE2_EXTENDED_PARAMETERS) -> ::winnt::HRESULT; /* winbase.h:5300:1 */
    pub fn FindStringOrdinal(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::minwindef::BOOL) -> ::libc::c_int; /* libloaderapi.h:198:1 */
    pub fn GetTimeZoneInformationForYear(_: ::minwindef::USHORT, _: ::timezoneapi::PDYNAMIC_TIME_ZONE_INFORMATION, _: ::timezoneapi::LPTIME_ZONE_INFORMATION) -> ::minwindef::BOOL; /* timezoneapi.h:175:1 */
    pub fn QueryUnbiasedInterruptTime(_: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* realtimeapiset.h:99:1 */
    pub fn ResolveLocaleName(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2574:1 */
    pub fn SetThreadIdealProcessorEx(_: ::winnt::HANDLE, _: ::winnt::PPROCESSOR_NUMBER, _: ::winnt::PPROCESSOR_NUMBER) -> ::minwindef::BOOL; /* processthreadsapi.h:1015:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010100"))] #[cfg(any(target_arch="x86", target_arch="x86_64"))] 
extern "system" {
    pub fn SetXStateFeaturesMask(_: ::winnt::PCONTEXT, _: ::basetsd::DWORD64) -> ::minwindef::BOOL; /* winbase.h:8693:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010100"))] 
extern "system" {
    pub fn CopyContext(_: ::winnt::PCONTEXT, _: ::minwindef::DWORD, _: ::winnt::PCONTEXT) -> ::minwindef::BOOL; /* winbase.h:8627:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010100"))] #[cfg(any(target_arch="x86", target_arch="x86_64"))] 
extern "system" {
    pub fn GetEnabledXStateFeatures() -> ::basetsd::DWORD64; /* winbase.h:8660:1 */
    pub fn GetXStateFeaturesMask(_: ::winnt::PCONTEXT, _: ::basetsd::PDWORD64) -> ::minwindef::BOOL; /* winbase.h:8668:1 */
    pub fn LocateXStateFeature(_: ::winnt::PCONTEXT, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::winnt::PVOID; /* winbase.h:8677:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010100"))] 
extern "system" {
    pub fn InitializeContext(_: ::winnt::PVOID, _: ::minwindef::DWORD, _: *mut *mut ::winnt::CONTEXT, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:8643:1 */
}
#[cfg(any(feature="winapi_ver_06020000"))] #[doc(inline)] pub use self::InterlockedPushListSListEx as InterlockedPushListSList; /* interlockedapi.h:75:9 */
#[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn InterlockedPushListSListEx(_: ::winnt::PSLIST_HEADER, _: ::winnt::PSLIST_ENTRY, _: ::winnt::PSLIST_ENTRY, _: ::minwindef::ULONG) -> ::winnt::PSLIST_ENTRY; /* interlockedapi.h:80:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn AddResourceAttributeAce(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PSID, _: ::winnt::PCLAIM_SECURITY_ATTRIBUTES_INFORMATION, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:362:1 */
    pub fn AddScopedPolicyIDAce(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:376:1 */
    pub fn CheckTokenCapability(_: ::winnt::HANDLE, _: ::winnt::PSID, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:477:1 */
    pub fn CheckTokenMembershipEx(_: ::winnt::HANDLE, _: ::winnt::PSID, _: ::minwindef::DWORD, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:498:1 */
    pub fn GetAppContainerAce(_: ::winnt::PACL, _: ::minwindef::DWORD, _: *mut *mut ::libc::c_void, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* securitybaseapi.h:487:1 */
    pub fn GetAppContainerNamedObjectPath(_: ::winnt::HANDLE, _: ::winnt::PSID, _: ::minwindef::ULONG, _: ::winnt::LPWSTR, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* securityappcontainer.h:51:1 */
    pub fn GetCachedSigningLevel(_: ::winnt::HANDLE, _: ::minwindef::PULONG, _: ::minwindef::PULONG, _: ::minwindef::PUCHAR, _: ::minwindef::PULONG, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* securitybaseapi.h:1318:1 */
    pub fn GetCurrentThreadStackLimits(_: ::basetsd::PULONG_PTR, _: ::basetsd::PULONG_PTR); /* processthreadsapi.h:848:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[doc(inline)] pub use self::GetFirmwareEnvironmentVariableExW as GetFirmwareEnvironmentVariableEx; /* winbase.h:3582:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn GetFirmwareEnvironmentVariableExA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::PVOID, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::DWORD; /* winbase.h:3564:1 */
    pub fn GetFirmwareEnvironmentVariableExW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::PVOID, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::DWORD; /* winbase.h:3574:1 */
    pub fn GetFirmwareType(_: ::winnt::PFIRMWARE_TYPE) -> ::minwindef::BOOL; /* winbase.h:3648:1 */
    pub fn GetMemoryErrorHandlingCapabilities(_: ::minwindef::PULONG) -> ::minwindef::BOOL; /* memoryapi.h:653:1 */
    pub fn GetProcessInformation(_: ::winnt::HANDLE, _: ::winbase::PROCESS_INFORMATION_CLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:1610:1 */
    pub fn GetProcessMitigationPolicy(_: ::winnt::HANDLE, _: ::winnt::PROCESS_MITIGATION_POLICY, _: ::winnt::PVOID, _: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:890:1 */
    pub fn GetThreadInformation(_: ::winnt::HANDLE, _: ::processthreadsapi::THREAD_INFORMATION_CLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:1119:1 */
    pub fn IsNativeVhdBoot(_: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* winbase.h:3656:1 */
    pub fn PrefetchVirtualMemory(_: ::winnt::HANDLE, _: ::basetsd::ULONG_PTR, _: ::memoryapi::PWIN32_MEMORY_RANGE_ENTRY, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* memoryapi.h:504:1 */
    pub fn RegisterBadMemoryNotification(_: ::memoryapi::PBAD_MEMORY_CALLBACK_ROUTINE) -> ::winnt::PVOID; /* memoryapi.h:672:1 */
    pub fn SetCachedSigningLevel(_: ::winnt::PHANDLE, _: ::minwindef::ULONG, _: ::minwindef::ULONG, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:1307:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[doc(inline)] pub use self::SetFirmwareEnvironmentVariableExW as SetFirmwareEnvironmentVariableEx; /* winbase.h:3636:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn SetFirmwareEnvironmentVariableExA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::PVOID, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3618:1 */
    pub fn SetFirmwareEnvironmentVariableExW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::PVOID, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3628:1 */
    pub fn SetProcessInformation(_: ::winnt::HANDLE, _: ::winbase::PROCESS_INFORMATION_CLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:1620:1 */
    pub fn SetProcessMitigationPolicy(_: ::winnt::PROCESS_MITIGATION_POLICY, _: ::winnt::PVOID, _: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:880:1 */
    pub fn SetThreadInformation(_: ::winnt::HANDLE, _: ::processthreadsapi::THREAD_INFORMATION_CLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:1130:1 */
    pub fn UnmapViewOfFileEx(_: ::winnt::PVOID, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* memoryapi.h:515:1 */
    pub fn UnregisterBadMemoryNotification(_: ::winnt::PVOID) -> ::minwindef::BOOL; /* memoryapi.h:681:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn CreateFile2(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::fileapi::LPCREATEFILE2_EXTENDED_PARAMETERS) -> ::winnt::HANDLE; /* fileapi.h:1272:1 */
    pub fn CreateFileMappingFromApp(_: ::winnt::HANDLE, _: ::minwinbase::PSECURITY_ATTRIBUTES, _: ::minwindef::ULONG, _: ::basetsd::ULONG64, _: ::winnt::PCWSTR) -> ::winnt::HANDLE; /* memoryapi.h:537:1 */
    pub fn IsValidNLSVersion(_: ::winnls::NLS_FUNCTION, _: ::winnt::LPCWSTR, _: ::winnls::LPNLSVERSIONINFOEX) -> ::minwindef::DWORD; /* winnls.h:2459:1 */
    pub fn LoadPackagedLibrary(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::minwindef::HMODULE; /* winbase.h:3207:1 */
    pub fn MapViewOfFileFromApp(_: ::winnt::HANDLE, _: ::minwindef::ULONG, _: ::basetsd::ULONG64, _: ::basetsd::SIZE_T) -> ::winnt::PVOID; /* memoryapi.h:550:1 */
    pub fn SystemTimeToTzSpecificLocalTimeEx(_: *const ::timezoneapi::DYNAMIC_TIME_ZONE_INFORMATION, _: *const ::minwinbase::SYSTEMTIME, _: ::minwinbase::LPSYSTEMTIME) -> ::minwindef::BOOL; /* timezoneapi.h:214:1 */
    pub fn TzSpecificLocalTimeToSystemTimeEx(_: *const ::timezoneapi::DYNAMIC_TIME_ZONE_INFORMATION, _: *const ::minwinbase::SYSTEMTIME, _: ::minwinbase::LPSYSTEMTIME) -> ::minwindef::BOOL; /* timezoneapi.h:225:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_0a000000"))] 
extern "system" {
    pub fn QueryProtectedPolicy(_: ::guiddef::LPCGUID, _: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* processthreadsapi.h:1185:1 */
    pub fn SetProtectedPolicy(_: ::guiddef::LPCGUID, _: ::basetsd::ULONG_PTR, _: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* processthreadsapi.h:1175:1 */
}
