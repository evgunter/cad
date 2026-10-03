//! **Per-half-edge pcurve caches**: minting, the face-level one-branch
//! walk, and the derive-on-demand accessor (M5 PR 6; C4).
//!
//! The cache *value* and its certification gate live in
//! [`geom_brep::pcurve_cache`]; this module is everything that needs
//! half-edges, loops, and faces — i.e. everything about **where** a
//! pcurve lives and **which branch** it takes.
//!
//! # The half-edge IS the key (spec §1)
//!
//! A pcurve belongs to an (edge, face-side) incidence, and the
//! half-edge is exactly that incidence. Seam edges are the forcing
//! case and the reason no coarser key works: a full cylinder wall
//! closed by its seam meridian has **both half-edges of one edge in
//! one loop of one face**, on one surface, with two different chart
//! curves (`u = α` and `u = α + 2π`). "Per edge" cannot hold two;
//! "per (edge, face)" cannot hold two either. Per half-edge has no
//! special case — [`crate::Body::pcurve`].
//!
//! # What gets minted, and what deliberately does not
//!
//! - **Planar faces store nothing.** M2's derive-on-demand status
//!   stands (C4 verbatim): a plane chart is affine, so its pcurve is an
//!   exact closed form with no point inversion to re-run, and C4's own
//!   argument for storing (avoiding a hidden iterative inversion per
//!   query) does not apply. An all-planar body therefore carries **zero
//!   stored pcurves** — pinned by test.
//! - **Cylinder charts mint.** They are what the C5 table's
//!   Plane×Cylinder splitting lane produces, and every carrier that
//!   lane mints (rim circle, seam/meridian line, tilted-section conic)
//!   has an exact closed-form cylinder-chart image.
//! - **Cone / sphere / torus charts mint (M6-3, walk row 4).** Their
//!   closed-form classes (cone rims/rulings; sphere polar/meridian
//!   circles; torus parallels/meridians) derive and certify exactly as
//!   the cylinder's; the sphere walk additionally knows the chart's
//!   involution twin and the poles where azimuth names no point (see
//!   `sphere_twin`/[`singular_at`]). A sphere's GENERAL circle (neither
//!   polar nor meridian) has no closed form and takes the fitted lane:
//!   its image from [`geom_brep::FittedLane::sphere_circle_image`],
//!   certified by [`geom_brep::PcurveCache::certify_fitted`]'s Circle
//!   arm (`analytic_derive`). Any other carrier outside the closed-form
//!   classes that can still lie on the chart refuses
//!   [`PcurveCertifyError::UnsupportedCarrier`] with the class named,
//!   and its face stays uncached, excused by C4's exemption
//!   ([`not_owed`]) until the class's route lands — the cone/torus
//!   oblique classes have no honest route yet (no ring-computable
//!   meters composite). A carrier
//!   that cannot lie on its face
//!   ([`PcurveCertifyError::CarrierOffChart`]) is a defect, and the
//!   pass refuses with it.
//! - **Described NURBS charts mint** their iso lane (M6-3,
//!   `nurbs_iso_derive`) — RATIONAL ones too since M8-3, whose ARC cap
//!   rims map through the chart's own rational-quadratic parameter
//!   (`Pcurve::IsoArc`). Only the mvfs placeholder mints nothing: it
//!   is not a described surface.
//!
//! # The one-branch walk (spec §3)
//!
//! A [`geom_brep::Pcurve`] cannot express a branch jump: its azimuth
//! channel is `α + β·t` with one stored `α`. What remains is *which*
//! branch `α + kτ` each half-edge of a face takes, and that is decided
//! **once per loop**, by walking the loop in `next` order and lifting
//! each half-edge's entry onto the previous half-edge's exit. Two chart
//! points with one 3-D image differ by a deck element of the chart, so
//! each joint states WHICH one, as integers decided with half the step
//! to the next orbit point as room ([`lift_joint`]): the whole periods
//! per periodic channel and, on a sphere, the involution twin. A joint
//! no representation lifts
//! refuses typed ([`PcurveMintError::LoopDiscontinuity`]). The joint's
//! 3-D coincidence is not decided again in the chart: it follows from
//! the two rows' envelopes and the edge certificate's endpoint pinning.
//! The loop must also close, with a winding of zero, or exactly one
//! period for a loop that wraps the chart (the seam case). A vertex on
//! the chart's singular set (a sphere's pole, a cone's apex) is decided
//! as 3-D incidence, and lifts no azimuth; a spline chart's joint also
//! states its chart-space gap.
//!
//! This is the M2 PR 5 meridian finding generalized:
//! "the junction's meridian column unwrapped nearest prev_u, but past
//! 3π/2 the wrong branch is closer"). The fix there was to anchor the
//! unwrap to a point that is *exact by construction* rather than
//! nearest-previous per sample; here the anchor is the loop's own
//! chain of shared vertices, the per-sample choice does not exist at
//! all, and the anchor's correctness is decided rather than assumed.
//!
//! # The chart-boundary description (TRIM-3)
//!
//! [`chart_boundary`] is the walk's second consumer: it runs the same
//! [`walk_loop`] on a chart the CALLER names and turns each loop into
//! a closed chord polygon — [`crate::chart_bound::ChartBound`], whose
//! outside test is what a subdivision driver asks. It mints nothing
//! and stores nothing; it reads a pcurve CACHE only for a
//! `Fitted`/`General` image's certified envelope. The charts it
//! describes and the ones it refuses are stated at the function.
//!
//! # Persistence and transfer posture
//!
//! Caches are minted at construction and are immutable with the body;
//! there is no general invalidation machinery, and what stands in for
//! one is per door: a door declares its posture below, and the doors
//! that change which chart a row is stated in — the three that move a
//! whole loop between charts, the two that move a run of half-edges
//! between two faces' loops, and the setter that re-charts a face in
//! place — dispose of those rows themselves ([`crate::Body::drop_rows`]).
//! Content-keyed cache transfer stays banked (C4). Persistence (M4 PR 6 / D6.1) is
//! **recipe-level**: a document stores its edit list, and loading
//! re-evaluates it — so a round-trip **re-mints** pcurves from the same
//! deterministic pipeline rather than reading stored bytes, and no
//! pcurve-shaped field enters the schema. [`crate::transform`] takes
//! the same posture as it already does for carriers and witnesses:
//! construction-fresh re-derivation against the mapped geometry, never
//! a mapped stored cache.
//!
//! ## Stale rows: which ops maintain this map, and which do not
//!
//! Read this before storing a cache from a new call site. Four
//! postures exist, and they classify **the op a consumer calls**, not
//! every primitive that op uses internally: an op may drive `split_edge`
//! or a bare Euler run in its own interior and still MAINTAIN, so long
//! as it re-mints before it hands the body back.
//!
//! **`mev`, `mef` and `mekr` leave no complete face half-minted.**
//! These operators, with their sugar, add half-edges to an existing
//! loop, and on a face whose rows are COMPLETE they re-mint the loops
//! they rewire with the new halves in them, before they mutate
//! ([`site_rows`], which states the rule and its two edges: a spline
//! chart refuses typed, and a face the closed-form lane cannot mint as
//! the surgery leaves it stores nothing). A face that stores no row is
//! the minting pass's and stays rowless. `mev_null` adds halves too,
//! and cannot mint them: its edge has no carrier, so the loop it joins
//! cannot be walked while the edge is on it. So the site mint takes
//! two more kinds of minted face ([`StoredRows::remints`]): one whose
//! only gaps are on loops a null edge holds open, and one its door
//! takes the last null edge off, whatever it misses. It leaves a loop
//! a null edge still holds open as found — its new halves rowless — and
//! mints whole every loop its door rewires and leaves running through
//! no null edge. The door that releases a loop is either an operator
//! that rewires it out from under the null edge — the boolean's and the
//! splitting lane's join, whose chord `mef`s leave the section's null
//! halves on the sliver between the chords — or the edge's first
//! description ([`crate::Body::set_edge_curve`]), which re-walks every
//! loop of the face; on a spline chart either leaves the face as found.
//! A face half-minted any other way is left as found. Other doors still
//! produce a half-minted face
//! ([`PcurveMintError::MissingCache`] lists them), so a producer's final
//! pass is still what mints the faces the producer BUILT — which its
//! operators leave unminted by design — and re-derives what its other
//! doors staled, dropped or left incomplete.
//!
//! **Maintains the map** — runs [`mint_pcurves`] on the result, and
//! that pass CLEARS the map before re-minting. [`mint_pcurves_of`] is
//! the same posture over a SUBSET, **for a producer that kills no
//! half-edge**: it re-derives the rows of exactly the faces it wrote
//! and leaves the rest as found, so no row it could have staled
//! survives its return. It does NOT discharge the whole-body claim —
//! a row on a dead half-edge is reachable from no face, so it survives
//! that pass over every face of the body (the entry's own docs carry
//! the case, pinned in `sweep`'s SHELL-10 probes). The two simultaneous
//! offset doors are entitled to it because they perform no surgery. In
//! this crate: the
//! splitting lane (on each side it produces), the boolean pipeline (on
//! the finished body), [`crate::Body::merge_coplanar_faces`] (on the
//! staged result before commit, when the input carried rows — as an
//! input at rest does on every face whose chart mints), and
//! [`crate::transform`] (on the same terms), and
//! [`crate::shell`](mod@crate::shell) (on the
//! assembled thin solid). **Downstream crates hold the same posture and
//! are part of the list**: `sweep`'s extrude, revolve and tube, loft,
//! fillet build and fillet surgery, and `step_import`'s assembly all
//! re-mint on the body they return.
//!
//! **Transfers the map** — the graft (`boolean::combine`, and
//! [`crate::graft_disjoint`] through it) remaps each row onto the
//! transplanted half-edge's fresh key and DROPS any row whose key the
//! graft walk did not reach, which is exactly the staleness test.
//! [`crate::Body::split_edge`] holds the same posture at a parameter
//! split: a [`geom_brep::Pcurve`] is a function of the carrier
//! parameter, so each child's chart image IS the parent's restricted
//! to the child's sub-interval. The op re-certifies both restrictions
//! before it mutates ([`split_cache`]) and writes them onto the parent
//! halves and the two new halves, deriving nothing and minting nothing
//! where there was nothing. Two frontiers ride with it, both stated at
//! [`split_cache`]: a `Fitted`/`General` row is left exactly as found
//! (its certification doors take the fitted door,
//! [`crate::AtRestPolicy::fitted_lane`]), and
//! on a SPLINE chart the carry is exact but [`mint_pcurves`] — the
//! recovery step this module's caveats name for a face left rowless —
//! refuses on the split body, because [`nurbs_iso_derive`]'s rim arms
//! map an edge's whole carrier interval onto the chart's whole `u`
//! domain, which a sub-edge no longer spans. So the claim that a
//! carried row is the row the pass would derive is a claim about the
//! ANALYTIC charts, where the pass runs.
//!
//! **[`crate::Body::revert`] carries the map, re-stated with the
//! frames.** Not a fifth posture: the four this section names classify the
//! `&mut Body` doors the guard walks, and `revert` is a `&self ->
//! Self` producer outside that walk (the guard's "does NOT establish"
//! list below), so this is a prose position with nothing checking it.
//! Every row keeps its key, its interval and its certificate: the
//! curved charts' frames do not move under a reversal (those faces
//! flip their `sense` bit), so their rows are the rows; a `Plane`'s
//! frame is reflected (`v_ref` negates with the normal), so the rows
//! on its faces are re-stated under `(u, v) ↦ (u, −v)` through
//! [`geom_brep::PcurveCache::mirrored_v`] — why the certificate is
//! still the certificate is stated once, on
//! [`geom_brep::Pcurve::mirror_v`]. The dead-key exception: a row
//! whose half-edge no longer resolves (the stale-row consequence
//! below) is on no face, so it is on no plane face, and it travels as
//! found — not a refusal, because the boolean's `revert` of a split
//! operand carries such rows routinely, and the graft or the
//! producer's closing mint disposes of them. The loop walk's branch
//! choice is re-stated too, without touching a row: the forward walk
//! parks a periodic chart's one-period wrap at the loop's closure,
//! the joint before `first`, and that joint would sit mid-chain once
//! the loop runs the other way — so `revert` moves every loop's
//! `first` to its source predecessor, which puts the same joint at
//! the reversed closure (`crate::entity::LoopBoundary::Cycle`'s
//! `first` states the invariant; the anchor bullet in `revert`'s
//! module docs carries the argument). Tier 3 of a reverted body whose
//! faces carry rows reports nothing but `NegativeVolume`; `sweep`'s
//! `revert_periodic_wrap` rows pin that on the two-arc sphere's
//! cavity and a cone through its apex.
//!
//! The **loop-re-parenting** doors hold the same posture for a
//! different reason: [`crate::Body::kfmrh`],
//! [`crate::Body::mfkrh`] and [`crate::Body::ring_move`] move a whole
//! LOOP between faces, which changes the CHART every row on that loop
//! is stated in while changing no key. Each carries the moved loop's
//! rows where the two faces are on one CHART
//! ([`crate::Body::same_chart`]) and drops them where they are not
//! ([`crate::Body::drop_rows`]). Where the moved rows do not stand on
//! the destination — dropped, or missing — and the destination was
//! complete, a re-mint is owed, and each door answers it at one of two
//! doors, as the kill family does ([`crate::Body::kev`] and
//! [`crate::Body::kev_describing`]): the keys-only door refuses
//! [`SiteRowRefusal::KeysOnly`] before it mutates, and its `_minting`
//! twin takes a band and runs the site mint over the destination,
//! planned before it mutates ([`crate::Body::plan_moved_rows`]). The
//! moved loop is a rewired loop ([`SiteFace::moved`]), walked in the
//! destination's chart, so a destination whose rows were complete
//! leaves complete, or storing nothing where the closed-form lane
//! cannot mint it. On a spline
//! chart, or a destination that was unminted or half-minted, the drop
//! is the whole answer, and the producer's closing mint re-derives the
//! destination; a body left at rest without it reads loud at tier 3,
//! which re-derives a face that stores no row ([`validate_pcurves`]).
//! Their old `Neither` reading was the one the guard's table could not see: no posture
//! makes a claim about what a row MEANS, and these doors changed
//! nothing else. Two more doors move a RUN of half-edges between two
//! faces' loops rather than a whole loop — [`crate::Body::mef`]'s
//! chord surgery and [`crate::Body::kef`]'s unsplice — and take the
//! same answer over the run; `staleness_posture::DECLARED` carries
//! each door's note.
//!
//! [`crate::Body::set_face_surface`] reaches the same posture from the
//! other side: nothing moves, and the CHART moves under every row the
//! face stores at once. It carries them across a swap onto the same
//! chart and drops them on any other
//! ([`crate::Body::drop_face_rows`]), which is what makes its own
//! declaration below true as written — a swap onto a plane or a
//! placeholder used to leave a COMPLETE row set stated in the chart
//! the face left, and this pass skips exactly that face.
//!
//! **Completes the map** — [`crate::Body::set_edge_curve`], the
//! surface setter's sibling, which is NOT the same case: a carrier swap
//! moves neither the row's key nor its chart, and pass 2 re-derives
//! every row's agreement from the edge's current carrier, so what it
//! stales is refused loud, on a complete face and on the rows a
//! half-minted one stores alike. So it keeps the rows it finds, as a `Neither` door does, with one
//! exception: a null edge's first description is the first door that
//! can derive the rows of its halves, and on a face the site mint
//! selects ([`StoredRows::remints`]) it re-mints every loop no other
//! null edge holds open ([`site_rows`]) — the whole face, once no null
//! edge is left on it.
//!
//! **Neither clears nor re-mints** — the Euler operators that add no
//! half-edge to an existing loop (`mvfs`, `kemr`), the null-edge `mev`
//! (whose scaffolding has no carrier to derive a row from), and the
//! kill ops. These are primitives, and they are what the stale-row
//! consequence below is about. A kill that takes the last null edge
//! off a loop leaves the rows that loop missed while it was held open
//! missing: `kemr`, `kev` and `kef` take no `Tol` to mint with, and
//! [`crate::Body::kef_minting`] runs its site mint only over a remnant
//! whose rows do not stand
//! (`work/topo/a-kill-that-releases-a-loop-from-its-last-null-edge-leaves-its-gaps`).
//!
//! The consequence is bounded but real: a `SecondaryMap` row outlives
//! its key until the slot is reused, so surgery on a body that already
//! carries caches can leave a row attached to a half-edge that no
//! longer means what the cache says (or, once a slot is recycled, to a
//! different half-edge entirely). What bounds it is the backstop, the
//! tier-3 pcurve pass: it reports a face that stores none of its rows
//! or misses some, saying why by re-deriving it, and a stored row that
//! no longer certifies against the current carrier and surface or
//! breaks its loop's branch or winding. It says nothing about the two shapes
//! outside that — a COMPLETE face whose rows were stated in another
//! chart and certify against this one anyway, and a face on a chart
//! [`chart_mints`] refuses, which the pass skips entirely. So the
//! posture is fail-loud where the pass looks, and an op that mutates
//! an already-minted body must either clear the map or re-mint before
//! returning, and must say which.
//!
//! **Where it says which, and what checks it.** For a `&mut Body` door
//! in this crate, in
//! `staleness_posture::every_mutation_door_declares_its_pcurve_posture`
//! — a walk of `topo/src` requiring every such door to either call the
//! pass in its own body, in either of its two spellings
//! ([`mint_pcurves`] whole-body, [`mint_pcurves_of`] over a subset), or
//! carry a declared posture. It goes red the day a door is added and
//! nobody says which bucket it is in, and red the day a door whose
//! entry says it does not re-mint starts calling the pass directly.
//!
//! **What the guard does NOT establish**, so that nothing above reads
//! as more than it is:
//!
//! - **It checks that an entry exists, not that it is true.** A door
//!   declared `Maintains` is taken at its word: no walk can see that a
//!   delegate re-mints, and `mint_pcurves`'s own entry is a claim
//!   about the pass this module defines. Only the
//!   not-`Maintains`-but-minting direction is mechanical.
//! - **Delegation is invisible to it.** The one entry that rotted
//!   historically — an op that started re-minting through a helper
//!   while its bucket entry stayed put — is caught here only if the
//!   helper is itself a `&mut Body` door on this surface, which for
//!   [`crate::Body::merge_coplanar_faces`] it happens to be. A private
//!   delegate would be a green guard over the same rot.
//! - **It reads `topo/src` only.** The pipelines that take a body and
//!   return one rather than taking `&mut Body` (the splitting lane, the
//!   boolean pipeline, [`crate::transform`]) and every downstream crate
//!   are outside the walk; over that remainder the buckets above are a
//!   survey, checked by nothing.

use geom::Surface;
use geom_brep::{
    BranchMiss, Pcurve, PcurveCache, PcurveCertifyError, UncoveredClass, chart_pcurve,
    whole_periods,
};
use geom_core::Tol;
use geom_core::k_stats::decide;
use geom_core::predicate::{Band, BandError};
use geom_core::{Decide, Indeterminate, Margin, Point2, Point3, Real, Sign, SupSpeed};

use crate::body::Body;
use crate::chart_bound::{ChartBound, ChartEdge, ChartLoop};
use crate::entity::{FaceKey, HalfEdgeKey, LoopKey};
use crate::null::CurveGeom;
use crate::props::AtRestPolicy;

/// Typed refusal of the pcurve minting pass (D4 ¶3).
#[derive(Clone, Debug, PartialEq)]
// The variant roster the sample-coverage row reads (test builds only).
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(PcurveMintErrorKind), vis(pub(crate)), derive(strum::EnumIter))
)]
pub enum PcurveMintError {
    /// A key failed to resolve mid-pass — a structurally corrupt body
    /// (tier 1's job to report; this pass only refuses to guess).
    Corrupt,
    /// A half-edge's pcurve failed certification at mint.
    Certify {
        /// The half-edge whose cache was refused.
        half_edge: HalfEdgeKey,
        /// The typed certification failure, nested whole.
        error: PcurveCertifyError,
    },
    /// The chart image of a half-edge does not meet its predecessor's
    /// in the chart: the loop's one-branch unwrap is not continuous
    /// there. Typed, never patched by picking the nearest branch (the
    /// M2 PR 5 finding — module docs).
    LoopDiscontinuity {
        /// The half-edge whose entry point did not meet its
        /// predecessor's exit point.
        half_edge: HalfEdgeKey,
    },
    /// A loop's chart walk did not close: after one full traversal the
    /// azimuth advance is neither zero nor exactly one period, or the
    /// height does not return.
    LoopNotClosed {
        /// The face whose loop failed to close.
        face: FaceKey,
    },
    /// A loop of the face passes through a point where the chart's
    /// FIRST channel has no lever — a sphere pole or a cone apex.
    ///
    /// The walk accepts such a joint (it skips the branch shift there,
    /// deliberately: every azimuth agrees at a pole, so there is no
    /// branch to pick), and that is right for MINTING. It is not right
    /// for a chart POLYGON: the joint's azimuth is whatever the
    /// derivation happened to produce, the chord to it is drawn to a
    /// point the face does not have an azimuth at, and the polygon
    /// that results describes a different region from the face. A
    /// quarter disc revolved 90° about its own edge measures the
    /// difference — the region is a rectangle, the chord polygon a
    /// triangle, and cells in the difference certify outside while
    /// holding material.
    SingularChartJoint {
        /// The face whose loop meets the singularity.
        face: FaceKey,
        /// The loop.
        r#loop: LoopKey,
        /// The half-edge whose ENTRY sits at the singular point.
        half_edge: HalfEdgeKey,
    },
    /// The outer loop's own chart span is definitely wider than the
    /// chart's period, so the face wraps onto itself and its region is
    /// not periodic within its own outer — the premise every ring lift
    /// rests on (`chart_bound::RING_SHIFTS`).
    OuterSpansPeriod,
    /// A loop's chart walk closed only by a whole period of the chart
    /// (or, on a sphere chart, through the involution): the walk is a
    /// LIFT, not a closed chart polygon, so it bounds no chart region
    /// and [`chart_boundary`] refuses rather than describing the
    /// polygon of an open lift.
    ///
    /// **What actually reaches it.** The claim that no head
    /// constructor makes one is false, and was measured false: a
    /// revolve whose profile touches the axis produces a cone face
    /// whose loop runs through the apex, and its walk closes a period
    /// off. That case is now [`PcurveMintError::SingularChartJoint`]'s
    /// — it is refused one check earlier, for the sharper reason —
    /// and what remains here is the genuine period-off closure: a wall
    /// whose loop lifts the chart with no seam chain to close it. No
    /// constructor in the tree is known to build one, and the variant
    /// stays because "known" is not "cannot".
    LoopWraps {
        /// The face whose loop wraps the chart.
        face: FaceKey,
        /// The wrapping loop.
        r#loop: LoopKey,
    },
    /// A half-edge carries no stored pcurve at rest although its face
    /// carries some: the face's cache set is incomplete, a defect. A
    /// face that stores none is [`PcurveMintError::Unminted`], or the
    /// refusal its derivation meets; a planar face stores none by
    /// construction, and a face of an uncovered class
    /// ([`PcurveCertifyError::UnsupportedCarrier`]) none until its
    /// route lands.
    ///
    /// **On the output of `mev`, `mef` and `mekr`, and of the doors
    /// that move a loop or run onto a face (`kfmrh`, `ring_move`,
    /// `mfkrh`, `kef` and their `_minting` twins), this is a kernel-bug
    /// detector** for a face that was complete on an analytic chart:
    /// they leave such a face complete or rowless, or refuse
    /// ([`site_rows`], [`crate::Body::plan_moved_rows`]). The doors that can still
    /// produce this state are:
    ///
    /// - the moving doors onto a complete SPLINE face, where the moved
    ///   loop's rows are dropped or missing and the fitted lane that
    ///   could derive them is not theirs ([`crate::Body::drop_rows`]);
    /// - the moving doors carrying a loop's rows, across one chart,
    ///   onto a face that stored none of its own — or a rowless loop
    ///   onto a half-minted one — which leaves the rows they find;
    /// - [`crate::Body::mev_null`], whose scaffolding edge has no
    ///   carrier to derive a row from. The loop it joins misses the
    ///   edge's two rows, and those of any half-edge an operator adds
    ///   to the loop meanwhile, while a null edge holds it open; the
    ///   door that releases the loop — an operator rewiring it out from
    ///   under the edge, as the pipelines' joins do, or the edge's first
    ///   description ([`crate::Body::set_edge_curve`]) — mints it whole,
    ///   except on a spline chart. A kill that releases it leaves those
    ///   rows missing
    ///   (`work/topo/a-kill-that-releases-a-loop-from-its-last-null-edge-leaves-its-gaps`);
    /// - `split_edge`'s `Fitted`/`General` frontier ([`split_cache`]);
    /// - a caller's own [`crate::Body::detach_pcurve`];
    ///
    /// and a face that arrives half-minted any of these ways, but for a
    /// loop a null edge holds open, stays so through the three
    /// operators, which leave it as found — unless the operator takes
    /// the face's last null edge off it, and then only its kept loops
    /// keep their gaps.
    MissingCache {
        /// The half-edge with no stored cache.
        half_edge: HalfEdgeKey,
    },
    /// A face on a chart that mints stores none of its rows, and the
    /// minting pass derives and certifies every one of them: the body
    /// was not minted after it was built or edited. Rows are mandatory
    /// at rest (C4), so this is a producer's omission, or an assembly
    /// of Euler operations left without its closing mint.
    Unminted {
        /// The face that stores no row.
        face: FaceKey,
    },
    /// A stored row states a carrier interval that is not its edge's:
    /// every row is stated over exactly its edge's certified interval
    /// (the mint copies it, a split restricts it), so this row is about
    /// another edge — a row left from before a split or a carrier swap.
    RowInterval {
        /// The half-edge whose row states the wrong interval.
        half_edge: HalfEdgeKey,
    },
    /// A classification escalated (sliver band or poison).
    Escalated {
        /// The half-edge under classification.
        half_edge: HalfEdgeKey,
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
    /// The run's linear band could not be built.
    Band(BandError),
    /// A half-edge's walked image is a `Fitted` or `General` one, and
    /// its face stores no pcurve rows, so no certificate bounds the
    /// image against its carrier: the description refuses rather than
    /// claim a bound it does not hold. The face is either unminted
    /// (tier 3's [`PcurveMintError::Unminted`]) or one the mint leaves
    /// uncached for a pair no lane covers yet
    /// ([`PcurveCertifyError::UnsupportedCarrier`]). A face that stores
    /// OTHER rows refuses [`PcurveMintError::MissingCache`] instead.
    UncertifiedImage {
        /// The half-edge whose image has no stored certificate.
        half_edge: HalfEdgeKey,
    },
    /// The chart [`chart_boundary`] was handed is the placeholder
    /// ([`Surface::is_placeholder_chart`]): it has no description yet,
    /// so there is no lever arm to meter a joint gap through and no
    /// region of it for a loop to bound. Refused before any loop is
    /// walked, so there is no half-edge to carry a
    /// [`PcurveMintError::Certify`] with
    /// [`PcurveCertifyError::PlaceholderChart`] — the same fact, refused
    /// per half-edge by the certification lanes. Never produced at rest:
    /// the minting pass and the validator skip a placeholder face.
    PlaceholderChart {
        /// The face whose boundary was asked for.
        face: FaceKey,
    },
}

impl core::fmt::Display for PcurveMintError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Corrupt => write!(
                f,
                "the body is structurally corrupt (a key did not resolve), so its faces \
                 cannot be parametrized; read the structural validators' report and repair \
                 the reference it names"
            ),
            Self::Certify { half_edge, error } => write!(
                f,
                "the pcurve of half-edge {half_edge:?} failed certification: {error}"
            ),
            Self::LoopDiscontinuity { half_edge } => write!(
                f,
                "the pcurve of half-edge {half_edge:?} does not meet its predecessor's in \
                 its face's chart. \
                 Recourse: re-mint the body if it was edited after minting; if a fresh mint \
                 refuses too, repair the face's boundary"
            ),
            Self::LoopNotClosed { face } => write!(
                f,
                "the pcurves of a loop of face {face:?} do not close in its chart (the \
                 azimuth advance is neither zero nor one full period). Recourse: re-mint the body after any \
                 surgery; if a fresh mint refuses too, repair the loop"
            ),
            Self::SingularChartJoint {
                face,
                r#loop: lp,
                half_edge,
            } => write!(
                f,
                "loop {lp:?} of face {face:?} meets a sphere pole or cone apex at half-edge \
                 {half_edge:?}, where no face description is built yet; there is nothing in \
                 the body to repair. Recourse: ask for the description of a face whose loops \
                 stay clear of the pole or apex"
            ),
            Self::OuterSpansPeriod => write!(
                f,
                "the outer loop spans more than one period of its chart, so the face wraps \
                 onto itself and has no description. Recourse: describe it as pieces within \
                 one period (a revolve's angle headroom holds this)"
            ),
            Self::LoopWraps { face, r#loop: lp } => write!(
                f,
                "loop {lp:?} of face {face:?} closes one whole period off, so it bounds no \
                 region of its chart; no constructor is known to produce this, so report \
                 the body that reached it"
            ),
            Self::MissingCache { half_edge } => write!(
                f,
                "half-edge {half_edge:?} carries no pcurve although its face's chart mints \
                 them: the body changed after minting. Recourse: re-mint the \
                 body, and report the op that returned it"
            ),
            Self::Unminted { face } => write!(
                f,
                "face {face:?} stores no pcurve although its chart mints them and every one \
                 derives and certifies: the body was not minted after it was built or edited. \
                 Recourse: re-mint the body, and report the op that returned it"
            ),
            Self::RowInterval { half_edge } => write!(
                f,
                "the pcurve of half-edge {half_edge:?} is stated over a carrier interval that \
                 is not its edge's: the body changed after minting. Recourse: re-mint the \
                 body, and report the op that returned it"
            ),
            Self::Escalated { half_edge, cause } => write!(
                f,
                "the pcurve at half-edge {half_edge:?} escalated: {cause}"
            ),
            Self::Band(e) => write!(f, "{e}"),
            Self::UncertifiedImage { half_edge } => write!(
                f,
                "half-edge {half_edge:?} has a fitted or general chart image, and its face \
                 stores no pcurves, so no certificate bounds the image against its carrier \
                 and the face has no description yet. Recourse: mint the body's pcurves and \
                 ask again; a face the mint still leaves uncached is bounded by a carrier \
                 class no lane covers yet, and has no description until one does"
            ),
            Self::PlaceholderChart { face } => write!(
                f,
                "the chart offered for face {face:?}: {}, so nothing on it can be metred. \
                 {PLACEHOLDER_RECOURSE}",
                geom::PLACEHOLDER_SURFACE
            ),
        }
    }
}

