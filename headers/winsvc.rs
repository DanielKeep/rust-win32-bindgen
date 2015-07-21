#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_TRIGGER_CUSTOM_STATE_ID { Data: *mut [::minwindef::DWORD; 2] } /* winsvc.h:456:9, winsvc.h:456:9, winsvc.h:456:9 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM { u: ::winsvc::SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_Child_0 } /* winsvc.h:461:16, winsvc.h:461:16, winsvc.h:461:16 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub /*union*/ struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_Child_0 { _payload0: u32, _payload1: u32 } #[cfg(feature="winapi_desktop")] union_field! { SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_Child_0.{CustomStateId, CustomStateId_mut}: ::winsvc::SERVICE_TRIGGER_CUSTOM_STATE_ID } #[cfg(feature="winapi_desktop")] union_field! { SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_Child_0.{s, s_mut}: ::winsvc::SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_Child_0_Child_1 } /* winsvc.h:462:5, winsvc.h:462:5, winsvc.h:462:5 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_Child_0_Child_1 { DataOffset: ::minwindef::DWORD, Data: *mut [::minwindef::BYTE; 1] } /* winsvc.h:464:9, winsvc.h:464:9, winsvc.h:464:9 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM = *mut ::winsvc::SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM; /* winsvc.h:469:50, winsvc.h:469:50, winsvc.h:469:50 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_DESCRIPTIONA { lpDescription: ::winnt::LPSTR } /* winsvc.h:486:16, winsvc.h:486:16, winsvc.h:486:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_DESCRIPTIONA = *mut ::winsvc::SERVICE_DESCRIPTIONA; /* winsvc.h:488:26, winsvc.h:488:26, winsvc.h:488:26 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_DESCRIPTIONW { lpDescription: ::winnt::LPWSTR } /* winsvc.h:492:16, winsvc.h:492:16, winsvc.h:492:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_DESCRIPTIONW = *mut ::winsvc::SERVICE_DESCRIPTIONW; /* winsvc.h:494:26, winsvc.h:494:26, winsvc.h:494:26 */
#[cfg(feature="winapi_desktop")] pub type SERVICE_DESCRIPTION = ::winsvc::SERVICE_DESCRIPTIONW; /* winsvc.h:496:30, winsvc.h:496:30, winsvc.h:496:30 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_DESCRIPTION = ::winsvc::LPSERVICE_DESCRIPTIONW; /* winsvc.h:497:32, winsvc.h:497:32, winsvc.h:497:32 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub enum SC_ACTION_TYPE {SC_ACTION_NONE = 0, SC_ACTION_RESTART = 1, SC_ACTION_REBOOT = 2, SC_ACTION_RUN_COMMAND = 3} pub use self::SC_ACTION_TYPE::{SC_ACTION_NONE, SC_ACTION_RESTART, SC_ACTION_REBOOT, SC_ACTION_RUN_COMMAND}; /* winsvc.h:506:14, winsvc.h:506:14, winsvc.h:506:14 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SC_ACTION { Type: ::winsvc::SC_ACTION_TYPE, Delay: ::minwindef::DWORD } /* winsvc.h:513:16, winsvc.h:513:16, winsvc.h:513:16 */
#[cfg(feature="winapi_desktop")] pub type LPSC_ACTION = *mut ::winsvc::SC_ACTION; /* winsvc.h:516:15, winsvc.h:516:15, winsvc.h:516:15 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_FAILURE_ACTIONSA { dwResetPeriod: ::minwindef::DWORD, lpRebootMsg: ::winnt::LPSTR, lpCommand: ::winnt::LPSTR, cActions: ::minwindef::DWORD, lpsaActions: *mut ::winsvc::SC_ACTION } /* winsvc.h:518:16, winsvc.h:518:16, winsvc.h:518:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_FAILURE_ACTIONSA = *mut ::winsvc::SERVICE_FAILURE_ACTIONSA; /* winsvc.h:530:30, winsvc.h:530:30, winsvc.h:530:30 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_FAILURE_ACTIONSW { dwResetPeriod: ::minwindef::DWORD, lpRebootMsg: ::winnt::LPWSTR, lpCommand: ::winnt::LPWSTR, cActions: ::minwindef::DWORD, lpsaActions: *mut ::winsvc::SC_ACTION } /* winsvc.h:531:16, winsvc.h:531:16, winsvc.h:531:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_FAILURE_ACTIONSW = *mut ::winsvc::SERVICE_FAILURE_ACTIONSW; /* winsvc.h:543:30, winsvc.h:543:30, winsvc.h:543:30 */
#[cfg(feature="winapi_desktop")] pub type SERVICE_FAILURE_ACTIONS = ::winsvc::SERVICE_FAILURE_ACTIONSW; /* winsvc.h:545:34, winsvc.h:545:34, winsvc.h:545:34 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_FAILURE_ACTIONS = ::winsvc::LPSERVICE_FAILURE_ACTIONSW; /* winsvc.h:546:36, winsvc.h:546:36, winsvc.h:546:36 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_DELAYED_AUTO_START_INFO { fDelayedAutostart: ::minwindef::BOOL } /* winsvc.h:555:16, winsvc.h:555:16, winsvc.h:555:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_DELAYED_AUTO_START_INFO = *mut ::winsvc::SERVICE_DELAYED_AUTO_START_INFO; /* winsvc.h:557:37, winsvc.h:557:37, winsvc.h:557:37 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_FAILURE_ACTIONS_FLAG { fFailureActionsOnNonCrashFailures: ::minwindef::BOOL } /* winsvc.h:562:16, winsvc.h:562:16, winsvc.h:562:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_FAILURE_ACTIONS_FLAG = *mut ::winsvc::SERVICE_FAILURE_ACTIONS_FLAG; /* winsvc.h:564:34, winsvc.h:564:34, winsvc.h:564:34 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_SID_INFO { dwServiceSidType: ::minwindef::DWORD } /* winsvc.h:569:16, winsvc.h:569:16, winsvc.h:569:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_SID_INFO = *mut ::winsvc::SERVICE_SID_INFO; /* winsvc.h:571:22, winsvc.h:571:22, winsvc.h:571:22 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_REQUIRED_PRIVILEGES_INFOA { pmszRequiredPrivileges: ::winnt::LPSTR } /* winsvc.h:576:16, winsvc.h:576:16, winsvc.h:576:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_REQUIRED_PRIVILEGES_INFOA = *mut ::winsvc::SERVICE_REQUIRED_PRIVILEGES_INFOA; /* winsvc.h:578:39, winsvc.h:578:39, winsvc.h:578:39 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_REQUIRED_PRIVILEGES_INFOW { pmszRequiredPrivileges: ::winnt::LPWSTR } /* winsvc.h:582:16, winsvc.h:582:16, winsvc.h:582:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_REQUIRED_PRIVILEGES_INFOW = *mut ::winsvc::SERVICE_REQUIRED_PRIVILEGES_INFOW; /* winsvc.h:584:39, winsvc.h:584:39, winsvc.h:584:39 */
#[cfg(feature="winapi_desktop")] pub type SERVICE_REQUIRED_PRIVILEGES_INFO = ::winsvc::SERVICE_REQUIRED_PRIVILEGES_INFOW; /* winsvc.h:586:43, winsvc.h:586:43, winsvc.h:586:43 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_REQUIRED_PRIVILEGES_INFO = ::winsvc::LPSERVICE_REQUIRED_PRIVILEGES_INFOW; /* winsvc.h:587:45, winsvc.h:587:45, winsvc.h:587:45 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_PRESHUTDOWN_INFO { dwPreshutdownTimeout: ::minwindef::DWORD } /* winsvc.h:596:16, winsvc.h:596:16, winsvc.h:596:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_PRESHUTDOWN_INFO = *mut ::winsvc::SERVICE_PRESHUTDOWN_INFO; /* winsvc.h:598:30, winsvc.h:598:30, winsvc.h:598:30 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_TRIGGER_SPECIFIC_DATA_ITEM { dwDataType: ::minwindef::DWORD, cbData: ::minwindef::DWORD, pData: ::minwindef::PBYTE } /* winsvc.h:603:16, winsvc.h:603:16, winsvc.h:603:16 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_TRIGGER_SPECIFIC_DATA_ITEM = *mut ::winsvc::SERVICE_TRIGGER_SPECIFIC_DATA_ITEM; /* winsvc.h:614:40, winsvc.h:614:40, winsvc.h:614:40 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_TRIGGER { dwTriggerType: ::minwindef::DWORD, dwAction: ::minwindef::DWORD, pTriggerSubtype: *mut ::guiddef::GUID, cDataItems: ::minwindef::DWORD, pDataItems: ::winsvc::PSERVICE_TRIGGER_SPECIFIC_DATA_ITEM } /* winsvc.h:619:16, winsvc.h:619:16, winsvc.h:619:16 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_TRIGGER = *mut ::winsvc::SERVICE_TRIGGER; /* winsvc.h:634:21, winsvc.h:634:21, winsvc.h:634:21 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_TRIGGER_INFO { cTriggers: ::minwindef::DWORD, pTriggers: ::winsvc::PSERVICE_TRIGGER, pReserved: ::minwindef::PBYTE } /* winsvc.h:639:16, winsvc.h:639:16, winsvc.h:639:16 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_TRIGGER_INFO = *mut ::winsvc::SERVICE_TRIGGER_INFO; /* winsvc.h:649:26, winsvc.h:649:26, winsvc.h:649:26 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_PREFERRED_NODE_INFO { usPreferredNode: ::minwindef::USHORT, fDelete: ::winnt::BOOLEAN } /* winsvc.h:654:16, winsvc.h:654:16, winsvc.h:654:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_PREFERRED_NODE_INFO = *mut ::winsvc::SERVICE_PREFERRED_NODE_INFO; /* winsvc.h:657:33, winsvc.h:657:33, winsvc.h:657:33 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_TIMECHANGE_INFO { liNewTime: ::winnt::LARGE_INTEGER, liOldTime: ::winnt::LARGE_INTEGER } /* winsvc.h:662:16, winsvc.h:662:16, winsvc.h:662:16 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_TIMECHANGE_INFO = *mut ::winsvc::SERVICE_TIMECHANGE_INFO; /* winsvc.h:665:29, winsvc.h:665:29, winsvc.h:665:29 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_LAUNCH_PROTECTED_INFO { dwLaunchProtected: ::minwindef::DWORD } /* winsvc.h:670:16, winsvc.h:670:16, winsvc.h:670:16 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_LAUNCH_PROTECTED_INFO = *mut ::winsvc::SERVICE_LAUNCH_PROTECTED_INFO; /* winsvc.h:672:35, winsvc.h:672:35, winsvc.h:672:35 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SC_HANDLE__ { unused: ::libc::c_int } /* winsvc.h:678:1, winsvc.h:678:1, winsvc.h:678:1 */
#[cfg(feature="winapi_desktop")] pub type SC_HANDLE = *mut ::winsvc::SC_HANDLE__; /* winsvc.h:678:1, winsvc.h:678:1, winsvc.h:678:1 */
#[cfg(feature="winapi_desktop")] pub type LPSC_HANDLE = *mut *mut ::winsvc::SC_HANDLE__; /* winsvc.h:679:22, winsvc.h:679:22, winsvc.h:679:22 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_STATUS_HANDLE__ { unused: ::libc::c_int } /* winsvc.h:681:1, winsvc.h:681:1, winsvc.h:681:1 */
#[cfg(feature="winapi_desktop")] pub type SERVICE_STATUS_HANDLE = *mut ::winsvc::SERVICE_STATUS_HANDLE__; /* winsvc.h:681:1, winsvc.h:681:1, winsvc.h:681:1 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub enum SC_STATUS_TYPE {SC_STATUS_PROCESS_INFO = 0, __SeeGhIssue10292} pub use self::SC_STATUS_TYPE::{SC_STATUS_PROCESS_INFO}; /* winsvc.h:687:14, winsvc.h:687:14, winsvc.h:687:14 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub enum SC_ENUM_TYPE {SC_ENUM_PROCESS_INFO = 0, __SeeGhIssue10292} pub use self::SC_ENUM_TYPE::{SC_ENUM_PROCESS_INFO}; /* winsvc.h:694:14, winsvc.h:694:14, winsvc.h:694:14 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_STATUS { dwServiceType: ::minwindef::DWORD, dwCurrentState: ::minwindef::DWORD, dwControlsAccepted: ::minwindef::DWORD, dwWin32ExitCode: ::minwindef::DWORD, dwServiceSpecificExitCode: ::minwindef::DWORD, dwCheckPoint: ::minwindef::DWORD, dwWaitHint: ::minwindef::DWORD } /* winsvc.h:703:16, winsvc.h:703:16, winsvc.h:703:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_STATUS = *mut ::winsvc::SERVICE_STATUS; /* winsvc.h:711:20, winsvc.h:711:20, winsvc.h:711:20 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_STATUS_PROCESS { dwServiceType: ::minwindef::DWORD, dwCurrentState: ::minwindef::DWORD, dwControlsAccepted: ::minwindef::DWORD, dwWin32ExitCode: ::minwindef::DWORD, dwServiceSpecificExitCode: ::minwindef::DWORD, dwCheckPoint: ::minwindef::DWORD, dwWaitHint: ::minwindef::DWORD, dwProcessId: ::minwindef::DWORD, dwServiceFlags: ::minwindef::DWORD } /* winsvc.h:713:16, winsvc.h:713:16, winsvc.h:713:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_STATUS_PROCESS = *mut ::winsvc::SERVICE_STATUS_PROCESS; /* winsvc.h:723:28, winsvc.h:723:28, winsvc.h:723:28 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct ENUM_SERVICE_STATUSA { lpServiceName: ::winnt::LPSTR, lpDisplayName: ::winnt::LPSTR, ServiceStatus: ::winsvc::SERVICE_STATUS } /* winsvc.h:730:16, winsvc.h:730:16, winsvc.h:730:16 */
#[cfg(feature="winapi_desktop")] pub type LPENUM_SERVICE_STATUSA = *mut ::winsvc::ENUM_SERVICE_STATUSA; /* winsvc.h:734:26, winsvc.h:734:26, winsvc.h:734:26 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct ENUM_SERVICE_STATUSW { lpServiceName: ::winnt::LPWSTR, lpDisplayName: ::winnt::LPWSTR, ServiceStatus: ::winsvc::SERVICE_STATUS } /* winsvc.h:735:16, winsvc.h:735:16, winsvc.h:735:16 */
#[cfg(feature="winapi_desktop")] pub type LPENUM_SERVICE_STATUSW = *mut ::winsvc::ENUM_SERVICE_STATUSW; /* winsvc.h:739:26, winsvc.h:739:26, winsvc.h:739:26 */
#[cfg(feature="winapi_desktop")] pub type ENUM_SERVICE_STATUS = ::winsvc::ENUM_SERVICE_STATUSW; /* winsvc.h:741:30, winsvc.h:741:30, winsvc.h:741:30 */
#[cfg(feature="winapi_desktop")] pub type LPENUM_SERVICE_STATUS = ::winsvc::LPENUM_SERVICE_STATUSW; /* winsvc.h:742:32, winsvc.h:742:32, winsvc.h:742:32 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct ENUM_SERVICE_STATUS_PROCESSA { lpServiceName: ::winnt::LPSTR, lpDisplayName: ::winnt::LPSTR, ServiceStatusProcess: ::winsvc::SERVICE_STATUS_PROCESS } /* winsvc.h:748:16, winsvc.h:748:16, winsvc.h:748:16 */
#[cfg(feature="winapi_desktop")] pub type LPENUM_SERVICE_STATUS_PROCESSA = *mut ::winsvc::ENUM_SERVICE_STATUS_PROCESSA; /* winsvc.h:752:34, winsvc.h:752:34, winsvc.h:752:34 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct ENUM_SERVICE_STATUS_PROCESSW { lpServiceName: ::winnt::LPWSTR, lpDisplayName: ::winnt::LPWSTR, ServiceStatusProcess: ::winsvc::SERVICE_STATUS_PROCESS } /* winsvc.h:753:16, winsvc.h:753:16, winsvc.h:753:16 */
#[cfg(feature="winapi_desktop")] pub type LPENUM_SERVICE_STATUS_PROCESSW = *mut ::winsvc::ENUM_SERVICE_STATUS_PROCESSW; /* winsvc.h:757:34, winsvc.h:757:34, winsvc.h:757:34 */
#[cfg(feature="winapi_desktop")] pub type ENUM_SERVICE_STATUS_PROCESS = ::winsvc::ENUM_SERVICE_STATUS_PROCESSW; /* winsvc.h:759:38, winsvc.h:759:38, winsvc.h:759:38 */
#[cfg(feature="winapi_desktop")] pub type LPENUM_SERVICE_STATUS_PROCESS = ::winsvc::LPENUM_SERVICE_STATUS_PROCESSW; /* winsvc.h:760:40, winsvc.h:760:40, winsvc.h:760:40 */
#[cfg(feature="winapi_desktop")] pub type SC_LOCK = ::minwindef::LPVOID; /* winsvc.h:770:17, winsvc.h:770:17, winsvc.h:770:17 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct QUERY_SERVICE_LOCK_STATUSA { fIsLocked: ::minwindef::DWORD, lpLockOwner: ::winnt::LPSTR, dwLockDuration: ::minwindef::DWORD } /* winsvc.h:772:16, winsvc.h:772:16, winsvc.h:772:16 */
#[cfg(feature="winapi_desktop")] pub type LPQUERY_SERVICE_LOCK_STATUSA = *mut ::winsvc::QUERY_SERVICE_LOCK_STATUSA; /* winsvc.h:776:32, winsvc.h:776:32, winsvc.h:776:32 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct QUERY_SERVICE_LOCK_STATUSW { fIsLocked: ::minwindef::DWORD, lpLockOwner: ::winnt::LPWSTR, dwLockDuration: ::minwindef::DWORD } /* winsvc.h:777:16, winsvc.h:777:16, winsvc.h:777:16 */
#[cfg(feature="winapi_desktop")] pub type LPQUERY_SERVICE_LOCK_STATUSW = *mut ::winsvc::QUERY_SERVICE_LOCK_STATUSW; /* winsvc.h:781:32, winsvc.h:781:32, winsvc.h:781:32 */
#[cfg(feature="winapi_desktop")] pub type QUERY_SERVICE_LOCK_STATUS = ::winsvc::QUERY_SERVICE_LOCK_STATUSW; /* winsvc.h:783:36, winsvc.h:783:36, winsvc.h:783:36 */
#[cfg(feature="winapi_desktop")] pub type LPQUERY_SERVICE_LOCK_STATUS = ::winsvc::LPQUERY_SERVICE_LOCK_STATUSW; /* winsvc.h:784:38, winsvc.h:784:38, winsvc.h:784:38 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct QUERY_SERVICE_CONFIGA { dwServiceType: ::minwindef::DWORD, dwStartType: ::minwindef::DWORD, dwErrorControl: ::minwindef::DWORD, lpBinaryPathName: ::winnt::LPSTR, lpLoadOrderGroup: ::winnt::LPSTR, dwTagId: ::minwindef::DWORD, lpDependencies: ::winnt::LPSTR, lpServiceStartName: ::winnt::LPSTR, lpDisplayName: ::winnt::LPSTR } /* winsvc.h:796:16, winsvc.h:796:16, winsvc.h:796:16 */
#[cfg(feature="winapi_desktop")] pub type LPQUERY_SERVICE_CONFIGA = *mut ::winsvc::QUERY_SERVICE_CONFIGA; /* winsvc.h:806:27, winsvc.h:806:27, winsvc.h:806:27 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct QUERY_SERVICE_CONFIGW { dwServiceType: ::minwindef::DWORD, dwStartType: ::minwindef::DWORD, dwErrorControl: ::minwindef::DWORD, lpBinaryPathName: ::winnt::LPWSTR, lpLoadOrderGroup: ::winnt::LPWSTR, dwTagId: ::minwindef::DWORD, lpDependencies: ::winnt::LPWSTR, lpServiceStartName: ::winnt::LPWSTR, lpDisplayName: ::winnt::LPWSTR } /* winsvc.h:807:16, winsvc.h:807:16, winsvc.h:807:16 */
#[cfg(feature="winapi_desktop")] pub type LPQUERY_SERVICE_CONFIGW = *mut ::winsvc::QUERY_SERVICE_CONFIGW; /* winsvc.h:817:27, winsvc.h:817:27, winsvc.h:817:27 */
#[cfg(feature="winapi_desktop")] pub type QUERY_SERVICE_CONFIG = ::winsvc::QUERY_SERVICE_CONFIGW; /* winsvc.h:819:31, winsvc.h:819:31, winsvc.h:819:31 */
#[cfg(feature="winapi_desktop")] pub type LPQUERY_SERVICE_CONFIG = ::winsvc::LPQUERY_SERVICE_CONFIGW; /* winsvc.h:820:33, winsvc.h:820:33, winsvc.h:820:33 */
#[cfg(feature="winapi_desktop")] pub type SERVICE_MAIN_FUNCTIONW = Option<extern "system" fn(::libc::c_ulong, *mut *mut ::libc::c_ushort)>; /* winsvc.h:832:21, winsvc.h:832:21, winsvc.h:832:21 */
#[cfg(feature="winapi_desktop")] pub type SERVICE_MAIN_FUNCTIONA = Option<extern "system" fn(::libc::c_ulong, *mut *mut ::libc::c_ushort)>; /* winsvc.h:837:21, winsvc.h:837:21, winsvc.h:837:21 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_MAIN_FUNCTIONW = Option<extern "system" fn(::libc::c_ulong, *mut *mut ::libc::c_ushort)>; /* winsvc.h:848:23, winsvc.h:848:23, winsvc.h:848:23 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_MAIN_FUNCTIONA = Option<extern "system" fn(::libc::c_ulong, *mut *mut ::libc::c_schar)>; /* winsvc.h:853:23, winsvc.h:853:23, winsvc.h:853:23 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_TABLE_ENTRYA { lpServiceName: ::winnt::LPSTR, lpServiceProc: ::winsvc::LPSERVICE_MAIN_FUNCTIONA } /* winsvc.h:869:16, winsvc.h:869:16, winsvc.h:869:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_TABLE_ENTRYA = *mut ::winsvc::SERVICE_TABLE_ENTRYA; /* winsvc.h:872:25, winsvc.h:872:25, winsvc.h:872:25 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_TABLE_ENTRYW { lpServiceName: ::winnt::LPWSTR, lpServiceProc: ::winsvc::LPSERVICE_MAIN_FUNCTIONW } /* winsvc.h:873:16, winsvc.h:873:16, winsvc.h:873:16 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_TABLE_ENTRYW = *mut ::winsvc::SERVICE_TABLE_ENTRYW; /* winsvc.h:876:25, winsvc.h:876:25, winsvc.h:876:25 */
#[cfg(feature="winapi_desktop")] pub type SERVICE_TABLE_ENTRY = ::winsvc::SERVICE_TABLE_ENTRYW; /* winsvc.h:878:30, winsvc.h:878:30, winsvc.h:878:30 */
#[cfg(feature="winapi_desktop")] pub type LPSERVICE_TABLE_ENTRY = ::winsvc::LPSERVICE_TABLE_ENTRYW; /* winsvc.h:879:32, winsvc.h:879:32, winsvc.h:879:32 */
#[cfg(feature="winapi_desktop")] pub type HANDLER_FUNCTION = Option<extern "system" fn(::libc::c_ulong)>; /* winsvc.h:889:21, winsvc.h:889:21, winsvc.h:889:21 */
#[cfg(feature="winapi_desktop")] pub type HANDLER_FUNCTION_EX = Option<extern "system" fn(::libc::c_ulong, ::libc::c_ulong, *mut ::libc::c_void, *mut ::libc::c_void) -> ::libc::c_ulong>; /* winsvc.h:893:22, winsvc.h:893:22, winsvc.h:893:22 */
#[cfg(feature="winapi_desktop")] pub type LPHANDLER_FUNCTION = Option<extern "system" fn(::libc::c_ulong)>; /* winsvc.h:900:23, winsvc.h:900:23, winsvc.h:900:23 */
#[cfg(feature="winapi_desktop")] pub type LPHANDLER_FUNCTION_EX = Option<extern "system" fn(::libc::c_ulong, ::libc::c_ulong, *mut ::libc::c_void, *mut ::libc::c_void) -> ::libc::c_ulong>; /* winsvc.h:904:24, winsvc.h:904:24, winsvc.h:904:24 */
#[cfg(feature="winapi_desktop")] pub type PFN_SC_NOTIFY_CALLBACK = Option<extern "system" fn(*mut ::libc::c_void)>; /* winsvc.h:916:14, winsvc.h:916:14, winsvc.h:916:14 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_NOTIFY_1 { dwVersion: ::minwindef::DWORD, pfnNotifyCallback: ::winsvc::PFN_SC_NOTIFY_CALLBACK, pContext: ::winnt::PVOID, dwNotificationStatus: ::minwindef::DWORD, ServiceStatus: ::winsvc::SERVICE_STATUS_PROCESS } /* winsvc.h:923:16, winsvc.h:923:16, winsvc.h:923:16 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_NOTIFY_1 = *mut ::winsvc::SERVICE_NOTIFY_1; /* winsvc.h:929:22, winsvc.h:929:22, winsvc.h:929:22 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_NOTIFY_2A { dwVersion: ::minwindef::DWORD, pfnNotifyCallback: ::winsvc::PFN_SC_NOTIFY_CALLBACK, pContext: ::winnt::PVOID, dwNotificationStatus: ::minwindef::DWORD, ServiceStatus: ::winsvc::SERVICE_STATUS_PROCESS, dwNotificationTriggered: ::minwindef::DWORD, pszServiceNames: ::winnt::LPSTR } /* winsvc.h:931:16, winsvc.h:931:16, winsvc.h:931:16 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_NOTIFY_2A = *mut ::winsvc::SERVICE_NOTIFY_2A; /* winsvc.h:939:23, winsvc.h:939:23, winsvc.h:939:23 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_NOTIFY_2W { dwVersion: ::minwindef::DWORD, pfnNotifyCallback: ::winsvc::PFN_SC_NOTIFY_CALLBACK, pContext: ::winnt::PVOID, dwNotificationStatus: ::minwindef::DWORD, ServiceStatus: ::winsvc::SERVICE_STATUS_PROCESS, dwNotificationTriggered: ::minwindef::DWORD, pszServiceNames: ::winnt::LPWSTR } /* winsvc.h:940:16, winsvc.h:940:16, winsvc.h:940:16 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_NOTIFY_2W = *mut ::winsvc::SERVICE_NOTIFY_2W; /* winsvc.h:948:23, winsvc.h:948:23, winsvc.h:948:23 */
#[cfg(feature="winapi_desktop")] pub type SERVICE_NOTIFY_2 = ::winsvc::SERVICE_NOTIFY_2W; /* winsvc.h:950:27, winsvc.h:950:27, winsvc.h:950:27 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_NOTIFY_2 = ::winsvc::PSERVICE_NOTIFY_2W; /* winsvc.h:951:28, winsvc.h:951:28, winsvc.h:951:28 */
#[cfg(feature="winapi_desktop")] pub type SERVICE_NOTIFYA = ::winsvc::SERVICE_NOTIFY_2A; /* winsvc.h:957:27, winsvc.h:957:27, winsvc.h:957:27 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_NOTIFYA = *mut ::winsvc::SERVICE_NOTIFY_2A; /* winsvc.h:957:45, winsvc.h:957:45, winsvc.h:957:45 */
#[cfg(feature="winapi_desktop")] pub type SERVICE_NOTIFYW = ::winsvc::SERVICE_NOTIFY_2W; /* winsvc.h:958:27, winsvc.h:958:27, winsvc.h:958:27 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_NOTIFYW = *mut ::winsvc::SERVICE_NOTIFY_2W; /* winsvc.h:958:45, winsvc.h:958:45, winsvc.h:958:45 */
#[cfg(feature="winapi_desktop")] pub type SERVICE_NOTIFY = ::winsvc::SERVICE_NOTIFYW; /* winsvc.h:960:25, winsvc.h:960:25, winsvc.h:960:25 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_NOTIFY = ::winsvc::PSERVICE_NOTIFYW; /* winsvc.h:961:26, winsvc.h:961:26, winsvc.h:961:26 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSA { dwReason: ::minwindef::DWORD, pszComment: ::winnt::LPSTR, ServiceStatus: ::winsvc::SERVICE_STATUS_PROCESS } /* winsvc.h:970:16, winsvc.h:970:16, winsvc.h:970:16 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_CONTROL_STATUS_REASON_PARAMSA = *mut ::winsvc::SERVICE_CONTROL_STATUS_REASON_PARAMSA; /* winsvc.h:974:43, winsvc.h:974:43, winsvc.h:974:43 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSW { dwReason: ::minwindef::DWORD, pszComment: ::winnt::LPWSTR, ServiceStatus: ::winsvc::SERVICE_STATUS_PROCESS } /* winsvc.h:978:16, winsvc.h:978:16, winsvc.h:978:16 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_CONTROL_STATUS_REASON_PARAMSW = *mut ::winsvc::SERVICE_CONTROL_STATUS_REASON_PARAMSW; /* winsvc.h:982:43, winsvc.h:982:43, winsvc.h:982:43 */
#[cfg(feature="winapi_desktop")] pub type SERVICE_CONTROL_STATUS_REASON_PARAMS = ::winsvc::SERVICE_CONTROL_STATUS_REASON_PARAMSW; /* winsvc.h:984:47, winsvc.h:984:47, winsvc.h:984:47 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_CONTROL_STATUS_REASON_PARAMS = ::winsvc::PSERVICE_CONTROL_STATUS_REASON_PARAMSW; /* winsvc.h:985:48, winsvc.h:985:48, winsvc.h:985:48 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SERVICE_START_REASON { dwReason: ::minwindef::DWORD } /* winsvc.h:994:16, winsvc.h:994:16, winsvc.h:994:16 */
#[cfg(feature="winapi_desktop")] pub type PSERVICE_START_REASON = *mut ::winsvc::SERVICE_START_REASON; /* winsvc.h:996:26, winsvc.h:996:26, winsvc.h:996:26 */
#[cfg(feature="winapi_desktop")] pub const SERVICES_ACTIVE_DATABASEW: &'static str = "ServicesActive"; /* String("ServicesActive", Wide) */ /* winsvc.h:50:9, winsvc.h:50:9, winsvc.h:50:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICES_FAILED_DATABASEW: &'static str = "ServicesFailed"; /* String("ServicesFailed", Wide) */ /* winsvc.h:51:9, winsvc.h:51:9, winsvc.h:51:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICES_ACTIVE_DATABASEA: &'static str = "ServicesActive"; /* String("ServicesActive", Narrow) */ /* winsvc.h:53:9, winsvc.h:53:9, winsvc.h:53:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICES_FAILED_DATABASEA: &'static str = "ServicesFailed"; /* String("ServicesFailed", Narrow) */ /* winsvc.h:54:9, winsvc.h:54:9, winsvc.h:54:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winsvc::SERVICES_ACTIVE_DATABASEW as SERVICES_ACTIVE_DATABASE; /* winsvc.h:65:9, winsvc.h:65:9, winsvc.h:65:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winsvc::SERVICES_FAILED_DATABASEW as SERVICES_FAILED_DATABASE; /* winsvc.h:66:9, winsvc.h:66:9, winsvc.h:66:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_NO_CHANGE: i32 = 0xffffffffi32; /* Integer(4294967295, Yes, Unknown) */ /* winsvc.h:83:9, winsvc.h:83:9, winsvc.h:83:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_ACTIVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:88:9, winsvc.h:88:9, winsvc.h:88:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_INACTIVE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winsvc.h:89:9, winsvc.h:89:9, winsvc.h:89:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_STOP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:96:9, winsvc.h:96:9, winsvc.h:96:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_PAUSE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winsvc.h:97:9, winsvc.h:97:9, winsvc.h:97:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_CONTINUE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winsvc.h:98:9, winsvc.h:98:9, winsvc.h:98:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_INTERROGATE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winsvc.h:99:9, winsvc.h:99:9, winsvc.h:99:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_SHUTDOWN: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winsvc.h:100:9, winsvc.h:100:9, winsvc.h:100:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_PARAMCHANGE: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winsvc.h:101:9, winsvc.h:101:9, winsvc.h:101:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_NETBINDADD: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winsvc.h:102:9, winsvc.h:102:9, winsvc.h:102:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_NETBINDREMOVE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winsvc.h:103:9, winsvc.h:103:9, winsvc.h:103:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_NETBINDENABLE: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winsvc.h:104:9, winsvc.h:104:9, winsvc.h:104:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_NETBINDDISABLE: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winsvc.h:105:9, winsvc.h:105:9, winsvc.h:105:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_DEVICEEVENT: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winsvc.h:106:9, winsvc.h:106:9, winsvc.h:106:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_HARDWAREPROFILECHANGE: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winsvc.h:107:9, winsvc.h:107:9, winsvc.h:107:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_POWEREVENT: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winsvc.h:108:9, winsvc.h:108:9, winsvc.h:108:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_SESSIONCHANGE: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winsvc.h:109:9, winsvc.h:109:9, winsvc.h:109:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_PRESHUTDOWN: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winsvc.h:110:9, winsvc.h:110:9, winsvc.h:110:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_TIMECHANGE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winsvc.h:111:9, winsvc.h:111:9, winsvc.h:111:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_TRIGGEREVENT: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winsvc.h:112:9, winsvc.h:112:9, winsvc.h:112:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOPPED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:117:9, winsvc.h:117:9, winsvc.h:117:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_START_PENDING: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winsvc.h:118:9, winsvc.h:118:9, winsvc.h:118:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_PENDING: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winsvc.h:119:9, winsvc.h:119:9, winsvc.h:119:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_RUNNING: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winsvc.h:120:9, winsvc.h:120:9, winsvc.h:120:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTINUE_PENDING: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winsvc.h:121:9, winsvc.h:121:9, winsvc.h:121:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_PAUSE_PENDING: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winsvc.h:122:9, winsvc.h:122:9, winsvc.h:122:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_PAUSED: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winsvc.h:123:9, winsvc.h:123:9, winsvc.h:123:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_ACCEPT_STOP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:128:9, winsvc.h:128:9, winsvc.h:128:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_ACCEPT_PAUSE_CONTINUE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winsvc.h:129:9, winsvc.h:129:9, winsvc.h:129:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_ACCEPT_SHUTDOWN: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winsvc.h:130:9, winsvc.h:130:9, winsvc.h:130:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_ACCEPT_PARAMCHANGE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winsvc.h:131:9, winsvc.h:131:9, winsvc.h:131:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_ACCEPT_NETBINDCHANGE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winsvc.h:132:9, winsvc.h:132:9, winsvc.h:132:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_ACCEPT_HARDWAREPROFILECHANGE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winsvc.h:133:9, winsvc.h:133:9, winsvc.h:133:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_ACCEPT_POWEREVENT: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winsvc.h:134:9, winsvc.h:134:9, winsvc.h:134:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_ACCEPT_SESSIONCHANGE: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winsvc.h:135:9, winsvc.h:135:9, winsvc.h:135:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_ACCEPT_PRESHUTDOWN: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winsvc.h:136:9, winsvc.h:136:9, winsvc.h:136:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_ACCEPT_TIMECHANGE: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winsvc.h:137:9, winsvc.h:137:9, winsvc.h:137:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_ACCEPT_TRIGGEREVENT: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winsvc.h:138:9, winsvc.h:138:9, winsvc.h:138:9 */
#[cfg(feature="winapi_desktop")] pub const SC_MANAGER_CONNECT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:143:9, winsvc.h:143:9, winsvc.h:143:9 */
#[cfg(feature="winapi_desktop")] pub const SC_MANAGER_CREATE_SERVICE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winsvc.h:144:9, winsvc.h:144:9, winsvc.h:144:9 */
#[cfg(feature="winapi_desktop")] pub const SC_MANAGER_ENUMERATE_SERVICE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winsvc.h:145:9, winsvc.h:145:9, winsvc.h:145:9 */
#[cfg(feature="winapi_desktop")] pub const SC_MANAGER_LOCK: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winsvc.h:146:9, winsvc.h:146:9, winsvc.h:146:9 */
#[cfg(feature="winapi_desktop")] pub const SC_MANAGER_QUERY_LOCK_STATUS: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winsvc.h:147:9, winsvc.h:147:9, winsvc.h:147:9 */
#[cfg(feature="winapi_desktop")] pub const SC_MANAGER_MODIFY_BOOT_CONFIG: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winsvc.h:148:9, winsvc.h:148:9, winsvc.h:148:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_QUERY_CONFIG: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:163:9, winsvc.h:163:9, winsvc.h:163:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CHANGE_CONFIG: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winsvc.h:164:9, winsvc.h:164:9, winsvc.h:164:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_QUERY_STATUS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winsvc.h:165:9, winsvc.h:165:9, winsvc.h:165:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_ENUMERATE_DEPENDENTS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winsvc.h:166:9, winsvc.h:166:9, winsvc.h:166:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_START: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winsvc.h:167:9, winsvc.h:167:9, winsvc.h:167:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winsvc.h:168:9, winsvc.h:168:9, winsvc.h:168:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_PAUSE_CONTINUE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winsvc.h:169:9, winsvc.h:169:9, winsvc.h:169:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_INTERROGATE: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winsvc.h:170:9, winsvc.h:170:9, winsvc.h:170:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_USER_DEFINED_CONTROL: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winsvc.h:171:9, winsvc.h:171:9, winsvc.h:171:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_RUNS_IN_SYSTEM_PROCESS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:187:9, winsvc.h:187:9, winsvc.h:187:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONFIG_DESCRIPTION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:192:9, winsvc.h:192:9, winsvc.h:192:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONFIG_FAILURE_ACTIONS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winsvc.h:193:9, winsvc.h:193:9, winsvc.h:193:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONFIG_DELAYED_AUTO_START_INFO: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winsvc.h:194:9, winsvc.h:194:9, winsvc.h:194:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONFIG_FAILURE_ACTIONS_FLAG: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winsvc.h:195:9, winsvc.h:195:9, winsvc.h:195:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONFIG_SERVICE_SID_INFO: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winsvc.h:196:9, winsvc.h:196:9, winsvc.h:196:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winsvc.h:197:9, winsvc.h:197:9, winsvc.h:197:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONFIG_PRESHUTDOWN_INFO: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winsvc.h:198:9, winsvc.h:198:9, winsvc.h:198:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONFIG_TRIGGER_INFO: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winsvc.h:199:9, winsvc.h:199:9, winsvc.h:199:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONFIG_PREFERRED_NODE: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winsvc.h:200:9, winsvc.h:200:9, winsvc.h:200:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONFIG_LAUNCH_PROTECTED: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winsvc.h:203:9, winsvc.h:203:9, winsvc.h:203:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_NOTIFY_STATUS_CHANGE_1: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:208:9, winsvc.h:208:9, winsvc.h:208:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_NOTIFY_STATUS_CHANGE_2: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winsvc.h:209:9, winsvc.h:209:9, winsvc.h:209:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winsvc::SERVICE_NOTIFY_STATUS_CHANGE_2 as SERVICE_NOTIFY_STATUS_CHANGE; /* winsvc.h:211:9, winsvc.h:211:9, winsvc.h:211:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_NOTIFY_STOPPED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:216:9, winsvc.h:216:9, winsvc.h:216:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_NOTIFY_START_PENDING: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winsvc.h:217:9, winsvc.h:217:9, winsvc.h:217:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_NOTIFY_STOP_PENDING: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winsvc.h:218:9, winsvc.h:218:9, winsvc.h:218:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_NOTIFY_RUNNING: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winsvc.h:219:9, winsvc.h:219:9, winsvc.h:219:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_NOTIFY_CONTINUE_PENDING: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winsvc.h:220:9, winsvc.h:220:9, winsvc.h:220:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_NOTIFY_PAUSE_PENDING: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winsvc.h:221:9, winsvc.h:221:9, winsvc.h:221:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_NOTIFY_PAUSED: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winsvc.h:222:9, winsvc.h:222:9, winsvc.h:222:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_NOTIFY_CREATED: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winsvc.h:223:9, winsvc.h:223:9, winsvc.h:223:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_NOTIFY_DELETED: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winsvc.h:224:9, winsvc.h:224:9, winsvc.h:224:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_NOTIFY_DELETE_PENDING: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winsvc.h:225:9, winsvc.h:225:9, winsvc.h:225:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_FLAG_MIN: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winsvc.h:235:9, winsvc.h:235:9, winsvc.h:235:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_FLAG_UNPLANNED: i32 = 0x10000000i32; /* Integer(268435456, Yes, Unknown) */ /* winsvc.h:236:9, winsvc.h:236:9, winsvc.h:236:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_FLAG_CUSTOM: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* winsvc.h:237:9, winsvc.h:237:9, winsvc.h:237:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_FLAG_PLANNED: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* winsvc.h:238:9, winsvc.h:238:9, winsvc.h:238:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_FLAG_MAX: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winsvc.h:239:9, winsvc.h:239:9, winsvc.h:239:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MAJOR_MIN: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winsvc.h:245:9, winsvc.h:245:9, winsvc.h:245:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MAJOR_OTHER: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winsvc.h:246:9, winsvc.h:246:9, winsvc.h:246:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MAJOR_HARDWARE: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winsvc.h:247:9, winsvc.h:247:9, winsvc.h:247:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MAJOR_OPERATINGSYSTEM: i32 = 0x30000i32; /* Integer(196608, Yes, Unknown) */ /* winsvc.h:248:9, winsvc.h:248:9, winsvc.h:248:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MAJOR_SOFTWARE: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* winsvc.h:249:9, winsvc.h:249:9, winsvc.h:249:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MAJOR_APPLICATION: i32 = 0x50000i32; /* Integer(327680, Yes, Unknown) */ /* winsvc.h:250:9, winsvc.h:250:9, winsvc.h:250:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MAJOR_NONE: i32 = 0x60000i32; /* Integer(393216, Yes, Unknown) */ /* winsvc.h:251:9, winsvc.h:251:9, winsvc.h:251:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MAJOR_MAX: i32 = 0x70000i32; /* Integer(458752, Yes, Unknown) */ /* winsvc.h:252:9, winsvc.h:252:9, winsvc.h:252:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MAJOR_MIN_CUSTOM: i32 = 0x400000i32; /* Integer(4194304, Yes, Unknown) */ /* winsvc.h:253:9, winsvc.h:253:9, winsvc.h:253:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MAJOR_MAX_CUSTOM: i32 = 0xff0000i32; /* Integer(16711680, Yes, Unknown) */ /* winsvc.h:254:9, winsvc.h:254:9, winsvc.h:254:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_MIN: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winsvc.h:260:9, winsvc.h:260:9, winsvc.h:260:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_OTHER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:261:9, winsvc.h:261:9, winsvc.h:261:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_MAINTENANCE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winsvc.h:262:9, winsvc.h:262:9, winsvc.h:262:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_INSTALLATION: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winsvc.h:263:9, winsvc.h:263:9, winsvc.h:263:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_UPGRADE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winsvc.h:264:9, winsvc.h:264:9, winsvc.h:264:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_RECONFIG: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winsvc.h:265:9, winsvc.h:265:9, winsvc.h:265:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_HUNG: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winsvc.h:266:9, winsvc.h:266:9, winsvc.h:266:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_UNSTABLE: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winsvc.h:267:9, winsvc.h:267:9, winsvc.h:267:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_DISK: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winsvc.h:268:9, winsvc.h:268:9, winsvc.h:268:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_NETWORKCARD: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winsvc.h:269:9, winsvc.h:269:9, winsvc.h:269:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_ENVIRONMENT: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winsvc.h:270:9, winsvc.h:270:9, winsvc.h:270:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_HARDWARE_DRIVER: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winsvc.h:271:9, winsvc.h:271:9, winsvc.h:271:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_OTHERDRIVER: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winsvc.h:272:9, winsvc.h:272:9, winsvc.h:272:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winsvc.h:273:9, winsvc.h:273:9, winsvc.h:273:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winsvc.h:274:9, winsvc.h:274:9, winsvc.h:274:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winsvc.h:275:9, winsvc.h:275:9, winsvc.h:275:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_SECURITY: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winsvc.h:276:9, winsvc.h:276:9, winsvc.h:276:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_NETWORK_CONNECTIVITY: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winsvc.h:277:9, winsvc.h:277:9, winsvc.h:277:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_WMI: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winsvc.h:278:9, winsvc.h:278:9, winsvc.h:278:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK_UNINSTALL: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winsvc.h:279:9, winsvc.h:279:9, winsvc.h:279:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE_UNINSTALL: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winsvc.h:280:9, winsvc.h:280:9, winsvc.h:280:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX_UNINSTALL: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winsvc.h:281:9, winsvc.h:281:9, winsvc.h:281:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_MMC: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winsvc.h:282:9, winsvc.h:282:9, winsvc.h:282:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_NONE: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winsvc.h:283:9, winsvc.h:283:9, winsvc.h:283:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_MAX: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winsvc.h:284:9, winsvc.h:284:9, winsvc.h:284:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_MIN_CUSTOM: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winsvc.h:285:9, winsvc.h:285:9, winsvc.h:285:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_STOP_REASON_MINOR_MAX_CUSTOM: i32 = 0xffffi32; /* Integer(65535, Yes, Unknown) */ /* winsvc.h:286:9, winsvc.h:286:9, winsvc.h:286:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_CONTROL_STATUS_REASON_INFO: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:291:9, winsvc.h:291:9, winsvc.h:291:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_SID_TYPE_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winsvc.h:296:9, winsvc.h:296:9, winsvc.h:296:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_SID_TYPE_UNRESTRICTED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:297:9, winsvc.h:297:9, winsvc.h:297:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_TYPE_DEVICE_INTERFACE_ARRIVAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:303:9, winsvc.h:303:9, winsvc.h:303:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_TYPE_IP_ADDRESS_AVAILABILITY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winsvc.h:304:9, winsvc.h:304:9, winsvc.h:304:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_TYPE_DOMAIN_JOIN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winsvc.h:305:9, winsvc.h:305:9, winsvc.h:305:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_TYPE_FIREWALL_PORT_EVENT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winsvc.h:306:9, winsvc.h:306:9, winsvc.h:306:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_TYPE_GROUP_POLICY: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winsvc.h:307:9, winsvc.h:307:9, winsvc.h:307:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_TYPE_NETWORK_ENDPOINT: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winsvc.h:308:9, winsvc.h:308:9, winsvc.h:308:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_TYPE_CUSTOM_SYSTEM_STATE_CHANGE: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winsvc.h:309:9, winsvc.h:309:9, winsvc.h:309:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_TYPE_CUSTOM: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winsvc.h:310:9, winsvc.h:310:9, winsvc.h:310:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_DATA_TYPE_BINARY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:315:9, winsvc.h:315:9, winsvc.h:315:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_DATA_TYPE_STRING: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winsvc.h:316:9, winsvc.h:316:9, winsvc.h:316:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_DATA_TYPE_LEVEL: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winsvc.h:317:9, winsvc.h:317:9, winsvc.h:317:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ANY: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winsvc.h:318:9, winsvc.h:318:9, winsvc.h:318:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ALL: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winsvc.h:319:9, winsvc.h:319:9, winsvc.h:319:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_START_REASON_DEMAND: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:324:9, winsvc.h:324:9, winsvc.h:324:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_START_REASON_AUTO: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winsvc.h:325:9, winsvc.h:325:9, winsvc.h:325:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_START_REASON_TRIGGER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winsvc.h:326:9, winsvc.h:326:9, winsvc.h:326:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_START_REASON_RESTART_ON_FAILURE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winsvc.h:327:9, winsvc.h:327:9, winsvc.h:327:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_START_REASON_DELAYEDAUTO: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winsvc.h:328:9, winsvc.h:328:9, winsvc.h:328:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_DYNAMIC_INFORMATION_LEVEL_START_REASON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:333:9, winsvc.h:333:9, winsvc.h:333:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_LAUNCH_PROTECTED_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winsvc.h:338:9, winsvc.h:338:9, winsvc.h:338:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_LAUNCH_PROTECTED_WINDOWS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:339:9, winsvc.h:339:9, winsvc.h:339:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_LAUNCH_PROTECTED_WINDOWS_LIGHT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winsvc.h:340:9, winsvc.h:340:9, winsvc.h:340:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_LAUNCH_PROTECTED_ANTIMALWARE_LIGHT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winsvc.h:341:9, winsvc.h:341:9, winsvc.h:341:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_ACTION_SERVICE_START: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winsvc.h:474:9, winsvc.h:474:9, winsvc.h:474:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_ACTION_SERVICE_STOP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winsvc.h:475:9, winsvc.h:475:9, winsvc.h:475:9 */
#[cfg(feature="winapi_desktop")] pub const SERVICE_TRIGGER_STARTED_ARGUMENT: &'static str = "TriggerStarted"; /* String("TriggerStarted", Wide) */ /* winsvc.h:481:9, winsvc.h:481:9, winsvc.h:481:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winsvc::SERVICE_MAIN_FUNCTIONW as SERVICE_MAIN_FUNCTION; /* winsvc.h:843:9, winsvc.h:843:9, winsvc.h:843:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winsvc::LPSERVICE_MAIN_FUNCTIONW as LPSERVICE_MAIN_FUNCTION; /* winsvc.h:859:9, winsvc.h:859:9, winsvc.h:859:9 */
