#[cfg(feature="winapi_app")] pub type SRWLOCK = ::winnt::RTL_SRWLOCK; /* synchapi.h:57:21, synchapi.h:57:21, synchapi.h:57:21 */
#[cfg(feature="winapi_app")] pub type PSRWLOCK = *mut ::winnt::RTL_SRWLOCK; /* synchapi.h:57:31, synchapi.h:57:31, synchapi.h:57:31 */
#[cfg(feature="winapi_app")] pub type INIT_ONCE = ::winnt::RTL_RUN_ONCE; /* synchapi.h:283:22, synchapi.h:283:22, synchapi.h:283:22 */
#[cfg(feature="winapi_app")] pub type PINIT_ONCE = ::winnt::PRTL_RUN_ONCE; /* synchapi.h:284:23, synchapi.h:284:23, synchapi.h:284:23 */
#[cfg(feature="winapi_app")] pub type LPINIT_ONCE = ::winnt::PRTL_RUN_ONCE; /* synchapi.h:285:23, synchapi.h:285:23, synchapi.h:285:23 */
#[cfg(feature="winapi_app")] pub type PINIT_ONCE_FN = Option<extern "system" fn(*mut ::winnt::RTL_RUN_ONCE, *mut ::libc::c_void, *mut *mut ::libc::c_void) -> ::libc::c_int>; /* synchapi.h:306:10, synchapi.h:306:10, synchapi.h:306:10 */
#[cfg(feature="winapi_app")] pub type CONDITION_VARIABLE = ::winnt::RTL_CONDITION_VARIABLE; /* synchapi.h:361:32, synchapi.h:361:32, synchapi.h:361:32 */
#[cfg(feature="winapi_app")] pub type PCONDITION_VARIABLE = *mut ::winnt::RTL_CONDITION_VARIABLE; /* synchapi.h:361:53, synchapi.h:361:53, synchapi.h:361:53 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PTIMERAPCROUTINE = Option<extern "system" fn(*mut ::libc::c_void, ::libc::c_ulong, ::libc::c_ulong)>; /* synchapi.h:674:12, synchapi.h:674:12, synchapi.h:674:12 */
#[cfg(feature="winapi_desktop")] pub type SYNCHRONIZATION_BARRIER = ::winnt::RTL_BARRIER; /* synchapi.h:866:21, synchapi.h:866:21, synchapi.h:866:21 */
#[cfg(feature="winapi_desktop")] pub type PSYNCHRONIZATION_BARRIER = ::winnt::PRTL_BARRIER; /* synchapi.h:867:22, synchapi.h:867:22, synchapi.h:867:22 */
#[cfg(feature="winapi_desktop")] pub type LPSYNCHRONIZATION_BARRIER = ::winnt::PRTL_BARRIER; /* synchapi.h:868:22, synchapi.h:868:22, synchapi.h:868:22 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnt::RTL_RUN_ONCE_CHECK_ONLY as INIT_ONCE_CHECK_ONLY; /* synchapi.h:293:9, synchapi.h:293:9, synchapi.h:293:9 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnt::RTL_RUN_ONCE_ASYNC as INIT_ONCE_ASYNC; /* synchapi.h:294:9, synchapi.h:294:9, synchapi.h:294:9 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnt::RTL_RUN_ONCE_INIT_FAILED as INIT_ONCE_INIT_FAILED; /* synchapi.h:295:9, synchapi.h:295:9, synchapi.h:295:9 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnt::RTL_RUN_ONCE_CTX_RESERVED_BITS as INIT_ONCE_CTX_RESERVED_BITS; /* synchapi.h:302:9, synchapi.h:302:9, synchapi.h:302:9 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnt::RTL_CONDITION_VARIABLE_LOCKMODE_SHARED as CONDITION_VARIABLE_LOCKMODE_SHARED; /* synchapi.h:373:9, synchapi.h:373:9, synchapi.h:373:9 */
#[cfg(feature="winapi_app")] #[doc(inline)] pub use ::winnt::MUTANT_QUERY_STATE as MUTEX_MODIFY_STATE; /* synchapi.h:521:9, synchapi.h:521:9, synchapi.h:521:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const CREATE_MUTEX_INITIAL_OWNER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* synchapi.h:744:9, synchapi.h:744:9, synchapi.h:744:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const CREATE_EVENT_MANUAL_RESET: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* synchapi.h:774:9, synchapi.h:774:9, synchapi.h:774:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const CREATE_EVENT_INITIAL_SET: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* synchapi.h:775:9, synchapi.h:775:9, synchapi.h:775:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06000000"))] pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* synchapi.h:830:9, synchapi.h:830:9, synchapi.h:830:9 */
#[cfg(feature="winapi_desktop")] pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* synchapi.h:870:9, synchapi.h:870:9, synchapi.h:870:9 */
#[cfg(feature="winapi_desktop")] pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* synchapi.h:871:9, synchapi.h:871:9, synchapi.h:871:9 */
#[cfg(feature="winapi_desktop")] pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* synchapi.h:872:9, synchapi.h:872:9, synchapi.h:872:9 */
