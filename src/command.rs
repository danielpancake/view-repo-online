use crate::{CLSID, remote, theme::Theme};
use std::path::PathBuf;
use windows::{
    ApplicationModel::Package, Win32::Foundation::*, Win32::System::Com::*, Win32::UI::HiDpi::*,
    Win32::UI::Shell::*, Win32::UI::WindowsAndMessaging::*, core::*,
};

#[implement(IExplorerCommand)]
pub struct ViewRepositoryCommand;

impl IExplorerCommand_Impl for ViewRepositoryCommand_Impl {
    fn GetTitle(&self, _: Ref<IShellItemArray>) -> Result<PWSTR> {
        unsafe { SHStrDupW(w!("View repository online")) }
    }

    fn GetState(&self, items: Ref<IShellItemArray>, _: BOOL) -> Result<u32> {
        let state = match folder(items) {
            Ok(folder) if remote::is_repository(&folder) => ECS_ENABLED,
            _ => ECS_HIDDEN,
        };
        Ok(state.0 as u32)
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
            Err(message) => show_error(&message),
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
        let icon = match Theme::current() {
            Theme::Light => "menu-light.ico",
            Theme::Dark => "menu-dark.ico",
        };

        let folder = Package::Current()?.InstalledPath()?;
        unsafe { SHStrDupW(&HSTRING::from(format!(r"{folder}\Assets\{icon}"))) }
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

fn show_error(message: &str) {
    unsafe {
        // dllhost.exe isn't DPI aware
        let previous = SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);

        MessageBoxW(
            None,
            &HSTRING::from(message),
            w!("View repository online"),
            MB_ICONERROR | MB_SETFOREGROUND | MB_TOPMOST,
        );

        SetThreadDpiAwarenessContext(previous);
    }
}
