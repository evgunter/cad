//! User preferences: what a viewer remembers between runs.
//!
//! # The shape, and why it is this shape
//!
//! **The format is a value; the storage is one thin edge.** [`Prefs`]
//! parses from and renders to a TOML document as a `&str`, with no
//! filesystem in sight, and a [`PrefsStore`] is the only thing that
//! knows where that string comes from. The currency of that trait is
//! deliberately a `String` and never a `Path`.
//!
//! That single choice is what makes the browser cheap later. eframe's
//! windowing on wasm32 IS the browser and there is no filesystem
//! there, so the web build's store will be `web_sys::Storage` —
//! already re-exported by eframe, so no new dependency — and it is a
//! second impl of two methods rather than a retrofit. Written the
//! other way, as "read the file at this path", every caller would
//! have had to learn about a path that does not exist.
//!
//! # What refuses, and what merely reports
//!
//! A preferences file holds no work. Losing one costs a person their
//! colour scheme, not their model, and it is read by viewers of
//! different ages — so the failure posture here is deliberately
//! softer than the document path's, and each arm says which it is:
//!
//! - **Malformed TOML refuses** ([`PrefsError::Syntax`]). That is a
//!   typo in a file somebody hand-edited, and the whole reason to
//!   choose a hand-editable format is that its errors are worth
//!   showing.
//! - **An unknown key reports** ([`Notice::UnknownKey`]) and the rest
//!   of the file still applies. Documents use `deny_unknown_fields`
//!   because a key nobody understands may mean the geometry is not
//!   what it looks like; nothing here can be that. A newer viewer's
//!   key must not stop an older one from opening.
//! - **An unknown VALUE reports and falls back**
//!   ([`Notice::UnknownTheme`], [`Notice::UnknownPreset`]) — a theme
//!   may be renamed between versions, and the file is a memory of an
//!   older session rather than an instruction typed just now. **A
//!   name given on the command line is refused instead**, because
//!   that one WAS typed just now and a silent fallback would hide the
//!   typo. Same word, different provenance, different answer.
//! - **A missing file is the default, silently.** Never having set a
//!   preference is not a fault.
//!
//! Every notice is returned, never logged: the caller decides whether
//! a person sees it, the same way every other refusal in this crate
//! is a value.

use crate::input::{self, InputMap};
use crate::theme::Theme;

/// The TOML table appearance settings live under.
const APPEARANCE: &str = "appearance";
/// The TOML table input settings live under.
const KEYS: &str = "keys";
/// The key naming a [`Theme`].
const THEME: &str = "theme";
/// The key naming an [`InputMap`] preset.
const PRESET: &str = "preset";

/// What a viewer remembers between runs.
///
/// Names, not values: the file records *which* theme, and the registry
/// says what that theme is. A palette copied into the preferences file
/// would be a second definition able to drift from the real one, and
/// would freeze a theme's colours at whatever they were the day it was
/// written.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Prefs {
    /// The chosen theme's name, or `None` to take the default.
    pub theme: Option<String>,
    /// The chosen input preset's name, or `None` for the default.
    pub keys: Option<String>,
}

/// Something worth telling a person about a file that still loaded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Notice {
    /// A key this viewer does not know. Carries the dotted path.
    UnknownKey(String),
    /// A key whose value is not the type it should be; the stated
    /// setting is ignored and the default stands.
    WrongType {
        /// The dotted path of the offending key.
        key: String,
        /// What this viewer expected to find there.
        expected: &'static str,
    },
    /// A theme name no longer in the registry.
    UnknownTheme(String),
    /// An input preset name no longer in the registry.
    UnknownPreset(String),
}

impl std::fmt::Display for Notice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownKey(key) => {
                write!(f, "preferences: unknown setting `{key}`, ignored")
            }
            Self::WrongType { key, expected } => {
                write!(f, "preferences: `{key}` should be {expected}; ignored")
            }
            Self::UnknownTheme(name) => write!(
                f,
                "preferences: no theme called `{name}`; using `{}`",
                Theme::DEFAULT.name
            ),
            Self::UnknownPreset(name) => {
                write!(
                    f,
                    "preferences: no input preset called `{name}`; using the default"
                )
            }
        }
    }
}

/// A preferences document that could not be read at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrefsError {
    /// The document is not TOML. Carries the parser's own message,
    /// which names the line.
    Syntax(String),
    /// The document parsed but its root is not a table.
    NotATable,
}

impl std::fmt::Display for PrefsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Syntax(message) => write!(f, "preferences are not valid TOML: {message}"),
            Self::NotATable => write!(f, "preferences must be a TOML table"),
        }
    }
}

