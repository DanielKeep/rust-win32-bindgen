#[cfg(feature="winapi_app")] 
extern "system" {
    pub fn WaitOnAddress(Address: *mut ::libc::c_void, CompareAddress: ::winnt::PVOID, AddressSize: ::basetsd::SIZE_T, dwMilliseconds: ::minwindef::DWORD) -> ::minwindef::BOOL; /* synchapi.h:915:1 */
    pub fn WakeByAddressAll(Address: ::winnt::PVOID); /* synchapi.h:932:1 */
    pub fn WakeByAddressSingle(Address: ::winnt::PVOID); /* synchapi.h:925:1 */
}
