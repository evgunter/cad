//! The B-rep geometry layer: D2's intensional edge descriptions,
//! certified carrier caches, the dihedral classification predicate, and
//! Newell face equations (M2 PR 3).
//!
//! This crate sits between the evaluators (`geom-curves` /
//! `geom-surfaces`) and the arena store (`topo`): it defines **what an
//! edge's geometry is** ([`EdgeGeometry`] — a description, never a bare
//! curve), **how a concrete cache earns its place** ([`EdgeCurve`] —
//! certification against the description, D4 ¶2; an uncertified carrier
//! is unrepresentable), and the two geometric classifiers M2's
//! constructions and tier-3 validation stand on:
//!
//! - [`classify_dihedral`] — the material wedge-angle predicate's first
//!   arrival: transverse / smooth / sliver-escalation, via
//!   implicit-form gradients (never chart normals) and D4 ¶1's
//!   displacement-through-lever-arm margins.
//! - [`newell_plane`] — certified planes from loop vertex data,
//!   translate-to-origin by default.
//!
//! The geometry-arena key types ([`PointKey`], [`CurveKey`],
//! [`SurfaceKey`]) are defined here (descriptions reference surfaces by
//! arena key) and re-exported by `topo` for its `Body<T>` arenas —
//! see [`keys`] for the layering rationale and Q1's lineage scoping.
//!
//! # Discipline (inherited, uniform)
//!
//! Everything is generic over [`geom_core::Real`] with decisions only
//! through [`geom_core::Decide`]'s trilean door (named predicates, D4
//! ¶3 escalation); evaluation is comparison-free and total (poison in,
//! poison out — including the `Nurbs` representable-unimplemented
//! placeholders, which certification rejects loudly); sampling is
//! deterministic by fixed schedule (D9). Instantiates at `f64`,
//! `Dual<f64>`, `Interval`, and `Dual<Interval>`.

pub mod certify;
pub mod dihedral;
pub mod edge_geometry;
pub mod implicit;
pub mod keys;
pub mod newell;

pub use certify::{CERT_SAMPLES, CertCheck, Certificate, CertifyError, EdgeCurve, EdgeCurveSpec};
pub use dihedral::{DihedralClass, classify_dihedral};
pub use edge_geometry::{EdgeGeometry, MappedCurve, SketchSegment};
pub use implicit::{curvature_lever_arm, implicit_gradient, implicit_residual};
pub use keys::{CurveKey, PointKey, SurfaceKey};
pub use newell::{NewellError, newell_plane};
