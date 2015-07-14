#[repr(C)] pub struct GUID { Data1: ::libc::c_ulong, Data2: ::libc::c_ushort, Data3: ::libc::c_ushort, Data4: *mut [::libc::c_uchar; 8] } /* guiddef.h:22:16, guiddef.h:22:16, guiddef.h:22:16 */
pub type LPGUID = *mut ::guiddef::GUID; /* guiddef.h:75:15, guiddef.h:75:15, guiddef.h:75:15 */
pub type LPCGUID = *const ::guiddef::GUID; /* guiddef.h:80:21, guiddef.h:80:21, guiddef.h:80:21 */
pub type IID = ::guiddef::GUID; /* guiddef.h:86:14, guiddef.h:86:14, guiddef.h:86:14 */
pub type LPIID = *mut ::guiddef::GUID; /* guiddef.h:87:14, guiddef.h:87:14, guiddef.h:87:14 */
pub type CLSID = ::guiddef::GUID; /* guiddef.h:90:14, guiddef.h:90:14, guiddef.h:90:14 */
pub type LPCLSID = *mut ::guiddef::GUID; /* guiddef.h:91:16, guiddef.h:91:16, guiddef.h:91:16 */
pub type FMTID = ::guiddef::GUID; /* guiddef.h:94:14, guiddef.h:94:14, guiddef.h:94:14 */
pub type LPFMTID = *mut ::guiddef::GUID; /* guiddef.h:95:16, guiddef.h:95:16, guiddef.h:95:16 */