impl std::error::Error for PcurveMintError {}

/// The recourse for a placeholder chart, whichever enum refuses it.
pub(crate) const PLACEHOLDER_RECOURSE: &str =
    "Recourse: describe the surface first, then ask again";

/// A chart that is not the placeholder
/// ([`Surface::is_placeholder_chart`]) — the only kind of chart the
/// loop walk and its meters ([`chart_u_arm`], [`v_meter`]) accept.
///
/// The placeholder's control net is all-poison: it has no locus, so
/// `geom_brep::chart_stretch_sup` refuses it rather than answer a lever
/// arm, and a gap metred through it would be a verdict about nothing.
/// Every door into the walk takes the refusal here, once, where it can
/// still say what to do about it: the minting pass and the validator
/// skip the face (a placeholder mints nothing), and [`chart_boundary`]
/// refuses with [`PcurveMintError::PlaceholderChart`].
#[derive(Clone, Copy)]
pub(crate) struct DescribedChart<'a, T: Real>(&'a Surface<T>);

impl<'a, T: Real> DescribedChart<'a, T> {
    /// `None` for exactly the surface `geom_brep::chart_stretch_sup`
    /// refuses as [`geom_brep::NoChartSup::Placeholder`]: both read
    /// [`Surface::is_placeholder_chart`].
    pub(crate) fn of(surface: &'a Surface<T>) -> Option<Self> {
        (!surface.is_placeholder_chart()).then_some(Self(surface))
    }

    /// The chart of a face the minting pass writes rows for: described,
    /// and of a kind that mints ([`chart_mints`]). `None` is "this face
    /// carries no rows by construction" — the placeholder and the plane.
    pub(crate) fn minting(surface: &'a Surface<T>) -> Option<Self> {
        Self::of(surface).filter(|c| chart_mints(*c))
    }

    pub(crate) fn surface(self) -> &'a Surface<T> {
        self.0
    }
}

/// Does this chart kind mint stored caches (module docs)? A
/// compile-time routing decision per surface kind, exhaustively
/// matched — adding a kind is a compiler-guided edit (D3).
fn chart_mints<T: Real>(chart: DescribedChart<'_, T>) -> bool {
    match chart.surface() {
        Surface::Cylinder { .. } => true,
        // Planar faces keep M2's derive-on-demand status (C4 verbatim).
        Surface::Plane { .. } => false,
        // The analytic-chart completion (M6-3, walk row 4): cone,
        // sphere and torus charts certify their closed-form classes
        // (rim/ruling; polar/meridian; parallel/meridian) and mint
        // stored caches wherever the pass runs.
        Surface::Cone { .. } | Surface::Sphere { .. } | Surface::Torus { .. } => true,
        // Described NURBS charts mint (M6-3; RATIONAL charts since
        // M8-3): every loft/sweep wall boundary is an iso-parameter
        // curve of the wall it bounds. Seams and LINE cap rims have
        // exact line images (`Pcurve::IsoLine`); an ARC cap rim on a
        // rational wall has an exact image too — the same boundary
        // line, on the chart's own rational-quadratic parameter
        // (`Pcurve::IsoArc`). The placeholder is not a described chart
        // and never reaches here; it mints nothing. An approximating
        // surface's chart is its fit's, described by construction.
        Surface::Nurbs(_) | Surface::Approx(_) => true,
    }
}

/// The pcurve of `half_edge` on its own face's chart — the **stored
/// cache** when there is one, otherwise **derived on demand**.
///
/// The derived answer is on the chart's *principal* branch (there is no
/// loop context in a single-half-edge query); a caller that needs a
/// face-consistent branch reads the stored cache or walks the loop
/// through [`mint_pcurves`]. For the affine (plane) charts that are the
/// standing derive-on-demand case this distinction is vacuous — a plane
/// chart has no branches.
///
/// # Errors
///
/// [`PcurveMintError`] — a corrupt key, or a typed chart refusal for a
/// frontier chart/carrier kind.
pub fn pcurve_of<T: AtRestPolicy>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
    band: Band,
) -> Result<Pcurve<T>, PcurveMintError> {
    if let Some(cache) = body.pcurve(half_edge) {
        return Ok(cache.pcurve().clone());
    }
    let (carrier, t0, t1) = half_edge_carrier(body, half_edge)?;
    let surface = half_edge_surface(body, half_edge)?;
    // A SPLINE chart's images are description-driven (M6-3) — the iso
    // derivation, not the closed-form harmonic table. An approximating
    // surface's chart IS its fit's, so it takes the same route.
    if surface.spline_chart().is_some() {
        return nurbs_iso_derive(body, half_edge, &surface, band);
    }
    analytic_derive(&carrier, t0, t1, &surface, band, half_edge)
}

/// The chart image of a carrier on an ANALYTIC chart, on the chart's
/// principal branch: the closed form ([`chart_pcurve`]) wherever one
/// exists, and for a sphere's GENERAL circle — the class the closed-form
/// door names `UncoveredClass::SphereGeneralCircle`, its incidence with
/// the chart already decided — the fitted image
/// ([`geom_brep::FittedLane::sphere_circle_image`]), certified by
/// [`PcurveCache::certify_fitted`]'s Circle arm.
///
/// # Errors
///
/// [`PcurveMintError::Certify`] with the closed-form door's refusal,
/// the fitted image's own refusal (an arc through a pole of the chart),
/// or [`PcurveCertifyError::FittedLaneUnsupported`] for a general
/// circle at a scalar with no fitted door.
fn analytic_derive<T: AtRestPolicy>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    surface: &Surface<T>,
    band: Band,
    half_edge: HalfEdgeKey,
) -> Result<Pcurve<T>, PcurveMintError> {
    let certify = |error| PcurveMintError::Certify { half_edge, error };
    match chart_pcurve(carrier, surface, band) {
        Err(PcurveCertifyError::UnsupportedCarrier {
            class: UncoveredClass::SphereGeneralCircle,
            ..
        }) => {
            let Some(lane) = T::fitted_lane() else {
                return Err(certify(PcurveCertifyError::FittedLaneUnsupported {
                    scalar: T::NAME,
                }));
            };
            lane.sphere_circle_image(carrier, t0, t1, surface, band)
                .map(|image| Pcurve::Fitted(std::sync::Arc::new(image)))
                .map_err(certify)
        }
        image => image.map_err(certify),
    }
}

/// **Whether a stored row states its edge's whole interval** — copied
/// from the edge at the mint and at a split; one stated over more or
/// less of the carrier is about another edge
/// ([`PcurveMintError::RowInterval`]). Measured where the edge's own
/// endpoint pinning is (certification's check 3): the carrier at the
/// row's ends against the edge's vertices, in metres. A row that copied
/// its edge's interval evaluates exactly what that check did, so it
/// reads that check's verdict; a parameter difference instead would be
/// radians against a band in metres, and at an interval scalar would
/// carry the copy's own width twice. Read by tier 3 on every stored row
/// ([`validate_pcurves`]) and by a producer's closing mint on every row
/// it carries ([`carry_rows`]).
fn row_interval<T: Decide>(
    body: &Body<T>,
    he: HalfEdgeKey,
    row: &PcurveCache<T>,
    carrier: &geom::Curve3<T>,
    band: Band,
) -> Result<(), PcurveMintError> {
    let (r0, r1) = row.params();
    let (start, end) = edge_vertex_points(body, he)?;
    match [(r0, start), (r1, end)]
        .into_iter()
        .map(|(r, p)| {
            decide(
                "pcurve_row_interval",
                Margin::of(carrier.eval(r).distance(p)),
                band,
            )
        })
        .find(|v| !matches!(v, Ok(Sign::Zero)))
    {
        None => Ok(()),
        Some(Ok(_)) => Err(PcurveMintError::RowInterval { half_edge: he }),
        Some(Err(cause)) => Err(PcurveMintError::Escalated {
            half_edge: he,
            cause,
        }),
    }
}

/// The points of `half_edge`'s edge's vertices, in its `he_plus`
/// order: where the carrier's certified interval starts and ends.
fn edge_vertex_points<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<(geom_core::Point3<T>, geom_core::Point3<T>), PcurveMintError> {
    let he = body
        .get_half_edge(half_edge)
        .ok_or(PcurveMintError::Corrupt)?;
    let edge = body.get_edge(he.edge).ok_or(PcurveMintError::Corrupt)?;
    let plus = body
        .get_half_edge(edge.he_plus)
        .ok_or(PcurveMintError::Corrupt)?;
    let point = |v| {
        body.get_vertex(v)
            .and_then(|v| body.get_point(v.point))
            .copied()
            .ok_or(PcurveMintError::Corrupt)
    };
    let end = body
        .get_half_edge(edge.he_minus)
        .ok_or(PcurveMintError::Corrupt)?
        .start;
    Ok((point(plus.start)?, point(end)?))
}

/// The point of the vertex `half_edge` starts at: the joint where the
/// loop enters it.
fn entry_vertex<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<geom_core::Point3<T>, PcurveMintError> {
    let he = body
        .get_half_edge(half_edge)
        .ok_or(PcurveMintError::Corrupt)?;
    body.get_vertex(he.start)
        .and_then(|v| body.get_point(v.point))
        .copied()
        .ok_or(PcurveMintError::Corrupt)
}

/// The certified carrier and parameter interval of `half_edge`'s edge.
fn half_edge_carrier<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<(geom::Curve3<T>, T, T), PcurveMintError> {
    let he = body
        .get_half_edge(half_edge)
        .ok_or(PcurveMintError::Corrupt)?;
    let edge = body.get_edge(he.edge).ok_or(PcurveMintError::Corrupt)?;
    let Some(CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
        return Err(PcurveMintError::Corrupt);
    };
    let (t0, t1) = curve.params();
    Ok((curve.carrier().clone(), t0, t1))
}

/// The **mate operand** a fitted (rung-3) pcurve's certificate needs:
/// the other surface of the pair whose intersection minted the carrier
/// (`geom_brep::PcurveCache::certify_fitted` — the uniqueness tube is a
/// statement about the PAIR, so one surface cannot produce one).
///
/// It is read from the edge's **intensional description**, not from the
/// topology, and that is the D2 answer rather than a convenience: the
/// description is what is authoritative about which two surfaces the
/// locus belongs to (`EdgeDescription::Intersection { s1, s2 }` names them
/// by key), while "the face across the edge" is a derived fact that a
/// mid-construction body, a spur edge or a seam can perfectly well have
/// wrong. Re-read from the body at rest, never stored with the cache,
/// so it cannot drift from the body's own geometry.
///
/// `None` when the edge is not an intersection edge, or when the face's
/// own surface is neither of the pair — both of which the fitted lane
/// then refuses typed rather than inventing a second operand.
fn mate_surface<T: Decide>(body: &Body<T>, half_edge: HalfEdgeKey) -> Option<Surface<T>> {
    let he = body.get_half_edge(half_edge)?;
    let edge = body.get_edge(he.edge)?;
    let CurveGeom::Certified(curve) = body.get_curve_geom(edge.curve)? else {
        return None;
    };
    let geom_brep::EdgeDescription::Intersection { s1, s2, .. } = *curve.description() else {
        return None;
    };
    let own = body.get_face(body.face_of_half_edge(half_edge)?)?.surface;
    let other = if own == s1 {
        s2
    } else if own == s2 {
        s1
    } else {
        return None;
    };
    body.get_surface(other).cloned()
}

/// The surface of the face `half_edge` bounds.
fn half_edge_surface<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<Surface<T>, PcurveMintError> {
    body.get_surface(half_edge_surface_key(body, half_edge)?)
        .cloned()
        .ok_or(PcurveMintError::Corrupt)
}

/// The surface KEY of the face `half_edge` bounds (the key twin of
/// [`half_edge_surface`], for the iso derivation's own-side test).
fn half_edge_surface_key<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<geom_brep::SurfaceKey, PcurveMintError> {
    let face = body
        .face_of_half_edge(half_edge)
        .ok_or(PcurveMintError::Corrupt)?;
    Ok(body.get_face(face).ok_or(PcurveMintError::Corrupt)?.surface)
}

/// The certified description of `half_edge`'s edge.
fn half_edge_description<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<geom_brep::EdgeDescription<T>, PcurveMintError> {
    let he = body
        .get_half_edge(half_edge)
        .ok_or(PcurveMintError::Corrupt)?;
    let edge = body.get_edge(he.edge).ok_or(PcurveMintError::Corrupt)?;
    let Some(CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
        return Err(PcurveMintError::Corrupt);
    };
    Ok(curve.description().clone())
}

/// The uniform clamped degree-1 knot vector on `[0, 1]` with `spans`
/// spans — an [`Pcurve::IsoArc`]'s sub-arc locator (pure `f64`
/// structure, which is what keeps the variant `T`-generic).
fn uniform_breaks(spans: usize) -> Option<geom_core::spline::KnotVector> {
    if spans == 0 {
        return None;
    }
    let mut knots = vec![0.0, 0.0];
    #[allow(clippy::cast_precision_loss)]
    for k in 1..spans {
        knots.push(k as f64 / spans as f64);
    }
    knots.extend([1.0, 1.0]);
    geom_core::spline::KnotVector::clamped(knots, 1).ok()
}

/// Derives the **exact iso chart image** of `half_edge` on a described
/// NURBS chart (M6-3; arc rims since M8-3) — the NURBS-chart
/// counterpart of `geom_brep::chart_pcurve`, driven by the edge's
/// INTENSIONAL description (D2: the description is what is
/// authoritative about which iso this locus is):
///
/// - A chart image naming THIS face's surface IS the answer: since
///   the conventional descriptions collapsed (U2) there is nothing
///   left to derive.
/// - An iso LINE image naming the OTHER wall maps as this chart's own
///   `u = const` column: a domain end when a definite endpoint residual
///   (`pcurve_iso_side`) places the carrier's start there, otherwise
///   the column its certified chart foot measures (a chart wider than
///   the face it trims), and then CERTIFIED by the full iso lane — a
///   wrong pick fails loudly, never silently.
/// - A cap–wall rim over a LINE carrier maps as `(u(t), v)` with `u` affine
///   (`t0 ↦ u₀`, `t1 ↦ u₁` — the wall's u IS the segment parameter by
///   construction, up to the chart's own affine scale) and
///   `v ∈ {v₀, v₁}` by the same endpoint selection.
///
/// **The chart's own domain, not the unit square (#327).** Every
/// boundary above is the payload's KNOT domain end. A chart the kernel
/// BUILT is normalized to `[0, 1]²`, so this reads the same values it
/// always did; a chart the kernel IMPORTED carries the file's
/// parameterization — dm1's cylinder wall is `u ∈ [0, 3√3]` — where
/// `u = 1` is an interior column and every pick against it silently
/// answers about the wrong locus.
/// - A cap–wall rim over a CIRCLE carrier maps to the same boundary
///   line through the chart's own rational-quadratic parameter
///   ([`Pcurve::IsoArc`], M8-3). Both mapped description forms the
///   kernel mints for a circle reach it — see the arm's own note.
/// - An [`geom_brep::EdgeDescription::Intersection`] over a SPLINE carrier
///   that lies on a boundary column maps as that column: the same iso
///   line the `IsoCurve` arm mints, recovered from the carrier because
///   the intrinsic description names no chart coordinate. The residency
///   is a pick over the columns and the two `v` directions, definite or
///   escalated. A locus that is NOT a boundary column — an INTERIOR
///   column is the executed case (#498) — has no exact closed form and
///   takes U2's `General` curve-in-UV arm at the honest Fitted grade,
///   derived from the wall's own foot schedule.
/// - Everything else on a NURBS chart refuses typed with the class
///   named.
fn nurbs_iso_derive<T: AtRestPolicy>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
    surface: &Surface<T>,
    band: Band,
) -> Result<Pcurve<T>, PcurveMintError> {
    use geom_core::{Point2, Vec2};
    let refuse = |what: &'static str| PcurveMintError::Certify {
        half_edge,
        error: PcurveCertifyError::IsoUnsupported { what },
    };
    let (carrier, t0, t1) = half_edge_carrier(body, half_edge)?;
    let span = t1 - t0;
    // The chart's OWN domain (doc above): `[0, 1]²` for a kernel-built
    // patch, the file's parameterization for an imported one.
    // The catch-all is SPLIT: an approximating surface's domain is its
    // FIT's knot domain, not the unit square — reading `[0,1]²` off a
    // chart parameterized otherwise would place every derived image on
    // the wrong rectangle.
    let (cu0, cu1, cv0, cv1) = match surface.spline_chart() {
        Some(payload) => {
            let (a, b) = payload.knots_u().domain();
            let (c, d) = payload.knots_v().domain();
            (
                T::from_f64(a),
                T::from_f64(b),
                T::from_f64(c),
                T::from_f64(d),
            )
        }
        None => (T::zero(), T::one(), T::zero(), T::one()),
    };
    // A definite endpoint-side selection: which of the two candidate
    // chart values places the carrier's START on the surface. The
    // selection is structure (a two-way pick), the CHECK is the full
    // iso-lane certification that follows every derivation.
    //
    // `None` is "no candidate is definitely it", NOT a refusal: a
    // caller may have a wider candidate to offer (the rim arms measure
    // one when the chart is wider than the face it trims). The refusal
    // text lives at the call sites that have nothing left to try, so
    // three arms can no longer share one message for three different
    // situations.
    let side_pick = |eval_at: &dyn Fn(T) -> geom_core::Point3<T>,
                     cands: &[T]|
     -> Result<Option<T>, PcurveMintError> {
        let start = carrier.eval(t0);
        for cand in cands {
            match decide(
                "pcurve_iso_side",
                Margin::of(start.distance(eval_at(*cand))),
                band,
            ) {
                Ok(Sign::Zero) => return Ok(Some(*cand)),
                Ok(Sign::Positive | Sign::Negative) => {}
                Err(cause) => {
                    return Err(PcurveMintError::Escalated { half_edge, cause });
                }
            }
        }
        Ok(None)
    };
    let no_boundary = || {
        refuse(
            "the carrier's start point lies on neither chart boundary — not a boundary \
             iso of this face's chart",
        )
    };
    let own = half_edge_surface_key(body, half_edge)?;
    match half_edge_description(body, half_edge)? {
        // **This face's OWN chart image is the answer.** Since the
        // conventional descriptions collapsed (U2), an edge described
        // as an image in THIS chart carries the image itself — there
        // is nothing left to derive, and re-deriving it would be a
        // second opinion about a locus the description already states.
        geom_brep::EdgeDescription::Chart(ref c) if c.surface == own => Ok(c.pcurve.clone()),
        // **The wall–wall seam stated on the OTHER wall.** An iso
        // LINE image (`u` fixed, `v` moving) on the neighbour's chart
        // maps as this chart's own `u = u₀`/`u = u₁` boundary, the
        // side selected by the endpoint. The moving channel is the
        // description's own, verbatim: the two walls share the seam's
        // parameterization, which is what makes them one seam.
        geom_brep::EdgeDescription::Chart(geom_brep::ChartCurve {
            pcurve: Pcurve::IsoLine { p0, pl },
            ..
        }) => {
            let v0 = p0.y + pl.y * t0;
            let column = |cand: T| surface.eval(cand, v0);
            // The image's SHAPE is the neighbour's own description (a
            // column, `v` moving as the neighbour says); only its
            // POSITION is derived here. A domain end wins where one is
            // definitely it; on a chart wider than the face it trims
            // the seam is an interior column, and its position is the
            // carrier start's certified chart foot — one sample,
            // offered to the same metre-valued check — with no snap to
            // a knot: the certifier collapses the chart at exactly the
            // value it is handed, and any error in that value is what
            // its hull meters.
            let x = match side_pick(&column, &[cu0, cu1])? {
                Some(x) => x,
                None => {
                    let Some(foot) = derive_chart_foot(carrier.eval(t0), surface, half_edge)?
                    else {
                        return Err(refuse(
                            "the carrier's start point lies on neither chart boundary, and \
                             no lane measures a chart foot at this scalar, so the interior \
                             seam column it sits on cannot be positioned",
                        ));
                    };
                    side_pick(&column, &[T::from_f64(foot.x)])?.ok_or_else(|| {
                        refuse(
                            "the carrier's start has a certified chart foot, but the \
                             neighbour's v map does not place it there — the two walls do \
                             not share the seam's parameterization",
                        )
                    })?
                }
            };
            Ok(Pcurve::IsoLine {
                p0: Point2::new(x, p0.y),
                pl: Vec2::new(T::zero(), pl.y),
            })
        }
        // **The M8-3 ARC-RIM arm.** An ARC cap rim's chart image is
        // the same boundary line the LINE arm mints, but its moving
        // channel is the chart's own rational-quadratic parameter
        // rather than the arc angle — the `Pcurve::IsoArc` map.
        //
        // The arm is keyed on the CARRIER, not on the description
        // form, because the carrier is what the certification reads:
        // a circle rim is a circle rim however its own chart writes
        // it down. Keying on the form would have made the SAME
        // geometry mint natively and refuse on the round trip — a
        // description-form accident, not a fact about the rim.
        //
        // The sub-arc count is read off the chart's u structure (one
        // span per sub-arc, by the loft's construction); that read is
        // a SELECTION, and the CHECK is the full arc-rim certification
        // that follows, which compares the chart's boundary column
        // against the carrier circle's own rational-quadratic form and
        // refuses a chart that is not this construction.
        geom_brep::EdgeDescription::Chart(_) | geom_brep::EdgeDescription::Scaffold(_)
            if matches!(carrier, geom::Curve3::Circle { .. }) =>
        {
            let Some(payload) = surface.spline_chart() else {
                return Err(refuse("an arc cap rim on a non-spline chart"));
            };
            let ku = payload.knots_u();
            let (d0, d1) = ku.domain();
            let spans = ku.knots().iter().filter(|k| **k > d0 && **k < d1).count() / 2 + 1;
            let breaks = uniform_breaks(spans)
                .ok_or_else(|| refuse("an arc rim whose chart has no usable sub-arc structure"))?;
            let v =
                side_pick(&|cand| surface.eval(cu0, cand), &[cv0, cv1])?.ok_or_else(no_boundary)?;
            // **The u-DIRECTION pick (#327).** M8-3's arm assumed the
            // rim's increasing carrier parameter runs with the chart's
            // increasing `u` — true by construction for a wall the
            // kernel BUILT, and false in general for one it IMPORTED:
            // a promoted rim circle's `axis` is derived from the
            // file's own NURBS winding, and a cylinder wall's two rims
            // routinely wind oppositely in the file, so exactly one of
            // them runs against the chart. Assuming `+u` there does not
            // refuse — it mints a chart image that traverses the wall
            // BACKWARDS, and the loop walk reports it as a chart
            // discontinuity or a double-period closure, naming a
            // symptom two steps from the cause.
            //
            // The pick is the `side_pick` idiom on the other axis: the
            // two candidate images (`u: 0 → 1` and `u: 1 → 0`),
            // evaluated at a fixed interior probe of the arc and
            // metered against the carrier there. The start point
            // cannot decide it — on a FULL-PERIOD rim the chart's two
            // u-boundaries are the same 3-D point — so the probe sits
            // at a quarter of the arc, where the two candidates are a
            // half-period apart. A selection, checked: the full
            // arc-rim certification still follows.
            let probe_t = t0 + span * T::from_f64(0.25);
            let probe = carrier.eval(probe_t);
            let image = |p0x: T, sign: T| Pcurve::IsoArc {
                p0: Point2::new(p0x, v),
                pd: Vec2::new(sign, T::zero()),
                t0,
                angle: span,
                breaks: breaks.clone(),
            };
            // Escalations are DEFERRED per candidate (the loop walk's
            // posture): an indeterminate first candidate must not rob
            // the second of its turn.
            let mut deferred: Option<Indeterminate> = None;
            for (p0x, sign) in [(cu0, cu1 - cu0), (cu1, cu0 - cu1)] {
                let cand = image(p0x, sign);
                let uv = cand.eval(probe_t);
                let gap = probe.distance(surface.eval(uv.x, uv.y));
                match decide("pcurve_iso_arc_direction", Margin::of(gap), band) {
                    Ok(Sign::Zero) => return Ok(cand),
                    Ok(Sign::Positive | Sign::Negative) => {}
                    Err(cause) => {
                        if deferred.is_none() {
                            deferred = Some(cause);
                        }
                    }
                }
            }
            match deferred {
                Some(cause) => Err(PcurveMintError::Escalated { half_edge, cause }),
                None => Err(refuse(
                    "an arc rim whose chart image runs in neither u direction — the \
                     rim does not lie on this chart's boundary column",
                )),
            }
        }
        geom_brep::EdgeDescription::Chart(_) | geom_brep::EdgeDescription::Scaffold(_)
            if matches!(carrier, geom::Curve3::Line { .. }) =>
        {
            // The closed form: the rim spans the chart's whole `u`
            // domain, which is true for every wall the kernel BUILT.
            let plx = (cu1 - cu0) / span;
            let p0x = cu0 - (cu1 - cu0) * t0 / span;
            let row = |p0x: T, plx: T| move |cand: T| surface.eval(p0x + plx * t0, cand);
            match side_pick(&row(p0x, plx), &[cv0, cv1])? {
                Some(v) => Ok(Pcurve::IsoLine {
                    p0: Point2::new(p0x, v),
                    pl: Vec2::new(plx, T::zero()),
                }),
                // **The rim does not span this chart.** Same cause as
                // the wall-seam arm above: a chart wider than the face
                // it trims. The `u` MAP is what the closed form got
                // wrong (the `v` side is still a boundary), so measure
                // the rim's two endpoints and build the map from them.
                // Same certified foot producer, same metre-valued check
                // on the `v` side, and the full iso certification still
                // follows every derivation.
                None => {
                    let (f0, f1) = (
                        derive_chart_foot(carrier.eval(t0), surface, half_edge)?,
                        derive_chart_foot(carrier.eval(t1), surface, half_edge)?,
                    );
                    let (Some(f0), Some(f1)) = (f0, f1) else {
                        return Err(no_boundary());
                    };
                    let (u0, u1) = (T::from_f64(f0.x), T::from_f64(f1.x));
                    let plx = (u1 - u0) / span;
                    let p0x = u0 - plx * t0;
                    let v = side_pick(&row(p0x, plx), &[cv0, cv1])?.ok_or_else(no_boundary)?;
                    Ok(Pcurve::IsoLine {
                        p0: Point2::new(p0x, v),
                        pl: Vec2::new(plx, T::zero()),
                    })
                }
            }
        }
        // **The boundary-iso INTERSECTION arm.** The same wall–wall
        // seam the `IsoCurve` arm maps, stated INTRINSICALLY: the locus
        // is `S₁ ∩ S₂`, and where that locus IS this chart's own
        // `u = u₀`/`u = u₁` boundary column its chart image is the same
        // iso line. The description carries no chart coordinates at all
        // — it names two surfaces and a witness — so the image is
        // recovered from the CARRIER against the chart's own domain.
        //
        // Keyed on the carrier and on BOUNDARY RESIDENCY, never on the
        // description form or the operand order: which of `s1`/`s2` is
        // the plane is not a fact about the locus, and the same seam
        // stated natively or restated foreign must mint the same image.
        //
        // The pick is ONE fixed schedule (D9): the chart's two boundary
        // columns × the two directions the carrier can traverse the
        // chart's `v`, each candidate image evaluated at an interior
        // probe and metered against the carrier there in METRES. The
        // start point cannot decide it alone — it fixes a chart CORNER,
        // and a column and a direction both pass through one — so the
        // probe sits a quarter span in, where the candidates separate.
        // A SELECTION, checked: the full seam-class certification
        // follows every derivation.
        //
        // **What the schedule selects is wider than what the seam class
        // CERTIFIES, and deliberately so.** The certified inventory is
        // FORWARD only: the class's parameter-map slack is metered
        // against the identity map and its control hull against the
        // boundary row's own ordering, so a carrier that traverses the
        // chart's `v` backwards refuses at certification (the envelope)
        // rather than charting. The backward candidates stay in the
        // schedule because the honest answer for such a carrier is the
        // image it actually has, judged by the certification — not a
        // forward image that fits the class by construction and states
        // the wrong traversal.
        //
        // An `Intersection` whose carrier traverses NEITHER boundary
        // column under the chart's own parameterization refuses typed
        // and permanently (C5).
        geom_brep::EdgeDescription::Intersection { .. } => {
            let geom::Curve3::Nurbs(spline) = &carrier else {
                return Err(refuse(
                    "an Intersection carrier that is not a spline — the certified \
                     boundary-column class compares the carrier against the chart's own \
                     boundary ROW, which is a spline",
                ));
            };
            let Some(wall) = surface.spline_chart() else {
                unreachable!(
                    "nurbs_iso_derive: both callers route only a chart with a spline payload \
                     here"
                )
            };
            let probe_t = t0 + span * T::from_f64(0.25);
            let probe = carrier.eval(probe_t);
            // Escalations are DEFERRED per candidate: an indeterminate
            // first candidate must not rob the rest of their turn.
            let mut deferred: Option<Indeterminate> = None;
            for x in [cu0, cu1] {
                for (v_at_t0, v_at_t1) in [(cv0, cv1), (cv1, cv0)] {
                    let slope = (v_at_t1 - v_at_t0) / span;
                    let cand = Pcurve::IsoLine {
                        p0: Point2::new(x, v_at_t0 - slope * t0),
                        pl: Vec2::new(T::zero(), slope),
                    };
                    let uv = cand.eval(probe_t);
                    let gap = probe.distance(surface.eval(uv.x, uv.y));
                    match decide("pcurve_iso_seam_column", Margin::of(gap), band) {
                        Ok(Sign::Zero) => return Ok(cand),
                        Ok(Sign::Positive | Sign::Negative) => {}
                        Err(cause) => {
                            if deferred.is_none() {
                                deferred = Some(cause);
                            }
                        }
                    }
                }
            }
            // ---- The fixed schedule found nothing. Derive the image. ----
            // The four candidates above assume the carrier traverses
            // the chart's WHOLE v domain, because that is what a
            // natively built wall's seam does. The foot schedule
            // measures the image instead of assuming it, and what it
            // measures decides the class (P-2):
            //
            // * offered back as the two boundary columns with the v map
            //   the image MEASURED, which is what a partial or affinely
            //   reparameterized restatement of a column looks like —
            //   still judged by the same metre-valued probe, so the
            //   exact class is preferred wherever it applies and no UV
            //   quantity reaches ε (D4 ¶1);
            // * otherwise stored as it is, a `General` curve in UV at
            //   the honest Fitted grade (U2), which is where an
            //   INTERIOR column lands: it has no boundary-row closed
            //   form and never will, and `General` is #498's home for
            //   it rather than a refusal.
            //
            // A DIAGONAL locus reaches here too and is not this unit's:
            // it refuses earlier, at edge certification, on
            // `PXN_IMAGE_DEGREE` (`geom-brep/src/edge_nurbs.rs`, banked
            // to #264), so no body carrying one reaches this pass.
            let image = match derive_general_image(spline, wall, half_edge) {
                Ok(image) => image,
                // An escalated candidate still outranks a derivation
                // refusal: a row that escalates today keeps escalating,
                // and the new path only ever speaks on a DEFINITE
                // fall-through.
                Err(e) => match deferred {
                    Some(cause) => return Err(PcurveMintError::Escalated { half_edge, cause }),
                    None => return Err(e),
                },
            };
            // **No re-offer of the exact class here, and the spec was
            // wrong to ask for one.** Item 7 read "a partial or
            // reparameterized restatement of a column" off the refusal
            // payload and inferred the exact class applies to it. It
            // does not: the seam class's hull limb compares the image
            // against the chart's own boundary ROW, and that comparison
            // needs ONE spline space — a partial column is not a
            // control-net copy of the boundary row and cannot be made
            // into one. Offering it here would hand the certifier an
            // image it must structurally refuse, and the mint would
            // then fail with text about a boundary row for a locus that
            // is not one. That is the same defect this arm's sibling
            // (the wall–wall seam arm) was reverted for.
            //
            // So the fixed schedule above IS the exact class's whole
            // reach — a boundary column traversed end to end, in either
            // direction — and everything it does not claim goes to
            // `General`, which certifies against the operand pair and
            // has no boundary-row hypothesis to violate. `General` is
            // not a downgrade for these loci; it is the only grade that
            // can state anything true about them.
            match deferred {
                Some(cause) => Err(PcurveMintError::Escalated { half_edge, cause }),
                None => Ok(Pcurve::General(std::sync::Arc::new(image))),
            }
        }
        _ => Err(refuse(
            "no iso derivation for this locus on a NURBS chart — only chart images of \
             this chart, iso-line images of the neighbouring wall, boundary-iso \
             Intersection seams, and LINE or CIRCLE cap rims have exact chart images \
             (the trimmed-NURBS pcurve lane is the cut-loft unit's)",
        )),
    }
}

