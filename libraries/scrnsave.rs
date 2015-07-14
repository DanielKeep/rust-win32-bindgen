#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WinMain(_: ::minwindef::HINSTANCE, _: ::minwindef::HINSTANCE, _: ::winnt::LPSTR, _: ::libc::c_int) -> ::libc::c_int; /* winbase.h:905:1 */
}
