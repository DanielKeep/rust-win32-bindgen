#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn WaitOnAddress(_: *mut ::libc::c_void, _: ::winnt::PVOID, _: ::basetsd::SIZE_T, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:915:1 */
    pub fn WakeByAddressAll(_: ::winnt::PVOID); /* synchapi.h:932:1 */
    pub fn WakeByAddressSingle(_: ::winnt::PVOID); /* synchapi.h:925:1 */
}