/// The **general curve-in-UV image** of a spline carrier on this face's
/// own spline chart — U2's `General` arm, at the honest Fitted grade.
///
/// The producer is `geom_brep`'s one derivation of this object
/// ([`geom_brep::FittedLane::general_image`], whose body is the same
/// `edge_nurbs` foot schedule the plane × NURBS edge certificate uses
/// at adopt time). Nothing is certified here: this returns EVIDENCE,
/// and the mint's next move is `PcurveCache::certify_general`, which
/// bounds `sup_t |S(P(t)) − C(t)|` over the whole span against the
/// operand pair the edge's own description names.
///
/// # Errors
///
/// [`PcurveMintError::Certify`] carrying
/// [`PcurveCertifyError::FittedLaneUnsupported`] at a scalar with no
/// certified lane (a dual body may not certify — D1), or the
/// derivation's own typed refusal.
fn derive_general_image<T: AtRestPolicy>(
    spline: &geom::NurbsCurve3<T>,
    wall: &geom::NurbsSurface<T>,
    half_edge: HalfEdgeKey,
) -> Result<geom::NurbsCurve2<T>, PcurveMintError> {
    let certify = |error| PcurveMintError::Certify { half_edge, error };
    let Some(lane) = T::fitted_lane() else {
        return Err(certify(PcurveCertifyError::FittedLaneUnsupported {
            scalar: T::NAME,
        }));
    };
    lane.general_image(spline, wall).map_err(certify)
}

/// The **certified chart foot** of one model-space point on this face's
/// own spline chart — [`derive_general_image`]'s single-sample sibling,
/// same producer ([`geom_brep::FittedLane::chart_foot`]).
///
/// `None` at a scalar with no certified lane, which lets a caller keep
/// its previous typed refusal rather than inventing a foot: a dual body
/// answers exactly what it answered before this widening existed.
///
/// # Errors
///
/// [`PcurveMintError::Certify`] when the projection will not converge.
fn derive_chart_foot<T: AtRestPolicy>(
    point: geom_core::Point3<T>,
    surface: &Surface<T>,
    half_edge: HalfEdgeKey,
) -> Result<Option<geom_core::Point2<f64>>, PcurveMintError> {
    let (Some(wall), Some(lane)) = (surface.spline_chart(), T::fitted_lane()) else {
        return Ok(None);
    };
    lane.chart_foot(point, wall)
        .map(Some)
        .map_err(|error| PcurveMintError::Certify { half_edge, error })
}

/// Is `half_edge` the `he_plus` of its edge (so the loop traverses it
/// forward in the carrier parameter)?
pub(crate) fn is_plus<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<bool, PcurveMintError> {
    let he = body
        .get_half_edge(half_edge)
        .ok_or(PcurveMintError::Corrupt)?;
    let edge = body.get_edge(he.edge).ok_or(PcurveMintError::Corrupt)?;
    Ok(edge.he_plus == half_edge)
}

/// **A chart channel's arm, and which kind of arm it is** — the answer
/// [`chart_u_arm`] gives, so that the door a gap on that channel
/// reaches the band through is picked by the type rather than by a
/// paragraph.
///
/// The two variants are two different crossings, and on the first
/// channel the difference is what the chart's `u` MEANS:
///
/// - [`ChartArm::Angular`] — `u` is an AZIMUTH, so the arm is metres
///   per RADIAN and the gap it meters is an angle. Metres per radian
///   is not a rate per parameter unit, so the crossing is
///   [`Margin::levered`]'s and no [`SupSpeed`] is minted.
/// - [`ChartArm::Rate`] — `u` is the CHART'S OWN parameter (a plane's
///   metre axis, a spline chart's net parameter), so the arm is metres
///   per chart-`u` unit and the gap it meters is a parameter span.
///   That is the rate pair's own crossing, sup side, and it goes
///   through [`Margin::metered_sup`].
///
/// The second channel splits the same way — a polar radius is angular
/// ([`polar_arm`]), a spline chart's `v` rate is not ([`v_meter`]) —
/// and the polar arm is spelled with this type where it meters a gap.
///
/// Both leave metres, and [`ChartArm::meter`] is the one place that
/// says which door does it.
#[derive(Clone, Copy, Debug)]
pub enum ChartArm<T> {
    /// Metres per RADIAN at a latitude — see the type docs.
    Angular(T),
    /// Metres per chart-`u` unit — see the type docs.
    Rate(SupSpeed<T>),
}

impl<T: Real> ChartArm<T> {
    /// A first-channel gap crossed to metres by whichever door this
    /// arm's kind names — one multiply either way, so the two doors
    /// differ in what they assert and in nothing else.
    pub(crate) fn meter(self, gap: T) -> Margin<T> {
        match self {
            Self::Angular(arm) => Margin::levered(gap, arm),
            Self::Rate(rate) => Margin::metered_sup(gap, rate),
        }
    }

    /// The arm's own magnitude, for the gate that asks whether the arm
    /// itself is collapsed (a spline chart's net-level singular gate,
    /// [`singular_at`]: *"can any first-channel displacement move a
    /// point at all?"*). That is a question about the arm and not a
    /// crossing, so no door applies and the tag comes off here,
    /// deliberately.
    pub(crate) fn magnitude(self) -> T {
        match self {
            Self::Angular(arm) => arm,
            Self::Rate(rate) => rate.get(),
        }
    }
}

/// The FIRST-CHANNEL arm of a chart at second-parameter value `v` —
/// the metres a unit step of the chart's `u` moves the mapped point
/// (D4 ¶1: no UV-space tolerance ever reaches ε), **and which kind of
/// arm that is** ([`ChartArm`]).
///
/// On the azimuth charts this is the LOCAL lever at that latitude:
/// cylinder `r`, sphere `|r·cos v|`, torus `|R + r·cos v|`, cone
/// `|v·sin α|` — metres per radian, every one. A joint does not read
/// it: its lever is the joint vertex's own distance from the axis
/// ([`joint_arm`]), the same quantity without a stored latitude in it.
///
/// On the non-azimuth charts there is no latitude and no angle: a
/// plane's `u` IS metres, so its arm is exactly 1 by construction; a
/// spline chart's `u` is the net's own parameter, whose metre stretch
/// is whatever the net says. On those three kinds the number is
/// `geom_brep::chart_stretch_sup`'s first component — a rate per
/// parameter unit, minted a [`SupSpeed`] there and carried as one
/// here, because the gap it meters is a chart-parameter span and not
/// an angle.
///
/// # Direction of error: why the spline arm is the SUP bound
///
/// Every spline-chart caller of this function divides a chart-space
/// discrepancy against the linear band and asks whether the mapped
/// points are within ε — an ESCAPE claim in both of its two uses, and
/// the sup bound is the conservative side of each:
///
/// - `pcurve_loop_continuity` ([`spline_gap_closes`]) asks *"does this
///   joint gap keep the loop closed?"*. An OVER-stated arm over-states
///   the metre gap, which can only turn a `Zero` (closed) into a
///   definite discontinuity or an escalation. It refuses; it cannot
///   certify a loop closed across a gap the model can see. An
///   UNDER-stated arm does the reverse, and `1` on a chart whose
///   stretch is 100 m per chart unit under-states by exactly that
///   factor.
/// - `pcurve_loop_pole_joint` ([`singular_at`]'s net-level form) asks
///   *"is this lever zero, so that no shift can select a branch?"*.
///   Here too sup is the safe side: `Zero` under the SUP bound means no
///   `u` displacement anywhere on the chart moves the point past the
///   band, so skipping the branch shift is honest. Under an inf reading
///   the same verdict would claim a collapsed lever on a chart that has
///   one.
///
/// `geom_brep::chart_stretch_sup` is that bound and states the same
/// split at the export; it is emphatically not a lower bound, and
/// nothing here may be read as one.
fn chart_u_arm<T: Real>(chart: DescribedChart<'_, T>, v: T) -> ChartArm<T> {
    let surface = chart.surface();
    match *surface {
        Surface::Cylinder { radius, .. } => ChartArm::Angular(radius),
        Surface::Sphere { radius, .. } => ChartArm::Angular((radius * v.cos()).abs()),
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => ChartArm::Angular((major_radius + minor_radius * v.cos()).abs()),
        Surface::Cone { half_angle, .. } => ChartArm::Angular((v * half_angle.sin()).abs()),
        // The plane answers exactly 1 through this door (its chart
        // parameters ARE metres), and each spline kind answers its
        // net's own `sup |S_u|`. `chart_stretch_sup` refuses the cone,
        // answered above, and the placeholder, which a
        // [`DescribedChart`] is not.
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
            let Ok((sup_u, _)) = geom_brep::chart_stretch_sup(surface) else {
                unreachable!(
                    "chart_stretch_sup refuses only the cone, answered above, and the \
                     placeholder, which DescribedChart::of excludes"
                )
            };
            ChartArm::Rate(sup_u)
        }
    }
}

/// The chart's **u period** — the whole-number shift the loop walk may
/// apply to land a half-edge's entry on its predecessor's exit
/// (`walk_loop`), and the wrap [`loop_closes`] allows.
///
/// * every AZIMUTH chart (cylinder, cone, sphere, torus) answers `τ`;
/// * a PLANE answers `None`: its `u` is a length, not an angle, so it
///   has no branch to decide;
/// * a NURBS chart CLOSED in `u` — first and last control columns the
///   same locus at the band — answers the payload's own u knot-domain
///   length;
/// * a NURBS chart NOT closed in `u` answers `None`: no shift offered,
///   no wrapped closure accepted.
///
/// A kernel-built chart is normalized to `[0, 1]²`, so a closed-in-u
/// one answers `Some(1.0)`, and a gap can shift by a whole chart period
/// there. The joint's own chart-space gap still certifies the shifted
/// entry ([`spline_gap_closes`]).
///
/// On a spline chart the shift is offered only when the net-level
/// singular gate ([`singular_at`]) reads its `u` stretch as definitely
/// nonzero; that stretch is the net's own `sup |S_u|`
/// ([`chart_u_arm`]), not a constant. A chart whose whole `u` stretch
/// sits under the band takes no shift at all (no `u` displacement on
/// it moves a point past ε), one whose stretch lands inside the band
/// takes none either, and the joint's chart-space gap decides; only a
/// definitely-metric chart reaches the branch decision.
///
/// **Why a NURBS chart needs this (#327).** A full-period cylinder
/// wall stated as ONE B-spline patch with a seam generator used twice
/// is the shape every translator writes and the kernel's own band
/// re-mint produces. Its loop walks the chart's whole u range and its
/// seam edge takes the two u-boundary branches — exactly the analytic
/// seam case, but at the chart's own period rather than `τ`. Without
/// this the walk reports the wrap as a discontinuity: a symptom, not
/// the cause.
fn chart_u_period<T: Decide>(surface: &Surface<T>, band: Band) -> Option<T> {
    // `τ` belongs to the AZIMUTH charts. A spline chart — the
    // payload's or an approximating surface's fit — has whatever period
    // its knot domain says, and only if the net actually closes; handing
    // it `τ` would let the loop walk wrap a chart that does not.
    if matches!(surface, Surface::Plane { .. }) {
        return None;
    }
    let Some(payload) = surface.spline_chart() else {
        return Some(T::tau());
    };
    let (u0, u1) = payload.knots_u().domain();
    let (nu, nv) = payload.control_counts();
    if nu < 2 || nv == 0 {
        return None;
    }
    let control = payload.control();
    for iv in 0..nv {
        let a = control.get(iv)?;
        let b = control.get((nu - 1) * nv + iv)?;
        match decide("pcurve_chart_u_closed", Margin::of(a.distance(*b)), band) {
            Ok(Sign::Zero) => {}
            _ => return None,
        }
    }
    Some(T::from_f64(u1 - u0))
}

/// The meridional (second-channel) lever arm where that channel is
/// itself an angle — sphere `v` (arm `r`), torus `v` (arm `r_minor`).
/// `None` = the channel is NOT an angle, which is a different claim
/// from "already metres": see [`v_meter`], which is what a caller
/// wanting the channel's metre rate must ask.
fn polar_arm<T: Real>(surface: &Surface<T>) -> Option<T> {
    match *surface {
        Surface::Sphere { radius, .. } => Some(radius),
        Surface::Torus { minor_radius, .. } => Some(minor_radius),
        // The second channel is not an ANGLE on these charts, so no
        // polar radius levers it. That does not make it metres: on a
        // plane, cylinder or cone `v` IS a length, but a spline
        // chart's `v` is the net's own parameter, whose metre rate
        // [`v_meter`] reads off the chart. `None` here means "no
        // polar arm", and `v_meter` is the door that answers what the
        // rate actually is.
        Surface::Plane { .. }
        | Surface::Cylinder { .. }
        | Surface::Cone { .. }
        | Surface::Nurbs(_)
        | Surface::Approx(_) => None,
    }
}

/// The SECOND channel's metre rate for a spline chart's joint gap
/// ([`spline_gap_closes`]) — the `v` companion of [`chart_u_arm`], and
/// the same escape claim.
///
/// Where the channel is an angle ([`polar_arm`]: sphere, torus) the
/// arm is that exact radius. Where it is not, the channel's metre
/// rate is the chart's `sup |S_v|`: exactly 1 on a plane, cylinder or
/// cone, where `v` IS a length, and the net's own stretch on a spline
/// chart, where it is not. The direction argument is
/// [`chart_u_arm`]'s — an over-stated rate can only refuse a closure,
/// while `1` on a chart with a 100 m/unit stretch under-states the
/// metre gap by that factor and certifies a loop closed across it.
/// Unlike [`chart_u_arm`] this channel's parameter is not always an
/// angle, so the crossing IS a rate per parameter unit and the bound
/// direction rides out as a [`SupSpeed`]: the exact polar radius is a
/// sup by being exact, and the spline stretch is one by derivation.
fn v_meter<T: Real>(chart: DescribedChart<'_, T>) -> SupSpeed<T> {
    match polar_arm(chart.surface()) {
        Some(radius) => SupSpeed::new(radius),
        // Every described kind has a second-channel sup, the cone
        // included (its `v` is a slant length, so the rate is exactly
        // 1), which is why this reads the `v`-only door rather than
        // the pair.
        None => geom_brep::chart_stretch_sup_v(chart.surface()).unwrap_or_else(|_| {
            unreachable!(
                "chart_stretch_sup_v refuses only the placeholder, which \
                 DescribedChart::of excludes"
            )
        }),
    }
}

/// A whole-period shift of the MERIDIONAL channel — the `v` twin of
/// [`geom_brep::Pcurve::shift_branch`], for the charts whose second
/// parameter is an angle (sphere/torus). The harmonic form carries its
/// meridional constant in `p0.y` and a spiric WALL image in `v0` (a
/// spiric cap's chart is a plane, which has no periodic channel to
/// shift); a fitted image (a sphere's general circle) translates its
/// control net. Other variants answer themselves unchanged — the walk
/// never computes a nonzero shift for them.
fn shift_polar_branch<T: Real>(pcurve: &Pcurve<T>, k: T, period: T) -> Pcurve<T> {
    match pcurve {
        Pcurve::Harmonic { p0, pa, pb, pl } => Pcurve::Harmonic {
            p0: geom_core::Point2::new(p0.x, p0.y + k * period),
            pa: *pa,
            pb: *pb,
            pl: *pl,
        },
        Pcurve::Spiric {
            major,
            minor,
            offset,
            image: geom_brep::SpiricImage::Wall { u0, v0, sense },
        } => Pcurve::Spiric {
            major: *major,
            minor: *minor,
            offset: *offset,
            image: geom_brep::SpiricImage::Wall {
                u0: *u0,
                v0: *v0 + k * period,
                sense: *sense,
            },
        },
        // A translation is affine, and a fitted image takes it exactly.
        fitted @ Pcurve::Fitted(_) => {
            fitted.map_affine(|p| geom_core::Point2::new(p.x, p.y + k * period), |v| v)
        }
        other => other.clone(),
    }
}

/// The sphere chart's INVOLUTION twin of a harmonic or fitted image:
/// `S(u + π, π − v) = S(u, v)` holds identically on a sphere chart
/// (`radial(u+π) = −radial(u)`, `cos(π−v) = −cos v`, `sin(π−v) =
/// sin v`), so every sphere pcurve has exactly two harmonic
/// representations and a pole-crossing walk legitimately needs the
/// OTHER one on the far side — a π azimuth step no whole-period shift
/// can produce. The torus has no such twin (R > 0 breaks the
/// symmetry), and neither does the cone (cos α > 0).
fn sphere_twin<T: Decide>(surface: &Surface<T>, pcurve: &Pcurve<T>) -> Option<Pcurve<T>> {
    if !matches!(surface, Surface::Sphere { .. }) {
        return None;
    }
    let pi = T::pi();
    if matches!(pcurve, Pcurve::Fitted(_)) {
        // The involution is affine, so the fitted image's twin is its
        // control net mapped through it.
        return Some(pcurve.map_affine(
            |p| geom_core::Point2::new(p.x + pi, pi - p.y),
            |v| geom_core::Vec2::new(v.x, T::zero() - v.y),
        ));
    }
    let Pcurve::Harmonic { p0, pa, pb, pl } = pcurve else {
        return None;
    };
    Some(Pcurve::Harmonic {
        p0: geom_core::Point2::new(p0.x + pi, pi - p0.y),
        pa: geom_core::Vec2::new(pa.x, T::zero() - pa.y),
        pb: geom_core::Vec2::new(pb.x, T::zero() - pb.y),
        pl: geom_core::Vec2::new(pl.x, T::zero() - pl.y),
    })
}

/// The loop-closure test of a chart walk: the closing joint's deck
/// element ([`lift_joint`]) — the first row read against the last
/// row's exit `end`, at the first row's entry vertex — is the loop's
/// **winding**, and the loop closes where [`Lift::closes`] admits it.
/// A closure the deck element does not decide does not close.
fn loop_closes<T: Decide>(
    chart: DescribedChart<'_, T>,
    first: &Pcurve<T>,
    entry_t: T,
    vertex: geom_core::Point3<T>,
    end: geom_core::Point2<T>,
    u_period: Option<T>,
    band: Band,
) -> bool {
    lift_joint(chart, first.clone(), entry_t, end, vertex, u_period, band)
        .is_ok_and(|lift| lift.closes())
}

/// One half-edge's minted chart curve, before certification: the
/// branch-pinned image, its carrier and the carrier interval, keyed on
/// whatever the walk names its half-edges by — a [`HalfEdgeKey`] in the
/// pass, a [`SiteHalf`] in an Euler operator's plan.
pub(crate) struct Walked<T: Real, K = HalfEdgeKey> {
    key: K,
    carrier: geom::Curve3<T>,
    pcurve: Pcurve<T>,
    t0: T,
    t1: T,
}

/// One face's boundary as this module's passes walk it: each loop's
/// half-edge cycle, **whether that boundary already STORES a row**, and
/// where the row set has a gap.
///
/// Read by [`validate_pcurves`], which re-certifies the stored rows and
/// reports the gaps, and by the site mint ([`site_rows_from`]), which
/// asks [`StoredRows::remints`] whether its door re-mints the face. So
/// "which rows does this face store" and "is the set complete" each
/// have one answer.
pub(crate) struct StoredRows {
    /// The face's loops in walk order, outer first, each with its
    /// half-edge cycle: `None` where the loop record or its cycle did
    /// not resolve — tier 1's corruption. A loop whose boundary is not
    /// a `Cycle` is ABSENT from this list: there is nothing to walk and
    /// nothing wrong.
    pub(crate) loops: Vec<(LoopKey, Option<Vec<HalfEdgeKey>>)>,
    /// Whether those cycles store any row: `false` when the face's
    /// boundary stores none at all, so the face has not been minted.
    pub(crate) stores: bool,
    /// The gaps in the row set, in walk order: a loop that did not
    /// walk, and each half-edge of a walked loop with no row.
    pub(crate) gaps: Vec<RowGap>,
}

/// One gap in a face's row set ([`StoredRows::gaps`]).
#[derive(Clone, Copy, Debug)]
pub(crate) enum RowGap {
    /// A loop of the face whose record or cycle did not resolve.
    Corrupt,
    /// A half-edge of a walked loop that stores no row.
    Missing {
        /// The half-edge.
        half_edge: HalfEdgeKey,
        /// The loop it is on.
        r#loop: LoopKey,
    },
}

impl StoredRows {
    /// **Whether the face's row set is COMPLETE**: it stores a row,
    /// every loop walks, and every half-edge of every loop carries a
    /// row. A face storing no row is unminted, not complete; an `Empty`
    /// loop holds no half-edge and so misses no row.
    pub(crate) fn complete(&self) -> bool {
        self.stores && self.gaps.is_empty()
    }

