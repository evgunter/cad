//! **The saved layout**: where a document's shown spaces are drawn,
//! kept beside the document file so a layout outlives the session
//! (IDENTITY DI5: "a free-move may write display metadata … viewer data
//! outside the canonical bytes, never hashed into a `ContentPin` and
//! never read by evaluation").
//!
//! # A file beside the document, not a section of it
//!
//! The layout of `part.pncad` is `part.pncad.layout` ([`path_of`]), a
//! file of its own that only the viewer reads or writes. The document
//! file is the document's state and nothing else (GQ3: its id and its
//! current snapshot; history that outlives a session is a separate
//! store "which the document file never depends on"), so the layout is
//! not a section of it either: the bytes a save writes into the
//! document file are the same with a layout or without one, a document
//! written through the API carries none, and the store's scan, which
//! reads `*.pncad` files alone, never sees it. The canonical bytes and
//! the content pin are a function of the document value
//! (`pncad::document::canonical_bytes`), which a layout never enters.
//!
//! A document written through the API or `crate::docio` over a file
//! the viewer saved leaves that file's layout in place. An open then
//! restores it as it restores any layout: each frame whose space the
//! document still shows with its witness standing in it, and every
//! other frame dropped and said ([`crate::display::DisplayState::restore`]).
//!
//! # What it holds
//!
//! The document it was written for, by id, and every committed display
//! frame of the session that wrote it ([`SavedFrame`]): the space the
//! frame is keyed by (DI5), the frame, and a **witness** — a node the
//! saving document drew in that space. The witness is what says the
//! space an id names in the opened document is the space the frame was
//! drawn for: a space id is minted from the document's chain, and an
//! undo followed by another action mints the same id for another space.
//! Each also carries how the saving document said the space and the
//! witness, so a frame the opened document cannot hold is said by the
//! names it last had rather than by a bare id. A free-move in flight, a
//! hide and a show are not held: they are session state (G3).
//!
//! # One posture with the preferences file
//!
//! The file is TOML, read through the same dynamic door as the
//! preferences (`crate::prefs`) and kept through the same store
//! (`crate::prefs::file::FileStore`): a missing file is `None`, and a
//! file there that cannot be read is a [`StoreError`]. It is read by the
//! posture the preferences are, for the same reason — viewers of
//! different ages read it, and it holds no work: a key this viewer does
//! not know is reported and the rest applies, and a frame it cannot read
//! (a malformed id or coordinate, a frame on the world, a frame no pose
//! admits, two frames for one space) is skipped and reported while the
//! others apply ([`Skipped`]). A file that is not TOML, or names no
//! document, applies nothing and is reported ([`LayoutFault`]). None of
//! it refuses the open: the document file never depends on its layout.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use pncad::document::{DocumentId, Frame, MintId, RecipeNodeId, VarId};
use pncad::geom_core::Tol;

use crate::display::Unrestored;
use crate::prefs::{PrefsStore, StoreError};

/// The suffix a layout's file name adds to its document's.
pub const LAYOUT_SUFFIX: &str = ".layout";

/// **The layout file of the document at `doc_path`**: the document's
/// own file name with [`LAYOUT_SUFFIX`] after it, in the same folder.
pub fn path_of(doc_path: &Path) -> PathBuf {
    let mut name = doc_path.as_os_str().to_owned();
    name.push(LAYOUT_SUFFIX);
    PathBuf::from(name)
}

/// **The store a layout is kept in**: the viewer's file store over
/// [`path_of`], whose refusals say it was a layout.
#[cfg(not(target_family = "wasm"))]
pub fn store(doc_path: &Path) -> impl PrefsStore {
    crate::prefs::file::FileStore::with_doing(
        path_of(doc_path),
        crate::prefs::file::Doing {
            read: "read the saved layout",
            write: "write the saved layout",
            create_dir: "create the saved layout's directory",
        },
    )
}

