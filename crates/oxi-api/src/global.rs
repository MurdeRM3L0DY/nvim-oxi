use std::path::{Path, PathBuf};

use oxi_types::{
    self as nvim,
    conversion::{FromObject, ToObject},
    Array,
    Dictionary,
    Integer,
    Object,
};

use crate::choose;
use crate::ffi::global::*;
use crate::opts::*;
use crate::types::*;
use crate::StringOrFunction;
use crate::SuperIterator;
use crate::LUA_INTERNAL_CALL;
use crate::{Buffer, TabPage, Window};
use crate::{Error, Result};

/// Binding to [`nvim_chan_send()`][1].
///
/// Sends data to a channel.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_chan_send()
pub fn chan_send(channel_id: u32, data: &str) -> Result<()> {
    let mut err = nvim::Error::new();
    let data = nvim::String::from(data);
    unsafe { nvim_chan_send(channel_id.into(), data.non_owning(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_create_buf()`][1].
///
/// Creates a new, empty, unnamed buffer.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_create_buf()
pub fn create_buf(is_listed: bool, is_scratch: bool) -> Result<Buffer> {
    let mut err = nvim::Error::new();
    let handle = unsafe { nvim_create_buf(is_listed, is_scratch, &mut err) };
    choose!(err, Ok(handle.into()))
}

/// Binding to [`nvim_create_user_command()`][1].
///
/// Creates a new [user command](https://neovim.io/doc/user/map.html#user-commands).
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_create_user_command()
pub fn create_user_command<Cmd>(
    name: &str,
    command: Cmd,
    opts: &CreateCommandOpts,
) -> Result<()>
where
    Cmd: StringOrFunction<CommandArgs, ()>,
{
    let name = nvim::String::from(name);
    let command = command.to_object();
    let mut err = nvim::Error::new();
    unsafe {
        nvim_create_user_command(
            #[cfg(not(feature = "neovim-0-8"))]
            LUA_INTERNAL_CALL,
            name.non_owning(),
            command.non_owning(),
            opts,
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_del_current_line()`][1].
///
/// Deletes the current line.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_del_current_line()
pub fn del_current_line() -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_del_current_line(&mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_del_keymap()`][1].
///
/// Unmaps a global mapping for the given mode. To unmap a buffer-local mapping
/// use [`Buffer::del_keymap`] instead.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_del_keymap()
pub fn del_keymap(mode: Mode, lhs: &str) -> Result<()> {
    let mode = nvim::String::from(mode);
    let lhs = nvim::String::from(lhs);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_del_keymap(
            LUA_INTERNAL_CALL,
            mode.non_owning(),
            lhs.non_owning(),
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_del_mark()`][1].
///
/// Deletes an uppercase/file named mark. Returns an error if a lowercase or
/// buffer-local named mark is used. Use [`Buffer::del_mark`] to delete a
/// buffer-local mark.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_del_mark()
pub fn del_mark(name: char) -> Result<()> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let was_deleted = unsafe { nvim_del_mark(name.non_owning(), &mut err) };
    choose!(
        err,
        match was_deleted {
            true => Ok(()),
            _ => Err(Error::custom("Couldn't delete mark")),
        }
    )
}

/// Binding to [`nvim_del_user_command()`][1].
///
/// Deletes a global user-defined command.  Use [`Buffer::del_user_command`] to
/// delete a buffer-local command.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_del_user_command()
pub fn del_user_command(name: &str) -> Result<()> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    unsafe { nvim_del_user_command(name.non_owning(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_del_var()`][1].
///
/// Removes a global (`g:`) variable.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_del_var()
pub fn del_var(name: &str) -> Result<()> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    unsafe { nvim_del_var(name.non_owning(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_echo()`][1].
///
/// Echoes a message to the Neovim message area.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_echo()
pub fn echo<'hl, Text, Chunks>(chunks: Chunks, history: bool) -> Result<()>
where
    Chunks: IntoIterator<Item = (Text, Option<&'hl str>)>,
    Text: Into<nvim::String>,
{
    let chunks = chunks
        .into_iter()
        .map(|(text, hlgroup)| {
            Array::from_iter([
                Object::from(text.into()),
                Object::from(hlgroup.map(|hl| hl.to_owned())),
            ])
        })
        .collect::<Array>();

    let mut err = nvim::Error::new();
    let opts = Dictionary::new();
    unsafe {
        nvim_echo(chunks.non_owning(), history, opts.non_owning(), &mut err)
    };
    choose!(err, ())
}

/// Binding to [`nvim_err_write()`][1].
///
/// Writes a message to the Neovim error buffer. Does not append a newline
/// (`"\n"`); the message gets buffered and won't be displayed until a linefeed
/// is written.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_err_write()
pub fn err_write(str: &str) {
    unsafe { nvim_err_write(nvim::String::from(str).non_owning()) }
}

/// Binding to [`nvim_err_writeln()`][1].
///
/// Writes a message to the Neovim error buffer. Appends a newline (`"\n"`), so
/// the buffer is flushed and displayed.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_err_writeln()
pub fn err_writeln(str: &str) {
    unsafe { nvim_err_writeln(nvim::String::from(str).non_owning()) }
}

/// Binding to [`nvim_eval_statusline()`][1].
///
/// Evaluates a string to be displayed in the statusline.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_eval_statusline()
pub fn eval_statusline(
    str: &str,
    opts: &EvalStatuslineOpts,
) -> Result<StatuslineInfos> {
    let str = nvim::String::from(str);
    let mut err = nvim::Error::new();
    let dict =
        unsafe { nvim_eval_statusline(str.non_owning(), opts, &mut err) };
    choose!(err, Ok(StatuslineInfos::from_object(dict.into())?))
}

/// Binding to [`nvim_feedkeys()`][1].
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_feedkeys()
pub fn feedkeys(keys: &str, mode: Mode, escape_ks: bool) {
    let keys = nvim::String::from(keys);
    let mode = nvim::String::from(mode);
    unsafe { nvim_feedkeys(keys.non_owning(), mode.non_owning(), escape_ks) }
}

/// Binding to [`nvim_get_all_options_info()`][1].
///
/// Gets the option information for all options.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_all_options_info()
pub fn get_all_options_info() -> Result<impl SuperIterator<OptionInfos>> {
    let mut err = nvim::Error::new();
    let infos = unsafe { nvim_get_all_options_info(&mut err) };
    choose!(
        err,
        Ok({
            infos
                .into_iter()
                .map(|(_, optinf)| OptionInfos::from_object(optinf).unwrap())
        })
    )
}

/// Binding to [`nvim_get_chan_info()`][1].
///
/// Gets information about a channel.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_chan_info()
pub fn get_chan_info(channel_id: u32) -> Result<ChannelInfos> {
    let mut err = nvim::Error::new();
    let infos = unsafe { nvim_get_chan_info(channel_id.into(), &mut err) };
    choose!(err, Ok(ChannelInfos::from_object(infos.into())?))
}

/// Binding to [`nvim_get_color_by_name()`][1].
///
/// Returns the 24-bit RGB value of a `crate::api::get_color_map` color name or
/// "#rrggbb" hexadecimal string.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_color_by_name()
pub fn get_color_by_name(name: &str) -> Result<u32> {
    let name = nvim::String::from(name);
    let color = unsafe { nvim_get_color_by_name(name.non_owning()) };
    (color != -1).then(|| color.try_into().unwrap()).ok_or_else(|| {
        Error::custom(format!("{name:?} is not a valid color name"))
    })
}

/// Binding to [`nvim_get_color_map()`][1].
///
/// Returns an iterator over tuples representing color names and 24-bit RGB
/// values (e.g. 65535).
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_color_map()
pub fn get_color_map() -> impl SuperIterator<(String, u32)> {
    unsafe { nvim_get_color_map() }.into_iter().map(|(k, v)| {
        (k.to_string_lossy().into(), u32::from_object(v).unwrap())
    })
}

/// Binding to [`nvim_get_commands()`][1].
///
/// Returns an iterator over the infos of the global ex commands. Only
/// user-defined commands are returned, not builtin ones.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_commands()
pub fn get_commands(
    opts: &GetCommandsOpts,
) -> Result<impl SuperIterator<CommandInfos>> {
    let mut err = nvim::Error::new();
    let cmds = unsafe { nvim_get_commands(opts, &mut err) };
    choose!(
        err,
        Ok({
            cmds.into_iter()
                .map(|(_, cmd)| CommandInfos::from_object(cmd).unwrap())
        })
    )
}

/// Binding to [`nvim_get_context()`][1].
///
/// Returns a snapshot of the current editor state.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_context()
pub fn get_context(opts: &GetContextOpts) -> Result<EditorContext> {
    let mut err = nvim::Error::new();
    let ctx = unsafe { nvim_get_context(opts, &mut err) };
    choose!(err, Ok(EditorContext::from_object(ctx.into())?))
}

/// Binding to [`nvim_get_current_buf()`][1].
///
/// Gets the current buffer.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_current_buf()
pub fn get_current_buf() -> Buffer {
    unsafe { nvim_get_current_buf() }.into()
}

/// Binding to [`nvim_get_current_line()`][1].
///
/// Gets the current line in the current bufferr.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_current_line()
pub fn get_current_line() -> Result<String> {
    let mut err = nvim::Error::new();
    let s = unsafe { nvim_get_current_line(&mut err) };
    choose!(err, Ok(s.to_string_lossy().into()))
}

/// Binding to [`nvim_get_current_tabpage()`][1].
///
/// Gets the current tabpage.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_current_tabpage()
pub fn get_current_tabpage() -> TabPage {
    unsafe { nvim_get_current_tabpage() }.into()
}

/// Binding to [`nvim_get_current_win()`][1].
///
/// Gets the current window.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_current_win()
pub fn get_current_win() -> Window {
    unsafe { nvim_get_current_win() }.into()
}

/// Binding to [`nvim_get_hl_by_id()`][1].
///
/// Gets a highlight definition by id.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_hl_by_id[1]
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_current_win()
pub fn get_hl_by_id(hl_id: u32, rgb: bool) -> Result<HighlightInfos> {
    let mut err = nvim::Error::new();

    let hl = unsafe {
        nvim_get_hl_by_id(hl_id.into(), rgb, core::ptr::null_mut(), &mut err)
    };

    choose!(err, Ok(HighlightInfos::from_object(hl.into())?))
}

/// Binding to [`nvim_get_hl_by_name()`][1].
///
/// Gets a highlight definition by name.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_hl_by_name()
pub fn get_hl_by_name(name: &str, rgb: bool) -> Result<HighlightInfos> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let hl = unsafe {
        nvim_get_hl_by_name(
            name.non_owning(),
            rgb,
            core::ptr::null_mut(),
            &mut err,
        )
    };
    choose!(err, Ok(HighlightInfos::from_object(hl.into())?))
}

/// Binding to [`nvim_get_hl_id_by_name()`][1].
///
/// Gets a highlight definition by name.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_hl_id_by_name()
pub fn get_hl_id_by_name(name: &str) -> Result<u32> {
    let name = nvim::String::from(name);
    let id = unsafe { nvim_get_hl_id_by_name(name.non_owning()) };
    id.try_into().map_err(Into::into)
}

/// Binding to [`nvim_get_keymap()`][1].
///
/// Returns an iterator over the global mapping definitions.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_keymap()
pub fn get_keymap(mode: Mode) -> impl SuperIterator<KeymapInfos> {
    let mode = nvim::String::from(mode);
    let keymaps = unsafe { nvim_get_keymap(mode.non_owning()) };
    keymaps.into_iter().map(|obj| KeymapInfos::from_object(obj).unwrap())
}

/// Binding to [`nvim_get_mark()`][1].
///
/// Returns a tuple `(row, col, buffer, buffername)` representing the position
/// of the named mark. Marks are (1,0)-indexed.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_mark()
pub fn get_mark(
    name: char,
    opts: &GetMarkOpts,
) -> Result<(usize, usize, Buffer, String)> {
    let name = nvim::String::from(name);
    let opts = Dictionary::from(opts);
    let mut err = nvim::Error::new();
    let mark = unsafe {
        nvim_get_mark(name.non_owning(), opts.non_owning(), &mut err)
    };
    choose!(err, {
        let mut iter = mark.into_iter();
        let row = usize::from_object(iter.next().expect("row is present"))?;
        let col = usize::from_object(iter.next().expect("col is present"))?;
        let buffer =
            Buffer::from_object(iter.next().expect("buffer is present"))?;
        let buffername =
            String::from_object(iter.next().expect("buffername is present"))?;
        Ok((row, col, buffer, buffername))
    })
}

/// Binding to [`nvim_get_mode()`][1].
///
/// Gets the current mode. The [`blocking`](GotMode::blocking) field of
/// [`GotMode`] is `true` if Neovim is waiting for input.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_mode()
pub fn get_mode() -> Result<GotMode> {
    Ok(GotMode::from_object(unsafe { nvim_get_mode() }.into())?)
}

/// Binding to [`nvim_get_option()`][1].
///
/// Gets the value of a global option.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_option()
pub fn get_option<Opt>(name: &str) -> Result<Opt>
where
    Opt: FromObject,
{
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let obj = unsafe {
        nvim_get_option(
            name.non_owning(),
            #[cfg(not(feature = "neovim-0-8"))]
            core::ptr::null_mut(),
            &mut err,
        )
    };
    choose!(err, Ok(Opt::from_object(obj)?))
}

/// Binding to [`nvim_get_option_info()`][1].
///
/// Gets all the informations related to an option.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_option_info()
pub fn get_option_info(name: &str) -> Result<OptionInfos> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let obj = unsafe { nvim_get_option_info(name.non_owning(), &mut err) };
    choose!(err, Ok(OptionInfos::from_object(obj.into())?))
}

/// Binding to [`nvim_get_option_value()`][1].
///
/// Gets the local value of an option if it exists, or the global value
/// otherwise. Local values always correspond to the current buffer or window.
///
/// To get a buffer-local orr window-local option for a specific buffer of
/// window consider using [`Buffer::get_option`] or [`Window::get_option`] instead.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_option_value()
pub fn get_option_value<Opt>(name: &str, opts: &OptionValueOpts) -> Result<Opt>
where
    Opt: FromObject,
{
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let obj =
        unsafe { nvim_get_option_value(name.non_owning(), opts, &mut err) };
    choose!(err, Ok(Opt::from_object(obj)?))
}

/// Binding to [`nvim_get_proc()`][1].
///
/// Gets informations about a process with a given `pid`.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_proc()
pub fn get_proc(pid: u32) -> Result<ProcInfos> {
    let mut err = nvim::Error::new();
    let obj = unsafe { nvim_get_proc(pid.into(), &mut err) };
    choose!(err, Ok(ProcInfos::from_object(obj)?))
}

/// Binding to [`nvim_get_proc_children()`][1].
///
/// Gets the immediate children of process `pid`.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_proc_children()
pub fn get_proc_children(pid: u32) -> Result<impl SuperIterator<u32>> {
    let mut err = nvim::Error::new();
    let procs = unsafe { nvim_get_proc_children(pid.into(), &mut err) };
    choose!(
        err,
        Ok(procs.into_iter().map(|obj| u32::from_object(obj).unwrap()))
    )
}

/// Binding to [`nvim_get_runtime_file()`][1].
///
/// Returns an iterator over all the files matching `name` in the runtime path.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_runtime_file()
pub fn get_runtime_file(
    name: impl AsRef<Path>,
    get_all: bool,
) -> Result<impl SuperIterator<PathBuf>> {
    let name = nvim::String::from(name.as_ref());
    let mut err = nvim::Error::new();
    let files =
        unsafe { nvim_get_runtime_file(name.non_owning(), get_all, &mut err) };
    choose!(
        err,
        Ok({
            files.into_iter().map(|obj| {
                PathBuf::from(nvim::String::from_object(obj).unwrap())
            })
        })
    )
}

/// Binding to [`nvim_get_var()`][1].
///
/// Gets a global (`g:`) variable.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_var()
pub fn get_var<Var>(name: &str) -> Result<Var>
where
    Var: FromObject,
{
    let mut err = nvim::Error::new();
    let name = nvim::String::from(name);
    let obj = unsafe { nvim_get_var(name.non_owning(), &mut err) };
    choose!(err, Ok(Var::from_object(obj)?))
}

/// Binding to [`nvim_get_vvar()`][1].
///
/// Gets a `v:` variable.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_get_vvar()
pub fn get_vvar<Var>(name: &str) -> Result<Var>
where
    Var: FromObject,
{
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    let obj = unsafe { nvim_get_vvar(name.non_owning(), &mut err) };
    choose!(err, Ok(Var::from_object(obj)?))
}

/// Binding to [`nvim_input()`][1].
///
/// Queues raw user-input. Unlike [`api::feedkeys`](feedkeys) this uses a
/// low-level input buffer and the call is non-blocking.
///
/// Returns the number of bytes written to the buffer.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_input()
pub fn input<Input>(keys: Input) -> Result<usize>
where
    Input: Into<nvim::String>,
{
    unsafe { nvim_input(keys.into().non_owning()) }
        .try_into()
        .map_err(From::from)
}

/// Binding to [`nvim_input_mouse()`][1].
///
/// Send mouse event from GUI. The call is non-blocking.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_input_mouse()
pub fn input_mouse(
    button: MouseButton,
    action: MouseAction,
    modifier: &str,
    grid: u32,
    row: usize,
    col: usize,
) -> Result<()> {
    let button = nvim::String::from(button);
    let action = nvim::String::from(action);
    let modifier = nvim::String::from(modifier);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_input_mouse(
            button.non_owning(),
            action.non_owning(),
            modifier.non_owning(),
            grid.into(),
            row.try_into()?,
            col.try_into()?,
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_list_bufs()`][1].
///
/// Gets the current list of [`Buffer`]s, including unlisted [1]
/// buffers (like `:ls!`). Use [`Buffer::is_loaded`] to check if a
/// buffer is loaded.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_list_bufs()
///
/// [1]: unloaded/deleted
pub fn list_bufs() -> impl SuperIterator<Buffer> {
    unsafe { nvim_list_bufs() }
        .into_iter()
        .map(|obj| Buffer::from_object(obj).unwrap())
}

/// Binding to [`nvim_list_chans()`][1].
///
/// Returns an iterator over the informations about all the open channels.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_list_chans()
pub fn list_chans() -> impl SuperIterator<ChannelInfos> {
    unsafe { nvim_list_chans() }
        .into_iter()
        .map(|obj| ChannelInfos::from_object(obj).unwrap())
}

/// Binding to [`nvim_list_runtime_paths()`][1].
///
/// Gets the paths contained in https://neovim's runtimepath.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_list_runtime_paths()
pub fn list_runtime_paths() -> Result<impl SuperIterator<PathBuf>> {
    let mut err = nvim::Error::new();
    let paths = unsafe { nvim_list_runtime_paths(&mut err) };
    choose!(
        err,
        Ok({
            paths.into_iter().map(|obj| {
                PathBuf::from(nvim::String::from_object(obj).unwrap())
            })
        })
    )
}

/// Binding to [`nvim_list_bufs()`][1].
///
/// Gets the current list of `Tabpage`s.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_list_bufs()
pub fn list_tabpages() -> impl SuperIterator<TabPage> {
    unsafe { nvim_list_tabpages() }
        .into_iter()
        .map(|obj| TabPage::from_object(obj).unwrap())
}

/// Binding to [`nvim_list_uis()`][1].
///
/// Returns an iterator over the informations about all the attached UIs.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_list_uis()
pub fn list_uis() -> impl SuperIterator<UiInfos> {
    unsafe { nvim_list_uis() }
        .into_iter()
        .map(|obj| UiInfos::from_object(obj).unwrap())
}

/// Binding to [`nvim_list_wins()`][1].
///
/// Gets the current list of `Window`s.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_list_wins()
pub fn list_wins() -> impl SuperIterator<Window> {
    unsafe { nvim_list_wins() }
        .into_iter()
        .map(|obj| Window::from_object(obj).unwrap())
}

/// Binding to [`nvim_load_context()`][1].
///
/// Sets the current editor state from the given [`EditorContext`].
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_load_context()
pub fn load_context(ctx: EditorContext) {
    let ctx = Dictionary::from(ctx);
    let _ = unsafe { nvim_load_context(ctx.non_owning()) };
}

/// Binding to [`nvim_notify()`][1].
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_notify()
pub fn notify(
    msg: &str,
    log_level: LogLevel,
    opts: &NotifyOpts,
) -> Result<()> {
    let msg = nvim::String::from(msg);
    let opts = Dictionary::from(opts);
    let mut err = nvim::Error::new();
    let _ = unsafe {
        nvim_notify(
            msg.non_owning(),
            log_level as Integer,
            opts.non_owning(),
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_open_term()`][1].
///
/// Opens a terminal instance in a buffer. Returns the id of a channel that can
/// be used to send data to the instance via
/// [`nvim_oxi::api::chan_send`](chan_send).
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_open_term()
pub fn open_term(buffer: &Buffer, opts: &OpenTermOpts) -> Result<u32> {
    let opts = Dictionary::from(opts);
    let mut err = nvim::Error::new();
    let channel_id =
        unsafe { nvim_open_term(buffer.0, opts.non_owning(), &mut err) };
    choose!(
        err,
        match channel_id {
            0 => Err(Error::custom("Couldn't create terminal instance")),
            other => Ok(other.try_into().expect("always positive")),
        }
    )
}

/// Binding to [`nvim_out_write()`][1].
///
/// Writes a message to the Vim output buffer, without appending a "\n". The
/// message is buffered and won't be displayed until a linefeed is written.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_out_write()
pub fn out_write<Msg>(str: Msg)
where
    Msg: Into<nvim::String>,
{
    unsafe { nvim_out_write(str.into().non_owning()) }
}

/// Binding to [`nvim_paste()`][1].
///
/// Returns `true` if the client may continue the paste, `false` if it must
/// cancel it.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_paste()
pub fn paste<Data>(data: Data, crlf: bool, phase: PastePhase) -> Result<bool>
where
    Data: Into<nvim::String>,
{
    let mut err = nvim::Error::new();
    let go_on = unsafe {
        nvim_paste(data.into().non_owning(), crlf, phase as Integer, &mut err)
    };
    choose!(err, Ok(go_on))
}

/// Binding to [`nvim_put()`][1].
///
/// Puts text at cursor, in any mode.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_put()
pub fn put<Line, Lines>(
    lines: Lines,
    reg_type: RegisterType,
    after: bool,
    follow: bool,
) -> Result<()>
where
    Lines: Iterator<Item = Line>,
    Line: Into<nvim::String>,
{
    let lines = lines.into_iter().map(Into::into).collect::<Array>();
    let reg_type = nvim::String::from(reg_type);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_put(
            lines.non_owning(),
            reg_type.non_owning(),
            after,
            follow,
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_replace_termcodes()`][1].
///
/// Replaces terminal codes and keycodes (`<CR>`, `<Esc>`, ...) in a string
/// with the internal representation.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_replace_termcodes()
pub fn replace_termcodes<Input>(
    str: Input,
    from_part: bool,
    do_lt: bool,
    special: bool,
) -> nvim::String
where
    Input: Into<nvim::String>,
{
    let str = str.into();
    unsafe {
        nvim_replace_termcodes(str.non_owning(), from_part, do_lt, special)
    }
}

/// Binding to [`nvim_select_popupmenu_item()`][1].
///
/// Selects an item in the completion popupmenu.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_select_popupmenu_item()
pub fn select_popupmenu_item(
    item: usize,
    insert: bool,
    finish: bool,
    opts: &SelectPopupMenuItemOpts,
) -> Result<()> {
    let opts = Dictionary::from(opts);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_select_popupmenu_item(
            item.try_into()?,
            insert,
            finish,
            opts.non_owning(),
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_set_current_buf()`][1].
///
/// Sets the current buffer.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_current_buf()
pub fn set_current_buf(buf: &Buffer) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_set_current_buf(buf.0, &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_set_current_dir()`][1].
///
/// Changes the global working directory.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_current_dir()
pub fn set_current_dir<Dir>(dir: Dir) -> Result<()>
where
    Dir: AsRef<Path>,
{
    let dir = nvim::String::from(dir.as_ref());
    let mut err = nvim::Error::new();
    unsafe { nvim_set_current_dir(dir.non_owning(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_set_current_line()`][1].
///
/// Sets the current line.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_current_line()
pub fn set_current_line<Line>(line: Line) -> Result<()>
where
    Line: Into<nvim::String>,
{
    let mut err = nvim::Error::new();
    unsafe { nvim_set_current_line(line.into().non_owning(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_set_current_tabpage()`][1].
///
/// Sets the current tabpage.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_current_tabpage()
pub fn set_current_tabpage(tabpage: &TabPage) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_set_current_tabpage(tabpage.0, &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_set_current_win()`][1].
///
/// Sets the current window.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_current_win()
pub fn set_current_win(win: &Window) -> Result<()> {
    let mut err = nvim::Error::new();
    unsafe { nvim_set_current_win(win.0, &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_set_hl()`][1].
///
/// Sets a highlight group.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_hl()
pub fn set_hl(ns_id: u32, name: &str, opts: &SetHighlightOpts) -> Result<()> {
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_set_hl(ns_id as Integer, name.non_owning(), opts, &mut err)
    };
    choose!(err, ())
}

/// Binding to [`nvim_set_keymap()`][1].
///
/// Sets a global mapping for the given mode. To set a buffer-local mapping use
/// [`Buffer::set_keymap`] instead.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_keymap()
pub fn set_keymap(
    mode: Mode,
    lhs: &str,
    rhs: &str,
    opts: &SetKeymapOpts,
) -> Result<()> {
    let mode = nvim::String::from(mode);
    let lhs = nvim::String::from(lhs);
    let rhs = nvim::String::from(rhs);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_set_keymap(
            LUA_INTERNAL_CALL,
            mode.non_owning(),
            lhs.non_owning(),
            rhs.non_owning(),
            opts,
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_set_option()`][1].
///
/// Sets the global value of an option.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_option()
pub fn set_option<Opt>(name: &str, value: Opt) -> Result<()>
where
    Opt: ToObject,
{
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_set_option(
            LUA_INTERNAL_CALL,
            name.non_owning(),
            value.to_object()?.non_owning(),
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_set_option_value()`][1].
///
/// Sets the value of an option. The behaviour of this function matches that of
/// `:set`: for global-local options, both the global and local value are set
/// unless specified otherwise in the [`scope`](OptionValueOptsBuilder::scope)
/// field of `opts`.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_option_value()
pub fn set_option_value<Opt>(
    name: &str,
    value: Opt,
    opts: &OptionValueOpts,
) -> Result<()>
where
    Opt: ToObject,
{
    let name = nvim::String::from(name);
    let mut err = nvim::Error::new();
    unsafe {
        nvim_set_option_value(
            #[cfg(any(feature = "neovim-0-9", feature = "neovim-nightly"))]
            LUA_INTERNAL_CALL,
            name.non_owning(),
            value.to_object()?.non_owning(),
            opts,
            &mut err,
        )
    };
    choose!(err, ())
}

/// Binding to [`nvim_set_var()`][1].
///
/// Sets a global (`g:`) variable.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_var()
pub fn set_var<Var>(name: &str, value: Var) -> Result<()>
where
    Var: ToObject,
{
    let name = nvim::String::from(name);
    let value = value.to_object()?;
    let mut err = nvim::Error::new();
    unsafe { nvim_set_var(name.non_owning(), value.non_owning(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_set_vvar()`][1].
///
/// Sets a `v:` variable, if it's not readonly.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_set_vvar()
pub fn set_vvar<Var>(name: &str, value: Var) -> Result<()>
where
    Var: ToObject,
{
    let name = nvim::String::from(name);
    let value = value.to_object()?;
    let mut err = nvim::Error::new();
    unsafe { nvim_set_vvar(name.non_owning(), value.non_owning(), &mut err) };
    choose!(err, ())
}

/// Binding to [`nvim_strwidth()`][1].
///
/// Calculates the number of display cells occupied by `text`. Control
/// characters like `<Tab>` count as one cell.
///
/// [1]: https://neovim.io/doc/user/api.html#nvim_strwidth()
pub fn strwidth(text: &str) -> Result<usize> {
    let text = nvim::String::from(text);
    let mut err = nvim::Error::new();
    let width = unsafe { nvim_strwidth(text.non_owning(), &mut err) };
    choose!(err, Ok(width.try_into().expect("always positive")))
}
