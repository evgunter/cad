//! Preferences: the format, the resolution, and the store.
//!
//! Renderer-free like the module it exercises — no `app` feature, no
//! toolkit. The file store is exercised through a real path in a
//! temporary directory rather than a fake, because the one thing that
//! matters about it is whether it reads and writes what it says.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::path::PathBuf;

use viewer::input::InputMap;
use viewer::prefs::{Absent, Notice, Prefs, PrefsError, PrefsStore, file::FileStore};
use viewer::theme::Theme;

/// A scratch path unique to one test, under the OS temp directory.
fn scratch(tag: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("pncad-prefs-{}-{tag}", std::process::id()));
    path.push("viewer.toml");
    path
}

/// An empty document is valid and means "all defaults".
#[test]
fn an_empty_document_is_the_default() {
    let (prefs, notices) = Prefs::from_toml("").expect("empty TOML parses");
    assert_eq!(prefs, Prefs::default());
    assert!(notices.is_empty(), "{notices:?}");
    assert_eq!(prefs.resolve_theme().0, Theme::DEFAULT);
    assert_eq!(prefs.resolve_keys().0, InputMap::DEFAULT);
}

/// Both settings round-trip through the rendered document.
///
/// The renderer is hand-written, so this is the row that says its
/// output is something the parser accepts — the failure it prevents
/// is a viewer that writes a file it cannot read back.
#[test]
fn what_is_written_is_what_is_read() {
    let written = Prefs {
        theme: Some("colorblind-safe".to_owned()),
        keys: Some("default".to_owned()),
    };
    let (read, notices) = Prefs::from_toml(&written.to_toml()).expect("its own output parses");
    assert_eq!(read, written);
    assert!(notices.is_empty(), "{notices:?}");
}

/// The commented-out default document also round-trips — to nothing.
///
/// A file written before anybody chose anything is all comments, and
/// a parser that choked on it would break the first save.
#[test]
fn the_untouched_document_round_trips_to_defaults() {
    let (read, notices) = Prefs::from_toml(&Prefs::default().to_toml()).expect("parses");
    assert_eq!(read, Prefs::default());
    assert!(notices.is_empty(), "{notices:?}");
}

/// Malformed TOML refuses, and the refusal carries the parser's words.
#[test]
fn malformed_toml_refuses() {
    let error = Prefs::from_toml("[appearance\ntheme = ").expect_err("refuses");
    let PrefsError::Syntax(message) = &error else {
        panic!("expected a syntax refusal, got {error:?}");
    };
    assert!(!message.is_empty(), "the refusal says nothing");
    // The refusal is a value with a sentence, like every other one in
    // this crate.
    assert!(error.to_string().contains("not valid TOML"));
}

/// An unknown key reports and the rest of the file still applies.
///
/// The posture that separates a preferences file from a document: a
/// newer viewer's key must not stop an older one from opening.
#[test]
fn an_unknown_key_reports_and_the_file_still_loads() {
    let (prefs, notices) = Prefs::from_toml(
        "[appearance]\ntheme = \"light-neutral\"\nsparkle = true\n\n[nonsense]\nx = 1\n",
    )
    .expect("loads");
    assert_eq!(prefs.theme.as_deref(), Some("light-neutral"));
    assert!(
        notices.contains(&Notice::UnknownKey("appearance.sparkle".to_owned())),
        "{notices:?}",
    );
    assert!(
        notices.contains(&Notice::UnknownKey("nonsense".to_owned())),
        "{notices:?}",
    );
}

/// A key of the wrong type reports and the default stands.
#[test]
fn a_wrongly_typed_setting_reports_and_defaults() {
    let (prefs, notices) = Prefs::from_toml("[appearance]\ntheme = 7\n").expect("loads");
    assert_eq!(prefs.theme, None);
    assert_eq!(prefs.resolve_theme().0, Theme::DEFAULT);
    assert!(
        notices.iter().any(|n| matches!(
            n,
            Notice::WrongType { key, .. } if key == "appearance.theme"
        )),
        "{notices:?}",
    );
}

