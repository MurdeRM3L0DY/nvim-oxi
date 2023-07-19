use oxi_types::{
    Array,
    Boolean,
    Dictionary,
    Error,
    NonOwning,
    Object,
    String,
};

use crate::opts::*;

extern "C" {
    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vimscript.c#L283
    pub(crate) fn nvim_call_dict_function(
        dict: NonOwning<Object>,
        r#fn: NonOwning<String>,
        args: NonOwning<Array>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vimscript.c#L268
    pub(crate) fn nvim_call_function(
        r#fn: NonOwning<String>,
        args: NonOwning<Array>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/command.c#L320
    pub(crate) fn nvim_cmd(
        channel_id: u64,
        cmd: *const crate::types::KeyDict_cmd,
        opts: *const CmdOpts,
        err: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vimscript.c#L138
    pub(crate) fn nvim_command(command: NonOwning<String>, err: *mut Error);

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vimscript.c#L154
    pub(crate) fn nvim_eval(
        expr: NonOwning<String>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/deprecated.c#L33
    pub(crate) fn nvim_exec(
        channel_id: u64,
        src: NonOwning<String>,
        output: Boolean,
        error: *mut Error,
    ) -> String;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/command.c#L98
    pub(crate) fn nvim_parse_cmd(
        src: NonOwning<String>,
        opts: NonOwning<Dictionary>,
        error: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/vimscript.c#L438
    pub fn nvim_parse_expression(
        expr: NonOwning<String>,
        flags: NonOwning<String>,
        highlight: bool,
        err: *mut Error,
    ) -> Dictionary;
}
