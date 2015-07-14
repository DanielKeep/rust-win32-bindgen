#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DeviceCapabilitiesW as DeviceCapabilities; /* wingdi.h:3818:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DeviceCapabilitiesA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::minwindef::WORD, _: ::winnt::LPSTR, _: *const ::wingdi::DEVMODEA) -> ::libc::c_int; /* wingdi.h:3800:1 */
    pub fn DeviceCapabilitiesW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::minwindef::WORD, _: ::winnt::LPWSTR, _: *const ::wingdi::DEVMODEW) -> ::libc::c_int; /* wingdi.h:3810:1 */
}
