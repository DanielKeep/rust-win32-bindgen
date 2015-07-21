#[cfg(feature="winapi_app")] #[repr(C)] pub struct COORD { X: ::winnt::SHORT, Y: ::winnt::SHORT } /* wincon.h:40:16, wincon.h:40:16, wincon.h:40:16 */
#[cfg(feature="winapi_app")] pub type PCOORD = *mut ::wincon::COORD; /* wincon.h:43:11, wincon.h:43:11, wincon.h:43:11 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct SMALL_RECT { Left: ::winnt::SHORT, Top: ::winnt::SHORT, Right: ::winnt::SHORT, Bottom: ::winnt::SHORT } /* wincon.h:51:16, wincon.h:51:16, wincon.h:51:16 */
#[cfg(feature="winapi_desktop")] pub type PSMALL_RECT = *mut ::wincon::SMALL_RECT; /* wincon.h:56:16, wincon.h:56:16, wincon.h:56:16 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct KEY_EVENT_RECORD { bKeyDown: ::minwindef::BOOL, wRepeatCount: ::minwindef::WORD, wVirtualKeyCode: ::minwindef::WORD, wVirtualScanCode: ::minwindef::WORD, uChar: ::wincon::KEY_EVENT_RECORD_Child_4, dwControlKeyState: ::minwindef::DWORD } /* wincon.h:64:16, wincon.h:64:16, wincon.h:64:16 */
#[cfg(feature="winapi_app")] #[repr(C)] pub /*union*/ struct KEY_EVENT_RECORD_Child_4 { _payload0: u16 } #[cfg(feature="winapi_app")] union_field! { KEY_EVENT_RECORD_Child_4.{UnicodeChar, UnicodeChar_mut}: ::winnt::WCHAR } #[cfg(feature="winapi_app")] union_field! { KEY_EVENT_RECORD_Child_4.{AsciiChar, AsciiChar_mut}: ::winnt::CHAR } /* wincon.h:69:5, wincon.h:69:5, wincon.h:69:5 */
#[cfg(feature="winapi_app")] pub type PKEY_EVENT_RECORD = *mut ::wincon::KEY_EVENT_RECORD; /* wincon.h:74:22, wincon.h:74:22, wincon.h:74:22 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct MOUSE_EVENT_RECORD { dwMousePosition: ::wincon::COORD, dwButtonState: ::minwindef::DWORD, dwControlKeyState: ::minwindef::DWORD, dwEventFlags: ::minwindef::DWORD } /* wincon.h:97:16, wincon.h:97:16, wincon.h:97:16 */
#[cfg(feature="winapi_app")] pub type PMOUSE_EVENT_RECORD = *mut ::wincon::MOUSE_EVENT_RECORD; /* wincon.h:102:24, wincon.h:102:24, wincon.h:102:24 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct WINDOW_BUFFER_SIZE_RECORD { dwSize: ::wincon::COORD } /* wincon.h:125:16, wincon.h:125:16, wincon.h:125:16 */
#[cfg(feature="winapi_app")] pub type PWINDOW_BUFFER_SIZE_RECORD = *mut ::wincon::WINDOW_BUFFER_SIZE_RECORD; /* wincon.h:127:31, wincon.h:127:31, wincon.h:127:31 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct MENU_EVENT_RECORD { dwCommandId: ::minwindef::UINT } /* wincon.h:129:16, wincon.h:129:16, wincon.h:129:16 */
#[cfg(feature="winapi_app")] pub type PMENU_EVENT_RECORD = *mut ::wincon::MENU_EVENT_RECORD; /* wincon.h:131:23, wincon.h:131:23, wincon.h:131:23 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct FOCUS_EVENT_RECORD { bSetFocus: ::minwindef::BOOL } /* wincon.h:133:16, wincon.h:133:16, wincon.h:133:16 */
#[cfg(feature="winapi_app")] pub type PFOCUS_EVENT_RECORD = *mut ::wincon::FOCUS_EVENT_RECORD; /* wincon.h:135:24, wincon.h:135:24, wincon.h:135:24 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct INPUT_RECORD { EventType: ::minwindef::WORD, Event: ::wincon::INPUT_RECORD_Child_1 } /* wincon.h:137:16, wincon.h:137:16, wincon.h:137:16 */
#[cfg(feature="winapi_app")] #[repr(C)] pub /*union*/ struct INPUT_RECORD_Child_1 { _payload0: u32, _payload1: u64, _payload2: u32 } #[cfg(feature="winapi_app")] union_field! { INPUT_RECORD_Child_1.{KeyEvent, KeyEvent_mut}: ::wincon::KEY_EVENT_RECORD } #[cfg(feature="winapi_app")] union_field! { INPUT_RECORD_Child_1.{MouseEvent, MouseEvent_mut}: ::wincon::MOUSE_EVENT_RECORD } #[cfg(feature="winapi_app")] union_field! { INPUT_RECORD_Child_1.{WindowBufferSizeEvent, WindowBufferSizeEvent_mut}: ::wincon::WINDOW_BUFFER_SIZE_RECORD } #[cfg(feature="winapi_app")] union_field! { INPUT_RECORD_Child_1.{MenuEvent, MenuEvent_mut}: ::wincon::MENU_EVENT_RECORD } #[cfg(feature="winapi_app")] union_field! { INPUT_RECORD_Child_1.{FocusEvent, FocusEvent_mut}: ::wincon::FOCUS_EVENT_RECORD } /* wincon.h:139:5, wincon.h:139:5, wincon.h:139:5 */
#[cfg(feature="winapi_app")] pub type PINPUT_RECORD = *mut ::wincon::INPUT_RECORD; /* wincon.h:146:18, wincon.h:146:18, wincon.h:146:18 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct CHAR_INFO { Char: ::wincon::CHAR_INFO_Child_0, Attributes: ::minwindef::WORD } /* wincon.h:164:16, wincon.h:164:16, wincon.h:164:16 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub /*union*/ struct CHAR_INFO_Child_0 { _payload0: u16 } #[cfg(feature="winapi_desktop")] union_field! { CHAR_INFO_Child_0.{UnicodeChar, UnicodeChar_mut}: ::winnt::WCHAR } #[cfg(feature="winapi_desktop")] union_field! { CHAR_INFO_Child_0.{AsciiChar, AsciiChar_mut}: ::winnt::CHAR } /* wincon.h:165:5, wincon.h:165:5, wincon.h:165:5 */
#[cfg(feature="winapi_desktop")] pub type PCHAR_INFO = *mut ::wincon::CHAR_INFO; /* wincon.h:170:15, wincon.h:170:15, wincon.h:170:15 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct CONSOLE_SCREEN_BUFFER_INFO { dwSize: ::wincon::COORD, dwCursorPosition: ::wincon::COORD, wAttributes: ::minwindef::WORD, srWindow: ::wincon::SMALL_RECT, dwMaximumWindowSize: ::wincon::COORD } /* wincon.h:200:16, wincon.h:200:16, wincon.h:200:16 */
#[cfg(feature="winapi_desktop")] pub type PCONSOLE_SCREEN_BUFFER_INFO = *mut ::wincon::CONSOLE_SCREEN_BUFFER_INFO; /* wincon.h:206:32, wincon.h:206:32, wincon.h:206:32 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct CONSOLE_SCREEN_BUFFER_INFOEX { cbSize: ::minwindef::ULONG, dwSize: ::wincon::COORD, dwCursorPosition: ::wincon::COORD, wAttributes: ::minwindef::WORD, srWindow: ::wincon::SMALL_RECT, dwMaximumWindowSize: ::wincon::COORD, wPopupAttributes: ::minwindef::WORD, bFullscreenSupported: ::minwindef::BOOL, ColorTable: *mut [::windef::COLORREF; 16] } /* wincon.h:208:16, wincon.h:208:16, wincon.h:208:16 */
#[cfg(feature="winapi_desktop")] pub type PCONSOLE_SCREEN_BUFFER_INFOEX = *mut ::wincon::CONSOLE_SCREEN_BUFFER_INFOEX; /* wincon.h:218:34, wincon.h:218:34, wincon.h:218:34 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct CONSOLE_CURSOR_INFO { dwSize: ::minwindef::DWORD, bVisible: ::minwindef::BOOL } /* wincon.h:220:16, wincon.h:220:16, wincon.h:220:16 */
#[cfg(feature="winapi_desktop")] pub type PCONSOLE_CURSOR_INFO = *mut ::wincon::CONSOLE_CURSOR_INFO; /* wincon.h:223:25, wincon.h:223:25, wincon.h:223:25 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct CONSOLE_FONT_INFO { nFont: ::minwindef::DWORD, dwFontSize: ::wincon::COORD } /* wincon.h:225:16, wincon.h:225:16, wincon.h:225:16 */
#[cfg(feature="winapi_desktop")] pub type PCONSOLE_FONT_INFO = *mut ::wincon::CONSOLE_FONT_INFO; /* wincon.h:228:23, wincon.h:228:23, wincon.h:228:23 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct CONSOLE_FONT_INFOEX { cbSize: ::minwindef::ULONG, nFont: ::minwindef::DWORD, dwFontSize: ::wincon::COORD, FontFamily: ::minwindef::UINT, FontWeight: ::minwindef::UINT, FaceName: *mut [::winnt::WCHAR; 32] } /* wincon.h:231:16, wincon.h:231:16, wincon.h:231:16 */
#[cfg(feature="winapi_desktop")] pub type PCONSOLE_FONT_INFOEX = *mut ::wincon::CONSOLE_FONT_INFOEX; /* wincon.h:238:25, wincon.h:238:25, wincon.h:238:25 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct CONSOLE_HISTORY_INFO { cbSize: ::minwindef::UINT, HistoryBufferSize: ::minwindef::UINT, NumberOfHistoryBuffers: ::minwindef::UINT, dwFlags: ::minwindef::DWORD } /* wincon.h:243:16, wincon.h:243:16, wincon.h:243:16 */
#[cfg(feature="winapi_desktop")] pub type PCONSOLE_HISTORY_INFO = *mut ::wincon::CONSOLE_HISTORY_INFO; /* wincon.h:248:26, wincon.h:248:26, wincon.h:248:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct CONSOLE_SELECTION_INFO { dwFlags: ::minwindef::DWORD, dwSelectionAnchor: ::wincon::COORD, srSelection: ::wincon::SMALL_RECT } /* wincon.h:258:16, wincon.h:258:16, wincon.h:258:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PCONSOLE_SELECTION_INFO = *mut ::wincon::CONSOLE_SELECTION_INFO; /* wincon.h:262:28, wincon.h:262:28, wincon.h:262:28 */
#[cfg(feature="winapi_app")] pub type PHANDLER_ROUTINE = Option<extern "system" fn(::libc::c_ulong) -> ::libc::c_int>; /* wincon.h:287:10, wincon.h:287:10, wincon.h:287:10 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct CONSOLE_READCONSOLE_CONTROL { nLength: ::minwindef::ULONG, nInitialChars: ::minwindef::ULONG, dwCtrlWakeupMask: ::minwindef::ULONG, dwControlKeyState: ::minwindef::ULONG } /* wincon.h:804:16, wincon.h:804:16, wincon.h:804:16 */
#[cfg(feature="winapi_app")] pub type PCONSOLE_READCONSOLE_CONTROL = *mut ::wincon::CONSOLE_READCONSOLE_CONTROL; /* wincon.h:809:33, wincon.h:809:33, wincon.h:809:33 */
#[cfg(feature="winapi_app")] pub const RIGHT_ALT_PRESSED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wincon.h:80:9, wincon.h:80:9, wincon.h:80:9 */
#[cfg(feature="winapi_app")] pub const LEFT_ALT_PRESSED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wincon.h:81:9, wincon.h:81:9, wincon.h:81:9 */
#[cfg(feature="winapi_app")] pub const RIGHT_CTRL_PRESSED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wincon.h:82:9, wincon.h:82:9, wincon.h:82:9 */
#[cfg(feature="winapi_app")] pub const LEFT_CTRL_PRESSED: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wincon.h:83:9, wincon.h:83:9, wincon.h:83:9 */
#[cfg(feature="winapi_app")] pub const SHIFT_PRESSED: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wincon.h:84:9, wincon.h:84:9, wincon.h:84:9 */
#[cfg(feature="winapi_app")] pub const NUMLOCK_ON: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wincon.h:85:9, wincon.h:85:9, wincon.h:85:9 */
#[cfg(feature="winapi_app")] pub const SCROLLLOCK_ON: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wincon.h:86:9, wincon.h:86:9, wincon.h:86:9 */
#[cfg(feature="winapi_app")] pub const CAPSLOCK_ON: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wincon.h:87:9, wincon.h:87:9, wincon.h:87:9 */
#[cfg(feature="winapi_app")] pub const ENHANCED_KEY: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wincon.h:88:9, wincon.h:88:9, wincon.h:88:9 */
#[cfg(feature="winapi_app")] pub const NLS_DBCSCHAR: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* wincon.h:89:9, wincon.h:89:9, wincon.h:89:9 */
#[cfg(feature="winapi_app")] pub const NLS_ALPHANUMERIC: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wincon.h:90:9, wincon.h:90:9, wincon.h:90:9 */
#[cfg(feature="winapi_app")] pub const NLS_KATAKANA: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* wincon.h:91:9, wincon.h:91:9, wincon.h:91:9 */
#[cfg(feature="winapi_app")] pub const NLS_HIRAGANA: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* wincon.h:92:9, wincon.h:92:9, wincon.h:92:9 */
#[cfg(feature="winapi_app")] pub const NLS_ROMAN: i32 = 0x400000i32; /* Integer(4194304, Yes, Unknown) */ /* wincon.h:93:9, wincon.h:93:9, wincon.h:93:9 */
#[cfg(feature="winapi_app")] pub const NLS_IME_CONVERSION: i32 = 0x800000i32; /* Integer(8388608, Yes, Unknown) */ /* wincon.h:94:9, wincon.h:94:9, wincon.h:94:9 */
#[cfg(feature="winapi_app")] pub const NLS_IME_DISABLE: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* wincon.h:95:9, wincon.h:95:9, wincon.h:95:9 */
#[cfg(feature="winapi_app")] pub const FROM_LEFT_1ST_BUTTON_PRESSED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wincon.h:108:9, wincon.h:108:9, wincon.h:108:9 */
#[cfg(feature="winapi_app")] pub const RIGHTMOST_BUTTON_PRESSED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wincon.h:109:9, wincon.h:109:9, wincon.h:109:9 */
#[cfg(feature="winapi_app")] pub const FROM_LEFT_2ND_BUTTON_PRESSED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wincon.h:110:9, wincon.h:110:9, wincon.h:110:9 */
#[cfg(feature="winapi_app")] pub const FROM_LEFT_3RD_BUTTON_PRESSED: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wincon.h:111:9, wincon.h:111:9, wincon.h:111:9 */
#[cfg(feature="winapi_app")] pub const FROM_LEFT_4TH_BUTTON_PRESSED: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wincon.h:112:9, wincon.h:112:9, wincon.h:112:9 */
#[cfg(feature="winapi_app")] pub const MOUSE_MOVED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wincon.h:118:9, wincon.h:118:9, wincon.h:118:9 */
#[cfg(feature="winapi_app")] pub const DOUBLE_CLICK: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wincon.h:119:9, wincon.h:119:9, wincon.h:119:9 */
#[cfg(feature="winapi_app")] pub const MOUSE_WHEELED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wincon.h:120:9, wincon.h:120:9, wincon.h:120:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06000000"))] pub const MOUSE_HWHEELED: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wincon.h:122:9, wincon.h:122:9, wincon.h:122:9 */
pub const KEY_EVENT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wincon.h:155:9, wincon.h:155:9, wincon.h:155:9 */
pub const MOUSE_EVENT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wincon.h:156:9, wincon.h:156:9, wincon.h:156:9 */
pub const WINDOW_BUFFER_SIZE_EVENT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wincon.h:157:9, wincon.h:157:9, wincon.h:157:9 */
pub const MENU_EVENT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wincon.h:158:9, wincon.h:158:9, wincon.h:158:9 */
pub const FOCUS_EVENT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wincon.h:159:9, wincon.h:159:9, wincon.h:159:9 */
pub const FOREGROUND_BLUE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wincon.h:179:9, wincon.h:179:9, wincon.h:179:9 */
pub const FOREGROUND_GREEN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wincon.h:180:9, wincon.h:180:9, wincon.h:180:9 */
pub const FOREGROUND_RED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wincon.h:181:9, wincon.h:181:9, wincon.h:181:9 */
pub const FOREGROUND_INTENSITY: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wincon.h:182:9, wincon.h:182:9, wincon.h:182:9 */
pub const BACKGROUND_BLUE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wincon.h:183:9, wincon.h:183:9, wincon.h:183:9 */
pub const BACKGROUND_GREEN: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wincon.h:184:9, wincon.h:184:9, wincon.h:184:9 */
pub const BACKGROUND_RED: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wincon.h:185:9, wincon.h:185:9, wincon.h:185:9 */
pub const BACKGROUND_INTENSITY: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wincon.h:186:9, wincon.h:186:9, wincon.h:186:9 */
pub const COMMON_LVB_LEADING_BYTE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wincon.h:187:9, wincon.h:187:9, wincon.h:187:9 */
pub const COMMON_LVB_TRAILING_BYTE: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* wincon.h:188:9, wincon.h:188:9, wincon.h:188:9 */
pub const COMMON_LVB_GRID_HORIZONTAL: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* wincon.h:189:9, wincon.h:189:9, wincon.h:189:9 */
pub const COMMON_LVB_GRID_LVERTICAL: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* wincon.h:190:9, wincon.h:190:9, wincon.h:190:9 */
pub const COMMON_LVB_GRID_RVERTICAL: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* wincon.h:191:9, wincon.h:191:9, wincon.h:191:9 */
pub const COMMON_LVB_REVERSE_VIDEO: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* wincon.h:192:9, wincon.h:192:9, wincon.h:192:9 */
pub const COMMON_LVB_UNDERSCORE: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* wincon.h:193:9, wincon.h:193:9, wincon.h:193:9 */
pub const COMMON_LVB_SBCSDBCS: i32 = 0x300i32; /* Integer(768, Yes, Unknown) */ /* wincon.h:195:9, wincon.h:195:9, wincon.h:195:9 */
#[cfg(feature="winapi_desktop")] pub const HISTORY_NO_DUP_FLAG: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wincon.h:241:9, wincon.h:241:9, wincon.h:241:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CONSOLE_NO_SELECTION: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wincon.h:271:9, wincon.h:271:9, wincon.h:271:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CONSOLE_SELECTION_IN_PROGRESS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wincon.h:272:9, wincon.h:272:9, wincon.h:272:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CONSOLE_SELECTION_NOT_EMPTY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wincon.h:273:9, wincon.h:273:9, wincon.h:273:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CONSOLE_MOUSE_SELECTION: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wincon.h:274:9, wincon.h:274:9, wincon.h:274:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CONSOLE_MOUSE_DOWN: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wincon.h:275:9, wincon.h:275:9, wincon.h:275:9 */
pub const CTRL_C_EVENT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wincon.h:294:9, wincon.h:294:9, wincon.h:294:9 */
pub const CTRL_BREAK_EVENT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wincon.h:295:9, wincon.h:295:9, wincon.h:295:9 */
pub const CTRL_CLOSE_EVENT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wincon.h:296:9, wincon.h:296:9, wincon.h:296:9 */
pub const CTRL_LOGOFF_EVENT: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wincon.h:299:9, wincon.h:299:9, wincon.h:299:9 */
pub const CTRL_SHUTDOWN_EVENT: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wincon.h:300:9, wincon.h:300:9, wincon.h:300:9 */
pub const ENABLE_PROCESSED_INPUT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wincon.h:306:9, wincon.h:306:9, wincon.h:306:9 */
pub const ENABLE_LINE_INPUT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wincon.h:307:9, wincon.h:307:9, wincon.h:307:9 */
pub const ENABLE_ECHO_INPUT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wincon.h:308:9, wincon.h:308:9, wincon.h:308:9 */
pub const ENABLE_WINDOW_INPUT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wincon.h:309:9, wincon.h:309:9, wincon.h:309:9 */
pub const ENABLE_MOUSE_INPUT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wincon.h:310:9, wincon.h:310:9, wincon.h:310:9 */
pub const ENABLE_INSERT_MODE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wincon.h:311:9, wincon.h:311:9, wincon.h:311:9 */
pub const ENABLE_QUICK_EDIT_MODE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wincon.h:312:9, wincon.h:312:9, wincon.h:312:9 */
pub const ENABLE_EXTENDED_FLAGS: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wincon.h:313:9, wincon.h:313:9, wincon.h:313:9 */
pub const ENABLE_AUTO_POSITION: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wincon.h:314:9, wincon.h:314:9, wincon.h:314:9 */
pub const ENABLE_PROCESSED_OUTPUT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wincon.h:320:9, wincon.h:320:9, wincon.h:320:9 */
pub const ENABLE_WRAP_AT_EOL_OUTPUT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wincon.h:321:9, wincon.h:321:9, wincon.h:321:9 */
pub const CONSOLE_TEXTMODE_BUFFER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wincon.h:814:9, wincon.h:814:9, wincon.h:814:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const CONSOLE_FULLSCREEN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wincon.h:850:9, wincon.h:850:9, wincon.h:850:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const CONSOLE_FULLSCREEN_HARDWARE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wincon.h:851:9, wincon.h:851:9, wincon.h:851:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const CONSOLE_FULLSCREEN_MODE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wincon.h:859:9, wincon.h:859:9, wincon.h:859:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const CONSOLE_WINDOWED_MODE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wincon.h:860:9, wincon.h:860:9, wincon.h:860:9 */
