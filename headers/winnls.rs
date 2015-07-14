#[cfg(feature="winapi_app")] pub type LGRPID = ::minwindef::DWORD; /* winnls.h:934:15, winnls.h:934:15, winnls.h:934:15 */
#[cfg(feature="winapi_app")] pub type LCTYPE = ::minwindef::DWORD; /* winnls.h:939:15, winnls.h:939:15, winnls.h:939:15 */
#[cfg(feature="winapi_app")] pub type CALTYPE = ::minwindef::DWORD; /* winnls.h:944:15, winnls.h:944:15, winnls.h:944:15 */
#[cfg(feature="winapi_app")] pub type CALID = ::minwindef::DWORD; /* winnls.h:950:15, winnls.h:950:15, winnls.h:950:15 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct CPINFO { MaxCharSize: ::minwindef::UINT, DefaultChar: *mut [::minwindef::BYTE; 2], LeadByte: *mut [::minwindef::BYTE; 12] } /* winnls.h:957:16, winnls.h:957:16, winnls.h:957:16 */
#[cfg(feature="winapi_app")] pub type LPCPINFO = *mut ::winnls::CPINFO; /* winnls.h:961:12, winnls.h:961:12, winnls.h:961:12 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct CPINFOEXA { MaxCharSize: ::minwindef::UINT, DefaultChar: *mut [::minwindef::BYTE; 2], LeadByte: *mut [::minwindef::BYTE; 12], UnicodeDefaultChar: ::winnt::WCHAR, CodePage: ::minwindef::UINT, CodePageName: *mut [::winnt::CHAR; 260] } /* winnls.h:963:16, winnls.h:963:16, winnls.h:963:16 */
#[cfg(feature="winapi_app")] pub type LPCPINFOEXA = *mut ::winnls::CPINFOEXA; /* winnls.h:970:15, winnls.h:970:15, winnls.h:970:15 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct CPINFOEXW { MaxCharSize: ::minwindef::UINT, DefaultChar: *mut [::minwindef::BYTE; 2], LeadByte: *mut [::minwindef::BYTE; 12], UnicodeDefaultChar: ::winnt::WCHAR, CodePage: ::minwindef::UINT, CodePageName: *mut [::winnt::WCHAR; 260] } /* winnls.h:971:16, winnls.h:971:16, winnls.h:971:16 */
#[cfg(feature="winapi_app")] pub type LPCPINFOEXW = *mut ::winnls::CPINFOEXW; /* winnls.h:978:15, winnls.h:978:15, winnls.h:978:15 */
#[cfg(feature="winapi_app")] pub type CPINFOEX = ::winnls::CPINFOEXW; /* winnls.h:980:19, winnls.h:980:19, winnls.h:980:19 */
#[cfg(feature="winapi_app")] pub type LPCPINFOEX = ::winnls::LPCPINFOEXW; /* winnls.h:981:21, winnls.h:981:21, winnls.h:981:21 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct NUMBERFMTA { NumDigits: ::minwindef::UINT, LeadingZero: ::minwindef::UINT, Grouping: ::minwindef::UINT, lpDecimalSep: ::winnt::LPSTR, lpThousandSep: ::winnt::LPSTR, NegativeOrder: ::minwindef::UINT } /* winnls.h:992:16, winnls.h:992:16, winnls.h:992:16 */
#[cfg(feature="winapi_app")] pub type LPNUMBERFMTA = *mut ::winnls::NUMBERFMTA; /* winnls.h:999:16, winnls.h:999:16, winnls.h:999:16 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct NUMBERFMTW { NumDigits: ::minwindef::UINT, LeadingZero: ::minwindef::UINT, Grouping: ::minwindef::UINT, lpDecimalSep: ::winnt::LPWSTR, lpThousandSep: ::winnt::LPWSTR, NegativeOrder: ::minwindef::UINT } /* winnls.h:1000:16, winnls.h:1000:16, winnls.h:1000:16 */
#[cfg(feature="winapi_app")] pub type LPNUMBERFMTW = *mut ::winnls::NUMBERFMTW; /* winnls.h:1007:16, winnls.h:1007:16, winnls.h:1007:16 */
#[cfg(feature="winapi_app")] pub type NUMBERFMT = ::winnls::NUMBERFMTW; /* winnls.h:1009:20, winnls.h:1009:20, winnls.h:1009:20 */
#[cfg(feature="winapi_app")] pub type LPNUMBERFMT = ::winnls::LPNUMBERFMTW; /* winnls.h:1010:22, winnls.h:1010:22, winnls.h:1010:22 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct CURRENCYFMTA { NumDigits: ::minwindef::UINT, LeadingZero: ::minwindef::UINT, Grouping: ::minwindef::UINT, lpDecimalSep: ::winnt::LPSTR, lpThousandSep: ::winnt::LPSTR, NegativeOrder: ::minwindef::UINT, PositiveOrder: ::minwindef::UINT, lpCurrencySymbol: ::winnt::LPSTR } /* winnls.h:1021:16, winnls.h:1021:16, winnls.h:1021:16 */
#[cfg(feature="winapi_app")] pub type LPCURRENCYFMTA = *mut ::winnls::CURRENCYFMTA; /* winnls.h:1030:18, winnls.h:1030:18, winnls.h:1030:18 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct CURRENCYFMTW { NumDigits: ::minwindef::UINT, LeadingZero: ::minwindef::UINT, Grouping: ::minwindef::UINT, lpDecimalSep: ::winnt::LPWSTR, lpThousandSep: ::winnt::LPWSTR, NegativeOrder: ::minwindef::UINT, PositiveOrder: ::minwindef::UINT, lpCurrencySymbol: ::winnt::LPWSTR } /* winnls.h:1031:16, winnls.h:1031:16, winnls.h:1031:16 */
#[cfg(feature="winapi_app")] pub type LPCURRENCYFMTW = *mut ::winnls::CURRENCYFMTW; /* winnls.h:1040:18, winnls.h:1040:18, winnls.h:1040:18 */
#[cfg(feature="winapi_app")] pub type CURRENCYFMT = ::winnls::CURRENCYFMTW; /* winnls.h:1042:22, winnls.h:1042:22, winnls.h:1042:22 */
#[cfg(feature="winapi_app")] pub type LPCURRENCYFMT = ::winnls::LPCURRENCYFMTW; /* winnls.h:1043:24, winnls.h:1043:24, winnls.h:1043:24 */
#[cfg(feature="winapi_app")] #[repr(C)] pub enum SYSNLS_FUNCTION {COMPARE_STRING = 1, __SeeGhIssue10292} pub use self::SYSNLS_FUNCTION::{COMPARE_STRING}; /* winnls.h:1053:6, winnls.h:1053:6, winnls.h:1053:6 */
#[cfg(feature="winapi_app")] pub type NLS_FUNCTION = ::minwindef::DWORD; /* winnls.h:1056:15, winnls.h:1056:15, winnls.h:1056:15 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06020000"))] #[repr(C)] pub struct NLSVERSIONINFO { dwNLSVersionInfoSize: ::minwindef::DWORD, dwNLSVersion: ::minwindef::DWORD, dwDefinedVersion: ::minwindef::DWORD, dwEffectiveId: ::minwindef::DWORD, guidCustomVersion: ::guiddef::GUID } /* winnls.h:1071:16, winnls.h:1071:16, winnls.h:1071:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06020000"))] pub type LPNLSVERSIONINFO = *mut ::winnls::NLSVERSIONINFO; /* winnls.h:1077:20, winnls.h:1077:20, winnls.h:1077:20 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct NLSVERSIONINFOEX { dwNLSVersionInfoSize: ::minwindef::DWORD, dwNLSVersion: ::minwindef::DWORD, dwDefinedVersion: ::minwindef::DWORD, dwEffectiveId: ::minwindef::DWORD, guidCustomVersion: ::guiddef::GUID } /* winnls.h:1096:16, winnls.h:1096:16, winnls.h:1096:16 */
#[cfg(feature="winapi_app")] pub type LPNLSVERSIONINFOEX = *mut ::winnls::NLSVERSIONINFOEX; /* winnls.h:1102:22, winnls.h:1102:22, winnls.h:1102:22 */
#[cfg(feature="winapi_app")] pub type GEOID = ::winnt::LONG; /* winnls.h:1108:17, winnls.h:1108:17, winnls.h:1108:17 */
#[cfg(feature="winapi_app")] pub type GEOTYPE = ::minwindef::DWORD; /* winnls.h:1109:17, winnls.h:1109:17, winnls.h:1109:17 */
#[cfg(feature="winapi_app")] pub type GEOCLASS = ::minwindef::DWORD; /* winnls.h:1110:17, winnls.h:1110:17, winnls.h:1110:17 */
#[cfg(feature="winapi_app")] #[repr(C)] pub enum SYSGEOTYPE {GEO_NATION = 1, GEO_LATITUDE = 2, GEO_LONGITUDE = 3, GEO_ISO2 = 4, GEO_ISO3 = 5, GEO_RFC1766 = 6, GEO_LCID = 7, GEO_FRIENDLYNAME = 8, GEO_OFFICIALNAME = 9, GEO_TIMEZONES = 10, GEO_OFFICIALLANGUAGES = 11, GEO_ISO_UN_NUMBER = 12, GEO_PARENT = 13} pub use self::SYSGEOTYPE::{GEO_NATION, GEO_LATITUDE, GEO_LONGITUDE, GEO_ISO2, GEO_ISO3, GEO_RFC1766, GEO_LCID, GEO_FRIENDLYNAME, GEO_OFFICIALNAME, GEO_TIMEZONES, GEO_OFFICIALLANGUAGES, GEO_ISO_UN_NUMBER, GEO_PARENT}; /* winnls.h:1118:6, winnls.h:1118:6, winnls.h:1118:6 */
#[cfg(feature="winapi_app")] #[repr(C)] pub enum SYSGEOCLASS {GEOCLASS_NATION = 16, GEOCLASS_REGION = 14, GEOCLASS_ALL = 0} pub use self::SYSGEOCLASS::{GEOCLASS_NATION, GEOCLASS_REGION, GEOCLASS_ALL}; /* winnls.h:1138:6, winnls.h:1138:6, winnls.h:1138:6 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] #[repr(C)] pub enum NORM_FORM {NormalizationOther = 0, NormalizationC = 1, NormalizationD = 2, NormalizationKC = 5, NormalizationKD = 6} pub use self::NORM_FORM::{NormalizationOther, NormalizationC, NormalizationD, NormalizationKC, NormalizationKD}; /* winnls.h:1149:14, winnls.h:1149:14, winnls.h:1149:14 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type LANGUAGEGROUP_ENUMPROCA = extern "system" fn(::libc::c_ulong, *mut ::libc::c_schar, *mut ::libc::c_schar, ::libc::c_ulong, ::libc::c_long) -> ::libc::c_int; /* winnls.h:1179:25, winnls.h:1179:25 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type LANGGROUPLOCALE_ENUMPROCA = extern "system" fn(::libc::c_ulong, ::libc::c_ulong, *mut ::libc::c_schar, ::libc::c_long) -> ::libc::c_int; /* winnls.h:1180:25, winnls.h:1180:25 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type UILANGUAGE_ENUMPROCA = extern "system" fn(*mut ::libc::c_schar, ::libc::c_long) -> ::libc::c_int; /* winnls.h:1181:25, winnls.h:1181:25 */
#[cfg(feature="winapi_app")] pub type CODEPAGE_ENUMPROCA = extern "system" fn(*mut ::libc::c_schar) -> ::libc::c_int; /* winnls.h:1182:25, winnls.h:1182:25, winnls.h:1182:25 */
#[cfg(feature="winapi_app")] pub type DATEFMT_ENUMPROCA = extern "system" fn(*mut ::libc::c_schar) -> ::libc::c_int; /* winnls.h:1183:25, winnls.h:1183:25, winnls.h:1183:25 */
#[cfg(feature="winapi_app")] pub type DATEFMT_ENUMPROCEXA = extern "system" fn(*mut ::libc::c_schar, ::libc::c_ulong) -> ::libc::c_int; /* winnls.h:1184:25, winnls.h:1184:25, winnls.h:1184:25 */
#[cfg(feature="winapi_app")] pub type TIMEFMT_ENUMPROCA = extern "system" fn(*mut ::libc::c_schar) -> ::libc::c_int; /* winnls.h:1185:25, winnls.h:1185:25, winnls.h:1185:25 */
#[cfg(feature="winapi_app")] pub type CALINFO_ENUMPROCA = extern "system" fn(*mut ::libc::c_schar) -> ::libc::c_int; /* winnls.h:1186:25, winnls.h:1186:25, winnls.h:1186:25 */
#[cfg(feature="winapi_app")] pub type CALINFO_ENUMPROCEXA = extern "system" fn(*mut ::libc::c_schar, ::libc::c_ulong) -> ::libc::c_int; /* winnls.h:1187:25, winnls.h:1187:25, winnls.h:1187:25 */
#[cfg(feature="winapi_app")] pub type LOCALE_ENUMPROCA = extern "system" fn(*mut ::libc::c_schar) -> ::libc::c_int; /* winnls.h:1188:25, winnls.h:1188:25, winnls.h:1188:25 */
#[cfg(feature="winapi_app")] pub type LOCALE_ENUMPROCW = extern "system" fn(*mut ::libc::c_ushort) -> ::libc::c_int; /* winnls.h:1189:25, winnls.h:1189:25, winnls.h:1189:25 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type LANGUAGEGROUP_ENUMPROCW = extern "system" fn(::libc::c_ulong, *mut ::libc::c_ushort, *mut ::libc::c_ushort, ::libc::c_ulong, ::libc::c_long) -> ::libc::c_int; /* winnls.h:1191:25, winnls.h:1191:25 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type LANGGROUPLOCALE_ENUMPROCW = extern "system" fn(::libc::c_ulong, ::libc::c_ulong, *mut ::libc::c_ushort, ::libc::c_long) -> ::libc::c_int; /* winnls.h:1192:25, winnls.h:1192:25 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type UILANGUAGE_ENUMPROCW = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_long) -> ::libc::c_int; /* winnls.h:1193:25, winnls.h:1193:25 */
#[cfg(feature="winapi_app")] pub type CODEPAGE_ENUMPROCW = extern "system" fn(*mut ::libc::c_ushort) -> ::libc::c_int; /* winnls.h:1194:25, winnls.h:1194:25, winnls.h:1194:25 */
#[cfg(feature="winapi_app")] pub type DATEFMT_ENUMPROCW = extern "system" fn(*mut ::libc::c_ushort) -> ::libc::c_int; /* winnls.h:1195:25, winnls.h:1195:25, winnls.h:1195:25 */
#[cfg(feature="winapi_app")] pub type DATEFMT_ENUMPROCEXW = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_ulong) -> ::libc::c_int; /* winnls.h:1196:25, winnls.h:1196:25, winnls.h:1196:25 */
#[cfg(feature="winapi_app")] pub type TIMEFMT_ENUMPROCW = extern "system" fn(*mut ::libc::c_ushort) -> ::libc::c_int; /* winnls.h:1197:25, winnls.h:1197:25, winnls.h:1197:25 */
#[cfg(feature="winapi_app")] pub type CALINFO_ENUMPROCW = extern "system" fn(*mut ::libc::c_ushort) -> ::libc::c_int; /* winnls.h:1198:25, winnls.h:1198:25, winnls.h:1198:25 */
#[cfg(feature="winapi_app")] pub type CALINFO_ENUMPROCEXW = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_ulong) -> ::libc::c_int; /* winnls.h:1199:25, winnls.h:1199:25, winnls.h:1199:25 */
#[cfg(feature="winapi_app")] pub type GEO_ENUMPROC = extern "system" fn(::libc::c_long) -> ::libc::c_int; /* winnls.h:1200:25, winnls.h:1200:25, winnls.h:1200:25 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct FILEMUIINFO { dwSize: ::minwindef::DWORD, dwVersion: ::minwindef::DWORD, dwFileType: ::minwindef::DWORD, pChecksum: *mut [::minwindef::BYTE; 16], pServiceChecksum: *mut [::minwindef::BYTE; 16], dwLanguageNameOffset: ::minwindef::DWORD, dwTypeIDMainSize: ::minwindef::DWORD, dwTypeIDMainOffset: ::minwindef::DWORD, dwTypeNameMainOffset: ::minwindef::DWORD, dwTypeIDMUISize: ::minwindef::DWORD, dwTypeIDMUIOffset: ::minwindef::DWORD, dwTypeNameMUIOffset: ::minwindef::DWORD, abBuffer: *mut [::minwindef::BYTE; 8] } /* winnls.h:1262:16, winnls.h:1262:16, winnls.h:1262:16 */
#[cfg(feature="winapi_app")] pub type PFILEMUIINFO = *mut ::winnls::FILEMUIINFO; /* winnls.h:1276:17, winnls.h:1276:17, winnls.h:1276:17 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type CALINFO_ENUMPROCEXEX = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_ulong, *mut ::libc::c_ushort, ::libc::c_long) -> ::libc::c_int; /* winnls.h:2511:25, winnls.h:2511:25 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type DATEFMT_ENUMPROCEXEX = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_ulong, ::libc::c_long) -> ::libc::c_int; /* winnls.h:2525:25, winnls.h:2525:25 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type TIMEFMT_ENUMPROCEX = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_long) -> ::libc::c_int; /* winnls.h:2537:25, winnls.h:2537:25 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type LOCALE_ENUMPROCEX = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_ulong, ::libc::c_long) -> ::libc::c_int; /* winnls.h:2549:25, winnls.h:2549:25 */
#[cfg(feature="winapi_app")] pub const MAX_LEADBYTES: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnls.h:63:9, winnls.h:63:9, winnls.h:63:9 */
#[cfg(feature="winapi_app")] pub const MAX_DEFAULTCHAR: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:64:9, winnls.h:64:9, winnls.h:64:9 */
#[cfg(feature="winapi_app")] pub const HIGH_SURROGATE_START: i32 = 0xd800i32; /* Integer(55296, Yes, Unknown) */ /* winnls.h:93:9, winnls.h:93:9, winnls.h:93:9 */
#[cfg(feature="winapi_app")] pub const HIGH_SURROGATE_END: i32 = 0xdbffi32; /* Integer(56319, Yes, Unknown) */ /* winnls.h:94:9, winnls.h:94:9, winnls.h:94:9 */
#[cfg(feature="winapi_app")] pub const LOW_SURROGATE_START: i32 = 0xdc00i32; /* Integer(56320, Yes, Unknown) */ /* winnls.h:95:9, winnls.h:95:9, winnls.h:95:9 */
#[cfg(feature="winapi_app")] pub const LOW_SURROGATE_END: i32 = 0xdfffi32; /* Integer(57343, Yes, Unknown) */ /* winnls.h:96:9, winnls.h:96:9, winnls.h:96:9 */
#[cfg(feature="winapi_app")] pub const MB_PRECOMPOSED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:102:9, winnls.h:102:9, winnls.h:102:9 */
#[cfg(feature="winapi_app")] pub const MB_COMPOSITE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:103:9, winnls.h:103:9, winnls.h:103:9 */
#[cfg(feature="winapi_app")] pub const MB_USEGLYPHCHARS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:104:9, winnls.h:104:9, winnls.h:104:9 */
#[cfg(feature="winapi_app")] pub const MB_ERR_INVALID_CHARS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:105:9, winnls.h:105:9, winnls.h:105:9 */
#[cfg(feature="winapi_app")] pub const WC_COMPOSITECHECK: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnls.h:107:9, winnls.h:107:9, winnls.h:107:9 */
#[cfg(feature="winapi_app")] pub const WC_DISCARDNS: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnls.h:108:9, winnls.h:108:9, winnls.h:108:9 */
#[cfg(feature="winapi_app")] pub const WC_SEPCHARS: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnls.h:109:9, winnls.h:109:9, winnls.h:109:9 */
#[cfg(feature="winapi_app")] pub const WC_DEFAULTCHAR: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnls.h:110:9, winnls.h:110:9, winnls.h:110:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const WC_ERR_INVALID_CHARS: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnls.h:112:9, winnls.h:112:9, winnls.h:112:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub const WC_NO_BEST_FIT_CHARS: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnls.h:116:9, winnls.h:116:9, winnls.h:116:9 */
#[cfg(feature="winapi_app")] pub const CT_CTYPE1: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:123:9, winnls.h:123:9, winnls.h:123:9 */
#[cfg(feature="winapi_app")] pub const CT_CTYPE2: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:124:9, winnls.h:124:9, winnls.h:124:9 */
#[cfg(feature="winapi_app")] pub const CT_CTYPE3: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:125:9, winnls.h:125:9, winnls.h:125:9 */
#[cfg(feature="winapi_app")] pub const C1_UPPER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:130:9, winnls.h:130:9, winnls.h:130:9 */
#[cfg(feature="winapi_app")] pub const C1_LOWER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:131:9, winnls.h:131:9, winnls.h:131:9 */
#[cfg(feature="winapi_app")] pub const C1_DIGIT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:132:9, winnls.h:132:9, winnls.h:132:9 */
#[cfg(feature="winapi_app")] pub const C1_SPACE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:133:9, winnls.h:133:9, winnls.h:133:9 */
#[cfg(feature="winapi_app")] pub const C1_PUNCT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnls.h:134:9, winnls.h:134:9, winnls.h:134:9 */
#[cfg(feature="winapi_app")] pub const C1_CNTRL: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnls.h:135:9, winnls.h:135:9, winnls.h:135:9 */
#[cfg(feature="winapi_app")] pub const C1_BLANK: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnls.h:136:9, winnls.h:136:9, winnls.h:136:9 */
#[cfg(feature="winapi_app")] pub const C1_XDIGIT: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnls.h:137:9, winnls.h:137:9, winnls.h:137:9 */
#[cfg(feature="winapi_app")] pub const C1_ALPHA: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnls.h:138:9, winnls.h:138:9, winnls.h:138:9 */
#[cfg(feature="winapi_app")] pub const C1_DEFINED: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnls.h:139:9, winnls.h:139:9, winnls.h:139:9 */
#[cfg(feature="winapi_app")] pub const C2_LEFTTORIGHT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:144:9, winnls.h:144:9, winnls.h:144:9 */
#[cfg(feature="winapi_app")] pub const C2_RIGHTTOLEFT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:145:9, winnls.h:145:9, winnls.h:145:9 */
#[cfg(feature="winapi_app")] pub const C2_EUROPENUMBER: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnls.h:147:9, winnls.h:147:9, winnls.h:147:9 */
#[cfg(feature="winapi_app")] pub const C2_EUROPESEPARATOR: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:148:9, winnls.h:148:9, winnls.h:148:9 */
#[cfg(feature="winapi_app")] pub const C2_EUROPETERMINATOR: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnls.h:149:9, winnls.h:149:9, winnls.h:149:9 */
#[cfg(feature="winapi_app")] pub const C2_ARABICNUMBER: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnls.h:150:9, winnls.h:150:9, winnls.h:150:9 */
#[cfg(feature="winapi_app")] pub const C2_COMMONSEPARATOR: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnls.h:151:9, winnls.h:151:9, winnls.h:151:9 */
#[cfg(feature="winapi_app")] pub const C2_BLOCKSEPARATOR: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:153:9, winnls.h:153:9, winnls.h:153:9 */
#[cfg(feature="winapi_app")] pub const C2_SEGMENTSEPARATOR: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnls.h:154:9, winnls.h:154:9, winnls.h:154:9 */
#[cfg(feature="winapi_app")] pub const C2_WHITESPACE: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnls.h:155:9, winnls.h:155:9, winnls.h:155:9 */
#[cfg(feature="winapi_app")] pub const C2_OTHERNEUTRAL: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnls.h:156:9, winnls.h:156:9, winnls.h:156:9 */
#[cfg(feature="winapi_app")] pub const C2_NOTAPPLICABLE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnls.h:158:9, winnls.h:158:9, winnls.h:158:9 */
#[cfg(feature="winapi_app")] pub const C3_NONSPACING: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:163:9, winnls.h:163:9, winnls.h:163:9 */
#[cfg(feature="winapi_app")] pub const C3_DIACRITIC: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:164:9, winnls.h:164:9, winnls.h:164:9 */
#[cfg(feature="winapi_app")] pub const C3_VOWELMARK: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:165:9, winnls.h:165:9, winnls.h:165:9 */
#[cfg(feature="winapi_app")] pub const C3_SYMBOL: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:166:9, winnls.h:166:9, winnls.h:166:9 */
#[cfg(feature="winapi_app")] pub const C3_KATAKANA: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnls.h:168:9, winnls.h:168:9, winnls.h:168:9 */
#[cfg(feature="winapi_app")] pub const C3_HIRAGANA: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnls.h:169:9, winnls.h:169:9, winnls.h:169:9 */
#[cfg(feature="winapi_app")] pub const C3_HALFWIDTH: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnls.h:170:9, winnls.h:170:9, winnls.h:170:9 */
#[cfg(feature="winapi_app")] pub const C3_FULLWIDTH: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnls.h:171:9, winnls.h:171:9, winnls.h:171:9 */
#[cfg(feature="winapi_app")] pub const C3_IDEOGRAPH: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnls.h:172:9, winnls.h:172:9, winnls.h:172:9 */
#[cfg(feature="winapi_app")] pub const C3_KASHIDA: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnls.h:173:9, winnls.h:173:9, winnls.h:173:9 */
#[cfg(feature="winapi_app")] pub const C3_LEXICAL: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnls.h:174:9, winnls.h:174:9, winnls.h:174:9 */
#[cfg(feature="winapi_app")] pub const C3_HIGHSURROGATE: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winnls.h:175:9, winnls.h:175:9, winnls.h:175:9 */
#[cfg(feature="winapi_app")] pub const C3_LOWSURROGATE: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winnls.h:176:9, winnls.h:176:9, winnls.h:176:9 */
#[cfg(feature="winapi_app")] pub const C3_ALPHA: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* winnls.h:178:9, winnls.h:178:9, winnls.h:178:9 */
#[cfg(feature="winapi_app")] pub const C3_NOTAPPLICABLE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnls.h:180:9, winnls.h:180:9, winnls.h:180:9 */
#[cfg(feature="winapi_app")] pub const NORM_IGNORECASE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:186:9, winnls.h:186:9, winnls.h:186:9 */
#[cfg(feature="winapi_app")] pub const NORM_IGNORENONSPACE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:187:9, winnls.h:187:9, winnls.h:187:9 */
#[cfg(feature="winapi_app")] pub const NORM_IGNORESYMBOLS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:188:9, winnls.h:188:9, winnls.h:188:9 */
#[cfg(feature="winapi_app")] pub const LINGUISTIC_IGNORECASE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnls.h:190:9, winnls.h:190:9, winnls.h:190:9 */
#[cfg(feature="winapi_app")] pub const LINGUISTIC_IGNOREDIACRITIC: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnls.h:191:9, winnls.h:191:9, winnls.h:191:9 */
#[cfg(feature="winapi_app")] pub const NORM_IGNOREKANATYPE: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* winnls.h:193:9, winnls.h:193:9, winnls.h:193:9 */
#[cfg(feature="winapi_app")] pub const NORM_IGNOREWIDTH: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* winnls.h:194:9, winnls.h:194:9, winnls.h:194:9 */
#[cfg(feature="winapi_app")] pub const NORM_LINGUISTIC_CASING: i32 = 0x8000000i32; /* Integer(134217728, Yes, Unknown) */ /* winnls.h:195:9, winnls.h:195:9, winnls.h:195:9 */
#[cfg(feature="winapi_app")] pub const MAP_FOLDCZONE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnls.h:201:9, winnls.h:201:9, winnls.h:201:9 */
#[cfg(feature="winapi_app")] pub const MAP_PRECOMPOSED: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnls.h:202:9, winnls.h:202:9, winnls.h:202:9 */
#[cfg(feature="winapi_app")] pub const MAP_COMPOSITE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnls.h:203:9, winnls.h:203:9, winnls.h:203:9 */
#[cfg(feature="winapi_app")] pub const MAP_FOLDDIGITS: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* winnls.h:204:9, winnls.h:204:9, winnls.h:204:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MAP_EXPAND_LIGATURES: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* winnls.h:207:9, winnls.h:207:9, winnls.h:207:9 */
#[cfg(feature="winapi_app")] pub const LCMAP_LOWERCASE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnls.h:213:9, winnls.h:213:9, winnls.h:213:9 */
#[cfg(feature="winapi_app")] pub const LCMAP_UPPERCASE: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnls.h:214:9, winnls.h:214:9, winnls.h:214:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LCMAP_TITLECASE: i32 = 0x300i32; /* Integer(768, Yes, Unknown) */ /* winnls.h:216:9, winnls.h:216:9, winnls.h:216:9 */
#[cfg(feature="winapi_app")] pub const LCMAP_SORTKEY: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnls.h:219:9, winnls.h:219:9, winnls.h:219:9 */
#[cfg(feature="winapi_app")] pub const LCMAP_BYTEREV: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* winnls.h:220:9, winnls.h:220:9, winnls.h:220:9 */
#[cfg(feature="winapi_app")] pub const LCMAP_HIRAGANA: i32 = 0x100000i32; /* Integer(1048576, Yes, Unknown) */ /* winnls.h:222:9, winnls.h:222:9, winnls.h:222:9 */
#[cfg(feature="winapi_app")] pub const LCMAP_KATAKANA: i32 = 0x200000i32; /* Integer(2097152, Yes, Unknown) */ /* winnls.h:223:9, winnls.h:223:9, winnls.h:223:9 */
#[cfg(feature="winapi_app")] pub const LCMAP_HALFWIDTH: i32 = 0x400000i32; /* Integer(4194304, Yes, Unknown) */ /* winnls.h:224:9, winnls.h:224:9, winnls.h:224:9 */
#[cfg(feature="winapi_app")] pub const LCMAP_FULLWIDTH: i32 = 0x800000i32; /* Integer(8388608, Yes, Unknown) */ /* winnls.h:225:9, winnls.h:225:9, winnls.h:225:9 */
#[cfg(feature="winapi_app")] pub const LCMAP_LINGUISTIC_CASING: i32 = 0x1000000i32; /* Integer(16777216, Yes, Unknown) */ /* winnls.h:227:9, winnls.h:227:9, winnls.h:227:9 */
#[cfg(feature="winapi_app")] pub const LCMAP_SIMPLIFIED_CHINESE: i32 = 0x2000000i32; /* Integer(33554432, Yes, Unknown) */ /* winnls.h:229:9, winnls.h:229:9, winnls.h:229:9 */
#[cfg(feature="winapi_app")] pub const LCMAP_TRADITIONAL_CHINESE: i32 = 0x4000000i32; /* Integer(67108864, Yes, Unknown) */ /* winnls.h:230:9, winnls.h:230:9, winnls.h:230:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06020000"))] pub const LCMAP_SORTHANDLE: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* winnls.h:234:9, winnls.h:234:9, winnls.h:234:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06020000"))] pub const LCMAP_HASH: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* winnls.h:235:9, winnls.h:235:9, winnls.h:235:9 */
#[cfg(feature="winapi_app")] pub const FIND_STARTSWITH: i32 = 0x100000i32; /* Integer(1048576, Yes, Unknown) */ /* winnls.h:241:9, winnls.h:241:9, winnls.h:241:9 */
#[cfg(feature="winapi_app")] pub const FIND_ENDSWITH: i32 = 0x200000i32; /* Integer(2097152, Yes, Unknown) */ /* winnls.h:242:9, winnls.h:242:9, winnls.h:242:9 */
#[cfg(feature="winapi_app")] pub const FIND_FROMSTART: i32 = 0x400000i32; /* Integer(4194304, Yes, Unknown) */ /* winnls.h:243:9, winnls.h:243:9, winnls.h:243:9 */
#[cfg(feature="winapi_app")] pub const FIND_FROMEND: i32 = 0x800000i32; /* Integer(8388608, Yes, Unknown) */ /* winnls.h:244:9, winnls.h:244:9, winnls.h:244:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_INSTALLED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:250:9, winnls.h:250:9, winnls.h:250:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_SUPPORTED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:251:9, winnls.h:251:9, winnls.h:251:9 */
#[cfg(feature="winapi_app")] pub const LCID_INSTALLED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:257:9, winnls.h:257:9, winnls.h:257:9 */
#[cfg(feature="winapi_app")] pub const LCID_SUPPORTED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:258:9, winnls.h:258:9, winnls.h:258:9 */
#[cfg(feature="winapi_app")] pub const LCID_ALTERNATE_SORTS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:259:9, winnls.h:259:9, winnls.h:259:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_ALL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnls.h:266:9, winnls.h:266:9, winnls.h:266:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_WINDOWS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:267:9, winnls.h:267:9, winnls.h:267:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SUPPLEMENTAL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:268:9, winnls.h:268:9, winnls.h:268:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_ALTERNATE_SORTS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:269:9, winnls.h:269:9, winnls.h:269:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_REPLACEMENT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:270:9, winnls.h:270:9, winnls.h:270:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_NEUTRALDATA: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnls.h:273:9, winnls.h:273:9, winnls.h:273:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_SPECIFICDATA: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnls.h:274:9, winnls.h:274:9, winnls.h:274:9 */
#[cfg(feature="winapi_app")] pub const CP_INSTALLED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:281:9, winnls.h:281:9, winnls.h:281:9 */
#[cfg(feature="winapi_app")] pub const CP_SUPPORTED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:282:9, winnls.h:282:9, winnls.h:282:9 */
#[cfg(feature="winapi_app")] pub const SORT_STRINGSORT: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* winnls.h:315:9, winnls.h:315:9, winnls.h:315:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const SORT_DIGITSASNUMBERS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:319:9, winnls.h:319:9, winnls.h:319:9 */
#[cfg(feature="winapi_app")] pub const CSTR_LESS_THAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:326:9, winnls.h:326:9, winnls.h:326:9 */
#[cfg(feature="winapi_app")] pub const CSTR_EQUAL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:327:9, winnls.h:327:9, winnls.h:327:9 */
#[cfg(feature="winapi_app")] pub const CSTR_GREATER_THAN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnls.h:328:9, winnls.h:328:9, winnls.h:328:9 */
#[cfg(feature="winapi_app")] pub const CP_ACP: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnls.h:334:9, winnls.h:334:9, winnls.h:334:9 */
#[cfg(feature="winapi_app")] pub const CP_OEMCP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:335:9, winnls.h:335:9, winnls.h:335:9 */
#[cfg(feature="winapi_app")] pub const CP_MACCP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:336:9, winnls.h:336:9, winnls.h:336:9 */
#[cfg(feature="winapi_app")] pub const CP_THREAD_ACP: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnls.h:337:9, winnls.h:337:9, winnls.h:337:9 */
#[cfg(feature="winapi_app")] pub const CP_SYMBOL: i32 = 0x2ai32; /* Integer(42, Yes, Unknown) */ /* winnls.h:338:9, winnls.h:338:9, winnls.h:338:9 */
#[cfg(feature="winapi_app")] pub const CP_UTF7: i32 = 0xfde8i32; /* Integer(65000, Yes, Unknown) */ /* winnls.h:340:9, winnls.h:340:9, winnls.h:340:9 */
#[cfg(feature="winapi_app")] pub const CP_UTF8: i32 = 0xfde9i32; /* Integer(65001, Yes, Unknown) */ /* winnls.h:341:9, winnls.h:341:9, winnls.h:341:9 */
#[cfg(feature="winapi_app")] pub const CTRY_DEFAULT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* winnls.h:347:9, winnls.h:347:9, winnls.h:347:9 */
#[cfg(feature="winapi_app")] pub const CTRY_ALBANIA: i32 = 0x163i32; /* Integer(355, Yes, Unknown) */ /* winnls.h:349:9, winnls.h:349:9, winnls.h:349:9 */
#[cfg(feature="winapi_app")] pub const CTRY_ALGERIA: i32 = 0xd5i32; /* Integer(213, Yes, Unknown) */ /* winnls.h:350:9, winnls.h:350:9, winnls.h:350:9 */
#[cfg(feature="winapi_app")] pub const CTRY_ARGENTINA: i32 = 0x36i32; /* Integer(54, Yes, Unknown) */ /* winnls.h:351:9, winnls.h:351:9, winnls.h:351:9 */
#[cfg(feature="winapi_app")] pub const CTRY_ARMENIA: i32 = 0x176i32; /* Integer(374, Yes, Unknown) */ /* winnls.h:352:9, winnls.h:352:9, winnls.h:352:9 */
#[cfg(feature="winapi_app")] pub const CTRY_AUSTRALIA: i32 = 0x3di32; /* Integer(61, Yes, Unknown) */ /* winnls.h:353:9, winnls.h:353:9, winnls.h:353:9 */
#[cfg(feature="winapi_app")] pub const CTRY_AUSTRIA: i32 = 0x2bi32; /* Integer(43, Yes, Unknown) */ /* winnls.h:354:9, winnls.h:354:9, winnls.h:354:9 */
#[cfg(feature="winapi_app")] pub const CTRY_AZERBAIJAN: i32 = 0x3e2i32; /* Integer(994, Yes, Unknown) */ /* winnls.h:355:9, winnls.h:355:9, winnls.h:355:9 */
#[cfg(feature="winapi_app")] pub const CTRY_BAHRAIN: i32 = 0x3cdi32; /* Integer(973, Yes, Unknown) */ /* winnls.h:356:9, winnls.h:356:9, winnls.h:356:9 */
#[cfg(feature="winapi_app")] pub const CTRY_BELARUS: i32 = 0x177i32; /* Integer(375, Yes, Unknown) */ /* winnls.h:357:9, winnls.h:357:9, winnls.h:357:9 */
#[cfg(feature="winapi_app")] pub const CTRY_BELGIUM: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnls.h:358:9, winnls.h:358:9, winnls.h:358:9 */
#[cfg(feature="winapi_app")] pub const CTRY_BELIZE: i32 = 0x1f5i32; /* Integer(501, Yes, Unknown) */ /* winnls.h:359:9, winnls.h:359:9, winnls.h:359:9 */
#[cfg(feature="winapi_app")] pub const CTRY_BOLIVIA: i32 = 0x24fi32; /* Integer(591, Yes, Unknown) */ /* winnls.h:360:9, winnls.h:360:9, winnls.h:360:9 */
#[cfg(feature="winapi_app")] pub const CTRY_BRAZIL: i32 = 0x37i32; /* Integer(55, Yes, Unknown) */ /* winnls.h:361:9, winnls.h:361:9, winnls.h:361:9 */
#[cfg(feature="winapi_app")] pub const CTRY_BRUNEI_DARUSSALAM: i32 = 0x2a1i32; /* Integer(673, Yes, Unknown) */ /* winnls.h:362:9, winnls.h:362:9, winnls.h:362:9 */
#[cfg(feature="winapi_app")] pub const CTRY_BULGARIA: i32 = 0x167i32; /* Integer(359, Yes, Unknown) */ /* winnls.h:363:9, winnls.h:363:9, winnls.h:363:9 */
#[cfg(feature="winapi_app")] pub const CTRY_CANADA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:364:9, winnls.h:364:9, winnls.h:364:9 */
#[cfg(feature="winapi_app")] pub const CTRY_CARIBBEAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:365:9, winnls.h:365:9, winnls.h:365:9 */
#[cfg(feature="winapi_app")] pub const CTRY_CHILE: i32 = 0x38i32; /* Integer(56, Yes, Unknown) */ /* winnls.h:366:9, winnls.h:366:9, winnls.h:366:9 */
#[cfg(feature="winapi_app")] pub const CTRY_COLOMBIA: i32 = 0x39i32; /* Integer(57, Yes, Unknown) */ /* winnls.h:367:9, winnls.h:367:9, winnls.h:367:9 */
#[cfg(feature="winapi_app")] pub const CTRY_COSTA_RICA: i32 = 0x1fai32; /* Integer(506, Yes, Unknown) */ /* winnls.h:368:9, winnls.h:368:9, winnls.h:368:9 */
#[cfg(feature="winapi_app")] pub const CTRY_CROATIA: i32 = 0x181i32; /* Integer(385, Yes, Unknown) */ /* winnls.h:369:9, winnls.h:369:9, winnls.h:369:9 */
#[cfg(feature="winapi_app")] pub const CTRY_CZECH: i32 = 0x1a4i32; /* Integer(420, Yes, Unknown) */ /* winnls.h:370:9, winnls.h:370:9, winnls.h:370:9 */
#[cfg(feature="winapi_app")] pub const CTRY_DENMARK: i32 = 0x2di32; /* Integer(45, Yes, Unknown) */ /* winnls.h:371:9, winnls.h:371:9, winnls.h:371:9 */
#[cfg(feature="winapi_app")] pub const CTRY_DOMINICAN_REPUBLIC: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:372:9, winnls.h:372:9, winnls.h:372:9 */
#[cfg(feature="winapi_app")] pub const CTRY_ECUADOR: i32 = 0x251i32; /* Integer(593, Yes, Unknown) */ /* winnls.h:373:9, winnls.h:373:9, winnls.h:373:9 */
#[cfg(feature="winapi_app")] pub const CTRY_EGYPT: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnls.h:374:9, winnls.h:374:9, winnls.h:374:9 */
#[cfg(feature="winapi_app")] pub const CTRY_EL_SALVADOR: i32 = 0x1f7i32; /* Integer(503, Yes, Unknown) */ /* winnls.h:375:9, winnls.h:375:9, winnls.h:375:9 */
#[cfg(feature="winapi_app")] pub const CTRY_ESTONIA: i32 = 0x174i32; /* Integer(372, Yes, Unknown) */ /* winnls.h:376:9, winnls.h:376:9, winnls.h:376:9 */
#[cfg(feature="winapi_app")] pub const CTRY_FAEROE_ISLANDS: i32 = 0x12ai32; /* Integer(298, Yes, Unknown) */ /* winnls.h:377:9, winnls.h:377:9, winnls.h:377:9 */
#[cfg(feature="winapi_app")] pub const CTRY_FINLAND: i32 = 0x166i32; /* Integer(358, Yes, Unknown) */ /* winnls.h:378:9, winnls.h:378:9, winnls.h:378:9 */
#[cfg(feature="winapi_app")] pub const CTRY_FRANCE: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* winnls.h:379:9, winnls.h:379:9, winnls.h:379:9 */
#[cfg(feature="winapi_app")] pub const CTRY_GEORGIA: i32 = 0x3e3i32; /* Integer(995, Yes, Unknown) */ /* winnls.h:380:9, winnls.h:380:9, winnls.h:380:9 */
#[cfg(feature="winapi_app")] pub const CTRY_GERMANY: i32 = 0x31i32; /* Integer(49, Yes, Unknown) */ /* winnls.h:381:9, winnls.h:381:9, winnls.h:381:9 */
#[cfg(feature="winapi_app")] pub const CTRY_GREECE: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* winnls.h:382:9, winnls.h:382:9, winnls.h:382:9 */
#[cfg(feature="winapi_app")] pub const CTRY_GUATEMALA: i32 = 0x1f6i32; /* Integer(502, Yes, Unknown) */ /* winnls.h:383:9, winnls.h:383:9, winnls.h:383:9 */
#[cfg(feature="winapi_app")] pub const CTRY_HONDURAS: i32 = 0x1f8i32; /* Integer(504, Yes, Unknown) */ /* winnls.h:384:9, winnls.h:384:9, winnls.h:384:9 */
#[cfg(feature="winapi_app")] pub const CTRY_HONG_KONG: i32 = 0x354i32; /* Integer(852, Yes, Unknown) */ /* winnls.h:385:9, winnls.h:385:9, winnls.h:385:9 */
#[cfg(feature="winapi_app")] pub const CTRY_HUNGARY: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* winnls.h:386:9, winnls.h:386:9, winnls.h:386:9 */
#[cfg(feature="winapi_app")] pub const CTRY_ICELAND: i32 = 0x162i32; /* Integer(354, Yes, Unknown) */ /* winnls.h:387:9, winnls.h:387:9, winnls.h:387:9 */
#[cfg(feature="winapi_app")] pub const CTRY_INDIA: i32 = 0x5bi32; /* Integer(91, Yes, Unknown) */ /* winnls.h:388:9, winnls.h:388:9, winnls.h:388:9 */
#[cfg(feature="winapi_app")] pub const CTRY_INDONESIA: i32 = 0x3ei32; /* Integer(62, Yes, Unknown) */ /* winnls.h:389:9, winnls.h:389:9, winnls.h:389:9 */
#[cfg(feature="winapi_app")] pub const CTRY_IRAN: i32 = 0x3d5i32; /* Integer(981, Yes, Unknown) */ /* winnls.h:390:9, winnls.h:390:9, winnls.h:390:9 */
#[cfg(feature="winapi_app")] pub const CTRY_IRAQ: i32 = 0x3c4i32; /* Integer(964, Yes, Unknown) */ /* winnls.h:391:9, winnls.h:391:9, winnls.h:391:9 */
#[cfg(feature="winapi_app")] pub const CTRY_IRELAND: i32 = 0x161i32; /* Integer(353, Yes, Unknown) */ /* winnls.h:392:9, winnls.h:392:9, winnls.h:392:9 */
#[cfg(feature="winapi_app")] pub const CTRY_ISRAEL: i32 = 0x3cci32; /* Integer(972, Yes, Unknown) */ /* winnls.h:393:9, winnls.h:393:9, winnls.h:393:9 */
#[cfg(feature="winapi_app")] pub const CTRY_ITALY: i32 = 0x27i32; /* Integer(39, Yes, Unknown) */ /* winnls.h:394:9, winnls.h:394:9, winnls.h:394:9 */
#[cfg(feature="winapi_app")] pub const CTRY_JAMAICA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:395:9, winnls.h:395:9, winnls.h:395:9 */
#[cfg(feature="winapi_app")] pub const CTRY_JAPAN: i32 = 0x51i32; /* Integer(81, Yes, Unknown) */ /* winnls.h:396:9, winnls.h:396:9, winnls.h:396:9 */
#[cfg(feature="winapi_app")] pub const CTRY_JORDAN: i32 = 0x3c2i32; /* Integer(962, Yes, Unknown) */ /* winnls.h:397:9, winnls.h:397:9, winnls.h:397:9 */
#[cfg(feature="winapi_app")] pub const CTRY_KAZAKSTAN: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnls.h:398:9, winnls.h:398:9, winnls.h:398:9 */
#[cfg(feature="winapi_app")] pub const CTRY_KENYA: i32 = 0xfei32; /* Integer(254, Yes, Unknown) */ /* winnls.h:399:9, winnls.h:399:9, winnls.h:399:9 */
#[cfg(feature="winapi_app")] pub const CTRY_KUWAIT: i32 = 0x3c5i32; /* Integer(965, Yes, Unknown) */ /* winnls.h:400:9, winnls.h:400:9, winnls.h:400:9 */
#[cfg(feature="winapi_app")] pub const CTRY_KYRGYZSTAN: i32 = 0x3e4i32; /* Integer(996, Yes, Unknown) */ /* winnls.h:401:9, winnls.h:401:9, winnls.h:401:9 */
#[cfg(feature="winapi_app")] pub const CTRY_LATVIA: i32 = 0x173i32; /* Integer(371, Yes, Unknown) */ /* winnls.h:402:9, winnls.h:402:9, winnls.h:402:9 */
#[cfg(feature="winapi_app")] pub const CTRY_LEBANON: i32 = 0x3c1i32; /* Integer(961, Yes, Unknown) */ /* winnls.h:403:9, winnls.h:403:9, winnls.h:403:9 */
#[cfg(feature="winapi_app")] pub const CTRY_LIBYA: i32 = 0xdai32; /* Integer(218, Yes, Unknown) */ /* winnls.h:404:9, winnls.h:404:9, winnls.h:404:9 */
#[cfg(feature="winapi_app")] pub const CTRY_LIECHTENSTEIN: i32 = 0x29i32; /* Integer(41, Yes, Unknown) */ /* winnls.h:405:9, winnls.h:405:9, winnls.h:405:9 */
#[cfg(feature="winapi_app")] pub const CTRY_LITHUANIA: i32 = 0x172i32; /* Integer(370, Yes, Unknown) */ /* winnls.h:406:9, winnls.h:406:9, winnls.h:406:9 */
#[cfg(feature="winapi_app")] pub const CTRY_LUXEMBOURG: i32 = 0x160i32; /* Integer(352, Yes, Unknown) */ /* winnls.h:407:9, winnls.h:407:9, winnls.h:407:9 */
#[cfg(feature="winapi_app")] pub const CTRY_MACAU: i32 = 0x355i32; /* Integer(853, Yes, Unknown) */ /* winnls.h:408:9, winnls.h:408:9, winnls.h:408:9 */
#[cfg(feature="winapi_app")] pub const CTRY_MACEDONIA: i32 = 0x185i32; /* Integer(389, Yes, Unknown) */ /* winnls.h:409:9, winnls.h:409:9, winnls.h:409:9 */
#[cfg(feature="winapi_app")] pub const CTRY_MALAYSIA: i32 = 0x3ci32; /* Integer(60, Yes, Unknown) */ /* winnls.h:410:9, winnls.h:410:9, winnls.h:410:9 */
#[cfg(feature="winapi_app")] pub const CTRY_MALDIVES: i32 = 0x3c0i32; /* Integer(960, Yes, Unknown) */ /* winnls.h:411:9, winnls.h:411:9, winnls.h:411:9 */
#[cfg(feature="winapi_app")] pub const CTRY_MEXICO: i32 = 0x34i32; /* Integer(52, Yes, Unknown) */ /* winnls.h:412:9, winnls.h:412:9, winnls.h:412:9 */
#[cfg(feature="winapi_app")] pub const CTRY_MONACO: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* winnls.h:413:9, winnls.h:413:9, winnls.h:413:9 */
#[cfg(feature="winapi_app")] pub const CTRY_MONGOLIA: i32 = 0x3d0i32; /* Integer(976, Yes, Unknown) */ /* winnls.h:414:9, winnls.h:414:9, winnls.h:414:9 */
#[cfg(feature="winapi_app")] pub const CTRY_MOROCCO: i32 = 0xd4i32; /* Integer(212, Yes, Unknown) */ /* winnls.h:415:9, winnls.h:415:9, winnls.h:415:9 */
#[cfg(feature="winapi_app")] pub const CTRY_NETHERLANDS: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* winnls.h:416:9, winnls.h:416:9, winnls.h:416:9 */
#[cfg(feature="winapi_app")] pub const CTRY_NEW_ZEALAND: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnls.h:417:9, winnls.h:417:9, winnls.h:417:9 */
#[cfg(feature="winapi_app")] pub const CTRY_NICARAGUA: i32 = 0x1f9i32; /* Integer(505, Yes, Unknown) */ /* winnls.h:418:9, winnls.h:418:9, winnls.h:418:9 */
#[cfg(feature="winapi_app")] pub const CTRY_NORWAY: i32 = 0x2fi32; /* Integer(47, Yes, Unknown) */ /* winnls.h:419:9, winnls.h:419:9, winnls.h:419:9 */
#[cfg(feature="winapi_app")] pub const CTRY_OMAN: i32 = 0x3c8i32; /* Integer(968, Yes, Unknown) */ /* winnls.h:420:9, winnls.h:420:9, winnls.h:420:9 */
#[cfg(feature="winapi_app")] pub const CTRY_PAKISTAN: i32 = 0x5ci32; /* Integer(92, Yes, Unknown) */ /* winnls.h:421:9, winnls.h:421:9, winnls.h:421:9 */
#[cfg(feature="winapi_app")] pub const CTRY_PANAMA: i32 = 0x1fbi32; /* Integer(507, Yes, Unknown) */ /* winnls.h:422:9, winnls.h:422:9, winnls.h:422:9 */
#[cfg(feature="winapi_app")] pub const CTRY_PARAGUAY: i32 = 0x253i32; /* Integer(595, Yes, Unknown) */ /* winnls.h:423:9, winnls.h:423:9, winnls.h:423:9 */
#[cfg(feature="winapi_app")] pub const CTRY_PERU: i32 = 0x33i32; /* Integer(51, Yes, Unknown) */ /* winnls.h:424:9, winnls.h:424:9, winnls.h:424:9 */
#[cfg(feature="winapi_app")] pub const CTRY_PHILIPPINES: i32 = 0x3fi32; /* Integer(63, Yes, Unknown) */ /* winnls.h:425:9, winnls.h:425:9, winnls.h:425:9 */
#[cfg(feature="winapi_app")] pub const CTRY_POLAND: i32 = 0x30i32; /* Integer(48, Yes, Unknown) */ /* winnls.h:426:9, winnls.h:426:9, winnls.h:426:9 */
#[cfg(feature="winapi_app")] pub const CTRY_PORTUGAL: i32 = 0x15fi32; /* Integer(351, Yes, Unknown) */ /* winnls.h:427:9, winnls.h:427:9, winnls.h:427:9 */
#[cfg(feature="winapi_app")] pub const CTRY_PRCHINA: i32 = 0x56i32; /* Integer(86, Yes, Unknown) */ /* winnls.h:428:9, winnls.h:428:9, winnls.h:428:9 */
#[cfg(feature="winapi_app")] pub const CTRY_PUERTO_RICO: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:429:9, winnls.h:429:9, winnls.h:429:9 */
#[cfg(feature="winapi_app")] pub const CTRY_QATAR: i32 = 0x3cei32; /* Integer(974, Yes, Unknown) */ /* winnls.h:430:9, winnls.h:430:9, winnls.h:430:9 */
#[cfg(feature="winapi_app")] pub const CTRY_ROMANIA: i32 = 0x28i32; /* Integer(40, Yes, Unknown) */ /* winnls.h:431:9, winnls.h:431:9, winnls.h:431:9 */
#[cfg(feature="winapi_app")] pub const CTRY_RUSSIA: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnls.h:432:9, winnls.h:432:9, winnls.h:432:9 */
#[cfg(feature="winapi_app")] pub const CTRY_SAUDI_ARABIA: i32 = 0x3c6i32; /* Integer(966, Yes, Unknown) */ /* winnls.h:433:9, winnls.h:433:9, winnls.h:433:9 */
#[cfg(feature="winapi_app")] pub const CTRY_SERBIA: i32 = 0x17di32; /* Integer(381, Yes, Unknown) */ /* winnls.h:434:9, winnls.h:434:9, winnls.h:434:9 */
#[cfg(feature="winapi_app")] pub const CTRY_SINGAPORE: i32 = 0x41i32; /* Integer(65, Yes, Unknown) */ /* winnls.h:435:9, winnls.h:435:9, winnls.h:435:9 */
#[cfg(feature="winapi_app")] pub const CTRY_SLOVAK: i32 = 0x1a5i32; /* Integer(421, Yes, Unknown) */ /* winnls.h:436:9, winnls.h:436:9, winnls.h:436:9 */
#[cfg(feature="winapi_app")] pub const CTRY_SLOVENIA: i32 = 0x182i32; /* Integer(386, Yes, Unknown) */ /* winnls.h:437:9, winnls.h:437:9, winnls.h:437:9 */
#[cfg(feature="winapi_app")] pub const CTRY_SOUTH_AFRICA: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* winnls.h:438:9, winnls.h:438:9, winnls.h:438:9 */
#[cfg(feature="winapi_app")] pub const CTRY_SOUTH_KOREA: i32 = 0x52i32; /* Integer(82, Yes, Unknown) */ /* winnls.h:439:9, winnls.h:439:9, winnls.h:439:9 */
#[cfg(feature="winapi_app")] pub const CTRY_SPAIN: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* winnls.h:440:9, winnls.h:440:9, winnls.h:440:9 */
#[cfg(feature="winapi_app")] pub const CTRY_SWEDEN: i32 = 0x2ei32; /* Integer(46, Yes, Unknown) */ /* winnls.h:441:9, winnls.h:441:9, winnls.h:441:9 */
#[cfg(feature="winapi_app")] pub const CTRY_SWITZERLAND: i32 = 0x29i32; /* Integer(41, Yes, Unknown) */ /* winnls.h:442:9, winnls.h:442:9, winnls.h:442:9 */
#[cfg(feature="winapi_app")] pub const CTRY_SYRIA: i32 = 0x3c3i32; /* Integer(963, Yes, Unknown) */ /* winnls.h:443:9, winnls.h:443:9, winnls.h:443:9 */
#[cfg(feature="winapi_app")] pub const CTRY_TAIWAN: i32 = 0x376i32; /* Integer(886, Yes, Unknown) */ /* winnls.h:444:9, winnls.h:444:9, winnls.h:444:9 */
#[cfg(feature="winapi_app")] pub const CTRY_TATARSTAN: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnls.h:445:9, winnls.h:445:9, winnls.h:445:9 */
#[cfg(feature="winapi_app")] pub const CTRY_THAILAND: i32 = 0x42i32; /* Integer(66, Yes, Unknown) */ /* winnls.h:446:9, winnls.h:446:9, winnls.h:446:9 */
#[cfg(feature="winapi_app")] pub const CTRY_TRINIDAD_Y_TOBAGO: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:447:9, winnls.h:447:9, winnls.h:447:9 */
#[cfg(feature="winapi_app")] pub const CTRY_TUNISIA: i32 = 0xd8i32; /* Integer(216, Yes, Unknown) */ /* winnls.h:448:9, winnls.h:448:9, winnls.h:448:9 */
#[cfg(feature="winapi_app")] pub const CTRY_TURKEY: i32 = 0x5ai32; /* Integer(90, Yes, Unknown) */ /* winnls.h:449:9, winnls.h:449:9, winnls.h:449:9 */
#[cfg(feature="winapi_app")] pub const CTRY_UAE: i32 = 0x3cbi32; /* Integer(971, Yes, Unknown) */ /* winnls.h:450:9, winnls.h:450:9, winnls.h:450:9 */
#[cfg(feature="winapi_app")] pub const CTRY_UKRAINE: i32 = 0x17ci32; /* Integer(380, Yes, Unknown) */ /* winnls.h:451:9, winnls.h:451:9, winnls.h:451:9 */
#[cfg(feature="winapi_app")] pub const CTRY_UNITED_KINGDOM: i32 = 0x2ci32; /* Integer(44, Yes, Unknown) */ /* winnls.h:452:9, winnls.h:452:9, winnls.h:452:9 */
#[cfg(feature="winapi_app")] pub const CTRY_UNITED_STATES: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:453:9, winnls.h:453:9, winnls.h:453:9 */
#[cfg(feature="winapi_app")] pub const CTRY_URUGUAY: i32 = 0x256i32; /* Integer(598, Yes, Unknown) */ /* winnls.h:454:9, winnls.h:454:9, winnls.h:454:9 */
#[cfg(feature="winapi_app")] pub const CTRY_UZBEKISTAN: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnls.h:455:9, winnls.h:455:9, winnls.h:455:9 */
#[cfg(feature="winapi_app")] pub const CTRY_VENEZUELA: i32 = 0x3ai32; /* Integer(58, Yes, Unknown) */ /* winnls.h:456:9, winnls.h:456:9, winnls.h:456:9 */
#[cfg(feature="winapi_app")] pub const CTRY_VIET_NAM: i32 = 0x54i32; /* Integer(84, Yes, Unknown) */ /* winnls.h:457:9, winnls.h:457:9, winnls.h:457:9 */
#[cfg(feature="winapi_app")] pub const CTRY_YEMEN: i32 = 0x3c7i32; /* Integer(967, Yes, Unknown) */ /* winnls.h:458:9, winnls.h:458:9, winnls.h:458:9 */
#[cfg(feature="winapi_app")] pub const CTRY_ZIMBABWE: i32 = 0x107i32; /* Integer(263, Yes, Unknown) */ /* winnls.h:459:9, winnls.h:459:9, winnls.h:459:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_NOUSEROVERRIDE: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* winnls.h:482:9, winnls.h:482:9, winnls.h:482:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_USE_CP_ACP: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* winnls.h:483:9, winnls.h:483:9, winnls.h:483:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const LOCALE_RETURN_NUMBER: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* winnls.h:486:9, winnls.h:486:9, winnls.h:486:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_RETURN_GENITIVE_NAMES: i32 = 0x10000000i32; /* Integer(268435456, Yes, Unknown) */ /* winnls.h:490:9, winnls.h:490:9, winnls.h:490:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_ALLOW_NEUTRAL_NAMES: i32 = 0x8000000i32; /* Integer(134217728, Yes, Unknown) */ /* winnls.h:491:9, winnls.h:491:9, winnls.h:491:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SLOCALIZEDDISPLAYNAME: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:502:9, winnls.h:502:9, winnls.h:502:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_SENGLISHDISPLAYNAME: i32 = 0x72i32; /* Integer(114, Yes, Unknown) */ /* winnls.h:504:9, winnls.h:504:9, winnls.h:504:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_SNATIVEDISPLAYNAME: i32 = 0x73i32; /* Integer(115, Yes, Unknown) */ /* winnls.h:505:9, winnls.h:505:9, winnls.h:505:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SLOCALIZEDLANGUAGENAME: i32 = 0x6fi32; /* Integer(111, Yes, Unknown) */ /* winnls.h:509:9, winnls.h:509:9, winnls.h:509:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SENGLISHLANGUAGENAME: i32 = 0x1001i32; /* Integer(4097, Yes, Unknown) */ /* winnls.h:511:9, winnls.h:511:9, winnls.h:511:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SNATIVELANGUAGENAME: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:512:9, winnls.h:512:9, winnls.h:512:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SLOCALIZEDCOUNTRYNAME: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnls.h:514:9, winnls.h:514:9, winnls.h:514:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SENGLISHCOUNTRYNAME: i32 = 0x1002i32; /* Integer(4098, Yes, Unknown) */ /* winnls.h:515:9, winnls.h:515:9, winnls.h:515:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SNATIVECOUNTRYNAME: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:516:9, winnls.h:516:9, winnls.h:516:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SLANGUAGE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:521:9, winnls.h:521:9, winnls.h:521:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SLANGDISPLAYNAME: i32 = 0x6fi32; /* Integer(111, Yes, Unknown) */ /* winnls.h:523:9, winnls.h:523:9, winnls.h:523:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SENGLANGUAGE: i32 = 0x1001i32; /* Integer(4097, Yes, Unknown) */ /* winnls.h:525:9, winnls.h:525:9, winnls.h:525:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SNATIVELANGNAME: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:526:9, winnls.h:526:9, winnls.h:526:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SCOUNTRY: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnls.h:527:9, winnls.h:527:9, winnls.h:527:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SENGCOUNTRY: i32 = 0x1002i32; /* Integer(4098, Yes, Unknown) */ /* winnls.h:528:9, winnls.h:528:9, winnls.h:528:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SNATIVECTRYNAME: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:529:9, winnls.h:529:9, winnls.h:529:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_ILANGUAGE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:532:9, winnls.h:532:9, winnls.h:532:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVLANGNAME: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnls.h:534:9, winnls.h:534:9, winnls.h:534:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_ICOUNTRY: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnls.h:536:9, winnls.h:536:9, winnls.h:536:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVCTRYNAME: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnls.h:537:9, winnls.h:537:9, winnls.h:537:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IGEOID: i32 = 0x5bi32; /* Integer(91, Yes, Unknown) */ /* winnls.h:538:9, winnls.h:538:9, winnls.h:538:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IDEFAULTLANGUAGE: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnls.h:540:9, winnls.h:540:9, winnls.h:540:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IDEFAULTCOUNTRY: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnls.h:541:9, winnls.h:541:9, winnls.h:541:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IDEFAULTCODEPAGE: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnls.h:542:9, winnls.h:542:9, winnls.h:542:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IDEFAULTANSICODEPAGE: i32 = 0x1004i32; /* Integer(4100, Yes, Unknown) */ /* winnls.h:543:9, winnls.h:543:9, winnls.h:543:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IDEFAULTMACCODEPAGE: i32 = 0x1011i32; /* Integer(4113, Yes, Unknown) */ /* winnls.h:544:9, winnls.h:544:9, winnls.h:544:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SLIST: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnls.h:546:9, winnls.h:546:9, winnls.h:546:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IMEASURE: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnls.h:547:9, winnls.h:547:9, winnls.h:547:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SDECIMAL: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnls.h:549:9, winnls.h:549:9, winnls.h:549:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_STHOUSAND: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnls.h:550:9, winnls.h:550:9, winnls.h:550:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SGROUPING: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnls.h:551:9, winnls.h:551:9, winnls.h:551:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IDIGITS: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winnls.h:552:9, winnls.h:552:9, winnls.h:552:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_ILZERO: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnls.h:553:9, winnls.h:553:9, winnls.h:553:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_INEGNUMBER: i32 = 0x1010i32; /* Integer(4112, Yes, Unknown) */ /* winnls.h:554:9, winnls.h:554:9, winnls.h:554:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SNATIVEDIGITS: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winnls.h:555:9, winnls.h:555:9, winnls.h:555:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SCURRENCY: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnls.h:557:9, winnls.h:557:9, winnls.h:557:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SINTLSYMBOL: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winnls.h:558:9, winnls.h:558:9, winnls.h:558:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONDECIMALSEP: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winnls.h:559:9, winnls.h:559:9, winnls.h:559:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONTHOUSANDSEP: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winnls.h:560:9, winnls.h:560:9, winnls.h:560:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONGROUPING: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winnls.h:561:9, winnls.h:561:9, winnls.h:561:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_ICURRDIGITS: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* winnls.h:562:9, winnls.h:562:9, winnls.h:562:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IINTLCURRDIGITS: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* winnls.h:563:9, winnls.h:563:9, winnls.h:563:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_ICURRENCY: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* winnls.h:564:9, winnls.h:564:9, winnls.h:564:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_INEGCURR: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* winnls.h:565:9, winnls.h:565:9, winnls.h:565:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SDATE: i32 = 0x1di32; /* Integer(29, Yes, Unknown) */ /* winnls.h:567:9, winnls.h:567:9, winnls.h:567:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_STIME: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* winnls.h:568:9, winnls.h:568:9, winnls.h:568:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SSHORTDATE: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* winnls.h:569:9, winnls.h:569:9, winnls.h:569:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SLONGDATE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnls.h:570:9, winnls.h:570:9, winnls.h:570:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_STIMEFORMAT: i32 = 0x1003i32; /* Integer(4099, Yes, Unknown) */ /* winnls.h:571:9, winnls.h:571:9, winnls.h:571:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IDATE: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* winnls.h:572:9, winnls.h:572:9, winnls.h:572:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_ILDATE: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* winnls.h:573:9, winnls.h:573:9, winnls.h:573:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_ITIME: i32 = 0x23i32; /* Integer(35, Yes, Unknown) */ /* winnls.h:574:9, winnls.h:574:9, winnls.h:574:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_ITIMEMARKPOSN: i32 = 0x1005i32; /* Integer(4101, Yes, Unknown) */ /* winnls.h:575:9, winnls.h:575:9, winnls.h:575:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_ICENTURY: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* winnls.h:576:9, winnls.h:576:9, winnls.h:576:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_ITLZERO: i32 = 0x25i32; /* Integer(37, Yes, Unknown) */ /* winnls.h:577:9, winnls.h:577:9, winnls.h:577:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IDAYLZERO: i32 = 0x26i32; /* Integer(38, Yes, Unknown) */ /* winnls.h:578:9, winnls.h:578:9, winnls.h:578:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IMONLZERO: i32 = 0x27i32; /* Integer(39, Yes, Unknown) */ /* winnls.h:579:9, winnls.h:579:9, winnls.h:579:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_S1159: i32 = 0x28i32; /* Integer(40, Yes, Unknown) */ /* winnls.h:580:9, winnls.h:580:9, winnls.h:580:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_S2359: i32 = 0x29i32; /* Integer(41, Yes, Unknown) */ /* winnls.h:581:9, winnls.h:581:9, winnls.h:581:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_ICALENDARTYPE: i32 = 0x1009i32; /* Integer(4105, Yes, Unknown) */ /* winnls.h:583:9, winnls.h:583:9, winnls.h:583:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IOPTIONALCALENDAR: i32 = 0x100bi32; /* Integer(4107, Yes, Unknown) */ /* winnls.h:584:9, winnls.h:584:9, winnls.h:584:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IFIRSTDAYOFWEEK: i32 = 0x100ci32; /* Integer(4108, Yes, Unknown) */ /* winnls.h:585:9, winnls.h:585:9, winnls.h:585:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IFIRSTWEEKOFYEAR: i32 = 0x100di32; /* Integer(4109, Yes, Unknown) */ /* winnls.h:586:9, winnls.h:586:9, winnls.h:586:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SDAYNAME1: i32 = 0x2ai32; /* Integer(42, Yes, Unknown) */ /* winnls.h:588:9, winnls.h:588:9, winnls.h:588:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SDAYNAME2: i32 = 0x2bi32; /* Integer(43, Yes, Unknown) */ /* winnls.h:589:9, winnls.h:589:9, winnls.h:589:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SDAYNAME3: i32 = 0x2ci32; /* Integer(44, Yes, Unknown) */ /* winnls.h:590:9, winnls.h:590:9, winnls.h:590:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SDAYNAME4: i32 = 0x2di32; /* Integer(45, Yes, Unknown) */ /* winnls.h:591:9, winnls.h:591:9, winnls.h:591:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SDAYNAME5: i32 = 0x2ei32; /* Integer(46, Yes, Unknown) */ /* winnls.h:592:9, winnls.h:592:9, winnls.h:592:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SDAYNAME6: i32 = 0x2fi32; /* Integer(47, Yes, Unknown) */ /* winnls.h:593:9, winnls.h:593:9, winnls.h:593:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SDAYNAME7: i32 = 0x30i32; /* Integer(48, Yes, Unknown) */ /* winnls.h:594:9, winnls.h:594:9, winnls.h:594:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVDAYNAME1: i32 = 0x31i32; /* Integer(49, Yes, Unknown) */ /* winnls.h:595:9, winnls.h:595:9, winnls.h:595:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVDAYNAME2: i32 = 0x32i32; /* Integer(50, Yes, Unknown) */ /* winnls.h:596:9, winnls.h:596:9, winnls.h:596:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVDAYNAME3: i32 = 0x33i32; /* Integer(51, Yes, Unknown) */ /* winnls.h:597:9, winnls.h:597:9, winnls.h:597:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVDAYNAME4: i32 = 0x34i32; /* Integer(52, Yes, Unknown) */ /* winnls.h:598:9, winnls.h:598:9, winnls.h:598:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVDAYNAME5: i32 = 0x35i32; /* Integer(53, Yes, Unknown) */ /* winnls.h:599:9, winnls.h:599:9, winnls.h:599:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVDAYNAME6: i32 = 0x36i32; /* Integer(54, Yes, Unknown) */ /* winnls.h:600:9, winnls.h:600:9, winnls.h:600:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVDAYNAME7: i32 = 0x37i32; /* Integer(55, Yes, Unknown) */ /* winnls.h:601:9, winnls.h:601:9, winnls.h:601:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONTHNAME1: i32 = 0x38i32; /* Integer(56, Yes, Unknown) */ /* winnls.h:602:9, winnls.h:602:9, winnls.h:602:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONTHNAME2: i32 = 0x39i32; /* Integer(57, Yes, Unknown) */ /* winnls.h:603:9, winnls.h:603:9, winnls.h:603:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONTHNAME3: i32 = 0x3ai32; /* Integer(58, Yes, Unknown) */ /* winnls.h:604:9, winnls.h:604:9, winnls.h:604:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONTHNAME4: i32 = 0x3bi32; /* Integer(59, Yes, Unknown) */ /* winnls.h:605:9, winnls.h:605:9, winnls.h:605:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONTHNAME5: i32 = 0x3ci32; /* Integer(60, Yes, Unknown) */ /* winnls.h:606:9, winnls.h:606:9, winnls.h:606:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONTHNAME6: i32 = 0x3di32; /* Integer(61, Yes, Unknown) */ /* winnls.h:607:9, winnls.h:607:9, winnls.h:607:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONTHNAME7: i32 = 0x3ei32; /* Integer(62, Yes, Unknown) */ /* winnls.h:608:9, winnls.h:608:9, winnls.h:608:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONTHNAME8: i32 = 0x3fi32; /* Integer(63, Yes, Unknown) */ /* winnls.h:609:9, winnls.h:609:9, winnls.h:609:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONTHNAME9: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnls.h:610:9, winnls.h:610:9, winnls.h:610:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONTHNAME10: i32 = 0x41i32; /* Integer(65, Yes, Unknown) */ /* winnls.h:611:9, winnls.h:611:9, winnls.h:611:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONTHNAME11: i32 = 0x42i32; /* Integer(66, Yes, Unknown) */ /* winnls.h:612:9, winnls.h:612:9, winnls.h:612:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONTHNAME12: i32 = 0x43i32; /* Integer(67, Yes, Unknown) */ /* winnls.h:613:9, winnls.h:613:9, winnls.h:613:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SMONTHNAME13: i32 = 0x100ei32; /* Integer(4110, Yes, Unknown) */ /* winnls.h:614:9, winnls.h:614:9, winnls.h:614:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVMONTHNAME1: i32 = 0x44i32; /* Integer(68, Yes, Unknown) */ /* winnls.h:615:9, winnls.h:615:9, winnls.h:615:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVMONTHNAME2: i32 = 0x45i32; /* Integer(69, Yes, Unknown) */ /* winnls.h:616:9, winnls.h:616:9, winnls.h:616:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVMONTHNAME3: i32 = 0x46i32; /* Integer(70, Yes, Unknown) */ /* winnls.h:617:9, winnls.h:617:9, winnls.h:617:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVMONTHNAME4: i32 = 0x47i32; /* Integer(71, Yes, Unknown) */ /* winnls.h:618:9, winnls.h:618:9, winnls.h:618:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVMONTHNAME5: i32 = 0x48i32; /* Integer(72, Yes, Unknown) */ /* winnls.h:619:9, winnls.h:619:9, winnls.h:619:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVMONTHNAME6: i32 = 0x49i32; /* Integer(73, Yes, Unknown) */ /* winnls.h:620:9, winnls.h:620:9, winnls.h:620:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVMONTHNAME7: i32 = 0x4ai32; /* Integer(74, Yes, Unknown) */ /* winnls.h:621:9, winnls.h:621:9, winnls.h:621:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVMONTHNAME8: i32 = 0x4bi32; /* Integer(75, Yes, Unknown) */ /* winnls.h:622:9, winnls.h:622:9, winnls.h:622:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVMONTHNAME9: i32 = 0x4ci32; /* Integer(76, Yes, Unknown) */ /* winnls.h:623:9, winnls.h:623:9, winnls.h:623:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVMONTHNAME10: i32 = 0x4di32; /* Integer(77, Yes, Unknown) */ /* winnls.h:624:9, winnls.h:624:9, winnls.h:624:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVMONTHNAME11: i32 = 0x4ei32; /* Integer(78, Yes, Unknown) */ /* winnls.h:625:9, winnls.h:625:9, winnls.h:625:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVMONTHNAME12: i32 = 0x4fi32; /* Integer(79, Yes, Unknown) */ /* winnls.h:626:9, winnls.h:626:9, winnls.h:626:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SABBREVMONTHNAME13: i32 = 0x100fi32; /* Integer(4111, Yes, Unknown) */ /* winnls.h:627:9, winnls.h:627:9, winnls.h:627:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SPOSITIVESIGN: i32 = 0x50i32; /* Integer(80, Yes, Unknown) */ /* winnls.h:629:9, winnls.h:629:9, winnls.h:629:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_SNEGATIVESIGN: i32 = 0x51i32; /* Integer(81, Yes, Unknown) */ /* winnls.h:630:9, winnls.h:630:9, winnls.h:630:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IPOSSIGNPOSN: i32 = 0x52i32; /* Integer(82, Yes, Unknown) */ /* winnls.h:631:9, winnls.h:631:9, winnls.h:631:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_INEGSIGNPOSN: i32 = 0x53i32; /* Integer(83, Yes, Unknown) */ /* winnls.h:632:9, winnls.h:632:9, winnls.h:632:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IPOSSYMPRECEDES: i32 = 0x54i32; /* Integer(84, Yes, Unknown) */ /* winnls.h:633:9, winnls.h:633:9, winnls.h:633:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_IPOSSEPBYSPACE: i32 = 0x55i32; /* Integer(85, Yes, Unknown) */ /* winnls.h:634:9, winnls.h:634:9, winnls.h:634:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_INEGSYMPRECEDES: i32 = 0x56i32; /* Integer(86, Yes, Unknown) */ /* winnls.h:635:9, winnls.h:635:9, winnls.h:635:9 */
#[cfg(feature="winapi_app")] pub const LOCALE_INEGSEPBYSPACE: i32 = 0x57i32; /* Integer(87, Yes, Unknown) */ /* winnls.h:636:9, winnls.h:636:9, winnls.h:636:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const LOCALE_FONTSIGNATURE: i32 = 0x58i32; /* Integer(88, Yes, Unknown) */ /* winnls.h:639:9, winnls.h:639:9, winnls.h:639:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const LOCALE_SISO639LANGNAME: i32 = 0x59i32; /* Integer(89, Yes, Unknown) */ /* winnls.h:640:9, winnls.h:640:9, winnls.h:640:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const LOCALE_SISO3166CTRYNAME: i32 = 0x5ai32; /* Integer(90, Yes, Unknown) */ /* winnls.h:641:9, winnls.h:641:9, winnls.h:641:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub const LOCALE_IDEFAULTEBCDICCODEPAGE: i32 = 0x1012i32; /* Integer(4114, Yes, Unknown) */ /* winnls.h:645:9, winnls.h:645:9, winnls.h:645:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub const LOCALE_IPAPERSIZE: i32 = 0x100ai32; /* Integer(4106, Yes, Unknown) */ /* winnls.h:646:9, winnls.h:646:9, winnls.h:646:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub const LOCALE_SENGCURRNAME: i32 = 0x1007i32; /* Integer(4103, Yes, Unknown) */ /* winnls.h:647:9, winnls.h:647:9, winnls.h:647:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub const LOCALE_SNATIVECURRNAME: i32 = 0x1008i32; /* Integer(4104, Yes, Unknown) */ /* winnls.h:648:9, winnls.h:648:9, winnls.h:648:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub const LOCALE_SYEARMONTH: i32 = 0x1006i32; /* Integer(4102, Yes, Unknown) */ /* winnls.h:649:9, winnls.h:649:9, winnls.h:649:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub const LOCALE_SSORTNAME: i32 = 0x1013i32; /* Integer(4115, Yes, Unknown) */ /* winnls.h:650:9, winnls.h:650:9, winnls.h:650:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub const LOCALE_IDIGITSUBSTITUTION: i32 = 0x1014i32; /* Integer(4116, Yes, Unknown) */ /* winnls.h:651:9, winnls.h:651:9, winnls.h:651:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SNAME: i32 = 0x5ci32; /* Integer(92, Yes, Unknown) */ /* winnls.h:656:9, winnls.h:656:9, winnls.h:656:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SDURATION: i32 = 0x5di32; /* Integer(93, Yes, Unknown) */ /* winnls.h:657:9, winnls.h:657:9, winnls.h:657:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SKEYBOARDSTOINSTALL: i32 = 0x5ei32; /* Integer(94, Yes, Unknown) */ /* winnls.h:658:9, winnls.h:658:9, winnls.h:658:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SSHORTESTDAYNAME1: i32 = 0x60i32; /* Integer(96, Yes, Unknown) */ /* winnls.h:659:9, winnls.h:659:9, winnls.h:659:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SSHORTESTDAYNAME2: i32 = 0x61i32; /* Integer(97, Yes, Unknown) */ /* winnls.h:660:9, winnls.h:660:9, winnls.h:660:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SSHORTESTDAYNAME3: i32 = 0x62i32; /* Integer(98, Yes, Unknown) */ /* winnls.h:661:9, winnls.h:661:9, winnls.h:661:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SSHORTESTDAYNAME4: i32 = 0x63i32; /* Integer(99, Yes, Unknown) */ /* winnls.h:662:9, winnls.h:662:9, winnls.h:662:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SSHORTESTDAYNAME5: i32 = 0x64i32; /* Integer(100, Yes, Unknown) */ /* winnls.h:663:9, winnls.h:663:9, winnls.h:663:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SSHORTESTDAYNAME6: i32 = 0x65i32; /* Integer(101, Yes, Unknown) */ /* winnls.h:664:9, winnls.h:664:9, winnls.h:664:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SSHORTESTDAYNAME7: i32 = 0x66i32; /* Integer(102, Yes, Unknown) */ /* winnls.h:665:9, winnls.h:665:9, winnls.h:665:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SISO639LANGNAME2: i32 = 0x67i32; /* Integer(103, Yes, Unknown) */ /* winnls.h:666:9, winnls.h:666:9, winnls.h:666:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SISO3166CTRYNAME2: i32 = 0x68i32; /* Integer(104, Yes, Unknown) */ /* winnls.h:667:9, winnls.h:667:9, winnls.h:667:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SNAN: i32 = 0x69i32; /* Integer(105, Yes, Unknown) */ /* winnls.h:668:9, winnls.h:668:9, winnls.h:668:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SPOSINFINITY: i32 = 0x6ai32; /* Integer(106, Yes, Unknown) */ /* winnls.h:669:9, winnls.h:669:9, winnls.h:669:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SNEGINFINITY: i32 = 0x6bi32; /* Integer(107, Yes, Unknown) */ /* winnls.h:670:9, winnls.h:670:9, winnls.h:670:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SSCRIPTS: i32 = 0x6ci32; /* Integer(108, Yes, Unknown) */ /* winnls.h:671:9, winnls.h:671:9, winnls.h:671:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SPARENT: i32 = 0x6di32; /* Integer(109, Yes, Unknown) */ /* winnls.h:672:9, winnls.h:672:9, winnls.h:672:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_SCONSOLEFALLBACKNAME: i32 = 0x6ei32; /* Integer(110, Yes, Unknown) */ /* winnls.h:673:9, winnls.h:673:9, winnls.h:673:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_IREADINGLAYOUT: i32 = 0x70i32; /* Integer(112, Yes, Unknown) */ /* winnls.h:677:9, winnls.h:677:9, winnls.h:677:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_INEUTRAL: i32 = 0x71i32; /* Integer(113, Yes, Unknown) */ /* winnls.h:682:9, winnls.h:682:9, winnls.h:682:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_INEGATIVEPERCENT: i32 = 0x74i32; /* Integer(116, Yes, Unknown) */ /* winnls.h:683:9, winnls.h:683:9, winnls.h:683:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_IPOSITIVEPERCENT: i32 = 0x75i32; /* Integer(117, Yes, Unknown) */ /* winnls.h:684:9, winnls.h:684:9, winnls.h:684:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_SPERCENT: i32 = 0x76i32; /* Integer(118, Yes, Unknown) */ /* winnls.h:685:9, winnls.h:685:9, winnls.h:685:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_SPERMILLE: i32 = 0x77i32; /* Integer(119, Yes, Unknown) */ /* winnls.h:686:9, winnls.h:686:9, winnls.h:686:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_SMONTHDAY: i32 = 0x78i32; /* Integer(120, Yes, Unknown) */ /* winnls.h:687:9, winnls.h:687:9, winnls.h:687:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_SSHORTTIME: i32 = 0x79i32; /* Integer(121, Yes, Unknown) */ /* winnls.h:688:9, winnls.h:688:9, winnls.h:688:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_SOPENTYPELANGUAGETAG: i32 = 0x7ai32; /* Integer(122, Yes, Unknown) */ /* winnls.h:689:9, winnls.h:689:9, winnls.h:689:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const LOCALE_SSORTLOCALE: i32 = 0x7bi32; /* Integer(123, Yes, Unknown) */ /* winnls.h:690:9, winnls.h:690:9, winnls.h:690:9 */
#[cfg(feature="winapi_app")] pub const TIME_NOMINUTESORSECONDS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:701:9, winnls.h:701:9, winnls.h:701:9 */
#[cfg(feature="winapi_app")] pub const TIME_NOSECONDS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:702:9, winnls.h:702:9, winnls.h:702:9 */
#[cfg(feature="winapi_app")] pub const TIME_NOTIMEMARKER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:703:9, winnls.h:703:9, winnls.h:703:9 */
#[cfg(feature="winapi_app")] pub const TIME_FORCE24HOURFORMAT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:704:9, winnls.h:704:9, winnls.h:704:9 */
#[cfg(feature="winapi_app")] pub const DATE_SHORTDATE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:710:9, winnls.h:710:9, winnls.h:710:9 */
#[cfg(feature="winapi_app")] pub const DATE_LONGDATE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:711:9, winnls.h:711:9, winnls.h:711:9 */
#[cfg(feature="winapi_app")] pub const DATE_USE_ALT_CALENDAR: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:712:9, winnls.h:712:9, winnls.h:712:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub const DATE_YEARMONTH: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:715:9, winnls.h:715:9, winnls.h:715:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub const DATE_LTRREADING: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnls.h:716:9, winnls.h:716:9, winnls.h:716:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub const DATE_RTLREADING: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnls.h:717:9, winnls.h:717:9, winnls.h:717:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const DATE_AUTOLAYOUT: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnls.h:721:9, winnls.h:721:9, winnls.h:721:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use ::winnls::LOCALE_NOUSEROVERRIDE as CAL_NOUSEROVERRIDE; /* winnls.h:746:9, winnls.h:746:9, winnls.h:746:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use ::winnls::LOCALE_USE_CP_ACP as CAL_USE_CP_ACP; /* winnls.h:747:9, winnls.h:747:9, winnls.h:747:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use ::winnls::LOCALE_RETURN_NUMBER as CAL_RETURN_NUMBER; /* winnls.h:748:9, winnls.h:748:9, winnls.h:748:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[doc(inline)] pub use ::winnls::LOCALE_RETURN_GENITIVE_NAMES as CAL_RETURN_GENITIVE_NAMES; /* winnls.h:752:9, winnls.h:752:9, winnls.h:752:9 */
#[cfg(feature="winapi_app")] pub const CAL_ICALINTVALUE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:759:9, winnls.h:759:9, winnls.h:759:9 */
#[cfg(feature="winapi_app")] pub const CAL_SCALNAME: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:760:9, winnls.h:760:9, winnls.h:760:9 */
#[cfg(feature="winapi_app")] pub const CAL_IYEAROFFSETRANGE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnls.h:761:9, winnls.h:761:9, winnls.h:761:9 */
#[cfg(feature="winapi_app")] pub const CAL_SERASTRING: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:762:9, winnls.h:762:9, winnls.h:762:9 */
#[cfg(feature="winapi_app")] pub const CAL_SSHORTDATE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnls.h:763:9, winnls.h:763:9, winnls.h:763:9 */
#[cfg(feature="winapi_app")] pub const CAL_SLONGDATE: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnls.h:764:9, winnls.h:764:9, winnls.h:764:9 */
#[cfg(feature="winapi_app")] pub const CAL_SDAYNAME1: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnls.h:765:9, winnls.h:765:9, winnls.h:765:9 */
#[cfg(feature="winapi_app")] pub const CAL_SDAYNAME2: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:766:9, winnls.h:766:9, winnls.h:766:9 */
#[cfg(feature="winapi_app")] pub const CAL_SDAYNAME3: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnls.h:767:9, winnls.h:767:9, winnls.h:767:9 */
#[cfg(feature="winapi_app")] pub const CAL_SDAYNAME4: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnls.h:768:9, winnls.h:768:9, winnls.h:768:9 */
#[cfg(feature="winapi_app")] pub const CAL_SDAYNAME5: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnls.h:769:9, winnls.h:769:9, winnls.h:769:9 */
#[cfg(feature="winapi_app")] pub const CAL_SDAYNAME6: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnls.h:770:9, winnls.h:770:9, winnls.h:770:9 */
#[cfg(feature="winapi_app")] pub const CAL_SDAYNAME7: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnls.h:771:9, winnls.h:771:9, winnls.h:771:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVDAYNAME1: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnls.h:772:9, winnls.h:772:9, winnls.h:772:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVDAYNAME2: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnls.h:773:9, winnls.h:773:9, winnls.h:773:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVDAYNAME3: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnls.h:774:9, winnls.h:774:9, winnls.h:774:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVDAYNAME4: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winnls.h:775:9, winnls.h:775:9, winnls.h:775:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVDAYNAME5: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* winnls.h:776:9, winnls.h:776:9, winnls.h:776:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVDAYNAME6: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* winnls.h:777:9, winnls.h:777:9, winnls.h:777:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVDAYNAME7: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* winnls.h:778:9, winnls.h:778:9, winnls.h:778:9 */
#[cfg(feature="winapi_app")] pub const CAL_SMONTHNAME1: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* winnls.h:780:9, winnls.h:780:9, winnls.h:780:9 */
#[cfg(feature="winapi_app")] pub const CAL_SMONTHNAME2: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* winnls.h:781:9, winnls.h:781:9, winnls.h:781:9 */
#[cfg(feature="winapi_app")] pub const CAL_SMONTHNAME3: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winnls.h:782:9, winnls.h:782:9, winnls.h:782:9 */
#[cfg(feature="winapi_app")] pub const CAL_SMONTHNAME4: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* winnls.h:783:9, winnls.h:783:9, winnls.h:783:9 */
#[cfg(feature="winapi_app")] pub const CAL_SMONTHNAME5: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* winnls.h:784:9, winnls.h:784:9, winnls.h:784:9 */
#[cfg(feature="winapi_app")] pub const CAL_SMONTHNAME6: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* winnls.h:785:9, winnls.h:785:9, winnls.h:785:9 */
#[cfg(feature="winapi_app")] pub const CAL_SMONTHNAME7: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* winnls.h:786:9, winnls.h:786:9, winnls.h:786:9 */
#[cfg(feature="winapi_app")] pub const CAL_SMONTHNAME8: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* winnls.h:787:9, winnls.h:787:9, winnls.h:787:9 */
#[cfg(feature="winapi_app")] pub const CAL_SMONTHNAME9: i32 = 0x1di32; /* Integer(29, Yes, Unknown) */ /* winnls.h:788:9, winnls.h:788:9, winnls.h:788:9 */
#[cfg(feature="winapi_app")] pub const CAL_SMONTHNAME10: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* winnls.h:789:9, winnls.h:789:9, winnls.h:789:9 */
#[cfg(feature="winapi_app")] pub const CAL_SMONTHNAME11: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* winnls.h:790:9, winnls.h:790:9, winnls.h:790:9 */
#[cfg(feature="winapi_app")] pub const CAL_SMONTHNAME12: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnls.h:791:9, winnls.h:791:9, winnls.h:791:9 */
#[cfg(feature="winapi_app")] pub const CAL_SMONTHNAME13: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* winnls.h:792:9, winnls.h:792:9, winnls.h:792:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVMONTHNAME1: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* winnls.h:793:9, winnls.h:793:9, winnls.h:793:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVMONTHNAME2: i32 = 0x23i32; /* Integer(35, Yes, Unknown) */ /* winnls.h:794:9, winnls.h:794:9, winnls.h:794:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVMONTHNAME3: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* winnls.h:795:9, winnls.h:795:9, winnls.h:795:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVMONTHNAME4: i32 = 0x25i32; /* Integer(37, Yes, Unknown) */ /* winnls.h:796:9, winnls.h:796:9, winnls.h:796:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVMONTHNAME5: i32 = 0x26i32; /* Integer(38, Yes, Unknown) */ /* winnls.h:797:9, winnls.h:797:9, winnls.h:797:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVMONTHNAME6: i32 = 0x27i32; /* Integer(39, Yes, Unknown) */ /* winnls.h:798:9, winnls.h:798:9, winnls.h:798:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVMONTHNAME7: i32 = 0x28i32; /* Integer(40, Yes, Unknown) */ /* winnls.h:799:9, winnls.h:799:9, winnls.h:799:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVMONTHNAME8: i32 = 0x29i32; /* Integer(41, Yes, Unknown) */ /* winnls.h:800:9, winnls.h:800:9, winnls.h:800:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVMONTHNAME9: i32 = 0x2ai32; /* Integer(42, Yes, Unknown) */ /* winnls.h:801:9, winnls.h:801:9, winnls.h:801:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVMONTHNAME10: i32 = 0x2bi32; /* Integer(43, Yes, Unknown) */ /* winnls.h:802:9, winnls.h:802:9, winnls.h:802:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVMONTHNAME11: i32 = 0x2ci32; /* Integer(44, Yes, Unknown) */ /* winnls.h:803:9, winnls.h:803:9, winnls.h:803:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVMONTHNAME12: i32 = 0x2di32; /* Integer(45, Yes, Unknown) */ /* winnls.h:804:9, winnls.h:804:9, winnls.h:804:9 */
#[cfg(feature="winapi_app")] pub const CAL_SABBREVMONTHNAME13: i32 = 0x2ei32; /* Integer(46, Yes, Unknown) */ /* winnls.h:805:9, winnls.h:805:9, winnls.h:805:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub const CAL_SYEARMONTH: i32 = 0x2fi32; /* Integer(47, Yes, Unknown) */ /* winnls.h:808:9, winnls.h:808:9, winnls.h:808:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub const CAL_ITWODIGITYEARMAX: i32 = 0x30i32; /* Integer(48, Yes, Unknown) */ /* winnls.h:809:9, winnls.h:809:9, winnls.h:809:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const CAL_SSHORTESTDAYNAME1: i32 = 0x31i32; /* Integer(49, Yes, Unknown) */ /* winnls.h:813:9, winnls.h:813:9, winnls.h:813:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const CAL_SSHORTESTDAYNAME2: i32 = 0x32i32; /* Integer(50, Yes, Unknown) */ /* winnls.h:814:9, winnls.h:814:9, winnls.h:814:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const CAL_SSHORTESTDAYNAME3: i32 = 0x33i32; /* Integer(51, Yes, Unknown) */ /* winnls.h:815:9, winnls.h:815:9, winnls.h:815:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const CAL_SSHORTESTDAYNAME4: i32 = 0x34i32; /* Integer(52, Yes, Unknown) */ /* winnls.h:816:9, winnls.h:816:9, winnls.h:816:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const CAL_SSHORTESTDAYNAME5: i32 = 0x35i32; /* Integer(53, Yes, Unknown) */ /* winnls.h:817:9, winnls.h:817:9, winnls.h:817:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const CAL_SSHORTESTDAYNAME6: i32 = 0x36i32; /* Integer(54, Yes, Unknown) */ /* winnls.h:818:9, winnls.h:818:9, winnls.h:818:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const CAL_SSHORTESTDAYNAME7: i32 = 0x37i32; /* Integer(55, Yes, Unknown) */ /* winnls.h:819:9, winnls.h:819:9, winnls.h:819:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const CAL_SMONTHDAY: i32 = 0x38i32; /* Integer(56, Yes, Unknown) */ /* winnls.h:823:9, winnls.h:823:9, winnls.h:823:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const CAL_SABBREVERASTRING: i32 = 0x39i32; /* Integer(57, Yes, Unknown) */ /* winnls.h:824:9, winnls.h:824:9, winnls.h:824:9 */
#[cfg(feature="winapi_app")] pub const ENUM_ALL_CALENDARS: i32 = 0xffffffffi32; /* Integer(4294967295, Yes, Unknown) */ /* winnls.h:833:9, winnls.h:833:9, winnls.h:833:9 */
#[cfg(feature="winapi_app")] pub const CAL_GREGORIAN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:839:9, winnls.h:839:9, winnls.h:839:9 */
#[cfg(feature="winapi_app")] pub const CAL_GREGORIAN_US: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:840:9, winnls.h:840:9, winnls.h:840:9 */
#[cfg(feature="winapi_app")] pub const CAL_JAPAN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnls.h:841:9, winnls.h:841:9, winnls.h:841:9 */
#[cfg(feature="winapi_app")] pub const CAL_TAIWAN: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:842:9, winnls.h:842:9, winnls.h:842:9 */
#[cfg(feature="winapi_app")] pub const CAL_KOREA: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnls.h:843:9, winnls.h:843:9, winnls.h:843:9 */
#[cfg(feature="winapi_app")] pub const CAL_HIJRI: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnls.h:844:9, winnls.h:844:9, winnls.h:844:9 */
#[cfg(feature="winapi_app")] pub const CAL_THAI: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnls.h:845:9, winnls.h:845:9, winnls.h:845:9 */
#[cfg(feature="winapi_app")] pub const CAL_HEBREW: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:846:9, winnls.h:846:9, winnls.h:846:9 */
#[cfg(feature="winapi_app")] pub const CAL_GREGORIAN_ME_FRENCH: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnls.h:847:9, winnls.h:847:9, winnls.h:847:9 */
#[cfg(feature="winapi_app")] pub const CAL_GREGORIAN_ARABIC: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnls.h:848:9, winnls.h:848:9, winnls.h:848:9 */
#[cfg(feature="winapi_app")] pub const CAL_GREGORIAN_XLIT_ENGLISH: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnls.h:849:9, winnls.h:849:9, winnls.h:849:9 */
#[cfg(feature="winapi_app")] pub const CAL_GREGORIAN_XLIT_FRENCH: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnls.h:850:9, winnls.h:850:9, winnls.h:850:9 */
#[cfg(feature="winapi_app")] pub const CAL_UMALQURA: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* winnls.h:851:9, winnls.h:851:9, winnls.h:851:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_WESTERN_EUROPE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:857:9, winnls.h:857:9, winnls.h:857:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_CENTRAL_EUROPE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:858:9, winnls.h:858:9, winnls.h:858:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_BALTIC: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* winnls.h:859:9, winnls.h:859:9, winnls.h:859:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_GREEK: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:860:9, winnls.h:860:9, winnls.h:860:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_CYRILLIC: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* winnls.h:861:9, winnls.h:861:9, winnls.h:861:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_TURKIC: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnls.h:862:9, winnls.h:862:9, winnls.h:862:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_TURKISH: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* winnls.h:863:9, winnls.h:863:9, winnls.h:863:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_JAPANESE: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* winnls.h:864:9, winnls.h:864:9, winnls.h:864:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_KOREAN: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:865:9, winnls.h:865:9, winnls.h:865:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_TRADITIONAL_CHINESE: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* winnls.h:866:9, winnls.h:866:9, winnls.h:866:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_SIMPLIFIED_CHINESE: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* winnls.h:867:9, winnls.h:867:9, winnls.h:867:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_THAI: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* winnls.h:868:9, winnls.h:868:9, winnls.h:868:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_HEBREW: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* winnls.h:869:9, winnls.h:869:9, winnls.h:869:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_ARABIC: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* winnls.h:870:9, winnls.h:870:9, winnls.h:870:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_VIETNAMESE: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* winnls.h:871:9, winnls.h:871:9, winnls.h:871:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_INDIC: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* winnls.h:872:9, winnls.h:872:9, winnls.h:872:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_GEORGIAN: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnls.h:873:9, winnls.h:873:9, winnls.h:873:9 */
#[cfg(feature="winapi_app")] pub const LGRPID_ARMENIAN: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* winnls.h:874:9, winnls.h:874:9, winnls.h:874:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_LANGUAGE_ID: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:881:9, winnls.h:881:9, winnls.h:881:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_LANGUAGE_NAME: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:882:9, winnls.h:882:9, winnls.h:882:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_MERGE_SYSTEM_FALLBACK: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnls.h:883:9, winnls.h:883:9, winnls.h:883:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_MERGE_USER_FALLBACK: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnls.h:884:9, winnls.h:884:9, winnls.h:884:9 */
/* #[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] // #define MUI_UI_FALLBACK Binary(BitOr, Ident("MUI_MERGE_SYSTEM_FALLBACK"), Ident("MUI_MERGE_USER_FALLBACK")) */ /* winnls.h:885:9, winnls.h:885:9, winnls.h:885:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_THREAD_LANGUAGES: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnls.h:886:9, winnls.h:886:9, winnls.h:886:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_CONSOLE_FILTER: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnls.h:887:9, winnls.h:887:9, winnls.h:887:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_COMPLEX_SCRIPT_FILTER: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnls.h:888:9, winnls.h:888:9, winnls.h:888:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_RESET_FILTERS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:889:9, winnls.h:889:9, winnls.h:889:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_USER_PREFERRED_UI_LANGUAGES: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnls.h:890:9, winnls.h:890:9, winnls.h:890:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_USE_INSTALLED_LANGUAGES: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnls.h:891:9, winnls.h:891:9, winnls.h:891:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_USE_SEARCH_ALL_LANGUAGES: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnls.h:892:9, winnls.h:892:9, winnls.h:892:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_LANG_NEUTRAL_PE_FILE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* winnls.h:893:9, winnls.h:893:9, winnls.h:893:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_NON_LANG_NEUTRAL_FILE: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* winnls.h:894:9, winnls.h:894:9, winnls.h:894:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_MACHINE_LANGUAGE_SETTINGS: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* winnls.h:895:9, winnls.h:895:9, winnls.h:895:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_FILETYPE_NOT_LANGUAGE_NEUTRAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:896:9, winnls.h:896:9, winnls.h:896:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_FILETYPE_LANGUAGE_NEUTRAL_MAIN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:897:9, winnls.h:897:9, winnls.h:897:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_FILETYPE_LANGUAGE_NEUTRAL_MUI: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:898:9, winnls.h:898:9, winnls.h:898:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_QUERY_TYPE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:899:9, winnls.h:899:9, winnls.h:899:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_QUERY_CHECKSUM: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:900:9, winnls.h:900:9, winnls.h:900:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_QUERY_LANGUAGE_NAME: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:901:9, winnls.h:901:9, winnls.h:901:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_QUERY_RESOURCE_TYPES: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:902:9, winnls.h:902:9, winnls.h:902:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_FILEINFO_VERSION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:903:9, winnls.h:903:9, winnls.h:903:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_FULL_LANGUAGE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:905:9, winnls.h:905:9, winnls.h:905:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_PARTIAL_LANGUAGE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:906:9, winnls.h:906:9, winnls.h:906:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_LIP_LANGUAGE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:907:9, winnls.h:907:9, winnls.h:907:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_LANGUAGE_INSTALLED: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* winnls.h:908:9, winnls.h:908:9, winnls.h:908:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MUI_LANGUAGE_LICENSED: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* winnls.h:909:9, winnls.h:909:9, winnls.h:909:9 */
#[cfg(feature="winapi_app")] pub const GEOID_NOT_AVAILABLE: i32 = -0x1i32; /* Unary(Neg, Integer(1, Yes, Unknown)) */ /* winnls.h:1112:9, winnls.h:1112:9, winnls.h:1112:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const IDN_ALLOW_UNASSIGNED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:1163:9, winnls.h:1163:9, winnls.h:1163:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const IDN_USE_STD3_ASCII_RULES: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:1164:9, winnls.h:1164:9, winnls.h:1164:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const IDN_EMAIL_ADDRESS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:1165:9, winnls.h:1165:9, winnls.h:1165:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const IDN_RAW_PUNYCODE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:1166:9, winnls.h:1166:9, winnls.h:1166:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const VS_ALLOW_LATIN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:1168:9, winnls.h:1168:9, winnls.h:1168:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const GSS_ALLOW_INHERITED_COMMON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:1170:9, winnls.h:1170:9, winnls.h:1170:9 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnls::LANGUAGEGROUP_ENUMPROCW as LANGUAGEGROUP_ENUMPROC; /* winnls.h:1231:9, winnls.h:1231:9, winnls.h:1231:9 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnls::LANGGROUPLOCALE_ENUMPROCW as LANGGROUPLOCALE_ENUMPROC; /* winnls.h:1232:9, winnls.h:1232:9, winnls.h:1232:9 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnls::UILANGUAGE_ENUMPROCW as UILANGUAGE_ENUMPROC; /* winnls.h:1233:9, winnls.h:1233:9, winnls.h:1233:9 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnls::CODEPAGE_ENUMPROCW as CODEPAGE_ENUMPROC; /* winnls.h:1234:9, winnls.h:1234:9, winnls.h:1234:9 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnls::DATEFMT_ENUMPROCW as DATEFMT_ENUMPROC; /* winnls.h:1235:9, winnls.h:1235:9, winnls.h:1235:9 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnls::DATEFMT_ENUMPROCEXW as DATEFMT_ENUMPROCEX; /* winnls.h:1236:9, winnls.h:1236:9, winnls.h:1236:9 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnls::TIMEFMT_ENUMPROCW as TIMEFMT_ENUMPROC; /* winnls.h:1237:9, winnls.h:1237:9, winnls.h:1237:9 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnls::CALINFO_ENUMPROCW as CALINFO_ENUMPROC; /* winnls.h:1238:9, winnls.h:1238:9, winnls.h:1238:9 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnls::CALINFO_ENUMPROCEXW as CALINFO_ENUMPROCEX; /* winnls.h:1239:9, winnls.h:1239:9, winnls.h:1239:9 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnls::LOCALE_ENUMPROCW as LOCALE_ENUMPROC; /* winnls.h:1240:9, winnls.h:1240:9, winnls.h:1240:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub const MUI_FORMAT_REG_COMPAT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* winnls.h:1606:9, winnls.h:1606:9, winnls.h:1606:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub const MUI_FORMAT_INF_COMPAT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* winnls.h:1607:9, winnls.h:1607:9, winnls.h:1607:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub const MUI_VERIFY_FILE_EXISTS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* winnls.h:1608:9, winnls.h:1608:9, winnls.h:1608:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub const MUI_SKIP_STRING_CACHE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* winnls.h:1609:9, winnls.h:1609:9, winnls.h:1609:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] pub const MUI_IMMUTABLE_LOOKUP: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* winnls.h:1610:9, winnls.h:1610:9, winnls.h:1610:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_NAME_INVARIANT: &'static str = ""; /* String("", Wide) */ /* winnls.h:2367:9, winnls.h:2367:9, winnls.h:2367:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const LOCALE_NAME_SYSTEM_DEFAULT: &'static str = "!x-sys-default-locale"; /* String("!x-sys-default-locale", Wide) */ /* winnls.h:2368:9, winnls.h:2368:9, winnls.h:2368:9 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] pub type LANGUAGEGROUP_ENUMPROCA = extern "system" fn(::libc::c_ulong, *mut ::libc::c_schar, *mut ::libc::c_schar, ::libc::c_ulong, ::libc::c_longlong) -> ::libc::c_int; /* winnls.h:1179:25 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] pub type LANGGROUPLOCALE_ENUMPROCA = extern "system" fn(::libc::c_ulong, ::libc::c_ulong, *mut ::libc::c_schar, ::libc::c_longlong) -> ::libc::c_int; /* winnls.h:1180:25 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] pub type UILANGUAGE_ENUMPROCA = extern "system" fn(*mut ::libc::c_schar, ::libc::c_longlong) -> ::libc::c_int; /* winnls.h:1181:25 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] pub type LANGUAGEGROUP_ENUMPROCW = extern "system" fn(::libc::c_ulong, *mut ::libc::c_ushort, *mut ::libc::c_ushort, ::libc::c_ulong, ::libc::c_longlong) -> ::libc::c_int; /* winnls.h:1191:25 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] pub type LANGGROUPLOCALE_ENUMPROCW = extern "system" fn(::libc::c_ulong, ::libc::c_ulong, *mut ::libc::c_ushort, ::libc::c_longlong) -> ::libc::c_int; /* winnls.h:1192:25 */
#[cfg(feature="winapi_app")] #[cfg(any(target_arch="x86_64"))] pub type UILANGUAGE_ENUMPROCW = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_longlong) -> ::libc::c_int; /* winnls.h:1193:25 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] #[cfg(any(target_arch="x86_64"))] pub type CALINFO_ENUMPROCEXEX = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_ulong, *mut ::libc::c_ushort, ::libc::c_longlong) -> ::libc::c_int; /* winnls.h:2511:25 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] #[cfg(any(target_arch="x86_64"))] pub type DATEFMT_ENUMPROCEXEX = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_ulong, ::libc::c_longlong) -> ::libc::c_int; /* winnls.h:2525:25 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] #[cfg(any(target_arch="x86_64"))] pub type TIMEFMT_ENUMPROCEX = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_longlong) -> ::libc::c_int; /* winnls.h:2537:25 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] #[cfg(any(target_arch="x86_64"))] pub type LOCALE_ENUMPROCEX = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_ulong, ::libc::c_longlong) -> ::libc::c_int; /* winnls.h:2549:25 */
