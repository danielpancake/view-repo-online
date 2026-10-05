mod remote;

use std::{ffi::c_void, path::PathBuf};
use windows::{
    Win32::Foundation::*, Win32::System::Com::*, Win32::UI::Shell::*,
    Win32::UI::WindowsAndMessaging::*, core::*,
};

// Must match every CLSID in AppxManifest.xml
const CLSID: GUID = GUID::from_u128(0x9e3579f8_17d2_4bb8_b9a2_d315a572ae41);

#[implement(IExplorerCommand)]
struct TestCommand;

impl IExplorerCommand_Impl for TestCommand_Impl {
    fn GetTitle(&self, _: Ref<IShellItemArray>) -> Result<PWSTR> {
        unsafe { SHStrDupW(w!("View repository online")) }
    }

    fn GetState(&self, _: Ref<IShellItemArray>, _: BOOL) -> Result<u32> {
        Ok(ECS_ENABLED.0 as u32)
    }

    fn Invoke(&self, items: Ref<IShellItemArray>, _: Ref<IBindCtx>) -> Result<()> {
        match remote::repository_url(&folder(items)?) {
            Ok(url) => unsafe {
                ShellExecuteW(
                    None,
                    w!("open"),
                    &HSTRING::from(url),
                    None,
                    None,
                    SW_SHOWNORMAL,
                );
            },
            Err(message) => unsafe {
                MessageBoxW(
                    None,
                    &HSTRING::from(message),
                    w!("View repository online"),
                    MB_ICONERROR,
                );
            },
        }
        Ok(())
    }

    fn GetFlags(&self) -> Result<u32> {
        Ok(ECF_DEFAULT.0 as u32)
    }

    fn GetCanonicalName(&self) -> Result<GUID> {
        Ok(CLSID)
    }

    fn GetIcon(&self, _: Ref<IShellItemArray>) -> Result<PWSTR> {
        Err(E_NOTIMPL.into())
    }

    fn GetToolTip(&self, _: Ref<IShellItemArray>) -> Result<PWSTR> {
        Err(E_NOTIMPL.into())
    }

    fn EnumSubCommands(&self) -> Result<IEnumExplorerCommand> {
        Err(E_NOTIMPL.into())
    }
}

/// The right-clicked folder, or the open folder when its background was clicked
fn folder(items: Ref<IShellItemArray>) -> Result<PathBuf> {
    unsafe {
        let path = items
            .ok()?
            .GetItemAt(0)?
            .GetDisplayName(SIGDN_FILESYSPATH)?;

        let folder = path.to_hstring();

        CoTaskMemFree(Some(path.0 as _));
        Ok(folder.to_os_string().into())
    }
}

#[implement(IClassFactory)]
struct Factory;

impl IClassFactory_Impl for Factory_Impl {
    fn CreateInstance(
        &self,
        _: Ref<IUnknown>,
        iid: *const GUID,
        object: *mut *mut c_void,
    ) -> Result<()> {
        let command: IExplorerCommand = TestCommand.into();
        unsafe { command.query(iid, object).ok() }
    }

    fn LockServer(&self, _: BOOL) -> Result<()> {
        Ok(())
    }
}

/// Actual DLL export
///
/// # Safety
/// Called by COM with valid pointers
#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllGetClassObject(
    clsid: *const GUID,
    iid: *const GUID,
    object: *mut *mut c_void,
) -> HRESULT {
    if unsafe { *clsid } != CLSID {
        return CLASS_E_CLASSNOTAVAILABLE;
    }

    let factory: IClassFactory = Factory.into();
    unsafe { factory.query(iid, object) }
}