    /// **Whether a site mint re-mints the face** ([`site_rows`]): it
    /// stores a row, every loop walks, and every row it misses is on a
    /// loop in `open` — the loops a null edge holds open
    /// ([`StoredRows::open_loops`]) — unless the door `released` it,
    /// taking the last null edge off it, whatever else it misses.
    ///
    /// **"Minted" is read as "stores a row"** (`stores`). No door writes
    /// a row onto a face no mint has run over: the site mint leaves a
    /// face storing no row as found, and `mev_null` derives none. So a
    /// face storing any row was minted, and a half missing one arrived
    /// after that mint; a face storing none — never minted, or emptied
    /// by a door — is the producer's closing mint's, and stays rowless
    /// until it runs.
    ///
    /// **A loop a null edge holds open cannot be walked**: the edge's
    /// halves have no carrier. So no door before the edge leaves the
    /// loop can mint its rows — the edge's own two, or those of the
    /// halves an operator adds to the loop meanwhile — and those gaps
    /// do not keep the face from its site mint, which mints every loop
    /// its door leaves running through no null edge ([`site_rows`]).
    ///
    /// **The door that takes a face's last null edge off it re-mints
    /// it whatever it misses**, as the minting pass would once the
    /// scaffolding is gone: a null edge's first description, which
    /// re-walks every loop, and an operator that rewires a loop out
    /// from under the edge, as the pipelines' joins do. An operator
    /// re-mints only the loops it rewires, so a gap on a loop it keeps
    /// stays. **Any other gap is not the site mint's to fill**: a door
    /// left the face half-minted ([`PcurveMintError::MissingCache`]
    /// lists them) and the site mint leaves it as found, for the
    /// producer's final pass.
    pub(crate) fn remints(&self, open: &[LoopKey], released: bool) -> bool {
        self.stores
            && self.loops.iter().all(|(_, cycle)| cycle.is_some())
            && (released
                || self.gaps.iter().all(
                    |gap| matches!(gap, RowGap::Missing { r#loop, .. } if open.contains(r#loop)),
                ))
    }
}

impl StoredRows {
    /// The loops of the face a null edge holds open ([`held_open`]), in
    /// walk order. Empty, with nothing read, on a face that misses no
    /// row: a null half never stores one, since every row derives from
    /// its half's carrier and a null edge has none.
    ///
    /// # Errors
    ///
    /// [`held_open`]'s.
    pub(crate) fn open_loops<T: Decide>(
        &self,
        body: &Body<T>,
    ) -> Result<Vec<LoopKey>, SiteRowRefusal> {
        let mut open = Vec::new();
        if self.gaps.is_empty() {
            return Ok(open);
        }
        for (lk, cycle) in &self.loops {
            if let Some(cycle) = cycle
                && held_open(body, cycle.iter().copied())?
            {
                open.push(*lk);
            }
        }
        Ok(open)
    }
}

/// **Whether a null edge holds a loop open**: one of `halves`, the
/// loop's half-edges, is a half of a null edge
/// ([`CurveGeom::NullScaffold`]), scaffolding with no carrier, so the
/// loop cannot be walked. The site mint's one reading of it
/// ([`StoredRows::open_loops`], [`site_rows`]).
///
/// # Errors
///
/// [`SiteRowRefusal::Corrupt`] where a half, its edge or its curve does
/// not resolve — tier 1's corruption, which cannot be read as either.
fn held_open<T: Decide>(
    body: &Body<T>,
    halves: impl IntoIterator<Item = HalfEdgeKey>,
) -> Result<bool, SiteRowRefusal> {
    let mut open = false;
    for he in halves {
        let curve = body
            .get_half_edge(he)
            .and_then(|h| body.get_edge(h.edge))
            .and_then(|e| body.get_curve_geom(e.curve))
            .ok_or(SiteRowRefusal::Corrupt)?;
        open |= curve.null_scaffold().is_some();
    }
    Ok(open)
}

/// One loop's half-edge cycle, as this module's walks read it —
/// [`loop_rows`]'s answer.
pub(crate) enum LoopRows {
    /// The loop walked: its half-edges in cycle order. These are the
    /// keys the map is keyed on for this loop, and the only ones.
    Cycle(Vec<HalfEdgeKey>),
    /// The loop's boundary is not a cycle
    /// ([`crate::LoopBoundary::Empty`]): there is nothing to walk and
    /// nothing wrong. It holds no half-edge, so it holds no row.
    NoCycle,
    /// The loop record or its cycle did not resolve — tier 1's
    /// corruption, which each caller reports in its own vocabulary.
    Corrupt,
}

/// **The one per-loop rows walk**: the half-edges of `r#loop` a pcurve
/// row can be keyed on.
///
/// A row is keyed on a half-edge and belongs to the face that
/// half-edge's loop is on, so every question this module asks about
/// one loop's rows — which rows a face STORES ([`stored_rows`], and
/// through it [`validate_pcurves`] and [`split_cache`]), and which
/// rows a door that re-parents the loop must dispose of
/// ([`crate::Body::drop_rows_on_chart_change`]) — is a walk of this
/// one cycle. Two spellings of it would be two answers to "which rows
/// does this loop have", and a door and the validator disagreeing
/// about that is exactly the defect neither could see.
pub(crate) fn loop_rows<T: Decide>(body: &Body<T>, r#loop: LoopKey) -> LoopRows {
    let Some(loop_data) = body.get_loop(r#loop) else {
        return LoopRows::Corrupt;
    };
    let crate::entity::LoopBoundary::Cycle { first } = loop_data.boundary else {
        return LoopRows::NoCycle;
    };
    match body.loop_cycle(first) {
        Some(cycle) => LoopRows::Cycle(cycle),
        None => LoopRows::Corrupt,
    }
}

/// [`StoredRows`] for one face: its loops walked once.
pub(crate) fn stored_rows<T: Decide>(body: &Body<T>, face: &crate::entity::Face) -> StoredRows {
    let mut loops = Vec::new();
    let mut gaps = Vec::new();
    let mut stores = false;
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        let cycle = match loop_rows(body, lk) {
            LoopRows::NoCycle => continue,
            LoopRows::Corrupt => {
                gaps.push(RowGap::Corrupt);
                loops.push((lk, None));
                continue;
            }
            LoopRows::Cycle(cycle) => cycle,
        };
        for &he in &cycle {
            if body.pcurve(he).is_some() {
                stores = true;
            } else {
                gaps.push(RowGap::Missing {
                    half_edge: he,
                    r#loop: lk,
                });
            }
        }
        loops.push((lk, Some(cycle)));
    }
    StoredRows {
        loops,
        stores,
        gaps,
    }
}

/// The two rows a parameter split leaves where ONE parent half-edge's
/// row was, named by where they go: the first child's `[t₀, t]` stays
/// on the parent half (which IS the first child's half), the second
/// child's `[t, t₁]` goes to the new half minted beside it.
pub(crate) struct CarriedRows<T: Real> {
    /// The first child's row, for the parent half-edge.
    pub(crate) parent_half: PcurveCache<T>,
    /// The second child's row, for the half-edge minted beside it.
    pub(crate) new_half: PcurveCache<T>,
}

/// Why [`split_cache`] could not state a restriction.
#[derive(Clone, Debug)]
pub(crate) enum SplitRowError {
    /// The topology the stored row is ABOUT does not resolve: the
    /// half-edge, its edge's certified curve, its parent loop, that
    /// loop's face, that face's surface, or that face's boundary not
    /// carrying the half-edge whose row this is. A row states a curve
    /// in a FACE's chart, so without the chart there is nothing to
    /// restrict it to, and a body that holds the row and not the chart
    /// is tier-1 corrupt — the caller reports it as the stale key it
    /// is, rather than proceeding rowless.
    ///
    /// Not reachable from [`crate::Body::split_edge`]: its own gates
    /// resolve both halves live and their edge's curve `Certified`
    /// before this runs, and a live half-edge's loop, face and surface
    /// are tier-1 invariants of a body that passed them.
    Stale {
        /// The half-edge whose row could not be placed.
        half_edge: HalfEdgeKey,
    },
    /// The parent's image, restricted to one child's sub-interval,
    /// failed the closed-form certification the whole image passed. A
    /// covered lane that refuses here is a genuine defect and is
    /// raised, never swallowed.
    Certify {
        /// The parent half-edge whose row was being restricted.
        half_edge: HalfEdgeKey,
        /// The typed certification failure.
        error: PcurveCertifyError,
    },
}

/// The **restriction of an edge's two half-edge rows to the two
/// children of a parameter split** — [`crate::Body::split_edge`]'s
/// pcurve limb, and the reason that op carries its rows across the
/// surgery instead of staling them. The answer holds one entry per
/// given half-edge: that half-edge's [`CarriedRows`], or `None` where
/// there is nothing to carry.
///
/// A [`Pcurve`] is a function of the **carrier parameter** and carries
/// no interval of its own ([`Pcurve::eval`]), exactly as an
/// [`geom_brep::EdgeCurve`]'s carrier does: the children of a split at
/// `t` have the parent's chart image, restricted to `[t₀, t]` and
/// `[t, t₁]`. So this re-certifies the parent's own image over each
/// sub-interval rather than deriving anything — and that one sentence
/// is the whole bound argument: a restriction re-certifies through
/// [`PcurveCache::certify`], which `geom-brep` declares in an
/// `impl<T: Decide>` block, so `split_edge` keeps the `Decide` bound it
/// has and no caller's bound moves. The fitted door
/// ([`crate::AtRestPolicy::fitted_lane`]) is the DERIVATION lanes', and
/// nothing here derives.
///
/// **`t₀` and `t₁` are the EDGE's certified interval**, read from the
/// carrier through [`half_edge_carrier`] — the same interval
/// `EdgeCurve::split_specs` cuts the children's curves from — and not
/// the row's own stored `cache.params()`. The two agree on a face the
/// minting pass wrote; where a stored row disagrees, the edge is what
/// the children are made of and the row is what is being re-certified,
/// so the carrier decides the sub-intervals and the certification
/// measures the row against them.
///
/// Read-only, so a refusal reaches `split_edge` before any mutation
/// and the op's "untouched on `Err`" contract is unaffected.
///
/// # `None`, and what it does NOT claim
///
/// - `half_edge` carries no row (an all-planar body, a face of an
///   uncovered class, or one a door has left rowless for its
///   producer's closing mint);
/// - the row's image is [`Pcurve::Fitted`] or [`Pcurve::General`],
///   whose certification doors are the fitted door's
///   ([`PcurveCache::certify_fitted`] / [`PcurveCache::certify_general`],
///   which need the mate operand and the fitted machinery). Widening
///   `split_edge` to reach them is the bound ripple banked at
///   [`mint_faces`]; until it lands, a split of an edge carrying a
///   `General` row leaves that face exactly as it found it — the
///   pre-existing behaviour, tracked on TOPO's slate as
///   `split-edge-cannot-carry-a-fitted-or-general-pcurve-row`.
///
/// In both cases the caller writes nothing, so the map is left exactly
/// as found. A key that does not resolve is NOT one of them: it
/// refuses [`SplitRowError::Stale`].
///
/// # The SPLINE-chart frontier
///
/// The carry is exact on every kind it covers, spline charts included
/// (a described-NURBS wall's `IsoLine` and `IsoArc` rows restrict like
/// any other, and tier 3 reads `Ok` after the split). What a spline
/// chart does not have is the RECOVERY step: [`mint_pcurves`], the
/// pass a rowless face's caveat names, refuses on the body such a
/// split produces, because [`nurbs_iso_derive`]'s rim arms map an
/// edge's WHOLE carrier interval onto the chart's whole `u` domain,
/// which a sub-edge no longer spans (`IsoUnsupported` on an arc rim, a
/// loop-continuity finding on a line rim). Pre-existing and untouched
/// here — the split now leaves nothing for that pass to do — and filed
/// on TRIM's slate as
/// `iso-derivation-arms-assume-an-edge-spans-the-charts-whole-domain`.
/// So "the carried rows are the mint pass's rows" is a claim about the
/// ANALYTIC charts (cylinder, sphere, torus), where the pass runs.
///
/// # Errors
///
/// [`SplitRowError`] — a certification the whole image passed failing
/// over a sub-interval, or a key the restriction needs not resolving.
pub(crate) fn split_cache<T: Decide>(
    body: &Body<T>,
    halves: [HalfEdgeKey; 2],
    t: T,
    band: Band,
) -> Result<[Option<CarriedRows<T>>; 2], SplitRowError> {
    let mut rows = [None, None];
    for (slot, half_edge) in halves.into_iter().enumerate() {
        let Some(cache) = body.pcurve(half_edge) else {
            continue;
        };
        if matches!(cache.pcurve(), Pcurve::Fitted(_) | Pcurve::General(_)) {
            continue;
        }
        let stale = || SplitRowError::Stale { half_edge };
        let (carrier, t0, t1) = half_edge_carrier(body, half_edge).map_err(|_| stale())?;
        let surface = half_edge_surface(body, half_edge).map_err(|_| stale())?;
        let image = cache.pcurve().clone();
        let certify = |a: T, b: T, image: Pcurve<T>| {
            PcurveCache::certify(image, a, b, &carrier, &surface, band)
                .map_err(|error| SplitRowError::Certify { half_edge, error })
        };
        rows[slot] = Some(CarriedRows {
            parent_half: certify(t0, t, image.clone())?,
            new_half: certify(t, t1, image)?,
        });
    }
    Ok(rows)
}

/// Mints (and certifies) the pcurve caches of every curved face of
/// `body` whose chart has a certified closed-form lane — the pass the
/// C5 splitting lane runs on each side it produces (spec §1: caches are
/// minted where curved faces are minted).
///
/// Idempotent by construction on a given body: it walks faces in arena
/// order, loops in outer-then-rings order, and half-edges in `next`
/// order, and overwrites any existing row with the same derivation
/// (D9 — same body, same bits).
///
/// # Errors
///
/// [`PcurveMintError`] — a certification refusal, a discontinuous or
/// unclosed loop walk, or an escalated classification. Never a silent
/// skip of a face the lane covers.
pub fn mint_pcurves<T: AtRestPolicy>(body: &mut Body<T>, tol: Tol) -> Result<(), PcurveMintError> {
    let band = Band::linear(tol).map_err(PcurveMintError::Band)?;
    // Start from empty. A body reaching this pass may have been carved
    // from a scratch clone that inherited rows for half-edges the
    // surgery killed (a `SecondaryMap` row outlives its key until the
    // slot is reused), and a stale cache is worse than no cache. What
    // this pass leaves behind is exactly what it minted and certified.
    // The whole-body entry is the only one that can make that claim: a
    // row whose half-edge is dead is reachable from no face, so the
    // subset entry below clears through the faces it is given.
    let found = core::mem::take(&mut body.pcurves);
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    mint_faces(body, &faces, band, &found)?;
    Ok(())
}

/// [`mint_pcurves`] restricted to `faces`: the rows of exactly those
/// faces' half-edges are cleared and re-derived, and **every other row
/// of `body` is left exactly as it was found**.
///
/// The pass is **per face**: a face's branch pinning and
/// certification read that face's own loops, surface and edge
/// descriptions and nothing else, so on the named faces the result is
/// bit-for-bit [`mint_pcurves`]'s, and that pass's idempotence and
/// determinism statements hold verbatim for them — in the order they
/// are named.
///
/// # This is NOT `mint_pcurves` over a subset of the map
///
/// The two differ in what they CLEAR, and the difference is not
/// cosmetic. `mint_pcurves` empties the map first, so it also drops
/// rows whose half-edge no longer exists — a `SecondaryMap` row
/// outlives its key until the slot is reused. This entry reaches rows
/// through the faces it is given, and **a dead half-edge is reachable
/// from no face**: after a kill op, the rows of the killed half-edges
/// survive this pass over *every* face of the body, where the
/// whole-body pass leaves none. `validate_pcurves` cannot see them
/// either — it reaches rows through face loops — so they are invisible
/// to tier 3 until the slot is recycled.
///
/// **A caller that has killed a half-edge and wants the map's at-rest
/// guarantee must use [`mint_pcurves`].** What this entry gives is the
/// weaker, scoped statement a partial producer can honestly make: the
/// rows it may have staled are re-derived, and it asserts nothing at
/// all about the rest of the map. The simultaneous offset doors are
/// exactly that caller: they perform no topological surgery, so no
/// half-edge of theirs ever dies and the two statements coincide.
/// Pinned by `sweep`'s `shell10_r1_probes` and `shell10_r2_probes`,
/// which kill a half-edge through the public `kef` and count what each
/// pass leaves.
///
/// # Returns
///
/// The number of rows the named faces carry when the pass returns.
/// Computed as the map's growth across the mint, which is the same
/// number: every row cleared above belonged to a named face, and a
/// half-edge lies on exactly one loop and so on exactly one face, so
/// minting a named face can only re-fill rows the clearing emptied.
///
/// # Errors
///
/// [`mint_pcurves`]'s, raised by a face in `faces`.
pub fn mint_pcurves_of<T: AtRestPolicy>(
    body: &mut Body<T>,
    faces: &[FaceKey],
    tol: Tol,
) -> Result<usize, PcurveMintError> {
    let band = Band::linear(tol).map_err(PcurveMintError::Band)?;
    let mut found = FoundRows::new();
    for &face in faces {
        for (he, row) in clear_face_caches(body, face) {
            found.insert(he, row);
        }
    }
    let before = body.pcurves.len();
    mint_faces(body, faces, band, &found)?;
    Ok(body.pcurves.len() - before)
}

/// The rows a mint cleared before re-deriving, by half-edge: what it
/// carries onto a face whose rows it cannot derive ([`carry_rows`]).
type FoundRows<T> = slotmap::SecondaryMap<HalfEdgeKey, PcurveCache<T>>;

/// The mint itself, over the faces it is handed: the shared body of
/// [`mint_pcurves`] and [`mint_pcurves_of`], which differ only in which
/// rows they clear first.
fn mint_faces<T: AtRestPolicy>(
    body: &mut Body<T>,
    faces: &[FaceKey],
    band: Band,
    found: &FoundRows<T>,
) -> Result<(), PcurveMintError> {
    for &face in faces {
        match mint_face(body, face, band) {
            Ok(()) => {}
            // Rows not owed ([`not_owed`]): a pair the chart can hold
            // but no route covers yet (an oblique torus circle, a tilted
            // cone section, a spline carrier on an analytic chart), or a
            // fitted face at a scalar with no fitted door. The at-rest
            // pass excuses exactly these, by the same predicate.
            // `mint_face` stores rows only once every half-edge
            // certified, so a refused face holds none and there is
            // nothing to clear. Every OTHER failure — a carrier off its
            // face, an image that is not its carrier's, a general sphere
            // circle its fitted route refuses, a covered class whose
            // residuals, envelope, continuity or closure refuse — is a
            // genuine defect and propagates.
            // A row the face held is carried, re-certified, so the
            // mint never drops a certificate it cannot re-derive
            // ([`carry_rows`]).
            Err(e) if not_owed::<T>(&e) => carry_rows(body, face, band, found)?,
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

/// **The rows a face held, carried across a mint that cannot derive
/// them**: each one the face's loops reach in `found` is re-certified
/// through its own door against the current carrier and surface, and
/// stored. So a row the mint has
/// no route to — a `Fitted` row on a class the closed-form lane does not
/// cover, stated by a certifying door — survives a producer's closing
/// mint (a transform, a merge) exactly when it still certifies, and a
/// half the face held no row for stays without one, for tier 3 to read.
///
/// # Errors
///
/// [`PcurveMintError::Certify`] for a carried row that no longer
/// certifies, and [`PcurveMintError::RowInterval`] for one whose
/// interval is no longer its edge's ([`row_interval`]): the producer
/// refuses rather than drop it or store it stale.
fn carry_rows<T: AtRestPolicy>(
    body: &mut Body<T>,
    face: FaceKey,
    band: Band,
    found: &FoundRows<T>,
) -> Result<(), PcurveMintError> {
    let face_data = body.get_face(face).ok_or(PcurveMintError::Corrupt)?;
    let surface = body
        .get_surface(face_data.surface)
        .cloned()
        .ok_or(PcurveMintError::Corrupt)?;
    let mut held: Vec<(HalfEdgeKey, &PcurveCache<T>)> = Vec::new();
    for lp in core::iter::once(face_data.outer).chain(face_data.rings.iter().copied()) {
        if let LoopRows::Cycle(cycle) = loop_rows(body, lp) {
            held.extend(
                cycle
                    .into_iter()
                    .filter_map(|he| found.get(he).map(|row| (he, row))),
            );
        }
    }
    let mut carried = Vec::with_capacity(held.len());
    for (he, row) in held {
        let (carrier, _, _) = half_edge_carrier(body, he)?;
        // A producer that moved the edge's interval under the row
        // refuses here, not later at tier 3.
        row_interval(body, he, row, &carrier, band)?;
        let (t0, t1) = row.params();
        let mate = mate_surface(body, he);
        let restated = match row.pcurve() {
            Pcurve::Fitted(image) => match T::fitted_lane() {
                Some(lane) => PcurveCache::certify_fitted(
                    std::sync::Arc::clone(image),
                    t0,
                    t1,
                    &carrier,
                    &surface,
                    mate.as_ref(),
                    band,
                    lane,
                ),
                None => Err(PcurveCertifyError::FittedLaneUnsupported { scalar: T::NAME }),
            },
            Pcurve::General(image) => PcurveCache::certify_general(
                std::sync::Arc::clone(image),
                t0,
                t1,
                &carrier,
                &surface,
                mate.as_ref(),
                band,
                T::fitted_lane(),
            ),
            image => PcurveCache::certify(image.clone(), t0, t1, &carrier, &surface, band),
        }
        .map_err(|error| PcurveMintError::Certify {
            half_edge: he,
            error,
        })?;
        carried.push((he, restated));
    }
    for (he, row) in carried {
        body.pcurves.insert(he, row);
    }
    Ok(())
}

/// Drops `face`'s rows before it is re-derived, and returns them for
/// the mint to carry where it cannot derive the face ([`carry_rows`]).
fn clear_face_caches<T: Decide>(
    body: &mut Body<T>,
    face: FaceKey,
) -> Vec<(HalfEdgeKey, PcurveCache<T>)> {
    let Some(face_data) = body.get_face(face) else {
        return Vec::new();
    };
    let loops: Vec<LoopKey> = core::iter::once(face_data.outer)
        .chain(face_data.rings.iter().copied())
        .collect();
    let mut hes: Vec<HalfEdgeKey> = Vec::new();
    for lk in loops {
        let Some(lp) = body.get_loop(lk) else {
            continue;
        };
        let crate::entity::LoopBoundary::Cycle { first } = lp.boundary else {
            continue;
        };
        let Some(cycle) = body.loop_cycle(first) else {
            continue;
        };
        hes.extend(cycle);
    }
    hes.into_iter()
        .filter_map(|he| body.pcurves.remove(he).map(|row| (he, row)))
        .collect()
}

/// Mints the caches of one face: [`derive_face`], stored.
fn mint_face<T: AtRestPolicy>(
    body: &mut Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<(), PcurveMintError> {
    let rows = derive_face(body, face, band)?;
    for (half_edge, cache) in rows {
        body.pcurves.insert(half_edge, cache);
    }
    Ok(())
}

/// **Whether a face's refusal means its rows are not owed**: the one
/// reading, shared by the minting pass ([`mint_faces`]), which leaves
/// such a face storing nothing, and the at-rest pass
/// ([`validate_pcurves`]), which reports nothing about it. Two arms, and
/// only these:
///
/// - **an uncovered class** — a pair the chart can hold but no lane
///   covers yet ([`PcurveCertifyError::UnsupportedCarrier`], whose
///   `class` names it); each class leaves this arm in the change that
///   wires its route. The sphere's general circle has left it: its
///   route is the fitted lane ([`analytic_derive`]), so that class
///   reaching here would be a derivation that skipped its route;
/// - **a scalar with no fitted door** ([`AtRestPolicy::fitted_lane`]
///   answers `None`, as a dual's does) refusing a face only the fitted
///   lane can image ([`PcurveCertifyError::FittedLaneUnsupported`]):
///   that scalar certifies nothing fitted, so it owes no fitted row. The
///   rule is the scalar's, not a class's — at every scalar that holds
///   the door, a fitted refusal is a finding.
fn not_owed<T: AtRestPolicy>(e: &PcurveMintError) -> bool {
    match e {
        PcurveMintError::Certify { error, .. } => match error {
            PcurveCertifyError::UnsupportedCarrier { class, .. } => {
                *class != UncoveredClass::SphereGeneralCircle
            }
            PcurveCertifyError::FittedLaneUnsupported { .. } => T::fitted_lane().is_none(),
            _ => false,
        },
        _ => false,
    }
}

/// **The minting pass over one face, storing nothing** (module docs:
/// the two-pass shape — walk the loops to pin branches, then certify
/// every pcurve against its carrier and chart, [`certify_walked`]). [`mint_face`] stores what this derives;
/// [`validate_pcurves`] reads a face that does not store its rows
/// through it, so "what the mint would write here" has one answer.
///
/// A face on a chart that mints nothing ([`DescribedChart::minting`])
/// derives no row.
///
/// # Errors
///
/// The face's refusal ([`first_owed`]), read whole: the walk's
/// ([`walk_loop`]) or a certification's.
fn derive_face<T: AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<Certified<T, HalfEdgeKey>, PcurveMintError> {
    let face_data = body.get_face(face).ok_or(PcurveMintError::Corrupt)?;
    let surface = body
        .get_surface(face_data.surface)
        .ok_or(PcurveMintError::Corrupt)?;
    let Some(chart) = DescribedChart::minting(surface) else {
        return Ok(Vec::new());
    };
    let loops: Vec<LoopKey> = core::iter::once(face_data.outer)
        .chain(face_data.rings.iter().copied())
        .collect();
    let mut walked: Vec<Walked<T>> = Vec::new();
    let mut refused: Vec<PcurveMintError> = Vec::new();
    let mut broken: Vec<LoopKey> = Vec::new();
    for &lp in &loops {
        if let Err(e) = walk_loop(body, face, lp, chart, band, &mut walked) {
            refused.push(e);
            broken.push(lp);
        }
    }
    // U2's `General` arm certifies at the FITTED grade: the same four
    // checks in the same order, but check 4 is the full C2 certificate
    // against the operand PAIR, so it needs the mate the edge's own
    // description names (D2 — read from the description, never from the
    // topology, and re-read from the body rather than stored). The
    // certificate is the fitted door's, read off the scalar's policy,
    // and the image is not always one this pass derived: the own-chart
    // arm of `nurbs_iso_derive` hands a construction's STATED `General`
    // image here at every scalar, a dual included. So the door goes in
    // as the policy answers it, `None` and all, and an absent one
    // refuses at check 4 — an image that fails checks 1–3 draws those
    // checks' verdict at every scalar.
    //
    // A `Fitted` image is one this pass derived (a sphere's general
    // circle, `analytic_derive`), which it did only through the fitted
    // door, so the door is in hand; `certify_fitted` takes it bare.
    let fitted = |w: &Walked<T>| match &w.pcurve {
        Pcurve::General(image) => PcurveCache::certify_general(
            std::sync::Arc::clone(image),
            w.t0,
            w.t1,
            &w.carrier,
            surface,
            mate_surface(body, w.key).as_ref(),
            band,
            T::fitted_lane(),
        ),
        Pcurve::Fitted(image) => {
            let lane = T::fitted_lane()
                .ok_or(PcurveCertifyError::FittedLaneUnsupported { scalar: T::NAME })?;
            PcurveCache::certify_fitted(
                std::sync::Arc::clone(image),
                w.t0,
                w.t1,
                &w.carrier,
                surface,
                mate_surface(body, w.key).as_ref(),
                band,
                lane,
            )
        }
        closed => unreachable!(
            "certify_walked hands the fitted door only Fitted and General images: {closed:?}"
        ),
    };
    if !refused.is_empty() {
        // A walk stops at the first refusal of its loop, so an edge
        // whose rows are not owed would hide every edge after it: the
        // face is read whole before it is excused ([`not_owed`]). Whole
        // means the mint's own reading of every edge it could image —
        // each run of covered edges a broken loop leaves walked through
        // its joints ([`walk_runs`]), and every image derived, on those
        // runs and on the loops that walked, certified through the
        // mint's doors — so a
        // covered edge's refusal, a joint's or a certificate's, is the
        // face's whichever uncovered edge shares the face.
        if refused.iter().all(not_owed::<T>) {
            for lp in broken {
                if let LoopRows::Cycle(cycle) = loop_rows(body, lp) {
                    walk_runs(body, chart, &cycle, band, &mut walked, &mut refused);
                }
            }
            if let Err(certs) = certify_walked(walked, surface, band, Some(&fitted)) {
                refused.extend(
                    certs
                        .into_iter()
                        .map(|(half_edge, error)| PcurveMintError::Certify { half_edge, error }),
                );
            }
        }
        return Err(first_owed::<T>(refused));
    }
    certify_walked(walked, surface, band, Some(&fitted)).map_err(|refused| {
        first_owed::<T>(
            refused
                .into_iter()
                .map(|(half_edge, error)| PcurveMintError::Certify { half_edge, error })
                .collect(),
        )
    })
}

/// The refusal a face reports: its first whose rows are owed, or — when
/// none is ([`not_owed`]) — its first, which excuses the face. So a
/// face is excused only when every refusal it meets is, whichever edge
/// the walk meets first.
fn first_owed<T: AtRestPolicy>(refused: Vec<PcurveMintError>) -> PcurveMintError {
    let at = refused.iter().position(|e| !not_owed::<T>(e)).unwrap_or(0);
    refused
        .into_iter()
        .nth(at)
        .unwrap_or(PcurveMintError::Corrupt)
}

/// The rows [`certify_walked`] certified, in walk order.
type Certified<T, K> = Vec<(K, PcurveCache<T>)>;

/// A fitted-grade certifier for a `Fitted` or `General` image
/// ([`certify_walked`]).
type FittedDoor<'a, T, K> = &'a dyn Fn(&Walked<T, K>) -> Result<PcurveCache<T>, PcurveCertifyError>;

/// **Pass 2 of the minting walk, for one face**: every image the walk
/// derived certified against its carrier and chart, in walk order. The
/// one home of that pass, shared by [`mint_face`] (the minting pass)
/// and [`site_rows`] (the Euler operators' site mint), so the rows the
/// two write for one walk are one set of bits. A row's certificate
/// reads that row alone; the loop's facts (its branches and winding)
/// are the walk's ([`walk_cycle`]).
///
/// Every image goes through the `Decide`-scalar door,
/// [`PcurveCache::certify`], except a `Fitted` or `General` one, which
/// goes to `fitted` — the fitted door, which only the pass holds.
/// Without it such an image meets the closed-form door, which refuses
/// it.
///
/// # Errors
///
/// Every half-edge whose certification refused, with its refusal, in
/// walk order.
fn certify_walked<T: Decide, K: Copy>(
    walked: Vec<Walked<T, K>>,
    surface: &Surface<T>,
    band: Band,
    fitted: Option<FittedDoor<'_, T, K>>,
) -> Result<Certified<T, K>, Vec<(K, PcurveCertifyError)>> {
    let mut rows = Vec::with_capacity(walked.len());
    let mut refused = Vec::new();
    for w in walked {
        let cache = match (&w.pcurve, fitted) {
            (Pcurve::Fitted(_) | Pcurve::General(_), Some(fitted)) => fitted(&w),
            _ => PcurveCache::certify(w.pcurve.clone(), w.t0, w.t1, &w.carrier, surface, band),
        };
        match cache {
            Ok(cache) => rows.push((w.key, cache)),
            Err(error) => refused.push((w.key, error)),
        }
    }
    if refused.is_empty() {
        Ok(rows)
    } else {
        Err(refused)
    }
}

/// One half-edge of a face as a site mint's plan phase names it,
/// before its door mutates: an Euler operator's two new half-edges have
/// no key yet, and a null edge's two halves have no carrier yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SiteHalf {
    /// A half-edge the body already holds, under its edge's carrier.
    Existing(HalfEdgeKey),
    /// The new edge's `he_plus`, which the surgery mints.
    NewPlus,
    /// The new edge's `he_minus`, which the surgery mints.
    NewMinus,
    /// A half-edge the body already holds, of the edge being described:
    /// the walk reads it under the carrier the description installs.
    Described(HalfEdgeKey),
}

/// Why a site mint refused to write a face's rows — [`site_rows`]'
/// refusal, raised before its door mutates anything, so the body is
/// untouched.
#[derive(Clone, Debug, PartialEq)]
pub enum SiteRowRefusal {
    /// **The fitted frontier.** The face is on a SPLINE chart (a
    /// described NURBS or an approximating surface), where a chart
    /// image derives from the edge's description through the iso and
    /// general lanes (`nurbs_iso_derive`), which read their fitted
    /// door through [`crate::AtRestPolicy`]. The Euler operators are
    /// generic over `Decide`, so they refuse here rather than leave a
    /// COMPLETE face half-minted. A face a null edge holds open is
    /// already incomplete, and is left as found instead
    /// ([`site_rows`]).
    SplineChart,
    /// A half-edge of a face the site mint reads did not resolve: to an
    /// edge and a curve, where it asks whether a null edge holds the
    /// loop open ([`held_open`]), or to a carrier and a direction, where
    /// it walks the loop — tier 1's corruption.
    Corrupt,
    /// **A keys-only door owes the face a row it holds no band to
    /// derive.** The door moves a loop or run onto a face whose rows
    /// are complete on an analytic chart, and the moved rows do not
    /// stand there — stated in another chart, or missing — so the face
    /// would leave half-minted. The keys-only kills and moves
    /// (`kef`, `kfmrh`, `ring_move`, `mfkrh`) take no band and refuse
    /// here, before mutating; their `_minting` siblings take one and
    /// re-mint the face ([`site_rows_owed`]).
    KeysOnly,
}

impl core::fmt::Display for SiteRowRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SplineChart => write!(
                f,
                "the face is on a spline chart, where a new edge's pcurve row derives only \
                 through the fitted lane, which the Euler operators do not carry. Recourse: \
                 run the operator before the face is minted, and mint once the surgery is done"
            ),
            Self::Corrupt => write!(
                f,
                "a key the face's records hold does not resolve, so the body is torn. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::KeysOnly => write!(
                f,
                "the loop or run this door moves lands on a face whose pcurve rows are \
                 complete, and its own rows do not stand there, so the face would be left \
                 half-minted; this door takes no band to re-mint it. Recourse: call the \
                 door's `_minting` sibling with the run's tolerance, or move the loop \
                 before the face is minted"
            ),
        }
    }
}

impl std::error::Error for SiteRowRefusal {}

/// One loop of a face after a site mint's door, as its plan phase
/// knows it before mutating.
pub(crate) enum SiteLoop {
    /// A loop the surgery rewires: its half-edges in `next` order from
    /// the loop's `first` AFTER the surgery — the order the minting
    /// walk reads it in.
    Rewired(Vec<SiteHalf>),
    /// A loop of the face the surgery leaves as it is.
    Kept(LoopKey),
}

/// A face a site mint re-mints — one an Euler operator's new
/// half-edges land on, one a null edge's halves are on at its
/// description, or one a door moves a loop or run onto — described as
/// its door leaves it.
pub(crate) struct SiteFace<T: Real> {
    /// The face whose rows decide whether this one is minted: the face
    /// itself, or — for a face a door makes (`mef`'s new face,
    /// `mfkrh`'s) — the face it is carved or promoted from. Named by
    /// the refusal.
    pub(crate) rows_from: FaceKey,
    /// The chart the face is on after the surgery.
    pub(crate) surface: Surface<T>,
    /// Whether a door moves a loop or run onto this face whose rows do
    /// not stand on it: stated in another chart, or missing
    /// ([`crate::Body::drop_rows`]). Such a loop is a rewired loop like
    /// any other, and on an analytic chart it is minted like one; on a
    /// spline chart the face is left as found rather than refused
    /// ([`site_rows`] says why).
    pub(crate) moved: bool,
    /// Its loops after the surgery, outer first.
    pub(crate) loops: Vec<SiteLoop>,
}

/// What a site mint writes into the pcurve map for one face once its
/// door has mutated ([`apply_site_rows`]).
pub(crate) enum SiteRows<T: Real> {
    /// The face is not re-minted — [`StoredRows::remints`] did not
    /// select it, its chart mints nothing, it is on a spline chart with
    /// a null edge holding it open or a loop or run moved onto it, or
    /// every loop the door rewires still runs through a null edge — and
    /// the door leaves its rows exactly as found.
    Leave,
    /// Every row of the loops the door rewires that run through no null
    /// edge, its two halves' among them. A loop the door keeps keeps
    /// its rows, and a rewired loop a null edge still holds open keeps
    /// the rows it has.
    Mint(Certified<T, SiteHalf>),
    /// The face as the surgery leaves it has no closed-form row set
    /// that certifies, so it stores nothing ([`site_rows`] says when).
    /// Every half-edge of the face after the surgery.
    Clear(Vec<SiteHalf>),
}

/// A face as a site mint found it, and read further
/// ([`site_rows_from`]).
pub(crate) struct SiteFrom {
    /// Its rows as found.
    pub(crate) rows: StoredRows,
    /// Its loops a null edge holds open ([`StoredRows::open_loops`]).
    pub(crate) open: Vec<LoopKey>,
}

/// `face` as found, when a site mint's door could re-mint it: its
/// chart mints ([`chart_mints`]) and [`StoredRows::remints`] selects it
/// were the door to release every loop a null edge holds open, the most
/// a door can do. `None` for every other face, which the door leaves as
/// found; [`site_rows`] asks the predicate again with what the door
/// does.
///
/// The chart is read first, so a face on a chart that mints nothing
/// costs no walk.
///
/// # Errors
///
/// [`held_open`]'s.
pub(crate) fn site_rows_from<T: Decide>(
    body: &Body<T>,
    face: &crate::entity::Face,
    surface: &Surface<T>,
) -> Result<Option<SiteFrom>, SiteRowRefusal> {
    if DescribedChart::minting(surface).is_none() {
        return Ok(None);
    }
    let rows = stored_rows(body, face);
    let open = rows.open_loops(body)?;
    Ok(rows
        .remints(&open, !open.is_empty())
        .then_some(SiteFrom { rows, open }))
}

/// **The loops a site mint walks on `face`**, band-free: the part of
/// [`site_rows`] that decides before it derives, and the one reading
/// of it. Empty where the face is left as found ([`SiteRows::Leave`]):
/// its chart mints nothing, [`StoredRows::remints`] does not select it,
/// or every loop the door rewires still runs through a null edge.
/// Otherwise each rewired loop no null edge holds open as the door
/// leaves it, in `face`'s loop order; [`site_rows`] walks exactly
/// these and writes the face — rows, or a clear.
///
/// # Errors
///
/// [`SiteRowRefusal::SplineChart`] on a complete spline face a door
/// adds half-edges to, and [`held_open`]'s.
fn site_walks<'a, T: Decide>(
    body: &Body<T>,
    chart: DescribedChart<'_, T>,
    face: &'a SiteFace<T>,
    from: &SiteFrom,
) -> Result<Vec<&'a [SiteHalf]>, SiteRowRefusal> {
    // A spline chart's rows derive through the fitted lane, which a
    // `Decide` door does not hold, so no loop of it is minted here and
    // the question is per face, not per loop: refuse, or leave as
    // found. A face a null edge holds open anywhere is left as found,
    // and an operator on one of its complete loops leaves that loop's
    // new halves rowless too, by intent: the face is already
    // incomplete, and a refusal would strand the pipeline mid-surgery
    // with its null edge, which tier 2 refuses at rest. A face a door
    // moves a loop or run onto is left as found too, its moved rows
    // dropped: the doors that move a loop are the ones that fuse and
    // merge bodies that arrive minted (a boolean's seam zip, the merge
    // door, a blend's kills), which have no "move before minting" to
    // take as a refusal's recourse. A complete face the door adds
    // half-edges to refuses rather than go half-minted.
    if chart.surface().spline_chart().is_some() {
        return if from.open.is_empty() && !face.moved {
            Err(SiteRowRefusal::SplineChart)
        } else {
            Ok(Vec::new())
        };
    }
    let mut open_after = Vec::with_capacity(face.loops.len());
    for lp in &face.loops {
        open_after.push(match lp {
            SiteLoop::Rewired(halves) => held_open(
                body,
                halves.iter().filter_map(|&at| match at {
                    SiteHalf::Existing(he) => Some(he),
                    SiteHalf::NewPlus | SiteHalf::NewMinus | SiteHalf::Described(_) => None,
                }),
            )?,
            SiteLoop::Kept(key) => from.open.contains(key),
        });
    }
    let released = !from.open.is_empty() && !open_after.contains(&true);
    if !from.rows.remints(&from.open, released) {
        return Ok(Vec::new());
    }
    Ok(face
        .loops
        .iter()
        .zip(open_after)
        .filter_map(|(lp, open)| match lp {
            SiteLoop::Rewired(halves) if !open && !halves.is_empty() => Some(halves.as_slice()),
            SiteLoop::Rewired(_) | SiteLoop::Kept(_) => None,
        })
        .collect())
}

