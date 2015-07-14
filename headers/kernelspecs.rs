pub const DISPATCH_LEVEL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* kernelspecs.h:50:13, kernelspecs.h:50:13, kernelspecs.h:50:13 */
pub const APC_LEVEL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* kernelspecs.h:51:13, kernelspecs.h:51:13, kernelspecs.h:51:13 */
pub const PASSIVE_LEVEL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* kernelspecs.h:52:13, kernelspecs.h:52:13, kernelspecs.h:52:13 */
#[cfg(any(target_arch="x86"))] pub const HIGH_LEVEL: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* kernelspecs.h:55:13 */
#[cfg(any(target_arch="x86_64", target_arch="arm"))] pub const HIGH_LEVEL: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* kernelspecs.h:58:13, kernelspecs.h:61:13 */
