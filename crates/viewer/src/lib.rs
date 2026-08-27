//! The viewer: layer 3 of the GUI/editor architecture
//! (`docs/GUI-DESIGN.md` G1) — interaction over the headless
//! `editor-core` document and the kernel below it.
//!
//! # What is a value here, and what is a widget
//!
//! G1's operations-are-API rule binds this crate: **every navigation
//! move is a typed operation on a state value, callable with no
//! renderer present.** [`Camera`] is that value for viewport
//! navigation, [`CameraOp`] is the vocabulary of moves, and
//! [`camera::apply`] is the one function that performs them. The
//! toolkit's contribution is confined to two things it alone can do:
//! turning platform pointer events into [`ViewportEvent`]s, and
//! painting pixels. Everything between those two ends —
//! [`InputMap::map`], [`camera::apply`], [`SceneMesh::build`] — is
//! ordinary library code with no `egui`, no `wgpu` and no window in
//! sight, which is why it is all exercised by `tests/` in ordinary
//! headless CI.
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

pub mod camera;
pub mod input;
pub mod scene;

#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "app")]
mod gpu;

pub use camera::{Camera, CameraError, CameraOp, CameraOpError};
pub use input::{InputMap, PointerButton, ViewportEvent, ViewportSize};
pub use scene::{DisplayTolerance, SceneDocError, SceneError, SceneMesh, SceneStats};
