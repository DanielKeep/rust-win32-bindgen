#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AbortSystemShutdownW as AbortSystemShutdown; /* winreg.h:1307:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AbortSystemShutdownA(lpMachineName: ::winnt::LPSTR) -> ::minwindef::BOOL; /* winreg.h:1297:1 */
    pub fn AbortSystemShutdownW(lpMachineName: ::winnt::LPWSTR) -> ::minwindef::BOOL; /* winreg.h:1303:1 */
    pub fn AccessCheck(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, ClientToken: ::winnt::HANDLE, DesiredAccess: ::minwindef::DWORD, GenericMapping: ::winnt::PGENERIC_MAPPING, PrivilegeSet: ::winnt::PPRIVILEGE_SET, PrivilegeSetLength: ::minwindef::LPDWORD, GrantedAccess: ::minwindef::LPDWORD, AccessStatus: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:56:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AccessCheckAndAuditAlarmW as AccessCheckAndAuditAlarm; /* securitybaseapi.h:87:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AccessCheckAndAuditAlarmA(SubsystemName: ::winnt::LPCSTR, HandleId: ::minwindef::LPVOID, ObjectTypeName: ::winnt::LPSTR, ObjectName: ::winnt::LPSTR, SecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, DesiredAccess: ::minwindef::DWORD, GenericMapping: ::winnt::PGENERIC_MAPPING, ObjectCreation: ::minwindef::BOOL, GrantedAccess: ::minwindef::LPDWORD, AccessStatus: ::minwindef::LPBOOL, pfGenerateOnClose: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* winbase.h:6152:1 */
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
    pub fn AdjustTokenGroups(TokenHandle: ::winnt::HANDLE, ResetToDefault: ::minwindef::BOOL, NewState: ::winnt::PTOKEN_GROUPS, BufferLength: ::minwindef::DWORD, PreviousState: ::winnt::PTOKEN_GROUPS, ReturnLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:392:1 */
    pub fn AdjustTokenPrivileges(TokenHandle: ::winnt::HANDLE, DisableAllPrivileges: ::minwindef::BOOL, NewState: ::winnt::PTOKEN_PRIVILEGES, BufferLength: ::minwindef::DWORD, PreviousState: ::winnt::PTOKEN_PRIVILEGES, ReturnLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:405:1 */
    pub fn AllocateAndInitializeSid(pIdentifierAuthority: ::winnt::PSID_IDENTIFIER_AUTHORITY, nSubAuthorityCount: ::minwindef::BYTE, nSubAuthority0: ::minwindef::DWORD, nSubAuthority1: ::minwindef::DWORD, nSubAuthority2: ::minwindef::DWORD, nSubAuthority3: ::minwindef::DWORD, nSubAuthority4: ::minwindef::DWORD, nSubAuthority5: ::minwindef::DWORD, nSubAuthority6: ::minwindef::DWORD, nSubAuthority7: ::minwindef::DWORD, pSid: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:418:1 */
    pub fn AllocateLocallyUniqueId(Luid: ::winnt::PLUID) -> ::minwindef::BOOL; /* securitybaseapi.h:436:1 */
    pub fn AreAllAccessesGranted(GrantedAccess: ::minwindef::DWORD, DesiredAccess: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:444:1 */
    pub fn AreAnyAccessesGranted(GrantedAccess: ::minwindef::DWORD, DesiredAccess: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:453:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::BackupEventLogW as BackupEventLog; /* winbase.h:5907:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn BackupEventLogA(hEventLog: ::winnt::HANDLE, lpBackupFileName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:5895:1 */
    pub fn BackupEventLogW(hEventLog: ::winnt::HANDLE, lpBackupFileName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:5902:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ChangeServiceConfigW as ChangeServiceConfig; /* winsvc.h:1035:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ChangeServiceConfig2W as ChangeServiceConfig2; /* winsvc.h:1057:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ChangeServiceConfig2A(hService: ::winsvc::SC_HANDLE, dwInfoLevel: ::minwindef::DWORD, lpInfo: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winsvc.h:1043:1 */
    pub fn ChangeServiceConfig2W(hService: ::winsvc::SC_HANDLE, dwInfoLevel: ::minwindef::DWORD, lpInfo: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winsvc.h:1051:1 */
    pub fn ChangeServiceConfigA(hService: ::winsvc::SC_HANDLE, dwServiceType: ::minwindef::DWORD, dwStartType: ::minwindef::DWORD, dwErrorControl: ::minwindef::DWORD, lpBinaryPathName: ::winnt::LPCSTR, lpLoadOrderGroup: ::winnt::LPCSTR, lpdwTagId: ::minwindef::LPDWORD, lpDependencies: ::winnt::LPCSTR, lpServiceStartName: ::winnt::LPCSTR, lpPassword: ::winnt::LPCSTR, lpDisplayName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winsvc.h:1005:1 */
    pub fn ChangeServiceConfigW(hService: ::winsvc::SC_HANDLE, dwServiceType: ::minwindef::DWORD, dwStartType: ::minwindef::DWORD, dwErrorControl: ::minwindef::DWORD, lpBinaryPathName: ::winnt::LPCWSTR, lpLoadOrderGroup: ::winnt::LPCWSTR, lpdwTagId: ::minwindef::LPDWORD, lpDependencies: ::winnt::LPCWSTR, lpServiceStartName: ::winnt::LPCWSTR, lpPassword: ::winnt::LPCWSTR, lpDisplayName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winsvc.h:1021:1 */
    pub fn CheckForHiberboot(pHiberboot: ::winnt::PBOOLEAN, bClearFlag: ::winnt::BOOLEAN) -> ::minwindef::DWORD; /* winreg.h:1414:1 */
    pub fn CheckTokenMembership(TokenHandle: ::winnt::HANDLE, SidToCheck: ::winnt::PSID, IsMember: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:462:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ClearEventLogW as ClearEventLog; /* winbase.h:5887:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ClearEventLogA(hEventLog: ::winnt::HANDLE, lpBackupFileName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:5875:1 */
    pub fn ClearEventLogW(hEventLog: ::winnt::HANDLE, lpBackupFileName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:5882:1 */
    pub fn CloseEncryptedFileRaw(pvContext: ::winnt::PVOID); /* winbase.h:2616:1 */
    pub fn CloseEventLog(hEventLog: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5915:1 */
    pub fn CloseServiceHandle(hSCObject: ::winsvc::SC_HANDLE) -> ::minwindef::BOOL; /* winsvc.h:1065:1 */
    pub fn ControlService(hService: ::winsvc::SC_HANDLE, dwControl: ::minwindef::DWORD, lpServiceStatus: ::winsvc::LPSERVICE_STATUS) -> ::minwindef::BOOL; /* winsvc.h:1072:1 */
    pub fn ConvertToAutoInheritPrivateObjectSecurity(ParentDescriptor: ::winnt::PSECURITY_DESCRIPTOR, CurrentSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, NewSecurityDescriptor: *mut *mut ::libc::c_void, ObjectType: *mut ::guiddef::GUID, IsDirectoryObject: ::winnt::BOOLEAN, GenericMapping: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:513:1 */
    pub fn CopySid(nDestinationSidLength: ::minwindef::DWORD, pDestinationSid: ::winnt::PSID, pSourceSid: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:526:1 */
    pub fn CreatePrivateObjectSecurity(ParentDescriptor: ::winnt::PSECURITY_DESCRIPTOR, CreatorDescriptor: ::winnt::PSECURITY_DESCRIPTOR, NewDescriptor: *mut *mut ::libc::c_void, IsDirectoryObject: ::minwindef::BOOL, Token: ::winnt::HANDLE, GenericMapping: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:536:1 */
    pub fn CreatePrivateObjectSecurityEx(ParentDescriptor: ::winnt::PSECURITY_DESCRIPTOR, CreatorDescriptor: ::winnt::PSECURITY_DESCRIPTOR, NewDescriptor: *mut *mut ::libc::c_void, ObjectType: *mut ::guiddef::GUID, IsContainerObject: ::minwindef::BOOL, AutoInheritFlags: ::minwindef::ULONG, Token: ::winnt::HANDLE, GenericMapping: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:549:1 */
    pub fn CreatePrivateObjectSecurityWithMultipleInheritance(ParentDescriptor: ::winnt::PSECURITY_DESCRIPTOR, CreatorDescriptor: ::winnt::PSECURITY_DESCRIPTOR, NewDescriptor: *mut *mut ::libc::c_void, ObjectTypes: *mut *mut ::guiddef::GUID, GuidCount: ::minwindef::ULONG, IsContainerObject: ::minwindef::BOOL, AutoInheritFlags: ::minwindef::ULONG, Token: ::winnt::HANDLE, GenericMapping: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:564:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateProcessAsUserW as CreateProcessAsUser; /* processthreadsapi.h:597:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateProcessAsUserA(hToken: ::winnt::HANDLE, lpApplicationName: ::winnt::LPCSTR, lpCommandLine: ::winnt::LPSTR, lpProcessAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpThreadAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, bInheritHandles: ::minwindef::BOOL, dwCreationFlags: ::minwindef::DWORD, lpEnvironment: ::minwindef::LPVOID, lpCurrentDirectory: ::winnt::LPCSTR, lpStartupInfo: ::processthreadsapi::LPSTARTUPINFOA, lpProcessInformation: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* winbase.h:6987:1 */
    pub fn CreateProcessAsUserW(hToken: ::winnt::HANDLE, lpApplicationName: ::winnt::LPCWSTR, lpCommandLine: ::winnt::LPWSTR, lpProcessAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, lpThreadAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, bInheritHandles: ::minwindef::BOOL, dwCreationFlags: ::minwindef::DWORD, lpEnvironment: ::minwindef::LPVOID, lpCurrentDirectory: ::winnt::LPCWSTR, lpStartupInfo: ::processthreadsapi::LPSTARTUPINFOW, lpProcessInformation: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* processthreadsapi.h:581:1 */
    pub fn CreateRestrictedToken(ExistingTokenHandle: ::winnt::HANDLE, Flags: ::minwindef::DWORD, DisableSidCount: ::minwindef::DWORD, SidsToDisable: ::winnt::PSID_AND_ATTRIBUTES, DeletePrivilegeCount: ::minwindef::DWORD, PrivilegesToDelete: ::winnt::PLUID_AND_ATTRIBUTES, RestrictedSidCount: ::minwindef::DWORD, SidsToRestrict: ::winnt::PSID_AND_ATTRIBUTES, NewTokenHandle: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:580:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateServiceW as CreateService; /* winsvc.h:1117:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateServiceA(hSCManager: ::winsvc::SC_HANDLE, lpServiceName: ::winnt::LPCSTR, lpDisplayName: ::winnt::LPCSTR, dwDesiredAccess: ::minwindef::DWORD, dwServiceType: ::minwindef::DWORD, dwStartType: ::minwindef::DWORD, dwErrorControl: ::minwindef::DWORD, lpBinaryPathName: ::winnt::LPCSTR, lpLoadOrderGroup: ::winnt::LPCSTR, lpdwTagId: ::minwindef::LPDWORD, lpDependencies: ::winnt::LPCSTR, lpServiceStartName: ::winnt::LPCSTR, lpPassword: ::winnt::LPCSTR) -> ::winsvc::SC_HANDLE; /* winsvc.h:1082:1 */
    pub fn CreateServiceW(hSCManager: ::winsvc::SC_HANDLE, lpServiceName: ::winnt::LPCWSTR, lpDisplayName: ::winnt::LPCWSTR, dwDesiredAccess: ::minwindef::DWORD, dwServiceType: ::minwindef::DWORD, dwStartType: ::minwindef::DWORD, dwErrorControl: ::minwindef::DWORD, lpBinaryPathName: ::winnt::LPCWSTR, lpLoadOrderGroup: ::winnt::LPCWSTR, lpdwTagId: ::minwindef::LPDWORD, lpDependencies: ::winnt::LPCWSTR, lpServiceStartName: ::winnt::LPCWSTR, lpPassword: ::winnt::LPCWSTR) -> ::winsvc::SC_HANDLE; /* winsvc.h:1101:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DecryptFileW as DecryptFile; /* winbase.h:2500:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DecryptFileA(lpFileName: ::winnt::LPCSTR, dwReserved: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2488:1 */
    pub fn DecryptFileW(lpFileName: ::winnt::LPCWSTR, dwReserved: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2495:1 */
    pub fn DeleteAce(pAcl: ::winnt::PACL, dwAceIndex: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:624:1 */
    pub fn DeleteService(hService: ::winsvc::SC_HANDLE) -> ::minwindef::BOOL; /* winsvc.h:1125:1 */
    pub fn DeregisterEventSource(hEventLog: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5922:1 */
    pub fn DestroyPrivateObjectSecurity(ObjectDescriptor: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:633:1 */
    pub fn DuplicateToken(ExistingTokenHandle: ::winnt::HANDLE, ImpersonationLevel: ::winnt::SECURITY_IMPERSONATION_LEVEL, DuplicateTokenHandle: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:641:1 */
    pub fn DuplicateTokenEx(hExistingToken: ::winnt::HANDLE, dwDesiredAccess: ::minwindef::DWORD, lpTokenAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, ImpersonationLevel: ::winnt::SECURITY_IMPERSONATION_LEVEL, TokenType: ::winnt::TOKEN_TYPE, phNewToken: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:651:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EncryptFileW as EncryptFile; /* winbase.h:2480:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EncryptFileA(lpFileName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:2470:1 */
    pub fn EncryptFileW(lpFileName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:2476:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumDependentServicesW as EnumDependentServices; /* winsvc.h:1156:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumDependentServicesA(hService: ::winsvc::SC_HANDLE, dwServiceState: ::minwindef::DWORD, lpServices: ::winsvc::LPENUM_SERVICE_STATUSA, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD, lpServicesReturned: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1133:1 */
    pub fn EnumDependentServicesW(hService: ::winsvc::SC_HANDLE, dwServiceState: ::minwindef::DWORD, lpServices: ::winsvc::LPENUM_SERVICE_STATUSW, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD, lpServicesReturned: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1146:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumServicesStatusW as EnumServicesStatus; /* winsvc.h:1192:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumServicesStatusA(hSCManager: ::winsvc::SC_HANDLE, dwServiceType: ::minwindef::DWORD, dwServiceState: ::minwindef::DWORD, lpServices: ::winsvc::LPENUM_SERVICE_STATUSA, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD, lpServicesReturned: ::minwindef::LPDWORD, lpResumeHandle: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1165:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumServicesStatusExW as EnumServicesStatusEx; /* winsvc.h:1232:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumServicesStatusExA(hSCManager: ::winsvc::SC_HANDLE, InfoLevel: ::winsvc::SC_ENUM_TYPE, dwServiceType: ::minwindef::DWORD, dwServiceState: ::minwindef::DWORD, lpServices: ::minwindef::LPBYTE, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD, lpServicesReturned: ::minwindef::LPDWORD, lpResumeHandle: ::minwindef::LPDWORD, pszGroupName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winsvc.h:1201:1 */
    pub fn EnumServicesStatusExW(hSCManager: ::winsvc::SC_HANDLE, InfoLevel: ::winsvc::SC_ENUM_TYPE, dwServiceType: ::minwindef::DWORD, dwServiceState: ::minwindef::DWORD, lpServices: ::minwindef::LPBYTE, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD, lpServicesReturned: ::minwindef::LPDWORD, lpResumeHandle: ::minwindef::LPDWORD, pszGroupName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winsvc.h:1218:1 */
    pub fn EnumServicesStatusW(hSCManager: ::winsvc::SC_HANDLE, dwServiceType: ::minwindef::DWORD, dwServiceState: ::minwindef::DWORD, lpServices: ::winsvc::LPENUM_SERVICE_STATUSW, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD, lpServicesReturned: ::minwindef::LPDWORD, lpResumeHandle: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1180:1 */
    pub fn EqualPrefixSid(pSid1: ::winnt::PSID, pSid2: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:664:1 */
    pub fn EqualSid(pSid1: ::winnt::PSID, pSid2: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:673:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FileEncryptionStatusW as FileEncryptionStatus; /* winbase.h:2535:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FileEncryptionStatusA(lpFileName: ::winnt::LPCSTR, lpStatus: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2523:1 */
    pub fn FileEncryptionStatusW(lpFileName: ::winnt::LPCWSTR, lpStatus: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2530:1 */
    pub fn FindFirstFreeAce(pAcl: ::winnt::PACL, pAce: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:682:1 */
    pub fn FreeSid(pSid: ::winnt::PSID) -> ::winnt::PVOID; /* securitybaseapi.h:691:1 */
    pub fn GetAce(pAcl: ::winnt::PACL, dwAceIndex: ::minwindef::DWORD, pAce: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:699:1 */
    pub fn GetAclInformation(pAcl: ::winnt::PACL, pAclInformation: ::minwindef::LPVOID, nAclInformationLength: ::minwindef::DWORD, dwAclInformationClass: ::winnt::ACL_INFORMATION_CLASS) -> ::minwindef::BOOL; /* securitybaseapi.h:709:1 */
    pub fn GetEventLogInformation(hEventLog: ::winnt::HANDLE, dwInfoLevel: ::minwindef::DWORD, lpBuffer: ::minwindef::LPVOID, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6086:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileSecurityW as GetFileSecurity; /* securitybaseapi.h:730:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileSecurityA(lpFileName: ::winnt::LPCSTR, RequestedInformation: ::winnt::SECURITY_INFORMATION, pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, nLength: ::minwindef::DWORD, lpnLengthNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6353:1 */
    pub fn GetFileSecurityW(lpFileName: ::winnt::LPCWSTR, RequestedInformation: ::winnt::SECURITY_INFORMATION, pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, nLength: ::minwindef::DWORD, lpnLengthNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:720:1 */
    pub fn GetKernelObjectSecurity(Handle: ::winnt::HANDLE, RequestedInformation: ::winnt::SECURITY_INFORMATION, pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, nLength: ::minwindef::DWORD, lpnLengthNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:736:1 */
    pub fn GetLengthSid(pSid: ::winnt::PSID) -> ::minwindef::DWORD; /* securitybaseapi.h:750:1 */
    pub fn GetNumberOfEventLogRecords(hEventLog: ::winnt::HANDLE, NumberOfRecords: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:5937:1 */
    pub fn GetOldestEventLogRecord(hEventLog: ::winnt::HANDLE, OldestRecord: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:5945:1 */
    pub fn GetPrivateObjectSecurity(ObjectDescriptor: ::winnt::PSECURITY_DESCRIPTOR, SecurityInformation: ::winnt::SECURITY_INFORMATION, ResultantDescriptor: ::winnt::PSECURITY_DESCRIPTOR, DescriptorLength: ::minwindef::DWORD, ReturnLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:759:1 */
    pub fn GetSecurityDescriptorControl(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, pControl: ::winnt::PSECURITY_DESCRIPTOR_CONTROL, lpdwRevision: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:771:1 */
    pub fn GetSecurityDescriptorDacl(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, lpbDaclPresent: ::minwindef::LPBOOL, pDacl: *mut *mut ::winnt::ACL, lpbDaclDefaulted: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:781:1 */
    pub fn GetSecurityDescriptorGroup(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, pGroup: *mut *mut ::libc::c_void, lpbGroupDefaulted: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:792:1 */
    pub fn GetSecurityDescriptorLength(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::DWORD; /* securitybaseapi.h:802:1 */
    pub fn GetSecurityDescriptorOwner(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, pOwner: *mut *mut ::libc::c_void, lpbOwnerDefaulted: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:810:1 */
    pub fn GetSecurityDescriptorRMControl(SecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, RMControl: ::minwindef::PUCHAR) -> ::minwindef::DWORD; /* securitybaseapi.h:820:1 */
    pub fn GetSecurityDescriptorSacl(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, lpbSaclPresent: ::minwindef::LPBOOL, pSacl: *mut *mut ::winnt::ACL, lpbSaclDefaulted: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:829:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetServiceDisplayNameW as GetServiceDisplayName; /* winsvc.h:1288:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetServiceDisplayNameA(hSCManager: ::winsvc::SC_HANDLE, lpServiceName: ::winnt::LPCSTR, lpDisplayName: ::winnt::LPSTR, lpcchBuffer: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1269:1 */
    pub fn GetServiceDisplayNameW(hSCManager: ::winsvc::SC_HANDLE, lpServiceName: ::winnt::LPCWSTR, lpDisplayName: ::winnt::LPWSTR, lpcchBuffer: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1280:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetServiceKeyNameW as GetServiceKeyName; /* winsvc.h:1260:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetServiceKeyNameA(hSCManager: ::winsvc::SC_HANDLE, lpDisplayName: ::winnt::LPCSTR, lpServiceName: ::winnt::LPSTR, lpcchBuffer: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1241:1 */
    pub fn GetServiceKeyNameW(hSCManager: ::winsvc::SC_HANDLE, lpDisplayName: ::winnt::LPCWSTR, lpServiceName: ::winnt::LPWSTR, lpcchBuffer: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1252:1 */
    pub fn GetSidIdentifierAuthority(pSid: ::winnt::PSID) -> ::winnt::PSID_IDENTIFIER_AUTHORITY; /* securitybaseapi.h:840:1 */
    pub fn GetSidLengthRequired(nSubAuthorityCount: ::minwindef::UCHAR) -> ::minwindef::DWORD; /* securitybaseapi.h:848:1 */
    pub fn GetSidSubAuthority(pSid: ::winnt::PSID, nSubAuthority: ::minwindef::DWORD) -> ::minwindef::PDWORD; /* securitybaseapi.h:856:1 */
    pub fn GetSidSubAuthorityCount(pSid: ::winnt::PSID) -> ::minwindef::PUCHAR; /* securitybaseapi.h:865:1 */
    pub fn GetTokenInformation(TokenHandle: ::winnt::HANDLE, TokenInformationClass: ::winnt::TOKEN_INFORMATION_CLASS, TokenInformation: ::minwindef::LPVOID, TokenInformationLength: ::minwindef::DWORD, ReturnLength: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:873:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetUserNameW as GetUserName; /* winbase.h:6881:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetUserNameA(lpBuffer: ::winnt::LPSTR, pcbBuffer: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6869:1 */
    pub fn GetUserNameW(lpBuffer: ::winnt::LPWSTR, pcbBuffer: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6876:1 */
    pub fn ImpersonateAnonymousToken(ThreadHandle: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:901:1 */
    pub fn ImpersonateLoggedOnUser(hToken: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:910:1 */
    pub fn ImpersonateNamedPipeClient(hNamedPipe: ::winnt::HANDLE) -> ::minwindef::BOOL; /* namedpipeapi.h:175:1 */
    pub fn ImpersonateSelf(ImpersonationLevel: ::winnt::SECURITY_IMPERSONATION_LEVEL) -> ::minwindef::BOOL; /* securitybaseapi.h:919:1 */
    pub fn InitializeAcl(pAcl: ::winnt::PACL, nAclLength: ::minwindef::DWORD, dwAclRevision: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:927:1 */
    pub fn InitializeSecurityDescriptor(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, dwRevision: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:937:1 */
    pub fn InitializeSid(Sid: ::winnt::PSID, pIdentifierAuthority: ::winnt::PSID_IDENTIFIER_AUTHORITY, nSubAuthorityCount: ::minwindef::BYTE) -> ::minwindef::BOOL; /* securitybaseapi.h:946:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::InitiateShutdownW as InitiateShutdown; /* winreg.h:1406:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn InitiateShutdownA(lpMachineName: ::winnt::LPSTR, lpMessage: ::winnt::LPSTR, dwGracePeriod: ::minwindef::DWORD, dwShutdownFlags: ::minwindef::DWORD, dwReason: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winreg.h:1388:1 */
    pub fn InitiateShutdownW(lpMachineName: ::winnt::LPWSTR, lpMessage: ::winnt::LPWSTR, dwGracePeriod: ::minwindef::DWORD, dwShutdownFlags: ::minwindef::DWORD, dwReason: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winreg.h:1398:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::InitiateSystemShutdownW as InitiateSystemShutdown; /* winreg.h:1288:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn InitiateSystemShutdownA(lpMachineName: ::winnt::LPSTR, lpMessage: ::winnt::LPSTR, dwTimeout: ::minwindef::DWORD, bForceAppsClosed: ::minwindef::BOOL, bRebootAfterShutdown: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winreg.h:1269:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::InitiateSystemShutdownExW as InitiateSystemShutdownEx; /* winreg.h:1364:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn InitiateSystemShutdownExA(lpMachineName: ::winnt::LPSTR, lpMessage: ::winnt::LPSTR, dwTimeout: ::minwindef::DWORD, bForceAppsClosed: ::minwindef::BOOL, bRebootAfterShutdown: ::minwindef::BOOL, dwReason: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winreg.h:1341:1 */
    pub fn InitiateSystemShutdownExW(lpMachineName: ::winnt::LPWSTR, lpMessage: ::winnt::LPWSTR, dwTimeout: ::minwindef::DWORD, bForceAppsClosed: ::minwindef::BOOL, bRebootAfterShutdown: ::minwindef::BOOL, dwReason: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winreg.h:1355:1 */
    pub fn InitiateSystemShutdownW(lpMachineName: ::winnt::LPWSTR, lpMessage: ::winnt::LPWSTR, dwTimeout: ::minwindef::DWORD, bForceAppsClosed: ::minwindef::BOOL, bRebootAfterShutdown: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winreg.h:1280:1 */
    pub fn IsTextUnicode(lpv: *const ::libc::c_void, iSize: ::libc::c_int, lpiResult: ::minwindef::LPINT) -> ::minwindef::BOOL; /* winbase.h:2841:1 */
    pub fn IsTokenRestricted(TokenHandle: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:956:1 */
    pub fn IsTokenUntrusted(TokenHandle: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:7051:1 */
    pub fn IsValidAcl(pAcl: ::winnt::PACL) -> ::minwindef::BOOL; /* securitybaseapi.h:964:1 */
    pub fn IsValidSecurityDescriptor(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* securitybaseapi.h:972:1 */
    pub fn IsValidSid(pSid: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:980:1 */
    pub fn LockServiceDatabase(hSCManager: ::winsvc::SC_HANDLE) -> ::winsvc::SC_LOCK; /* winsvc.h:1296:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LogonUserW as LogonUser; /* winbase.h:6937:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LogonUserA(lpszUsername: ::winnt::LPCSTR, lpszDomain: ::winnt::LPCSTR, lpszPassword: ::winnt::LPCSTR, dwLogonType: ::minwindef::DWORD, dwLogonProvider: ::minwindef::DWORD, phToken: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* winbase.h:6917:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LogonUserExW as LogonUserEx; /* winbase.h:6973:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LogonUserExA(lpszUsername: ::winnt::LPCSTR, lpszDomain: ::winnt::LPCSTR, lpszPassword: ::winnt::LPCSTR, dwLogonType: ::minwindef::DWORD, dwLogonProvider: ::minwindef::DWORD, phToken: ::winnt::PHANDLE, ppLogonSid: *mut *mut ::libc::c_void, ppProfileBuffer: *mut *mut ::libc::c_void, pdwProfileLength: ::minwindef::LPDWORD, pQuotaLimits: ::winnt::PQUOTA_LIMITS) -> ::minwindef::BOOL; /* winbase.h:6945:1 */
    pub fn LogonUserExW(lpszUsername: ::winnt::LPCWSTR, lpszDomain: ::winnt::LPCWSTR, lpszPassword: ::winnt::LPCWSTR, dwLogonType: ::minwindef::DWORD, dwLogonProvider: ::minwindef::DWORD, phToken: ::winnt::PHANDLE, ppLogonSid: *mut *mut ::libc::c_void, ppProfileBuffer: *mut *mut ::libc::c_void, pdwProfileLength: ::minwindef::LPDWORD, pQuotaLimits: ::winnt::PQUOTA_LIMITS) -> ::minwindef::BOOL; /* winbase.h:6960:1 */
    pub fn LogonUserW(lpszUsername: ::winnt::LPCWSTR, lpszDomain: ::winnt::LPCWSTR, lpszPassword: ::winnt::LPCWSTR, dwLogonType: ::minwindef::DWORD, dwLogonProvider: ::minwindef::DWORD, phToken: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* winbase.h:6928:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupAccountNameW as LookupAccountName; /* winbase.h:6512:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupAccountNameA(lpSystemName: ::winnt::LPCSTR, lpAccountName: ::winnt::LPCSTR, Sid: ::winnt::PSID, cbSid: ::minwindef::LPDWORD, ReferencedDomainName: ::winnt::LPSTR, cchReferencedDomainName: ::minwindef::LPDWORD, peUse: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6490:1 */
    pub fn LookupAccountNameW(lpSystemName: ::winnt::LPCWSTR, lpAccountName: ::winnt::LPCWSTR, Sid: ::winnt::PSID, cbSid: ::minwindef::LPDWORD, ReferencedDomainName: ::winnt::LPWSTR, cchReferencedDomainName: ::minwindef::LPDWORD, peUse: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6502:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupAccountSidW as LookupAccountSid; /* winbase.h:6482:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupAccountSidA(lpSystemName: ::winnt::LPCSTR, Sid: ::winnt::PSID, Name: ::winnt::LPSTR, cchName: ::minwindef::LPDWORD, ReferencedDomainName: ::winnt::LPSTR, cchReferencedDomainName: ::minwindef::LPDWORD, peUse: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6460:1 */
    pub fn LookupAccountSidW(lpSystemName: ::winnt::LPCWSTR, Sid: ::winnt::PSID, Name: ::winnt::LPWSTR, cchName: ::minwindef::LPDWORD, ReferencedDomainName: ::winnt::LPWSTR, cchReferencedDomainName: ::minwindef::LPDWORD, peUse: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6472:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupPrivilegeDisplayNameW as LookupPrivilegeDisplayName; /* winbase.h:6666:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupPrivilegeDisplayNameA(lpSystemName: ::winnt::LPCSTR, lpName: ::winnt::LPCSTR, lpDisplayName: ::winnt::LPSTR, cchDisplayName: ::minwindef::LPDWORD, lpLanguageId: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6648:1 */
    pub fn LookupPrivilegeDisplayNameW(lpSystemName: ::winnt::LPCWSTR, lpName: ::winnt::LPCWSTR, lpDisplayName: ::winnt::LPWSTR, cchDisplayName: ::minwindef::LPDWORD, lpLanguageId: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6658:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupPrivilegeNameW as LookupPrivilegeName; /* winbase.h:6640:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupPrivilegeNameA(lpSystemName: ::winnt::LPCSTR, lpLuid: ::winnt::PLUID, lpName: ::winnt::LPSTR, cchName: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6624:1 */
    pub fn LookupPrivilegeNameW(lpSystemName: ::winnt::LPCWSTR, lpLuid: ::winnt::PLUID, lpName: ::winnt::LPWSTR, cchName: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6633:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupPrivilegeValueW as LookupPrivilegeValue; /* winbase.h:6616:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupPrivilegeValueA(lpSystemName: ::winnt::LPCSTR, lpName: ::winnt::LPCSTR, lpLuid: ::winnt::PLUID) -> ::minwindef::BOOL; /* winbase.h:6602:1 */
    pub fn LookupPrivilegeValueW(lpSystemName: ::winnt::LPCWSTR, lpName: ::winnt::LPCWSTR, lpLuid: ::winnt::PLUID) -> ::minwindef::BOOL; /* winbase.h:6610:1 */
    pub fn MakeAbsoluteSD(pSelfRelativeSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, pAbsoluteSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, lpdwAbsoluteSecurityDescriptorSize: ::minwindef::LPDWORD, pDacl: ::winnt::PACL, lpdwDaclSize: ::minwindef::LPDWORD, pSacl: ::winnt::PACL, lpdwSaclSize: ::minwindef::LPDWORD, pOwner: ::winnt::PSID, lpdwOwnerSize: ::minwindef::LPDWORD, pPrimaryGroup: ::winnt::PSID, lpdwPrimaryGroupSize: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:1003:1 */
    pub fn MakeSelfRelativeSD(pAbsoluteSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, pSelfRelativeSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, lpdwBufferLength: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:1022:1 */
    pub fn MapGenericMask(AccessMask: ::minwindef::PDWORD, GenericMapping: ::winnt::PGENERIC_MAPPING); /* securitybaseapi.h:1032:1 */
    pub fn NotifyBootConfigStatus(BootAcceptable: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winsvc.h:1303:1 */
    pub fn NotifyChangeEventLog(hEventLog: ::winnt::HANDLE, hEvent: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5929:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectCloseAuditAlarmW as ObjectCloseAuditAlarm; /* securitybaseapi.h:1049:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectCloseAuditAlarmA(SubsystemName: ::winnt::LPCSTR, HandleId: ::minwindef::LPVOID, GenerateOnClose: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:6287:1 */
    pub fn ObjectCloseAuditAlarmW(SubsystemName: ::winnt::LPCWSTR, HandleId: ::minwindef::LPVOID, GenerateOnClose: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1041:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectDeleteAuditAlarmW as ObjectDeleteAuditAlarm; /* securitybaseapi.h:1063:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectDeleteAuditAlarmA(SubsystemName: ::winnt::LPCSTR, HandleId: ::minwindef::LPVOID, GenerateOnClose: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:6299:1 */
    pub fn ObjectDeleteAuditAlarmW(SubsystemName: ::winnt::LPCWSTR, HandleId: ::minwindef::LPVOID, GenerateOnClose: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1055:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectOpenAuditAlarmW as ObjectOpenAuditAlarm; /* securitybaseapi.h:1086:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectOpenAuditAlarmA(SubsystemName: ::winnt::LPCSTR, HandleId: ::minwindef::LPVOID, ObjectTypeName: ::winnt::LPSTR, ObjectName: ::winnt::LPSTR, pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, ClientToken: ::winnt::HANDLE, DesiredAccess: ::minwindef::DWORD, GrantedAccess: ::minwindef::DWORD, Privileges: ::winnt::PPRIVILEGE_SET, ObjectCreation: ::minwindef::BOOL, AccessGranted: ::minwindef::BOOL, GenerateOnClose: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* winbase.h:6251:1 */
    pub fn ObjectOpenAuditAlarmW(SubsystemName: ::winnt::LPCWSTR, HandleId: ::minwindef::LPVOID, ObjectTypeName: ::winnt::LPWSTR, ObjectName: ::winnt::LPWSTR, pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, ClientToken: ::winnt::HANDLE, DesiredAccess: ::minwindef::DWORD, GrantedAccess: ::minwindef::DWORD, Privileges: ::winnt::PPRIVILEGE_SET, ObjectCreation: ::minwindef::BOOL, AccessGranted: ::minwindef::BOOL, GenerateOnClose: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1069:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectPrivilegeAuditAlarmW as ObjectPrivilegeAuditAlarm; /* securitybaseapi.h:1103:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectPrivilegeAuditAlarmA(SubsystemName: ::winnt::LPCSTR, HandleId: ::minwindef::LPVOID, ClientToken: ::winnt::HANDLE, DesiredAccess: ::minwindef::DWORD, Privileges: ::winnt::PPRIVILEGE_SET, AccessGranted: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:6272:1 */
    pub fn ObjectPrivilegeAuditAlarmW(SubsystemName: ::winnt::LPCWSTR, HandleId: ::minwindef::LPVOID, ClientToken: ::winnt::HANDLE, DesiredAccess: ::minwindef::DWORD, Privileges: ::winnt::PPRIVILEGE_SET, AccessGranted: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1092:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenBackupEventLogW as OpenBackupEventLog; /* winbase.h:6005:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenBackupEventLogA(lpUNCServerName: ::winnt::LPCSTR, lpFileName: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:5993:1 */
    pub fn OpenBackupEventLogW(lpUNCServerName: ::winnt::LPCWSTR, lpFileName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winbase.h:6000:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenEncryptedFileRawW as OpenEncryptedFileRaw; /* winbase.h:2590:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenEncryptedFileRawA(lpFileName: ::winnt::LPCSTR, ulFlags: ::minwindef::ULONG, pvContext: *mut *mut ::libc::c_void) -> ::minwindef::DWORD; /* winbase.h:2576:1 */
    pub fn OpenEncryptedFileRawW(lpFileName: ::winnt::LPCWSTR, ulFlags: ::minwindef::ULONG, pvContext: *mut *mut ::libc::c_void) -> ::minwindef::DWORD; /* winbase.h:2584:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenEventLogW as OpenEventLog; /* winbase.h:5965:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenEventLogA(lpUNCServerName: ::winnt::LPCSTR, lpSourceName: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:5953:1 */
    pub fn OpenEventLogW(lpUNCServerName: ::winnt::LPCWSTR, lpSourceName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winbase.h:5960:1 */
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
    pub fn OpenThreadToken(ThreadHandle: ::winnt::HANDLE, DesiredAccess: ::minwindef::DWORD, OpenAsSelf: ::minwindef::BOOL, TokenHandle: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* processthreadsapi.h:632:1 */
    pub fn PrivilegeCheck(ClientToken: ::winnt::HANDLE, RequiredPrivileges: ::winnt::PPRIVILEGE_SET, pfResult: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1109:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PrivilegedServiceAuditAlarmW as PrivilegedServiceAuditAlarm; /* securitybaseapi.h:1129:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PrivilegedServiceAuditAlarmA(SubsystemName: ::winnt::LPCSTR, ServiceName: ::winnt::LPCSTR, ClientToken: ::winnt::HANDLE, Privileges: ::winnt::PPRIVILEGE_SET, AccessGranted: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:6311:1 */
    pub fn PrivilegedServiceAuditAlarmW(SubsystemName: ::winnt::LPCWSTR, ServiceName: ::winnt::LPCWSTR, ClientToken: ::winnt::HANDLE, Privileges: ::winnt::PPRIVILEGE_SET, AccessGranted: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1119:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::QueryServiceConfigW as QueryServiceConfig; /* winsvc.h:1378:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::QueryServiceConfig2W as QueryServiceConfig2; /* winsvc.h:1422:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn QueryServiceConfig2A(hService: ::winsvc::SC_HANDLE, dwInfoLevel: ::minwindef::DWORD, lpBuffer: ::minwindef::LPBYTE, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1394:1 */
    pub fn QueryServiceConfig2W(hService: ::winsvc::SC_HANDLE, dwInfoLevel: ::minwindef::DWORD, lpBuffer: ::minwindef::LPBYTE, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1413:1 */
    pub fn QueryServiceConfigA(hService: ::winsvc::SC_HANDLE, lpServiceConfig: ::winsvc::LPQUERY_SERVICE_CONFIGA, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1359:1 */
    pub fn QueryServiceConfigW(hService: ::winsvc::SC_HANDLE, lpServiceConfig: ::winsvc::LPQUERY_SERVICE_CONFIGW, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1370:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::QueryServiceLockStatusW as QueryServiceLockStatus; /* winsvc.h:1450:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn QueryServiceLockStatusA(hSCManager: ::winsvc::SC_HANDLE, lpLockStatus: ::winsvc::LPQUERY_SERVICE_LOCK_STATUSA, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1431:1 */
    pub fn QueryServiceLockStatusW(hSCManager: ::winsvc::SC_HANDLE, lpLockStatus: ::winsvc::LPQUERY_SERVICE_LOCK_STATUSW, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1442:1 */
    pub fn QueryServiceObjectSecurity(hService: ::winsvc::SC_HANDLE, dwSecurityInformation: ::winnt::SECURITY_INFORMATION, lpSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1459:1 */
    pub fn QueryServiceStatus(hService: ::winsvc::SC_HANDLE, lpServiceStatus: ::winsvc::LPSERVICE_STATUS) -> ::minwindef::BOOL; /* winsvc.h:1472:1 */
    pub fn QueryServiceStatusEx(hService: ::winsvc::SC_HANDLE, InfoLevel: ::winsvc::SC_STATUS_TYPE, lpBuffer: ::minwindef::LPBYTE, cbBufSize: ::minwindef::DWORD, pcbBytesNeeded: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1481:1 */
    pub fn ReadEncryptedFileRaw(pfExportCallback: ::winbase::PFE_EXPORT_FUNC, pvCallbackContext: ::winnt::PVOID, pvContext: ::winnt::PVOID) -> ::minwindef::DWORD; /* winbase.h:2598:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ReadEventLogW as ReadEventLog; /* winbase.h:6035:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ReadEventLogA(hEventLog: ::winnt::HANDLE, dwReadFlags: ::minwindef::DWORD, dwRecordOffset: ::minwindef::DWORD, lpBuffer: ::minwindef::LPVOID, nNumberOfBytesToRead: ::minwindef::DWORD, pnBytesRead: *mut ::libc::c_ulong, pnMinNumberOfBytesNeeded: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winbase.h:6013:1 */
    pub fn ReadEventLogW(hEventLog: ::winnt::HANDLE, dwReadFlags: ::minwindef::DWORD, dwRecordOffset: ::minwindef::DWORD, lpBuffer: ::minwindef::LPVOID, nNumberOfBytesToRead: ::minwindef::DWORD, pnBytesRead: *mut ::libc::c_ulong, pnMinNumberOfBytesNeeded: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winbase.h:6025:1 */
    pub fn RegCloseKey(hKey: ::minwindef::HKEY) -> ::winreg::LSTATUS; /* winreg.h:206:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegConnectRegistryW as RegConnectRegistry; /* winreg.h:271:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegConnectRegistryA(lpMachineName: ::winnt::LPCSTR, hKey: ::minwindef::HKEY, phkResult: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:257:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegConnectRegistryExW as RegConnectRegistryEx; /* winreg.h:295:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegConnectRegistryExA(lpMachineName: ::winnt::LPCSTR, hKey: ::minwindef::HKEY, Flags: ::minwindef::ULONG, phkResult: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:279:1 */
    pub fn RegConnectRegistryExW(lpMachineName: ::winnt::LPCWSTR, hKey: ::minwindef::HKEY, Flags: ::minwindef::ULONG, phkResult: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:288:1 */
    pub fn RegConnectRegistryW(lpMachineName: ::winnt::LPCWSTR, hKey: ::minwindef::HKEY, phkResult: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:265:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegCreateKeyW as RegCreateKey; /* winreg.h:317:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegCreateKeyA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, phkResult: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:303:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegCreateKeyExW as RegCreateKeyEx; /* winreg.h:353:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegCreateKeyExA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, Reserved: ::minwindef::DWORD, lpClass: ::winnt::LPSTR, dwOptions: ::minwindef::DWORD, samDesired: ::winreg::REGSAM, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, phkResult: ::minwindef::PHKEY, lpdwDisposition: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:325:1 */
    pub fn RegCreateKeyExW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, Reserved: ::minwindef::DWORD, lpClass: ::winnt::LPWSTR, dwOptions: ::minwindef::DWORD, samDesired: ::winreg::REGSAM, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, phkResult: ::minwindef::PHKEY, lpdwDisposition: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:340:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegCreateKeyTransactedW as RegCreateKeyTransacted; /* winreg.h:391:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegCreateKeyTransactedA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, Reserved: ::minwindef::DWORD, lpClass: ::winnt::LPSTR, dwOptions: ::minwindef::DWORD, samDesired: ::winreg::REGSAM, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, phkResult: ::minwindef::PHKEY, lpdwDisposition: ::minwindef::LPDWORD, hTransaction: ::winnt::HANDLE, pExtendedParemeter: ::winnt::PVOID) -> ::winreg::LSTATUS; /* winreg.h:361:1 */
    pub fn RegCreateKeyTransactedW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, Reserved: ::minwindef::DWORD, lpClass: ::winnt::LPWSTR, dwOptions: ::minwindef::DWORD, samDesired: ::winreg::REGSAM, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, phkResult: ::minwindef::PHKEY, lpdwDisposition: ::minwindef::LPDWORD, hTransaction: ::winnt::HANDLE, pExtendedParemeter: ::winnt::PVOID) -> ::winreg::LSTATUS; /* winreg.h:377:1 */
    pub fn RegCreateKeyW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, phkResult: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:311:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegDeleteKeyW as RegDeleteKey; /* winreg.h:411:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegDeleteKeyA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:399:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegDeleteKeyExW as RegDeleteKeyEx; /* winreg.h:437:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegDeleteKeyExA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, samDesired: ::winreg::REGSAM, Reserved: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:419:1 */
    pub fn RegDeleteKeyExW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, samDesired: ::winreg::REGSAM, Reserved: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:429:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegDeleteKeyTransactedW as RegDeleteKeyTransacted; /* winreg.h:465:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegDeleteKeyTransactedA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, samDesired: ::winreg::REGSAM, Reserved: ::minwindef::DWORD, hTransaction: ::winnt::HANDLE, pExtendedParameter: ::winnt::PVOID) -> ::winreg::LSTATUS; /* winreg.h:445:1 */
    pub fn RegDeleteKeyTransactedW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, samDesired: ::winreg::REGSAM, Reserved: ::minwindef::DWORD, hTransaction: ::winnt::HANDLE, pExtendedParameter: ::winnt::PVOID) -> ::winreg::LSTATUS; /* winreg.h:456:1 */
    pub fn RegDeleteKeyW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:406:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegDeleteValueW as RegDeleteValue; /* winreg.h:509:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegDeleteValueA(hKey: ::minwindef::HKEY, lpValueName: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:495:1 */
    pub fn RegDeleteValueW(hKey: ::minwindef::HKEY, lpValueName: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:503:1 */
    pub fn RegDisablePredefinedCache() -> ::winreg::LSTATUS; /* winreg.h:242:1 */
    pub fn RegDisablePredefinedCacheEx() -> ::winreg::LSTATUS; /* winreg.h:249:1 */
    pub fn RegDisableReflectionKey(hBase: ::minwindef::HKEY) -> ::winnt::LONG; /* winreg.h:473:1 */
    pub fn RegEnableReflectionKey(hBase: ::minwindef::HKEY) -> ::winnt::LONG; /* winreg.h:480:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegEnumKeyW as RegEnumKey; /* winreg.h:533:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegEnumKeyA(hKey: ::minwindef::HKEY, dwIndex: ::minwindef::DWORD, lpName: ::winnt::LPSTR, cchName: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:517:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegEnumKeyExW as RegEnumKeyEx; /* winreg.h:567:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegEnumKeyExA(hKey: ::minwindef::HKEY, dwIndex: ::minwindef::DWORD, lpName: ::winnt::LPSTR, lpcchName: ::minwindef::LPDWORD, lpReserved: ::minwindef::LPDWORD, lpClass: ::winnt::LPSTR, lpcchClass: ::minwindef::LPDWORD, lpftLastWriteTime: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:541:1 */
    pub fn RegEnumKeyExW(hKey: ::minwindef::HKEY, dwIndex: ::minwindef::DWORD, lpName: ::winnt::LPWSTR, lpcchName: ::minwindef::LPDWORD, lpReserved: ::minwindef::LPDWORD, lpClass: ::winnt::LPWSTR, lpcchClass: ::minwindef::LPDWORD, lpftLastWriteTime: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:555:1 */
    pub fn RegEnumKeyW(hKey: ::minwindef::HKEY, dwIndex: ::minwindef::DWORD, lpName: ::winnt::LPWSTR, cchName: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:526:1 */
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
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegOpenKeyW as RegOpenKey; /* winreg.h:678:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegOpenKeyA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, phkResult: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:664:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegOpenKeyExW as RegOpenKeyEx; /* winreg.h:706:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegOpenKeyExA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, ulOptions: ::minwindef::DWORD, samDesired: ::winreg::REGSAM, phkResult: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:686:1 */
    pub fn RegOpenKeyExW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, ulOptions: ::minwindef::DWORD, samDesired: ::winreg::REGSAM, phkResult: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:697:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegOpenKeyTransactedW as RegOpenKeyTransacted; /* winreg.h:736:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegOpenKeyTransactedA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, ulOptions: ::minwindef::DWORD, samDesired: ::winreg::REGSAM, phkResult: ::minwindef::PHKEY, hTransaction: ::winnt::HANDLE, pExtendedParemeter: ::winnt::PVOID) -> ::winreg::LSTATUS; /* winreg.h:714:1 */
    pub fn RegOpenKeyTransactedW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, ulOptions: ::minwindef::DWORD, samDesired: ::winreg::REGSAM, phkResult: ::minwindef::PHKEY, hTransaction: ::winnt::HANDLE, pExtendedParemeter: ::winnt::PVOID) -> ::winreg::LSTATUS; /* winreg.h:726:1 */
    pub fn RegOpenKeyW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, phkResult: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:672:1 */
    pub fn RegOpenUserClassesRoot(hToken: ::winnt::HANDLE, dwOptions: ::minwindef::DWORD, samDesired: ::winreg::REGSAM, phkResult: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:222:1 */
    pub fn RegOverridePredefKey(hKey: ::minwindef::HKEY, hNewHKey: ::minwindef::HKEY) -> ::winreg::LSTATUS; /* winreg.h:214:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegQueryInfoKeyW as RegQueryInfoKey; /* winreg.h:778:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegQueryInfoKeyA(hKey: ::minwindef::HKEY, lpClass: ::winnt::LPSTR, lpcchClass: ::minwindef::LPDWORD, lpReserved: ::minwindef::LPDWORD, lpcSubKeys: ::minwindef::LPDWORD, lpcbMaxSubKeyLen: ::minwindef::LPDWORD, lpcbMaxClassLen: ::minwindef::LPDWORD, lpcValues: ::minwindef::LPDWORD, lpcbMaxValueNameLen: ::minwindef::LPDWORD, lpcbMaxValueLen: ::minwindef::LPDWORD, lpcbSecurityDescriptor: ::minwindef::LPDWORD, lpftLastWriteTime: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:744:1 */
    pub fn RegQueryInfoKeyW(hKey: ::minwindef::HKEY, lpClass: ::winnt::LPWSTR, lpcchClass: ::minwindef::LPDWORD, lpReserved: ::minwindef::LPDWORD, lpcSubKeys: ::minwindef::LPDWORD, lpcbMaxSubKeyLen: ::minwindef::LPDWORD, lpcbMaxClassLen: ::minwindef::LPDWORD, lpcValues: ::minwindef::LPDWORD, lpcbMaxValueNameLen: ::minwindef::LPDWORD, lpcbMaxValueLen: ::minwindef::LPDWORD, lpcbSecurityDescriptor: ::minwindef::LPDWORD, lpftLastWriteTime: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:762:1 */
    pub fn RegQueryReflectionKey(hBase: ::minwindef::HKEY, bIsReflectionDisabled: *mut ::libc::c_int) -> ::winnt::LONG; /* winreg.h:487:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegQueryValueW as RegQueryValue; /* winreg.h:802:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegQueryValueA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, lpData: ::winnt::LPSTR, lpcbData: ::winnt::PLONG) -> ::winreg::LSTATUS; /* winreg.h:786:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegQueryValueExW as RegQueryValueEx; /* winreg.h:864:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegQueryValueExA(hKey: ::minwindef::HKEY, lpValueName: ::winnt::LPCSTR, lpReserved: ::minwindef::LPDWORD, lpType: ::minwindef::LPDWORD, lpData: ::minwindef::LPBYTE, lpcbData: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:840:1 */
    pub fn RegQueryValueExW(hKey: ::minwindef::HKEY, lpValueName: ::winnt::LPCWSTR, lpReserved: ::minwindef::LPDWORD, lpType: ::minwindef::LPDWORD, lpData: ::minwindef::LPBYTE, lpcbData: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:853:1 */
    pub fn RegQueryValueW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, lpData: ::winnt::LPWSTR, lpcbData: ::winnt::PLONG) -> ::winreg::LSTATUS; /* winreg.h:795:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegReplaceKeyW as RegReplaceKey; /* winreg.h:888:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegReplaceKeyA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, lpNewFile: ::winnt::LPCSTR, lpOldFile: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:872:1 */
    pub fn RegReplaceKeyW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, lpNewFile: ::winnt::LPCWSTR, lpOldFile: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:881:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegRestoreKeyW as RegRestoreKey; /* winreg.h:912:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegRestoreKeyA(hKey: ::minwindef::HKEY, lpFile: ::winnt::LPCSTR, dwFlags: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:896:1 */
    pub fn RegRestoreKeyW(hKey: ::minwindef::HKEY, lpFile: ::winnt::LPCWSTR, dwFlags: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:905:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegSaveKeyW as RegSaveKey; /* winreg.h:948:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegSaveKeyA(hKey: ::minwindef::HKEY, lpFile: ::winnt::LPCSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::winreg::LSTATUS; /* winreg.h:934:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegSaveKeyExW as RegSaveKeyEx; /* winreg.h:1440:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegSaveKeyExA(hKey: ::minwindef::HKEY, lpFile: ::winnt::LPCSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, Flags: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1422:1 */
    pub fn RegSaveKeyExW(hKey: ::minwindef::HKEY, lpFile: ::winnt::LPCWSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES, Flags: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1432:1 */
    pub fn RegSaveKeyW(hKey: ::minwindef::HKEY, lpFile: ::winnt::LPCWSTR, lpSecurityAttributes: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::winreg::LSTATUS; /* winreg.h:942:1 */
    pub fn RegSetKeySecurity(hKey: ::minwindef::HKEY, SecurityInformation: ::winnt::SECURITY_INFORMATION, pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR) -> ::winreg::LSTATUS; /* winreg.h:956:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegSetValueW as RegSetValue; /* winreg.h:984:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegSetValueA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, dwType: ::minwindef::DWORD, lpData: ::winnt::LPCSTR, cbData: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:966:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegSetValueExW as RegSetValueEx; /* winreg.h:1014:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegSetValueExA(hKey: ::minwindef::HKEY, lpValueName: ::winnt::LPCSTR, Reserved: ::minwindef::DWORD, dwType: ::minwindef::DWORD, lpData: *const ::libc::c_uchar, cbData: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:992:1 */
    pub fn RegSetValueExW(hKey: ::minwindef::HKEY, lpValueName: ::winnt::LPCWSTR, Reserved: ::minwindef::DWORD, dwType: ::minwindef::DWORD, lpData: *const ::libc::c_uchar, cbData: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1004:1 */
    pub fn RegSetValueW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, dwType: ::minwindef::DWORD, lpData: ::winnt::LPCWSTR, cbData: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:976:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegUnLoadKeyW as RegUnLoadKey; /* winreg.h:1036:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegUnLoadKeyA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:1022:1 */
    pub fn RegUnLoadKeyW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:1030:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegisterEventSourceW as RegisterEventSource; /* winbase.h:5985:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegisterEventSourceA(lpUNCServerName: ::winnt::LPCSTR, lpSourceName: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:5973:1 */
    pub fn RegisterEventSourceW(lpUNCServerName: ::winnt::LPCWSTR, lpSourceName: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winbase.h:5980:1 */
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
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ReportEventW as ReportEvent; /* winbase.h:6069:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ReportEventA(hEventLog: ::winnt::HANDLE, wType: ::minwindef::WORD, wCategory: ::minwindef::WORD, dwEventID: ::minwindef::DWORD, lpUserSid: ::winnt::PSID, wNumStrings: ::minwindef::WORD, dwDataSize: ::minwindef::DWORD, lpStrings: *mut *const ::libc::c_schar, lpRawData: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winbase.h:6043:1 */
    pub fn ReportEventW(hEventLog: ::winnt::HANDLE, wType: ::minwindef::WORD, wCategory: ::minwindef::WORD, dwEventID: ::minwindef::DWORD, lpUserSid: ::winnt::PSID, wNumStrings: ::minwindef::WORD, dwDataSize: ::minwindef::DWORD, lpStrings: *mut *const ::libc::c_ushort, lpRawData: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winbase.h:6057:1 */
    pub fn RevertToSelf() -> ::minwindef::BOOL; /* securitybaseapi.h:1149:1 */
    pub fn SetAclInformation(pAcl: ::winnt::PACL, pAclInformation: ::minwindef::LPVOID, nAclInformationLength: ::minwindef::DWORD, dwAclInformationClass: ::winnt::ACL_INFORMATION_CLASS) -> ::minwindef::BOOL; /* securitybaseapi.h:1157:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetFileSecurityW as SetFileSecurity; /* securitybaseapi.h:1175:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetFileSecurityA(lpFileName: ::winnt::LPCSTR, SecurityInformation: ::winnt::SECURITY_INFORMATION, pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* winbase.h:6341:1 */
    pub fn SetFileSecurityW(lpFileName: ::winnt::LPCWSTR, SecurityInformation: ::winnt::SECURITY_INFORMATION, pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* securitybaseapi.h:1168:1 */
    pub fn SetKernelObjectSecurity(Handle: ::winnt::HANDLE, SecurityInformation: ::winnt::SECURITY_INFORMATION, SecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* securitybaseapi.h:1181:1 */
    pub fn SetPrivateObjectSecurity(SecurityInformation: ::winnt::SECURITY_INFORMATION, ModificationDescriptor: ::winnt::PSECURITY_DESCRIPTOR, ObjectsSecurityDescriptor: *mut *mut ::libc::c_void, GenericMapping: ::winnt::PGENERIC_MAPPING, Token: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:1191:1 */
    pub fn SetPrivateObjectSecurityEx(SecurityInformation: ::winnt::SECURITY_INFORMATION, ModificationDescriptor: ::winnt::PSECURITY_DESCRIPTOR, ObjectsSecurityDescriptor: *mut *mut ::libc::c_void, AutoInheritFlags: ::minwindef::ULONG, GenericMapping: ::winnt::PGENERIC_MAPPING, Token: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:1203:1 */
    pub fn SetSecurityDescriptorControl(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, ControlBitsOfInterest: ::winnt::SECURITY_DESCRIPTOR_CONTROL, ControlBitsToSet: ::winnt::SECURITY_DESCRIPTOR_CONTROL) -> ::minwindef::BOOL; /* securitybaseapi.h:1230:1 */
    pub fn SetSecurityDescriptorDacl(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, bDaclPresent: ::minwindef::BOOL, pDacl: ::winnt::PACL, bDaclDefaulted: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1240:1 */
    pub fn SetSecurityDescriptorGroup(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, pGroup: ::winnt::PSID, bGroupDefaulted: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1251:1 */
    pub fn SetSecurityDescriptorOwner(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, pOwner: ::winnt::PSID, bOwnerDefaulted: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1261:1 */
    pub fn SetSecurityDescriptorRMControl(SecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, RMControl: ::minwindef::PUCHAR) -> ::minwindef::DWORD; /* securitybaseapi.h:1271:1 */
    pub fn SetSecurityDescriptorSacl(pSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, bSaclPresent: ::minwindef::BOOL, pSacl: ::winnt::PACL, bSaclDefaulted: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1280:1 */
    pub fn SetServiceObjectSecurity(hService: ::winsvc::SC_HANDLE, dwSecurityInformation: ::winnt::SECURITY_INFORMATION, lpSecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* winsvc.h:1543:1 */
    pub fn SetServiceStatus(hServiceStatus: ::winsvc::SERVICE_STATUS_HANDLE, lpServiceStatus: ::winsvc::LPSERVICE_STATUS) -> ::minwindef::BOOL; /* winsvc.h:1552:1 */
    pub fn SetThreadToken(Thread: ::winnt::PHANDLE, Token: ::winnt::HANDLE) -> ::minwindef::BOOL; /* processthreadsapi.h:613:1 */
    pub fn SetTokenInformation(TokenHandle: ::winnt::HANDLE, TokenInformationClass: ::winnt::TOKEN_INFORMATION_CLASS, TokenInformation: ::minwindef::LPVOID, TokenInformationLength: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:1291:1 */
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
    pub fn UnlockServiceDatabase(ScLock: ::winsvc::SC_LOCK) -> ::minwindef::BOOL; /* winsvc.h:1603:1 */
    pub fn WriteEncryptedFileRaw(pfImportCallback: ::winbase::PFE_IMPORT_FUNC, pvCallbackContext: ::winnt::PVOID, pvContext: ::winnt::PVOID) -> ::minwindef::DWORD; /* winbase.h:2607:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::GetCurrentHwProfileW as GetCurrentHwProfile; /* winbase.h:7322:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn GetCurrentHwProfileA(lpHwProfileInfo: ::winbase::LPHW_PROFILE_INFOA) -> ::minwindef::BOOL; /* winbase.h:7312:1 */
    pub fn GetCurrentHwProfileW(lpHwProfileInfo: ::winbase::LPHW_PROFILE_INFOW) -> ::minwindef::BOOL; /* winbase.h:7318:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::RegQueryMultipleValuesW as RegQueryMultipleValues; /* winreg.h:831:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn RegQueryMultipleValuesA(hKey: ::minwindef::HKEY, val_list: ::winreg::PVALENTA, num_vals: ::minwindef::DWORD, lpValueBuf: ::winnt::LPSTR, ldwTotsize: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:813:1 */
    pub fn RegQueryMultipleValuesW(hKey: ::minwindef::HKEY, val_list: ::winreg::PVALENTW, num_vals: ::minwindef::DWORD, lpValueBuf: ::winnt::LPWSTR, ldwTotsize: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:823:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn AccessCheckByTypeAndAuditAlarmA(SubsystemName: ::winnt::LPCSTR, HandleId: ::minwindef::LPVOID, ObjectTypeName: ::winnt::LPCSTR, ObjectName: ::winnt::LPCSTR, SecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, PrincipalSelfSid: ::winnt::PSID, DesiredAccess: ::minwindef::DWORD, AuditType: ::winnt::AUDIT_EVENT_TYPE, Flags: ::minwindef::DWORD, ObjectTypeList: ::winnt::POBJECT_TYPE_LIST, ObjectTypeListLength: ::minwindef::DWORD, GenericMapping: ::winnt::PGENERIC_MAPPING, ObjectCreation: ::minwindef::BOOL, GrantedAccess: ::minwindef::LPDWORD, AccessStatus: ::minwindef::LPBOOL, pfGenerateOnClose: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* winbase.h:6174:1 */
    pub fn AccessCheckByTypeResultListAndAuditAlarmA(SubsystemName: ::winnt::LPCSTR, HandleId: ::minwindef::LPVOID, ObjectTypeName: ::winnt::LPCSTR, ObjectName: ::winnt::LPCSTR, SecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, PrincipalSelfSid: ::winnt::PSID, DesiredAccess: ::minwindef::DWORD, AuditType: ::winnt::AUDIT_EVENT_TYPE, Flags: ::minwindef::DWORD, ObjectTypeList: ::winnt::POBJECT_TYPE_LIST, ObjectTypeListLength: ::minwindef::DWORD, GenericMapping: ::winnt::PGENERIC_MAPPING, ObjectCreation: ::minwindef::BOOL, GrantedAccess: ::minwindef::LPDWORD, AccessStatusList: ::minwindef::LPDWORD, pfGenerateOnClose: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* winbase.h:6199:1 */
    pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleA(SubsystemName: ::winnt::LPCSTR, HandleId: ::minwindef::LPVOID, ClientToken: ::winnt::HANDLE, ObjectTypeName: ::winnt::LPCSTR, ObjectName: ::winnt::LPCSTR, SecurityDescriptor: ::winnt::PSECURITY_DESCRIPTOR, PrincipalSelfSid: ::winnt::PSID, DesiredAccess: ::minwindef::DWORD, AuditType: ::winnt::AUDIT_EVENT_TYPE, Flags: ::minwindef::DWORD, ObjectTypeList: ::winnt::POBJECT_TYPE_LIST, ObjectTypeListLength: ::minwindef::DWORD, GenericMapping: ::winnt::PGENERIC_MAPPING, ObjectCreation: ::minwindef::BOOL, GrantedAccess: ::minwindef::LPDWORD, AccessStatusList: ::minwindef::LPDWORD, pfGenerateOnClose: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* winbase.h:6224:1 */
    pub fn CreateProcessWithLogonW(lpUsername: ::winnt::LPCWSTR, lpDomain: ::winnt::LPCWSTR, lpPassword: ::winnt::LPCWSTR, dwLogonFlags: ::minwindef::DWORD, lpApplicationName: ::winnt::LPCWSTR, lpCommandLine: ::winnt::LPWSTR, dwCreationFlags: ::minwindef::DWORD, lpEnvironment: ::minwindef::LPVOID, lpCurrentDirectory: ::winnt::LPCWSTR, lpStartupInfo: ::processthreadsapi::LPSTARTUPINFOW, lpProcessInformation: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* winbase.h:7017:1 */
    pub fn CreateProcessWithTokenW(hToken: ::winnt::HANDLE, dwLogonFlags: ::minwindef::DWORD, lpApplicationName: ::winnt::LPCWSTR, lpCommandLine: ::winnt::LPWSTR, dwCreationFlags: ::minwindef::DWORD, lpEnvironment: ::minwindef::LPVOID, lpCurrentDirectory: ::winnt::LPCWSTR, lpStartupInfo: ::processthreadsapi::LPSTARTUPINFOW, lpProcessInformation: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* winbase.h:7034:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn CreateWellKnownSid(WellKnownSidType: ::winnt::WELL_KNOWN_SID_TYPE, DomainSid: ::winnt::PSID, pSid: ::winnt::PSID, cbSid: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* securitybaseapi.h:600:1 */
    pub fn EqualDomainSid(pSid1: ::winnt::PSID, pSid2: ::winnt::PSID, pfEqual: *mut ::libc::c_int) -> ::minwindef::BOOL; /* securitybaseapi.h:612:1 */
    pub fn GetWindowsAccountDomainSid(pSid: ::winnt::PSID, pDomainSid: ::winnt::PSID, cbDomainSid: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* securitybaseapi.h:889:1 */
    pub fn IsWellKnownSid(pSid: ::winnt::PSID, WellKnownSidType: ::winnt::WELL_KNOWN_SID_TYPE) -> ::minwindef::BOOL; /* securitybaseapi.h:991:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn AddMandatoryAce(pAcl: ::winnt::PACL, dwAceRevision: ::minwindef::DWORD, AceFlags: ::minwindef::DWORD, MandatoryPolicy: ::minwindef::DWORD, pLabelSid: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:343:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::ControlServiceExW as ControlServiceEx; /* winsvc.h:1650:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn ControlServiceExA(hService: ::winsvc::SC_HANDLE, dwControl: ::minwindef::DWORD, dwInfoLevel: ::minwindef::DWORD, pControlParams: ::winnt::PVOID) -> ::minwindef::BOOL; /* winsvc.h:1634:1 */
    pub fn ControlServiceExW(hService: ::winsvc::SC_HANDLE, dwControl: ::minwindef::DWORD, dwInfoLevel: ::minwindef::DWORD, pControlParams: ::winnt::PVOID) -> ::minwindef::BOOL; /* winsvc.h:1643:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::NotifyServiceStatusChangeW as NotifyServiceStatusChange; /* winsvc.h:1626:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn NotifyServiceStatusChangeA(hService: ::winsvc::SC_HANDLE, dwNotifyMask: ::minwindef::DWORD, pNotifyBuffer: ::winsvc::PSERVICE_NOTIFYA) -> ::minwindef::DWORD; /* winsvc.h:1612:1 */
    pub fn NotifyServiceStatusChangeW(hService: ::winsvc::SC_HANDLE, dwNotifyMask: ::minwindef::DWORD, pNotifyBuffer: ::winsvc::PSERVICE_NOTIFYW) -> ::minwindef::DWORD; /* winsvc.h:1620:1 */
    pub fn QuerySecurityAccessMask(SecurityInformation: ::winnt::SECURITY_INFORMATION, DesiredAccess: ::minwindef::LPDWORD); /* securitybaseapi.h:1138:1 */
    pub fn QueryServiceDynamicInformation(hServiceStatus: ::winsvc::SERVICE_STATUS_HANDLE, dwInfoLevel: ::minwindef::DWORD, ppDynamicInfo: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winsvc.h:1658:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegCopyTreeW as RegCopyTree; /* winreg.h:1195:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegCopyTreeA(hKeySrc: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, hKeyDest: ::minwindef::HKEY) -> ::winreg::LSTATUS; /* winreg.h:1122:1 */
    pub fn RegCopyTreeW(hKeySrc: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, hKeyDest: ::minwindef::HKEY) -> ::winreg::LSTATUS; /* winreg.h:1188:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegDeleteKeyValueW as RegDeleteKeyValue; /* winreg.h:1064:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegDeleteKeyValueA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, lpValueName: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:1050:1 */
    pub fn RegDeleteKeyValueW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, lpValueName: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:1058:1 */
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
    pub fn RegRenameKey(hKey: ::minwindef::HKEY, lpSubKeyName: ::winnt::LPCWSTR, lpNewKeyName: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:923:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegSetKeyValueW as RegSetKeyValue; /* winreg.h:1092:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegSetKeyValueA(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCSTR, lpValueName: ::winnt::LPCSTR, dwType: ::minwindef::DWORD, lpData: ::minwindef::LPCVOID, cbData: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1072:1 */
    pub fn RegSetKeyValueW(hKey: ::minwindef::HKEY, lpSubKey: ::winnt::LPCWSTR, lpValueName: ::winnt::LPCWSTR, dwType: ::minwindef::DWORD, lpData: ::minwindef::LPCVOID, cbData: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1083:1 */
    pub fn SetSecurityAccessMask(SecurityInformation: ::winnt::SECURITY_INFORMATION, DesiredAccess: ::minwindef::LPDWORD); /* securitybaseapi.h:1219:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn AddConditionalAce(pAcl: ::winnt::PACL, dwAceRevision: ::minwindef::DWORD, AceFlags: ::minwindef::DWORD, AceType: ::minwindef::UCHAR, AccessMask: ::minwindef::DWORD, pSid: ::winnt::PSID, ConditionStr: ::winnt::PWCHAR, ReturnLength: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winbase.h:6326:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn OperationEnd(OperationEndParams: *mut ::winbase::OPERATION_END_PARAMETERS) -> ::minwindef::BOOL; /* winbase.h:6137:1 */
    pub fn OperationStart(OperationStartParams: *mut ::winbase::OPERATION_START_PARAMETERS) -> ::minwindef::BOOL; /* winbase.h:6130:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn EnumDynamicTimeZoneInformation(dwIndex: ::minwindef::DWORD, lpTimeZoneInformation: ::timezoneapi::PDYNAMIC_TIME_ZONE_INFORMATION) -> ::minwindef::DWORD; /* timezoneapi.h:193:1 */
    pub fn GetDynamicTimeZoneInformationEffectiveYears(lpTimeZoneInformation: ::timezoneapi::PDYNAMIC_TIME_ZONE_INFORMATION, FirstYear: ::minwindef::LPDWORD, LastYear: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* timezoneapi.h:203:1 */
}
