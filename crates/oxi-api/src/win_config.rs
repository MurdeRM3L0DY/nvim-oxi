use oxi_types::{self as nvim, conversion::FromObject};

use crate::choose;
use crate::ffi::win_config::*;
use crate::types::*;
use crate::Result;
use crate::{Buffer, Window};

/// Binding to [`nvim_open_win()`][1].
///
/// Opens a new floating or external window.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_open_win()
pub fn open_win(
    buf: &Buffer,
    enter: bool,
    config: &WindowConfig,
) -> Result<Window> {
    let mut err = nvim::Error::new();
    let handle =
        unsafe { nvim_open_win(buf.0, enter, &config.into(), &mut err) };
    choose!(err, Ok(handle.into()))
}

impl Window {
    /// Binding to [`nvim_win_get_config()`][1].
    ///
    /// Gets the window configuration.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_get_config()
    pub fn get_config(&self) -> Result<WindowConfig> {
        let mut err = nvim::Error::new();
        let mut dict = unsafe { nvim_win_get_config(self.0, &mut err) };
        let win = dict.get("win").map(|obj| unsafe {
            // SAFETY: if the `win` key is present it's set to an integer
            // representing a window handle.
            obj.as_integer_unchecked() as i32
        });
        if let Some(handle) = win {
            dict["relative"] = handle.into();
        }
        choose!(err, Ok(WindowConfig::from_object(dict.into())?))
    }

    /// Binding to [`nvim_win_get_config()`][1].
    ///
    /// Configures the window layout. Only for floating and external windows.
    ///
    /// [1]: https://neovim.io/doc/user/api.html#nvim_win_get_config()
    pub fn set_config(&mut self, config: &WindowConfig) -> Result<()> {
        let mut err = nvim::Error::new();
        unsafe { nvim_win_set_config(self.0, &config.into(), &mut err) };
        choose!(err, ())
    }
}
