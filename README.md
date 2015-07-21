# `win32-bindgen`

This is an attempt to build a mostly-automated binding generator for the Windows API by processing `windows.h`.

In particular, it aims to generate a *mostly* complete binding, taking care to guard declarations based on target Windows version, API partition and architecture; something most bindings do not bother with.

It is based on Clang's CIndex API and lots of horrible hacks.

It is very much **not ready for use**.

## Example `local/config.json`

Here is the current `local/config.json` I am using for testing.  This is done mostly with the headers from the Windows SDK for Windows 8.1.

```json
{
    "header": "local\\include\\um\\windows.h",
    "generation": {
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
        "dont_ignore_decl_spelling": [
            "^H[A-Z]+__",
            "^_P?IMAGE_",
            "SC_HANDLE__",
            "SERVICE_STATUS_HANDLE__"
        ],
        "ignore_decls": [
            ["minwindef", 20, "INT"],
            ["sysinfoapi", 8, "VerSetConditionMask"],
            ["synchapi", 8, "SignalObjectAndWait"],
            ["windef", 501, "DM_UPDATE"],
            ["windef", 501, "DM_COPY"],
            ["windef", 501, "DM_PROMPT"],
            ["windef", 501, "DM_MODIFY"],
            ["windef", 501, "DM_IN_BUFFER"],
            ["windef", 501, "DM_IN_PROMPT"],
            ["windef", 501, "DM_OUT_BUFFER"],
            ["windef", 501, "DM_OUT_DEFAULT"],
            ["windef", 501, "DC_FIELDS"],
            ["windef", 501, "DC_PAPERS"],
            ["windef", 501, "DC_PAPERSIZE"],
            ["windef", 501, "DC_MINEXTENT"],
            ["windef", 501, "DC_MAXEXTENT"],
            ["windef", 501, "DC_BINS"],
            ["windef", 501, "DC_DUPLEX"],
            ["windef", 501, "DC_SIZE"],
            ["windef", 501, "DC_EXTRA"],
            ["windef", 501, "DC_VERSION"],
            ["windef", 501, "DC_DRIVER"],
            ["windef", 501, "DC_BINNAMES"],
            ["windef", 501, "DC_ENUMRESOLUTIONS"],
            ["windef", 501, "DC_FILEDEPENDENCIES"],
            ["windef", 501, "DC_TRUETYPE"],
            ["windef", 501, "DC_PAPERNAMES"],
            ["windef", 501, "DC_ORIENTATION"],
            ["windef", 501, "DC_COPIES"],
            ["winuser", 501, "MOD_ALT"],
            ["winuser", 501, "MOD_CONTROL"],
            ["winuser", 501, "MOD_SHIFT"]
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
            "-DUNICODE",
            "-D_WCHAR_T_DEFINED",
            "-Dwchar_t=unsigned short",
            "-Ilocal/Include/shared",
            "-Ilocal/Include/um"
        ],
        "non_canonical_tag_names": [
            "^$",
            "^tag[A-Z_]",
            "^_",
            "__$"
        ]
    },
    "output": {
        "output_dir": "local/output",
        "header_path": "headers/{}.rs",
        "library_path": "libraries/{}.rs",
        "function_library_map": "local/winsdk-symbols.lst",
        "function_library_fallback": "other"
    }
}
```

## Symbol Map

You need to generate a "symbol map" that indicates which library each function is included in.  It needs to be a text file where each line is of the form `FuncName: lib1 lib2 ...`.