/// **The store a layout is kept in**: none in a build with no
/// filesystem, which says so on a write. No document path reaches it
/// there, since `crate::docio` reads and writes none.
#[cfg(target_family = "wasm")]
pub fn store(_doc_path: &Path) -> impl PrefsStore {
    crate::prefs::Absent
}

/// **One saved display frame**: the space it is keyed by, the frame,
/// and the witness that the space is the one it was drawn for.
#[derive(Debug, Clone, PartialEq)]
pub struct SavedFrame {
    /// The space the frame draws (DI5: a frame is keyed by its space).
    pub space: VarId,
    /// A node the saving document drew in `space`
    /// ([`crate::display::DisplayState::saved_frames`]).
    pub witness: RecipeNodeId,
    /// The frame.
    pub frame: Frame,
    /// How the saving document said the space.
    pub space_said: String,
    /// How the saving document said the witness.
    pub witness_said: String,
}

/// **A document's saved layout**: the document it was written for and
/// its frames, at most one per space.
#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    /// The document the layout was written for.
    pub document: DocumentId,
    /// The frames, in space order.
    pub frames: Vec<SavedFrame>,
}

/// **Part of a layout this viewer did not apply**, and why; the rest of
/// the layout applies.
#[derive(Debug, Clone, PartialEq)]
pub enum Skipped {
    /// A key this viewer does not know, by its dotted path.
    UnknownKey(String),
    /// A frame this viewer cannot read, by its place in the file.
    Frame {
        /// Its index in the file's `frame` array.
        index: usize,
        /// What is wrong with it.
        why: String,
    },
}

impl core::fmt::Display for Skipped {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownKey(key) => write!(
                f,
                "the saved layout holds a key this viewer does not know, `{key}`, which is \
                 ignored"
            ),
            Self::Frame { index, why } => write!(
                f,
                "saved frame {index} is not read ({why}), so its space is drawn at the \
                 identity"
            ),
        }
    }
}

/// **Why a layout as a whole was not read or not written**, said beside
/// an open or a save that otherwise went through.
#[derive(Debug, Clone, PartialEq)]
pub enum LayoutFault {
    /// The layout file is there and could not be read.
    Read(StoreError),
    /// The layout file is not TOML, or names no document.
    Unreadable {
        /// The layout file.
        path: PathBuf,
        /// What is wrong with it.
        message: String,
    },
    /// The layout was written for another document than the one opened.
    OtherDocument {
        /// The layout file.
        path: PathBuf,
        /// The document it names.
        layout: DocumentId,
        /// The document opened.
        document: DocumentId,
    },
    /// The document was saved and its layout could not be written.
    Write(StoreError),
}

impl core::fmt::Display for LayoutFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Read(StoreError { doing, because }) => write!(
                f,
                "could not {doing} ({because}), so every space is drawn at the identity"
            ),
            Self::Unreadable { path, message } => write!(
                f,
                "the saved layout {} is not one this viewer reads ({message}), so every space \
                 is drawn at the identity",
                path.display()
            ),
            Self::OtherDocument {
                path,
                layout,
                document,
            } => write!(
                f,
                "the saved layout {} is of document {layout}, not of the opened document \
                 {document}, so every space is drawn at the identity",
                path.display()
            ),
            Self::Write(StoreError { doing, because }) => write!(
                f,
                "the document was saved, and could not {doing} ({because}). Any layout saved \
                 there before is left as it was, and a reopen draws by it"
            ),
        }
    }
}

impl core::error::Error for LayoutFault {}

/// **What a saved layout came to** on an open or a save — the news
/// [`crate::session::OpOutcome::layout`] carries to the chrome. Empty
/// on every other operation, and on an open or a save whose layout went
/// through whole (or that had none).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutReport {
    /// Saved frames the opened document cannot hold, each with why
    /// ([`crate::display::DisplayState::restore`]).
    pub dropped: Vec<Unrestored>,
    /// Keys and frames of the layout this viewer did not read.
    pub skipped: Vec<Skipped>,
    /// The layout as a whole not read or not written.
    pub fault: Option<LayoutFault>,
}

