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
//! # What it holds
//!
//! The document it was written for, by id, and every committed display
//! frame of the session that wrote it ([`SavedFrame`]): the space the
//! frame is keyed by, the node whose gesture wrote it — the "Add
//! part…" insert or a free-move — and the frame. The ids are written in
//! the document format's own spelling, so the layout has no second
//! spelling of them, and the file is TOML, the format this crate
//! already reads and writes (`crate::prefs`), so the viewer takes no
//! serde edge (`scripts/gates/kernel-serde-free.sh`). A free-move in
//! flight, a hide and a show are not held: they are session state
//! (G3).
//!
//! # Fail-loud, and never in the document's way
//!
//! A missing layout is a document drawn at the identity, silently: a
//! document saved without one, or written through the API, has none,
//! and that is not a fault. A layout that cannot be read, is not one
//! this build writes, holds a frame no pose admits, or was written for
//! another document is refused WHOLE and said ([`LayoutFault`]), and
//! the document opens all the same with every space at the identity:
//! the document file never depends on its layout. A layout that reads
//! is restored frame by frame against the opened document
//! ([`crate::display::DisplayState::restore`]), which drops and reports
//! a frame whose space the document does not show, rather than draw
//! another space by it.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use pncad::document::{DocumentId, Frame, MintId, RecipeNodeId, VarId};

use crate::display::Withdrawn;
use pncad::geom_core::Tol;

/// The suffix a layout's file name adds to its document's.
pub const LAYOUT_SUFFIX: &str = ".layout";

/// **The layout file of the document at `doc_path`**: the document's
/// own file name with [`LAYOUT_SUFFIX`] after it, in the same folder.
pub fn path_of(doc_path: &Path) -> PathBuf {
    let mut name = doc_path.as_os_str().to_owned();
    name.push(LAYOUT_SUFFIX);
    PathBuf::from(name)
}

/// **One saved display frame**: the space it is keyed by, the node
/// whose gesture wrote it, and the frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SavedFrame {
    /// The space the frame draws (DI5: a frame is keyed by its space).
    pub space: VarId,
    /// The node whose gesture wrote the frame last, which a withdrawal
    /// names and a restore asks is still in `space`.
    pub node: RecipeNodeId,
    /// The frame.
    pub frame: Frame,
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

/// **Why a layout was not read or not written**, said beside an open or
/// a save that otherwise went through.
#[derive(Debug, Clone, PartialEq)]
pub enum LayoutFault {
    /// The layout file is there and could not be read.
    Read {
        /// The layout file.
        path: PathBuf,
        /// What the OS said.
        message: String,
    },
    /// The layout file is not a layout this build writes: its text, a
    /// field, a frame no pose admits, a frame on the world, or two
    /// frames for one space.
    Malformed {
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
    /// The document was saved and its layout could not be written or
    /// cleared.
    Write {
        /// The layout file.
        path: PathBuf,
        /// What the OS said.
        message: String,
    },
}

/// **What a saved layout came to** on an open or a save — the news
/// [`crate::session::OpOutcome::layout`] carries to the chrome. Empty
/// on every other operation, and on an open or a save whose layout went
/// through whole (or that had none).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutReport {
    /// Saved frames the opened document cannot hold, each with the
    /// fault that dropped it ([`crate::display::DisplayState::restore`]).
    pub dropped: Vec<Withdrawn>,
    /// The layout as a whole not read or not written.
    pub fault: Option<LayoutFault>,
}

impl core::fmt::Display for LayoutFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Read { path, message } => write!(
                f,
                "the saved layout {} could not be read ({message}), so every space is drawn \
                 at the identity",
                path.display()
            ),
            Self::Malformed { path, message } => write!(
                f,
                "the saved layout {} is not one this viewer writes ({message}), so every \
                 space is drawn at the identity",
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
            Self::Write { path, message } => write!(
                f,
                "the document was saved, and its layout {} could not be written ({message}), \
                 so a reopen draws every space at the identity",
                path.display()
            ),
        }
    }
}

