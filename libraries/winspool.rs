#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::DeviceCapabilitiesW as DeviceCapabilities; /* wingdi.h:3818:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn DeviceCapabilitiesA(pDevice: ::winnt::LPCSTR, pPort: ::winnt::LPCSTR, fwCapability: ::minwindef::WORD, pOutput: ::winnt::LPSTR, pDevMode: *const ::wingdi::DEVMODEA) -> ::libc::c_int; /* wingdi.h:3800:1 */
    pub fn DeviceCapabilitiesW(pDevice: ::winnt::LPCWSTR, pPort: ::winnt::LPCWSTR, fwCapability: ::minwindef::WORD, pOutput: ::winnt::LPWSTR, pDevMode: *const ::wingdi::DEVMODEW) -> ::libc::c_int; /* wingdi.h:3810:1 */
}