impl std::error::Error for PrefsError {}

impl Prefs {
    /// Read a preferences document.
    ///
    /// # Errors
    ///
    /// [`PrefsError`] when the text is not a TOML table at all —
    /// everything short of that is a [`Notice`] beside a usable value.
    pub fn from_toml(text: &str) -> Result<(Self, Vec<Notice>), PrefsError> {
        let table: toml::Table = text
            .parse()
            .map_err(|e: toml::de::Error| PrefsError::Syntax(e.message().to_owned()))?;
        let mut prefs = Self::default();
        let mut notices = Vec::new();
        for (key, value) in &table {
            match key.as_str() {
                APPEARANCE => {
                    prefs.theme = section(value, APPEARANCE, THEME, &mut notices);
                }
                KEYS => {
                    prefs.keys = section(value, KEYS, PRESET, &mut notices);
                }
                other => notices.push(Notice::UnknownKey(other.to_owned())),
            }
        }
        Ok((prefs, notices))
    }

    /// Render these preferences as a TOML document.
    ///
    /// Hand-written rather than serialized, and the comments are why:
    /// this file's whole justification over an opaque blob is that a
    /// person can open it, so what it says about itself is part of
    /// the output. A `Serialize` derive would emit the keys and none
    /// of the sentences.
    pub fn to_toml(&self) -> String {
        let mut out = String::from(
            "# pncad viewer preferences.\n\
             #\n\
             # Settings are NAMES, resolved against the viewer's own\n\
             # registries — an unknown name is reported and the default\n\
             # stands, so an old file never stops a new viewer opening.\n",
        );
        out.push_str(&format!("\n[{APPEARANCE}]\n"));
        out.push_str("# One of: ");
        let names: Vec<&str> = Theme::ALL.iter().map(|t| t.name).collect();
        out.push_str(&names.join(", "));
        out.push('\n');
        match &self.theme {
            Some(name) => out.push_str(&format!("{THEME} = \"{name}\"\n")),
            None => out.push_str(&format!("# {THEME} = \"{}\"\n", Theme::DEFAULT.name)),
        }
        out.push_str(&format!("\n[{KEYS}]\n"));
        out.push_str(
            "# Mouse bindings, by preset name. There is one preset today;\n\
             # a keyboard-binding vocabulary does not exist yet, so this\n\
             # reserves the door rather than offering a choice.\n",
        );
        match &self.keys {
            Some(name) => out.push_str(&format!("{PRESET} = \"{name}\"\n")),
            None => out.push_str(&format!("# {PRESET} = \"{}\"\n", input::PRESETS[0].0)),
        }
        out
    }

    /// The theme these preferences name, with a notice if the name is
    /// not one the registry knows.
    pub fn resolve_theme(&self) -> (Theme, Option<Notice>) {
        match &self.theme {
            None => (Theme::DEFAULT, None),
            Some(name) => match Theme::by_name(name) {
                Some(theme) => (theme, None),
                None => (Theme::DEFAULT, Some(Notice::UnknownTheme(name.clone()))),
            },
        }
    }

    /// The input preset these preferences name, with a notice if the
    /// name is not one the registry knows.
    pub fn resolve_keys(&self) -> (InputMap, Option<Notice>) {
        match &self.keys {
            None => (InputMap::DEFAULT, None),
            Some(name) => match input::preset_by_name(name) {
                Some(map) => (map, None),
                None => (InputMap::DEFAULT, Some(Notice::UnknownPreset(name.clone()))),
            },
        }
    }
}

/// One `[section]` with one string key in it, reporting anything else
/// it finds rather than refusing over it.
fn section(
    value: &toml::Value,
    section: &str,
    wanted: &str,
    notices: &mut Vec<Notice>,
) -> Option<String> {
    let Some(table) = value.as_table() else {
        notices.push(Notice::WrongType {
            key: section.to_owned(),
            expected: "a table",
        });
        return None;
    };
    let mut found = None;
    for (key, value) in table {
        if key == wanted {
            match value.as_str() {
                Some(name) => found = Some(name.to_owned()),
                None => notices.push(Notice::WrongType {
                    key: format!("{section}.{key}"),
                    expected: "a string",
                }),
            }
        } else {
            notices.push(Notice::UnknownKey(format!("{section}.{key}")));
        }
    }
    found
}

/// Where a preferences document is kept.
///
/// **Two methods over a `String`, and nothing about a filesystem.**
/// The native store is a file; the browser's will be `localStorage`;
/// a store that cannot reach either is [`Absent`] and says so. See
/// this module's header for why the currency is the document rather
/// than a path.
pub trait PrefsStore {
    /// The stored document, or `None` where nothing has been saved.
    ///
    /// # Errors
    ///
    /// [`StoreError`] when the backing store exists but could not be
    /// read — which is not the same as nothing having been saved, and
    /// is why this is not simply `Option`.
    fn load(&self) -> Result<Option<String>, StoreError>;

