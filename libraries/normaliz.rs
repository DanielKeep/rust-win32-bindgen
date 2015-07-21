#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] 
extern "system" {
    pub fn IdnToNameprepUnicode(dwFlags: ::minwindef::DWORD, lpUnicodeCharStr: ::winnt::LPCWSTR, cchUnicodeChar: ::libc::c_int, lpNameprepCharStr: ::winnt::LPWSTR, cchNameprepChar: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2320:8 */
    pub fn IsNormalizedString(NormForm: ::winnls::NORM_FORM, lpString: ::winnt::LPCWSTR, cwLength: ::libc::c_int) -> ::minwindef::BOOL; /* winnls.h:2303:8 */
    pub fn NormalizeString(NormForm: ::winnls::NORM_FORM, lpSrcString: ::winnt::LPCWSTR, cwSrcLength: ::libc::c_int, lpDstString: ::winnt::LPWSTR, cwDstLength: ::libc::c_int) -> ::libc::c_int; /* winnls.h:2295:8 */
}