impl Layout {
    /// The layout as the text its file holds: TOML, the document id and
    /// each id in the document format's own spelling
    /// ([`DocumentId::hex`], `MintId`'s `Display`), each frame's twelve
    /// coordinates as floats, which the TOML writer spells so they read
    /// back bit for bit.
    pub fn to_text(&self) -> String {
        let text = |value: String| toml::Value::String(value);
        let floats = |values: &[f64]| {
            toml::Value::Array(values.iter().map(|&v| toml::Value::Float(v)).collect())
        };
        let frames = self
            .frames
            .iter()
            .map(|saved| {
                let mut entry = toml::Table::new();
                entry.insert("space".into(), text(saved.space.0.to_string()));
                entry.insert("witness".into(), text(saved.witness.0.to_string()));
                entry.insert(
                    "columns".into(),
                    toml::Value::Array(saved.frame.columns.iter().map(|c| floats(c)).collect()),
                );
                entry.insert("translation".into(), floats(&saved.frame.translation));
                entry.insert("space_said".into(), text(saved.space_said.clone()));
                entry.insert("witness_said".into(), text(saved.witness_said.clone()));
                toml::Value::Table(entry)
            })
            .collect();
        let mut top = toml::Table::new();
        top.insert("document".into(), text(self.document.hex()));
        top.insert("frame".into(), toml::Value::Array(frames));
        top.to_string()
    }

    /// **The layout `text` spells**, and what of it this viewer skipped
    /// (module docs: a key it does not know, a frame it cannot read —
    /// every frame of a space given two — each skipped and reported, the
    /// rest applied).
    ///
    /// # Errors
    ///
    /// Text that is not TOML or names no document, as the sentence
    /// [`LayoutFault::Unreadable`] carries.
    pub fn parse(text: &str, tol: Tol) -> Result<(Self, Vec<Skipped>), String> {
        let top: toml::Table = text
            .parse()
            .map_err(|error: toml::de::Error| error.message().to_owned())?;
        let mut skipped: Vec<Skipped> = top
            .keys()
            .filter(|key| !["document", "frame"].contains(&key.as_str()))
            .map(|key| Skipped::UnknownKey(key.clone()))
            .collect();
        let document = string(&top, "document")
            .ok()
            .and_then(DocumentId::parse_hex)
            .ok_or("it names no document by `document`")?;
        let raw: &[toml::Value] = match top.get("frame") {
            None => &[],
            Some(toml::Value::Array(raw)) => raw,
            Some(_) => {
                skipped.push(Skipped::Frame {
                    index: 0,
                    why: "`frame` is not an array of tables".into(),
                });
                &[]
            }
        };
        let mut read = Vec::new();
        for (index, entry) in raw.iter().enumerate() {
            match saved_frame(entry, tol, index, &mut skipped) {
                Ok(saved) => read.push((index, saved)),
                Err(why) => skipped.push(Skipped::Frame { index, why }),
            }
        }
        let mut per_space: BTreeMap<VarId, usize> = BTreeMap::new();
        for (_, saved) in &read {
            *per_space.entry(saved.space).or_default() += 1;
        }
        let mut frames = Vec::with_capacity(read.len());
        for (index, saved) in read {
            if per_space.get(&saved.space).is_some_and(|&count| count > 1) {
                skipped.push(Skipped::Frame {
                    index,
                    why: format!("the layout gives {} another frame too", saved.space_said),
                });
            } else {
                frames.push(saved);
            }
        }
        frames.sort_by_key(|saved| saved.space);
        Ok((Self { document, frames }, skipped))
    }
}

/// The keys a saved frame is written with.
const FRAME_KEYS: [&str; 6] = [
    "space",
    "witness",
    "columns",
    "translation",
    "space_said",
    "witness_said",
];