/// **Whether a site mint writes `face`**: [`site_walks`] walks a loop
/// of it. Band-free, for a keys-only door, which refuses
/// [`SiteRowRefusal::KeysOnly`] exactly where its `_minting` twin's
/// [`site_rows`] would write the face.
///
/// # Errors
///
/// [`site_walks`]'.
pub(crate) fn site_rows_owed<T: Decide>(
    body: &Body<T>,
    face: &SiteFace<T>,
    from: &SiteFrom,
) -> Result<bool, SiteRowRefusal> {
    let Some(chart) = DescribedChart::minting(&face.surface) else {
        return Ok(false);
    };
    Ok(!site_walks(body, chart, face, from)?.is_empty())
}

/// **The rows a site mint writes onto one face**, derived before its
/// door mutates: a face an Euler operator adds half-edges to, one a
/// null edge's halves are on at its first description
/// ([`crate::Body::set_edge_curve`]), which re-walks every loop of the
/// face, or one a door moves a loop or run onto whose rows do not
/// stand there ([`crate::Body::drop_rows`] names the doors). `from` is the face as found, on a face [`site_rows_from`]
/// read further; every other face is left as found, and never reaches
/// here. The face is re-minted where [`StoredRows::remints`] selects
/// it, `released` being whether the door takes the last null edge off
/// it: the face as found runs through one and the face as the door
/// leaves it through none.
///
/// - **The loops the surgery rewires are re-minted whole**, exactly as
///   [`mint_pcurves_of`] would re-mint them after the surgery: the same
///   walk from each loop's `first` ([`walk_cycle`]) and the same
///   certification ([`certify_walked`]). A loop the surgery keeps keeps
///   its rows: its walk reads only its own half-edges, their carriers
///   and the chart, none of which the surgery touches. So on a
///   face whose stored rows are the pass's, the rows after the op are
///   the pass's, byte for byte.
/// - **A rewired loop that still runs through a null edge keeps the
///   rows it has** ([`held_open`]): the null halves have no carrier,
///   so the loop cannot be walked, and its new halves go rowless until
///   the null edge leaves it. Every rewired loop the door leaves
///   running through no null edge is minted whole, the rows it missed
///   while it was held open among them.
/// - **A moved loop or run is a rewired loop** ([`SiteFace::moved`]):
///   its rows are stated in the chart it left, or missing, so it is
///   walked whole in the destination's chart, exactly as the minting
///   pass would walk it there. Which face's rows decide is the door's
///   (`rows_from`): the destination as found where one receives the
///   loop, and the face a new one is carved or promoted from.
/// - **On a spline chart the site mint refuses a COMPLETE face**
///   ([`SiteRowRefusal::SplineChart`]) and leaves a face a null edge
///   holds open anywhere, or one a door moves a loop or run onto, as
///   found (the arm below says why per face).
///   The mint of an ANALYTIC chart
///   needs nothing the fitted lane holds — [`chart_pcurve`],
///   [`walk_cycle`] and [`PcurveCache::certify`] are all `Decide` — and
///   the fitted lane is the spline chart's derivation
///   (`nurbs_iso_derive`) and U2's `General` certificate.
/// - **Where the face as the surgery leaves it has no closed-form row
///   set, it stores nothing** ([`SiteRows::Clear`]): a carrier outside
///   the chart's closed-form classes, a certification that refuses, a
///   branch that meets no neighbour, a loop that does not close. The
///   face is unminted, never half-minted. Where the minting pass would
///   RAISE on that face, the operator does not: it runs mid-surgery, on
///   states a later door finishes describing (C4: a door may drop rows
///   mid-surgery), and a producer's final pass is where such a face is
///   minted or refused. A body left at rest without that pass reads
///   loud at tier 3, which re-derives the rowless face and reports the
///   refusal the mint would raise ([`validate_pcurves`]).
///
/// **Cost.** One walk and one certification per half-edge of the loops
/// the surgery rewires, and one presence read per half-edge of the rest
/// of the face ([`site_rows_from`]); on a face missing a row, three
/// lookups more per half-edge of the face as found — its half, edge and
/// curve — for which loops a null edge holds open, and as many again
/// per half-edge of the rewired loops. So N operators on one minted
/// face whose loop grows with each cost O(N²)
/// (`work/topo/euler-site-mint-re-walks-the-rewired-loop-on-every-op`).
///
/// # Errors
///
/// [`SiteRowRefusal`]: the spline frontier, or a half-edge that did not
/// resolve.
pub(crate) fn site_rows<T: Decide>(
    body: &Body<T>,
    face: &SiteFace<T>,
    from: &SiteFrom,
    edge: Option<&geom_brep::EdgeCurve<T>>,
    band: Band,
) -> Result<SiteRows<T>, SiteRowRefusal> {
    let Some(chart) = DescribedChart::minting(&face.surface) else {
        return Ok(SiteRows::Leave);
    };
    let walks = site_walks(body, chart, face, from)?;
    if walks.is_empty() {
        return Ok(SiteRows::Leave);
    }
    let kept = |key: LoopKey| {
        from.rows
            .loops
            .iter()
            .find(|(lk, _)| *lk == key)
            .and_then(|(_, cycle)| cycle.as_deref())
            .unwrap_or_default()
            .iter()
            .copied()
            .map(SiteHalf::Existing)
    };
    let every_half = || -> Vec<SiteHalf> {
        face.loops
            .iter()
            .flat_map(|lp| -> Vec<SiteHalf> {
                match lp {
                    SiteLoop::Rewired(halves) => halves.clone(),
                    SiteLoop::Kept(key) => kept(*key).collect(),
                }
            })
            .collect()
    };
    fn named<T: Real>(edge: Option<&geom_brep::EdgeCurve<T>>) -> &geom_brep::EdgeCurve<T> {
        edge.unwrap_or_else(|| {
            unreachable!(
                "site_rows: a plan names a new or described half only beside the edge that \
                 carries it; a door that moves a loop names existing halves alone"
            )
        })
    }
    // A half the body holds enters at its start vertex. A new half's
    // vertex may not exist before the surgery, so it enters at its
    // carrier's end, which the edge certificate pins to the vertex the
    // surgery makes (`carrier_endpoint_start` / `_end`).
    let traversal = |at: SiteHalf| -> Result<SiteTraversal<T>, ItemFail> {
        let held = |he| entry_vertex(body, he).map_err(|_| ItemFail::Corrupt);
        match at {
            SiteHalf::Existing(he) => {
                let (carrier, t0, t1) =
                    half_edge_carrier(body, he).map_err(|_| ItemFail::Corrupt)?;
                let plus = is_plus(body, he).map_err(|_| ItemFail::Corrupt)?;
                Ok((carrier, t0, t1, plus, held(he)?))
            }
            SiteHalf::NewPlus | SiteHalf::NewMinus => {
                let edge = named(edge);
                let (t0, t1) = edge.params();
                let plus = at == SiteHalf::NewPlus;
                let carrier = edge.carrier().clone();
                let vertex = carrier.eval(if plus { t0 } else { t1 });
                Ok((carrier, t0, t1, plus, vertex))
            }
            SiteHalf::Described(he) => {
                let edge = named(edge);
                let (t0, t1) = edge.params();
                let plus = is_plus(body, he).map_err(|_| ItemFail::Corrupt)?;
                Ok((edge.carrier().clone(), t0, t1, plus, held(he)?))
            }
        }
    };
    let mut walked: Vec<Walked<T, SiteHalf>> = Vec::new();
    for halves in walks {
        let mut carriers: Vec<geom::Curve3<T>> = Vec::with_capacity(halves.len());
        let item = |i: usize| -> Result<WalkItem<T>, ItemFail> {
            let (carrier, t0, t1, plus, vertex) = traversal(halves[i])?;
            let base = chart_pcurve(&carrier, &face.surface, band).map_err(|_| ItemFail::Derive)?;
            carriers.push(carrier);
            Ok(WalkItem {
                base,
                t0,
                t1,
                plus,
                vertex,
            })
        };
        let pinned = match walk_cycle(chart, halves.len(), item, band, true) {
            Ok(pinned) => pinned,
            Err(WalkFail::Item(ItemFail::Corrupt)) => return Err(SiteRowRefusal::Corrupt),
            Err(WalkFail::Item(ItemFail::Derive) | WalkFail::Miss { .. } | WalkFail::NotClosed) => {
                return Ok(SiteRows::Clear(every_half()));
            }
        };
        walked.extend(halves.iter().zip(carriers).zip(pinned).map(
            |((&key, carrier), (pcurve, t0, t1))| Walked {
                key,
                carrier,
                pcurve,
                t0,
                t1,
            },
        ));
    }
    match certify_walked(walked, &face.surface, band, None) {
        Ok(rows) => Ok(SiteRows::Mint(rows)),
        Err(_) => Ok(SiteRows::Clear(every_half())),
    }
}

/// One half of a site mint's plan as its walk reads it: the carrier,
/// its interval, whether the loop runs it forward, and its entry
/// vertex's point.
type SiteTraversal<T> = (geom::Curve3<T>, T, T, bool, Point3<T>);

/// Why [`site_rows`] could not read or derive one half-edge of the
/// walk.
enum ItemFail {
    /// A key did not resolve: refused, [`SiteRowRefusal::Corrupt`].
    Corrupt,
    /// The closed-form derivation refused: the face is cleared.
    Derive,
}

/// Writes what [`site_rows`] decided, once its door has mutated.
/// `minted` is the `(he_plus, he_minus)` an Euler operator's surgery
/// minted for [`SiteHalf::NewPlus`] and [`SiteHalf::NewMinus`], and
/// `None` for a description, whose plan names none. Infallible: it
/// sits in the door's mutation phase and only writes the map.
pub(crate) fn apply_site_rows<T: Decide>(
    body: &mut Body<T>,
    plans: Vec<SiteRows<T>>,
    minted: Option<(HalfEdgeKey, HalfEdgeKey)>,
) {
    let key = |at: SiteHalf| match (at, minted) {
        (SiteHalf::Existing(he) | SiteHalf::Described(he), _) => he,
        (SiteHalf::NewPlus, Some((he_plus, _))) => he_plus,
        (SiteHalf::NewMinus, Some((_, he_minus))) => he_minus,
        (SiteHalf::NewPlus | SiteHalf::NewMinus, None) => unreachable!(
            "apply_site_rows: only an Euler operator's plan names a new half, and it passes \
             the halves its surgery minted"
        ),
    };
    for plan in plans {
        match plan {
            SiteRows::Leave => {}
            SiteRows::Mint(rows) => {
                for (at, cache) in rows {
                    body.pcurves.insert(key(at), cache);
                }
            }
            SiteRows::Clear(halves) => body.drop_rows(halves.into_iter().map(key)),
        }
    }
}

/// The one-branch walk of a single loop (module docs). Appends the
/// branch-pinned chart curves to `out`.
///
/// The derivation is the one step that forks by chart: a SPLINE chart
/// derives from the edge's description ([`nurbs_iso_derive`], the
/// fitted lane's), every analytic chart from the carrier's closed form
/// ([`chart_pcurve`]). Everything after the derivation — each joint's
/// deck element and the loop's winding — is [`walk_cycle`], which is
/// stated under `Decide`.
pub(crate) fn walk_loop<T: AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    lp: LoopKey,
    chart: DescribedChart<'_, T>,
    band: Band,
    out: &mut Vec<Walked<T>>,
) -> Result<(), PcurveMintError> {
    let surface = chart.surface();
    let loop_data = body.get_loop(lp).ok_or(PcurveMintError::Corrupt)?;
    let crate::entity::LoopBoundary::Cycle { first } = loop_data.boundary else {
        // An empty loop bounds nothing to chart.
        return Ok(());
    };
    let cycle = body.loop_cycle(first).ok_or(PcurveMintError::Corrupt)?;
    let mut carriers: Vec<geom::Curve3<T>> = Vec::with_capacity(cycle.len());
    let item = |i: usize| -> Result<WalkItem<T>, PcurveMintError> {
        let he = cycle[i];
        let (carrier, t0, t1, base) = derive_image(body, he, surface, band)?;
        let plus = is_plus(body, he)?;
        let vertex = entry_vertex(body, he)?;
        carriers.push(carrier);
        Ok(WalkItem {
            base,
            t0,
            t1,
            plus,
            vertex,
        })
    };
    let walked = walk_cycle(chart, cycle.len(), item, band, true).map_err(|fail| match fail {
        WalkFail::Item(e) => e,
        WalkFail::Miss {
            index,
            miss: PinMiss::Discontinuity,
        } => PcurveMintError::LoopDiscontinuity {
            half_edge: cycle[index],
        },
        WalkFail::Miss {
            index,
            miss: PinMiss::Escalated(cause),
        } => PcurveMintError::Escalated {
            half_edge: cycle[index],
            cause,
        },
        WalkFail::Miss {
            index,
            miss: PinMiss::OutOfReach,
        } => PcurveMintError::Certify {
            half_edge: cycle[index],
            error: PcurveCertifyError::BranchOutOfReach,
        },
        WalkFail::NotClosed => PcurveMintError::LoopNotClosed { face },
    })?;
    out.extend(cycle.iter().zip(carriers).zip(walked).map(
        |((&key, carrier), (pcurve, t0, t1))| Walked {
            key,
            carrier,
            pcurve,
            t0,
            t1,
        },
    ));
    Ok(())
}

/// **The runs of covered edges in a loop the walk could not close**:
/// the loop read as the mint reads it, minus the edges it cannot image.
/// Each half-edge's image is derived ([`derive_image`]), and every
/// maximal run of consecutive derived images — cyclically, so a run may
/// cross the loop's start — is walked as an open chain ([`walk_cycle`]
/// without the closure), its joints pinned and checked exactly as the
/// loop's own walk would. The pinned images join `out` for
/// certification; a derivation's refusal, a joint's or a key's joins
/// `refused`.
fn walk_runs<T: AtRestPolicy>(
    body: &Body<T>,
    chart: DescribedChart<'_, T>,
    cycle: &[HalfEdgeKey],
    band: Band,
    out: &mut Vec<Walked<T>>,
    refused: &mut Vec<PcurveMintError>,
) {
    let surface = chart.surface();
    type Derived<T> = (HalfEdgeKey, geom::Curve3<T>, WalkItem<T>);
    let mut items: Vec<Option<Derived<T>>> = cycle
        .iter()
        .map(|&he| {
            let derived =
                derive_image(body, he, surface, band).and_then(|(carrier, t0, t1, base)| {
                    let plus = is_plus(body, he)?;
                    let vertex = entry_vertex(body, he)?;
                    let item = WalkItem {
                        base,
                        t0,
                        t1,
                        plus,
                        vertex,
                    };
                    Ok((he, carrier, item))
                });
            derived.map_err(|e| refused.push(e)).ok()
        })
        .collect();
    let n = items.len();
    let start = items
        .iter()
        .position(Option::is_none)
        .map_or(0, |gap| (gap + 1) % n.max(1));
    let mut runs: Vec<Vec<Derived<T>>> = Vec::new();
    let mut run: Vec<Derived<T>> = Vec::new();
    for k in 0..n {
        match items[(start + k) % n].take() {
            Some(item) => run.push(item),
            None => runs.push(core::mem::take(&mut run)),
        }
    }
    runs.push(run);
    for run in runs.into_iter().filter(|run| !run.is_empty()) {
        let (keys, (carriers, mut walk_items)): (Vec<_>, (Vec<_>, Vec<_>)) = run
            .into_iter()
            .map(|(he, carrier, item)| (he, (carrier, Some(item))))
            .unzip();
        let item = |j: usize| walk_items[j].take().ok_or(PcurveMintError::Corrupt);
        match walk_cycle(chart, keys.len(), item, band, false) {
            Ok(pinned) => out.extend(keys.iter().zip(carriers).zip(pinned).map(
                |((&key, carrier), (pcurve, t0, t1))| Walked {
                    key,
                    carrier,
                    pcurve,
                    t0,
                    t1,
                },
            )),
            Err(WalkFail::Miss {
                index,
                miss: PinMiss::Discontinuity,
            }) => refused.push(PcurveMintError::LoopDiscontinuity {
                half_edge: keys[index],
            }),
            Err(WalkFail::Miss {
                index,
                miss: PinMiss::Escalated(cause),
            }) => refused.push(PcurveMintError::Escalated {
                half_edge: keys[index],
                cause,
            }),
            Err(WalkFail::Miss {
                index,
                miss: PinMiss::OutOfReach,
            }) => refused.push(PcurveMintError::Certify {
                half_edge: keys[index],
                error: PcurveCertifyError::BranchOutOfReach,
            }),
            Err(WalkFail::Item(e)) => refused.push(e),
            Err(WalkFail::NotClosed) => unreachable!("an open run checks no closure"),
        }
    }
}

/// **One half-edge's chart image on the chart's principal branch**,
/// with its carrier and interval — the derivation the walk pins
/// ([`walk_loop`]). The one step that forks by chart: a SPLINE chart
/// derives from the edge's description ([`nurbs_iso_derive`]), every
/// analytic chart from the carrier's closed form ([`chart_pcurve`]).
///
/// # Errors
///
/// A key that did not resolve, or the derivation's refusal.
fn derive_image<T: AtRestPolicy>(
    body: &Body<T>,
    he: HalfEdgeKey,
    surface: &Surface<T>,
    band: Band,
) -> Result<(geom::Curve3<T>, T, T, Pcurve<T>), PcurveMintError> {
    let (carrier, t0, t1) = half_edge_carrier(body, he)?;
    let base = if surface.spline_chart().is_some() {
        // Spline charts derive from the edge's intensional
        // description (M6-3) — see `nurbs_iso_derive`.
        nurbs_iso_derive(body, he, surface, band)?
    } else {
        analytic_derive(&carrier, t0, t1, surface, band, he)?
    };
    Ok((carrier, t0, t1, base))
}

/// One half-edge of a cycle as [`walk_cycle`] reads it: its chart
/// image on the chart's principal branch, its carrier interval,
/// whether the loop traverses it forward, and the 3-D point of the
/// vertex it enters at — the joint [`lift_joint`] reads the chart's
/// singular set and the azimuth's lever on.
pub(crate) struct WalkItem<T: Real> {
    base: Pcurve<T>,
    t0: T,
    t1: T,
    plus: bool,
    vertex: geom_core::Point3<T>,
}

/// One image [`walk_cycle`] placed: the branch-pinned chart curve with
/// its carrier interval.
type Pinned<T> = (Pcurve<T>, T, T);

/// Why [`lift_joint`] decided no deck element for a joint.
pub(crate) enum PinMiss {
    /// The image meets the predecessor's exit on no lift: its gap sits
    /// on a mark, or a spline chart's gap is definite.
    Discontinuity,
    /// A decision escalated.
    Escalated(Indeterminate),
    /// The gap is more than [`geom_brep::MAX_BRANCH_PERIODS`] steps of
    /// the orbit from the predecessor's exit.
    OutOfReach,
}

/// Why [`walk_cycle`] refused, with the index of the item it refused
/// at where there is one.
pub(crate) enum WalkFail<E> {
    /// The caller's item producer refused (a key that did not resolve,
    /// a derivation that refused).
    Item(E),
    /// The branch pin placed item `index` nowhere.
    Miss {
        /// The cycle position of the item.
        index: usize,
        /// Why.
        miss: PinMiss,
    },
    /// The walk did not return to its start with a winding of 0 or ±1
    /// period ([`loop_closes`]).
    NotClosed,
}

/// **The one-branch walk, stated under `Decide`.** Walks `len` items in
/// cycle order, asking `item` for each in turn (so an item's refusal
/// is reported in cycle order, before any later item is read), pins
/// each image's entry to its predecessor's exit ([`lift_joint`]), and
/// checks the closure ([`loop_closes`]) when `closes` — an open run of
/// a loop ([`walk_runs`]) has no closure to check. Returns each item's
/// pinned image with its interval.
///
/// This is the whole of the minting walk except the derivation, and
/// none of it needs the fitted lane: both of its callers —
/// [`walk_loop`], the pass's, and [`site_rows`], the Euler operators'
/// — hand it their own derivation through `item`.
fn walk_cycle<T: Decide, E>(
    chart: DescribedChart<'_, T>,
    len: usize,
    mut item: impl FnMut(usize) -> Result<WalkItem<T>, E>,
    band: Band,
    closes: bool,
) -> Result<Vec<Pinned<T>>, WalkFail<E>> {
    // The chart's own u period (`chart_u_period`): `τ` on an analytic
    // azimuth chart, the knot-domain length on a NURBS chart closed in
    // u, and NO shift at all on a chart that does not wrap.
    let u_period = chart_u_period(chart.surface(), band);
    // The walk's running exit point, in chart coordinates.
    let mut prev_exit: Option<geom_core::Point2<T>> = None;
    // The first row as pinned, its entry parameter and entry vertex:
    // the closing joint's far side.
    let mut first: Option<(Pcurve<T>, T, geom_core::Point3<T>)> = None;
    let mut out = Vec::with_capacity(len);
    for index in 0..len {
        let WalkItem {
            base,
            t0,
            t1,
            plus,
            vertex,
        } = item(index).map_err(WalkFail::Item)?;
        let (entry_t, exit_t) = if plus { (t0, t1) } else { (t1, t0) };
        let pcurve = match prev_exit {
            None => base,
            Some(prev) => lift_joint(chart, base, entry_t, prev, vertex, u_period, band)
                .map(|lift| lift.pcurve)
                .map_err(|miss| WalkFail::Miss { index, miss })?,
        };
        if first.is_none() {
            first = Some((pcurve.clone(), entry_t, vertex));
        }
        prev_exit = Some(pcurve.eval(exit_t));
        out.push((pcurve, t0, t1));
    }
    // Closure: the walk returns to its start with a winding of 0 (an
    // ordinary loop) or one full period around the chart (a loop that
    // wraps the periodic chart — the seam case, where the seam edge's
    // two half-edges take the two branches).
    if closes
        && let (Some((row, entry_t, vertex)), Some(end)) = (&first, prev_exit)
        && !loop_closes(chart, row, *entry_t, *vertex, end, u_period, band)
    {
        return Err(WalkFail::NotClosed);
    }
    Ok(out)
}

/// Where a joint's vertex sits against the chart's **singular set**:
/// the points where the first channel stops naming a point, so a joint
/// there has no azimuth and no first-channel lift to decide. Read by
/// [`singular_at`].
#[derive(Clone, Copy, Debug)]
enum Singular {
    /// Definitely off the set, or the chart has none.
    Off,
    /// On it.
    On,
    /// Undecided at this width.
    Undecided(Indeterminate),
}

