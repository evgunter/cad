//! Sweep operations: solids from validated 2-D profiles (M2 PR 4:
//! [`fn@extrude`]; PR 5 adds revolve).
//!
//! This crate sits on top of the whole M2 stack: it consumes the
//! `profile` crate's [`profile::ValidatedProfile`] (the only accepted
//! input — raw profiles must pass `Profile::validate` first), drives
//! `topo`'s certified Euler operators (Mäntylä ch. 12's translational
//! sweep, re-derived under our counterclockwise convention), and
//! attaches real geometry throughout: D2 intensional edge descriptions
//! with certified carrier caches (`geom-brep`), analytic side surfaces
//! (`geom`), Newell-certified cap planes. Every
//! topology-determining comparison goes through named trilean
//! predicates (Q1); every failure is a typed error (D4 ¶3); everything
//! is generic over [`geom_core::Real`] via the [`geom_core::Decide`]
//! door and instantiates at `f64`, `Dual<f64>`, `Interval`, and
//! `Dual<Interval>` (D8) — including inexact (non-dyadic, rotated,
//! arc-carrying) geometry at `Interval`, since geom-core's
//! tight-square `norm_squared` fix (M2 PR 3 fix pass, B1) keeps
//! straddling-zero residual enclosures decorated through `sqrt`.
//!
//! # Direction conventions (normative, stated once — owned here)
//!
//! The profile crate canonicalizes loops in **sketch coordinates**: the
//! outer loop counterclockwise, holes clockwise, viewed from the tip of
//! the sketch plane's normal `n = u × v` (the placement's third linear
//! column). The sweep owns how that winding meets 3-space:
//!
//! - **Extrusion direction.** The extrusion vector `w` must be
//!   trilean-parallel to `n` (M2 restriction: oblique extrusion is a
//!   typed error — see [`ExtrudeError::ObliqueExtrusion`]). The signed
//!   normal component `w · n` classifies through the named predicate
//!   `extrusion_normal_component` (margin in meters — it *is* the
//!   displacement): definitely positive or negative proceeds; zero
//!   (in-plane, or a sliver-thin extrusion) is
//!   [`ExtrudeError::DegenerateExtrusion`]; in-band escalates.
//! - **Which cap carries the profile winding.** The **bottom cap lies
//!   on the sketch plane**, the top cap on the plane translated by `w`.
//!   Under the ratified interior-left rule (outer loops
//!   counterclockwise viewed from outside), the cap whose outward side
//!   is `+n` carries the canonical winding: extruding along `+n`
//!   (`w · n > 0`) puts the canonical winding on the **top** cap;
//!   extruding along `−n` puts it on the **bottom** cap. Implementation
//!   form: the swept-face loops traverse the canonical chains as-is for
//!   `w · n > 0` and **reversed** (endpoints swapped, bulges negated,
//!   turns flipped — the profile crate's reversal involution) for
//!   `w · n < 0`; the built solid is outward-oriented in both cases.
//! - **Arc carriers: axis = turn-signed plane normal.** A profile arc
//!   segment's carrier circle (rim edges) and carrier cylinder (side
//!   faces) take `axis = +n` for a counterclockwise (positive-turn)
//!   segment and `axis = −n` for a clockwise one, so that increasing
//!   carrier parameter always runs along the segment's traversal —
//!   satisfying the ratified `he_plus` forward contract with positive
//!   parameter spans. The span is **θ = 4·atan|bulge|**, from the
//!   stored bulge (the sanctioned re-inspection; never endpoint
//!   `atan2`). A carrier circle's `u_ref` points from the center at the
//!   segment's start vertex; a shared side cylinder's `u_ref` comes
//!   from the first segment of its cosurface run in sweep order (seam
//!   placement is conventional data, D2 — no `Seam` edges exist in an
//!   extrusion, and none of these choices obstructs PR 5's use of
//!   `Seam { surface }` at `u = 0`).
//!
//! # What an extrusion stores (the D2 story, applied)
//!
//! - **Side struts** — `MappedCurve::ExtrudedPoint { point, place,
//!   vec }`; **bottom rims** minted as `MappedCurve::PlacedSegment` at
//!   the sketch placement; **top rims** minted as `PlacedSegment` at
//!   the placement translated by `w` (PR 3's binding handoff).
//! - **Profile-corner joins**: once both side faces of a join exist,
//!   the join (strut) edge upgrades to `Intersection { s1, s2,
//!   witness }` via `topo`'s certified `set_edge_curve`
//!   (mint-time `Intersection` is impossible — the surfaces don't exist
//!   yet). The choice is `geom_brep::classify_dihedral` at the strut
//!   midpoint with the strut chord as extent: Transverse ⇒ upgrade;
//!   Smooth ⇒ the edge keeps its conventional `MappedCurve` description
//!   (the ratified no-face-merging split, D2); Indeterminate ⇒
//!   [`ExtrudeError::SliverJoin`] (escalate-never-guess).
//! - **Cap–wall rims upgrade too** (the ratified rim decision — Evan,
//!   M2-LOG 2026-07-19): after both cap planes are set, every rim edge
//!   (bottom and top, outer and ring loops) re-describes as
//!   `Intersection { cap plane, side surface, witness }` through the
//!   same `classify_dihedral` → `set_edge_curve` pattern, with the
//!   witness minted as the **carrier's mid-parameter point** (the S2
//!   witness contract). Every rim of a normal extrusion is definitely
//!   transverse (cap ⊥ wall), matching tier 3's prefer-intrinsic
//!   enforcement: at rest, definitely-transverse edges must carry
//!   `Intersection`.
//! - **Cosurface sharing**: smooth joins whose side faces lie on the
//!   identical-by-construction surface — collinear line segments (one
//!   plane), tangent arcs on one carrier circle (one cylinder) — share
//!   the surface **key** (`FaceSurface::Shared`), decided by the named
//!   predicates `side_planes_cosurface` (margin: perpendicular distance
//!   of the next chord's far endpoint from the previous carrier line)
//!   and `side_cylinders_cosurface` (margin: center distance plus
//!   radius difference, meters). All of a loop's consecutive-pair
//!   decisions (including the wrap pair at the canonical start vertex)
//!   are made **before any wall is minted**, so a same-carrier run that
//!   crosses the canonical start still resolves to one key — its
//!   `u_ref` comes from the run's first segment in sweep order, which
//!   for a wrap-crossing run is segment 0. Smooth joins across
//!   genuinely distinct surfaces (line–arc tangency: plane–cylinder)
//!   keep distinct surfaces and a conventional join edge.
//! - **Caps** via `geom_brep::newell_plane` over the loop vertices in
//!   `next` order (outer loop in next order ⇒ outward normal).
//!
//! # Holes
//!
//! A profile hole becomes a ring in **both** caps plus a band of wall
//! faces: the hole cycle is planted in the swept face as a ring
//! (strut `mev` + `kemr` — the §9.3 hole-planting state — then grown
//! and closed), its transient disc face is consumed by a same-shell
//! `kfmrh` that demotes the disc's loop into the bottom cap — the genus
//! supplier: an extrusion with `h` holes has genus `h` (its boundary is
//! an `h`-holed torus; the component Euler–Poincaré identity
//! `v − e + f − r = 2(1 − g)` balances with `r = 2h` rings).
//!
//! # K-telemetry
//!
//! Decisions here are name-tagged through the same local funnel shape
//! as `geom-brep`'s, which delegates to the unified recorder funnel
//! `geom_core::k_stats::decide` (M2 PR 7) — a `Probe`-lane run records
//! every decision under its predicate name.

