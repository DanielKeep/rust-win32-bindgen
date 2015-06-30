# `win32-bindgen`

This is an attempt to build a mostly-automated binding generator for the Windows API by processing `windows.h`.

In particular, it aims to generate a *mostly* complete binding, taking care to guard declarations based on target Windows version, API partition and architecture; something most bindings do not bother with.

It is based on Clang's CIndex API and lots of horrible hacks.

It is very much **not ready for use**.

## Example `local/config.json`

Here is the current `local/config.json` I am using for testing.  This is done mostly with the headers from the Windows SDK for Windows 8.1.

```json
{
    "header": "local\\Include\\um\\windows.h",
    "expansion_configs": [
        {
            "architecture": "X86_32",
            "windows_version_short": "WIN8",
            "windows_version_full": "WINBLUE",
            "native_calling_conv": "Stdcall"
        },
        {
            "architecture": "X86_64",
            "windows_version_short": "WIN8",
            "windows_version_full": "WINBLUE",
            "native_calling_conv": "C"
        },
        {
            "architecture": "Arm",
            "windows_version_short": "WIN8",
            "windows_version_full": "WINBLUE",
            "native_calling_conv": "C"
        }
    ],
    "ignore_decl_spellings": [
        "^_", "__"
    ],
    "ignore_file_paths": [
        "^$",
        "\\\\sdkddkver.h$"
    ],
    "switches": [
        "-fms-extensions",
        "-fms-compatibility",
        "-D_MSC_VER=1800",
        "-D_STDCALL_SUPPORTED",
        "-DWIN32_LEAN_AND_MEAN",
        "-DSECURITY_WIN32",
        "-Ilocal/Include/shared",
        "-Ilocal/Include/um"
    ]
}
```
