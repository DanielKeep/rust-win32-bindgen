#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn WinMain(hInstance: ::minwindef::HINSTANCE, hPrevInstance: ::minwindef::HINSTANCE, lpCmdLine: ::winnt::LPSTR, nShowCmd: ::libc::c_int) -> ::libc::c_int; /* winbase.h:905:1 */
}
