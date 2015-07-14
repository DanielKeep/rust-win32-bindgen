#[cfg(feature="winapi_app")] #[repr(C)] pub struct HWND__ { unused: ::libc::c_int } /* windef.h:39:1, windef.h:39:1, windef.h:39:1 */
#[cfg(feature="winapi_app")] pub type HWND = *mut ::windef::HWND__; /* windef.h:39:1, windef.h:39:1, windef.h:39:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HHOOK__ { unused: ::libc::c_int } /* windef.h:40:1, windef.h:40:1, windef.h:40:1 */
#[cfg(feature="winapi_app")] pub type HHOOK = *mut ::windef::HHOOK__; /* windef.h:40:1, windef.h:40:1, windef.h:40:1 */
#[cfg(feature="winapi_app")] pub type HGDIOBJ = *mut ::libc::c_void; /* windef.h:63:20, windef.h:63:20, windef.h:63:20 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HACCEL__ { unused: ::libc::c_int } /* windef.h:70:1, windef.h:70:1, windef.h:70:1 */
#[cfg(feature="winapi_app")] pub type HACCEL = *mut ::windef::HACCEL__; /* windef.h:70:1, windef.h:70:1, windef.h:70:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HBITMAP__ { unused: ::libc::c_int } /* windef.h:73:1, windef.h:73:1, windef.h:73:1 */
#[cfg(feature="winapi_app")] pub type HBITMAP = *mut ::windef::HBITMAP__; /* windef.h:73:1, windef.h:73:1, windef.h:73:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HBRUSH__ { unused: ::libc::c_int } /* windef.h:74:1, windef.h:74:1, windef.h:74:1 */
#[cfg(feature="winapi_app")] pub type HBRUSH = *mut ::windef::HBRUSH__; /* windef.h:74:1, windef.h:74:1, windef.h:74:1 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct HCOLORSPACE__ { unused: ::libc::c_int } /* windef.h:77:1, windef.h:77:1, windef.h:77:1 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type HCOLORSPACE = *mut ::windef::HCOLORSPACE__; /* windef.h:77:1, windef.h:77:1, windef.h:77:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HDC__ { unused: ::libc::c_int } /* windef.h:80:1, windef.h:80:1, windef.h:80:1 */
#[cfg(feature="winapi_app")] pub type HDC = *mut ::windef::HDC__; /* windef.h:80:1, windef.h:80:1, windef.h:80:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HGLRC__ { unused: ::libc::c_int } /* windef.h:82:1, windef.h:82:1, windef.h:82:1 */
#[cfg(feature="winapi_app")] pub type HGLRC = *mut ::windef::HGLRC__; /* windef.h:82:1, windef.h:82:1, windef.h:82:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HDESK__ { unused: ::libc::c_int } /* windef.h:83:1, windef.h:83:1, windef.h:83:1 */
#[cfg(feature="winapi_app")] pub type HDESK = *mut ::windef::HDESK__; /* windef.h:83:1, windef.h:83:1, windef.h:83:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HENHMETAFILE__ { unused: ::libc::c_int } /* windef.h:84:1, windef.h:84:1, windef.h:84:1 */
#[cfg(feature="winapi_app")] pub type HENHMETAFILE = *mut ::windef::HENHMETAFILE__; /* windef.h:84:1, windef.h:84:1, windef.h:84:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HFONT__ { unused: ::libc::c_int } /* windef.h:86:1, windef.h:86:1, windef.h:86:1 */
#[cfg(feature="winapi_app")] pub type HFONT = *mut ::windef::HFONT__; /* windef.h:86:1, windef.h:86:1, windef.h:86:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HICON__ { unused: ::libc::c_int } /* windef.h:88:1, windef.h:88:1, windef.h:88:1 */
#[cfg(feature="winapi_app")] pub type HICON = *mut ::windef::HICON__; /* windef.h:88:1, windef.h:88:1, windef.h:88:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HMENU__ { unused: ::libc::c_int } /* windef.h:90:1, windef.h:90:1, windef.h:90:1 */
#[cfg(feature="winapi_app")] pub type HMENU = *mut ::windef::HMENU__; /* windef.h:90:1, windef.h:90:1, windef.h:90:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HPALETTE__ { unused: ::libc::c_int } /* windef.h:93:1, windef.h:93:1, windef.h:93:1 */
#[cfg(feature="winapi_app")] pub type HPALETTE = *mut ::windef::HPALETTE__; /* windef.h:93:1, windef.h:93:1, windef.h:93:1 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HPEN__ { unused: ::libc::c_int } /* windef.h:94:1, windef.h:94:1, windef.h:94:1 */
#[cfg(feature="winapi_app")] pub type HPEN = *mut ::windef::HPEN__; /* windef.h:94:1, windef.h:94:1, windef.h:94:1 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct HWINEVENTHOOK__ { unused: ::libc::c_int } /* windef.h:98:1, windef.h:98:1, windef.h:98:1 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type HWINEVENTHOOK = *mut ::windef::HWINEVENTHOOK__; /* windef.h:98:1, windef.h:98:1, windef.h:98:1 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct HMONITOR__ { unused: ::libc::c_int } /* windef.h:110:1, windef.h:110:1, windef.h:110:1 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05000000"))] pub type HMONITOR = *mut ::windef::HMONITOR__; /* windef.h:110:1, windef.h:110:1, windef.h:110:1 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct HUMPD__ { unused: ::libc::c_int } /* windef.h:120:1, windef.h:120:1, windef.h:120:1 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type HUMPD = *mut ::windef::HUMPD__; /* windef.h:120:1, windef.h:120:1, windef.h:120:1 */
#[cfg(feature="winapi_app")] pub type HCURSOR = ::windef::HICON; /* windef.h:131:15, windef.h:131:15, windef.h:131:15 */
#[cfg(feature="winapi_app")] pub type COLORREF = ::minwindef::DWORD; /* windef.h:136:17, windef.h:136:17, windef.h:136:17 */
#[cfg(feature="winapi_desktop")] pub type LPCOLORREF = *mut ::libc::c_ulong; /* windef.h:144:18, windef.h:144:18, windef.h:144:18 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct RECT { left: ::winnt::LONG, top: ::winnt::LONG, right: ::winnt::LONG, bottom: ::winnt::LONG } /* windef.h:154:16, windef.h:154:16, windef.h:154:16 */
#[cfg(feature="winapi_app")] pub type PRECT = *mut ::windef::RECT; /* windef.h:160:10, windef.h:160:10, windef.h:160:10 */
#[cfg(feature="winapi_app")] pub type NPRECT = *mut ::windef::RECT; /* windef.h:160:23, windef.h:160:23, windef.h:160:23 */
#[cfg(feature="winapi_app")] pub type LPRECT = *mut ::windef::RECT; /* windef.h:160:36, windef.h:160:36, windef.h:160:36 */
#[cfg(feature="winapi_app")] pub type LPCRECT = *const ::windef::RECT; /* windef.h:162:25, windef.h:162:25, windef.h:162:25 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct RECTL { left: ::winnt::LONG, top: ::winnt::LONG, right: ::winnt::LONG, bottom: ::winnt::LONG } /* windef.h:164:16, windef.h:164:16, windef.h:164:16 */
#[cfg(feature="winapi_app")] pub type PRECTL = *mut ::windef::RECTL; /* windef.h:170:11, windef.h:170:11, windef.h:170:11 */
#[cfg(feature="winapi_app")] pub type LPRECTL = *mut ::windef::RECTL; /* windef.h:170:20, windef.h:170:20, windef.h:170:20 */
#[cfg(feature="winapi_app")] pub type LPCRECTL = *const ::windef::RECTL; /* windef.h:172:26, windef.h:172:26, windef.h:172:26 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct POINT { x: ::winnt::LONG, y: ::winnt::LONG } /* windef.h:174:16, windef.h:174:16, windef.h:174:16 */
#[cfg(feature="winapi_app")] pub type PPOINT = *mut ::windef::POINT; /* windef.h:178:11, windef.h:178:11, windef.h:178:11 */
#[cfg(feature="winapi_app")] pub type NPPOINT = *mut ::windef::POINT; /* windef.h:178:25, windef.h:178:25, windef.h:178:25 */
#[cfg(feature="winapi_app")] pub type LPPOINT = *mut ::windef::POINT; /* windef.h:178:39, windef.h:178:39, windef.h:178:39 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct POINTL { x: ::winnt::LONG, y: ::winnt::LONG } /* windef.h:180:16, windef.h:180:16, windef.h:180:16 */
#[cfg(feature="winapi_app")] pub type PPOINTL = *mut ::windef::POINTL; /* windef.h:184:12, windef.h:184:12, windef.h:184:12 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct SIZE { cx: ::winnt::LONG, cy: ::winnt::LONG } /* windef.h:186:16, windef.h:186:16, windef.h:186:16 */
#[cfg(feature="winapi_app")] pub type PSIZE = *mut ::windef::SIZE; /* windef.h:190:10, windef.h:190:10, windef.h:190:10 */
#[cfg(feature="winapi_app")] pub type LPSIZE = *mut ::windef::SIZE; /* windef.h:190:18, windef.h:190:18, windef.h:190:18 */
#[cfg(feature="winapi_app")] pub type SIZEL = ::windef::SIZE; /* windef.h:192:28, windef.h:192:28, windef.h:192:28 */
#[cfg(feature="winapi_app")] pub type PSIZEL = *mut ::windef::SIZE; /* windef.h:193:29, windef.h:193:29, windef.h:193:29 */
#[cfg(feature="winapi_app")] pub type LPSIZEL = *mut ::windef::SIZE; /* windef.h:193:38, windef.h:193:38, windef.h:193:38 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct POINTS { x: ::winnt::SHORT, y: ::winnt::SHORT } /* windef.h:195:16, windef.h:195:16, windef.h:195:16 */
#[cfg(feature="winapi_app")] pub type PPOINTS = *mut ::windef::POINTS; /* windef.h:204:12, windef.h:204:12, windef.h:204:12 */
#[cfg(feature="winapi_app")] pub type LPPOINTS = *mut ::windef::POINTS; /* windef.h:204:22, windef.h:204:22, windef.h:204:22 */
