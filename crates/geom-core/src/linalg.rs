//! Fixed-dimension linear algebra for the 2-D/3-D evaluation layer:
//! vectors and points in both dimensions, matrices and affine maps in
//! 3-D only (see the omission below).
//!
//! Hand-rolled per `docs/DESIGN.md`'s layering table — small, fixed
//! dimension, and generic over our own [`Real`](crate::real::Real) scalar
//! trait. No external
//! linear-algebra crate sits in the kernel core: the scalar abstraction
//! (and with it the totality, determinism, and no-comparison disciplines)
//! is ours to control.
//!
//! # The affine/linear distinction is load-bearing
//!
//! Euclidean 2-/3-space is an *affine* space: a set of points acted on
//! simply transitively by its translation vector space (equivalently, its
//! tangent space — canonically the same space at every point, since the
//! space is flat). The types encode the two sides of that torsor structure
//! separately, so category errors fail to typecheck:
//!
//! - [`Vec2`] / [`Vec3`] are the **linear side** — tangent vectors:
//!   displacements, directions, normals, derivatives. They form a vector
//!   space: addition, negation, right scalar multiplication, dot/cross.
//! - [`Point2`] / [`Point3`] are the **affine side** — locations. Points
//!   do not add or scale; the operations are `Point − Point = Vec`,
//!   `Point ± Vec = Point`, and affine combinations ([`Point3::lerp`]).
//!   `Point + Point` does not typecheck, by design.
//! - [`Mat3`] is a **linear endomorphism of the tangent space**, stored
//!   column-major as named column fields.
//! - [`Affine3`] is an **affine map**: a linear part (the
//!   map's differential, which is all a pushed-forward tangent vector
//!   feels) plus a translation (which only points feel).
//!
//! # Deliberate omissions
//!
//! - **No 2-D linear or affine maps.** [`Vec2`] and [`Point2`] have no
//!   `Mat2`/`Affine2` to move them: the asymmetry with the 3-D half is
//!   intended, not an oversight. Nothing in the kernel maps the 2-D
//!   tangent space through a stored matrix — chart work carries its own
//!   parameterization — and this layer adds only on consumer demand, so
//!   the pair is absent until something asks for it.
//! - **No array storage, no indexing, no `Index` impls.** Indexing has a
//!   panic path on out-of-range input, and D9 forbids panic paths; named
//!   fields (`x`, `y`, `z`, `c0`, …) make every component access total
//!   and readable.
//! - **No `PartialEq` / `PartialOrd` / `Hash` derives.** Comparison is
//!   the predicate layer's job (M0 PR 3): geometric equality is a
//!   tolerance decision, never a bit pattern — and the scalar `T` carries
//!   no comparison surface anyway (see `real.rs` on why).
//! - **No left scalar multiplication `s * v`.** That impl must live on
//!   the *scalar* type (`impl Mul<Vec3<T>> for T`), which coherence
//!   forbids for a generic scalar and which we decline to special-case
//!   for `f64` (generic evaluation code could not use it). Write `v * s`.
//! - **No `Display`.** `Debug` (a [`Real`](crate::real::Real) supertrait)
//!   is the diagnostic
//!   affordance; user-facing formatting belongs above the kernel.
//!
//! # Totality and determinism
//!
//! Every operation is **total** — no `Result`, no panic. Out-of-domain
//! inputs ([`Vec3::normalize`] of the zero vector, [`Mat3::inverse`] of a
//! singular matrix) produce poison values (NaN at `f64`) that flow through
//! subsequent *arithmetic*; only the predicate layer turns numbers into
//! *decisions*, and certified residual checks catch poisoned caches (D4
//! ¶2). This is the crate-wide policy documented in `real.rs`.
//!
//! Every reduction — dot, cross, determinant, matrix products — has a
//! **fixed, documented association order** (D9: deterministic evaluation;
//! reassociating floating-point sums changes results). The order is stated
//! in each operation's doc comment and is part of its contract.
//!
//! The [`svd`] submodule (M5 PR 7) is the second C12.8 addition: the
//! fixed-shape 2×3/3×4 decomposition of the SSI marcher's underdetermined
//! derivative systems (Householder QR + one-sided Jacobi, fixed
//! reflection/rotation order). Like [`lsq`] it is `f64`-only — the
//! marcher is a candidate generator and the certificate, not the
//! stepper, is what runs at every `T`.
//!
//! The [`frame`] submodule (LIB-U4b) is the placement vocabulary:
//! point-at, mirror, and the path-start frame, as plain [`Affine3`]
//! values rather than a new type. It is the one deciding module here —
//! its degeneracy policy is stated in its own docs and routed through
//! the predicate funnel, so the totality contract above is unchanged
//! for everything else in this layer.
//!
//! The [`lsq`] submodule (M5 PR 4) is the one variable-size resident:
//! `f64`-only structure machinery for the fitting systems (C6's f64
//! lane), `Vec`-based with shapes validated at entry so every internal
//! index is justified — the no-panic rule holds there too, with typed
//! refusals in place of the fixed-dimension types' totality.

mod affine;
pub mod frame;
pub mod lsq;
mod mat;
mod point;
pub mod svd;
mod vec;

pub use affine::Affine3;
pub use frame::{FrameError, FrameInput};
pub use mat::Mat3;
pub use point::{Point2, Point3};
pub use svd::{Svd, Svd2x3, Svd3x4};
pub use vec::{Vec2, Vec3};