/// One entry of a layout's `frame` array, or why it is not read; a key
/// it does not know goes on `skipped` and the entry still reads.
fn saved_frame(
    entry: &toml::Value,
    tol: Tol,
    index: usize,
    skipped: &mut Vec<Skipped>,
) -> Result<SavedFrame, String> {
    let toml::Value::Table(entry) = entry else {
        return Err("not a table".into());
    };
    skipped.extend(
        entry
            .keys()
            .filter(|key| !FRAME_KEYS.contains(&key.as_str()))
            .map(|key| Skipped::UnknownKey(format!("frame.{index}.{key}"))),
    );
    let space = VarId(id(entry, "space")?);
    let witness = RecipeNodeId(id(entry, "witness")?);
    let columns = match entry.get("columns") {
        Some(toml::Value::Array(columns)) if columns.len() == 3 => [
            floats(&columns[0], "columns")?,
            floats(&columns[1], "columns")?,
            floats(&columns[2], "columns")?,
        ],
        _ => return Err("`columns` is not three columns".into()),
    };
    let translation = floats(
        entry.get("translation").ok_or("missing `translation`")?,
        "translation",
    )?;
    let frame = Frame {
        columns,
        translation,
    };
    if space == VarId::WORLD {
        return Err("a frame on the world, which no gesture writes".into());
    }
    if let Some(fault) = frame.admission_fault(tol) {
        return Err(format!("the frame {fault}"));
    }
    Ok(SavedFrame {
        space,
        witness,
        frame,
        space_said: string(entry, "space_said")?.to_owned(),
        witness_said: string(entry, "witness_said")?.to_owned(),
    })
}

/// The string at `key` of `table`.
fn string<'a>(table: &'a toml::Table, key: &str) -> Result<&'a str, String> {
    match table.get(key) {
        Some(toml::Value::String(text)) => Ok(text),
        _ => Err(format!("missing `{key}`, a string")),
    }
}

/// The id at `key` of `table`, in the document format's spelling.
fn id(table: &toml::Table, key: &str) -> Result<MintId, String> {
    MintId::parse(string(table, key)?).ok_or_else(|| format!("`{key}` is not an id"))
}

/// Three floats, the shape of a frame's column or translation.
fn floats(value: &toml::Value, key: &str) -> Result<[f64; 3], String> {
    let bad = || format!("`{key}` holds something other than three floats");
    let toml::Value::Array(values) = value else {
        return Err(bad());
    };
    let [
        toml::Value::Float(x),
        toml::Value::Float(y),
        toml::Value::Float(z),
    ] = values.as_slice()
    else {
        return Err(bad());
    };
    Ok([*x, *y, *z])
}

/// **The layout saved beside the document at `doc_path`**, and what of
/// it was skipped, or `None` when there is none — a document drawn at
/// the identity, which is no fault.
///
/// # Errors
///
/// [`LayoutFault::Read`] and [`LayoutFault::Unreadable`].
pub fn read(doc_path: &Path, tol: Tol) -> Result<Option<(Layout, Vec<Skipped>)>, LayoutFault> {
    let Some(text) = store(doc_path).load().map_err(LayoutFault::Read)? else {
        return Ok(None);
    };
    Layout::parse(&text, tol)
        .map(Some)
        .map_err(|message| LayoutFault::Unreadable {
            path: path_of(doc_path),
            message,
        })
}

/// **Write `layout` beside the document at `doc_path`.** A layout with
/// no frame is written only over one already there, so a layout from an
/// earlier save cannot draw the document by frames its session no
/// longer holds, and a document never given a frame gets no file.
///
/// # Errors
///
/// [`LayoutFault::Write`]; the file there before, if any, is left as it
/// was.
pub fn write(doc_path: &Path, layout: &Layout) -> Result<(), LayoutFault> {
    let store = store(doc_path);
    if layout.frames.is_empty() && matches!(store.load(), Ok(None)) {
        return Ok(());
    }
    store.save(&layout.to_text()).map_err(LayoutFault::Write)
}
