#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn wglCopyContext(_: ::windef::HGLRC, _: ::windef::HGLRC, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:6073:24 */
    pub fn wglCreateContext(_: ::windef::HDC) -> ::windef::HGLRC; /* wingdi.h:6074:24 */
    pub fn wglCreateLayerContext(_: ::windef::HDC, _: ::libc::c_int) -> ::windef::HGLRC; /* wingdi.h:6075:24 */
    pub fn wglDeleteContext(_: ::windef::HGLRC) -> ::minwindef::BOOL; /* wingdi.h:6076:24 */
    pub fn wglDescribeLayerPlane(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT, _: ::wingdi::LPLAYERPLANEDESCRIPTOR) -> ::minwindef::BOOL; /* wingdi.h:6192:24 */
    pub fn wglGetCurrentContext() -> ::windef::HGLRC; /* wingdi.h:6077:24 */
    pub fn wglGetCurrentDC() -> ::windef::HDC; /* wingdi.h:6078:24 */
    pub fn wglGetLayerPaletteEntries(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: *mut ::libc::c_ulong) -> ::libc::c_int; /* wingdi.h:6196:24 */
    pub fn wglGetProcAddress(_: ::winnt::LPCSTR) -> ::minwindef::PROC; /* wingdi.h:6079:24 */
    pub fn wglMakeCurrent(_: ::windef::HDC, _: ::windef::HGLRC) -> ::minwindef::BOOL; /* wingdi.h:6080:24 */
    pub fn wglRealizeLayerPalette(_: ::windef::HDC, _: ::libc::c_int, _: ::minwindef::BOOL) -> ::minwindef::BOOL; /* wingdi.h:6198:24 */
    pub fn wglSetLayerPaletteEntries(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: *const ::libc::c_ulong) -> ::libc::c_int; /* wingdi.h:6194:24 */
    pub fn wglShareLists(_: ::windef::HGLRC, _: ::windef::HGLRC) -> ::minwindef::BOOL; /* wingdi.h:6081:24 */
    pub fn wglSwapLayerBuffers(_: ::windef::HDC, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:6199:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::wglUseFontBitmapsW as wglUseFontBitmaps; /* wingdi.h:6085:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn wglUseFontBitmapsA(_: ::windef::HDC, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:6082:24 */
    pub fn wglUseFontBitmapsW(_: ::windef::HDC, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:6083:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::wglUseFontOutlinesW as wglUseFontOutlines; /* wingdi.h:6111:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn wglUseFontOutlinesA(_: ::windef::HDC, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::FLOAT, _: ::minwindef::FLOAT, _: ::libc::c_int, _: ::wingdi::LPGLYPHMETRICSFLOAT) -> ::minwindef::BOOL; /* wingdi.h:6106:24 */
    pub fn wglUseFontOutlinesW(_: ::windef::HDC, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::FLOAT, _: ::minwindef::FLOAT, _: ::libc::c_int, _: ::wingdi::LPGLYPHMETRICSFLOAT) -> ::minwindef::BOOL; /* wingdi.h:6108:24 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn wglSwapMultipleBuffers(_: ::minwindef::UINT, _: *const ::wingdi::WGLSWAP) -> ::minwindef::DWORD; /* wingdi.h:6211:24 */
}
