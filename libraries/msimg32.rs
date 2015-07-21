#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn AlphaBlend(hdcDest: ::windef::HDC, xoriginDest: ::libc::c_int, yoriginDest: ::libc::c_int, wDest: ::libc::c_int, hDest: ::libc::c_int, hdcSrc: ::windef::HDC, xoriginSrc: ::libc::c_int, yoriginSrc: ::libc::c_int, wSrc: ::libc::c_int, hSrc: ::libc::c_int, ftn: ::wingdi::BLENDFUNCTION) -> ::minwindef::BOOL; /* wingdi.h:4575:23 */
    pub fn GradientFill(hdc: ::windef::HDC, pVertex: ::wingdi::PTRIVERTEX, nVertex: ::minwindef::ULONG, pMesh: ::winnt::PVOID, nMesh: ::minwindef::ULONG, ulMode: ::minwindef::ULONG) -> ::minwindef::BOOL; /* wingdi.h:4614:1 */
    pub fn TransparentBlt(hdcDest: ::windef::HDC, xoriginDest: ::libc::c_int, yoriginDest: ::libc::c_int, wDest: ::libc::c_int, hDest: ::libc::c_int, hdcSrc: ::windef::HDC, xoriginSrc: ::libc::c_int, yoriginSrc: ::libc::c_int, wSrc: ::libc::c_int, hSrc: ::libc::c_int, crTransparent: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:4588:23 */
}
