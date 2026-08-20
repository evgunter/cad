//! Tessellation: watertight triangle meshes from B-rep bodies (M2 PR 6).
//!
//! [`fn@tessellate`] triangulates every face of a closed body into a
//! [`Mesh`] whose triangles are certified to lie within a caller-chosen
//! **chordal tolerance δ** of the exact analytic surfaces. Planar faces
//! (rings included) go through constrained Delaunay triangulation (CDT,
//! the `spade` crate) in the face plane; iso-rectangle curved analytic
//! faces through UV-grid sampling plus CDT in parameter space; curved
//! faces with conic TRIM boundaries (M5 PR 11) through the
//! pcurve-driven trimmed lane (`trimmed` module: stored PR 6 caches
//! provide the UV trim polyline, the planar lane's even-odd flood fill
//! classifies the interior).
//!
//! # Chordal tolerance δ is NOT the kernel tolerance ε
//!
//! δ is a **per-call display/export parameter**: how far the piecewise
//! linear mesh may sag from the exact geometry. It is deliberately not
//! the kernel's global ε (D4 ¶1, unaffected): ε defines what the *model*
//! is (predicate bands, residual certification); δ defines how coarsely
//! a *view* of the model is allowed to approximate it. Two calls with
//! different δ see the same body; no kernel state depends on δ.
//!
//! The bound is **certified-conservative** (ratified 2026-07-16): each
//! emitted triangle carries a closed-form per-surface-kind deviation
//! certificate checked against δ — an export promise, explicitly not a
//! kernel validity invariant. The certificates (derivations on
//! [`cert`]): plane — exact 0; cylinder — `r − dist(axis, T)` (radial
//! convexity, exact); sphere — `r − dist(center, T)` (the triangle
//! plane cuts the sphere in its circumcircle, exact); cone —
//! `cos α · ρ_maxᵀ · (1 − cos(Δu/2))` (perpendicular distance to the
//! generator ray at each point's azimuth; triangle-local max radius, so
//! apex fans certify tightly); torus — `(3/4)(R + 2r)·L_uv²` (linear
//! interpolation against the closed-form Hessian bound `R + 2r`);
//! described NURBS (M7, the trimmed-NURBS lane) — the
//! same interpolation derivation against a **hull-derived** Hessian
//! bound (second-derivative control nets by knot differencing, sup by
//! convexity — the `nurbs_cert` module's anisotropic
//! `(muu·a_u² + 2·muv·a_u·a_v + mvv·a_v²)/4`; RATIONAL faces since
//! M8-5, through the quotient-rule assembly over the homogeneous
//! nets); illegal-rational or C⁰-creased
//! NURBS faces refuse typed (partial coverage stated, never an
//! estimated bound). All
//! bounds are exact-arithmetic statements about the vertex positions;
//! two documented additive slacks sit outside them: boundary vertices
//! lie on certified edge carriers (within the kernel's ε of each
//! adjacent surface, D4 ¶2) rather than exactly on the surface, and
//! f64 rounding of the evaluations themselves. The honest promise is
//! therefore δ + ε (+ rounding); grid sizing targets δ/2 so the slack
//! never decides in practice. ε is never *read* for sizing — mesh
//! structure is a function of (body, δ) alone (D9).
//!
//! # Watertightness and the memo-key contract
//!
//! Shared-edge chord points are computed **once per edge** from the
//! edge's certified carrier and consumed by both adjacent faces; chord
//! polyline endpoints are the topology vertices' points bitwise. Every
//! boundary polyline segment is inserted as a CDT constraint in both
//! adjacent faces, so the two triangulations conform to the same
//! segments and the mesh is watertight by construction (validated by
//! [`validate::check_mesh`]).
//!
//! **Invariant (ratified via PR #32): per-face tessellation is a pure
//! function of (face surface, loops, per-edge chord points, δ).** This
//! is the memo-key contract future incremental re-tessellation
//! consumes: a face whose surface, loops, and boundary chord points are
//! unchanged re-tessellates identically and its patch can be reused
//! across rebuilds. The chord points themselves are a pure function of
//! (edge carrier + interval, endpoint vertex points, the adjacent
//! faces' surface parameters, δ) — adjacent surfaces enter only through
//! the torus and trimmed-NURBS boundary-step requirements, documented
//! on [`chords`].
//!
//! # Structure kept, structure dropped
//!
//! - **Per-face patch separability** (ratified): [`Mesh`] keeps one
//!   [`FacePatch`] per face, individually addressable; nothing flattens
//!   the per-face structure away. No keying machinery at M2.
//! - **Entity back-references** (ratified, incl. the PR #32 Vertex-key
//!   addition): every patch carries its source `FaceKey`; every
//!   boundary polyline its source `EdgeKey` (each segment is a
//!   consecutive point pair of that polyline) and its endpoint
//!   `VertexKey`s. STL export (PR 7) simply drops them.
//! - **No appearance artifact** (final, Evan 2026-07-20): display
//!   attributes live in the document layer from M4, keyed by stable
//!   names — nothing attaches anywhere at M2 (DESIGN.md Band 1).
//! - **No face merging** (ratified): coplanar/cosurface neighbors stay
//!   separate patches; conventional split edges tessellate like any
//!   other shared edge.
//!
//! # Orientation, seams, poles
//!
//! Outward triangle winding falls out of the loop conventions (D1
//! interior-left): the boundary walk's UV polygon is CCW exactly when
//! the face's outward normal agrees with the chart normal; a negative
//! UV signed area flips the emitted triangles.
//!
//! **The orientation-sense contract (M5 S10).** A face's outward
//! normal is `topo::Face::sense_sign() · chart_normal`, so "agrees
//! with the chart normal" is now a real question with a stored answer
//! — and the paragraph above is why this crate almost never has to ask
//! it. Winding is read off the loop's *stored traversal*, and `revert`
//! reverses the loops and flips `sense` together, so a reversed face
//! arrives with a negative UV/projected area and flips itself.
//! Multiplying those shoelaces by `sense_sign` would double-count the
//! reversal — the hazard is named in place at both sites (`planar`,
//! `curved`). The single exception is the pole-to-pole band's
//! azimuth disambiguation below, which reads a *direction in the chart
//! frame* rather than a winding; it is the one `sense_sign` read in
//! this crate. Every face this build mints has `sense: true`, so that
//! read is `· 1.0` and the mesh is bitwise unchanged.
//!
//! The walk assigns each iso side's constant coordinate once per ISO
//! SIDE — not per point, and (since #653) not per edge either, so a
//! side carried by several edges is bitwise straight too. It was per
//! EDGE for a milestone, which left a multiply-carried side only
//! ANALYTICALLY straight and, under a general rigid placement, ulps
//! apart; `walk::iso_side_starts` groups the edges into runs and gives
//! each run one coordinate. `curved`'s domain guard still bands its
//! comparison in metres, now as a backstop rather than as the thing
//! that keeps valid parts from being refused. A rim-anchored loop's
//! closure is exact for a second, independent reason: it takes its
//! column from the closing vertex itself (S22).
//!
//! Periodic charts are
//! unwrapped by continuity along the walk (chord steps are capped at
//! π/4, so branch choice is unambiguous); a full-2π patch traverses its
//! seam meridian twice, at u = 0 and u = 2π, with both traversals
//! consuming the same 3-D chord points, which welds the seam. Sphere
//! poles and the cone apex are chart singularities handled from the
//! loop structure — they enter the CDT as duplicated UV corner points
//! whose triangles collapse-and-drop into a fan around the single pole
//! mesh vertex. The collapse yields ONE fan, rather than two
//! overlapping ones, only while the interior grid separates the two
//! corner points; `curved::pole_columns` is what guarantees that
//! (issue #678 — at `nu == 2` a single equidistant column gives both
//! corners a fan over it and the identified edge is used four times).
//! `Surface::normal` is never sampled anywhere (winding
//! needs no normals), so the ∂u → 0 poison is unreachable. Pole-to-pole
//! bands (no rim in the loop) disambiguate their azimuth half via the
//! loop's 3-D area vector, taken into the chart frame through the
//! face's `sense_sign` — see [`walk`]. (That threading replaced the
//! former "assumes outward-oriented shells" assumption; the band is
//! the same face kind whose flux sign `geom_brep::props::curved`
//! cannot read off any rim, and it takes the same fix.)
//!
//! # Determinism (D9)
//!
//! Byte-identical [`Mesh`] for identical (body, δ): mesh vertex ids are
//! minted in fixed arena iteration order (vertices, then per-edge
//! interior chord points, then per-face interior grid points); chord
//! and grid counts come from deterministic ceil arithmetic; CDT
//! insertion order is fixed (boundary walk order, then grid points
//! row-major), and `spade`'s insertion algorithm is deterministic in
//! insertion order (Vec-based DCEL, no randomized data structures in
//! the insertion/constraint path — the one place insertion order could
//! leak is cocircular tie-breaking, which is a function of insertion
//! order and therefore fixed here).
//!
//! # Performance (documented characteristic)
//!
//! Wall-clock on the CDT insertion path is **quadratic for faces whose
//! boundary is two or more nested near-cocircular loops** — a planar
//! face with a hole (washer, plate-with-hole, counterbore, boss∪plate,
//! die pip) — and **near-linear otherwise**. Measured on a washer body:
//! ~19 ms at δ = 1e-4, ~1.2 s at 1e-6, over 11 minutes at 1e-9. Point
//! counts scale like 1/√δ per axis, so each 100× tightening of δ costs
//! ~100× more triangles. Fine-tolerance STL export (PR 7) should expect
//! this; the [`TessellateError::ResolutionOverflow`] 2²⁴ cap bounds
//! *allocation*, not wall-clock — a δ well inside the cap can still
//! take hours.
//!
//! Re-measured 2026-08-14 (`docs/PERF-SCAN-2026-08.md` finding 7b), in
//! an isolated harness against the pinned `spade` that reproduces the
//! washer figure above to within noise. Three corrections to what this
//! section used to claim:
//!
//! - The blow-up is **not** general in per-face point count. A swept UV
//!   rectangle (the cylinder/cone/sphere/torus/NURBS lanes) runs 66 049
//!   points in ~101 ms; a single circular loop, 65 536 points in
//!   ~144 ms. An annulus — two concentric loops — takes ~609 ms at
//!   16 384 points and ~8.9 s at 65 536: 4× the points for 14.6× the
//!   time. The washer's whole 1.2 s is its two planar annulus faces.
//! - The old text derived "~10⁴× more CDT time" per 100× δ tightening.
//!   Its own datapoints measure **63×** (1e-4 → 1e-6), not ~10⁴×.
//! - Insertion order does not rescue it (outer-first 8.9 s, inner-first
//!   5.6 s, interleaved 8.6 s), and neither does the hint generator:
//!   `HierarchyHintGenerator<f64>` measured 39.26 s against the default
//!   `LastUsedVertexHintGenerator`'s 39.05 s on a 131k-point annulus.
//!   **The "hierarchy-hinted insertion" half of the remedy this section
//!   used to name is a dead end** — the cost is the flip/legalization
//!   cascade against a degenerate cocircular hull, not point location.
//!
//! **Bulk loading is the remedy that works**: `spade`'s `bulk_load_cdt`
//! measures 257 ms against 8 914 ms on the 65 536-point annulus (~35×),
//! and the gap grows with point count. It has NOT been adopted, and it
//! must not be adopted against stock `spade` 2.15.1: that version's
//! bulk loader iterates a `std::collections::HashSet` under the default
//! randomly-seeded `RandomState` on its skipped-vertex/skipped-edge
//! paths, which fire on cocircular input — exactly ours — and would let
//! mesh bytes vary run to run, violating D9. Upstream a `Vec`/
//! `BTreeSet` fix and `[patch.crates-io]` it first.
//!
//! **Nothing here has a mechanical guard, and nothing can** (§Q6 of
//! `docs/prompts/reviewer-style-lane.md`): every figure above is a
//! wall-clock timing on a named box, so no row in the tree can go red
//! when one stops being true. That is not hypothetical — the "~10⁴× more CDT
//! time" this section used to claim was wrong by ~150× **on its own
//! datapoints**, for three milestones, and what caught it was a scan run
//! for another reason. What re-checks these numbers is the perf lane's
//! periodic re-measurement, which is real (it produced the corrections
//! above) but unscheduled. Read them as dated observations, not as
//! premises to compute with.
//!
//! # Scalar policy (judgment call, reported in the PR)
//!
//! This crate is **f64-only**: it takes `&Body<f64>` and performs raw
//! f64 comparisons freely. Tessellation is a display/export operation
//! downstream of the kernel — none of its branches decide kernel
//! topology or feed anything back into a body, so the Q1 reified-
//! predicate discipline does not apply (its comparisons are honest
//! display-layer choices, backstopped by the per-triangle certificates
//! and [`validate::check_mesh`]). Genericity over `Real` would buy
//! nothing here (a mesh of intervals is not a displayable artifact) and
//! `spade` wants f64 coordinates; D8 replay at other scalars reaches
//! display through the f64 lane.

// The tessellation budget meter's kernel half (issue #320). Public in
// both configurations, because its measurement types are the CONTRACT
// with the consumer half (`tools/tess-meter`), which reads them
// without needing the instrument compiled in. What the feature gates
// is the instrument itself: `arm` and `take` exist only in the live
// half, so a build without the feature exports the measurement types,
// a `const fn armed()` answering `false`, and no way to arm anything.
pub mod budget;
pub mod cert;
mod chords;
mod curved;
mod nurbs_cert;
mod planar;
pub mod sizing;
mod tessellate;
mod trimmed;
pub mod types;
pub mod validate;
pub mod walk;

pub use tessellate::tessellate;
pub use types::{BoundaryPolyline, FacePatch, Mesh, TessellateError};
