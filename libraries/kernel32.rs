
extern "system" {
    pub fn InitializeSListHead(ListHead: ::winnt::PSLIST_HEADER); /* interlockedapi.h:50:1 */
    pub fn InterlockedFlushSList(ListHead: ::winnt::PSLIST_HEADER) -> ::winnt::PSLIST_ENTRY; /* interlockedapi.h:93:1 */
    pub fn InterlockedPopEntrySList(ListHead: ::winnt::PSLIST_HEADER) -> ::winnt::PSLIST_ENTRY; /* interlockedapi.h:58:1 */
    pub fn InterlockedPushEntrySList(ListHead: ::winnt::PSLIST_HEADER, ListEntry: ::winnt::PSLIST_ENTRY) -> ::winnt::PSLIST_ENTRY; /* interlockedapi.h:66:1 */
    pub fn QueryDepthSList(ListHead: ::winnt::PSLIST_HEADER) -> ::minwindef::USHORT; /* interlockedapi.h:101:1 */
    pub fn QueryPerformanceCounter(lpPerformanceCount: *mut ::winnt::LARGE_INTEGER) -> ::minwindef::BOOL; /* profileapi.h:42:1 */
    pub fn QueryPerformanceFrequency(lpFrequency: *mut ::winnt::LARGE_INTEGER) -> ::minwindef::BOOL; /* profileapi.h:50:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::lstrcmpW as ua_lstrcmpW; /* stralign.h:95:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::lstrcmpiW as ua_lstrcmpiW; /* stralign.h:94:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::lstrlenW as ua_lstrlenW; /* stralign.h:96:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn GetWriteWatch(dwFlags: ::minwindef::DWORD, lpBaseAddress: ::winnt::PVOID, dwRegionSize: ::basetsd::SIZE_T, lpAddresses: *mut *mut ::libc::c_void, lpdwCount: *mut ::libc::c_ulonglong, lpdwGranularity: ::minwindef::LPDWORD) -> ::minwindef::UINT; /* memoryapi.h:389:1 */
    pub fn ReadProcessMemory(hProcess: ::winnt::HANDLE, lpBaseAddress: ::minwindef::LPCVOID, lpBuffer: ::minwindef::LPVOID, nSize: ::basetsd::SIZE_T, lpNumberOfBytesRead: *mut ::libc::c_ulonglong) -> ::minwindef::BOOL; /* memoryapi.h:217:1 */
    pub fn RtlAddFunctionTable(FunctionTable: ::winnt::PRUNTIME_FUNCTION, EntryCount: ::minwindef::DWORD, BaseAddress: ::basetsd::DWORD64) -> ::winnt::BOOLEAN; /* winnt.h:16974:1 */
    pub fn RtlInstallFunctionTableCallback(TableIdentifier: ::basetsd::DWORD64, BaseAddress: ::basetsd::DWORD64, Length: ::minwindef::DWORD, Callback: ::winnt::PGET_RUNTIME_FUNCTION_CALLBACK, Context: ::winnt::PVOID, OutOfProcessCallbackDll: ::winnt::PCWSTR) -> ::winnt::BOOLEAN; /* winnt.h:16992:1 */
    pub fn RtlVirtualUnwind(HandlerType: ::minwindef::DWORD, ImageBase: ::basetsd::DWORD64, ControlPc: ::basetsd::DWORD64, FunctionEntry: ::winnt::PRUNTIME_FUNCTION, ContextRecord: ::winnt::PCONTEXT, HandlerData: *mut *mut ::libc::c_void, EstablisherFrame: ::basetsd::PDWORD64, ContextPointers: ::winnt::PKNONVOLATILE_CONTEXT_POINTERS) -> ::winnt::PEXCEPTION_ROUTINE; /* winnt.h:17104:1 */
    pub fn WriteProcessMemory(hProcess: ::winnt::HANDLE, lpBaseAddress: ::minwindef::LPVOID, lpBuffer: ::minwindef::LPCVOID, nSize: ::basetsd::SIZE_T, lpNumberOfBytesWritten: *mut ::libc::c_ulonglong) -> ::minwindef::BOOL; /* memoryapi.h:230:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn RtlAddFunctionTable(FunctionTable: ::winnt::PRUNTIME_FUNCTION, EntryCount: ::minwindef::DWORD, BaseAddress: ::minwindef::DWORD) -> ::winnt::BOOLEAN; /* winnt.h:17132:1 */
    pub fn RtlInstallFunctionTableCallback(TableIdentifier: ::minwindef::DWORD, BaseAddress: ::minwindef::DWORD, Length: ::minwindef::DWORD, Callback: ::winnt::PGET_RUNTIME_FUNCTION_CALLBACK, Context: ::winnt::PVOID, OutOfProcessCallbackDll: ::winnt::PCWSTR) -> ::winnt::BOOLEAN; /* winnt.h:17150:1 */
    pub fn RtlVirtualUnwind(HandlerType: ::minwindef::DWORD, ImageBase: ::minwindef::DWORD, ControlPc: ::minwindef::DWORD, FunctionEntry: ::winnt::PRUNTIME_FUNCTION, ContextRecord: ::winnt::PCONTEXT, HandlerData: *mut *mut ::libc::c_void, EstablisherFrame: ::minwindef::PDWORD, ContextPointers: ::winnt::PKNONVOLATILE_CONTEXT_POINTERS) -> ::winnt::PEXCEPTION_ROUTINE; /* winnt.h:17262:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] 
extern "system" {
    pub fn GetWriteWatch(dwFlags: ::minwindef::DWORD, lpBaseAddress: ::winnt::PVOID, dwRegionSize: ::basetsd::SIZE_T, lpAddresses: *mut *mut ::libc::c_void, lpdwCount: *mut ::libc::c_ulong, lpdwGranularity: ::minwindef::LPDWORD) -> ::minwindef::UINT; /* memoryapi.h:389:1 */
    pub fn ReadProcessMemory(hProcess: ::winnt::HANDLE, lpBaseAddress: ::minwindef::LPCVOID, lpBuffer: ::minwindef::LPVOID, nSize: ::basetsd::SIZE_T, lpNumberOfBytesRead: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* memoryapi.h:217:1 */
    pub fn WriteProcessMemory(hProcess: ::winnt::HANDLE, lpBaseAddress: ::minwindef::LPVOID, lpBuffer: ::minwindef::LPCVOID, nSize: ::basetsd::SIZE_T, lpNumberOfBytesWritten: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* memoryapi.h:230:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64", target_arch="arm"))] 
extern "system" {
    pub fn RtlDeleteFunctionTable(FunctionTable: ::winnt::PRUNTIME_FUNCTION) -> ::winnt::BOOLEAN; /* winnt.h:16984:1, winnt.h:17142:1 */
    pub fn RtlRestoreContext(ContextRecord: ::winnt::PCONTEXT, ExceptionRecord: *mut ::winnt::EXCEPTION_RECORD); /* winnt.h:17068:1, winnt.h:17226:1 */
    pub fn uaw_lstrcmpW(String1: ::winnt::PCUWSTR, String2: ::winnt::PCUWSTR) -> ::libc::c_int; /* stralign.h:159:1 */
    pub fn uaw_lstrcmpiW(String1: ::winnt::PCUWSTR, String2: ::winnt::PCUWSTR) -> ::libc::c_int; /* stralign.h:166:1 */
    pub fn uaw_lstrlenW(String: ::winnt::LPCUWSTR) -> ::libc::c_int; /* stralign.h:173:1 */
    pub fn uaw_wcschr(String: ::winnt::PCUWSTR, Character: ::winnt::WCHAR) -> ::winnt::PUWSTR; /* stralign.h:179:1 */
    pub fn uaw_wcscpy(Destination: ::winnt::PUWSTR, Source: ::winnt::PCUWSTR) -> ::winnt::PUWSTR; /* stralign.h:186:1 */
    pub fn uaw_wcsicmp(String1: ::winnt::PCUWSTR, String2: ::winnt::PCUWSTR) -> ::libc::c_int; /* stralign.h:193:1 */
    pub fn uaw_wcsrchr(String: ::winnt::PCUWSTR, Character: ::winnt::WCHAR) -> ::winnt::PUWSTR; /* stralign.h:206:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AddAtomW as AddAtom; /* winbase.h:3933:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AddAtomA(lpString: ::winnt::LPCSTR) -> ::minwindef::ATOM; /* winbase.h:3923:1 */
    pub fn AddAtomW(lpString: ::winnt::LPCWSTR) -> ::minwindef::ATOM; /* winbase.h:3929:1 */
    pub fn AddDllDirectory(NewDirectory: ::winnt::PCWSTR) -> ::libloaderapi::DLL_DIRECTORY_COOKIE; /* libloaderapi.h:498:1 */
    pub fn AddSIDToBoundaryDescriptor(BoundaryDescriptor: *mut *mut ::libc::c_void, RequiredSid: ::winnt::PSID) -> ::minwindef::BOOL; /* namespaceapi.h:82:1 */
    pub fn AllocConsole() -> ::minwindef::BOOL; /* consoleapi.h:44:1 */
    pub fn AreFileApisANSI() -> ::minwindef::BOOL; /* winbase.h:5820:1 */
    pub fn BackupRead(hFile: ::winnt::HANDLE, lpBuffer: ::minwindef::LPBYTE, nNumberOfBytesToRead: ::minwindef::DWORD, lpNumberOfBytesRead: ::minwindef::LPDWORD, bAbort: ::minwindef::BOOL, bProcessSecurity: ::minwindef::BOOL, lpContext: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winbase.h:2862:1 */
    pub fn BackupSeek(hFile: ::winnt::HANDLE, dwLowBytesToSeek: ::minwindef::DWORD, dwHighBytesToSeek: ::minwindef::DWORD, lpdwLowByteSeeked: ::minwindef::LPDWORD, lpdwHighByteSeeked: ::minwindef::LPDWORD, lpContext: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winbase.h:2875:1 */
    pub fn BackupWrite(hFile: ::winnt::HANDLE, lpBuffer: ::minwindef::LPBYTE, nNumberOfBytesToWrite: ::minwindef::DWORD, lpNumberOfBytesWritten: ::minwindef::LPDWORD, bAbort: ::minwindef::BOOL, bProcessSecurity: ::minwindef::BOOL, lpContext: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winbase.h:2887:1 */
    pub fn Beep(dwFreq: ::minwindef::DWORD, dwDuration: ::minwindef::DWORD) -> ::minwindef::BOOL; /* utilapiset.h:85:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::BeginUpdateResourceW as BeginUpdateResource; /* winbase.h:3787:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn BeginUpdateResourceA(pFileName: ::winnt::LPCSTR, bDeleteExistingResources: ::minwindef::BOOL) -> ::winnt::HANDLE; /* winbase.h:3775:1 */
    pub fn BeginUpdateResourceW(pFileName: ::winnt::LPCWSTR, bDeleteExistingResources: ::minwindef::BOOL) -> ::winnt::HANDLE; /* winbase.h:3782:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::BuildCommDCBW as BuildCommDCB; /* winbase.h:6686:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn BuildCommDCBA(lpDef: ::winnt::LPCSTR, lpDCB: ::winbase::LPDCB) -> ::minwindef::BOOL; /* winbase.h:6674:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::BuildCommDCBAndTimeoutsW as BuildCommDCBAndTimeouts; /* winbase.h:6708:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn BuildCommDCBAndTimeoutsA(lpDef: ::winnt::LPCSTR, lpDCB: ::winbase::LPDCB, lpCommTimeouts: ::winbase::LPCOMMTIMEOUTS) -> ::minwindef::BOOL; /* winbase.h:6694:1 */
    pub fn BuildCommDCBAndTimeoutsW(lpDef: ::winnt::LPCWSTR, lpDCB: ::winbase::LPDCB, lpCommTimeouts: ::winbase::LPCOMMTIMEOUTS) -> ::minwindef::BOOL; /* winbase.h:6702:1 */
    pub fn BuildCommDCBW(lpDef: ::winnt::LPCWSTR, lpDCB: ::winbase::LPDCB) -> ::minwindef::BOOL; /* winbase.h:6681:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CallNamedPipeW as CallNamedPipe; /* winbase.h:5721:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CallNamedPipeA(lpNamedPipeName: ::winnt::LPCSTR, lpInBuffer: ::minwindef::LPVOID, nInBufferSize: ::minwindef::DWORD, lpOutBuffer: ::minwindef::LPVOID, nOutBufferSize: ::minwindef::DWORD, lpBytesRead: ::minwindef::LPDWORD, nTimeOut: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5699:1 */
    pub fn CallNamedPipeW(lpNamedPipeName: ::winnt::LPCWSTR, lpInBuffer: ::minwindef::LPVOID, nInBufferSize: ::minwindef::DWORD, lpOutBuffer: ::minwindef::LPVOID, nOutBufferSize: ::minwindef::DWORD, lpBytesRead: ::minwindef::LPDWORD, nTimeOut: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5711:1 */
    pub fn CancelDeviceWakeupRequest(hDevice: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:1937:1 */
    pub fn CancelIo(hFile: ::winnt::HANDLE) -> ::minwindef::BOOL; /* ioapiset.h:178:1 */
    pub fn ChangeTimerQueueTimer(TimerQueue: ::winnt::HANDLE, Timer: ::winnt::HANDLE, DueTime: ::minwindef::ULONG, Period: ::minwindef::ULONG) -> ::minwindef::BOOL; /* threadpoollegacyapiset.h:91:1 */
    pub fn ClearCommBreak(hFile: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:2004:1 */
    pub fn ClearCommError(hFile: ::winnt::HANDLE, lpErrors: ::minwindef::LPDWORD, lpStat: ::winbase::LPCOMSTAT) -> ::minwindef::BOOL; /* winbase.h:2011:1 */
    pub fn ClosePrivateNamespace(Handle: ::winnt::HANDLE, Flags: ::minwindef::ULONG) -> ::winnt::BOOLEAN; /* namespaceapi.h:64:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CommConfigDialogW as CommConfigDialog; /* winbase.h:6730:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CommConfigDialogA(lpszName: ::winnt::LPCSTR, hWnd: ::windef::HWND, lpCC: ::winbase::LPCOMMCONFIG) -> ::minwindef::BOOL; /* winbase.h:6716:1 */
    pub fn CommConfigDialogW(lpszName: ::winnt::LPCWSTR, hWnd: ::windef::HWND, lpCC: ::winbase::LPCOMMCONFIG) -> ::minwindef::BOOL; /* winbase.h:6724:1 */
    pub fn CompareFileTime(lpFileTime1: *const ::minwindef::FILETIME, lpFileTime2: *const ::minwindef::FILETIME) -> ::winnt::LONG; /* fileapi.h:77:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CompareStringW as CompareString; /* stringapiset.h:93:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CompareStringA(Locale: ::winnt::LCID, dwCmpFlags: ::minwindef::DWORD, lpString1: ::winnt::PCNZCH, cchCount1: ::libc::c_int, lpString2: ::winnt::PCNZCH, cchCount2: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1414:1 */
    pub fn CompareStringW(Locale: ::winnt::LCID, dwCmpFlags: ::minwindef::DWORD, lpString1: ::winnt::PCNZWCH, cchCount1: ::libc::c_int, lpString2: ::winnt::PCNZWCH, cchCount2: ::libc::c_int) -> ::libc::c_int; /* stringapiset.h:83:1 */
    pub fn ConnectNamedPipe(hNamedPipe: ::winnt::HANDLE, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* namedpipeapi.h:61:1 */
    pub fn ContinueDebugEvent(dwProcessId: ::minwindef::DWORD, dwThreadId: ::minwindef::DWORD, dwContinueStatus: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:117:1 */
    pub fn ConvertDefaultLocale(Locale: ::winnt::LCID) -> ::winnt::LCID; /* winnls.h:1957:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CopyFileW as CopyFile; /* winbase.h:5054:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CopyFileA(lpExistingFileName: ::winnt::LPCSTR, lpNewFileName: ::winnt::LPCSTR, bFailIfExists: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:5040:1 */
    pub fn CopyFileW(lpExistingFileName: ::winnt::LPCWSTR, lpNewFileName: ::winnt::LPCWSTR, bFailIfExists: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:5048:1 */
    pub fn CreateBoundaryDescriptorW(Name: ::winnt::LPCWSTR, Flags: ::minwindef::ULONG) -> ::winnt::HANDLE; /* namespaceapi.h:73:1 */
    pub fn CreateConsoleScreenBuffer(dwDesiredAccess: ::minwindef::DWORD, dwShareMode: ::minwindef::DWORD, lpSecurityAttributes: *const ::minwinbase::SECURITY_ATTRIBUTES, dwFlags: ::minwindef::DWORD, lpScreenBufferData: ::minwindef::LPVOID) -> ::winnt::HANDLE; /* wincon.h:826:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateDirectoryExW as CreateDirectoryEx; /* winbase.h:4665:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateDirectoryExA(lpTemplateDirectory: ::winnt::LPCSTR, lpNewDirectory: ::winnt::LPCSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::BOOL; /* winbase.h:4651:1 */
    pub fn CreateDirectoryExW(lpTemplateDirectory: ::winnt::LPCWSTR, lpNewDirectory: ::winnt::LPCWSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::BOOL; /* winbase.h:4659:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateEventW as CreateEvent; /* synchapi.h:609:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateEventA(lpEventAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, bManualReset: ::minwindef::BOOL, bInitialState: ::minwindef::BOOL, lpName: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* synchapi.h:590:1 */
    pub fn CreateEventW(lpEventAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, bManualReset: ::minwindef::BOOL, bInitialState: ::minwindef::BOOL, lpName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* synchapi.h:601:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateFileW as CreateFile; /* fileapi.h:146:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateFileA(lpFileName: ::winnt::LPCSTR, dwDesiredAccess: ::minwindef::DWORD, dwShareMode: ::minwindef::DWORD, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, dwCreationDisposition: ::minwindef::DWORD, dwFlagsAndAttributes: ::minwindef::DWORD, hTemplateFile: ::winnt::HANDLE) -> ::winnt::HANDLE; /* fileapi.h:122:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateFileMappingW as CreateFileMapping; /* memoryapi.h:254:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateFileMappingA(hFile: ::winnt::HANDLE, lpFileMappingAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, flProtect: ::minwindef::DWORD, dwMaximumSizeHigh: ::minwindef::DWORD, dwMaximumSizeLow: ::minwindef::DWORD, lpName: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:3116:1 */
    pub fn CreateFileMappingW(hFile: ::winnt::HANDLE, lpFileMappingAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, flProtect: ::minwindef::DWORD, dwMaximumSizeHigh: ::minwindef::DWORD, dwMaximumSizeLow: ::minwindef::DWORD, lpName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* memoryapi.h:243:1 */
    pub fn CreateFileW(lpFileName: ::winnt::LPCWSTR, dwDesiredAccess: ::minwindef::DWORD, dwShareMode: ::minwindef::DWORD, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, dwCreationDisposition: ::minwindef::DWORD, dwFlagsAndAttributes: ::minwindef::DWORD, hTemplateFile: ::winnt::HANDLE) -> ::winnt::HANDLE; /* fileapi.h:135:1 */
    pub fn CreateIoCompletionPort(FileHandle: ::winnt::HANDLE, ExistingCompletionPort: ::winnt::HANDLE, CompletionKey: ::basetsd::ULONG_PTR, NumberOfConcurrentThreads: ::minwindef::DWORD) -> ::winnt::HANDLE; /* ioapiset.h:64:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateMailslotW as CreateMailslot; /* winbase.h:2439:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateMailslotA(lpName: ::winnt::LPCSTR, nMaxMessageSize: ::minwindef::DWORD, lReadTimeout: ::minwindef::DWORD, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::winnt::HANDLE; /* winbase.h:2423:1 */
    pub fn CreateMailslotW(lpName: ::winnt::LPCWSTR, nMaxMessageSize: ::minwindef::DWORD, lReadTimeout: ::minwindef::DWORD, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::winnt::HANDLE; /* winbase.h:2432:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateMutexW as CreateMutex; /* synchapi.h:552:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateMutexA(lpMutexAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, bInitialOwner: ::minwindef::BOOL, lpName: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* synchapi.h:535:1 */
    pub fn CreateMutexW(lpMutexAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, bInitialOwner: ::minwindef::BOOL, lpName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* synchapi.h:545:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateNamedPipeW as CreateNamedPipe; /* namedpipeapi.h:129:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateNamedPipeA(lpName: ::winnt::LPCSTR, dwOpenMode: ::minwindef::DWORD, dwPipeMode: ::minwindef::DWORD, nMaxInstances: ::minwindef::DWORD, nOutBufferSize: ::minwindef::DWORD, nInBufferSize: ::minwindef::DWORD, nDefaultTimeOut: ::minwindef::DWORD, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::winnt::HANDLE; /* winbase.h:5652:1 */
    pub fn CreateNamedPipeW(lpName: ::winnt::LPCWSTR, dwOpenMode: ::minwindef::DWORD, dwPipeMode: ::minwindef::DWORD, nMaxInstances: ::minwindef::DWORD, nOutBufferSize: ::minwindef::DWORD, nInBufferSize: ::minwindef::DWORD, nDefaultTimeOut: ::minwindef::DWORD, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::winnt::HANDLE; /* namedpipeapi.h:116:1 */
    pub fn CreatePipe(hReadPipe: ::winnt::PHANDLE, hWritePipe: ::winnt::PHANDLE, lpPipeAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, nSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:50:1 */
    pub fn CreatePrivateNamespaceW(lpPrivateNamespaceAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpBoundaryDescriptor: ::minwindef::LPVOID, lpAliasPrefix: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* namespaceapi.h:45:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateProcessW as CreateProcess; /* processthreadsapi.h:532:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateProcessA(lpApplicationName: ::winnt::LPCSTR, lpCommandLine: ::winnt::LPSTR, lpProcessAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpThreadAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, bInheritHandles: ::minwindef::BOOL, dwCreationFlags: ::minwindef::DWORD, lpEnvironment: ::minwindef::LPVOID, lpCurrentDirectory: ::winnt::LPCSTR, lpStartupInfo: ::processthreadsapi::LPSTARTUPINFOA, lpProcessInformation: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* processthreadsapi.h:502:1 */
    pub fn CreateProcessW(lpApplicationName: ::winnt::LPCWSTR, lpCommandLine: ::winnt::LPWSTR, lpProcessAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpThreadAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, bInheritHandles: ::minwindef::BOOL, dwCreationFlags: ::minwindef::DWORD, lpEnvironment: ::minwindef::LPVOID, lpCurrentDirectory: ::winnt::LPCWSTR, lpStartupInfo: ::processthreadsapi::LPSTARTUPINFOW, lpProcessInformation: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* processthreadsapi.h:518:1 */
    pub fn CreateRemoteThread(hProcess: ::winnt::HANDLE, lpThreadAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, dwStackSize: ::basetsd::SIZE_T, lpStartAddress: ::minwinbase::LPTHREAD_START_ROUTINE, lpParameter: ::minwindef::LPVOID, dwCreationFlags: ::minwindef::DWORD, lpThreadId: ::minwindef::LPDWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:228:1 */
    pub fn CreateRemoteThreadEx(hProcess: ::winnt::HANDLE, lpThreadAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, dwStackSize: ::basetsd::SIZE_T, lpStartAddress: ::minwinbase::LPTHREAD_START_ROUTINE, lpParameter: ::minwindef::LPVOID, dwCreationFlags: ::minwindef::DWORD, lpAttributeList: ::processthreadsapi::LPPROC_THREAD_ATTRIBUTE_LIST, lpThreadId: ::minwindef::LPDWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:815:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateSemaphoreW as CreateSemaphore; /* winbase.h:3019:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateSemaphoreA(lpSemaphoreAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lInitialCount: ::winnt::LONG, lMaximumCount: ::winnt::LONG, lpName: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:3002:1 */
    pub fn CreateSemaphoreW(lpSemaphoreAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lInitialCount: ::winnt::LONG, lMaximumCount: ::winnt::LONG, lpName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winbase.h:3012:1 */
    pub fn CreateTapePartition(hDevice: ::winnt::HANDLE, dwPartitionMethod: ::minwindef::DWORD, dwCount: ::minwindef::DWORD, dwSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:2194:1 */
    pub fn CreateTimerQueue() -> ::winnt::HANDLE; /* threadpoollegacyapiset.h:68:1 */
    pub fn CreateTimerQueueTimer(phNewTimer: ::winnt::PHANDLE, TimerQueue: ::winnt::HANDLE, Callback: ::winnt::WAITORTIMERCALLBACK, Parameter: ::winnt::PVOID, DueTime: ::minwindef::DWORD, Period: ::minwindef::DWORD, Flags: ::minwindef::ULONG) -> ::minwindef::BOOL; /* threadpoollegacyapiset.h:76:1 */
    pub fn DebugActiveProcess(dwProcessId: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:136:1 */
    pub fn DebugActiveProcessStop(dwProcessId: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:144:1 */
    pub fn DebugBreak(); /* debugapi.h:70:1 */
    pub fn DebugBreakProcess(Process: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:1862:1 */
    pub fn DebugSetProcessKillOnExit(KillOnExit: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:1855:1 */
    pub fn DecodeSystemPointer(Ptr: ::winnt::PVOID) -> ::winnt::PVOID; /* utilapiset.h:77:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DefineDosDeviceW as DefineDosDevice; /* fileapi.h:162:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DefineDosDeviceA(dwFlags: ::minwindef::DWORD, lpDeviceName: ::winnt::LPCSTR, lpTargetPath: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4755:1 */
    pub fn DefineDosDeviceW(dwFlags: ::minwindef::DWORD, lpDeviceName: ::winnt::LPCWSTR, lpTargetPath: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* fileapi.h:154:1 */
    pub fn DeleteAtom(nAtom: ::minwindef::ATOM) -> ::minwindef::ATOM; /* winbase.h:1916:1 */
    pub fn DeleteBoundaryDescriptor(BoundaryDescriptor: ::winnt::HANDLE); /* namespaceapi.h:91:1 */
    pub fn DeleteSynchronizationBarrier(lpBarrier: ::synchapi::LPSYNCHRONIZATION_BARRIER) -> ::minwindef::BOOL; /* synchapi.h:893:1 */
    pub fn DeleteTimerQueueEx(TimerQueue: ::winnt::HANDLE, CompletionEvent: ::winnt::HANDLE) -> ::minwindef::BOOL; /* threadpoollegacyapiset.h:114:1 */
    pub fn DeleteTimerQueueTimer(TimerQueue: ::winnt::HANDLE, Timer: ::winnt::HANDLE, CompletionEvent: ::winnt::HANDLE) -> ::minwindef::BOOL; /* threadpoollegacyapiset.h:103:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DeleteVolumeMountPointW as DeleteVolumeMountPoint; /* fileapi.h:208:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DeleteVolumeMountPointW(lpszVolumeMountPoint: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* fileapi.h:202:1 */
    pub fn DeviceIoControl(hDevice: ::winnt::HANDLE, dwIoControlCode: ::minwindef::DWORD, lpInBuffer: ::minwindef::LPVOID, nInBufferSize: ::minwindef::DWORD, lpOutBuffer: ::minwindef::LPVOID, nOutBufferSize: ::minwindef::DWORD, lpBytesReturned: ::minwindef::LPDWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* ioapiset.h:116:1 */
    pub fn DisconnectNamedPipe(hNamedPipe: ::winnt::HANDLE) -> ::minwindef::BOOL; /* namedpipeapi.h:70:1 */
    pub fn DnsHostnameToComputerNameExW(Hostname: ::winnt::LPCWSTR, ComputerName: ::winnt::LPWSTR, nSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:602:1 */
    pub fn DosDateTimeToFileTime(wFatDate: ::minwindef::WORD, wFatTime: ::minwindef::WORD, lpFileTime: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* winbase.h:2311:1 */
    pub fn EncodeSystemPointer(Ptr: ::winnt::PVOID) -> ::winnt::PVOID; /* utilapiset.h:68:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EndUpdateResourceW as EndUpdateResource; /* winbase.h:3835:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EndUpdateResourceA(hUpdate: ::winnt::HANDLE, fDiscard: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:3823:1 */
    pub fn EndUpdateResourceW(hUpdate: ::winnt::HANDLE, fDiscard: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:3830:1 */
    pub fn EnterSynchronizationBarrier(lpBarrier: ::synchapi::LPSYNCHRONIZATION_BARRIER, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:876:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumCalendarInfoW as EnumCalendarInfo; /* winnls.h:1763:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumCalendarInfoA(lpCalInfoEnumProc: ::winnls::CALINFO_ENUMPROCA, Locale: ::winnt::LCID, Calendar: ::winnls::CALID, CalType: ::winnls::CALTYPE) -> ::minwindef::BOOL; /* winnls.h:1748:1 */
    pub fn EnumCalendarInfoW(lpCalInfoEnumProc: ::winnls::CALINFO_ENUMPROCW, Locale: ::winnt::LCID, Calendar: ::winnls::CALID, CalType: ::winnls::CALTYPE) -> ::minwindef::BOOL; /* winnls.h:1757:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumDateFormatsW as EnumDateFormats; /* winnls.h:1833:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumDateFormatsA(lpDateFmtEnumProc: ::winnls::DATEFMT_ENUMPROCA, Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1820:1 */
    pub fn EnumDateFormatsW(lpDateFmtEnumProc: ::winnls::DATEFMT_ENUMPROCW, Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1828:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumResourceLanguagesW as EnumResourceLanguages; /* winbase.h:3767:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumResourceLanguagesA(hModule: ::minwindef::HMODULE, lpType: ::winnt::LPCSTR, lpName: ::winnt::LPCSTR, lpEnumFunc: ::libloaderapi::ENUMRESLANGPROCA, lParam: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winbase.h:3749:1 */
    pub fn EnumResourceLanguagesW(hModule: ::minwindef::HMODULE, lpType: ::winnt::LPCWSTR, lpName: ::winnt::LPCWSTR, lpEnumFunc: ::libloaderapi::ENUMRESLANGPROCW, lParam: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winbase.h:3759:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumResourceNamesW as EnumResourceNames; /* winbase.h:3741:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumResourceNamesA(hModule: ::minwindef::HMODULE, lpType: ::winnt::LPCSTR, lpEnumFunc: ::libloaderapi::ENUMRESNAMEPROCA, lParam: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winbase.h:3725:1 */
    pub fn EnumResourceNamesW(hModule: ::minwindef::HMODULE, lpType: ::winnt::LPCWSTR, lpEnumFunc: ::libloaderapi::ENUMRESNAMEPROCW, lParam: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winbase.h:3734:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumResourceTypesW as EnumResourceTypes; /* winbase.h:3717:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumResourceTypesA(hModule: ::minwindef::HMODULE, lpEnumFunc: ::libloaderapi::ENUMRESTYPEPROCA, lParam: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winbase.h:3703:1 */
    pub fn EnumResourceTypesW(hModule: ::minwindef::HMODULE, lpEnumFunc: ::libloaderapi::ENUMRESTYPEPROCW, lParam: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winbase.h:3711:1 */
    pub fn EnumSystemFirmwareTables(FirmwareTableProviderSignature: ::minwindef::DWORD, pFirmwareTableEnumBuffer: ::winnt::PVOID, BufferSize: ::minwindef::DWORD) -> ::minwindef::UINT; /* sysinfoapi.h:565:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumTimeFormatsW as EnumTimeFormats; /* winnls.h:1811:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumTimeFormatsA(lpTimeFmtEnumProc: ::winnls::TIMEFMT_ENUMPROCA, Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1798:1 */
    pub fn EnumTimeFormatsW(lpTimeFmtEnumProc: ::winnls::TIMEFMT_ENUMPROCW, Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1806:1 */
    pub fn EraseTape(hDevice: ::winnt::HANDLE, dwEraseType: ::minwindef::DWORD, bImmediate: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winbase.h:2185:1 */
    pub fn EscapeCommFunction(hFile: ::winnt::HANDLE, dwFunc: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2029:1 */
    pub fn ExitProcess(uExitCode: ::minwindef::UINT); /* processthreadsapi.h:169:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ExpandEnvironmentStringsW as ExpandEnvironmentStrings; /* processenv.h:248:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ExpandEnvironmentStringsA(lpSrc: ::winnt::LPCSTR, lpDst: ::winnt::LPSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processenv.h:231:1 */
    pub fn ExpandEnvironmentStringsW(lpSrc: ::winnt::LPCWSTR, lpDst: ::winnt::LPWSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processenv.h:241:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FatalAppExitW as FatalAppExit; /* winbase.h:3466:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FatalAppExitA(uAction: ::minwindef::UINT, lpMessageText: ::winnt::LPCSTR); /* winbase.h:3454:1 */
    pub fn FatalAppExitW(uAction: ::minwindef::UINT, lpMessageText: ::winnt::LPCWSTR); /* winbase.h:3461:1 */
    pub fn FatalExit(ExitCode: ::libc::c_int); /* winbase.h:1254:1 */
    pub fn FileTimeToDosDateTime(lpFileTime: *const ::minwindef::FILETIME, lpFatDate: ::minwindef::LPWORD, lpFatTime: ::minwindef::LPWORD) -> ::minwindef::BOOL; /* winbase.h:2302:1 */
    pub fn FileTimeToLocalFileTime(lpFileTime: *const ::minwindef::FILETIME, lpLocalFileTime: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* fileapi.h:214:1 */
    pub fn FillConsoleOutputAttribute(hConsoleOutput: ::winnt::HANDLE, wAttribute: ::minwindef::WORD, nLength: ::minwindef::DWORD, dwWriteCoord: ::wincon::COORD, lpNumberOfAttrsWritten: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:522:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FillConsoleOutputCharacterW as FillConsoleOutputCharacter; /* wincon.h:514:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FillConsoleOutputCharacterA(hConsoleOutput: ::winnt::HANDLE, cCharacter: ::winnt::CHAR, nLength: ::minwindef::DWORD, dwWriteCoord: ::wincon::COORD, lpNumberOfCharsWritten: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:496:1 */
    pub fn FillConsoleOutputCharacterW(hConsoleOutput: ::winnt::HANDLE, cCharacter: ::winnt::WCHAR, nLength: ::minwindef::DWORD, dwWriteCoord: ::wincon::COORD, lpNumberOfCharsWritten: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:506:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindAtomW as FindAtom; /* winbase.h:3951:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindAtomA(lpString: ::winnt::LPCSTR) -> ::minwindef::ATOM; /* winbase.h:3941:1 */
    pub fn FindAtomW(lpString: ::winnt::LPCWSTR) -> ::minwindef::ATOM; /* winbase.h:3947:1 */
    pub fn FindCloseChangeNotification(hChangeHandle: ::winnt::HANDLE) -> ::minwindef::BOOL; /* fileapi.h:248:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindFirstChangeNotificationW as FindFirstChangeNotification; /* fileapi.h:272:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindFirstChangeNotificationA(lpPathName: ::winnt::LPCSTR, bWatchSubtree: ::minwindef::BOOL, dwNotifyFilter: ::minwindef::DWORD) -> ::winnt::HANDLE; /* fileapi.h:256:1 */
    pub fn FindFirstChangeNotificationW(lpPathName: ::winnt::LPCWSTR, bWatchSubtree: ::minwindef::BOOL, dwNotifyFilter: ::minwindef::DWORD) -> ::winnt::HANDLE; /* fileapi.h:265:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindFirstFileW as FindFirstFile; /* fileapi.h:294:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindFirstFileA(lpFileName: ::winnt::LPCSTR, lpFindFileData: ::minwinbase::LPWIN32_FIND_DATAA) -> ::winnt::HANDLE; /* fileapi.h:280:1 */
    pub fn FindFirstFileW(lpFileName: ::winnt::LPCWSTR, lpFindFileData: ::minwinbase::LPWIN32_FIND_DATAW) -> ::winnt::HANDLE; /* fileapi.h:288:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindFirstVolumeW as FindFirstVolume; /* fileapi.h:358:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindFirstVolumeW(lpszVolumeName: ::winnt::LPWSTR, cchBufferLength: ::minwindef::DWORD) -> ::winnt::HANDLE; /* fileapi.h:351:1 */
    pub fn FindNextChangeNotification(hChangeHandle: ::winnt::HANDLE) -> ::minwindef::BOOL; /* fileapi.h:364:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindNextVolumeW as FindNextVolume; /* fileapi.h:416:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindNextVolumeW(hFindVolume: ::winnt::HANDLE, lpszVolumeName: ::winnt::LPWSTR, cchBufferLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:408:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindResourceW as FindResource; /* winbase.h:3681:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindResourceA(hModule: ::minwindef::HMODULE, lpName: ::winnt::LPCSTR, lpType: ::winnt::LPCSTR) -> ::minwindef::HRSRC; /* winbase.h:3666:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindResourceExW as FindResourceEx; /* libloaderapi.h:182:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindResourceExA(hModule: ::minwindef::HMODULE, lpType: ::winnt::LPCSTR, lpName: ::winnt::LPCSTR, wLanguage: ::minwindef::WORD) -> ::minwindef::HRSRC; /* winbase.h:3690:1 */
    pub fn FindResourceExW(hModule: ::minwindef::HMODULE, lpType: ::winnt::LPCWSTR, lpName: ::winnt::LPCWSTR, wLanguage: ::minwindef::WORD) -> ::minwindef::HRSRC; /* libloaderapi.h:173:1 */
    pub fn FindResourceW(hModule: ::minwindef::HMODULE, lpName: ::winnt::LPCWSTR, lpType: ::winnt::LPCWSTR) -> ::minwindef::HRSRC; /* winbase.h:3675:1 */
    pub fn FindVolumeClose(hFindVolume: ::winnt::HANDLE) -> ::minwindef::BOOL; /* fileapi.h:422:1 */
    pub fn FlushConsoleInputBuffer(hConsoleInput: ::winnt::HANDLE) -> ::minwindef::BOOL; /* wincon.h:645:1 */
    pub fn FlushInstructionCache(hProcess: ::winnt::HANDLE, lpBaseAddress: ::minwindef::LPCVOID, dwSize: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:916:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FoldStringW as FoldString; /* stringapiset.h:108:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FoldStringA(dwMapFlags: ::minwindef::DWORD, lpSrcStr: ::winnt::LPCSTR, cchSrc: ::libc::c_int, lpDestStr: ::winnt::LPSTR, cchDest: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2170:1 */
    pub fn FoldStringW(dwMapFlags: ::minwindef::DWORD, lpSrcStr: ::winnt::LPCWCH, cchSrc: ::libc::c_int, lpDestStr: ::winnt::LPWSTR, cchDest: ::libc::c_int) -> ::libc::c_int; /* stringapiset.h:99:1 */
    pub fn FreeConsole() -> ::minwindef::BOOL; /* wincon.h:726:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FreeEnvironmentStringsW as FreeEnvironmentStrings; /* processenv.h:100:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FreeEnvironmentStringsA(penv: ::winnt::LPCH) -> ::minwindef::BOOL; /* processenv.h:88:1 */
    pub fn FreeEnvironmentStringsW(penv: ::winnt::LPWCH) -> ::minwindef::BOOL; /* processenv.h:95:1 */
    pub fn FreeLibraryAndExitThread(hLibModule: ::minwindef::HMODULE, dwExitCode: ::minwindef::DWORD); /* libloaderapi.h:229:1 */
    pub fn FreeResource(hResData: ::minwindef::HGLOBAL) -> ::minwindef::BOOL; /* libloaderapi.h:238:1 */
    pub fn GenerateConsoleCtrlEvent(dwCtrlEvent: ::minwindef::DWORD, dwProcessGroupId: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wincon.h:719:1 */
    pub fn GetACP() -> ::minwindef::UINT; /* winnls.h:1360:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetAtomNameW as GetAtomName; /* winbase.h:3973:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetAtomNameA(nAtom: ::minwindef::ATOM, lpBuffer: ::winnt::LPSTR, nSize: ::libc::c_int) -> ::minwindef::UINT; /* winbase.h:3959:1 */
    pub fn GetAtomNameW(nAtom: ::minwindef::ATOM, lpBuffer: ::winnt::LPWSTR, nSize: ::libc::c_int) -> ::minwindef::UINT; /* winbase.h:3967:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetBinaryTypeW as GetBinaryType; /* winbase.h:1159:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetBinaryTypeA(lpApplicationName: ::winnt::LPCSTR, lpBinaryType: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:1147:1 */
    pub fn GetBinaryTypeW(lpApplicationName: ::winnt::LPCWSTR, lpBinaryType: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:1154:1 */
    pub fn GetCommConfig(hCommDev: ::winnt::HANDLE, lpCC: ::winbase::LPCOMMCONFIG, lpdwSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2038:1 */
    pub fn GetCommMask(hFile: ::winnt::HANDLE, lpEvtMask: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2047:1 */
    pub fn GetCommModemStatus(hFile: ::winnt::HANDLE, lpModemStat: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2063:1 */
    pub fn GetCommProperties(hFile: ::winnt::HANDLE, lpCommProp: ::winbase::LPCOMMPROP) -> ::minwindef::BOOL; /* winbase.h:2055:1 */
    pub fn GetCommState(hFile: ::winnt::HANDLE, lpDCB: ::winbase::LPDCB) -> ::minwindef::BOOL; /* winbase.h:2071:1 */
    pub fn GetCommTimeouts(hFile: ::winnt::HANDLE, lpCommTimeouts: ::winbase::LPCOMMTIMEOUTS) -> ::minwindef::BOOL; /* winbase.h:2079:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetComputerNameW as GetComputerName; /* winbase.h:6802:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetComputerNameA(lpBuffer: ::winnt::LPSTR, nSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6789:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetComputerNameExW as GetComputerNameEx; /* sysinfoapi.h:377:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetComputerNameExA(NameType: ::sysinfoapi::COMPUTER_NAME_FORMAT, lpBuffer: ::winnt::LPSTR, nSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:360:1 */
    pub fn GetComputerNameExW(NameType: ::sysinfoapi::COMPUTER_NAME_FORMAT, lpBuffer: ::winnt::LPWSTR, nSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:370:1 */
    pub fn GetComputerNameW(lpBuffer: ::winnt::LPWSTR, nSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6797:1 */
    pub fn GetConsoleCP() -> ::minwindef::UINT; /* consoleapi.h:52:1 */
    pub fn GetConsoleCursorInfo(hConsoleOutput: ::winnt::HANDLE, lpConsoleCursorInfo: ::wincon::PCONSOLE_CURSOR_INFO) -> ::minwindef::BOOL; /* wincon.h:565:1 */
    pub fn GetConsoleMode(hConsoleHandle: ::winnt::HANDLE, lpMode: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* consoleapi.h:60:1 */
    pub fn GetConsoleOutputCP() -> ::minwindef::UINT; /* consoleapi.h:69:1 */
    pub fn GetConsoleScreenBufferInfo(hConsoleOutput: ::winnt::HANDLE, lpConsoleScreenBufferInfo: ::wincon::PCONSOLE_SCREEN_BUFFER_INFO) -> ::minwindef::BOOL; /* wincon.h:536:1 */
    pub fn GetConsoleScreenBufferInfoEx(hConsoleOutput: ::winnt::HANDLE, lpConsoleScreenBufferInfoEx: ::wincon::PCONSOLE_SCREEN_BUFFER_INFOEX) -> ::minwindef::BOOL; /* wincon.h:544:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetConsoleTitleW as GetConsoleTitle; /* wincon.h:755:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetConsoleTitleA(lpConsoleTitle: ::winnt::LPSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:743:1 */
    pub fn GetConsoleTitleW(lpConsoleTitle: ::winnt::LPWSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:750:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetCurrencyFormatW as GetCurrencyFormat; /* winnls.h:1739:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetCurrencyFormatA(Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD, lpValue: ::winnt::LPCSTR, lpFormat: *const ::winnls::CURRENCYFMTA, lpCurrencyStr: ::winnt::LPSTR, cchCurrency: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1720:1 */
    pub fn GetCurrencyFormatW(Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD, lpValue: ::winnt::LPCWSTR, lpFormat: *const ::winnls::CURRENCYFMTW, lpCurrencyStr: ::winnt::LPWSTR, cchCurrency: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1731:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetCurrentDirectoryW as GetCurrentDirectory; /* processenv.h:292:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetCurrentDirectoryA(nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPSTR) -> ::minwindef::DWORD; /* processenv.h:277:1 */
    pub fn GetCurrentDirectoryW(nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* processenv.h:286:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetDateFormatW as GetDateFormat; /* datetimeapi.h:76:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetDateFormatA(Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD, lpDate: *const ::minwinbase::SYSTEMTIME, lpFormat: ::winnt::LPCSTR, lpDateStr: ::winnt::LPSTR, cchDate: ::libc::c_int) -> ::libc::c_int; /* datetimeapi.h:53:1 */
    pub fn GetDateFormatW(Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD, lpDate: *const ::minwinbase::SYSTEMTIME, lpFormat: ::winnt::LPCWSTR, lpDateStr: ::winnt::LPWSTR, cchDate: ::libc::c_int) -> ::libc::c_int; /* datetimeapi.h:66:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetDefaultCommConfigW as GetDefaultCommConfig; /* winbase.h:6752:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetDefaultCommConfigA(lpszName: ::winnt::LPCSTR, lpCC: ::winbase::LPCOMMCONFIG, lpdwSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6738:1 */
    pub fn GetDefaultCommConfigW(lpszName: ::winnt::LPCWSTR, lpCC: ::winbase::LPCOMMCONFIG, lpdwSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6746:1 */
    pub fn GetDevicePowerState(hDevice: ::winnt::HANDLE, pfOn: *mut ::libc::c_int) -> ::minwindef::BOOL; /* winbase.h:1944:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetDiskFreeSpaceW as GetDiskFreeSpace; /* fileapi.h:472:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetDiskFreeSpaceA(lpRootPathName: ::winnt::LPCSTR, lpSectorsPerCluster: ::minwindef::LPDWORD, lpBytesPerSector: ::minwindef::LPDWORD, lpNumberOfFreeClusters: ::minwindef::LPDWORD, lpTotalNumberOfClusters: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* fileapi.h:452:1 */
    pub fn GetDiskFreeSpaceW(lpRootPathName: ::winnt::LPCWSTR, lpSectorsPerCluster: ::minwindef::LPDWORD, lpBytesPerSector: ::minwindef::LPDWORD, lpNumberOfFreeClusters: ::minwindef::LPDWORD, lpTotalNumberOfClusters: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* fileapi.h:463:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetDriveTypeW as GetDriveType; /* fileapi.h:532:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetDriveTypeA(lpRootPathName: ::winnt::LPCSTR) -> ::minwindef::UINT; /* fileapi.h:520:1 */
    pub fn GetDriveTypeW(lpRootPathName: ::winnt::LPCWSTR) -> ::minwindef::UINT; /* fileapi.h:527:1 */
    pub fn GetEnvironmentStrings() -> ::winnt::LPCH; /* processenv.h:54:1 */
    pub fn GetEnvironmentStringsW() -> ::winnt::LPWCH; /* processenv.h:63:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetEnvironmentVariableW as GetEnvironmentVariable; /* processenv.h:200:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetEnvironmentVariableA(lpName: ::winnt::LPCSTR, lpBuffer: ::winnt::LPSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processenv.h:183:1 */
    pub fn GetEnvironmentVariableW(lpName: ::winnt::LPCWSTR, lpBuffer: ::winnt::LPWSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processenv.h:193:1 */
    pub fn GetExitCodeProcess(hProcess: ::winnt::HANDLE, lpExitCode: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:186:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileAttributesW as GetFileAttributes; /* fileapi.h:552:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileAttributesA(lpFileName: ::winnt::LPCSTR) -> ::minwindef::DWORD; /* fileapi.h:540:1 */
    pub fn GetFileAttributesW(lpFileName: ::winnt::LPCWSTR) -> ::minwindef::DWORD; /* fileapi.h:547:1 */
    pub fn GetFileInformationByHandle(hFile: ::winnt::HANDLE, lpFileInformation: ::fileapi::LPBY_HANDLE_FILE_INFORMATION) -> ::minwindef::BOOL; /* fileapi.h:627:1 */
    pub fn GetFileSize(hFile: ::winnt::HANDLE, lpFileSizeHigh: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* fileapi.h:636:1 */
    pub fn GetFileSizeEx(hFile: ::winnt::HANDLE, lpFileSize: ::winnt::PLARGE_INTEGER) -> ::minwindef::BOOL; /* fileapi.h:645:1 */
    pub fn GetFileTime(hFile: ::winnt::HANDLE, lpCreationTime: ::minwindef::LPFILETIME, lpLastAccessTime: ::minwindef::LPFILETIME, lpLastWriteTime: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* fileapi.h:654:1 */
    pub fn GetFileType(hFile: ::winnt::HANDLE) -> ::minwindef::DWORD; /* fileapi.h:665:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFirmwareEnvironmentVariableW as GetFirmwareEnvironmentVariable; /* winbase.h:3554:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFirmwareEnvironmentVariableA(lpName: ::winnt::LPCSTR, lpGuid: ::winnt::LPCSTR, pBuffer: ::winnt::PVOID, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:3538:1 */
    pub fn GetFirmwareEnvironmentVariableW(lpName: ::winnt::LPCWSTR, lpGuid: ::winnt::LPCWSTR, pBuffer: ::winnt::PVOID, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:3547:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFullPathNameW as GetFullPathName; /* fileapi.h:724:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFullPathNameA(lpFileName: ::winnt::LPCSTR, nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPSTR, lpFilePart: *mut *mut ::libc::c_schar) -> ::minwindef::DWORD; /* fileapi.h:705:1 */
    pub fn GetFullPathNameW(lpFileName: ::winnt::LPCWSTR, nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPWSTR, lpFilePart: *mut *mut ::libc::c_ushort) -> ::minwindef::DWORD; /* fileapi.h:716:1 */
    pub fn GetHandleInformation(hObject: ::winnt::HANDLE, lpdwFlags: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* handleapi.h:79:1 */
    pub fn GetLargePageMinimum() -> ::basetsd::SIZE_T; /* memoryapi.h:339:1 */
    pub fn GetLargestConsoleWindowSize(hConsoleOutput: ::winnt::HANDLE) -> ::wincon::COORD; /* wincon.h:558:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetLocaleInfoW as GetLocaleInfo; /* winnls.h:1512:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetLocaleInfoA(Locale: ::winnt::LCID, LCType: ::winnls::LCTYPE, lpLCData: ::winnt::LPSTR, cchData: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1518:1 */
    pub fn GetLocaleInfoW(Locale: ::winnt::LCID, LCType: ::winnls::LCTYPE, lpLCData: ::winnt::LPWSTR, cchData: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1505:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetLogicalDriveStringsW as GetLogicalDriveStrings; /* fileapi.h:747:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetLogicalDriveStringsA(nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPSTR) -> ::minwindef::DWORD; /* winbase.h:3166:1 */
    pub fn GetLogicalDriveStringsW(nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* fileapi.h:740:1 */
    pub fn GetLogicalDrives() -> ::minwindef::DWORD; /* fileapi.h:732:1 */
    pub fn GetLogicalProcessorInformation(Buffer: ::winnt::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION, ReturnedLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:455:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetLongPathNameW as GetLongPathName; /* fileapi.h:771:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetLongPathNameA(lpszShortPath: ::winnt::LPCSTR, lpszLongPath: ::winnt::LPSTR, cchBuffer: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:754:1 */
    pub fn GetLongPathNameW(lpszShortPath: ::winnt::LPCWSTR, lpszLongPath: ::winnt::LPWSTR, cchBuffer: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:764:1 */
    pub fn GetMailslotInfo(hMailslot: ::winnt::HANDLE, lpMaxMessageSize: ::minwindef::LPDWORD, lpNextSize: ::minwindef::LPDWORD, lpMessageCount: ::minwindef::LPDWORD, lpReadTimeout: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2447:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetModuleFileNameW as GetModuleFileName; /* libloaderapi.h:266:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetModuleFileNameA(hModule: ::minwindef::HMODULE, lpFilename: ::winnt::LPSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* libloaderapi.h:248:1 */
    pub fn GetModuleFileNameW(hModule: ::minwindef::HMODULE, lpFilename: ::winnt::LPWSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* libloaderapi.h:259:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetModuleHandleW as GetModuleHandle; /* libloaderapi.h:290:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetModuleHandleA(lpModuleName: ::winnt::LPCSTR) -> ::minwindef::HMODULE; /* libloaderapi.h:276:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetModuleHandleExW as GetModuleHandleEx; /* libloaderapi.h:343:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetModuleHandleExA(dwFlags: ::minwindef::DWORD, lpModuleName: ::winnt::LPCSTR, phModule: *mut *mut ::minwindef::HINSTANCE__) -> ::minwindef::BOOL; /* libloaderapi.h:327:1 */
    pub fn GetModuleHandleExW(dwFlags: ::minwindef::DWORD, lpModuleName: ::winnt::LPCWSTR, phModule: *mut *mut ::minwindef::HINSTANCE__) -> ::minwindef::BOOL; /* libloaderapi.h:336:1 */
    pub fn GetModuleHandleW(lpModuleName: ::winnt::LPCWSTR) -> ::minwindef::HMODULE; /* libloaderapi.h:285:1 */
    pub fn GetNLSVersion(Function: ::winnls::NLS_FUNCTION, Locale: ::winnt::LCID, lpVersionInformation: ::winnls::LPNLSVERSIONINFO) -> ::minwindef::BOOL; /* winnls.h:1875:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetNamedPipeClientComputerNameW as GetNamedPipeClientComputerName; /* namedpipeapi.h:161:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetNamedPipeHandleStateW as GetNamedPipeHandleState; /* winbase.h:5691:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetNamedPipeHandleStateA(hNamedPipe: ::winnt::HANDLE, lpState: ::minwindef::LPDWORD, lpCurInstances: ::minwindef::LPDWORD, lpMaxCollectionCount: ::minwindef::LPDWORD, lpCollectDataTimeout: ::minwindef::LPDWORD, lpUserName: ::winnt::LPSTR, nMaxUserNameSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5669:1 */
    pub fn GetNamedPipeHandleStateW(hNamedPipe: ::winnt::HANDLE, lpState: ::minwindef::LPDWORD, lpCurInstances: ::minwindef::LPDWORD, lpMaxCollectionCount: ::minwindef::LPDWORD, lpCollectDataTimeout: ::minwindef::LPDWORD, lpUserName: ::winnt::LPWSTR, nMaxUserNameSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5681:1 */
    pub fn GetNamedPipeInfo(hNamedPipe: ::winnt::HANDLE, lpFlags: ::minwindef::LPDWORD, lpOutBufferSize: ::minwindef::LPDWORD, lpInBufferSize: ::minwindef::LPDWORD, lpMaxInstances: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2412:1 */
    pub fn GetNumaAvailableMemoryNode(Node: ::minwindef::UCHAR, AvailableBytes: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* winbase.h:8044:1 */
    pub fn GetNumaHighestNodeNumber(HighestNodeNumber: ::minwindef::PULONG) -> ::minwindef::BOOL; /* systemtopologyapi.h:43:1 */
    pub fn GetNumaNodeProcessorMask(Node: ::minwindef::UCHAR, ProcessorMask: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* winbase.h:8036:1 */
    pub fn GetNumaProcessorNode(Processor: ::minwindef::UCHAR, NodeNumber: ::minwindef::PUCHAR) -> ::minwindef::BOOL; /* winbase.h:8004:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetNumberFormatW as GetNumberFormat; /* winnls.h:1711:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetNumberFormatA(Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD, lpValue: ::winnt::LPCSTR, lpFormat: *const ::winnls::NUMBERFMTA, lpNumberStr: ::winnt::LPSTR, cchNumber: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1692:1 */
    pub fn GetNumberFormatW(Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD, lpValue: ::winnt::LPCWSTR, lpFormat: *const ::winnls::NUMBERFMTW, lpNumberStr: ::winnt::LPWSTR, cchNumber: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1703:1 */
    pub fn GetNumberOfConsoleInputEvents(hConsoleInput: ::winnt::HANDLE, lpNumberOfEvents: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* consoleapi.h:77:1 */
    pub fn GetNumberOfConsoleMouseButtons(lpNumberOfMouseButtons: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:631:1 */
    pub fn GetOEMCP() -> ::minwindef::UINT; /* winnls.h:1365:1 */
    pub fn GetOverlappedResult(hFile: ::winnt::HANDLE, lpOverlapped: ::minwinbase::LPOVERLAPPED, lpNumberOfBytesTransferred: ::minwindef::LPDWORD, bWait: ::minwindef::BOOL) -> ::minwindef::BOOL; /* ioapiset.h:52:1 */
    pub fn GetPhysicallyInstalledSystemMemory(TotalMemoryInKilobytes: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* sysinfoapi.h:613:1 */
    pub fn GetPriorityClass(hProcess: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:669:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetPrivateProfileIntW as GetPrivateProfileInt; /* winbase.h:4109:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetPrivateProfileIntA(lpAppName: ::winnt::LPCSTR, lpKeyName: ::winnt::LPCSTR, nDefault: ::winnt::INT, lpFileName: ::winnt::LPCSTR) -> ::minwindef::UINT; /* winbase.h:4093:1 */
    pub fn GetPrivateProfileIntW(lpAppName: ::winnt::LPCWSTR, lpKeyName: ::winnt::LPCWSTR, nDefault: ::winnt::INT, lpFileName: ::winnt::LPCWSTR) -> ::minwindef::UINT; /* winbase.h:4102:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetPrivateProfileSectionW as GetPrivateProfileSection; /* winbase.h:4237:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetPrivateProfileSectionA(lpAppName: ::winnt::LPCSTR, lpReturnedString: ::winnt::LPSTR, nSize: ::minwindef::DWORD, lpFileName: ::winnt::LPCSTR) -> ::minwindef::DWORD; /* winbase.h:4221:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetPrivateProfileSectionNamesW as GetPrivateProfileSectionNames; /* winbase.h:4306:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetPrivateProfileSectionNamesA(lpszReturnBuffer: ::winnt::LPSTR, nSize: ::minwindef::DWORD, lpFileName: ::winnt::LPCSTR) -> ::minwindef::DWORD; /* winbase.h:4292:1 */
    pub fn GetPrivateProfileSectionNamesW(lpszReturnBuffer: ::winnt::LPWSTR, nSize: ::minwindef::DWORD, lpFileName: ::winnt::LPCWSTR) -> ::minwindef::DWORD; /* winbase.h:4300:1 */
    pub fn GetPrivateProfileSectionW(lpAppName: ::winnt::LPCWSTR, lpReturnedString: ::winnt::LPWSTR, nSize: ::minwindef::DWORD, lpFileName: ::winnt::LPCWSTR) -> ::minwindef::DWORD; /* winbase.h:4230:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetPrivateProfileStringW as GetPrivateProfileString; /* winbase.h:4161:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetPrivateProfileStringA(lpAppName: ::winnt::LPCSTR, lpKeyName: ::winnt::LPCSTR, lpDefault: ::winnt::LPCSTR, lpReturnedString: ::winnt::LPSTR, nSize: ::minwindef::DWORD, lpFileName: ::winnt::LPCSTR) -> ::minwindef::DWORD; /* winbase.h:4141:1 */
    pub fn GetPrivateProfileStringW(lpAppName: ::winnt::LPCWSTR, lpKeyName: ::winnt::LPCWSTR, lpDefault: ::winnt::LPCWSTR, lpReturnedString: ::winnt::LPWSTR, nSize: ::minwindef::DWORD, lpFileName: ::winnt::LPCWSTR) -> ::minwindef::DWORD; /* winbase.h:4152:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetPrivateProfileStructW as GetPrivateProfileStruct; /* winbase.h:4354:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetPrivateProfileStructA(lpszSection: ::winnt::LPCSTR, lpszKey: ::winnt::LPCSTR, lpStruct: ::minwindef::LPVOID, uSizeStruct: ::minwindef::UINT, szFile: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4336:1 */
    pub fn GetPrivateProfileStructW(lpszSection: ::winnt::LPCWSTR, lpszKey: ::winnt::LPCWSTR, lpStruct: ::minwindef::LPVOID, uSizeStruct: ::minwindef::UINT, szFile: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:4346:1 */
    pub fn GetProcessAffinityMask(hProcess: ::winnt::HANDLE, lpProcessAffinityMask: ::basetsd::PDWORD_PTR, lpSystemAffinityMask: ::basetsd::PDWORD_PTR) -> ::minwindef::BOOL; /* winbase.h:1210:1 */
    pub fn GetProcessHeaps(NumberOfHeaps: ::minwindef::DWORD, ProcessHeaps: ::winnt::PHANDLE) -> ::minwindef::DWORD; /* heapapi.h:204:1 */
    pub fn GetProcessIoCounters(hProcess: ::winnt::HANDLE, lpIoCounters: ::winnt::PIO_COUNTERS) -> ::minwindef::BOOL; /* winbase.h:1227:1 */
    pub fn GetProcessShutdownParameters(lpdwLevel: ::minwindef::LPDWORD, lpdwFlags: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:3446:1 */
    pub fn GetProcessTimes(hProcess: ::winnt::HANDLE, lpCreationTime: ::minwindef::LPFILETIME, lpExitTime: ::minwindef::LPFILETIME, lpKernelTime: ::minwindef::LPFILETIME, lpUserTime: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* processthreadsapi.h:126:1 */
    pub fn GetProcessVersion(ProcessId: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processthreadsapi.h:551:1 */
    pub fn GetProcessWorkingSetSize(hProcess: ::winnt::HANDLE, lpMinimumWorkingSetSize: ::basetsd::PSIZE_T, lpMaximumWorkingSetSize: ::basetsd::PSIZE_T) -> ::minwindef::BOOL; /* winbase.h:1235:1 */
    pub fn GetProcessWorkingSetSizeEx(hProcess: ::winnt::HANDLE, lpMinimumWorkingSetSize: ::basetsd::PSIZE_T, lpMaximumWorkingSetSize: ::basetsd::PSIZE_T, Flags: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:348:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetProfileIntW as GetProfileInt; /* winbase.h:3995:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetProfileIntA(lpAppName: ::winnt::LPCSTR, lpKeyName: ::winnt::LPCSTR, nDefault: ::winnt::INT) -> ::minwindef::UINT; /* winbase.h:3981:1 */
    pub fn GetProfileIntW(lpAppName: ::winnt::LPCWSTR, lpKeyName: ::winnt::LPCWSTR, nDefault: ::winnt::INT) -> ::minwindef::UINT; /* winbase.h:3989:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetProfileSectionW as GetProfileSection; /* winbase.h:4065:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetProfileSectionA(lpAppName: ::winnt::LPCSTR, lpReturnedString: ::winnt::LPSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:4051:1 */
    pub fn GetProfileSectionW(lpAppName: ::winnt::LPCWSTR, lpReturnedString: ::winnt::LPWSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:4059:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetProfileStringW as GetProfileString; /* winbase.h:4021:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetProfileStringA(lpAppName: ::winnt::LPCSTR, lpKeyName: ::winnt::LPCSTR, lpDefault: ::winnt::LPCSTR, lpReturnedString: ::winnt::LPSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:4003:1 */
    pub fn GetProfileStringW(lpAppName: ::winnt::LPCWSTR, lpKeyName: ::winnt::LPCWSTR, lpDefault: ::winnt::LPCWSTR, lpReturnedString: ::winnt::LPWSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:4013:1 */
    pub fn GetQueuedCompletionStatus(CompletionPort: ::winnt::HANDLE, lpNumberOfBytesTransferred: ::minwindef::LPDWORD, lpCompletionKey: ::basetsd::PULONG_PTR, lpOverlapped: *mut *mut ::minwinbase::OVERLAPPED, dwMilliseconds: ::minwindef::DWORD) -> ::minwindef::BOOL; /* ioapiset.h:75:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetShortPathNameW as GetShortPathName; /* fileapi.h:788:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetShortPathNameA(lpszLongPath: ::winnt::LPCSTR, lpszShortPath: ::winnt::LPSTR, cchBuffer: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:1168:1 */
    pub fn GetShortPathNameW(lpszLongPath: ::winnt::LPCWSTR, lpszShortPath: ::winnt::LPWSTR, cchBuffer: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:780:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetStartupInfoW as GetStartupInfo; /* processthreadsapi.h:564:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetStartupInfoA(lpStartupInfo: ::processthreadsapi::LPSTARTUPINFOA); /* winbase.h:3474:1 */
    pub fn GetStartupInfoW(lpStartupInfo: ::processthreadsapi::LPSTARTUPINFOW); /* processthreadsapi.h:559:1 */
    pub fn GetStdHandle(nStdHandle: ::minwindef::DWORD) -> ::winnt::HANDLE; /* processenv.h:108:1 */
    pub fn GetStringTypeA(Locale: ::winnt::LCID, dwInfoType: ::minwindef::DWORD, lpSrcStr: ::winnt::LPCSTR, cchSrc: ::libc::c_int, lpCharType: ::minwindef::LPWORD) -> ::minwindef::BOOL; /* winnls.h:2160:1 */
    pub fn GetStringTypeExA(Locale: ::winnt::LCID, dwInfoType: ::minwindef::DWORD, lpSrcStr: ::winnt::LPCSTR, cchSrc: ::libc::c_int, lpCharType: ::minwindef::LPWORD) -> ::minwindef::BOOL; /* winnls.h:2136:1 */
    pub fn GetSystemDefaultLCID() -> ::winnt::LCID; /* winnls.h:1997:1 */
    pub fn GetSystemDefaultLangID() -> ::winnt::LANGID; /* winnls.h:1987:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetSystemDirectoryW as GetSystemDirectory; /* sysinfoapi.h:265:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetSystemDirectoryA(lpBuffer: ::winnt::LPSTR, uSize: ::minwindef::UINT) -> ::minwindef::UINT; /* sysinfoapi.h:250:1 */
    pub fn GetSystemDirectoryW(lpBuffer: ::winnt::LPWSTR, uSize: ::minwindef::UINT) -> ::minwindef::UINT; /* sysinfoapi.h:259:1 */
    pub fn GetSystemFirmwareTable(FirmwareTableProviderSignature: ::minwindef::DWORD, FirmwareTableID: ::minwindef::DWORD, pFirmwareTableBuffer: ::winnt::PVOID, BufferSize: ::minwindef::DWORD) -> ::minwindef::UINT; /* sysinfoapi.h:575:1 */
    pub fn GetSystemInfo(lpSystemInfo: ::sysinfoapi::LPSYSTEM_INFO); /* sysinfoapi.h:184:1 */
    pub fn GetSystemTimeAdjustment(lpTimeAdjustment: ::minwindef::PDWORD, lpTimeIncrement: ::minwindef::PDWORD, lpTimeAdjustmentDisabled: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* sysinfoapi.h:239:1 */
    pub fn GetSystemTimePreciseAsFileTime(lpSystemTimeAsFileTime: ::minwindef::LPFILETIME); /* sysinfoapi.h:557:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetSystemWindowsDirectoryW as GetSystemWindowsDirectory; /* sysinfoapi.h:315:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetSystemWindowsDirectoryA(lpBuffer: ::winnt::LPSTR, uSize: ::minwindef::UINT) -> ::minwindef::UINT; /* sysinfoapi.h:300:1 */
    pub fn GetSystemWindowsDirectoryW(lpBuffer: ::winnt::LPWSTR, uSize: ::minwindef::UINT) -> ::minwindef::UINT; /* sysinfoapi.h:309:1 */
    pub fn GetTapeParameters(hDevice: ::winnt::HANDLE, dwOperation: ::minwindef::DWORD, lpdwSize: ::minwindef::LPDWORD, lpTapeInformation: ::minwindef::LPVOID) -> ::minwindef::DWORD; /* winbase.h:2221:1 */
    pub fn GetTapePosition(hDevice: ::winnt::HANDLE, dwPositionType: ::minwindef::DWORD, lpdwPartition: ::minwindef::LPDWORD, lpdwOffsetLow: ::minwindef::LPDWORD, lpdwOffsetHigh: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winbase.h:2165:1 */
    pub fn GetTapeStatus(hDevice: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:2214:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTempFileNameW as GetTempFileName; /* fileapi.h:803:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTempFileNameA(lpPathName: ::winnt::LPCSTR, lpPrefixString: ::winnt::LPCSTR, uUnique: ::minwindef::UINT, lpTempFileName: ::winnt::LPSTR) -> ::minwindef::UINT; /* winbase.h:4425:1 */
    pub fn GetTempFileNameW(lpPathName: ::winnt::LPCWSTR, lpPrefixString: ::winnt::LPCWSTR, uUnique: ::minwindef::UINT, lpTempFileName: ::winnt::LPWSTR) -> ::minwindef::UINT; /* fileapi.h:794:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTempPathW as GetTempPath; /* fileapi.h:1213:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTempPathA(nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPSTR) -> ::minwindef::DWORD; /* winbase.h:4414:1 */
    pub fn GetTempPathW(nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* fileapi.h:1206:1 */
    pub fn GetThreadContext(hThread: ::winnt::HANDLE, lpContext: ::minwinbase::LPCONTEXT) -> ::minwindef::BOOL; /* processthreadsapi.h:859:1 */
    pub fn GetThreadErrorMode() -> ::minwindef::DWORD; /* winbase.h:1792:1 */
    pub fn GetThreadLocale() -> ::winnt::LCID; /* winnls.h:1963:1 */
    pub fn GetThreadPriorityBoost(hThread: ::winnt::HANDLE, pDisablePriorityBoost: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:312:1 */
    pub fn GetThreadSelectorEntry(hThread: ::winnt::HANDLE, dwSelector: ::minwindef::DWORD, lpSelectorEntry: ::winbase::LPLDT_ENTRY) -> ::minwindef::BOOL; /* winbase.h:1669:1 */
    pub fn GetThreadTimes(hThread: ::winnt::HANDLE, lpCreationTime: ::minwindef::LPFILETIME, lpExitTime: ::minwindef::LPFILETIME, lpKernelTime: ::minwindef::LPFILETIME, lpUserTime: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* processthreadsapi.h:926:1 */
    pub fn GetTickCount() -> ::minwindef::DWORD; /* sysinfoapi.h:203:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTimeFormatW as GetTimeFormat; /* datetimeapi.h:109:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTimeFormatA(Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD, lpTime: *const ::minwinbase::SYSTEMTIME, lpFormat: ::winnt::LPCSTR, lpTimeStr: ::winnt::LPSTR, cchTime: ::libc::c_int) -> ::libc::c_int; /* datetimeapi.h:86:1 */
    pub fn GetTimeFormatW(Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD, lpTime: *const ::minwinbase::SYSTEMTIME, lpFormat: ::winnt::LPCWSTR, lpTimeStr: ::winnt::LPWSTR, cchTime: ::libc::c_int) -> ::libc::c_int; /* datetimeapi.h:99:1 */
    pub fn GetUserDefaultLCID() -> ::winnt::LCID; /* winnls.h:2002:1 */
    pub fn GetUserDefaultLangID() -> ::winnt::LANGID; /* winnls.h:1992:1 */
    pub fn GetVersion() -> ::minwindef::DWORD; /* sysinfoapi.h:110:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetVersionExW as GetVersionEx; /* sysinfoapi.h:447:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetVersionExA(lpVersionInformation: ::winnt::LPOSVERSIONINFOA) -> ::minwindef::BOOL; /* sysinfoapi.h:433:1 */
    pub fn GetVersionExW(lpVersionInformation: ::winnt::LPOSVERSIONINFOW) -> ::minwindef::BOOL; /* sysinfoapi.h:442:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetVolumeInformationW as GetVolumeInformation; /* fileapi.h:842:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetVolumeInformationA(lpRootPathName: ::winnt::LPCSTR, lpVolumeNameBuffer: ::winnt::LPSTR, nVolumeNameSize: ::minwindef::DWORD, lpVolumeSerialNumber: ::minwindef::LPDWORD, lpMaximumComponentLength: ::minwindef::LPDWORD, lpFileSystemFlags: ::minwindef::LPDWORD, lpFileSystemNameBuffer: ::winnt::LPSTR, nFileSystemNameSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5825:1 */
    pub fn GetVolumeInformationW(lpRootPathName: ::winnt::LPCWSTR, lpVolumeNameBuffer: ::winnt::LPWSTR, nVolumeNameSize: ::minwindef::DWORD, lpVolumeSerialNumber: ::minwindef::LPDWORD, lpMaximumComponentLength: ::minwindef::LPDWORD, lpFileSystemFlags: ::minwindef::LPDWORD, lpFileSystemNameBuffer: ::winnt::LPWSTR, nFileSystemNameSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:829:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetVolumeNameForVolumeMountPointW as GetVolumeNameForVolumeMountPoint; /* fileapi.h:1227:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetVolumeNameForVolumeMountPointW(lpszVolumeMountPoint: ::winnt::LPCWSTR, lpszVolumeName: ::winnt::LPWSTR, cchBufferLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1219:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetVolumePathNameW as GetVolumePathName; /* fileapi.h:856:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetVolumePathNameW(lpszFileName: ::winnt::LPCWSTR, lpszVolumePathName: ::winnt::LPWSTR, cchBufferLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:848:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetWindowsDirectoryW as GetWindowsDirectory; /* sysinfoapi.h:291:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetWindowsDirectoryA(lpBuffer: ::winnt::LPSTR, uSize: ::minwindef::UINT) -> ::minwindef::UINT; /* sysinfoapi.h:275:1 */
    pub fn GetWindowsDirectoryW(lpBuffer: ::winnt::LPWSTR, uSize: ::minwindef::UINT) -> ::minwindef::UINT; /* sysinfoapi.h:285:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GlobalAddAtomW as GlobalAddAtom; /* winbase.h:3855:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GlobalAddAtomA(lpString: ::winnt::LPCSTR) -> ::minwindef::ATOM; /* winbase.h:3845:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GlobalAddAtomExW as GlobalAddAtomEx; /* winbase.h:3875:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GlobalAddAtomExA(lpString: ::winnt::LPCSTR, Flags: ::minwindef::DWORD) -> ::minwindef::ATOM; /* winbase.h:3863:1 */
    pub fn GlobalAddAtomExW(lpString: ::winnt::LPCWSTR, Flags: ::minwindef::DWORD) -> ::minwindef::ATOM; /* winbase.h:3870:1 */
    pub fn GlobalAddAtomW(lpString: ::winnt::LPCWSTR) -> ::minwindef::ATOM; /* winbase.h:3851:1 */
    pub fn GlobalAlloc(uFlags: ::minwindef::UINT, dwBytes: ::basetsd::SIZE_T) -> ::minwindef::HGLOBAL; /* winbase.h:930:1 */
    pub fn GlobalCompact(dwMinFree: ::minwindef::DWORD) -> ::basetsd::SIZE_T; /* winbase.h:994:1 */
    pub fn GlobalDeleteAtom(nAtom: ::minwindef::ATOM) -> ::minwindef::ATOM; /* winbase.h:1902:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GlobalFindAtomW as GlobalFindAtom; /* winbase.h:3893:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GlobalFindAtomA(lpString: ::winnt::LPCSTR) -> ::minwindef::ATOM; /* winbase.h:3883:1 */
    pub fn GlobalFindAtomW(lpString: ::winnt::LPCWSTR) -> ::minwindef::ATOM; /* winbase.h:3889:1 */
    pub fn GlobalFix(hMem: ::minwindef::HGLOBAL); /* winbase.h:1001:1 */
    pub fn GlobalFlags(hMem: ::minwindef::HGLOBAL) -> ::minwindef::UINT; /* winbase.h:955:1 */
    pub fn GlobalFree(hMem: ::minwindef::HGLOBAL) -> ::minwindef::HGLOBAL; /* winbase.h:987:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GlobalGetAtomNameW as GlobalGetAtomName; /* winbase.h:3915:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GlobalGetAtomNameA(nAtom: ::minwindef::ATOM, lpBuffer: ::winnt::LPSTR, nSize: ::libc::c_int) -> ::minwindef::UINT; /* winbase.h:3901:1 */
    pub fn GlobalGetAtomNameW(nAtom: ::minwindef::ATOM, lpBuffer: ::winnt::LPWSTR, nSize: ::libc::c_int) -> ::minwindef::UINT; /* winbase.h:3909:1 */
    pub fn GlobalHandle(pMem: ::minwindef::LPCVOID) -> ::minwindef::HGLOBAL; /* winbase.h:971:1 */
    pub fn GlobalLock(hMem: ::minwindef::HGLOBAL) -> ::minwindef::LPVOID; /* winbase.h:963:1 */
    pub fn GlobalMemoryStatus(lpBuffer: ::winbase::LPMEMORYSTATUS); /* winbase.h:1030:1 */
    pub fn GlobalMemoryStatusEx(lpBuffer: ::sysinfoapi::LPMEMORYSTATUSEX) -> ::minwindef::BOOL; /* sysinfoapi.h:130:1 */
    pub fn GlobalReAlloc(hMem: ::minwindef::HGLOBAL, dwBytes: ::basetsd::SIZE_T, uFlags: ::minwindef::UINT) -> ::minwindef::HGLOBAL; /* winbase.h:939:1 */
    pub fn GlobalSize(hMem: ::minwindef::HGLOBAL) -> ::basetsd::SIZE_T; /* winbase.h:948:1 */
    pub fn GlobalUnWire(hMem: ::minwindef::HGLOBAL) -> ::minwindef::BOOL; /* winbase.h:1022:1 */
    pub fn GlobalUnfix(hMem: ::minwindef::HGLOBAL); /* winbase.h:1008:1 */
    pub fn GlobalUnlock(hMem: ::minwindef::HGLOBAL) -> ::minwindef::BOOL; /* winbase.h:978:1 */
    pub fn GlobalWire(hMem: ::minwindef::HGLOBAL) -> ::minwindef::LPVOID; /* winbase.h:1015:1 */
    pub fn HeapCompact(hHeap: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD) -> ::basetsd::SIZE_T; /* heapapi.h:159:1 */
    pub fn HeapCreate(flOptions: ::minwindef::DWORD, dwInitialSize: ::basetsd::SIZE_T, dwMaximumSize: ::basetsd::SIZE_T) -> ::winnt::HANDLE; /* heapapi.h:70:1 */
    pub fn HeapDestroy(hHeap: ::winnt::HANDLE) -> ::minwindef::BOOL; /* heapapi.h:80:1 */
    pub fn HeapLock(hHeap: ::winnt::HANDLE) -> ::minwindef::BOOL; /* heapapi.h:213:1 */
    pub fn HeapQueryInformation(HeapHandle: ::winnt::HANDLE, HeapInformationClass: ::winnt::HEAP_INFORMATION_CLASS, HeapInformation: ::winnt::PVOID, HeapInformationLength: ::basetsd::SIZE_T, ReturnLength: ::basetsd::PSIZE_T) -> ::minwindef::BOOL; /* heapapi.h:249:1 */
    pub fn HeapSetInformation(HeapHandle: ::winnt::HANDLE, HeapInformationClass: ::winnt::HEAP_INFORMATION_CLASS, HeapInformation: ::winnt::PVOID, HeapInformationLength: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* heapapi.h:238:1 */
    pub fn HeapSummary(hHeap: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD, lpSummary: ::heapapi::LPHEAP_SUMMARY) -> ::minwindef::BOOL; /* heapapi.h:170:1 */
    pub fn HeapUnlock(hHeap: ::winnt::HANDLE) -> ::minwindef::BOOL; /* heapapi.h:221:1 */
    pub fn HeapValidate(hHeap: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD, lpMem: ::minwindef::LPCVOID) -> ::minwindef::BOOL; /* heapapi.h:149:1 */
    pub fn HeapWalk(hHeap: ::winnt::HANDLE, lpEntry: ::minwinbase::LPPROCESS_HEAP_ENTRY) -> ::minwindef::BOOL; /* heapapi.h:229:1 */
    pub fn InitAtomTable(nSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:1909:1 */
    pub fn InitializeCriticalSection(lpCriticalSection: ::minwinbase::LPCRITICAL_SECTION); /* synchapi.h:162:1 */
    pub fn InitializeCriticalSectionAndSpinCount(lpCriticalSection: ::minwinbase::LPCRITICAL_SECTION, dwSpinCount: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:203:1 */
    pub fn InitializeSynchronizationBarrier(lpBarrier: ::synchapi::LPSYNCHRONIZATION_BARRIER, lTotalThreads: ::winnt::LONG, lSpinCount: ::winnt::LONG) -> ::minwindef::BOOL; /* synchapi.h:884:1 */
    pub fn InstallELAMCertificateInfo(ELAMFile: ::winnt::HANDLE) -> ::minwindef::BOOL; /* sysinfoapi.h:647:1 */
    pub fn IsBadCodePtr(lpfn: ::minwindef::FARPROC) -> ::minwindef::BOOL; /* winbase.h:6433:1 */
    pub fn IsBadHugeReadPtr(lp: *const ::libc::c_void, ucb: ::basetsd::UINT_PTR) -> ::minwindef::BOOL; /* winbase.h:6417:1 */
    pub fn IsBadHugeWritePtr(lp: ::minwindef::LPVOID, ucb: ::basetsd::UINT_PTR) -> ::minwindef::BOOL; /* winbase.h:6425:1 */
    pub fn IsBadReadPtr(lp: *const ::libc::c_void, ucb: ::basetsd::UINT_PTR) -> ::minwindef::BOOL; /* winbase.h:6401:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsBadStringPtrW as IsBadStringPtr; /* winbase.h:6452:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn IsBadStringPtrA(lpsz: ::winnt::LPCSTR, ucchMax: ::basetsd::UINT_PTR) -> ::minwindef::BOOL; /* winbase.h:6440:1 */
    pub fn IsBadStringPtrW(lpsz: ::winnt::LPCWSTR, ucchMax: ::basetsd::UINT_PTR) -> ::minwindef::BOOL; /* winbase.h:6447:1 */
    pub fn IsBadWritePtr(lp: ::minwindef::LPVOID, ucb: ::basetsd::UINT_PTR) -> ::minwindef::BOOL; /* winbase.h:6409:1 */
    pub fn IsDBCSLeadByte(TestChar: ::minwindef::BYTE) -> ::minwindef::BOOL; /* winnls.h:1630:1 */
    pub fn IsDBCSLeadByteEx(CodePage: ::minwindef::UINT, TestChar: ::minwindef::BYTE) -> ::minwindef::BOOL; /* winnls.h:1637:1 */
    pub fn IsNLSDefinedString(Function: ::winnls::NLS_FUNCTION, dwFlags: ::minwindef::DWORD, lpVersionInformation: ::winnls::LPNLSVERSIONINFO, lpString: ::winnt::LPCWSTR, cchStr: ::winnt::INT) -> ::minwindef::BOOL; /* winnls.h:1883:1 */
    pub fn IsSystemResumeAutomatic() -> ::minwindef::BOOL; /* winbase.h:1662:1 */
    pub fn IsValidLocale(Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1894:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LCMapStringW as LCMapString; /* winnls.h:1483:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LCMapStringA(Locale: ::winnt::LCID, dwMapFlags: ::minwindef::DWORD, lpSrcStr: ::winnt::LPCSTR, cchSrc: ::libc::c_int, lpDestStr: ::winnt::LPSTR, cchDest: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1489:1 */
    pub fn LCMapStringW(Locale: ::winnt::LCID, dwMapFlags: ::minwindef::DWORD, lpSrcStr: ::winnt::LPCWSTR, cchSrc: ::libc::c_int, lpDestStr: ::winnt::LPWSTR, cchDest: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1475:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadLibraryW as LoadLibrary; /* winbase.h:3190:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadLibraryA(lpLibFileName: ::winnt::LPCSTR) -> ::minwindef::HMODULE; /* winbase.h:3179:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadLibraryExW as LoadLibraryEx; /* libloaderapi.h:394:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadLibraryExA(lpLibFileName: ::winnt::LPCSTR, hFile: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD) -> ::minwindef::HMODULE; /* libloaderapi.h:377:1 */
    pub fn LoadLibraryExW(lpLibFileName: ::winnt::LPCWSTR, hFile: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD) -> ::minwindef::HMODULE; /* libloaderapi.h:387:1 */
    pub fn LoadLibraryW(lpLibFileName: ::winnt::LPCWSTR) -> ::minwindef::HMODULE; /* winbase.h:3186:1 */
    pub fn LoadModule(lpModuleName: ::winnt::LPCSTR, lpParameterBlock: ::minwindef::LPVOID) -> ::minwindef::DWORD; /* winbase.h:1986:1 */
    pub fn LoadResource(hModule: ::minwindef::HMODULE, hResInfo: ::minwindef::HRSRC) -> ::minwindef::HGLOBAL; /* libloaderapi.h:417:1 */
    pub fn LocalAlloc(uFlags: ::minwindef::UINT, uBytes: ::basetsd::SIZE_T) -> ::minwindef::HLOCAL; /* winbase.h:1039:1 */
    pub fn LocalCompact(uMinFree: ::minwindef::UINT) -> ::basetsd::SIZE_T; /* winbase.h:1111:1 */
    pub fn LocalFileTimeToFileTime(lpLocalFileTime: *const ::minwindef::FILETIME, lpFileTime: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* fileapi.h:862:1 */
    pub fn LocalFlags(hMem: ::minwindef::HLOCAL) -> ::minwindef::UINT; /* winbase.h:1087:1 */
    pub fn LocalFree(hMem: ::minwindef::HLOCAL) -> ::minwindef::HLOCAL; /* winbase.h:1096:1 */
    pub fn LocalHandle(pMem: ::minwindef::LPCVOID) -> ::minwindef::HLOCAL; /* winbase.h:1066:1 */
    pub fn LocalLock(hMem: ::minwindef::HLOCAL) -> ::minwindef::LPVOID; /* winbase.h:1058:1 */
    pub fn LocalReAlloc(hMem: ::minwindef::HLOCAL, uBytes: ::basetsd::SIZE_T, uFlags: ::minwindef::UINT) -> ::minwindef::HLOCAL; /* winbase.h:1048:1 */
    pub fn LocalShrink(hMem: ::minwindef::HLOCAL, cbNewSize: ::minwindef::UINT) -> ::basetsd::SIZE_T; /* winbase.h:1103:1 */
    pub fn LocalSize(hMem: ::minwindef::HLOCAL) -> ::basetsd::SIZE_T; /* winbase.h:1080:1 */
    pub fn LocalUnlock(hMem: ::minwindef::HLOCAL) -> ::minwindef::BOOL; /* winbase.h:1073:1 */
    pub fn LockFile(hFile: ::winnt::HANDLE, dwFileOffsetLow: ::minwindef::DWORD, dwFileOffsetHigh: ::minwindef::DWORD, nNumberOfBytesToLockLow: ::minwindef::DWORD, nNumberOfBytesToLockHigh: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:871:1 */
    pub fn LockResource(hResData: ::minwindef::HGLOBAL) -> ::minwindef::LPVOID; /* libloaderapi.h:470:1 */
    pub fn MapViewOfFile(hFileMappingObject: ::winnt::HANDLE, dwDesiredAccess: ::minwindef::DWORD, dwFileOffsetHigh: ::minwindef::DWORD, dwFileOffsetLow: ::minwindef::DWORD, dwNumberOfBytesToMap: ::basetsd::SIZE_T) -> ::minwindef::LPVOID; /* memoryapi.h:276:1 */
    pub fn MapViewOfFileEx(hFileMappingObject: ::winnt::HANDLE, dwDesiredAccess: ::minwindef::DWORD, dwFileOffsetHigh: ::minwindef::DWORD, dwFileOffsetLow: ::minwindef::DWORD, dwNumberOfBytesToMap: ::basetsd::SIZE_T, lpBaseAddress: ::minwindef::LPVOID) -> ::minwindef::LPVOID; /* memoryapi.h:289:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::MoveFileW as MoveFile; /* winbase.h:5331:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn MoveFileA(lpExistingFileName: ::winnt::LPCSTR, lpNewFileName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:5319:1 */
    pub fn MoveFileW(lpExistingFileName: ::winnt::LPCWSTR, lpNewFileName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:5326:1 */
    pub fn OpenFile(lpFileName: ::winnt::LPCSTR, lpReOpenBuff: ::winbase::LPOFSTRUCT, uStyle: ::minwindef::UINT) -> ::minwindef::HFILE; /* winbase.h:2764:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenFileMappingW as OpenFileMapping; /* memoryapi.h:269:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenFileMappingA(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, lpName: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:3153:1 */
    pub fn OpenFileMappingW(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, lpName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* memoryapi.h:261:1 */
    pub fn OpenMutexA(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, lpName: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:2989:1 */
    pub fn OpenPrivateNamespaceW(lpBoundaryDescriptor: ::minwindef::LPVOID, lpAliasPrefix: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* namespaceapi.h:55:1 */
    pub fn OpenProcess(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, dwProcessId: ::minwindef::DWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:938:1 */
    pub fn OpenSemaphoreA(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, lpName: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:3028:1 */
    pub fn OpenThread(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, dwThreadId: ::minwindef::DWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:273:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PeekConsoleInputW as PeekConsoleInput; /* wincon.h:340:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PeekConsoleInputA(hConsoleInput: ::winnt::HANDLE, lpBuffer: ::wincon::PINPUT_RECORD, nLength: ::minwindef::DWORD, lpNumberOfEventsRead: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* consoleapi.h:86:1 */
    pub fn PeekConsoleInputW(hConsoleInput: ::winnt::HANDLE, lpBuffer: ::wincon::PINPUT_RECORD, nLength: ::minwindef::DWORD, lpNumberOfEventsRead: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:333:1 */
    pub fn PeekNamedPipe(hNamedPipe: ::winnt::HANDLE, lpBuffer: ::minwindef::LPVOID, nBufferSize: ::minwindef::DWORD, lpBytesRead: ::minwindef::LPDWORD, lpTotalBytesAvail: ::minwindef::LPDWORD, lpBytesLeftThisMessage: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:89:1 */
    pub fn PostQueuedCompletionStatus(CompletionPort: ::winnt::HANDLE, dwNumberOfBytesTransferred: ::minwindef::DWORD, dwCompletionKey: ::basetsd::ULONG_PTR, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* ioapiset.h:105:1 */
    pub fn PrepareTape(hDevice: ::winnt::HANDLE, dwOperation: ::minwindef::DWORD, bImmediate: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winbase.h:2176:1 */
    pub fn ProcessIdToSessionId(dwProcessId: ::minwindef::DWORD, pSessionId: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* processthreadsapi.h:677:1 */
    pub fn PulseEvent(hEvent: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:1885:1 */
    pub fn PurgeComm(hFile: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2087:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::QueryDosDeviceW as QueryDosDevice; /* fileapi.h:918:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn QueryDosDeviceA(lpDeviceName: ::winnt::LPCSTR, lpTargetPath: ::winnt::LPSTR, ucchMax: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:4767:1 */
    pub fn QueryDosDeviceW(lpDeviceName: ::winnt::LPCWSTR, lpTargetPath: ::winnt::LPWSTR, ucchMax: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:910:1 */
    pub fn QueueUserWorkItem(Function: ::minwinbase::LPTHREAD_START_ROUTINE, Context: ::winnt::PVOID, Flags: ::minwindef::ULONG) -> ::minwindef::BOOL; /* threadpoollegacyapiset.h:47:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ReadConsoleW as ReadConsole; /* consoleapi.h:123:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ReadConsoleA(hConsoleInput: ::winnt::HANDLE, lpBuffer: ::minwindef::LPVOID, nNumberOfCharsToRead: ::minwindef::DWORD, lpNumberOfCharsRead: ::minwindef::LPDWORD, pInputControl: ::wincon::PCONSOLE_READCONSOLE_CONTROL) -> ::minwindef::BOOL; /* consoleapi.h:102:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ReadConsoleInputW as ReadConsoleInput; /* consoleapi.h:151:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ReadConsoleInputA(hConsoleInput: ::winnt::HANDLE, lpBuffer: ::wincon::PINPUT_RECORD, nLength: ::minwindef::DWORD, lpNumberOfEventsRead: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* consoleapi.h:132:1 */
    pub fn ReadConsoleInputW(hConsoleInput: ::winnt::HANDLE, lpBuffer: ::wincon::PINPUT_RECORD, nLength: ::minwindef::DWORD, lpNumberOfEventsRead: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* consoleapi.h:143:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ReadConsoleOutputW as ReadConsoleOutput; /* wincon.h:388:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ReadConsoleOutputA(hConsoleOutput: ::winnt::HANDLE, lpBuffer: ::wincon::PCHAR_INFO, dwBufferSize: ::wincon::COORD, dwBufferCoord: ::wincon::COORD, lpReadRegion: ::wincon::PSMALL_RECT) -> ::minwindef::BOOL; /* wincon.h:370:1 */
    pub fn ReadConsoleOutputAttribute(hConsoleOutput: ::winnt::HANDLE, lpAttribute: ::minwindef::LPWORD, nLength: ::minwindef::DWORD, dwReadCoord: ::wincon::COORD, lpNumberOfAttrsRead: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:448:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ReadConsoleOutputCharacterW as ReadConsoleOutputCharacter; /* wincon.h:440:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ReadConsoleOutputCharacterA(hConsoleOutput: ::winnt::HANDLE, lpCharacter: ::winnt::LPSTR, nLength: ::minwindef::DWORD, dwReadCoord: ::wincon::COORD, lpNumberOfCharsRead: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:422:1 */
    pub fn ReadConsoleOutputCharacterW(hConsoleOutput: ::winnt::HANDLE, lpCharacter: ::winnt::LPWSTR, nLength: ::minwindef::DWORD, dwReadCoord: ::wincon::COORD, lpNumberOfCharsRead: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:432:1 */
    pub fn ReadConsoleOutputW(hConsoleOutput: ::winnt::HANDLE, lpBuffer: ::wincon::PCHAR_INFO, dwBufferSize: ::wincon::COORD, dwBufferCoord: ::wincon::COORD, lpReadRegion: ::wincon::PSMALL_RECT) -> ::minwindef::BOOL; /* wincon.h:380:1 */
    pub fn ReadConsoleW(hConsoleInput: ::winnt::HANDLE, lpBuffer: ::minwindef::LPVOID, nNumberOfCharsToRead: ::minwindef::DWORD, lpNumberOfCharsRead: ::minwindef::LPDWORD, pInputControl: ::wincon::PCONSOLE_READCONSOLE_CONTROL) -> ::minwindef::BOOL; /* consoleapi.h:114:1 */
    pub fn ReadFileEx(hFile: ::winnt::HANDLE, lpBuffer: ::minwindef::LPVOID, nNumberOfBytesToRead: ::minwindef::DWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED, lpCompletionRoutine: ::minwinbase::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::minwindef::BOOL; /* fileapi.h:952:1 */
    pub fn ReadFileScatter(hFile: ::winnt::HANDLE, aSegmentArray: *mut ::winnt::FILE_SEGMENT_ELEMENT, nNumberOfBytesToRead: ::minwindef::DWORD, lpReserved: ::minwindef::LPDWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* fileapi.h:965:1 */
    pub fn RemoveDllDirectory(Cookie: ::libloaderapi::DLL_DIRECTORY_COOKIE) -> ::minwindef::BOOL; /* libloaderapi.h:506:1 */
    pub fn RequestDeviceWakeup(hDevice: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:1930:1 */
    pub fn RequestWakeupLatency(latency: ::winnt::LATENCY_TIME) -> ::minwindef::BOOL; /* winbase.h:1655:1 */
    pub fn ResetWriteWatch(lpBaseAddress: ::minwindef::LPVOID, dwRegionSize: ::basetsd::SIZE_T) -> ::minwindef::UINT; /* memoryapi.h:402:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ScrollConsoleScreenBufferW as ScrollConsoleScreenBuffer; /* wincon.h:694:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ScrollConsoleScreenBufferA(hConsoleOutput: ::winnt::HANDLE, lpScrollRectangle: *const ::wincon::SMALL_RECT, lpClipRectangle: *const ::wincon::SMALL_RECT, dwDestinationOrigin: ::wincon::COORD, lpFill: *const ::wincon::CHAR_INFO) -> ::minwindef::BOOL; /* wincon.h:676:1 */
    pub fn ScrollConsoleScreenBufferW(hConsoleOutput: ::winnt::HANDLE, lpScrollRectangle: *const ::wincon::SMALL_RECT, lpClipRectangle: *const ::wincon::SMALL_RECT, dwDestinationOrigin: ::wincon::COORD, lpFill: *const ::wincon::CHAR_INFO) -> ::minwindef::BOOL; /* wincon.h:686:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SearchPathW as SearchPath; /* processenv.h:311:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SearchPathA(lpPath: ::winnt::LPCSTR, lpFileName: ::winnt::LPCSTR, lpExtension: ::winnt::LPCSTR, nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPSTR, lpFilePart: *mut *mut ::libc::c_schar) -> ::minwindef::DWORD; /* processenv.h:326:1 */
    pub fn SearchPathW(lpPath: ::winnt::LPCWSTR, lpFileName: ::winnt::LPCWSTR, lpExtension: ::winnt::LPCWSTR, nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPWSTR, lpFilePart: *mut *mut ::libc::c_ushort) -> ::minwindef::DWORD; /* processenv.h:300:1 */
    pub fn SetCommBreak(hFile: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:2095:1 */
    pub fn SetCommConfig(hCommDev: ::winnt::HANDLE, lpCC: ::winbase::LPCOMMCONFIG, dwSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2102:1 */
    pub fn SetCommMask(hFile: ::winnt::HANDLE, dwEvtMask: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2111:1 */
    pub fn SetCommState(hFile: ::winnt::HANDLE, lpDCB: ::winbase::LPDCB) -> ::minwindef::BOOL; /* winbase.h:2119:1 */
    pub fn SetCommTimeouts(hFile: ::winnt::HANDLE, lpCommTimeouts: ::winbase::LPCOMMTIMEOUTS) -> ::minwindef::BOOL; /* winbase.h:2127:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetComputerNameW as SetComputerName; /* winbase.h:6820:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetComputerNameA(lpComputerName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:6810:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetComputerNameExW as SetComputerNameEx; /* sysinfoapi.h:405:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetComputerNameEx2W as SetComputerNameEx2; /* sysinfoapi.h:631:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetComputerNameEx2W(NameType: ::sysinfoapi::COMPUTER_NAME_FORMAT, Flags: ::minwindef::DWORD, lpBuffer: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* sysinfoapi.h:623:1 */
    pub fn SetComputerNameExW(NameType: ::sysinfoapi::COMPUTER_NAME_FORMAT, lpBuffer: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* sysinfoapi.h:398:1 */
    pub fn SetComputerNameW(lpComputerName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:6816:1 */
    pub fn SetConsoleActiveScreenBuffer(hConsoleOutput: ::winnt::HANDLE) -> ::minwindef::BOOL; /* wincon.h:638:1 */
    pub fn SetConsoleCP(wCodePageID: ::minwindef::UINT) -> ::minwindef::BOOL; /* wincon.h:837:1 */
    pub fn SetConsoleCtrlHandler(HandlerRoutine: ::wincon::PHANDLER_ROUTINE, Add: ::minwindef::BOOL) -> ::minwindef::BOOL; /* consoleapi.h:159:1 */
    pub fn SetConsoleCursorInfo(hConsoleOutput: ::winnt::HANDLE, lpConsoleCursorInfo: *const ::wincon::CONSOLE_CURSOR_INFO) -> ::minwindef::BOOL; /* wincon.h:668:1 */
    pub fn SetConsoleCursorPosition(hConsoleOutput: ::winnt::HANDLE, dwCursorPosition: ::wincon::COORD) -> ::minwindef::BOOL; /* wincon.h:660:1 */
    pub fn SetConsoleMode(hConsoleHandle: ::winnt::HANDLE, dwMode: ::minwindef::DWORD) -> ::minwindef::BOOL; /* consoleapi.h:168:1 */
    pub fn SetConsoleOutputCP(wCodePageID: ::minwindef::UINT) -> ::minwindef::BOOL; /* wincon.h:844:1 */
    pub fn SetConsoleScreenBufferInfoEx(hConsoleOutput: ::winnt::HANDLE, lpConsoleScreenBufferInfoEx: ::wincon::PCONSOLE_SCREEN_BUFFER_INFOEX) -> ::minwindef::BOOL; /* wincon.h:551:1 */
    pub fn SetConsoleScreenBufferSize(hConsoleOutput: ::winnt::HANDLE, dwSize: ::wincon::COORD) -> ::minwindef::BOOL; /* wincon.h:652:1 */
    pub fn SetConsoleTextAttribute(hConsoleOutput: ::winnt::HANDLE, wAttributes: ::minwindef::WORD) -> ::minwindef::BOOL; /* wincon.h:711:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetConsoleTitleW as SetConsoleTitle; /* wincon.h:793:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetConsoleTitleA(lpConsoleTitle: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* wincon.h:783:1 */
    pub fn SetConsoleTitleW(lpConsoleTitle: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* wincon.h:789:1 */
    pub fn SetConsoleWindowInfo(hConsoleOutput: ::winnt::HANDLE, bAbsolute: ::minwindef::BOOL, lpConsoleWindow: *const ::wincon::SMALL_RECT) -> ::minwindef::BOOL; /* wincon.h:702:1 */
    pub fn SetCriticalSectionSpinCount(lpCriticalSection: ::minwinbase::LPCRITICAL_SECTION, dwSpinCount: ::minwindef::DWORD) -> ::minwindef::DWORD; /* synchapi.h:241:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetCurrentDirectoryW as SetCurrentDirectory; /* processenv.h:268:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetCurrentDirectoryA(lpPathName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* processenv.h:256:1 */
    pub fn SetCurrentDirectoryW(lpPathName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* processenv.h:263:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetDefaultCommConfigW as SetDefaultCommConfig; /* winbase.h:6774:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetDefaultCommConfigA(lpszName: ::winnt::LPCSTR, lpCC: ::winbase::LPCOMMCONFIG, dwSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:6760:1 */
    pub fn SetDefaultCommConfigW(lpszName: ::winnt::LPCWSTR, lpCC: ::winbase::LPCOMMCONFIG, dwSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:6768:1 */
    pub fn SetDefaultDllDirectories(DirectoryFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* libloaderapi.h:514:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetEnvironmentStringsW as SetEnvironmentStrings; /* processenv.h:82:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetEnvironmentStringsA(NewEnvironment: ::winnt::LPCH) -> ::minwindef::BOOL; /* winbase.h:1261:1 */
    pub fn SetEnvironmentStringsW(NewEnvironment: ::winnt::LPWCH) -> ::minwindef::BOOL; /* processenv.h:77:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetEnvironmentVariableW as SetEnvironmentVariable; /* processenv.h:222:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetEnvironmentVariableA(lpName: ::winnt::LPCSTR, lpValue: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* processenv.h:208:1 */
    pub fn SetEnvironmentVariableW(lpName: ::winnt::LPCWSTR, lpValue: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* processenv.h:216:1 */
    pub fn SetErrorMode(uMode: ::minwindef::UINT) -> ::minwindef::UINT; /* errhandlingapi.h:156:1 */
    pub fn SetFileApisToANSI(); /* winbase.h:5815:1 */
    pub fn SetFileApisToOEM(); /* winbase.h:5810:1 */
    pub fn SetFilePointer(hFile: ::winnt::HANDLE, lDistanceToMove: ::winnt::LONG, lpDistanceToMoveHigh: ::winnt::PLONG, dwMoveMethod: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:1057:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetFileShortNameW as SetFileShortName; /* winbase.h:1973:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetFileShortNameA(hFile: ::winnt::HANDLE, lpShortName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:1961:1 */
    pub fn SetFileShortNameW(hFile: ::winnt::HANDLE, lpShortName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:1968:1 */
    pub fn SetFileTime(hFile: ::winnt::HANDLE, lpCreationTime: *const ::minwindef::FILETIME, lpLastAccessTime: *const ::minwindef::FILETIME, lpLastWriteTime: *const ::minwindef::FILETIME) -> ::minwindef::BOOL; /* fileapi.h:1093:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetFirmwareEnvironmentVariableW as SetFirmwareEnvironmentVariable; /* winbase.h:3608:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetFirmwareEnvironmentVariableA(lpName: ::winnt::LPCSTR, lpGuid: ::winnt::LPCSTR, pValue: ::winnt::PVOID, nSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3592:1 */
    pub fn SetFirmwareEnvironmentVariableW(lpName: ::winnt::LPCWSTR, lpGuid: ::winnt::LPCWSTR, pValue: ::winnt::PVOID, nSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3601:1 */
    pub fn SetHandleCount(uNumber: ::minwindef::UINT) -> ::minwindef::UINT; /* winbase.h:1923:1 */
    pub fn SetHandleInformation(hObject: ::winnt::HANDLE, dwMask: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* handleapi.h:88:1 */
    pub fn SetLocalTime(lpSystemTime: *const ::minwinbase::SYSTEMTIME) -> ::minwindef::BOOL; /* sysinfoapi.h:176:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetLocaleInfoW as SetLocaleInfo; /* winnls.h:1544:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetLocaleInfoA(Locale: ::winnt::LCID, LCType: ::winnls::LCTYPE, lpLCData: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winnls.h:1532:1 */
    pub fn SetLocaleInfoW(Locale: ::winnt::LCID, LCType: ::winnls::LCTYPE, lpLCData: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winnls.h:1539:1 */
    pub fn SetMailslotInfo(hMailslot: ::winnt::HANDLE, lReadTimeout: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2458:1 */
    pub fn SetMessageWaitingIndicator(hMsgIndicator: ::winnt::HANDLE, ulMsgCount: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winbase.h:1952:1 */
    pub fn SetNamedPipeHandleState(hNamedPipe: ::winnt::HANDLE, lpMode: ::minwindef::LPDWORD, lpMaxCollectionCount: ::minwindef::LPDWORD, lpCollectDataTimeout: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:78:1 */
    pub fn SetPriorityClass(hProcess: ::winnt::HANDLE, dwPriorityClass: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:652:1 */
    pub fn SetProcessAffinityMask(hProcess: ::winnt::HANDLE, dwProcessAffinityMask: ::basetsd::DWORD_PTR) -> ::minwindef::BOOL; /* winbase.h:1219:1 */
    pub fn SetProcessShutdownParameters(dwLevel: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:542:1 */
    pub fn SetProcessWorkingSetSize(hProcess: ::winnt::HANDLE, dwMinimumWorkingSetSize: ::basetsd::SIZE_T, dwMaximumWorkingSetSize: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* winbase.h:1244:1 */
    pub fn SetProcessWorkingSetSizeEx(hProcess: ::winnt::HANDLE, dwMinimumWorkingSetSize: ::basetsd::SIZE_T, dwMaximumWorkingSetSize: ::basetsd::SIZE_T, Flags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:359:1 */
    pub fn SetSearchPathMode(Flags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:4612:1 */
    pub fn SetStdHandle(nStdHandle: ::minwindef::DWORD, hHandle: ::winnt::HANDLE) -> ::minwindef::BOOL; /* processenv.h:116:1 */
    pub fn SetSystemTime(lpSystemTime: *const ::minwinbase::SYSTEMTIME) -> ::minwindef::BOOL; /* sysinfoapi.h:411:1 */
    pub fn SetSystemTimeAdjustment(dwTimeAdjustment: ::minwindef::DWORD, bTimeAdjustmentDisabled: ::minwindef::BOOL) -> ::minwindef::BOOL; /* sysinfoapi.h:638:1 */
    pub fn SetTapeParameters(hDevice: ::winnt::HANDLE, dwOperation: ::minwindef::DWORD, lpTapeInformation: ::minwindef::LPVOID) -> ::minwindef::DWORD; /* winbase.h:2234:1 */
    pub fn SetTapePosition(hDevice: ::winnt::HANDLE, dwPositionMethod: ::minwindef::DWORD, dwPartition: ::minwindef::DWORD, dwOffsetLow: ::minwindef::DWORD, dwOffsetHigh: ::minwindef::DWORD, bImmediate: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winbase.h:2153:1 */
    pub fn SetThreadAffinityMask(hThread: ::winnt::HANDLE, dwThreadAffinityMask: ::basetsd::DWORD_PTR) -> ::basetsd::DWORD_PTR; /* winbase.h:1572:1 */
    pub fn SetThreadContext(hThread: ::winnt::HANDLE, lpContext: *const ::winnt::CONTEXT) -> ::minwindef::BOOL; /* processthreadsapi.h:868:1 */
    pub fn SetThreadErrorMode(dwNewMode: ::minwindef::DWORD, lpOldMode: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:1799:1 */
    pub fn SetThreadExecutionState(esFlags: ::winnt::EXECUTION_STATE) -> ::winnt::EXECUTION_STATE; /* winbase.h:1678:1 */
    pub fn SetThreadLocale(Locale: ::winnt::LCID) -> ::minwindef::BOOL; /* winnls.h:1968:1 */
    pub fn SetThreadPriorityBoost(hThread: ::winnt::HANDLE, bDisablePriorityBoost: ::minwindef::BOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:303:1 */
    pub fn SetThreadStackGuarantee(StackSizeInBytes: ::minwindef::PULONG) -> ::minwindef::BOOL; /* processthreadsapi.h:661:1 */
    pub fn SetThreadUILanguage(LangId: ::winnt::LANGID) -> ::winnt::LANGID; /* winnls.h:2008:1 */
    pub fn SetTimeZoneInformation(lpTimeZoneInformation: *const ::timezoneapi::TIME_ZONE_INFORMATION) -> ::minwindef::BOOL; /* timezoneapi.h:130:1 */
    pub fn SetUnhandledExceptionFilter(lpTopLevelExceptionFilter: ::errhandlingapi::LPTOP_LEVEL_EXCEPTION_FILTER) -> ::errhandlingapi::LPTOP_LEVEL_EXCEPTION_FILTER; /* errhandlingapi.h:100:1 */
    pub fn SetUserGeoID(GeoId: ::winnls::GEOID) -> ::minwindef::BOOL; /* winnls.h:1951:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetVolumeLabelW as SetVolumeLabel; /* winbase.h:5802:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetVolumeLabelA(lpRootPathName: ::winnt::LPCSTR, lpVolumeName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:5790:1 */
    pub fn SetVolumeLabelW(lpRootPathName: ::winnt::LPCWSTR, lpVolumeName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:5797:1 */
    pub fn SetupComm(hFile: ::winnt::HANDLE, dwInQueue: ::minwindef::DWORD, dwOutQueue: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2020:1 */
    pub fn SizeofResource(hModule: ::minwindef::HMODULE, hResInfo: ::minwindef::HRSRC) -> ::minwindef::DWORD; /* libloaderapi.h:478:1 */
    pub fn SwitchToThread() -> ::minwindef::BOOL; /* processthreadsapi.h:195:1 */
    pub fn TerminateProcess(hProcess: ::winnt::HANDLE, uExitCode: ::minwindef::UINT) -> ::minwindef::BOOL; /* processthreadsapi.h:177:1 */
    pub fn TerminateThread(hThread: ::winnt::HANDLE, dwExitCode: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:348:1 */
    pub fn TlsAlloc() -> ::minwindef::DWORD; /* processthreadsapi.h:460:1 */
    pub fn TlsFree(dwTlsIndex: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:485:1 */
    pub fn TlsGetValue(dwTlsIndex: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* processthreadsapi.h:468:1 */
    pub fn TlsSetValue(dwTlsIndex: ::minwindef::DWORD, lpTlsValue: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* processthreadsapi.h:476:1 */
    pub fn TransactNamedPipe(hNamedPipe: ::winnt::HANDLE, lpInBuffer: ::minwindef::LPVOID, nInBufferSize: ::minwindef::DWORD, lpOutBuffer: ::minwindef::LPVOID, nOutBufferSize: ::minwindef::DWORD, lpBytesRead: ::minwindef::LPDWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* namedpipeapi.h:102:1 */
    pub fn TransmitCommChar(hFile: ::winnt::HANDLE, cChar: ::libc::c_schar) -> ::minwindef::BOOL; /* winbase.h:2135:1 */
    pub fn UnhandledExceptionFilter(ExceptionInfo: *mut ::winnt::EXCEPTION_POINTERS) -> ::winnt::LONG; /* errhandlingapi.h:92:1 */
    pub fn UnlockFile(hFile: ::winnt::HANDLE, dwFileOffsetLow: ::minwindef::DWORD, dwFileOffsetHigh: ::minwindef::DWORD, nNumberOfBytesToUnlockLow: ::minwindef::DWORD, nNumberOfBytesToUnlockHigh: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1118:1 */
    pub fn UnregisterWaitEx(WaitHandle: ::winnt::HANDLE, CompletionEvent: ::winnt::HANDLE) -> ::minwindef::BOOL; /* threadpoollegacyapiset.h:58:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::UpdateResourceW as UpdateResource; /* winbase.h:3815:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn UpdateResourceA(hUpdate: ::winnt::HANDLE, lpType: ::winnt::LPCSTR, lpName: ::winnt::LPCSTR, wLanguage: ::minwindef::WORD, lpData: ::minwindef::LPVOID, cb: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3795:1 */
    pub fn UpdateResourceW(hUpdate: ::winnt::HANDLE, lpType: ::winnt::LPCWSTR, lpName: ::winnt::LPCWSTR, wLanguage: ::minwindef::WORD, lpData: ::minwindef::LPVOID, cb: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3806:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VerLanguageNameW as VerLanguageName; /* winver.h:178:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VerLanguageNameA(wLang: ::minwindef::DWORD, szLang: ::winnt::LPSTR, cchLang: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winver.h:165:1 */
    pub fn VerLanguageNameW(wLang: ::minwindef::DWORD, szLang: ::winnt::LPWSTR, cchLang: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winver.h:172:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VerifyVersionInfoW as VerifyVersionInfo; /* winbase.h:7345:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VerifyVersionInfoA(lpVersionInformation: ::winnt::LPOSVERSIONINFOEXA, dwTypeMask: ::minwindef::DWORD, dwlConditionMask: ::winnt::DWORDLONG) -> ::minwindef::BOOL; /* winbase.h:7331:1 */
    pub fn VerifyVersionInfoW(lpVersionInformation: ::winnt::LPOSVERSIONINFOEXW, dwTypeMask: ::minwindef::DWORD, dwlConditionMask: ::winnt::DWORDLONG) -> ::minwindef::BOOL; /* winbase.h:7339:1 */
    pub fn VirtualAlloc(lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T, flAllocationType: ::minwindef::DWORD, flProtect: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* memoryapi.h:90:1 */
    pub fn VirtualAllocEx(hProcess: ::winnt::HANDLE, lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T, flAllocationType: ::minwindef::DWORD, flProtect: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* memoryapi.h:157:1 */
    pub fn VirtualFree(lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T, dwFreeType: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:110:1 */
    pub fn VirtualFreeEx(hProcess: ::winnt::HANDLE, lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T, dwFreeType: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:181:1 */
    pub fn VirtualLock(lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* memoryapi.h:370:1 */
    pub fn VirtualProtect(lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T, flNewProtect: ::minwindef::DWORD, lpflOldProtect: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:121:1 */
    pub fn VirtualProtectEx(hProcess: ::winnt::HANDLE, lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T, flNewProtect: ::minwindef::DWORD, lpflOldProtect: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:193:1 */
    pub fn VirtualQueryEx(hProcess: ::winnt::HANDLE, lpAddress: ::minwindef::LPCVOID, lpBuffer: ::winnt::PMEMORY_BASIC_INFORMATION, dwLength: ::basetsd::SIZE_T) -> ::basetsd::SIZE_T; /* memoryapi.h:205:1 */
    pub fn VirtualUnlock(lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* memoryapi.h:379:1 */
    pub fn WaitCommEvent(hFile: ::winnt::HANDLE, lpEvtMask: ::minwindef::LPDWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* winbase.h:2143:1 */
    pub fn WaitForDebugEvent(lpDebugEvent: ::minwinbase::LPDEBUG_EVENT, dwMilliseconds: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:127:1 */
    pub fn WaitForMultipleObjects(nCount: ::minwindef::DWORD, lpHandles: *const *mut ::libc::c_void, bWaitAll: ::minwindef::BOOL, dwMilliseconds: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:1892:1 */
    pub fn WaitForSingleObject(hHandle: ::winnt::HANDLE, dwMilliseconds: ::minwindef::DWORD) -> ::minwindef::DWORD; /* synchapi.h:473:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WaitNamedPipeW as WaitNamedPipe; /* namedpipeapi.h:142:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WaitNamedPipeA(lpNamedPipeName: ::winnt::LPCSTR, nTimeOut: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5729:1 */
    pub fn WaitNamedPipeW(lpNamedPipeName: ::winnt::LPCWSTR, nTimeOut: ::minwindef::DWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:135:1 */
    pub fn WinExec(lpCmdLine: ::winnt::LPCSTR, uCmdShow: ::minwindef::UINT) -> ::minwindef::UINT; /* winbase.h:1996:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WriteConsoleW as WriteConsole; /* consoleapi.h:197:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WriteConsoleA(hConsoleOutput: ::winnt::HANDLE, lpBuffer: *const ::libc::c_void, nNumberOfCharsToWrite: ::minwindef::DWORD, lpNumberOfCharsWritten: ::minwindef::LPDWORD, lpReserved: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* consoleapi.h:177:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WriteConsoleInputW as WriteConsoleInput; /* wincon.h:362:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WriteConsoleInputA(hConsoleInput: ::winnt::HANDLE, lpBuffer: *const ::wincon::INPUT_RECORD, nLength: ::minwindef::DWORD, lpNumberOfEventsWritten: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:346:1 */
    pub fn WriteConsoleInputW(hConsoleInput: ::winnt::HANDLE, lpBuffer: *const ::wincon::INPUT_RECORD, nLength: ::minwindef::DWORD, lpNumberOfEventsWritten: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:355:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WriteConsoleOutputW as WriteConsoleOutput; /* wincon.h:414:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WriteConsoleOutputA(hConsoleOutput: ::winnt::HANDLE, lpBuffer: *const ::wincon::CHAR_INFO, dwBufferSize: ::wincon::COORD, dwBufferCoord: ::wincon::COORD, lpWriteRegion: ::wincon::PSMALL_RECT) -> ::minwindef::BOOL; /* wincon.h:396:1 */
    pub fn WriteConsoleOutputAttribute(hConsoleOutput: ::winnt::HANDLE, lpAttribute: *const ::libc::c_ushort, nLength: ::minwindef::DWORD, dwWriteCoord: ::wincon::COORD, lpNumberOfAttrsWritten: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:485:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WriteConsoleOutputCharacterW as WriteConsoleOutputCharacter; /* wincon.h:477:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WriteConsoleOutputCharacterA(hConsoleOutput: ::winnt::HANDLE, lpCharacter: ::winnt::LPCSTR, nLength: ::minwindef::DWORD, dwWriteCoord: ::wincon::COORD, lpNumberOfCharsWritten: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:459:1 */
    pub fn WriteConsoleOutputCharacterW(hConsoleOutput: ::winnt::HANDLE, lpCharacter: ::winnt::LPCWSTR, nLength: ::minwindef::DWORD, dwWriteCoord: ::wincon::COORD, lpNumberOfCharsWritten: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:469:1 */
    pub fn WriteConsoleOutputW(hConsoleOutput: ::winnt::HANDLE, lpBuffer: *const ::wincon::CHAR_INFO, dwBufferSize: ::wincon::COORD, dwBufferCoord: ::wincon::COORD, lpWriteRegion: ::wincon::PSMALL_RECT) -> ::minwindef::BOOL; /* wincon.h:406:1 */
    pub fn WriteConsoleW(hConsoleOutput: ::winnt::HANDLE, lpBuffer: *const ::libc::c_void, nNumberOfCharsToWrite: ::minwindef::DWORD, lpNumberOfCharsWritten: ::minwindef::LPDWORD, lpReserved: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* consoleapi.h:188:1 */
    pub fn WriteFileEx(hFile: ::winnt::HANDLE, lpBuffer: ::minwindef::LPCVOID, nNumberOfBytesToWrite: ::minwindef::DWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED, lpCompletionRoutine: ::minwinbase::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::minwindef::BOOL; /* fileapi.h:1168:1 */
    pub fn WriteFileGather(hFile: ::winnt::HANDLE, aSegmentArray: *mut ::winnt::FILE_SEGMENT_ELEMENT, nNumberOfBytesToWrite: ::minwindef::DWORD, lpReserved: ::minwindef::LPDWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* fileapi.h:1180:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WritePrivateProfileSectionW as WritePrivateProfileSection; /* winbase.h:4283:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WritePrivateProfileSectionA(lpAppName: ::winnt::LPCSTR, lpString: ::winnt::LPCSTR, lpFileName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4269:1 */
    pub fn WritePrivateProfileSectionW(lpAppName: ::winnt::LPCWSTR, lpString: ::winnt::LPCWSTR, lpFileName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:4277:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WritePrivateProfileStringW as WritePrivateProfileString; /* winbase.h:4213:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WritePrivateProfileStringA(lpAppName: ::winnt::LPCSTR, lpKeyName: ::winnt::LPCSTR, lpString: ::winnt::LPCSTR, lpFileName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4197:1 */
    pub fn WritePrivateProfileStringW(lpAppName: ::winnt::LPCWSTR, lpKeyName: ::winnt::LPCWSTR, lpString: ::winnt::LPCWSTR, lpFileName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:4206:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WritePrivateProfileStructW as WritePrivateProfileStruct; /* winbase.h:4406:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WritePrivateProfileStructA(lpszSection: ::winnt::LPCSTR, lpszKey: ::winnt::LPCSTR, lpStruct: ::minwindef::LPVOID, uSizeStruct: ::minwindef::UINT, szFile: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4388:1 */
    pub fn WritePrivateProfileStructW(lpszSection: ::winnt::LPCWSTR, lpszKey: ::winnt::LPCWSTR, lpStruct: ::minwindef::LPVOID, uSizeStruct: ::minwindef::UINT, szFile: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:4398:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WriteProfileSectionW as WriteProfileSection; /* winbase.h:4085:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WriteProfileSectionA(lpAppName: ::winnt::LPCSTR, lpString: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4073:1 */
    pub fn WriteProfileSectionW(lpAppName: ::winnt::LPCWSTR, lpString: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:4080:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WriteProfileStringW as WriteProfileString; /* winbase.h:4043:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WriteProfileStringA(lpAppName: ::winnt::LPCSTR, lpKeyName: ::winnt::LPCSTR, lpString: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4029:1 */
    pub fn WriteProfileStringW(lpAppName: ::winnt::LPCWSTR, lpKeyName: ::winnt::LPCWSTR, lpString: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:4037:1 */
    pub fn WriteTapemark(hDevice: ::winnt::HANDLE, dwTapemarkType: ::minwindef::DWORD, dwTapemarkCount: ::minwindef::DWORD, bImmediate: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winbase.h:2204:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::lstrcatW as lstrcat; /* winbase.h:2734:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn lstrcatA(lpString1: ::winnt::LPSTR, lpString2: ::winnt::LPCSTR) -> ::winnt::LPSTR; /* winbase.h:2722:1 */
    pub fn lstrcatW(lpString1: ::winnt::LPWSTR, lpString2: ::winnt::LPCWSTR) -> ::winnt::LPWSTR; /* winbase.h:2729:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::lstrcmpW as lstrcmp; /* winbase.h:2639:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn lstrcmpA(lpString1: ::winnt::LPCSTR, lpString2: ::winnt::LPCSTR) -> ::libc::c_int; /* winbase.h:2627:1 */
    pub fn lstrcmpW(lpString1: ::winnt::LPCWSTR, lpString2: ::winnt::LPCWSTR) -> ::libc::c_int; /* winbase.h:2634:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::lstrcmpiW as lstrcmpi; /* winbase.h:2659:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn lstrcmpiA(lpString1: ::winnt::LPCSTR, lpString2: ::winnt::LPCSTR) -> ::libc::c_int; /* winbase.h:2647:1 */
    pub fn lstrcmpiW(lpString1: ::winnt::LPCWSTR, lpString2: ::winnt::LPCWSTR) -> ::libc::c_int; /* winbase.h:2654:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::lstrcpyW as lstrcpy; /* winbase.h:2714:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn lstrcpyA(lpString1: ::winnt::LPSTR, lpString2: ::winnt::LPCSTR) -> ::winnt::LPSTR; /* winbase.h:2702:1 */
    pub fn lstrcpyW(lpString1: ::winnt::LPWSTR, lpString2: ::winnt::LPCWSTR) -> ::winnt::LPWSTR; /* winbase.h:2709:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::lstrcpynW as lstrcpyn; /* winbase.h:2689:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn lstrcpynA(lpString1: ::winnt::LPSTR, lpString2: ::winnt::LPCSTR, iMaxLength: ::libc::c_int) -> ::winnt::LPSTR; /* winbase.h:2671:1 */
    pub fn lstrcpynW(lpString1: ::winnt::LPWSTR, lpString2: ::winnt::LPCWSTR, iMaxLength: ::libc::c_int) -> ::winnt::LPWSTR; /* winbase.h:2683:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::lstrlenW as lstrlen; /* winbase.h:2756:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn lstrlenA(lpString: ::winnt::LPCSTR) -> ::libc::c_int; /* winbase.h:2746:1 */
    pub fn lstrlenW(lpString: ::winnt::LPCWSTR) -> ::libc::c_int; /* winbase.h:2752:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn RaiseException(dwExceptionCode: ::minwindef::DWORD, dwExceptionFlags: ::minwindef::DWORD, nNumberOfArguments: ::minwindef::DWORD, lpArguments: *const ::libc::c_ulonglong); /* errhandlingapi.h:73:1 */
    pub fn RtlLookupFunctionEntry(ControlPc: ::basetsd::DWORD64, ImageBase: ::basetsd::PDWORD64, HistoryTable: ::winnt::PUNWIND_HISTORY_TABLE) -> ::winnt::PRUNTIME_FUNCTION; /* winnt.h:17051:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn RtlLookupFunctionEntry(ControlPc: ::basetsd::ULONG_PTR, ImageBase: ::minwindef::PDWORD, HistoryTable: ::winnt::PUNWIND_HISTORY_TABLE) -> ::winnt::PRUNTIME_FUNCTION; /* winnt.h:17209:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86", target_arch="arm"))] 
extern "system" {
    pub fn RaiseException(dwExceptionCode: ::minwindef::DWORD, dwExceptionFlags: ::minwindef::DWORD, nNumberOfArguments: ::minwindef::DWORD, lpArguments: *const ::libc::c_ulong); /* errhandlingapi.h:73:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64", target_arch="arm"))] 
extern "system" {
    pub fn RtlUnwindEx(TargetFrame: ::winnt::PVOID, TargetIp: ::winnt::PVOID, ExceptionRecord: ::winnt::PEXCEPTION_RECORD, ReturnValue: ::winnt::PVOID, ContextRecord: ::winnt::PCONTEXT, HistoryTable: ::winnt::PUNWIND_HISTORY_TABLE); /* winnt.h:17084:1, winnt.h:17242:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::RtlCaptureStackBackTrace as CaptureStackBackTrace; /* winbase.h:112:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn CloseHandle(hObject: ::winnt::HANDLE) -> ::minwindef::BOOL; /* handleapi.h:50:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::CreateDirectoryW as CreateDirectory; /* fileapi.h:107:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn CreateDirectoryA(lpPathName: ::winnt::LPCSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::BOOL; /* fileapi.h:93:1 */
    pub fn CreateDirectoryW(lpPathName: ::winnt::LPCWSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::BOOL; /* fileapi.h:101:1 */
    pub fn CreateThread(lpThreadAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, dwStackSize: ::basetsd::SIZE_T, lpStartAddress: ::minwinbase::LPTHREAD_START_ROUTINE, lpParameter: ::minwindef::LPVOID, dwCreationFlags: ::minwindef::DWORD, lpThreadId: ::minwindef::LPDWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:209:1 */
    pub fn DecodePointer(Ptr: ::winnt::PVOID) -> ::winnt::PVOID; /* utilapiset.h:52:1 */
    pub fn DeleteCriticalSection(lpCriticalSection: ::minwinbase::LPCRITICAL_SECTION); /* synchapi.h:270:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::DeleteFileW as DeleteFile; /* fileapi.h:187:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn DeleteFileA(lpFileName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* fileapi.h:175:1 */
    pub fn DeleteFileW(lpFileName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* fileapi.h:182:1 */
    pub fn DisableThreadLibraryCalls(hLibModule: ::minwindef::HMODULE) -> ::minwindef::BOOL; /* libloaderapi.h:157:1 */
    pub fn DuplicateHandle(hSourceProcessHandle: ::winnt::HANDLE, hSourceHandle: ::winnt::HANDLE, hTargetProcessHandle: ::winnt::HANDLE, lpTargetHandle: ::minwindef::LPHANDLE, dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, dwOptions: ::minwindef::DWORD) -> ::minwindef::BOOL; /* handleapi.h:58:1 */
    pub fn EncodePointer(Ptr: ::winnt::PVOID) -> ::winnt::PVOID; /* utilapiset.h:43:1 */
    pub fn EnterCriticalSection(lpCriticalSection: ::minwinbase::LPCRITICAL_SECTION); /* synchapi.h:179:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::EnumSystemCodePagesW as EnumSystemCodePages; /* winnls.h:2283:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn EnumSystemCodePagesA(lpCodePageEnumProc: ::winnls::CODEPAGE_ENUMPROCA, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:2273:1 */
    pub fn EnumSystemCodePagesW(lpCodePageEnumProc: ::winnls::CODEPAGE_ENUMPROCW, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:2279:1 */
    pub fn EnumSystemGeoID(GeoClass: ::winnls::GEOCLASS, ParentGeoId: ::winnls::GEOID, lpGeoEnumProc: ::winnls::GEO_ENUMPROC) -> ::minwindef::BOOL; /* winnls.h:1931:1 */
    pub fn ExitThread(dwExitCode: ::minwindef::DWORD); /* processthreadsapi.h:335:1 */
    pub fn FileTimeToSystemTime(lpFileTime: *const ::minwindef::FILETIME, lpSystemTime: ::minwinbase::LPSYSTEMTIME) -> ::minwindef::BOOL; /* timezoneapi.h:95:1 */
    pub fn FindClose(hFindFile: ::winnt::HANDLE) -> ::minwindef::BOOL; /* fileapi.h:233:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::FindNextFileW as FindNextFile; /* fileapi.h:393:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn FindNextFileA(hFindFile: ::winnt::HANDLE, lpFindFileData: ::minwinbase::LPWIN32_FIND_DATAA) -> ::minwindef::BOOL; /* fileapi.h:379:1 */
    pub fn FindNextFileW(hFindFile: ::winnt::HANDLE, lpFindFileData: ::minwinbase::LPWIN32_FIND_DATAW) -> ::minwindef::BOOL; /* fileapi.h:387:1 */
    pub fn FlushFileBuffers(hFile: ::winnt::HANDLE) -> ::minwindef::BOOL; /* fileapi.h:437:1 */
    pub fn FlushViewOfFile(lpBaseAddress: ::minwindef::LPCVOID, dwNumberOfBytesToFlush: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* memoryapi.h:309:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::FormatMessageW as FormatMessage; /* winbase.h:2352:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn FormatMessageA(dwFlags: ::minwindef::DWORD, lpSource: ::minwindef::LPCVOID, dwMessageId: ::minwindef::DWORD, dwLanguageId: ::minwindef::DWORD, lpBuffer: ::winnt::LPSTR, nSize: ::minwindef::DWORD, Arguments: *mut ::libc::c_int) -> ::minwindef::DWORD; /* winbase.h:2329:1 */
    pub fn FormatMessageW(dwFlags: ::minwindef::DWORD, lpSource: ::minwindef::LPCVOID, dwMessageId: ::minwindef::DWORD, dwLanguageId: ::minwindef::DWORD, lpBuffer: ::winnt::LPWSTR, nSize: ::minwindef::DWORD, Arguments: *mut ::libc::c_int) -> ::minwindef::DWORD; /* winbase.h:2342:1 */
    pub fn FreeLibrary(hLibModule: ::minwindef::HMODULE) -> ::minwindef::BOOL; /* libloaderapi.h:213:1 */
    pub fn GetCPInfo(CodePage: ::minwindef::UINT, lpCPInfo: ::winnls::LPCPINFO) -> ::minwindef::BOOL; /* winnls.h:1376:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::GetCPInfoExW as GetCPInfoEx; /* winnls.h:1395:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn GetCPInfoExA(CodePage: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, lpCPInfoEx: ::winnls::LPCPINFOEXA) -> ::minwindef::BOOL; /* winnls.h:1383:1 */
    pub fn GetCPInfoExW(CodePage: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, lpCPInfoEx: ::winnls::LPCPINFOEXW) -> ::minwindef::BOOL; /* winnls.h:1390:1 */
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
    pub fn GetDateFormatEx(lpLocaleName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, lpDate: *const ::minwinbase::SYSTEMTIME, lpFormat: ::winnt::LPCWSTR, lpDateStr: ::winnt::LPWSTR, cchDate: ::libc::c_int, lpCalendar: ::winnt::LPCWSTR) -> ::libc::c_int; /* datetimeapi.h:144:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::GetDiskFreeSpaceExW as GetDiskFreeSpaceEx; /* fileapi.h:505:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn GetDiskFreeSpaceExA(lpDirectoryName: ::winnt::LPCSTR, lpFreeBytesAvailableToCaller: ::winnt::PULARGE_INTEGER, lpTotalNumberOfBytes: ::winnt::PULARGE_INTEGER, lpTotalNumberOfFreeBytes: ::winnt::PULARGE_INTEGER) -> ::minwindef::BOOL; /* fileapi.h:487:1 */
    pub fn GetDiskFreeSpaceExW(lpDirectoryName: ::winnt::LPCWSTR, lpFreeBytesAvailableToCaller: ::winnt::PULARGE_INTEGER, lpTotalNumberOfBytes: ::winnt::PULARGE_INTEGER, lpTotalNumberOfFreeBytes: ::winnt::PULARGE_INTEGER) -> ::minwindef::BOOL; /* fileapi.h:497:1 */
    pub fn GetExitCodeThread(hThread: ::winnt::HANDLE, lpExitCode: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:364:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::GetFileAttributesExW as GetFileAttributesEx; /* fileapi.h:599:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn GetFileAttributesExA(lpFileName: ::winnt::LPCSTR, fInfoLevelId: ::minwinbase::GET_FILEEX_INFO_LEVELS, lpFileInformation: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* fileapi.h:583:1 */
    pub fn GetFileAttributesExW(lpFileName: ::winnt::LPCWSTR, fInfoLevelId: ::minwinbase::GET_FILEEX_INFO_LEVELS, lpFileInformation: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* fileapi.h:592:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::GetGeoInfoW as GetGeoInfo; /* winnls.h:1923:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn GetGeoInfoA(Location: ::winnls::GEOID, GeoType: ::winnls::GEOTYPE, lpGeoData: ::winnt::LPSTR, cchData: ::libc::c_int, LangId: ::winnt::LANGID) -> ::libc::c_int; /* winnls.h:1907:1 */
    pub fn GetGeoInfoW(Location: ::winnls::GEOID, GeoType: ::winnls::GEOTYPE, lpGeoData: ::winnt::LPWSTR, cchData: ::libc::c_int, LangId: ::winnt::LANGID) -> ::libc::c_int; /* winnls.h:1916:1 */
    pub fn GetLastError() -> ::minwindef::DWORD; /* errhandlingapi.h:118:1 */
    pub fn GetLocalTime(lpSystemTime: ::minwinbase::LPSYSTEMTIME); /* sysinfoapi.h:161:1 */
    pub fn GetOverlappedResultEx(hFile: ::winnt::HANDLE, lpOverlapped: ::minwinbase::LPOVERLAPPED, lpNumberOfBytesTransferred: ::minwindef::LPDWORD, dwMilliseconds: ::minwindef::DWORD, bAlertable: ::minwindef::BOOL) -> ::minwindef::BOOL; /* ioapiset.h:159:1 */
    pub fn GetProcAddress(hModule: ::minwindef::HMODULE, lpProcName: ::winnt::LPCSTR) -> ::minwindef::FARPROC; /* libloaderapi.h:360:1 */
    pub fn GetProcessHeap() -> ::winnt::HANDLE; /* heapapi.h:189:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::GetStringTypeExW as GetStringTypeEx; /* stringapiset.h:130:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn GetStringTypeExW(Locale: ::winnt::LCID, dwInfoType: ::minwindef::DWORD, lpSrcStr: ::winnt::LPCWCH, cchSrc: ::libc::c_int, lpCharType: ::minwindef::LPWORD) -> ::minwindef::BOOL; /* stringapiset.h:121:1 */
    pub fn GetStringTypeW(dwInfoType: ::minwindef::DWORD, lpSrcStr: ::winnt::LPCWCH, cchSrc: ::libc::c_int, lpCharType: ::minwindef::LPWORD) -> ::minwindef::BOOL; /* stringapiset.h:136:1 */
    pub fn GetSystemTime(lpSystemTime: ::minwinbase::LPSYSTEMTIME); /* sysinfoapi.h:145:1 */
    pub fn GetSystemTimeAsFileTime(lpSystemTimeAsFileTime: ::minwindef::LPFILETIME); /* sysinfoapi.h:153:1 */
    pub fn GetThreadPriority(hThread: ::winnt::HANDLE) -> ::libc::c_int; /* processthreadsapi.h:326:1 */
    pub fn GetTimeFormatEx(lpLocaleName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, lpTime: *const ::minwinbase::SYSTEMTIME, lpFormat: ::winnt::LPCWSTR, lpTimeStr: ::winnt::LPWSTR, cchTime: ::libc::c_int) -> ::libc::c_int; /* datetimeapi.h:131:1 */
    pub fn GetTimeZoneInformation(lpTimeZoneInformation: ::timezoneapi::LPTIME_ZONE_INFORMATION) -> ::minwindef::DWORD; /* timezoneapi.h:115:1 */
    pub fn GetUserGeoID(GeoClass: ::winnls::GEOCLASS) -> ::winnls::GEOID; /* winnls.h:1939:1 */
    pub fn HeapAlloc(hHeap: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD, dwBytes: ::basetsd::SIZE_T) -> ::minwindef::LPVOID; /* heapapi.h:97:1 */
    pub fn HeapFree(hHeap: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD, lpMem: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* heapapi.h:122:1 */
    pub fn HeapReAlloc(hHeap: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD, lpMem: ::minwindef::LPVOID, dwBytes: ::basetsd::SIZE_T) -> ::minwindef::LPVOID; /* heapapi.h:110:1 */
    pub fn HeapSize(hHeap: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD, lpMem: ::minwindef::LPCVOID) -> ::basetsd::SIZE_T; /* heapapi.h:132:1 */
    pub fn IsProcessorFeaturePresent(ProcessorFeature: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:954:1 */
    pub fn IsValidCodePage(CodePage: ::minwindef::UINT) -> ::minwindef::BOOL; /* winnls.h:1348:1 */
    pub fn LeaveCriticalSection(lpCriticalSection: ::minwinbase::LPCRITICAL_SECTION); /* synchapi.h:187:1 */
    pub fn LockFileEx(hFile: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD, dwReserved: ::minwindef::DWORD, nNumberOfBytesToLockLow: ::minwindef::DWORD, nNumberOfBytesToLockHigh: ::minwindef::DWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* fileapi.h:890:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::MoveFileExW as MoveFileEx; /* winbase.h:5379:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn MoveFileExA(lpExistingFileName: ::winnt::LPCSTR, lpNewFileName: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5365:1 */
    pub fn MoveFileExW(lpExistingFileName: ::winnt::LPCWSTR, lpNewFileName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5373:1 */
    pub fn MulDiv(nNumber: ::libc::c_int, nNumerator: ::libc::c_int, nDenominator: ::libc::c_int) -> ::libc::c_int; /* winbase.h:2252:1 */
    pub fn MultiByteToWideChar(CodePage: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, lpMultiByteStr: ::winnt::LPCCH, cbMultiByte: ::libc::c_int, lpWideCharStr: ::winnt::LPWSTR, cchWideChar: ::libc::c_int) -> ::libc::c_int; /* stringapiset.h:154:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::OpenEventW as OpenEvent; /* synchapi.h:642:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn OpenEventA(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, lpName: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* synchapi.h:625:1 */
    pub fn OpenEventW(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, lpName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* synchapi.h:635:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::OpenMutexW as OpenMutex; /* synchapi.h:576:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn OpenMutexW(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, lpName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* synchapi.h:568:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::OpenSemaphoreW as OpenSemaphore; /* synchapi.h:659:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn OpenSemaphoreW(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, lpName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* synchapi.h:651:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::OutputDebugStringW as OutputDebugString; /* debugapi.h:97:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn OutputDebugStringA(lpOutputString: ::winnt::LPCSTR); /* debugapi.h:85:1 */
    pub fn OutputDebugStringW(lpOutputString: ::winnt::LPCWSTR); /* debugapi.h:92:1 */
    pub fn RaiseFailFastException(pExceptionRecord: ::winnt::PEXCEPTION_RECORD, pContextRecord: ::winnt::PCONTEXT, dwFlags: ::minwindef::DWORD); /* winbase.h:1277:1 */
    pub fn ReadFile(hFile: ::winnt::HANDLE, lpBuffer: ::minwindef::LPVOID, nNumberOfBytesToRead: ::minwindef::DWORD, lpNumberOfBytesRead: ::minwindef::LPDWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* fileapi.h:932:1 */
    pub fn ReleaseMutex(hMutex: ::winnt::HANDLE) -> ::minwindef::BOOL; /* synchapi.h:458:1 */
    pub fn ReleaseSemaphore(hSemaphore: ::winnt::HANDLE, lReleaseCount: ::winnt::LONG, lpPreviousCount: ::minwindef::LPLONG) -> ::minwindef::BOOL; /* synchapi.h:448:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::RemoveDirectoryW as RemoveDirectory; /* fileapi.h:996:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn RemoveDirectoryA(lpPathName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* fileapi.h:984:1 */
    pub fn RemoveDirectoryW(lpPathName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* fileapi.h:991:1 */
    pub fn ResetEvent(hEvent: ::winnt::HANDLE) -> ::minwindef::BOOL; /* synchapi.h:440:1 */
    pub fn ResumeThread(hThread: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:381:1 */
    pub fn RtlPcToFileHeader(PcValue: ::winnt::PVOID, BaseOfImage: *mut *mut ::libc::c_void) -> ::winnt::PVOID; /* winnt.h:17417:1 */
    pub fn RtlUnwind(TargetFrame: ::winnt::PVOID, TargetIp: ::winnt::PVOID, ExceptionRecord: ::winnt::PEXCEPTION_RECORD, ReturnValue: ::winnt::PVOID); /* winnt.h:16953:1 */
    pub fn SetEndOfFile(hFile: ::winnt::HANDLE) -> ::minwindef::BOOL; /* fileapi.h:1004:1 */
    pub fn SetEvent(hEvent: ::winnt::HANDLE) -> ::minwindef::BOOL; /* synchapi.h:432:1 */
}
#[cfg(feature="winapi_app")] #[doc(inline)] pub use self::SetFileAttributesW as SetFileAttributes; /* fileapi.h:1026:9 */
#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn SetFileAttributesA(lpFileName: ::winnt::LPCSTR, dwFileAttributes: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1012:1 */
    pub fn SetFileAttributesW(lpFileName: ::winnt::LPCWSTR, dwFileAttributes: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1020:1 */
    pub fn SetFilePointerEx(hFile: ::winnt::HANDLE, liDistanceToMove: ::winnt::LARGE_INTEGER, lpNewFilePointer: ::winnt::PLARGE_INTEGER, dwMoveMethod: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1075:1 */
    pub fn SetLastError(dwErrCode: ::minwindef::DWORD); /* errhandlingapi.h:128:1 */
    pub fn SetThreadPriority(hThread: ::winnt::HANDLE, nPriority: ::libc::c_int) -> ::minwindef::BOOL; /* processthreadsapi.h:288:1 */
    pub fn Sleep(dwMilliseconds: ::minwindef::DWORD); /* synchapi.h:908:1 */
    pub fn SleepEx(dwMilliseconds: ::minwindef::DWORD, bAlertable: ::minwindef::BOOL) -> ::minwindef::DWORD; /* synchapi.h:489:1 */
    pub fn SuspendThread(hThread: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:373:1 */
    pub fn SystemTimeToFileTime(lpSystemTime: *const ::minwinbase::SYSTEMTIME, lpFileTime: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* timezoneapi.h:105:1 */
    pub fn SystemTimeToTzSpecificLocalTime(lpTimeZoneInformation: *const ::timezoneapi::TIME_ZONE_INFORMATION, lpUniversalTime: *const ::minwinbase::SYSTEMTIME, lpLocalTime: ::minwinbase::LPSYSTEMTIME) -> ::minwindef::BOOL; /* timezoneapi.h:73:1 */
    pub fn TzSpecificLocalTimeToSystemTime(lpTimeZoneInformation: *const ::timezoneapi::TIME_ZONE_INFORMATION, lpLocalTime: *const ::minwinbase::SYSTEMTIME, lpUniversalTime: ::minwinbase::LPSYSTEMTIME) -> ::minwindef::BOOL; /* timezoneapi.h:84:1 */
    pub fn UnlockFileEx(hFile: ::winnt::HANDLE, dwReserved: ::minwindef::DWORD, nNumberOfBytesToUnlockLow: ::minwindef::DWORD, nNumberOfBytesToUnlockHigh: ::minwindef::DWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* fileapi.h:1137:1 */
    pub fn UnmapViewOfFile(lpBaseAddress: ::minwindef::LPCVOID) -> ::minwindef::BOOL; /* memoryapi.h:318:1 */
    pub fn VirtualQuery(lpAddress: ::minwindef::LPCVOID, lpBuffer: ::winnt::PMEMORY_BASIC_INFORMATION, dwLength: ::basetsd::SIZE_T) -> ::basetsd::SIZE_T; /* memoryapi.h:139:1 */
    pub fn WaitForMultipleObjectsEx(nCount: ::minwindef::DWORD, lpHandles: *const *mut ::libc::c_void, bWaitAll: ::minwindef::BOOL, dwMilliseconds: ::minwindef::DWORD, bAlertable: ::minwindef::BOOL) -> ::minwindef::DWORD; /* synchapi.h:508:1 */
    pub fn WaitForSingleObjectEx(hHandle: ::winnt::HANDLE, dwMilliseconds: ::minwindef::DWORD, bAlertable: ::minwindef::BOOL) -> ::minwindef::DWORD; /* synchapi.h:498:1 */
    pub fn WideCharToMultiByte(CodePage: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, lpWideCharStr: ::winnt::LPCWCH, cchWideChar: ::libc::c_int, lpMultiByteStr: ::winnt::LPSTR, cbMultiByte: ::libc::c_int, lpDefaultChar: ::winnt::LPCCH, lpUsedDefaultChar: ::minwindef::LPBOOL) -> ::libc::c_int; /* stringapiset.h:169:1 */
    pub fn WriteFile(hFile: ::winnt::HANDLE, lpBuffer: ::minwindef::LPCVOID, nNumberOfBytesToWrite: ::minwindef::DWORD, lpNumberOfBytesWritten: ::minwindef::LPDWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* fileapi.h:1149:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn CancelWaitableTimer(hTimer: ::winnt::HANDLE) -> ::minwindef::BOOL; /* synchapi.h:729:1 */
    pub fn ConvertThreadToFiber(lpParameter: ::minwindef::LPVOID) -> ::minwindef::LPVOID; /* winbase.h:1377:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::CopyFileExW as CopyFileEx; /* winbase.h:5128:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn CopyFileExA(lpExistingFileName: ::winnt::LPCSTR, lpNewFileName: ::winnt::LPCSTR, lpProgressRoutine: ::winbase::LPPROGRESS_ROUTINE, lpData: ::minwindef::LPVOID, pbCancel: ::minwindef::LPBOOL, dwCopyFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5106:1 */
    pub fn CopyFileExW(lpExistingFileName: ::winnt::LPCWSTR, lpNewFileName: ::winnt::LPCWSTR, lpProgressRoutine: ::winbase::LPPROGRESS_ROUTINE, lpData: ::minwindef::LPVOID, pbCancel: ::minwindef::LPBOOL, dwCopyFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5118:1 */
    pub fn CreateFiber(dwStackSize: ::basetsd::SIZE_T, lpStartAddress: ::winbase::LPFIBER_START_ROUTINE, lpParameter: ::minwindef::LPVOID) -> ::minwindef::LPVOID; /* winbase.h:1367:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::CreateWaitableTimerW as CreateWaitableTimer; /* winbase.h:3058:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn CreateWaitableTimerA(lpTimerAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, bManualReset: ::minwindef::BOOL, lpTimerName: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:3043:1 */
    pub fn CreateWaitableTimerW(lpTimerAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, bManualReset: ::minwindef::BOOL, lpTimerName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winbase.h:3052:1 */
    pub fn GetSystemPowerStatus(lpSystemPowerStatus: ::winbase::LPSYSTEM_POWER_STATUS) -> ::minwindef::BOOL; /* winbase.h:7404:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::OpenWaitableTimerW as OpenWaitableTimer; /* synchapi.h:692:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn OpenWaitableTimerA(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, lpTimerName: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:3067:1 */
    pub fn OpenWaitableTimerW(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, lpTimerName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* synchapi.h:684:1 */
    pub fn QueueUserAPC(pfnAPC: ::winnt::PAPCFUNC, hThread: ::winnt::HANDLE, dwData: ::basetsd::ULONG_PTR) -> ::minwindef::DWORD; /* processthreadsapi.h:114:1 */
    pub fn ReadDirectoryChangesW(hDirectory: ::winnt::HANDLE, lpBuffer: ::minwindef::LPVOID, nBufferLength: ::minwindef::DWORD, bWatchSubtree: ::minwindef::BOOL, dwNotifyFilter: ::minwindef::DWORD, lpBytesReturned: ::minwindef::LPDWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED, lpCompletionRoutine: ::minwinbase::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::minwindef::BOOL; /* winbase.h:6368:1 */
    pub fn SetSystemPowerState(fSuspend: ::minwindef::BOOL, fForce: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:7411:1 */
    pub fn SetWaitableTimer(hTimer: ::winnt::HANDLE, lpDueTime: *const ::winnt::LARGE_INTEGER, lPeriod: ::winnt::LONG, pfnCompletionRoutine: ::synchapi::PTIMERAPCROUTINE, lpArgToCompletionRoutine: ::minwindef::LPVOID, fResume: ::minwindef::BOOL) -> ::minwindef::BOOL; /* synchapi.h:716:1 */
    pub fn SignalObjectAndWait(hObjectToSignal: ::winnt::HANDLE, hObjectToWaitOn: ::winnt::HANDLE, dwMilliseconds: ::minwindef::DWORD, bAlertable: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winbase.h:2851:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn ConvertThreadToFiberEx(lpParameter: ::minwindef::LPVOID, dwFlags: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* winbase.h:1352:1 */
    pub fn CreateFiberEx(dwStackCommitSize: ::basetsd::SIZE_T, dwStackReserveSize: ::basetsd::SIZE_T, dwFlags: ::minwindef::DWORD, lpStartAddress: ::winbase::LPFIBER_START_ROUTINE, lpParameter: ::minwindef::LPVOID) -> ::minwindef::LPVOID; /* winbase.h:1340:1 */
    pub fn DeleteFiber(lpFiber: ::minwindef::LPVOID); /* winbase.h:1307:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::FindFirstFileExW as FindFirstFileEx; /* fileapi.h:334:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn FindFirstFileExA(lpFileName: ::winnt::LPCSTR, fInfoLevelId: ::minwinbase::FINDEX_INFO_LEVELS, lpFindFileData: ::minwindef::LPVOID, fSearchOp: ::minwinbase::FINDEX_SEARCH_OPS, lpSearchFilter: ::minwindef::LPVOID, dwAdditionalFlags: ::minwindef::DWORD) -> ::winnt::HANDLE; /* fileapi.h:312:1 */
    pub fn FindFirstFileExW(lpFileName: ::winnt::LPCWSTR, fInfoLevelId: ::minwinbase::FINDEX_INFO_LEVELS, lpFindFileData: ::minwindef::LPVOID, fSearchOp: ::minwinbase::FINDEX_SEARCH_OPS, lpSearchFilter: ::minwindef::LPVOID, dwAdditionalFlags: ::minwindef::DWORD) -> ::winnt::HANDLE; /* fileapi.h:324:1 */
    pub fn IsDebuggerPresent() -> ::minwindef::BOOL; /* debugapi.h:54:1 */
    pub fn SetThreadIdealProcessor(hThread: ::winnt::HANDLE, dwIdealProcessor: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:1585:1 */
    pub fn SwitchToFiber(lpFiber: ::minwindef::LPVOID); /* winbase.h:1300:1 */
    pub fn TryEnterCriticalSection(lpCriticalSection: ::minwinbase::LPCRITICAL_SECTION) -> ::minwindef::BOOL; /* synchapi.h:260:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn ActivateActCtx(hActCtx: ::winnt::HANDLE, lpCookie: *mut ::libc::c_ulonglong) -> ::minwindef::BOOL; /* winbase.h:7767:1 */
    pub fn QueryActCtxW(dwFlags: ::minwindef::DWORD, hActCtx: ::winnt::HANDLE, pvSubInstance: ::winnt::PVOID, ulInfoClass: ::minwindef::ULONG, pvBuffer: ::winnt::PVOID, cbBuffer: ::basetsd::SIZE_T, pcbWrittenOrRequired: *mut ::libc::c_ulonglong) -> ::minwindef::BOOL; /* winbase.h:7931:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[cfg(any(target_arch="x86", target_arch="arm"))] 
extern "system" {
    pub fn ActivateActCtx(hActCtx: ::winnt::HANDLE, lpCookie: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winbase.h:7767:1 */
    pub fn QueryActCtxW(dwFlags: ::minwindef::DWORD, hActCtx: ::winnt::HANDLE, pvSubInstance: ::winnt::PVOID, ulInfoClass: ::minwindef::ULONG, pvBuffer: ::winnt::PVOID, cbBuffer: ::basetsd::SIZE_T, pcbWrittenOrRequired: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winbase.h:7931:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn AddRefActCtx(hActCtx: ::winnt::HANDLE); /* winbase.h:7743:1 */
    pub fn AssignProcessToJobObject(hJob: ::winnt::HANDLE, hProcess: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:7481:1 */
    pub fn AttachConsole(dwProcessId: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wincon.h:733:1 */
    pub fn BindIoCompletionCallback(FileHandle: ::winnt::HANDLE, Function: ::minwinbase::LPOVERLAPPED_COMPLETION_ROUTINE, Flags: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winbase.h:7084:1 */
    pub fn CancelTimerQueueTimer(TimerQueue: ::winnt::HANDLE, Timer: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:7106:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::CreateActCtxW as CreateActCtx; /* winbase.h:7735:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn CreateActCtxA(pActCtx: ::winbase::PCACTCTXA) -> ::winnt::HANDLE; /* winbase.h:7725:1 */
    pub fn CreateActCtxW(pActCtx: ::winbase::PCACTCTXW) -> ::winnt::HANDLE; /* winbase.h:7731:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::CreateHardLinkW as CreateHardLink; /* winbase.h:5524:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn CreateHardLinkA(lpFileName: ::winnt::LPCSTR, lpExistingFileName: ::winnt::LPCSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::BOOL; /* winbase.h:5510:1 */
    pub fn CreateHardLinkW(lpFileName: ::winnt::LPCWSTR, lpExistingFileName: ::winnt::LPCWSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::BOOL; /* winbase.h:5518:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::CreateJobObjectW as CreateJobObject; /* winbase.h:7449:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn CreateJobObjectA(lpJobAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpName: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:7436:1 */
    pub fn CreateJobObjectW(lpJobAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winbase.h:7444:1 */
    pub fn CreateJobSet(NumJob: ::minwindef::ULONG, UserJobSet: ::winnt::PJOB_SET_ARRAY, Flags: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winbase.h:7518:1 */
    pub fn DeactivateActCtx(dwFlags: ::minwindef::DWORD, ulCookie: ::basetsd::ULONG_PTR) -> ::minwindef::BOOL; /* winbase.h:7779:1 */
    pub fn DeleteTimerQueue(TimerQueue: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:7115:1 */
    pub fn DeleteVolumeMountPointA(lpszVolumeMountPoint: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:7620:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::DnsHostnameToComputerNameW as DnsHostnameToComputerName; /* winbase.h:6859:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn DnsHostnameToComputerNameA(Hostname: ::winnt::LPCSTR, ComputerName: ::winnt::LPSTR, nSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6844:1 */
    pub fn DnsHostnameToComputerNameW(Hostname: ::winnt::LPCWSTR, ComputerName: ::winnt::LPWSTR, nSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6853:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumCalendarInfoExW as EnumCalendarInfoEx; /* winnls.h:1788:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumCalendarInfoExA(lpCalInfoEnumProcEx: ::winnls::CALINFO_ENUMPROCEXA, Locale: ::winnt::LCID, Calendar: ::winnls::CALID, CalType: ::winnls::CALTYPE) -> ::minwindef::BOOL; /* winnls.h:1773:1 */
    pub fn EnumCalendarInfoExW(lpCalInfoEnumProcEx: ::winnls::CALINFO_ENUMPROCEXW, Locale: ::winnt::LCID, Calendar: ::winnls::CALID, CalType: ::winnls::CALTYPE) -> ::minwindef::BOOL; /* winnls.h:1782:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumDateFormatsExW as EnumDateFormatsEx; /* winnls.h:1856:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumDateFormatsExA(lpDateFmtEnumProcEx: ::winnls::DATEFMT_ENUMPROCEXA, Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1843:1 */
    pub fn EnumDateFormatsExW(lpDateFmtEnumProcEx: ::winnls::DATEFMT_ENUMPROCEXW, Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1851:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumLanguageGroupLocalesW as EnumLanguageGroupLocales; /* winnls.h:2238:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumLanguageGroupLocalesA(lpLangGroupLocaleEnumProc: ::winnls::LANGGROUPLOCALE_ENUMPROCA, LanguageGroup: ::winnls::LGRPID, dwFlags: ::minwindef::DWORD, lParam: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winnls.h:2224:1 */
    pub fn EnumLanguageGroupLocalesW(lpLangGroupLocaleEnumProc: ::winnls::LANGGROUPLOCALE_ENUMPROCW, LanguageGroup: ::winnls::LGRPID, dwFlags: ::minwindef::DWORD, lParam: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winnls.h:2232:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumSystemLanguageGroupsW as EnumSystemLanguageGroups; /* winnls.h:2216:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumSystemLanguageGroupsA(lpLanguageGroupEnumProc: ::winnls::LANGUAGEGROUP_ENUMPROCA, dwFlags: ::minwindef::DWORD, lParam: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winnls.h:2204:1 */
    pub fn EnumSystemLanguageGroupsW(lpLanguageGroupEnumProc: ::winnls::LANGUAGEGROUP_ENUMPROCW, dwFlags: ::minwindef::DWORD, lParam: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winnls.h:2211:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumSystemLocalesW as EnumSystemLocales; /* winnls.h:2195:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumSystemLocalesA(lpLocaleEnumProc: ::winnls::LOCALE_ENUMPROCA, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:2185:1 */
    pub fn EnumSystemLocalesW(lpLocaleEnumProc: ::winnls::LOCALE_ENUMPROCW, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:2191:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumUILanguagesW as EnumUILanguages; /* winnls.h:2258:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumUILanguagesA(lpUILanguageEnumProc: ::winnls::UILANGUAGE_ENUMPROCA, dwFlags: ::minwindef::DWORD, lParam: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winnls.h:2246:1 */
    pub fn EnumUILanguagesW(lpUILanguageEnumProc: ::winnls::UILANGUAGE_ENUMPROCW, dwFlags: ::minwindef::DWORD, lParam: ::basetsd::LONG_PTR) -> ::minwindef::BOOL; /* winnls.h:2253:1 */
    pub fn FindActCtxSectionGuid(dwFlags: ::minwindef::DWORD, lpExtensionGuid: *const ::guiddef::GUID, ulSectionId: ::minwindef::ULONG, lpGuidToFind: *const ::guiddef::GUID, ReturnedData: ::winbase::PACTCTX_SECTION_KEYED_DATA) -> ::minwindef::BOOL; /* winbase.h:7868:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::FindActCtxSectionStringW as FindActCtxSectionString; /* winbase.h:7860:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn FindActCtxSectionStringA(dwFlags: ::minwindef::DWORD, lpExtensionGuid: *const ::guiddef::GUID, ulSectionId: ::minwindef::ULONG, lpStringToFind: ::winnt::LPCSTR, ReturnedData: ::winbase::PACTCTX_SECTION_KEYED_DATA) -> ::minwindef::BOOL; /* winbase.h:7841:1 */
    pub fn FindActCtxSectionStringW(dwFlags: ::minwindef::DWORD, lpExtensionGuid: *const ::guiddef::GUID, ulSectionId: ::minwindef::ULONG, lpStringToFind: ::winnt::LPCWSTR, ReturnedData: ::winbase::PACTCTX_SECTION_KEYED_DATA) -> ::minwindef::BOOL; /* winbase.h:7852:1 */
    pub fn FindFirstVolumeA(lpszVolumeName: ::winnt::LPSTR, cchBufferLength: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:7526:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::FindFirstVolumeMountPointW as FindFirstVolumeMountPoint; /* winbase.h:7563:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn FindFirstVolumeMountPointA(lpszRootPathName: ::winnt::LPCSTR, lpszVolumeMountPoint: ::winnt::LPSTR, cchBufferLength: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:7549:1 */
    pub fn FindFirstVolumeMountPointW(lpszRootPathName: ::winnt::LPCWSTR, lpszVolumeMountPoint: ::winnt::LPWSTR, cchBufferLength: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:7557:1 */
    pub fn FindNextVolumeA(hFindVolume: ::winnt::HANDLE, lpszVolumeName: ::winnt::LPSTR, cchBufferLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:7537:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::FindNextVolumeMountPointW as FindNextVolumeMountPoint; /* winbase.h:7585:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn FindNextVolumeMountPointA(hFindVolumeMountPoint: ::winnt::HANDLE, lpszVolumeMountPoint: ::winnt::LPSTR, cchBufferLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:7571:1 */
    pub fn FindNextVolumeMountPointW(hFindVolumeMountPoint: ::winnt::HANDLE, lpszVolumeMountPoint: ::winnt::LPWSTR, cchBufferLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:7579:1 */
    pub fn FindVolumeMountPointClose(hFindVolumeMountPoint: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:7593:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::GetCalendarInfoW as GetCalendarInfo; /* winnls.h:1574:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn GetCalendarInfoA(Locale: ::winnt::LCID, Calendar: ::winnls::CALID, CalType: ::winnls::CALTYPE, lpCalData: ::winnt::LPSTR, cchData: ::libc::c_int, lpValue: ::minwindef::LPDWORD) -> ::libc::c_int; /* winnls.h:1555:1 */
    pub fn GetCalendarInfoW(Locale: ::winnt::LCID, Calendar: ::winnls::CALID, CalType: ::winnls::CALTYPE, lpCalData: ::winnt::LPWSTR, cchData: ::libc::c_int, lpValue: ::minwindef::LPDWORD) -> ::libc::c_int; /* winnls.h:1566:1 */
    pub fn GetConsoleDisplayMode(lpModeFlags: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:856:1 */
    pub fn GetConsoleFontSize(hConsoleOutput: ::winnt::HANDLE, nFont: ::minwindef::DWORD) -> ::wincon::COORD; /* wincon.h:614:1 */
    pub fn GetConsoleHistoryInfo(lpConsoleHistoryInfo: ::wincon::PCONSOLE_HISTORY_INFO) -> ::minwindef::BOOL; /* wincon.h:602:1 */
    pub fn GetConsoleSelectionInfo(lpConsoleSelectionInfo: ::wincon::PCONSOLE_SELECTION_INFO) -> ::minwindef::BOOL; /* wincon.h:622:1 */
    pub fn GetConsoleWindow() -> ::windef::HWND; /* wincon.h:872:1 */
    pub fn GetCurrentActCtx(lphActCtx: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winbase.h:7787:1 */
    pub fn GetCurrentConsoleFont(hConsoleOutput: ::winnt::HANDLE, bMaximumWindow: ::minwindef::BOOL, lpConsoleCurrentFont: ::wincon::PCONSOLE_FONT_INFO) -> ::minwindef::BOOL; /* wincon.h:575:1 */
    pub fn GetCurrentConsoleFontEx(hConsoleOutput: ::winnt::HANDLE, bMaximumWindow: ::minwindef::BOOL, lpConsoleCurrentFontEx: ::wincon::PCONSOLE_FONT_INFOEX) -> ::minwindef::BOOL; /* wincon.h:585:1 */
    pub fn GetSystemDefaultUILanguage() -> ::winnt::LANGID; /* winnls.h:1976:1 */
    pub fn GetUserDefaultUILanguage() -> ::winnt::LANGID; /* winnls.h:1981:1 */
    pub fn GetVolumeNameForVolumeMountPointA(lpszVolumeMountPoint: ::winnt::LPCSTR, lpszVolumeName: ::winnt::LPSTR, cchBufferLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:7630:1 */
    pub fn GetVolumePathNameA(lpszFileName: ::winnt::LPCSTR, lpszVolumePathName: ::winnt::LPSTR, cchBufferLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:7642:1 */
    pub fn IsValidLanguageGroup(LanguageGroup: ::winnls::LGRPID, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1866:1 */
    pub fn MapUserPhysicalPagesScatter(VirtualAddresses: *mut *mut ::libc::c_void, NumberOfPages: ::basetsd::ULONG_PTR, PageArray: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* winbase.h:7426:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::MoveFileWithProgressW as MoveFileWithProgress; /* winbase.h:5412:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn MoveFileWithProgressA(lpExistingFileName: ::winnt::LPCSTR, lpNewFileName: ::winnt::LPCSTR, lpProgressRoutine: ::winbase::LPPROGRESS_ROUTINE, lpData: ::minwindef::LPVOID, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5394:1 */
    pub fn MoveFileWithProgressW(lpExistingFileName: ::winnt::LPCWSTR, lpNewFileName: ::winnt::LPCWSTR, lpProgressRoutine: ::winbase::LPPROGRESS_ROUTINE, lpData: ::minwindef::LPVOID, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5404:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::OpenJobObjectW as OpenJobObject; /* winbase.h:7473:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn OpenJobObjectA(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, lpName: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:7458:1 */
    pub fn OpenJobObjectW(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, lpName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winbase.h:7467:1 */
    pub fn QueryInformationJobObject(hJob: ::winnt::HANDLE, JobObjectInformationClass: ::winnt::JOBOBJECTINFOCLASS, lpJobObjectInformation: ::minwindef::LPVOID, cbJobObjectInformationLength: ::minwindef::DWORD, lpReturnLength: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:7497:1 */
    pub fn RegisterWaitForSingleObject(phNewWaitObject: ::winnt::PHANDLE, hObject: ::winnt::HANDLE, Callback: ::winnt::WAITORTIMERCALLBACK, Context: ::winnt::PVOID, dwMilliseconds: ::minwindef::ULONG, dwFlags: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winbase.h:7064:1 */
    pub fn ReleaseActCtx(hActCtx: ::winnt::HANDLE); /* winbase.h:7751:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::ReplaceFileW as ReplaceFile; /* winbase.h:5495:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn ReplaceFileA(lpReplacedFileName: ::winnt::LPCSTR, lpReplacementFileName: ::winnt::LPCSTR, lpBackupFileName: ::winnt::LPCSTR, dwReplaceFlags: ::minwindef::DWORD, lpExclude: ::minwindef::LPVOID, lpReserved: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winbase.h:5475:1 */
    pub fn ReplaceFileW(lpReplacedFileName: ::winnt::LPCWSTR, lpReplacementFileName: ::winnt::LPCWSTR, lpBackupFileName: ::winnt::LPCWSTR, dwReplaceFlags: ::minwindef::DWORD, lpExclude: ::minwindef::LPVOID, lpReserved: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winbase.h:5486:1 */
    pub fn RtlCompareMemory(Source1: *const ::libc::c_void, Source2: *const ::libc::c_void, Length: ::basetsd::SIZE_T) -> ::basetsd::SIZE_T; /* winnt.h:17442:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::SetCalendarInfoW as SetCalendarInfo; /* winnls.h:1596:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn SetCalendarInfoA(Locale: ::winnt::LCID, Calendar: ::winnls::CALID, CalType: ::winnls::CALTYPE, lpCalData: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winnls.h:1582:1 */
    pub fn SetCalendarInfoW(Locale: ::winnt::LCID, Calendar: ::winnls::CALID, CalType: ::winnls::CALTYPE, lpCalData: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winnls.h:1590:1 */
    pub fn SetComputerNameExA(NameType: ::sysinfoapi::COMPUTER_NAME_FORMAT, lpBuffer: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:6831:1 */
    pub fn SetConsoleDisplayMode(hConsoleOutput: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD, lpNewScreenBufferDimensions: ::wincon::PCOORD) -> ::minwindef::BOOL; /* wincon.h:864:1 */
    pub fn SetConsoleHistoryInfo(lpConsoleHistoryInfo: ::wincon::PCONSOLE_HISTORY_INFO) -> ::minwindef::BOOL; /* wincon.h:608:1 */
    pub fn SetCurrentConsoleFontEx(hConsoleOutput: ::winnt::HANDLE, bMaximumWindow: ::minwindef::BOOL, lpConsoleCurrentFontEx: ::wincon::PCONSOLE_FONT_INFOEX) -> ::minwindef::BOOL; /* wincon.h:593:1 */
    pub fn SetInformationJobObject(hJob: ::winnt::HANDLE, JobObjectInformationClass: ::winnt::JOBOBJECTINFOCLASS, lpJobObjectInformation: ::minwindef::LPVOID, cbJobObjectInformationLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:7508:1 */
    pub fn SetTimerQueueTimer(TimerQueue: ::winnt::HANDLE, Callback: ::winnt::WAITORTIMERCALLBACK, Parameter: ::winnt::PVOID, DueTime: ::minwindef::DWORD, Period: ::minwindef::DWORD, PreferIo: ::minwindef::BOOL) -> ::winnt::HANDLE; /* winbase.h:7093:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::SetVolumeMountPointW as SetVolumeMountPoint; /* winbase.h:7612:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn SetVolumeMountPointA(lpszVolumeMountPoint: ::winnt::LPCSTR, lpszVolumeName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:7600:1 */
    pub fn SetVolumeMountPointW(lpszVolumeMountPoint: ::winnt::LPCWSTR, lpszVolumeName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:7607:1 */
    pub fn TerminateJobObject(hJob: ::winnt::HANDLE, uExitCode: ::minwindef::UINT) -> ::minwindef::BOOL; /* winbase.h:7489:1 */
    pub fn UnregisterWait(WaitHandle: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:7077:1 */
    pub fn VerSetConditionMask(ConditionMask: ::winnt::ULONGLONG, TypeMask: ::minwindef::DWORD, Condition: ::minwindef::BYTE) -> ::winnt::ULONGLONG; /* winnt.h:17986:1 */
    pub fn ZombifyActCtx(hActCtx: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:7758:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000100"))] 
extern "system" {
    pub fn RtlCaptureContext(ContextRecord: ::winnt::PCONTEXT); /* winnt.h:16934:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::AddConsoleAliasW as AddConsoleAlias; /* wincon.h:906:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn AddConsoleAliasA(Source: ::winnt::LPSTR, Target: ::winnt::LPSTR, ExeName: ::winnt::LPSTR) -> ::minwindef::BOOL; /* wincon.h:894:1 */
    pub fn AddConsoleAliasW(Source: ::winnt::LPWSTR, Target: ::winnt::LPWSTR, ExeName: ::winnt::LPWSTR) -> ::minwindef::BOOL; /* wincon.h:901:1 */
    pub fn AddVectoredContinueHandler(First: ::minwindef::ULONG, Handler: ::winnt::PVECTORED_EXCEPTION_HANDLER) -> ::winnt::PVOID; /* errhandlingapi.h:197:1 */
    pub fn AddVectoredExceptionHandler(First: ::minwindef::ULONG, Handler: ::winnt::PVECTORED_EXCEPTION_HANDLER) -> ::winnt::PVOID; /* errhandlingapi.h:179:1 */
    pub fn AllocateUserPhysicalPages(hProcess: ::winnt::HANDLE, NumberOfPages: ::basetsd::PULONG_PTR, PageArray: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* memoryapi.h:582:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::CheckNameLegalDOS8Dot3W as CheckNameLegalDOS8Dot3; /* winbase.h:4979:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn CheckNameLegalDOS8Dot3A(lpName: ::winnt::LPCSTR, lpOemName: ::winnt::LPSTR, OemNameSize: ::minwindef::DWORD, pbNameContainsSpaces: ::minwindef::PBOOL, pbNameLegal: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* winbase.h:4961:1 */
    pub fn CheckNameLegalDOS8Dot3W(lpName: ::winnt::LPCWSTR, lpOemName: ::winnt::LPSTR, OemNameSize: ::minwindef::DWORD, pbNameContainsSpaces: ::minwindef::PBOOL, pbNameLegal: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* winbase.h:4971:1 */
    pub fn CheckRemoteDebuggerPresent(hProcess: ::winnt::HANDLE, pbDebuggerPresent: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* debugapi.h:155:1 */
    pub fn CreateMemoryResourceNotification(NotificationType: ::memoryapi::MEMORY_RESOURCE_NOTIFICATION_TYPE) -> ::winnt::HANDLE; /* memoryapi.h:420:1 */
    pub fn FindFirstStreamW(lpFileName: ::winnt::LPCWSTR, InfoLevel: ::winbase::STREAM_INFO_LEVELS, lpFindStreamData: ::minwindef::LPVOID, dwFlags: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:5586:1 */
    pub fn FindNextStreamW(hFindStream: ::winnt::HANDLE, lpFindStreamData: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winbase.h:5609:1 */
    pub fn FreeUserPhysicalPages(hProcess: ::winnt::HANDLE, NumberOfPages: ::basetsd::PULONG_PTR, PageArray: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* memoryapi.h:593:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetCompressedFileSizeW as GetCompressedFileSize; /* fileapi.h:1340:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetCompressedFileSizeA(lpFileName: ::winnt::LPCSTR, lpFileSizeHigh: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* fileapi.h:1326:1 */
    pub fn GetCompressedFileSizeW(lpFileName: ::winnt::LPCWSTR, lpFileSizeHigh: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* fileapi.h:1334:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetConsoleAliasW as GetConsoleAlias; /* wincon.h:928:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetConsoleAliasA(Source: ::winnt::LPSTR, TargetBuffer: ::winnt::LPSTR, TargetBufferLength: ::minwindef::DWORD, ExeName: ::winnt::LPSTR) -> ::minwindef::DWORD; /* wincon.h:914:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetConsoleAliasExesW as GetConsoleAliasExes; /* wincon.h:998:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetConsoleAliasExesA(ExeNameBuffer: ::winnt::LPSTR, ExeNameBufferLength: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:988:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetConsoleAliasExesLengthW as GetConsoleAliasExesLength; /* wincon.h:960:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetConsoleAliasExesLengthA() -> ::minwindef::DWORD; /* wincon.h:952:1 */
    pub fn GetConsoleAliasExesLengthW() -> ::minwindef::DWORD; /* wincon.h:957:1 */
    pub fn GetConsoleAliasExesW(ExeNameBuffer: ::winnt::LPWSTR, ExeNameBufferLength: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:994:1 */
    pub fn GetConsoleAliasW(Source: ::winnt::LPWSTR, TargetBuffer: ::winnt::LPWSTR, TargetBufferLength: ::minwindef::DWORD, ExeName: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* wincon.h:922:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetConsoleAliasesW as GetConsoleAliases; /* wincon.h:980:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetConsoleAliasesA(AliasBuffer: ::winnt::LPSTR, AliasBufferLength: ::minwindef::DWORD, ExeName: ::winnt::LPSTR) -> ::minwindef::DWORD; /* wincon.h:968:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetConsoleAliasesLengthW as GetConsoleAliasesLength; /* wincon.h:944:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetConsoleAliasesLengthA(ExeName: ::winnt::LPSTR) -> ::minwindef::DWORD; /* wincon.h:936:1 */
    pub fn GetConsoleAliasesLengthW(ExeName: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* wincon.h:941:1 */
    pub fn GetConsoleAliasesW(AliasBuffer: ::winnt::LPWSTR, AliasBufferLength: ::minwindef::DWORD, ExeName: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* wincon.h:975:1 */
    pub fn GetConsoleProcessList(lpdwProcessList: ::minwindef::LPDWORD, dwProcessCount: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:883:1 */
    pub fn GetProcessHandleCount(hProcess: ::winnt::HANDLE, pdwHandleCount: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:974:1 */
    pub fn GetProcessId(Process: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:690:1 */
    pub fn GetProcessPriorityBoost(hProcess: ::winnt::HANDLE, pDisablePriorityBoost: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:1067:1 */
    pub fn GetSystemRegistryQuota(pdwQuotaAllowed: ::minwindef::PDWORD, pdwQuotaUsed: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:2288:1 */
    pub fn GetSystemTimes(lpIdleTime: ::minwindef::PFILETIME, lpKernelTime: ::minwindef::PFILETIME, lpUserTime: ::minwindef::PFILETIME) -> ::minwindef::BOOL; /* processthreadsapi.h:1094:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetSystemWow64DirectoryW as GetSystemWow64Directory; /* winbase.h:4479:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetSystemWow64DirectoryA(lpBuffer: ::winnt::LPSTR, uSize: ::minwindef::UINT) -> ::minwindef::UINT; /* winbase.h:4466:1 */
    pub fn GetSystemWow64DirectoryW(lpBuffer: ::winnt::LPWSTR, uSize: ::minwindef::UINT) -> ::minwindef::UINT; /* winbase.h:4474:1 */
    pub fn GetThreadIOPendingFlag(hThread: ::winnt::HANDLE, lpIOIsPending: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:1085:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetVolumePathNamesForVolumeNameW as GetVolumePathNamesForVolumeName; /* fileapi.h:1245:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetVolumePathNamesForVolumeNameA(lpszVolumeName: ::winnt::LPCSTR, lpszVolumePathNames: ::winnt::LPCH, cchBufferLength: ::minwindef::DWORD, lpcchReturnLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:7658:1 */
    pub fn GetVolumePathNamesForVolumeNameW(lpszVolumeName: ::winnt::LPCWSTR, lpszVolumePathNames: ::winnt::LPWCH, cchBufferLength: ::minwindef::DWORD, lpcchReturnLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* fileapi.h:1236:1 */
    pub fn IsProcessInJob(ProcessHandle: ::winnt::HANDLE, JobHandle: ::winnt::HANDLE, Result: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* jobapi.h:46:1 */
    pub fn IsWow64Process(hProcess: ::winnt::HANDLE, Wow64Process: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* wow64apiset.h:72:1 */
    pub fn MapUserPhysicalPages(VirtualAddress: ::winnt::PVOID, NumberOfPages: ::basetsd::ULONG_PTR, PageArray: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* memoryapi.h:604:1 */
    pub fn QueryMemoryResourceNotification(ResourceNotificationHandle: ::winnt::HANDLE, ResourceState: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* memoryapi.h:429:1 */
    pub fn RemoveVectoredContinueHandler(Handle: ::winnt::PVOID) -> ::minwindef::ULONG; /* errhandlingapi.h:206:1 */
    pub fn RemoveVectoredExceptionHandler(Handle: ::winnt::PVOID) -> ::minwindef::ULONG; /* errhandlingapi.h:188:1 */
    pub fn SetFileValidData(hFile: ::winnt::HANDLE, ValidDataLength: ::winnt::LONGLONG) -> ::minwindef::BOOL; /* fileapi.h:1107:1 */
    pub fn SetProcessPriorityBoost(hProcess: ::winnt::HANDLE, bDisablePriorityBoost: ::minwindef::BOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:1076:1 */
    pub fn WTSGetActiveConsoleSessionId() -> ::minwindef::DWORD; /* winbase.h:7959:1 */
    pub fn Wow64DisableWow64FsRedirection(OldValue: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* wow64apiset.h:49:1 */
    pub fn Wow64EnableWow64FsRedirection(Wow64FsEnableRedirection: ::winnt::BOOLEAN) -> ::winnt::BOOLEAN; /* winbase.h:4487:1 */
    pub fn Wow64RevertWow64FsRedirection(OlValue: ::winnt::PVOID) -> ::minwindef::BOOL; /* wow64apiset.h:57:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn ConvertFiberToThread() -> ::minwindef::BOOL; /* winbase.h:1316:1 */
    pub fn GetNativeSystemInfo(lpSystemInfo: ::sysinfoapi::LPSYSTEM_INFO); /* sysinfoapi.h:495:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010100"))] 
extern "system" {
    pub fn RtlCaptureStackBackTrace(FramesToSkip: ::minwindef::DWORD, FramesToCapture: ::minwindef::DWORD, BackTrace: *mut *mut ::libc::c_void, BackTraceHash: ::minwindef::PDWORD) -> ::minwindef::WORD; /* winnt.h:16911:1 */
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
    pub fn GetDllDirectoryA(nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPSTR) -> ::minwindef::DWORD; /* winbase.h:4584:1 */
    pub fn GetDllDirectoryW(nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* winbase.h:4592:1 */
    pub fn GetSystemFileCacheSize(lpMinimumFileCacheSize: ::basetsd::PSIZE_T, lpMaximumFileCacheSize: ::basetsd::PSIZE_T, lpFlags: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:450:1 */
    pub fn GetThreadId(Thread: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:703:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] #[doc(inline)] pub use self::NeedCurrentDirectoryForExePathW as NeedCurrentDirectoryForExePath; /* processenv.h:354:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] 
extern "system" {
    pub fn NeedCurrentDirectoryForExePathA(ExeName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* processenv.h:342:1 */
    pub fn NeedCurrentDirectoryForExePathW(ExeName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* processenv.h:349:1 */
    pub fn ReOpenFile(hOriginalFile: ::winnt::HANDLE, dwDesiredAccess: ::minwindef::DWORD, dwShareMode: ::minwindef::DWORD, dwFlagsAndAttributes: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:4824:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] #[doc(inline)] pub use self::SetDllDirectoryW as SetDllDirectory; /* winbase.h:4575:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] 
extern "system" {
    pub fn SetDllDirectoryA(lpPathName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:4565:1 */
    pub fn SetDllDirectoryW(lpPathName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:4571:1 */
    pub fn SetSystemFileCacheSize(MinimumFileCacheSize: ::basetsd::SIZE_T, MaximumFileCacheSize: ::basetsd::SIZE_T, Flags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:460:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn QueryActCtxSettingsW(dwFlags: ::minwindef::DWORD, hActCtx: ::winnt::HANDLE, settingsNameSpace: ::winnt::PCWSTR, settingName: ::winnt::PCWSTR, pvBuffer: ::winnt::PWSTR, dwBuffer: ::basetsd::SIZE_T, pdwWrittenOrRequired: *mut ::libc::c_ulonglong) -> ::minwindef::BOOL; /* winbase.h:8571:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[cfg(any(target_arch="x86", target_arch="arm"))] 
extern "system" {
    pub fn QueryActCtxSettingsW(dwFlags: ::minwindef::DWORD, hActCtx: ::winnt::HANDLE, settingsNameSpace: ::winnt::PCWSTR, settingName: ::winnt::PCWSTR, pvBuffer: ::winnt::PWSTR, dwBuffer: ::basetsd::SIZE_T, pdwWrittenOrRequired: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winbase.h:8571:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn AddIntegrityLabelToBoundaryDescriptor(BoundaryDescriptor: *mut *mut ::libc::c_void, IntegrityLabel: ::winnt::PSID) -> ::minwindef::BOOL; /* winbase.h:7265:1 */
    pub fn AddSecureMemoryCacheCallback(pfnCallBack: ::winnt::PSECURE_MEMORY_CACHE_CALLBACK) -> ::minwindef::BOOL; /* winbase.h:8602:1 */
    pub fn AllocateUserPhysicalPagesNuma(hProcess: ::winnt::HANDLE, NumberOfPages: ::basetsd::PULONG_PTR, PageArray: ::basetsd::PULONG_PTR, nndPreferred: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:620:1 */
    pub fn ApplicationRecoveryFinished(bSuccess: ::minwindef::BOOL); /* winbase.h:8179:1 */
    pub fn ApplicationRecoveryInProgress(pbCancelled: ::minwindef::PBOOL) -> ::winnt::HRESULT; /* winbase.h:8172:1 */
    pub fn CallbackMayRunLong(pci: ::winnt::PTP_CALLBACK_INSTANCE) -> ::minwindef::BOOL; /* threadpoolapiset.h:190:1 */
    pub fn CancelIoEx(hFile: ::winnt::HANDLE, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* ioapiset.h:134:1 */
    pub fn CancelSynchronousIo(hThread: ::winnt::HANDLE) -> ::minwindef::BOOL; /* ioapiset.h:189:1 */
    pub fn CancelThreadpoolIo(pio: ::winnt::PTP_IO); /* threadpoolapiset.h:358:1 */
    pub fn CloseThreadpool(ptpp: ::winnt::PTP_POOL); /* threadpoolapiset.h:109:1 */
    pub fn CloseThreadpoolCleanupGroup(ptpcg: ::winnt::PTP_CLEANUP_GROUP); /* threadpoolapiset.h:136:1 */
    pub fn CloseThreadpoolCleanupGroupMembers(ptpcg: ::winnt::PTP_CLEANUP_GROUP, fCancelPendingCallbacks: ::minwindef::BOOL, pvCleanupContext: ::winnt::PVOID); /* threadpoolapiset.h:126:1 */
    pub fn CloseThreadpoolIo(pio: ::winnt::PTP_IO); /* threadpoolapiset.h:375:1 */
    pub fn CloseThreadpoolTimer(pti: ::winnt::PTP_TIMER); /* threadpoolapiset.h:292:1 */
    pub fn CloseThreadpoolWait(pwa: ::winnt::PTP_WAIT); /* threadpoolapiset.h:330:1 */
    pub fn CloseThreadpoolWork(pwk: ::winnt::PTP_WORK); /* threadpoolapiset.h:245:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CopyFileTransactedW as CopyFileTransacted; /* winbase.h:5160:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CopyFileTransactedA(lpExistingFileName: ::winnt::LPCSTR, lpNewFileName: ::winnt::LPCSTR, lpProgressRoutine: ::winbase::LPPROGRESS_ROUTINE, lpData: ::minwindef::LPVOID, pbCancel: ::minwindef::LPBOOL, dwCopyFlags: ::minwindef::DWORD, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5138:1 */
    pub fn CopyFileTransactedW(lpExistingFileName: ::winnt::LPCWSTR, lpNewFileName: ::winnt::LPCWSTR, lpProgressRoutine: ::winbase::LPPROGRESS_ROUTINE, lpData: ::minwindef::LPVOID, pbCancel: ::minwindef::LPBOOL, dwCopyFlags: ::minwindef::DWORD, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5150:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateBoundaryDescriptorW as CreateBoundaryDescriptor; /* winbase.h:7258:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateBoundaryDescriptorA(Name: ::winnt::LPCSTR, Flags: ::minwindef::ULONG) -> ::winnt::HANDLE; /* winbase.h:7250:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateDirectoryTransactedW as CreateDirectoryTransacted; /* winbase.h:4691:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateDirectoryTransactedA(lpTemplateDirectory: ::winnt::LPCSTR, lpNewDirectory: ::winnt::LPCSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4675:1 */
    pub fn CreateDirectoryTransactedW(lpTemplateDirectory: ::winnt::LPCWSTR, lpNewDirectory: ::winnt::LPCWSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4684:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateFileMappingNumaW as CreateFileMappingNuma; /* memoryapi.h:488:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateFileMappingNumaA(hFile: ::winnt::HANDLE, lpFileMappingAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, flProtect: ::minwindef::DWORD, dwMaximumSizeHigh: ::minwindef::DWORD, dwMaximumSizeLow: ::minwindef::DWORD, lpName: ::winnt::LPCSTR, nndPreferred: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:3134:1 */
    pub fn CreateFileMappingNumaW(hFile: ::winnt::HANDLE, lpFileMappingAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, flProtect: ::minwindef::DWORD, dwMaximumSizeHigh: ::minwindef::DWORD, dwMaximumSizeLow: ::minwindef::DWORD, lpName: ::winnt::LPCWSTR, nndPreferred: ::minwindef::DWORD) -> ::winnt::HANDLE; /* memoryapi.h:476:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateFileTransactedW as CreateFileTransacted; /* winbase.h:4811:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateFileTransactedA(lpFileName: ::winnt::LPCSTR, dwDesiredAccess: ::minwindef::DWORD, dwShareMode: ::minwindef::DWORD, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, dwCreationDisposition: ::minwindef::DWORD, dwFlagsAndAttributes: ::minwindef::DWORD, hTemplateFile: ::winnt::HANDLE, hTransaction: ::winnt::HANDLE, pusMiniVersion: ::minwindef::PUSHORT, lpExtendedParameter: ::winnt::PVOID) -> ::winnt::HANDLE; /* winbase.h:4783:1 */
    pub fn CreateFileTransactedW(lpFileName: ::winnt::LPCWSTR, dwDesiredAccess: ::minwindef::DWORD, dwShareMode: ::minwindef::DWORD, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, dwCreationDisposition: ::minwindef::DWORD, dwFlagsAndAttributes: ::minwindef::DWORD, hTemplateFile: ::winnt::HANDLE, hTransaction: ::winnt::HANDLE, pusMiniVersion: ::minwindef::PUSHORT, lpExtendedParameter: ::winnt::PVOID) -> ::winnt::HANDLE; /* winbase.h:4798:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateHardLinkTransactedW as CreateHardLinkTransacted; /* winbase.h:5555:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateHardLinkTransactedA(lpFileName: ::winnt::LPCSTR, lpExistingFileName: ::winnt::LPCSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5539:1 */
    pub fn CreateHardLinkTransactedW(lpFileName: ::winnt::LPCWSTR, lpExistingFileName: ::winnt::LPCWSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5548:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreatePrivateNamespaceW as CreatePrivateNamespace; /* winbase.h:7223:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreatePrivateNamespaceA(lpPrivateNamespaceAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpBoundaryDescriptor: ::minwindef::LPVOID, lpAliasPrefix: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:7214:1 */
    pub fn CreateSemaphoreExA(lpSemaphoreAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lInitialCount: ::winnt::LONG, lMaximumCount: ::winnt::LONG, lpName: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD, dwDesiredAccess: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:3082:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateSymbolicLinkW as CreateSymbolicLink; /* winbase.h:8534:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateSymbolicLinkA(lpSymlinkFileName: ::winnt::LPCSTR, lpTargetFileName: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD) -> ::winnt::BOOLEAN; /* winbase.h:8520:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateSymbolicLinkTransactedW as CreateSymbolicLinkTransacted; /* winbase.h:8558:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateSymbolicLinkTransactedA(lpSymlinkFileName: ::winnt::LPCSTR, lpTargetFileName: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD, hTransaction: ::winnt::HANDLE) -> ::winnt::BOOLEAN; /* winbase.h:8542:1 */
    pub fn CreateSymbolicLinkTransactedW(lpSymlinkFileName: ::winnt::LPCWSTR, lpTargetFileName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, hTransaction: ::winnt::HANDLE) -> ::winnt::BOOLEAN; /* winbase.h:8551:1 */
    pub fn CreateSymbolicLinkW(lpSymlinkFileName: ::winnt::LPCWSTR, lpTargetFileName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD) -> ::winnt::BOOLEAN; /* winbase.h:8528:1 */
    pub fn CreateThreadpool(reserved: ::winnt::PVOID) -> ::winnt::PTP_POOL; /* threadpoolapiset.h:65:1 */
    pub fn CreateThreadpoolCleanupGroup() -> ::winnt::PTP_CLEANUP_GROUP; /* threadpoolapiset.h:118:1 */
    pub fn CreateThreadpoolIo(fl: ::winnt::HANDLE, pfnio: ::threadpoolapiset::PTP_WIN32_IO_CALLBACK, pv: ::winnt::PVOID, pcbe: ::winnt::PTP_CALLBACK_ENVIRON) -> ::winnt::PTP_IO; /* threadpoolapiset.h:339:1 */
    pub fn CreateThreadpoolTimer(pfnti: ::winnt::PTP_TIMER_CALLBACK, pv: ::winnt::PVOID, pcbe: ::winnt::PTP_CALLBACK_ENVIRON) -> ::winnt::PTP_TIMER; /* threadpoolapiset.h:254:1 */
    pub fn CreateThreadpoolWait(pfnwa: ::winnt::PTP_WAIT_CALLBACK, pv: ::winnt::PVOID, pcbe: ::winnt::PTP_CALLBACK_ENVIRON) -> ::winnt::PTP_WAIT; /* threadpoolapiset.h:301:1 */
    pub fn CreateThreadpoolWork(pfnwk: ::winnt::PTP_WORK_CALLBACK, pv: ::winnt::PVOID, pcbe: ::winnt::PTP_CALLBACK_ENVIRON) -> ::winnt::PTP_WORK; /* threadpoolapiset.h:218:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateWaitableTimerExW as CreateWaitableTimerEx; /* synchapi.h:845:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateWaitableTimerExA(lpTimerAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpTimerName: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD, dwDesiredAccess: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:3098:1 */
    pub fn CreateWaitableTimerExW(lpTimerAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpTimerName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, dwDesiredAccess: ::minwindef::DWORD) -> ::winnt::HANDLE; /* synchapi.h:836:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::DeleteFileTransactedW as DeleteFileTransacted; /* winbase.h:4919:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn DeleteFileTransactedA(lpFileName: ::winnt::LPCSTR, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4907:1 */
    pub fn DeleteFileTransactedW(lpFileName: ::winnt::LPCWSTR, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4914:1 */
    pub fn DeleteProcThreadAttributeList(lpAttributeList: ::processthreadsapi::LPPROC_THREAD_ATTRIBUTE_LIST); /* processthreadsapi.h:761:1 */
    pub fn DisassociateCurrentThreadFromCallback(pci: ::winnt::PTP_CALLBACK_INSTANCE); /* threadpoolapiset.h:198:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::EnumResourceLanguagesExW as EnumResourceLanguagesEx; /* libloaderapi.h:561:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn EnumResourceLanguagesExA(hModule: ::minwindef::HMODULE, lpType: ::winnt::LPCSTR, lpName: ::winnt::LPCSTR, lpEnumFunc: ::libloaderapi::ENUMRESLANGPROCA, lParam: ::basetsd::LONG_PTR, dwFlags: ::minwindef::DWORD, LangId: ::winnt::LANGID) -> ::minwindef::BOOL; /* libloaderapi.h:537:1 */
    pub fn EnumResourceLanguagesExW(hModule: ::minwindef::HMODULE, lpType: ::winnt::LPCWSTR, lpName: ::winnt::LPCWSTR, lpEnumFunc: ::libloaderapi::ENUMRESLANGPROCW, lParam: ::basetsd::LONG_PTR, dwFlags: ::minwindef::DWORD, LangId: ::winnt::LANGID) -> ::minwindef::BOOL; /* libloaderapi.h:550:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::EnumResourceNamesExW as EnumResourceNamesEx; /* libloaderapi.h:591:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn EnumResourceNamesExA(hModule: ::minwindef::HMODULE, lpType: ::winnt::LPCSTR, lpEnumFunc: ::libloaderapi::ENUMRESNAMEPROCA, lParam: ::basetsd::LONG_PTR, dwFlags: ::minwindef::DWORD, LangId: ::winnt::LANGID) -> ::minwindef::BOOL; /* libloaderapi.h:569:1 */
    pub fn EnumResourceNamesExW(hModule: ::minwindef::HMODULE, lpType: ::winnt::LPCWSTR, lpEnumFunc: ::libloaderapi::ENUMRESNAMEPROCW, lParam: ::basetsd::LONG_PTR, dwFlags: ::minwindef::DWORD, LangId: ::winnt::LANGID) -> ::minwindef::BOOL; /* libloaderapi.h:581:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::EnumResourceTypesExW as EnumResourceTypesEx; /* libloaderapi.h:619:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn EnumResourceTypesExA(hModule: ::minwindef::HMODULE, lpEnumFunc: ::libloaderapi::ENUMRESTYPEPROCA, lParam: ::basetsd::LONG_PTR, dwFlags: ::minwindef::DWORD, LangId: ::winnt::LANGID) -> ::minwindef::BOOL; /* libloaderapi.h:599:1 */
    pub fn EnumResourceTypesExW(hModule: ::minwindef::HMODULE, lpEnumFunc: ::libloaderapi::ENUMRESTYPEPROCW, lParam: ::basetsd::LONG_PTR, dwFlags: ::minwindef::DWORD, LangId: ::winnt::LANGID) -> ::minwindef::BOOL; /* libloaderapi.h:610:1 */
    pub fn FindFirstFileNameTransactedW(lpFileName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, StringLength: ::minwindef::LPDWORD, LinkName: ::winnt::PWSTR, hTransaction: ::winnt::HANDLE) -> ::winnt::HANDLE; /* winbase.h:5639:1 */
    pub fn FindFirstFileNameW(lpFileName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, StringLength: ::minwindef::LPDWORD, LinkName: ::winnt::PWSTR) -> ::winnt::HANDLE; /* winbase.h:5620:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::FindFirstFileTransactedW as FindFirstFileTransacted; /* winbase.h:5021:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn FindFirstFileTransactedA(lpFileName: ::winnt::LPCSTR, fInfoLevelId: ::minwinbase::FINDEX_INFO_LEVELS, lpFindFileData: ::minwindef::LPVOID, fSearchOp: ::minwinbase::FINDEX_SEARCH_OPS, lpSearchFilter: ::minwindef::LPVOID, dwAdditionalFlags: ::minwindef::DWORD, hTransaction: ::winnt::HANDLE) -> ::winnt::HANDLE; /* winbase.h:4999:1 */
    pub fn FindFirstFileTransactedW(lpFileName: ::winnt::LPCWSTR, fInfoLevelId: ::minwinbase::FINDEX_INFO_LEVELS, lpFindFileData: ::minwindef::LPVOID, fSearchOp: ::minwinbase::FINDEX_SEARCH_OPS, lpSearchFilter: ::minwindef::LPVOID, dwAdditionalFlags: ::minwindef::DWORD, hTransaction: ::winnt::HANDLE) -> ::winnt::HANDLE; /* winbase.h:5011:1 */
    pub fn FindFirstStreamTransactedW(lpFileName: ::winnt::LPCWSTR, InfoLevel: ::winbase::STREAM_INFO_LEVELS, lpFindStreamData: ::minwindef::LPVOID, dwFlags: ::minwindef::DWORD, hTransaction: ::winnt::HANDLE) -> ::winnt::HANDLE; /* winbase.h:5597:1 */
    pub fn FindNLSString(Locale: ::winnt::LCID, dwFindNLSStringFlags: ::minwindef::DWORD, lpStringSource: ::winnt::LPCWSTR, cchSource: ::libc::c_int, lpStringValue: ::winnt::LPCWSTR, cchValue: ::libc::c_int, pcchFound: ::minwindef::LPINT) -> ::libc::c_int; /* winnls.h:1460:1 */
    pub fn FindNextFileNameW(hFindStream: ::winnt::HANDLE, StringLength: ::minwindef::LPDWORD, LinkName: ::winnt::PWSTR) -> ::minwindef::BOOL; /* winbase.h:5630:1 */
    pub fn FreeLibraryWhenCallbackReturns(pci: ::winnt::PTP_CALLBACK_INSTANCE, mod_: ::minwindef::HMODULE); /* threadpoolapiset.h:181:1 */
    pub fn GetApplicationRecoveryCallback(hProcess: ::winnt::HANDLE, pRecoveryCallback: *mut Option<extern "system" fn(*mut ::libc::c_void) -> ::libc::c_ulong>, ppvParameter: *mut *mut ::libc::c_void, pdwPingInterval: ::minwindef::PDWORD, pdwFlags: ::minwindef::PDWORD) -> ::winnt::HRESULT; /* winbase.h:8151:1 */
    pub fn GetApplicationRestartSettings(hProcess: ::winnt::HANDLE, pwzCommandline: ::winnt::PWSTR, pcchSize: ::minwindef::PDWORD, pdwFlags: ::minwindef::PDWORD) -> ::winnt::HRESULT; /* winbase.h:8162:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetCompressedFileSizeTransactedW as GetCompressedFileSizeTransacted; /* winbase.h:4899:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetCompressedFileSizeTransactedA(lpFileName: ::winnt::LPCSTR, lpFileSizeHigh: ::minwindef::LPDWORD, hTransaction: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:4885:1 */
    pub fn GetCompressedFileSizeTransactedW(lpFileName: ::winnt::LPCWSTR, lpFileSizeHigh: ::minwindef::LPDWORD, hTransaction: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:4893:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetConsoleOriginalTitleW as GetConsoleOriginalTitle; /* wincon.h:774:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetConsoleOriginalTitleA(lpConsoleTitle: ::winnt::LPSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:764:1 */
    pub fn GetConsoleOriginalTitleW(lpConsoleTitle: ::winnt::LPWSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:770:1 */
    pub fn GetDurationFormat(Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD, lpDuration: *const ::minwinbase::SYSTEMTIME, ullDuration: ::winnt::ULONGLONG, lpFormat: ::winnt::LPCWSTR, lpDurationStr: ::winnt::LPWSTR, cchDuration: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1678:1 */
    pub fn GetErrorMode() -> ::minwindef::UINT; /* errhandlingapi.h:146:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetFileAttributesTransactedW as GetFileAttributesTransacted; /* winbase.h:4877:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetFileAttributesTransactedA(lpFileName: ::winnt::LPCSTR, fInfoLevelId: ::minwinbase::GET_FILEEX_INFO_LEVELS, lpFileInformation: ::minwindef::LPVOID, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4861:1 */
    pub fn GetFileAttributesTransactedW(lpFileName: ::winnt::LPCWSTR, fInfoLevelId: ::minwinbase::GET_FILEEX_INFO_LEVELS, lpFileInformation: ::minwindef::LPVOID, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4870:1 */
    pub fn GetFileBandwidthReservation(hFile: ::winnt::HANDLE, lpPeriodMilliseconds: ::minwindef::LPDWORD, lpBytesPerPeriod: ::minwindef::LPDWORD, pDiscardable: ::minwindef::LPBOOL, lpTransferSize: ::minwindef::LPDWORD, lpNumOutstandingRequests: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:5857:1 */
    pub fn GetFileMUIInfo(dwFlags: ::minwindef::DWORD, pcwszFilePath: ::winnt::PCWSTR, pFileMUIInfo: ::winnls::PFILEMUIINFO, pcbFileMUIInfo: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winnls.h:2084:1 */
    pub fn GetFileMUIPath(dwFlags: ::minwindef::DWORD, pcwszFilePath: ::winnt::PCWSTR, pwszLanguage: ::winnt::PWSTR, pcchLanguage: ::minwindef::PULONG, pwszFileMUIPath: ::winnt::PWSTR, pcchFileMUIPath: ::minwindef::PULONG, pululEnumerator: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* winnls.h:2093:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetFinalPathNameByHandleW as GetFinalPathNameByHandle; /* fileapi.h:694:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetFinalPathNameByHandleA(hFile: ::winnt::HANDLE, lpszFilePath: ::winnt::LPSTR, cchFilePath: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:676:1 */
    pub fn GetFinalPathNameByHandleW(hFile: ::winnt::HANDLE, lpszFilePath: ::winnt::LPWSTR, cchFilePath: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:686:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetFullPathNameTransactedW as GetFullPathNameTransacted; /* winbase.h:4739:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetFullPathNameTransactedA(lpFileName: ::winnt::LPCSTR, nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPSTR, lpFilePart: *mut *mut ::libc::c_schar, hTransaction: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:4720:1 */
    pub fn GetFullPathNameTransactedW(lpFileName: ::winnt::LPCWSTR, nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPWSTR, lpFilePart: *mut *mut ::libc::c_ushort, hTransaction: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:4731:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetLongPathNameTransactedW as GetLongPathNameTransacted; /* winbase.h:1200:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetLongPathNameTransactedA(lpszShortPath: ::winnt::LPCSTR, lpszLongPath: ::winnt::LPSTR, cchBuffer: ::minwindef::DWORD, hTransaction: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:1183:1 */
    pub fn GetLongPathNameTransactedW(lpszShortPath: ::winnt::LPCWSTR, lpszLongPath: ::winnt::LPWSTR, cchBuffer: ::minwindef::DWORD, hTransaction: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:1193:1 */
    pub fn GetNamedPipeClientComputerNameA(Pipe: ::winnt::HANDLE, ClientComputerName: ::winnt::LPSTR, ClientComputerNameLength: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winbase.h:5743:1 */
    pub fn GetNamedPipeClientComputerNameW(Pipe: ::winnt::HANDLE, ClientComputerName: ::winnt::LPWSTR, ClientComputerNameLength: ::minwindef::ULONG) -> ::minwindef::BOOL; /* namedpipeapi.h:151:1 */
    pub fn GetNamedPipeClientProcessId(Pipe: ::winnt::HANDLE, ClientProcessId: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winbase.h:5756:1 */
    pub fn GetNamedPipeClientSessionId(Pipe: ::winnt::HANDLE, ClientSessionId: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winbase.h:5764:1 */
    pub fn GetNamedPipeServerProcessId(Pipe: ::winnt::HANDLE, ServerProcessId: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winbase.h:5772:1 */
    pub fn GetNamedPipeServerSessionId(Pipe: ::winnt::HANDLE, ServerSessionId: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winbase.h:5780:1 */
    pub fn GetNumaProximityNode(ProximityId: ::minwindef::ULONG, NodeNumber: ::minwindef::PUCHAR) -> ::minwindef::BOOL; /* winbase.h:8066:1 */
    pub fn GetProcessDEPPolicy(hProcess: ::winnt::HANDLE, lpFlags: ::minwindef::LPDWORD, lpPermanent: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* winbase.h:1644:1 */
    pub fn GetProcessIdOfThread(Thread: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:741:1 */
    pub fn GetProcessPreferredUILanguages(dwFlags: ::minwindef::DWORD, pulNumLanguages: ::minwindef::PULONG, pwszLanguagesBuffer: ::winnt::PZZWSTR, pcchLanguagesBuffer: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:2021:1 */
    pub fn GetProductInfo(dwOSMajorVersion: ::minwindef::DWORD, dwOSMinorVersion: ::minwindef::DWORD, dwSpMajorVersion: ::minwindef::DWORD, dwSpMinorVersion: ::minwindef::DWORD, pdwReturnedProductType: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:515:1 */
    pub fn GetQueuedCompletionStatusEx(CompletionPort: ::winnt::HANDLE, lpCompletionPortEntries: ::minwinbase::LPOVERLAPPED_ENTRY, ulCount: ::minwindef::ULONG, ulNumEntriesRemoved: ::minwindef::PULONG, dwMilliseconds: ::minwindef::DWORD, fAlertable: ::minwindef::BOOL) -> ::minwindef::BOOL; /* ioapiset.h:90:1 */
    pub fn GetSystemPreferredUILanguages(dwFlags: ::minwindef::DWORD, pulNumLanguages: ::minwindef::PULONG, pwszLanguagesBuffer: ::winnt::PZZWSTR, pcchLanguagesBuffer: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:2052:1 */
    pub fn GetThreadPreferredUILanguages(dwFlags: ::minwindef::DWORD, pulNumLanguages: ::minwindef::PULONG, pwszLanguagesBuffer: ::winnt::PZZWSTR, pcchLanguagesBuffer: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:2063:1 */
    pub fn GetThreadUILanguage() -> ::winnt::LANGID; /* winnls.h:2016:1 */
    pub fn GetUILanguageInfo(dwFlags: ::minwindef::DWORD, pwmszLanguage: ::winnt::PCZZWSTR, pwszFallbackLanguages: ::winnt::PZZWSTR, pcchFallbackLanguages: ::minwindef::PDWORD, pAttributes: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winnls.h:2107:1 */
    pub fn GetUserPreferredUILanguages(dwFlags: ::minwindef::DWORD, pulNumLanguages: ::minwindef::PULONG, pwszLanguagesBuffer: ::winnt::PZZWSTR, pcchLanguagesBuffer: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:2041:1 */
    pub fn GetVolumeInformationByHandleW(hFile: ::winnt::HANDLE, lpVolumeNameBuffer: ::winnt::LPWSTR, nVolumeNameSize: ::minwindef::DWORD, lpVolumeSerialNumber: ::minwindef::LPDWORD, lpMaximumComponentLength: ::minwindef::LPDWORD, lpFileSystemFlags: ::minwindef::LPDWORD, lpFileSystemNameBuffer: ::winnt::LPWSTR, nFileSystemNameSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:812:1 */
    pub fn InitializeProcThreadAttributeList(lpAttributeList: ::processthreadsapi::LPPROC_THREAD_ATTRIBUTE_LIST, dwAttributeCount: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD, lpSize: ::basetsd::PSIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:750:1 */
    pub fn IsThreadpoolTimerSet(pti: ::winnt::PTP_TIMER) -> ::minwindef::BOOL; /* threadpoolapiset.h:275:1 */
    pub fn LeaveCriticalSectionWhenCallbackReturns(pci: ::winnt::PTP_CALLBACK_INSTANCE, pcs: ::minwinbase::PCRITICAL_SECTION); /* threadpoolapiset.h:172:1 */
    pub fn MapViewOfFileExNuma(hFileMappingObject: ::winnt::HANDLE, dwDesiredAccess: ::minwindef::DWORD, dwFileOffsetHigh: ::minwindef::DWORD, dwFileOffsetLow: ::minwindef::DWORD, dwNumberOfBytesToMap: ::basetsd::SIZE_T, lpBaseAddress: ::minwindef::LPVOID, nndPreferred: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* winbase.h:6386:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::MoveFileTransactedW as MoveFileTransacted; /* winbase.h:5442:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn MoveFileTransactedA(lpExistingFileName: ::winnt::LPCSTR, lpNewFileName: ::winnt::LPCSTR, lpProgressRoutine: ::winbase::LPPROGRESS_ROUTINE, lpData: ::minwindef::LPVOID, dwFlags: ::minwindef::DWORD, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5422:1 */
    pub fn MoveFileTransactedW(lpExistingFileName: ::winnt::LPCWSTR, lpNewFileName: ::winnt::LPCWSTR, lpProgressRoutine: ::winbase::LPPROGRESS_ROUTINE, lpData: ::minwindef::LPVOID, dwFlags: ::minwindef::DWORD, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5433:1 */
    pub fn NotifyUILanguageChange(dwFlags: ::minwindef::DWORD, pcwstrNewLanguage: ::winnt::PCWSTR, pcwstrPreviousLanguage: ::winnt::PCWSTR, dwReserved: ::minwindef::DWORD, pdwStatusRtrn: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winnls.h:2119:1 */
    pub fn OpenFileById(hVolumeHint: ::winnt::HANDLE, lpFileId: ::winbase::LPFILE_ID_DESCRIPTOR, dwDesiredAccess: ::minwindef::DWORD, dwShareMode: ::minwindef::DWORD, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, dwFlagsAndAttributes: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:8490:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::OpenPrivateNamespaceW as OpenPrivateNamespace; /* winbase.h:7238:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn OpenPrivateNamespaceA(lpBoundaryDescriptor: ::minwindef::LPVOID, lpAliasPrefix: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:7230:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::QueryFullProcessImageNameW as QueryFullProcessImageName; /* winbase.h:3243:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn QueryFullProcessImageNameA(hProcess: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD, lpExeName: ::winnt::LPSTR, lpdwSize: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:3227:1 */
    pub fn QueryFullProcessImageNameW(hProcess: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD, lpExeName: ::winnt::LPWSTR, lpdwSize: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:3236:1 */
    pub fn QueryIdleProcessorCycleTime(BufferLength: ::minwindef::PULONG, ProcessorIdleCycleTime: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:63:1 */
    pub fn QueryProcessAffinityUpdateMode(hProcess: ::winnt::HANDLE, lpdwFlags: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:780:1 */
    pub fn QueryProcessCycleTime(ProcessHandle: ::winnt::HANDLE, CycleTime: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:54:1 */
    pub fn QueryThreadCycleTime(ThreadHandle: ::winnt::HANDLE, CycleTime: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:45:1 */
    pub fn QueryThreadpoolStackInformation(ptpp: ::winnt::PTP_POOL, ptpsi: ::winnt::PTP_POOL_STACK_INFORMATION) -> ::minwindef::BOOL; /* threadpoolapiset.h:100:1 */
    pub fn RegisterApplicationRecoveryCallback(pRecoveyCallback: ::winbase::APPLICATION_RECOVERY_CALLBACK, pvParameter: ::winnt::PVOID, dwPingInterval: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD) -> ::winnt::HRESULT; /* winbase.h:8123:1 */
    pub fn RegisterApplicationRestart(pwzCommandline: ::winnt::PCWSTR, dwFlags: ::minwindef::DWORD) -> ::winnt::HRESULT; /* winbase.h:8138:1 */
    pub fn ReleaseMutexWhenCallbackReturns(pci: ::winnt::PTP_CALLBACK_INSTANCE, mut_: ::winnt::HANDLE); /* threadpoolapiset.h:163:1 */
    pub fn ReleaseSemaphoreWhenCallbackReturns(pci: ::winnt::PTP_CALLBACK_INSTANCE, sem: ::winnt::HANDLE, crel: ::minwindef::DWORD); /* threadpoolapiset.h:153:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RemoveDirectoryTransactedW as RemoveDirectoryTransacted; /* winbase.h:4711:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RemoveDirectoryTransactedA(lpPathName: ::winnt::LPCSTR, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4699:1 */
    pub fn RemoveDirectoryTransactedW(lpPathName: ::winnt::LPCWSTR, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4706:1 */
    pub fn RemoveSecureMemoryCacheCallback(pfnCallBack: ::winnt::PSECURE_MEMORY_CACHE_CALLBACK) -> ::minwindef::BOOL; /* winbase.h:8609:1 */
    pub fn ReplacePartitionUnit(TargetPartition: ::winnt::PWSTR, SparePartition: ::winnt::PWSTR, Flags: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winbase.h:8588:1 */
    pub fn SetDynamicTimeZoneInformation(lpTimeZoneInformation: *const ::timezoneapi::DYNAMIC_TIME_ZONE_INFORMATION) -> ::minwindef::BOOL; /* timezoneapi.h:141:1 */
    pub fn SetEventWhenCallbackReturns(pci: ::winnt::PTP_CALLBACK_INSTANCE, evt: ::winnt::HANDLE); /* threadpoolapiset.h:144:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::SetFileAttributesTransactedW as SetFileAttributesTransacted; /* winbase.h:4853:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn SetFileAttributesTransactedA(lpFileName: ::winnt::LPCSTR, dwFileAttributes: ::minwindef::DWORD, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4839:1 */
    pub fn SetFileAttributesTransactedW(lpFileName: ::winnt::LPCWSTR, dwFileAttributes: ::minwindef::DWORD, hTransaction: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:4847:1 */
    pub fn SetFileBandwidthReservation(hFile: ::winnt::HANDLE, nPeriodMilliseconds: ::minwindef::DWORD, nBytesPerPeriod: ::minwindef::DWORD, bDiscardable: ::minwindef::BOOL, lpTransferSize: ::minwindef::LPDWORD, lpNumOutstandingRequests: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:5845:1 */
    pub fn SetFileCompletionNotificationModes(FileHandle: ::winnt::HANDLE, Flags: ::minwindef::UCHAR) -> ::minwindef::BOOL; /* winbase.h:1773:1 */
    pub fn SetFileIoOverlappedRange(FileHandle: ::winnt::HANDLE, OverlappedRangeStart: ::minwindef::PUCHAR, Length: ::minwindef::ULONG) -> ::minwindef::BOOL; /* fileapi.h:1304:1 */
    pub fn SetProcessAffinityUpdateMode(hProcess: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:771:1 */
    pub fn SetProcessDEPPolicy(dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:1637:1 */
    pub fn SetProcessPreferredUILanguages(dwFlags: ::minwindef::DWORD, pwszLanguagesBuffer: ::winnt::PCZZWSTR, pulNumLanguages: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:2032:1 */
    pub fn SetStdHandleEx(nStdHandle: ::minwindef::DWORD, hHandle: ::winnt::HANDLE, phPrevValue: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* processenv.h:128:1 */
    pub fn SetThreadPreferredUILanguages(dwFlags: ::minwindef::DWORD, pwszLanguagesBuffer: ::winnt::PCZZWSTR, pulNumLanguages: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:2074:1 */
    pub fn SetThreadpoolStackInformation(ptpp: ::winnt::PTP_POOL, ptpsi: ::winnt::PTP_POOL_STACK_INFORMATION) -> ::minwindef::BOOL; /* threadpoolapiset.h:91:1 */
    pub fn SetThreadpoolThreadMaximum(ptpp: ::winnt::PTP_POOL, cthrdMost: ::minwindef::DWORD); /* threadpoolapiset.h:73:1 */
    pub fn SetThreadpoolThreadMinimum(ptpp: ::winnt::PTP_POOL, cthrdMic: ::minwindef::DWORD) -> ::minwindef::BOOL; /* threadpoolapiset.h:82:1 */
    pub fn SetThreadpoolTimer(pti: ::winnt::PTP_TIMER, pftDueTime: ::minwindef::PFILETIME, msPeriod: ::minwindef::DWORD, msWindowLength: ::minwindef::DWORD); /* threadpoolapiset.h:264:1 */
    pub fn SetThreadpoolTimerEx(pti: ::winnt::PTP_TIMER, pftDueTime: ::minwindef::PFILETIME, msPeriod: ::minwindef::DWORD, msWindowLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* threadpoolapiset.h:386:1 */
    pub fn SetThreadpoolWait(pwa: ::winnt::PTP_WAIT, h: ::winnt::HANDLE, pftTimeout: ::minwindef::PFILETIME); /* threadpoolapiset.h:311:1 */
    pub fn SetThreadpoolWaitEx(pwa: ::winnt::PTP_WAIT, h: ::winnt::HANDLE, pftTimeout: ::minwindef::PFILETIME, Reserved: ::winnt::PVOID) -> ::minwindef::BOOL; /* threadpoolapiset.h:397:1 */
    pub fn StartThreadpoolIo(pio: ::winnt::PTP_IO); /* threadpoolapiset.h:350:1 */
    pub fn SubmitThreadpoolWork(pwk: ::winnt::PTP_WORK); /* threadpoolapiset.h:228:1 */
    pub fn TrySubmitThreadpoolCallback(pfns: ::winnt::PTP_SIMPLE_CALLBACK, pv: ::winnt::PVOID, pcbe: ::winnt::PTP_CALLBACK_ENVIRON) -> ::minwindef::BOOL; /* threadpoolapiset.h:207:1 */
    pub fn UnregisterApplicationRecoveryCallback() -> ::winnt::HRESULT; /* winbase.h:8133:1 */
    pub fn UnregisterApplicationRestart() -> ::winnt::HRESULT; /* winbase.h:8146:1 */
    pub fn UpdateProcThreadAttribute(lpAttributeList: ::processthreadsapi::LPPROC_THREAD_ATTRIBUTE_LIST, dwFlags: ::minwindef::DWORD, Attribute: ::basetsd::DWORD_PTR, lpValue: ::winnt::PVOID, cbSize: ::basetsd::SIZE_T, lpPreviousValue: ::winnt::PVOID, lpReturnSize: ::basetsd::PSIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:791:1 */
    pub fn VirtualAllocExNuma(hProcess: ::winnt::HANDLE, lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T, flAllocationType: ::minwindef::DWORD, flProtect: ::minwindef::DWORD, nndPreferred: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* memoryapi.h:632:1 */
    pub fn WaitForThreadpoolIoCallbacks(pio: ::winnt::PTP_IO, fCancelPendingCallbacks: ::minwindef::BOOL); /* threadpoolapiset.h:366:1 */
    pub fn WaitForThreadpoolTimerCallbacks(pti: ::winnt::PTP_TIMER, fCancelPendingCallbacks: ::minwindef::BOOL); /* threadpoolapiset.h:283:1 */
    pub fn WaitForThreadpoolWaitCallbacks(pwa: ::winnt::PTP_WAIT, fCancelPendingCallbacks: ::minwindef::BOOL); /* threadpoolapiset.h:321:1 */
    pub fn WaitForThreadpoolWorkCallbacks(pwk: ::winnt::PTP_WORK, fCancelPendingCallbacks: ::minwindef::BOOL); /* threadpoolapiset.h:236:1 */
    pub fn Wow64GetThreadContext(hThread: ::winnt::HANDLE, lpContext: ::winnt::PWOW64_CONTEXT) -> ::minwindef::BOOL; /* winbase.h:1811:1 */
    pub fn Wow64SetThreadContext(hThread: ::winnt::HANDLE, lpContext: *const ::winnt::WOW64_CONTEXT) -> ::minwindef::BOOL; /* winbase.h:1819:1 */
    pub fn Wow64SuspendThread(hThread: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:1846:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn AcquireSRWLockExclusive(SRWLock: ::synchapi::PSRWLOCK); /* synchapi.h:104:1 */
    pub fn AcquireSRWLockShared(SRWLock: ::synchapi::PSRWLOCK); /* synchapi.h:113:1 */
    pub fn CompareStringEx(lpLocaleName: ::winnt::LPCWSTR, dwCmpFlags: ::minwindef::DWORD, lpString1: ::winnt::LPCWCH, cchCount1: ::libc::c_int, lpString2: ::winnt::LPCWCH, cchCount2: ::libc::c_int, lpVersionInformation: ::winnls::LPNLSVERSIONINFO, lpReserved: ::minwindef::LPVOID, lParam: ::minwindef::LPARAM) -> ::libc::c_int; /* stringapiset.h:46:1 */
    pub fn CompareStringOrdinal(lpString1: ::winnt::LPCWCH, cchCount1: ::libc::c_int, lpString2: ::winnt::LPCWCH, cchCount2: ::libc::c_int, bIgnoreCase: ::minwindef::BOOL) -> ::libc::c_int; /* stringapiset.h:62:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateEventExW as CreateEventEx; /* synchapi.h:800:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateEventExA(lpEventAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpName: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD, dwDesiredAccess: ::minwindef::DWORD) -> ::winnt::HANDLE; /* synchapi.h:781:1 */
    pub fn CreateEventExW(lpEventAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, dwDesiredAccess: ::minwindef::DWORD) -> ::winnt::HANDLE; /* synchapi.h:792:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateMutexExW as CreateMutexEx; /* synchapi.h:769:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateMutexExA(lpMutexAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpName: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD, dwDesiredAccess: ::minwindef::DWORD) -> ::winnt::HANDLE; /* synchapi.h:750:1 */
    pub fn CreateMutexExW(lpMutexAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, dwDesiredAccess: ::minwindef::DWORD) -> ::winnt::HANDLE; /* synchapi.h:761:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateSemaphoreExW as CreateSemaphoreEx; /* synchapi.h:820:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateSemaphoreExW(lpSemaphoreAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lInitialCount: ::winnt::LONG, lMaximumCount: ::winnt::LONG, lpName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, dwDesiredAccess: ::minwindef::DWORD) -> ::winnt::HANDLE; /* synchapi.h:809:1 */
    pub fn EnumCalendarInfoExEx(pCalInfoEnumProcExEx: ::winnls::CALINFO_ENUMPROCEXEX, lpLocaleName: ::winnt::LPCWSTR, Calendar: ::winnls::CALID, lpReserved: ::winnt::LPCWSTR, CalType: ::winnls::CALTYPE, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winnls.h:2516:1 */
    pub fn EnumDateFormatsExEx(lpDateFmtEnumProcExEx: ::winnls::DATEFMT_ENUMPROCEXEX, lpLocaleName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winnls.h:2530:1 */
    pub fn EnumSystemLocalesEx(lpLocaleEnumProcEx: ::winnls::LOCALE_ENUMPROCEX, dwFlags: ::minwindef::DWORD, lParam: ::minwindef::LPARAM, lpReserved: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winnls.h:2554:1 */
    pub fn EnumTimeFormatsEx(lpTimeFmtEnumProcEx: ::winnls::TIMEFMT_ENUMPROCEX, lpLocaleName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, lParam: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* winnls.h:2542:1 */
    pub fn FindNLSStringEx(lpLocaleName: ::winnt::LPCWSTR, dwFindNLSStringFlags: ::minwindef::DWORD, lpStringSource: ::winnt::LPCWSTR, cchSource: ::libc::c_int, lpStringValue: ::winnt::LPCWSTR, cchValue: ::libc::c_int, pcchFound: ::minwindef::LPINT, lpVersionInformation: ::winnls::LPNLSVERSIONINFO, lpReserved: ::minwindef::LPVOID, sortHandle: ::minwindef::LPARAM) -> ::libc::c_int; /* winnls.h:2469:1 */
    pub fn FlsAlloc(lpCallback: ::winnt::PFLS_CALLBACK_FUNCTION) -> ::minwindef::DWORD; /* fibersapi.h:58:1 */
    pub fn FlsFree(dwFlsIndex: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fibersapi.h:83:1 */
    pub fn FlsGetValue(dwFlsIndex: ::minwindef::DWORD) -> ::winnt::PVOID; /* fibersapi.h:66:1 */
    pub fn FlsSetValue(dwFlsIndex: ::minwindef::DWORD, lpFlsData: ::winnt::PVOID) -> ::minwindef::BOOL; /* fibersapi.h:74:1 */
    pub fn FlushProcessWriteBuffers(); /* processthreadsapi.h:726:1 */
    pub fn GetCalendarInfoEx(lpLocaleName: ::winnt::LPCWSTR, Calendar: ::winnls::CALID, lpReserved: ::winnt::LPCWSTR, CalType: ::winnls::CALTYPE, lpCalData: ::winnt::LPWSTR, cchData: ::libc::c_int, lpValue: ::minwindef::LPDWORD) -> ::libc::c_int; /* winnls.h:2383:1 */
    pub fn GetCurrencyFormatEx(lpLocaleName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, lpValue: ::winnt::LPCWSTR, lpFormat: *const ::winnls::CURRENCYFMTW, lpCurrencyStr: ::winnt::LPWSTR, cchCurrency: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2421:1 */
    pub fn GetDurationFormatEx(lpLocaleName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, lpDuration: *const ::minwinbase::SYSTEMTIME, ullDuration: ::winnt::ULONGLONG, lpFormat: ::winnt::LPCWSTR, lpDurationStr: ::winnt::LPWSTR, cchDuration: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2396:1 */
    pub fn GetDynamicTimeZoneInformation(pTimeZoneInformation: ::timezoneapi::PDYNAMIC_TIME_ZONE_INFORMATION) -> ::minwindef::DWORD; /* timezoneapi.h:162:1 */
    pub fn GetFileInformationByHandleEx(hFile: ::winnt::HANDLE, FileInformationClass: ::minwinbase::FILE_INFO_BY_HANDLE_CLASS, lpFileInformation: ::minwindef::LPVOID, dwBufferSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:8455:1 */
    pub fn GetLocaleInfoEx(lpLocaleName: ::winnt::LPCWSTR, LCType: ::winnls::LCTYPE, lpLCData: ::winnt::LPWSTR, cchData: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2373:1 */
    pub fn GetNLSVersionEx(function: ::winnls::NLS_FUNCTION, lpLocaleName: ::winnt::LPCWSTR, lpVersionInformation: ::winnls::LPNLSVERSIONINFOEX) -> ::minwindef::BOOL; /* winnls.h:2449:1 */
    pub fn GetNumberFormatEx(lpLocaleName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, lpValue: ::winnt::LPCWSTR, lpFormat: *const ::winnls::NUMBERFMTW, lpNumberStr: ::winnt::LPWSTR, cchNumber: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2409:1 */
    pub fn GetStringScripts(dwFlags: ::minwindef::DWORD, lpString: ::winnt::LPCWSTR, cchString: ::libc::c_int, lpScripts: ::winnt::LPWSTR, cchScripts: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2345:8 */
    pub fn GetSystemDefaultLocaleName(lpLocaleName: ::winnt::LPWSTR, cchLocaleName: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2441:1 */
    pub fn GetTickCount64() -> ::winnt::ULONGLONG; /* sysinfoapi.h:221:1 */
    pub fn GetUserDefaultLocaleName(lpLocaleName: ::winnt::LPWSTR, cchLocaleName: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2433:1 */
    pub fn InitOnceBeginInitialize(lpInitOnce: ::synchapi::LPINIT_ONCE, dwFlags: ::minwindef::DWORD, fPending: ::minwindef::PBOOL, lpContext: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* synchapi.h:337:1 */
    pub fn InitOnceComplete(lpInitOnce: ::synchapi::LPINIT_ONCE, dwFlags: ::minwindef::DWORD, lpContext: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* synchapi.h:348:1 */
    pub fn InitOnceExecuteOnce(InitOnce: ::synchapi::PINIT_ONCE, InitFn: ::synchapi::PINIT_ONCE_FN, Parameter: ::winnt::PVOID, Context: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* synchapi.h:326:1 */
    pub fn InitOnceInitialize(InitOnce: ::synchapi::PINIT_ONCE); /* synchapi.h:318:1 */
    pub fn InitializeConditionVariable(ConditionVariable: ::synchapi::PCONDITION_VARIABLE); /* synchapi.h:381:1 */
    pub fn InitializeCriticalSectionEx(lpCriticalSection: ::minwinbase::LPCRITICAL_SECTION, dwSpinCount: ::minwindef::DWORD, Flags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:222:1 */
    pub fn InitializeSRWLock(SRWLock: ::synchapi::PSRWLOCK); /* synchapi.h:77:1 */
    pub fn IsNormalizedString(NormForm: ::winnls::NORM_FORM, lpString: ::winnt::LPCWSTR, cwLength: ::libc::c_int) -> ::minwindef::BOOL; /* winnls.h:2303:8 */
    pub fn IsThreadAFiber() -> ::minwindef::BOOL; /* fibersapi.h:107:1 */
    pub fn IsValidLocaleName(lpLocaleName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winnls.h:2507:1 */
    pub fn LCIDToLocaleName(Locale: ::winnt::LCID, lpName: ::winnt::LPWSTR, cchName: ::libc::c_int, dwFlags: ::minwindef::DWORD) -> ::libc::c_int; /* winnls.h:1652:1 */
    pub fn LCMapStringEx(lpLocaleName: ::winnt::LPCWSTR, dwMapFlags: ::minwindef::DWORD, lpSrcStr: ::winnt::LPCWSTR, cchSrc: ::libc::c_int, lpDestStr: ::winnt::LPWSTR, cchDest: ::libc::c_int, lpVersionInformation: ::winnls::LPNLSVERSIONINFO, lpReserved: ::minwindef::LPVOID, sortHandle: ::minwindef::LPARAM) -> ::libc::c_int; /* winnls.h:2492:1 */
    pub fn LocaleNameToLCID(lpName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD) -> ::winnt::LCID; /* winnls.h:1661:1 */
    pub fn NormalizeString(NormForm: ::winnls::NORM_FORM, lpSrcString: ::winnt::LPCWSTR, cwSrcLength: ::libc::c_int, lpDstString: ::winnt::LPWSTR, cwDstLength: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2295:8 */
    pub fn ReleaseSRWLockExclusive(SRWLock: ::synchapi::PSRWLOCK); /* synchapi.h:86:1 */
    pub fn ReleaseSRWLockShared(SRWLock: ::synchapi::PSRWLOCK); /* synchapi.h:95:1 */
    pub fn SetFileInformationByHandle(hFile: ::winnt::HANDLE, FileInformationClass: ::minwinbase::FILE_INFO_BY_HANDLE_CLASS, lpFileInformation: ::minwindef::LPVOID, dwBufferSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1037:1 */
    pub fn SleepConditionVariableCS(ConditionVariable: ::synchapi::PCONDITION_VARIABLE, CriticalSection: ::minwinbase::PCRITICAL_SECTION, dwMilliseconds: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:405:1 */
    pub fn SleepConditionVariableSRW(ConditionVariable: ::synchapi::PCONDITION_VARIABLE, SRWLock: ::synchapi::PSRWLOCK, dwMilliseconds: ::minwindef::DWORD, Flags: ::minwindef::ULONG) -> ::minwindef::BOOL; /* synchapi.h:415:1 */
    pub fn TryAcquireSRWLockExclusive(SRWLock: ::synchapi::PSRWLOCK) -> ::winnt::BOOLEAN; /* synchapi.h:122:1 */
    pub fn TryAcquireSRWLockShared(SRWLock: ::synchapi::PSRWLOCK) -> ::winnt::BOOLEAN; /* synchapi.h:131:1 */
    pub fn VerifyScripts(dwFlags: ::minwindef::DWORD, lpLocaleScripts: ::winnt::LPCWSTR, cchLocaleScripts: ::libc::c_int, lpTestScripts: ::winnt::LPCWSTR, cchTestScripts: ::libc::c_int) -> ::minwindef::BOOL; /* winnls.h:2336:8 */
    pub fn WakeAllConditionVariable(ConditionVariable: ::synchapi::PCONDITION_VARIABLE); /* synchapi.h:397:1 */
    pub fn WakeConditionVariable(ConditionVariable: ::synchapi::PCONDITION_VARIABLE); /* synchapi.h:389:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn CreateUmsCompletionList(UmsCompletionList: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winbase.h:1449:1 */
    pub fn CreateUmsThreadContext(lpUmsThread: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winbase.h:1536:1 */
    pub fn DeleteUmsCompletionList(UmsCompletionList: ::winbase::PUMS_COMPLETION_LIST) -> ::minwindef::BOOL; /* winbase.h:1487:1 */
    pub fn DeleteUmsThreadContext(UmsThread: ::winbase::PUMS_CONTEXT) -> ::minwindef::BOOL; /* winbase.h:1529:1 */
    pub fn DequeueUmsCompletionListItems(UmsCompletionList: ::winbase::PUMS_COMPLETION_LIST, WaitTimeOut: ::minwindef::DWORD, UmsThreadList: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winbase.h:1456:1 */
    pub fn DisableThreadProfiling(PerformanceDataHandle: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winbase.h:8723:1 */
    pub fn EnableThreadProfiling(ThreadHandle: ::winnt::HANDLE, Flags: ::minwindef::DWORD, HardwareCounters: ::basetsd::DWORD64, PerformanceDataHandle: *mut *mut ::libc::c_void) -> ::minwindef::DWORD; /* winbase.h:8713:1 */
    pub fn EnterUmsSchedulingMode(SchedulerStartupInfo: ::winbase::PUMS_SCHEDULER_STARTUP_INFO) -> ::minwindef::BOOL; /* winbase.h:1543:1 */
    pub fn ExecuteUmsThread(UmsThread: ::winbase::PUMS_CONTEXT) -> ::minwindef::BOOL; /* winbase.h:1473:1 */
    pub fn GetActiveProcessorCount(GroupNumber: ::minwindef::WORD) -> ::minwindef::DWORD; /* winbase.h:7984:1 */
    pub fn GetActiveProcessorGroupCount() -> ::minwindef::WORD; /* winbase.h:7970:1 */
    pub fn GetCurrentProcessorNumberEx(ProcNumber: ::winnt::PPROCESSOR_NUMBER); /* processthreadsapi.h:1041:1 */
    pub fn GetCurrentUmsThread() -> ::winbase::PUMS_CONTEXT; /* winbase.h:1494:1 */
    pub fn GetLogicalProcessorInformationEx(RelationshipType: ::winnt::LOGICAL_PROCESSOR_RELATIONSHIP, Buffer: ::winnt::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, ReturnedLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:467:1 */
    pub fn GetMaximumProcessorCount(GroupNumber: ::minwindef::WORD) -> ::minwindef::DWORD; /* winbase.h:7991:1 */
    pub fn GetMaximumProcessorGroupCount() -> ::minwindef::WORD; /* winbase.h:7977:1 */
    pub fn GetNextUmsListItem(UmsContext: ::winbase::PUMS_CONTEXT) -> ::winbase::PUMS_CONTEXT; /* winbase.h:1501:1 */
    pub fn GetNumaAvailableMemoryNodeEx(Node: ::minwindef::USHORT, AvailableBytes: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* winbase.h:8054:1 */
    pub fn GetNumaNodeNumberFromHandle(hFile: ::winnt::HANDLE, NodeNumber: ::minwindef::PUSHORT) -> ::minwindef::BOOL; /* winbase.h:8014:1 */
    pub fn GetNumaNodeProcessorMaskEx(Node: ::minwindef::USHORT, ProcessorMask: ::winnt::PGROUP_AFFINITY) -> ::minwindef::BOOL; /* systemtopologyapi.h:54:1 */
    pub fn GetNumaProcessorNodeEx(Processor: ::winnt::PPROCESSOR_NUMBER, NodeNumber: ::minwindef::PUSHORT) -> ::minwindef::BOOL; /* winbase.h:8026:1 */
    pub fn GetNumaProximityNodeEx(ProximityId: ::minwindef::ULONG, NodeNumber: ::minwindef::PUSHORT) -> ::minwindef::BOOL; /* winbase.h:8078:1 */
    pub fn GetProcessGroupAffinity(hProcess: ::winnt::HANDLE, GroupCount: ::minwindef::PUSHORT, GroupArray: ::minwindef::PUSHORT) -> ::minwindef::BOOL; /* processtopologyapi.h:49:1 */
    pub fn GetProcessorSystemCycleTime(Group: ::minwindef::USHORT, Buffer: ::winnt::PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, ReturnedLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:1120:1 */
    pub fn GetThreadGroupAffinity(hThread: ::winnt::HANDLE, GroupAffinity: ::winnt::PGROUP_AFFINITY) -> ::minwindef::BOOL; /* processtopologyapi.h:72:1 */
    pub fn GetThreadIdealProcessorEx(hThread: ::winnt::HANDLE, lpIdealProcessor: ::winnt::PPROCESSOR_NUMBER) -> ::minwindef::BOOL; /* processthreadsapi.h:1032:1 */
    pub fn GetUmsCompletionListEvent(UmsCompletionList: ::winbase::PUMS_COMPLETION_LIST, UmsCompletionEvent: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* winbase.h:1465:1 */
    pub fn GetUmsSystemThreadInformation(ThreadHandle: ::winnt::HANDLE, SystemThreadInfo: ::winbase::PUMS_SYSTEM_THREAD_INFORMATION) -> ::minwindef::BOOL; /* winbase.h:1550:1 */
    pub fn PowerClearRequest(PowerRequest: ::winnt::HANDLE, RequestType: ::winnt::POWER_REQUEST_TYPE) -> ::minwindef::BOOL; /* winbase.h:1708:1 */
    pub fn PowerCreateRequest(Context: ::minwinbase::PREASON_CONTEXT) -> ::winnt::HANDLE; /* winbase.h:1693:1 */
    pub fn PowerSetRequest(PowerRequest: ::winnt::HANDLE, RequestType: ::winnt::POWER_REQUEST_TYPE) -> ::minwindef::BOOL; /* winbase.h:1700:1 */
    pub fn QueryIdleProcessorCycleTimeEx(Group: ::minwindef::USHORT, BufferLength: ::minwindef::PULONG, ProcessorIdleCycleTime: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:77:1 */
    pub fn QueryThreadProfiling(ThreadHandle: ::winnt::HANDLE, Enabled: ::winnt::PBOOLEAN) -> ::minwindef::DWORD; /* winbase.h:8730:1 */
    pub fn QueryUmsThreadInformation(UmsThread: ::winbase::PUMS_CONTEXT, UmsThreadInfoClass: ::winbase::UMS_THREAD_INFO_CLASS, UmsThreadInformation: ::winnt::PVOID, UmsThreadInformationLength: ::minwindef::ULONG, ReturnLength: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winbase.h:1508:1 */
    pub fn ReadThreadProfilingData(PerformanceDataHandle: ::winnt::HANDLE, Flags: ::minwindef::DWORD, PerformanceData: ::winnt::PPERFORMANCE_DATA) -> ::minwindef::DWORD; /* winbase.h:8738:1 */
    pub fn SetThreadGroupAffinity(hThread: ::winnt::HANDLE, GroupAffinity: *const ::winnt::GROUP_AFFINITY, PreviousGroupAffinity: ::winnt::PGROUP_AFFINITY) -> ::minwindef::BOOL; /* processtopologyapi.h:81:1 */
    pub fn SetUmsThreadInformation(UmsThread: ::winbase::PUMS_CONTEXT, UmsThreadInfoClass: ::winbase::UMS_THREAD_INFO_CLASS, UmsThreadInformation: ::winnt::PVOID, UmsThreadInformationLength: ::minwindef::ULONG) -> ::minwindef::BOOL; /* winbase.h:1519:1 */
    pub fn SetWaitableTimerEx(hTimer: ::winnt::HANDLE, lpDueTime: *const ::winnt::LARGE_INTEGER, lPeriod: ::winnt::LONG, pfnCompletionRoutine: ::synchapi::PTIMERAPCROUTINE, lpArgToCompletionRoutine: ::minwindef::LPVOID, WakeContext: ::minwinbase::PREASON_CONTEXT, TolerableDelay: ::minwindef::ULONG) -> ::minwindef::BOOL; /* synchapi.h:700:1 */
    pub fn UmsThreadYield(SchedulerParam: ::winnt::PVOID) -> ::minwindef::BOOL; /* winbase.h:1480:1 */
    pub fn Wow64GetThreadSelectorEntry(hThread: ::winnt::HANDLE, dwSelector: ::minwindef::DWORD, lpSelectorEntry: ::winnt::PWOW64_LDT_ENTRY) -> ::minwindef::BOOL; /* winbase.h:1831:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn CopyFile2(pwszExistingFileName: ::winnt::PCWSTR, pwszNewFileName: ::winnt::PCWSTR, pExtendedParameters: *mut ::winbase::COPYFILE2_EXTENDED_PARAMETERS) -> ::winnt::HRESULT; /* winbase.h:5300:1 */
    pub fn FindStringOrdinal(dwFindStringOrdinalFlags: ::minwindef::DWORD, lpStringSource: ::winnt::LPCWSTR, cchSource: ::libc::c_int, lpStringValue: ::winnt::LPCWSTR, cchValue: ::libc::c_int, bIgnoreCase: ::minwindef::BOOL) -> ::libc::c_int; /* libloaderapi.h:198:1 */
    pub fn GetTimeZoneInformationForYear(wYear: ::minwindef::USHORT, pdtzi: ::timezoneapi::PDYNAMIC_TIME_ZONE_INFORMATION, ptzi: ::timezoneapi::LPTIME_ZONE_INFORMATION) -> ::minwindef::BOOL; /* timezoneapi.h:175:1 */
    pub fn QueryUnbiasedInterruptTime(UnbiasedTime: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* realtimeapiset.h:99:1 */
    pub fn ResolveLocaleName(lpNameToResolve: ::winnt::LPCWSTR, lpLocaleName: ::winnt::LPWSTR, cchLocaleName: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2574:1 */
    pub fn SetThreadIdealProcessorEx(hThread: ::winnt::HANDLE, lpIdealProcessor: ::winnt::PPROCESSOR_NUMBER, lpPreviousIdealProcessor: ::winnt::PPROCESSOR_NUMBER) -> ::minwindef::BOOL; /* processthreadsapi.h:1015:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010100"))] #[cfg(any(target_arch="x86", target_arch="x86_64"))] 
extern "system" {
    pub fn SetXStateFeaturesMask(Context: ::winnt::PCONTEXT, FeatureMask: ::basetsd::DWORD64) -> ::minwindef::BOOL; /* winbase.h:8693:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010100"))] 
extern "system" {
    pub fn CopyContext(Destination: ::winnt::PCONTEXT, ContextFlags: ::minwindef::DWORD, Source: ::winnt::PCONTEXT) -> ::minwindef::BOOL; /* winbase.h:8627:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010100"))] #[cfg(any(target_arch="x86", target_arch="x86_64"))] 
extern "system" {
    pub fn GetEnabledXStateFeatures() -> ::basetsd::DWORD64; /* winbase.h:8660:1 */
    pub fn GetXStateFeaturesMask(Context: ::winnt::PCONTEXT, FeatureMask: ::basetsd::PDWORD64) -> ::minwindef::BOOL; /* winbase.h:8668:1 */
    pub fn LocateXStateFeature(Context: ::winnt::PCONTEXT, FeatureId: ::minwindef::DWORD, Length: ::minwindef::PDWORD) -> ::winnt::PVOID; /* winbase.h:8677:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010100"))] 
extern "system" {
    pub fn InitializeContext(Buffer: ::winnt::PVOID, ContextFlags: ::minwindef::DWORD, Context: *mut *mut ::winnt::CONTEXT, ContextLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:8643:1 */
}
#[cfg(any(feature="winapi_ver_06020000"))] #[doc(inline)] pub use self::InterlockedPushListSListEx as InterlockedPushListSList; /* interlockedapi.h:75:9 */
#[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn InterlockedPushListSListEx(ListHead: ::winnt::PSLIST_HEADER, List: ::winnt::PSLIST_ENTRY, ListEnd: ::winnt::PSLIST_ENTRY, Count: ::minwindef::ULONG) -> ::winnt::PSLIST_ENTRY; /* interlockedapi.h:80:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn AddResourceAttributeAce(pAcl: ::winnt::PACL, dwAceRevision: ::minwindef::DWORD, AceFlags: ::minwindef::DWORD, AccessMask: ::minwindef::DWORD, pSid: ::winnt::PSID, pAttributeInfo: ::winnt::PCLAIM_SECURITY_ATTRIBUTES_INFORMATION, pReturnLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:362:1 */
    pub fn AddScopedPolicyIDAce(pAcl: ::winnt::PACL, dwAceRevision: ::minwindef::DWORD, AceFlags: ::minwindef::DWORD, AccessMask: ::minwindef::DWORD, pSid: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:376:1 */
    pub fn CheckTokenCapability(TokenHandle: ::winnt::HANDLE, CapabilitySidToCheck: ::winnt::PSID, HasCapability: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:477:1 */
    pub fn CheckTokenMembershipEx(TokenHandle: ::winnt::HANDLE, SidToCheck: ::winnt::PSID, Flags: ::minwindef::DWORD, IsMember: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:498:1 */
    pub fn GetAppContainerAce(Acl: ::winnt::PACL, StartingAceIndex: ::minwindef::DWORD, AppContainerAce: *mut *mut ::libc::c_void, AppContainerAceIndex: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* securitybaseapi.h:487:1 */
    pub fn GetAppContainerNamedObjectPath(Token: ::winnt::HANDLE, AppContainerSid: ::winnt::PSID, ObjectPathLength: ::minwindef::ULONG, ObjectPath: ::winnt::LPWSTR, ReturnLength: ::minwindef::PULONG) -> ::minwindef::BOOL; /* securityappcontainer.h:51:1 */
    pub fn GetCachedSigningLevel(File: ::winnt::HANDLE, Flags: ::minwindef::PULONG, SigningLevel: ::minwindef::PULONG, Thumbprint: ::minwindef::PUCHAR, ThumbprintSize: ::minwindef::PULONG, ThumbprintAlgorithm: ::minwindef::PULONG) -> ::minwindef::BOOL; /* securitybaseapi.h:1318:1 */
    pub fn GetCurrentThreadStackLimits(LowLimit: ::basetsd::PULONG_PTR, HighLimit: ::basetsd::PULONG_PTR); /* processthreadsapi.h:848:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[doc(inline)] pub use self::GetFirmwareEnvironmentVariableExW as GetFirmwareEnvironmentVariableEx; /* winbase.h:3582:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn GetFirmwareEnvironmentVariableExA(lpName: ::winnt::LPCSTR, lpGuid: ::winnt::LPCSTR, pBuffer: ::winnt::PVOID, nSize: ::minwindef::DWORD, pdwAttribubutes: ::minwindef::PDWORD) -> ::minwindef::DWORD; /* winbase.h:3564:1 */
    pub fn GetFirmwareEnvironmentVariableExW(lpName: ::winnt::LPCWSTR, lpGuid: ::winnt::LPCWSTR, pBuffer: ::winnt::PVOID, nSize: ::minwindef::DWORD, pdwAttribubutes: ::minwindef::PDWORD) -> ::minwindef::DWORD; /* winbase.h:3574:1 */
    pub fn GetFirmwareType(FirmwareType: ::winnt::PFIRMWARE_TYPE) -> ::minwindef::BOOL; /* winbase.h:3648:1 */
    pub fn GetMemoryErrorHandlingCapabilities(Capabilities: ::minwindef::PULONG) -> ::minwindef::BOOL; /* memoryapi.h:653:1 */
    pub fn GetProcessInformation(hProcess: ::winnt::HANDLE, ProcessInformationClass: ::winbase::PROCESS_INFORMATION_CLASS, ProcessInformation: ::minwindef::LPVOID, ProcessInformationSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:1610:1 */
    pub fn GetProcessMitigationPolicy(hProcess: ::winnt::HANDLE, MitigationPolicy: ::winnt::PROCESS_MITIGATION_POLICY, lpBuffer: ::winnt::PVOID, dwLength: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:890:1 */
    pub fn GetThreadInformation(hThread: ::winnt::HANDLE, ThreadInformationClass: ::processthreadsapi::THREAD_INFORMATION_CLASS, ThreadInformation: ::minwindef::LPVOID, ThreadInformationSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:1119:1 */
    pub fn IsNativeVhdBoot(NativeVhdBoot: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* winbase.h:3656:1 */
    pub fn PrefetchVirtualMemory(hProcess: ::winnt::HANDLE, NumberOfEntries: ::basetsd::ULONG_PTR, VirtualAddresses: ::memoryapi::PWIN32_MEMORY_RANGE_ENTRY, Flags: ::minwindef::ULONG) -> ::minwindef::BOOL; /* memoryapi.h:504:1 */
    pub fn RegisterBadMemoryNotification(Callback: ::memoryapi::PBAD_MEMORY_CALLBACK_ROUTINE) -> ::winnt::PVOID; /* memoryapi.h:672:1 */
    pub fn SetCachedSigningLevel(SourceFiles: ::winnt::PHANDLE, SourceFileCount: ::minwindef::ULONG, Flags: ::minwindef::ULONG, TargetFile: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:1307:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[doc(inline)] pub use self::SetFirmwareEnvironmentVariableExW as SetFirmwareEnvironmentVariableEx; /* winbase.h:3636:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn SetFirmwareEnvironmentVariableExA(lpName: ::winnt::LPCSTR, lpGuid: ::winnt::LPCSTR, pValue: ::winnt::PVOID, nSize: ::minwindef::DWORD, dwAttributes: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3618:1 */
    pub fn SetFirmwareEnvironmentVariableExW(lpName: ::winnt::LPCWSTR, lpGuid: ::winnt::LPCWSTR, pValue: ::winnt::PVOID, nSize: ::minwindef::DWORD, dwAttributes: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3628:1 */
    pub fn SetProcessInformation(hProcess: ::winnt::HANDLE, ProcessInformationClass: ::winbase::PROCESS_INFORMATION_CLASS, ProcessInformation: ::minwindef::LPVOID, ProcessInformationSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:1620:1 */
    pub fn SetProcessMitigationPolicy(MitigationPolicy: ::winnt::PROCESS_MITIGATION_POLICY, lpBuffer: ::winnt::PVOID, dwLength: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:880:1 */
    pub fn SetThreadInformation(hThread: ::winnt::HANDLE, ThreadInformationClass: ::processthreadsapi::THREAD_INFORMATION_CLASS, ThreadInformation: ::minwindef::LPVOID, ThreadInformationSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:1130:1 */
    pub fn UnmapViewOfFileEx(BaseAddress: ::winnt::PVOID, UnmapFlags: ::minwindef::ULONG) -> ::minwindef::BOOL; /* memoryapi.h:515:1 */
    pub fn UnregisterBadMemoryNotification(RegistrationHandle: ::winnt::PVOID) -> ::minwindef::BOOL; /* memoryapi.h:681:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn CreateFile2(lpFileName: ::winnt::LPCWSTR, dwDesiredAccess: ::minwindef::DWORD, dwShareMode: ::minwindef::DWORD, dwCreationDisposition: ::minwindef::DWORD, pCreateExParams: ::fileapi::LPCREATEFILE2_EXTENDED_PARAMETERS) -> ::winnt::HANDLE; /* fileapi.h:1272:1 */
    pub fn CreateFileMappingFromApp(hFile: ::winnt::HANDLE, SecurityAttributes: ::minwinbase::PSECURITY_ATTRIBUTES, PageProtection: ::minwindef::ULONG, MaximumSize: ::basetsd::ULONG64, Name: ::winnt::PCWSTR) -> ::winnt::HANDLE; /* memoryapi.h:537:1 */
    pub fn IsValidNLSVersion(function: ::winnls::NLS_FUNCTION, lpLocaleName: ::winnt::LPCWSTR, lpVersionInformation: ::winnls::LPNLSVERSIONINFOEX) -> ::minwindef::DWORD; /* winnls.h:2459:1 */
    pub fn LoadPackagedLibrary(lpwLibFileName: ::winnt::LPCWSTR, Reserved: ::minwindef::DWORD) -> ::minwindef::HMODULE; /* winbase.h:3207:1 */
    pub fn MapViewOfFileFromApp(hFileMappingObject: ::winnt::HANDLE, DesiredAccess: ::minwindef::ULONG, FileOffset: ::basetsd::ULONG64, NumberOfBytesToMap: ::basetsd::SIZE_T) -> ::winnt::PVOID; /* memoryapi.h:550:1 */
    pub fn SystemTimeToTzSpecificLocalTimeEx(lpTimeZoneInformation: *const ::timezoneapi::DYNAMIC_TIME_ZONE_INFORMATION, lpUniversalTime: *const ::minwinbase::SYSTEMTIME, lpLocalTime: ::minwinbase::LPSYSTEMTIME) -> ::minwindef::BOOL; /* timezoneapi.h:214:1 */
    pub fn TzSpecificLocalTimeToSystemTimeEx(lpTimeZoneInformation: *const ::timezoneapi::DYNAMIC_TIME_ZONE_INFORMATION, lpLocalTime: *const ::minwinbase::SYSTEMTIME, lpUniversalTime: ::minwinbase::LPSYSTEMTIME) -> ::minwindef::BOOL; /* timezoneapi.h:225:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_0a000000"))] 
extern "system" {
    pub fn QueryProtectedPolicy(PolicyGuid: ::guiddef::LPCGUID, PolicyValue: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* processthreadsapi.h:1185:1 */
    pub fn SetProtectedPolicy(PolicyGuid: ::guiddef::LPCGUID, PolicyValue: ::basetsd::ULONG_PTR, OldPolicyValue: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* processthreadsapi.h:1175:1 */
}
