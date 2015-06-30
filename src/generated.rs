/*!
This module simply provides access to generated modules.
*/

/**
This module contains the `WinVersion` enumeration.
*/
pub mod winver {
    include!(concat!(env!("OUT_DIR"), "/src/winver.rs"));
}
