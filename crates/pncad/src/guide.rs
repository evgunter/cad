//! The narrative guide, rendered here and **executed** here.
//!
//! The guide's prose lives in Markdown under `docs/` so it reads on
//! its own; this module pulls those pages into rustdoc with
//! `include_str!`, which has one load-bearing consequence: **every
//! Rust code block in the guide is a doctest**. A guide example that
//! stops compiling, or whose assertion stops holding, is a red build
//! — not a documentation bug someone notices months later.
//!
//! The convention across the guide pages:
//!
//! - Unlabelled and ` ```rust ` blocks are Rust doctests — compiled
//!   and run by `cargo test --doc -p pncad`.
//! - ` ```python ` blocks are skipped by rustdoc and executed instead
//!   by `crates/pncad-py/tests/test_guide.py`, which reads the same
//!   Markdown files. Nothing in the guide is transcribed twice.
//! - ` ```console ` blocks are shell transcripts, checked by eye.
//!
//! This module contains no code and is compiled away to nothing; it
//! exists so that the documentation has somewhere to be true.

/// The guide proper: quickstart, then the canonical journey from
/// authoring to export, in Rust and Python side by side.
pub mod journey {
    #![doc = include_str!("../../../docs/GUIDE.md")]
}

/// The corpus as the example set — an index of every tour scene and
/// every corpus document, and what each one is evidence for.
pub mod examples {
    #![doc = include_str!("../../../docs/guide/examples.md")]
}

/// The fail-loud tour: what a typed refusal looks like at each layer,
/// and how to read one.
pub mod fail_loud {
    #![doc = include_str!("../../../docs/guide/fail-loud.md")]
}

/// Selecting entities: materializers, the structural pattern
/// language, the geometric filters, the doors from a name back to
/// geometry, and the detect/declare protocol.
pub mod selecting {
    #![doc = include_str!("../../../docs/guide/selecting.md")]
}

/// Meshing: the ladder's tessellate and cross-check rungs, what a
/// `Mesh` carries across the Python boundary, and STL.
pub mod meshing {
    #![doc = include_str!("../../../docs/guide/meshing.md")]
}

/// The north-star audit: which demos are authorable through the
/// Python bindings today, and the named gap for each that is not.
pub mod north_star_audit {
    #![doc = include_str!("../../../docs/guide/north-star-audit.md")]
}
