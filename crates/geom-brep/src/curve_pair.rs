//! **The curve-pair meeting reader**: whether, and where, two carrier
//! spans meet (C5 in `crates/geom-brep/README.md`, D3's "One reading per
//! question"). Its core is one point-on-span reading ([`span`]), which
//! settles every candidate point a meeting method proposes.

pub mod span;

pub use span::{Contact, Span};

#[cfg(test)]
mod span_rows;
