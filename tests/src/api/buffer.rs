use all_asserts::*;
use nvim_oxi as oxi;
use nvim_oxi::api::{self, opts::*, types::*, Buffer};

#[oxi::test]
fn attach() {
    let buf = Buffer::current();

    let opts = BufAttachOpts::builder()
        .on_lines(|_args| Ok(false))
        .on_bytes(|_args| Ok(false))
        .on_detach(|_args| Ok(false))
        .on_reload(|_args| Ok(false))
        .on_changedtick(|_args| Ok(false))
        .build();

    let res = buf.attach(false, &opts);
    assert_eq!(Ok(()), res);

    let bytes_written = api::input("ifoo<Esc>");
    assert!(bytes_written.is_ok(), "{bytes_written:?}");
}

#[oxi::test]
fn buf_call() {
    let buf = Buffer::current();
    let res = buf.call(|_| Ok(()));
    assert_eq!(Ok(()), res);
}

#[oxi::test]
fn buf_create_del_user_command() {
    let mut buf = Buffer::current();

    let res = buf.create_user_command("Foo", ":", &Default::default());
    assert_eq!(Ok(()), res);
    api::command("Foo").unwrap();

    let res =
        buf.create_user_command("Bar", |_args| Ok(()), &Default::default());
    assert_eq!(Ok(()), res);
    api::command("Bar").unwrap();

    assert_eq!(
        2,
        buf.get_commands(&Default::default())
            .unwrap()
            .collect::<Vec<_>>()
            .len()
    );

    assert_eq!(Ok(()), buf.del_user_command("Foo"));
    assert_eq!(Ok(()), buf.del_user_command("Bar"));
}

#[oxi::test]
fn get_changedtick() {
    let buf = Buffer::current();
    assert!(buf.get_changedtick().is_ok());
}

#[oxi::test]
fn loaded_n_valid() {
    let buf = Buffer::current();
    assert!(buf.is_loaded());
    assert!(buf.is_valid());
}

#[oxi::test]
fn new_buf_delete() {
    let buf = api::create_buf(true, false).unwrap();
    assert_eq!(Ok(()), buf.delete(&Default::default()));
}

#[oxi::test]
fn buf_set_get_del_keymap() {
    let mut buf = Buffer::current();

    let opts = SetKeymapOpts::builder()
        .callback(|_| Ok(()))
        .desc("does nothing")
        .expr(true)
        .build();

    let res = buf.set_keymap(Mode::Insert, "a", "", &opts);
    assert_eq!(Ok(()), res);

    let keymaps = buf.get_keymap(Mode::Insert).unwrap().collect::<Vec<_>>();
    assert_eq!(1, keymaps.len());

    let res = buf.del_keymap(Mode::Insert, "a");
    assert_eq!(Ok(()), res);
}

#[oxi::test]
fn buf_set_get_del_nvo_keymap() {
    let mut buf = Buffer::current();

    let res = buf.set_keymap(
        Mode::NormalVisualOperator,
        "a",
        "b",
        &Default::default(),
    );
    assert_eq!(Ok(()), res);

    let keymaps = buf
        .get_keymap(Mode::NormalVisualOperator)
        .unwrap()
        .collect::<Vec<_>>();
    assert_le!(1, keymaps.len());

    let res = buf.del_keymap(Mode::NormalVisualOperator, "a");
    assert_eq!(Ok(()), res);
}

#[oxi::test]
fn set_get_del_lines() {
    let mut buf = Buffer::current();

    assert_eq!(Ok(()), buf.set_lines(.., true, ["foo", "bar", "baz"]));
    assert_eq!(
        vec!["foo", "bar", "baz"],
        buf.get_lines(.., true)
            .unwrap()
            .map(|s| s.to_string_lossy().into())
            .collect::<Vec<String>>()
    );
    assert_eq!(Ok(3), buf.line_count());

    assert_eq!(Ok(()), buf.set_lines::<&str, _, _>(.., true, []));
    assert_eq!(Ok(1), buf.line_count());
}

#[oxi::test]
fn buf_set_get_del_mark() {
    let mut buf = Buffer::current();

    let res = buf.set_mark('a', 1, 0);
    assert_eq!(Ok(()), res);

    assert_eq!((1, 0), buf.get_mark('a').unwrap());

    let res = buf.del_mark('a');
    assert_eq!(Ok(()), res);
}

#[oxi::test]
fn set_get_del_text() {
    let mut buf = Buffer::current();

    assert_eq!(Ok(()), buf.set_text(.., 0, 0, ["foo", "bar", "baz"]));
    assert_eq!(
        vec!["foo", "bar", "baz"],
        buf.get_text(.., 0, 3, &Default::default())
            .unwrap()
            .map(|s| s.to_string_lossy().into())
            .collect::<Vec<String>>()
    );
    assert_eq!(Ok(3), buf.line_count());

    assert_eq!(
        vec!["oo", "ba"],
        buf.get_text(..2, 1, 2, &Default::default())
            .unwrap()
            .map(|s| s.to_string_lossy().into())
            .collect::<Vec<String>>()
    );

    assert_eq!(Ok(()), buf.set_text::<&str, _, _>(.., 0, 3, []));

    assert_eq!(
        1,
        buf.get_text(.., 0, 1, &Default::default()).unwrap().count()
    );

    assert_eq!(Ok(1), buf.line_count());
}

#[oxi::test]
fn buf_set_get_del_var() {
    let mut buf = Buffer::current();
    buf.set_var("foo", 42).unwrap();
    assert_eq!(Ok(42), buf.get_var("foo"));
    assert_eq!(Ok(()), buf.del_var("foo"));
}

#[oxi::test]
fn set_get_name() {
    let mut buf = Buffer::current();

    assert_eq!("", buf.get_name().unwrap().display().to_string());

    assert_eq!(Ok(()), buf.set_name("foo"));

    assert_eq!(
        "foo",
        buf.get_name().unwrap().file_name().unwrap().to_string_lossy()
    );

    assert_eq!(Ok(()), buf.set_name(""));
}

#[oxi::test]
fn buf_set_get_option() {
    let mut buf = Buffer::current();

    buf.set_option("modified", true).unwrap();
    assert!(buf.get_option::<bool>("modified").unwrap());

    buf.set_option("modified", false).unwrap();
    assert!(!buf.get_option::<bool>("modified").unwrap());
}
