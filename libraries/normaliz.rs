#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn IdnToNameprepUnicode(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2320:8 */
    pub fn IsNormalizedString(_: ::winnls::NORM_FORM, _: ::winnt::LPCWSTR, _: ::libc::c_int) -> ::minwindef::BOOL; /* winnls.h:2303:8 */
    pub fn NormalizeString(_: ::winnls::NORM_FORM, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::winnt::LPWSTR, _: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2295:8 */
}
