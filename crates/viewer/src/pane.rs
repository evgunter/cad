//! **The pane bodies**: one module per docked pane, each holding the
//! `*_ui` functions that draw it.
//!
//! Each is part of the `app` driver rather than a vocabulary — they
//! name `egui` and they hold `impl ViewerBehavior`. What they do not
//! do is mutate the session: a pane reads it as a value and pushes
//! [`crate::session::SessionOp`]s into a queue the application drains
//! after the layout has been walked (`crate::app`'s header states the
//! rule).
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

pub mod create;
pub mod features;
pub mod properties;
pub mod view;
pub mod viewport;