    /// Replace the stored document.
    ///
    /// # Errors
    ///
    /// [`StoreError`] when the store could not be written.
    fn save(&self, document: &str) -> Result<(), StoreError>;

    /// Whether this store can hold anything at all. A caller shows
    /// the difference between "not saved yet" and "cannot save here".
    fn usable(&self) -> bool {
        true
    }
}

/// A store that could not do what was asked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreError {
    /// What was being attempted, for the message.
    pub doing: &'static str,
    /// The backing store's own words.
    pub because: String,
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "preferences: could not {} ({})",
            self.doing, self.because
        )
    }
}

impl std::error::Error for StoreError {}

/// The store for a build with nowhere to keep preferences.
///
/// **Reports rather than pretends**, exactly as `frame::chooser_backend`
/// does for a desktop with no portal and no `zenity`: a control backed
/// by this is disabled with a reason, never offered and then silently
/// ineffective. It is what the browser build uses until a
/// `web_sys::Storage` store is written.
#[derive(Debug, Clone, Copy, Default)]
pub struct Absent;

impl PrefsStore for Absent {
    fn load(&self) -> Result<Option<String>, StoreError> {
        Ok(None)
    }

    fn save(&self, _document: &str) -> Result<(), StoreError> {
        Err(StoreError {
            doing: "save preferences",
            because: "this build has nowhere to keep them".to_owned(),
        })
    }

    fn usable(&self) -> bool {
        false
    }
}

/// The native store: one TOML file under the user's config directory.
#[cfg(not(target_family = "wasm"))]
pub mod file {
    use std::path::PathBuf;

    use super::{PrefsStore, StoreError};

    /// The directory this project keeps user files in.
    const DIR: &str = "pncad";
    /// The preferences file's name inside it.
    const FILE: &str = "viewer.toml";

    /// Where preferences live: `$XDG_CONFIG_HOME/pncad/viewer.toml`,
    /// falling back to `$HOME/.config` as the XDG base-directory
    /// specification says to.
    ///
    /// Resolved by hand rather than through a crate: it is two
    /// environment variables and a join, and the alternative
    /// (`directories`) is a dependency whose whole value is the two
    /// platforms this project does not build for.
    ///
    /// `None` when neither variable is set, which is a real
    /// possibility in a stripped environment and is why this is not a
    /// `PathBuf` with an invented default.
    pub fn path() -> Option<PathBuf> {
        let base = match std::env::var_os("XDG_CONFIG_HOME") {
            Some(value) if !value.is_empty() => PathBuf::from(value),
            _ => PathBuf::from(std::env::var_os("HOME")?).join(".config"),
        };
        Some(base.join(DIR).join(FILE))
    }

    /// Preferences in a file at [`path`].
    #[derive(Debug, Clone)]
    pub struct FileStore {
        /// The document's path; `None` in an environment that names
        /// no config directory, which makes this store unusable
        /// rather than making it guess.
        path: Option<PathBuf>,
    }

    impl FileStore {
        /// The store at this environment's config path.
        #[must_use]
        pub fn discover() -> Self {
            Self { path: path() }
        }

        /// A store at an explicit path — what the tests use, so the
        /// suite exercises the real read and write rather than a
        /// stand-in for them.
        #[must_use]
        pub fn at(path: PathBuf) -> Self {
            Self { path: Some(path) }
        }
    }

    impl PrefsStore for FileStore {
        fn load(&self) -> Result<Option<String>, StoreError> {
            let Some(path) = &self.path else {
                return Ok(None);
            };
            match std::fs::read_to_string(path) {
                Ok(text) => Ok(Some(text)),
                // Never having saved is not a failure — it is the
                // ordinary state of a viewer someone just installed.
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(StoreError {
                    doing: "read preferences",
                    because: e.to_string(),
                }),
            }
        }

        fn save(&self, document: &str) -> Result<(), StoreError> {
            let Some(path) = &self.path else {
                return Err(StoreError {
                    doing: "save preferences",
                    because: "no config directory in this environment".to_owned(),
                });
            };
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| StoreError {
                    doing: "create the preferences directory",
                    because: e.to_string(),
                })?;
            }
            std::fs::write(path, document).map_err(|e| StoreError {
                doing: "write preferences",
                because: e.to_string(),
            })
        }

        fn usable(&self) -> bool {
            self.path.is_some()
        }
    }
}