/// **Whether a joint's vertex lies on the chart's singular set**,
/// decided as 3-D incidence of the vertex on that set — a sphere's
/// poles `c ± r·axis`, a cone's apex — so the distance decided is
/// polynomial in the vertex and the chart's data, never a chart
/// coordinate read back through a lever. A cylinder, a plane and a ring
/// torus (horn and spindle tori are refused at the constructor) have no
/// singular set, and decide nothing.
///
/// A spline chart keeps the net-level form: its whole `u` stretch
/// (`sup |S_u|`, [`chart_u_arm`]) under the band reads as singular
/// everywhere. That is a property of the net, not of the vertex, and
/// the chart-space gap that spline joints still state
/// ([`spline_gap_closes`]) is what keeps its skip safe.
fn singular_at<T: Decide>(
    chart: DescribedChart<'_, T>,
    vertex: geom_core::Point3<T>,
    band: Band,
) -> Singular {
    let reach = match *chart.surface() {
        Surface::Sphere {
            center,
            radius,
            axis,
            ..
        } => {
            let pole = axis * radius;
            vertex
                .distance(center + pole)
                .min(vertex.distance(center - pole))
        }
        Surface::Cone { apex, .. } => vertex.distance(apex),
        Surface::Cylinder { .. } | Surface::Torus { .. } | Surface::Plane { .. } => {
            return Singular::Off;
        }
        Surface::Nurbs(_) | Surface::Approx(_) => chart_u_arm(chart, T::zero()).magnitude(),
    };
    match decide("pcurve_loop_pole_joint", Margin::of(reach), band) {
        Ok(Sign::Zero) => Singular::On,
        Ok(Sign::Positive | Sign::Negative) => Singular::Off,
        Err(cause) => Singular::Undecided(cause),
    }
}

/// The first channel's lever **at a joint**, read off the joint's
/// vertex: on an azimuth chart, the vertex's distance from the chart's
/// axis — cylinder `r`, sphere `r·|cos v|`, torus `R + r·cos v`, cone
/// `|v·sin α|`, each at the vertex, but computed from the vertex
/// point rather than from a stored latitude. On a plane or a spline
/// chart the first channel is not an angle, and the arm is
/// [`chart_u_arm`]'s rate.
fn joint_arm<T: Real>(chart: DescribedChart<'_, T>, vertex: geom_core::Point3<T>) -> ChartArm<T> {
    let off_axis = |origin: geom_core::Point3<T>, axis: geom_core::Vec3<T>| {
        let w = vertex - origin;
        (w - axis * w.dot(axis)).norm()
    };
    match *chart.surface() {
        Surface::Cylinder { radius, .. } => ChartArm::Angular(radius),
        Surface::Sphere { center, axis, .. } | Surface::Torus { center, axis, .. } => {
            ChartArm::Angular(off_axis(center, axis))
        }
        Surface::Cone { apex, axis, .. } => ChartArm::Angular(off_axis(apex, axis)),
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
            chart_u_arm(chart, T::zero())
        }
    }
}

/// One joint's **deck element**, as [`lift_joint`] decided it: which
/// representation of the image (`twin`: a sphere's involution twin),
/// the whole periods `ku`, `kv` that land its entry on the
/// predecessor's exit, the image shifted by them, and what the joint's
/// vertex read against the chart's singular set.
struct Lift<T: Real> {
    pcurve: Pcurve<T>,
    twin: bool,
    ku: i32,
    kv: i32,
    singular: Singular,
}

impl<T: Real> Lift<T> {
    /// The deck element is the identity: the two rows already stand on
    /// one lift. A twin is never the identity, whatever its periods.
    fn identity(&self) -> bool {
        !self.twin && self.ku == 0 && self.kv == 0
    }

    /// **The windings a closed loop may have**, read at its closing
    /// joint ([`loop_closes`]): at most one whole period on each
    /// periodic channel, independently (a seam-closed wall's azimuth, a
    /// torus annulus's meridian, and a torus face whose boundary winds
    /// once both ways), and on a sphere the involution twin, alone or
    /// with at most one period of azimuth beyond it (a pole-crossing
    /// loop, which may also wind once about the axis). Counted in half
    /// turns of azimuth, `2·ku + twin`: at most 2 without the twin, at
    /// most 3 with it.
    fn closes(&self) -> bool {
        let half_turns = 2 * self.ku + i32::from(self.twin);
        let reach = if self.twin { 3 } else { 2 };
        half_turns.abs() <= reach && self.kv.abs() <= 1
    }
}

/// How [`lift_joint`] reads a joint's first channel: the chart × the
/// vertex's [`Singular`] reading, as one value.
#[derive(Clone, Copy, Debug)]
enum AzimuthRule {
    /// No azimuth integer: the chart has no period, the vertex is on the
    /// singular set (every azimuth names the point), or a spline chart's
    /// net-level gate did not read `Off` (its chart-space gap decides,
    /// [`spline_gap_closes`]).
    None,
    /// The integer, by the half-period marks.
    Marks,
    /// The integer by the marks; where they are undecided too, the
    /// unshifted gap at the vertex's lever ([`near_pole_gap_closes`]),
    /// and otherwise the incidence's escalation.
    MarksNearPole(Indeterminate),
}

impl AzimuthRule {
    fn of(periodic: bool, singular: Singular, spline: bool) -> Self {
        match (periodic, singular) {
            (false, _) | (true, Singular::On) => Self::None,
            (true, Singular::Undecided(_)) if spline => Self::None,
            (true, Singular::Off) => Self::Marks,
            (true, Singular::Undecided(cause)) => Self::MarksNearPole(cause),
        }
    }
}

/// **A joint's deck element, decided as integers.** Two chart points
/// with one 3-D image differ by a deck element of the chart: a whole
/// number of periods per periodic channel and, on a sphere, possibly
/// the involution `(u, v) ↦ (u + π, π − v)` ([`sphere_twin`]). This
/// decides which, for the image `base` entering at `entry_t` against
/// the predecessor's exit `prev`, by [`whole_periods`] on each channel,
/// the azimuth's marks metered at the vertex's own lever
/// ([`joint_arm`]).
///
/// **The margin of each decision.** On a cylinder, cone or torus the
/// orbit of a chart point steps a whole period in azimuth, so the
/// integer has half a period of room either side. On a sphere the orbit
/// steps half a period: the image and its twin sit at `kτ` and
/// `π + kτ` in azimuth. So the azimuth is decided once, as a whole
/// number `m` of HALF periods (marks at `±π/2`), and `m` names both the
/// sheet (`m` odd: the twin) and the period (`⌊m/2⌋`). Each orbit point
/// then has a quarter period of room, `(π/2)·d` in metres at the
/// vertex's distance `d` from the axis: half the separation of the two
/// nearest orbit points in the chart metric. It collapses only at the
/// poles, where the 3-D incidence decides instead. A margin that
/// small is a decided sign or an escalation, never a wrong sheet,
/// whatever the band's `K`.
///
/// **The joint's 3-D coincidence is not decided again here**, and needs
/// no chart margin: it follows from two certified bounds. Each row's
/// envelope bounds `|S(P(t)) − C(t)| ≤ ε` over its whole span (check 4,
/// `geom_brep::PcurveCache`), and the edge certificate pins each
/// carrier's ends to its vertices within ε (`carrier_endpoint_start` /
/// `_end`), so the two chart ends of a joint map within `4ε` of each
/// other. On the lift decided here (the image or its twin, `|Δu| ≤ π`),
/// `sin(x/2) ≥ x/π` turns that into `r·|Δu| ≤ 2π·ε` and `|Δv| ≤ 4ε` on
/// a cylinder of radius `r`; the sphere, cone and torus give the same
/// shape with the vertex's own lever away from their singular set. That
/// is a bound the chart polygon may read ([`chart_boundary`]), not a
/// decision.
///
/// **The first channel's rule** is [`AzimuthRule`]'s. On the singular
/// set no azimuth is decided: every azimuth names the point, and that
/// skip rests on the 3-D decision alone. Where the incidence is
/// UNDECIDED nothing is skipped on an analytic chart: the integer is
/// decided as at any other joint, which is sound either way (a deck
/// transformation moves no point), and where its marks cannot be
/// decided either, the unshifted gap must be decided within ε at the
/// vertex's own lever ([`near_pole_gap_closes`]). A spline chart has no
/// injectivity lemma (a net can fold, so a 3-D coincidence does not
/// name the sheet), and each of its joints states its chart-space gap
/// as well ([`spline_gap_closes`]), which is what keeps its net-level
/// skip safe.
fn lift_joint<T: Decide>(
    chart: DescribedChart<'_, T>,
    base: Pcurve<T>,
    entry_t: T,
    prev: geom_core::Point2<T>,
    vertex: geom_core::Point3<T>,
    u_period: Option<T>,
    band: Band,
) -> Result<Lift<T>, PinMiss> {
    let surface = chart.surface();
    let tau = T::tau();
    let spline = surface.spline_chart().is_some();
    let singular = singular_at(chart, vertex, band);
    let arm = joint_arm(chart, vertex);
    let whole = |k: i32| T::from_f64(f64::from(k));
    let twin = sphere_twin(surface, &base);
    // The orbit's azimuth step: half a period where the image has a
    // twin, a whole one elsewhere.
    let step = u_period.map(|p| {
        if twin.is_some() {
            p * T::from_f64(0.5)
        } else {
            p
        }
    });
    let gap = prev.x - base.eval(entry_t).x;
    let marks = |s: T| whole_periods("pcurve_loop_branch", gap, s, |g| arm.meter(g), band);
    let rule = AzimuthRule::of(step.is_some(), singular, spline);
    let m = match (rule, step) {
        (AzimuthRule::Marks, Some(s)) => marks(s).map_err(PinMiss::from)?,
        (AzimuthRule::MarksNearPole(cause), Some(s)) => match marks(s) {
            Err(BranchMiss::Undecided(_) | BranchMiss::OnMark) => {
                near_pole_gap_closes(arm, gap, s, twin.is_some(), band)
                    .ok_or(PinMiss::Escalated(cause))?
            }
            decided => decided.map_err(|_| PinMiss::Escalated(cause))?,
        },
        _ => 0,
    };
    let (is_twin, ku) = match twin {
        Some(_) => (m.rem_euclid(2) == 1, m.div_euclid(2)),
        None => (false, m),
    };
    let sheet = match twin {
        Some(t) if is_twin => t,
        _ => base,
    };
    let mut lifted = sheet.shift_branch(whole(ku), u_period.unwrap_or_else(T::zero));
    let mut kv = 0;
    if let Some(polar) = polar_arm(surface) {
        kv = whole_periods(
            "pcurve_loop_branch",
            prev.y - lifted.eval(entry_t).y,
            tau,
            |g| Margin::levered(g, polar),
            band,
        )
        .map_err(PinMiss::from)?;
        lifted = shift_polar_branch(&lifted, whole(kv), tau);
    }
    if spline {
        spline_gap_closes(chart, lifted.eval(entry_t), prev, band)?;
    }
    Ok(Lift {
        pcurve: lifted,
        twin: is_twin,
        ku,
        kv,
        singular,
    })
}

impl From<BranchMiss> for PinMiss {
    fn from(miss: BranchMiss) -> Self {
        match miss {
            BranchMiss::Undecided(cause) => Self::Escalated(cause),
            BranchMiss::OutOfReach => Self::OutOfReach,
            BranchMiss::OnMark => Self::Discontinuity,
        }
    }
}

/// The branch of a joint whose vertex's incidence on the singular set
/// is undecided, where the orbit's marks are undecided too: the vertex
/// sits within the band of a pole or an apex, so its own lever `arm` is
/// band-sized and no mark reads definitely. An orbit point is taken
/// only where the gap to it, metered at that lever, is decided within ε
/// (`pcurve_loop_pole_gap`): the image itself (`m = 0`) or, where the
/// image has a twin, the twin either way round (`m = ±1`, a `step` of
/// half a period off). The two chart points then agree in metres at the
/// vertex, and no branch is asserted that a margin did not see. `None`
/// otherwise; the caller escalates with the incidence's cause.
///
/// A vertex ON the axis but off the surface (within the band of a pole)
/// has lever 0, so any gap decides Zero there and the image itself is
/// taken. That is sound in 3-D: every chart end of the joint then lies
/// within a few ε of the pole, where no azimuth names a point; and
/// [`chart_boundary`]'s fence refuses such a joint before a polygon
/// reads its azimuth.
fn near_pole_gap_closes<T: Decide>(
    arm: ChartArm<T>,
    gap: T,
    step: T,
    twin: bool,
    band: Band,
) -> Option<i32> {
    let orbit: &[i32] = if twin { &[0, 1, -1] } else { &[0] };
    orbit.iter().copied().find(|&m| {
        let off = gap - T::from_f64(f64::from(m)) * step;
        matches!(
            decide("pcurve_loop_pole_gap", arm.meter(off), band),
            Ok(Sign::Zero)
        )
    })
}

/// **A spline chart's joint gap**: the entry `entry` and the
/// predecessor's exit `prev` agree within ε through the chart's sup
/// stretch on each channel ([`chart_u_arm`], [`v_meter`]). A net can
/// fold, so on a spline chart a 3-D coincidence does not name the
/// sheet, and the joint states its chart-space gap ([`lift_joint`]).
///
/// # Errors
///
/// [`PinMiss::Discontinuity`] for a definite gap,
/// [`PinMiss::Escalated`] for an undecided one.
fn spline_gap_closes<T: Decide>(
    chart: DescribedChart<'_, T>,
    entry: geom_core::Point2<T>,
    prev: geom_core::Point2<T>,
    band: Band,
) -> Result<(), PinMiss> {
    let arm = chart_u_arm(chart, prev.y);
    for margin in [
        arm.meter(entry.x - prev.x),
        Margin::metered_sup(entry.y - prev.y, v_meter(chart)),
    ] {
        match decide("pcurve_loop_continuity", margin, band) {
            Ok(Sign::Zero) => {}
            Ok(Sign::Positive | Sign::Negative) => return Err(PinMiss::Discontinuity),
            Err(cause) => return Err(PinMiss::Escalated(cause)),
        }
    }
    Ok(())
}

/// The chart image of one walked half-edge, in loop direction.
///
/// # Errors
///
/// A `Fitted` or `General` image carrying no stored certificate: its
/// envelope is the only statement bounding the image against its
/// carrier, and inventing one would widen nothing while claiming a
/// bound. [`PcurveMintError::UncertifiedImage`] when the face stores
/// no row at all (`minted` false — never minted, or left uncached by
/// the mint for a pair no lane covers yet), and
/// [`PcurveMintError::MissingCache`] when it stores others: a
/// half-minted face.
fn chart_edge<T: Decide>(
    body: &Body<T>,
    walked: &Walked<T>,
    chart: &Surface<T>,
    plus: bool,
    minted: bool,
) -> Result<ChartEdge<T>, PcurveMintError> {
    let (entry_t, exit_t) = if plus {
        (walked.t0, walked.t1)
    } else {
        (walked.t1, walked.t0)
    };
    let a = walked.pcurve.eval(entry_t);
    let b = walked.pcurve.eval(exit_t);
    let straight = match &walked.pcurve {
        // Straight by VARIANT: `IsoLine` is `p0 + pl·t`, and an
        // `IsoArc`'s UV image is the segment `p0 → p0 + pd` (only the
        // parameterization along it is transcendental).
        Pcurve::IsoLine { .. } | Pcurve::IsoArc { .. } => true,
        // Straight by the STRUCTURE of two matched arms rather than by
        // a zero-test on `T` (C6): `carrier_harmonic` gives a
        // `Curve3::Line` the coefficients `a = b = 0` — the zero
        // VECTOR, not a small one — and `chart_pcurve`'s plane arm is
        // affine, mapping them through one by one. So a line carrier
        // on a plane chart has an exactly straight image, and every
        // other harmonic is described by its envelope.
        //
        // What that concedes is nil where it looks like a concession.
        // A rim or a meridian on an azimuth chart is also straight,
        // and becomes an `Envelope` here — but its image box is the
        // chord's own box, degenerate in the channel the chord does
        // not move, so the box axes already state everything the
        // segment-normal axis would. The only straight image that
        // loses anything is a chart DIAGONAL off a plane chart (a
        // helical harmonic), which no construction mints.
        Pcurve::Harmonic { .. } => {
            matches!(chart, Surface::Plane { .. })
                && matches!(walked.carrier, geom::Curve3::Line { .. })
        }
        // A spiric image is curved on BOTH charts it lives on — the
        // cap's `pm·f(t) + pa·sin t` and the wall's `atan2(f, d)`
        // azimuth — so it takes the envelope door below, which the
        // `_` arm there already answers from `eval` over the span
        // hull.
        Pcurve::Spiric { .. } => false,
        // A cone section's image is curved in both channels, and takes
        // the same envelope door.
        Pcurve::ConeSection { .. } => false,
        Pcurve::Fitted(_) | Pcurve::General(_) => false,
    };
    if straight {
        return Ok(ChartEdge::Segment { a, b });
    }
    match &walked.pcurve {
        // A fitted or general image's box is its CONTROL HULL (the
        // convex-hull property), and what stands between that image
        // and the carrier is the stored certificate's envelope — in
        // metres, so `metred` is where it widens the box.
        Pcurve::Fitted(_) | Pcurve::General(_) => {
            let half_edge = walked.key;
            let cache = body.pcurve(half_edge).ok_or(if minted {
                PcurveMintError::MissingCache { half_edge }
            } else {
                PcurveMintError::UncertifiedImage { half_edge }
            })?;
            let hull = walked.pcurve.chart_box(walked.t0, walked.t1);
            Ok(ChartEdge::Envelope {
                a,
                b,
                image: Point2::new(
                    hull.u_min.enclosure_hull(hull.u_max),
                    hull.v_min.enclosure_hull(hull.v_max),
                ),
                slack: cache.certificate().envelope,
            })
        }
        // The closed-form image is exact in its family, so the slack is
        // zero and the image's enclosure is the image evaluated over
        // the span's enclosure — the interval natural extension, not
        // `chart_box`. At a POINT scalar the span hull is poison, the
        // enclosure is poison, and the outside test certifies nothing
        // against it — which is the safe direction.
        _ => Ok(ChartEdge::Envelope {
            a,
            b,
            image: walked.pcurve.eval(walked.t0.enclosure_hull(walked.t1)),
            slack: T::zero(),
        }),
    }
}

