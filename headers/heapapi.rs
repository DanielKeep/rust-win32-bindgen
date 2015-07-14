#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct HEAP_SUMMARY { cb: ::minwindef::DWORD, cbAllocated: ::basetsd::SIZE_T, cbCommitted: ::basetsd::SIZE_T, cbReserved: ::basetsd::SIZE_T, cbMaxReserve: ::basetsd::SIZE_T } /* heapapi.h:46:16, heapapi.h:46:16, heapapi.h:46:16 */
#[cfg(feature="winapi_desktop")] pub type PHEAP_SUMMARY = *mut ::heapapi::HEAP_SUMMARY; /* heapapi.h:52:18, heapapi.h:52:18, heapapi.h:52:18 */
#[cfg(feature="winapi_desktop")] pub type LPHEAP_SUMMARY = ::heapapi::PHEAP_SUMMARY; /* heapapi.h:53:23, heapapi.h:53:23, heapapi.h:53:23 */
