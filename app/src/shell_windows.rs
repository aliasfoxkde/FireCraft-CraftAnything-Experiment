//! Windows shell integration: an explicit AppUserModelID plus a taskbar Jump List with recent
//! documents and common tasks.
//!
//! All failures are non-fatal. If COM is unavailable (locked-down session, unsupported shell, or a
//! Wine environment), FireCraft still starts normally and logs the reason to stderr.

use std::path::PathBuf;

use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
};
use windows::Win32::UI::Shell::Common::{IObjectArray, IObjectCollection};
use windows::Win32::UI::Shell::{
    ICustomDestinationList, IShellLinkW, SetCurrentProcessExplicitAppUserModelID,
};
use windows::core::{GUID, HSTRING, Interface};

pub const APP_USER_MODEL_ID: &str = "FireCraft.Studio.Editor";

const CLSID_DESTINATION_LIST: GUID = GUID::from_u128(0x77f10cf0_3db5_4966_b520_b7c54fd35ed6);
const CLSID_ENUMERABLE_OBJECT_COLLECTION: GUID =
    GUID::from_u128(0x2d3468c1_36a7_43b6_ac24_d3f02fd9607a);
const CLSID_SHELL_LINK: GUID = GUID::from_u128(0x00021401_0000_0000_c000_000000000046);

/// Must be called before the first window is created so Windows groups the taskbar button under
/// FireCraft instead of the host process.
pub fn init_app_id() {
    if let Err(err) = unsafe {
        SetCurrentProcessExplicitAppUserModelID(&HSTRING::from(APP_USER_MODEL_ID))
            .map_err(|e| e.to_string())
    } {
        eprintln!("FireCraft: could not set AppUserModelID: {err}");
    }
}

/// Replaces the current Jump List. Called at startup (to install tasks) and whenever the recent
/// document list changes.
pub fn update_jump_list(recent_files: &[PathBuf]) {
    if let Err(err) = unsafe { rebuild(recent_files) } {
        // A missing shell, disabled taskbar, or Wine is not a startup blocker.
        eprintln!("FireCraft: Jump List unavailable: {err}");
    }
}

unsafe fn rebuild(recent_files: &[PathBuf]) -> Result<(), String> {
    let _ = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe_w = HSTRING::from(exe.to_string_lossy().as_ref());

    let destinations: ICustomDestinationList =
        unsafe { CoCreateInstance(&CLSID_DESTINATION_LIST, None, CLSCTX_INPROC_SERVER) }
            .map_err(|e| e.to_string())?;
    unsafe { destinations.SetAppID(&HSTRING::from(APP_USER_MODEL_ID)) }
        .map_err(|e| e.to_string())?;

    let mut min_slots = 0u32;
    let _removed: IObjectArray =
        unsafe { destinations.BeginList(&mut min_slots) }.map_err(|e| e.to_string())?;

    if !recent_files.is_empty() {
        let recent = unsafe { new_collection() }?;
        for path in recent_files.iter().filter(|p| p.is_file()).take(10) {
            let display = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.display().to_string());
            let link = unsafe { new_link(&exe_w, &format!("\"{}\"", path.display()), &display)? };
            unsafe { recent.AddObject(&link) }.map_err(|e| e.to_string())?;
        }
        let recent_array: IObjectArray = recent.cast().map_err(|e| e.to_string())?;
        unsafe { destinations.AppendCategory(&HSTRING::from("Recent Documents"), &recent_array) }
            .map_err(|e| e.to_string())?;
    }

    let tasks = unsafe { new_collection() }?;
    for (label, args) in [("New Document", "--new"), ("About FireCraft", "--about")] {
        let link = unsafe { new_link(&exe_w, args, label)? };
        unsafe { tasks.AddObject(&link) }.map_err(|e| e.to_string())?;
    }
    let task_array: IObjectArray = tasks.cast().map_err(|e| e.to_string())?;
    unsafe { destinations.AddUserTasks(&task_array) }.map_err(|e| e.to_string())?;
    unsafe { destinations.CommitList() }.map_err(|e| e.to_string())?;
    Ok(())
}

unsafe fn new_collection() -> Result<IObjectCollection, String> {
    unsafe {
        CoCreateInstance(
            &CLSID_ENUMERABLE_OBJECT_COLLECTION,
            None,
            CLSCTX_INPROC_SERVER,
        )
    }
    .map_err(|e| e.to_string())
}

unsafe fn new_link(
    executable: &HSTRING,
    arguments: &str,
    description: &str,
) -> Result<IShellLinkW, String> {
    let link: IShellLinkW =
        unsafe { CoCreateInstance(&CLSID_SHELL_LINK, None, CLSCTX_INPROC_SERVER) }
            .map_err(|e| e.to_string())?;
    unsafe { link.SetPath(executable) }.map_err(|e| e.to_string())?;
    unsafe { link.SetArguments(&HSTRING::from(arguments)) }.map_err(|e| e.to_string())?;
    unsafe { link.SetDescription(&HSTRING::from(description)) }.map_err(|e| e.to_string())?;
    unsafe { link.SetIconLocation(executable, 0) }.map_err(|e| e.to_string())?;
    Ok(link)
}