impl core::error::Error for LayoutFault {}

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
                entry.insert("node".into(), text(saved.node.0.to_string()));
                entry.insert(
                    "columns".into(),
                    toml::Value::Array(saved.frame.columns.iter().map(|c| floats(c)).collect()),
                );
                entry.insert("translation".into(), floats(&saved.frame.translation));
                toml::Value::Table(entry)
            })
            .collect();
        let mut top = toml::Table::new();
        top.insert("document".into(), text(self.document.hex()));
        top.insert("frame".into(), toml::Value::Array(frames));
        top.to_string()
    }

    /// **The layout `text` spells**, read strictly: exactly the keys
    /// [`Layout::to_text`] writes, every frame admitted as a pose at
    /// `tol` (`Frame::admission_fault`), none on the world, at most one
    /// per space.
    ///
    /// # Errors
    ///
    /// What is wrong, as a sentence [`LayoutFault::Malformed`] carries.
    pub fn parse(text: &str, tol: Tol) -> Result<Self, String> {
        let top: toml::Table = text
            .parse()
            .map_err(|error: toml::de::Error| error.message().to_owned())?;
        keys(&top, &["document", "frame"])?;
        let document = DocumentId::parse_hex(string(&top, "document")?)
            .ok_or("`document` is not a document id")?;
        let Some(toml::Value::Array(raw)) = top.get("frame") else {
            return Err("missing `frame`, an array of tables".into());
        };
        let mut frames = Vec::with_capacity(raw.len());
        let mut spaces = BTreeSet::new();
        for (index, entry) in raw.iter().enumerate() {
            let saved = saved_frame(entry, tol, &mut spaces)
                .map_err(|why| format!("frame {index}: {why}"))?;
            frames.push(saved);
        }
        frames.sort_by_key(|saved| saved.space);
        Ok(Self { document, frames })
    }
}

/// One entry of a layout's `frame` array, read strictly; `spaces` holds
/// the spaces read before it.
fn saved_frame(
    entry: &toml::Value,
    tol: Tol,
    spaces: &mut BTreeSet<VarId>,
) -> Result<SavedFrame, String> {
    let toml::Value::Table(entry) = entry else {
        return Err("not a table".into());
    };
    keys(entry, &["space", "node", "columns", "translation"])?;
    let space = VarId(id(entry, "space")?);
    let node = RecipeNodeId(id(entry, "node")?);
    let columns = match entry.get("columns") {
        Some(toml::Value::Array(columns)) if columns.len() == 3 => [
            floats(&columns[0], "columns")?,
            floats(&columns[1], "columns")?,
            floats(&columns[2], "columns")?,
        ],
        _ => return Err("`columns` is not three columns".into()),
    };
    let translation = entry
        .get("translation")
        .ok_or("missing `translation`")
        .and_then(|value| {
            floats(value, "translation").map_err(|_| "`translation` is not three floats")
        })?;
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
    if !spaces.insert(space) {
        return Err(format!("a second frame for space {space}"));
    }
    Ok(SavedFrame { space, node, frame })
}

/// Refuse a key of `table` that is not in `wanted`; a missing one is
/// its reader's to refuse.
fn keys(table: &toml::Table, wanted: &[&str]) -> Result<(), String> {
    match table.keys().find(|key| !wanted.contains(&key.as_str())) {
        Some(stray) => Err(format!("unknown key `{stray}`")),
        None => Ok(()),
    }
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

/// **The layout saved beside the document at `doc_path`**, or `None`
/// when there is none — a document drawn at the identity, which is no
/// fault.
///
/// # Errors
///
/// [`LayoutFault::Read`] and [`LayoutFault::Malformed`].
pub fn read(doc_path: &Path, tol: Tol) -> Result<Option<Layout>, LayoutFault> {
    let path = path_of(doc_path);
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(LayoutFault::Read {
                path,
                message: error.to_string(),
            });
        }
    };
    Layout::parse(&text, tol)
        .map(Some)
        .map_err(|message| LayoutFault::Malformed { path, message })
}

/// **Write `layout` beside the document at `doc_path`**, or, when it
/// holds no frame, remove any layout there: a layout left from an
/// earlier save would draw the document by frames its session no longer
/// holds.
///
/// # Errors
///
/// [`LayoutFault::Write`].
pub fn write(doc_path: &Path, layout: &Layout) -> Result<(), LayoutFault> {
    let path = path_of(doc_path);
    let written = if layout.frames.is_empty() {
        match std::fs::remove_file(&path) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            other => other,
        }
    } else {
        std::fs::write(&path, layout.to_text())
    };
    written.map_err(|error| LayoutFault::Write {
        path,
        message: error.to_string(),
    })
}
