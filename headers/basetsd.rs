#[cfg(any(target_arch="x86", target_arch="arm"))] pub type POINTER_64_INT = ::libc::c_ulong; /* basetsd.h:40:25, basetsd.h:40:25 */
pub type INT8 = ::libc::c_schar; /* basetsd.h:70:29, basetsd.h:70:29, basetsd.h:70:29 */
pub type PINT8 = *mut ::libc::c_schar; /* basetsd.h:70:36, basetsd.h:70:36, basetsd.h:70:36 */
pub type INT16 = ::libc::c_short; /* basetsd.h:71:29, basetsd.h:71:29, basetsd.h:71:29 */
pub type PINT16 = *mut ::libc::c_short; /* basetsd.h:71:37, basetsd.h:71:37, basetsd.h:71:37 */
pub type INT32 = ::libc::c_int; /* basetsd.h:72:29, basetsd.h:72:29, basetsd.h:72:29 */
pub type PINT32 = *mut ::libc::c_int; /* basetsd.h:72:37, basetsd.h:72:37, basetsd.h:72:37 */
pub type INT64 = ::libc::c_longlong; /* basetsd.h:73:29, basetsd.h:73:29, basetsd.h:73:29 */
pub type PINT64 = *mut ::libc::c_longlong; /* basetsd.h:73:37, basetsd.h:73:37, basetsd.h:73:37 */
pub type UINT8 = ::libc::c_uchar; /* basetsd.h:74:29, basetsd.h:74:29, basetsd.h:74:29 */
pub type PUINT8 = *mut ::libc::c_uchar; /* basetsd.h:74:37, basetsd.h:74:37, basetsd.h:74:37 */
pub type UINT16 = ::libc::c_ushort; /* basetsd.h:75:29, basetsd.h:75:29, basetsd.h:75:29 */
pub type PUINT16 = *mut ::libc::c_ushort; /* basetsd.h:75:38, basetsd.h:75:38, basetsd.h:75:38 */
pub type UINT32 = ::libc::c_uint; /* basetsd.h:76:29, basetsd.h:76:29, basetsd.h:76:29 */
pub type PUINT32 = *mut ::libc::c_uint; /* basetsd.h:76:38, basetsd.h:76:38, basetsd.h:76:38 */
pub type UINT64 = ::libc::c_ulonglong; /* basetsd.h:77:29, basetsd.h:77:29, basetsd.h:77:29 */
pub type PUINT64 = *mut ::libc::c_ulonglong; /* basetsd.h:77:38, basetsd.h:77:38, basetsd.h:77:38 */
pub type LONG32 = ::libc::c_int; /* basetsd.h:83:20, basetsd.h:83:20, basetsd.h:83:20 */
pub type PLONG32 = *mut ::libc::c_int; /* basetsd.h:83:29, basetsd.h:83:29, basetsd.h:83:29 */
pub type ULONG32 = ::libc::c_uint; /* basetsd.h:89:22, basetsd.h:89:22, basetsd.h:89:22 */
pub type PULONG32 = *mut ::libc::c_uint; /* basetsd.h:89:32, basetsd.h:89:32, basetsd.h:89:32 */
pub type DWORD32 = ::libc::c_uint; /* basetsd.h:90:22, basetsd.h:90:22, basetsd.h:90:22 */
pub type PDWORD32 = *mut ::libc::c_uint; /* basetsd.h:90:32, basetsd.h:90:32, basetsd.h:90:32 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type INT_PTR = ::libc::c_int; /* basetsd.h:129:22, basetsd.h:129:22 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PINT_PTR = *mut ::libc::c_int; /* basetsd.h:129:32, basetsd.h:129:32 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type UINT_PTR = ::libc::c_uint; /* basetsd.h:130:31, basetsd.h:130:31 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PUINT_PTR = *mut ::libc::c_uint; /* basetsd.h:130:42, basetsd.h:130:42 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type LONG_PTR = ::libc::c_long; /* basetsd.h:132:23, basetsd.h:132:23 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PLONG_PTR = *mut ::libc::c_long; /* basetsd.h:132:34, basetsd.h:132:34 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type ULONG_PTR = ::libc::c_ulong; /* basetsd.h:133:32, basetsd.h:133:32 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PULONG_PTR = *mut ::libc::c_ulong; /* basetsd.h:133:44, basetsd.h:133:44 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type UHALF_PTR = ::libc::c_ushort; /* basetsd.h:328:24, basetsd.h:328:24 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PUHALF_PTR = *mut ::libc::c_ushort; /* basetsd.h:328:36, basetsd.h:328:36 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type HALF_PTR = ::libc::c_short; /* basetsd.h:329:15, basetsd.h:329:15 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PHALF_PTR = *mut ::libc::c_short; /* basetsd.h:329:26, basetsd.h:329:26 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type SHANDLE_PTR = ::libc::c_long; /* basetsd.h:330:19, basetsd.h:330:19 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type HANDLE_PTR = ::libc::c_ulong; /* basetsd.h:331:28, basetsd.h:331:28 */
pub type SIZE_T = ::basetsd::ULONG_PTR; /* basetsd.h:415:19, basetsd.h:415:19, basetsd.h:415:19 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PSIZE_T = *mut ::libc::c_ulong; /* basetsd.h:415:28, basetsd.h:415:28 */
pub type SSIZE_T = ::basetsd::LONG_PTR; /* basetsd.h:416:18, basetsd.h:416:18, basetsd.h:416:18 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PSSIZE_T = *mut ::libc::c_long; /* basetsd.h:416:28, basetsd.h:416:28 */
pub type DWORD_PTR = ::basetsd::ULONG_PTR; /* basetsd.h:464:19, basetsd.h:464:19, basetsd.h:464:19 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PDWORD_PTR = *mut ::libc::c_ulong; /* basetsd.h:464:31, basetsd.h:464:31 */
pub type LONG64 = ::libc::c_longlong; /* basetsd.h:470:17, basetsd.h:470:17, basetsd.h:470:17 */
pub type PLONG64 = *mut ::libc::c_longlong; /* basetsd.h:470:26, basetsd.h:470:26, basetsd.h:470:26 */
pub type ULONG64 = ::libc::c_ulonglong; /* basetsd.h:477:26, basetsd.h:477:26, basetsd.h:477:26 */
pub type PULONG64 = *mut ::libc::c_ulonglong; /* basetsd.h:477:36, basetsd.h:477:36, basetsd.h:477:36 */
pub type DWORD64 = ::libc::c_ulonglong; /* basetsd.h:478:26, basetsd.h:478:26, basetsd.h:478:26 */
pub type PDWORD64 = *mut ::libc::c_ulonglong; /* basetsd.h:478:36, basetsd.h:478:36, basetsd.h:478:36 */
pub type KAFFINITY = ::basetsd::ULONG_PTR; /* basetsd.h:486:19, basetsd.h:486:19, basetsd.h:486:19 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub type PKAFFINITY = *mut ::libc::c_ulong; /* basetsd.h:487:20, basetsd.h:487:20 */
#[cfg(any(target_arch="x86", target_arch="arm"))] pub const ADDRESS_TAG_BIT: u64 = 0x80000000u64; /* Integer(2147483648, No, Long) */ /* basetsd.h:326:9, basetsd.h:326:9 */
#[cfg(any(target_arch="x86_64"))] pub type POINTER_64_INT = ::libc::c_ulonglong; /* basetsd.h:24:27 */
#[cfg(any(target_arch="x86_64"))] pub type INT_PTR = ::libc::c_longlong; /* basetsd.h:120:21 */
#[cfg(any(target_arch="x86_64"))] pub type PINT_PTR = *mut ::libc::c_longlong; /* basetsd.h:120:31 */
#[cfg(any(target_arch="x86_64"))] pub type UINT_PTR = ::libc::c_ulonglong; /* basetsd.h:121:30 */
#[cfg(any(target_arch="x86_64"))] pub type PUINT_PTR = *mut ::libc::c_ulonglong; /* basetsd.h:121:41 */
#[cfg(any(target_arch="x86_64"))] pub type LONG_PTR = ::libc::c_longlong; /* basetsd.h:123:21 */
#[cfg(any(target_arch="x86_64"))] pub type PLONG_PTR = *mut ::libc::c_longlong; /* basetsd.h:123:32 */
#[cfg(any(target_arch="x86_64"))] pub type ULONG_PTR = ::libc::c_ulonglong; /* basetsd.h:124:30 */
#[cfg(any(target_arch="x86_64"))] pub type PULONG_PTR = *mut ::libc::c_ulonglong; /* basetsd.h:124:42 */
#[cfg(any(target_arch="x86_64"))] pub type SHANDLE_PTR = ::libc::c_longlong; /* basetsd.h:150:17 */
#[cfg(any(target_arch="x86_64"))] pub type HANDLE_PTR = ::libc::c_ulonglong; /* basetsd.h:151:26 */
#[cfg(any(target_arch="x86_64"))] pub type UHALF_PTR = ::libc::c_uint; /* basetsd.h:152:22 */
#[cfg(any(target_arch="x86_64"))] pub type PUHALF_PTR = *mut ::libc::c_uint; /* basetsd.h:152:34 */
#[cfg(any(target_arch="x86_64"))] pub type HALF_PTR = ::libc::c_int; /* basetsd.h:153:13 */
#[cfg(any(target_arch="x86_64"))] pub type PHALF_PTR = *mut ::libc::c_int; /* basetsd.h:153:24 */
#[cfg(any(target_arch="x86_64"))] pub type PSIZE_T = *mut ::libc::c_ulonglong; /* basetsd.h:415:28 */
#[cfg(any(target_arch="x86_64"))] pub type PSSIZE_T = *mut ::libc::c_longlong; /* basetsd.h:416:28 */
#[cfg(any(target_arch="x86_64"))] pub type PDWORD_PTR = *mut ::libc::c_ulonglong; /* basetsd.h:464:31 */
#[cfg(any(target_arch="x86_64"))] pub type PKAFFINITY = *mut ::libc::c_ulonglong; /* basetsd.h:487:20 */
