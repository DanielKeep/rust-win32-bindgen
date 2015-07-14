#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RevertToSelf() -> ::minwindef::BOOL; /* securitybaseapi.h:1149:1 */
}
