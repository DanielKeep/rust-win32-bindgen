#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AbortSystemShutdownW as AbortSystemShutdown; /* winreg.h:1307:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AbortSystemShutdownA(_: ::winnt::LPSTR) -> ::minwindef::BOOL; /* winreg.h:1297:1 */
    pub fn AbortSystemShutdownW(_: ::winnt::LPWSTR) -> ::minwindef::BOOL; /* winreg.h:1303:1 */
    pub fn AccessCheck(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winnt::PGENERIC_MAPPING, _: ::winnt::PPRIVILEGE_SET, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:56:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AccessCheckAndAuditAlarmW as AccessCheckAndAuditAlarm; /* securitybaseapi.h:87:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AccessCheckAndAuditAlarmA(_: ::winnt::LPCSTR, _: ::minwindef::LPVOID, _: ::winnt::LPSTR, _: ::winnt::LPSTR, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::DWORD, _: ::winnt::PGENERIC_MAPPING, _: ::minwindef::BOOL, _: ::minwindef::LPDWORD, _: ::minwindef::LPBOOL, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* winbase.h:6152:1 */
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
    pub fn AdjustTokenGroups(_: ::winnt::HANDLE, _: ::minwindef::BOOL, _: ::winnt::PTOKEN_GROUPS, _: ::minwindef::DWORD, _: ::winnt::PTOKEN_GROUPS, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:392:1 */
    pub fn AdjustTokenPrivileges(_: ::winnt::HANDLE, _: ::minwindef::BOOL, _: ::winnt::PTOKEN_PRIVILEGES, _: ::minwindef::DWORD, _: ::winnt::PTOKEN_PRIVILEGES, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:405:1 */
    pub fn AllocateAndInitializeSid(_: ::winnt::PSID_IDENTIFIER_AUTHORITY, _: ::minwindef::BYTE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:418:1 */
    pub fn AllocateLocallyUniqueId(_: ::winnt::PLUID) -> ::minwindef::BOOL; /* securitybaseapi.h:436:1 */
    pub fn AreAllAccessesGranted(_: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:444:1 */
    pub fn AreAnyAccessesGranted(_: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:453:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::BackupEventLogW as BackupEventLog; /* winbase.h:5907:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn BackupEventLogA(_: ::winnt::HANDLE, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:5895:1 */
    pub fn BackupEventLogW(_: ::winnt::HANDLE, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:5902:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ChangeServiceConfigW as ChangeServiceConfig; /* winsvc.h:1035:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ChangeServiceConfig2W as ChangeServiceConfig2; /* winsvc.h:1057:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ChangeServiceConfig2A(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winsvc.h:1043:1 */
    pub fn ChangeServiceConfig2W(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winsvc.h:1051:1 */
    pub fn ChangeServiceConfigA(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::LPDWORD, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winsvc.h:1005:1 */
    pub fn ChangeServiceConfigW(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::LPDWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winsvc.h:1021:1 */
    pub fn CheckForHiberboot(_: ::winnt::PBOOLEAN, _: ::winnt::BOOLEAN) -> ::minwindef::DWORD; /* winreg.h:1414:1 */
    pub fn CheckTokenMembership(_: ::winnt::HANDLE, _: ::winnt::PSID, _: ::minwindef::PBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:462:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ClearEventLogW as ClearEventLog; /* winbase.h:5887:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ClearEventLogA(_: ::winnt::HANDLE, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:5875:1 */
    pub fn ClearEventLogW(_: ::winnt::HANDLE, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:5882:1 */
    pub fn CloseEncryptedFileRaw(_: ::winnt::PVOID); /* winbase.h:2616:1 */
    pub fn CloseEventLog(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5915:1 */
    pub fn CloseServiceHandle(_: ::winsvc::SC_HANDLE) -> ::minwindef::BOOL; /* winsvc.h:1065:1 */
    pub fn ControlService(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::winsvc::LPSERVICE_STATUS) -> ::minwindef::BOOL; /* winsvc.h:1072:1 */
    pub fn ConvertToAutoInheritPrivateObjectSecurity(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: *mut ::guiddef::GUID, _: ::winnt::BOOLEAN, _: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:513:1 */
    pub fn CopySid(_: ::minwindef::DWORD, _: ::winnt::PSID, _: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:526:1 */
    pub fn CreatePrivateObjectSecurity(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: ::minwindef::BOOL, _: ::winnt::HANDLE, _: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:536:1 */
    pub fn CreatePrivateObjectSecurityEx(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: *mut ::guiddef::GUID, _: ::minwindef::BOOL, _: ::minwindef::ULONG, _: ::winnt::HANDLE, _: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:549:1 */
    pub fn CreatePrivateObjectSecurityWithMultipleInheritance(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: *mut *mut ::guiddef::GUID, _: ::minwindef::ULONG, _: ::minwindef::BOOL, _: ::minwindef::ULONG, _: ::winnt::HANDLE, _: ::winnt::PGENERIC_MAPPING) -> ::minwindef::BOOL; /* securitybaseapi.h:564:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateProcessAsUserW as CreateProcessAsUser; /* processthreadsapi.h:597:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateProcessAsUserA(_: ::winnt::HANDLE, _: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::BOOL, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::winnt::LPCSTR, _: ::processthreadsapi::LPSTARTUPINFOA, _: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* winbase.h:6987:1 */
    pub fn CreateProcessAsUserW(_: ::winnt::HANDLE, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::BOOL, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::winnt::LPCWSTR, _: ::processthreadsapi::LPSTARTUPINFOW, _: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* processthreadsapi.h:581:1 */
    pub fn CreateRestrictedToken(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PSID_AND_ATTRIBUTES, _: ::minwindef::DWORD, _: ::winnt::PLUID_AND_ATTRIBUTES, _: ::minwindef::DWORD, _: ::winnt::PSID_AND_ATTRIBUTES, _: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:580:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateServiceW as CreateService; /* winsvc.h:1117:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateServiceA(_: ::winsvc::SC_HANDLE, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::LPDWORD, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::winsvc::SC_HANDLE; /* winsvc.h:1082:1 */
    pub fn CreateServiceW(_: ::winsvc::SC_HANDLE, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::LPDWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::winsvc::SC_HANDLE; /* winsvc.h:1101:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DecryptFileW as DecryptFile; /* winbase.h:2500:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DecryptFileA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2488:1 */
    pub fn DecryptFileW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winbase.h:2495:1 */
    pub fn DeleteAce(_: ::winnt::PACL, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:624:1 */
    pub fn DeleteService(_: ::winsvc::SC_HANDLE) -> ::minwindef::BOOL; /* winsvc.h:1125:1 */
    pub fn DeregisterEventSource(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5922:1 */
    pub fn DestroyPrivateObjectSecurity(_: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:633:1 */
    pub fn DuplicateToken(_: ::winnt::HANDLE, _: ::winnt::SECURITY_IMPERSONATION_LEVEL, _: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:641:1 */
    pub fn DuplicateTokenEx(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::winnt::SECURITY_IMPERSONATION_LEVEL, _: ::winnt::TOKEN_TYPE, _: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:651:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EncryptFileW as EncryptFile; /* winbase.h:2480:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EncryptFileA(_: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winbase.h:2470:1 */
    pub fn EncryptFileW(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winbase.h:2476:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumDependentServicesW as EnumDependentServices; /* winsvc.h:1156:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumDependentServicesA(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::winsvc::LPENUM_SERVICE_STATUSA, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1133:1 */
    pub fn EnumDependentServicesW(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::winsvc::LPENUM_SERVICE_STATUSW, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1146:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumServicesStatusW as EnumServicesStatus; /* winsvc.h:1192:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumServicesStatusA(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winsvc::LPENUM_SERVICE_STATUSA, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1165:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumServicesStatusExW as EnumServicesStatusEx; /* winsvc.h:1232:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumServicesStatusExA(_: ::winsvc::SC_HANDLE, _: ::winsvc::SC_ENUM_TYPE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::LPBYTE, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* winsvc.h:1201:1 */
    pub fn EnumServicesStatusExW(_: ::winsvc::SC_HANDLE, _: ::winsvc::SC_ENUM_TYPE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::LPBYTE, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* winsvc.h:1218:1 */
    pub fn EnumServicesStatusW(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winsvc::LPENUM_SERVICE_STATUSW, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1180:1 */
    pub fn EqualPrefixSid(_: ::winnt::PSID, _: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:664:1 */
    pub fn EqualSid(_: ::winnt::PSID, _: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:673:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::FileEncryptionStatusW as FileEncryptionStatus; /* winbase.h:2535:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn FileEncryptionStatusA(_: ::winnt::LPCSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2523:1 */
    pub fn FileEncryptionStatusW(_: ::winnt::LPCWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:2530:1 */
    pub fn FindFirstFreeAce(_: ::winnt::PACL, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:682:1 */
    pub fn FreeSid(_: ::winnt::PSID) -> ::winnt::PVOID; /* securitybaseapi.h:691:1 */
    pub fn GetAce(_: ::winnt::PACL, _: ::minwindef::DWORD, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* securitybaseapi.h:699:1 */
    pub fn GetAclInformation(_: ::winnt::PACL, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::winnt::ACL_INFORMATION_CLASS) -> ::minwindef::BOOL; /* securitybaseapi.h:709:1 */
    pub fn GetEventLogInformation(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6086:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetFileSecurityW as GetFileSecurity; /* securitybaseapi.h:730:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetFileSecurityA(_: ::winnt::LPCSTR, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6353:1 */
    pub fn GetFileSecurityW(_: ::winnt::LPCWSTR, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:720:1 */
    pub fn GetKernelObjectSecurity(_: ::winnt::HANDLE, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:736:1 */
    pub fn GetLengthSid(_: ::winnt::PSID) -> ::minwindef::DWORD; /* securitybaseapi.h:750:1 */
    pub fn GetNumberOfEventLogRecords(_: ::winnt::HANDLE, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:5937:1 */
    pub fn GetOldestEventLogRecord(_: ::winnt::HANDLE, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* winbase.h:5945:1 */
    pub fn GetPrivateObjectSecurity(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:759:1 */
    pub fn GetSecurityDescriptorControl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSECURITY_DESCRIPTOR_CONTROL, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:771:1 */
    pub fn GetSecurityDescriptorDacl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::LPBOOL, _: *mut *mut ::winnt::ACL, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:781:1 */
    pub fn GetSecurityDescriptorGroup(_: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:792:1 */
    pub fn GetSecurityDescriptorLength(_: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::DWORD; /* securitybaseapi.h:802:1 */
    pub fn GetSecurityDescriptorOwner(_: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:810:1 */
    pub fn GetSecurityDescriptorRMControl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::PUCHAR) -> ::minwindef::DWORD; /* securitybaseapi.h:820:1 */
    pub fn GetSecurityDescriptorSacl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::LPBOOL, _: *mut *mut ::winnt::ACL, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:829:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetServiceDisplayNameW as GetServiceDisplayName; /* winsvc.h:1288:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetServiceDisplayNameA(_: ::winsvc::SC_HANDLE, _: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1269:1 */
    pub fn GetServiceDisplayNameW(_: ::winsvc::SC_HANDLE, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1280:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetServiceKeyNameW as GetServiceKeyName; /* winsvc.h:1260:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetServiceKeyNameA(_: ::winsvc::SC_HANDLE, _: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1241:1 */
    pub fn GetServiceKeyNameW(_: ::winsvc::SC_HANDLE, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1252:1 */
    pub fn GetSidIdentifierAuthority(_: ::winnt::PSID) -> ::winnt::PSID_IDENTIFIER_AUTHORITY; /* securitybaseapi.h:840:1 */
    pub fn GetSidLengthRequired(_: ::minwindef::UCHAR) -> ::minwindef::DWORD; /* securitybaseapi.h:848:1 */
    pub fn GetSidSubAuthority(_: ::winnt::PSID, _: ::minwindef::DWORD) -> ::minwindef::PDWORD; /* securitybaseapi.h:856:1 */
    pub fn GetSidSubAuthorityCount(_: ::winnt::PSID) -> ::minwindef::PUCHAR; /* securitybaseapi.h:865:1 */
    pub fn GetTokenInformation(_: ::winnt::HANDLE, _: ::winnt::TOKEN_INFORMATION_CLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::minwindef::PDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:873:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetUserNameW as GetUserName; /* winbase.h:6881:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetUserNameA(_: ::winnt::LPSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6869:1 */
    pub fn GetUserNameW(_: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6876:1 */
    pub fn ImpersonateAnonymousToken(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:901:1 */
    pub fn ImpersonateLoggedOnUser(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:910:1 */
    pub fn ImpersonateNamedPipeClient(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* namedpipeapi.h:175:1 */
    pub fn ImpersonateSelf(_: ::winnt::SECURITY_IMPERSONATION_LEVEL) -> ::minwindef::BOOL; /* securitybaseapi.h:919:1 */
    pub fn InitializeAcl(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:927:1 */
    pub fn InitializeSecurityDescriptor(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:937:1 */
    pub fn InitializeSid(_: ::winnt::PSID, _: ::winnt::PSID_IDENTIFIER_AUTHORITY, _: ::minwindef::BYTE) -> ::minwindef::BOOL; /* securitybaseapi.h:946:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::InitiateShutdownW as InitiateShutdown; /* winreg.h:1406:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn InitiateShutdownA(_: ::winnt::LPSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winreg.h:1388:1 */
    pub fn InitiateShutdownW(_: ::winnt::LPWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* winreg.h:1398:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::InitiateSystemShutdownW as InitiateSystemShutdown; /* winreg.h:1288:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn InitiateSystemShutdownA(_: ::winnt::LPSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winreg.h:1269:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::InitiateSystemShutdownExW as InitiateSystemShutdownEx; /* winreg.h:1364:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn InitiateSystemShutdownExA(_: ::winnt::LPSTR, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::BOOL, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winreg.h:1341:1 */
    pub fn InitiateSystemShutdownExW(_: ::winnt::LPWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::BOOL, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* winreg.h:1355:1 */
    pub fn InitiateSystemShutdownW(_: ::winnt::LPWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winreg.h:1280:1 */
    pub fn IsTextUnicode(_: *const ::libc::c_void, _: ::libc::c_int, _: ::minwindef::LPINT) -> ::minwindef::BOOL; /* winbase.h:2841:1 */
    pub fn IsTokenRestricted(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:956:1 */
    pub fn IsTokenUntrusted(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:7051:1 */
    pub fn IsValidAcl(_: ::winnt::PACL) -> ::minwindef::BOOL; /* securitybaseapi.h:964:1 */
    pub fn IsValidSecurityDescriptor(_: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* securitybaseapi.h:972:1 */
    pub fn IsValidSid(_: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:980:1 */
    pub fn LockServiceDatabase(_: ::winsvc::SC_HANDLE) -> ::winsvc::SC_LOCK; /* winsvc.h:1296:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LogonUserW as LogonUser; /* winbase.h:6937:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LogonUserA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* winbase.h:6917:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LogonUserExW as LogonUserEx; /* winbase.h:6973:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LogonUserExA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PHANDLE, _: *mut *mut ::libc::c_void, _: *mut *mut ::libc::c_void, _: ::minwindef::LPDWORD, _: ::winnt::PQUOTA_LIMITS) -> ::minwindef::BOOL; /* winbase.h:6945:1 */
    pub fn LogonUserExW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PHANDLE, _: *mut *mut ::libc::c_void, _: *mut *mut ::libc::c_void, _: ::minwindef::LPDWORD, _: ::winnt::PQUOTA_LIMITS) -> ::minwindef::BOOL; /* winbase.h:6960:1 */
    pub fn LogonUserW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* winbase.h:6928:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupAccountNameW as LookupAccountName; /* winbase.h:6512:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupAccountNameA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::PSID, _: ::minwindef::LPDWORD, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6490:1 */
    pub fn LookupAccountNameW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::PSID, _: ::minwindef::LPDWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6502:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupAccountSidW as LookupAccountSid; /* winbase.h:6482:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupAccountSidA(_: ::winnt::LPCSTR, _: ::winnt::PSID, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6460:1 */
    pub fn LookupAccountSidW(_: ::winnt::LPCWSTR, _: ::winnt::PSID, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::winnt::PSID_NAME_USE) -> ::minwindef::BOOL; /* winbase.h:6472:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupPrivilegeDisplayNameW as LookupPrivilegeDisplayName; /* winbase.h:6666:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupPrivilegeDisplayNameA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6648:1 */
    pub fn LookupPrivilegeDisplayNameW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6658:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupPrivilegeNameW as LookupPrivilegeName; /* winbase.h:6640:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupPrivilegeNameA(_: ::winnt::LPCSTR, _: ::winnt::PLUID, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6624:1 */
    pub fn LookupPrivilegeNameW(_: ::winnt::LPCWSTR, _: ::winnt::PLUID, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winbase.h:6633:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::LookupPrivilegeValueW as LookupPrivilegeValue; /* winbase.h:6616:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn LookupPrivilegeValueA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::PLUID) -> ::minwindef::BOOL; /* winbase.h:6602:1 */
    pub fn LookupPrivilegeValueW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::PLUID) -> ::minwindef::BOOL; /* winbase.h:6610:1 */
    pub fn MakeAbsoluteSD(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::LPDWORD, _: ::winnt::PACL, _: ::minwindef::LPDWORD, _: ::winnt::PACL, _: ::minwindef::LPDWORD, _: ::winnt::PSID, _: ::minwindef::LPDWORD, _: ::winnt::PSID, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:1003:1 */
    pub fn MakeSelfRelativeSD(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:1022:1 */
    pub fn MapGenericMask(_: ::minwindef::PDWORD, _: ::winnt::PGENERIC_MAPPING); /* securitybaseapi.h:1032:1 */
    pub fn NotifyBootConfigStatus(_: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winsvc.h:1303:1 */
    pub fn NotifyChangeEventLog(_: ::winnt::HANDLE, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* winbase.h:5929:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectCloseAuditAlarmW as ObjectCloseAuditAlarm; /* securitybaseapi.h:1049:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectCloseAuditAlarmA(_: ::winnt::LPCSTR, _: ::minwindef::LPVOID, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:6287:1 */
    pub fn ObjectCloseAuditAlarmW(_: ::winnt::LPCWSTR, _: ::minwindef::LPVOID, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1041:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectDeleteAuditAlarmW as ObjectDeleteAuditAlarm; /* securitybaseapi.h:1063:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectDeleteAuditAlarmA(_: ::winnt::LPCSTR, _: ::minwindef::LPVOID, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:6299:1 */
    pub fn ObjectDeleteAuditAlarmW(_: ::winnt::LPCWSTR, _: ::minwindef::LPVOID, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1055:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectOpenAuditAlarmW as ObjectOpenAuditAlarm; /* securitybaseapi.h:1086:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectOpenAuditAlarmA(_: ::winnt::LPCSTR, _: ::minwindef::LPVOID, _: ::winnt::LPSTR, _: ::winnt::LPSTR, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PPRIVILEGE_SET, _: ::minwindef::BOOL, _: ::minwindef::BOOL, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* winbase.h:6251:1 */
    pub fn ObjectOpenAuditAlarmW(_: ::winnt::LPCWSTR, _: ::minwindef::LPVOID, _: ::winnt::LPWSTR, _: ::winnt::LPWSTR, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PPRIVILEGE_SET, _: ::minwindef::BOOL, _: ::minwindef::BOOL, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1069:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ObjectPrivilegeAuditAlarmW as ObjectPrivilegeAuditAlarm; /* securitybaseapi.h:1103:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ObjectPrivilegeAuditAlarmA(_: ::winnt::LPCSTR, _: ::minwindef::LPVOID, _: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winnt::PPRIVILEGE_SET, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:6272:1 */
    pub fn ObjectPrivilegeAuditAlarmW(_: ::winnt::LPCWSTR, _: ::minwindef::LPVOID, _: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winnt::PPRIVILEGE_SET, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1092:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenBackupEventLogW as OpenBackupEventLog; /* winbase.h:6005:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenBackupEventLogA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:5993:1 */
    pub fn OpenBackupEventLogW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winbase.h:6000:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenEncryptedFileRawW as OpenEncryptedFileRaw; /* winbase.h:2590:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenEncryptedFileRawA(_: ::winnt::LPCSTR, _: ::minwindef::ULONG, _: *mut *mut ::libc::c_void) -> ::minwindef::DWORD; /* winbase.h:2576:1 */
    pub fn OpenEncryptedFileRawW(_: ::winnt::LPCWSTR, _: ::minwindef::ULONG, _: *mut *mut ::libc::c_void) -> ::minwindef::DWORD; /* winbase.h:2584:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::OpenEventLogW as OpenEventLog; /* winbase.h:5965:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn OpenEventLogA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:5953:1 */
    pub fn OpenEventLogW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winbase.h:5960:1 */
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
    pub fn OpenThreadToken(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::BOOL, _: ::winnt::PHANDLE) -> ::minwindef::BOOL; /* processthreadsapi.h:632:1 */
    pub fn PrivilegeCheck(_: ::winnt::HANDLE, _: ::winnt::PPRIVILEGE_SET, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1109:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PrivilegedServiceAuditAlarmW as PrivilegedServiceAuditAlarm; /* securitybaseapi.h:1129:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PrivilegedServiceAuditAlarmA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::HANDLE, _: ::winnt::PPRIVILEGE_SET, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* winbase.h:6311:1 */
    pub fn PrivilegedServiceAuditAlarmW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::HANDLE, _: ::winnt::PPRIVILEGE_SET, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1119:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::QueryServiceConfigW as QueryServiceConfig; /* winsvc.h:1378:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::QueryServiceConfig2W as QueryServiceConfig2; /* winsvc.h:1422:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn QueryServiceConfig2A(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPBYTE, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1394:1 */
    pub fn QueryServiceConfig2W(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::LPBYTE, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1413:1 */
    pub fn QueryServiceConfigA(_: ::winsvc::SC_HANDLE, _: ::winsvc::LPQUERY_SERVICE_CONFIGA, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1359:1 */
    pub fn QueryServiceConfigW(_: ::winsvc::SC_HANDLE, _: ::winsvc::LPQUERY_SERVICE_CONFIGW, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1370:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::QueryServiceLockStatusW as QueryServiceLockStatus; /* winsvc.h:1450:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn QueryServiceLockStatusA(_: ::winsvc::SC_HANDLE, _: ::winsvc::LPQUERY_SERVICE_LOCK_STATUSA, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1431:1 */
    pub fn QueryServiceLockStatusW(_: ::winsvc::SC_HANDLE, _: ::winsvc::LPQUERY_SERVICE_LOCK_STATUSW, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1442:1 */
    pub fn QueryServiceObjectSecurity(_: ::winsvc::SC_HANDLE, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1459:1 */
    pub fn QueryServiceStatus(_: ::winsvc::SC_HANDLE, _: ::winsvc::LPSERVICE_STATUS) -> ::minwindef::BOOL; /* winsvc.h:1472:1 */
    pub fn QueryServiceStatusEx(_: ::winsvc::SC_HANDLE, _: ::winsvc::SC_STATUS_TYPE, _: ::minwindef::LPBYTE, _: ::minwindef::DWORD, _: ::minwindef::LPDWORD) -> ::minwindef::BOOL; /* winsvc.h:1481:1 */
    pub fn ReadEncryptedFileRaw(_: ::winbase::PFE_EXPORT_FUNC, _: ::winnt::PVOID, _: ::winnt::PVOID) -> ::minwindef::DWORD; /* winbase.h:2598:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ReadEventLogW as ReadEventLog; /* winbase.h:6035:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ReadEventLogA(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: *mut ::libc::c_ulong, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winbase.h:6013:1 */
    pub fn ReadEventLogW(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: *mut ::libc::c_ulong, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winbase.h:6025:1 */
    pub fn RegCloseKey(_: ::minwindef::HKEY) -> ::winreg::LSTATUS; /* winreg.h:206:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegConnectRegistryW as RegConnectRegistry; /* winreg.h:271:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegConnectRegistryA(_: ::winnt::LPCSTR, _: ::minwindef::HKEY, _: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:257:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegConnectRegistryExW as RegConnectRegistryEx; /* winreg.h:295:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegConnectRegistryExA(_: ::winnt::LPCSTR, _: ::minwindef::HKEY, _: ::minwindef::ULONG, _: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:279:1 */
    pub fn RegConnectRegistryExW(_: ::winnt::LPCWSTR, _: ::minwindef::HKEY, _: ::minwindef::ULONG, _: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:288:1 */
    pub fn RegConnectRegistryW(_: ::winnt::LPCWSTR, _: ::minwindef::HKEY, _: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:265:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegCreateKeyW as RegCreateKey; /* winreg.h:317:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegCreateKeyA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:303:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegCreateKeyExW as RegCreateKeyEx; /* winreg.h:353:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegCreateKeyExA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::winreg::REGSAM, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::PHKEY, _: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:325:1 */
    pub fn RegCreateKeyExW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::winreg::REGSAM, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::PHKEY, _: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:340:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegCreateKeyTransactedW as RegCreateKeyTransacted; /* winreg.h:391:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegCreateKeyTransactedA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::DWORD, _: ::winreg::REGSAM, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::PHKEY, _: ::minwindef::LPDWORD, _: ::winnt::HANDLE, _: ::winnt::PVOID) -> ::winreg::LSTATUS; /* winreg.h:361:1 */
    pub fn RegCreateKeyTransactedW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::winreg::REGSAM, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::PHKEY, _: ::minwindef::LPDWORD, _: ::winnt::HANDLE, _: ::winnt::PVOID) -> ::winreg::LSTATUS; /* winreg.h:377:1 */
    pub fn RegCreateKeyW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:311:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegDeleteKeyW as RegDeleteKey; /* winreg.h:411:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegDeleteKeyA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:399:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegDeleteKeyExW as RegDeleteKeyEx; /* winreg.h:437:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegDeleteKeyExA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::winreg::REGSAM, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:419:1 */
    pub fn RegDeleteKeyExW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::winreg::REGSAM, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:429:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegDeleteKeyTransactedW as RegDeleteKeyTransacted; /* winreg.h:465:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegDeleteKeyTransactedA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::winreg::REGSAM, _: ::minwindef::DWORD, _: ::winnt::HANDLE, _: ::winnt::PVOID) -> ::winreg::LSTATUS; /* winreg.h:445:1 */
    pub fn RegDeleteKeyTransactedW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::winreg::REGSAM, _: ::minwindef::DWORD, _: ::winnt::HANDLE, _: ::winnt::PVOID) -> ::winreg::LSTATUS; /* winreg.h:456:1 */
    pub fn RegDeleteKeyW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:406:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegDeleteValueW as RegDeleteValue; /* winreg.h:509:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegDeleteValueA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:495:1 */
    pub fn RegDeleteValueW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:503:1 */
    pub fn RegDisablePredefinedCache() -> ::winreg::LSTATUS; /* winreg.h:242:1 */
    pub fn RegDisablePredefinedCacheEx() -> ::winreg::LSTATUS; /* winreg.h:249:1 */
    pub fn RegDisableReflectionKey(_: ::minwindef::HKEY) -> ::winnt::LONG; /* winreg.h:473:1 */
    pub fn RegEnableReflectionKey(_: ::minwindef::HKEY) -> ::winnt::LONG; /* winreg.h:480:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegEnumKeyW as RegEnumKey; /* winreg.h:533:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegEnumKeyA(_: ::minwindef::HKEY, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:517:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegEnumKeyExW as RegEnumKeyEx; /* winreg.h:567:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegEnumKeyExA(_: ::minwindef::HKEY, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:541:1 */
    pub fn RegEnumKeyExW(_: ::minwindef::HKEY, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:555:1 */
    pub fn RegEnumKeyW(_: ::minwindef::HKEY, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:526:1 */
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
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegOpenKeyW as RegOpenKey; /* winreg.h:678:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegOpenKeyA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:664:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegOpenKeyExW as RegOpenKeyEx; /* winreg.h:706:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegOpenKeyExA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winreg::REGSAM, _: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:686:1 */
    pub fn RegOpenKeyExW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winreg::REGSAM, _: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:697:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegOpenKeyTransactedW as RegOpenKeyTransacted; /* winreg.h:736:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegOpenKeyTransactedA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winreg::REGSAM, _: ::minwindef::PHKEY, _: ::winnt::HANDLE, _: ::winnt::PVOID) -> ::winreg::LSTATUS; /* winreg.h:714:1 */
    pub fn RegOpenKeyTransactedW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winreg::REGSAM, _: ::minwindef::PHKEY, _: ::winnt::HANDLE, _: ::winnt::PVOID) -> ::winreg::LSTATUS; /* winreg.h:726:1 */
    pub fn RegOpenKeyW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:672:1 */
    pub fn RegOpenUserClassesRoot(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winreg::REGSAM, _: ::minwindef::PHKEY) -> ::winreg::LSTATUS; /* winreg.h:222:1 */
    pub fn RegOverridePredefKey(_: ::minwindef::HKEY, _: ::minwindef::HKEY) -> ::winreg::LSTATUS; /* winreg.h:214:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegQueryInfoKeyW as RegQueryInfoKey; /* winreg.h:778:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegQueryInfoKeyA(_: ::minwindef::HKEY, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:744:1 */
    pub fn RegQueryInfoKeyW(_: ::minwindef::HKEY, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::PFILETIME) -> ::winreg::LSTATUS; /* winreg.h:762:1 */
    pub fn RegQueryReflectionKey(_: ::minwindef::HKEY, _: *mut ::libc::c_int) -> ::winnt::LONG; /* winreg.h:487:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegQueryValueW as RegQueryValue; /* winreg.h:802:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegQueryValueA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::winnt::LPSTR, _: ::winnt::PLONG) -> ::winreg::LSTATUS; /* winreg.h:786:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegQueryValueExW as RegQueryValueEx; /* winreg.h:864:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegQueryValueExA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPBYTE, _: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:840:1 */
    pub fn RegQueryValueExW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPBYTE, _: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:853:1 */
    pub fn RegQueryValueW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::winnt::PLONG) -> ::winreg::LSTATUS; /* winreg.h:795:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegReplaceKeyW as RegReplaceKey; /* winreg.h:888:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegReplaceKeyA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:872:1 */
    pub fn RegReplaceKeyW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:881:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegRestoreKeyW as RegRestoreKey; /* winreg.h:912:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegRestoreKeyA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:896:1 */
    pub fn RegRestoreKeyW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:905:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegSaveKeyW as RegSaveKey; /* winreg.h:948:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegSaveKeyA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::winreg::LSTATUS; /* winreg.h:934:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegSaveKeyExW as RegSaveKeyEx; /* winreg.h:1440:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegSaveKeyExA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1422:1 */
    pub fn RegSaveKeyExW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1432:1 */
    pub fn RegSaveKeyW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwinbase::LPSECURITY_ATTRIBUTES) -> ::winreg::LSTATUS; /* winreg.h:942:1 */
    pub fn RegSetKeySecurity(_: ::minwindef::HKEY, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR) -> ::winreg::LSTATUS; /* winreg.h:956:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegSetValueW as RegSetValue; /* winreg.h:984:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegSetValueA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:966:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegSetValueExW as RegSetValueEx; /* winreg.h:1014:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegSetValueExA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *const ::libc::c_uchar, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:992:1 */
    pub fn RegSetValueExW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *const ::libc::c_uchar, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1004:1 */
    pub fn RegSetValueW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:976:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegUnLoadKeyW as RegUnLoadKey; /* winreg.h:1036:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegUnLoadKeyA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:1022:1 */
    pub fn RegUnLoadKeyW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:1030:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RegisterEventSourceW as RegisterEventSource; /* winbase.h:5985:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RegisterEventSourceA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::winnt::HANDLE; /* winbase.h:5973:1 */
    pub fn RegisterEventSourceW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::winnt::HANDLE; /* winbase.h:5980:1 */
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
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ReportEventW as ReportEvent; /* winbase.h:6069:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ReportEventA(_: ::winnt::HANDLE, _: ::minwindef::WORD, _: ::minwindef::WORD, _: ::minwindef::DWORD, _: ::winnt::PSID, _: ::minwindef::WORD, _: ::minwindef::DWORD, _: *mut *const ::libc::c_schar, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winbase.h:6043:1 */
    pub fn ReportEventW(_: ::winnt::HANDLE, _: ::minwindef::WORD, _: ::minwindef::WORD, _: ::minwindef::DWORD, _: ::winnt::PSID, _: ::minwindef::WORD, _: ::minwindef::DWORD, _: *mut *const ::libc::c_ushort, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* winbase.h:6057:1 */
    pub fn RevertToSelf() -> ::minwindef::BOOL; /* securitybaseapi.h:1149:1 */
    pub fn SetAclInformation(_: ::winnt::PACL, _: ::minwindef::LPVOID, _: ::minwindef::DWORD, _: ::winnt::ACL_INFORMATION_CLASS) -> ::minwindef::BOOL; /* securitybaseapi.h:1157:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::SetFileSecurityW as SetFileSecurity; /* securitybaseapi.h:1175:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn SetFileSecurityA(_: ::winnt::LPCSTR, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* winbase.h:6341:1 */
    pub fn SetFileSecurityW(_: ::winnt::LPCWSTR, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* securitybaseapi.h:1168:1 */
    pub fn SetKernelObjectSecurity(_: ::winnt::HANDLE, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* securitybaseapi.h:1181:1 */
    pub fn SetPrivateObjectSecurity(_: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: ::winnt::PGENERIC_MAPPING, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:1191:1 */
    pub fn SetPrivateObjectSecurityEx(_: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR, _: *mut *mut ::libc::c_void, _: ::minwindef::ULONG, _: ::winnt::PGENERIC_MAPPING, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* securitybaseapi.h:1203:1 */
    pub fn SetSecurityDescriptorControl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::SECURITY_DESCRIPTOR_CONTROL, _: ::winnt::SECURITY_DESCRIPTOR_CONTROL) -> ::minwindef::BOOL; /* securitybaseapi.h:1230:1 */
    pub fn SetSecurityDescriptorDacl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::BOOL, _: ::winnt::PACL, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1240:1 */
    pub fn SetSecurityDescriptorGroup(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSID, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1251:1 */
    pub fn SetSecurityDescriptorOwner(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSID, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1261:1 */
    pub fn SetSecurityDescriptorRMControl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::PUCHAR) -> ::minwindef::DWORD; /* securitybaseapi.h:1271:1 */
    pub fn SetSecurityDescriptorSacl(_: ::winnt::PSECURITY_DESCRIPTOR, _: ::minwindef::BOOL, _: ::winnt::PACL, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* securitybaseapi.h:1280:1 */
    pub fn SetServiceObjectSecurity(_: ::winsvc::SC_HANDLE, _: ::winnt::SECURITY_INFORMATION, _: ::winnt::PSECURITY_DESCRIPTOR) -> ::minwindef::BOOL; /* winsvc.h:1543:1 */
    pub fn SetServiceStatus(_: ::winsvc::SERVICE_STATUS_HANDLE, _: ::winsvc::LPSERVICE_STATUS) -> ::minwindef::BOOL; /* winsvc.h:1552:1 */
    pub fn SetThreadToken(_: ::winnt::PHANDLE, _: ::winnt::HANDLE) -> ::minwindef::BOOL; /* processthreadsapi.h:613:1 */
    pub fn SetTokenInformation(_: ::winnt::HANDLE, _: ::winnt::TOKEN_INFORMATION_CLASS, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* securitybaseapi.h:1291:1 */
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
    pub fn UnlockServiceDatabase(_: ::winsvc::SC_LOCK) -> ::minwindef::BOOL; /* winsvc.h:1603:1 */
    pub fn WriteEncryptedFileRaw(_: ::winbase::PFE_IMPORT_FUNC, _: ::winnt::PVOID, _: ::winnt::PVOID) -> ::minwindef::DWORD; /* winbase.h:2607:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::GetCurrentHwProfileW as GetCurrentHwProfile; /* winbase.h:7322:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn GetCurrentHwProfileA(_: ::winbase::LPHW_PROFILE_INFOA) -> ::minwindef::BOOL; /* winbase.h:7312:1 */
    pub fn GetCurrentHwProfileW(_: ::winbase::LPHW_PROFILE_INFOW) -> ::minwindef::BOOL; /* winbase.h:7318:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::RegQueryMultipleValuesW as RegQueryMultipleValues; /* winreg.h:831:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn RegQueryMultipleValuesA(_: ::minwindef::HKEY, _: ::winreg::PVALENTA, _: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:813:1 */
    pub fn RegQueryMultipleValuesW(_: ::minwindef::HKEY, _: ::winreg::PVALENTW, _: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::minwindef::LPDWORD) -> ::winreg::LSTATUS; /* winreg.h:823:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn AccessCheckByTypeAndAuditAlarmA(_: ::winnt::LPCSTR, _: ::minwindef::LPVOID, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSID, _: ::minwindef::DWORD, _: ::winnt::AUDIT_EVENT_TYPE, _: ::minwindef::DWORD, _: ::winnt::POBJECT_TYPE_LIST, _: ::minwindef::DWORD, _: ::winnt::PGENERIC_MAPPING, _: ::minwindef::BOOL, _: ::minwindef::LPDWORD, _: ::minwindef::LPBOOL, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* winbase.h:6174:1 */
    pub fn AccessCheckByTypeResultListAndAuditAlarmA(_: ::winnt::LPCSTR, _: ::minwindef::LPVOID, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSID, _: ::minwindef::DWORD, _: ::winnt::AUDIT_EVENT_TYPE, _: ::minwindef::DWORD, _: ::winnt::POBJECT_TYPE_LIST, _: ::minwindef::DWORD, _: ::winnt::PGENERIC_MAPPING, _: ::minwindef::BOOL, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* winbase.h:6199:1 */
    pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleA(_: ::winnt::LPCSTR, _: ::minwindef::LPVOID, _: ::winnt::HANDLE, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::PSECURITY_DESCRIPTOR, _: ::winnt::PSID, _: ::minwindef::DWORD, _: ::winnt::AUDIT_EVENT_TYPE, _: ::minwindef::DWORD, _: ::winnt::POBJECT_TYPE_LIST, _: ::minwindef::DWORD, _: ::winnt::PGENERIC_MAPPING, _: ::minwindef::BOOL, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD, _: ::minwindef::LPBOOL) -> ::minwindef::BOOL; /* winbase.h:6224:1 */
    pub fn CreateProcessWithLogonW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::winnt::LPCWSTR, _: ::processthreadsapi::LPSTARTUPINFOW, _: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* winbase.h:7017:1 */
    pub fn CreateProcessWithTokenW(_: ::winnt::HANDLE, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPWSTR, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: ::winnt::LPCWSTR, _: ::processthreadsapi::LPSTARTUPINFOW, _: ::processthreadsapi::LPPROCESS_INFORMATION) -> ::minwindef::BOOL; /* winbase.h:7034:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] 
extern "system" {
    pub fn CreateWellKnownSid(_: ::winnt::WELL_KNOWN_SID_TYPE, _: ::winnt::PSID, _: ::winnt::PSID, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* securitybaseapi.h:600:1 */
    pub fn EqualDomainSid(_: ::winnt::PSID, _: ::winnt::PSID, _: *mut ::libc::c_int) -> ::minwindef::BOOL; /* securitybaseapi.h:612:1 */
    pub fn GetWindowsAccountDomainSid(_: ::winnt::PSID, _: ::winnt::PSID, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* securitybaseapi.h:889:1 */
    pub fn IsWellKnownSid(_: ::winnt::PSID, _: ::winnt::WELL_KNOWN_SID_TYPE) -> ::minwindef::BOOL; /* securitybaseapi.h:991:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn AddMandatoryAce(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PSID) -> ::minwindef::BOOL; /* securitybaseapi.h:343:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::ControlServiceExW as ControlServiceEx; /* winsvc.h:1650:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn ControlServiceExA(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PVOID) -> ::minwindef::BOOL; /* winsvc.h:1634:1 */
    pub fn ControlServiceExW(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PVOID) -> ::minwindef::BOOL; /* winsvc.h:1643:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::NotifyServiceStatusChangeW as NotifyServiceStatusChange; /* winsvc.h:1626:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn NotifyServiceStatusChangeA(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::winsvc::PSERVICE_NOTIFYA) -> ::minwindef::DWORD; /* winsvc.h:1612:1 */
    pub fn NotifyServiceStatusChangeW(_: ::winsvc::SC_HANDLE, _: ::minwindef::DWORD, _: ::winsvc::PSERVICE_NOTIFYW) -> ::minwindef::DWORD; /* winsvc.h:1620:1 */
    pub fn QuerySecurityAccessMask(_: ::winnt::SECURITY_INFORMATION, _: ::minwindef::LPDWORD); /* securitybaseapi.h:1138:1 */
    pub fn QueryServiceDynamicInformation(_: ::winsvc::SERVICE_STATUS_HANDLE, _: ::minwindef::DWORD, _: *mut *mut ::libc::c_void) -> ::minwindef::BOOL; /* winsvc.h:1658:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegCopyTreeW as RegCopyTree; /* winreg.h:1195:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegCopyTreeA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::minwindef::HKEY) -> ::winreg::LSTATUS; /* winreg.h:1122:1 */
    pub fn RegCopyTreeW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::minwindef::HKEY) -> ::winreg::LSTATUS; /* winreg.h:1188:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegDeleteKeyValueW as RegDeleteKeyValue; /* winreg.h:1064:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegDeleteKeyValueA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::winreg::LSTATUS; /* winreg.h:1050:1 */
    pub fn RegDeleteKeyValueW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:1058:1 */
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
    pub fn RegRenameKey(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::winreg::LSTATUS; /* winreg.h:923:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] #[doc(inline)] pub use self::RegSetKeyValueW as RegSetKeyValue; /* winreg.h:1092:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn RegSetKeyValueA(_: ::minwindef::HKEY, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::minwindef::LPCVOID, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1072:1 */
    pub fn RegSetKeyValueW(_: ::minwindef::HKEY, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::minwindef::LPCVOID, _: ::minwindef::DWORD) -> ::winreg::LSTATUS; /* winreg.h:1083:1 */
    pub fn SetSecurityAccessMask(_: ::winnt::SECURITY_INFORMATION, _: ::minwindef::LPDWORD); /* securitybaseapi.h:1219:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] 
extern "system" {
    pub fn AddConditionalAce(_: ::winnt::PACL, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::UCHAR, _: ::minwindef::DWORD, _: ::winnt::PSID, _: ::winnt::PWCHAR, _: *mut ::libc::c_ulong) -> ::minwindef::BOOL; /* winbase.h:6326:1 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn OperationEnd(_: *mut ::winbase::OPERATION_END_PARAMETERS) -> ::minwindef::BOOL; /* winbase.h:6137:1 */
    pub fn OperationStart(_: *mut ::winbase::OPERATION_START_PARAMETERS) -> ::minwindef::BOOL; /* winbase.h:6130:1 */
}
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06020000"))] 
extern "system" {
    pub fn EnumDynamicTimeZoneInformation(_: ::minwindef::DWORD, _: ::timezoneapi::PDYNAMIC_TIME_ZONE_INFORMATION) -> ::minwindef::DWORD; /* timezoneapi.h:193:1 */
    pub fn GetDynamicTimeZoneInformationEffectiveYears(_: ::timezoneapi::PDYNAMIC_TIME_ZONE_INFORMATION, _: ::minwindef::LPDWORD, _: ::minwindef::LPDWORD) -> ::minwindef::DWORD; /* timezoneapi.h:203:1 */
}
