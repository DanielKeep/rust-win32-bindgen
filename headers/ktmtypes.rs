pub type UOW = ::guiddef::GUID; /* ktmtypes.h:27:14, ktmtypes.h:27:14, ktmtypes.h:27:14 */
pub type PUOW = *mut ::guiddef::GUID; /* ktmtypes.h:27:20, ktmtypes.h:27:20, ktmtypes.h:27:20 */
pub type CRM_PROTOCOL_ID = ::guiddef::GUID; /* ktmtypes.h:28:14, ktmtypes.h:28:14, ktmtypes.h:28:14 */
pub type PCRM_PROTOCOL_ID = *mut ::guiddef::GUID; /* ktmtypes.h:28:32, ktmtypes.h:28:32, ktmtypes.h:28:32 */
pub type NOTIFICATION_MASK = ::minwindef::ULONG; /* ktmtypes.h:78:15, ktmtypes.h:78:15, ktmtypes.h:78:15 */
#[repr(C)] pub struct TRANSACTION_NOTIFICATION { TransactionKey: ::winnt::PVOID, TransactionNotification: ::minwindef::ULONG, TmVirtualClock: ::winnt::LARGE_INTEGER, ArgumentLength: ::minwindef::ULONG } /* ktmtypes.h:133:16, ktmtypes.h:133:16, ktmtypes.h:133:16 */
pub type PTRANSACTION_NOTIFICATION = *mut ::ktmtypes::TRANSACTION_NOTIFICATION; /* ktmtypes.h:138:30, ktmtypes.h:138:30, ktmtypes.h:138:30 */
#[repr(C)] pub struct TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT { EnlistmentId: ::guiddef::GUID, UOW: ::ktmtypes::UOW } /* ktmtypes.h:140:16, ktmtypes.h:140:16, ktmtypes.h:140:16 */
pub type PTRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT = *mut ::ktmtypes::TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT; /* ktmtypes.h:143:48, ktmtypes.h:143:48, ktmtypes.h:143:48 */
#[repr(C)] pub struct TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT { TmIdentity: ::guiddef::GUID, Flags: ::minwindef::ULONG } /* ktmtypes.h:147:16, ktmtypes.h:147:16, ktmtypes.h:147:16 */
pub type PTRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT = *mut ::ktmtypes::TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT; /* ktmtypes.h:150:49, ktmtypes.h:150:49, ktmtypes.h:150:49 */
pub type SAVEPOINT_ID = ::minwindef::ULONG; /* ktmtypes.h:152:15, ktmtypes.h:152:15, ktmtypes.h:152:15 */
pub type PSAVEPOINT_ID = *mut ::libc::c_ulong; /* ktmtypes.h:152:30, ktmtypes.h:152:30, ktmtypes.h:152:30 */
#[repr(C)] pub struct TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT { SavepointId: ::ktmtypes::SAVEPOINT_ID } /* ktmtypes.h:154:16, ktmtypes.h:154:16, ktmtypes.h:154:16 */
pub type PTRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT = *mut ::ktmtypes::TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT; /* ktmtypes.h:156:49, ktmtypes.h:156:49, ktmtypes.h:156:49 */
#[repr(C)] pub struct TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT { PropagationCookie: ::minwindef::ULONG, UOW: ::guiddef::GUID, TmIdentity: ::guiddef::GUID, BufferLength: ::minwindef::ULONG } /* ktmtypes.h:158:16, ktmtypes.h:158:16, ktmtypes.h:158:16 */
pub type PTRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT = *mut ::ktmtypes::TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT; /* ktmtypes.h:164:49, ktmtypes.h:164:49, ktmtypes.h:164:49 */
#[repr(C)] pub struct TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT { MarshalCookie: ::minwindef::ULONG, UOW: ::guiddef::GUID } /* ktmtypes.h:166:16, ktmtypes.h:166:16, ktmtypes.h:166:16 */
pub type PTRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT = *mut ::ktmtypes::TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT; /* ktmtypes.h:169:47, ktmtypes.h:169:47, ktmtypes.h:169:47 */
pub type TRANSACTION_NOTIFICATION_PROMOTE_ARGUMENT = ::ktmtypes::TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT; /* ktmtypes.h:171:53, ktmtypes.h:171:53, ktmtypes.h:171:53 */
pub type PTRANSACTION_NOTIFICATION_PROMOTE_ARGUMENT = *mut ::ktmtypes::TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT; /* ktmtypes.h:171:97, ktmtypes.h:171:97, ktmtypes.h:171:97 */
#[repr(C)] pub struct KCRM_MARSHAL_HEADER { VersionMajor: ::minwindef::ULONG, VersionMinor: ::minwindef::ULONG, NumProtocols: ::minwindef::ULONG, Unused: ::minwindef::ULONG } /* ktmtypes.h:179:16, ktmtypes.h:179:16, ktmtypes.h:179:16 */
pub type PKCRM_MARSHAL_HEADER = *mut ::ktmtypes::KCRM_MARSHAL_HEADER; /* ktmtypes.h:184:25, ktmtypes.h:184:25, ktmtypes.h:184:25 */
pub type PRKCRM_MARSHAL_HEADER = *mut ::ktmtypes::KCRM_MARSHAL_HEADER; /* ktmtypes.h:184:67, ktmtypes.h:184:67, ktmtypes.h:184:67 */
#[repr(C)] pub struct KCRM_TRANSACTION_BLOB { UOW: ::ktmtypes::UOW, TmIdentity: ::guiddef::GUID, IsolationLevel: ::minwindef::ULONG, IsolationFlags: ::minwindef::ULONG, Timeout: ::minwindef::ULONG, Description: *mut [::winnt::WCHAR; 64] } /* ktmtypes.h:186:16, ktmtypes.h:186:16, ktmtypes.h:186:16 */
pub type PKCRM_TRANSACTION_BLOB = *mut ::ktmtypes::KCRM_TRANSACTION_BLOB; /* ktmtypes.h:193:27, ktmtypes.h:193:27, ktmtypes.h:193:27 */
pub type PRKCRM_TRANSACTION_BLOB = *mut ::ktmtypes::KCRM_TRANSACTION_BLOB; /* ktmtypes.h:193:71, ktmtypes.h:193:71, ktmtypes.h:193:71 */
#[repr(C)] pub struct KCRM_PROTOCOL_BLOB { ProtocolId: ::ktmtypes::CRM_PROTOCOL_ID, StaticInfoLength: ::minwindef::ULONG, TransactionIdInfoLength: ::minwindef::ULONG, Unused1: ::minwindef::ULONG, Unused2: ::minwindef::ULONG } /* ktmtypes.h:195:16, ktmtypes.h:195:16, ktmtypes.h:195:16 */
pub type PKCRM_PROTOCOL_BLOB = *mut ::ktmtypes::KCRM_PROTOCOL_BLOB; /* ktmtypes.h:201:24, ktmtypes.h:201:24, ktmtypes.h:201:24 */
pub type PRKCRM_PROTOCOL_BLOB = *mut ::ktmtypes::KCRM_PROTOCOL_BLOB; /* ktmtypes.h:201:65, ktmtypes.h:201:65, ktmtypes.h:201:65 */
pub const TRANSACTION_MANAGER_VOLATILE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* ktmtypes.h:34:9, ktmtypes.h:34:9, ktmtypes.h:34:9 */
pub const TRANSACTION_MANAGER_COMMIT_DEFAULT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* ktmtypes.h:35:9, ktmtypes.h:35:9, ktmtypes.h:35:9 */
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_VOLUME: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* ktmtypes.h:36:9, ktmtypes.h:36:9, ktmtypes.h:36:9 */
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_HIVES: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* ktmtypes.h:37:9, ktmtypes.h:37:9, ktmtypes.h:37:9 */
pub const TRANSACTION_MANAGER_COMMIT_LOWEST: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* ktmtypes.h:38:9, ktmtypes.h:38:9, ktmtypes.h:38:9 */
pub const TRANSACTION_MANAGER_CORRUPT_FOR_RECOVERY: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* ktmtypes.h:39:9, ktmtypes.h:39:9, ktmtypes.h:39:9 */
pub const TRANSACTION_MANAGER_CORRUPT_FOR_PROGRESS: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* ktmtypes.h:40:9, ktmtypes.h:40:9, ktmtypes.h:40:9 */
pub const TRANSACTION_MANAGER_MAXIMUM_OPTION: i32 = 0x3fi32; /* Integer(63, Yes, Unknown) */ /* ktmtypes.h:41:9, ktmtypes.h:41:9, ktmtypes.h:41:9 */
pub const TRANSACTION_DO_NOT_PROMOTE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* ktmtypes.h:48:9, ktmtypes.h:48:9, ktmtypes.h:48:9 */
pub const TRANSACTION_MAXIMUM_OPTION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* ktmtypes.h:49:9, ktmtypes.h:49:9, ktmtypes.h:49:9 */
pub const RESOURCE_MANAGER_VOLATILE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* ktmtypes.h:56:9, ktmtypes.h:56:9, ktmtypes.h:56:9 */
pub const RESOURCE_MANAGER_COMMUNICATION: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* ktmtypes.h:57:9, ktmtypes.h:57:9, ktmtypes.h:57:9 */
pub const RESOURCE_MANAGER_MAXIMUM_OPTION: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* ktmtypes.h:58:9, ktmtypes.h:58:9, ktmtypes.h:58:9 */
pub const CRM_PROTOCOL_EXPLICIT_MARSHAL_ONLY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* ktmtypes.h:65:9, ktmtypes.h:65:9, ktmtypes.h:65:9 */
pub const CRM_PROTOCOL_DYNAMIC_MARSHAL_INFO: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* ktmtypes.h:66:9, ktmtypes.h:66:9, ktmtypes.h:66:9 */
pub const CRM_PROTOCOL_MAXIMUM_OPTION: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* ktmtypes.h:67:9, ktmtypes.h:67:9, ktmtypes.h:67:9 */
pub const ENLISTMENT_SUPERIOR: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* ktmtypes.h:74:9, ktmtypes.h:74:9, ktmtypes.h:74:9 */
pub const ENLISTMENT_MAXIMUM_OPTION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* ktmtypes.h:75:9, ktmtypes.h:75:9, ktmtypes.h:75:9 */
pub const TRANSACTION_NOTIFY_MASK: i32 = 0x3fffffffi32; /* Integer(1073741823, Yes, Unknown) */ /* ktmtypes.h:79:9, ktmtypes.h:79:9, ktmtypes.h:79:9 */
pub const TRANSACTION_NOTIFY_PREPREPARE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* ktmtypes.h:80:9, ktmtypes.h:80:9, ktmtypes.h:80:9 */
pub const TRANSACTION_NOTIFY_PREPARE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* ktmtypes.h:81:9, ktmtypes.h:81:9, ktmtypes.h:81:9 */
pub const TRANSACTION_NOTIFY_COMMIT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* ktmtypes.h:82:9, ktmtypes.h:82:9, ktmtypes.h:82:9 */
pub const TRANSACTION_NOTIFY_ROLLBACK: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* ktmtypes.h:83:9, ktmtypes.h:83:9, ktmtypes.h:83:9 */
pub const TRANSACTION_NOTIFY_PREPREPARE_COMPLETE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* ktmtypes.h:84:9, ktmtypes.h:84:9, ktmtypes.h:84:9 */
pub const TRANSACTION_NOTIFY_PREPARE_COMPLETE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* ktmtypes.h:85:9, ktmtypes.h:85:9, ktmtypes.h:85:9 */
pub const TRANSACTION_NOTIFY_COMMIT_COMPLETE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* ktmtypes.h:86:9, ktmtypes.h:86:9, ktmtypes.h:86:9 */
pub const TRANSACTION_NOTIFY_ROLLBACK_COMPLETE: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* ktmtypes.h:87:9, ktmtypes.h:87:9, ktmtypes.h:87:9 */
pub const TRANSACTION_NOTIFY_RECOVER: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* ktmtypes.h:88:9, ktmtypes.h:88:9, ktmtypes.h:88:9 */
pub const TRANSACTION_NOTIFY_SINGLE_PHASE_COMMIT: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* ktmtypes.h:89:9, ktmtypes.h:89:9, ktmtypes.h:89:9 */
pub const TRANSACTION_NOTIFY_DELEGATE_COMMIT: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* ktmtypes.h:90:9, ktmtypes.h:90:9, ktmtypes.h:90:9 */
pub const TRANSACTION_NOTIFY_RECOVER_QUERY: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* ktmtypes.h:91:9, ktmtypes.h:91:9, ktmtypes.h:91:9 */
pub const TRANSACTION_NOTIFY_ENLIST_PREPREPARE: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* ktmtypes.h:92:9, ktmtypes.h:92:9, ktmtypes.h:92:9 */
pub const TRANSACTION_NOTIFY_LAST_RECOVER: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* ktmtypes.h:93:9, ktmtypes.h:93:9, ktmtypes.h:93:9 */
pub const TRANSACTION_NOTIFY_INDOUBT: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* ktmtypes.h:94:9, ktmtypes.h:94:9, ktmtypes.h:94:9 */
pub const TRANSACTION_NOTIFY_PROPAGATE_PULL: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* ktmtypes.h:95:9, ktmtypes.h:95:9, ktmtypes.h:95:9 */
pub const TRANSACTION_NOTIFY_PROPAGATE_PUSH: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* ktmtypes.h:96:9, ktmtypes.h:96:9, ktmtypes.h:96:9 */
pub const TRANSACTION_NOTIFY_MARSHAL: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* ktmtypes.h:97:9, ktmtypes.h:97:9, ktmtypes.h:97:9 */
pub const TRANSACTION_NOTIFY_ENLIST_MASK: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* ktmtypes.h:98:9, ktmtypes.h:98:9, ktmtypes.h:98:9 */
pub const TRANSACTION_NOTIFY_RM_DISCONNECTED: i32 = 0x1000000i32; /* Integer(16777216, Yes, Unknown) */ /* ktmtypes.h:99:9, ktmtypes.h:99:9, ktmtypes.h:99:9 */
pub const TRANSACTION_NOTIFY_TM_ONLINE: i32 = 0x2000000i32; /* Integer(33554432, Yes, Unknown) */ /* ktmtypes.h:100:9, ktmtypes.h:100:9, ktmtypes.h:100:9 */
pub const TRANSACTION_NOTIFY_COMMIT_REQUEST: i32 = 0x4000000i32; /* Integer(67108864, Yes, Unknown) */ /* ktmtypes.h:101:9, ktmtypes.h:101:9, ktmtypes.h:101:9 */
pub const TRANSACTION_NOTIFY_PROMOTE: i32 = 0x8000000i32; /* Integer(134217728, Yes, Unknown) */ /* ktmtypes.h:102:9, ktmtypes.h:102:9, ktmtypes.h:102:9 */
pub const TRANSACTION_NOTIFY_PROMOTE_NEW: i32 = 0x10000000i32; /* Integer(268435456, Yes, Unknown) */ /* ktmtypes.h:103:9, ktmtypes.h:103:9, ktmtypes.h:103:9 */
pub const TRANSACTION_NOTIFY_REQUEST_OUTCOME: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* ktmtypes.h:104:9, ktmtypes.h:104:9, ktmtypes.h:104:9 */
pub const TRANSACTION_NOTIFY_COMMIT_FINALIZE: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* ktmtypes.h:111:9, ktmtypes.h:111:9, ktmtypes.h:111:9 */
pub const TRANSACTIONMANAGER_OBJECT_PATH: &'static str = "\\TransactionManager\\"; /* String("\\\\TransactionManager\\\\", Wide) */ /* ktmtypes.h:117:9, ktmtypes.h:117:9, ktmtypes.h:117:9 */
pub const TRANSACTION_OBJECT_PATH: &'static str = "\\Transaction\\"; /* String("\\\\Transaction\\\\", Wide) */ /* ktmtypes.h:118:9, ktmtypes.h:118:9, ktmtypes.h:118:9 */
pub const ENLISTMENT_OBJECT_PATH: &'static str = "\\Enlistment\\"; /* String("\\\\Enlistment\\\\", Wide) */ /* ktmtypes.h:119:9, ktmtypes.h:119:9, ktmtypes.h:119:9 */
pub const RESOURCE_MANAGER_OBJECT_PATH: &'static str = "\\ResourceManager\\"; /* String("\\\\ResourceManager\\\\", Wide) */ /* ktmtypes.h:120:9, ktmtypes.h:120:9, ktmtypes.h:120:9 */
pub const TRANSACTION_NOTIFICATION_TM_ONLINE_FLAG_IS_CLUSTERED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* ktmtypes.h:145:9, ktmtypes.h:145:9, ktmtypes.h:145:9 */
pub const KTM_MARSHAL_BLOB_VERSION_MAJOR: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* ktmtypes.h:173:9, ktmtypes.h:173:9, ktmtypes.h:173:9 */
pub const KTM_MARSHAL_BLOB_VERSION_MINOR: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* ktmtypes.h:174:9, ktmtypes.h:174:9, ktmtypes.h:174:9 */
pub const MAX_TRANSACTION_DESCRIPTION_LENGTH: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* ktmtypes.h:176:9, ktmtypes.h:176:9, ktmtypes.h:176:9 */
pub const MAX_RESOURCEMANAGER_DESCRIPTION_LENGTH: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* ktmtypes.h:177:9, ktmtypes.h:177:9, ktmtypes.h:177:9 */
