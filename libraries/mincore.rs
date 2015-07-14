
extern "system" {
    pub fn InitializeSListHead(_: ::winnt::PSLIST_HEADER); /* interlockedapi.h:50:1 */
    pub fn InterlockedFlushSList(_: ::winnt::PSLIST_HEADER) -> ::winnt::PSLIST_ENTRY; /* interlockedapi.h:93:1 */
    pub fn InterlockedPopEntrySList(_: ::winnt::PSLIST_HEADER) -> ::winnt::PSLIST_ENTRY; /* interlockedapi.h:58:1 */
    pub fn InterlockedPushEntrySList(_: ::winnt::PSLIST_HEADER, _: ::winnt::PSLIST_ENTRY) -> ::winnt::PSLIST_ENTRY; /* interlockedapi.h:66:1 */
    pub fn QueryDepthSList(_: ::winnt::PSLIST_HEADER) -> ::minwindef::USHORT; /* interlockedapi.h:101:1 */
    pub fn QueryPerformanceCounter(_: *mut ::winnt::LARGE_INTEGER) -> ::minwindef::BOOL; /* profileapi.h:42:1 */
    pub fn QueryPerformanceFrequency(_: *mut ::winnt::LARGE_INTEGER) -> ::minwindef::BOOL; /* profileapi.h:50:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::CharUpperW as ua_CharUpperW; /* stralign.h:93:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn GetWriteWatch(_: ::minwindef::DWORD, _: ::winnt::PVOID, _: ::basetsd::SIZE_T, _: *mut *mut ::libc::c_void, _: *mut ::libc::c_ulonglong, _: ::minwindef::LPDWORD) -> ::minwindef::UINT; /* memoryapi.h:389:1 */
    pub fn ReadProcessMemory(_: ::winnt::HANDLE, _: ::minwindef::LPCVOID, _: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: *mut ::libc::c_ulonglong) -> ::minwindef::BOOL; /* memoryapi.h:217:1 */
    pub fn RtlAddFunctionTable(_: ::winnt::PRUNTIME_FUNCTION, _: ::minwindef::DWORD, _: ::basetsd::DWORD64) -> ::winnt::BOOLEAN; /* winnt.h:16974:1 */
    pub fn RtlInstallFunctionTableCallback(_: ::basetsd::DWORD64, _: ::basetsd::DWORD64, _: ::minwindef::DWORD, _: ::winnt::PGET_RUNTIME_FUNCTION_CALLBACK, _: ::winnt::PVOID, _: ::winnt::PCWSTR) -> ::winnt::BOOLEAN; /* winnt.h:16992:1 */
    pub fn WriteProcessMemory(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::minwindef::LPCVOID, _: ::basetsd::SIZE_T, _: *mut ::libc::c_ulonglong) -> ::minwindef::BOOL; /* memoryapi.h:230:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn RtlAddFunctionTable(_: ::winnt::PRUNTIME_FUNCTION, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winnt::BOOLEAN; /* winnt.h:17132:1 */
    pub fn RtlInstallFunctionTableCallback(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PGET_RUNTIME_FUNCTION_CALLBACK, _: ::winnt::PVOID, _: ::winnt::PCWSTR) -> ::winnt::BOOLEAN; /* winnt.h:17150:1 */
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
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AbortSystemShutdownW as AbortSystemShutdown; /* winreg.h:1307:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AbortSystemShutdownW(_: ::winnt::LPWSTR) -> ::minwindef::BOOL; /* winreg.h:1303:1 */
    pub fn AccessCheck(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winnt::PGENERIC_MAPPING, _: ::winnt::PPRIVILEGE_SET, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:56:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AccessCheckAndAuditAlarmW as AccessCheckAndAuditAlarm; /* securitybaseapi.h:87:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AccessCheckAndAuditAlarmW(_: ::winnt::LPCWSTR, _: ::minwindef::LPVOID, _: ::winnt::LPWSTR, _: ::winnt::LPWSTR, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::DWORD, _: ::winnt::PGENERIC_MAPPING, _: ::minwindef::BOOL, _: ::minwindef::LPDWORD, _: ::minwindef::LPBOOL, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:71:1 */
    pub fn AccessCheckByType(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSID, _: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winnt::POBJECT_TYPE_LIST, _: ::minwindef::DWORD, _: ::winnt::PGENERIC_MAPPING, _: ::winnt::PPRIVILEGE_SET, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:93:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AccessCheckByTypeAndAuditAlarmW as AccessCheckByTypeAndAuditAlarm; /* securitybaseapi.h:150:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AccessCheckByTypeAndAuditAlarmW(_: ::winnt::LPCWSTR, _: ::minwindef::LPVOID, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSID, _: ::minwindef::DWORD, _: ::winnt::AUDIT_EVENT_TYPE, _: ::minwindef::DWORD, _: ::winnt::POBJECT_TYPE_LIST, _: ::minwindef::DWORD, _: ::winnt::PGENERIC_MAPPING, _: ::minwindef::BOOL, _: ::minwindef::LPDWORD, _: ::minwindef::LPBOOL, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:129:1 */
    pub fn AccessCheckByTypeResultList(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSID, _: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winnt::POBJECT_TYPE_LIST, _: ::minwindef::DWORD, _: ::winnt::PGENERIC_MAPPING, _: ::winnt::PPRIVILEGE_SET, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:111:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AccessCheckByTypeResultListAndAuditAlarmW as AccessCheckByTypeResultListAndAuditAlarm; /* securitybaseapi.h:177:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AccessCheckByTypeResultListAndAuditAlarmByHandleW as AccessCheckByTypeResultListAndAuditAlarmByHandle; /* securitybaseapi.h:205:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleW(_: ::winnt::LPCWSTR, _: ::minwindef::LPVOID, _: ::winnt::HANDLE, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSID, _: ::minwindef::DWORD, _: ::winnt::AUDIT_EVENT_TYPE, _: ::minwindef::DWORD, _: ::winnt::POBJECT_TYPE_LIST, _: ::minwindef::DWORD, _: ::winnt::PGENERIC_MAPPING, _: ::minwindef::BOOL, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:183:1 */
    pub fn AccessCheckByTypeResultListAndAuditAlarmW(_: ::winnt::LPCWSTR, _: ::minwindef::LPVOID, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSID, _: ::minwindef::DWORD, _: ::winnt::AUDIT_EVENT_TYPE, _: ::minwindef::DWORD, _: ::winnt::POBJECT_TYPE_LIST, _: ::minwindef::DWORD, _: ::winnt::PGENERIC_MAPPING, _: ::minwindef::BOOL, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:156:1 */
    pub fn AddAccessAllowedAce(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:211:1 */
    pub fn AddAccessAllowedAceEx(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:222:1 */
    pub fn AddAccessAllowedObjectAce(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *mut ::guiddef::GUID, _: *mut ::guiddef::GUID, _: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:234:1 */
    pub fn AddAccessDeniedAce(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:248:1 */
    pub fn AddAccessDeniedAceEx(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:259:1 */
    pub fn AddAccessDeniedObjectAce(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *mut ::guiddef::GUID, _: *mut ::guiddef::GUID, _: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:271:1 */
    pub fn AddAce(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:285:1 */
    pub fn AddAuditAccessAce(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PSID, _: ::minwindef::BOOL, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:297:1 */
    pub fn AddAuditAccessAceEx(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PSID, _: ::minwindef::BOOL, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:310:1 */
    pub fn AddAuditAccessObjectAce(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *mut ::guiddef::GUID, _: *mut ::guiddef::GUID, _: ::winnt::PSID, _: ::minwindef::BOOL, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:324:1 */
    pub fn AddDllDirectory(_: ::winnt::PCWSTR) -> ::libloaderapi::DLL_DIRECTORY_COOKIE; /* libloaderapi.h:498:1 */
    pub fn AddSIDToBoundaryDescriptor(_: *mut *mut ::libc::c_void, _: ::winnt::PSID) -> ::minwindef::BOOL; /* namespaceapi.h:82:1 */
    pub fn AdjustTokenGroups(_: ::winnt::HANDLE, _: ::minwindef::BOOL, _: ::winnt::PTOKEN_GROUPS, _: ::minwindef::DWORD, _: ::winnt::PTOKEN_GROUPS, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:392:1 */
    pub fn AdjustTokenPrivileges(_: ::winnt::HANDLE, _: ::minwindef::BOOL, _: ::winnt::PTOKEN_PRIVILEGES, _: ::minwindef::DWORD, _: ::winnt::PTOKEN_PRIVILEGES, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:405:1 */
    pub fn AllocConsole() -> ::minwindef::BOOL; /* consoleapi.h:44:1 */
    pub fn AllocateAndInitializeSid(_: ::winnt::PSID_IDENTIFIER_AUTHORITY, _: ::minwindef::BYTE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:418:1 */
    pub fn AllocateLocallyUniqueId(_: ::winnt::PLUID) -> ::minwindef::BOOL; /* securitybaseapi.h:436:1 */
    pub fn AreAllAccessesGranted(_: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:444:1 */
    pub fn AreAnyAccessesGranted(_: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:453:1 */
    pub fn Beep(_: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* utilapiset.h:85:1 */
    pub fn CancelIo(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* ioapiset.h:178:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ChangeServiceConfigW as ChangeServiceConfig; /* winsvc.h:1035:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ChangeServiceConfig2W as ChangeServiceConfig2; /* winsvc.h:1057:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ChangeServiceConfig2A(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winsvc.h:1043:1 */
    pub fn ChangeServiceConfig2W(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winsvc.h:1051:1 */
    pub fn ChangeServiceConfigA(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::LPDWORD, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winsvc.h:1005:1 */
    pub fn ChangeServiceConfigW(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::LPDWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winsvc.h:1021:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharLowerW as CharLower; /* winuser.h:5424:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharLowerBuffW as CharLowerBuff; /* winuser.h:5442:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharLowerBuffW(_: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:5438:1 */
    pub fn CharLowerW(_: ::winnt::LPWSTR) -> ::winnt::LPWSTR; /* winuser.h:5421:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharNextW as CharNext; /* winuser.h:5458:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharNextW(_: ::winnt::LPCWSTR) -> ::winnt::LPWSTR; /* winuser.h:5455:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharPrevW as CharPrev; /* winuser.h:5476:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharPrevW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::winnt::LPWSTR; /* winuser.h:5472:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharUpperW as CharUpper; /* winuser.h:5390:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharUpperBuffW as CharUpperBuff; /* winuser.h:5408:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharUpperBuffW(_: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:5404:1 */
    pub fn CharUpperW(_: ::winnt::LPWSTR) -> ::winnt::LPWSTR; /* winuser.h:5387:1 */
    pub fn CheckTokenMembership(_: ::winnt::HANDLE, _: ::winnt::PSID, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:462:1 */
    pub fn ClearCommBreak(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:2004:1 */
    pub fn ClearCommError(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::winbase::LPCOMSTAT) -> ::minwindef::BOOL; /* winbase.h:2011:1 */
    pub fn ClosePrivateNamespace(_: ::winnt::HANDLE, _: ::minwindef::ULONG) -> ::winnt::BOOLEAN; /* namespaceapi.h:64:1 */
    pub fn CloseServiceHandle(_: ::winsvc::SC_HANDLE) -> ::minwindef::BOOL; /* winsvc.h:1065:1 */
    pub fn CompareFileTime(_: *const ::minwindef::FILETIME, _: *const ::minwindef::FILETIME) -> ::winnt::LONG; /* fileapi.h:77:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CompareStringW as CompareString; /* stringapiset.h:93:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CompareStringW(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::PCNZWCH, _: ::libc::c_int, _: ::winnt::PCNZWCH, _: ::libc::c_int) -> ::libc::c_int; /* stringapiset.h:83:1 */
    pub fn ConnectNamedPipe(_: ::winnt::HANDLE, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* namedpipeapi.h:61:1 */
    pub fn ContinueDebugEvent(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:117:1 */
    pub fn ControlService(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::winsvc::LPSERVICE_STATUS) -> ::minwindef::BOOL; /* winsvc.h:1072:1 */
    pub fn ConvertDefaultLocale(_: ::winnt::LCID) -> ::winnt::LCID; /* winnls.h:1957:1 */
    pub fn ConvertToAutoInheritPrivateObjectSecurity(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: *mut ::guiddef::GUID, _: ::winnt::BOOLEAN, _: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:513:1 */
    pub fn CopySid(_: ::minwindef::DWORD, _: ::winnt::PSID, _: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:526:1 */
    pub fn CreateBoundaryDescriptorW(_: ::winnt::LPCWSTR, _: ::minwindef::ULONG) -> ::winnt::HANDLE; /* namespaceapi.h:73:1 */
    pub fn CreateConsoleScreenBuffer(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *const ::minwinbase::SECURITY_ATTRIBUTES, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::winnt::HANDLE; /* wincon.h:826:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateDirectoryExW as CreateDirectoryEx; /* winbase.h:4665:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
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
    pub fn CreateFileMappingW(_: ::winnt::HANDLE, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* memoryapi.h:243:1 */
    pub fn CreateFileW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::winnt::HANDLE; /* fileapi.h:135:1 */
    pub fn CreateIoCompletionPort(_: ::winnt::HANDLE, _: ::winnt::HANDLE, _: ::basetsd::ULONG_PTR, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* ioapiset.h:64:1 */
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
    pub fn CreateNamedPipeW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::winnt::HANDLE; /* namedpipeapi.h:116:1 */
    pub fn CreatePipe(_: ::winnt::PHANDLE, _: ::winnt::PHANDLE, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:50:1 */
    pub fn CreatePrivateNamespaceW(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::LPVOID, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* namespaceapi.h:45:1 */
    pub fn CreatePrivateObjectSecurity(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: ::minwindef::BOOL, _: ::winnt::HANDLE, _: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:536:1 */
    pub fn CreatePrivateObjectSecurityEx(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: *mut ::guiddef::GUID, _: ::minwindef::BOOL, _: ::minwindef::ULONG, _: ::winnt::HANDLE, _: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:549:1 */
    pub fn CreatePrivateObjectSecurityWithMultipleInheritance(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: *mut *mut ::guiddef::GUID, _: ::minwindef::ULONG, _: ::minwindef::BOOL, _: ::minwindef::ULONG, _: ::winnt::HANDLE, _: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:564:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateProcessW as CreateProcess; /* processthreadsapi.h:532:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateProcessA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::BOOL, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::winnt::LPCSTR, _: ::processthreadsapi::LPSTARTUPINFOA, _: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* processthreadsapi.h:502:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateProcessAsUserW as CreateProcessAsUser; /* processthreadsapi.h:597:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateProcessAsUserW(_: ::winnt::HANDLE, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::BOOL, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::winnt::LPCWSTR, _: ::processthreadsapi::LPSTARTUPINFOW, _: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* processthreadsapi.h:581:1 */
    pub fn CreateProcessW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::BOOL, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::winnt::LPCWSTR, _: ::processthreadsapi::LPSTARTUPINFOW, _: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* processthreadsapi.h:518:1 */
    pub fn CreateRemoteThread(_: ::winnt::HANDLE, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::basetsd::SIZE_T, _: ::minwinbase::LPTHREAD_START_ROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:228:1 */
    pub fn CreateRemoteThreadEx(_: ::winnt::HANDLE, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::basetsd::SIZE_T, _: ::minwinbase::LPTHREAD_START_ROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::processthreadsapi::LPPROC_THREAD_ATTRIBUTE_LIST, _: ::minwindef::LPDWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:815:1 */
    pub fn CreateRestrictedToken(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PSID_AND_ATTRIBUTES, _: ::minwindef::DWORD, _: ::winnt::PLUID_AND_ATTRIBUTES, _: ::minwindef::DWORD, _: ::winnt::PSID_AND_ATTRIBUTES, _: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:580:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateServiceW as CreateService; /* winsvc.h:1117:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateServiceA(_: ::winsvc::SC_HANDLE, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::LPDWORD, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::winsvc::SC_HANDLE; /* winsvc.h:1082:1 */
    pub fn CreateServiceW(_: ::winsvc::SC_HANDLE, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::LPDWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::winsvc::SC_HANDLE; /* winsvc.h:1101:1 */
    pub fn DebugActiveProcess(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:136:1 */
    pub fn DebugActiveProcessStop(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:144:1 */
    pub fn DebugBreak(); /* debugapi.h:70:1 */
    pub fn DecodeSystemPointer(_: ::winnt::PVOID) -> ::winnt::PVOID; /* utilapiset.h:77:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DefineDosDeviceW as DefineDosDevice; /* fileapi.h:162:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DefineDosDeviceW(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* fileapi.h:154:1 */
    pub fn DeleteAce(_: ::winnt::PACL, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:624:1 */
    pub fn DeleteBoundaryDescriptor(_: ::winnt::HANDLE); /* namespaceapi.h:91:1 */
    pub fn DeleteService(_: ::winsvc::SC_HANDLE) -> ::minwindef::BOOL; /* winsvc.h:1125:1 */
    pub fn DeleteSynchronizationBarrier(_: ::synchapi::LPSYNCHRONIZATION_BARRIER) -> ::minwindef::BOOL; /* synchapi.h:893:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DeleteVolumeMountPointW as DeleteVolumeMountPoint; /* fileapi.h:208:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DeleteVolumeMountPointW(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* fileapi.h:202:1 */
    pub fn DestroyPrivateObjectSecurity(_: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:633:1 */
    pub fn DeviceIoControl(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* ioapiset.h:116:1 */
    pub fn DisconnectNamedPipe(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* namedpipeapi.h:70:1 */
    pub fn DnsHostnameToComputerNameExW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:602:1 */
    pub fn DuplicateToken(_: ::winnt::HANDLE, _: ::winnt::SECURITY_IMPERSONATION_LEVEL, _: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:641:1 */
    pub fn DuplicateTokenEx(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::SECURITY_IMPERSONATION_LEVEL, _: ::winnt::TOKEN_TYPE, _: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:651:1 */
    pub fn EncodeSystemPointer(_: ::winnt::PVOID) -> ::winnt::PVOID; /* utilapiset.h:68:1 */
    pub fn EnterSynchronizationBarrier(_: ::synchapi::LPSYNCHRONIZATION_BARRIER, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:876:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumDependentServicesW as EnumDependentServices; /* winsvc.h:1156:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumDependentServicesW(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::winsvc::LPENUM_SERVICE_STATUSW, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1146:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumServicesStatusExW as EnumServicesStatusEx; /* winsvc.h:1232:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumServicesStatusExW(_: ::winsvc::SC_HANDLE, _: ::winsvc::SC_ENUM_TYPE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::LPBYTE, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winsvc.h:1218:1 */
    pub fn EnumSystemFirmwareTables(_: ::minwindef::DWORD, _: ::winnt::PVOID, _: ::minwindef::DWORD) -> ::minwindef::UINT; /* sysinfoapi.h:565:1 */
    pub fn EqualPrefixSid(_: ::winnt::PSID, _: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:664:1 */
    pub fn EqualSid(_: ::winnt::PSID, _: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:673:1 */
    pub fn EscapeCommFunction(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2029:1 */
    pub fn ExitProcess(_: ::minwindef::UINT); /* processthreadsapi.h:169:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ExpandEnvironmentStringsW as ExpandEnvironmentStrings; /* processenv.h:248:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ExpandEnvironmentStringsA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processenv.h:231:1 */
    pub fn ExpandEnvironmentStringsW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processenv.h:241:1 */
    pub fn FileTimeToLocalFileTime(_: *const ::minwindef::FILETIME, _: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* fileapi.h:214:1 */
    pub fn FillConsoleOutputAttribute(_: ::winnt::HANDLE, _: ::minwindef::WORD, _: ::minwindef::DWORD, _: ::wincon::COORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:522:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FillConsoleOutputCharacterW as FillConsoleOutputCharacter; /* wincon.h:514:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FillConsoleOutputCharacterA(_: ::winnt::HANDLE, _: ::winnt::CHAR, _: ::minwindef::DWORD, _: ::wincon::COORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:496:1 */
    pub fn FillConsoleOutputCharacterW(_: ::winnt::HANDLE, _: ::winnt::WCHAR, _: ::minwindef::DWORD, _: ::wincon::COORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:506:1 */
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
    pub fn FindFirstFreeAce(_: ::winnt::PACL, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:682:1 */
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
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindResourceExW as FindResourceEx; /* libloaderapi.h:182:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindResourceExW(_: ::minwindef::HMODULE, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::WORD) -> ::minwindef::HRSRC; /* libloaderapi.h:173:1 */
    pub fn FindVolumeClose(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* fileapi.h:422:1 */
    pub fn FlushConsoleInputBuffer(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* wincon.h:645:1 */
    pub fn FlushInstructionCache(_: ::winnt::HANDLE, _: ::minwindef::LPCVOID, _: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:916:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FoldStringW as FoldString; /* stringapiset.h:108:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
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
    pub fn FreeSid(_: ::winnt::PSID) -> ::winnt::PVOID; /* securitybaseapi.h:691:1 */
    pub fn GenerateConsoleCtrlEvent(_: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wincon.h:719:1 */
    pub fn GetACP() -> ::minwindef::UINT; /* winnls.h:1360:1 */
    pub fn GetAce(_: ::winnt::PACL, _: ::minwindef::DWORD, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:699:1 */
    pub fn GetAclInformation(_: ::winnt::PACL, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::winnt::ACL_INFORMATION_CLASS) -> ::minwindef::BOOL; /* securitybaseapi.h:709:1 */
    pub fn GetCommConfig(_: ::winnt::HANDLE, _: ::winbase::LPCOMMCONFIG, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2038:1 */
    pub fn GetCommMask(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2047:1 */
    pub fn GetCommModemStatus(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2063:1 */
    pub fn GetCommProperties(_: ::winnt::HANDLE, _: ::winbase::LPCOMMPROP) -> ::minwindef::BOOL; /* winbase.h:2055:1 */
    pub fn GetCommState(_: ::winnt::HANDLE, _: ::winbase::LPDCB) -> ::minwindef::BOOL; /* winbase.h:2071:1 */
    pub fn GetCommTimeouts(_: ::winnt::HANDLE, _: ::winbase::LPCOMMTIMEOUTS) -> ::minwindef::BOOL; /* winbase.h:2079:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetComputerNameExW as GetComputerNameEx; /* sysinfoapi.h:377:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetComputerNameExA(_: ::sysinfoapi::COMPUTER_NAME_FORMAT, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:360:1 */
    pub fn GetComputerNameExW(_: ::sysinfoapi::COMPUTER_NAME_FORMAT, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:370:1 */
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
    pub fn GetConsoleTitleW(_: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:750:1 */
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
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileSecurityW as GetFileSecurity; /* securitybaseapi.h:730:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileSecurityW(_: ::winnt::LPCWSTR, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:720:1 */
    pub fn GetFileSize(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* fileapi.h:636:1 */
    pub fn GetFileSizeEx(_: ::winnt::HANDLE, _: ::winnt::PLARGE_INTEGER) -> ::minwindef::BOOL; /* fileapi.h:645:1 */
    pub fn GetFileTime(_: ::winnt::HANDLE, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* fileapi.h:654:1 */
    pub fn GetFileType(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* fileapi.h:665:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileVersionInfoExW as GetFileVersionInfoEx; /* winver.h:157:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileVersionInfoExW(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winver.h:151:15 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileVersionInfoSizeExW as GetFileVersionInfoSizeEx; /* winver.h:141:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileVersionInfoSizeExW(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winver.h:139:16 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFirmwareEnvironmentVariableW as GetFirmwareEnvironmentVariable; /* winbase.h:3554:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFirmwareEnvironmentVariableW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::PVOID, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:3547:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFullPathNameW as GetFullPathName; /* fileapi.h:724:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFullPathNameA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: *mut *mut ::libc::c_schar) -> ::minwindef::DWORD; /* fileapi.h:705:1 */
    pub fn GetFullPathNameW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: *mut *mut ::libc::c_ushort) -> ::minwindef::DWORD; /* fileapi.h:716:1 */
    pub fn GetHandleInformation(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* handleapi.h:79:1 */
    pub fn GetKernelObjectSecurity(_: ::winnt::HANDLE, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:736:1 */
    pub fn GetLargePageMinimum() -> ::basetsd::SIZE_T; /* memoryapi.h:339:1 */
    pub fn GetLargestConsoleWindowSize(_: ::winnt::HANDLE) -> ::wincon::COORD; /* wincon.h:558:1 */
    pub fn GetLengthSid(_: ::winnt::PSID) -> ::minwindef::DWORD; /* securitybaseapi.h:750:1 */
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
    pub fn GetLogicalDriveStringsW(_: ::minwindef::DWORD, _: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* fileapi.h:740:1 */
    pub fn GetLogicalDrives() -> ::minwindef::DWORD; /* fileapi.h:732:1 */
    pub fn GetLogicalProcessorInformation(_: ::winnt::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:455:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetLongPathNameW as GetLongPathName; /* fileapi.h:771:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetLongPathNameA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:754:1 */
    pub fn GetLongPathNameW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:764:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetModuleFileNameW as GetModuleFileName; /* libloaderapi.h:266:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetModuleFileNameA(_: ::minwindef::HMODULE, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* libloaderapi.h:248:1 */
    pub fn GetModuleFileNameW(_: ::minwindef::HMODULE, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* libloaderapi.h:259:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetModuleHandleW as GetModuleHandle; /* libloaderapi.h:290:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetModuleHandleExW as GetModuleHandleEx; /* libloaderapi.h:343:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetModuleHandleExA(_: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: *mut *mut ::minwindef::HINSTANCE__) -> ::minwindef::BOOL; /* libloaderapi.h:327:1 */
    pub fn GetModuleHandleExW(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: *mut *mut ::minwindef::HINSTANCE__) -> ::minwindef::BOOL; /* libloaderapi.h:336:1 */
    pub fn GetModuleHandleW(_: ::winnt::LPCWSTR) -> ::minwindef::HMODULE; /* libloaderapi.h:285:1 */
    pub fn GetNLSVersion(_: ::winnls::NLS_FUNCTION, _: ::winnt::LCID, _: ::winnls::LPNLSVERSIONINFO) -> ::minwindef::BOOL; /* winnls.h:1875:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetNamedPipeClientComputerNameW as GetNamedPipeClientComputerName; /* namedpipeapi.h:161:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetNumaHighestNodeNumber(_: ::minwindef::PULONG) -> ::minwindef::BOOL; /* systemtopologyapi.h:43:1 */
    pub fn GetNumberOfConsoleInputEvents(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* consoleapi.h:77:1 */
    pub fn GetOEMCP() -> ::minwindef::UINT; /* winnls.h:1365:1 */
    pub fn GetOverlappedResult(_: ::winnt::HANDLE, _: ::minwinbase::LPOVERLAPPED, _: ::minwindef::LPDWORD, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* ioapiset.h:52:1 */
    pub fn GetPhysicallyInstalledSystemMemory(_: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* sysinfoapi.h:613:1 */
    pub fn GetPriorityClass(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:669:1 */
    pub fn GetPrivateObjectSecurity(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:759:1 */
    pub fn GetProcessHeaps(_: ::minwindef::DWORD, _: ::winnt::PHANDLE) -> ::minwindef::DWORD; /* heapapi.h:204:1 */
    pub fn GetProcessTimes(_: ::winnt::HANDLE, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* processthreadsapi.h:126:1 */
    pub fn GetProcessVersion(_: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processthreadsapi.h:551:1 */
    pub fn GetProcessWorkingSetSizeEx(_: ::winnt::HANDLE, _: ::basetsd::PSIZE_T, _: ::basetsd::PSIZE_T, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:348:1 */
    pub fn GetQueuedCompletionStatus(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::basetsd::PULONG_PTR, _: *mut *mut ::minwinbase::OVERLAPPED, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* ioapiset.h:75:1 */
    pub fn GetSecurityDescriptorControl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSECURITY_DESCRIPTOR_CONTROL, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:771:1 */
    pub fn GetSecurityDescriptorDacl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::LPBOOL, _: *mut *mut ::winnt::ACL, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:781:1 */
    pub fn GetSecurityDescriptorGroup(_: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:792:1 */
    pub fn GetSecurityDescriptorLength(_: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::DWORD; /* securitybaseapi.h:802:1 */
    pub fn GetSecurityDescriptorOwner(_: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:810:1 */
    pub fn GetSecurityDescriptorRMControl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::PUCHAR) -> ::minwindef::DWORD; /* securitybaseapi.h:820:1 */
    pub fn GetSecurityDescriptorSacl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::LPBOOL, _: *mut *mut ::winnt::ACL, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:829:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetShortPathNameW as GetShortPathName; /* fileapi.h:788:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetShortPathNameW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:780:1 */
    pub fn GetSidIdentifierAuthority(_: ::winnt::PSID) -> ::winnt::PSID_IDENTIFIER_AUTHORITY; /* securitybaseapi.h:840:1 */
    pub fn GetSidLengthRequired(_: ::minwindef::UCHAR) -> ::minwindef::DWORD; /* securitybaseapi.h:848:1 */
    pub fn GetSidSubAuthority(_: ::winnt::PSID, _: ::minwindef::DWORD) -> ::minwindef::PDWORD; /* securitybaseapi.h:856:1 */
    pub fn GetSidSubAuthorityCount(_: ::winnt::PSID) -> ::minwindef::PUCHAR; /* securitybaseapi.h:865:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetStartupInfoW as GetStartupInfo; /* processthreadsapi.h:564:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetStartupInfoW(_: ::processthreadsapi::LPSTARTUPINFOW); /* processthreadsapi.h:559:1 */
    pub fn GetStdHandle(_: ::minwindef::DWORD) -> ::winnt::HANDLE; /* processenv.h:108:1 */
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
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTempFileNameW as GetTempFileName; /* fileapi.h:803:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTempFileNameW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::UINT, _: ::winnt::LPWSTR) -> ::minwindef::UINT; /* fileapi.h:794:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTempPathW as GetTempPath; /* fileapi.h:1213:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTempPathW(_: ::minwindef::DWORD, _: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* fileapi.h:1206:1 */
    pub fn GetThreadContext(_: ::winnt::HANDLE, _: ::minwinbase::LPCONTEXT) -> ::minwindef::BOOL; /* processthreadsapi.h:859:1 */
    pub fn GetThreadLocale() -> ::winnt::LCID; /* winnls.h:1963:1 */
    pub fn GetThreadPriorityBoost(_: ::winnt::HANDLE, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:312:1 */
    pub fn GetThreadTimes(_: ::winnt::HANDLE, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME, _: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* processthreadsapi.h:926:1 */
    pub fn GetTickCount() -> ::minwindef::DWORD; /* sysinfoapi.h:203:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTimeFormatW as GetTimeFormat; /* datetimeapi.h:109:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTimeFormatA(_: ::winnt::LCID, _: ::minwindef::DWORD, _: *const ::minwinbase::SYSTEMTIME, _: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* datetimeapi.h:86:1 */
    pub fn GetTimeFormatW(_: ::winnt::LCID, _: ::minwindef::DWORD, _: *const ::minwinbase::SYSTEMTIME, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* datetimeapi.h:99:1 */
    pub fn GetTokenInformation(_: ::winnt::HANDLE, _: ::winnt::TOKEN_INFORMATION_CLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:873:1 */
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
    pub fn GlobalMemoryStatusEx(_: ::sysinfoapi::LPMEMORYSTATUSEX) -> ::minwindef::BOOL; /* sysinfoapi.h:130:1 */
    pub fn HeapCompact(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::basetsd::SIZE_T; /* heapapi.h:159:1 */
    pub fn HeapCreate(_: ::minwindef::DWORD, _: ::basetsd::SIZE_T, _: ::basetsd::SIZE_T) -> ::winnt::HANDLE; /* heapapi.h:70:1 */
    pub fn HeapDestroy(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* heapapi.h:80:1 */
    pub fn HeapLock(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* heapapi.h:213:1 */
    pub fn HeapQueryInformation(_: ::winnt::HANDLE, _: ::winnt::HEAP_INFORMATION_CLASS, _: ::winnt::PVOID, _: ::basetsd::SIZE_T, _: ::basetsd::PSIZE_T) -> ::minwindef::BOOL; /* heapapi.h:249:1 */
    pub fn HeapSetInformation(_: ::winnt::HANDLE, _: ::winnt::HEAP_INFORMATION_CLASS, _: ::winnt::PVOID, _: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* heapapi.h:238:1 */
    pub fn HeapUnlock(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* heapapi.h:221:1 */
    pub fn HeapValidate(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPCVOID) -> ::minwindef::BOOL; /* heapapi.h:149:1 */
    pub fn HeapWalk(_: ::winnt::HANDLE, _: ::minwinbase::LPPROCESS_HEAP_ENTRY) -> ::minwindef::BOOL; /* heapapi.h:229:1 */
    pub fn ImpersonateAnonymousToken(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:901:1 */
    pub fn ImpersonateLoggedOnUser(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:910:1 */
    pub fn ImpersonateNamedPipeClient(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* namedpipeapi.h:175:1 */
    pub fn ImpersonateSelf(_: ::winnt::SECURITY_IMPERSONATION_LEVEL) -> ::minwindef::BOOL; /* securitybaseapi.h:919:1 */
    pub fn InitializeAcl(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:927:1 */
    pub fn InitializeCriticalSection(_: ::minwinbase::LPCRITICAL_SECTION); /* synchapi.h:162:1 */
    pub fn InitializeCriticalSectionAndSpinCount(_: ::minwinbase::LPCRITICAL_SECTION, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:203:1 */
    pub fn InitializeSecurityDescriptor(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:937:1 */
    pub fn InitializeSid(_: ::winnt::PSID, _: ::winnt::PSID_IDENTIFIER_AUTHORITY, _: ::minwindef::BYTE) -> ::minwindef::BOOL; /* securitybaseapi.h:946:1 */
    pub fn InitializeSynchronizationBarrier(_: ::synchapi::LPSYNCHRONIZATION_BARRIER, _: ::winnt::LONG, _: ::winnt::LONG) -> ::minwindef::BOOL; /* synchapi.h:884:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::InitiateShutdownW as InitiateShutdown; /* winreg.h:1406:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn InitiateShutdownW(_: ::winnt::LPWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winreg.h:1398:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::InitiateSystemShutdownExW as InitiateSystemShutdownEx; /* winreg.h:1364:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn InitiateSystemShutdownExW(_: ::winnt::LPWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::BOOL, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winreg.h:1355:1 */
    pub fn InstallELAMCertificateInfo(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* sysinfoapi.h:647:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsCharAlphaW as IsCharAlpha; /* winuser.h:5536:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsCharAlphaNumericW as IsCharAlphaNumeric; /* winuser.h:5552:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn IsCharAlphaNumericW(_: ::winnt::WCHAR) -> ::minwindef::BOOL; /* winuser.h:5549:1 */
    pub fn IsCharAlphaW(_: ::winnt::WCHAR) -> ::minwindef::BOOL; /* winuser.h:5533:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsCharLowerW as IsCharLower; /* winuser.h:5584:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn IsCharLowerW(_: ::winnt::WCHAR) -> ::minwindef::BOOL; /* winuser.h:5581:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsCharUpperW as IsCharUpper; /* winuser.h:5568:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn IsCharUpperW(_: ::winnt::WCHAR) -> ::minwindef::BOOL; /* winuser.h:5565:1 */
    pub fn IsDBCSLeadByte(_: ::minwindef::BYTE) -> ::minwindef::BOOL; /* winnls.h:1630:1 */
    pub fn IsDBCSLeadByteEx(_: ::minwindef::UINT, _: ::minwindef::BYTE) -> ::minwindef::BOOL; /* winnls.h:1637:1 */
    pub fn IsNLSDefinedString(_: ::winnls::NLS_FUNCTION, _: ::minwindef::DWORD, _: ::winnls::LPNLSVERSIONINFO, _: ::winnt::LPCWSTR, _: ::winnt::INT) -> ::minwindef::BOOL; /* winnls.h:1883:1 */
    pub fn IsTokenRestricted(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:956:1 */
    pub fn IsValidAcl(_: ::winnt::PACL) -> ::minwindef::BOOL; /* securitybaseapi.h:964:1 */
    pub fn IsValidLocale(_: ::winnt::LCID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1894:1 */
    pub fn IsValidSecurityDescriptor(_: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* securitybaseapi.h:972:1 */
    pub fn IsValidSid(_: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:980:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LCMapStringW as LCMapString; /* winnls.h:1483:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LCMapStringA(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::libc::c_int, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1489:1 */
    pub fn LCMapStringW(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1475:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadLibraryExW as LoadLibraryEx; /* libloaderapi.h:394:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadLibraryExA(_: ::winnt::LPCSTR, _: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::HMODULE; /* libloaderapi.h:377:1 */
    pub fn LoadLibraryExW(_: ::winnt::LPCWSTR, _: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::HMODULE; /* libloaderapi.h:387:1 */
    pub fn LoadResource(_: ::minwindef::HMODULE, _: ::minwindef::HRSRC) -> ::minwindef::HGLOBAL; /* libloaderapi.h:417:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadStringW as LoadString; /* libloaderapi.h:453:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadStringA(_: ::minwindef::HINSTANCE, _: ::minwindef::UINT, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* libloaderapi.h:435:1 */
    pub fn LoadStringW(_: ::minwindef::HINSTANCE, _: ::minwindef::UINT, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* libloaderapi.h:445:1 */
    pub fn LocalFileTimeToFileTime(_: *const ::minwindef::FILETIME, _: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* fileapi.h:862:1 */
    pub fn LockFile(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:871:1 */
    pub fn LockResource(_: ::minwindef::HGLOBAL) -> ::minwindef::LPVOID; /* libloaderapi.h:470:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupAccountNameW as LookupAccountName; /* winbase.h:6512:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupAccountNameW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::PSID, _: ::minwindef::LPDWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6502:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupAccountSidW as LookupAccountSid; /* winbase.h:6482:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupAccountSidW(_: ::winnt::LPCWSTR, _: ::winnt::PSID, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6472:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupPrivilegeDisplayNameW as LookupPrivilegeDisplayName; /* winbase.h:6666:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupPrivilegeDisplayNameW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6658:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupPrivilegeNameW as LookupPrivilegeName; /* winbase.h:6640:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupPrivilegeNameW(_: ::winnt::LPCWSTR, _: ::winnt::PLUID, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6633:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupPrivilegeValueW as LookupPrivilegeValue; /* winbase.h:6616:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupPrivilegeValueW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::PLUID) -> ::minwindef::BOOL; /* winbase.h:6610:1 */
    pub fn MakeAbsoluteSD(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::LPDWORD, _: ::winnt::PACL, _: ::minwindef::LPDWORD, _: ::winnt::PACL, _: ::minwindef::LPDWORD, _: ::winnt::PSID, _: ::minwindef::LPDWORD, _: ::winnt::PSID, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:1003:1 */
    pub fn MakeSelfRelativeSD(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:1022:1 */
    pub fn MapGenericMask(_: ::minwindef::PDWORD, _: ::winnt::PGENERIC_MAPPING); /* securitybaseapi.h:1032:1 */
    pub fn MapViewOfFile(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::basetsd::SIZE_T) -> ::minwindef::LPVOID; /* memoryapi.h:276:1 */
    pub fn MapViewOfFileEx(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::basetsd::SIZE_T, _: ::minwindef::LPVOID) -> ::minwindef::LPVOID; /* memoryapi.h:289:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectCloseAuditAlarmW as ObjectCloseAuditAlarm; /* securitybaseapi.h:1049:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectCloseAuditAlarmW(_: ::winnt::LPCWSTR, _: ::minwindef::LPVOID, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1041:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectDeleteAuditAlarmW as ObjectDeleteAuditAlarm; /* securitybaseapi.h:1063:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectDeleteAuditAlarmW(_: ::winnt::LPCWSTR, _: ::minwindef::LPVOID, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1055:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectOpenAuditAlarmW as ObjectOpenAuditAlarm; /* securitybaseapi.h:1086:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectOpenAuditAlarmW(_: ::winnt::LPCWSTR, _: ::minwindef::LPVOID, _: ::winnt::LPWSTR, _: ::winnt::LPWSTR, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PPRIVILEGE_SET, _: ::minwindef::BOOL, _: ::minwindef::BOOL, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1069:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectPrivilegeAuditAlarmW as ObjectPrivilegeAuditAlarm; /* securitybaseapi.h:1103:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectPrivilegeAuditAlarmW(_: ::winnt::LPCWSTR, _: ::minwindef::LPVOID, _: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winnt::PPRIVILEGE_SET, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1092:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenFileMappingW as OpenFileMapping; /* memoryapi.h:269:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenFileMappingW(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* memoryapi.h:261:1 */
    pub fn OpenPrivateNamespaceW(_: ::minwindef::LPVOID, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* namespaceapi.h:55:1 */
    pub fn OpenProcess(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:938:1 */
    pub fn OpenProcessToken(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* processthreadsapi.h:622:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenSCManagerW as OpenSCManager; /* winsvc.h:1326:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenSCManagerA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::winsvc::SC_HANDLE; /* winsvc.h:1311:1 */
    pub fn OpenSCManagerW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::winsvc::SC_HANDLE; /* winsvc.h:1320:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenServiceW as OpenService; /* winsvc.h:1350:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenServiceA(_: ::winsvc::SC_HANDLE, _: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::winsvc::SC_HANDLE; /* winsvc.h:1335:1 */
    pub fn OpenServiceW(_: ::winsvc::SC_HANDLE, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::winsvc::SC_HANDLE; /* winsvc.h:1344:1 */
    pub fn OpenThread(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:273:1 */
    pub fn OpenThreadToken(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* processthreadsapi.h:632:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PeekConsoleInputW as PeekConsoleInput; /* wincon.h:340:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PeekConsoleInputA(_: ::winnt::HANDLE, _: ::wincon::PINPUT_RECORD, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* consoleapi.h:86:1 */
    pub fn PeekConsoleInputW(_: ::winnt::HANDLE, _: ::wincon::PINPUT_RECORD, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:333:1 */
    pub fn PeekNamedPipe(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:89:1 */
    pub fn PostQueuedCompletionStatus(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::basetsd::ULONG_PTR, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* ioapiset.h:105:1 */
    pub fn PrivilegeCheck(_: ::winnt::HANDLE, _: ::winnt::PPRIVILEGE_SET, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1109:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PrivilegedServiceAuditAlarmW as PrivilegedServiceAuditAlarm; /* securitybaseapi.h:1129:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PrivilegedServiceAuditAlarmW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::HANDLE, _: ::winnt::PPRIVILEGE_SET, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1119:1 */
    pub fn ProcessIdToSessionId(_: ::minwindef::DWORD, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* processthreadsapi.h:677:1 */
    pub fn PurgeComm(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2087:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::QueryDosDeviceW as QueryDosDevice; /* fileapi.h:918:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn QueryDosDeviceW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:910:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::QueryServiceConfigW as QueryServiceConfig; /* winsvc.h:1378:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::QueryServiceConfig2W as QueryServiceConfig2; /* winsvc.h:1422:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn QueryServiceConfig2A(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPBYTE, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1394:1 */
    pub fn QueryServiceConfig2W(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPBYTE, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1413:1 */
    pub fn QueryServiceConfigA(_: ::winsvc::SC_HANDLE, _: ::winsvc::LPQUERY_SERVICE_CONFIGA, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1359:1 */
    pub fn QueryServiceConfigW(_: ::winsvc::SC_HANDLE, _: ::winsvc::LPQUERY_SERVICE_CONFIGW, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1370:1 */
    pub fn QueryServiceObjectSecurity(_: ::winsvc::SC_HANDLE, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1459:1 */
    pub fn QueryServiceStatus(_: ::winsvc::SC_HANDLE, _: ::winsvc::LPSERVICE_STATUS) -> ::minwindef::BOOL; /* winsvc.h:1472:1 */
    pub fn QueryServiceStatusEx(_: ::winsvc::SC_HANDLE, _: ::winsvc::SC_STATUS_TYPE, _: ::minwindef::LPBYTE, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1481:1 */
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
    pub fn RegCloseKey(_: ::minwindef::HKEY) -> ::winreg::LSTATUS; /* winreg.h:206:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegCreateKeyExW as RegCreateKeyEx; /* winreg.h:353:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegCreateKeyExA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::winreg::REGSAM, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::PHKEY, _: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:325:1 */
    pub fn RegCreateKeyExW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::winreg::REGSAM, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::PHKEY, _: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:340:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegDeleteKeyExW as RegDeleteKeyEx; /* winreg.h:437:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegDeleteKeyExA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::winreg::REGSAM, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:419:1 */
    pub fn RegDeleteKeyExW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::winreg::REGSAM, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:429:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegDeleteValueW as RegDeleteValue; /* winreg.h:509:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegDeleteValueA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:495:1 */
    pub fn RegDeleteValueW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:503:1 */
    pub fn RegDisablePredefinedCacheEx() -> ::winreg::LSTATUS; /* winreg.h:249:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegEnumKeyExW as RegEnumKeyEx; /* winreg.h:567:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegEnumKeyExA(_: ::minwindef::HKEY, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:541:1 */
    pub fn RegEnumKeyExW(_: ::minwindef::HKEY, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:555:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegEnumValueW as RegEnumValue; /* winreg.h:601:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegEnumValueA(_: ::minwindef::HKEY, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPBYTE, _: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:575:1 */
    pub fn RegEnumValueW(_: ::minwindef::HKEY, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPBYTE, _: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:589:1 */
    pub fn RegFlushKey(_: ::minwindef::HKEY) -> ::winreg::LSTATUS; /* winreg.h:609:1 */
    pub fn RegGetKeySecurity(_: ::minwindef::HKEY, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:617:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegLoadKeyW as RegLoadKey; /* winreg.h:644:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegLoadKeyA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:628:1 */
    pub fn RegLoadKeyW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:637:1 */
    pub fn RegNotifyChangeKeyValue(_: ::minwindef::HKEY, _: ::minwindef::BOOL, _: ::minwindef::DWORD, _: ::winnt::HANDLE, _: ::minwindef::BOOL) -> ::winreg::LSTATUS; /* winreg.h:652:1 */
    pub fn RegOpenCurrentUser(_: ::winreg::REGSAM, _: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:233:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegOpenKeyExW as RegOpenKeyEx; /* winreg.h:706:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegOpenKeyExA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winreg::REGSAM, _: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:686:1 */
    pub fn RegOpenKeyExW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winreg::REGSAM, _: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:697:1 */
    pub fn RegOpenUserClassesRoot(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winreg::REGSAM, _: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:222:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegQueryInfoKeyW as RegQueryInfoKey; /* winreg.h:778:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegQueryInfoKeyA(_: ::minwindef::HKEY, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:744:1 */
    pub fn RegQueryInfoKeyW(_: ::minwindef::HKEY, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:762:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegQueryValueExW as RegQueryValueEx; /* winreg.h:864:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegQueryValueExA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPBYTE, _: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:840:1 */
    pub fn RegQueryValueExW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPBYTE, _: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:853:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegRestoreKeyW as RegRestoreKey; /* winreg.h:912:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegRestoreKeyA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:896:1 */
    pub fn RegRestoreKeyW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:905:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegSaveKeyExW as RegSaveKeyEx; /* winreg.h:1440:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegSaveKeyExA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1422:1 */
    pub fn RegSaveKeyExW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1432:1 */
    pub fn RegSetKeySecurity(_: ::minwindef::HKEY, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR) -> ::winreg::LSTATUS; /* winreg.h:956:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegSetValueExW as RegSetValueEx; /* winreg.h:1014:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegSetValueExA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *const ::libc::c_uchar, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:992:1 */
    pub fn RegSetValueExW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *const ::libc::c_uchar, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1004:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegUnLoadKeyW as RegUnLoadKey; /* winreg.h:1036:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegUnLoadKeyA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:1022:1 */
    pub fn RegUnLoadKeyW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:1030:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegisterServiceCtrlHandlerW as RegisterServiceCtrlHandler; /* winsvc.h:1509:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegisterServiceCtrlHandlerA(_: ::winnt::LPCSTR, _: ::winsvc::LPHANDLER_FUNCTION) -> ::winsvc::SERVICE_STATUS_HANDLE; /* winsvc.h:1494:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegisterServiceCtrlHandlerExW as RegisterServiceCtrlHandlerEx; /* winsvc.h:1535:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegisterServiceCtrlHandlerExA(_: ::winnt::LPCSTR, _: ::winsvc::LPHANDLER_FUNCTION_EX, _: ::minwindef::LPVOID) -> ::winsvc::SERVICE_STATUS_HANDLE; /* winsvc.h:1518:1 */
    pub fn RegisterServiceCtrlHandlerExW(_: ::winnt::LPCWSTR, _: ::winsvc::LPHANDLER_FUNCTION_EX, _: ::minwindef::LPVOID) -> ::winsvc::SERVICE_STATUS_HANDLE; /* winsvc.h:1528:1 */
    pub fn RegisterServiceCtrlHandlerW(_: ::winnt::LPCWSTR, _: ::winsvc::LPHANDLER_FUNCTION) -> ::winsvc::SERVICE_STATUS_HANDLE; /* winsvc.h:1503:1 */
    pub fn RemoveDllDirectory(_: ::libloaderapi::DLL_DIRECTORY_COOKIE) -> ::minwindef::BOOL; /* libloaderapi.h:506:1 */
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
    pub fn SetAclInformation(_: ::winnt::PACL, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::winnt::ACL_INFORMATION_CLASS) -> ::minwindef::BOOL; /* securitybaseapi.h:1157:1 */
    pub fn SetCommBreak(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:2095:1 */
    pub fn SetCommConfig(_: ::winnt::HANDLE, _: ::winbase::LPCOMMCONFIG, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2102:1 */
    pub fn SetCommMask(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2111:1 */
    pub fn SetCommState(_: ::winnt::HANDLE, _: ::winbase::LPDCB) -> ::minwindef::BOOL; /* winbase.h:2119:1 */
    pub fn SetCommTimeouts(_: ::winnt::HANDLE, _: ::winbase::LPCOMMTIMEOUTS) -> ::minwindef::BOOL; /* winbase.h:2127:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetComputerNameExW as SetComputerNameEx; /* sysinfoapi.h:405:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetComputerNameEx2W as SetComputerNameEx2; /* sysinfoapi.h:631:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetComputerNameEx2W(_: ::sysinfoapi::COMPUTER_NAME_FORMAT, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* sysinfoapi.h:623:1 */
    pub fn SetComputerNameExW(_: ::sysinfoapi::COMPUTER_NAME_FORMAT, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* sysinfoapi.h:398:1 */
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
    pub fn SetConsoleTitleW(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* wincon.h:789:1 */
    pub fn SetConsoleWindowInfo(_: ::winnt::HANDLE, _: ::minwindef::BOOL, _: *const ::wincon::SMALL_RECT) -> ::minwindef::BOOL; /* wincon.h:702:1 */
    pub fn SetCriticalSectionSpinCount(_: ::minwinbase::LPCRITICAL_SECTION, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* synchapi.h:241:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetCurrentDirectoryW as SetCurrentDirectory; /* processenv.h:268:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetCurrentDirectoryA(_: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* processenv.h:256:1 */
    pub fn SetCurrentDirectoryW(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* processenv.h:263:1 */
    pub fn SetDefaultDllDirectories(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* libloaderapi.h:514:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetEnvironmentStringsW as SetEnvironmentStrings; /* processenv.h:82:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetEnvironmentStringsW(_: ::winnt::LPWCH) -> ::minwindef::BOOL; /* processenv.h:77:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetEnvironmentVariableW as SetEnvironmentVariable; /* processenv.h:222:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetEnvironmentVariableA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* processenv.h:208:1 */
    pub fn SetEnvironmentVariableW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* processenv.h:216:1 */
    pub fn SetErrorMode(_: ::minwindef::UINT) -> ::minwindef::UINT; /* errhandlingapi.h:156:1 */
    pub fn SetFilePointer(_: ::winnt::HANDLE, _: ::winnt::LONG, _: ::winnt::PLONG, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:1057:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetFileSecurityW as SetFileSecurity; /* securitybaseapi.h:1175:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetFileSecurityW(_: ::winnt::LPCWSTR, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* securitybaseapi.h:1168:1 */
    pub fn SetFileTime(_: ::winnt::HANDLE, _: *const ::minwindef::FILETIME, _: *const ::minwindef::FILETIME, _: *const ::minwindef::FILETIME) -> ::minwindef::BOOL; /* fileapi.h:1093:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetFirmwareEnvironmentVariableW as SetFirmwareEnvironmentVariable; /* winbase.h:3608:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetFirmwareEnvironmentVariableW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::PVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3601:1 */
    pub fn SetHandleInformation(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* handleapi.h:88:1 */
    pub fn SetKernelObjectSecurity(_: ::winnt::HANDLE, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* securitybaseapi.h:1181:1 */
    pub fn SetLocalTime(_: *const ::minwinbase::SYSTEMTIME) -> ::minwindef::BOOL; /* sysinfoapi.h:176:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetLocaleInfoW as SetLocaleInfo; /* winnls.h:1544:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetLocaleInfoW(_: ::winnt::LCID, _: ::winnls::LCTYPE, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winnls.h:1539:1 */
    pub fn SetNamedPipeHandleState(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:78:1 */
    pub fn SetPriorityClass(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:652:1 */
    pub fn SetPrivateObjectSecurity(_: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: ::winnt::PGENERIC_MAPPING, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:1191:1 */
    pub fn SetPrivateObjectSecurityEx(_: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: ::minwindef::ULONG, _: ::winnt::PGENERIC_MAPPING, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:1203:1 */
    pub fn SetProcessShutdownParameters(_: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:542:1 */
    pub fn SetProcessWorkingSetSizeEx(_: ::winnt::HANDLE, _: ::basetsd::SIZE_T, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:359:1 */
    pub fn SetSecurityDescriptorControl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::SECURITY_DESCRIPTOR_CONTROL, _: ::winnt::SECURITY_DESCRIPTOR_CONTROL) -> ::minwindef::BOOL; /* securitybaseapi.h:1230:1 */
    pub fn SetSecurityDescriptorDacl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::BOOL, _: ::winnt::PACL, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1240:1 */
    pub fn SetSecurityDescriptorGroup(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSID, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1251:1 */
    pub fn SetSecurityDescriptorOwner(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSID, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1261:1 */
    pub fn SetSecurityDescriptorRMControl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::PUCHAR) -> ::minwindef::DWORD; /* securitybaseapi.h:1271:1 */
    pub fn SetSecurityDescriptorSacl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::BOOL, _: ::winnt::PACL, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1280:1 */
    pub fn SetServiceObjectSecurity(_: ::winsvc::SC_HANDLE, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* winsvc.h:1543:1 */
    pub fn SetServiceStatus(_: ::winsvc::SERVICE_STATUS_HANDLE, _: ::winsvc::LPSERVICE_STATUS) -> ::minwindef::BOOL; /* winsvc.h:1552:1 */
    pub fn SetStdHandle(_: ::minwindef::DWORD, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* processenv.h:116:1 */
    pub fn SetSystemTime(_: *const ::minwinbase::SYSTEMTIME) -> ::minwindef::BOOL; /* sysinfoapi.h:411:1 */
    pub fn SetSystemTimeAdjustment(_: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* sysinfoapi.h:638:1 */
    pub fn SetThreadContext(_: ::winnt::HANDLE, _: *const ::winnt::CONTEXT) -> ::minwindef::BOOL; /* processthreadsapi.h:868:1 */
    pub fn SetThreadLocale(_: ::winnt::LCID) -> ::minwindef::BOOL; /* winnls.h:1968:1 */
    pub fn SetThreadPriorityBoost(_: ::winnt::HANDLE, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:303:1 */
    pub fn SetThreadStackGuarantee(_: ::minwindef::PULONG) -> ::minwindef::BOOL; /* processthreadsapi.h:661:1 */
    pub fn SetThreadToken(_: ::winnt::PHANDLE, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* processthreadsapi.h:613:1 */
    pub fn SetThreadUILanguage(_: ::winnt::LANGID) -> ::winnt::LANGID; /* winnls.h:2008:1 */
    pub fn SetTimeZoneInformation(_: *const ::timezoneapi::TIME_ZONE_INFORMATION) -> ::minwindef::BOOL; /* timezoneapi.h:130:1 */
    pub fn SetTokenInformation(_: ::winnt::HANDLE, _: ::winnt::TOKEN_INFORMATION_CLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:1291:1 */
    pub fn SetUnhandledExceptionFilter(_: ::errhandlingapi::LPTOP_LEVEL_EXCEPTION_FILTER) -> ::errhandlingapi::LPTOP_LEVEL_EXCEPTION_FILTER; /* errhandlingapi.h:100:1 */
    pub fn SetUserGeoID(_: ::winnls::GEOID) -> ::minwindef::BOOL; /* winnls.h:1951:1 */
    pub fn SetupComm(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2020:1 */
    pub fn SizeofResource(_: ::minwindef::HMODULE, _: ::minwindef::HRSRC) -> ::minwindef::DWORD; /* libloaderapi.h:478:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::StartServiceW as StartService; /* winsvc.h:1595:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn StartServiceA(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: *mut *const ::libc::c_schar) -> ::minwindef::BOOL; /* winsvc.h:1579:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::StartServiceCtrlDispatcherW as StartServiceCtrlDispatcher; /* winsvc.h:1570:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn StartServiceCtrlDispatcherA(_: *const ::winsvc::SERVICE_TABLE_ENTRYA) -> ::minwindef::BOOL; /* winsvc.h:1560:1 */
    pub fn StartServiceCtrlDispatcherW(_: *const ::winsvc::SERVICE_TABLE_ENTRYW) -> ::minwindef::BOOL; /* winsvc.h:1566:1 */
    pub fn StartServiceW(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: *mut *const ::libc::c_ushort) -> ::minwindef::BOOL; /* winsvc.h:1588:1 */
    pub fn SwitchToThread() -> ::minwindef::BOOL; /* processthreadsapi.h:195:1 */
    pub fn TerminateProcess(_: ::winnt::HANDLE, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* processthreadsapi.h:177:1 */
    pub fn TerminateThread(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:348:1 */
    pub fn TlsAlloc() -> ::minwindef::DWORD; /* processthreadsapi.h:460:1 */
    pub fn TlsFree(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:485:1 */
    pub fn TlsSetValue(_: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* processthreadsapi.h:476:1 */
    pub fn TransactNamedPipe(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* namedpipeapi.h:102:1 */
    pub fn TransmitCommChar(_: ::winnt::HANDLE, _: ::libc::c_schar) -> ::minwindef::BOOL; /* winbase.h:2135:1 */
    pub fn UnlockFile(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1118:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VerFindFileW as VerFindFile; /* winver.h:59:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VerFindFileW(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::PUINT, _: ::winnt::LPWSTR, _: ::minwindef::PUINT) -> ::minwindef::DWORD; /* winver.h:48:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VerLanguageNameW as VerLanguageName; /* winver.h:178:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VerLanguageNameA(_: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winver.h:165:1 */
    pub fn VerLanguageNameW(_: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winver.h:172:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VerQueryValueW as VerQueryValue; /* winver.h:200:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VerQueryValueW(_: ::minwindef::LPCVOID, _: ::winnt::LPCWSTR, _: *mut *mut ::libc::c_void, _: ::minwindef::PUINT) -> ::minwindef::BOOL; /* winver.h:193:1 */
    pub fn VirtualAlloc(_: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* memoryapi.h:90:1 */
    pub fn VirtualAllocEx(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* memoryapi.h:157:1 */
    pub fn VirtualFree(_: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:110:1 */
    pub fn VirtualFreeEx(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:181:1 */
    pub fn VirtualLock(_: ::minwindef::LPVOID, _: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* memoryapi.h:370:1 */
    pub fn VirtualProtect(_: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:121:1 */
    pub fn VirtualProtectEx(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:193:1 */
    pub fn VirtualQueryEx(_: ::winnt::HANDLE, _: ::minwindef::LPCVOID, _: ::winnt::PMEMORY_BASIC_INFORMATION, _: ::basetsd::SIZE_T) -> ::basetsd::SIZE_T; /* memoryapi.h:205:1 */
    pub fn VirtualUnlock(_: ::minwindef::LPVOID, _: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* memoryapi.h:379:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetAddConnectionW as WNetAddConnection; /* winnetwk.h:164:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetAddConnection2W as WNetAddConnection2; /* winnetwk.h:186:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetAddConnection2A(_: ::winnetwk::LPNETRESOURCEA, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:171:1 */
    pub fn WNetAddConnection2W(_: ::winnetwk::LPNETRESOURCEW, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:179:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetAddConnection3W as WNetAddConnection3; /* winnetwk.h:210:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetAddConnection3A(_: ::windef::HWND, _: ::winnetwk::LPNETRESOURCEA, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:193:1 */
    pub fn WNetAddConnection3W(_: ::windef::HWND, _: ::winnetwk::LPNETRESOURCEW, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:202:1 */
    pub fn WNetAddConnectionA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::DWORD; /* winnetwk.h:151:1 */
    pub fn WNetAddConnectionW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::DWORD; /* winnetwk.h:158:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetCancelConnectionW as WNetCancelConnection; /* winnetwk.h:228:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetCancelConnection2W as WNetCancelConnection2; /* winnetwk.h:248:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetCancelConnection2A(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:235:1 */
    pub fn WNetCancelConnection2W(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:242:1 */
    pub fn WNetCancelConnectionA(_: ::winnt::LPCSTR, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:217:1 */
    pub fn WNetCancelConnectionW(_: ::winnt::LPCWSTR, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:223:1 */
    pub fn WNetCloseEnum(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winnetwk.h:490:1 */
    pub fn WNetConnectionDialog(_: ::windef::HWND, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:332:1 */
    pub fn WNetDisconnectDialog(_: ::windef::HWND, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:339:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetEnumResourceW as WNetEnumResource; /* winnetwk.h:483:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetEnumResourceA(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:468:1 */
    pub fn WNetEnumResourceW(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:476:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetConnectionW as WNetGetConnection; /* winnetwk.h:268:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetConnectionA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:255:1 */
    pub fn WNetGetConnectionW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:262:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetLastErrorW as WNetGetLastError; /* winnetwk.h:718:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetLastErrorA(_: ::minwindef::LPDWORD, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:701:1 */
    pub fn WNetGetLastErrorW(_: ::minwindef::LPDWORD, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:710:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetUniversalNameW as WNetGetUniversalName; /* winnetwk.h:594:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetUniversalNameA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:579:1 */
    pub fn WNetGetUniversalNameW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:587:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetUserW as WNetGetUser; /* winnetwk.h:620:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetUserA(_: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:604:1 */
    pub fn WNetGetUserW(_: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:614:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetOpenEnumW as WNetOpenEnum; /* winnetwk.h:461:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetOpenEnumA(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnetwk::LPNETRESOURCEA, _: ::minwindef::LPHANDLE) -> ::minwindef::DWORD; /* winnetwk.h:444:1 */
    pub fn WNetOpenEnumW(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnetwk::LPNETRESOURCEW, _: ::minwindef::LPHANDLE) -> ::minwindef::DWORD; /* winnetwk.h:453:1 */
    pub fn WaitCommEvent(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* winbase.h:2143:1 */
    pub fn WaitForDebugEvent(_: ::minwinbase::LPDEBUG_EVENT, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:127:1 */
    pub fn WaitForSingleObject(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* synchapi.h:473:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WaitNamedPipeW as WaitNamedPipe; /* namedpipeapi.h:142:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WaitNamedPipeW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:135:1 */
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
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn RaiseException(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *const ::libc::c_ulonglong); /* errhandlingapi.h:73:1 */
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
    pub fn GetGeoInfoW(_: ::winnls::GEOID, _: ::winnls::GEOTYPE, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::winnt::LANGID) -> ::libc::c_int; /* winnls.h:1916:1 */
    pub fn GetLastError() -> ::minwindef::DWORD; /* errhandlingapi.h:118:1 */
    pub fn GetLocalTime(_: ::minwinbase::LPSYSTEMTIME); /* sysinfoapi.h:161:1 */
    pub fn GetOverlappedResultEx(_: ::winnt::HANDLE, _: ::minwinbase::LPOVERLAPPED, _: ::minwindef::LPDWORD, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* ioapiset.h:159:1 */
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
    pub fn MoveFileExW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5373:1 */
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
    pub fn WaitOnAddress(_: *mut ::libc::c_void, _: ::winnt::PVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:915:1 */
    pub fn WakeByAddressAll(_: ::winnt::PVOID); /* synchapi.h:932:1 */
    pub fn WakeByAddressSingle(_: ::winnt::PVOID); /* synchapi.h:925:1 */
    pub fn WideCharToMultiByte(_: ::minwindef::UINT, _: ::minwindef::DWORD, _: ::winnt::LPCWCH, _: ::libc::c_int, _: ::winnt::LPSTR, _: ::libc::c_int, _: ::winnt::LPCCH, _: ::minwindef::LPBOOL) -> ::libc::c_int; /* stringapiset.h:169:1 */
    pub fn WriteFile(_: ::winnt::HANDLE, _: ::minwindef::LPCVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* fileapi.h:1149:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn CancelWaitableTimer(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* synchapi.h:729:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::CopyFileExW as CopyFileEx; /* winbase.h:5128:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn CopyFileExW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winbase::LPPROGRESS_ROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::LPBOOL, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5118:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::MultinetGetConnectionPerformanceW as MultinetGetConnectionPerformance; /* winnetwk.h:818:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn MultinetGetConnectionPerformanceA(_: ::winnetwk::LPNETRESOURCEA, _: ::winnetwk::LPNETCONNECTINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:807:1 */
    pub fn MultinetGetConnectionPerformanceW(_: ::winnetwk::LPNETRESOURCEW, _: ::winnetwk::LPNETCONNECTINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:813:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::OpenWaitableTimerW as OpenWaitableTimer; /* synchapi.h:692:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn OpenWaitableTimerW(_: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* synchapi.h:684:1 */
    pub fn QueueUserAPC(_: ::winnt::PAPCFUNC, _: ::winnt::HANDLE, _: ::basetsd::ULONG_PTR) -> ::minwindef::DWORD; /* processthreadsapi.h:114:1 */
    pub fn ReadDirectoryChangesW(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwinbase::LPOVERLAPPED, _: ::minwinbase::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::minwindef::BOOL; /* winbase.h:6368:1 */
    pub fn SetWaitableTimer(_: ::winnt::HANDLE, _: *const ::winnt::LARGE_INTEGER, _: ::winnt::LONG, _: ::synchapi::PTIMERAPCROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* synchapi.h:716:1 */
    pub fn SignalObjectAndWait(_: ::winnt::HANDLE, _: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winbase.h:2851:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetConnectionDialog1W as WNetConnectionDialog1; /* winnetwk.h:391:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetConnectionDialog1A(_: ::winnetwk::LPCONNECTDLGSTRUCTA) -> ::minwindef::DWORD; /* winnetwk.h:382:1 */
    pub fn WNetConnectionDialog1W(_: ::winnetwk::LPCONNECTDLGSTRUCTW) -> ::minwindef::DWORD; /* winnetwk.h:387:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetDisconnectDialog1W as WNetDisconnectDialog1; /* winnetwk.h:432:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetDisconnectDialog1A(_: ::winnetwk::LPDISCDLGSTRUCTA) -> ::minwindef::DWORD; /* winnetwk.h:423:1 */
    pub fn WNetDisconnectDialog1W(_: ::winnetwk::LPDISCDLGSTRUCTW) -> ::minwindef::DWORD; /* winnetwk.h:428:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetNetworkInformationW as WNetGetNetworkInformation; /* winnetwk.h:688:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetNetworkInformationA(_: ::winnt::LPCSTR, _: ::winnetwk::LPNETINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:677:1 */
    pub fn WNetGetNetworkInformationW(_: ::winnt::LPCWSTR, _: ::winnetwk::LPNETINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:683:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetProviderNameW as WNetGetProviderName; /* winnetwk.h:655:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetProviderNameA(_: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:642:1 */
    pub fn WNetGetProviderNameW(_: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:649:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetResourceInformationW as WNetGetResourceInformation; /* winnetwk.h:532:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetResourceInformationA(_: ::winnetwk::LPNETRESOURCEA, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD, _: *mut *mut ::libc::c_schar) -> ::minwindef::DWORD; /* winnetwk.h:517:1 */
    pub fn WNetGetResourceInformationW(_: ::winnetwk::LPNETRESOURCEW, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD, _: *mut *mut ::libc::c_ushort) -> ::minwindef::DWORD; /* winnetwk.h:525:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetResourceParentW as WNetGetResourceParent; /* winnetwk.h:510:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetResourceParentA(_: ::winnetwk::LPNETRESOURCEA, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:497:1 */
    pub fn WNetGetResourceParentW(_: ::winnetwk::LPNETRESOURCEW, _: ::minwindef::LPVOID, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:504:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetUseConnectionW as WNetUseConnection; /* winnetwk.h:320:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetUseConnectionA(_: ::windef::HWND, _: ::winnetwk::LPNETRESOURCEA, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:297:1 */
    pub fn WNetUseConnectionW(_: ::windef::HWND, _: ::winnetwk::LPNETRESOURCEW, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:309:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::FindFirstFileExW as FindFirstFileEx; /* fileapi.h:334:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn FindFirstFileExA(_: ::winnt::LPCSTR, _: ::minwinbase::FINDEX_INFO_LEVELS, _: ::minwindef::LPVOID, _: ::minwinbase::FINDEX_SEARCH_OPS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* fileapi.h:312:1 */
    pub fn FindFirstFileExW(_: ::winnt::LPCWSTR, _: ::minwinbase::FINDEX_INFO_LEVELS, _: ::minwindef::LPVOID, _: ::minwinbase::FINDEX_SEARCH_OPS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* fileapi.h:324:1 */
    pub fn IsDebuggerPresent() -> ::minwindef::BOOL; /* debugapi.h:54:1 */
    pub fn TryEnterCriticalSection(_: ::minwinbase::LPCRITICAL_SECTION) -> ::minwindef::BOOL; /* synchapi.h:260:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn AttachConsole(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wincon.h:733:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::CreateHardLinkW as CreateHardLink; /* winbase.h:5524:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn CreateHardLinkW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::BOOL; /* winbase.h:5518:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumSystemLocalesW as EnumSystemLocales; /* winnls.h:2195:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumSystemLocalesA(_: ::winnls::LOCALE_ENUMPROCA, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:2185:1 */
    pub fn EnumSystemLocalesW(_: ::winnls::LOCALE_ENUMPROCW, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:2191:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::GetCalendarInfoW as GetCalendarInfo; /* winnls.h:1574:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn GetCalendarInfoW(_: ::winnt::LCID, _: ::winnls::CALID, _: ::winnls::CALTYPE, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::minwindef::LPDWORD) -> ::libc::c_int; /* winnls.h:1566:1 */
    pub fn IsValidLanguageGroup(_: ::winnls::LGRPID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1866:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::MoveFileWithProgressW as MoveFileWithProgress; /* winbase.h:5412:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn MoveFileWithProgressW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winbase::LPPROGRESS_ROUTINE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5404:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::ReplaceFileW as ReplaceFile; /* winbase.h:5495:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn ReplaceFileW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winbase.h:5486:1 */
    pub fn RtlCompareMemory(_: *const ::libc::c_void, _: *const ::libc::c_void, _: ::basetsd::SIZE_T) -> ::basetsd::SIZE_T; /* winnt.h:17442:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::SetCalendarInfoW as SetCalendarInfo; /* winnls.h:1596:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn SetCalendarInfoW(_: ::winnt::LCID, _: ::winnls::CALID, _: ::winnls::CALTYPE, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winnls.h:1590:1 */
    pub fn VerSetConditionMask(_: ::winnt::ULONGLONG, _: ::minwindef::DWORD, _: ::minwindef::BYTE) -> ::winnt::ULONGLONG; /* winnt.h:17986:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn AddVectoredContinueHandler(_: ::minwindef::ULONG, _: ::winnt::PVECTORED_EXCEPTION_HANDLER) -> ::winnt::PVOID; /* errhandlingapi.h:197:1 */
    pub fn AddVectoredExceptionHandler(_: ::minwindef::ULONG, _: ::winnt::PVECTORED_EXCEPTION_HANDLER) -> ::winnt::PVOID; /* errhandlingapi.h:179:1 */
    pub fn AllocateUserPhysicalPages(_: ::winnt::HANDLE, _: ::basetsd::PULONG_PTR, _: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* memoryapi.h:582:1 */
    pub fn CheckRemoteDebuggerPresent(_: ::winnt::HANDLE, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* debugapi.h:155:1 */
    pub fn CreateMemoryResourceNotification(_: ::memoryapi::MEMORY_RESOURCE_NOTIFICATION_TYPE) -> ::winnt::HANDLE; /* memoryapi.h:420:1 */
    pub fn CreateWellKnownSid(_: ::winnt::WELL_KNOWN_SID_TYPE, _: ::winnt::PSID, _: ::winnt::PSID, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* securitybaseapi.h:600:1 */
    pub fn EqualDomainSid(_: ::winnt::PSID, _: ::winnt::PSID, _: *mut ::libc::c_int) -> ::minwindef::BOOL; /* securitybaseapi.h:612:1 */
    pub fn FreeUserPhysicalPages(_: ::winnt::HANDLE, _: ::basetsd::PULONG_PTR, _: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* memoryapi.h:593:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetCompressedFileSizeW as GetCompressedFileSize; /* fileapi.h:1340:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetCompressedFileSizeA(_: ::winnt::LPCSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* fileapi.h:1326:1 */
    pub fn GetCompressedFileSizeW(_: ::winnt::LPCWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* fileapi.h:1334:1 */
    pub fn GetProcessHandleCount(_: ::winnt::HANDLE, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:974:1 */
    pub fn GetProcessId(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:690:1 */
    pub fn GetProcessPriorityBoost(_: ::winnt::HANDLE, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:1067:1 */
    pub fn GetSystemTimes(_: ::minwindef::PFILETIME, _: ::minwindef::PFILETIME, _: ::minwindef::PFILETIME) -> ::minwindef::BOOL; /* processthreadsapi.h:1094:1 */
    pub fn GetThreadIOPendingFlag(_: ::winnt::HANDLE, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:1085:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetVolumePathNamesForVolumeNameW as GetVolumePathNamesForVolumeName; /* fileapi.h:1245:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetVolumePathNamesForVolumeNameW(_: ::winnt::LPCWSTR, _: ::winnt::LPWCH, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* fileapi.h:1236:1 */
    pub fn GetWindowsAccountDomainSid(_: ::winnt::PSID, _: ::winnt::PSID, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* securitybaseapi.h:889:1 */
    pub fn IsProcessInJob(_: ::winnt::HANDLE, _: ::winnt::HANDLE, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* jobapi.h:46:1 */
    pub fn IsWellKnownSid(_: ::winnt::PSID, _: ::winnt::WELL_KNOWN_SID_TYPE) -> ::minwindef::BOOL; /* securitybaseapi.h:991:1 */
    pub fn MapUserPhysicalPages(_: ::winnt::PVOID, _: ::basetsd::ULONG_PTR, _: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* memoryapi.h:604:1 */
    pub fn QueryMemoryResourceNotification(_: ::winnt::HANDLE, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* memoryapi.h:429:1 */
    pub fn RemoveVectoredContinueHandler(_: ::winnt::PVOID) -> ::minwindef::ULONG; /* errhandlingapi.h:206:1 */
    pub fn RemoveVectoredExceptionHandler(_: ::winnt::PVOID) -> ::minwindef::ULONG; /* errhandlingapi.h:188:1 */
    pub fn SetFileValidData(_: ::winnt::HANDLE, _: ::winnt::LONGLONG) -> ::minwindef::BOOL; /* fileapi.h:1107:1 */
    pub fn SetProcessPriorityBoost(_: ::winnt::HANDLE, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:1076:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetNativeSystemInfo(_: ::sysinfoapi::LPSYSTEM_INFO); /* sysinfoapi.h:495:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010100"))] 
extern "system" {
    pub fn RtlCaptureStackBackTrace(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *mut *mut ::libc::c_void, _: ::minwindef::PDWORD) -> ::minwindef::WORD; /* winnt.h:16911:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] 
extern "system" {
    pub fn GetCurrentProcessorNumber() -> ::minwindef::DWORD; /* processthreadsapi.h:995:1 */
    pub fn GetSystemFileCacheSize(_: ::basetsd::PSIZE_T, _: ::basetsd::PSIZE_T, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:450:1 */
    pub fn GetThreadId(_: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:703:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] #[doc(inline)] pub use self::NeedCurrentDirectoryForExePathW as NeedCurrentDirectoryForExePath; /* processenv.h:354:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] 
extern "system" {
    pub fn NeedCurrentDirectoryForExePathA(_: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* processenv.h:342:1 */
    pub fn NeedCurrentDirectoryForExePathW(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* processenv.h:349:1 */
    pub fn ReOpenFile(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:4824:1 */
    pub fn SetSystemFileCacheSize(_: ::basetsd::SIZE_T, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:460:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn AddMandatoryAce(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:343:1 */
    pub fn AllocateUserPhysicalPagesNuma(_: ::winnt::HANDLE, _: ::basetsd::PULONG_PTR, _: ::basetsd::PULONG_PTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:620:1 */
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
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::ControlServiceExW as ControlServiceEx; /* winsvc.h:1650:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn ControlServiceExA(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PVOID) -> ::minwindef::BOOL; /* winsvc.h:1634:1 */
    pub fn ControlServiceExW(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PVOID) -> ::minwindef::BOOL; /* winsvc.h:1643:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateBoundaryDescriptorW as CreateBoundaryDescriptor; /* winbase.h:7258:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateFileMappingNumaW as CreateFileMappingNuma; /* memoryapi.h:488:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateFileMappingNumaW(_: ::winnt::HANDLE, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* memoryapi.h:476:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreatePrivateNamespaceW as CreatePrivateNamespace; /* winbase.h:7223:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateSymbolicLinkW as CreateSymbolicLink; /* winbase.h:8534:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
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
    pub fn CreateWaitableTimerExW(_: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* synchapi.h:836:1 */
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
    pub fn FindNLSString(_: ::winnt::LCID, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::minwindef::LPINT) -> ::libc::c_int; /* winnls.h:1460:1 */
    pub fn FreeLibraryWhenCallbackReturns(_: ::winnt::PTP_CALLBACK_INSTANCE, _: ::minwindef::HMODULE); /* threadpoolapiset.h:181:1 */
    pub fn GetErrorMode() -> ::minwindef::UINT; /* errhandlingapi.h:146:1 */
    pub fn GetFileMUIInfo(_: ::minwindef::DWORD, _: ::winnt::PCWSTR, _: ::winnls::PFILEMUIINFO, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winnls.h:2084:1 */
    pub fn GetFileMUIPath(_: ::minwindef::DWORD, _: ::winnt::PCWSTR, _: ::winnt::PWSTR, _: ::minwindef::PULONG, _: ::winnt::PWSTR, _: ::minwindef::PULONG, _: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* winnls.h:2093:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetFinalPathNameByHandleW as GetFinalPathNameByHandle; /* fileapi.h:694:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetFinalPathNameByHandleA(_: ::winnt::HANDLE, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:676:1 */
    pub fn GetFinalPathNameByHandleW(_: ::winnt::HANDLE, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:686:1 */
    pub fn GetNamedPipeClientComputerNameW(_: ::winnt::HANDLE, _: ::winnt::LPWSTR, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* namedpipeapi.h:151:1 */
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
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::NotifyServiceStatusChangeW as NotifyServiceStatusChange; /* winsvc.h:1626:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn NotifyServiceStatusChangeA(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::winsvc::PSERVICE_NOTIFYA) -> ::minwindef::DWORD; /* winsvc.h:1612:1 */
    pub fn NotifyServiceStatusChangeW(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::winsvc::PSERVICE_NOTIFYW) -> ::minwindef::DWORD; /* winsvc.h:1620:1 */
    pub fn OpenFileById(_: ::winnt::HANDLE, _: ::winbase::LPFILE_ID_DESCRIPTOR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:8490:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::OpenPrivateNamespaceW as OpenPrivateNamespace; /* winbase.h:7238:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::QueryFullProcessImageNameW as QueryFullProcessImageName; /* winbase.h:3243:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn QueryFullProcessImageNameW(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:3236:1 */
    pub fn QueryIdleProcessorCycleTime(_: ::minwindef::PULONG, _: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:63:1 */
    pub fn QueryProcessAffinityUpdateMode(_: ::winnt::HANDLE, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:780:1 */
    pub fn QueryProcessCycleTime(_: ::winnt::HANDLE, _: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:54:1 */
    pub fn QuerySecurityAccessMask(_: ::winnt::SECURITY_INFORMATION, _: ::minwindef::LPDWORD); /* securitybaseapi.h:1138:1 */
    pub fn QueryServiceDynamicInformation(_: ::winsvc::SERVICE_STATUS_HANDLE, _: ::minwindef::DWORD, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winsvc.h:1658:1 */
    pub fn QueryThreadCycleTime(_: ::winnt::HANDLE, _: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:45:1 */
    pub fn QueryThreadpoolStackInformation(_: ::winnt::PTP_POOL, _: ::winnt::PTP_POOL_STACK_INFORMATION) -> ::minwindef::BOOL; /* threadpoolapiset.h:100:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegCopyTreeW as RegCopyTree; /* winreg.h:1195:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegCopyTreeW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::HKEY) -> ::winreg::LSTATUS; /* winreg.h:1188:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegDeleteTreeW as RegDeleteTree; /* winreg.h:1114:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegDeleteTreeA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:1100:1 */
    pub fn RegDeleteTreeW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:1108:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegGetValueW as RegGetValue; /* winreg.h:1175:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegGetValueA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::winnt::PVOID, _: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:1137:1 */
    pub fn RegGetValueW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::winnt::PVOID, _: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:1157:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegLoadAppKeyW as RegLoadAppKey; /* winreg.h:1254:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegLoadAppKeyA(_: ::winnt::LPCSTR, _: ::minwindef::PHKEY, _: ::winreg::REGSAM, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1234:1 */
    pub fn RegLoadAppKeyW(_: ::winnt::LPCWSTR, _: ::minwindef::PHKEY, _: ::winreg::REGSAM, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1245:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegLoadMUIStringW as RegLoadMUIString; /* winreg.h:1226:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegLoadMUIStringA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::DWORD, _: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:1202:1 */
    pub fn RegLoadMUIStringW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:1215:1 */
    pub fn ReleaseMutexWhenCallbackReturns(_: ::winnt::PTP_CALLBACK_INSTANCE, _: ::winnt::HANDLE); /* threadpoolapiset.h:163:1 */
    pub fn ReleaseSemaphoreWhenCallbackReturns(_: ::winnt::PTP_CALLBACK_INSTANCE, _: ::winnt::HANDLE, _: ::minwindef::DWORD); /* threadpoolapiset.h:153:1 */
    pub fn SetDynamicTimeZoneInformation(_: *const ::timezoneapi::DYNAMIC_TIME_ZONE_INFORMATION) -> ::minwindef::BOOL; /* timezoneapi.h:141:1 */
    pub fn SetEventWhenCallbackReturns(_: ::winnt::PTP_CALLBACK_INSTANCE, _: ::winnt::HANDLE); /* threadpoolapiset.h:144:1 */
    pub fn SetFileIoOverlappedRange(_: ::winnt::HANDLE, _: ::minwindef::PUCHAR, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* fileapi.h:1304:1 */
    pub fn SetProcessAffinityUpdateMode(_: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:771:1 */
    pub fn SetProcessPreferredUILanguages(_: ::minwindef::DWORD, _: ::winnt::PCZZWSTR, _: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:2032:1 */
    pub fn SetSecurityAccessMask(_: ::winnt::SECURITY_INFORMATION, _: ::minwindef::LPDWORD); /* securitybaseapi.h:1219:1 */
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
    pub fn UpdateProcThreadAttribute(_: ::processthreadsapi::LPPROC_THREAD_ATTRIBUTE_LIST, _: ::minwindef::DWORD, _: ::basetsd::DWORD_PTR, _: ::winnt::PVOID, _: ::basetsd::SIZE_T, _: ::winnt::PVOID, _: ::basetsd::PSIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:791:1 */
    pub fn VirtualAllocExNuma(_: ::winnt::HANDLE, _: ::minwindef::LPVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* memoryapi.h:632:1 */
    pub fn WaitForThreadpoolIoCallbacks(_: ::winnt::PTP_IO, _: ::minwindef::BOOL); /* threadpoolapiset.h:366:1 */
    pub fn WaitForThreadpoolTimerCallbacks(_: ::winnt::PTP_TIMER, _: ::minwindef::BOOL); /* threadpoolapiset.h:283:1 */
    pub fn WaitForThreadpoolWaitCallbacks(_: ::winnt::PTP_WAIT, _: ::minwindef::BOOL); /* threadpoolapiset.h:321:1 */
    pub fn WaitForThreadpoolWorkCallbacks(_: ::winnt::PTP_WORK, _: ::minwindef::BOOL); /* threadpoolapiset.h:236:1 */
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
    pub fn EnumSystemLocalesEx(_: ::winnls::LOCALE_ENUMPROCEX, _: ::minwindef::DWORD, _: ::minwindef::LPARAM, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winnls.h:2554:1 */
    pub fn FindNLSStringEx(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::minwindef::LPINT, _: ::winnls::LPNLSVERSIONINFO, _: ::minwindef::LPVOID, _: ::minwindef::LPARAM) -> ::libc::c_int; /* winnls.h:2469:1 */
    pub fn FlsAlloc(_: ::winnt::PFLS_CALLBACK_FUNCTION) -> ::minwindef::DWORD; /* fibersapi.h:58:1 */
    pub fn FlsFree(_: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fibersapi.h:83:1 */
    pub fn FlsGetValue(_: ::minwindef::DWORD) -> ::winnt::PVOID; /* fibersapi.h:66:1 */
    pub fn FlsSetValue(_: ::minwindef::DWORD, _: ::winnt::PVOID) -> ::minwindef::BOOL; /* fibersapi.h:74:1 */
    pub fn FlushProcessWriteBuffers(); /* processthreadsapi.h:726:1 */
    pub fn GetCalendarInfoEx(_: ::winnt::LPCWSTR, _: ::winnls::CALID, _: ::winnt::LPCWSTR, _: ::winnls::CALTYPE, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::minwindef::LPDWORD) -> ::libc::c_int; /* winnls.h:2383:1 */
    pub fn GetDynamicTimeZoneInformation(_: ::timezoneapi::PDYNAMIC_TIME_ZONE_INFORMATION) -> ::minwindef::DWORD; /* timezoneapi.h:162:1 */
    pub fn GetFileInformationByHandleEx(_: ::winnt::HANDLE, _: ::minwinbase::FILE_INFO_BY_HANDLE_CLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:8455:1 */
    pub fn GetLocaleInfoEx(_: ::winnt::LPCWSTR, _: ::winnls::LCTYPE, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2373:1 */
    pub fn GetNLSVersionEx(_: ::winnls::NLS_FUNCTION, _: ::winnt::LPCWSTR, _: ::winnls::LPNLSVERSIONINFOEX) -> ::minwindef::BOOL; /* winnls.h:2449:1 */
    pub fn GetTickCount64() -> ::winnt::ULONGLONG; /* sysinfoapi.h:221:1 */
    pub fn GetUserDefaultLocaleName(_: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2433:1 */
    pub fn IdnToAscii(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2312:8 */
    pub fn IdnToUnicode(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2328:8 */
    pub fn InitOnceBeginInitialize(_: ::synchapi::LPINIT_ONCE, _: ::minwindef::DWORD, _: ::minwindef::PBOOL, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* synchapi.h:337:1 */
    pub fn InitOnceComplete(_: ::synchapi::LPINIT_ONCE, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* synchapi.h:348:1 */
    pub fn InitOnceExecuteOnce(_: ::synchapi::PINIT_ONCE, _: ::synchapi::PINIT_ONCE_FN, _: ::winnt::PVOID, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* synchapi.h:326:1 */
    pub fn InitOnceInitialize(_: ::synchapi::PINIT_ONCE); /* synchapi.h:318:1 */
    pub fn InitializeConditionVariable(_: ::synchapi::PCONDITION_VARIABLE); /* synchapi.h:381:1 */
    pub fn InitializeCriticalSectionEx(_: ::minwinbase::LPCRITICAL_SECTION, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:222:1 */
    pub fn InitializeSRWLock(_: ::synchapi::PSRWLOCK); /* synchapi.h:77:1 */
    pub fn IsThreadAFiber() -> ::minwindef::BOOL; /* fibersapi.h:107:1 */
    pub fn IsValidLocaleName(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winnls.h:2507:1 */
    pub fn LCMapStringEx(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPWSTR, _: ::libc::c_int, _: ::winnls::LPNLSVERSIONINFO, _: ::minwindef::LPVOID, _: ::minwindef::LPARAM) -> ::libc::c_int; /* winnls.h:2492:1 */
    pub fn LocaleNameToLCID(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::winnt::LCID; /* winnls.h:1661:1 */
    pub fn ReleaseSRWLockExclusive(_: ::synchapi::PSRWLOCK); /* synchapi.h:86:1 */
    pub fn ReleaseSRWLockShared(_: ::synchapi::PSRWLOCK); /* synchapi.h:95:1 */
    pub fn SleepConditionVariableCS(_: ::synchapi::PCONDITION_VARIABLE, _: ::minwinbase::PCRITICAL_SECTION, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:405:1 */
    pub fn SleepConditionVariableSRW(_: ::synchapi::PCONDITION_VARIABLE, _: ::synchapi::PSRWLOCK, _: ::minwindef::DWORD, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* synchapi.h:415:1 */
    pub fn TryAcquireSRWLockExclusive(_: ::synchapi::PSRWLOCK) -> ::winnt::BOOLEAN; /* synchapi.h:122:1 */
    pub fn TryAcquireSRWLockShared(_: ::synchapi::PSRWLOCK) -> ::winnt::BOOLEAN; /* synchapi.h:131:1 */
    pub fn WakeAllConditionVariable(_: ::synchapi::PCONDITION_VARIABLE); /* synchapi.h:397:1 */
    pub fn WakeConditionVariable(_: ::synchapi::PCONDITION_VARIABLE); /* synchapi.h:389:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn GetCurrentProcessorNumberEx(_: ::winnt::PPROCESSOR_NUMBER); /* processthreadsapi.h:1041:1 */
    pub fn GetLogicalProcessorInformationEx(_: ::winnt::LOGICAL_PROCESSOR_RELATIONSHIP, _: ::winnt::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:467:1 */
    pub fn GetNumaNodeProcessorMaskEx(_: ::minwindef::USHORT, _: ::winnt::PGROUP_AFFINITY) -> ::minwindef::BOOL; /* systemtopologyapi.h:54:1 */
    pub fn GetProcessGroupAffinity(_: ::winnt::HANDLE, _: ::minwindef::PUSHORT, _: ::minwindef::PUSHORT) -> ::minwindef::BOOL; /* processtopologyapi.h:49:1 */
    pub fn GetThreadGroupAffinity(_: ::winnt::HANDLE, _: ::winnt::PGROUP_AFFINITY) -> ::minwindef::BOOL; /* processtopologyapi.h:72:1 */
    pub fn GetThreadIdealProcessorEx(_: ::winnt::HANDLE, _: ::winnt::PPROCESSOR_NUMBER) -> ::minwindef::BOOL; /* processthreadsapi.h:1032:1 */
    pub fn QueryIdleProcessorCycleTimeEx(_: ::minwindef::USHORT, _: ::minwindef::PULONG, _: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:77:1 */
    pub fn SetThreadGroupAffinity(_: ::winnt::HANDLE, _: *const ::winnt::GROUP_AFFINITY, _: ::winnt::PGROUP_AFFINITY) -> ::minwindef::BOOL; /* processtopologyapi.h:81:1 */
    pub fn SetWaitableTimerEx(_: ::winnt::HANDLE, _: *const ::winnt::LARGE_INTEGER, _: ::winnt::LONG, _: ::synchapi::PTIMERAPCROUTINE, _: ::minwindef::LPVOID, _: ::minwinbase::PREASON_CONTEXT, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* synchapi.h:700:1 */
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
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[cfg(any(target_arch="x86_64", target_arch="arm"))] 
extern "system" {
    pub fn RtlAddGrowableFunctionTable(_: *mut *mut ::libc::c_void, _: ::winnt::PRUNTIME_FUNCTION, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::basetsd::ULONG_PTR, _: ::basetsd::ULONG_PTR) -> ::minwindef::DWORD; /* winnt.h:17010:1, winnt.h:17168:1 */
    pub fn RtlDeleteGrowableFunctionTable(_: ::winnt::PVOID); /* winnt.h:17032:1, winnt.h:17190:1 */
    pub fn RtlGrowFunctionTable(_: ::winnt::PVOID, _: ::minwindef::DWORD); /* winnt.h:17023:1, winnt.h:17181:1 */
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
    pub fn GetFirmwareEnvironmentVariableExW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::PVOID, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::DWORD; /* winbase.h:3574:1 */
    pub fn GetMemoryErrorHandlingCapabilities(_: ::minwindef::PULONG) -> ::minwindef::BOOL; /* memoryapi.h:653:1 */
    pub fn GetOsSafeBootMode(_: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:547:1 */
    pub fn GetProcessMitigationPolicy(_: ::winnt::HANDLE, _: ::winnt::PROCESS_MITIGATION_POLICY, _: ::winnt::PVOID, _: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:890:1 */
    pub fn GetThreadInformation(_: ::winnt::HANDLE, _: ::processthreadsapi::THREAD_INFORMATION_CLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:1119:1 */
    pub fn PrefetchVirtualMemory(_: ::winnt::HANDLE, _: ::basetsd::ULONG_PTR, _: ::memoryapi::PWIN32_MEMORY_RANGE_ENTRY, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* memoryapi.h:504:1 */
    pub fn RegisterBadMemoryNotification(_: ::memoryapi::PBAD_MEMORY_CALLBACK_ROUTINE) -> ::winnt::PVOID; /* memoryapi.h:672:1 */
    pub fn SetCachedSigningLevel(_: ::winnt::PHANDLE, _: ::minwindef::ULONG, _: ::minwindef::ULONG, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:1307:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[doc(inline)] pub use self::SetFirmwareEnvironmentVariableExW as SetFirmwareEnvironmentVariableEx; /* winbase.h:3636:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn SetFirmwareEnvironmentVariableExW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::PVOID, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3628:1 */
    pub fn SetProcessMitigationPolicy(_: ::winnt::PROCESS_MITIGATION_POLICY, _: ::winnt::PVOID, _: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:880:1 */
    pub fn SetThreadInformation(_: ::winnt::HANDLE, _: ::processthreadsapi::THREAD_INFORMATION_CLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:1130:1 */
    pub fn UnmapViewOfFileEx(_: ::winnt::PVOID, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* memoryapi.h:515:1 */
    pub fn UnregisterBadMemoryNotification(_: ::winnt::PVOID) -> ::minwindef::BOOL; /* memoryapi.h:681:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn CreateFile2(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::fileapi::LPCREATEFILE2_EXTENDED_PARAMETERS) -> ::winnt::HANDLE; /* fileapi.h:1272:1 */
    pub fn CreateFileMappingFromApp(_: ::winnt::HANDLE, _: ::minwinbase::PSECURITY_ATTRIBUTES, _: ::minwindef::ULONG, _: ::basetsd::ULONG64, _: ::winnt::PCWSTR) -> ::winnt::HANDLE; /* memoryapi.h:537:1 */
    pub fn EnumDynamicTimeZoneInformation(_: ::minwindef::DWORD, _: ::timezoneapi::PDYNAMIC_TIME_ZONE_INFORMATION) -> ::minwindef::DWORD; /* timezoneapi.h:193:1 */
    pub fn GetDynamicTimeZoneInformationEffectiveYears(_: ::timezoneapi::PDYNAMIC_TIME_ZONE_INFORMATION, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* timezoneapi.h:203:1 */
    pub fn IsValidNLSVersion(_: ::winnls::NLS_FUNCTION, _: ::winnt::LPCWSTR, _: ::winnls::LPNLSVERSIONINFOEX) -> ::minwindef::DWORD; /* winnls.h:2459:1 */
    pub fn MapViewOfFileFromApp(_: ::winnt::HANDLE, _: ::minwindef::ULONG, _: ::basetsd::ULONG64, _: ::basetsd::SIZE_T) -> ::winnt::PVOID; /* memoryapi.h:550:1 */
    pub fn SystemTimeToTzSpecificLocalTimeEx(_: *const ::timezoneapi::DYNAMIC_TIME_ZONE_INFORMATION, _: *const ::minwinbase::SYSTEMTIME, _: ::minwinbase::LPSYSTEMTIME) -> ::minwindef::BOOL; /* timezoneapi.h:214:1 */
    pub fn TzSpecificLocalTimeToSystemTimeEx(_: *const ::timezoneapi::DYNAMIC_TIME_ZONE_INFORMATION, _: *const ::minwinbase::SYSTEMTIME, _: ::minwinbase::LPSYSTEMTIME) -> ::minwindef::BOOL; /* timezoneapi.h:225:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_0a000000"))] 
extern "system" {
    pub fn QueryProtectedPolicy(_: ::guiddef::LPCGUID, _: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* processthreadsapi.h:1185:1 */
}
