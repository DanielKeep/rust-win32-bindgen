pub const API_SET_PREFIX_NAME_A: &'static str = "API-"; /* String("API-", Narrow) */ /* apiset.h:26:9, apiset.h:26:9, apiset.h:26:9 */
/* // #define API_SET_PREFIX_NAME_U Call { subject: Ident("TEXT"), args: [Ident("API_SET_PREFIX_NAME_A")] } */ /* apiset.h:28:9, apiset.h:28:9, apiset.h:28:9 */
pub const API_SET_EXTENSION_NAME_A: &'static str = "EXT-"; /* String("EXT-", Narrow) */ /* apiset.h:32:9, apiset.h:32:9, apiset.h:32:9 */
/* // #define API_SET_EXTENSION_NAME_U Call { subject: Ident("TEXT"), args: [Ident("API_SET_EXTENSION_NAME_A")] } */ /* apiset.h:34:9, apiset.h:34:9, apiset.h:34:9 */
pub const API_SET_SECTION_NAME: &'static str = ".apiset"; /* String(".apiset", Narrow) */ /* apiset.h:39:9, apiset.h:39:9, apiset.h:39:9 */
pub const API_SET_SCHEMA_SUFFIX: &'static str = ".sys"; /* String(".sys", Wide) */ /* apiset.h:40:9, apiset.h:40:9, apiset.h:40:9 */
pub const API_SET_SCHEMA_VERSION_V2: u64 = 0x2u64; /* Integer(2, No, Long) */ /* apiset.h:48:9, apiset.h:48:9, apiset.h:48:9 */
pub const API_SET_SCHEMA_VERSION_V3: u64 = 0x3u64; /* Integer(3, No, Long) */ /* apiset.h:49:9, apiset.h:49:9, apiset.h:49:9 */
pub const API_SET_SCHEMA_VERSION_V4: u64 = 0x4u64; /* Integer(4, No, Long) */ /* apiset.h:50:9, apiset.h:50:9, apiset.h:50:9 */
#[doc(inline)] pub use ::apiset::API_SET_SCHEMA_VERSION_V4 as API_SET_SCHEMA_VERSION; /* apiset.h:51:9, apiset.h:51:9, apiset.h:51:9 */
pub const API_SET_LOAD_SCHEMA_ORDINAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* apiset.h:55:9, apiset.h:55:9, apiset.h:55:9 */
pub const API_SET_LOOKUP_ORDINAL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* apiset.h:56:9, apiset.h:56:9, apiset.h:56:9 */
pub const API_SET_RELEASE_SCHEMA_ORDINAL: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* apiset.h:57:9, apiset.h:57:9, apiset.h:57:9 */