/// A theme name nobody registers reports and falls back — it does not
/// refuse.
///
/// **The asymmetry with the command line is deliberate** and is the
/// module header's rule: a name in a file is a memory of an older
/// session and may have been renamed since, where a name typed just
/// now is a typo worth showing. Same word, different provenance.
#[test]
fn an_unregistered_theme_name_falls_back_with_a_notice() {
    let (prefs, notices) =
        Prefs::from_toml("[appearance]\ntheme = \"solarized\"\n").expect("loads");
    assert!(notices.is_empty(), "parsing itself has no complaint");
    let (theme, notice) = prefs.resolve_theme();
    assert_eq!(theme, Theme::DEFAULT);
    assert_eq!(notice, Some(Notice::UnknownTheme("solarized".to_owned())));
    assert!(
        notice
            .expect("the notice is there")
            .to_string()
            .contains("solarized")
    );
}

/// Every registered theme is reachable through a preferences file.
///
/// The registry, the picker and the file are one set: a theme that
/// shipped but could not be named here would be unreachable to
/// anybody who does not click.
#[test]
fn every_registered_theme_resolves_from_a_document() {
    for theme in Theme::ALL {
        let document = format!("[appearance]\ntheme = \"{}\"\n", theme.name);
        let (prefs, notices) = Prefs::from_toml(&document).expect("loads");
        assert!(notices.is_empty(), "{}: {notices:?}", theme.name);
        let (resolved, notice) = prefs.resolve_theme();
        assert_eq!(resolved, *theme);
        assert_eq!(notice, None);
    }
}

/// The same for input presets — one today, and the row is what makes
/// a second one arrive already reachable.
#[test]
fn every_registered_input_preset_resolves_from_a_document() {
    for (name, map) in viewer::input::PRESETS {
        let document = format!("[keys]\npreset = \"{name}\"\n");
        let (prefs, notices) = Prefs::from_toml(&document).expect("loads");
        assert!(notices.is_empty(), "{name}: {notices:?}");
        assert_eq!(prefs.resolve_keys(), (*map, None));
    }
}

/// An unregistered preset name falls back with a notice, like a theme.
#[test]
fn an_unregistered_preset_name_falls_back_with_a_notice() {
    let (prefs, _) = Prefs::from_toml("[keys]\npreset = \"vim\"\n").expect("loads");
    let (map, notice) = prefs.resolve_keys();
    assert_eq!(map, InputMap::DEFAULT);
    assert_eq!(notice, Some(Notice::UnknownPreset("vim".to_owned())));
}

/// A store with nothing behind it says so instead of pretending.
///
/// The `frame::chooser_backend` posture: a control backed by this is
/// disabled with a reason, never offered and silently ineffective.
#[test]
fn an_absent_store_reports_rather_than_pretends() {
    let store = Absent;
    assert!(!store.usable());
    assert_eq!(store.load(), Ok(None));
    let error = store.save("[appearance]\n").expect_err("cannot save");
    assert!(error.to_string().contains("nowhere to keep them"));
}

/// Never having saved is not a failure.
#[test]
fn a_missing_file_loads_as_nothing() {
    let store = FileStore::at(scratch("missing"));
    assert_eq!(
        store.load(),
        Ok(None),
        "a file nobody wrote is not an error"
    );
}

/// The file store actually writes and reads back, creating the
/// directory it needs.
#[test]
fn the_file_store_round_trips_through_a_real_path() {
    let path = scratch("roundtrip");
    let store = FileStore::at(path.clone());
    assert!(store.usable());
    let prefs = Prefs {
        theme: Some("colorblind-safe".to_owned()),
        keys: None,
    };
    store.save(&prefs.to_toml()).expect("saves");
    let text = store.load().expect("loads").expect("something is there");
    let (read, notices) = Prefs::from_toml(&text).expect("parses");
    assert_eq!(read, prefs);
    assert!(notices.is_empty(), "{notices:?}");
    std::fs::remove_dir_all(path.parent().expect("has a parent")).ok();
}
