use std::ffi::c_void;
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
        unsafe { SHStrDupW(w!("Say hello")) }
    }

    fn GetState(&self, _: Ref<IShellItemArray>, _: BOOL) -> Result<u32> {
        Ok(ECS_ENABLED.0 as u32)
    }

    fn Invoke(&self, _: Ref<IShellItemArray>, _: Ref<IBindCtx>) -> Result<()> {
        unsafe {
            MessageBoxW(None, w!("Hellow world!"), w!("TITLE TEXT"), MB_OK);
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
