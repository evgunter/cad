//! The viewer: layer 3 of the GUI/editor architecture
//! (`docs/GUI-DESIGN.md` G1) — interaction over the headless
//! `editor-core` document and the kernel below it.
//!
//! # What is a value here, and what is a widget
//!
//! G1's operations-are-API rule binds this crate: **every move a user
//! can make is a typed operation on a state value, callable with no
//! renderer present.** There are two such vocabularies, and between
//! them they are the crate:
//!
//! - **Navigating**: [`Camera`] is the value, [`CameraOp`] the moves,
//!   [`camera::apply`] the one function that performs them.
//! - **Editing a document**: [`DocSession`] is the value — an undo
//!   [`History`] of `Doc`s, a [`Selection`], the gesture in flight and
//!   the evaluation seam — [`SessionOp`] is the moves, and
//!   [`DocSession::perform`] is the one function that performs them.
//!   [`OpOutcome`] reports what each emitted, so a test asserts on the
//!   `DocEdit`s rather than on pixels.
//!
//! The rest is a view of those two: [`tree`] turns a document plus its
//! evaluation into feature-tree rows with typed status badges,
//! [`props`] turns a selected node into editable slot rows, [`docio`]
//! is `open`/`save` over the shipped persistence, [`evalseam`] is the
//! boundary evaluation runs behind (a background thread natively, and
//! nothing above it may assume one), and [`scene`] is the tessellation
//! the viewport draws.
//!
//! The toolkit's contribution is confined to two things it alone can
//! do: turning platform events into [`ViewportEvent`]s and
//! [`SessionOp`]s, and painting pixels. Everything between those two
//! ends has no `egui`, no `wgpu` and no window in sight, which is why
//! it is all exercised by `tests/` in ordinary headless CI.
//!
//! # The `app` feature
//!
//! The eframe application — windowing, the docked chrome, the wgpu
//! viewport — lives behind the non-default `app` feature
//! (`cargo run -p viewer --features app`). Without it this crate is
//! the renderer-free half above, and its dependency graph is the
//! kernel's. The manifest states why the feature is not on by
//! default; it is a CI-cost decision, not a statement about what the
//! deliverable is.
//!
//! # Chordal δ is the fidelity lever, never ε
//!
//! A view is drawn at a **display tolerance** ([`DisplayTolerance`]):
//! how far the triangles may sag from the exact surfaces. The kernel
//! tolerance ε — what the model *is* — is never touched by anything
//! in this crate (the ratified micro-decision in GUI-DESIGN, and
//! `mesh`'s own δ-is-not-ε contract).

pub mod bounds;
pub mod camera;
pub mod display;
pub mod docio;
pub mod evalseam;
pub mod frame;
pub mod history;
pub mod input;
pub mod matetool;
pub mod pick;
pub mod prefs;
pub mod props;
pub mod revolvetool;
pub mod scene;
pub mod session;
pub mod theme;
pub mod tree;

#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "app")]
mod gpu;

pub use camera::{Camera, CameraError, CameraOp, CameraOpError};
pub use docio::DocIoError;
pub use evalseam::{EvalDone, EvalRequest, EvalService, Generation, InlineEvaluator};
// The two seam lanes are meant to be interchangeable, so they are named
// the same way. `ThreadEvaluator` carries the `cfg` its module does.
pub use display::{DisplayFault, DisplayState, DisplayView, free_move_check, mates_naming};
#[cfg(not(target_family = "wasm"))]
pub use evalseam::{SpawnError, ThreadEvaluator};
pub use history::{History, HistoryId};
pub use input::{InputMap, PickAction, PointerButton, ViewportEvent, ViewportSize};
pub use matetool::{
    MateAdmission, MateChoice, MateProposal, MateTool, MateToolError, MateToolEvent, MateToolState,
    admitted_classes,
};
pub use pick::{
    Highlight, IdMap, IdMapError, PatchId, PickError, PickIndex, PickIndexError, cursor_projection,
    highlight,
};
pub use prefs::{Notice, Prefs, PrefsError, PrefsStore, StoreError};
pub use props::{SlotDriver, SlotFault, SlotRow, SlotValue};
pub use revolvetool::{RevolveSeat, RevolveTool, RevolveToolError, RevolveToolEvent};
pub use scene::{DisplayTolerance, SceneDocError, SceneError, SceneMesh, ScenePart, SceneStats};
pub use session::{
    DatumSpec, DocSession, FaceSelection, Landing, OpOutcome, ProfileShape, Refusal, Selection,
    SessionOp, Standing,
};
pub use theme::{Mark, Polarity, Safety, Theme};
pub use tree::{RowStatus, TreeRow};
