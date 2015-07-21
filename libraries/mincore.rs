
extern "system" {
    pub fn InitializeSListHead(ListHead: ::winnt::PSLIST_HEADER); /* interlockedapi.h:50:1 */
    pub fn InterlockedFlushSList(ListHead: ::winnt::PSLIST_HEADER) -> ::winnt::PSLIST_ENTRY; /* interlockedapi.h:93:1 */
    pub fn InterlockedPopEntrySList(ListHead: ::winnt::PSLIST_HEADER) -> ::winnt::PSLIST_ENTRY; /* interlockedapi.h:58:1 */
    pub fn InterlockedPushEntrySList(ListHead: ::winnt::PSLIST_HEADER, ListEntry: ::winnt::PSLIST_ENTRY) -> ::winnt::PSLIST_ENTRY; /* interlockedapi.h:66:1 */
    pub fn QueryDepthSList(ListHead: ::winnt::PSLIST_HEADER) -> ::minwindef::USHORT; /* interlockedapi.h:101:1 */
    pub fn QueryPerformanceCounter(lpPerformanceCount: *mut ::winnt::LARGE_INTEGER) -> ::minwindef::BOOL; /* profileapi.h:42:1 */
    pub fn QueryPerformanceFrequency(lpFrequency: *mut ::winnt::LARGE_INTEGER) -> ::minwindef::BOOL; /* profileapi.h:50:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86"))] #[doc(inline)] pub use self::CharUpperW as ua_CharUpperW; /* stralign.h:93:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn GetWriteWatch(dwFlags: ::minwindef::DWORD, lpBaseAddress: ::winnt::PVOID, dwRegionSize: ::basetsd::SIZE_T, lpAddresses: *mut *mut ::libc::c_void, lpdwCount: *mut ::libc::c_ulonglong, lpdwGranularity: ::minwindef::LPDWORD) -> ::minwindef::UINT; /* memoryapi.h:389:1 */
    pub fn ReadProcessMemory(hProcess: ::winnt::HANDLE, lpBaseAddress: ::minwindef::LPCVOID, lpBuffer: ::minwindef::LPVOID, nSize: ::basetsd::SIZE_T, lpNumberOfBytesRead: *mut ::libc::c_ulonglong) -> ::minwindef::BOOL; /* memoryapi.h:217:1 */
    pub fn RtlAddFunctionTable(FunctionTable: ::winnt::PRUNTIME_FUNCTION, EntryCount: ::minwindef::DWORD, BaseAddress: ::basetsd::DWORD64) -> ::winnt::BOOLEAN; /* winnt.h:16974:1 */
    pub fn RtlInstallFunctionTableCallback(TableIdentifier: ::basetsd::DWORD64, BaseAddress: ::basetsd::DWORD64, Length: ::minwindef::DWORD, Callback: ::winnt::PGET_RUNTIME_FUNCTION_CALLBACK, Context: ::winnt::PVOID, OutOfProcessCallbackDll: ::winnt::PCWSTR) -> ::winnt::BOOLEAN; /* winnt.h:16992:1 */
    pub fn WriteProcessMemory(hProcess: ::winnt::HANDLE, lpBaseAddress: ::minwindef::LPVOID, lpBuffer: ::minwindef::LPCVOID, nSize: ::basetsd::SIZE_T, lpNumberOfBytesWritten: *mut ::libc::c_ulonglong) -> ::minwindef::BOOL; /* memoryapi.h:230:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="arm"))] 
extern "system" {
    pub fn RtlAddFunctionTable(FunctionTable: ::winnt::PRUNTIME_FUNCTION, EntryCount: ::minwindef::DWORD, BaseAddress: ::minwindef::DWORD) -> ::winnt::BOOLEAN; /* winnt.h:17132:1 */
    pub fn RtlInstallFunctionTableCallback(TableIdentifier: ::minwindef::DWORD, BaseAddress: ::minwindef::DWORD, Length: ::minwindef::DWORD, Callback: ::winnt::PGET_RUNTIME_FUNCTION_CALLBACK, Context: ::winnt::PVOID, OutOfProcessCallbackDll: ::winnt::PCWSTR) -> ::winnt::BOOLEAN; /* winnt.h:17150:1 */
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
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AbortSystemShutdownW as AbortSystemShutdown; /* winreg.h:1307:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AbortSystemShutdownW(lpMachineName: ::winnt::LPWSTR) -> ::minwindef::BOOL; /* winreg.h:1303:1 */
    pub fn AccessCheck(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, ClientToken: ::winnt::HANDLE, DesiredAccess: ::minwindef::DWORD, GenericMapping: ::winnt::PGENERIC_MAPPING, PrivilegeSet: ::winnt::PPRIVILEGE_SET, PrivilegeSetLength: ::minwindef::LPDWORD, GrantedAccess: ::minwindef::LPDWORD, AccessStatus: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:56:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AccessCheckAndAuditAlarmW as AccessCheckAndAuditAlarm; /* securitybaseapi.h:87:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AccessCheckAndAuditAlarmW(SubsystemName: ::winnt::LPCWSTR, HandleId: ::minwindef::LPVOID, ObjectTypeName: ::winnt::LPWSTR, ObjectName: ::winnt::LPWSTR, SecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, DesiredAccess: ::minwindef::DWORD, GenericMapping: ::winnt::PGENERIC_MAPPING, ObjectCreation: ::minwindef::BOOL, GrantedAccess: ::minwindef::LPDWORD, AccessStatus: ::minwindef::LPBOOL, pfGenerateOnClose: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:71:1 */
    pub fn AccessCheckByType(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, PrincipalSelfSid: ::winnt::PSID, ClientToken: ::winnt::HANDLE, DesiredAccess: ::minwindef::DWORD, ObjectTypeList: ::winnt::POBJECT_TYPE_LIST, ObjectTypeListLength: ::minwindef::DWORD, GenericMapping: ::winnt::PGENERIC_MAPPING, PrivilegeSet: ::winnt::PPRIVILEGE_SET, PrivilegeSetLength: ::minwindef::LPDWORD, GrantedAccess: ::minwindef::LPDWORD, AccessStatus: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:93:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AccessCheckByTypeAndAuditAlarmW as AccessCheckByTypeAndAuditAlarm; /* securitybaseapi.h:150:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AccessCheckByTypeAndAuditAlarmW(SubsystemName: ::winnt::LPCWSTR, HandleId: ::minwindef::LPVOID, ObjectTypeName: ::winnt::LPCWSTR, ObjectName: ::winnt::LPCWSTR, SecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, PrincipalSelfSid: ::winnt::PSID, DesiredAccess: ::minwindef::DWORD, AuditType: ::winnt::AUDIT_EVENT_TYPE, Flags: ::minwindef::DWORD, ObjectTypeList: ::winnt::POBJECT_TYPE_LIST, ObjectTypeListLength: ::minwindef::DWORD, GenericMapping: ::winnt::PGENERIC_MAPPING, ObjectCreation: ::minwindef::BOOL, GrantedAccess: ::minwindef::LPDWORD, AccessStatus: ::minwindef::LPBOOL, pfGenerateOnClose: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:129:1 */
    pub fn AccessCheckByTypeResultList(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, PrincipalSelfSid: ::winnt::PSID, ClientToken: ::winnt::HANDLE, DesiredAccess: ::minwindef::DWORD, ObjectTypeList: ::winnt::POBJECT_TYPE_LIST, ObjectTypeListLength: ::minwindef::DWORD, GenericMapping: ::winnt::PGENERIC_MAPPING, PrivilegeSet: ::winnt::PPRIVILEGE_SET, PrivilegeSetLength: ::minwindef::LPDWORD, GrantedAccessList: ::minwindef::LPDWORD, AccessStatusList: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:111:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AccessCheckByTypeResultListAndAuditAlarmW as AccessCheckByTypeResultListAndAuditAlarm; /* securitybaseapi.h:177:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AccessCheckByTypeResultListAndAuditAlarmByHandleW as AccessCheckByTypeResultListAndAuditAlarmByHandle; /* securitybaseapi.h:205:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleW(SubsystemName: ::winnt::LPCWSTR, HandleId: ::minwindef::LPVOID, ClientToken: ::winnt::HANDLE, ObjectTypeName: ::winnt::LPCWSTR, ObjectName: ::winnt::LPCWSTR, SecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, PrincipalSelfSid: ::winnt::PSID, DesiredAccess: ::minwindef::DWORD, AuditType: ::winnt::AUDIT_EVENT_TYPE, Flags: ::minwindef::DWORD, ObjectTypeList: ::winnt::POBJECT_TYPE_LIST, ObjectTypeListLength: ::minwindef::DWORD, GenericMapping: ::winnt::PGENERIC_MAPPING, ObjectCreation: ::minwindef::BOOL, GrantedAccessList: ::minwindef::LPDWORD, AccessStatusList: ::minwindef::LPDWORD, pfGenerateOnClose: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:183:1 */
    pub fn AccessCheckByTypeResultListAndAuditAlarmW(SubsystemName: ::winnt::LPCWSTR, HandleId: ::minwindef::LPVOID, ObjectTypeName: ::winnt::LPCWSTR, ObjectName: ::winnt::LPCWSTR, SecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, PrincipalSelfSid: ::winnt::PSID, DesiredAccess: ::minwindef::DWORD, AuditType: ::winnt::AUDIT_EVENT_TYPE, Flags: ::minwindef::DWORD, ObjectTypeList: ::winnt::POBJECT_TYPE_LIST, ObjectTypeListLength: ::minwindef::DWORD, GenericMapping: ::winnt::PGENERIC_MAPPING, ObjectCreation: ::minwindef::BOOL, GrantedAccessList: ::minwindef::LPDWORD, AccessStatusList: ::minwindef::LPDWORD, pfGenerateOnClose: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:156:1 */
    pub fn AddAccessAllowedAce(pAcl: ::winnt::PACL, dwAceRevision: ::minwindef::DWORD, AccessMask: ::minwindef::DWORD, pSid: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:211:1 */
    pub fn AddAccessAllowedAceEx(pAcl: ::winnt::PACL, dwAceRevision: ::minwindef::DWORD, AceFlags: ::minwindef::DWORD, AccessMask: ::minwindef::DWORD, pSid: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:222:1 */
    pub fn AddAccessAllowedObjectAce(pAcl: ::winnt::PACL, dwAceRevision: ::minwindef::DWORD, AceFlags: ::minwindef::DWORD, AccessMask: ::minwindef::DWORD, ObjectTypeGuid: *mut ::guiddef::GUID, InheritedObjectTypeGuid: *mut ::guiddef::GUID, pSid: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:234:1 */
    pub fn AddAccessDeniedAce(pAcl: ::winnt::PACL, dwAceRevision: ::minwindef::DWORD, AccessMask: ::minwindef::DWORD, pSid: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:248:1 */
    pub fn AddAccessDeniedAceEx(pAcl: ::winnt::PACL, dwAceRevision: ::minwindef::DWORD, AceFlags: ::minwindef::DWORD, AccessMask: ::minwindef::DWORD, pSid: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:259:1 */
    pub fn AddAccessDeniedObjectAce(pAcl: ::winnt::PACL, dwAceRevision: ::minwindef::DWORD, AceFlags: ::minwindef::DWORD, AccessMask: ::minwindef::DWORD, ObjectTypeGuid: *mut ::guiddef::GUID, InheritedObjectTypeGuid: *mut ::guiddef::GUID, pSid: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:271:1 */
    pub fn AddAce(pAcl: ::winnt::PACL, dwAceRevision: ::minwindef::DWORD, dwStartingAceIndex: ::minwindef::DWORD, pAceList: ::minwindef::LPVOID, nAceListLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:285:1 */
    pub fn AddAuditAccessAce(pAcl: ::winnt::PACL, dwAceRevision: ::minwindef::DWORD, dwAccessMask: ::minwindef::DWORD, pSid: ::winnt::PSID, bAuditSuccess: ::minwindef::BOOL, bAuditFailure: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:297:1 */
    pub fn AddAuditAccessAceEx(pAcl: ::winnt::PACL, dwAceRevision: ::minwindef::DWORD, AceFlags: ::minwindef::DWORD, dwAccessMask: ::minwindef::DWORD, pSid: ::winnt::PSID, bAuditSuccess: ::minwindef::BOOL, bAuditFailure: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:310:1 */
    pub fn AddAuditAccessObjectAce(pAcl: ::winnt::PACL, dwAceRevision: ::minwindef::DWORD, AceFlags: ::minwindef::DWORD, AccessMask: ::minwindef::DWORD, ObjectTypeGuid: *mut ::guiddef::GUID, InheritedObjectTypeGuid: *mut ::guiddef::GUID, pSid: ::winnt::PSID, bAuditSuccess: ::minwindef::BOOL, bAuditFailure: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:324:1 */
    pub fn AddDllDirectory(NewDirectory: ::winnt::PCWSTR) -> ::libloaderapi::DLL_DIRECTORY_COOKIE; /* libloaderapi.h:498:1 */
    pub fn AddSIDToBoundaryDescriptor(BoundaryDescriptor: *mut *mut ::libc::c_void, RequiredSid: ::winnt::PSID) -> ::minwindef::BOOL; /* namespaceapi.h:82:1 */
    pub fn AdjustTokenGroups(TokenHandle: ::winnt::HANDLE, ResetToDefault: ::minwindef::BOOL, NewState: ::winnt::PTOKEN_GROUPS, BufferLength: ::minwindef::DWORD, PreviousState: ::winnt::PTOKEN_GROUPS, ReturnLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:392:1 */
    pub fn AdjustTokenPrivileges(TokenHandle: ::winnt::HANDLE, DisableAllPrivileges: ::minwindef::BOOL, NewState: ::winnt::PTOKEN_PRIVILEGES, BufferLength: ::minwindef::DWORD, PreviousState: ::winnt::PTOKEN_PRIVILEGES, ReturnLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:405:1 */
    pub fn AllocConsole() -> ::minwindef::BOOL; /* consoleapi.h:44:1 */
    pub fn AllocateAndInitializeSid(pIdentifierAuthority: ::winnt::PSID_IDENTIFIER_AUTHORITY, nSubAuthorityCount: ::minwindef::BYTE, nSubAuthority0: ::minwindef::DWORD, nSubAuthority1: ::minwindef::DWORD, nSubAuthority2: ::minwindef::DWORD, nSubAuthority3: ::minwindef::DWORD, nSubAuthority4: ::minwindef::DWORD, nSubAuthority5: ::minwindef::DWORD, nSubAuthority6: ::minwindef::DWORD, nSubAuthority7: ::minwindef::DWORD, pSid: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:418:1 */
    pub fn AllocateLocallyUniqueId(Luid: ::winnt::PLUID) -> ::minwindef::BOOL; /* securitybaseapi.h:436:1 */
    pub fn AreAllAccessesGranted(GrantedAccess: ::minwindef::DWORD, DesiredAccess: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:444:1 */
    pub fn AreAnyAccessesGranted(GrantedAccess: ::minwindef::DWORD, DesiredAccess: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:453:1 */
    pub fn Beep(dwFreq: ::minwindef::DWORD, dwDuration: ::minwindef::DWORD) -> ::minwindef::BOOL; /* utilapiset.h:85:1 */
    pub fn CancelIo(hFile: ::winnt::HANDLE) -> ::minwindef::BOOL; /* ioapiset.h:178:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ChangeServiceConfigW as ChangeServiceConfig; /* winsvc.h:1035:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ChangeServiceConfig2W as ChangeServiceConfig2; /* winsvc.h:1057:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ChangeServiceConfig2A(hService: ::winsvc::SC_HANDLE, dwInfoLevel: ::minwindef::DWORD, lpInfo: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winsvc.h:1043:1 */
    pub fn ChangeServiceConfig2W(hService: ::winsvc::SC_HANDLE, dwInfoLevel: ::minwindef::DWORD, lpInfo: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winsvc.h:1051:1 */
    pub fn ChangeServiceConfigA(hService: ::winsvc::SC_HANDLE, dwServiceType: ::minwindef::DWORD, dwStartType: ::minwindef::DWORD, dwErrorControl: ::minwindef::DWORD, lpBinaryPathName: ::winnt::LPCSTR, lpLoadOrderGroup: ::winnt::LPCSTR, lpdwTagId: ::minwindef::LPDWORD, lpDependencies: ::winnt::LPCSTR, lpServiceStartName: ::winnt::LPCSTR, lpPassword: ::winnt::LPCSTR, lpDisplayName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winsvc.h:1005:1 */
    pub fn ChangeServiceConfigW(hService: ::winsvc::SC_HANDLE, dwServiceType: ::minwindef::DWORD, dwStartType: ::minwindef::DWORD, dwErrorControl: ::minwindef::DWORD, lpBinaryPathName: ::winnt::LPCWSTR, lpLoadOrderGroup: ::winnt::LPCWSTR, lpdwTagId: ::minwindef::LPDWORD, lpDependencies: ::winnt::LPCWSTR, lpServiceStartName: ::winnt::LPCWSTR, lpPassword: ::winnt::LPCWSTR, lpDisplayName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winsvc.h:1021:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharLowerW as CharLower; /* winuser.h:5424:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharLowerBuffW as CharLowerBuff; /* winuser.h:5442:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharLowerBuffW(lpsz: ::winnt::LPWSTR, cchLength: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:5438:1 */
    pub fn CharLowerW(lpsz: ::winnt::LPWSTR) -> ::winnt::LPWSTR; /* winuser.h:5421:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharNextW as CharNext; /* winuser.h:5458:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharNextW(lpsz: ::winnt::LPCWSTR) -> ::winnt::LPWSTR; /* winuser.h:5455:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharPrevW as CharPrev; /* winuser.h:5476:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharPrevW(lpszStart: ::winnt::LPCWSTR, lpszCurrent: ::winnt::LPCWSTR) -> ::winnt::LPWSTR; /* winuser.h:5472:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharUpperW as CharUpper; /* winuser.h:5390:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CharUpperBuffW as CharUpperBuff; /* winuser.h:5408:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CharUpperBuffW(lpsz: ::winnt::LPWSTR, cchLength: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winuser.h:5404:1 */
    pub fn CharUpperW(lpsz: ::winnt::LPWSTR) -> ::winnt::LPWSTR; /* winuser.h:5387:1 */
    pub fn CheckTokenMembership(TokenHandle: ::winnt::HANDLE, SidToCheck: ::winnt::PSID, IsMember: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:462:1 */
    pub fn ClearCommBreak(hFile: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:2004:1 */
    pub fn ClearCommError(hFile: ::winnt::HANDLE, lpErrors: ::minwindef::LPDWORD, lpStat: ::winbase::LPCOMSTAT) -> ::minwindef::BOOL; /* winbase.h:2011:1 */
    pub fn ClosePrivateNamespace(Handle: ::winnt::HANDLE, Flags: ::minwindef::ULONG) -> ::winnt::BOOLEAN; /* namespaceapi.h:64:1 */
    pub fn CloseServiceHandle(hSCObject: ::winsvc::SC_HANDLE) -> ::minwindef::BOOL; /* winsvc.h:1065:1 */
    pub fn CompareFileTime(lpFileTime1: *const ::minwindef::FILETIME, lpFileTime2: *const ::minwindef::FILETIME) -> ::winnt::LONG; /* fileapi.h:77:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CompareStringW as CompareString; /* stringapiset.h:93:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CompareStringW(Locale: ::winnt::LCID, dwCmpFlags: ::minwindef::DWORD, lpString1: ::winnt::PCNZWCH, cchCount1: ::libc::c_int, lpString2: ::winnt::PCNZWCH, cchCount2: ::libc::c_int) -> ::libc::c_int; /* stringapiset.h:83:1 */
    pub fn ConnectNamedPipe(hNamedPipe: ::winnt::HANDLE, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* namedpipeapi.h:61:1 */
    pub fn ContinueDebugEvent(dwProcessId: ::minwindef::DWORD, dwThreadId: ::minwindef::DWORD, dwContinueStatus: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:117:1 */
    pub fn ControlService(hService: ::winsvc::SC_HANDLE, dwControl: ::minwindef::DWORD, lpServiceStatus: ::winsvc::LPSERVICE_STATUS) -> ::minwindef::BOOL; /* winsvc.h:1072:1 */
    pub fn ConvertDefaultLocale(Locale: ::winnt::LCID) -> ::winnt::LCID; /* winnls.h:1957:1 */
    pub fn ConvertToAutoInheritPrivateObjectSecurity(ParentDescriptor: ::winnt::PSECURITY_DESCRIPTOR, CurrentSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, NewSecurityDescriptor: *mut *mut ::libc::c_void, ObjectType: *mut ::guiddef::GUID, IsDirectoryObject: ::winnt::BOOLEAN, GenericMapping: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:513:1 */
    pub fn CopySid(nDestinationSidLength: ::minwindef::DWORD, pDestinationSid: ::winnt::PSID, pSourceSid: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:526:1 */
    pub fn CreateBoundaryDescriptorW(Name: ::winnt::LPCWSTR, Flags: ::minwindef::ULONG) -> ::winnt::HANDLE; /* namespaceapi.h:73:1 */
    pub fn CreateConsoleScreenBuffer(dwDesiredAccess: ::minwindef::DWORD, dwShareMode: ::minwindef::DWORD, lpSecurityAttributes: *const ::minwinbase::SECURITY_ATTRIBUTES, dwFlags: ::minwindef::DWORD, lpScreenBufferData: ::minwindef::LPVOID) -> ::winnt::HANDLE; /* wincon.h:826:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateDirectoryExW as CreateDirectoryEx; /* winbase.h:4665:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
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
    pub fn CreateFileMappingW(hFile: ::winnt::HANDLE, lpFileMappingAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, flProtect: ::minwindef::DWORD, dwMaximumSizeHigh: ::minwindef::DWORD, dwMaximumSizeLow: ::minwindef::DWORD, lpName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* memoryapi.h:243:1 */
    pub fn CreateFileW(lpFileName: ::winnt::LPCWSTR, dwDesiredAccess: ::minwindef::DWORD, dwShareMode: ::minwindef::DWORD, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, dwCreationDisposition: ::minwindef::DWORD, dwFlagsAndAttributes: ::minwindef::DWORD, hTemplateFile: ::winnt::HANDLE) -> ::winnt::HANDLE; /* fileapi.h:135:1 */
    pub fn CreateIoCompletionPort(FileHandle: ::winnt::HANDLE, ExistingCompletionPort: ::winnt::HANDLE, CompletionKey: ::basetsd::ULONG_PTR, NumberOfConcurrentThreads: ::minwindef::DWORD) -> ::winnt::HANDLE; /* ioapiset.h:64:1 */
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
    pub fn CreateNamedPipeW(lpName: ::winnt::LPCWSTR, dwOpenMode: ::minwindef::DWORD, dwPipeMode: ::minwindef::DWORD, nMaxInstances: ::minwindef::DWORD, nOutBufferSize: ::minwindef::DWORD, nInBufferSize: ::minwindef::DWORD, nDefaultTimeOut: ::minwindef::DWORD, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::winnt::HANDLE; /* namedpipeapi.h:116:1 */
    pub fn CreatePipe(hReadPipe: ::winnt::PHANDLE, hWritePipe: ::winnt::PHANDLE, lpPipeAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, nSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:50:1 */
    pub fn CreatePrivateNamespaceW(lpPrivateNamespaceAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpBoundaryDescriptor: ::minwindef::LPVOID, lpAliasPrefix: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* namespaceapi.h:45:1 */
    pub fn CreatePrivateObjectSecurity(ParentDescriptor: ::winnt::PSECURITY_DESCRIPTOR, CreatorDescriptor: ::winnt::PSECURITY_DESCRIPTOR, NewDescriptor: *mut *mut ::libc::c_void, IsDirectoryObject: ::minwindef::BOOL, Token: ::winnt::HANDLE, GenericMapping: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:536:1 */
    pub fn CreatePrivateObjectSecurityEx(ParentDescriptor: ::winnt::PSECURITY_DESCRIPTOR, CreatorDescriptor: ::winnt::PSECURITY_DESCRIPTOR, NewDescriptor: *mut *mut ::libc::c_void, ObjectType: *mut ::guiddef::GUID, IsContainerObject: ::minwindef::BOOL, AutoInheritFlags: ::minwindef::ULONG, Token: ::winnt::HANDLE, GenericMapping: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:549:1 */
    pub fn CreatePrivateObjectSecurityWithMultipleInheritance(ParentDescriptor: ::winnt::PSECURITY_DESCRIPTOR, CreatorDescriptor: ::winnt::PSECURITY_DESCRIPTOR, NewDescriptor: *mut *mut ::libc::c_void, ObjectTypes: *mut *mut ::guiddef::GUID, GuidCount: ::minwindef::ULONG, IsContainerObject: ::minwindef::BOOL, AutoInheritFlags: ::minwindef::ULONG, Token: ::winnt::HANDLE, GenericMapping: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:564:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateProcessW as CreateProcess; /* processthreadsapi.h:532:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateProcessA(lpApplicationName: ::winnt::LPCSTR, lpCommandLine: ::winnt::LPSTR, lpProcessAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpThreadAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, bInheritHandles: ::minwindef::BOOL, dwCreationFlags: ::minwindef::DWORD, lpEnvironment: ::minwindef::LPVOID, lpCurrentDirectory: ::winnt::LPCSTR, lpStartupInfo: ::processthreadsapi::LPSTARTUPINFOA, lpProcessInformation: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* processthreadsapi.h:502:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateProcessAsUserW as CreateProcessAsUser; /* processthreadsapi.h:597:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateProcessAsUserW(hToken: ::winnt::HANDLE, lpApplicationName: ::winnt::LPCWSTR, lpCommandLine: ::winnt::LPWSTR, lpProcessAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpThreadAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, bInheritHandles: ::minwindef::BOOL, dwCreationFlags: ::minwindef::DWORD, lpEnvironment: ::minwindef::LPVOID, lpCurrentDirectory: ::winnt::LPCWSTR, lpStartupInfo: ::processthreadsapi::LPSTARTUPINFOW, lpProcessInformation: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* processthreadsapi.h:581:1 */
    pub fn CreateProcessW(lpApplicationName: ::winnt::LPCWSTR, lpCommandLine: ::winnt::LPWSTR, lpProcessAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpThreadAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, bInheritHandles: ::minwindef::BOOL, dwCreationFlags: ::minwindef::DWORD, lpEnvironment: ::minwindef::LPVOID, lpCurrentDirectory: ::winnt::LPCWSTR, lpStartupInfo: ::processthreadsapi::LPSTARTUPINFOW, lpProcessInformation: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* processthreadsapi.h:518:1 */
    pub fn CreateRemoteThread(hProcess: ::winnt::HANDLE, lpThreadAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, dwStackSize: ::basetsd::SIZE_T, lpStartAddress: ::minwinbase::LPTHREAD_START_ROUTINE, lpParameter: ::minwindef::LPVOID, dwCreationFlags: ::minwindef::DWORD, lpThreadId: ::minwindef::LPDWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:228:1 */
    pub fn CreateRemoteThreadEx(hProcess: ::winnt::HANDLE, lpThreadAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, dwStackSize: ::basetsd::SIZE_T, lpStartAddress: ::minwinbase::LPTHREAD_START_ROUTINE, lpParameter: ::minwindef::LPVOID, dwCreationFlags: ::minwindef::DWORD, lpAttributeList: ::processthreadsapi::LPPROC_THREAD_ATTRIBUTE_LIST, lpThreadId: ::minwindef::LPDWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:815:1 */
    pub fn CreateRestrictedToken(ExistingTokenHandle: ::winnt::HANDLE, Flags: ::minwindef::DWORD, DisableSidCount: ::minwindef::DWORD, SidsToDisable: ::winnt::PSID_AND_ATTRIBUTES, DeletePrivilegeCount: ::minwindef::DWORD, PrivilegesToDelete: ::winnt::PLUID_AND_ATTRIBUTES, RestrictedSidCount: ::minwindef::DWORD, SidsToRestrict: ::winnt::PSID_AND_ATTRIBUTES, NewTokenHandle: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:580:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateServiceW as CreateService; /* winsvc.h:1117:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateServiceA(hSCManager: ::winsvc::SC_HANDLE, lpServiceName: ::winnt::LPCSTR, lpDisplayName: ::winnt::LPCSTR, dwDesiredAccess: ::minwindef::DWORD, dwServiceType: ::minwindef::DWORD, dwStartType: ::minwindef::DWORD, dwErrorControl: ::minwindef::DWORD, lpBinaryPathName: ::winnt::LPCSTR, lpLoadOrderGroup: ::winnt::LPCSTR, lpdwTagId: ::minwindef::LPDWORD, lpDependencies: ::winnt::LPCSTR, lpServiceStartName: ::winnt::LPCSTR, lpPassword: ::winnt::LPCSTR) -> ::winsvc::SC_HANDLE; /* winsvc.h:1082:1 */
    pub fn CreateServiceW(hSCManager: ::winsvc::SC_HANDLE, lpServiceName: ::winnt::LPCWSTR, lpDisplayName: ::winnt::LPCWSTR, dwDesiredAccess: ::minwindef::DWORD, dwServiceType: ::minwindef::DWORD, dwStartType: ::minwindef::DWORD, dwErrorControl: ::minwindef::DWORD, lpBinaryPathName: ::winnt::LPCWSTR, lpLoadOrderGroup: ::winnt::LPCWSTR, lpdwTagId: ::minwindef::LPDWORD, lpDependencies: ::winnt::LPCWSTR, lpServiceStartName: ::winnt::LPCWSTR, lpPassword: ::winnt::LPCWSTR) -> ::winsvc::SC_HANDLE; /* winsvc.h:1101:1 */
    pub fn DebugActiveProcess(dwProcessId: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:136:1 */
    pub fn DebugActiveProcessStop(dwProcessId: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:144:1 */
    pub fn DebugBreak(); /* debugapi.h:70:1 */
    pub fn DecodeSystemPointer(Ptr: ::winnt::PVOID) -> ::winnt::PVOID; /* utilapiset.h:77:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DefineDosDeviceW as DefineDosDevice; /* fileapi.h:162:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DefineDosDeviceW(dwFlags: ::minwindef::DWORD, lpDeviceName: ::winnt::LPCWSTR, lpTargetPath: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* fileapi.h:154:1 */
    pub fn DeleteAce(pAcl: ::winnt::PACL, dwAceIndex: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:624:1 */
    pub fn DeleteBoundaryDescriptor(BoundaryDescriptor: ::winnt::HANDLE); /* namespaceapi.h:91:1 */
    pub fn DeleteService(hService: ::winsvc::SC_HANDLE) -> ::minwindef::BOOL; /* winsvc.h:1125:1 */
    pub fn DeleteSynchronizationBarrier(lpBarrier: ::synchapi::LPSYNCHRONIZATION_BARRIER) -> ::minwindef::BOOL; /* synchapi.h:893:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DeleteVolumeMountPointW as DeleteVolumeMountPoint; /* fileapi.h:208:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DeleteVolumeMountPointW(lpszVolumeMountPoint: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* fileapi.h:202:1 */
    pub fn DestroyPrivateObjectSecurity(ObjectDescriptor: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:633:1 */
    pub fn DeviceIoControl(hDevice: ::winnt::HANDLE, dwIoControlCode: ::minwindef::DWORD, lpInBuffer: ::minwindef::LPVOID, nInBufferSize: ::minwindef::DWORD, lpOutBuffer: ::minwindef::LPVOID, nOutBufferSize: ::minwindef::DWORD, lpBytesReturned: ::minwindef::LPDWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* ioapiset.h:116:1 */
    pub fn DisconnectNamedPipe(hNamedPipe: ::winnt::HANDLE) -> ::minwindef::BOOL; /* namedpipeapi.h:70:1 */
    pub fn DnsHostnameToComputerNameExW(Hostname: ::winnt::LPCWSTR, ComputerName: ::winnt::LPWSTR, nSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:602:1 */
    pub fn DuplicateToken(ExistingTokenHandle: ::winnt::HANDLE, ImpersonationLevel: ::winnt::SECURITY_IMPERSONATION_LEVEL, DuplicateTokenHandle: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:641:1 */
    pub fn DuplicateTokenEx(hExistingToken: ::winnt::HANDLE, dwDesiredAccess: ::minwindef::DWORD, lpTokenAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, ImpersonationLevel: ::winnt::SECURITY_IMPERSONATION_LEVEL, TokenType: ::winnt::TOKEN_TYPE, phNewToken: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:651:1 */
    pub fn EncodeSystemPointer(Ptr: ::winnt::PVOID) -> ::winnt::PVOID; /* utilapiset.h:68:1 */
    pub fn EnterSynchronizationBarrier(lpBarrier: ::synchapi::LPSYNCHRONIZATION_BARRIER, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:876:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumDependentServicesW as EnumDependentServices; /* winsvc.h:1156:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumDependentServicesW(hService: ::winsvc::SC_HANDLE, dwServiceState: ::minwindef::DWORD, lpServices: ::winsvc::LPENUM_SERVICE_STATUSW, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD, lpServicesReturned: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1146:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumServicesStatusExW as EnumServicesStatusEx; /* winsvc.h:1232:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumServicesStatusExW(hSCManager: ::winsvc::SC_HANDLE, InfoLevel: ::winsvc::SC_ENUM_TYPE, dwServiceType: ::minwindef::DWORD, dwServiceState: ::minwindef::DWORD, lpServices: ::minwindef::LPBYTE, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD, lpServicesReturned: ::minwindef::LPDWORD, lpResumeHandle: ::minwindef::LPDWORD, pszGroupName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winsvc.h:1218:1 */
    pub fn EnumSystemFirmwareTables(FirmwareTableProviderSignature: ::minwindef::DWORD, pFirmwareTableEnumBuffer: ::winnt::PVOID, BufferSize: ::minwindef::DWORD) -> ::minwindef::UINT; /* sysinfoapi.h:565:1 */
    pub fn EqualPrefixSid(pSid1: ::winnt::PSID, pSid2: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:664:1 */
    pub fn EqualSid(pSid1: ::winnt::PSID, pSid2: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:673:1 */
    pub fn EscapeCommFunction(hFile: ::winnt::HANDLE, dwFunc: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2029:1 */
    pub fn ExitProcess(uExitCode: ::minwindef::UINT); /* processthreadsapi.h:169:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ExpandEnvironmentStringsW as ExpandEnvironmentStrings; /* processenv.h:248:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ExpandEnvironmentStringsA(lpSrc: ::winnt::LPCSTR, lpDst: ::winnt::LPSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processenv.h:231:1 */
    pub fn ExpandEnvironmentStringsW(lpSrc: ::winnt::LPCWSTR, lpDst: ::winnt::LPWSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processenv.h:241:1 */
    pub fn FileTimeToLocalFileTime(lpFileTime: *const ::minwindef::FILETIME, lpLocalFileTime: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* fileapi.h:214:1 */
    pub fn FillConsoleOutputAttribute(hConsoleOutput: ::winnt::HANDLE, wAttribute: ::minwindef::WORD, nLength: ::minwindef::DWORD, dwWriteCoord: ::wincon::COORD, lpNumberOfAttrsWritten: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:522:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FillConsoleOutputCharacterW as FillConsoleOutputCharacter; /* wincon.h:514:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FillConsoleOutputCharacterA(hConsoleOutput: ::winnt::HANDLE, cCharacter: ::winnt::CHAR, nLength: ::minwindef::DWORD, dwWriteCoord: ::wincon::COORD, lpNumberOfCharsWritten: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:496:1 */
    pub fn FillConsoleOutputCharacterW(hConsoleOutput: ::winnt::HANDLE, cCharacter: ::winnt::WCHAR, nLength: ::minwindef::DWORD, dwWriteCoord: ::wincon::COORD, lpNumberOfCharsWritten: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:506:1 */
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
    pub fn FindFirstFreeAce(pAcl: ::winnt::PACL, pAce: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:682:1 */
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
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FindResourceExW as FindResourceEx; /* libloaderapi.h:182:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FindResourceExW(hModule: ::minwindef::HMODULE, lpType: ::winnt::LPCWSTR, lpName: ::winnt::LPCWSTR, wLanguage: ::minwindef::WORD) -> ::minwindef::HRSRC; /* libloaderapi.h:173:1 */
    pub fn FindVolumeClose(hFindVolume: ::winnt::HANDLE) -> ::minwindef::BOOL; /* fileapi.h:422:1 */
    pub fn FlushConsoleInputBuffer(hConsoleInput: ::winnt::HANDLE) -> ::minwindef::BOOL; /* wincon.h:645:1 */
    pub fn FlushInstructionCache(hProcess: ::winnt::HANDLE, lpBaseAddress: ::minwindef::LPCVOID, dwSize: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:916:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FoldStringW as FoldString; /* stringapiset.h:108:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
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
    pub fn FreeSid(pSid: ::winnt::PSID) -> ::winnt::PVOID; /* securitybaseapi.h:691:1 */
    pub fn GenerateConsoleCtrlEvent(dwCtrlEvent: ::minwindef::DWORD, dwProcessGroupId: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wincon.h:719:1 */
    pub fn GetACP() -> ::minwindef::UINT; /* winnls.h:1360:1 */
    pub fn GetAce(pAcl: ::winnt::PACL, dwAceIndex: ::minwindef::DWORD, pAce: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:699:1 */
    pub fn GetAclInformation(pAcl: ::winnt::PACL, pAclInformation: ::minwindef::LPVOID, nAclInformationLength: ::minwindef::DWORD, dwAclInformationClass: ::winnt::ACL_INFORMATION_CLASS) -> ::minwindef::BOOL; /* securitybaseapi.h:709:1 */
    pub fn GetCommConfig(hCommDev: ::winnt::HANDLE, lpCC: ::winbase::LPCOMMCONFIG, lpdwSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2038:1 */
    pub fn GetCommMask(hFile: ::winnt::HANDLE, lpEvtMask: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2047:1 */
    pub fn GetCommModemStatus(hFile: ::winnt::HANDLE, lpModemStat: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2063:1 */
    pub fn GetCommProperties(hFile: ::winnt::HANDLE, lpCommProp: ::winbase::LPCOMMPROP) -> ::minwindef::BOOL; /* winbase.h:2055:1 */
    pub fn GetCommState(hFile: ::winnt::HANDLE, lpDCB: ::winbase::LPDCB) -> ::minwindef::BOOL; /* winbase.h:2071:1 */
    pub fn GetCommTimeouts(hFile: ::winnt::HANDLE, lpCommTimeouts: ::winbase::LPCOMMTIMEOUTS) -> ::minwindef::BOOL; /* winbase.h:2079:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetComputerNameExW as GetComputerNameEx; /* sysinfoapi.h:377:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetComputerNameExA(NameType: ::sysinfoapi::COMPUTER_NAME_FORMAT, lpBuffer: ::winnt::LPSTR, nSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:360:1 */
    pub fn GetComputerNameExW(NameType: ::sysinfoapi::COMPUTER_NAME_FORMAT, lpBuffer: ::winnt::LPWSTR, nSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:370:1 */
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
    pub fn GetConsoleTitleW(lpConsoleTitle: ::winnt::LPWSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wincon.h:750:1 */
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
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileSecurityW as GetFileSecurity; /* securitybaseapi.h:730:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileSecurityW(lpFileName: ::winnt::LPCWSTR, RequestedInformation: ::winnt::SECURITY_INFORMATION, pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, nLength: ::minwindef::DWORD, lpnLengthNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:720:1 */
    pub fn GetFileSize(hFile: ::winnt::HANDLE, lpFileSizeHigh: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* fileapi.h:636:1 */
    pub fn GetFileSizeEx(hFile: ::winnt::HANDLE, lpFileSize: ::winnt::PLARGE_INTEGER) -> ::minwindef::BOOL; /* fileapi.h:645:1 */
    pub fn GetFileTime(hFile: ::winnt::HANDLE, lpCreationTime: ::minwindef::LPFILETIME, lpLastAccessTime: ::minwindef::LPFILETIME, lpLastWriteTime: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* fileapi.h:654:1 */
    pub fn GetFileType(hFile: ::winnt::HANDLE) -> ::minwindef::DWORD; /* fileapi.h:665:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileVersionInfoExW as GetFileVersionInfoEx; /* winver.h:157:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileVersionInfoExW(dwFlags: ::minwindef::DWORD, lpwstrFilename: ::winnt::LPCWSTR, dwHandle: ::minwindef::DWORD, dwLen: ::minwindef::DWORD, lpData: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winver.h:151:15 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileVersionInfoSizeExW as GetFileVersionInfoSizeEx; /* winver.h:141:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileVersionInfoSizeExW(dwFlags: ::minwindef::DWORD, lpwstrFilename: ::winnt::LPCWSTR, lpdwHandle: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winver.h:139:16 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFirmwareEnvironmentVariableW as GetFirmwareEnvironmentVariable; /* winbase.h:3554:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFirmwareEnvironmentVariableW(lpName: ::winnt::LPCWSTR, lpGuid: ::winnt::LPCWSTR, pBuffer: ::winnt::PVOID, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winbase.h:3547:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFullPathNameW as GetFullPathName; /* fileapi.h:724:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFullPathNameA(lpFileName: ::winnt::LPCSTR, nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPSTR, lpFilePart: *mut *mut ::libc::c_schar) -> ::minwindef::DWORD; /* fileapi.h:705:1 */
    pub fn GetFullPathNameW(lpFileName: ::winnt::LPCWSTR, nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPWSTR, lpFilePart: *mut *mut ::libc::c_ushort) -> ::minwindef::DWORD; /* fileapi.h:716:1 */
    pub fn GetHandleInformation(hObject: ::winnt::HANDLE, lpdwFlags: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* handleapi.h:79:1 */
    pub fn GetKernelObjectSecurity(Handle: ::winnt::HANDLE, RequestedInformation: ::winnt::SECURITY_INFORMATION, pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, nLength: ::minwindef::DWORD, lpnLengthNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:736:1 */
    pub fn GetLargePageMinimum() -> ::basetsd::SIZE_T; /* memoryapi.h:339:1 */
    pub fn GetLargestConsoleWindowSize(hConsoleOutput: ::winnt::HANDLE) -> ::wincon::COORD; /* wincon.h:558:1 */
    pub fn GetLengthSid(pSid: ::winnt::PSID) -> ::minwindef::DWORD; /* securitybaseapi.h:750:1 */
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
    pub fn GetLogicalDriveStringsW(nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* fileapi.h:740:1 */
    pub fn GetLogicalDrives() -> ::minwindef::DWORD; /* fileapi.h:732:1 */
    pub fn GetLogicalProcessorInformation(Buffer: ::winnt::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION, ReturnedLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:455:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetLongPathNameW as GetLongPathName; /* fileapi.h:771:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetLongPathNameA(lpszShortPath: ::winnt::LPCSTR, lpszLongPath: ::winnt::LPSTR, cchBuffer: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:754:1 */
    pub fn GetLongPathNameW(lpszShortPath: ::winnt::LPCWSTR, lpszLongPath: ::winnt::LPWSTR, cchBuffer: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:764:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetModuleFileNameW as GetModuleFileName; /* libloaderapi.h:266:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetModuleFileNameA(hModule: ::minwindef::HMODULE, lpFilename: ::winnt::LPSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* libloaderapi.h:248:1 */
    pub fn GetModuleFileNameW(hModule: ::minwindef::HMODULE, lpFilename: ::winnt::LPWSTR, nSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* libloaderapi.h:259:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetModuleHandleW as GetModuleHandle; /* libloaderapi.h:290:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetModuleHandleExW as GetModuleHandleEx; /* libloaderapi.h:343:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetModuleHandleExA(dwFlags: ::minwindef::DWORD, lpModuleName: ::winnt::LPCSTR, phModule: *mut *mut ::minwindef::HINSTANCE__) -> ::minwindef::BOOL; /* libloaderapi.h:327:1 */
    pub fn GetModuleHandleExW(dwFlags: ::minwindef::DWORD, lpModuleName: ::winnt::LPCWSTR, phModule: *mut *mut ::minwindef::HINSTANCE__) -> ::minwindef::BOOL; /* libloaderapi.h:336:1 */
    pub fn GetModuleHandleW(lpModuleName: ::winnt::LPCWSTR) -> ::minwindef::HMODULE; /* libloaderapi.h:285:1 */
    pub fn GetNLSVersion(Function: ::winnls::NLS_FUNCTION, Locale: ::winnt::LCID, lpVersionInformation: ::winnls::LPNLSVERSIONINFO) -> ::minwindef::BOOL; /* winnls.h:1875:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetNamedPipeClientComputerNameW as GetNamedPipeClientComputerName; /* namedpipeapi.h:161:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetNumaHighestNodeNumber(HighestNodeNumber: ::minwindef::PULONG) -> ::minwindef::BOOL; /* systemtopologyapi.h:43:1 */
    pub fn GetNumberOfConsoleInputEvents(hConsoleInput: ::winnt::HANDLE, lpNumberOfEvents: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* consoleapi.h:77:1 */
    pub fn GetOEMCP() -> ::minwindef::UINT; /* winnls.h:1365:1 */
    pub fn GetOverlappedResult(hFile: ::winnt::HANDLE, lpOverlapped: ::minwinbase::LPOVERLAPPED, lpNumberOfBytesTransferred: ::minwindef::LPDWORD, bWait: ::minwindef::BOOL) -> ::minwindef::BOOL; /* ioapiset.h:52:1 */
    pub fn GetPhysicallyInstalledSystemMemory(TotalMemoryInKilobytes: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* sysinfoapi.h:613:1 */
    pub fn GetPriorityClass(hProcess: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:669:1 */
    pub fn GetPrivateObjectSecurity(ObjectDescriptor: ::winnt::PSECURITY_DESCRIPTOR, SecurityInformation: ::winnt::SECURITY_INFORMATION, ResultantDescriptor: ::winnt::PSECURITY_DESCRIPTOR, DescriptorLength: ::minwindef::DWORD, ReturnLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:759:1 */
    pub fn GetProcessHeaps(NumberOfHeaps: ::minwindef::DWORD, ProcessHeaps: ::winnt::PHANDLE) -> ::minwindef::DWORD; /* heapapi.h:204:1 */
    pub fn GetProcessTimes(hProcess: ::winnt::HANDLE, lpCreationTime: ::minwindef::LPFILETIME, lpExitTime: ::minwindef::LPFILETIME, lpKernelTime: ::minwindef::LPFILETIME, lpUserTime: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* processthreadsapi.h:126:1 */
    pub fn GetProcessVersion(ProcessId: ::minwindef::DWORD) -> ::minwindef::DWORD; /* processthreadsapi.h:551:1 */
    pub fn GetProcessWorkingSetSizeEx(hProcess: ::winnt::HANDLE, lpMinimumWorkingSetSize: ::basetsd::PSIZE_T, lpMaximumWorkingSetSize: ::basetsd::PSIZE_T, Flags: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:348:1 */
    pub fn GetQueuedCompletionStatus(CompletionPort: ::winnt::HANDLE, lpNumberOfBytesTransferred: ::minwindef::LPDWORD, lpCompletionKey: ::basetsd::PULONG_PTR, lpOverlapped: *mut *mut ::minwinbase::OVERLAPPED, dwMilliseconds: ::minwindef::DWORD) -> ::minwindef::BOOL; /* ioapiset.h:75:1 */
    pub fn GetSecurityDescriptorControl(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, pControl: ::winnt::PSECURITY_DESCRIPTOR_CONTROL, lpdwRevision: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:771:1 */
    pub fn GetSecurityDescriptorDacl(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, lpbDaclPresent: ::minwindef::LPBOOL, pDacl: *mut *mut ::winnt::ACL, lpbDaclDefaulted: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:781:1 */
    pub fn GetSecurityDescriptorGroup(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, pGroup: *mut *mut ::libc::c_void, lpbGroupDefaulted: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:792:1 */
    pub fn GetSecurityDescriptorLength(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::DWORD; /* securitybaseapi.h:802:1 */
    pub fn GetSecurityDescriptorOwner(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, pOwner: *mut *mut ::libc::c_void, lpbOwnerDefaulted: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:810:1 */
    pub fn GetSecurityDescriptorRMControl(SecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, RMControl: ::minwindef::PUCHAR) -> ::minwindef::DWORD; /* securitybaseapi.h:820:1 */
    pub fn GetSecurityDescriptorSacl(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, lpbSaclPresent: ::minwindef::LPBOOL, pSacl: *mut *mut ::winnt::ACL, lpbSaclDefaulted: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:829:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetShortPathNameW as GetShortPathName; /* fileapi.h:788:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetShortPathNameW(lpszLongPath: ::winnt::LPCWSTR, lpszShortPath: ::winnt::LPWSTR, cchBuffer: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:780:1 */
    pub fn GetSidIdentifierAuthority(pSid: ::winnt::PSID) -> ::winnt::PSID_IDENTIFIER_AUTHORITY; /* securitybaseapi.h:840:1 */
    pub fn GetSidLengthRequired(nSubAuthorityCount: ::minwindef::UCHAR) -> ::minwindef::DWORD; /* securitybaseapi.h:848:1 */
    pub fn GetSidSubAuthority(pSid: ::winnt::PSID, nSubAuthority: ::minwindef::DWORD) -> ::minwindef::PDWORD; /* securitybaseapi.h:856:1 */
    pub fn GetSidSubAuthorityCount(pSid: ::winnt::PSID) -> ::minwindef::PUCHAR; /* securitybaseapi.h:865:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetStartupInfoW as GetStartupInfo; /* processthreadsapi.h:564:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetStartupInfoW(lpStartupInfo: ::processthreadsapi::LPSTARTUPINFOW); /* processthreadsapi.h:559:1 */
    pub fn GetStdHandle(nStdHandle: ::minwindef::DWORD) -> ::winnt::HANDLE; /* processenv.h:108:1 */
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
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTempFileNameW as GetTempFileName; /* fileapi.h:803:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTempFileNameW(lpPathName: ::winnt::LPCWSTR, lpPrefixString: ::winnt::LPCWSTR, uUnique: ::minwindef::UINT, lpTempFileName: ::winnt::LPWSTR) -> ::minwindef::UINT; /* fileapi.h:794:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTempPathW as GetTempPath; /* fileapi.h:1213:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTempPathW(nBufferLength: ::minwindef::DWORD, lpBuffer: ::winnt::LPWSTR) -> ::minwindef::DWORD; /* fileapi.h:1206:1 */
    pub fn GetThreadContext(hThread: ::winnt::HANDLE, lpContext: ::minwinbase::LPCONTEXT) -> ::minwindef::BOOL; /* processthreadsapi.h:859:1 */
    pub fn GetThreadLocale() -> ::winnt::LCID; /* winnls.h:1963:1 */
    pub fn GetThreadPriorityBoost(hThread: ::winnt::HANDLE, pDisablePriorityBoost: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:312:1 */
    pub fn GetThreadTimes(hThread: ::winnt::HANDLE, lpCreationTime: ::minwindef::LPFILETIME, lpExitTime: ::minwindef::LPFILETIME, lpKernelTime: ::minwindef::LPFILETIME, lpUserTime: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* processthreadsapi.h:926:1 */
    pub fn GetTickCount() -> ::minwindef::DWORD; /* sysinfoapi.h:203:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTimeFormatW as GetTimeFormat; /* datetimeapi.h:109:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTimeFormatA(Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD, lpTime: *const ::minwinbase::SYSTEMTIME, lpFormat: ::winnt::LPCSTR, lpTimeStr: ::winnt::LPSTR, cchTime: ::libc::c_int) -> ::libc::c_int; /* datetimeapi.h:86:1 */
    pub fn GetTimeFormatW(Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD, lpTime: *const ::minwinbase::SYSTEMTIME, lpFormat: ::winnt::LPCWSTR, lpTimeStr: ::winnt::LPWSTR, cchTime: ::libc::c_int) -> ::libc::c_int; /* datetimeapi.h:99:1 */
    pub fn GetTokenInformation(TokenHandle: ::winnt::HANDLE, TokenInformationClass: ::winnt::TOKEN_INFORMATION_CLASS, TokenInformation: ::minwindef::LPVOID, TokenInformationLength: ::minwindef::DWORD, ReturnLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:873:1 */
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
    pub fn GlobalMemoryStatusEx(lpBuffer: ::sysinfoapi::LPMEMORYSTATUSEX) -> ::minwindef::BOOL; /* sysinfoapi.h:130:1 */
    pub fn HeapCompact(hHeap: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD) -> ::basetsd::SIZE_T; /* heapapi.h:159:1 */
    pub fn HeapCreate(flOptions: ::minwindef::DWORD, dwInitialSize: ::basetsd::SIZE_T, dwMaximumSize: ::basetsd::SIZE_T) -> ::winnt::HANDLE; /* heapapi.h:70:1 */
    pub fn HeapDestroy(hHeap: ::winnt::HANDLE) -> ::minwindef::BOOL; /* heapapi.h:80:1 */
    pub fn HeapLock(hHeap: ::winnt::HANDLE) -> ::minwindef::BOOL; /* heapapi.h:213:1 */
    pub fn HeapQueryInformation(HeapHandle: ::winnt::HANDLE, HeapInformationClass: ::winnt::HEAP_INFORMATION_CLASS, HeapInformation: ::winnt::PVOID, HeapInformationLength: ::basetsd::SIZE_T, ReturnLength: ::basetsd::PSIZE_T) -> ::minwindef::BOOL; /* heapapi.h:249:1 */
    pub fn HeapSetInformation(HeapHandle: ::winnt::HANDLE, HeapInformationClass: ::winnt::HEAP_INFORMATION_CLASS, HeapInformation: ::winnt::PVOID, HeapInformationLength: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* heapapi.h:238:1 */
    pub fn HeapUnlock(hHeap: ::winnt::HANDLE) -> ::minwindef::BOOL; /* heapapi.h:221:1 */
    pub fn HeapValidate(hHeap: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD, lpMem: ::minwindef::LPCVOID) -> ::minwindef::BOOL; /* heapapi.h:149:1 */
    pub fn HeapWalk(hHeap: ::winnt::HANDLE, lpEntry: ::minwinbase::LPPROCESS_HEAP_ENTRY) -> ::minwindef::BOOL; /* heapapi.h:229:1 */
    pub fn ImpersonateAnonymousToken(ThreadHandle: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:901:1 */
    pub fn ImpersonateLoggedOnUser(hToken: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:910:1 */
    pub fn ImpersonateNamedPipeClient(hNamedPipe: ::winnt::HANDLE) -> ::minwindef::BOOL; /* namedpipeapi.h:175:1 */
    pub fn ImpersonateSelf(ImpersonationLevel: ::winnt::SECURITY_IMPERSONATION_LEVEL) -> ::minwindef::BOOL; /* securitybaseapi.h:919:1 */
    pub fn InitializeAcl(pAcl: ::winnt::PACL, nAclLength: ::minwindef::DWORD, dwAclRevision: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:927:1 */
    pub fn InitializeCriticalSection(lpCriticalSection: ::minwinbase::LPCRITICAL_SECTION); /* synchapi.h:162:1 */
    pub fn InitializeCriticalSectionAndSpinCount(lpCriticalSection: ::minwinbase::LPCRITICAL_SECTION, dwSpinCount: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:203:1 */
    pub fn InitializeSecurityDescriptor(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, dwRevision: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:937:1 */
    pub fn InitializeSid(Sid: ::winnt::PSID, pIdentifierAuthority: ::winnt::PSID_IDENTIFIER_AUTHORITY, nSubAuthorityCount: ::minwindef::BYTE) -> ::minwindef::BOOL; /* securitybaseapi.h:946:1 */
    pub fn InitializeSynchronizationBarrier(lpBarrier: ::synchapi::LPSYNCHRONIZATION_BARRIER, lTotalThreads: ::winnt::LONG, lSpinCount: ::winnt::LONG) -> ::minwindef::BOOL; /* synchapi.h:884:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::InitiateShutdownW as InitiateShutdown; /* winreg.h:1406:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn InitiateShutdownW(lpMachineName: ::winnt::LPWSTR, lpMessage: ::winnt::LPWSTR, dwGracePeriod: ::minwindef::DWORD, dwShutdownFlags: ::minwindef::DWORD, dwReason: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winreg.h:1398:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::InitiateSystemShutdownExW as InitiateSystemShutdownEx; /* winreg.h:1364:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn InitiateSystemShutdownExW(lpMachineName: ::winnt::LPWSTR, lpMessage: ::winnt::LPWSTR, dwTimeout: ::minwindef::DWORD, bForceAppsClosed: ::minwindef::BOOL, bRebootAfterShutdown: ::minwindef::BOOL, dwReason: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winreg.h:1355:1 */
    pub fn InstallELAMCertificateInfo(ELAMFile: ::winnt::HANDLE) -> ::minwindef::BOOL; /* sysinfoapi.h:647:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsCharAlphaW as IsCharAlpha; /* winuser.h:5536:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsCharAlphaNumericW as IsCharAlphaNumeric; /* winuser.h:5552:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn IsCharAlphaNumericW(ch: ::winnt::WCHAR) -> ::minwindef::BOOL; /* winuser.h:5549:1 */
    pub fn IsCharAlphaW(ch: ::winnt::WCHAR) -> ::minwindef::BOOL; /* winuser.h:5533:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsCharLowerW as IsCharLower; /* winuser.h:5584:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn IsCharLowerW(ch: ::winnt::WCHAR) -> ::minwindef::BOOL; /* winuser.h:5581:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::IsCharUpperW as IsCharUpper; /* winuser.h:5568:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn IsCharUpperW(ch: ::winnt::WCHAR) -> ::minwindef::BOOL; /* winuser.h:5565:1 */
    pub fn IsDBCSLeadByte(TestChar: ::minwindef::BYTE) -> ::minwindef::BOOL; /* winnls.h:1630:1 */
    pub fn IsDBCSLeadByteEx(CodePage: ::minwindef::UINT, TestChar: ::minwindef::BYTE) -> ::minwindef::BOOL; /* winnls.h:1637:1 */
    pub fn IsNLSDefinedString(Function: ::winnls::NLS_FUNCTION, dwFlags: ::minwindef::DWORD, lpVersionInformation: ::winnls::LPNLSVERSIONINFO, lpString: ::winnt::LPCWSTR, cchStr: ::winnt::INT) -> ::minwindef::BOOL; /* winnls.h:1883:1 */
    pub fn IsTokenRestricted(TokenHandle: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:956:1 */
    pub fn IsValidAcl(pAcl: ::winnt::PACL) -> ::minwindef::BOOL; /* securitybaseapi.h:964:1 */
    pub fn IsValidLocale(Locale: ::winnt::LCID, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1894:1 */
    pub fn IsValidSecurityDescriptor(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* securitybaseapi.h:972:1 */
    pub fn IsValidSid(pSid: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:980:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LCMapStringW as LCMapString; /* winnls.h:1483:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LCMapStringA(Locale: ::winnt::LCID, dwMapFlags: ::minwindef::DWORD, lpSrcStr: ::winnt::LPCSTR, cchSrc: ::libc::c_int, lpDestStr: ::winnt::LPSTR, cchDest: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1489:1 */
    pub fn LCMapStringW(Locale: ::winnt::LCID, dwMapFlags: ::minwindef::DWORD, lpSrcStr: ::winnt::LPCWSTR, cchSrc: ::libc::c_int, lpDestStr: ::winnt::LPWSTR, cchDest: ::libc::c_int) -> ::libc::c_int; /* winnls.h:1475:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadLibraryExW as LoadLibraryEx; /* libloaderapi.h:394:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadLibraryExA(lpLibFileName: ::winnt::LPCSTR, hFile: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD) -> ::minwindef::HMODULE; /* libloaderapi.h:377:1 */
    pub fn LoadLibraryExW(lpLibFileName: ::winnt::LPCWSTR, hFile: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD) -> ::minwindef::HMODULE; /* libloaderapi.h:387:1 */
    pub fn LoadResource(hModule: ::minwindef::HMODULE, hResInfo: ::minwindef::HRSRC) -> ::minwindef::HGLOBAL; /* libloaderapi.h:417:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LoadStringW as LoadString; /* libloaderapi.h:453:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LoadStringA(hInstance: ::minwindef::HINSTANCE, uID: ::minwindef::UINT, lpBuffer: ::winnt::LPSTR, cchBufferMax: ::libc::c_int) -> ::libc::c_int; /* libloaderapi.h:435:1 */
    pub fn LoadStringW(hInstance: ::minwindef::HINSTANCE, uID: ::minwindef::UINT, lpBuffer: ::winnt::LPWSTR, cchBufferMax: ::libc::c_int) -> ::libc::c_int; /* libloaderapi.h:445:1 */
    pub fn LocalFileTimeToFileTime(lpLocalFileTime: *const ::minwindef::FILETIME, lpFileTime: ::minwindef::LPFILETIME) -> ::minwindef::BOOL; /* fileapi.h:862:1 */
    pub fn LockFile(hFile: ::winnt::HANDLE, dwFileOffsetLow: ::minwindef::DWORD, dwFileOffsetHigh: ::minwindef::DWORD, nNumberOfBytesToLockLow: ::minwindef::DWORD, nNumberOfBytesToLockHigh: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:871:1 */
    pub fn LockResource(hResData: ::minwindef::HGLOBAL) -> ::minwindef::LPVOID; /* libloaderapi.h:470:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupAccountNameW as LookupAccountName; /* winbase.h:6512:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupAccountNameW(lpSystemName: ::winnt::LPCWSTR, lpAccountName: ::winnt::LPCWSTR, Sid: ::winnt::PSID, cbSid: ::minwindef::LPDWORD, ReferencedDomainName: ::winnt::LPWSTR, cchReferencedDomainName: ::minwindef::LPDWORD, peUse: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6502:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupAccountSidW as LookupAccountSid; /* winbase.h:6482:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupAccountSidW(lpSystemName: ::winnt::LPCWSTR, Sid: ::winnt::PSID, Name: ::winnt::LPWSTR, cchName: ::minwindef::LPDWORD, ReferencedDomainName: ::winnt::LPWSTR, cchReferencedDomainName: ::minwindef::LPDWORD, peUse: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6472:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupPrivilegeDisplayNameW as LookupPrivilegeDisplayName; /* winbase.h:6666:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupPrivilegeDisplayNameW(lpSystemName: ::winnt::LPCWSTR, lpName: ::winnt::LPCWSTR, lpDisplayName: ::winnt::LPWSTR, cchDisplayName: ::minwindef::LPDWORD, lpLanguageId: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6658:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupPrivilegeNameW as LookupPrivilegeName; /* winbase.h:6640:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupPrivilegeNameW(lpSystemName: ::winnt::LPCWSTR, lpLuid: ::winnt::PLUID, lpName: ::winnt::LPWSTR, cchName: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6633:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupPrivilegeValueW as LookupPrivilegeValue; /* winbase.h:6616:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupPrivilegeValueW(lpSystemName: ::winnt::LPCWSTR, lpName: ::winnt::LPCWSTR, lpLuid: ::winnt::PLUID) -> ::minwindef::BOOL; /* winbase.h:6610:1 */
    pub fn MakeAbsoluteSD(pSelfRelativeSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, pAbsoluteSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, lpdwAbsoluteSecurityDescriptorSize: ::minwindef::LPDWORD, pDacl: ::winnt::PACL, lpdwDaclSize: ::minwindef::LPDWORD, pSacl: ::winnt::PACL, lpdwSaclSize: ::minwindef::LPDWORD, pOwner: ::winnt::PSID, lpdwOwnerSize: ::minwindef::LPDWORD, pPrimaryGroup: ::winnt::PSID, lpdwPrimaryGroupSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:1003:1 */
    pub fn MakeSelfRelativeSD(pAbsoluteSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, pSelfRelativeSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, lpdwBufferLength: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:1022:1 */
    pub fn MapGenericMask(AccessMask: ::minwindef::PDWORD, GenericMapping: ::winnt::PGENERIC_MAPPING); /* securitybaseapi.h:1032:1 */
    pub fn MapViewOfFile(hFileMappingObject: ::winnt::HANDLE, dwDesiredAccess: ::minwindef::DWORD, dwFileOffsetHigh: ::minwindef::DWORD, dwFileOffsetLow: ::minwindef::DWORD, dwNumberOfBytesToMap: ::basetsd::SIZE_T) -> ::minwindef::LPVOID; /* memoryapi.h:276:1 */
    pub fn MapViewOfFileEx(hFileMappingObject: ::winnt::HANDLE, dwDesiredAccess: ::minwindef::DWORD, dwFileOffsetHigh: ::minwindef::DWORD, dwFileOffsetLow: ::minwindef::DWORD, dwNumberOfBytesToMap: ::basetsd::SIZE_T, lpBaseAddress: ::minwindef::LPVOID) -> ::minwindef::LPVOID; /* memoryapi.h:289:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectCloseAuditAlarmW as ObjectCloseAuditAlarm; /* securitybaseapi.h:1049:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectCloseAuditAlarmW(SubsystemName: ::winnt::LPCWSTR, HandleId: ::minwindef::LPVOID, GenerateOnClose: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1041:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectDeleteAuditAlarmW as ObjectDeleteAuditAlarm; /* securitybaseapi.h:1063:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectDeleteAuditAlarmW(SubsystemName: ::winnt::LPCWSTR, HandleId: ::minwindef::LPVOID, GenerateOnClose: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1055:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectOpenAuditAlarmW as ObjectOpenAuditAlarm; /* securitybaseapi.h:1086:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectOpenAuditAlarmW(SubsystemName: ::winnt::LPCWSTR, HandleId: ::minwindef::LPVOID, ObjectTypeName: ::winnt::LPWSTR, ObjectName: ::winnt::LPWSTR, pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, ClientToken: ::winnt::HANDLE, DesiredAccess: ::minwindef::DWORD, GrantedAccess: ::minwindef::DWORD, Privileges: ::winnt::PPRIVILEGE_SET, ObjectCreation: ::minwindef::BOOL, AccessGranted: ::minwindef::BOOL, GenerateOnClose: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1069:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectPrivilegeAuditAlarmW as ObjectPrivilegeAuditAlarm; /* securitybaseapi.h:1103:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectPrivilegeAuditAlarmW(SubsystemName: ::winnt::LPCWSTR, HandleId: ::minwindef::LPVOID, ClientToken: ::winnt::HANDLE, DesiredAccess: ::minwindef::DWORD, Privileges: ::winnt::PPRIVILEGE_SET, AccessGranted: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1092:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenFileMappingW as OpenFileMapping; /* memoryapi.h:269:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenFileMappingW(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, lpName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* memoryapi.h:261:1 */
    pub fn OpenPrivateNamespaceW(lpBoundaryDescriptor: ::minwindef::LPVOID, lpAliasPrefix: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* namespaceapi.h:55:1 */
    pub fn OpenProcess(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, dwProcessId: ::minwindef::DWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:938:1 */
    pub fn OpenProcessToken(ProcessHandle: ::winnt::HANDLE, DesiredAccess: ::minwindef::DWORD, TokenHandle: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* processthreadsapi.h:622:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenSCManagerW as OpenSCManager; /* winsvc.h:1326:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenSCManagerA(lpMachineName: ::winnt::LPCSTR, lpDatabaseName: ::winnt::LPCSTR, dwDesiredAccess: ::minwindef::DWORD) -> ::winsvc::SC_HANDLE; /* winsvc.h:1311:1 */
    pub fn OpenSCManagerW(lpMachineName: ::winnt::LPCWSTR, lpDatabaseName: ::winnt::LPCWSTR, dwDesiredAccess: ::minwindef::DWORD) -> ::winsvc::SC_HANDLE; /* winsvc.h:1320:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenServiceW as OpenService; /* winsvc.h:1350:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenServiceA(hSCManager: ::winsvc::SC_HANDLE, lpServiceName: ::winnt::LPCSTR, dwDesiredAccess: ::minwindef::DWORD) -> ::winsvc::SC_HANDLE; /* winsvc.h:1335:1 */
    pub fn OpenServiceW(hSCManager: ::winsvc::SC_HANDLE, lpServiceName: ::winnt::LPCWSTR, dwDesiredAccess: ::minwindef::DWORD) -> ::winsvc::SC_HANDLE; /* winsvc.h:1344:1 */
    pub fn OpenThread(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, dwThreadId: ::minwindef::DWORD) -> ::winnt::HANDLE; /* processthreadsapi.h:273:1 */
    pub fn OpenThreadToken(ThreadHandle: ::winnt::HANDLE, DesiredAccess: ::minwindef::DWORD, OpenAsSelf: ::minwindef::BOOL, TokenHandle: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* processthreadsapi.h:632:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PeekConsoleInputW as PeekConsoleInput; /* wincon.h:340:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PeekConsoleInputA(hConsoleInput: ::winnt::HANDLE, lpBuffer: ::wincon::PINPUT_RECORD, nLength: ::minwindef::DWORD, lpNumberOfEventsRead: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* consoleapi.h:86:1 */
    pub fn PeekConsoleInputW(hConsoleInput: ::winnt::HANDLE, lpBuffer: ::wincon::PINPUT_RECORD, nLength: ::minwindef::DWORD, lpNumberOfEventsRead: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* wincon.h:333:1 */
    pub fn PeekNamedPipe(hNamedPipe: ::winnt::HANDLE, lpBuffer: ::minwindef::LPVOID, nBufferSize: ::minwindef::DWORD, lpBytesRead: ::minwindef::LPDWORD, lpTotalBytesAvail: ::minwindef::LPDWORD, lpBytesLeftThisMessage: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:89:1 */
    pub fn PostQueuedCompletionStatus(CompletionPort: ::winnt::HANDLE, dwNumberOfBytesTransferred: ::minwindef::DWORD, dwCompletionKey: ::basetsd::ULONG_PTR, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* ioapiset.h:105:1 */
    pub fn PrivilegeCheck(ClientToken: ::winnt::HANDLE, RequiredPrivileges: ::winnt::PPRIVILEGE_SET, pfResult: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1109:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PrivilegedServiceAuditAlarmW as PrivilegedServiceAuditAlarm; /* securitybaseapi.h:1129:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PrivilegedServiceAuditAlarmW(SubsystemName: ::winnt::LPCWSTR, ServiceName: ::winnt::LPCWSTR, ClientToken: ::winnt::HANDLE, Privileges: ::winnt::PPRIVILEGE_SET, AccessGranted: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1119:1 */
    pub fn ProcessIdToSessionId(dwProcessId: ::minwindef::DWORD, pSessionId: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* processthreadsapi.h:677:1 */
    pub fn PurgeComm(hFile: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2087:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::QueryDosDeviceW as QueryDosDevice; /* fileapi.h:918:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn QueryDosDeviceW(lpDeviceName: ::winnt::LPCWSTR, lpTargetPath: ::winnt::LPWSTR, ucchMax: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:910:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::QueryServiceConfigW as QueryServiceConfig; /* winsvc.h:1378:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::QueryServiceConfig2W as QueryServiceConfig2; /* winsvc.h:1422:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn QueryServiceConfig2A(hService: ::winsvc::SC_HANDLE, dwInfoLevel: ::minwindef::DWORD, lpBuffer: ::minwindef::LPBYTE, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1394:1 */
    pub fn QueryServiceConfig2W(hService: ::winsvc::SC_HANDLE, dwInfoLevel: ::minwindef::DWORD, lpBuffer: ::minwindef::LPBYTE, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1413:1 */
    pub fn QueryServiceConfigA(hService: ::winsvc::SC_HANDLE, lpServiceConfig: ::winsvc::LPQUERY_SERVICE_CONFIGA, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1359:1 */
    pub fn QueryServiceConfigW(hService: ::winsvc::SC_HANDLE, lpServiceConfig: ::winsvc::LPQUERY_SERVICE_CONFIGW, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1370:1 */
    pub fn QueryServiceObjectSecurity(hService: ::winsvc::SC_HANDLE, dwSecurityInformation: ::winnt::SECURITY_INFORMATION, lpSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1459:1 */
    pub fn QueryServiceStatus(hService: ::winsvc::SC_HANDLE, lpServiceStatus: ::winsvc::LPSERVICE_STATUS) -> ::minwindef::BOOL; /* winsvc.h:1472:1 */
    pub fn QueryServiceStatusEx(hService: ::winsvc::SC_HANDLE, InfoLevel: ::winsvc::SC_STATUS_TYPE, lpBuffer: ::minwindef::LPBYTE, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1481:1 */
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
    pub fn RegCloseKey(hKey: ::minwindef::HKEY) -> ::winreg::LSTATUS; /* winreg.h:206:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegCreateKeyExW as RegCreateKeyEx; /* winreg.h:353:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegCreateKeyExA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, Reserved: ::minwindef::DWORD, lpClass: ::winnt::LPSTR, dwOptions: ::minwindef::DWORD, samDesired: ::winreg::REGSAM, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, phkResult: ::minwindef::PHKEY, lpdwDisposition: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:325:1 */
    pub fn RegCreateKeyExW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, Reserved: ::minwindef::DWORD, lpClass: ::winnt::LPWSTR, dwOptions: ::minwindef::DWORD, samDesired: ::winreg::REGSAM, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, phkResult: ::minwindef::PHKEY, lpdwDisposition: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:340:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegDeleteKeyExW as RegDeleteKeyEx; /* winreg.h:437:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegDeleteKeyExA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, samDesired: ::winreg::REGSAM, Reserved: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:419:1 */
    pub fn RegDeleteKeyExW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, samDesired: ::winreg::REGSAM, Reserved: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:429:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegDeleteValueW as RegDeleteValue; /* winreg.h:509:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegDeleteValueA(hKey: ::minwindef::HKEY, lpValueName: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:495:1 */
    pub fn RegDeleteValueW(hKey: ::minwindef::HKEY, lpValueName: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:503:1 */
    pub fn RegDisablePredefinedCacheEx() -> ::winreg::LSTATUS; /* winreg.h:249:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegEnumKeyExW as RegEnumKeyEx; /* winreg.h:567:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegEnumKeyExA(hKey: ::minwindef::HKEY, dwIndex: ::minwindef::DWORD, lpName: ::winnt::LPSTR, lpcchName: ::minwindef::LPDWORD, lpReserved: ::minwindef::LPDWORD, lpClass: ::winnt::LPSTR, lpcchClass: ::minwindef::LPDWORD, lpftLastWriteTime: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:541:1 */
    pub fn RegEnumKeyExW(hKey: ::minwindef::HKEY, dwIndex: ::minwindef::DWORD, lpName: ::winnt::LPWSTR, lpcchName: ::minwindef::LPDWORD, lpReserved: ::minwindef::LPDWORD, lpClass: ::winnt::LPWSTR, lpcchClass: ::minwindef::LPDWORD, lpftLastWriteTime: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:555:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegEnumValueW as RegEnumValue; /* winreg.h:601:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegEnumValueA(hKey: ::minwindef::HKEY, dwIndex: ::minwindef::DWORD, lpValueName: ::winnt::LPSTR, lpcchValueName: ::minwindef::LPDWORD, lpReserved: ::minwindef::LPDWORD, lpType: ::minwindef::LPDWORD, lpData: ::minwindef::LPBYTE, lpcbData: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:575:1 */
    pub fn RegEnumValueW(hKey: ::minwindef::HKEY, dwIndex: ::minwindef::DWORD, lpValueName: ::winnt::LPWSTR, lpcchValueName: ::minwindef::LPDWORD, lpReserved: ::minwindef::LPDWORD, lpType: ::minwindef::LPDWORD, lpData: ::minwindef::LPBYTE, lpcbData: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:589:1 */
    pub fn RegFlushKey(hKey: ::minwindef::HKEY) -> ::winreg::LSTATUS; /* winreg.h:609:1 */
    pub fn RegGetKeySecurity(hKey: ::minwindef::HKEY, SecurityInformation: ::winnt::SECURITY_INFORMATION, pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, lpcbSecurityDescriptor: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:617:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegLoadKeyW as RegLoadKey; /* winreg.h:644:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegLoadKeyA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, lpFile: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:628:1 */
    pub fn RegLoadKeyW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, lpFile: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:637:1 */
    pub fn RegNotifyChangeKeyValue(hKey: ::minwindef::HKEY, bWatchSubtree: ::minwindef::BOOL, dwNotifyFilter: ::minwindef::DWORD, hEvent: ::winnt::HANDLE, fAsynchronous: ::minwindef::BOOL) -> ::winreg::LSTATUS; /* winreg.h:652:1 */
    pub fn RegOpenCurrentUser(samDesired: ::winreg::REGSAM, phkResult: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:233:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegOpenKeyExW as RegOpenKeyEx; /* winreg.h:706:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegOpenKeyExA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, ulOptions: ::minwindef::DWORD, samDesired: ::winreg::REGSAM, phkResult: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:686:1 */
    pub fn RegOpenKeyExW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, ulOptions: ::minwindef::DWORD, samDesired: ::winreg::REGSAM, phkResult: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:697:1 */
    pub fn RegOpenUserClassesRoot(hToken: ::winnt::HANDLE, dwOptions: ::minwindef::DWORD, samDesired: ::winreg::REGSAM, phkResult: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:222:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegQueryInfoKeyW as RegQueryInfoKey; /* winreg.h:778:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegQueryInfoKeyA(hKey: ::minwindef::HKEY, lpClass: ::winnt::LPSTR, lpcchClass: ::minwindef::LPDWORD, lpReserved: ::minwindef::LPDWORD, lpcSubKeys: ::minwindef::LPDWORD, lpcbMaxSubKeyLen: ::minwindef::LPDWORD, lpcbMaxClassLen: ::minwindef::LPDWORD, lpcValues: ::minwindef::LPDWORD, lpcbMaxValueNameLen: ::minwindef::LPDWORD, lpcbMaxValueLen: ::minwindef::LPDWORD, lpcbSecurityDescriptor: ::minwindef::LPDWORD, lpftLastWriteTime: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:744:1 */
    pub fn RegQueryInfoKeyW(hKey: ::minwindef::HKEY, lpClass: ::winnt::LPWSTR, lpcchClass: ::minwindef::LPDWORD, lpReserved: ::minwindef::LPDWORD, lpcSubKeys: ::minwindef::LPDWORD, lpcbMaxSubKeyLen: ::minwindef::LPDWORD, lpcbMaxClassLen: ::minwindef::LPDWORD, lpcValues: ::minwindef::LPDWORD, lpcbMaxValueNameLen: ::minwindef::LPDWORD, lpcbMaxValueLen: ::minwindef::LPDWORD, lpcbSecurityDescriptor: ::minwindef::LPDWORD, lpftLastWriteTime: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:762:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegQueryValueExW as RegQueryValueEx; /* winreg.h:864:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegQueryValueExA(hKey: ::minwindef::HKEY, lpValueName: ::winnt::LPCSTR, lpReserved: ::minwindef::LPDWORD, lpType: ::minwindef::LPDWORD, lpData: ::minwindef::LPBYTE, lpcbData: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:840:1 */
    pub fn RegQueryValueExW(hKey: ::minwindef::HKEY, lpValueName: ::winnt::LPCWSTR, lpReserved: ::minwindef::LPDWORD, lpType: ::minwindef::LPDWORD, lpData: ::minwindef::LPBYTE, lpcbData: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:853:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegRestoreKeyW as RegRestoreKey; /* winreg.h:912:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegRestoreKeyA(hKey: ::minwindef::HKEY, lpFile: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:896:1 */
    pub fn RegRestoreKeyW(hKey: ::minwindef::HKEY, lpFile: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:905:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegSaveKeyExW as RegSaveKeyEx; /* winreg.h:1440:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegSaveKeyExA(hKey: ::minwindef::HKEY, lpFile: ::winnt::LPCSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, Flags: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1422:1 */
    pub fn RegSaveKeyExW(hKey: ::minwindef::HKEY, lpFile: ::winnt::LPCWSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, Flags: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1432:1 */
    pub fn RegSetKeySecurity(hKey: ::minwindef::HKEY, SecurityInformation: ::winnt::SECURITY_INFORMATION, pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR) -> ::winreg::LSTATUS; /* winreg.h:956:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegSetValueExW as RegSetValueEx; /* winreg.h:1014:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegSetValueExA(hKey: ::minwindef::HKEY, lpValueName: ::winnt::LPCSTR, Reserved: ::minwindef::DWORD, dwType: ::minwindef::DWORD, lpData: *const ::libc::c_uchar, cbData: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:992:1 */
    pub fn RegSetValueExW(hKey: ::minwindef::HKEY, lpValueName: ::winnt::LPCWSTR, Reserved: ::minwindef::DWORD, dwType: ::minwindef::DWORD, lpData: *const ::libc::c_uchar, cbData: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1004:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegUnLoadKeyW as RegUnLoadKey; /* winreg.h:1036:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegUnLoadKeyA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:1022:1 */
    pub fn RegUnLoadKeyW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:1030:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegisterServiceCtrlHandlerW as RegisterServiceCtrlHandler; /* winsvc.h:1509:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegisterServiceCtrlHandlerA(lpServiceName: ::winnt::LPCSTR, lpHandlerProc: ::winsvc::LPHANDLER_FUNCTION) -> ::winsvc::SERVICE_STATUS_HANDLE; /* winsvc.h:1494:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegisterServiceCtrlHandlerExW as RegisterServiceCtrlHandlerEx; /* winsvc.h:1535:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegisterServiceCtrlHandlerExA(lpServiceName: ::winnt::LPCSTR, lpHandlerProc: ::winsvc::LPHANDLER_FUNCTION_EX, lpContext: ::minwindef::LPVOID) -> ::winsvc::SERVICE_STATUS_HANDLE; /* winsvc.h:1518:1 */
    pub fn RegisterServiceCtrlHandlerExW(lpServiceName: ::winnt::LPCWSTR, lpHandlerProc: ::winsvc::LPHANDLER_FUNCTION_EX, lpContext: ::minwindef::LPVOID) -> ::winsvc::SERVICE_STATUS_HANDLE; /* winsvc.h:1528:1 */
    pub fn RegisterServiceCtrlHandlerW(lpServiceName: ::winnt::LPCWSTR, lpHandlerProc: ::winsvc::LPHANDLER_FUNCTION) -> ::winsvc::SERVICE_STATUS_HANDLE; /* winsvc.h:1503:1 */
    pub fn RemoveDllDirectory(Cookie: ::libloaderapi::DLL_DIRECTORY_COOKIE) -> ::minwindef::BOOL; /* libloaderapi.h:506:1 */
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
    pub fn SetAclInformation(pAcl: ::winnt::PACL, pAclInformation: ::minwindef::LPVOID, nAclInformationLength: ::minwindef::DWORD, dwAclInformationClass: ::winnt::ACL_INFORMATION_CLASS) -> ::minwindef::BOOL; /* securitybaseapi.h:1157:1 */
    pub fn SetCommBreak(hFile: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:2095:1 */
    pub fn SetCommConfig(hCommDev: ::winnt::HANDLE, lpCC: ::winbase::LPCOMMCONFIG, dwSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2102:1 */
    pub fn SetCommMask(hFile: ::winnt::HANDLE, dwEvtMask: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2111:1 */
    pub fn SetCommState(hFile: ::winnt::HANDLE, lpDCB: ::winbase::LPDCB) -> ::minwindef::BOOL; /* winbase.h:2119:1 */
    pub fn SetCommTimeouts(hFile: ::winnt::HANDLE, lpCommTimeouts: ::winbase::LPCOMMTIMEOUTS) -> ::minwindef::BOOL; /* winbase.h:2127:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetComputerNameExW as SetComputerNameEx; /* sysinfoapi.h:405:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetComputerNameEx2W as SetComputerNameEx2; /* sysinfoapi.h:631:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetComputerNameEx2W(NameType: ::sysinfoapi::COMPUTER_NAME_FORMAT, Flags: ::minwindef::DWORD, lpBuffer: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* sysinfoapi.h:623:1 */
    pub fn SetComputerNameExW(NameType: ::sysinfoapi::COMPUTER_NAME_FORMAT, lpBuffer: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* sysinfoapi.h:398:1 */
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
    pub fn SetConsoleTitleW(lpConsoleTitle: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* wincon.h:789:1 */
    pub fn SetConsoleWindowInfo(hConsoleOutput: ::winnt::HANDLE, bAbsolute: ::minwindef::BOOL, lpConsoleWindow: *const ::wincon::SMALL_RECT) -> ::minwindef::BOOL; /* wincon.h:702:1 */
    pub fn SetCriticalSectionSpinCount(lpCriticalSection: ::minwinbase::LPCRITICAL_SECTION, dwSpinCount: ::minwindef::DWORD) -> ::minwindef::DWORD; /* synchapi.h:241:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetCurrentDirectoryW as SetCurrentDirectory; /* processenv.h:268:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetCurrentDirectoryA(lpPathName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* processenv.h:256:1 */
    pub fn SetCurrentDirectoryW(lpPathName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* processenv.h:263:1 */
    pub fn SetDefaultDllDirectories(DirectoryFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* libloaderapi.h:514:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetEnvironmentStringsW as SetEnvironmentStrings; /* processenv.h:82:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetEnvironmentStringsW(NewEnvironment: ::winnt::LPWCH) -> ::minwindef::BOOL; /* processenv.h:77:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetEnvironmentVariableW as SetEnvironmentVariable; /* processenv.h:222:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetEnvironmentVariableA(lpName: ::winnt::LPCSTR, lpValue: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* processenv.h:208:1 */
    pub fn SetEnvironmentVariableW(lpName: ::winnt::LPCWSTR, lpValue: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* processenv.h:216:1 */
    pub fn SetErrorMode(uMode: ::minwindef::UINT) -> ::minwindef::UINT; /* errhandlingapi.h:156:1 */
    pub fn SetFilePointer(hFile: ::winnt::HANDLE, lDistanceToMove: ::winnt::LONG, lpDistanceToMoveHigh: ::winnt::PLONG, dwMoveMethod: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:1057:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetFileSecurityW as SetFileSecurity; /* securitybaseapi.h:1175:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetFileSecurityW(lpFileName: ::winnt::LPCWSTR, SecurityInformation: ::winnt::SECURITY_INFORMATION, pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* securitybaseapi.h:1168:1 */
    pub fn SetFileTime(hFile: ::winnt::HANDLE, lpCreationTime: *const ::minwindef::FILETIME, lpLastAccessTime: *const ::minwindef::FILETIME, lpLastWriteTime: *const ::minwindef::FILETIME) -> ::minwindef::BOOL; /* fileapi.h:1093:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetFirmwareEnvironmentVariableW as SetFirmwareEnvironmentVariable; /* winbase.h:3608:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetFirmwareEnvironmentVariableW(lpName: ::winnt::LPCWSTR, lpGuid: ::winnt::LPCWSTR, pValue: ::winnt::PVOID, nSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3601:1 */
    pub fn SetHandleInformation(hObject: ::winnt::HANDLE, dwMask: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* handleapi.h:88:1 */
    pub fn SetKernelObjectSecurity(Handle: ::winnt::HANDLE, SecurityInformation: ::winnt::SECURITY_INFORMATION, SecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* securitybaseapi.h:1181:1 */
    pub fn SetLocalTime(lpSystemTime: *const ::minwinbase::SYSTEMTIME) -> ::minwindef::BOOL; /* sysinfoapi.h:176:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetLocaleInfoW as SetLocaleInfo; /* winnls.h:1544:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetLocaleInfoW(Locale: ::winnt::LCID, LCType: ::winnls::LCTYPE, lpLCData: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winnls.h:1539:1 */
    pub fn SetNamedPipeHandleState(hNamedPipe: ::winnt::HANDLE, lpMode: ::minwindef::LPDWORD, lpMaxCollectionCount: ::minwindef::LPDWORD, lpCollectDataTimeout: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:78:1 */
    pub fn SetPriorityClass(hProcess: ::winnt::HANDLE, dwPriorityClass: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:652:1 */
    pub fn SetPrivateObjectSecurity(SecurityInformation: ::winnt::SECURITY_INFORMATION, ModificationDescriptor: ::winnt::PSECURITY_DESCRIPTOR, ObjectsSecurityDescriptor: *mut *mut ::libc::c_void, GenericMapping: ::winnt::PGENERIC_MAPPING, Token: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:1191:1 */
    pub fn SetPrivateObjectSecurityEx(SecurityInformation: ::winnt::SECURITY_INFORMATION, ModificationDescriptor: ::winnt::PSECURITY_DESCRIPTOR, ObjectsSecurityDescriptor: *mut *mut ::libc::c_void, AutoInheritFlags: ::minwindef::ULONG, GenericMapping: ::winnt::PGENERIC_MAPPING, Token: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:1203:1 */
    pub fn SetProcessShutdownParameters(dwLevel: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:542:1 */
    pub fn SetProcessWorkingSetSizeEx(hProcess: ::winnt::HANDLE, dwMinimumWorkingSetSize: ::basetsd::SIZE_T, dwMaximumWorkingSetSize: ::basetsd::SIZE_T, Flags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:359:1 */
    pub fn SetSecurityDescriptorControl(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, ControlBitsOfInterest: ::winnt::SECURITY_DESCRIPTOR_CONTROL, ControlBitsToSet: ::winnt::SECURITY_DESCRIPTOR_CONTROL) -> ::minwindef::BOOL; /* securitybaseapi.h:1230:1 */
    pub fn SetSecurityDescriptorDacl(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, bDaclPresent: ::minwindef::BOOL, pDacl: ::winnt::PACL, bDaclDefaulted: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1240:1 */
    pub fn SetSecurityDescriptorGroup(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, pGroup: ::winnt::PSID, bGroupDefaulted: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1251:1 */
    pub fn SetSecurityDescriptorOwner(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, pOwner: ::winnt::PSID, bOwnerDefaulted: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1261:1 */
    pub fn SetSecurityDescriptorRMControl(SecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, RMControl: ::minwindef::PUCHAR) -> ::minwindef::DWORD; /* securitybaseapi.h:1271:1 */
    pub fn SetSecurityDescriptorSacl(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, bSaclPresent: ::minwindef::BOOL, pSacl: ::winnt::PACL, bSaclDefaulted: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1280:1 */
    pub fn SetServiceObjectSecurity(hService: ::winsvc::SC_HANDLE, dwSecurityInformation: ::winnt::SECURITY_INFORMATION, lpSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* winsvc.h:1543:1 */
    pub fn SetServiceStatus(hServiceStatus: ::winsvc::SERVICE_STATUS_HANDLE, lpServiceStatus: ::winsvc::LPSERVICE_STATUS) -> ::minwindef::BOOL; /* winsvc.h:1552:1 */
    pub fn SetStdHandle(nStdHandle: ::minwindef::DWORD, hHandle: ::winnt::HANDLE) -> ::minwindef::BOOL; /* processenv.h:116:1 */
    pub fn SetSystemTime(lpSystemTime: *const ::minwinbase::SYSTEMTIME) -> ::minwindef::BOOL; /* sysinfoapi.h:411:1 */
    pub fn SetSystemTimeAdjustment(dwTimeAdjustment: ::minwindef::DWORD, bTimeAdjustmentDisabled: ::minwindef::BOOL) -> ::minwindef::BOOL; /* sysinfoapi.h:638:1 */
    pub fn SetThreadContext(hThread: ::winnt::HANDLE, lpContext: *const ::winnt::CONTEXT) -> ::minwindef::BOOL; /* processthreadsapi.h:868:1 */
    pub fn SetThreadLocale(Locale: ::winnt::LCID) -> ::minwindef::BOOL; /* winnls.h:1968:1 */
    pub fn SetThreadPriorityBoost(hThread: ::winnt::HANDLE, bDisablePriorityBoost: ::minwindef::BOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:303:1 */
    pub fn SetThreadStackGuarantee(StackSizeInBytes: ::minwindef::PULONG) -> ::minwindef::BOOL; /* processthreadsapi.h:661:1 */
    pub fn SetThreadToken(Thread: ::winnt::PHANDLE, Token: ::winnt::HANDLE) -> ::minwindef::BOOL; /* processthreadsapi.h:613:1 */
    pub fn SetThreadUILanguage(LangId: ::winnt::LANGID) -> ::winnt::LANGID; /* winnls.h:2008:1 */
    pub fn SetTimeZoneInformation(lpTimeZoneInformation: *const ::timezoneapi::TIME_ZONE_INFORMATION) -> ::minwindef::BOOL; /* timezoneapi.h:130:1 */
    pub fn SetTokenInformation(TokenHandle: ::winnt::HANDLE, TokenInformationClass: ::winnt::TOKEN_INFORMATION_CLASS, TokenInformation: ::minwindef::LPVOID, TokenInformationLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:1291:1 */
    pub fn SetUnhandledExceptionFilter(lpTopLevelExceptionFilter: ::errhandlingapi::LPTOP_LEVEL_EXCEPTION_FILTER) -> ::errhandlingapi::LPTOP_LEVEL_EXCEPTION_FILTER; /* errhandlingapi.h:100:1 */
    pub fn SetUserGeoID(GeoId: ::winnls::GEOID) -> ::minwindef::BOOL; /* winnls.h:1951:1 */
    pub fn SetupComm(hFile: ::winnt::HANDLE, dwInQueue: ::minwindef::DWORD, dwOutQueue: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2020:1 */
    pub fn SizeofResource(hModule: ::minwindef::HMODULE, hResInfo: ::minwindef::HRSRC) -> ::minwindef::DWORD; /* libloaderapi.h:478:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::StartServiceW as StartService; /* winsvc.h:1595:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn StartServiceA(hService: ::winsvc::SC_HANDLE, dwNumServiceArgs: ::minwindef::DWORD, lpServiceArgVectors: *mut *const ::libc::c_schar) -> ::minwindef::BOOL; /* winsvc.h:1579:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::StartServiceCtrlDispatcherW as StartServiceCtrlDispatcher; /* winsvc.h:1570:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn StartServiceCtrlDispatcherA(lpServiceStartTable: *const ::winsvc::SERVICE_TABLE_ENTRYA) -> ::minwindef::BOOL; /* winsvc.h:1560:1 */
    pub fn StartServiceCtrlDispatcherW(lpServiceStartTable: *const ::winsvc::SERVICE_TABLE_ENTRYW) -> ::minwindef::BOOL; /* winsvc.h:1566:1 */
    pub fn StartServiceW(hService: ::winsvc::SC_HANDLE, dwNumServiceArgs: ::minwindef::DWORD, lpServiceArgVectors: *mut *const ::libc::c_ushort) -> ::minwindef::BOOL; /* winsvc.h:1588:1 */
    pub fn SwitchToThread() -> ::minwindef::BOOL; /* processthreadsapi.h:195:1 */
    pub fn TerminateProcess(hProcess: ::winnt::HANDLE, uExitCode: ::minwindef::UINT) -> ::minwindef::BOOL; /* processthreadsapi.h:177:1 */
    pub fn TerminateThread(hThread: ::winnt::HANDLE, dwExitCode: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:348:1 */
    pub fn TlsAlloc() -> ::minwindef::DWORD; /* processthreadsapi.h:460:1 */
    pub fn TlsFree(dwTlsIndex: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:485:1 */
    pub fn TlsSetValue(dwTlsIndex: ::minwindef::DWORD, lpTlsValue: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* processthreadsapi.h:476:1 */
    pub fn TransactNamedPipe(hNamedPipe: ::winnt::HANDLE, lpInBuffer: ::minwindef::LPVOID, nInBufferSize: ::minwindef::DWORD, lpOutBuffer: ::minwindef::LPVOID, nOutBufferSize: ::minwindef::DWORD, lpBytesRead: ::minwindef::LPDWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* namedpipeapi.h:102:1 */
    pub fn TransmitCommChar(hFile: ::winnt::HANDLE, cChar: ::libc::c_schar) -> ::minwindef::BOOL; /* winbase.h:2135:1 */
    pub fn UnlockFile(hFile: ::winnt::HANDLE, dwFileOffsetLow: ::minwindef::DWORD, dwFileOffsetHigh: ::minwindef::DWORD, nNumberOfBytesToUnlockLow: ::minwindef::DWORD, nNumberOfBytesToUnlockHigh: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fileapi.h:1118:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VerFindFileW as VerFindFile; /* winver.h:59:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VerFindFileW(uFlags: ::minwindef::DWORD, szFileName: ::winnt::LPCWSTR, szWinDir: ::winnt::LPCWSTR, szAppDir: ::winnt::LPCWSTR, szCurDir: ::winnt::LPWSTR, puCurDirLen: ::minwindef::PUINT, szDestDir: ::winnt::LPWSTR, puDestDirLen: ::minwindef::PUINT) -> ::minwindef::DWORD; /* winver.h:48:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VerLanguageNameW as VerLanguageName; /* winver.h:178:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VerLanguageNameA(wLang: ::minwindef::DWORD, szLang: ::winnt::LPSTR, cchLang: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winver.h:165:1 */
    pub fn VerLanguageNameW(wLang: ::minwindef::DWORD, szLang: ::winnt::LPWSTR, cchLang: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winver.h:172:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::VerQueryValueW as VerQueryValue; /* winver.h:200:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn VerQueryValueW(pBlock: ::minwindef::LPCVOID, lpSubBlock: ::winnt::LPCWSTR, lplpBuffer: *mut *mut ::libc::c_void, puLen: ::minwindef::PUINT) -> ::minwindef::BOOL; /* winver.h:193:1 */
    pub fn VirtualAlloc(lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T, flAllocationType: ::minwindef::DWORD, flProtect: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* memoryapi.h:90:1 */
    pub fn VirtualAllocEx(hProcess: ::winnt::HANDLE, lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T, flAllocationType: ::minwindef::DWORD, flProtect: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* memoryapi.h:157:1 */
    pub fn VirtualFree(lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T, dwFreeType: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:110:1 */
    pub fn VirtualFreeEx(hProcess: ::winnt::HANDLE, lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T, dwFreeType: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:181:1 */
    pub fn VirtualLock(lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* memoryapi.h:370:1 */
    pub fn VirtualProtect(lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T, flNewProtect: ::minwindef::DWORD, lpflOldProtect: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:121:1 */
    pub fn VirtualProtectEx(hProcess: ::winnt::HANDLE, lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T, flNewProtect: ::minwindef::DWORD, lpflOldProtect: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:193:1 */
    pub fn VirtualQueryEx(hProcess: ::winnt::HANDLE, lpAddress: ::minwindef::LPCVOID, lpBuffer: ::winnt::PMEMORY_BASIC_INFORMATION, dwLength: ::basetsd::SIZE_T) -> ::basetsd::SIZE_T; /* memoryapi.h:205:1 */
    pub fn VirtualUnlock(lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* memoryapi.h:379:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetAddConnectionW as WNetAddConnection; /* winnetwk.h:164:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetAddConnection2W as WNetAddConnection2; /* winnetwk.h:186:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetAddConnection2A(lpNetResource: ::winnetwk::LPNETRESOURCEA, lpPassword: ::winnt::LPCSTR, lpUserName: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:171:1 */
    pub fn WNetAddConnection2W(lpNetResource: ::winnetwk::LPNETRESOURCEW, lpPassword: ::winnt::LPCWSTR, lpUserName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:179:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetAddConnection3W as WNetAddConnection3; /* winnetwk.h:210:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetAddConnection3A(hwndOwner: ::windef::HWND, lpNetResource: ::winnetwk::LPNETRESOURCEA, lpPassword: ::winnt::LPCSTR, lpUserName: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:193:1 */
    pub fn WNetAddConnection3W(hwndOwner: ::windef::HWND, lpNetResource: ::winnetwk::LPNETRESOURCEW, lpPassword: ::winnt::LPCWSTR, lpUserName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:202:1 */
    pub fn WNetAddConnectionA(lpRemoteName: ::winnt::LPCSTR, lpPassword: ::winnt::LPCSTR, lpLocalName: ::winnt::LPCSTR) -> ::minwindef::DWORD; /* winnetwk.h:151:1 */
    pub fn WNetAddConnectionW(lpRemoteName: ::winnt::LPCWSTR, lpPassword: ::winnt::LPCWSTR, lpLocalName: ::winnt::LPCWSTR) -> ::minwindef::DWORD; /* winnetwk.h:158:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetCancelConnectionW as WNetCancelConnection; /* winnetwk.h:228:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetCancelConnection2W as WNetCancelConnection2; /* winnetwk.h:248:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetCancelConnection2A(lpName: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD, fForce: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:235:1 */
    pub fn WNetCancelConnection2W(lpName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, fForce: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:242:1 */
    pub fn WNetCancelConnectionA(lpName: ::winnt::LPCSTR, fForce: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:217:1 */
    pub fn WNetCancelConnectionW(lpName: ::winnt::LPCWSTR, fForce: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winnetwk.h:223:1 */
    pub fn WNetCloseEnum(hEnum: ::winnt::HANDLE) -> ::minwindef::DWORD; /* winnetwk.h:490:1 */
    pub fn WNetConnectionDialog(hwnd: ::windef::HWND, dwType: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:332:1 */
    pub fn WNetDisconnectDialog(hwnd: ::windef::HWND, dwType: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:339:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetEnumResourceW as WNetEnumResource; /* winnetwk.h:483:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetEnumResourceA(hEnum: ::winnt::HANDLE, lpcCount: ::minwindef::LPDWORD, lpBuffer: ::minwindef::LPVOID, lpBufferSize: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:468:1 */
    pub fn WNetEnumResourceW(hEnum: ::winnt::HANDLE, lpcCount: ::minwindef::LPDWORD, lpBuffer: ::minwindef::LPVOID, lpBufferSize: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:476:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetConnectionW as WNetGetConnection; /* winnetwk.h:268:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetConnectionA(lpLocalName: ::winnt::LPCSTR, lpRemoteName: ::winnt::LPSTR, lpnLength: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:255:1 */
    pub fn WNetGetConnectionW(lpLocalName: ::winnt::LPCWSTR, lpRemoteName: ::winnt::LPWSTR, lpnLength: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:262:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetLastErrorW as WNetGetLastError; /* winnetwk.h:718:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetLastErrorA(lpError: ::minwindef::LPDWORD, lpErrorBuf: ::winnt::LPSTR, nErrorBufSize: ::minwindef::DWORD, lpNameBuf: ::winnt::LPSTR, nNameBufSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:701:1 */
    pub fn WNetGetLastErrorW(lpError: ::minwindef::LPDWORD, lpErrorBuf: ::winnt::LPWSTR, nErrorBufSize: ::minwindef::DWORD, lpNameBuf: ::winnt::LPWSTR, nNameBufSize: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winnetwk.h:710:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetUniversalNameW as WNetGetUniversalName; /* winnetwk.h:594:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetUniversalNameA(lpLocalPath: ::winnt::LPCSTR, dwInfoLevel: ::minwindef::DWORD, lpBuffer: ::minwindef::LPVOID, lpBufferSize: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:579:1 */
    pub fn WNetGetUniversalNameW(lpLocalPath: ::winnt::LPCWSTR, dwInfoLevel: ::minwindef::DWORD, lpBuffer: ::minwindef::LPVOID, lpBufferSize: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:587:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetGetUserW as WNetGetUser; /* winnetwk.h:620:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetGetUserA(lpName: ::winnt::LPCSTR, lpUserName: ::winnt::LPSTR, lpnLength: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:604:1 */
    pub fn WNetGetUserW(lpName: ::winnt::LPCWSTR, lpUserName: ::winnt::LPWSTR, lpnLength: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:614:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WNetOpenEnumW as WNetOpenEnum; /* winnetwk.h:461:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WNetOpenEnumA(dwScope: ::minwindef::DWORD, dwType: ::minwindef::DWORD, dwUsage: ::minwindef::DWORD, lpNetResource: ::winnetwk::LPNETRESOURCEA, lphEnum: ::minwindef::LPHANDLE) -> ::minwindef::DWORD; /* winnetwk.h:444:1 */
    pub fn WNetOpenEnumW(dwScope: ::minwindef::DWORD, dwType: ::minwindef::DWORD, dwUsage: ::minwindef::DWORD, lpNetResource: ::winnetwk::LPNETRESOURCEW, lphEnum: ::minwindef::LPHANDLE) -> ::minwindef::DWORD; /* winnetwk.h:453:1 */
    pub fn WaitCommEvent(hFile: ::winnt::HANDLE, lpEvtMask: ::minwindef::LPDWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* winbase.h:2143:1 */
    pub fn WaitForDebugEvent(lpDebugEvent: ::minwinbase::LPDEBUG_EVENT, dwMilliseconds: ::minwindef::DWORD) -> ::minwindef::BOOL; /* debugapi.h:127:1 */
    pub fn WaitForSingleObject(hHandle: ::winnt::HANDLE, dwMilliseconds: ::minwindef::DWORD) -> ::minwindef::DWORD; /* synchapi.h:473:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::WaitNamedPipeW as WaitNamedPipe; /* namedpipeapi.h:142:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WaitNamedPipeW(lpNamedPipeName: ::winnt::LPCWSTR, nTimeOut: ::minwindef::DWORD) -> ::minwindef::BOOL; /* namedpipeapi.h:135:1 */
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
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] 
extern "system" {
    pub fn RaiseException(dwExceptionCode: ::minwindef::DWORD, dwExceptionFlags: ::minwindef::DWORD, nNumberOfArguments: ::minwindef::DWORD, lpArguments: *const ::libc::c_ulonglong); /* errhandlingapi.h:73:1 */
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
    pub fn GetGeoInfoW(Location: ::winnls::GEOID, GeoType: ::winnls::GEOTYPE, lpGeoData: ::winnt::LPWSTR, cchData: ::libc::c_int, LangId: ::winnt::LANGID) -> ::libc::c_int; /* winnls.h:1916:1 */
    pub fn GetLastError() -> ::minwindef::DWORD; /* errhandlingapi.h:118:1 */
    pub fn GetLocalTime(lpSystemTime: ::minwinbase::LPSYSTEMTIME); /* sysinfoapi.h:161:1 */
    pub fn GetOverlappedResultEx(hFile: ::winnt::HANDLE, lpOverlapped: ::minwinbase::LPOVERLAPPED, lpNumberOfBytesTransferred: ::minwindef::LPDWORD, dwMilliseconds: ::minwindef::DWORD, bAlertable: ::minwindef::BOOL) -> ::minwindef::BOOL; /* ioapiset.h:159:1 */
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
    pub fn MoveFileExW(lpExistingFileName: ::winnt::LPCWSTR, lpNewFileName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5373:1 */
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
    pub fn WaitOnAddress(Address: *mut ::libc::c_void, CompareAddress: ::winnt::PVOID, AddressSize: ::basetsd::SIZE_T, dwMilliseconds: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:915:1 */
    pub fn WakeByAddressAll(Address: ::winnt::PVOID); /* synchapi.h:932:1 */
    pub fn WakeByAddressSingle(Address: ::winnt::PVOID); /* synchapi.h:925:1 */
    pub fn WideCharToMultiByte(CodePage: ::minwindef::UINT, dwFlags: ::minwindef::DWORD, lpWideCharStr: ::winnt::LPCWCH, cchWideChar: ::libc::c_int, lpMultiByteStr: ::winnt::LPSTR, cbMultiByte: ::libc::c_int, lpDefaultChar: ::winnt::LPCCH, lpUsedDefaultChar: ::minwindef::LPBOOL) -> ::libc::c_int; /* stringapiset.h:169:1 */
    pub fn WriteFile(hFile: ::winnt::HANDLE, lpBuffer: ::minwindef::LPCVOID, nNumberOfBytesToWrite: ::minwindef::DWORD, lpNumberOfBytesWritten: ::minwindef::LPDWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED) -> ::minwindef::BOOL; /* fileapi.h:1149:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn CancelWaitableTimer(hTimer: ::winnt::HANDLE) -> ::minwindef::BOOL; /* synchapi.h:729:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::CopyFileExW as CopyFileEx; /* winbase.h:5128:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn CopyFileExW(lpExistingFileName: ::winnt::LPCWSTR, lpNewFileName: ::winnt::LPCWSTR, lpProgressRoutine: ::winbase::LPPROGRESS_ROUTINE, lpData: ::minwindef::LPVOID, pbCancel: ::minwindef::LPBOOL, dwCopyFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5118:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::MultinetGetConnectionPerformanceW as MultinetGetConnectionPerformance; /* winnetwk.h:818:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn MultinetGetConnectionPerformanceA(lpNetResource: ::winnetwk::LPNETRESOURCEA, lpNetConnectInfoStruct: ::winnetwk::LPNETCONNECTINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:807:1 */
    pub fn MultinetGetConnectionPerformanceW(lpNetResource: ::winnetwk::LPNETRESOURCEW, lpNetConnectInfoStruct: ::winnetwk::LPNETCONNECTINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:813:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::OpenWaitableTimerW as OpenWaitableTimer; /* synchapi.h:692:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn OpenWaitableTimerW(dwDesiredAccess: ::minwindef::DWORD, bInheritHandle: ::minwindef::BOOL, lpTimerName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* synchapi.h:684:1 */
    pub fn QueueUserAPC(pfnAPC: ::winnt::PAPCFUNC, hThread: ::winnt::HANDLE, dwData: ::basetsd::ULONG_PTR) -> ::minwindef::DWORD; /* processthreadsapi.h:114:1 */
    pub fn ReadDirectoryChangesW(hDirectory: ::winnt::HANDLE, lpBuffer: ::minwindef::LPVOID, nBufferLength: ::minwindef::DWORD, bWatchSubtree: ::minwindef::BOOL, dwNotifyFilter: ::minwindef::DWORD, lpBytesReturned: ::minwindef::LPDWORD, lpOverlapped: ::minwinbase::LPOVERLAPPED, lpCompletionRoutine: ::minwinbase::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::minwindef::BOOL; /* winbase.h:6368:1 */
    pub fn SetWaitableTimer(hTimer: ::winnt::HANDLE, lpDueTime: *const ::winnt::LARGE_INTEGER, lPeriod: ::winnt::LONG, pfnCompletionRoutine: ::synchapi::PTIMERAPCROUTINE, lpArgToCompletionRoutine: ::minwindef::LPVOID, fResume: ::minwindef::BOOL) -> ::minwindef::BOOL; /* synchapi.h:716:1 */
    pub fn SignalObjectAndWait(hObjectToSignal: ::winnt::HANDLE, hObjectToWaitOn: ::winnt::HANDLE, dwMilliseconds: ::minwindef::DWORD, bAlertable: ::minwindef::BOOL) -> ::minwindef::DWORD; /* winbase.h:2851:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetConnectionDialog1W as WNetConnectionDialog1; /* winnetwk.h:391:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetConnectionDialog1A(lpConnDlgStruct: ::winnetwk::LPCONNECTDLGSTRUCTA) -> ::minwindef::DWORD; /* winnetwk.h:382:1 */
    pub fn WNetConnectionDialog1W(lpConnDlgStruct: ::winnetwk::LPCONNECTDLGSTRUCTW) -> ::minwindef::DWORD; /* winnetwk.h:387:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetDisconnectDialog1W as WNetDisconnectDialog1; /* winnetwk.h:432:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetDisconnectDialog1A(lpConnDlgStruct: ::winnetwk::LPDISCDLGSTRUCTA) -> ::minwindef::DWORD; /* winnetwk.h:423:1 */
    pub fn WNetDisconnectDialog1W(lpConnDlgStruct: ::winnetwk::LPDISCDLGSTRUCTW) -> ::minwindef::DWORD; /* winnetwk.h:428:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetNetworkInformationW as WNetGetNetworkInformation; /* winnetwk.h:688:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetNetworkInformationA(lpProvider: ::winnt::LPCSTR, lpNetInfoStruct: ::winnetwk::LPNETINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:677:1 */
    pub fn WNetGetNetworkInformationW(lpProvider: ::winnt::LPCWSTR, lpNetInfoStruct: ::winnetwk::LPNETINFOSTRUCT) -> ::minwindef::DWORD; /* winnetwk.h:683:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetProviderNameW as WNetGetProviderName; /* winnetwk.h:655:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetProviderNameA(dwNetType: ::minwindef::DWORD, lpProviderName: ::winnt::LPSTR, lpBufferSize: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:642:1 */
    pub fn WNetGetProviderNameW(dwNetType: ::minwindef::DWORD, lpProviderName: ::winnt::LPWSTR, lpBufferSize: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:649:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetResourceInformationW as WNetGetResourceInformation; /* winnetwk.h:532:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetResourceInformationA(lpNetResource: ::winnetwk::LPNETRESOURCEA, lpBuffer: ::minwindef::LPVOID, lpcbBuffer: ::minwindef::LPDWORD, lplpSystem: *mut *mut ::libc::c_schar) -> ::minwindef::DWORD; /* winnetwk.h:517:1 */
    pub fn WNetGetResourceInformationW(lpNetResource: ::winnetwk::LPNETRESOURCEW, lpBuffer: ::minwindef::LPVOID, lpcbBuffer: ::minwindef::LPDWORD, lplpSystem: *mut *mut ::libc::c_ushort) -> ::minwindef::DWORD; /* winnetwk.h:525:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetGetResourceParentW as WNetGetResourceParent; /* winnetwk.h:510:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetGetResourceParentA(lpNetResource: ::winnetwk::LPNETRESOURCEA, lpBuffer: ::minwindef::LPVOID, lpcbBuffer: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:497:1 */
    pub fn WNetGetResourceParentW(lpNetResource: ::winnetwk::LPNETRESOURCEW, lpBuffer: ::minwindef::LPVOID, lpcbBuffer: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:504:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::WNetUseConnectionW as WNetUseConnection; /* winnetwk.h:320:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn WNetUseConnectionA(hwndOwner: ::windef::HWND, lpNetResource: ::winnetwk::LPNETRESOURCEA, lpPassword: ::winnt::LPCSTR, lpUserId: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD, lpAccessName: ::winnt::LPSTR, lpBufferSize: ::minwindef::LPDWORD, lpResult: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:297:1 */
    pub fn WNetUseConnectionW(hwndOwner: ::windef::HWND, lpNetResource: ::winnetwk::LPNETRESOURCEW, lpPassword: ::winnt::LPCWSTR, lpUserId: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, lpAccessName: ::winnt::LPWSTR, lpBufferSize: ::minwindef::LPDWORD, lpResult: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* winnetwk.h:309:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::FindFirstFileExW as FindFirstFileEx; /* fileapi.h:334:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn FindFirstFileExA(lpFileName: ::winnt::LPCSTR, fInfoLevelId: ::minwinbase::FINDEX_INFO_LEVELS, lpFindFileData: ::minwindef::LPVOID, fSearchOp: ::minwinbase::FINDEX_SEARCH_OPS, lpSearchFilter: ::minwindef::LPVOID, dwAdditionalFlags: ::minwindef::DWORD) -> ::winnt::HANDLE; /* fileapi.h:312:1 */
    pub fn FindFirstFileExW(lpFileName: ::winnt::LPCWSTR, fInfoLevelId: ::minwinbase::FINDEX_INFO_LEVELS, lpFindFileData: ::minwindef::LPVOID, fSearchOp: ::minwinbase::FINDEX_SEARCH_OPS, lpSearchFilter: ::minwindef::LPVOID, dwAdditionalFlags: ::minwindef::DWORD) -> ::winnt::HANDLE; /* fileapi.h:324:1 */
    pub fn IsDebuggerPresent() -> ::minwindef::BOOL; /* debugapi.h:54:1 */
    pub fn TryEnterCriticalSection(lpCriticalSection: ::minwinbase::LPCRITICAL_SECTION) -> ::minwindef::BOOL; /* synchapi.h:260:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn AttachConsole(dwProcessId: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wincon.h:733:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::CreateHardLinkW as CreateHardLink; /* winbase.h:5524:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn CreateHardLinkW(lpFileName: ::winnt::LPCWSTR, lpExistingFileName: ::winnt::LPCWSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::minwindef::BOOL; /* winbase.h:5518:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::EnumSystemLocalesW as EnumSystemLocales; /* winnls.h:2195:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn EnumSystemLocalesA(lpLocaleEnumProc: ::winnls::LOCALE_ENUMPROCA, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:2185:1 */
    pub fn EnumSystemLocalesW(lpLocaleEnumProc: ::winnls::LOCALE_ENUMPROCW, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:2191:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::GetCalendarInfoW as GetCalendarInfo; /* winnls.h:1574:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn GetCalendarInfoW(Locale: ::winnt::LCID, Calendar: ::winnls::CALID, CalType: ::winnls::CALTYPE, lpCalData: ::winnt::LPWSTR, cchData: ::libc::c_int, lpValue: ::minwindef::LPDWORD) -> ::libc::c_int; /* winnls.h:1566:1 */
    pub fn IsValidLanguageGroup(LanguageGroup: ::winnls::LGRPID, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winnls.h:1866:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::MoveFileWithProgressW as MoveFileWithProgress; /* winbase.h:5412:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn MoveFileWithProgressW(lpExistingFileName: ::winnt::LPCWSTR, lpNewFileName: ::winnt::LPCWSTR, lpProgressRoutine: ::winbase::LPPROGRESS_ROUTINE, lpData: ::minwindef::LPVOID, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:5404:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::ReplaceFileW as ReplaceFile; /* winbase.h:5495:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn ReplaceFileW(lpReplacedFileName: ::winnt::LPCWSTR, lpReplacementFileName: ::winnt::LPCWSTR, lpBackupFileName: ::winnt::LPCWSTR, dwReplaceFlags: ::minwindef::DWORD, lpExclude: ::minwindef::LPVOID, lpReserved: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winbase.h:5486:1 */
    pub fn RtlCompareMemory(Source1: *const ::libc::c_void, Source2: *const ::libc::c_void, Length: ::basetsd::SIZE_T) -> ::basetsd::SIZE_T; /* winnt.h:17442:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::SetCalendarInfoW as SetCalendarInfo; /* winnls.h:1596:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn SetCalendarInfoW(Locale: ::winnt::LCID, Calendar: ::winnls::CALID, CalType: ::winnls::CALTYPE, lpCalData: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winnls.h:1590:1 */
    pub fn VerSetConditionMask(ConditionMask: ::winnt::ULONGLONG, TypeMask: ::minwindef::DWORD, Condition: ::minwindef::BYTE) -> ::winnt::ULONGLONG; /* winnt.h:17986:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn AddVectoredContinueHandler(First: ::minwindef::ULONG, Handler: ::winnt::PVECTORED_EXCEPTION_HANDLER) -> ::winnt::PVOID; /* errhandlingapi.h:197:1 */
    pub fn AddVectoredExceptionHandler(First: ::minwindef::ULONG, Handler: ::winnt::PVECTORED_EXCEPTION_HANDLER) -> ::winnt::PVOID; /* errhandlingapi.h:179:1 */
    pub fn AllocateUserPhysicalPages(hProcess: ::winnt::HANDLE, NumberOfPages: ::basetsd::PULONG_PTR, PageArray: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* memoryapi.h:582:1 */
    pub fn CheckRemoteDebuggerPresent(hProcess: ::winnt::HANDLE, pbDebuggerPresent: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* debugapi.h:155:1 */
    pub fn CreateMemoryResourceNotification(NotificationType: ::memoryapi::MEMORY_RESOURCE_NOTIFICATION_TYPE) -> ::winnt::HANDLE; /* memoryapi.h:420:1 */
    pub fn CreateWellKnownSid(WellKnownSidType: ::winnt::WELL_KNOWN_SID_TYPE, DomainSid: ::winnt::PSID, pSid: ::winnt::PSID, cbSid: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* securitybaseapi.h:600:1 */
    pub fn EqualDomainSid(pSid1: ::winnt::PSID, pSid2: ::winnt::PSID, pfEqual: *mut ::libc::c_int) -> ::minwindef::BOOL; /* securitybaseapi.h:612:1 */
    pub fn FreeUserPhysicalPages(hProcess: ::winnt::HANDLE, NumberOfPages: ::basetsd::PULONG_PTR, PageArray: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* memoryapi.h:593:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetCompressedFileSizeW as GetCompressedFileSize; /* fileapi.h:1340:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetCompressedFileSizeA(lpFileName: ::winnt::LPCSTR, lpFileSizeHigh: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* fileapi.h:1326:1 */
    pub fn GetCompressedFileSizeW(lpFileName: ::winnt::LPCWSTR, lpFileSizeHigh: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* fileapi.h:1334:1 */
    pub fn GetProcessHandleCount(hProcess: ::winnt::HANDLE, pdwHandleCount: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:974:1 */
    pub fn GetProcessId(Process: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:690:1 */
    pub fn GetProcessPriorityBoost(hProcess: ::winnt::HANDLE, pDisablePriorityBoost: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:1067:1 */
    pub fn GetSystemTimes(lpIdleTime: ::minwindef::PFILETIME, lpKernelTime: ::minwindef::PFILETIME, lpUserTime: ::minwindef::PFILETIME) -> ::minwindef::BOOL; /* processthreadsapi.h:1094:1 */
    pub fn GetThreadIOPendingFlag(hThread: ::winnt::HANDLE, lpIOIsPending: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:1085:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use self::GetVolumePathNamesForVolumeNameW as GetVolumePathNamesForVolumeName; /* fileapi.h:1245:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetVolumePathNamesForVolumeNameW(lpszVolumeName: ::winnt::LPCWSTR, lpszVolumePathNames: ::winnt::LPWCH, cchBufferLength: ::minwindef::DWORD, lpcchReturnLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* fileapi.h:1236:1 */
    pub fn GetWindowsAccountDomainSid(pSid: ::winnt::PSID, pDomainSid: ::winnt::PSID, cbDomainSid: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* securitybaseapi.h:889:1 */
    pub fn IsProcessInJob(ProcessHandle: ::winnt::HANDLE, JobHandle: ::winnt::HANDLE, Result: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* jobapi.h:46:1 */
    pub fn IsWellKnownSid(pSid: ::winnt::PSID, WellKnownSidType: ::winnt::WELL_KNOWN_SID_TYPE) -> ::minwindef::BOOL; /* securitybaseapi.h:991:1 */
    pub fn MapUserPhysicalPages(VirtualAddress: ::winnt::PVOID, NumberOfPages: ::basetsd::ULONG_PTR, PageArray: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* memoryapi.h:604:1 */
    pub fn QueryMemoryResourceNotification(ResourceNotificationHandle: ::winnt::HANDLE, ResourceState: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* memoryapi.h:429:1 */
    pub fn RemoveVectoredContinueHandler(Handle: ::winnt::PVOID) -> ::minwindef::ULONG; /* errhandlingapi.h:206:1 */
    pub fn RemoveVectoredExceptionHandler(Handle: ::winnt::PVOID) -> ::minwindef::ULONG; /* errhandlingapi.h:188:1 */
    pub fn SetFileValidData(hFile: ::winnt::HANDLE, ValidDataLength: ::winnt::LONGLONG) -> ::minwindef::BOOL; /* fileapi.h:1107:1 */
    pub fn SetProcessPriorityBoost(hProcess: ::winnt::HANDLE, bDisablePriorityBoost: ::minwindef::BOOL) -> ::minwindef::BOOL; /* processthreadsapi.h:1076:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn GetNativeSystemInfo(lpSystemInfo: ::sysinfoapi::LPSYSTEM_INFO); /* sysinfoapi.h:495:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010100"))] 
extern "system" {
    pub fn RtlCaptureStackBackTrace(FramesToSkip: ::minwindef::DWORD, FramesToCapture: ::minwindef::DWORD, BackTrace: *mut *mut ::libc::c_void, BackTraceHash: ::minwindef::PDWORD) -> ::minwindef::WORD; /* winnt.h:16911:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] 
extern "system" {
    pub fn GetCurrentProcessorNumber() -> ::minwindef::DWORD; /* processthreadsapi.h:995:1 */
    pub fn GetSystemFileCacheSize(lpMinimumFileCacheSize: ::basetsd::PSIZE_T, lpMaximumFileCacheSize: ::basetsd::PSIZE_T, lpFlags: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* memoryapi.h:450:1 */
    pub fn GetThreadId(Thread: ::winnt::HANDLE) -> ::minwindef::DWORD; /* processthreadsapi.h:703:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] #[doc(inline)] pub use self::NeedCurrentDirectoryForExePathW as NeedCurrentDirectoryForExePath; /* processenv.h:354:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05020000"))] 
extern "system" {
    pub fn NeedCurrentDirectoryForExePathA(ExeName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* processenv.h:342:1 */
    pub fn NeedCurrentDirectoryForExePathW(ExeName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* processenv.h:349:1 */
    pub fn ReOpenFile(hOriginalFile: ::winnt::HANDLE, dwDesiredAccess: ::minwindef::DWORD, dwShareMode: ::minwindef::DWORD, dwFlagsAndAttributes: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:4824:1 */
    pub fn SetSystemFileCacheSize(MinimumFileCacheSize: ::basetsd::SIZE_T, MaximumFileCacheSize: ::basetsd::SIZE_T, Flags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:460:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn AddMandatoryAce(pAcl: ::winnt::PACL, dwAceRevision: ::minwindef::DWORD, AceFlags: ::minwindef::DWORD, MandatoryPolicy: ::minwindef::DWORD, pLabelSid: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:343:1 */
    pub fn AllocateUserPhysicalPagesNuma(hProcess: ::winnt::HANDLE, NumberOfPages: ::basetsd::PULONG_PTR, PageArray: ::basetsd::PULONG_PTR, nndPreferred: ::minwindef::DWORD) -> ::minwindef::BOOL; /* memoryapi.h:620:1 */
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
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::ControlServiceExW as ControlServiceEx; /* winsvc.h:1650:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn ControlServiceExA(hService: ::winsvc::SC_HANDLE, dwControl: ::minwindef::DWORD, dwInfoLevel: ::minwindef::DWORD, pControlParams: ::winnt::PVOID) -> ::minwindef::BOOL; /* winsvc.h:1634:1 */
    pub fn ControlServiceExW(hService: ::winsvc::SC_HANDLE, dwControl: ::minwindef::DWORD, dwInfoLevel: ::minwindef::DWORD, pControlParams: ::winnt::PVOID) -> ::minwindef::BOOL; /* winsvc.h:1643:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateBoundaryDescriptorW as CreateBoundaryDescriptor; /* winbase.h:7258:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateFileMappingNumaW as CreateFileMappingNuma; /* memoryapi.h:488:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn CreateFileMappingNumaW(hFile: ::winnt::HANDLE, lpFileMappingAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, flProtect: ::minwindef::DWORD, dwMaximumSizeHigh: ::minwindef::DWORD, dwMaximumSizeLow: ::minwindef::DWORD, lpName: ::winnt::LPCWSTR, nndPreferred: ::minwindef::DWORD) -> ::winnt::HANDLE; /* memoryapi.h:476:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreatePrivateNamespaceW as CreatePrivateNamespace; /* winbase.h:7223:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::CreateSymbolicLinkW as CreateSymbolicLink; /* winbase.h:8534:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
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
    pub fn CreateWaitableTimerExW(lpTimerAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpTimerName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, dwDesiredAccess: ::minwindef::DWORD) -> ::winnt::HANDLE; /* synchapi.h:836:1 */
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
    pub fn FindNLSString(Locale: ::winnt::LCID, dwFindNLSStringFlags: ::minwindef::DWORD, lpStringSource: ::winnt::LPCWSTR, cchSource: ::libc::c_int, lpStringValue: ::winnt::LPCWSTR, cchValue: ::libc::c_int, pcchFound: ::minwindef::LPINT) -> ::libc::c_int; /* winnls.h:1460:1 */
    pub fn FreeLibraryWhenCallbackReturns(pci: ::winnt::PTP_CALLBACK_INSTANCE, mod_: ::minwindef::HMODULE); /* threadpoolapiset.h:181:1 */
    pub fn GetErrorMode() -> ::minwindef::UINT; /* errhandlingapi.h:146:1 */
    pub fn GetFileMUIInfo(dwFlags: ::minwindef::DWORD, pcwszFilePath: ::winnt::PCWSTR, pFileMUIInfo: ::winnls::PFILEMUIINFO, pcbFileMUIInfo: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winnls.h:2084:1 */
    pub fn GetFileMUIPath(dwFlags: ::minwindef::DWORD, pcwszFilePath: ::winnt::PCWSTR, pwszLanguage: ::winnt::PWSTR, pcchLanguage: ::minwindef::PULONG, pwszFileMUIPath: ::winnt::PWSTR, pcchFileMUIPath: ::minwindef::PULONG, pululEnumerator: ::winnt::PULONGLONG) -> ::minwindef::BOOL; /* winnls.h:2093:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::GetFinalPathNameByHandleW as GetFinalPathNameByHandle; /* fileapi.h:694:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn GetFinalPathNameByHandleA(hFile: ::winnt::HANDLE, lpszFilePath: ::winnt::LPSTR, cchFilePath: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:676:1 */
    pub fn GetFinalPathNameByHandleW(hFile: ::winnt::HANDLE, lpszFilePath: ::winnt::LPWSTR, cchFilePath: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* fileapi.h:686:1 */
    pub fn GetNamedPipeClientComputerNameW(Pipe: ::winnt::HANDLE, ClientComputerName: ::winnt::LPWSTR, ClientComputerNameLength: ::minwindef::ULONG) -> ::minwindef::BOOL; /* namedpipeapi.h:151:1 */
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
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::NotifyServiceStatusChangeW as NotifyServiceStatusChange; /* winsvc.h:1626:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn NotifyServiceStatusChangeA(hService: ::winsvc::SC_HANDLE, dwNotifyMask: ::minwindef::DWORD, pNotifyBuffer: ::winsvc::PSERVICE_NOTIFYA) -> ::minwindef::DWORD; /* winsvc.h:1612:1 */
    pub fn NotifyServiceStatusChangeW(hService: ::winsvc::SC_HANDLE, dwNotifyMask: ::minwindef::DWORD, pNotifyBuffer: ::winsvc::PSERVICE_NOTIFYW) -> ::minwindef::DWORD; /* winsvc.h:1620:1 */
    pub fn OpenFileById(hVolumeHint: ::winnt::HANDLE, lpFileId: ::winbase::LPFILE_ID_DESCRIPTOR, dwDesiredAccess: ::minwindef::DWORD, dwShareMode: ::minwindef::DWORD, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, dwFlagsAndAttributes: ::minwindef::DWORD) -> ::winnt::HANDLE; /* winbase.h:8490:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::OpenPrivateNamespaceW as OpenPrivateNamespace; /* winbase.h:7238:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::QueryFullProcessImageNameW as QueryFullProcessImageName; /* winbase.h:3243:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn QueryFullProcessImageNameW(hProcess: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD, lpExeName: ::winnt::LPWSTR, lpdwSize: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:3236:1 */
    pub fn QueryIdleProcessorCycleTime(BufferLength: ::minwindef::PULONG, ProcessorIdleCycleTime: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:63:1 */
    pub fn QueryProcessAffinityUpdateMode(hProcess: ::winnt::HANDLE, lpdwFlags: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:780:1 */
    pub fn QueryProcessCycleTime(ProcessHandle: ::winnt::HANDLE, CycleTime: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:54:1 */
    pub fn QuerySecurityAccessMask(SecurityInformation: ::winnt::SECURITY_INFORMATION, DesiredAccess: ::minwindef::LPDWORD); /* securitybaseapi.h:1138:1 */
    pub fn QueryServiceDynamicInformation(hServiceStatus: ::winsvc::SERVICE_STATUS_HANDLE, dwInfoLevel: ::minwindef::DWORD, ppDynamicInfo: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winsvc.h:1658:1 */
    pub fn QueryThreadCycleTime(ThreadHandle: ::winnt::HANDLE, CycleTime: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:45:1 */
    pub fn QueryThreadpoolStackInformation(ptpp: ::winnt::PTP_POOL, ptpsi: ::winnt::PTP_POOL_STACK_INFORMATION) -> ::minwindef::BOOL; /* threadpoolapiset.h:100:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegCopyTreeW as RegCopyTree; /* winreg.h:1195:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegCopyTreeW(hKeySrc: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, hKeyDest: ::minwindef::HKEY) -> ::winreg::LSTATUS; /* winreg.h:1188:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegDeleteTreeW as RegDeleteTree; /* winreg.h:1114:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegDeleteTreeA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:1100:1 */
    pub fn RegDeleteTreeW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:1108:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegGetValueW as RegGetValue; /* winreg.h:1175:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegGetValueA(hkey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, lpValue: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD, pdwType: ::minwindef::LPDWORD, pvData: ::winnt::PVOID, pcbData: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:1137:1 */
    pub fn RegGetValueW(hkey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, lpValue: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD, pdwType: ::minwindef::LPDWORD, pvData: ::winnt::PVOID, pcbData: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:1157:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegLoadAppKeyW as RegLoadAppKey; /* winreg.h:1254:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegLoadAppKeyA(lpFile: ::winnt::LPCSTR, phkResult: ::minwindef::PHKEY, samDesired: ::winreg::REGSAM, dwOptions: ::minwindef::DWORD, Reserved: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1234:1 */
    pub fn RegLoadAppKeyW(lpFile: ::winnt::LPCWSTR, phkResult: ::minwindef::PHKEY, samDesired: ::winreg::REGSAM, dwOptions: ::minwindef::DWORD, Reserved: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1245:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegLoadMUIStringW as RegLoadMUIString; /* winreg.h:1226:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegLoadMUIStringA(hKey: ::minwindef::HKEY, pszValue: ::winnt::LPCSTR, pszOutBuf: ::winnt::LPSTR, cbOutBuf: ::minwindef::DWORD, pcbData: ::minwindef::LPDWORD, Flags: ::minwindef::DWORD, pszDirectory: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:1202:1 */
    pub fn RegLoadMUIStringW(hKey: ::minwindef::HKEY, pszValue: ::winnt::LPCWSTR, pszOutBuf: ::winnt::LPWSTR, cbOutBuf: ::minwindef::DWORD, pcbData: ::minwindef::LPDWORD, Flags: ::minwindef::DWORD, pszDirectory: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:1215:1 */
    pub fn ReleaseMutexWhenCallbackReturns(pci: ::winnt::PTP_CALLBACK_INSTANCE, mut_: ::winnt::HANDLE); /* threadpoolapiset.h:163:1 */
    pub fn ReleaseSemaphoreWhenCallbackReturns(pci: ::winnt::PTP_CALLBACK_INSTANCE, sem: ::winnt::HANDLE, crel: ::minwindef::DWORD); /* threadpoolapiset.h:153:1 */
    pub fn SetDynamicTimeZoneInformation(lpTimeZoneInformation: *const ::timezoneapi::DYNAMIC_TIME_ZONE_INFORMATION) -> ::minwindef::BOOL; /* timezoneapi.h:141:1 */
    pub fn SetEventWhenCallbackReturns(pci: ::winnt::PTP_CALLBACK_INSTANCE, evt: ::winnt::HANDLE); /* threadpoolapiset.h:144:1 */
    pub fn SetFileIoOverlappedRange(FileHandle: ::winnt::HANDLE, OverlappedRangeStart: ::minwindef::PUCHAR, Length: ::minwindef::ULONG) -> ::minwindef::BOOL; /* fileapi.h:1304:1 */
    pub fn SetProcessAffinityUpdateMode(hProcess: ::winnt::HANDLE, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:771:1 */
    pub fn SetProcessPreferredUILanguages(dwFlags: ::minwindef::DWORD, pwszLanguagesBuffer: ::winnt::PCZZWSTR, pulNumLanguages: ::minwindef::PULONG) -> ::minwindef::BOOL; /* winnls.h:2032:1 */
    pub fn SetSecurityAccessMask(SecurityInformation: ::winnt::SECURITY_INFORMATION, DesiredAccess: ::minwindef::LPDWORD); /* securitybaseapi.h:1219:1 */
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
    pub fn UpdateProcThreadAttribute(lpAttributeList: ::processthreadsapi::LPPROC_THREAD_ATTRIBUTE_LIST, dwFlags: ::minwindef::DWORD, Attribute: ::basetsd::DWORD_PTR, lpValue: ::winnt::PVOID, cbSize: ::basetsd::SIZE_T, lpPreviousValue: ::winnt::PVOID, lpReturnSize: ::basetsd::PSIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:791:1 */
    pub fn VirtualAllocExNuma(hProcess: ::winnt::HANDLE, lpAddress: ::minwindef::LPVOID, dwSize: ::basetsd::SIZE_T, flAllocationType: ::minwindef::DWORD, flProtect: ::minwindef::DWORD, nndPreferred: ::minwindef::DWORD) -> ::minwindef::LPVOID; /* memoryapi.h:632:1 */
    pub fn WaitForThreadpoolIoCallbacks(pio: ::winnt::PTP_IO, fCancelPendingCallbacks: ::minwindef::BOOL); /* threadpoolapiset.h:366:1 */
    pub fn WaitForThreadpoolTimerCallbacks(pti: ::winnt::PTP_TIMER, fCancelPendingCallbacks: ::minwindef::BOOL); /* threadpoolapiset.h:283:1 */
    pub fn WaitForThreadpoolWaitCallbacks(pwa: ::winnt::PTP_WAIT, fCancelPendingCallbacks: ::minwindef::BOOL); /* threadpoolapiset.h:321:1 */
    pub fn WaitForThreadpoolWorkCallbacks(pwk: ::winnt::PTP_WORK, fCancelPendingCallbacks: ::minwindef::BOOL); /* threadpoolapiset.h:236:1 */
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
    pub fn EnumSystemLocalesEx(lpLocaleEnumProcEx: ::winnls::LOCALE_ENUMPROCEX, dwFlags: ::minwindef::DWORD, lParam: ::minwindef::LPARAM, lpReserved: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winnls.h:2554:1 */
    pub fn FindNLSStringEx(lpLocaleName: ::winnt::LPCWSTR, dwFindNLSStringFlags: ::minwindef::DWORD, lpStringSource: ::winnt::LPCWSTR, cchSource: ::libc::c_int, lpStringValue: ::winnt::LPCWSTR, cchValue: ::libc::c_int, pcchFound: ::minwindef::LPINT, lpVersionInformation: ::winnls::LPNLSVERSIONINFO, lpReserved: ::minwindef::LPVOID, sortHandle: ::minwindef::LPARAM) -> ::libc::c_int; /* winnls.h:2469:1 */
    pub fn FlsAlloc(lpCallback: ::winnt::PFLS_CALLBACK_FUNCTION) -> ::minwindef::DWORD; /* fibersapi.h:58:1 */
    pub fn FlsFree(dwFlsIndex: ::minwindef::DWORD) -> ::minwindef::BOOL; /* fibersapi.h:83:1 */
    pub fn FlsGetValue(dwFlsIndex: ::minwindef::DWORD) -> ::winnt::PVOID; /* fibersapi.h:66:1 */
    pub fn FlsSetValue(dwFlsIndex: ::minwindef::DWORD, lpFlsData: ::winnt::PVOID) -> ::minwindef::BOOL; /* fibersapi.h:74:1 */
    pub fn FlushProcessWriteBuffers(); /* processthreadsapi.h:726:1 */
    pub fn GetCalendarInfoEx(lpLocaleName: ::winnt::LPCWSTR, Calendar: ::winnls::CALID, lpReserved: ::winnt::LPCWSTR, CalType: ::winnls::CALTYPE, lpCalData: ::winnt::LPWSTR, cchData: ::libc::c_int, lpValue: ::minwindef::LPDWORD) -> ::libc::c_int; /* winnls.h:2383:1 */
    pub fn GetDynamicTimeZoneInformation(pTimeZoneInformation: ::timezoneapi::PDYNAMIC_TIME_ZONE_INFORMATION) -> ::minwindef::DWORD; /* timezoneapi.h:162:1 */
    pub fn GetFileInformationByHandleEx(hFile: ::winnt::HANDLE, FileInformationClass: ::minwinbase::FILE_INFO_BY_HANDLE_CLASS, lpFileInformation: ::minwindef::LPVOID, dwBufferSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:8455:1 */
    pub fn GetLocaleInfoEx(lpLocaleName: ::winnt::LPCWSTR, LCType: ::winnls::LCTYPE, lpLCData: ::winnt::LPWSTR, cchData: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2373:1 */
    pub fn GetNLSVersionEx(function: ::winnls::NLS_FUNCTION, lpLocaleName: ::winnt::LPCWSTR, lpVersionInformation: ::winnls::LPNLSVERSIONINFOEX) -> ::minwindef::BOOL; /* winnls.h:2449:1 */
    pub fn GetTickCount64() -> ::winnt::ULONGLONG; /* sysinfoapi.h:221:1 */
    pub fn GetUserDefaultLocaleName(lpLocaleName: ::winnt::LPWSTR, cchLocaleName: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2433:1 */
    pub fn IdnToAscii(dwFlags: ::minwindef::DWORD, lpUnicodeCharStr: ::winnt::LPCWSTR, cchUnicodeChar: ::libc::c_int, lpASCIICharStr: ::winnt::LPWSTR, cchASCIIChar: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2312:8 */
    pub fn IdnToUnicode(dwFlags: ::minwindef::DWORD, lpASCIICharStr: ::winnt::LPCWSTR, cchASCIIChar: ::libc::c_int, lpUnicodeCharStr: ::winnt::LPWSTR, cchUnicodeChar: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2328:8 */
    pub fn InitOnceBeginInitialize(lpInitOnce: ::synchapi::LPINIT_ONCE, dwFlags: ::minwindef::DWORD, fPending: ::minwindef::PBOOL, lpContext: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* synchapi.h:337:1 */
    pub fn InitOnceComplete(lpInitOnce: ::synchapi::LPINIT_ONCE, dwFlags: ::minwindef::DWORD, lpContext: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* synchapi.h:348:1 */
    pub fn InitOnceExecuteOnce(InitOnce: ::synchapi::PINIT_ONCE, InitFn: ::synchapi::PINIT_ONCE_FN, Parameter: ::winnt::PVOID, Context: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* synchapi.h:326:1 */
    pub fn InitOnceInitialize(InitOnce: ::synchapi::PINIT_ONCE); /* synchapi.h:318:1 */
    pub fn InitializeConditionVariable(ConditionVariable: ::synchapi::PCONDITION_VARIABLE); /* synchapi.h:381:1 */
    pub fn InitializeCriticalSectionEx(lpCriticalSection: ::minwinbase::LPCRITICAL_SECTION, dwSpinCount: ::minwindef::DWORD, Flags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:222:1 */
    pub fn InitializeSRWLock(SRWLock: ::synchapi::PSRWLOCK); /* synchapi.h:77:1 */
    pub fn IsThreadAFiber() -> ::minwindef::BOOL; /* fibersapi.h:107:1 */
    pub fn IsValidLocaleName(lpLocaleName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winnls.h:2507:1 */
    pub fn LCMapStringEx(lpLocaleName: ::winnt::LPCWSTR, dwMapFlags: ::minwindef::DWORD, lpSrcStr: ::winnt::LPCWSTR, cchSrc: ::libc::c_int, lpDestStr: ::winnt::LPWSTR, cchDest: ::libc::c_int, lpVersionInformation: ::winnls::LPNLSVERSIONINFO, lpReserved: ::minwindef::LPVOID, sortHandle: ::minwindef::LPARAM) -> ::libc::c_int; /* winnls.h:2492:1 */
    pub fn LocaleNameToLCID(lpName: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD) -> ::winnt::LCID; /* winnls.h:1661:1 */
    pub fn ReleaseSRWLockExclusive(SRWLock: ::synchapi::PSRWLOCK); /* synchapi.h:86:1 */
    pub fn ReleaseSRWLockShared(SRWLock: ::synchapi::PSRWLOCK); /* synchapi.h:95:1 */
    pub fn SleepConditionVariableCS(ConditionVariable: ::synchapi::PCONDITION_VARIABLE, CriticalSection: ::minwinbase::PCRITICAL_SECTION, dwMilliseconds: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:405:1 */
    pub fn SleepConditionVariableSRW(ConditionVariable: ::synchapi::PCONDITION_VARIABLE, SRWLock: ::synchapi::PSRWLOCK, dwMilliseconds: ::minwindef::DWORD, Flags: ::minwindef::ULONG) -> ::minwindef::BOOL; /* synchapi.h:415:1 */
    pub fn TryAcquireSRWLockExclusive(SRWLock: ::synchapi::PSRWLOCK) -> ::winnt::BOOLEAN; /* synchapi.h:122:1 */
    pub fn TryAcquireSRWLockShared(SRWLock: ::synchapi::PSRWLOCK) -> ::winnt::BOOLEAN; /* synchapi.h:131:1 */
    pub fn WakeAllConditionVariable(ConditionVariable: ::synchapi::PCONDITION_VARIABLE); /* synchapi.h:397:1 */
    pub fn WakeConditionVariable(ConditionVariable: ::synchapi::PCONDITION_VARIABLE); /* synchapi.h:389:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn GetCurrentProcessorNumberEx(ProcNumber: ::winnt::PPROCESSOR_NUMBER); /* processthreadsapi.h:1041:1 */
    pub fn GetLogicalProcessorInformationEx(RelationshipType: ::winnt::LOGICAL_PROCESSOR_RELATIONSHIP, Buffer: ::winnt::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, ReturnedLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:467:1 */
    pub fn GetNumaNodeProcessorMaskEx(Node: ::minwindef::USHORT, ProcessorMask: ::winnt::PGROUP_AFFINITY) -> ::minwindef::BOOL; /* systemtopologyapi.h:54:1 */
    pub fn GetProcessGroupAffinity(hProcess: ::winnt::HANDLE, GroupCount: ::minwindef::PUSHORT, GroupArray: ::minwindef::PUSHORT) -> ::minwindef::BOOL; /* processtopologyapi.h:49:1 */
    pub fn GetThreadGroupAffinity(hThread: ::winnt::HANDLE, GroupAffinity: ::winnt::PGROUP_AFFINITY) -> ::minwindef::BOOL; /* processtopologyapi.h:72:1 */
    pub fn GetThreadIdealProcessorEx(hThread: ::winnt::HANDLE, lpIdealProcessor: ::winnt::PPROCESSOR_NUMBER) -> ::minwindef::BOOL; /* processthreadsapi.h:1032:1 */
    pub fn QueryIdleProcessorCycleTimeEx(Group: ::minwindef::USHORT, BufferLength: ::minwindef::PULONG, ProcessorIdleCycleTime: ::basetsd::PULONG64) -> ::minwindef::BOOL; /* realtimeapiset.h:77:1 */
    pub fn SetThreadGroupAffinity(hThread: ::winnt::HANDLE, GroupAffinity: *const ::winnt::GROUP_AFFINITY, PreviousGroupAffinity: ::winnt::PGROUP_AFFINITY) -> ::minwindef::BOOL; /* processtopologyapi.h:81:1 */
    pub fn SetWaitableTimerEx(hTimer: ::winnt::HANDLE, lpDueTime: *const ::winnt::LARGE_INTEGER, lPeriod: ::winnt::LONG, pfnCompletionRoutine: ::synchapi::PTIMERAPCROUTINE, lpArgToCompletionRoutine: ::minwindef::LPVOID, WakeContext: ::minwinbase::PREASON_CONTEXT, TolerableDelay: ::minwindef::ULONG) -> ::minwindef::BOOL; /* synchapi.h:700:1 */
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
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[cfg(any(target_arch="x86_64", target_arch="arm"))] 
extern "system" {
    pub fn RtlAddGrowableFunctionTable(DynamicTable: *mut *mut ::libc::c_void, FunctionTable: ::winnt::PRUNTIME_FUNCTION, EntryCount: ::minwindef::DWORD, MaximumEntryCount: ::minwindef::DWORD, RangeBase: ::basetsd::ULONG_PTR, RangeEnd: ::basetsd::ULONG_PTR) -> ::minwindef::DWORD; /* winnt.h:17010:1, winnt.h:17168:1 */
    pub fn RtlDeleteGrowableFunctionTable(DynamicTable: ::winnt::PVOID); /* winnt.h:17032:1, winnt.h:17190:1 */
    pub fn RtlGrowFunctionTable(DynamicTable: ::winnt::PVOID, NewEntryCount: ::minwindef::DWORD); /* winnt.h:17023:1, winnt.h:17181:1 */
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
    pub fn GetFirmwareEnvironmentVariableExW(lpName: ::winnt::LPCWSTR, lpGuid: ::winnt::LPCWSTR, pBuffer: ::winnt::PVOID, nSize: ::minwindef::DWORD, pdwAttribubutes: ::minwindef::PDWORD) -> ::minwindef::DWORD; /* winbase.h:3574:1 */
    pub fn GetMemoryErrorHandlingCapabilities(Capabilities: ::minwindef::PULONG) -> ::minwindef::BOOL; /* memoryapi.h:653:1 */
    pub fn GetOsSafeBootMode(Flags: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* sysinfoapi.h:547:1 */
    pub fn GetProcessMitigationPolicy(hProcess: ::winnt::HANDLE, MitigationPolicy: ::winnt::PROCESS_MITIGATION_POLICY, lpBuffer: ::winnt::PVOID, dwLength: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:890:1 */
    pub fn GetThreadInformation(hThread: ::winnt::HANDLE, ThreadInformationClass: ::processthreadsapi::THREAD_INFORMATION_CLASS, ThreadInformation: ::minwindef::LPVOID, ThreadInformationSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:1119:1 */
    pub fn PrefetchVirtualMemory(hProcess: ::winnt::HANDLE, NumberOfEntries: ::basetsd::ULONG_PTR, VirtualAddresses: ::memoryapi::PWIN32_MEMORY_RANGE_ENTRY, Flags: ::minwindef::ULONG) -> ::minwindef::BOOL; /* memoryapi.h:504:1 */
    pub fn RegisterBadMemoryNotification(Callback: ::memoryapi::PBAD_MEMORY_CALLBACK_ROUTINE) -> ::winnt::PVOID; /* memoryapi.h:672:1 */
    pub fn SetCachedSigningLevel(SourceFiles: ::winnt::PHANDLE, SourceFileCount: ::minwindef::ULONG, Flags: ::minwindef::ULONG, TargetFile: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:1307:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] #[doc(inline)] pub use self::SetFirmwareEnvironmentVariableExW as SetFirmwareEnvironmentVariableEx; /* winbase.h:3636:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn SetFirmwareEnvironmentVariableExW(lpName: ::winnt::LPCWSTR, lpGuid: ::winnt::LPCWSTR, pValue: ::winnt::PVOID, nSize: ::minwindef::DWORD, dwAttributes: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:3628:1 */
    pub fn SetProcessMitigationPolicy(MitigationPolicy: ::winnt::PROCESS_MITIGATION_POLICY, lpBuffer: ::winnt::PVOID, dwLength: ::basetsd::SIZE_T) -> ::minwindef::BOOL; /* processthreadsapi.h:880:1 */
    pub fn SetThreadInformation(hThread: ::winnt::HANDLE, ThreadInformationClass: ::processthreadsapi::THREAD_INFORMATION_CLASS, ThreadInformation: ::minwindef::LPVOID, ThreadInformationSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* processthreadsapi.h:1130:1 */
    pub fn UnmapViewOfFileEx(BaseAddress: ::winnt::PVOID, UnmapFlags: ::minwindef::ULONG) -> ::minwindef::BOOL; /* memoryapi.h:515:1 */
    pub fn UnregisterBadMemoryNotification(RegistrationHandle: ::winnt::PVOID) -> ::minwindef::BOOL; /* memoryapi.h:681:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn CreateFile2(lpFileName: ::winnt::LPCWSTR, dwDesiredAccess: ::minwindef::DWORD, dwShareMode: ::minwindef::DWORD, dwCreationDisposition: ::minwindef::DWORD, pCreateExParams: ::fileapi::LPCREATEFILE2_EXTENDED_PARAMETERS) -> ::winnt::HANDLE; /* fileapi.h:1272:1 */
    pub fn CreateFileMappingFromApp(hFile: ::winnt::HANDLE, SecurityAttributes: ::minwinbase::PSECURITY_ATTRIBUTES, PageProtection: ::minwindef::ULONG, MaximumSize: ::basetsd::ULONG64, Name: ::winnt::PCWSTR) -> ::winnt::HANDLE; /* memoryapi.h:537:1 */
    pub fn EnumDynamicTimeZoneInformation(dwIndex: ::minwindef::DWORD, lpTimeZoneInformation: ::timezoneapi::PDYNAMIC_TIME_ZONE_INFORMATION) -> ::minwindef::DWORD; /* timezoneapi.h:193:1 */
    pub fn GetDynamicTimeZoneInformationEffectiveYears(lpTimeZoneInformation: ::timezoneapi::PDYNAMIC_TIME_ZONE_INFORMATION, FirstYear: ::minwindef::LPDWORD, LastYear: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* timezoneapi.h:203:1 */
    pub fn IsValidNLSVersion(function: ::winnls::NLS_FUNCTION, lpLocaleName: ::winnt::LPCWSTR, lpVersionInformation: ::winnls::LPNLSVERSIONINFOEX) -> ::minwindef::DWORD; /* winnls.h:2459:1 */
    pub fn MapViewOfFileFromApp(hFileMappingObject: ::winnt::HANDLE, DesiredAccess: ::minwindef::ULONG, FileOffset: ::basetsd::ULONG64, NumberOfBytesToMap: ::basetsd::SIZE_T) -> ::winnt::PVOID; /* memoryapi.h:550:1 */
    pub fn SystemTimeToTzSpecificLocalTimeEx(lpTimeZoneInformation: *const ::timezoneapi::DYNAMIC_TIME_ZONE_INFORMATION, lpUniversalTime: *const ::minwinbase::SYSTEMTIME, lpLocalTime: ::minwinbase::LPSYSTEMTIME) -> ::minwindef::BOOL; /* timezoneapi.h:214:1 */
    pub fn TzSpecificLocalTimeToSystemTimeEx(lpTimeZoneInformation: *const ::timezoneapi::DYNAMIC_TIME_ZONE_INFORMATION, lpLocalTime: *const ::minwinbase::SYSTEMTIME, lpUniversalTime: ::minwinbase::LPSYSTEMTIME) -> ::minwindef::BOOL; /* timezoneapi.h:225:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_0a000000"))] 
extern "system" {
    pub fn QueryProtectedPolicy(PolicyGuid: ::guiddef::LPCGUID, PolicyValue: ::basetsd::PULONG_PTR) -> ::minwindef::BOOL; /* processthreadsapi.h:1185:1 */
}
