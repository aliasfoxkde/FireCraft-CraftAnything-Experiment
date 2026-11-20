# What we found in C:\Program Files (x86)\Adobe\Adobe Fireworks CS6\Fireworks.exe

Fireworks.exe is a program for Windows, built for x86 (32-bit). Its internal names were removed (stripped), so most functions will have generated names.

Evidence ev_9739f65b824728f25e9b585dde1e5509a819754ef3263d0e87c1cff248fe01df.

```json
{
  "architecture": "x86",
  "bits": 32,
  "dotnet_arch": null,
  "entry_point": "0x10f8cb0",
  "format": "pe",
  "format_name": "PE (Windows)",
  "kind": "executable",
  "libraries": [
    "ADVAPI32.dll",
    "AdobeOwl.dll",
    "AdobeOwlCanvas.dll",
    "COMCTL32.dll",
    "COMDLG32.dll",
    "GDI32.dll",
    "IMM32.dll",
    "KERNEL32.dll",
    "LogSession.dll",
    "MSIMG32.dll",
    "MSVCP100.dll",
    "MSVCR100.dll",
    "OLEAUT32.dll",
    "PlugPlug.dll",
    "RPCRT4.dll",
    "SHELL32.dll",
    "SHLWAPI.dll",
    "ScCore.dll",
    "USER32.dll",
    "VERSION.dll",
    "WINSPOOL.DRV",
    "WRServices.dll",
    "WS2_32.dll",
    "Workspace.dll",
    "ahclient.dll",
    "amtlib.dll",
    "boost_threads.dll",
    "dbghelp.dll",
    "gdiplus.dll",
    "giflib.dll",
    "jslib.dll",
    "libpng.dll"
  ],
  "next_steps": [
    {
      "tool": "core_overview_show",
      "why": "a one-page summary: sections, imports, counts"
    },
    {
      "tool": "native_functions_list",
      "why": "every function we found"
    },
    {
      "tool": "native_strings_find",
      "why": "find text the program shows or uses"
    },
    {
      "tool": "native_function_study",
      "why": "a full dossier on one function (start with main or the entry point)"
    },
    {
      "tool": "core_feature_find",
      "why": "follow a word you see in the app to the code behind it"
    },
    {
      "tool": "native_pe_resources_list",
      "why": "icons, version information and the manifest"
    }
  ],
  "operating_system": "Windows",
  "plain_english": "Fireworks.exe is a program for Windows, built for x86 (32-bit). Its internal names were removed (stripped), so most functions will have generated names.",
  "recommended_toolsets": [
    "core",
    "native",
    "formats"
  ],
  "stripped": true,
  "symbol_table_stripped": true,
  "toolchain_hints": [
    {
      "category": "compiler",
      "evidence": "Rich header",
      "id": "msvc",
      "label": "Microsoft Visual C++ toolchain",
      "strength": "strong"
    }
  ]
}
```

## Ghidra

Deep mode: Ghidra has analysed C:\Program Files (x86)\Adobe\Adobe Fireworks CS6\Fireworks.exe (24 min), so decompiling it is quick.

## How it looks running

capture/window.png is a picture of its window 15 seconds after it started: match its layout, colours and proportions.