/// **The face's boundary in a chart the CALLER names** — the certified
/// outer description a subdivision consumer intersects its carrier
/// window with, and tests its cells against
/// ([`crate::chart_bound::MetredBound::certifies_outside`]).
///
/// `chart`'s locus must be the face's carrier. It is a parameter
/// rather than the stored surface because a consumer tests this
/// description against a window of its OWN, and a description in a
/// chart the consumer does not use answers a question nobody asked: a
/// caller that charts a plane on axes of its own choosing — a
/// locus-equal chart with a different `u_ref` — needs the boundary in
/// that chart, not in the stored one. Consumers that read the stored
/// chart pass the stored surface, and the azimuth charts do.
///
/// Nothing is read from the pcurve CACHES except a `Fitted`/`General`
/// image's certificate: every chart image is re-derived by the loop
/// walk, exactly as [`mint_pcurves`] derives it, so a body that never
/// ran the minting pass describes as well as one that did.
///
/// # Which charts this describes
///
/// **Plane, cylinder and torus.** Their first channel has a lever
/// everywhere on the chart, so every joint of a loop has an azimuth
/// and the chord polygon is a statement about the same region the face
/// is.
///
/// **Sphere and cone are refused where a loop meets the singularity**
/// ([`PcurveMintError::SingularChartJoint`]): a joint vertex on a pole
/// or an apex, decided as 3-D incidence ([`singular_at`]), has no
/// azimuth, the walk decides no branch there, and the chord drawn to
/// such a joint bounds a different region from the face. A sphere or
/// cone face that stays clear of its singularity describes normally.
///
/// **A spline chart** describes when [`chart_u_period`] can answer for
/// it. A plane has no period, so its walk decides no branch.
///
/// Each joint's polygon vertex is the row's entry: the walk decided the
/// joint's deck element, and the entry lies within the joint bound
/// [`lift_joint`] states of the exit before it. That bound rests on each
/// row's envelope, so every closed-form row is certified against
/// `chart` here first ([`PcurveCertifyError`] through
/// [`PcurveMintError::Certify`] where one does not), minted face or
/// not, whatever chart the caller names.
///
/// # Errors
///
/// [`PcurveMintError::PlaceholderChart`] when `chart` is the mvfs
/// placeholder, before any loop is walked;
/// [`PcurveMintError`] — the loop walk's own refusals (a corrupt key,
/// a typed chart refusal such as [`PcurveCertifyError::UnsupportedCarrier`]
/// for a `Nurbs` carrier on an analytic chart, a discontinuous or
/// unclosed walk); [`PcurveMintError::UncertifiedImage`] for a fitted
/// or general image on a face that stores no rows, and
/// [`PcurveMintError::MissingCache`] for one missing from a face that
/// stores others; [`PcurveMintError::SingularChartJoint`] for a loop
/// through a pole or an apex; [`PcurveMintError::LoopWraps`] for a
/// walk that closes a whole period off; and
/// [`PcurveMintError::OuterSpansPeriod`] from
/// [`crate::chart_bound::ChartBound::assembled`].
pub fn chart_boundary<T: AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    chart: &Surface<T>,
    band: Band,
) -> Result<ChartBound<T>, PcurveMintError> {
    let described = DescribedChart::of(chart).ok_or(PcurveMintError::PlaceholderChart { face })?;
    let face_data = body.get_face(face).ok_or(PcurveMintError::Corrupt)?;
    let minted = stored_rows(body, face_data).stores;
    let loops: Vec<LoopKey> = core::iter::once(face_data.outer)
        .chain(face_data.rings.iter().copied())
        .collect();
    let period = chart_u_period(chart, band);
    let mut outer: Option<ChartLoop<T>> = None;
    let mut rings: Vec<ChartLoop<T>> = Vec::new();
    for (index, lp) in loops.iter().enumerate() {
        let mut walked: Vec<Walked<T>> = Vec::new();
        walk_loop(body, face, *lp, described, band, &mut walked)?;
        let (Some(first), Some(last)) = (walked.first(), walked.last()) else {
            // An empty loop bounds nothing to describe.
            continue;
        };
        // THE SINGULAR-JOINT FENCE, before anything else is read off
        // the walk. A joint on the chart's singular set — a sphere pole,
        // a cone apex — has no azimuth, so the walk decides no branch
        // there ([`lift_joint`]) and leaves whatever azimuth the
        // derivation produced. Minting is right to do that: no
        // downstream sample reads the azimuth AT the pole. A chord
        // polygon does read it, as a vertex, and the polygon it draws
        // bounds a different region from the face. Decided here as 3-D
        // incidence of each joint's vertex on that set
        // ([`singular_at`]), and anything but definitely off it refuses.
        for w in &walked {
            if !matches!(
                singular_at(described, entry_vertex(body, w.key)?, band),
                Singular::Off
            ) {
                return Err(PcurveMintError::SingularChartJoint {
                    face,
                    r#loop: *lp,
                    half_edge: w.key,
                });
            }
        }
        // The walk certified that the loop CLOSES; it accepts a winding
        // of one whole period (the seam case) and, on a sphere chart, a
        // closure through the involution. Neither is a closed chart
        // polygon: the chord from the last exit back to the first entry
        // is spurious, and a polygon built on it would bound a region
        // the face does not have. So the closing joint's deck element,
        // the loop's winding, is decided again here under the walk's
        // own rows, and anything but the identity refuses. Each row's
        // entry is the polygon's vertex ([`ChartLoop`]), within the
        // joint bound [`lift_joint`] states of the exit before it.
        let first_t = if is_plus(body, first.key)? {
            first.t0
        } else {
            first.t1
        };
        let end = last.pcurve.eval(if is_plus(body, last.key)? {
            last.t1
        } else {
            last.t0
        });
        let closing = lift_joint(
            described,
            first.pcurve.clone(),
            first_t,
            end,
            entry_vertex(body, first.key)?,
            period,
            band,
        );
        if !closing.is_ok_and(|lift| lift.identity()) {
            return Err(PcurveMintError::LoopWraps { face, r#loop: *lp });
        }
        // The joint bound the polygon leans on rests on each row's
        // envelope (check 4), and these rows are fresh derivations in
        // the CALLER's chart, which no mint certified. So each
        // closed-form row is certified against that chart here, before
        // its entry is read as a vertex; a fitted or general one reads
        // its stored certificate ([`chart_edge`]).
        for w in &walked {
            if !matches!(w.pcurve, Pcurve::Fitted(_) | Pcurve::General(_)) {
                PcurveCache::certify(w.pcurve.clone(), w.t0, w.t1, &w.carrier, chart, band)
                    .map_err(|error| PcurveMintError::Certify {
                        half_edge: w.key,
                        error,
                    })?;
            }
        }
        let mut edges = Vec::with_capacity(walked.len());
        for w in &walked {
            edges.push(chart_edge(body, w, chart, is_plus(body, w.key)?, minted)?);
        }
        let described = ChartLoop {
            edges,
            ring: index > 0,
        };
        if index == 0 {
            outer = Some(described);
        } else {
            rings.push(described);
        }
    }
    // A face whose OUTER loop describes nothing bounds no chart region;
    // there is no honest description to return and no empty one that
    // would not be a claim.
    let outer = outer.ok_or(PcurveMintError::Corrupt)?;
    // The span check's lever: the chart's own first-channel arm. Every
    // chart this function describes has a constant one except the
    // torus, where the local lever at the outer's lowest latitude is
    // the honest reading and an inexact one can only move where the
    // refusal fires.
    let u_arm = chart_u_arm(
        described,
        outer.edges.first().map_or_else(T::zero, |e| e.a().y),
    );
    ChartBound::assembled(outer, rings, period, u_arm, band)
}

/// The at-rest pcurve pass the tier-3 validator runs (C4: on every
/// chart that mints, **every half-edge stores its certified row** —
/// except on a face whose rows are not owed ([`not_owed`]), the
/// exemption C4 keeps until each uncovered class's route lands — and
/// every stored row re-certifies).
///
/// For every face whose chart mints ([`DescribedChart::minting`]):
///
/// 1. **A face that stores none of its rows is re-derived** — the
///    minting pass over it, storing nothing ([`derive_face`]) — so the
///    finding says why the rows are missing: they derive and certify
///    ([`PcurveMintError::Unminted`]: the body was not minted after it
///    was built or edited), or the derivation refuses (that refusal).
///    A face whose rows are not owed ([`not_owed`]: an uncovered class,
///    or a fitted face at a scalar with no fitted door) is excused here
///    by the same predicate as at the mint, read over the whole face:
///    every edge the mint can image is walked through its joints and
///    certified as the mint would, so a covered edge's refusal is the
///    face's whichever uncovered edge shares it.
///    A face a null edge holds open ([`held_open`]) is not derived: the
///    edge has no carrier to derive from, and the scaffold is tier 2's
///    finding at rest.
/// 2. **A face that stores some of its rows** reports each gap
///    ([`PcurveMintError::MissingCache`]), and the derivation's refusal
///    where it refuses; the rows it does store are measured as a
///    complete face's are, below.
/// 3. **Every stored row states its edge's interval**
///    ([`PcurveMintError::RowInterval`] where it does not — a row left
///    from before a split or a carrier swap) **and is re-certified**
///    against its carrier and chart, on a complete face and a
///    half-minted one alike. The stored certificate is never consulted
///    (re-certification re-derives, it does not trust, exactly as
///    [`geom_brep::EdgeCurve::recertify`]), and no branch enters what a
///    row is measured against: a loop's rows may stand a whole period
///    over, or on a sphere's involution twin, from a fresh walk and
///    describe the same face (`Body::revert`'s re-statement).
/// 4. **Each loop's lift is re-decided** on the stored pcurves, as the
///    mint's walk reads the loop ([`lift_joint`]): at every joint the
///    deck element is the identity (`k = 0`; a row a whole period off
///    its neighbour, or on the other sphere twin, is a
///    [`PcurveMintError::LoopDiscontinuity`]), except at the one the
///    walk lets wrap (into the cycle's first half-edge), read as the
///    closure, whose deck element is the loop's winding (`k = w`,
///    [`loop_closes`]). The chain runs once around from the loop's
///    first stored row. A half-edge that stores no row is carried by
///    the image the mint would derive there, pinned as the walk pins
///    it, so the rows either side of a gap are measured across it and
///    the wrap is read whenever the chain reaches it (at the next
///    stored row, where the gap is the cycle's first half-edge); a gap
///    the derivation cannot place breaks the chain. So a body whose
///    branches were tampered with fails here even if each pcurve
///    certifies in isolation, and the verdict hangs neither on which
///    half-edge is the gap nor on the branch the loop's rows stand on.
///
/// Returns the findings in face-arena / loop / cycle order (D9),
/// empty when the body is clean. A planar face stores nothing and is
/// not read.
pub fn validate_pcurves<T: AtRestPolicy>(body: &Body<T>, band: Band) -> Vec<PcurveMintError> {
    let mut findings = Vec::new();
    // The fitted door every `Fitted`/`General` row re-derives through,
    // read off the scalar's policy once: the certified and the
    // structural validation doors reach this pass alike, so neither
    // moves a verdict at any scalar.
    let lane = T::fitted_lane();
    for (face_key, face) in body.faces() {
        let Some(surface) = body.get_surface(face.surface) else {
            continue;
        };
        let Some(chart) = DescribedChart::minting(surface) else {
            continue;
        };
        // One walk of the face's loops ([`stored_rows`], shared with
        // the Euler operators' site mint): the rows it stores and the
        // gaps.
        let stored = stored_rows(body, face);
        if !stored.complete() {
            // A loop a null edge holds open cannot be walked (its halves
            // have no carrier), so there is no derivation to read; the
            // scaffold is tier 2's finding at rest, and the gaps are
            // reported as found.
            let open = match stored.open_loops(body) {
                Ok(open) => !open.is_empty(),
                Err(_) => {
                    findings.push(PcurveMintError::Corrupt);
                    continue;
                }
            };
            let refusal = if open {
                None
            } else {
                derive_face(body, face_key, band).err()
            };
            if !stored.stores {
                match refusal {
                    None if stored.gaps.is_empty() || open => {}
                    None => findings.push(PcurveMintError::Unminted { face: face_key }),
                    Some(e) if not_owed::<T>(&e) => {}
                    Some(e) => findings.push(e),
                }
                continue;
            }
            findings.extend(stored.gaps.iter().map(|gap| match *gap {
                RowGap::Corrupt => PcurveMintError::Corrupt,
                RowGap::Missing { half_edge, .. } => PcurveMintError::MissingCache { half_edge },
            }));
            if let Some(e) = refusal.filter(|e| !not_owed::<T>(e)) {
                findings.push(e);
            }
        }
        let cycles: Vec<Vec<HalfEdgeKey>> = stored
            .loops
            .into_iter()
            .filter_map(|(_, cycle)| cycle)
            .collect();
        // Re-certify every stored row against its carrier and chart.
        for cycle in &cycles {
            for &he in cycle {
                let Some(cache) = body.pcurve(he) else {
                    continue;
                };
                let (carrier, _, _) = match half_edge_carrier(body, he) {
                    Ok(found) => found,
                    Err(e) => {
                        findings.push(e);
                        continue;
                    }
                };
                if let Err(e) = row_interval(body, he, cache, &carrier, band) {
                    findings.push(e);
                }
                let mate = mate_surface(body, he);
                if let Err(error) = cache.recertify(&carrier, surface, mate.as_ref(), band, lane) {
                    findings.push(PcurveMintError::Certify {
                        half_edge: he,
                        error,
                    });
                }
            }
        }
        // The loop facts of the STORED rows, read as the mint's walk
        // reads a loop: at every joint the deck element is the identity
        // (the two rows stand on one lift), except at the one the walk
        // lets wrap — into the cycle's first half-edge, whose image the
        // walk takes on its own branch — which is read as the closure,
        // whose deck element is the loop's winding ([`loop_closes`]).
        // The chain starts at the loop's first stored row and runs once
        // around back to it. A half-edge that stores no row is carried
        // by the image the mint would derive for it ([`derive_image`]),
        // pinned to the chain as the walk pins it ([`lift_joint`]);
        // where that half-edge is the cycle's first, the wrap passes to
        // the next stored row the chain reaches. So the rows either side
        // of a gap are measured across it, the wrap is read whenever the
        // chain reaches it, and a loop's verdict does not hang on which
        // half-edge is the gap, nor on the branch its rows stand on (a
        // loop moved a whole period over reads as it did). A gap the
        // derivation cannot place breaks the chain until the next
        // stored row.
        let u_period = chart_u_period(surface, band);
        let ends = |he: HalfEdgeKey| -> Result<Option<(Pcurve<T>, T, T)>, PcurveMintError> {
            let plus = is_plus(body, he)?;
            Ok(body.pcurve(he).map(|cache| {
                let (t0, t1) = cache.params();
                let (entry_t, exit_t) = if plus { (t0, t1) } else { (t1, t0) };
                (cache.pcurve().clone(), entry_t, exit_t)
            }))
        };
        let derived = |he: HalfEdgeKey| -> Option<(Pcurve<T>, T, T)> {
            let (_, t0, t1, base) = derive_image(body, he, surface, band).ok()?;
            let plus = is_plus(body, he).ok()?;
            let (entry_t, exit_t) = if plus { (t0, t1) } else { (t1, t0) };
            Some((base, entry_t, exit_t))
        };
        for cycle in &cycles {
            let n = cycle.len();
            let mut rows = Vec::with_capacity(n);
            for &he in cycle {
                match ends(he).and_then(|row| Ok((row, entry_vertex(body, he)?))) {
                    Ok((row, vertex)) => rows.push(row.map(|row| (row, vertex))),
                    Err(e) => {
                        findings.push(e);
                        rows.push(None);
                    }
                }
            }
            let Some(anchor) = rows.iter().position(Option::is_some) else {
                continue;
            };
            let mut prev_exit: Option<geom_core::Point2<T>> = rows[anchor]
                .as_ref()
                .map(|((pcurve, _, exit_t), _)| pcurve.eval(*exit_t));
            // Whether the chain has passed the cycle's first half-edge
            // since its last stored row, so the next stored row's joint
            // is the wrap.
            let mut wrap_due = false;
            for k in 1..=n {
                let i = (anchor + k) % n;
                if i == 0 {
                    wrap_due = true;
                }
                let he = cycle[i];
                let Some(((pcurve, entry_t, exit_t), vertex)) = &rows[i] else {
                    prev_exit = prev_exit.and_then(|prev| {
                        let (base, entry_t, exit_t) = derived(he)?;
                        let vertex = entry_vertex(body, he).ok()?;
                        lift_joint(chart, base, entry_t, prev, vertex, u_period, band)
                            .ok()
                            .map(|lift| lift.pcurve.eval(exit_t))
                    });
                    continue;
                };
                if let Some(prev) = prev_exit {
                    if wrap_due {
                        if !loop_closes(chart, pcurve, *entry_t, *vertex, prev, u_period, band) {
                            findings.push(PcurveMintError::LoopNotClosed { face: face_key });
                        }
                    } else {
                        let lift = lift_joint(
                            chart,
                            pcurve.clone(),
                            *entry_t,
                            prev,
                            *vertex,
                            u_period,
                            band,
                        );
                        match lift {
                            Ok(lift) if lift.identity() => {}
                            // A lift other than the identity where the
                            // vertex's incidence on the singular set is
                            // undecided is that decision's escalation:
                            // on the set, any lift would do.
                            Ok(Lift {
                                singular: Singular::Undecided(cause),
                                ..
                            })
                            | Err(PinMiss::Escalated(cause)) => {
                                findings.push(PcurveMintError::Escalated {
                                    half_edge: he,
                                    cause,
                                });
                            }
                            Ok(_) | Err(PinMiss::Discontinuity | PinMiss::OutOfReach) => {
                                findings.push(PcurveMintError::LoopDiscontinuity { half_edge: he });
                            }
                        }
                    }
                }
                wrap_due = false;
                prev_exit = Some(pcurve.eval(*exit_t));
            }
        }
    }
    findings
}

#[cfg(test)]
pub(crate) mod staleness_posture {
    #![allow(clippy::expect_used)]

    /// Which of this module's four postures a mutation door holds.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub(crate) enum Posture {
        /// Clears and re-mints before returning — over the whole body
        /// or over exactly the faces it wrote. Read out of the source
        /// (a `mint_pcurves` or `mint_pcurves_of` call in the door's
        /// own body); an entry declares it only when the re-mint is one
        /// delegation away, which a source read cannot see, or is an
        /// Euler operator's site mint ([`super::site_rows`]).
        ///
        /// **The site mint is not the subset pass, and differs from it
        /// in two places.** What it shares with the pass is that no row
        /// the operator could have staled survives: on a face whose rows
        /// were complete, the loops it rewires are re-derived exactly
        /// as the pass derives them, and the loops it keeps held rows
        /// the surgery did not touch; on a face whose only gaps a null
        /// edge holds open, or that the operator takes the last null
        /// edge off, so are the loops it rewires and leaves running
        /// through no null edge, and a loop still held open keeps what
        /// it had.
        /// Where the pass would MINT — a face storing no row, or one
        /// half-minted any other way — an operator's site mint leaves
        /// the face as found: the pass owns it. Where the pass would
        /// REFUSE — a loop that does not close, a branch that meets no
        /// neighbour, a certification that refuses — the site mint
        /// clears the face and the operator returns `Ok`: it runs
        /// mid-surgery, on states a later door finishes describing, and
        /// the producer's final pass is where a finished state is
        /// refused.
        Maintains,
        /// Disposes of every row whose KEY or MEANING the door
        /// changed, rather than leaving one behind: moves it onto the
        /// key that now carries what it says — the transplanted
        /// half-edge's fresh key, or the two keys a parameter split
        /// leaves where one edge was — and drops what no key can
        /// carry, including a row whose key never moved but whose
        /// chart did (the loop-re-parenting doors, the run `mef`'s
        /// chord surgery or `kef`'s unsplice moves between two faces'
        /// loops, and the surface setter, under which a face's whole
        /// row set changes chart at once). What a door in this bucket
        /// never does is return with a row that says something the
        /// body no longer holds. Where a door in this bucket moves a
        /// loop or run onto a face that was complete and the moved rows
        /// do not stand there, it either refuses, keys-only, or — its
        /// `_minting` twin — re-mints that face through the site mint
        /// ([`crate::Body::plan_moved_rows`]), so a complete
        /// destination on an analytic chart leaves complete, or storing
        /// nothing; on a spline chart the drop stands.
        /// Whether a half-edge a door mints gets a row at the mint site
        /// is the minting posture ([`super::site_rows`]), and a door in
        /// this bucket that mints half-edges says so in its note.
        Transfers,
        /// Leaves the map exactly as it found it — a primitive, or a
        /// write the map is not keyed on. What this bucket rests on is
        /// the tier-3 pcurve pass, and only as far as that pass looks
        /// (module docs): it reports a face missing any of its rows,
        /// and a stored row that no longer certifies; it is silent about a complete
        /// face whose rows were stated in another chart and certify
        /// against this one, and about any face on a chart
        /// [`super::chart_mints`] refuses.
        Neither,
        /// Keeps the rows it finds, as `Neither` does, and rests on the
        /// same tier-3 pass for what its write stales in them — except
        /// on a face it COMPLETES: where it installs the first carrier of
        /// a null edge, whose halves no door before it could give a row,
        /// it re-mints through the site mint ([`super::site_rows`])
        /// every loop of the face they are on that no other null edge
        /// holds open, on a face the site mint selects
        /// ([`super::StoredRows::remints`]) — every loop of it, whatever
        /// it missed, once no null edge is left on it. Those loops leave
        /// complete, or the face rowless where the closed-form lane
        /// cannot mint it, or — on a spline chart — as found.
        Completes,
    }

    /// `(door, posture, note)` — the doors that do NOT re-mint the
    /// map in their own body. The reason a posture is SAFE lives once,
    /// on the [`Posture`] variant; a note here says only what is
    /// particular to this door.
    ///
    /// Module-scoped rather than local to the guard so that
    /// [`crate::review_m1_pr5_internal::the_two_door_tables_cover_the_same_surface`]
    /// can read it. That row is the only other reader; this table
    /// stays this guard's.
    pub(crate) const DECLARED: &[(&str, Posture, &str)] = {
        use Posture::{Completes, Maintains, Neither, Transfers};
        &[
            // ---- Maintains, one delegation away from the re-mint. ----
            (
                "merge_coplanar_faces",
                Maintains,
                "calls `merge_coplanar_faces_declared`, which re-mints the staged result",
            ),
            // ---- Maintains: the doors that re-mint their own staged
            // result. They carry prose here rather than in
            // `review_m1_pr5_internal::ALLOWED` because they assert
            // tier 1 through a surgery scope (`crate::surgery`) and so
            // are no longer allowlisted there; the two tables still
            // have to cover this one population between them. ----
            (
                "merge_coplanar_faces_declared",
                Maintains,
                "re-mints the staged result before it is adopted, whenever the operand \
                 carried rows",
            ),
            (
                "replace_faces_offset",
                Maintains,
                "re-mints the clone whole-body before adopting it",
            ),
            (
                "offset_planes_together",
                Maintains,
                "re-mints the moved faces' rows on the clone (`mint_pcurves_of`) before \
                 adopting it",
            ),
            (
                "offset_charts_together",
                Maintains,
                "the axial spelling of `offset_planes_together`, with the same re-mint",
            ),
            (
                "replace_face_offset",
                Maintains,
                "the one-face spelling of `replace_faces_offset`, which re-mints the clone \
                 before adopting it",
            ),
            // ---- The pass itself. ----
            (
                "mint_pcurves",
                Maintains,
                "IS the pass: clears the map, then re-mints every row of the body it is given",
            ),
            (
                "mint_pcurves_of",
                Maintains,
                "IS the pass restricted to a face subset: clears exactly those faces' rows, \
             then re-mints exactly those faces. `Maintains` FOR A CALLER THAT KILLS NO \
             HALF-EDGE, which is the whole of its production population (the two \
             simultaneous offset doors); it cannot discharge the whole-body claim, and its \
             own docs carry why",
            ),
            // ---- Transfers: the graft's remap-and-drop, and the
            // split's restriction onto the two keys it leaves. ----
            (
                "graft_disjoint",
                Transfers,
                "the graft, through `boolean::combine` — see `graft_disjoint_all_keyed`",
            ),
            (
                "graft_disjoint_all",
                Transfers,
                "the graft — see `graft_disjoint_all_keyed`",
            ),
            (
                "graft_disjoint_all_keyed",
                Transfers,
                "remaps each row onto the transplanted half-edge's fresh key and DROPS any \
             row the graft walk did not reach, which is the staleness test itself",
            ),
            (
                "insert_void",
                Transfers,
                "the void-insertion door — see `insert_voids`, which it calls with the one \
             destination as a slice",
            ),
            (
                "insert_voids",
                Transfers,
                "the void-insertion door: reverts the cavity (`Body::revert` carries every \
             row key for key, the plane faces' rows mirrored with their frames — the \
             module docs' producer position) and grafts through `boolean::combine`, \
             which remaps the transplanted rows onto fresh keys; every producer's final \
             mint pass — the boolean's, the revolve's and `shell`'s — re-derives every \
             row of the merged body",
            ),
            (
                "split_edge",
                Transfers,
                "restricts each parent half-edge's row to the two children's sub-intervals \
             and re-certifies both before it mutates (`split_cache`), so no row it could \
             have staled survives; a `Fitted`/`General` row is the one lane it leaves as \
             found",
            ),
            // ---- Neither: the primitives. Their stale rows are what
            // the tier-3 pcurve pass exists to catch. ----
            (
                "cyl_wall_sheet_keyed",
                Neither,
                "a fixture builder composed of `Neither` operators, minting no row of its \
             own; `cyl_wall_sheet` is the door that runs the pass over what it grew",
            ),
            ("mvfs", Neither, "Euler operator"),
            (
                "mev_null",
                Neither,
                "Euler operator minting a NULL edge: scaffolding with no carrier, so no row \
             to derive, and the loop it joins is held open, missing the edge's two rows. \
             The door that releases the loop mints it whole, except on a spline chart: an \
             operator rewiring it out from under the edge (the boolean's and the \
             splitting lane's joins) or the edge's first description \
             (`set_edge_curve`); a kill that releases it does not \
             (`work/topo/a-kill-that-releases-a-loop-from-its-last-null-edge-leaves-its-gaps`); \
             tier 2 refuses a null edge at rest",
            ),
            ("kemr", Neither, "Euler operator"),
            ("kev", Neither, "kill op"),
            (
                "kev_describing",
                Neither,
                "kill op; the members it re-describes keep their rows, as \
             `set_edge_curve` leaves them",
            ),
            ("kvfs", Neither, "kill op"),
            // ---- Maintains: the half-edge-minting Euler operators,
            // whose re-mint is the site mint, not a pass call. ----
            (
                "mev",
                Maintains,
                "Euler operator: on every face that its two new halves join whose rows were \
             complete, or whose only gaps a null edge holds open, re-mints the loops it \
             rewires that run through no null edge before it mutates and keeps the rest \
             (`pcurves::site_rows`), so no row it could have staled survives; the face \
             leaves as it was found — complete, or complete but for the loops a null edge \
             holds open — or rowless where the closed-form lane cannot mint it; where the \
             pass would refuse, this clears and returns `Ok`. An unminted face, or one \
             half-minted any other way, which the pass would mint, is left as found; a \
             spline chart refuses typed on a complete face and leaves a held-open one as \
             found",
            ),
            ("mev_line", Maintains, "Euler operator (sugar over `mev`)"),
            (
                "mekr",
                Maintains,
                "Euler operator: `mev`'s site mint over the one face whose ring it merges",
            ),
            (
                "mekr_chord",
                Maintains,
                "Euler operator (sugar over `mekr`)",
            ),
            // ---- Transfers: the loop-re-parenting doors, which carry
            // a moved loop's rows onto the target face and drop them
            // when that face is on another CHART. Each is two doors:
            // the keys-only one refuses where the target was complete
            // and the moved rows do not stand on it
            // (`SiteRowRefusal::KeysOnly`, before mutating); its
            // `_minting` twin takes a band and re-mints the target
            // through the site mint there (`Body::plan_moved_rows`). ----
            (
                "kfmrh",
                Transfers,
                "Euler operator, and a loop re-parenting: `f2`'s demoted outer loop keeps its \
             rows where `f1` is on the same chart (`Body::same_chart`) and loses them where \
             it is not (`Body::drop_rows_on_chart_change`); where they do not stand and \
             `f1`'s rows were complete on an analytic chart it refuses `KeysOnly` before \
             mutating, and a spline chart keeps the drop",
            ),
            (
                "kfmrh_minting",
                Transfers,
                "`kfmrh` with a band: where `kfmrh` refuses `KeysOnly`, `f1` is re-minted with \
             the demoted loop walked in its chart (`Body::plan_moved_rows`)",
            ),
            (
                "mfkrh",
                Transfers,
                "Euler operator, and a loop re-parenting: the promoted ring keeps its rows \
             where the spec lands on the demoting face's chart (`Body::same_chart`) and \
             loses them where it does not; where they do not stand and the demoting face's \
             rows were complete on an analytic chart it refuses `KeysOnly` before mutating, \
             and a spline chart keeps the drop",
            ),
            (
                "mfkrh_minting",
                Transfers,
                "`mfkrh` with a band: where `mfkrh` refuses `KeysOnly`, the new face is minted \
             with the ring walked in its chart (`Body::plan_moved_rows`)",
            ),
            (
                "mfkrh_plug",
                Transfers,
                "`mfkrh` with a PLACEHOLDER surface — see `mfkrh`; a placeholder is not a \
             described surface, so it is not the chart any row was stated in and the \
             promoted ring's rows always go, with no refusal: it is a spline chart. \
             Decided by kind, not by the fresh key the sugar happens to mint",
            ),
            (
                "ring_move",
                Transfers,
                "ring surgery: re-parents a ring, mints no half-edge, carries or drops the \
             ring's rows by whether the two faces are on one chart, and refuses `KeysOnly` \
             where they do not stand on a complete target — see `kfmrh`",
            ),
            (
                "ring_move_minting",
                Transfers,
                "`ring_move` with a band: where `ring_move` refuses `KeysOnly`, the target is \
             re-minted with the ring walked in its chart — see `kfmrh_minting`",
            ),
            // ---- Transfers: the two doors that move a RUN of
            // half-edges between two faces' loops, and dispose of the
            // run's rows the way the loop doors dispose of a loop's. ----
            (
                "mef",
                Transfers,
                "Euler operator, and a run re-parenting: the run `[he1 .. he2)` it moves onto \
             the new face keeps its rows where that face is on the old face's chart \
             (`Body::same_chart`) and loses them where it is not (`Body::drop_rows`). The \
             two halves it mints get their rows at the site, as `mev`'s do: the old face \
             is re-minted when the site mint selects it, and so is the new face, its run \
             walked in the new face's chart; a spline chart other than the old face's \
             keeps the drop",
            ),
            ("mef_chord", Transfers, "Euler operator (sugar over `mef`)"),
            (
                "kef",
                Transfers,
                "kill op, and a run re-parenting: the dying loop's remnant keeps its rows \
             where the surviving face is on the dying face's chart (`Body::same_chart`) and \
             loses them where it is not (`Body::drop_rows`); where they do not stand and the \
             surviving face's rows were complete on an analytic chart it refuses `KeysOnly` \
             before mutating, and a spline chart keeps the drop; the two killed halves' rows \
             outlive their keys as every kill op's do",
            ),
            (
                "kef_minting",
                Transfers,
                "`kef` with a band: where `kef` refuses `KeysOnly`, the surviving loop is \
             re-minted in the surviving face's chart (`Body::plan_moved_rows`)",
            ),
            (
                "movefac",
                Neither,
                "re-parents faces between shells; no half-edge key changes meaning",
            ),
            (
                "move_shells_to_new_solid",
                Neither,
                "re-parents shells between solids; no half-edge key changes meaning",
            ),
            // ---- Neither: the caller's own row-level control of the
            // map, and writes the map is not keyed on. ----
            (
                "attach_pcurve",
                Neither,
                "writes ONE row the caller chose; every other row is untouched, and \
             certifying this one is the caller's",
            ),
            ("detach_pcurve", Neither, "drops ONE row the caller chose"),
            (
                "set_face_surface",
                Transfers,
                "re-charts a face in place, which changes what every row the face stores is \
             ABOUT while changing no key: the rows are kept across a swap onto the same \
             chart (`Body::same_chart`) and dropped on any other (`Body::drop_face_rows`). \
             Content staleness alone would be \
             the tier-3 pass's, but only where the NEW surface mints — a swap onto a plane \
             or a placeholder left a COMPLETE row set stated in the chart the face left, \
             which that pass skips entirely",
            ),
            (
                "set_face_surface_stranding_for_tests",
                Transfers,
                "the failure-injection twin of `set_face_surface`, whose rows it keeps and \
             drops on the same terms",
            ),
            (
                "set_face_surfaces_describing",
                Transfers,
                "`set_face_surface`'s swap per face, on its terms: a face's rows are kept \
             across a move onto the same chart and dropped on any other. The edges it \
             re-describes are the listed certified ones, whose rows stand as \
             `set_edge_curve` leaves a certified edge's",
            ),
            (
                "set_edge_curve",
                Completes,
                "a carrier swap is content staleness the tier-3 pass re-certifies against, \
             and NOT the surface setter's case: neither the row's key nor its chart moves, \
             and pass 2 re-derives each row's agreement from the edge's current carrier, so \
             a staled row is refused per half-edge on a COMPLETE face. A face whose chart \
             mints nothing stores no minted row to stale; a HALF-MINTED one does, and this \
             pass re-certifies nothing on it \
             (`work/trim/validate-pcurves-never-recertifies-a-face-it-finds-incomplete`) — \
             the pass's property for every content staleness, which dropping rows here \
             would trade for a re-mint on every swap that certifies. A NULL edge's first \
             description is where its halves' rows can first be derived: on a face they are \
             on whose only gaps a null edge holds open, every loop no other null edge holds \
             open is re-minted whole before the door mutates, and on one no null edge is \
             left on, every loop, whatever it missed",
            ),
            (
                "describe_at_rest",
                Neither,
                "`set_edge_curve` with the edge's own carrier and interval put back \
                 verbatim — only the description moves, so not even content staleness \
                 reaches a pcurve",
            ),
            ("set_face_sense", Neither, "writes one `bool`"),
            ("set_surface_source", Neither, "GeomSource metadata"),
            ("set_curve_source", Neither, "GeomSource metadata"),
            ("set_point_source", Neither, "GeomSource metadata"),
            ("clear_geom_sources", Neither, "GeomSource metadata"),
            (
                "mark_imported",
                Neither,
                "origin metadata beside the GeomSource maps (`crate::GeomOrigin`)",
            ),
            (
                "set_surface_field_source",
                Neither,
                "ParamSource metadata: a per-field side record beside the surface",
            ),
            (
                "set_surface_axis_source",
                Neither,
                "axis-channel metadata: a per-component side record beside the surface",
            ),
            (
                "begin_surgery",
                Neither,
                "opens a debug-only surgery scope: no arena key, no pcurve row",
            ),
            ("set_null_face_pair", Neither, "null-face annotation"),
            ("clear_null_face_pair", Neither, "removes that annotation"),
            // ---- Neither: the test-support fixture builders. Why
            // they are in this walk's population is stated once, on
            // [`crate::source_walk::mutation_doors`]. ----
            (
                "prism_ops",
                Neither,
                "grows a prism with `mvfs`/`mev`/`mef` and `set_face_surface`, every one of \
                 them already sorted above; it attaches no pcurve of its own",
            ),
            (
                "describe_as_intersections",
                Completes,
                "`set_edge_curve` per transverse edge, on that entry's terms",
            ),
            (
                "cube_into",
                Neither,
                "`prism_ops` at the unit square then `describe_as_intersections`",
            ),
            (
                "plant_ring_face",
                Neither,
                "`mev_line`, `kemr` and `mef_chord`, every one of them already sorted above",
            ),
            (
                "drill_hole",
                Neither,
                "`plant_ring_face`, then `mev_line`, `mef_chord` and `kfmrh`, every one of \
                 them already sorted above",
            ),
            (
                "plane_every_face",
                Neither,
                "`set_face_surface` per face, on that entry's terms",
            ),
        ]
    };

    /// **The convention at the top of this module, checked rather than
    /// surveyed.** Every public mutation path into a [`crate::Body`] —
    /// `pub fn` taking `&mut self`, plus the free functions taking
    /// `&mut Body<T>` — either re-mints the map in its own body, which
    /// this walk reads directly, or is declared below with its posture
    /// and a note on that door.
    ///
    /// **Why a test and not a list in prose.** A prose index has no way
    /// to notice a door being added, and the previous one did not. This
    /// goes red the day one lands unsorted, which is the rot the prose
    /// could only describe.
    ///
    /// **What it checks, exactly.** Three failures, all mechanical: a
    /// door that neither calls the pass — in either spelling,
    /// `mint_pcurves` or `mint_pcurves_of` — nor appears below; a door
    /// whose entry says anything but `Maintains` while its body calls
    /// it; and an entry naming a door that no longer exists.
    ///
    /// **Where the door set comes from, and what it cannot see:**
    /// [`crate::source_walk::mutation_doors`], shared with the tier-1
    /// postcondition guard in [`crate::review_m1_pr5_internal`], which
    /// walks the same population to ask a different question. That
    /// function's docs carry the reason the two tables do not merge
    /// and the whole inherited blind-spot list; this guard does not
    /// restate either.
    ///
    /// **"Calls `mint_pcurves`" is a read of code, not of prose.** The
    /// body arrives with comments and literals blanked. This guard
    /// used a raw `body.contains`, and a planted door whose body only
    /// *mentioned* `mint_pcurves(` in a comment was counted as
    /// re-minting, in both this guard and the tier-1 one, both green.
    ///
    /// **What it does not check.** That a `Maintains` entry is TRUE. A
    /// re-mint reached through a delegate, and an Euler operator's
    /// site mint, are invisible to a source read, so every `Maintains`
    /// entry in `DECLARED` is taken at its word — the guard
    /// establishes that every door is sorted and that no door has
    /// silently started minting, not that each sort is correct.
    ///
    /// **Nor which SPELLING of the pass a door calls, nor what it
    /// passes.** A `mint_pcurves_of(` call reads as `Maintains` here
    /// whatever face list it is handed — including an empty one — and
    /// the subset pass's guarantee is strictly weaker than the
    /// whole-body pass's: it cannot reach a row whose half-edge is
    /// dead ([`mint_pcurves_of`]'s docs). So a door that KILLS a
    /// half-edge and closes with the subset pass is classified
    /// `Maintains` by this walk and is not. Nothing in this crate does
    /// today, and a source read has no way to tell; what covers it is
    /// the entry's own text and the SHELL-10 probe rows in `sweep`. The
    /// module docs' *"what the guard does NOT establish"* list carries
    /// this and the rest of the blind spot: delegation, and everything
    /// outside `topo/src`'s `&mut Body` surface. The full inherited
    /// list is on [`crate::source_walk::mutation_doors`].
    #[test]
    fn every_mutation_door_declares_its_pcurve_posture() {
        use Posture::Maintains;
        let mut minting: Vec<String> = Vec::new();
        let mut declared: Vec<&str> = Vec::new();
        let mut undeclared: Vec<String> = Vec::new();
        let mut mislabelled: Vec<String> = Vec::new();

        for door in crate::source_walk::mutation_doors() {
            let entry = DECLARED.iter().find(|(n, _, _)| *n == door.name);
            if door.names("mint_pcurves") || door.names("mint_pcurves_of") {
                if let Some((_, posture, _)) = entry.filter(|(_, p, _)| *p != Maintains) {
                    mislabelled.push(format!("{} declared {posture:?}", door.name));
                }
                minting.push(door.name);
            } else if let Some((n, _, _)) = entry {
                declared.push(n);
            } else {
                undeclared.push(door.site());
            }
        }

        assert!(
            undeclared.is_empty(),
            "public mutation path(s) that neither re-mint the pcurve map nor declare a \
             posture in this test: {undeclared:?}. Either re-mint before returning, or add \
             the door above with the posture it holds.",
        );
        assert!(
            mislabelled.is_empty(),
            "door(s) whose body now calls `mint_pcurves` but whose entry says otherwise: \
             {mislabelled:?}. This is the rot the prose index suffered — move the entry to \
             `Maintains`, or drop it and let the walk classify the door.",
        );
        // The entries rot in the other direction too.
        for (name, _, _) in DECLARED {
            assert!(
                declared.contains(name) || minting.iter().any(|n| n == name),
                "this test declares a posture for `{name}`, which is no longer a public \
                 mutation path — it was renamed or deleted. Drop the entry.",
            );
        }
        // **Over-stripping is SILENT here**, which is why this pin is
        // by name rather than a count. A door that stops reading as
        // calling `mint_pcurves` does not red — it falls into the
        // `else if let Some(entry)` arm and is accepted as declared.
        // Only a door with no entry at all reds, and today exactly one
        // door is classified by its body, so a lexing gap that erased
        // the needle everywhere would leave this guard green over a
        // surface it had stopped reading. The walk's own floor is
        // upstream on `mutation_doors`; this is the needle's.
        //
        // **It is exactly one door wide, and that is the whole of it.**
        // It does not cover the `DECLARED` doors: those land in the
        // same `else if` arm whether or not their needle survives, so a
        // declared non-`Maintains` door that STARTS minting while its
        // call is over-stripped would not reach `mislabelled`. Closing
        // that needs a second oracle for "does this body call it",
        // which a source read does not have.
        assert!(
            minting.iter().any(|n| n == "merge_coplanar_faces_declared"),
            "`merge_coplanar_faces_declared` no longer reads as calling `mint_pcurves`. \
             Either the door stopped re-minting — a finding, and its entry belongs below \
             — or the source read lost the call.",
        );
        println!(
            "[pcurve posture] {} door(s): {} re-mint, {} declared",
            declared.len() + minting.len(),
            minting.len(),
            declared.len(),
        );
    }
}

/// The chart-stretch meter rows: the arms these loop-continuity
/// margins are metred by, and the direction each claim needs.
#[cfg(test)]
mod stretch_meter {
    #![allow(clippy::unwrap_used, clippy::float_cmp)]

    use super::{ChartArm, DescribedChart, Singular, SupSpeed, chart_u_arm, singular_at, v_meter};
    use geom::{NurbsSurface, Surface};
    use geom_core::k_stats::decide;
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Margin, Point3, Sign, Vec3};
    use std::sync::Arc;

    /// The band every row here pins explicitly: `zero = 1e-9`,
    /// `escalate = 1e-8`. Pinned rather than read from the run's
    /// tolerance so the digits below mean the same thing at every ε
    /// the sweep runs.
    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// A bilinear NURBS chart on `[0, 1]²` whose image is a flat
    /// `span × span` metre square: `S(u, v) = (span·u, span·v, 0)`,
    /// so `|S_u| = |S_v| = span` EVERYWHERE — the chart's metre
    /// stretch is exactly `span`, not 1.
    fn flat_chart(span: f64) -> Surface<f64> {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, span, 0.0),
            Point3::new(span, 0.0, 0.0),
            Point3::new(span, span, 0.0),
        ];
        Surface::Nurbs(Arc::new(
            NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap(),
        ))
    }

    fn chart(s: &Surface<f64>) -> DescribedChart<'_, f64> {
        DescribedChart::of(s).unwrap()
    }

    fn plane() -> Surface<f64> {
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// **The red-first row, azimuth channel.** A 100× chart maps a
    /// `1e-10` chart-unit u gap to `1e-8` metres — a DEFINITE
    /// discontinuity at the pinned band. Metred by 1 it reads `1e-10`
    /// and certifies `Zero`: the loop closes on a gap the kernel can
    /// see. The arm is the whole of the defect.
    #[test]
    fn a_stretched_nurbs_chart_meters_its_azimuth_gap_in_metres() {
        let s = flat_chart(100.0);
        let arm = chart_u_arm(chart(&s), 0.0);
        assert!(
            matches!(arm, ChartArm::Rate(_)),
            "a spline chart's u gap is a parameter span, so the arm is a rate"
        );
        assert_eq!(
            arm.magnitude(),
            100.0,
            "the chart's own metre stretch, not 1"
        );
        let gap = 1e-10;
        assert_eq!(
            decide("pcurve_loop_continuity", arm.meter(gap), band()),
            Ok(Sign::Positive),
            "1e-10 chart units × 100 m/unit = 1e-8 m, at the escalate edge"
        );
        assert_eq!(
            decide(
                "pcurve_loop_continuity",
                ChartArm::Rate(SupSpeed::new(1.0)).meter(gap),
                band()
            ),
            Ok(Sign::Zero),
            "the under-stated arm certifies the same loop closed"
        );
    }

    /// **The red-first row, second channel.** Same digits, `v_meter`.
    #[test]
    fn a_stretched_nurbs_chart_meters_its_second_channel_in_metres() {
        let s = flat_chart(100.0);
        let meter = v_meter(chart(&s));
        assert_eq!(meter.get(), 100.0);
        let gap = 1e-10;
        assert_eq!(
            decide(
                "pcurve_loop_continuity",
                Margin::metered_sup(gap, meter),
                band()
            ),
            Ok(Sign::Positive)
        );
        assert_eq!(
            decide(
                "pcurve_loop_continuity",
                Margin::metered_sup(gap, SupSpeed::new(1.0)),
                band()
            ),
            Ok(Sign::Zero)
        );
    }

    /// **The scale twin.** The same loop at a uniform 1e3 scale: the
    /// arm scales exactly with the model, so a chart gap that was
    /// `1e-8` m is `1e-5` m — the metering carries the scale rather
    /// than fixing a metre size into the chart.
    #[test]
    fn the_stretch_arm_carries_a_uniform_scale() {
        let small = flat_chart(100.0);
        let large = flat_chart(100.0e3);
        assert_eq!(
            chart_u_arm(chart(&large), 0.0).magnitude(),
            chart_u_arm(chart(&small), 0.0).magnitude() * 1e3
        );
        assert_eq!(
            v_meter(chart(&large)).get(),
            v_meter(chart(&small)).get() * 1e3
        );
    }

    /// **The plane arm is 1 by construction, not by default**, and
    /// stays bit-identical: a plane chart's u and v ARE metres.
    #[test]
    fn a_plane_chart_keeps_its_exact_unit_arms() {
        let p = plane();
        assert_eq!(chart_u_arm(chart(&p), 0.0).magnitude(), 1.0);
        assert_eq!(chart_u_arm(chart(&p), 0.7).magnitude(), 1.0);
        assert_eq!(v_meter(chart(&p)).get(), 1.0);
    }

    /// **Three-outcome posture on the newly-honest arm.** A chart gap
    /// whose metred size lands INSIDE the band escalates typed rather
    /// than picking a side: `5e-11 × 100 = 5e-9 ∈ (1e-9, 1e-8)`.
    #[test]
    fn an_in_band_metred_gap_escalates_rather_than_deciding() {
        let s = flat_chart(100.0);
        let arm = chart_u_arm(chart(&s), 0.0);
        assert!(
            decide("pcurve_loop_continuity", arm.meter(5e-11), band()).is_err(),
            "in-band residue is the third outcome, not a verdict"
        );
        assert_eq!(
            decide("pcurve_loop_continuity", arm.meter(5e-12), band()),
            Ok(Sign::Zero),
            "5e-10 m is honestly closed"
        );
    }

    /// **The singular gate answers all three ways**, through
    /// [`singular_at`], the one door the walk reads. On a spline chart
    /// it is net-level (the chart's whole `u` stretch, whatever the
    /// vertex); on a sphere it is the vertex's 3-D distance to the
    /// nearer pole.
    #[test]
    fn the_singular_gate_answers_all_three_ways() {
        let anywhere = Point3::new(0.3, 0.4, 0.0);
        let read = |s: &Surface<f64>, at: Point3<f64>| singular_at(chart(s), at, band());
        // A chart whose whole u stretch is 1e-12 m per chart unit: no u
        // displacement on it moves a point past the band.
        assert!(
            matches!(read(&flat_chart(1e-12), anywhere), Singular::On),
            "a sub-band chart stretch is a collapsed lever"
        );
        for span in [5e-9_f64, 2e-9] {
            assert!(
                matches!(read(&flat_chart(span), anywhere), Singular::Undecided(_)),
                "an in-band stretch ({span:e}) escalates rather than deciding"
            );
        }
        assert!(matches!(read(&flat_chart(100.0), anywhere), Singular::Off));
        // The sphere: on a pole, within the band of one, and clear of both.
        let sphere: Surface<f64> = Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 2.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        assert!(matches!(
            read(&sphere, Point3::new(0.0, 0.0, -2.0)),
            Singular::On
        ));
        assert!(matches!(
            read(&sphere, Point3::new(5e-9, 0.0, 2.0)),
            Singular::Undecided(_)
        ));
        assert!(matches!(
            read(&sphere, Point3::new(2.0, 0.0, 0.0)),
            Singular::Off
        ));
        // A cylinder has no singular set, so it decides nothing, even on
        // its axis.
        let cylinder: Surface<f64> = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        assert!(matches!(
            read(&cylinder, Point3::new(0.0, 0.0, 0.0)),
            Singular::Off
        ));
    }

    /// A placeholder payload has no net to bound, so it has no arms:
    /// it is not a [`DescribedChart`], and these meters cannot be
    /// asked about it.
    #[test]
    fn a_placeholder_chart_is_not_a_described_chart() {
        let s: Surface<f64> = Surface::Nurbs(Arc::new(NurbsSurface::placeholder()));
        assert!(DescribedChart::of(&s).is_none());
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod recourse_tests {
    use super::PcurveMintError;
    use crate::entity::{FaceKey, HalfEdgeKey, LoopKey};
    use geom_brep::PcurveCertifyError;
    use geom_core::predicate::{Band, BandError, COINCIDENCE_RECOURSE};
    use geom_core::{Indeterminate, MarginDiag};

    /// **The recourse claim for the carrier tier 3 renders whole.**
    /// `ValidationError::Pcurve { finding }` is literally
    /// `"tier 3: {finding}"`, and four more wrappers (`ShellError`,
    /// `TransformError`, `ReplaceFaceError`, `MergeCoplanarError`)
    /// contribute a phrase each, so whatever this enum fails to say is
    /// absent from the message a user reads at five doors.
    ///
    /// **Three arms are asserted differently, because they render a
    /// carrier whole and contribute no prose of their own.** The rule
    /// is that an arm is checked TRANSITIVELY only where the carrier
    /// has an enforcement row of its own:
    ///
    /// - `Escalated` carries an `Indeterminate`, whose `Display` ends
    ///   in [`COINCIDENCE_RECOURSE`] on every margin arm, so the
    ///   recourse is asserted directly.
    /// - `Band` carries a `BandError`, whose own arms are covered by
    ///   `geom_core`'s `every_band_error_arm_names_a_recourse`; what is
    ///   asserted here is the delegation itself — that the arm renders
    ///   the carrier whole rather than summarising it.
    /// - `Certify` carries a `PcurveCertifyError`, which has **no such
    ///   row**, so the chain's claim is unproved at that hop and this
    ///   row does not pretend otherwise: it asserts the delegation and
    ///   nothing about the recourse.
    ///
    /// **A floor, not a proof**, on the terms
    /// `every_chart_region_arm_names_a_recourse` states: a vocabulary
    /// check cannot tell a recourse from a sentence containing one of
    /// its words, and an arm whose recourse uses a word not listed
    /// fails it honestly — extend the list in the same change. What it
    /// catches is the arm added with no second clause at all. The
    /// payloads below are keys, which render verb-free, so what the row
    /// measures is the variant's own clause.
    #[test]
    fn every_pcurve_mint_error_arm_names_a_recourse() {
        // A vocabulary, not a part-of-speech test: an arm that points
        // at a named lever rather than using an imperative satisfies
        // the claim the same way.
        const RECOURSE_WORDS: &[&str] = &[
            "read", "repair", "re-mint", "ask", "hold", "describe", "report",
        ];
        let cause = Indeterminate {
            margin: MarginDiag::value(5e-9),
            band: Band::new(1e-9, 1e-8).unwrap(),
            predicate: Some("pcurve_recourse_probe"),
            terminal_sliver: false,
        };
        let band_error = BandError::Empty {
            zero: 1e-8,
            escalate: 1e-9,
        };
        let certify_error = PcurveCertifyError::CarrierOffChart {
            chart: geom::SurfaceKind::Sphere,
            carrier: geom::CurveKind::Line,
            why: "a sphere holds no line",
        };
        let arms = [
            PcurveMintError::Corrupt,
            PcurveMintError::Certify {
                half_edge: HalfEdgeKey::default(),
                error: certify_error.clone(),
            },
            PcurveMintError::LoopDiscontinuity {
                half_edge: HalfEdgeKey::default(),
            },
            PcurveMintError::LoopNotClosed {
                face: FaceKey::default(),
            },
            PcurveMintError::SingularChartJoint {
                face: FaceKey::default(),
                r#loop: LoopKey::default(),
                half_edge: HalfEdgeKey::default(),
            },
            PcurveMintError::OuterSpansPeriod,
            PcurveMintError::LoopWraps {
                face: FaceKey::default(),
                r#loop: LoopKey::default(),
            },
            PcurveMintError::MissingCache {
                half_edge: HalfEdgeKey::default(),
            },
            PcurveMintError::Unminted {
                face: FaceKey::default(),
            },
            PcurveMintError::RowInterval {
                half_edge: HalfEdgeKey::default(),
            },
            PcurveMintError::Escalated {
                half_edge: HalfEdgeKey::default(),
                cause,
            },
            PcurveMintError::Band(band_error),
            PcurveMintError::UncertifiedImage {
                half_edge: HalfEdgeKey::default(),
            },
            PcurveMintError::PlaceholderChart {
                face: FaceKey::default(),
            },
        ];
        assert_eq!(arms.len(), 14, "an arm was added without a row here");
        for arm in &arms {
            let msg = arm.to_string();
            match arm {
                PcurveMintError::Escalated { .. } => {
                    assert!(msg.contains(COINCIDENCE_RECOURSE), "{msg}");
                }
                PcurveMintError::Band(_) => {
                    assert!(msg.contains(&band_error.to_string()), "{msg}");
                }
                PcurveMintError::Certify { .. } => {
                    assert!(msg.contains(&certify_error.to_string()), "{msg}");
                }
                _ => {
                    let lower = msg.to_lowercase();
                    assert!(
                        RECOURSE_WORDS.iter().any(|w| lower.contains(w)),
                        "no recourse in: {msg}"
                    );
                }
            }
        }
    }
}

/// **The fitted refusal raised before any check.** The mint's
/// general-image derivation meets an absent door before an image
/// exists, so the refusal it carries must not claim a check ran.
///
/// Called directly, not through a body: the one arm that reaches it is
/// an `Intersection` seam on a spline chart, and attaching that edge
/// needs the plane × NURBS lane, which only a certifying scalar holds
/// (`AtRestPolicy::nurbs_lane` is `None` at a dual), so no `Dual64`
/// body carries one.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod derive_without_a_door {
    use super::{PcurveMintError, derive_general_image};
    use crate::entity::HalfEdgeKey;
    use geom::{NurbsCurve3, NurbsSurface};
    use geom_brep::PcurveCertifyError;
    use geom_core::spline::KnotVector;
    use geom_core::{Dual64, Point3, Real};

    #[test]
    fn a_dual_general_image_refuses_before_any_check_and_says_only_that() {
        let lift = Dual64::from_f64;
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let chart = NurbsSurface::new(
            kv.clone(),
            kv.clone(),
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
            ],
            vec![1.0; 4],
        )
        .unwrap()
        .map_scalar(lift);
        let carrier = NurbsCurve3::new(
            kv,
            vec![Point3::new(0.5, 0.0, 0.0), Point3::new(0.5, 1.0, 0.0)],
            vec![1.0; 2],
        )
        .unwrap()
        .map_scalar(lift);
        let err = derive_general_image(&carrier, &chart, HalfEdgeKey::default()).unwrap_err();
        let PcurveMintError::Certify {
            error: PcurveCertifyError::FittedLaneUnsupported { scalar: "dual" },
            ..
        } = err
        else {
            panic!("a dual derives no general image, and says so by name: {err:?}");
        };
        let text = err.to_string();
        assert!(
            text.contains("dual scalar")
                && text.contains("certification rights")
                && !text.contains("check"),
            "the refusal names the dual and who holds the door, and claims no check ran: {text}"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::float_cmp, clippy::panic)]
mod polar_shift_tests {
    use super::shift_polar_branch;
    use geom_brep::{Pcurve, SpiricImage};
    use geom_core::{Point2, Vec2};

    /// **The meridional branch shift, at its own door.** The wall arm
    /// this lane added is the `v` twin of `Pcurve::shift_branch`, and
    /// a body cannot exercise it: a spiric rim's parameter span is the
    /// revolved PROFILE arc's, so the one spiric-bearing body's rims
    /// span 1.78 rad and the loop walk's `k` is 0 on every row it has.
    /// The arm's value is therefore pinned HERE, directly, rather than
    /// left resting on a shift nothing computes — which is exactly the
    /// state a planted `k·period → 0` survived.
    ///
    /// What the row asserts, per variant: a wall's `v0` takes the
    /// whole-period shift and its `u0`, `sense` and the three carrier
    /// scalars do not; a harmonic image's `p0.y` takes it; a CAP image
    /// does not move at all (a plane chart has no periodic channel) —
    /// and neither does any other variant.
    #[test]
    fn the_meridional_shift_moves_a_wall_images_v0_and_nothing_else() {
        let period = core::f64::consts::TAU;
        let wall = Pcurve::Spiric {
            major: 0.09375,
            minor: 0.0703125,
            offset: 0.0078125,
            image: SpiricImage::Wall {
                u0: 0.25,
                v0: 0.5,
                sense: -1.0,
            },
        };
        let Pcurve::Spiric {
            major,
            minor,
            offset,
            image: SpiricImage::Wall { u0, v0, sense },
        } = shift_polar_branch(&wall, 3.0, period)
        else {
            panic!("the wall arm keeps its variant and its image kind");
        };
        assert_eq!(v0, 0.5 + 3.0 * period, "v0 takes the whole-period shift");
        assert_eq!(u0, 0.25, "the azimuth constant is the other door's");
        assert_eq!(sense, -1.0, "the sign is not a branch");
        assert_eq!((major, minor, offset), (0.09375, 0.0703125, 0.0078125));

        // A cap lives on a plane chart, which has no periodic channel:
        // the shift is meaningless there and the image is answered as
        // it was, not moved.
        let cap = Pcurve::Spiric {
            major: 0.09375,
            minor: 0.0703125,
            offset: 0.0078125,
            image: SpiricImage::Cap {
                p0: Point2::new(0.1, 0.2),
                pm: Vec2::new(1.0, 0.0),
                pa: Vec2::new(0.0, 0.0703125),
            },
        };
        assert_eq!(
            format!("{:?}", shift_polar_branch(&cap, 3.0, period)),
            format!("{cap:?}"),
            "a plane chart's image has no meridional branch to shift"
        );

        // The harmonic arm, unchanged by this lane and asserted beside
        // the new one so the two cannot drift apart unnoticed.
        let harmonic = Pcurve::Harmonic {
            p0: Point2::new(0.3, 0.4),
            pa: Vec2::new(1.0, 0.25),
            pb: Vec2::new(-0.5, 1.0),
            pl: Vec2::new(0.125, -0.375),
        };
        let Pcurve::Harmonic { p0, pa, pb, pl } = shift_polar_branch(&harmonic, 3.0, period) else {
            panic!("the harmonic arm keeps its variant");
        };
        assert_eq!(p0.y, 0.4 + 3.0 * period);
        assert_eq!(p0.x, 0.3);
        // `Vec2` is deliberately not `PartialEq` (a geometric vector
        // is not a thing this kernel compares with `==`), so the three
        // untouched coefficients are read componentwise.
        assert_eq!(
            [pa.x, pa.y, pb.x, pb.y, pl.x, pl.y],
            [1.0, 0.25, -0.5, 1.0, 0.125, -0.375],
            "the trigonometric and linear coefficients are not a branch"
        );
    }
}

#[cfg(test)]
mod lift_rows {
    //! [`super::lift_joint`]'s decisions, each on a joint built to sit
    //! where one of them is the only thing deciding. Every row runs at
    //! `f64` and at `Interval`, under a band pinned here (`zero = ε`),
    //! so the digits mean the same at every ε row.
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::{DescribedChart, Lift, PinMiss, Singular, lift_joint};
    use geom::Surface;
    use geom_brep::Pcurve;
    use geom_core::{Band, Decide, Interval, Point2, Point3, Vec2, Vec3};

    const EPS: f64 = 1e-9;

    fn band(k: f64) -> Band {
        Band::new(EPS, k * EPS).unwrap()
    }

    fn unit_sphere<T: Decide>() -> Surface<T> {
        let f = T::from_f64;
        Surface::Sphere {
            center: Point3::new(f(0.0), f(0.0), f(0.0)),
            radius: f(1.0),
            axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
            u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
        }
    }

    fn torus<T: Decide>() -> Surface<T> {
        let f = T::from_f64;
        Surface::Torus {
            center: Point3::new(f(0.0), f(0.0), f(0.0)),
            axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
            major_radius: f(2.0),
            minor_radius: f(1.0),
            u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
        }
    }

    /// A row that sits at one chart point: what a joint reads of it is
    /// its entry, and its twin.
    fn at<T: Decide>(u: f64, v: f64) -> Pcurve<T> {
        let f = T::from_f64;
        let zero = Vec2::new(f(0.0), f(0.0));
        Pcurve::Harmonic {
            p0: Point2::new(f(u), f(v)),
            pa: zero,
            pb: zero,
            pl: zero,
        }
    }

    fn on_sphere<T: Decide>(u: f64, v: f64) -> Point3<T> {
        let f = T::from_f64;
        Point3::new(f(v.cos() * u.cos()), f(v.cos() * u.sin()), f(v.sin()))
    }

    /// The joint at `(u, v)` against a predecessor exit `prev`, on
    /// `surface`, with `vertex` the joint's point.
    fn lift<T: Decide>(
        surface: &Surface<T>,
        (u, v): (f64, f64),
        prev: (f64, f64),
        vertex: Point3<T>,
        k: f64,
    ) -> Result<Lift<T>, PinMiss> {
        let f = T::from_f64;
        lift_joint(
            DescribedChart::of(surface).unwrap(),
            at(u, v),
            f(0.0),
            Point2::new(f(prev.0), f(prev.1)),
            vertex,
            Some(T::tau()),
            band(k),
        )
    }

    /// **A joint whose predecessor sits on the image's TWIN is the
    /// twin, at every K** — the reproduction both reviews of the first
    /// cut made. The predecessor's exit is the twin of `(0.3, 0.4)`,
    /// `(0.3 + π, π − 0.4)`, moved `δ = ±3.5ε` in metres along the
    /// azimuth: inside the 4ε joint bound, and past `K·ε` at `K = 3` and
    /// `K = 2`. Read as a whole number of periods, the image's own
    /// azimuth then decides a branch half a period off and never
    /// reaches the twin; read as half periods, the twin has a quarter
    /// period of room. A twin is never the identity.
    fn twin_at_every_k<T: Decide>(lane: &str) {
        let s = unit_sphere::<T>();
        let (u, v): (f64, f64) = (0.3, 0.4);
        for k in [10.0, 3.0, 2.0] {
            for delta in [-3.5 * EPS, 3.5 * EPS] {
                let du = delta / v.cos();
                let prev = (u + core::f64::consts::PI + du, core::f64::consts::PI - v);
                let Ok(lift) = lift(&s, (u, v), prev, on_sphere(u, v), k) else {
                    panic!("{lane} K = {k} δ = {delta:e}: the joint lifts");
                };
                assert!(
                    lift.twin && lift.ku == 0 && lift.kv == 0,
                    "{lane} K = {k} δ = {delta:e}: the twin, no period ({}, {}, {})",
                    lift.twin,
                    lift.ku,
                    lift.kv
                );
                assert!(
                    !lift.identity(),
                    "{lane} K = {k}: a twin is never the identity"
                );
            }
        }
    }

    #[test]
    fn a_twin_joint_is_the_twin_at_every_k() {
        twin_at_every_k::<f64>("f64");
        twin_at_every_k::<Interval>("Interval");
    }

    /// **A joint whose vertex is within the band of a pole, at a gap no
    /// orbit point meets, escalates.** The vertex is `5ε` from the north
    /// pole (incidence undecided), so its lever is `5ε`; the gap is one
    /// radian, so no mark decides, and none of the orbit points the
    /// near-pole reading may take (the image, the twin either way) is
    /// within ε of it at that lever. Taking the image there would be a
    /// branch no margin saw.
    fn near_pole_off_orbit_escalates<T: Decide>(lane: &str) {
        let s = unit_sphere::<T>();
        let theta: f64 = 5e-9;
        let v = core::f64::consts::FRAC_PI_2 - theta;
        let out = lift(&s, (0.0, v), (1.0, v), on_sphere(0.0, v), 10.0);
        assert!(
            matches!(out, Err(PinMiss::Escalated(_))),
            "{lane}: the incidence's escalation, not a lift: {:?}",
            out.as_ref().map(|l| (l.twin, l.ku, l.kv)).ok()
        );
    }

    #[test]
    fn a_near_pole_joint_off_every_orbit_point_escalates() {
        near_pole_off_orbit_escalates::<f64>("f64");
        near_pole_off_orbit_escalates::<Interval>("Interval");
    }

    /// **An undecided incidence still decides the branch where its marks
    /// decide.** The vertex is `9ε` from the pole (undecided), and the
    /// gap is a whole period: every mark it meets is past the band at
    /// that lever, so the joint lifts one period over. Skipping the
    /// azimuth there, as on the singular set, would read it as the
    /// identity.
    fn undecided_incidence_still_lifts<T: Decide>(lane: &str) {
        let s = unit_sphere::<T>();
        let v = core::f64::consts::FRAC_PI_2 - 9e-9;
        let out = lift(
            &s,
            (0.0, v),
            (core::f64::consts::TAU, v),
            on_sphere(0.0, v),
            10.0,
        );
        let Ok(lift) = out else {
            panic!("{lane}: the marks decide")
        };
        assert!(matches!(lift.singular, Singular::Undecided(_)), "{lane}");
        assert!(
            !lift.twin && lift.ku == 1 && !lift.identity(),
            "{lane}: one period over ({}, {})",
            lift.twin,
            lift.ku
        );
    }

    #[test]
    fn an_undecided_incidence_still_decides_the_period() {
        undecided_incidence_still_lifts::<f64>("f64");
        undecided_incidence_still_lifts::<Interval>("Interval");
    }

    /// **The azimuth's marks are metered at the vertex's own lever.** At
    /// colatitude `1e-3` the lever is `1e-3` m per radian, not the
    /// sphere's radius: a gap `5e-6` rad past the quarter-period mark is
    /// `5e-9` m there, inside the band, so the joint escalates. Metered
    /// at the radius it would read `5e-6` m and decide.
    fn marks_at_the_vertex_lever<T: Decide>(lane: &str) {
        let s = unit_sphere::<T>();
        let v = core::f64::consts::FRAC_PI_2 - 1e-3;
        let gap = core::f64::consts::FRAC_PI_2 + 5e-6;
        let out = lift(&s, (0.0, v), (gap, v), on_sphere(0.0, v), 10.0);
        assert!(
            matches!(out, Err(PinMiss::Escalated(_))),
            "{lane}: the mark is undecided at the vertex's lever"
        );
    }

    #[test]
    fn the_marks_are_metered_at_the_vertex_lever() {
        marks_at_the_vertex_lever::<f64>("f64");
        marks_at_the_vertex_lever::<Interval>("Interval");
    }

    /// **The second channel's period is part of the deck element.** On a
    /// torus a predecessor one meridian period over is the same point,
    /// and the joint between them is not the identity.
    fn meridian_period<T: Decide>(lane: &str) {
        let s = torus::<T>();
        let (u, v): (f64, f64) = (0.3, 0.4);
        let f = T::from_f64;
        let r = 2.0 + v.cos();
        let vertex = Point3::new(f(r * u.cos()), f(r * u.sin()), f(v.sin()));
        let Ok(lift) = lift(&s, (u, v), (u, v + core::f64::consts::TAU), vertex, 10.0) else {
            panic!("{lane}: the joint lifts")
        };
        assert!(
            lift.ku == 0 && lift.kv == 1 && !lift.identity(),
            "{lane}: one meridian period ({}, {})",
            lift.ku,
            lift.kv
        );
    }

    #[test]
    fn a_meridian_period_is_not_the_identity() {
        meridian_period::<f64>("f64");
        meridian_period::<Interval>("Interval");
    }

    /// **The windings a loop may close with** ([`Lift::closes`]), the
    /// whole table: at most one period on each channel, independently,
    /// and the sphere's twin alone or with one period beyond it.
    #[test]
    fn the_closing_windings_are_these() {
        let row = |twin: bool, ku: i32, kv: i32| Lift::<f64> {
            pcurve: at(0.0, 0.0),
            twin,
            ku,
            kv,
            singular: Singular::Off,
        };
        let table: &[((bool, i32, i32), bool)] = &[
            ((false, 0, 0), true),
            ((false, 1, 0), true),
            ((false, -1, 0), true),
            ((false, 0, 1), true),
            ((false, 1, 1), true),
            ((false, -1, -1), true),
            ((false, 2, 0), false),
            ((false, -2, 0), false),
            ((false, 0, 2), false),
            ((false, 4, 0), false),
            ((true, 0, 0), true),
            ((true, -1, 0), true),
            ((true, 1, 0), true),
            ((true, -2, 0), true),
            ((true, 2, 0), false),
            ((true, -3, 0), false),
            ((true, 0, 2), false),
        ];
        for &((twin, ku, kv), closes) in table {
            assert_eq!(
                row(twin, ku, kv).closes(),
                closes,
                "twin {twin}, ku {ku}, kv {kv}"
            );
        }
    }
}
