#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn AlphaBlend(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::wingdi::BLENDFUNCTION) -> ::minwindef::BOOL; /* wingdi.h:4575:23 */
    pub fn GradientFill(_: ::windef::HDC, _: ::wingdi::PTRIVERTEX, _: ::minwindef::ULONG, _: ::winnt::PVOID, _: ::minwindef::ULONG, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* wingdi.h:4614:1 */
    pub fn TransparentBlt(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:4588:23 */
}
