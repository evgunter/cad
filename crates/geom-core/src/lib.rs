//! The scalar / tolerance / predicate substrate of the CAD kernel.
//!
//! `geom-core` is the bottom layer everything else stands on: the `Real`
//! scalar trait (instantiated at `f64`, intervals, and dual numbers), the
//! single global `Tolerance` value, the trilean robust predicates, and the
//! small fixed-dimension linear algebra that the geometry layers build upon.
//! It carries the determinism charter (D9) from the first line — no panics on
//! input, typed errors, transcendentals via `libm`, essentially no unsafe.
//! See `docs/DESIGN.md` (decisions D4, D9, and open question Q1) for the
//! design contract this crate implements.
