mod command;
mod remote;
mod theme;

use command::ViewRepositoryCommand;
use std::ffi::c_void;
use windows::{
    Win32::Foundation::*, Win32::System::Com::*, Win32::UI::Shell::IExplorerCommand, core::*,
};

// Must match every CLSID in AppxManifest.xml
const CLSID: GUID = GUID::from_u128(0x9e3579f8_17d2_4bb8_b9a2_d315a572ae41);

#[implement(IClassFactory)]
struct Factory;

impl IClassFactory_Impl for Factory_Impl {
    fn CreateInstance(
        &self,
        _: Ref<IUnknown>,
        iid: *const GUID,
        object: *mut *mut c_void,
    ) -> Result<()> {
        let command: IExplorerCommand = ViewRepositoryCommand.into();
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