pub mod extrude;
pub mod loft;
pub mod revolve;
pub mod skin;
// The lowering the sweep verbs share (its own docs say what is in it
// and what deliberately is not); crate-internal, so not `pub`.
mod swept;
// The gate is the module's own subject — see its docs for why
// `cfg(test)` cannot serve and what each arm buys. `doc(hidden)`
// because this repo's rustdoc gate runs `--all-features`, which turns
// `test-support` ON: without it the gate would publish, as public API, a
// module whose docs say it is not one.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub mod test_support;

pub use extrude::{ExtrudeError, Extruded, Extrusion, extrude};
pub use loft::{LoftError, Lofted, loft_body, sweep_body};
pub use revolve::tube::{TubeError, TubeWindow, tube_along_arc};
pub use revolve::{
    Revolution, RevolveAxis, RevolveError, Revolved, RevolvedKind, WedgeCapsError, WedgeFrames,
    revolve, revolved_caps,
};
// `SketchSegment` is re-exported for `segment_curve`, the retained
// 2-D-segment → 3-D-curve door (step-export builds exact arc path
// legs through it — the LIB-U4 exact-path territory): a caller must
// be able to spell its input without depending on `geom-brep`
// directly. Loft/sweep SECTIONS no longer speak it (LIB-U3): they
// are `Section`s — `profile::ProfileLoop` lists, re-exported here so
// section authors need not depend on `profile` directly.
pub use geom_brep::SketchSegment;
pub use profile::{ProfileLoop, ProfileVertex};
pub use skin::{
    LoftGeometry, Section, SkinError, lift_surface, loft_geometry, loft_parameters,
    make_compatible, segment_curve, skin, skin_on, skin_parameters, sweep_geometry, sweep_places,
};

pub mod chamfer;
pub mod fillet;
