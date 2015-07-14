#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct NETRESOURCEA { dwScope: ::minwindef::DWORD, dwType: ::minwindef::DWORD, dwDisplayType: ::minwindef::DWORD, dwUsage: ::minwindef::DWORD, lpLocalName: ::winnt::LPSTR, lpRemoteName: ::winnt::LPSTR, lpComment: ::winnt::LPSTR, lpProvider: ::winnt::LPSTR } /* winnetwk.h:92:17, winnetwk.h:92:17, winnetwk.h:92:17 */
#[cfg(feature="winapi_desktop")] pub type LPNETRESOURCEA = *mut ::winnetwk::NETRESOURCEA; /* winnetwk.h:101:17, winnetwk.h:101:17, winnetwk.h:101:17 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct NETRESOURCEW { dwScope: ::minwindef::DWORD, dwType: ::minwindef::DWORD, dwDisplayType: ::minwindef::DWORD, dwUsage: ::minwindef::DWORD, lpLocalName: ::winnt::LPWSTR, lpRemoteName: ::winnt::LPWSTR, lpComment: ::winnt::LPWSTR, lpProvider: ::winnt::LPWSTR } /* winnetwk.h:102:17, winnetwk.h:102:17, winnetwk.h:102:17 */
#[cfg(feature="winapi_desktop")] pub type LPNETRESOURCEW = *mut ::winnetwk::NETRESOURCEW; /* winnetwk.h:111:17, winnetwk.h:111:17, winnetwk.h:111:17 */
#[cfg(feature="winapi_desktop")] pub type NETRESOURCE = ::winnetwk::NETRESOURCEW; /* winnetwk.h:113:22, winnetwk.h:113:22, winnetwk.h:113:22 */
#[cfg(feature="winapi_desktop")] pub type LPNETRESOURCE = ::winnetwk::LPNETRESOURCEW; /* winnetwk.h:114:24, winnetwk.h:114:24, winnetwk.h:114:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct CONNECTDLGSTRUCTA { cbStructure: ::minwindef::DWORD, hwndOwner: ::windef::HWND, lpConnRes: ::winnetwk::LPNETRESOURCEA, dwFlags: ::minwindef::DWORD, dwDevNum: ::minwindef::DWORD } /* winnetwk.h:345:16, winnetwk.h:345:16, winnetwk.h:345:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPCONNECTDLGSTRUCTA = *mut ::winnetwk::CONNECTDLGSTRUCTA; /* winnetwk.h:351:27, winnetwk.h:351:27, winnetwk.h:351:27 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct CONNECTDLGSTRUCTW { cbStructure: ::minwindef::DWORD, hwndOwner: ::windef::HWND, lpConnRes: ::winnetwk::LPNETRESOURCEW, dwFlags: ::minwindef::DWORD, dwDevNum: ::minwindef::DWORD } /* winnetwk.h:352:16, winnetwk.h:352:16, winnetwk.h:352:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPCONNECTDLGSTRUCTW = *mut ::winnetwk::CONNECTDLGSTRUCTW; /* winnetwk.h:358:27, winnetwk.h:358:27, winnetwk.h:358:27 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type CONNECTDLGSTRUCT = ::winnetwk::CONNECTDLGSTRUCTW; /* winnetwk.h:360:27, winnetwk.h:360:27, winnetwk.h:360:27 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPCONNECTDLGSTRUCT = ::winnetwk::LPCONNECTDLGSTRUCTW; /* winnetwk.h:361:29, winnetwk.h:361:29, winnetwk.h:361:29 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct DISCDLGSTRUCTA { cbStructure: ::minwindef::DWORD, hwndOwner: ::windef::HWND, lpLocalName: ::winnt::LPSTR, lpRemoteName: ::winnt::LPSTR, dwFlags: ::minwindef::DWORD } /* winnetwk.h:396:16, winnetwk.h:396:16, winnetwk.h:396:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPDISCDLGSTRUCTA = *mut ::winnetwk::DISCDLGSTRUCTA; /* winnetwk.h:402:24, winnetwk.h:402:24, winnetwk.h:402:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct DISCDLGSTRUCTW { cbStructure: ::minwindef::DWORD, hwndOwner: ::windef::HWND, lpLocalName: ::winnt::LPWSTR, lpRemoteName: ::winnt::LPWSTR, dwFlags: ::minwindef::DWORD } /* winnetwk.h:403:16, winnetwk.h:403:16, winnetwk.h:403:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPDISCDLGSTRUCTW = *mut ::winnetwk::DISCDLGSTRUCTW; /* winnetwk.h:409:24, winnetwk.h:409:24, winnetwk.h:409:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type DISCDLGSTRUCT = ::winnetwk::DISCDLGSTRUCTW; /* winnetwk.h:411:24, winnetwk.h:411:24, winnetwk.h:411:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPDISCDLGSTRUCT = ::winnetwk::LPDISCDLGSTRUCTW; /* winnetwk.h:412:26, winnetwk.h:412:26, winnetwk.h:412:26 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct UNIVERSAL_NAME_INFOA { lpUniversalName: ::winnt::LPSTR } /* winnetwk.h:545:17, winnetwk.h:545:17, winnetwk.h:545:17 */
#[cfg(feature="winapi_desktop")] pub type LPUNIVERSAL_NAME_INFOA = *mut ::winnetwk::UNIVERSAL_NAME_INFOA; /* winnetwk.h:547:25, winnetwk.h:547:25, winnetwk.h:547:25 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct UNIVERSAL_NAME_INFOW { lpUniversalName: ::winnt::LPWSTR } /* winnetwk.h:548:17, winnetwk.h:548:17, winnetwk.h:548:17 */
#[cfg(feature="winapi_desktop")] pub type LPUNIVERSAL_NAME_INFOW = *mut ::winnetwk::UNIVERSAL_NAME_INFOW; /* winnetwk.h:550:25, winnetwk.h:550:25, winnetwk.h:550:25 */
#[cfg(feature="winapi_desktop")] pub type UNIVERSAL_NAME_INFO = ::winnetwk::UNIVERSAL_NAME_INFOW; /* winnetwk.h:552:30, winnetwk.h:552:30, winnetwk.h:552:30 */
#[cfg(feature="winapi_desktop")] pub type LPUNIVERSAL_NAME_INFO = ::winnetwk::LPUNIVERSAL_NAME_INFOW; /* winnetwk.h:553:32, winnetwk.h:553:32, winnetwk.h:553:32 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct REMOTE_NAME_INFOA { lpUniversalName: ::winnt::LPSTR, lpConnectionName: ::winnt::LPSTR, lpRemainingPath: ::winnt::LPSTR } /* winnetwk.h:559:17, winnetwk.h:559:17, winnetwk.h:559:17 */
#[cfg(feature="winapi_desktop")] pub type LPREMOTE_NAME_INFOA = *mut ::winnetwk::REMOTE_NAME_INFOA; /* winnetwk.h:563:22, winnetwk.h:563:22, winnetwk.h:563:22 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct REMOTE_NAME_INFOW { lpUniversalName: ::winnt::LPWSTR, lpConnectionName: ::winnt::LPWSTR, lpRemainingPath: ::winnt::LPWSTR } /* winnetwk.h:564:17, winnetwk.h:564:17, winnetwk.h:564:17 */
#[cfg(feature="winapi_desktop")] pub type LPREMOTE_NAME_INFOW = *mut ::winnetwk::REMOTE_NAME_INFOW; /* winnetwk.h:568:22, winnetwk.h:568:22, winnetwk.h:568:22 */
#[cfg(feature="winapi_desktop")] pub type REMOTE_NAME_INFO = ::winnetwk::REMOTE_NAME_INFOW; /* winnetwk.h:570:27, winnetwk.h:570:27, winnetwk.h:570:27 */
#[cfg(feature="winapi_desktop")] pub type LPREMOTE_NAME_INFO = ::winnetwk::LPREMOTE_NAME_INFOW; /* winnetwk.h:571:29, winnetwk.h:571:29, winnetwk.h:571:29 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct NETINFOSTRUCT { cbStructure: ::minwindef::DWORD, dwProviderVersion: ::minwindef::DWORD, dwStatus: ::minwindef::DWORD, dwCharacteristics: ::minwindef::DWORD, dwHandle: ::basetsd::ULONG_PTR, wNetType: ::minwindef::WORD, dwPrinters: ::minwindef::DWORD, dwDrives: ::minwindef::DWORD } /* winnetwk.h:660:16, winnetwk.h:660:16, winnetwk.h:660:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPNETINFOSTRUCT = *mut ::winnetwk::NETINFOSTRUCT; /* winnetwk.h:669:23, winnetwk.h:669:23, winnetwk.h:669:23 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct NETCONNECTINFOSTRUCT { cbStructure: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD, dwSpeed: ::minwindef::DWORD, dwDelay: ::minwindef::DWORD, dwOptDataSize: ::minwindef::DWORD } /* winnetwk.h:792:16, winnetwk.h:792:16, winnetwk.h:792:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPNETCONNECTINFOSTRUCT = *mut ::winnetwk::NETCONNECTINFOSTRUCT; /* winnetwk.h:798:27, winnetwk.h:798:27, winnetwk.h:798:27 */
#[cfg(feature="winapi_desktop")] pub const RESOURCE_CONNECTED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnetwk.h:49:9, winnetwk.h:49:9, winnetwk.h:49:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCE_GLOBALNET: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnetwk.h:50:9, winnetwk.h:50:9, winnetwk.h:50:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCE_REMEMBERED: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnetwk.h:51:9, winnetwk.h:51:9, winnetwk.h:51:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const RESOURCE_RECENT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnetwk.h:53:9, winnetwk.h:53:9, winnetwk.h:53:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const RESOURCE_CONTEXT: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnetwk.h:54:9, winnetwk.h:54:9, winnetwk.h:54:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCETYPE_ANY: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnetwk.h:57:9, winnetwk.h:57:9, winnetwk.h:57:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCETYPE_DISK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnetwk.h:58:9, winnetwk.h:58:9, winnetwk.h:58:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCETYPE_PRINT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnetwk.h:59:9, winnetwk.h:59:9, winnetwk.h:59:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const RESOURCETYPE_RESERVED: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnetwk.h:61:9, winnetwk.h:61:9, winnetwk.h:61:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCETYPE_UNKNOWN: i32 = 0xffffffffi32; /* Integer(4294967295, Yes, Unknown) */ /* winnetwk.h:63:9, winnetwk.h:63:9, winnetwk.h:63:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCEUSAGE_CONNECTABLE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnetwk.h:65:9, winnetwk.h:65:9, winnetwk.h:65:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCEUSAGE_CONTAINER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnetwk.h:66:9, winnetwk.h:66:9, winnetwk.h:66:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const RESOURCEUSAGE_NOLOCALDEVICE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnetwk.h:68:9, winnetwk.h:68:9, winnetwk.h:68:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const RESOURCEUSAGE_SIBLING: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnetwk.h:69:9, winnetwk.h:69:9, winnetwk.h:69:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const RESOURCEUSAGE_ATTACHED: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnetwk.h:70:9, winnetwk.h:70:9, winnetwk.h:70:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCEUSAGE_RESERVED: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnetwk.h:73:9, winnetwk.h:73:9, winnetwk.h:73:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCEDISPLAYTYPE_GENERIC: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnetwk.h:75:9, winnetwk.h:75:9, winnetwk.h:75:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCEDISPLAYTYPE_DOMAIN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnetwk.h:76:9, winnetwk.h:76:9, winnetwk.h:76:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCEDISPLAYTYPE_SERVER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnetwk.h:77:9, winnetwk.h:77:9, winnetwk.h:77:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCEDISPLAYTYPE_SHARE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnetwk.h:78:9, winnetwk.h:78:9, winnetwk.h:78:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCEDISPLAYTYPE_FILE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnetwk.h:79:9, winnetwk.h:79:9, winnetwk.h:79:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCEDISPLAYTYPE_GROUP: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnetwk.h:80:9, winnetwk.h:80:9, winnetwk.h:80:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const RESOURCEDISPLAYTYPE_NETWORK: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnetwk.h:82:9, winnetwk.h:82:9, winnetwk.h:82:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const RESOURCEDISPLAYTYPE_ROOT: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnetwk.h:83:9, winnetwk.h:83:9, winnetwk.h:83:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const RESOURCEDISPLAYTYPE_SHAREADMIN: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnetwk.h:84:9, winnetwk.h:84:9, winnetwk.h:84:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const RESOURCEDISPLAYTYPE_DIRECTORY: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnetwk.h:85:9, winnetwk.h:85:9, winnetwk.h:85:9 */
#[cfg(feature="winapi_desktop")] pub const RESOURCEDISPLAYTYPE_TREE: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnetwk.h:87:9, winnetwk.h:87:9, winnetwk.h:87:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const RESOURCEDISPLAYTYPE_NDSCONTAINER: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnetwk.h:89:9, winnetwk.h:89:9, winnetwk.h:89:9 */
#[cfg(feature="winapi_desktop")] pub const NETPROPERTY_PERSISTENT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnetwk.h:125:9, winnetwk.h:125:9, winnetwk.h:125:9 */
#[cfg(feature="winapi_desktop")] pub const CONNECT_UPDATE_PROFILE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnetwk.h:127:9, winnetwk.h:127:9, winnetwk.h:127:9 */
#[cfg(feature="winapi_desktop")] pub const CONNECT_UPDATE_RECENT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnetwk.h:128:9, winnetwk.h:128:9, winnetwk.h:128:9 */
#[cfg(feature="winapi_desktop")] pub const CONNECT_TEMPORARY: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnetwk.h:129:9, winnetwk.h:129:9, winnetwk.h:129:9 */
#[cfg(feature="winapi_desktop")] pub const CONNECT_INTERACTIVE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnetwk.h:130:9, winnetwk.h:130:9, winnetwk.h:130:9 */
#[cfg(feature="winapi_desktop")] pub const CONNECT_PROMPT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnetwk.h:131:9, winnetwk.h:131:9, winnetwk.h:131:9 */
#[cfg(feature="winapi_desktop")] pub const CONNECT_NEED_DRIVE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnetwk.h:132:9, winnetwk.h:132:9, winnetwk.h:132:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const CONNECT_REFCOUNT: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnetwk.h:134:9, winnetwk.h:134:9, winnetwk.h:134:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const CONNECT_REDIRECT: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnetwk.h:135:9, winnetwk.h:135:9, winnetwk.h:135:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const CONNECT_LOCALDRIVE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnetwk.h:136:9, winnetwk.h:136:9, winnetwk.h:136:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const CONNECT_CURRENT_MEDIA: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnetwk.h:137:9, winnetwk.h:137:9, winnetwk.h:137:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const CONNECT_DEFERRED: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnetwk.h:138:9, winnetwk.h:138:9, winnetwk.h:138:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const CONNECT_RESERVED: i32 = 0xff000000i32; /* Integer(4278190080, Yes, Unknown) */ /* winnetwk.h:139:9, winnetwk.h:139:9, winnetwk.h:139:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const CONNECT_COMMANDLINE: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winnetwk.h:142:9, winnetwk.h:142:9, winnetwk.h:142:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const CONNECT_CMD_SAVECRED: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winnetwk.h:143:9, winnetwk.h:143:9, winnetwk.h:143:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] pub const CONNECT_CRED_RESET: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winnetwk.h:146:9, winnetwk.h:146:9, winnetwk.h:146:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const CONNDLG_RO_PATH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnetwk.h:367:9, winnetwk.h:367:9, winnetwk.h:367:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const CONNDLG_CONN_POINT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnetwk.h:368:9, winnetwk.h:368:9, winnetwk.h:368:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const CONNDLG_USE_MRU: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnetwk.h:369:9, winnetwk.h:369:9, winnetwk.h:369:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const CONNDLG_HIDE_BOX: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnetwk.h:370:9, winnetwk.h:370:9, winnetwk.h:370:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const CONNDLG_PERSIST: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnetwk.h:377:9, winnetwk.h:377:9, winnetwk.h:377:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const CONNDLG_NOT_PERSIST: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnetwk.h:378:9, winnetwk.h:378:9, winnetwk.h:378:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const DISC_UPDATE_PROFILE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnetwk.h:418:9, winnetwk.h:418:9, winnetwk.h:418:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const DISC_NO_FORCE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnetwk.h:419:9, winnetwk.h:419:9, winnetwk.h:419:9 */
#[cfg(feature="winapi_desktop")] pub const UNIVERSAL_NAME_INFO_LEVEL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnetwk.h:542:9, winnetwk.h:542:9, winnetwk.h:542:9 */
#[cfg(feature="winapi_desktop")] pub const REMOTE_NAME_INFO_LEVEL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnetwk.h:543:9, winnetwk.h:543:9, winnetwk.h:543:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const WNFMT_MULTILINE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnetwk.h:632:9, winnetwk.h:632:9, winnetwk.h:632:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const WNFMT_ABBREVIATED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnetwk.h:633:9, winnetwk.h:633:9, winnetwk.h:633:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const WNFMT_INENUM: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnetwk.h:634:9, winnetwk.h:634:9, winnetwk.h:634:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const WNFMT_CONNECTION: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnetwk.h:635:9, winnetwk.h:635:9, winnetwk.h:635:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const NETINFO_DLL16: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnetwk.h:671:9, winnetwk.h:671:9, winnetwk.h:671:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const NETINFO_DISKRED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnetwk.h:672:9, winnetwk.h:672:9, winnetwk.h:672:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const NETINFO_PRINTERRED: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnetwk.h:673:9, winnetwk.h:673:9, winnetwk.h:673:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::NO_ERROR as WN_SUCCESS; /* winnetwk.h:729:9, winnetwk.h:729:9, winnetwk.h:729:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::NO_ERROR as WN_NO_ERROR; /* winnetwk.h:730:9, winnetwk.h:730:9, winnetwk.h:730:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_NOT_SUPPORTED as WN_NOT_SUPPORTED; /* winnetwk.h:731:9, winnetwk.h:731:9, winnetwk.h:731:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_CANCELLED as WN_CANCEL; /* winnetwk.h:732:9, winnetwk.h:732:9, winnetwk.h:732:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_RETRY as WN_RETRY; /* winnetwk.h:733:9, winnetwk.h:733:9, winnetwk.h:733:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_UNEXP_NET_ERR as WN_NET_ERROR; /* winnetwk.h:734:9, winnetwk.h:734:9, winnetwk.h:734:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_MORE_DATA as WN_MORE_DATA; /* winnetwk.h:735:9, winnetwk.h:735:9, winnetwk.h:735:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_INVALID_ADDRESS as WN_BAD_POINTER; /* winnetwk.h:736:9, winnetwk.h:736:9, winnetwk.h:736:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_INVALID_PARAMETER as WN_BAD_VALUE; /* winnetwk.h:737:9, winnetwk.h:737:9, winnetwk.h:737:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_BAD_USERNAME as WN_BAD_USER; /* winnetwk.h:738:9, winnetwk.h:738:9, winnetwk.h:738:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_INVALID_PASSWORD as WN_BAD_PASSWORD; /* winnetwk.h:739:9, winnetwk.h:739:9, winnetwk.h:739:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_ACCESS_DENIED as WN_ACCESS_DENIED; /* winnetwk.h:740:9, winnetwk.h:740:9, winnetwk.h:740:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_BUSY as WN_FUNCTION_BUSY; /* winnetwk.h:741:9, winnetwk.h:741:9, winnetwk.h:741:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_UNEXP_NET_ERR as WN_WINDOWS_ERROR; /* winnetwk.h:742:9, winnetwk.h:742:9, winnetwk.h:742:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_NOT_ENOUGH_MEMORY as WN_OUT_OF_MEMORY; /* winnetwk.h:743:9, winnetwk.h:743:9, winnetwk.h:743:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_NO_NETWORK as WN_NO_NETWORK; /* winnetwk.h:744:9, winnetwk.h:744:9, winnetwk.h:744:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_EXTENDED_ERROR as WN_EXTENDED_ERROR; /* winnetwk.h:745:9, winnetwk.h:745:9, winnetwk.h:745:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_INVALID_LEVEL as WN_BAD_LEVEL; /* winnetwk.h:746:9, winnetwk.h:746:9, winnetwk.h:746:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_INVALID_HANDLE as WN_BAD_HANDLE; /* winnetwk.h:747:9, winnetwk.h:747:9, winnetwk.h:747:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winerror::ERROR_ALREADY_INITIALIZED as WN_NOT_INITIALIZING; /* winnetwk.h:749:9, winnetwk.h:749:9, winnetwk.h:749:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winerror::ERROR_NO_MORE_DEVICES as WN_NO_MORE_DEVICES; /* winnetwk.h:750:9, winnetwk.h:750:9, winnetwk.h:750:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_NOT_CONNECTED as WN_NOT_CONNECTED; /* winnetwk.h:755:9, winnetwk.h:755:9, winnetwk.h:755:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_OPEN_FILES as WN_OPEN_FILES; /* winnetwk.h:756:9, winnetwk.h:756:9, winnetwk.h:756:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_DEVICE_IN_USE as WN_DEVICE_IN_USE; /* winnetwk.h:757:9, winnetwk.h:757:9, winnetwk.h:757:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_BAD_NET_NAME as WN_BAD_NETNAME; /* winnetwk.h:758:9, winnetwk.h:758:9, winnetwk.h:758:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_BAD_DEVICE as WN_BAD_LOCALNAME; /* winnetwk.h:759:9, winnetwk.h:759:9, winnetwk.h:759:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_ALREADY_ASSIGNED as WN_ALREADY_CONNECTED; /* winnetwk.h:760:9, winnetwk.h:760:9, winnetwk.h:760:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_GEN_FAILURE as WN_DEVICE_ERROR; /* winnetwk.h:761:9, winnetwk.h:761:9, winnetwk.h:761:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_CONNECTION_UNAVAIL as WN_CONNECTION_CLOSED; /* winnetwk.h:762:9, winnetwk.h:762:9, winnetwk.h:762:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_NO_NET_OR_BAD_PATH as WN_NO_NET_OR_BAD_PATH; /* winnetwk.h:763:9, winnetwk.h:763:9, winnetwk.h:763:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_BAD_PROVIDER as WN_BAD_PROVIDER; /* winnetwk.h:764:9, winnetwk.h:764:9, winnetwk.h:764:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_CANNOT_OPEN_PROFILE as WN_CANNOT_OPEN_PROFILE; /* winnetwk.h:765:9, winnetwk.h:765:9, winnetwk.h:765:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_BAD_PROFILE as WN_BAD_PROFILE; /* winnetwk.h:766:9, winnetwk.h:766:9, winnetwk.h:766:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_BAD_DEV_TYPE as WN_BAD_DEV_TYPE; /* winnetwk.h:767:9, winnetwk.h:767:9, winnetwk.h:767:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_DEVICE_ALREADY_REMEMBERED as WN_DEVICE_ALREADY_REMEMBERED; /* winnetwk.h:768:9, winnetwk.h:768:9, winnetwk.h:768:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_CONNECTED_OTHER_PASSWORD as WN_CONNECTED_OTHER_PASSWORD; /* winnetwk.h:769:9, winnetwk.h:769:9, winnetwk.h:769:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05010000"))] #[doc(inline)] pub use ::winerror::ERROR_CONNECTED_OTHER_PASSWORD_DEFAULT as WN_CONNECTED_OTHER_PASSWORD_DEFAULT; /* winnetwk.h:771:9, winnetwk.h:771:9, winnetwk.h:771:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_NO_MORE_ITEMS as WN_NO_MORE_ENTRIES; /* winnetwk.h:776:9, winnetwk.h:776:9, winnetwk.h:776:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::winerror::ERROR_NOT_CONTAINER as WN_NOT_CONTAINER; /* winnetwk.h:777:9, winnetwk.h:777:9, winnetwk.h:777:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winerror::ERROR_NOT_AUTHENTICATED as WN_NOT_AUTHENTICATED; /* winnetwk.h:782:9, winnetwk.h:782:9, winnetwk.h:782:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winerror::ERROR_NOT_LOGGED_ON as WN_NOT_LOGGED_ON; /* winnetwk.h:783:9, winnetwk.h:783:9, winnetwk.h:783:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::winerror::ERROR_NO_LOGON_SERVERS as WN_NOT_VALIDATED; /* winnetwk.h:784:9, winnetwk.h:784:9, winnetwk.h:784:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const WNCON_FORNETCARD: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnetwk.h:800:9, winnetwk.h:800:9, winnetwk.h:800:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const WNCON_NOTROUTED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnetwk.h:801:9, winnetwk.h:801:9, winnetwk.h:801:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const WNCON_SLOWLINK: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnetwk.h:802:9, winnetwk.h:802:9, winnetwk.h:802:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const WNCON_DYNAMIC: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnetwk.h:803:9, winnetwk.h:803:9, winnetwk.h:803:9 */
