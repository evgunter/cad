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
//! A pcurve row belongs to an (edge, face-side) incidence, and the
//! half-edge is exactly that incidence. A row is two things: the
//! edge's IMAGE in the face's chart — its certified chart curve, a
//! function of the edge and the chart alone ([`crate::Body::pcurve`])
//! — and the half-edge's JOINT ELEMENT, the deck transformation that
//! carries its image onto the end of its predecessor's image
//! ([`crate::Body::joint`], [`crate::joint`]). Seam edges are the forcing
//! case and the reason no coarser key works: a full cylinder wall
//! closed by its seam meridian has **both half-edges of one edge in
//! one loop of one face**, on one surface, with one image and two
//! joint elements, which the loop's lift places at `u = α` and
//! `u = α + 2π`. "Per edge" cannot hold the two joints; "per (edge,
//! face)" cannot either. Per half-edge has no special case.
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
//!   polar nor meridian, or with its class reading in band) and a spline
//!   carrier on any analytic chart have no closed form and store their
//!   projected image ([`geom_brep::Pcurve::Projected`], `ψ(C(t))` in the
//!   chart frame), derived and certified by
//!   [`geom_brep::chart_pcurve_over`] (`analytic_derive`). A cone's
//!   tilted section and a torus's Villarceau circle take their exact
//!   focal-section image ([`geom_brep::Pcurve::FocalSection`]). A carrier
//!   its face's incidence test shows off it
//!   ([`PcurveCertifyError::CarrierOffChart`]) is refused. A circle on a
//!   cone or torus whose class reading is in band takes no general lane
//!   (none is written for the pair) and refuses as that reading's
//!   escalation, as does a line or ellipse whose class reading is in
//!   band; on a cone every circle the chart holds is a rim, so a circle
//!   that is none is off the chart or in that band. Each refusal is owed
//!   like any other: the producer refuses, and at rest it is a check-8
//!   finding, never excused ([`not_owed`] excuses only a scalar with no
//!   fitted door refusing a fitted image; `odd_cone_circle` pins the
//!   cone's).
//! - **Described NURBS charts mint** their iso lane (M6-3,
//!   `nurbs_iso_derive`) — RATIONAL ones too since M8-3, whose ARC cap
//!   rims map through the chart's own rational-quadratic parameter
//!   (`Pcurve::IsoArc`). Only the mvfs placeholder mints nothing: it
//!   is not a described surface.
//!
//! # The loop walk: images and joint elements (spec §3)
//!
//! A [`geom_brep::Pcurve`] cannot express a branch jump: its azimuth
//! channel is `α + β·t` with one stored `α`. So a row stores the image
//! on the branch its derivation answers, and the branch a loop's chain
//! takes is integers: each joint's ELEMENT, the whole number of periods
//! per angular channel (and, on a sphere, the involution twin) that
//! carries the half-edge's image onto its predecessor's exit
//! ([`decide_joint`]). The element is decided between two images at
//! the joint's vertex, so it is a function of the two edges and the
//! chart, and **no stored byte depends on which half-edge is a loop's
//! `first`**. Two chart points with one 3-D image differ by a deck
//! element of the chart, so the decision is of integers, with half the
//! step to the next orbit point as room; a joint no representation
//! lifts refuses typed ([`PcurveMintError::LoopDiscontinuity`]). The
//! joint's 3-D coincidence is not decided again in the chart: it
//! follows from the two rows' envelopes and the edge certificate's
//! endpoint pinning. At a joint whose vertex is on the chart's singular
//! set — a pole, an apex, decided as 3-D incidence — the azimuth has no
//! lever and its periods are not decided: the element is the reset
//! marker, which carries the rest ([`JointElement::Reset`]). A spline
//! chart's joint also states its chart-space gap.
//!
//! A loop's **lift** — the chain of chart curves its readers draw — is
//! its images moved by the elements summed along it ([`loop_lift`]),
//! and its **winding** is the elements composed once around
//! ([`Winding`]). The loop must close: its winding a whole period of
//! the first channel at most (none counted across a reset), of a
//! second channel the chart is closed in (an angle, or a net's `v`
//! knot domain) at most, or — on a sphere — the involution
//! ([`Winding::closes`]), read off conjugation invariants, so the
//! verdict is the same from every member.
//!
//! This is the M2 PR 5 meridian finding generalized:
//! "the junction's meridian column unwrapped nearest prev_u, but past
//! 3π/2 the wrong branch is closer"). The fix there was to anchor the
//! unwrap to a point that is *exact by construction* rather than
//! nearest-previous per sample; here the anchor is the loop's own
//! chain of shared vertices, the per-sample choice does not exist at
//! all, and the anchor's correctness is decided rather than assumed.
//!
//! # The loop windings
//!
//! [`loop_windings`] is the walk's second consumer: it runs the same
//! [`walk_loop`] on a chart the CALLER names and reads off each loop
//! whether its lift closes. It mints nothing and stores nothing.
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
//! loop, and on a face whose rows are COMPLETE they mint, before they
//! mutate, what they create on the loops they rewire — the new halves'
//! images and the elements of the joints they make — and every other
//! row of those loops stands ([`site_rows`], which states the rule and
//! its two edges: a spline
//! chart refuses typed, and a face the closed-form lane cannot mint as
//! the surgery leaves it stores nothing). A face that stores no row is
//! the minting pass's and stays rowless. `mev_null` adds halves too,
//! and cannot mint them: its edge has no carrier, so the loop it joins
//! cannot be walked while the edge is on it. So the site mint takes
//! two more kinds of minted face ([`StoredRows::remints`]): one whose
//! only gaps are on loops a null edge holds open, and one its door
//! takes the last null edge off, whatever it misses. It leaves a loop
//! a null edge still holds open as found — its new halves rowless — and
//! mints what is missing on every loop its door rewires and leaves
//! running through no null edge. The door that releases a loop is either an operator
//! that rewires it out from under the null edge — the boolean's and the
//! splitting lane's join, whose chord `mef`s leave the section's null
//! halves on the sliver between the chords — or the edge's first
//! description ([`crate::Body::set_edge_curve`](crate::construction::Construct::set_edge_curve), or a
//! [`crate::Body::kev_describing`](crate::construction::Construct::kev_describing) that lists it), which walks every
//! loop of the face and mints what it misses; on a spline chart either
//! leaves the face as found.
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
//! [`crate::Body::split_edge`](crate::construction::Construct::split_edge) carries rows across a parameter split:
//! each child's row is its own edge's, so a closed-form or projected
//! image is derived again over the child's sub-interval and placed in
//! the chart's principal window as the mint places it (C4), and a
//! fitted or general image is the parent's restricted to the
//! sub-interval. The op certifies both children and decides the joints
//! they make before it mutates ([`split_cache`]), and writes them onto
//! the parent halves and the two new halves, minting nothing where
//! there was nothing. The carry is exact on every chart; two readers of
//! the split body are narrower, both stated at [`split_cache`]. On a
//! described-NURBS wall [`mint_pcurves`] — the recovery step this
//! module's caveats name for a face left rowless — refuses, because
//! [`nurbs_iso_derive`]'s rim arms map an edge's whole carrier
//! interval onto the chart's whole `u` domain, which a sub-edge no
//! longer spans; so the claim that a carried row is the row the pass
//! would derive does not reach the iso rows there (it does reach a
//! carried `General` row, which the pass re-derives bit for bit). And
//! the trimmed quadrature lane refuses a carried `General` child's
//! sub-range typed.
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
//! producer's closing mint disposes of them. Each joint element moves
//! onto its source predecessor, inverted — the same joint read from
//! the other side — and no loop's `first` moves (the joint bullet in
//! `revert`'s module docs). Tier 3 of a reverted body whose faces
//! carry rows reports nothing but `NegativeVolume`; `sweep`'s
//! `revert_periodic_wrap` rows pin that on the two-arc sphere's cavity
//! and a cone through its apex.
//!
//! The **loop-re-parenting** doors hold the same posture for a
//! different reason: [`crate::Body::kfmrh`](crate::construction::Construct::kfmrh),
//! [`crate::Body::mfkrh`](crate::construction::Construct::mfkrh) and [`crate::Body::ring_move`](crate::construction::Construct::ring_move) move a whole
//! LOOP between faces, which changes the CHART every row on that loop
//! is stated in while changing no key. Each carries the moved loop's
//! rows where the two faces are on one CHART
//! ([`crate::Body::same_chart`]) and drops them where they are not
//! ([`crate::Body::drop_rows`]). Where the moved rows do not stand on
//! the destination — dropped, or missing — and the destination was
//! complete, a re-mint is owed, and each door answers it at one of two
//! doors, as the kill family does ([`crate::Body::kev`](crate::construction::Construct::kev) and
//! [`crate::Body::kev_describing`](crate::construction::Construct::kev_describing)): the keys-only door refuses
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
//! faces' loops rather than a whole loop — [`crate::Body::mef`](crate::construction::Construct::mef)'s
//! chord surgery and [`crate::Body::kef`](crate::construction::Construct::kef)'s unsplice — and take the
//! same answer over the run; `staleness_posture::DECLARED` carries
//! each door's note.
//!
//! [`crate::Body::set_face_surface`](crate::construction::Construct::set_face_surface) reaches the same posture from the
//! other side: nothing moves, and the CHART moves under every row the
//! face stores at once. It carries them across a swap onto the same
//! chart and drops them on any other ([`crate::Body::drop_rows`], over
//! the loops [`crate::Body::face_cycles`] proves), which is what makes its own
//! declaration below true as written — a swap onto a plane or a
//! placeholder used to leave a COMPLETE row set stated in the chart
//! the face left, and this pass skips exactly that face.
//!
//! **Completes the map** — [`crate::Body::set_edge_curve`](crate::construction::Construct::set_edge_curve) and
//! [`crate::Body::kev_describing`](crate::construction::Construct::kev_describing)'s listed members. A carrier swap is
//! the surface setter's sibling, NOT the same case: it moves neither
//! the row's key nor its chart, and pass 2 re-derives every row's
//! agreement from the edge's current carrier, so what it stales is
//! refused loud, on a complete face and on the rows a half-minted one
//! stores alike. These doors re-mint the faces whose rows their
//! description moves and keep the rows they find everywhere else, as a
//! `Neither` door does. A null edge's first description is the first
//! door that can derive the rows of its halves; a certified edge's
//! rows are stated over the carrier and interval `set_edge_curve`
//! replaces; and a kill that lists a member moves its end with its
//! carrier, null or certified, so the rows its halves store would span
//! the interval the end moved from. On each face such a half is on, as the door leaves
//! it, and that the site mint selects ([`StoredRows::remints`]), the
//! door re-mints every loop no other null edge holds open
//! ([`site_rows`]) — the whole face, once no null edge is left on it. A
//! certified edge's face on a spline chart is left as found.
//!
//! **A kill that takes the last null edge off a loop releases it**, and
//! the rows the loop missed while it was held open are owed there
//! ([`releases_a_gap`]). The kills answer at two doors,
//! as the loop-re-parenting doors do: [`crate::Body::kev`](crate::construction::Construct::kev),
//! [`crate::Body::kef`](crate::construction::Construct::kef) and [`crate::Body::kemr`](crate::construction::Construct::kemr) take no band and
//! refuse [`SiteRowRefusal::KeysOnly`] before they mutate, and their
//! band twins — [`crate::Body::kev_describing`](crate::construction::Construct::kev_describing),
//! [`crate::Body::kef_minting`](crate::construction::Construct::kef_minting) and [`crate::Body::kemr_minting`](crate::construction::Construct::kemr_minting) — run
//! the site mint over the loops they release, planned before they mutate
//! ([`crate::Body::plan_released_rows`]), so the face leaves complete, or
//! storing nothing where the closed-form lane cannot mint it. A face the
//! site mint does not select, or one on a spline chart, is left as found
//! at either door.
//!
//! **Neither clears nor re-mints** — the Euler operators that add no
//! half-edge to an existing loop (`mvfs`, `kemr`), the null-edge `mev`
//! (whose scaffolding has no carrier to derive a row from), and the
//! kill ops other than [`crate::Body::kev_describing`](crate::construction::Construct::kev_describing) and the band
//! twins (above). These are primitives: they keep every image they
//! find, and on each joint they make they write its element keys-only
//! and exactly. A kill sums
//! the elements of the joints it bridges ([`crate::Body::kev`](crate::construction::Construct::kev),
//! [`crate::Body::kef`](crate::construction::Construct::kef), [`crate::Body::kemr`](crate::construction::Construct::kemr); [`JointElement::then`]):
//! an edge's two halves share one image, so where one ends the other
//! begins, and the sum carries the survivor's image onto the
//! predecessor's exit exactly as the pass's decision there does. A null
//! edge is a point with no image: its halves carry the identity, and
//! the joint out of one keeps the element read across the point, so a
//! kill of it sums the same way. So on a face whose rows are the pass's,
//! a kill leaves the pass's rows, byte for byte.
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
    BranchMiss, Pcurve, PcurveCache, PcurveCertifyError, Period, WalkStop, chart_pcurve_over,
};
use geom_brep::{first_within, probe_pick};
use geom_core::Tol;
use geom_core::k_stats::decide;
use geom_core::predicate::{Band, BandError};
use geom_core::{Decide, Indeterminate, Margin, Point2, Real, Sign, SupSpeed};
use std::borrow::Cow;

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdge, HalfEdgeKey, LoopKey};
use crate::joint::{Deck, JointElement, Winding};
use crate::live::{linked, proven};
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
    /// A key the caller passed does not resolve in this body: `role`
    /// names the argument. Checked at the door, before any work.
    Stale {
        /// The argument the key was passed as.
        role: &'static str,
        /// The key.
        key: EntityId,
    },
    /// `half_edge`'s edge is null scaffolding
    /// ([`crate::null::CurveGeom::NullScaffold`], minted by
    /// [`crate::Body::mev_null`](crate::construction::Construct::mev_null)): it has no carrier yet, so there is
    /// no chart image to derive over it. Scaffolding is a legal state
    /// mid-surgery; tier 2 refuses it at rest.
    NoCarrier {
        /// The half-edge whose edge has no carrier.
        half_edge: HalfEdgeKey,
    },
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
    /// A joint of a loop sits off the chart's singular set, but so near
    /// the axis that the walk decided its azimuth integer with less room
    /// than the joint bound: half the step to the next point of the
    /// joint's orbit, as the chord it spans at the vertex's own distance
    /// from the axis less the ends' `2ε`, is not definitely past the
    /// band's escalation four times over (`loop_windings_joint_room`).
    /// There a mark may name an orbit point other than the joint's own
    /// (both lifts of the one 3-D point), so the loop's winding may read
    /// zero while the loop truly winds. Raised by [`loop_windings`] alone,
    /// on a sphere near a pole or a cone near its apex.
    JointWithoutRoom {
        /// The face whose loop holds the joint.
        face: FaceKey,
        /// The loop.
        r#loop: LoopKey,
        /// The half-edge whose ENTRY is the joint.
        half_edge: HalfEdgeKey,
    },
    /// A half-edge carries no stored pcurve at rest although its face
    /// carries some: the face's cache set is incomplete, a defect. A
    /// face that stores none is [`PcurveMintError::Unminted`], or the
    /// refusal its derivation meets; a planar face stores none by
    /// construction, and at a scalar with no fitted door a face only
    /// that door can image stores none ([`not_owed`]).
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
    /// - [`crate::Body::mev_null`](crate::construction::Construct::mev_null), whose scaffolding edge has no
    ///   carrier to derive a row from. The loop it joins misses the
    ///   edge's two rows, and those of any half-edge an operator adds
    ///   to the loop meanwhile, while a null edge holds it open; the
    ///   door that releases the loop — an operator rewiring it out from
    ///   under the edge, as the pipelines' joins do, or the edge's first
    ///   description ([`crate::Body::set_edge_curve`](crate::construction::Construct::set_edge_curve), or a
    ///   [`crate::Body::kev_describing`](crate::construction::Construct::kev_describing) that lists it), or the band kill
    ///   of the edge ([`crate::Body::plan_released_rows`]) — mints it
    ///   whole, except on a spline chart;
    /// - a caller's own [`crate::Body::detach_pcurve`](crate::construction::Construct::detach_pcurve);
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
    /// The joint into a half-edge lies on a spline net closed in `v`
    /// whose whole `v` sweep moves a point neither definitely past the
    /// band nor definitely under it (`pcurve_loop_v_stretch`), so the
    /// joint's `v` periods are undecided and no element is written: a
    /// net whose `v` sections are within the band's sliver of collapsing
    /// to a curve. Refused under this name, not as a reset, because a
    /// reset keeps the element modulo the first channel's periods and
    /// would carry an undecided `v` while dropping a decided `u`.
    NetStretchUndecided {
        /// The half-edge whose ENTRY is the joint.
        half_edge: HalfEdgeKey,
        /// The gate's diagnostic.
        cause: Indeterminate,
    },
    /// The run's linear band could not be built.
    Band(BandError),
    /// The chart [`loop_windings`] was handed is the placeholder
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

impl PcurveMintError {
    /// **The ending this refusal gives at `at`**, for a door that reports
    /// it under its own sentence (the shell op's closing mint): a nested
    /// certification refusal's decision's ([`PcurveCertifyError::ending`]);
    /// an undecided reading's "not yet", as the at-rest table ends it
    /// (`validate::classify_pcurve`); and the lever of a refusal a lever
    /// answers (a joint too near a pole, a band the tolerance cannot form,
    /// a placeholder chart). `None` for a torn arm (a key that does not
    /// resolve, a null carrier, a loop that does not meet or close, a row
    /// missing, misstated or unminted): on a body the kernel built, the
    /// door's defect.
    #[must_use]
    pub fn ending(&self, at: impl Into<geom_brep::recourse::ReadAt>) -> Option<String> {
        let at = at.into();
        match self {
            Self::Certify { error, .. } => error.ending(at),
            Self::Escalated { cause, .. } | Self::NetStretchUndecided { cause, .. } => Some(
                geom_brep::recourse::not_yet(geom_brep::recourse::RefusedArm::Undecided(cause), at),
            ),
            Self::JointWithoutRoom { .. } => Some(JOINT_WITHOUT_ROOM_RECOURSE.to_owned()),
            Self::Band(_) => Some(crate::validate::TOLERANCE.to_owned()),
            Self::PlaceholderChart { .. } => Some(PLACEHOLDER_RECOURSE.to_owned()),
            Self::Stale { .. }
            | Self::NoCarrier { .. }
            | Self::LoopDiscontinuity { .. }
            | Self::LoopNotClosed { .. }
            | Self::MissingCache { .. }
            | Self::Unminted { .. }
            | Self::RowInterval { .. } => None,
        }
    }

    /// This refusal's text read at `at`: a certification refusal nested
    /// whole renders at that door ([`PcurveCertifyError::render`]).
    #[must_use]
    pub fn render(&self, at: impl Into<geom_brep::recourse::ReadAt>) -> String {
        match self {
            Self::Certify { half_edge, error } => format!(
                "the pcurve of half-edge {half_edge:?} failed certification: {}",
                error.render(at)
            ),
            _ => core::fmt::from_fn(|f| self.write_payload(f)).to_string(),
        }
    }

    /// Every arm's text but a nested certification refusal's, which no
    /// door reads differently.
    fn write_payload(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Stale { role, key } => write!(
                f,
                "the {role} argument, {key}, does not resolve in this body"
            ),
            Self::NoCarrier { half_edge } => write!(
                f,
                "the edge of half-edge {half_edge:?} is null scaffolding with no carrier yet, \
                 so it has no chart image. Recourse: describe the edge \
                 (`Body::set_edge_curve`), then ask again"
            ),
            Self::Certify { .. } => unreachable!("`render` writes a nested certification refusal"),
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
            Self::JointWithoutRoom {
                face,
                r#loop: lp,
                half_edge,
            } => write!(
                f,
                "loop {lp:?} of face {face:?} passes so near a sphere pole or cone apex at \
                 half-edge {half_edge:?} that its azimuth period is decided with less room than \
                 the joint bound, so its winding is not read; there is nothing in the body to \
                 repair. {JOINT_WITHOUT_ROOM_RECOURSE}"
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
            Self::NetStretchUndecided { half_edge, cause } => write!(
                f,
                "the joint into half-edge {half_edge:?} lies on a spline net closed in v whose \
                 v sections are too close to collapsing to decide its v periods: {cause}"
            ),
            Self::Band(e) => write!(f, "{e}"),
            Self::PlaceholderChart { face } => write!(
                f,
                "the chart offered for face {face:?}: {}, so nothing on it can be metred. \
                 {PLACEHOLDER_RECOURSE}",
                geom::PLACEHOLDER_SURFACE
            ),
        }
    }
}

/// Read at a build: the pcurve pass a body's own operation runs.
impl core::fmt::Display for PcurveMintError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.render(geom_brep::recourse::Reading::Build))
    }
}

impl std::error::Error for PcurveMintError {}

impl PcurveMintError {
    /// This refusal as a kernel driver receives it from a pass it called
    /// with keys it read out of the body: passed through, where it is a
    /// fact about the body's geometry.
    ///
    /// # Panics
    ///
    /// On [`PcurveMintError::Stale`]: a driver's keys are ones it read
    /// out of the body, so they resolve, and one that does not is a
    /// kernel bug and not the user's input.
    #[must_use]
    #[track_caller]
    pub(crate) fn for_driver(self) -> Self {
        if let Self::Stale { role, key } = self {
            unreachable!(
                "a kernel driver passed a pcurve pass a {role} key, {key}, that does not \
                 resolve: a driver's keys are ones it read out of the body, so they resolve"
            );
        }
        self
    }
}

/// The recourse for a joint decided with less room than the joint bound
/// ([`PcurveMintError::JointWithoutRoom`]).
const JOINT_WITHOUT_ROOM_RECOURSE: &str = "Recourse: ask about a face whose loops keep farther \
     from the pole or apex, or ask at a smaller ε";

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
/// skip the face (a placeholder mints nothing), and [`loop_windings`]
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
/// [`PcurveMintError::Stale`] when `half_edge` does not resolve,
/// [`PcurveMintError::NoCarrier`] when its edge is null scaffolding,
/// or a typed chart refusal for a frontier chart/carrier kind.
pub fn pcurve_of<T: AtRestPolicy>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
    band: Band,
) -> Result<Pcurve<T>, PcurveMintError> {
    if !body.half_edges.contains_key(half_edge) {
        return Err(PcurveMintError::Stale {
            role: "half_edge",
            key: EntityId::HalfEdge(half_edge),
        });
    }
    if let Some(cache) = body.pcurve(half_edge) {
        return Ok(cache.pcurve().clone());
    }
    let (carrier, t0, t1) = half_edge_carrier(body, half_edge)?;
    let surface = half_edge_surface(body, half_edge);
    // A SPLINE chart's images are description-driven (M6-3) — the iso
    // derivation, not the closed-form harmonic table. An approximating
    // surface's chart IS its fit's, so it takes the same route.
    if surface.spline_chart().is_some() {
        return nurbs_iso_derive(body, half_edge, &surface, band);
    }
    analytic_derive(&carrier, t0, t1, &surface, band, half_edge)
}

/// **A transverse seam's `Intersection`, its selector chosen by D2's
/// question: is the curve known in advance?** Where both faces are spline
/// walls and the edge's image on each ([`pcurve_of`]) is a row on its
/// chart's domain side that certifies there, the curve is that shared row
/// ([`geom_brep::IntersectionDraft::shared_row`]); otherwise the draft's
/// witness. Built for boundary rows (the reader's scope, not D2's rule).
/// `sides` are the seam's two half-edges, each with its face's surface
/// key and surface.
pub(crate) fn seam_intersection<T: AtRestPolicy>(
    body: &Body<T>,
    draft: geom_brep::IntersectionDraft<T>,
    sides: [(HalfEdgeKey, geom_brep::SurfaceKey, &Surface<T>); 2],
    band: Band,
) -> geom_brep::DraftedIntersection<T> {
    let [(ha, ka, wa), (hb, kb, wb)] = sides;
    let pair = geom_brep::SurfacePair::new(ka, kb);
    let spline = wa.spline_chart().is_some() && wb.spline_chart().is_some();
    let row = spline
        .then(|| {
            let (ia, ib) = (
                pcurve_of(body, ha, band).ok()?,
                pcurve_of(body, hb, band).ok()?,
            );
            draft.shared_row(pair, [(ka, wa, ia), (kb, wb, ib)], band)
        })
        .flatten();
    match row {
        Some(selector) => draft.into_selected(pair, selector),
        None => draft.into_intersection(pair),
    }
}

/// The chart image of a carrier on an ANALYTIC chart, on the chart's
/// principal branch: the closed form ([`chart_pcurve_over`]) wherever
/// one exists, and the projected image for a carrier with none (a spline
/// carrier, a sphere's general circle), partitioned over the edge's own
/// interval.
///
/// # Errors
///
/// [`PcurveMintError::Certify`] with the derivation's refusal.
fn analytic_derive<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    surface: &Surface<T>,
    band: Band,
    half_edge: HalfEdgeKey,
) -> Result<Pcurve<T>, PcurveMintError> {
    chart_pcurve_over(carrier, t0, t1, surface, band)
        .map_err(|error| PcurveMintError::Certify { half_edge, error })
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
    let (start, end) = edge_vertex_points(body, he);
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
#[track_caller]
fn edge_vertex_points<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> (geom_core::Point3<T>, geom_core::Point3<T>) {
    let (edge, data) = edge_record(body, half_edge);
    let point = |he, field| {
        let start = linked(
            &body.half_edges,
            he,
            EntityId::HalfEdge,
            EntityId::Edge(edge),
            field,
        )
        .start;
        body.linked_vertex_point(start, EntityId::HalfEdge(he), "start")
    };
    (
        point(data.he_plus, "he_plus"),
        point(data.he_minus, "he_minus"),
    )
}

/// The point of the vertex `half_edge` starts at: the joint where the
/// loop enters it.
#[track_caller]
fn entry_vertex<T: Real>(body: &Body<T>, half_edge: HalfEdgeKey) -> geom_core::Point3<T> {
    let start = half_edge_record(body, half_edge).start;
    body.linked_vertex_point(start, EntityId::HalfEdge(half_edge), "start")
}

/// `he`'s record. Every key this module reads a record through is one
/// a door resolved or one a record of the body holds, so a miss is a
/// kernel bug and panics ([`proven`]).
#[track_caller]
fn half_edge_record<T: Real>(body: &Body<T>, he: HalfEdgeKey) -> &HalfEdge {
    proven(&body.half_edges, he, EntityId::HalfEdge)
}

/// `he`'s edge and its record, a link `he`'s record holds.
#[track_caller]
fn edge_record<T: Real>(body: &Body<T>, he: HalfEdgeKey) -> (EdgeKey, &crate::entity::Edge) {
    let edge = half_edge_record(body, he).edge;
    let data = linked(
        &body.edges,
        edge,
        EntityId::Edge,
        EntityId::HalfEdge(he),
        "edge",
    );
    (edge, data)
}

/// The face `he` bounds and its record: links of `he`'s record and of
/// its loop's.
#[track_caller]
pub(crate) fn half_edge_face<T: Real>(
    body: &Body<T>,
    he: HalfEdgeKey,
) -> (FaceKey, &crate::entity::Face) {
    let lp = half_edge_record(body, he).parent_loop;
    let face = linked(
        &body.loops,
        lp,
        EntityId::Loop,
        EntityId::HalfEdge(he),
        "parent_loop",
    )
    .face;
    let data = linked(
        &body.faces,
        face,
        EntityId::Face,
        EntityId::Loop(lp),
        "face",
    );
    (face, data)
}

/// The certified curve of `half_edge`'s edge.
///
/// # Errors
///
/// [`PcurveMintError::NoCarrier`] where the edge is null scaffolding.
#[track_caller]
fn half_edge_curve<T: Real>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<&geom_brep::EdgeCurve<T>, PcurveMintError> {
    let (edge, data) = edge_record(body, half_edge);
    body.edge_curve_linked(edge, data)
        .certified()
        .ok_or(PcurveMintError::NoCarrier { half_edge })
}

/// The certified carrier and parameter interval of `half_edge`'s edge.
///
/// # Errors
///
/// [`half_edge_curve`]'s.
#[track_caller]
fn half_edge_carrier<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<(geom::Curve3<T>, T, T), PcurveMintError> {
    let curve = half_edge_curve(body, half_edge)?;
    let (t0, t1) = curve.params();
    Ok((curve.carrier().clone(), t0, t1))
}

/// The surface of the face `half_edge` bounds.
#[track_caller]
fn half_edge_surface<T: Decide>(body: &Body<T>, half_edge: HalfEdgeKey) -> Surface<T> {
    let (face, data) = half_edge_face(body, half_edge);
    body.face_surface_linked(face, data).clone()
}

/// The surface KEY of the face `half_edge` bounds (the key twin of
/// [`half_edge_surface`], for the iso derivation's own-side test).
#[track_caller]
fn half_edge_surface_key<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> geom_brep::SurfaceKey {
    half_edge_face(body, half_edge).1.surface
}

/// The certified description of `half_edge`'s edge.
///
/// # Errors
///
/// [`half_edge_curve`]'s.
#[track_caller]
fn half_edge_description<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<geom_brep::EdgeDescription<T>, PcurveMintError> {
    Ok(half_edge_curve(body, half_edge)?.description().clone())
}

/// Which ways a carrier over `carrier` can run a chart row over `row`
/// in the row's own spline space — the relation the seam certificate
/// reads, per direction: forward on the same knots, run back on an
/// exact reflection of them ([`KnotVector::is_reflection_of`]). A row
/// candidate is offered only in a direction this admits.
///
/// [`KnotVector::is_reflection_of`]: geom_core::spline::KnotVector::is_reflection_of
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RowSpace {
    forward: bool,
    backward: bool,
}

impl RowSpace {
    fn of(carrier: &geom_core::spline::KnotVector, row: &geom_core::spline::KnotVector) -> Self {
        RowSpace {
            forward: carrier.degree() == row.degree() && carrier.knots() == row.knots(),
            backward: row.is_reflection_of(carrier),
        }
    }

    /// Whether a carrier whose `u` rises (`rising`) or falls along it
    /// is in this space.
    fn admits(self, rising: bool) -> bool {
        if rising { self.forward } else { self.backward }
    }
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
/// - An iso LINE image naming the OTHER wall maps as a column of this
///   chart carrying the neighbour's own `v` map — a domain end, or the
///   column the start's certified chart foot measures (a chart wider
///   than the face it trims) — where one holds the carrier at the
///   probe ([`probe_pick`]); otherwise the carrier takes the
///   boundary-iso schedule below, as an `Intersection` seam does, down
///   to its `General` image. Every pick is then CERTIFIED by the full
///   iso lane.
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
///   ([`Pcurve::IsoArc`], M8-3), its boundary row and `u` direction
///   picked together at the probe ([`probe_pick`]): the whole
///   row over the edge's interval, then the row over the arc it
///   measures, which a rim covering part of the row (a sub-edge) takes. Both mapped
///   description forms the kernel mints for a circle reach it — see the
///   arm's own note.
/// - An [`geom_brep::EdgeDescription::Intersection`] over a spline or
///   line carrier, and a LINE carrier in any other form, that lies on a
///   boundary column maps as that column: the same iso line the
///   `IsoCurve` arm mints, recovered from the carrier because the
///   description names no chart coordinate; one on a boundary row (a
///   cap–wall rim) maps as that row. The residency is a pick over the
///   columns and rows and their two directions, each judged at the
///   edge's ends and a quarter span in ([`probe_pick`]), definite or
///   escalated; then over the
///   row, and for a line the column, whose moving map the carrier's
///   end feet measure (a sub-edge, a cap rim an offset moved, and for
///   a line an interior column). An Intersection circle is offered to
///   the arc rim, and kept where the rim certifies. Any other locus or
///   carrier — a SPLINE on an interior
///   column is the executed case (#498), an off-iso straight edge
///   another — takes U2's `General`
///   curve-in-UV arm at the honest Fitted grade, derived from the
///   wall's own foot schedule at degree 1: it certifies where the foot
///   path is straight to ε and refuses at the hull
///   (`FittedCertificate`) where a curved carrier bends it.
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
            let ((a, b), (c, d)) = (
                payload.knot_values_u().domain(),
                payload.knot_values_v().domain(),
            );
            (a, b, c, d)
        }
        None => (T::zero(), T::one(), T::zero(), T::one()),
    };
    // The ARC-RIM class: a circle on one of the chart's boundary rows,
    // over the whole row or part of it, its image the row on the
    // chart's own rational-quadratic parameter (`Pcurve::IsoArc`). An
    // IsoUnsupported refusal says this carrier is not such a rim.
    let arc_rim = || -> Result<Pcurve<T>, PcurveMintError> {
        // The row and the `u` direction are one pick, and the arc the
        // closed form or the row measures, the incidence lane's too
        // ([`geom_brep::pick_arc_rim`]); the full arc-rim certification
        // still follows.
        match geom_brep::pick_arc_rim(&carrier, (t0, span), surface, band) {
            Ok(Some(cand)) => Ok(cand),
            Err(cause) => Err(PcurveMintError::Escalated { half_edge, cause }),
            Ok(None) => Err(refuse(
                "no boundary row of this chart, in either u direction, holds the circle \
                 at the probe — not an arc rim of this chart",
            )),
        }
    };

    // The boundary-iso schedule (the arms' own note, below), named by
    // the probe's predicate label. `stated` leads its closed form: the
    // candidates a description itself states (the wall-seam arm's
    // columns), judged by the same probe.
    let iso_schedule = |what: &'static str,
                        stated: Vec<Pcurve<T>>|
     -> Result<Pcurve<T>, PcurveMintError> {
        let Some(wall) = surface.spline_chart() else {
            unreachable!(
                "nurbs_iso_derive: every caller routes only a chart with a spline payload \
                 here"
            )
        };
        let line = matches!(carrier, geom::Curve3::Line { .. });
        // The iso candidates are offered to the carriers the iso
        // class certifies: a spline and a line. A circle's exact
        // class is the arc rim, tried below; an ellipse or a spiric
        // has none on a spline chart.
        let iso = line || matches!(carrier, geom::Curve3::Nurbs(_));
        let columns = [cu0, cu1].into_iter().filter(|_| iso).flat_map(|x| {
            [(cv0, cv1), (cv1, cv0)].map(|(v_at_t0, v_at_t1)| {
                let slope = (v_at_t1 - v_at_t0) / span;
                Pcurve::IsoLine {
                    p0: Point2::new(x, v_at_t0 - slope * t0),
                    pl: Vec2::new(T::zero(), slope),
                }
            })
        });
        // A cap–wall rim traverses a boundary ROW, `u` moving —
        // offered to a spline only in a direction whose carrier is in
        // the row's own spline space (`RowSpace`: the row class
        // compares control nets over it), and to a line in both, whose
        // class reads no net.
        let row_space = match &carrier {
            geom::Curve3::Nurbs(spline) => {
                RowSpace::of(spline.knots_f64_interim(), wall.knots_u_f64_interim())
            }
            geom::Curve3::Line { .. } => RowSpace {
                forward: true,
                backward: true,
            },
            _ => RowSpace {
                forward: false,
                backward: false,
            },
        };
        let directions: Vec<(T, T)> = [(true, (cu0, cu1)), (false, (cu1, cu0))]
            .into_iter()
            .filter_map(|(rising, ends)| row_space.admits(rising).then_some(ends))
            .collect();
        let rows = [cv0, cv1].into_iter().flat_map(|y| {
            directions.iter().map(move |&(u_at_t0, u_at_t1)| {
                let slope = (u_at_t1 - u_at_t0) / span;
                Pcurve::IsoLine {
                    p0: Point2::new(u_at_t0 - slope * t0, y),
                    pl: Vec2::new(slope, T::zero()),
                }
            })
        });
        let mut deferred = match probe_pick(
            what,
            &carrier,
            (t0, span),
            surface,
            band,
            stated.into_iter().chain(columns).chain(rows),
        ) {
            Ok(Some(cand)) => return Ok(cand),
            Ok(None) => None,
            Err(cause) => Some(cause),
        };
        // ---- The fixed schedule found nothing: MEASURE the map. ----
        // The candidates above assume the carrier traverses the chart's
        // WHOLE domain along its moving axis, because that is what a
        // natively built wall's seam or rim does. An edge that covers
        // PART of a chart does not — a sub-edge a split left, a cap rim
        // an offset moved into the chart's interior, a chart wider than
        // the face it trims — so the carrier's two end feet measure the
        // moving channel's map, and the fixed channel is a boundary
        // where it is definitely one, else the value the start's foot
        // measures. Offered to the same metre-valued probe, so no UV
        // quantity reaches ε (D4 ¶1). A foot that will not converge
        // measures nothing, as at the wall-seam arm: the step is
        // skipped and the schedule goes on.
        //
        // A measured ROW is offered to every carrier the row space
        // admits in its direction. A measured COLUMN only to a line
        // (the `filter` below). A spline's column reaches the exact
        // class only as a STATED candidate (the wall-seam arm's), whose
        // interior or partial column the seam class certifies by de
        // Boor collapse; an `Intersection` spline seam on an interior
        // column takes `General` below (#498)
        // (`work/pcert/a-spline-seam-on-an-interior-column-is-offered-no-measured-column.md`).
        if deferred.is_none()
            && (row_space.forward || row_space.backward)
            && let (Some(f0), Some(f1)) = (
                derive_chart_foot(carrier.eval(t0), surface).ok().flatten(),
                derive_chart_foot(carrier.eval(t1), surface).ok().flatten(),
            )
        {
            let rising = f1.x > f0.x;
            let (f0, f1) = (computed_foot::<T>(f0), computed_foot::<T>(f1));
            let (su, sv) = ((f1.x - f0.x) / span, (f1.y - f0.y) / span);
            let rows = [cv0, cv1, f0.y]
                .into_iter()
                .filter(|_| row_space.admits(rising))
                .map(|y| Pcurve::IsoLine {
                    p0: Point2::new(f0.x - su * t0, y),
                    pl: Vec2::new(su, T::zero()),
                });
            let columns = [cu0, cu1, f0.x]
                .into_iter()
                .filter(|_| line)
                .map(|x| Pcurve::IsoLine {
                    p0: Point2::new(x, f0.y - sv * t0),
                    pl: Vec2::new(T::zero(), sv),
                });
            match probe_pick(
                what,
                &carrier,
                (t0, span),
                surface,
                band,
                rows.chain(columns),
            ) {
                Ok(Some(cand)) => return Ok(cand),
                Ok(None) => {}
                Err(cause) => deferred = Some(cause),
            }
        }
        // A circle on a boundary row is the arc rim's, where the arc-rim
        // certification holds; any other circle, an IsoUnsupported
        // answer, is not, and neither is a pick that certification
        // definitely refuses: the spline-pair edge certificates take a
        // circle carrier, so an `Intersection` circle on a spline chart
        // that is no arc rim falls through to `General` like any other
        // locus.
        if deferred.is_none() && matches!(carrier, geom::Curve3::Circle { .. }) {
            match arc_rim() {
                Err(PcurveMintError::Certify {
                    error: PcurveCertifyError::IsoUnsupported { .. },
                    ..
                }) => {}
                // An undecided certification is the rim's to report:
                // the mint certifies the pick again and escalates it.
                // A definite refusal of the rim's locus says the circle
                // is not this row, and falls through; any other refusal
                // is about the chart or the run, not the candidate, and
                // leaves with its own words.
                Ok(rim) => match PcurveCache::certify(rim.clone(), t0, t1, &carrier, surface, band)
                {
                    Ok(_)
                    | Err(
                        PcurveCertifyError::Escalated { .. }
                        | PcurveCertifyError::FittedEscalated { .. },
                    ) => return Ok(rim),
                    Err(
                        PcurveCertifyError::ResidualExceeded { .. }
                        | PcurveCertifyError::IsoUnsupported { .. },
                    ) => {}
                    Err(error) => return Err(PcurveMintError::Certify { half_edge, error }),
                },
                refused => return refused,
            }
        }
        // ---- Nothing exact holds. Store the image as it is. ----
        // A `General` curve in UV at the honest Fitted grade (U2): a
        // SPLINE on an interior column has no exact class (the measured
        // column above is a line's alone), and a DIAGONAL locus — an
        // off-iso straight edge among them — has none either. Its image is the chart image the edge
        // certificate itself derives (`edge_nurbs::chart_image`, at
        // `PXN_IMAGE_DEGREE`), so it certifies wherever the foot path
        // is straight to ε. A closed-form carrier's degree-1 image
        // refuses at the hull where a circle or ellipse bends the foot
        // path, as the edge certificate's own does.
        //
        // An escalated candidate outranks the image and its refusal
        // alike: a candidate that escalates keeps escalating, and the
        // derived image only ever speaks on a DEFINITE fall-through.
        if let Some(cause) = deferred {
            return Err(PcurveMintError::Escalated { half_edge, cause });
        }
        derive_general_image(&carrier, (t0, t1), wall)
            .map(|image| Pcurve::General(std::sync::Arc::new(image)))
            .map_err(|error| PcurveMintError::Certify { half_edge, error })
    };
    let own = half_edge_surface_key(body, half_edge);
    match half_edge_description(body, half_edge)? {
        // **This face's OWN chart image is the answer.** Since the
        // conventional descriptions collapsed (U2), an edge described
        // as an image in THIS chart carries the image itself — there
        // is nothing left to derive, and re-deriving it would be a
        // second opinion about a locus the description already states.
        geom_brep::EdgeDescription::Chart(ref c) if c.surface == own => Ok(c.pcurve.clone()),
        // **The wall–wall seam stated on the OTHER wall.** An iso LINE
        // image in another chart, read as a column of this one: the
        // stated `v` map is a candidate only, since nothing here says
        // the two charts share the seam's parameterization.
        geom_brep::EdgeDescription::Chart(geom_brep::ChartCurve {
            pcurve: Pcurve::IsoLine { p0, pl },
            ..
        }) => {
            // The neighbour's `v` map, verbatim, as a column of this
            // chart: at each chart boundary, then at the column the
            // start's measured foot names (a chart wider than the face
            // it trims). They lead the boundary-iso schedule, which
            // judges every candidate at the probe, so a stated
            // map this chart does not share, or a stated image that is
            // no column at all, is passed over rather than trusted. A
            // foot that will not converge places no column: that
            // candidate is dropped, and the schedule goes on.
            let column = |x: T| Pcurve::IsoLine {
                p0: Point2::new(x, p0.y),
                pl: Vec2::new(T::zero(), pl.y),
            };
            let at_foot = derive_chart_foot(carrier.eval(t0), surface)
                .ok()
                .flatten()
                .map(|foot| column(computed_foot::<T>(foot).x));
            let stated = [cu0, cu1].map(column).into_iter().chain(at_foot).collect();
            iso_schedule("pcurve_iso_wall_seam", stated)
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
        // that follows, which compares the chart's boundary row
        // against the carrier circle's own rational-quadratic form and
        // refuses a chart that is not this construction.
        geom_brep::EdgeDescription::Chart(_)
        | geom_brep::EdgeDescription::Scaffold(_)
        | geom_brep::EdgeDescription::OnPair { .. }
            if matches!(carrier, geom::Curve3::Circle { .. }) =>
        {
            arc_rim()
        }
        // **The boundary-iso arms: an INTERSECTION seam, and a LINE rim
        // in any other form.** The same wall–wall seam the `IsoCurve`
        // arm maps, stated INTRINSICALLY: the locus is `S₁ ∩ S₂`, and
        // where that locus IS this chart's own `u = u₀`/`u = u₁`
        // boundary column its chart image is the same iso line. The
        // description carries no chart coordinates at all — it names
        // two surfaces and a witness — so the image is recovered from
        // the CARRIER against the chart's own domain. A straight edge
        // described as a chart image or a scaffold is the same question
        // about the same carrier, and takes the same schedule.
        //
        // Keyed on the carrier and on BOUNDARY RESIDENCY, never on the
        // description form or the operand order: which of `s1`/`s2` is
        // the plane is not a fact about the locus, and the same seam
        // stated natively or restated foreign must mint the same image.
        //
        // The pick is ONE fixed schedule (D9): any candidates the
        // description states (the wall-seam arm's columns carrying the
        // neighbour's `v` map), then the chart's two boundary columns
        // and two boundary rows × the two directions the carrier can
        // traverse each, every candidate judged by the probe
        // ([`probe_pick`]): the edge's ends and the point a quarter in. A
        // SELECTION, checked: the full iso certification follows every
        // derivation.
        //
        // **Both directions are offered, and the certification decides.**
        // A LINE carrier's class reads no control net, so a line that
        // traverses the chart's `v` backwards certifies on its backward
        // column. A SPLINE carrier's class compares its net with the
        // column's in one spline space, so a spline traversing `v`
        // backwards certifies on its backward column when its knots are
        // the column's exact reflection, and refuses ("not the chart's
        // own column") otherwise. Either way the candidate offered is the image the
        // carrier actually has, not a forward one that fits the class
        // and states the wrong traversal.
        //
        // A carrier the exact candidates do not claim stores its
        // `General` image.
        geom_brep::EdgeDescription::Chart(_)
        | geom_brep::EdgeDescription::Scaffold(_)
        | geom_brep::EdgeDescription::OnPair { .. }
            if matches!(carrier, geom::Curve3::Line { .. }) =>
        {
            iso_schedule("pcurve_iso_line_rim", Vec::new())
        }
        // An `OnPair` names the same two surfaces an `Intersection`
        // does, and its locus is the same question about the carrier.
        geom_brep::EdgeDescription::Intersection { .. }
        | geom_brep::EdgeDescription::OnPair { .. } => {
            iso_schedule("pcurve_iso_seam_column", Vec::new())
        }
        geom_brep::EdgeDescription::TangentIntersection { .. }
        | geom_brep::EdgeDescription::Chart(_)
        | geom_brep::EdgeDescription::Scaffold(_) => Err(refuse(
            "no iso derivation for this locus on a NURBS chart — only chart images of \
             this chart, iso-line images of the neighbouring wall, boundary-iso \
             Intersection seams, and LINE or CIRCLE cap rims have exact chart images \
             (the trimmed-NURBS pcurve lane is the cut-loft unit's)",
        )),
    }
}

/// The **general curve-in-UV image** of a carrier on this face's own
/// spline chart over the edge's interval `params` — U2's `General`
/// arm, at the honest Fitted grade.
///
/// The producer is `geom_brep`'s one derivation of this object
/// ([`geom_brep::FittedLane::general_image`], whose body is the same
/// `edge_nurbs` foot schedule the plane × NURBS edge certificate uses
/// at adopt time). Nothing is certified here: this returns EVIDENCE,
/// and the mint's next move is `PcurveCache::certify_general`, which
/// bounds `sup_t |S(P(t)) − C(t)|` over the whole span against the
/// chart.
///
/// # Errors
///
/// [`PcurveCertifyError::FittedLaneUnsupported`] at a scalar with no
/// certified lane (a dual body may not certify — D1), or the
/// derivation's own typed refusal.
fn derive_general_image<T: AtRestPolicy>(
    carrier: &geom::Curve3<T>,
    params: (T, T),
    wall: &geom::NurbsSurface<T>,
) -> Result<geom::NurbsCurve2<T>, PcurveCertifyError> {
    let Some(lane) = T::fitted_lane() else {
        return Err(PcurveCertifyError::FittedLaneUnsupported { scalar: T::NAME });
    };
    lane.general_image(carrier, params, wall)
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
/// The lane's refusal when the projection will not converge.
fn derive_chart_foot<T: AtRestPolicy>(
    point: geom_core::Point3<T>,
    surface: &Surface<T>,
) -> Result<Option<geom_core::Point2<f64>>, PcurveCertifyError> {
    let (Some(wall), Some(lane)) = (surface.spline_chart(), T::fitted_lane()) else {
        return Ok(None);
    };
    lane.chart_foot(point, wall).map(Some)
}

/// A chart foot [`derive_chart_foot`] measured, in `T`: Newton's
/// `f64`, so each coordinate enters as a computed value.
fn computed_foot<T: Real>(foot: geom_core::Point2<f64>) -> Point2<T> {
    foot.map(T::from_computed)
}

/// Is `half_edge` the `he_plus` of its edge (so the loop traverses it
/// forward in the carrier parameter)?
#[track_caller]
pub(crate) fn is_plus<T: Decide>(body: &Body<T>, half_edge: HalfEdgeKey) -> bool {
    edge_record(body, half_edge).1.he_plus == half_edge
}

/// The FIRST-CHANNEL arm of a chart — the metres a unit step of the
/// chart's `u` moves the mapped point (D4 ¶1: no UV-space tolerance
/// ever reaches ε), as a sup over the WHOLE chart:
/// `geom_brep::chart_stretch_sup`'s first component. On a plane `u` IS
/// metres, so the arm is exactly 1; on a spline chart `u` is the net's
/// own parameter, and the arm is the net's `sup |S_u|`. A joint on an
/// azimuth chart reads the vertex's own distance from the axis instead
/// ([`joint_arm`]).
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
fn chart_u_arm<T: Real>(chart: DescribedChart<'_, T>) -> SupSpeed<T> {
    // `chart_stretch_sup` refuses only the cone, whose arm grows with
    // `v` and which no caller asks here, and the placeholder, which a
    // [`DescribedChart`] is not.
    let Ok((sup_u, _)) = geom_brep::chart_stretch_sup(chart.surface()) else {
        unreachable!(
            "chart_u_arm is asked on a plane or spline chart only, and DescribedChart::of \
             excludes the placeholder"
        )
    };
    sup_u
}

/// The chart's **u period** — the whole-number shift the loop walk may
/// apply to land a half-edge's entry on its predecessor's exit
/// (`walk_loop`), and the wrap a loop's winding may carry
/// ([`Winding::closes`]).
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
/// singular gate ([`singular_at`]) reads the metres its `u` knot domain
/// moves a point ([`net_reach`]: the domain's length times the net's own
/// `sup |S_u|`, [`chart_u_arm`]) as definitely past the band. A chart
/// whose reach sits under the band takes no shift at all (no `u`
/// displacement on it moves a point past ε), one whose reach lands
/// inside the band takes none either, and the joint's chart-space gap
/// decides; only a definitely-metric chart reaches the branch decision.
///
/// **Why a NURBS chart needs this (#327).** A full-period cylinder
/// wall stated as ONE B-spline patch with a seam generator used twice
/// is the shape every translator writes and the kernel's own band
/// re-mint produces. Its loop walks the chart's whole u range and its
/// seam edge takes the two u-boundary branches — exactly the analytic
/// seam case, but at the chart's own period rather than `τ`. Without
/// this the walk reports the wrap as a discontinuity: a symptom, not
/// the cause.
fn chart_u_period<T: Decide>(surface: &Surface<T>, band: Band) -> Option<Period<T>> {
    // `τ` belongs to the AZIMUTH charts. A spline chart — the
    // payload's or an approximating surface's fit — has whatever period
    // its knot domain says, and only if the net actually closes; handing
    // it `τ` would let the loop walk wrap a chart that does not.
    if matches!(surface, Surface::Plane { .. }) {
        return None;
    }
    let Some(payload) = surface.spline_chart() else {
        return Some(Period::new(T::tau()));
    };
    let (u0, u1) = payload.knot_values_u().domain();
    net_closes(payload, NetChannel::U, band).then(|| Period::new(u1 - u0))
}

/// The chart's SECOND channel period — the `v` companion of
/// [`chart_u_period`] (C4: every closed channel has its own period, which
/// the chart owns). `τ` where `v` is an angle (sphere, torus:
/// [`polar_arm`]); the knot domain's length on a spline net decided
/// closed in `v` ([`net_closes`], `pcurve_chart_v_closed`); `None` on a
/// plane, cylinder or cone, whose `v` is a length, and on an open net.
/// The kind of period it is ([`VPeriod`]) is what levers its marks
/// ([`decide_joint`]).
fn chart_v_period<T: Decide>(surface: &Surface<T>, band: Band) -> Option<VPeriod<T>> {
    if let Some(polar) = polar_arm(surface).map(SupSpeed::new) {
        return Some(VPeriod::Angle {
            period: Period::new(T::tau()),
            polar,
        });
    }
    let payload = surface.spline_chart()?;
    let (v0, v1) = payload.knot_values_v().domain();
    net_closes(payload, NetChannel::V, band).then(|| VPeriod::Net(Period::new(v1 - v0)))
}

/// A spline net's channel, for the closure question [`net_closes`] asks.
#[derive(Clone, Copy)]
enum NetChannel {
    U,
    V,
}

/// **Whether a spline net closes in `channel`**: every control point of
/// its first boundary row (column) is decided at zero distance from the
/// matching one of its last, at `band` (`pcurve_chart_u_closed`,
/// `pcurve_chart_v_closed`). One rule for both channels, so the two
/// periods cannot drift apart.
///
/// Any other answer is "open", an in-band one included; that fold, and
/// a test that compares neither weights nor reads an unclamped knot
/// vector's boundary, are
/// `work/pcert/chart-u-period-folds-an-in-band-net-closure-into-no-period.md`'s,
/// for both channels.
fn net_closes<T: Decide>(payload: &geom::NurbsSurface<T>, channel: NetChannel, band: Band) -> bool {
    let (nu, nv) = payload.control_counts();
    let control = payload.control();
    let (name, pairs): (_, Vec<(usize, usize)>) = match channel {
        NetChannel::U if nu >= 2 && nv > 0 => (
            "pcurve_chart_u_closed",
            (0..nv).map(|iv| (iv, (nu - 1) * nv + iv)).collect(),
        ),
        NetChannel::V if nv >= 2 && nu > 0 => (
            "pcurve_chart_v_closed",
            (0..nu).map(|iu| (iu * nv, iu * nv + nv - 1)).collect(),
        ),
        NetChannel::U | NetChannel::V => return false,
    };
    pairs.into_iter().all(|(i, j)| {
        let (Some(a), Some(b)) = (control.get(i), control.get(j)) else {
            return false;
        };
        matches!(
            decide(name, Margin::of(a.distance(*b)), band),
            Ok(Sign::Zero)
        )
    })
}

/// **The metres a whole sweep of a spline net's `channel` moves a point,
/// at most**: the channel's knot domain length crossed by the net's sup
/// stretch in it ([`chart_u_arm`] in `u`, [`v_meter`] in `v`). The
/// net-level gates ask whether this is under the band
/// (`pcurve_loop_pole_joint` in [`singular_at`], `pcurve_loop_v_stretch`
/// in [`decide_joint`]): where it is, no displacement in the channel
/// anywhere on the chart, a whole period included, moves a point past
/// ε. The stretch alone is a rate (metres per parameter unit), not a
/// length, so it is never compared with the band: under ε on a knot
/// domain 10¹⁰ long it still moves a point metres.
///
/// The sup side ([`Margin::metered_sup`]) is the escape side here, as in
/// [`spline_gap_closes`]: an over-stated reach can only keep a gate from
/// reading the channel as collapsed.
fn net_reach<T: Real>(
    chart: DescribedChart<'_, T>,
    payload: &geom::NurbsSurface<T>,
    channel: NetChannel,
) -> Margin<T> {
    match channel {
        NetChannel::U => {
            let (u0, u1) = payload.knot_values_u().domain();
            Margin::metered_sup(u1 - u0, chart_u_arm(chart))
        }
        NetChannel::V => {
            let (v0, v1) = payload.knot_values_v().domain();
            Margin::metered_sup(v1 - v0, v_meter(chart))
        }
    }
}

/// **The periods the chart owns**, one per channel ([`chart_u_period`],
/// [`chart_v_period`]), decided once where a walk reads its joints
/// ([`ChartPeriods::of`]), and the one band-free reading a reader with no
/// band takes ([`ChartPeriods::stated`]). Built only by those two, so a
/// value never mixes one channel decided at one band with the other at
/// another.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ChartPeriods<T> {
    u: Option<Period<T>>,
    v: Option<VPeriod<T>>,
}

/// The second channel's period, and what levers a gap on it: the polar
/// radius where `v` is an angle, the net's `sup |S_v|` ([`v_meter`]) on a
/// net closed in `v`.
#[derive(Clone, Copy, Debug)]
pub(crate) enum VPeriod<T> {
    /// `v` is an angle (sphere, torus): `τ`, metered by the polar radius
    /// ([`polar_arm`]).
    Angle {
        /// `τ`.
        period: Period<T>,
        /// The polar radius, metres per radian: exact, so a sup.
        polar: SupSpeed<T>,
    },
    /// A spline net closed in `v`: its `v` knot domain's length.
    Net(Period<T>),
}

impl<T: Copy> VPeriod<T> {
    /// The period, whichever kind it is.
    pub(crate) fn period(self) -> Period<T> {
        match self {
            Self::Angle { period, .. } | Self::Net(period) => period,
        }
    }
}

impl<T: Copy> ChartPeriods<T> {
    /// The first channel's period, where the chart has one.
    pub(crate) fn u(self) -> Option<Period<T>> {
        self.u
    }

    /// The second channel's period, where the chart has one.
    pub(crate) fn v(self) -> Option<Period<T>> {
        self.v.map(VPeriod::period)
    }
}

impl<T: Decide> ChartPeriods<T> {
    /// The periods `surface`'s chart owns, its closures decided at `band`.
    pub(crate) fn of(surface: &Surface<T>, band: Band) -> Self {
        Self {
            u: chart_u_period(surface, band),
            v: chart_v_period(surface, band),
        }
    }

    /// The periods an element applies in where no band is at hand
    /// ([`loop_lift`], whose rows and elements a decided walk wrote):
    /// `τ` per angle channel, and a net's knot domain lengths whether or
    /// not a band would decide the net closed. Only a net decided closed
    /// carries an element with periods on it, so on every element a
    /// door wrote this agrees with [`ChartPeriods::of`]. One planted by
    /// hand on an OPEN net applies here, where [`ChartPeriods::of`] would
    /// leave a gap; tier 3 re-decides it ([`validate_pcurves`]), and the
    /// disagreement is
    /// `work/pcert/chart-channel-periods-are-spelled-in-several-places.md`'s.
    pub(crate) fn stated(surface: &Surface<T>) -> Self {
        let net = surface.spline_chart();
        let u = match (surface, net) {
            (Surface::Plane { .. }, _) => None,
            (_, Some(payload)) => {
                let (u0, u1) = payload.knot_values_u().domain();
                Some(Period::new(u1 - u0))
            }
            (_, None) => Some(Period::new(T::tau())),
        };
        let v = match (polar_arm(surface).map(SupSpeed::new), net) {
            (Some(polar), _) => Some(VPeriod::Angle {
                period: Period::new(T::tau()),
                polar,
            }),
            (None, Some(payload)) => {
                let (v0, v1) = payload.knot_values_v().domain();
                Some(VPeriod::Net(Period::new(v1 - v0)))
            }
            (None, None) => None,
        };
        Self { u, v }
    }
}

#[cfg(test)]
impl<T: Decide> ChartPeriods<T> {
    /// A row's chart handed a `u` period of its own (a fixture that
    /// states a period its net is not decided closed in), the `v` period
    /// still the one `surface` owns at `band`, the band the row decides
    /// at.
    pub(crate) fn given_u(u: Option<Period<T>>, surface: &Surface<T>, band: Band) -> Self {
        Self {
            u,
            ..Self::of(surface, band)
        }
    }
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

/// The SECOND channel's metre rate for a loop-continuity gap — the
/// `v` companion of [`chart_u_arm`], and the same escape claim.
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

/// A whole-period shift of the SECOND channel — the `v` twin of
/// [`geom_brep::Pcurve::shift_branch`], for the charts closed in `v`
/// (sphere, torus, a net closed in `v`: [`chart_v_period`]): `image`
/// translated `k` periods along the second channel. A translation's linear part is the
/// identity, so every image form takes it exactly
/// ([`Pcurve::map_affine`]); the walk and the lift read the one map.
pub(crate) fn shift_v_branch<T: Real>(pcurve: &Pcurve<T>, k: i32, period: Period<T>) -> Pcurve<T> {
    pcurve.translate(T::zero(), period.times(k))
}

/// The sphere chart's INVOLUTION twin of an image: `S(u + π, π − v) =
/// S(u, v)` holds identically on a sphere chart (`radial(u+π) =
/// −radial(u)`, `cos(π−v) = −cos v`, `sin(π−v) = sin v`), so every
/// sphere pcurve has exactly two representations and a pole-crossing
/// walk legitimately needs the OTHER one on the far side — a π azimuth
/// step no whole-period shift can produce. The involution is affine
/// with linear part `diag(1, −1)`, which every image form a sphere
/// chart mints takes exactly ([`Pcurve::map_affine`]).
///
/// `None` off a sphere chart — the torus has no such twin (R > 0 breaks
/// the symmetry), and neither does the cone (cos α > 0) — and where
/// [`Pcurve::map_affine`] refuses the involution: a spiric wall image,
/// which no sphere chart mints and whose form has no image of a map
/// negating one channel (`WallReflected`), and a harmonic row whose
/// polar count has no negation in `i32` (`WindingUnnegatable`), which
/// no derivation mints. The walk reads `None` as no second candidate,
/// the lift as an element that does not apply.
pub(crate) fn sphere_twin<T: Real>(surface: &Surface<T>, pcurve: &Pcurve<T>) -> Option<Pcurve<T>> {
    if !matches!(surface, Surface::Sphere { .. }) {
        return None;
    }
    pcurve
        .map_affine(geom_brep::ChartMap::sphere_involution())
        .ok()
}

/// One half-edge's minted chart curve, before certification: the
/// image, its carrier and the carrier interval, and the element of the
/// joint into it, keyed on whatever the walk names its half-edges by — a
/// [`HalfEdgeKey`] in the pass, a [`SiteHalf`] in an Euler operator's
/// plan.
pub(crate) struct Walked<T: Real, K = HalfEdgeKey> {
    key: K,
    carrier: geom::Curve3<T>,
    pcurve: Pcurve<T>,
    t0: T,
    t1: T,
    element: Option<JointElement>,
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
    /// half-edge cycle. A loop whose boundary is not a `Cycle` is
    /// ABSENT from this list: there is nothing to walk and nothing
    /// wrong.
    pub(crate) loops: Vec<(LoopKey, Vec<HalfEdgeKey>)>,
    /// Whether those cycles store any row: `false` when the face's
    /// boundary stores none at all, so the face has not been minted.
    pub(crate) stores: bool,
    /// The gaps in the row set, in walk order: each half-edge of a
    /// loop with no row — no image, or no element on the joint into it
    /// from a predecessor that stores one.
    pub(crate) gaps: Vec<RowGap>,
}

/// One gap in a face's row set ([`StoredRows::gaps`]): a half-edge
/// that stores no image, or no element on the joint into it from a
/// predecessor that stores one.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RowGap {
    /// The half-edge.
    pub(crate) half_edge: HalfEdgeKey,
    /// The loop it is on.
    pub(crate) r#loop: LoopKey,
}

impl StoredRows {
    /// **Whether the face's row set is COMPLETE**: it stores a row, and
    /// every half-edge of every loop carries a row. A face storing no
    /// row is unminted, not complete; an `Empty` loop holds no
    /// half-edge and so misses no row.
    pub(crate) fn complete(&self) -> bool {
        self.stores && self.gaps.is_empty()
    }

    /// **Whether a site mint re-mints the face** ([`site_rows`]): it
    /// stores a row, and every row it misses is on a
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
    /// walks every loop of its faces, an operator that rewires a loop
    /// out from under the edge, as the pipelines' joins do, and the band
    /// kill of the edge ([`crate::Body::plan_released_rows`]). An
    /// operator or a kill walks only the loops it rewires, so a gap on a
    /// loop it keeps stays. **Any other gap is not the site mint's to fill**: a door
    /// left the face half-minted ([`PcurveMintError::MissingCache`]
    /// lists them) and the site mint leaves it as found, for the
    /// producer's final pass.
    pub(crate) fn remints(&self, open: &[LoopKey], released: bool) -> bool {
        self.stores && (released || self.gaps.iter().all(|gap| open.contains(&gap.r#loop)))
    }
}

impl StoredRows {
    /// The loops of the face a null edge holds open ([`held_open`]), in
    /// walk order. Empty, with nothing read, on a face that misses no
    /// row: a null half never stores one, since every row derives from
    /// its half's carrier and a null edge has none.
    #[track_caller]
    pub(crate) fn open_loops<T: Decide>(&self, body: &Body<T>) -> Vec<LoopKey> {
        if self.gaps.is_empty() {
            return Vec::new();
        }
        self.loops
            .iter()
            .filter(|(_, cycle)| held_open(body, cycle.iter().copied()))
            .map(|&(lk, _)| lk)
            .collect()
    }
}

/// **Whether a null edge holds a loop open**: one of `halves`, the
/// loop's half-edges, is a half of a null edge
/// ([`crate::null::CurveGeom::NullScaffold`]), scaffolding with no carrier, so the
/// loop cannot be walked. The site mint's one reading of it
/// ([`StoredRows::open_loops`], [`site_rows`]).
///
/// # Panics
///
/// Where a half's edge or that edge's curve does not resolve: each is a
/// link of the record before it, and `halves` are half-edges the caller
/// read out of the body's records.
#[track_caller]
fn held_open<T: Decide>(body: &Body<T>, halves: impl IntoIterator<Item = HalfEdgeKey>) -> bool {
    let mut open = false;
    for he in halves {
        let (edge, data) = edge_record(body, he);
        open |= body.edge_curve_linked(edge, data).null_scaffold().is_some();
    }
    open
}

/// **The one per-loop rows walk**: the half-edges of `r#loop` a pcurve
/// row can be keyed on, in cycle order from its `first` — empty for a
/// loop whose boundary is a lone vertex ([`crate::LoopBoundary::Empty`]),
/// which holds no half-edge and so no row. These are the keys the map
/// is keyed on for this loop, and the only ones.
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
///
/// The walk is [`crate::Body::site_cycle_from`]: a member that does not
/// claim `r#loop` panics, so a torn `next` cannot hand another loop's
/// rows to this one's reader. A walk that closes short of a member it
/// should reach is not caught here; a door that moves or re-charts the
/// whole loop proves that too ([`crate::Body::whole_cycle`]).
///
/// # Panics
///
/// Where `r#loop` — a loop a door resolved, or one a face record names —
/// does not resolve, or its walk does not close on the half-edges that
/// claim it ([`crate::body::CYCLES_ARE_CLAIMANTS`]).
#[track_caller]
pub(crate) fn loop_rows<T: Decide>(body: &Body<T>, r#loop: LoopKey) -> Vec<HalfEdgeKey> {
    match proven(&body.loops, r#loop, EntityId::Loop).boundary {
        crate::entity::LoopBoundary::Cycle { first } => body.site_cycle_from(first, r#loop),
        crate::entity::LoopBoundary::Empty { .. } => Vec::new(),
    }
}

/// One half-edge of a loop's lift ([`loop_lift`]).
#[derive(Clone, Debug)]
pub struct LiftedRow<'a, T: Real> {
    /// The half-edge.
    pub half_edge: HalfEdgeKey,
    /// Its stored row: the edge's image in the face's chart, with its
    /// interval and certificate.
    pub row: &'a PcurveCache<T>,
    /// The deck transformation the lift moves the image by: the joint
    /// elements summed along the loop ([`lift_decks`]).
    pub lift: Deck,
    /// The image moved by `lift`: the chart curve the loop's chain
    /// draws on this half-edge.
    pub pcurve: Pcurve<T>,
}

/// Why a loop has no lift ([`loop_lift`]): the first half-edge, in
/// cycle order from the loop's `first`, that stores no image or no
/// element on the joint into it, or whose image its lift does not
/// apply to ([`Deck::apply`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LiftGap {
    /// The half-edge.
    pub half_edge: HalfEdgeKey,
}

/// **Each member's lift, from the elements of a cycle's joints** —
/// `elements[i]` the element into member `i`, in cycle order. The lift
/// starts on one member's own image, at the identity, and crosses every
/// joint after it once round the cycle ([`JointElement::follow`]); the
/// joint into the start is the one it does not cross, where the loop's
/// winding shows. The start is the cycle's first member where no joint
/// is a reset, and otherwise the member after the last reset before the
/// first, so the azimuth restart a reset makes is where the chain
/// begins. Every reset after it is crossed like any joint, its
/// second-channel periods and twin carried on and its first channel
/// restarted. The winding is not the lift's: [`Winding`] reads it off
/// the same elements.
pub(crate) fn lift_decks(elements: &[JointElement]) -> Vec<Deck> {
    let n = elements.len();
    let start = elements.iter().rposition(|e| e.is_reset()).unwrap_or(0);
    let mut decks = vec![Deck::IDENTITY; n];
    let mut lift = Deck::IDENTITY;
    for k in 1..n {
        let i = (start + k) % n;
        lift = elements[i].follow(lift);
        decks[i] = lift;
    }
    decks
}

/// **A loop's lift** — the readers' one accessor for where a loop's
/// rows sit in its face's chart ([`crate::Body::loop_lift`]): each
/// half-edge of `r#loop` in cycle order from its `first`
/// ([`loop_rows`]), its stored image moved by the deck transformation
/// the elements sum to along the loop ([`lift_decks`]). Consecutive
/// curves of the chain meet at every joint but a reset (a pole, an
/// apex), and the loop's winding is the elements' ([`Winding`]): a
/// wrapping loop's chain ends one period from where it starts.
///
/// Where the lift starts is gauge: the chain from another start is the
/// same chain moved by one deck transformation, and every reader reads
/// the chain, not its offset.
///
/// # Errors
///
/// [`LiftGap`] naming the first half-edge, in cycle order, that stores
/// no image or no element on the joint into it, or whose image its
/// lift does not apply to.
///
/// # Panics
///
/// Where `r#loop`, its face or a member does not resolve, or its walk
/// does not close on the half-edges that claim it ([`loop_rows`]).
#[track_caller]
pub fn loop_lift<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
) -> Result<Vec<LiftedRow<'_, T>>, LiftGap> {
    let cycle = loop_rows(body, r#loop);
    let face = proven(&body.loops, r#loop, EntityId::Loop).face;
    let surface = body.face_surface_linked(face, proven(&body.faces, face, EntityId::Face));
    // No band is at hand, so the periods are the chart's as stated: on
    // every element a door wrote they are the decided ones
    // ([`ChartPeriods::stated`]).
    let periods = ChartPeriods::stated(surface);
    let mut rows = Vec::with_capacity(cycle.len());
    let mut elements = Vec::with_capacity(cycle.len());
    for &half_edge in &cycle {
        let (Some(row), Some(element)) = (body.pcurve(half_edge), body.joint(half_edge)) else {
            return Err(LiftGap { half_edge });
        };
        rows.push(row);
        elements.push(element);
    }
    cycle
        .into_iter()
        .zip(rows)
        .zip(lift_decks(&elements))
        .map(|((half_edge, row), lift)| {
            let pcurve = lift
                .apply(row.pcurve(), surface, periods)
                .ok_or(LiftGap { half_edge })?;
            Ok(LiftedRow {
                half_edge,
                row,
                lift,
                pcurve,
            })
        })
        .collect()
}

/// [`loop_lift`] for a reader that walks `halves` — members of one
/// loop, in the reader's own order: each one's lifted image, or `None`
/// for every one where that loop has no lift (a member stores no image,
/// or no element on the joint into it). Empty for no halves.
///
/// # Panics
///
/// [`loop_lift`]'s, and where the first half does not resolve.
#[track_caller]
pub(crate) fn lifted_images<T: Decide>(
    body: &Body<T>,
    halves: &[HalfEdgeKey],
) -> Vec<Option<Pcurve<T>>> {
    let Some(&first) = halves.first() else {
        return Vec::new();
    };
    let r#loop = proven(&body.half_edges, first, EntityId::HalfEdge).parent_loop;
    let Ok(lift) = loop_lift(body, r#loop) else {
        return vec![None; halves.len()];
    };
    let mut by_half: slotmap::SecondaryMap<HalfEdgeKey, Pcurve<T>> = slotmap::SecondaryMap::new();
    for row in lift {
        by_half.insert(row.half_edge, row.pcurve);
    }
    halves.iter().map(|&he| by_half.get(he).cloned()).collect()
}

/// The loops of `face` — a face of the body, an arena member or a key
/// its door resolved — outer first, each with its cycle
/// ([`loop_rows`]; empty for a loop whose boundary is a lone vertex).
/// The one walk of a face's cycles: [`face_loop_cycles`],
/// [`Body::face_cycles_linked`] and [`Body::face_cycles`] all read it.
///
/// # Panics
///
/// Where `face`, a loop its record names, or a loop's walk does not
/// resolve.
#[track_caller]
pub(crate) fn face_loop_walks<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Vec<(LoopKey, Vec<HalfEdgeKey>)> {
    let data = proven(&body.faces, face, EntityId::Face);
    body.face_loops_linked(face, data)
        .map(|(lk, _)| (lk, loop_rows(body, lk)))
        .collect()
}

/// [`face_loop_walks`], a loop whose boundary is a lone vertex left out.
#[track_caller]
fn face_loop_cycles<T: Decide>(body: &Body<T>, face: FaceKey) -> Vec<(LoopKey, Vec<HalfEdgeKey>)> {
    face_loop_walks(body, face)
        .into_iter()
        .filter(|(_, cycle)| !cycle.is_empty())
        .collect()
}

/// **Whether `he` misses a row** ([`StoredRows::gaps`]): it stores no
/// image, or the joint into it from `before`, its predecessor, has no
/// element (`element` false) where `before` stores an image — a joint
/// between two stored images owes its element.
fn misses_a_row<T: Real>(
    body: &Body<T>,
    before: HalfEdgeKey,
    he: HalfEdgeKey,
    element: bool,
) -> bool {
    body.pcurve(he).is_none() || (body.pcurve(before).is_some() && !element)
}

/// [`StoredRows`] for `face`, a face of the body: its loops walked
/// once.
///
/// # Panics
///
/// [`face_loop_cycles`]'.
#[track_caller]
pub(crate) fn stored_rows<T: Decide>(body: &Body<T>, face: FaceKey) -> StoredRows {
    let loops = face_loop_cycles(body, face);
    let mut gaps = Vec::new();
    let mut stores = false;
    for (lk, cycle) in &loops {
        for (i, &he) in cycle.iter().enumerate() {
            let before = cycle[(i + cycle.len() - 1) % cycle.len()];
            stores |= body.pcurve(he).is_some();
            if misses_a_row(body, before, he, body.joint(he).is_some()) {
                gaps.push(RowGap {
                    half_edge: he,
                    r#loop: *lk,
                });
            }
        }
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
    /// The element of the joint between the two children, at the split
    /// point: the identity, the two children being one image — a reset
    /// where that point is on the chart's singular set.
    pub(crate) joint: JointElement,
    /// Where a child's row was placed again over its own span: the
    /// element of the joint into the child the parent's incoming joint
    /// fed (the first child on a plus half, the second on a minus one),
    /// decided against the half before it.
    pub(crate) entering: Option<JointElement>,
    /// Likewise, the element of the joint out of the child whose exit is
    /// the parent's, into the half after it.
    pub(crate) leaving: Option<JointElement>,
}

/// Why [`split_cache`] could not state a child's row, for one parent
/// half-edge. Raised, never swallowed: the split leaves nothing it
/// could not state.
#[derive(Clone, Debug)]
pub(crate) struct SplitRowError {
    /// The parent half-edge whose children's rows were being stated.
    pub(crate) half_edge: HalfEdgeKey,
    /// What refused.
    pub(crate) refusal: SplitRefusal,
}

/// The two refusals of [`split_cache`].
#[derive(Clone, Debug)]
pub(crate) enum SplitRefusal {
    /// A child's row refused over its sub-interval: the derivation that
    /// places a closed-form or projected child there, or the
    /// certification of the child's row (derived, or a fitted or
    /// general parent's restricted), which the whole image passed. A
    /// covered lane that refuses here is a genuine defect.
    Certify(PcurveCertifyError),
    /// The one decider decided no element for a joint the split makes
    /// ([`decide_joint`]): between the two children, or one of the
    /// parent's end joints decided again against its neighbour (or, on a
    /// one-edge loop, its own other child). At the children's joint the
    /// gap is zero, and a
    /// zero gap meets the image's own orbit point at any lever
    /// ([`lever_periods`]), even a point off a narrow cone's apex by
    /// more than the band whose lever is under it: what refuses is a
    /// reading the band cannot decide (`Some`, with its cause), or a
    /// decided miss (`None`) of a channel that read does not serve.
    Joint(Option<Indeterminate>),
}

/// The **rows of an edge's two half-edges stated over the two
/// children of a parameter split** — [`crate::Body::split_edge`](crate::construction::Construct::split_edge)'s
/// pcurve limb, and the reason that op carries its rows across the
/// surgery instead of staling them. The answer holds one entry per
/// given half-edge: that half-edge's [`CarriedRows`], or `None` where
/// there is nothing to carry.
///
/// A row is its image placed over its own edge's span in the chart's
/// principal window (C4), so a child of a split at `t` is stated over
/// `[t₀, t]` or `[t, t₁]` as the mint would state it: a closed-form or
/// projected image is derived there ([`geom_brep::chart_pcurve_over`]),
/// and a fitted or general image, which has no angular placement, is
/// the parent's restricted to the sub-interval. Each is certified
/// through the image's own door ([`restate`], with the scalar's
/// [`crate::AtRestPolicy::fitted_lane`] for a projected row's hull
/// terms and a fitted-grade row's check 4). Children placed again can
/// sit whole turns from each other and from the parent's image, so the
/// joint between them and the parent's joints into and out of them are
/// decided here, as every joint is ([`CarriedRows`]).
///
/// **A fitted-grade row's check 4 reads the WHOLE carrier**, not the
/// child's span: its hull bounds the image's whole domain against the
/// chart. Its inputs (carrier, image, surface, band) are the parent's,
/// so at the band the parent's
/// row was certified at, each child's certificate IS the parent's. The
/// statement is conservative for a child, and such a split never
/// refuses a child at check 4 unless the parent's own row already
/// fails it. At a TIGHTER band than the parent's, check 4 can refuse a
/// child: a typed refusal ([`SplitRefusal::Certify`]), never a false
/// certificate.
///
/// **The joint between the children** is decided at `t`, where both
/// children's images hold one chart point, so its gap is zero and no
/// period is crossed. Off the chart's singular set it decides the
/// identity shift; it writes a reset at a split point on that set, or
/// on a `u`-closed spline chart whose `u` knot domain moves a point
/// within the band ([`net_reach`], [`decide_joint`]).
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
/// # `None`
///
/// `half_edge` carries no row (an all-planar body, a face of an
/// uncovered class, or one a door has left rowless for its producer's
/// closing mint). The caller writes nothing, so the map is left exactly
/// as found. A key that does not resolve is NOT one of them: it
/// panics (below).
///
/// # What still reads a split body short
///
/// The carry is exact on every kind it covers, spline charts included.
/// Two readers downstream of it are narrower:
///
/// - **The iso rim arms** are not among them. On a described-NURBS
///   wall the carried `IsoLine` and `IsoArc` rows restrict like any
///   other, and [`mint_pcurves`] re-derives them: [`nurbs_iso_derive`]'s
///   line rim takes the row its end feet measure, and its arc rim the
///   row over the arc the row measures (`sweep`'s
///   `split_edge_loft_charts`). A carried fitted-grade row is
///   re-derived bit for bit (`sweep`'s `split_edge_fitted_rows`).
/// - **The mesher's chord schedule.** It lands chord points on an arc
///   rim's sub-arc joints only where the row's arc is the edge's
///   interval, so it refuses a split rim's child typed
///   (`work/tess/a-split-arc-rim-meshes-no-chord-point-on-its-sub-arc-joints.md`).
/// - **The trimmed quadrature lane.** It subdivides a `General` image
///   over its whole knot domain, so it refuses a carried child's
///   sub-range typed rather than integrate chart the face does not
///   bound. A split of a seam that holds a `General` row is a standing
///   producer of that shape, filed as
///   `work/quad/a-fitted-cap-cut-by-planes-has-a-sub-range-trim-image.md`.
///   No regression: before this carry the split left the face
///   rowless, which the lane refuses too.
///
/// # Errors
///
/// [`SplitRowError`] — a certification the whole image passed failing
/// over a sub-interval ([`SplitRefusal::Certify`]), or the joint
/// between the two children undecided at the split point
/// ([`SplitRefusal::Joint`]): the split writes no element the one
/// decider did not decide.
///
/// # Panics
///
/// Where a record the restriction reads does not resolve — each half's
/// edge, curve, loop, face and surface — or a half's edge is null
/// scaffolding: `halves` are the live halves of an edge whose curve
/// [`crate::Body::split_edge`](crate::construction::Construct::split_edge) resolved `Certified`.
#[track_caller]
pub(crate) fn split_cache<T: AtRestPolicy>(
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
        let (carrier, t0, t1) = half_edge_carrier(body, half_edge).unwrap_or_else(|e| {
            unreachable!("split_edge resolved the curve of {half_edge:?}'s edge Certified: {e}")
        });
        let surface = half_edge_surface(body, half_edge);
        let image = cache.pcurve().clone();
        let refused = |error| SplitRowError {
            half_edge,
            refusal: SplitRefusal::Certify(error),
        };
        let certify = |a: T, b: T, image: Pcurve<T>| {
            restate(&image, (a, b), &carrier, &surface, band, T::fitted_lane()).map_err(refused)
        };
        let Some(chart) = DescribedChart::of(&surface) else {
            unreachable!(
                "{half_edge:?} stores a row, and rows are minted only on a described chart \
                 (`DescribedChart::minting`)"
            )
        };
        let periods = ChartPeriods::of(&surface, band);
        let pin = |image: &Pcurve<T>, at: T, onto: geom_core::Point2<T>| {
            decide_joint(chart, image, at, onto, carrier.eval(at), periods, band).map_err(|miss| {
                SplitRowError {
                    half_edge,
                    refusal: SplitRefusal::Joint(match miss {
                        PinMiss::Escalated(cause) | PinMiss::NetStretchUndecided(cause) => {
                            Some(cause)
                        }
                        PinMiss::Discontinuity => None,
                    }),
                }
            })
        };
        // C4 places a row over its own edge's span, in the chart's
        // principal window, so a child's row is the placement over its
        // sub-interval: a closed-form or projected image is derived
        // there, exactly as the mint derives it. A fitted, general or iso
        // image has no angular placement, and its children restrict the
        // parent's (`Pcurve::is_placed`).
        let placed_again = image.is_placed();
        // A derivation that refuses over a child's span refuses the split
        // (fail-loud): the parent's image is not the child's row there.
        let child = |a: T, b: T| {
            if placed_again {
                geom_brep::chart_pcurve_over(&carrier, a, b, &surface, band).map_err(refused)
            } else {
                Ok(image.clone())
            }
        };
        let (first, second) = (child(t0, t)?, child(t, t1)?);
        // The children meet at `t`, so the joint is decided there as every
        // joint is, which reads whether the split point is on the chart's
        // singular set. It runs the way the half does: from the first
        // child into the second on a plus half, the other way on a minus
        // one, the two children being placed apart where their spans
        // straddle the window's cut.
        let joint = if is_plus(body, half_edge) {
            pin(&second, t, first.eval(t))?
        } else {
            pin(&first, t, second.eval(t))?
        };
        // A child placed again over its own span may sit whole turns from
        // the parent's image, so the joints the parent's ends made are
        // decided again against their neighbours' rows, as every joint
        // is. A neighbour with no row, or one of this edge's own halves
        // (a strut's tip), keeps its element.
        let neighbour = |he: HalfEdgeKey| -> Option<(Pcurve<T>, T, T)> {
            if halves.contains(&he) {
                return None;
            }
            let row = body.pcurve(he)?;
            let (a, b) = row.params();
            let (entry, exit) = if is_plus(body, he) { (a, b) } else { (b, a) };
            Some((row.pcurve().clone(), entry, exit))
        };
        let record = half_edge_record(body, half_edge);
        let (entering, leaving) = if placed_again {
            let plus = is_plus(body, half_edge);
            let (fed, entry) = if plus { (&first, t0) } else { (&second, t1) };
            let (out, exit) = if plus { (&second, t1) } else { (&first, t0) };
            // A one-edge loop is its own neighbour: the split makes its
            // closing joint, out of the child whose exit is the parent's
            // into the one the parent's entry fed, and it is decided
            // between the two children. On a plus half that joint lands
            // on the parent half (`next(hp)` is `hp`), so it is both the
            // element into the fed child and the one out of the other; on
            // a minus half it lands on the new half alone, and `hm`'s old
            // `next` is `hm`, whose element is the children's own joint.
            if record.next == half_edge {
                let closing = pin(fed, entry, out.eval(exit))?;
                rows[slot] = Some(CarriedRows {
                    parent_half: certify(t0, t, first)?,
                    new_half: certify(t, t1, second)?,
                    joint,
                    entering: Some(closing),
                    leaving: plus.then_some(closing),
                });
                continue;
            }
            let entering = match neighbour(record.prev) {
                Some((before, _, before_exit)) => Some(pin(fed, entry, before.eval(before_exit))?),
                None => None,
            };
            let leaving = match neighbour(record.next) {
                Some((after, after_entry, _)) => Some(
                    decide_joint(
                        chart,
                        &after,
                        after_entry,
                        out.eval(exit),
                        carrier.eval(exit),
                        periods,
                        band,
                    )
                    .map_err(|miss| SplitRowError {
                        half_edge,
                        refusal: SplitRefusal::Joint(match miss {
                            PinMiss::Escalated(cause) | PinMiss::NetStretchUndecided(cause) => {
                                Some(cause)
                            }
                            PinMiss::Discontinuity => None,
                        }),
                    })?,
                ),
                None => None,
            };
            (entering, leaving)
        } else {
            (None, None)
        };
        rows[slot] = Some(CarriedRows {
            parent_half: certify(t0, t, first)?,
            new_half: certify(t, t1, second)?,
            joint,
            entering,
            leaving,
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
/// unclosed loop walk, an escalated classification, or
/// [`PcurveMintError::NoCarrier`] on a face a null edge bounds. Never a
/// silent skip of a face the lane covers. Every row is derived before
/// any is written, so a refusal leaves the body as found.
pub fn mint_pcurves<T: AtRestPolicy>(body: &mut Body<T>, tol: Tol) -> Result<(), PcurveMintError> {
    let band = Band::linear(tol).map_err(PcurveMintError::Band)?;
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    let rows = mint_rows(body, &faces, band)?;
    // Start from empty. A body reaching this pass may have been carved
    // from a scratch clone that inherited rows for half-edges the
    // surgery killed (a `SecondaryMap` row outlives its key until the
    // slot is reused), and a stale cache is worse than no cache. What
    // this pass leaves behind is exactly what it minted and certified.
    // The whole-body entry is the only one that can make that claim: a
    // row whose half-edge is dead is reachable from no face, so the
    // subset entry below clears through the faces it is given.
    body.pcurves.clear();
    body.joints.clear();
    for (he, row, element) in rows {
        body.write_row(he, row, element);
    }
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
/// [`PcurveMintError::Stale`] for a face in `faces` that does not
/// resolve, before any work; otherwise [`mint_pcurves`]'s, raised by a
/// face in `faces`, with the body left as found.
pub fn mint_pcurves_of<T: AtRestPolicy>(
    body: &mut Body<T>,
    faces: &[FaceKey],
    tol: Tol,
) -> Result<usize, PcurveMintError> {
    let band = Band::linear(tol).map_err(PcurveMintError::Band)?;
    if let Some(&face) = faces.iter().find(|&&face| !body.faces.contains_key(face)) {
        return Err(PcurveMintError::Stale {
            role: "faces",
            key: EntityId::Face(face),
        });
    }
    let rows = mint_rows(body, faces, band)?;
    let cleared: Vec<HalfEdgeKey> = faces
        .iter()
        .flat_map(|&face| face_loop_cycles(body, face))
        .flat_map(|(_, cycle)| cycle)
        .collect();
    body.drop_rows(cleared);
    let before = body.pcurves.len();
    for (he, row, element) in rows {
        body.write_row(he, row, element);
    }
    Ok(body.pcurves.len() - before)
}

/// **The rows the mint writes over `faces`**, derived and certified
/// before it writes any — the shared body of [`mint_pcurves`] and
/// [`mint_pcurves_of`], which differ only in which rows they clear —
/// in `faces` order, each face's in walk order. Read-only, so a
/// refusal, or a panic on a torn record, leaves the body as found.
///
/// A face whose rows are not owed ([`not_owed`]: a pair the chart can
/// hold but no route covers yet — a line, ellipse or spiric offered a
/// fitted image — or a fitted face at a scalar with no fitted door; the
/// at-rest pass excuses exactly these, by the same predicate)
/// contributes the rows it held, carried ([`carry_rows`]), so the mint
/// never drops a certificate it cannot re-derive. Every OTHER refusal
/// — a carrier off its face or grazing it, an image that is not its
/// carrier's, a general sphere circle its fitted
/// route refuses, a covered class whose residuals, envelope, continuity
/// or closure refuse — is a genuine defect and propagates.
fn mint_rows<T: AtRestPolicy>(
    body: &Body<T>,
    faces: &[FaceKey],
    band: Band,
) -> Result<Certified<T, HalfEdgeKey>, PcurveMintError> {
    let mut rows = Vec::new();
    for &face in faces {
        match derive_face(body, face, band) {
            Ok(derived) => rows.extend(derived),
            Err(e) if not_owed::<T>(&e) => rows.extend(carry_rows(body, face, band)?),
            Err(e) => return Err(e),
        }
    }
    Ok(rows)
}

/// **The rows a face holds, carried across a mint that cannot derive
/// them**: each one the face's loops reach is re-certified through its
/// own door against the current carrier and surface. So a row the mint
/// has no route to — a
/// `Fitted` row on a spline chart, stated by a certifying door —
/// survives a producer's closing mint (a
/// transform, a merge) exactly when it still certifies, and a half the
/// face held no row for stays without one, for tier 3 to read.
///
/// # Errors
///
/// [`PcurveMintError::Certify`] for a carried row that no longer
/// certifies, and [`PcurveMintError::RowInterval`] for one whose
/// interval is no longer its edge's ([`row_interval`]): the producer
/// refuses rather than drop it or store it stale.
/// [`PcurveMintError::NoCarrier`] for a row held on a half of a null
/// edge.
fn carry_rows<T: AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<Certified<T, HalfEdgeKey>, PcurveMintError> {
    let face_data = proven(&body.faces, face, EntityId::Face);
    let surface = body.face_surface_linked(face, face_data);
    let held: Vec<(HalfEdgeKey, &PcurveCache<T>)> = face_loop_cycles(body, face)
        .into_iter()
        .flat_map(|(_, cycle)| cycle)
        .filter_map(|he| body.pcurve(he).map(|row| (he, row)))
        .collect();
    let mut carried = Vec::with_capacity(held.len());
    for (he, row) in held {
        let (carrier, _, _) = half_edge_carrier(body, he)?;
        // A producer that moved the edge's interval under the row
        // refuses here, not later at tier 3.
        row_interval(body, he, row, &carrier, band)?;
        let (t0, t1) = row.params();
        let restated = restate(
            row.pcurve(),
            (t0, t1),
            &carrier,
            surface,
            band,
            T::fitted_lane(),
        )
        .map_err(|error| PcurveMintError::Certify {
            half_edge: he,
            error,
        })?;
        carried.push((he, restated, body.joint(he)));
    }
    Ok(carried)
}

/// A stored chart image re-certified over `[t0, t1]` through its own
/// door: the closed-form door for every image it covers, the fitted
/// door (`fitted`, the scalar's [`crate::AtRestPolicy::fitted_lane`])
/// for a `Fitted`, `General` or `Projected` one.
fn restate<T: Decide>(
    image: &Pcurve<T>,
    (t0, t1): (T, T),
    carrier: &geom::Curve3<T>,
    surface: &Surface<T>,
    band: Band,
    fitted: Option<geom_brep::FittedLane<T>>,
) -> Result<PcurveCache<T>, PcurveCertifyError> {
    match image {
        Pcurve::Fitted(image) => match fitted {
            Some(lane) => PcurveCache::certify_fitted(
                std::sync::Arc::clone(image),
                t0,
                t1,
                carrier,
                surface,
                band,
                lane,
            ),
            None => Err(PcurveCertifyError::FittedLaneUnsupported { scalar: T::NAME }),
        },
        Pcurve::General(image) => PcurveCache::certify_general(
            std::sync::Arc::clone(image),
            t0,
            t1,
            carrier,
            surface,
            band,
            fitted,
        ),
        Pcurve::Projected(image) => PcurveCache::certify_projected(
            (**image).clone(),
            t0,
            t1,
            carrier,
            surface,
            band,
            fitted,
        ),
        image => PcurveCache::certify(image.clone(), t0, t1, carrier, surface, band),
    }
}

/// **Whether a face's refusal means its rows are not owed**: the one
/// reading, shared by the minting pass ([`mint_rows`]), which leaves
/// such a face storing nothing, and the at-rest pass
/// ([`validate_pcurves`]), which reports nothing about it. One arm, and
/// only this: **a scalar with no fitted door**
/// ([`AtRestPolicy::fitted_lane`] answers `None`, as a dual's does)
/// refusing a face only the fitted lane can image
/// ([`PcurveCertifyError::FittedLaneUnsupported`]): that scalar
/// certifies nothing fitted, so it owes no fitted row. The rule is the
/// scalar's, not a class's — at every scalar that holds the door, a
/// fitted refusal is a finding. Every (chart, carrier) class a chart
/// can hold has a route, so no class is excused.
fn not_owed<T: AtRestPolicy>(e: &PcurveMintError) -> bool {
    match e {
        PcurveMintError::Certify {
            error: PcurveCertifyError::FittedLaneUnsupported { .. },
            ..
        } => T::fitted_lane().is_none(),
        _ => false,
    }
}

/// **The minting pass over one face, storing nothing** (module docs:
/// the two-pass shape — walk the loops to pin branches, then certify
/// every pcurve against its carrier and chart, [`certify_walked`]). [`mint_rows`] writes what this derives;
/// [`validate_pcurves`] reads a face that does not store its rows
/// through it, so "what the mint would write here" has one answer.
///
/// A face on a chart that mints nothing ([`DescribedChart::minting`])
/// derives no row. `face` is a face of the body: an arena member, or a
/// key its door resolved.
///
/// # Errors
///
/// The face's refusal ([`first_owed`]), read whole: the walk's
/// ([`walk_loop`]) or a certification's.
#[track_caller]
fn derive_face<T: AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<Certified<T, HalfEdgeKey>, PcurveMintError> {
    let face_data = proven(&body.faces, face, EntityId::Face);
    let surface = body.face_surface_linked(face, face_data);
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
    // The fitted door goes in as the scalar's policy answers it, `None`
    // and all: the own-chart arm of `nurbs_iso_derive` hands a
    // construction's STATED `General` image here at every scalar, a dual
    // included, and an absent door refuses at check 4, so an image that
    // fails checks 1–3 draws those checks' verdict at every scalar.
    let fitted = |w: &Walked<T>| match &w.pcurve {
        Pcurve::Fitted(_) | Pcurve::General(_) | Pcurve::Projected(_) => restate(
            &w.pcurve,
            (w.t0, w.t1),
            &w.carrier,
            surface,
            band,
            T::fitted_lane(),
        ),
        closed => unreachable!(
            "certify_walked hands the fitted door only Fitted, General and Projected images: \
             {closed:?}"
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
                let cycle = loop_rows(body, lp);
                walk_runs(body, chart, &cycle, band, &mut walked, &mut refused);
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
/// the walk meets first. Asked only of a face that refused, so
/// `refused` holds at least one.
fn first_owed<T: AtRestPolicy>(refused: Vec<PcurveMintError>) -> PcurveMintError {
    let at = refused.iter().position(|e| !not_owed::<T>(e)).unwrap_or(0);
    refused.into_iter().nth(at).unwrap_or_else(|| {
        unreachable!("first_owed is asked only of a face that refused at least once")
    })
}

/// The rows [`certify_walked`] certified, in walk order: each image's
/// certified row, with the element of the joint into it where the walk
/// decided one.
type Certified<T, K> = Vec<(K, PcurveCache<T>, Option<JointElement>)>;

/// A fitted-grade certifier for a `Fitted` or `General` image
/// ([`certify_walked`]).
type FittedDoor<'a, T, K> = &'a dyn Fn(&Walked<T, K>) -> Result<PcurveCache<T>, PcurveCertifyError>;

/// **Pass 2 of the minting walk, for one face**: every image the walk
/// derived certified against its carrier and chart, in walk order. The
/// one home of that pass, shared by `mint_face` (the minting pass)
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
            (Pcurve::Fitted(_) | Pcurve::General(_) | Pcurve::Projected(_), Some(fitted)) => {
                fitted(&w)
            }
            _ => PcurveCache::certify(w.pcurve.clone(), w.t0, w.t1, &w.carrier, surface, band),
        };
        match cache {
            Ok(cache) => rows.push((w.key, cache, w.element)),
            Err(error) => refused.push((w.key, error)),
        }
    }
    if refused.is_empty() {
        Ok(rows)
    } else {
        Err(refused)
    }
}

/// A site half's carrier, its interval, whether the loop traverses it
/// forward, and the point of the vertex it enters at ([`site_rows`]).
type SiteTraversal<T> = (geom::Curve3<T>, T, T, bool, geom_core::Point3<T>);

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
    /// A half-edge the body already holds, of an edge being described:
    /// the walk reads it under the carrier the description installs
    /// ([`SiteCarriers::Described`]).
    Described(HalfEdgeKey),
}

/// The carriers a site mint's plan reads for the halves it names that
/// the body does not carry yet ([`SiteHalf`]): no door both mints an
/// edge and describes one.
pub(crate) enum SiteCarriers<'a, T: Real> {
    /// A door that names existing halves alone.
    Existing,
    /// An Euler operator's new edge, for [`SiteHalf::NewPlus`] and
    /// [`SiteHalf::NewMinus`].
    New(&'a geom_brep::EdgeCurve<T>),
    /// The curves a description installs, by edge, for every
    /// [`SiteHalf::Described`] half of those edges.
    Described(&'a [(EdgeKey, &'a geom_brep::EdgeCurve<T>)]),
}

impl<T: Real> Clone for SiteCarriers<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Real> Copy for SiteCarriers<'_, T> {}

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
    /// **A keys-only door owes the face a row it holds no band to
    /// derive.** The door moves a loop or run onto a face whose rows
    /// are complete on an analytic chart, and the moved rows do not
    /// stand there — stated in another chart, or missing — so the face
    /// would leave half-minted. Or a null edge's kill releases a loop,
    /// which would leave missing the rows it missed while the edge held
    /// it open ([`releases_a_gap`]). The keys-only kills and moves
    /// (`kef`, `kemr`, `kev`, `kfmrh`, `ring_move`, `mfkrh`) take no
    /// band and refuse here, before mutating; their band siblings
    /// (`_minting`, or `kev_describing`) take one and re-mint the face
    /// ([`site_rows_owed`]). `kev` refuses here too where its unsplice
    /// would bridge a joint through a killed half's turn, which only a
    /// band decides.
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
            Self::KeysOnly => write!(
                f,
                "the door would leave a face half-minted: the rows it moves do not stand \
                 on a complete face, the joint it bridges needs an element only a band \
                 decides, or the loop it releases from its last null edge misses rows; this \
                 door takes no band. Recourse: call the door's band-taking sibling \
                 (`_minting`, or `kev_describing`) with the run's tolerance, or edit the \
                 loop before the face is minted"
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
/// description, one a door moves a loop or run onto, or one a null
/// edge's kill releases a loop of — described as its door leaves it.
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
    /// What the door creates on the loops it rewires that run through
    /// no null edge: the image of every half it adds, describes, moves
    /// or finds rowless, and the element of every joint it makes or
    /// finds missing. Everything else those loops hold, the loops it
    /// keeps, and a rewired loop a null edge still holds open keep what
    /// they have.
    Mint {
        /// The certified images, by half.
        images: Vec<(SiteHalf, PcurveCache<T>)>,
        /// The joint elements, by the half each joint enters.
        joints: Vec<(SiteHalf, JointElement)>,
    },
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
/// # Panics
///
/// Where a loop of `face`, a face of the body, does not resolve or walk
/// as its own ([`face_loop_cycles`]), or a half's edge or curve does
/// not resolve ([`held_open`]).
#[track_caller]
pub(crate) fn site_rows_from<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    surface: &Surface<T>,
) -> Option<SiteFrom> {
    DescribedChart::minting(surface)?;
    let rows = stored_rows(body, face);
    let open = rows.open_loops(body);
    rows.remints(&open, !open.is_empty())
        .then_some(SiteFrom { rows, open })
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
/// adds half-edges to.
#[track_caller]
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
            ),
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

/// **Whether a null-edge kill leaves a loop it releases with a gap**:
/// a loop of `face`, as the kill leaves it, that runs through no null
/// edge and has a half-edge that [`misses_a_row`] — the gaps
/// [`StoredRows::gaps`] reads, taken on the loop the kill leaves.
/// `made` are the joints the kill writes, each keyed by the half-edge
/// it is into, with the element it writes there; every other joint
/// keeps its element. Band-free; the band kills' site mint runs only
/// where this holds ([`crate::Body::plan_released_rows`]).
///
/// A loop a null edge still holds open is not the kill's to release,
/// and its gaps do not count: the site mint cannot walk it.
///
/// # Panics
///
/// Where a half of `face`'s rewired loops, each one the kill read out
/// of the body's records, does not resolve.
#[track_caller]
pub(crate) fn releases_a_gap<T: Decide>(
    body: &Body<T>,
    face: &SiteFace<T>,
    made: &[(HalfEdgeKey, Option<JointElement>)],
) -> bool {
    let element = |he: HalfEdgeKey| {
        made.iter()
            .find(|&&(into, _)| into == he)
            .map_or(body.joint(he).is_some(), |(_, element)| element.is_some())
    };
    face.loops.iter().any(|lp| {
        let SiteLoop::Rewired(halves) = lp else {
            return false;
        };
        let cycle: Vec<HalfEdgeKey> = halves
            .iter()
            .filter_map(|&at| match at {
                SiteHalf::Existing(he) => Some(he),
                SiteHalf::NewPlus | SiteHalf::NewMinus | SiteHalf::Described(_) => None,
            })
            .collect();
        if held_open(body, cycle.iter().copied()) {
            return false;
        }
        let n = cycle.len();
        (0..n).any(|i| {
            let (before, he) = (cycle[(i + n - 1) % n], cycle[i]);
            misses_a_row(body, before, he, element(he))
        })
    })
}

/// **The rows a site mint writes onto one face**, derived before its
/// door mutates: a face an Euler operator adds half-edges to, one a
/// null edge's halves are on at its first description
/// ([`crate::Body::set_edge_curve`](crate::construction::Construct::set_edge_curve)), one a door moves a loop or run
/// onto whose rows do not stand there ([`crate::Body::drop_rows`] names
/// the doors), or one a band kill releases a loop of with a gap
/// ([`crate::Body::plan_released_rows`]). `from` is the face as found, on a face
/// [`site_rows_from`] read further; every other face is left as found, and never reaches
/// here. The face is re-minted where [`StoredRows::remints`] selects
/// it, `released` being whether the door takes the last null edge off
/// it: the face as found runs through one and the face as the door
/// leaves it through none.
///
/// - **On the loops the surgery rewires, it mints what the surgery
///   creates**, exactly as [`mint_pcurves_of`] would after the surgery:
///   the same walk from each loop's `first` ([`walk_cycle`]) and the
///   same certification ([`certify_walked`]), over the image of every
///   half the surgery adds, describes or finds rowless, derived by the
///   closed form ([`chart_pcurve_over`]), and the element of every joint it
///   makes or finds missing, decided between the two images
///   ([`decide_joint`]); the loop must still close ([`Winding`]).
///   Every other image and element stands, and the walk reads it as
///   stored: an image is a function of its edge and the chart, an
///   element of its two images, and the surgery changes none of them.
///   A loop the surgery keeps keeps its rows. A row's certificate reads
///   that row alone, so on a face whose stored rows are the pass's the
///   rows after the op are the pass's, byte for byte.
/// - **A rewired loop that still runs through a null edge keeps the
///   rows it has** ([`held_open`]): the null halves have no carrier,
///   so the loop cannot be walked, and its new halves go rowless until
///   the null edge leaves it. Every rewired loop the door leaves
///   running through no null edge is walked, and the rows it missed
///   while it was held open are minted with the rest of what is
///   missing.
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
///   needs nothing the fitted lane holds — [`chart_pcurve_over`],
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
/// **Cost.** One derivation and certification per image the surgery
/// creates and one joint decision per joint it makes, whatever the
/// loop's length; a moved loop's images are all derived. Beside them,
/// reads: per half-edge of the rewired loops a presence read and its
/// element (for the winding), and per half-edge of the rest of the
/// face one presence read ([`site_rows_from`]); on a face missing a row, three lookups
/// more per half-edge of the face as found — its half, edge and curve —
/// for which loops a null edge holds open, and as many again per
/// half-edge of the rewired loops.
///
/// # Errors
///
/// [`SiteRowRefusal::SplineChart`]: the spline frontier.
///
/// # Panics
///
/// Where a record a walked half reads does not resolve: every half the
/// plan names is one the door resolved or read out of a record.
#[track_caller]
pub(crate) fn site_rows<T: Decide>(
    body: &Body<T>,
    face: &SiteFace<T>,
    from: &SiteFrom,
    curves: SiteCarriers<'_, T>,
    band: Band,
) -> Result<SiteRows<T>, SiteRowRefusal> {
    let Some(chart) = DescribedChart::minting(&face.surface) else {
        return Ok(SiteRows::Leave);
    };
    let walks = site_walks(body, chart, face, from)?;
    if walks.is_empty() {
        return Ok(SiteRows::Leave);
    }
    let kept_halves = |key: LoopKey| {
        from.rows
            .loops
            .iter()
            .find(|(lk, _)| *lk == key)
            .map(|(_, cycle)| cycle.as_slice())
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
                    SiteLoop::Kept(key) => kept_halves(*key).collect(),
                }
            })
            .collect()
    };
    // A half the body holds enters at its start vertex. A new half's
    // vertex may not exist before the surgery, so it enters at its
    // carrier's end, which the edge certificate pins to the vertex the
    // surgery makes (`carrier_endpoint_start` / `_end`).
    let traversal = |at: SiteHalf| -> SiteTraversal<T> {
        match at {
            SiteHalf::Existing(he) => {
                let (carrier, t0, t1) = half_edge_carrier(body, he).unwrap_or_else(|e| {
                    unreachable!(
                        "site_walks walks only a loop no null edge holds open (held_open): {e}"
                    )
                });
                (carrier, t0, t1, is_plus(body, he), entry_vertex(body, he))
            }
            SiteHalf::NewPlus | SiteHalf::NewMinus => {
                let SiteCarriers::New(edge) = curves else {
                    unreachable!(
                        "site_rows: a plan names a new half only beside the edge that carries it"
                    )
                };
                let (t0, t1) = edge.params();
                let plus = at == SiteHalf::NewPlus;
                let carrier = edge.carrier().clone();
                let vertex = carrier.eval(if plus { t0 } else { t1 });
                (carrier, t0, t1, plus, vertex)
            }
            SiteHalf::Described(he) => {
                let of = half_edge_record(body, he).edge;
                let SiteCarriers::Described(described) = curves else {
                    unreachable!(
                        "site_rows: a Described half under non-Described carriers; \
                         `Body::description_rows` is the one constructor of \
                         `SiteHalf::Described`, and it plans under \
                         `SiteCarriers::Described` alone"
                    )
                };
                let Some(&(_, edge)) = described.iter().find(|(e, _)| *e == of) else {
                    unreachable!(
                        "site_rows: a Described half whose edge the carriers do not list; \
                         `Body::description_rows` marks a half Described only when it is \
                         a half of one of `described`'s edges, and passes that same slice as \
                         the carriers"
                    )
                };
                let (t0, t1) = edge.params();
                let plus = is_plus(body, he);
                (edge.carrier().clone(), t0, t1, plus, entry_vertex(body, he))
            }
        }
    };
    // A half the door keeps, on a face whose images stand, keeps its
    // image: an image is a function of its edge and the chart, and the
    // door changes neither. A moved loop's images are stated in the chart
    // it left, or missing, and are all derived.
    let standing = |at: SiteHalf| match at {
        SiteHalf::Existing(he) if !face.moved => body.pcurve(he),
        SiteHalf::Existing(_) | SiteHalf::NewPlus | SiteHalf::NewMinus | SiteHalf::Described(_) => {
            None
        }
    };
    let mut derived: Vec<Walked<T, SiteHalf>> = Vec::new();
    let mut joints: Vec<(SiteHalf, JointElement)> = Vec::new();
    for halves in walks {
        let n = halves.len();
        let stands: Vec<Option<&PcurveCache<T>>> = halves.iter().map(|&at| standing(at)).collect();
        // The element of each joint the door leaves as found — between
        // two images that stand, linked as they were — stands too; every
        // other joint is decided.
        let keep: Vec<Option<JointElement>> = (0..n)
            .map(|i| {
                let p = (i + n - 1) % n;
                match (halves[p], halves[i]) {
                    (SiteHalf::Existing(before), SiteHalf::Existing(he))
                        if stands[p].is_some()
                            && stands[i].is_some()
                            && half_edge_record(body, he).prev == before =>
                    {
                        body.joint(he)
                    }
                    _ => None,
                }
            })
            .collect();
        let mut carriers: Vec<geom::Curve3<T>> = Vec::new();
        let item = |i: usize| -> Result<WalkItem<'_, T>, PcurveCertifyError> {
            match (stands[i], halves[i]) {
                (Some(cache), SiteHalf::Existing(he)) => {
                    debug_assert!(
                        !matches!(
                            geom_core::k_stats::detached(|| {
                                let (carrier, ..) = traversal(SiteHalf::Existing(he));
                                row_interval(body, he, cache, &carrier, band)
                            })
                            .0,
                            Err(PcurveMintError::RowInterval { .. })
                        ),
                        "site_rows: the image kept for {he:?} spans an interval its edge does not"
                    );
                    let (t0, t1) = cache.params();
                    Ok(WalkItem {
                        base: Cow::Borrowed(cache.pcurve()),
                        t0,
                        t1,
                        plus: is_plus(body, he),
                        vertex: entry_vertex(body, he),
                    })
                }
                (_, at) => {
                    let (carrier, t0, t1, plus, vertex) = traversal(at);
                    let base =
                        Cow::Owned(chart_pcurve_over(&carrier, t0, t1, &face.surface, band)?);
                    carriers.push(carrier);
                    Ok(WalkItem {
                        base,
                        t0,
                        t1,
                        plus,
                        vertex,
                    })
                }
            }
        };
        // The closed-form derivation refusing, a branch meeting no
        // neighbour, or a loop that does not close: the face is cleared.
        let Ok(pinned) = walk_cycle(chart, n, item, |i| keep[i], band, true) else {
            return Ok(SiteRows::Clear(every_half()));
        };
        let mut carriers = carriers.into_iter();
        for (i, (image, t0, t1, element)) in pinned.into_iter().enumerate() {
            if keep[i].is_none() {
                let element = element.unwrap_or_else(|| {
                    unreachable!("walk_cycle decides every joint of a loop it closes")
                });
                joints.push((halves[i], element));
            }
            match image {
                Cow::Borrowed(_) => {}
                Cow::Owned(pcurve) => derived.push(Walked {
                    key: halves[i],
                    carrier: carriers.next().unwrap_or_else(|| {
                        unreachable!("walk_cycle reads one carrier per image it derives")
                    }),
                    pcurve,
                    t0,
                    t1,
                    element: None,
                }),
            }
        }
    }
    match certify_walked(derived, &face.surface, band, None) {
        Ok(rows) => Ok(SiteRows::Mint {
            images: rows.into_iter().map(|(at, cache, _)| (at, cache)).collect(),
            joints,
        }),
        Err(_) => Ok(SiteRows::Clear(every_half())),
    }
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
            SiteRows::Mint { images, joints } => {
                for (at, cache) in images {
                    body.attach_pcurve(key(at), cache);
                }
                for (at, element) in joints {
                    body.write_joint(key(at), Some(element));
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
/// ([`chart_pcurve_over`]). Everything after the derivation — the branch
/// pin, the continuity margins, the closure — is [`walk_cycle`], which
/// is stated under `Decide`. An empty loop bounds nothing to chart and
/// appends nothing.
///
/// # Panics
///
/// [`loop_rows`]': `lp` is a loop of `face`'s record.
#[track_caller]
pub(crate) fn walk_loop<T: AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    lp: LoopKey,
    chart: DescribedChart<'_, T>,
    band: Band,
    out: &mut Vec<Walked<T>>,
) -> Result<(), PcurveMintError> {
    let surface = chart.surface();
    let cycle = loop_rows(body, lp);
    let mut carriers: Vec<geom::Curve3<T>> = Vec::with_capacity(cycle.len());
    let item = |i: usize| -> Result<WalkItem<'_, T>, PcurveMintError> {
        let he = cycle[i];
        let (carrier, t0, t1, base) = derive_image(body, he, surface, band)?;
        let plus = is_plus(body, he);
        carriers.push(carrier);
        Ok(WalkItem {
            base: Cow::Owned(base),
            t0,
            t1,
            plus,
            vertex: entry_vertex(body, he),
        })
    };
    let walked =
        walk_cycle(chart, cycle.len(), item, |_| None, band, true).map_err(|fail| match fail {
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
                miss: PinMiss::NetStretchUndecided(cause),
            } => PcurveMintError::NetStretchUndecided {
                half_edge: cycle[index],
                cause,
            },
            WalkFail::NotClosed => PcurveMintError::LoopNotClosed { face },
        })?;
    out.extend(cycle.iter().zip(carriers).zip(walked).map(
        |((&key, carrier), (pcurve, t0, t1, element))| Walked {
            key,
            carrier,
            pcurve: pcurve.into_owned(),
            t0,
            t1,
            element,
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
/// certification; a derivation's refusal or a joint's joins
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
    type Derived<'a, T> = (HalfEdgeKey, geom::Curve3<T>, WalkItem<'a, T>);
    let mut items: Vec<Option<Derived<T>>> = cycle
        .iter()
        .map(|&he| {
            let derived = derive_image(body, he, surface, band).map(|(carrier, t0, t1, base)| {
                let plus = is_plus(body, he);
                let base = Cow::Owned(base);
                let vertex = entry_vertex(body, he);
                (
                    he,
                    carrier,
                    WalkItem {
                        base,
                        t0,
                        t1,
                        plus,
                        vertex,
                    },
                )
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
        let item = |j: usize| -> Result<WalkItem<'_, T>, core::convert::Infallible> {
            Ok(walk_items[j]
                .take()
                .unwrap_or_else(|| unreachable!("walk_cycle reads each item of a run once")))
        };
        match walk_cycle(chart, keys.len(), item, |_| None, band, false) {
            Ok(pinned) => out.extend(keys.iter().zip(carriers).zip(pinned).map(
                |((&key, carrier), (pcurve, t0, t1, element))| Walked {
                    key,
                    carrier,
                    pcurve: pcurve.into_owned(),
                    t0,
                    t1,
                    element,
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
                miss: PinMiss::NetStretchUndecided(cause),
            }) => refused.push(PcurveMintError::NetStretchUndecided {
                half_edge: keys[index],
                cause,
            }),
            Err(WalkFail::Item(never)) => match never {},
            Err(WalkFail::NotClosed) => unreachable!("an open run checks no closure"),
        }
    }
}

/// **One half-edge's chart image on the chart's principal branch**,
/// with its carrier and interval — the derivation the walk pins
/// ([`walk_loop`]). The one step that forks by chart: a SPLINE chart
/// derives from the edge's description ([`nurbs_iso_derive`]), every
/// analytic chart from the carrier's closed form ([`chart_pcurve_over`]).
///
/// # Errors
///
/// [`PcurveMintError::NoCarrier`] for a half of a null edge, or the
/// derivation's refusal.
#[track_caller]
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
/// image on the chart's principal branch — derived, or borrowed from a
/// row that stands — its carrier interval, whether the loop traverses
/// it forward, and the 3-D point of the vertex it enters at — the joint
/// [`decide_joint`] reads the chart's singular set and the azimuth's
/// lever on.
pub(crate) struct WalkItem<'a, T: Real> {
    base: Cow<'a, Pcurve<T>>,
    t0: T,
    t1: T,
    plus: bool,
    vertex: geom_core::Point3<T>,
}

/// One image [`walk_cycle`] placed: the image with its carrier
/// interval, and the element of the joint into it — `None` only for the
/// first item of an open run, which joins nothing.
type Pinned<'a, T> = (Cow<'a, Pcurve<T>>, T, T, Option<JointElement>);

/// Why [`decide_joint`] decided no element.
pub(crate) enum PinMiss {
    /// The image meets the predecessor's exit on no lift: its gap sits
    /// on a mark, or a spline chart's gap is definite.
    Discontinuity,
    /// A decision escalated.
    Escalated(Indeterminate),
    /// The joint is on a spline net closed in `v` whose whole `v` sweep
    /// moves a point within the band (`pcurve_loop_v_stretch` undecided,
    /// [`net_reach`]): its `v` periods are not decided, and the joint
    /// refuses under that name rather than writing an element.
    NetStretchUndecided(Indeterminate),
}

/// Why [`walk_cycle`] refused, with the index of the item it refused
/// at where there is one.
pub(crate) enum WalkFail<E> {
    /// The caller's item producer refused (a derivation that refused,
    /// or a half with no carrier).
    Item(E),
    /// No element joins item `index` to the item before it.
    Miss {
        /// The cycle position of the item.
        index: usize,
        /// Why.
        miss: PinMiss,
    },
    /// The loop does not close: its elements compose to a winding no
    /// loop has ([`Winding::closes`]).
    NotClosed,
}

/// **The one-branch walk, stated under `Decide`.** Walks `len` items in
/// cycle order, asking `item` for each in turn (so an item's refusal
/// is reported in cycle order, before any later item is read), decides
/// the element of each joint between consecutive images
/// ([`decide_joint`]) that `kept` does not hand it, and — when `closes`
/// — the closure joint into the first item and the loop's winding
/// ([`Winding::closes`]), kept elements among it; an open run of a loop
/// ([`walk_runs`]) has no closure. Returns each item's image with its
/// interval and the element into it.
///
/// Every joint is decided between two IMAGES, each a function of its
/// edge and the chart, so no element depends on where the walk starts:
/// the closure joint is decided exactly as every other, and the winding
/// is read off conjugation invariants.
///
/// This is the whole of the minting walk except the derivation, and
/// none of it needs the fitted lane: its callers — [`walk_loop`] and
/// [`walk_runs`], the pass's, and [`site_rows`], the Euler operators'
/// — hand it their own derivation through `item`. The pass keeps no
/// joint; the site mint keeps the element of every joint its door
/// leaves as found, between two images that stand.
fn walk_cycle<'a, T: Decide, E>(
    chart: DescribedChart<'_, T>,
    len: usize,
    mut item: impl FnMut(usize) -> Result<WalkItem<'a, T>, E>,
    kept: impl Fn(usize) -> Option<JointElement>,
    band: Band,
    closes: bool,
) -> Result<Vec<Pinned<'a, T>>, WalkFail<E>> {
    // The chart's own periods (`ChartPeriods::of`): in `u`, `τ` on an
    // analytic azimuth chart, the knot-domain length on a NURBS chart
    // closed in u, and NO shift at all on a chart that does not wrap;
    // in `v`, likewise for the second channel.
    let periods = ChartPeriods::of(chart.surface(), band);
    // The exit of the previous item's image, in chart coordinates, and
    // the first item's entry parameter, for the closure joint.
    let mut prev_exit: Option<geom_core::Point2<T>> = None;
    let mut first_entry: Option<(T, geom_core::Point3<T>)> = None;
    let mut out: Vec<Pinned<'a, T>> = Vec::with_capacity(len);
    for index in 0..len {
        let WalkItem {
            base,
            t0,
            t1,
            plus,
            vertex,
        } = item(index).map_err(WalkFail::Item)?;
        let (entry_t, exit_t) = entry_exit(plus, t0, t1);
        let element = match (prev_exit, kept(index)) {
            (None, _) => None,
            (Some(_), Some(element)) => Some(element),
            (Some(prev), None) => Some(
                decide_joint(chart, &base, entry_t, prev, vertex, periods, band)
                    .map_err(|miss| WalkFail::Miss { index, miss })?,
            ),
        };
        first_entry.get_or_insert((entry_t, vertex));
        prev_exit = Some(base.eval(exit_t));
        out.push((base, t0, t1, element));
    }
    if closes && let (Some(prev), Some((entry_t, vertex))) = (prev_exit, first_entry) {
        // The closure joint: the first image's entry carried onto the
        // last image's exit, decided exactly as every other joint.
        let closure = match kept(0) {
            Some(element) => element,
            None => decide_joint(chart, &out[0].0, entry_t, prev, vertex, periods, band)
                .map_err(|miss| WalkFail::Miss { index: 0, miss })?,
        };
        out[0].3 = Some(closure);
        if !Winding::of(out.iter().filter_map(|pinned| pinned.3)).closes() {
            return Err(WalkFail::NotClosed);
        }
    }
    Ok(out)
}

/// Where a joint's vertex sits against the chart's **singular set**:
/// the points where the first channel stops naming a point, so a joint
/// there has no azimuth and no first-channel periods to decide. Read by
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
/// A spline chart keeps the net-level form: the metres its whole `u`
/// knot domain moves a point at most ([`net_reach`]: the domain's length
/// times `sup |S_u|`, [`chart_u_arm`]), under the band, reads as
/// singular everywhere. That is a property of the net, not of the vertex, and
/// the chart-space gap that spline joints still state
/// ([`spline_gap_closes`]) is what keeps its skip safe.
fn singular_at<T: Decide>(
    chart: DescribedChart<'_, T>,
    vertex: geom_core::Point3<T>,
    band: Band,
) -> Singular {
    let reach = match *chart.surface() {
        // The singular points, from their one home.
        Surface::Sphere { .. } | Surface::Cone { .. } => {
            let Ok(inverse) = geom::ChartInverse::of(chart.surface()) else {
                return Singular::Off;
            };
            let points = geom_brep::chart_singularity::singular_points(&inverse);
            let Some(nearest) = points
                .iter()
                .map(|&(_, p)| vertex.distance(p))
                .reduce(|a, b| a.min(b))
            else {
                return Singular::Off;
            };
            Margin::of(nearest)
        }
        Surface::Cylinder { .. } | Surface::Torus { .. } | Surface::Plane { .. } => {
            return Singular::Off;
        }
        Surface::Nurbs(_) | Surface::Approx(_) => {
            let Some(payload) = chart.surface().spline_chart() else {
                unreachable!("a spline kind answers its spline chart")
            };
            net_reach(chart, payload, NetChannel::U)
        }
    };
    match decide("pcurve_loop_pole_joint", reach, band) {
        Ok(Sign::Zero) => Singular::On,
        Ok(Sign::Positive | Sign::Negative) => Singular::Off,
        Err(cause) => Singular::Undecided(cause),
    }
}

/// The first channel's lever **at a joint**, read off the joint's
/// vertex: on an azimuth chart, the vertex's distance from the chart's
/// axis ([`axis_distance`]), exact at the vertex and so a sup there; on
/// a plane or a spline chart, [`chart_u_arm`]'s rate.
pub(crate) fn joint_arm<T: Real>(
    chart: DescribedChart<'_, T>,
    vertex: geom_core::Point3<T>,
) -> SupSpeed<T> {
    axis_distance(chart, vertex).map_or_else(|| chart_u_arm(chart), SupSpeed::new)
}

/// A vertex's distance from an azimuth chart's axis — cylinder `r`,
/// sphere `r·|cos v|`, torus `R + r·cos v`, cone `|v·sin α|`, each at
/// the vertex, but computed from the vertex point rather than from a
/// stored latitude. `None` on a plane or a spline chart, which has no
/// axis.
fn axis_distance<T: Real>(chart: DescribedChart<'_, T>, vertex: geom_core::Point3<T>) -> Option<T> {
    let off_axis = |origin: geom_core::Point3<T>, axis: geom_core::Vec3<T>| {
        let w = vertex - origin;
        (w - axis * w.dot(axis)).norm()
    };
    match *chart.surface() {
        Surface::Cylinder { radius, .. } => Some(radius),
        Surface::Sphere { center, axis, .. } | Surface::Torus { center, axis, .. } => {
            Some(off_axis(center, axis))
        }
        Surface::Cone { apex, axis, .. } => Some(off_axis(apex, axis)),
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => None,
    }
}

/// How [`decide_joint`] reads a joint's first channel: the chart × the
/// vertex's [`Singular`] reading, as one value.
#[derive(Clone, Copy, Debug)]
enum AzimuthRule {
    /// No azimuth integer, and the joint is an ordinary one: the chart
    /// has no period.
    Aperiodic,
    /// No azimuth integer, and the joint is a reset
    /// ([`JointElement::Reset`]): the vertex is decided ON the singular
    /// set, so every azimuth names the point. On an analytic chart that
    /// is 3-D incidence on a pole or an apex; on a spline chart, a net
    /// whose `u` knot domain moves a point under the band ([`net_reach`],
    /// [`singular_at`]). The skip rests on that decision alone.
    OnSingularSet,
    /// No azimuth integer, and the joint is a reset: a spline chart
    /// whose net-level gate is UNDECIDED. The reset is written on an
    /// undecided reading here, and only here. Its soundness is not the
    /// incidence's but the joint's chart-space gap, which the spline
    /// joint decides at the unshifted entry ([`spline_gap_closes`]): no
    /// azimuth integer is claimed, and the two chart points are shown to
    /// agree within the band through the net's sup stretch. An analytic
    /// chart's undecided incidence takes [`AzimuthRule::MarksNearPole`]
    /// instead.
    SplineUndecided,
    /// The integer by the marks, and where they miss, an orbit point
    /// whose gap is decided within ε at the vertex's lever
    /// ([`lever_periods`]): a vertex decided off the singular set can
    /// still have a band-sized lever (near a narrow cone's apex, where
    /// the incidence reads `|p − apex|` and the lever is that times
    /// `sin α`). Where neither closes, the marks' own miss, so a gap on
    /// a mark is a decided discontinuity, except that the read's
    /// escalation answers where one of its offsets is in band.
    Marks,
    /// The same read as [`AzimuthRule::Marks`], with one difference: a
    /// miss is the incidence's escalation, never the marks' own. The
    /// vertex may be ON the set, where every azimuth names the point, so
    /// no miss there is a decided discontinuity or a decided reach. The
    /// two differ in that error alone, and its readers read it: a
    /// discontinuity refuses as decided ([`turn_element`]'s `Err(None)`,
    /// [`PcurveMintError::LoopDiscontinuity`]), an escalation as
    /// undecided.
    MarksNearPole(Indeterminate),
}

impl AzimuthRule {
    fn of(periodic: bool, singular: Singular, spline: bool) -> Self {
        match (periodic, singular) {
            (false, _) => Self::Aperiodic,
            (true, Singular::On) => Self::OnSingularSet,
            (true, Singular::Undecided(_)) if spline => Self::SplineUndecided,
            (true, Singular::Off) => Self::Marks,
            (true, Singular::Undecided(cause)) => Self::MarksNearPole(cause),
        }
    }
}

/// **The ONE decision of one joint**: the element ([`JointElement`])
/// that carries `image`'s entry (at `entry_t`) onto `prev`, the exit of
/// the image before it, at the joint's vertex `vertex`. Two chart
/// points with one 3-D image differ by a deck element of the chart: a
/// whole number of periods per periodic channel and, on a sphere,
/// possibly the involution `(u, v) ↦ (u + π, π − v)` ([`sphere_twin`]).
/// This decides which, as integers ([`geom_brep::Period::decide`]),
/// the azimuth's marks metered at the vertex's own lever
/// ([`joint_arm`]).
///
/// Both points are images' — a function of their edges and the chart —
/// and the vertex is the joint's, so the element is too, and a joint
/// reads the same from every walk of its loop.
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
/// nearest orbit points in the chart metric.
///
/// The integer counts the orbit's steps, so on a sphere it counts half
/// periods. The marks are read outward from zero with no cap, however
/// many steps out the gap lies ([`geom_brep::Period::decide`]).
///
/// **What that room buys, and where it is not enough.** Where it exceeds
/// the joint bound below (the chart ends `≤ 4ε` apart in metres), the
/// decided integer is the joint's true deck element. It does not exceed
/// it everywhere [`singular_at`] reads `Off`: `Off` only puts the vertex
/// `K·ε` from a pole, so near a pole at a small `K` the lever `d` is
/// about `K·ε`, and near a narrow cone's apex it is `p·sin α` at slant
/// `p`, small even at `K = 10`. There a mark may decide an orbit point
/// other than the true one, but only one equally near the joint: every
/// orbit point is a lift of the SAME 3-D point, and two that are both
/// within the joint bound are within it of each other, so the lift
/// stays continuous in metres to that bound. Where a mark misses there
/// (the gap on it, or in its band), the orbit point the gap meets within
/// ε at the vertex's lever is taken ([`lever_periods`]): at a sub-band
/// lever every mark reads Zero, and a continuous joint whose gap is
/// within ε of an offset the read tries decides a lift of its own
/// point. Where an offset the read tries is in band, the chart points
/// it would join may be one point, and the joint escalates; a gap on a
/// mark refuses only where the read decides its own nearest orbit
/// points off ([`lever_periods`]).
/// No branch is ever decided that names a different point. What such a
/// pick can move is the loop's winding by one orbit step, so a reader of
/// the winding as a region checks each joint's room first:
/// [`loop_windings`] refuses a joint whose room is not past the joint
/// bound ([`joint_has_room`]).
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
/// is a bound the joint's room is read against ([`joint_has_room`]),
/// not a decision.
///
/// **The first channel's rule** is [`AzimuthRule`]'s. On the singular
/// set no azimuth is decided: every azimuth names the point, the joint
/// is a reset, and that skip rests on the 3-D decision alone. Where the
/// incidence is UNDECIDED nothing is skipped on an analytic chart: the
/// integer is decided as at any other joint, which is sound either way
/// (a deck transformation moves no point), with the same near-pole read
/// where its marks miss ([`lever_periods`]), and any miss left
/// escalates with the incidence's cause. A spline chart has no
/// injectivity lemma (a net can fold, so a 3-D coincidence does not
/// name the sheet), and each of its joints states its chart-space gap
/// as well ([`spline_gap_closes`]), which is what keeps its net-level
/// skip safe.
///
/// **Which writers decide, and which compose or copy.** The writers that
/// decide a joint call this and nothing else: the minting walk (the
/// closure joint read as every other), the site mint's new and missing
/// joints ([`site_rows`]), a split's joint between its children
/// ([`split_cache`]), a kill's turn across a closed carrier
/// ([`turn_element`]), and tier 3 ([`validate_pcurves`]). The rest write
/// elements by algebra or copy, as R's design has them: a kill's bridged
/// joint is the composition of the elements either side
/// ([`JointElement::then`]), a reversed loop's is the inverse
/// ([`JointElement::inverse`], `Body::revert`), a null edge's is the
/// identity (a point), and the site mint's kept elements, a split's
/// carried rows and the boolean graft copy elements between images that
/// stand unchanged. Each is the element this decision gives the same
/// geometry, where its integer has room: the composed path and the
/// bridged joint carry the same image onto the same point. And tier 3
/// re-decides every stored element here, so a written element this
/// decision does not give reads as a [`PcurveMintError::LoopDiscontinuity`]
/// at rest — loud, never silent. Where the room is short (a pole at a
/// small `K`, a narrow cone's apex), a composition may name another lift
/// of the same point than a fresh decision does; that too is loud at
/// rest (`work/topotail/a-kill-bridging-joints-that-straddle-the-pole-lever-band-writes-a-reset-the-pass-decides-a-shift`).
///
/// **So a reset is written in exactly two cases**: a vertex decided on
/// the singular set (any chart), and a spline chart whose `u` net-level
/// gate is undecided ([`AzimuthRule::SplineUndecided`]). The second is a
/// reset on an undecided reading; it is sound by the joint's chart-space
/// gap, not by the incidence. An analytic chart never writes a reset on
/// an undecided incidence. A net closed in `v` whose `v` gate is
/// undecided writes no element at all: the joint refuses as
/// [`PinMiss::NetStretchUndecided`], since a reset, the element modulo
/// `T_u`, would keep the `v` it cannot decide and drop the `u` it did.
fn decide_joint<T: Decide>(
    chart: DescribedChart<'_, T>,
    image: &Pcurve<T>,
    entry_t: T,
    prev: geom_core::Point2<T>,
    vertex: geom_core::Point3<T>,
    periods: ChartPeriods<T>,
    band: Band,
) -> Result<JointElement, PinMiss> {
    let surface = chart.surface();
    let u_period = periods.u();
    let spline = surface.spline_chart().is_some();
    let arm = joint_arm(chart, vertex);
    let twin = sphere_twin(surface, image);
    let orbit = Orbit::of(surface, image);
    let gap = prev.x - image.eval(entry_t).x;
    let marks = |p: Period<T>| lever_periods("pcurve_loop_branch", arm, gap, p, orbit, band);
    let rule = AzimuthRule::of(u_period.is_some(), singular_at(chart, vertex, band), spline);
    let m = match (rule, u_period) {
        (AzimuthRule::Marks, Some(p)) => marks(p).map_err(PinMiss::from)?,
        (AzimuthRule::MarksNearPole(cause), Some(p)) => {
            marks(p).map_err(|_| PinMiss::Escalated(cause))?
        }
        _ => 0,
    };
    // An odd step of a two-step orbit lands on the twin; the whole
    // periods are the steps over the orbit's count.
    let (is_twin, ku) = (
        m.rem_euclid(orbit.steps()) == 1,
        m.div_euclid(orbit.steps()),
    );
    let sheet = match twin {
        Some(t) if is_twin => t,
        _ => image.clone(),
    };
    // The second channel's periods, on the chosen sheet, in the period
    // the chart owns for it (C4): `τ` metered by the polar radius where
    // `v` is an angle, the knot length metred by the net's `sup |S_v|`
    // ([`v_meter`]) on a net closed in `v`. A shift of the first channel
    // moves no second-channel value.
    let gap_v = prev.y - sheet.eval(entry_t).y;
    let kv = match periods.v {
        // A mark decides the sign of `gap − mark`, which any positive
        // rate keeps, so the rate's direction only sizes the band about
        // the mark: the exact polar radius serves as both members, and
        // the net's sup as the escape side.
        Some(VPeriod::Angle { period, polar }) => period
            .decide(
                "pcurve_loop_branch",
                gap_v,
                |g| Margin::metered_sup(g, polar),
                band,
            )
            .map_err(|stop| PinMiss::from(stop.miss))?,
        // A net's `v` periods are read by the marks only where its `v`
        // knot domain moves a point past the band ([`net_reach`]), as its
        // `u` periods are ([`singular_at`]). Decided under it, no `v`
        // displacement on the chart reaches ε, every lift names the same
        // point, and the joint is an ordinary one with `kv = 0`, its
        // chart-space gap deciding ([`spline_gap_closes`]). Undecided, the
        // joint refuses ([`PinMiss::NetStretchUndecided`]). It does not
        // write a reset as `u`'s undecided gate does: a reset stores the
        // element modulo `T_u`, so it would drop a decided `u` element and
        // keep the undecided `v`, the inverse of what it marks. The case
        // is a degenerate net, one whose whole `v` sweep is within the
        // band's sliver.
        Some(VPeriod::Net(period)) => {
            let Some(payload) = surface.spline_chart() else {
                unreachable!("a net's period is a spline chart's")
            };
            match decide(
                "pcurve_loop_v_stretch",
                net_reach(chart, payload, NetChannel::V),
                band,
            ) {
                Ok(Sign::Positive) => period
                    .decide(
                        "pcurve_loop_branch",
                        gap_v,
                        |g| Margin::metered_sup(g, v_meter(chart)),
                        band,
                    )
                    .map_err(|stop| PinMiss::from(stop.miss))?,
                Ok(Sign::Zero | Sign::Negative) => 0,
                Err(cause) => return Err(PinMiss::NetStretchUndecided(cause)),
            }
        }
        None => 0,
    };
    // A spline chart has no angular channel, so its lift is the sheet
    // moved by the periods of each channel it closes in, and the
    // joint's chart-space gap is read on that lift.
    if spline {
        let mut lifted = sheet.clone();
        if let Some(p) = u_period {
            lifted = lifted.shift_branch(ku, p);
        }
        if let Some(p) = periods.v() {
            lifted = shift_v_branch(&lifted, kv, p);
        }
        spline_gap_closes(chart, lifted.eval(entry_t), prev, band)?;
    }
    let deck = Deck {
        u: ku,
        v: kv,
        twin: is_twin,
    };
    Ok(match rule {
        AzimuthRule::OnSingularSet | AzimuthRule::SplineUndecided => JointElement::Reset(deck),
        AzimuthRule::Aperiodic | AzimuthRule::Marks | AzimuthRule::MarksNearPole(_) => {
            JointElement::Shift(deck)
        }
    })
}

/// **A killed half's turn**: the element carrying its image's exit onto
/// its entry, where the two coincide in space — a closed carrier, whose
/// two vertices hold one point. A kill that crosses the half whole sums
/// it with the elements either side ([`crate::Body::kev_describing`](crate::construction::Construct::kev_describing)).
/// `Ok(None)` where the half stores no image or its ends do not meet.
///
/// Whether the ends meet is decided in 3-D, as the distance between the
/// edge's two vertices (`pcurve_turn_closes`): the joint decision
/// ([`decide_joint`]) takes the coincidence of the two points it joins as
/// given, so it is not asked until the coincidence is decided. Where they
/// meet, the turn is decided at that point as every joint is.
///
/// # Errors
///
/// Whether the ends meet escalated at `band` (`Some`, its cause), or
/// they meet and the turn's joint decision there missed: it escalated
/// (`Some`, a near-pole offset in band included), or decided a miss
/// (`None`) — the gap on a mark and decided off every orbit point the
/// near-pole read tries at the vertex's lever. The ends are one point,
/// so neither is "no turn"; a kill writes no element through any of
/// them.
///
/// # Panics
///
/// Where `half_edge`'s face, surface, edge or vertices do not resolve.
#[track_caller]
pub(crate) fn turn_element<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
    band: Band,
) -> Result<Option<JointElement>, Option<Indeterminate>> {
    let Some(row) = body.pcurve(half_edge) else {
        return Ok(None);
    };
    let surface = half_edge_surface(body, half_edge);
    let Some(chart) = DescribedChart::of(&surface) else {
        return Ok(None);
    };
    let (start, end) = edge_vertex_points(body, half_edge);
    match decide("pcurve_turn_closes", Margin::of(start.distance(end)), band) {
        Ok(Sign::Zero) => {}
        Ok(Sign::Positive | Sign::Negative) => return Ok(None),
        Err(cause) => return Err(Some(cause)),
    }
    let (t0, t1) = row.params();
    let (entry_t, exit_t) = entry_exit(is_plus(body, half_edge), t0, t1);
    let image = row.pcurve();
    match decide_joint(
        chart,
        image,
        exit_t,
        image.eval(entry_t),
        entry_vertex(body, half_edge),
        ChartPeriods::of(&surface, band),
        band,
    ) {
        Ok(element) => Ok(Some(element)),
        Err(PinMiss::Escalated(cause) | PinMiss::NetStretchUndecided(cause)) => Err(Some(cause)),
        Err(PinMiss::Discontinuity) => Err(None),
    }
}

/// A half-edge's `(entry, exit)` carrier parameters in its loop's
/// direction, from its row's forward interval `(t0, t1)`: its own order
/// on a plus half, reversed on a minus one.
fn entry_exit<T: Copy>(plus: bool, t0: T, t1: T) -> (T, T) {
    if plus { (t0, t1) } else { (t1, t0) }
}

impl From<BranchMiss> for PinMiss {
    fn from(miss: BranchMiss) -> Self {
        match miss {
            BranchMiss::Undecided(cause) => Self::Escalated(cause),
            BranchMiss::OnMark => Self::Discontinuity,
        }
    }
}

/// **A joint's orbit**: the step its marks count in, and the offsets
/// about the image the near-pole read tries first
/// ([`near_pole_gap_closes`]), in that step; the read then tries the
/// two branches astride the mark the walk stopped on. Both are read off
/// one fact, the steps in a period ([`Orbit::steps`]), so an offset is
/// never read in the other orbit's step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Orbit {
    /// No twin: whole periods. The image itself, then the image one
    /// period either way round.
    Periods,
    /// A sphere twin half a period over: half periods. The image, the
    /// twin either way round, then the image one period either way
    /// round.
    HalfPeriodsWithTwin,
}

impl Orbit {
    /// The orbit of a joint into `image` on `surface`: half periods
    /// where the image has a sphere twin ([`sphere_twin`]), whole periods
    /// elsewhere. The one fact the walk ([`decide_joint`]) and the room
    /// fence ([`joint_has_room`]) both read.
    pub(crate) fn of<T: Real>(surface: &Surface<T>, image: &Pcurve<T>) -> Self {
        if sphere_twin(surface, image).is_some() {
            Self::HalfPeriodsWithTwin
        } else {
            Self::Periods
        }
    }

    /// The steps in one period: 1, or 2 where the twin sits half a
    /// period over.
    pub(crate) fn steps(self) -> i32 {
        match self {
            Self::Periods => 1,
            Self::HalfPeriodsWithTwin => 2,
        }
    }

    /// The offsets tried, first to last, in [`Orbit::step`]s: the image,
    /// then each step out to one period, either way round.
    pub(crate) fn offsets(self) -> impl Iterator<Item = i32> + Clone {
        core::iter::once(0).chain((1..=self.steps()).flat_map(|m| [m, -m]))
    }

    /// The step of `period` this orbit counts in, as the period its
    /// marks are read in.
    pub(crate) fn step<T: Real>(self, period: Period<T>) -> Period<T> {
        match self.steps() {
            1 => period,
            n => Period::new(period.length() * T::from_f64(1.0 / f64::from(n))),
        }
    }
}

/// What the near-pole read decided of a joint's orbit
/// ([`near_pole_gap_closes`]).
#[derive(Clone, Debug)]
enum NearRead {
    /// The gap meets the offset `m` within ε at the vertex's lever.
    Closes(i32),
    /// Every offset tried is decided farther than ε off.
    Off,
    /// No offset closes, and one is in band (its cause): the chart
    /// points it would join may be within ε in metres, so whether the
    /// joint is continuous is undecided.
    InBand(Indeterminate),
}

/// The branch of a joint whose orbit's marks missed at a band-sized
/// lever `arm` ([`lever_periods`]): the vertex sits within the band of
/// a pole or an apex (its incidence undecided), or off a narrow cone's
/// apex by more than the band while its lever `|p − apex|·sin α` is not,
/// so no mark reads definitely. The offsets tried are `orbit`'s
/// ([`Orbit::offsets`]), then the branches `astride` the mark the
/// marks' walk stopped on, in `orbit`'s steps of `period`; the first
/// whose gap, metered at that lever, is decided within ε
/// (`pcurve_loop_pole_gap`) is taken ([`first_within`]).
/// The two chart points then agree in metres at the vertex, and no
/// branch is asserted that a margin did not see. Every offset tried is
/// a lift of the one point: a whole period is a deck transformation of
/// the chart, and a half step lands on the twin, which names the point
/// once the caller's second-channel count on it decides. Where none
/// closes, [`NearRead::InBand`] with the first in-band offset's cause,
/// else [`NearRead::Off`].
///
/// A vertex ON the axis but off the surface (within the band of a pole)
/// has lever 0, so any gap decides Zero there and the image itself is
/// taken. That is sound in 3-D: every chart end of the joint then lies
/// within a few ε of the pole, where no azimuth names a point; and
/// [`loop_windings`] reports such a joint's loop singular and reads no
/// winding of it.
fn near_pole_gap_closes<T: Decide>(
    arm: SupSpeed<T>,
    gap: T,
    period: Period<T>,
    orbit: Orbit,
    astride: Option<[i32; 2]>,
    band: Band,
) -> NearRead {
    let step = orbit.step(period);
    let bracket = astride
        .into_iter()
        .flatten()
        .filter(|m| !orbit.offsets().any(|o| o == *m));
    let tried = orbit.offsets().chain(bracket);
    match first_within("pcurve_loop_pole_gap", tried, band, |&m| {
        Margin::metered_sup(gap - step.times(m), arm)
    }) {
        Ok(Some(m)) => NearRead::Closes(m),
        Ok(None) => NearRead::Off,
        Err(cause) => NearRead::InBand(cause),
    }
}

/// **A joint's whole orbit steps by the marks, with the near-pole
/// read**: [`geom_brep::Period::decide`] of `gap` in `orbit`'s
/// steps of `period`, metered at the vertex's lever `arm`. Where a mark
/// misses (undecided, or the gap on it), an orbit point whose gap is
/// decided within ε at that lever is taken ([`near_pole_gap_closes`],
/// trying `orbit`'s offsets in order). A band-sized lever reads every
/// mark as Zero: a vertex near a pole, and a vertex near a narrow
/// cone's apex, whose incidence decides it OFF the apex ([`singular_at`]
/// reads `|p − apex|`) while its lever `|p − apex|·sin α` is under the
/// band. The two chart points then agree in metres at the vertex. A
/// whole-period offset is a lift of the joint's own point; an odd
/// offset of [`Orbit::HalfPeriodsWithTwin`] is a half step onto the
/// twin, which names the joint's point only once the caller's
/// second-channel count on the twin decides ([`decide_joint`]).
///
/// **Where no offset closes, the read's own verdict stands over the
/// marks'.** A mark decides Zero at a band-sized lever because every
/// azimuth there is within the band in metres, so the marks' `OnMark`
/// says only that the gap is half a step off an orbit point, not that
/// the two chart points are apart. Where an offset is in band, they may
/// be one point, and the answer is an escalation.
///
/// **A decided miss stands only where no orbit point is within the
/// band.** Besides the orbit's offsets about the image, the read tries
/// the two branches astride the mark the walk stopped on
/// ([`geom_brep::WalkStop`]). Where the marks miss as `OnMark`, that
/// mark decided Zero, so the gap is within the band's zero of it; where
/// the read then decides both branches astride it off, the gap lies
/// between them (past either, that branch would be within ε), so
/// `π·lever ≥ escalate`, and every other orbit point is at least
/// `3π·lever − ε` off, past the band too, at any `K > 1`.
///
/// At a lever with room nothing moves: the read is asked only where a
/// mark missed, so where the gap is within the band of a mark, and a
/// gap within ε of an orbit point is that near a mark only where half a
/// step at the lever is itself band-sized, whichever orbit point it is.
/// Likewise an offset in band needs the gap within the band of it.
///
/// # Errors
///
/// The marks' [`BranchMiss`] where the read does not close, except
/// that a decided `OnMark` is the read's [`BranchMiss::Undecided`]
/// where an offset is in band. An undecided mark keeps its own cause.
pub(crate) fn lever_periods<T: Decide>(
    name: &'static str,
    arm: SupSpeed<T>,
    gap: T,
    period: Period<T>,
    orbit: Orbit,
    band: Band,
) -> Result<i32, BranchMiss> {
    // A mark decides the sign of `gap − mark`, which any positive rate
    // keeps; the exact vertex lever serves as both members, the spline
    // chart's sup as the escape side.
    match orbit
        .step(period)
        .decide(name, gap, |g| Margin::metered_sup(g, arm), band)
    {
        Ok(k) => Ok(k),
        Err(WalkStop {
            miss: miss @ (BranchMiss::Undecided(_) | BranchMiss::OnMark),
            astride,
        }) => match near_pole_gap_closes(arm, gap, period, orbit, astride, band) {
            NearRead::Closes(m) => Ok(m),
            NearRead::InBand(cause) => match miss {
                BranchMiss::OnMark => Err(BranchMiss::Undecided(cause)),
                undecided => Err(undecided),
            },
            NearRead::Off => Err(miss),
        },
    }
}

/// **The two branches astride a half-period mark**: where `gap` sits on
/// or near the mark between `j` and `j + 1` periods, `[j, j + 1]`, with
/// `j` decided by the marks of `gap` less half a period, which then sits
/// near `j` periods, half a period from either of its own marks. A caller
/// whose marks missed asks its question on both and keeps an answer only
/// where the two agree.
///
/// # Errors
///
/// [`BranchMiss`] where `j`'s own marks miss.
pub(crate) fn branches_astride<T: Decide>(
    name: &'static str,
    gap: T,
    period: T,
    meter: impl Fn(T) -> Margin<T>,
    band: Band,
) -> Result<[i32; 2], BranchMiss> {
    let j = Period::new(period)
        .decide(name, gap - period * T::from_f64(0.5), meter, band)
        .map_err(|stop| stop.miss)?;
    Ok([j, j + 1])
}

/// **A spline chart's joint gap**: the entry `entry` and the
/// predecessor's exit `prev` agree within ε through the chart's sup
/// stretch on each channel ([`chart_u_arm`], [`v_meter`]). A net can
/// fold, so on a spline chart a 3-D coincidence does not name the
/// sheet, and the joint states its chart-space gap ([`decide_joint`]).
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
    let arm = chart_u_arm(chart);
    for margin in [
        Margin::metered_sup(entry.x - prev.x, arm),
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

/// **Whether a joint's azimuth integer was decided with room** past the
/// joint bound ([`loop_windings`]' room fence). The walk decides the
/// integer against marks half the step to the next orbit point away
/// ([`decide_joint`]): half a period on a cylinder, cone or torus, a
/// quarter on a sphere. A mark can name another orbit point than the
/// joint's own only where the joint's two chart ends lie at least that
/// half step apart in azimuth. Each end is within `2ε` of the vertex in
/// 3-D (its row's envelope and the edge certificate's pin), so its
/// distance from the axis is at least `d − 2ε` at the vertex's own
/// distance `d`, and two such points half a step apart are at least the
/// chord `2·(d − 2ε)·sin(step/4)` apart. Where that chord exceeds the
/// joint bound `4ε` no such pair exists, and the integer is the
/// joint's own. So the fence decides the chord in the same quantity
/// as the bound: a quarter of it, `(d − 2ε)·sin(step/4)/2`, must
/// decide definitely past the band's escalation (`loop_windings_joint_room`),
/// which puts the chord past `4·K·ε > 4ε` at every admitted `K > 1`.
/// Anything else refuses — the conservative side. On a sphere that
/// admits `d > 2ε + 2√2·K·ε`; on a cylinder, cone or torus,
/// `d > 2ε + 2·K·ε`.
///
/// A chart with no period decides no integer. A spline chart is exempt:
/// each of its joints already decided its lifted chart-space gap within
/// the band through the net's sup stretch ([`spline_gap_closes`]), which
/// pins the lift itself rather than the room of its integer.
fn joint_has_room<T: Decide>(
    chart: DescribedChart<'_, T>,
    vertex: geom_core::Point3<T>,
    u_period: Option<Period<T>>,
    orbit: Orbit,
    band: Band,
) -> bool {
    let Some(period) = u_period else {
        return true;
    };
    // A periodic chart with no axis is a spline chart, exempt above.
    let Some(distance) = axis_distance(chart, vertex) else {
        return true;
    };
    // The step the walk decided this joint's integer in.
    let step = orbit.step(period).length();
    // The ends' least distance from the axis, and a quarter of the
    // chord between two of them half a step apart.
    let lever = distance - T::from_f64(2.0 * band.zero());
    let quarter_chord = (step * T::from_f64(0.25)).sin() * T::from_f64(0.5);
    matches!(
        decide(
            "loop_windings_joint_room",
            Margin::levered(quarter_chord, lever),
            band,
        ),
        Ok(Sign::Positive)
    )
}

/// A chart's `u` period, decided by surface KIND: `τ` for every surface
/// of revolution, a closed spline chart's own knot domain, and none for
/// a plane. `chart_u_period` answers `τ` for a PLANE — it defaults every
/// non-spline chart to the azimuth period — so a reader that must tell a
/// plane from a periodic chart asks this one.
pub(crate) fn chart_period<T: AtRestPolicy>(chart: &Surface<T>, band: Band) -> Option<Period<T>> {
    match *chart {
        Surface::Plane { .. } => None,
        Surface::Cylinder { .. }
        | Surface::Cone { .. }
        | Surface::Sphere { .. }
        | Surface::Torus { .. } => Some(Period::new(T::tau())),
        Surface::Nurbs(_) | Surface::Approx(_) => chart_u_period(chart, band),
    }
}

/// One loop of a face as [`loop_windings`] reads it off the walk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoopWinding {
    /// The loop.
    pub r#loop: LoopKey,
    /// Some joint of the loop lies on the chart's singular set (a
    /// sphere pole, a cone apex), decided as 3-D incidence
    /// ([`singular_at`]). Such a loop has no first-channel winding to
    /// read, and its `zero` is `false`.
    pub singular: bool,
    /// The loop's joints compose to the identity deck element, in both
    /// channels ([`Winding::is_zero`]): its lift closes on its start.
    pub zero: bool,
}

/// One loop walked as [`loop_windings`] reads it ([`read_loop`]).
struct ReadLoop {
    /// Each joint's element, in walk order.
    elements: Vec<JointElement>,
    /// The first joint not definitely off the chart's singular set, and
    /// the reading's diagnostic where it was undecided rather than `On`.
    /// Nothing past the walk is read of such a loop.
    singular: Option<(HalfEdgeKey, Option<Indeterminate>)>,
}

/// **The loop walk [`loop_windings`] stands on**: `lp` walked in `chart`, and, off the singular
/// set, every closed-form row certified and every joint's room decided,
/// so the winding read off the elements is the loop's own. `None` for
/// an empty loop, which bounds nothing.
///
/// THE SINGULAR-JOINT FENCE comes first. A joint on the chart's singular
/// set — a sphere pole, a cone apex — has no azimuth, so the walk
/// decides no azimuth periods there (a reset, [`decide_joint`]) and
/// leaves whatever azimuth the derivation produced. Minting is right to
/// do that: no downstream sample reads the azimuth AT the pole. A winding
/// read across it has no first channel to count. Decided as 3-D incidence of each joint's
/// vertex on that set ([`singular_at`]). Every reset the walk wrote
/// reads not-Off here, because the walk writes a reset only on a
/// `singular_at` reading of `On` or, on a spline chart, an undecided
/// one ([`AzimuthRule`]), and this reads the same `singular_at` at the
/// same band; on a spline chart that reading is net-level
/// ([`net_reach`]).
///
/// Off the singular set each closed-form row is certified against
/// `chart` as the row it is: the joint bound rests on each row's
/// envelope (check 4), and these rows are fresh derivations in the
/// CALLER's chart, which no mint certified; a fitted or general one is
/// not re-certified here. Then THE ROOM FENCE:
/// the walk decided each joint's azimuth integer with half the step to
/// the next orbit point as room, at the vertex's own lever, and where
/// that room is not past the joint bound a mark may have named an orbit
/// point other than the joint's own, so the winding may read one step
/// off ([`joint_has_room`]).
///
/// # Errors
///
/// The loop walk's own refusals ([`walk_loop`]);
/// [`PcurveMintError::Certify`] for a row that does not certify against
/// `chart`; [`PcurveMintError::JointWithoutRoom`] for a joint without
/// room.
fn read_loop<T: AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    lp: LoopKey,
    (chart, described): (&Surface<T>, DescribedChart<'_, T>),
    period: Option<Period<T>>,
    band: Band,
) -> Result<Option<ReadLoop>, PcurveMintError> {
    let mut walked: Vec<Walked<T>> = Vec::new();
    walk_loop(body, face, lp, described, band, &mut walked)?;
    if walked.is_empty() {
        return Ok(None);
    }
    let elements: Vec<JointElement> = walked
        .iter()
        .map(|w| {
            w.element
                .unwrap_or_else(|| unreachable!("a closed walk decides every joint"))
        })
        .collect();
    let singular =
        walked.iter().find_map(
            |w| match singular_at(described, entry_vertex(body, w.key), band) {
                Singular::Off => None,
                Singular::On => Some((w.key, None)),
                Singular::Undecided(cause) => Some((w.key, Some(cause))),
            },
        );
    if singular.is_none() {
        for w in &walked {
            if !matches!(w.pcurve, Pcurve::Fitted(_) | Pcurve::General(_)) {
                PcurveCache::certify(w.pcurve.clone(), w.t0, w.t1, &w.carrier, chart, band)
                    .map_err(|error| PcurveMintError::Certify {
                        half_edge: w.key,
                        error,
                    })?;
            }
        }
        for w in &walked {
            let orbit = Orbit::of(chart, &w.pcurve);
            if !joint_has_room(described, entry_vertex(body, w.key), period, orbit, band) {
                return Err(PcurveMintError::JointWithoutRoom {
                    face,
                    r#loop: lp,
                    half_edge: w.key,
                });
            }
        }
    }
    Ok(Some(ReadLoop { elements, singular }))
}

/// **How each loop of `face` winds in `chart`**, read off the loop walk:
/// what the section certificate's W2 reads
/// (`boolean::ops::ChartCache::describes`). Every closed-form row of a
/// loop off the chart's singular set is certified against `chart`, and
/// every joint's room decided, before its winding is read. A loop with a
/// joint on the singular set is reported `singular`, and nothing else of
/// it is read. Loops come outer first; an empty loop is absent.
///
/// # Errors
///
/// [`PcurveMintError::Stale`] when `face` does not resolve, and
/// [`PcurveMintError::PlaceholderChart`] when `chart` is the mvfs
/// placeholder; the loop walk's own refusals;
/// [`PcurveMintError::Certify`] for a row that does not certify against
/// `chart`; [`PcurveMintError::JointWithoutRoom`] for a joint without
/// room; and [`PcurveMintError::Escalated`] for a joint whose incidence
/// on the singular set is undecided.
pub fn loop_windings<T: AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    chart: &Surface<T>,
    band: Band,
) -> Result<Vec<LoopWinding>, PcurveMintError> {
    let face_data = body.get_face(face).ok_or(PcurveMintError::Stale {
        role: "face",
        key: EntityId::Face(face),
    })?;
    let described = DescribedChart::of(chart).ok_or(PcurveMintError::PlaceholderChart { face })?;
    let period = ChartPeriods::of(chart, band).u();
    let mut out = Vec::new();
    for lp in core::iter::once(face_data.outer).chain(face_data.rings.iter().copied()) {
        let Some(read) = read_loop(body, face, lp, (chart, described), period, band)? else {
            continue;
        };
        out.push(match read.singular {
            Some((half_edge, Some(cause))) => {
                return Err(PcurveMintError::Escalated { half_edge, cause });
            }
            Some((_, None)) => LoopWinding {
                r#loop: lp,
                singular: true,
                zero: false,
            },
            None => LoopWinding {
                r#loop: lp,
                singular: false,
                zero: Winding::of(read.elements).is_zero(),
            },
        });
    }
    Ok(out)
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
///    [`geom_brep::EdgeCurve::recertify`]). A row is its placed image
///    (C4), or on a sphere its involution twin; where a loop's walk reads
///    it a whole period over, that is the row with its joint's deck
///    element, never a stored row of its own.
/// 4. **Each joint's element is re-decided, and the winding is their
///    sum.** At every joint of every loop — the one into the cycle's
///    `first` read as every other — the element is decided between the
///    two images ([`decide_joint`]), and a stored element other than
///    the decided one is a [`PcurveMintError::LoopDiscontinuity`]. A
///    half-edge that stores no image is carried by the image the mint
///    would derive there, and the joints either side of it by the
///    elements decided there, so the rows either side of a gap are
///    measured across it; a gap the derivation cannot place leaves its
///    two joints unread. Where every joint reads, the elements composed
///    once around must close ([`Winding::closes`],
///    [`PcurveMintError::LoopNotClosed`]). So a body whose elements
///    were tampered with fails here even if each image certifies in
///    isolation, and no verdict hangs on which half-edge is `first`,
///    which is the gap, or which branch the loop's images stand on.
///
/// Returns the findings in face-arena / loop / cycle order (D9),
/// empty when the body is clean. A planar face stores nothing and is
/// not read.
///
/// # Panics
///
/// Where a record the pass reads does not resolve or a loop walk does
/// not close: tier 3, the one caller in the validator, runs only on a
/// body tier 1 passed, and every public door keeps the body
/// tier-1-valid.
#[track_caller]
pub fn validate_pcurves<T: AtRestPolicy>(body: &Body<T>, band: Band) -> Vec<PcurveMintError> {
    let mut findings = Vec::new();
    // The fitted door every `Fitted`/`General` row re-derives through,
    // read off the scalar's policy once: the certified and the
    // structural validation doors reach this pass alike, so neither
    // moves a verdict at any scalar.
    let lane = T::fitted_lane();
    for (face_key, face) in body.faces() {
        let surface = body.face_surface_linked(face_key, face);
        let Some(chart) = DescribedChart::minting(surface) else {
            continue;
        };
        // One walk of the face's loops ([`stored_rows`], shared with
        // `split_cache` and the Euler operators' site mint): the rows
        // it stores and the gaps.
        let stored = stored_rows(body, face_key);
        if !stored.complete() {
            // A loop a null edge holds open cannot be walked (its halves
            // have no carrier), so there is no derivation to read; the
            // scaffold is tier 2's finding at rest, and the gaps are
            // reported as found.
            let open = !stored.open_loops(body).is_empty();
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
            findings.extend(stored.gaps.iter().map(|gap| PcurveMintError::MissingCache {
                half_edge: gap.half_edge,
            }));
            if let Some(e) = refusal.filter(|e| !not_owed::<T>(e)) {
                findings.push(e);
            }
        }
        let cycles: Vec<Vec<HalfEdgeKey>> =
            stored.loops.into_iter().map(|(_, cycle)| cycle).collect();
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
                if let Err(error) = cache.recertify(&carrier, surface, band, lane) {
                    findings.push(PcurveMintError::Certify {
                        half_edge: he,
                        error,
                    });
                }
            }
        }
        // Each joint's element, re-decided between the two images it
        // joins, and each loop's winding composed from them. A half that
        // stores no image is carried by the image the mint would derive
        // for it ([`derive_image`]), so the rows either side of a gap are
        // measured across it; a gap the derivation cannot place leaves
        // its two joints unread and the loop's winding unread with them.
        // Every joint is read alike, the one into the cycle's `first`
        // included, and the winding off conjugation invariants
        // ([`Winding::closes`]), so no verdict hangs on which half-edge
        // is `first`.
        let periods = ChartPeriods::of(surface, band);
        let image_of = |he: HalfEdgeKey| -> Option<(Pcurve<T>, T, T)> {
            let plus = is_plus(body, he);
            let (image, t0, t1) = match body.pcurve(he) {
                Some(cache) => {
                    let (t0, t1) = cache.params();
                    (cache.pcurve().clone(), t0, t1)
                }
                None => {
                    let (_, t0, t1, base) = derive_image(body, he, surface, band).ok()?;
                    (base, t0, t1)
                }
            };
            let (entry_t, exit_t) = entry_exit(plus, t0, t1);
            Some((image, entry_t, exit_t))
        };
        for cycle in &cycles {
            let n = cycle.len();
            let images: Vec<Option<(Pcurve<T>, T, T)>> =
                cycle.iter().map(|&he| image_of(he)).collect();
            let mut elements: Vec<Option<JointElement>> = vec![None; n];
            for (i, &he) in cycle.iter().enumerate() {
                let (Some((image, entry_t, _)), Some((before, _, exit_t))) =
                    (&images[i], &images[(i + n - 1) % n])
                else {
                    continue;
                };
                let prev = before.eval(*exit_t);
                let vertex = entry_vertex(body, he);
                let decided = decide_joint(chart, image, *entry_t, prev, vertex, periods, band);
                // A stored element is read only between two stored
                // images: across a gap the chain is carried as the walk
                // would carry it, by the element decided there.
                let stored = body.joint(he).filter(|_| {
                    body.pcurve(he).is_some() && body.pcurve(cycle[(i + n - 1) % n]).is_some()
                });
                let read = match (stored, decided) {
                    (Some(stored), Ok(element)) if stored == element => Ok(stored),
                    (Some(_), Ok(_)) => Err(PinMiss::Discontinuity),
                    (None, decided) => decided,
                    (Some(_), Err(miss)) => Err(miss),
                };
                match read {
                    Ok(element) => elements[i] = Some(element),
                    Err(PinMiss::Discontinuity) => {
                        findings.push(PcurveMintError::LoopDiscontinuity { half_edge: he });
                    }
                    Err(PinMiss::Escalated(cause)) => findings.push(PcurveMintError::Escalated {
                        half_edge: he,
                        cause,
                    }),
                    Err(PinMiss::NetStretchUndecided(cause)) => {
                        findings.push(PcurveMintError::NetStretchUndecided {
                            half_edge: he,
                            cause,
                        });
                    }
                }
            }
            if n > 0
                && elements.iter().all(Option::is_some)
                && !Winding::of(elements.iter().flatten().copied()).closes()
            {
                findings.push(PcurveMintError::LoopNotClosed { face: face_key });
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
        /// were complete, what the operator creates on the loops it
        /// rewires — new images, new joints' elements — and any image
        /// or element it finds missing there is derived exactly as the
        /// pass derives it, and every other row stood the surgery
        /// untouched; on a face whose only gaps a null edge holds open,
        /// or that the operator takes the last null edge off, so is
        /// what is missing on the loops it rewires and leaves running
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
        /// Leaves every image as it found it — a primitive, or a write
        /// the map is not keyed on — and writes on each joint it makes
        /// the element it sums from the joints it bridges (a kill) or
        /// carries across a null edge's point (`mev_null`), keys-only
        /// and exact (module docs). What this bucket rests on is
        /// the tier-3 pcurve pass, and only as far as that pass looks
        /// (module docs): it reports a face missing any of its rows,
        /// and a stored row that no longer certifies; it is silent about a complete
        /// face whose rows were stated in another chart and certify
        /// against this one, and about any face on a chart
        /// [`super::chart_mints`] refuses.
        Neither,
        /// Re-mints through the site mint ([`super::site_rows`]) every
        /// face its description moves the rows of, and keeps the rows it
        /// finds on every other face, as `Neither` does, resting on the
        /// same tier-3 pass for what its write stales in them. Those
        /// faces are the ones a null edge's halves are on where the door
        /// installs its first carrier, which no door before it could
        /// give a row; the ones a certified edge's halves are on where
        /// the door replaces its carrier, since the rows kept there
        /// state the carrier and interval the edge leaves; and, for a kill that re-describes the
        /// members it merges, every face a listed member's halves are
        /// on, null or certified, since the kill moves a certified
        /// member's end with its carrier. On such a face the site mint
        /// selects ([`super::StoredRows::remints`]) it re-mints every
        /// loop no other null edge holds open — every loop of it,
        /// whatever it missed, once no null edge is left on it. Those
        /// loops leave complete, or the face rowless where the
        /// closed-form lane cannot mint it. A certified edge's face on a
        /// spline chart is left as found, for the tier-3 pass.
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
            // ---- Maintains: the doors that re-mint their own staged
            // result. They carry prose here rather than in
            // `review_m1_pr5_internal::ALLOWED` because they assert
            // tier 1 through a surgery scope (`crate::surgery`) and so
            // are no longer allowlisted there; the two tables still
            // have to cover this one population between them. ----
            (
                "merge_coplanar_faces",
                Maintains,
                "calls `merge_coplanar_faces_staged`, which re-mints the staged result \
                 before it is adopted, whenever the operand carried rows, and ends with \
                 `join_edges`, which re-mints again where a join moved rows",
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
                "offset_surfaces_together",
                Maintains,
                "runs `replace_faces_offset`'s body over every moving chart, which re-mints \
                 the clone whole-body before adopting it; with nothing moving it adopts a \
                 joined clone, whose join re-mints where it moved rows",
            ),
            (
                "replace_face_offset",
                Maintains,
                "the one-face spelling of `replace_faces_offset`, which re-mints the clone \
                 before adopting it",
            ),
            (
                "join_edges",
                Maintains,
                "re-mints the staged body whole where it carried rows, before the sweeping \
                 close adopts it",
            ),
            (
                "join_edges_within",
                Maintains,
                "re-mints the faces its kills' kept edges bound (`mint_pcurves_of`), after \
                 dropping the dead cells' rows, before the sweeping close",
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
            (
                "validate_pseudomanifold",
                Neither,
                "`Certifier`'s gate pass: it reads the body through a shared borrow, and its \
                 `&mut self` keeps the certificate the pass derived; no body is written",
            ),
            (
                "restate_surface",
                Neither,
                "a test-support door that overwrites one surface datum and leaves every row \
                 as found; a row the new datum stales is the tier-3 pass's to report",
            ),
            (
                "rotate_shell_faces",
                Neither,
                "a test-support door that rotates each shell's stored face list; no face, \
                 loop or carrier moves, so every row stands as found",
            ),
            ("mvfs", Neither, "Euler operator"),
            (
                "merge_unjoined",
                Maintains,
                "test support: calls `merge_coplanar_faces_unjoined`, which re-mints the \
                 staged result before it is adopted",
            ),
            (
                "mev_null",
                Neither,
                "Euler operator minting a NULL edge: scaffolding with no carrier, so no image \
             to derive, and the loop it joins is held open, missing the edge's two images; \
             its halves carry the identity element and the joint out of each keeps its \
             successor's, read across the edge's point. \
             The door that releases the loop mints it whole, except on a spline chart: an \
             operator rewiring it out from under the edge (the boolean's and the \
             splitting lane's joins), the edge's first description (`set_edge_curve`, or \
             a `kev_describing` that lists it), or the edge's band kill \
             (`kev_describing`, `kef_minting`, `kemr_minting`); its keys-only kill \
             refuses `KeysOnly` where the loop would miss a row; \
             tier 2 refuses a null edge at rest",
            ),
            (
                "kemr",
                Neither,
                "Euler operator: each side's closing joint takes the sum of the two elements \
             it bridges; where it kills a null edge and the loop it releases would miss a \
             row, it refuses `KeysOnly` before mutating",
            ),
            (
                "kev",
                Neither,
                "kill op: the joint it makes takes the sum of the elements it bridges; where \
             it kills a null edge and the loop it releases would miss a row, it refuses \
             `KeysOnly` before mutating",
            ),
            ("kvfs", Neither, "kill op"),
            // ---- The kills' band twins: each mints what a loop its
            // killed null edge releases misses. ----
            (
                "kev_describing",
                Completes,
                "kill op, completing every face a listed member's halves are on: a NULL \
             member's description is its first, and a CERTIFIED member's end moves with its \
             carrier, so each such face is re-minted through `set_edge_curve`'s planner as \
             the kill leaves it (the killed halves gone), every listed member's halves under \
             the curve the kill installs, a certified member's face on a spline chart left \
             as found; and `kev` with a band, minting what a loop a killed null edge \
             releases misses on every other face it is on (`Body::plan_released_rows`)",
            ),
            (
                "kemr_minting",
                Maintains,
                "`kemr` with a band: where `kemr` refuses `KeysOnly`, the two sides it leaves \
             are walked and what they miss is minted (`Body::plan_released_rows`)",
            ),
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
                "kfmrh_describing",
                Transfers,
                "`kfmrh_minting` with the demoted loop's re-descriptions: a re-description \
             keeps its edge's carrier and interval, so the rows `kfmrh_minting` leaves stand",
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
             before mutating, and a spline chart keeps the drop; where they stand, each \
             joint it makes takes the sum of the two elements it bridges, and where it \
             kills a null edge and the loop it releases would miss a row it refuses \
             `KeysOnly` too; the two killed halves' rows outlive their keys as every kill \
             op's do",
            ),
            (
                "kef_minting",
                Transfers,
                "`kef` with a band: where `kef` refuses `KeysOnly`, the surviving loop is \
             re-minted in the surviving face's chart (`Body::plan_moved_rows`), or, where \
             the remnant's rows stand, what the loop a killed null edge releases misses is \
             minted (`Body::plan_released_rows`)",
            ),
            (
                "kef_describing",
                Transfers,
                "`kef_minting` with the moved edges' re-descriptions: a re-description keeps \
             its edge's carrier and interval, so the rows `kef_minting` leaves stand",
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
                "attach_joint",
                Neither,
                "writes ONE joint element the caller chose; tier 3 re-decides it",
            ),
            (
                "detach_joint",
                Neither,
                "drops ONE joint element the caller chose",
            ),
            (
                "set_face_surface",
                Transfers,
                "re-charts a face in place, which changes what every row the face stores is \
             ABOUT while changing no key: the rows are kept across a swap onto the same \
             chart (`Body::same_chart`) and dropped on any other (`Body::drop_rows`, over the loops \
             `Body::face_cycles` proves). \
             Content staleness alone would be \
             the tier-3 pass's, but only where the NEW surface mints — a swap onto a plane \
             or a placeholder left a COMPLETE row set stated in the chart the face left, \
             which that pass skips entirely",
            ),
            (
                "set_face_surface_unvouched_for_tests",
                Transfers,
                "the failure-injection twin of `set_face_surface`, whose rows it keeps and \
             drops on the same terms",
            ),
            (
                "lifting_rechart_refusals_for_tests",
                Neither,
                "a failure-injection scope that writes nothing itself: every door its \
             closure calls holds its own posture",
            ),
            (
                "set_face_surfaces_describing",
                Transfers,
                "`set_face_surface`'s swap per face, on its terms: a face's rows are kept \
             across a move onto the same chart and dropped on any other. The edges it \
             re-describes are the listed certified ones, whose rows it keeps on a face that \
             keeps its chart, for the tier-3 pass to re-certify against the new carrier \
             (`work/restate/set-face-surfaces-describing-keeps-a-moved-edges-rows-on-a-kept-chart`)",
            ),
            (
                "set_edge_curve",
                Completes,
                "a carrier swap is NOT the surface setter's case: neither the row's key nor \
             its chart moves, and pass 2 re-derives each row's agreement from the edge's \
             current carrier. Every description of a CERTIFIED edge re-mints the faces its \
             halves are on, since its rows are stated over the carrier and interval it \
             replaces; one that restates both re-derives the rows it found. A NULL edge's first description is where its halves' rows can first be \
             derived. On a face the halves are on whose only gaps a null edge holds open, \
             every loop no other null edge holds open is re-minted before the door mutates, \
             and on one no null edge is left on, every loop is. A HALF-MINTED face, and a \
             certified edge's face on a spline chart, keep the rows they store, for the \
             pass to re-certify",
            ),
            (
                "describe_at_rest",
                Completes,
                "`set_edge_curve` with the edge's own carrier and interval put back \
                 verbatim, which re-mints the faces the edge's halves are on as any \
                 certified description does",
            ),
            (
                "restate_stored_edge",
                Completes,
                "`describe_at_rest`'s write: `set_edge_curve` with the edge's own carrier and \
                 interval put back verbatim",
            ),
            ("set_face_sense", Neither, "writes one `bool`"),
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
                "plant_disc_face",
                Neither,
                "`mev_line`, `kemr` and `mef`, every one of them already sorted above",
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
            minting.iter().any(|n| n == "join_edges"),
            "`join_edges` no longer reads as calling `mint_pcurves`. Either the door \
             stopped re-minting the rows its kills moved — a finding, and its entry \
             belongs above — or the source read lost the call.",
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

    use super::{DescribedChart, Singular, SupSpeed, chart_u_arm, singular_at, v_meter};
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
            NurbsSurface::new(
                geom_core::spline::Knots::from(kv.clone()).lift(geom_core::Real::from_f64),
                geom_core::spline::Knots::from(kv).lift(geom_core::Real::from_f64),
                control,
                vec![geom_core::Real::one(); 4],
            )
            .unwrap(),
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
        let arm = chart_u_arm(chart(&s));
        assert_eq!(arm.get(), 100.0, "the chart's own metre stretch, not 1");
        let gap = 1e-10;
        assert_eq!(
            decide(
                "pcurve_loop_continuity",
                Margin::metered_sup(gap, arm),
                band()
            ),
            Ok(Sign::Positive),
            "1e-10 chart units × 100 m/unit = 1e-8 m, at the escalate edge"
        );
        assert_eq!(
            decide(
                "pcurve_loop_continuity",
                Margin::metered_sup(gap, SupSpeed::new(1.0)),
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
            chart_u_arm(chart(&large)).get(),
            chart_u_arm(chart(&small)).get() * 1e3
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
        assert_eq!(chart_u_arm(chart(&p)).get(), 1.0);
        assert_eq!(v_meter(chart(&p)).get(), 1.0);
    }

    /// **Three-outcome posture on the newly-honest arm.** A chart gap
    /// whose metred size lands INSIDE the band escalates typed rather
    /// than picking a side: `5e-11 × 100 = 5e-9 ∈ (1e-9, 1e-8)`.
    #[test]
    fn an_in_band_metred_gap_escalates_rather_than_deciding() {
        let s = flat_chart(100.0);
        let arm = chart_u_arm(chart(&s));
        assert!(
            decide(
                "pcurve_loop_continuity",
                Margin::metered_sup(5e-11, arm),
                band()
            )
            .is_err(),
            "in-band residue is the third outcome, not a verdict"
        );
        assert_eq!(
            decide(
                "pcurve_loop_continuity",
                Margin::metered_sup(5e-12, arm),
                band()
            ),
            Ok(Sign::Zero),
            "5e-10 m is honestly closed"
        );
    }

    /// **The singular gate answers all three ways**, through
    /// [`singular_at`], the one door the walk reads. On a spline chart
    /// it is net-level (the metres the chart's `u` knot domain moves a
    /// point, whatever the vertex); on a sphere it is the vertex's 3-D distance to the
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

    /// **On a spline chart, a gate that does not read `Off` writes a
    /// reset**, the undecided reading included: a net whose `u` knot
    /// domain moves a point within the band (`net_reach`) names no
    /// azimuth integer, so none is decided, and the joint's chart-space
    /// gap is what decides it (`spline_gap_closes`, here a zero gap).
    /// Read off a net whose reach is under the band, in it, and past it,
    /// on a chart given a period of 1: a reset, a reset, an ordinary
    /// identity. The net's control points are all within the band of
    /// each other below the band, so it also reads closed in `v`
    /// (`chart_v_period`); its `v` reach gates the `v` periods the same
    /// way (`pcurve_loop_v_stretch`), so no `v` mark is read there and
    /// the reset carries `v = 0`.
    #[test]
    fn a_spline_gate_that_is_not_off_writes_a_reset() {
        use super::{Deck, JointElement, decide_joint};
        use geom_brep::Pcurve;
        use geom_core::{Point2, Vec2};
        let zero = Vec2::new(0.0, 0.0);
        let at = Point2::new(0.4, 0.5);
        let image = Pcurve::free_harmonic(at, zero, zero, zero);
        for (span, reset) in [(1e-12, true), (5e-9, true), (100.0, false)] {
            let s = flat_chart(span);
            let vertex = Point3::new(0.4 * span, 0.5 * span, 0.0);
            let out = decide_joint(
                chart(&s),
                &image,
                0.0,
                at,
                vertex,
                super::ChartPeriods::given_u(Some(geom_brep::Period::new(1.0)), &s, band()),
                band(),
            );
            let want = if reset {
                JointElement::Reset(Deck::IDENTITY)
            } else {
                JointElement::IDENTITY
            };
            assert!(
                matches!(out, Ok(element) if element == want),
                "span {span:e}: {want:?}, read {:?}",
                out.as_ref().ok()
            );
        }
    }

    /// **A joint clear of the apex, or within the band of a pole, takes
    /// a lift of its own point at any gap on its orbit.** At `α = 0.01`
    /// a vertex `d` from the apex is decided `Off` the singular set from
    /// `d` past the band's escalate edge on, while its lever is `d·sin α`,
    /// a hundredth of that: under the band until `d` is a hundred times
    /// the edge. A continuous joint there (gap 0, a thousandth of a
    /// radian, a whole turn) reads every mark Zero or in band, and takes
    /// the orbit point its gap meets at that lever by the near-pole read
    /// ([`super::lever_periods`]): the image itself, or the image a
    /// period over. On a sphere, a vertex within the band of a pole reads
    /// the same way in half periods, and a whole turn's gap meets the
    /// image a period over, two half steps off.
    #[test]
    fn a_narrow_cone_loop_joint_clear_of_the_apex_takes_a_lift_of_its_point() {
        use super::{Deck, JointElement, NearRead, Orbit, decide_joint, near_pole_gap_closes};
        use geom_brep::Pcurve;
        use geom_core::Point2;
        let tau = std::f64::consts::TAU;
        let at_gap = |surface: &Surface<f64>, vertex, v: f64, gap: f64| {
            let image = Pcurve::Harmonic {
                u: geom_brep::Channel::fixed_angle(0.0),
                v: geom_brep::Channel::constant(v),
            };
            let prev = Point2::new(gap, v);
            decide_joint(
                chart(surface),
                &image,
                0.0,
                prev,
                vertex,
                super::ChartPeriods::given_u(Some(geom_brep::Period::new(tau)), surface, band()),
                band(),
            )
        };
        let read = |out: Result<JointElement, super::PinMiss>| match out {
            Ok(element) => format!("{element:?}"),
            Err(super::PinMiss::Discontinuity) => "Discontinuity".into(),
            Err(super::PinMiss::Escalated(_)) => "Escalated".into(),
            Err(super::PinMiss::NetStretchUndecided(_)) => "NetStretchUndecided".into(),
        };
        let lift = |u| {
            format!(
                "{:?}",
                JointElement::Shift(Deck {
                    u,
                    v: 0,
                    twin: false,
                })
            )
        };
        let alpha = 0.01f64;
        let cone: Surface<f64> = Surface::Cone {
            apex: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: alpha,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        for d in [2.0, 5.0, 10.0, 100.0, 1000.0].map(|m| m * band().escalate()) {
            let vertex = Point3::new(d * alpha.sin(), 0.0, d * alpha.cos());
            assert!(
                matches!(singular_at(chart(&cone), vertex, band()), Singular::Off),
                "d = {d:e}: the vertex is decided clear of the apex"
            );
            for (gap, turns) in [(0.0, 0), (1e-3, 0), (tau, 1), (-tau, -1)] {
                assert_eq!(
                    read(at_gap(&cone, vertex, d, gap)),
                    lift(turns),
                    "d = {d:e} (lever {:e}), gap {gap}: the image {turns} periods over",
                    d * alpha.sin()
                );
            }
            // Half a turn round, the gap sits on a mark and the read's
            // nearest offsets sit `π·lever` off: within ε, the image is
            // taken; in band, the two chart points (`2·lever` apart in
            // metres) may be one point and the joint escalates; past the
            // band, every offset is decided off and the marks' decided
            // miss stands.
            let lever = d * alpha.sin();
            let pi_lever = core::f64::consts::PI * lever;
            let want = if pi_lever < band().zero() {
                lift(0)
            } else if pi_lever < band().escalate() {
                "Escalated".to_owned()
            } else {
                "Discontinuity".to_owned()
            };
            assert_eq!(
                read(at_gap(&cone, vertex, d, 0.5 * tau)),
                want,
                "d = {d:e} (lever {lever:e}), gap π"
            );
        }
        let sphere: Surface<f64> = Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 2.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let near_pole = Point3::new(5e-9, 0.0, 2.0);
        assert!(
            matches!(
                singular_at(chart(&sphere), near_pole, band()),
                Singular::Undecided(_)
            ),
            "the vertex is within the band of the pole"
        );
        let v = (near_pole.z / 2.0).asin();
        for (gap, turns) in [(0.0, 0), (tau, 1), (-tau, -1)] {
            assert_eq!(
                read(at_gap(&sphere, near_pole, v, gap)),
                lift(turns),
                "sphere near its pole, gap {gap}: the image {turns} periods over"
            );
        }
        // Three half turns: the twin a period over, which the orbit's own
        // offsets do not reach and the branches astride the walk's mark do.
        assert_eq!(
            read(at_gap(&sphere, near_pole, v, 1.5 * tau)),
            format!(
                "{:?}",
                JointElement::Shift(Deck {
                    u: 1,
                    v: 0,
                    twin: true,
                })
            ),
            "sphere near its pole, gap 3π: the twin a period over"
        );
        // Where a whole turn at the lever is itself within ε, the image
        // and the image a period over both close, and the image is taken
        // first. Through `decide_joint` a sphere never reaches that: its
        // read runs only where the incidence is undecided, so its lever
        // is past the band's zero edge and no two offsets are within ε.
        // So the order is pinned at the read itself.
        let both = SupSpeed::new(0.05 * band().zero());
        let one = SupSpeed::new(0.5 * band().zero());
        for (orbit, period_over) in [(Orbit::Periods, 1), (Orbit::HalfPeriodsWithTwin, 2)] {
            for (sign, gap) in [(1, tau), (-1, -tau)] {
                let first = near_pole_gap_closes(
                    both,
                    gap,
                    geom_brep::Period::new(tau),
                    orbit,
                    None,
                    band(),
                );
                assert!(
                    matches!(first, NearRead::Closes(0)),
                    "{orbit:?}, gap {gap}, a whole turn within ε: the image itself first: {first:?}"
                );
                let over = near_pole_gap_closes(
                    one,
                    gap,
                    geom_brep::Period::new(tau),
                    orbit,
                    None,
                    band(),
                );
                assert!(
                    matches!(over, NearRead::Closes(m) if m == sign * period_over),
                    "{orbit:?}, gap {gap}, only the image a period over within ε: {over:?}"
                );
            }
        }
    }

    /// **A gap on a mark answers by the band at the lever, however many
    /// turns out.** On the `α = 0.01` cone, a gap of `k + ½` turns sits
    /// on a mark, and the orbit points astride it are `π·lever` off. Within
    /// ε the read takes one; in band it escalates; past the band the
    /// marks' decided miss stands. The read tries the branches astride
    /// the mark the walk stopped on, not only those about the image, so
    /// the answer is the same at every `k`. Levers sit just either side
    /// of each band edge, so a mutant moving an edge dies. A whole turn's
    /// gap at those levers is a lift a whole number of periods over.
    #[test]
    fn a_half_turn_gap_answers_by_the_band_at_every_turn_count() {
        use super::{JointElement, decide_joint};
        use geom_brep::Pcurve;
        use geom_core::Point2;
        let alpha = 0.01f64;
        let cone: Surface<f64> = Surface::Cone {
            apex: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: alpha,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let (pi, tau) = (core::f64::consts::PI, core::f64::consts::TAU);
        let (zero, esc) = (band().zero(), band().escalate());
        let at = |lever: f64, gap: f64| {
            let d = lever / alpha.sin();
            let vertex = Point3::new(lever, 0.0, d * alpha.cos());
            let image = Pcurve::Harmonic {
                u: geom_brep::Channel::fixed_angle(0.0),
                v: geom_brep::Channel::constant(d),
            };
            let prev = Point2::new(gap, d);
            match decide_joint(
                chart(&cone),
                &image,
                0.0,
                prev,
                vertex,
                super::ChartPeriods::given_u(Some(geom_brep::Period::new(tau)), &cone, band()),
                band(),
            ) {
                Ok(JointElement::Shift(deck)) if deck.v == 0 && !deck.twin => {
                    format!("u = {}", deck.u)
                }
                Ok(other) => format!("{other:?}"),
                Err(super::PinMiss::Discontinuity) => "Discontinuity".into(),
                Err(super::PinMiss::Escalated(_)) => "Escalated".into(),
                Err(super::PinMiss::NetStretchUndecided(_)) => "NetStretchUndecided".into(),
            }
        };
        // `π·lever` as a fraction of the escalate edge, in band: the
        // reviewers' levers, and a vertex `22.3·escalate` from the apex.
        let in_band = [0.55, 0.7, 0.75, 0.85, 0.95, 0.99]
            .map(|f| f * esc / pi)
            .into_iter()
            .chain([22.3 * esc * alpha.sin(), 1.01 * zero / pi]);
        for lever in in_band {
            for turns in [0.5, -0.5, 1.5, -1.5, 2.5, -2.5, 3.5, -3.5] {
                assert_eq!(
                    at(lever, turns * tau),
                    "Escalated",
                    "π·lever {:e}, gap {turns}τ: the branches astride the mark are in band",
                    pi * lever
                );
            }
        }
        // Just past the escalate edge every branch astride is decided off.
        let past = 1.01 * esc / pi;
        for turns in [0.5, -0.5, 2.5, -2.5, 3.5] {
            assert_eq!(
                at(past, turns * tau),
                "Discontinuity",
                "π·lever {:e}, gap {turns}τ: decided off every branch astride",
                pi * past
            );
        }
        // Just under the zero edge the image itself is within ε.
        assert_eq!(at(0.99 * zero / pi, 0.5 * tau), "u = 0", "gap π within ε");
        // A whole number of turns at levers with the marks in band.
        for lever in [0.55 * esc / pi, 0.95 * esc / pi, past] {
            for (turns, u) in [(1.0, 1), (2.0, 2), (-2.0, -2)] {
                assert_eq!(
                    at(lever, turns * tau),
                    format!("u = {u}"),
                    "π·lever {:e}, gap {turns}τ: the image {u} periods over",
                    pi * lever
                );
            }
        }
        // Two turns at a sub-band lever: the walk stops on its first
        // mark, half a turn from the image and undecided there, before it
        // reaches the gap, so no branch astride locates it; the marks'
        // own escalation answers, never a decided miss.
        for lever in [0.2 * zero, 0.5 * zero] {
            assert_eq!(
                at(lever, 2.0 * tau),
                "Escalated",
                "lever {lever:e}, gap 2τ: the walk's first mark is undecided"
            );
        }
    }

    /// **Each branch astride the walk's mark is tried.** A gap just off
    /// a half-turn mark sits unequally far from the two branches astride
    /// it: at `π·lever = escalate + ε/2`, a gap `(k + ½)·τ ± 0.8·ε/lever`
    /// leaves the nearer branch in band and the farther one decided off.
    /// The read must reach whichever is the nearer: the escalation is
    /// its own, at either side of the mark and at every `k`.
    #[test]
    fn each_branch_astride_the_walks_mark_is_tried() {
        let (pi, tau) = (core::f64::consts::PI, core::f64::consts::TAU);
        let (zero, esc) = (band().zero(), band().escalate());
        let lever = (esc + 0.5 * zero) / pi;
        for (turns, side) in [
            (2.5, 1.0),
            (2.5, -1.0),
            (-2.5, 1.0),
            (-2.5, -1.0),
            (0.5, 1.0),
            (0.5, -1.0),
        ] {
            let gap = turns * tau + side * 0.8 * zero / lever;
            let out = super::lever_periods(
                "each_branch_row",
                SupSpeed::new(lever),
                gap,
                geom_brep::Period::new(tau),
                super::Orbit::Periods,
                band(),
            );
            assert!(
                matches!(&out, Err(geom_brep::BranchMiss::Undecided(cause))
                    if cause.predicate == Some("pcurve_loop_pole_gap")),
                "gap {turns}τ {side:+}·0.8ε/lever: the nearer branch is in band: {out:?}"
            );
        }
    }

    /// **The first in-band candidate's cause is the one answered.**
    /// Two candidates in band, neither Zero: the escalation carries the
    /// first one's margin, whichever is larger.
    #[test]
    fn first_within_answers_the_first_deferred_cause() {
        let margins = [3e-9, 7e-9];
        let out = super::first_within("first_within_row", margins, band(), |&m| Margin::of(m));
        let want = decide("first_within_row", Margin::of(margins[0]), band());
        let got = out.err().map(|cause| format!("{:?}", cause.margin));
        let first = want.err().map(|cause| format!("{:?}", cause.margin));
        assert!(first.is_some(), "the first margin is in band");
        assert_eq!(got, first, "the first in-band candidate's margin");
        assert!(
            matches!(
                super::first_within("first_within_row", [7e-9, 0.0], band(), |&m| Margin::of(m)),
                Ok(Some(m)) if m == 0.0
            ),
            "an in-band candidate does not stop a later Zero"
        );
    }

    /// **An undecided mark keeps its own cause.** At a lever of half ε a
    /// gap of two turns stops the marks' walk on its first mark, in band,
    /// and the near-pole read's offsets are in band too: the escalation
    /// is the marks' (`name`), not the read's, so a caller that reads
    /// the predicate sees the walk it asked for.
    #[test]
    fn an_undecided_mark_keeps_its_own_cause() {
        let arm = SupSpeed::new(0.5 * band().zero());
        let tau = std::f64::consts::TAU;
        let out = super::lever_periods(
            "undecided_mark_row",
            arm,
            2.0 * tau,
            geom_brep::Period::new(tau),
            super::Orbit::Periods,
            band(),
        );
        assert!(
            matches!(&out, Err(geom_brep::BranchMiss::Undecided(cause))
                if cause.predicate == Some("undecided_mark_row")),
            "the marks' own escalation: {out:?}"
        );
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
    use crate::entity::{EntityId, FaceKey, HalfEdgeKey, LoopKey};
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
    /// `Stale` is the caller's own key, and states the fact with no
    /// recourse and no defect claim (D2 row 4), which is what it is held
    /// to.
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
            margin: None,
        };
        let arms = [
            PcurveMintError::Stale {
                role: "half_edge",
                key: EntityId::HalfEdge(HalfEdgeKey::default()),
            },
            PcurveMintError::NoCarrier {
                half_edge: HalfEdgeKey::default(),
            },
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
            PcurveMintError::JointWithoutRoom {
                face: FaceKey::default(),
                r#loop: LoopKey::default(),
                half_edge: HalfEdgeKey::default(),
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
            PcurveMintError::PlaceholderChart {
                face: FaceKey::default(),
            },
        ];
        assert_eq!(arms.len(), 12, "an arm was added without a row here");
        for arm in &arms {
            let msg = arm.to_string();
            match arm {
                // A caller's own key states the fact: no recourse, and
                // no defect claim.
                PcurveMintError::Stale { .. } => {
                    assert_eq!(test_utils::refusal::recourse_markers(&msg), 0, "{msg}");
                    assert!(!msg.contains("defect") && !msg.contains("torn"), "{msg}");
                }
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

    /// A fitted-lane escalation (the hull limb's, the one it raises),
    /// rendered through the mint's `Certify` arm, ends in its limb's own
    /// labelled recourse at a build.
    #[test]
    fn a_fitted_hull_escalation_renders_its_own_recourse_through_the_mint() {
        let finding = PcurveMintError::Certify {
            half_edge: HalfEdgeKey::default(),
            error: PcurveCertifyError::FittedEscalated {
                decision: geom_brep::SsiDecision::Limb(geom_brep::SsiLimb::HullSupChart),
                cause: Indeterminate {
                    margin: MarginDiag::value(5e-9),
                    band: Band::new(1e-9, 1e-8).unwrap(),
                    predicate: Some("ssi_hull_sup_chart"),
                    terminal_sliver: false,
                },
            },
        };
        assert_eq!(
            finding.to_string(),
            "the pcurve of half-edge HalfEdgeKey(null) failed certification: pcurve \
             certification: the fitted lane's certificate escalated — whether the fitted carrier \
             passes limb 2 (control-hull sup-norm bound through its pcurve) is undecided: \
             margin 5e-9 lies inside the ambiguity band (1e-9, 1e-8). Recourse: loosen the tolerance, as a last resort; this refusal may indicate a \
             kernel bug worth reporting"
        );
    }
}

/// **The fitted refusal raised before any check.** The mint's
/// general-image derivation meets an absent door before an image
/// exists, so the refusal it carries must not claim a check ran.
///
/// Called directly, not through a body: the one arm that reaches it is
/// an `Intersection` seam on a spline chart, which the mint reaches at a
/// certifying scalar's door, so the row is written at `f64`.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod derive_without_a_door {
    use super::derive_general_image;
    use geom::{NurbsCurve3, NurbsSurface};
    use geom_brep::PcurveCertifyError;
    use geom_core::spline::KnotVector;
    use geom_core::{Dual64, Point3, Real};

    #[test]
    fn a_dual_general_image_refuses_before_any_check_and_says_only_that() {
        let lift = Dual64::from_f64;
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let chart = NurbsSurface::new(
            geom_core::spline::Knots::from(kv.clone()).lift(geom_core::Real::from_f64),
            geom_core::spline::Knots::from(kv.clone()).lift(geom_core::Real::from_f64),
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
            ],
            vec![geom_core::Real::one(); 4],
        )
        .unwrap()
        .map_scalar(lift);
        let carrier = NurbsCurve3::new(
            geom_core::spline::Knots::from(kv).lift(geom_core::Real::from_f64),
            vec![Point3::new(0.5, 0.0, 0.0), Point3::new(0.5, 1.0, 0.0)],
            vec![geom_core::Real::one(); 2],
        )
        .unwrap()
        .map_scalar(lift);
        let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(carrier));
        let params = (Dual64::from_f64(0.0), Dual64::from_f64(1.0));
        let err = derive_general_image(&carrier, params, &chart).unwrap_err();
        let PcurveCertifyError::FittedLaneUnsupported { scalar: "dual" } = err else {
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
#[allow(clippy::unwrap_used, clippy::panic)]
mod odd_cone_circle {
    use super::{PcurveMintError, analytic_derive, not_owed};
    use crate::entity::HalfEdgeKey;
    use geom::{Curve3, Surface};
    use geom_brep::{PcurveCertifyError, PcurveCheck};
    use geom_core::{Band, Point3, Tol, Vec3};

    /// **An odd cone circle is owed at `f64`.** A circle about a cone's
    /// axis at its rim's height and radius, tilted off the rim's plane
    /// through the band and past it, on a grid of quarter decades about
    /// ε: untilted it is the rim and mints; tilted inside the band it
    /// still reads as the rim; where the class reading is in band it is
    /// none of the cone's circles and refuses as that reading's
    /// escalation; past the band it is off the chart. Every refusal is a
    /// check-8 finding: the one excuse ([`not_owed`]) is a scalar with no
    /// fitted door, and `f64` holds one. The grid must reach the class
    /// refusal, or the row has not read the arm it is about.
    #[test]
    fn an_odd_cone_circle_mints_or_is_a_finding_never_excused() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let alpha = 0.5_f64;
        let cone = Surface::Cone {
            apex: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: alpha,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let (h, tau) = (1.0_f64, core::f64::consts::TAU);
        let tilts = core::iter::once(0.0)
            .chain((-12..=16).map(|k| tol.eps() * 10f64.powf(f64::from(k) / 4.0)))
            .chain([0.3]);
        let mut class_refused = 0;
        for tilt in tilts {
            let circle = Curve3::Circle {
                center: Point3::new(0.0, 0.0, h),
                axis: Vec3::new(0.0, -tilt.sin(), tilt.cos()),
                radius: h * alpha.tan(),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            match analytic_derive(&circle, 0.0, tau, &cone, band, HalfEdgeKey::default()) {
                Ok(_) => assert!(
                    tilt < 0.3,
                    "tilt {tilt:e}: a circle well off the rim minted"
                ),
                Err(e) => {
                    assert!(tilt > 0.0, "tilt {tilt:e}: the rim itself refused: {e:?}");
                    assert!(!not_owed::<f64>(&e), "tilt {tilt:e}: excused: {e:?}");
                    if let PcurveMintError::Certify {
                        error:
                            PcurveCertifyError::Escalated {
                                check: PcurveCheck::ChartClass,
                                ..
                            },
                        ..
                    } = e
                    {
                        class_refused += 1;
                    }
                }
            }
        }
        assert!(
            class_refused > 0,
            "no tilt reached the class reading's in-band refusal"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::float_cmp, clippy::panic)]
mod v_shift_tests {
    use super::shift_v_branch;
    use geom_brep::{Pcurve, SpiricImage, WallTorus};
    use geom_core::{Point2, Vec2};

    /// **The second-channel branch shift, at its own door.** A body cannot
    /// exercise the wall arm: a spiric rim's parameter span is the
    /// revolved PROFILE arc's, so the one spiric-bearing body's rims span
    /// 1.78 rad and no joint decides a second-channel period on them.
    /// So the map's value is pinned HERE, per variant: the image is
    /// translated along the second channel and nothing else moves — a
    /// wall's `v0`, a cap's and a harmonic image's `p0.y` — so a planted
    /// `k·period → 0` is red.
    #[test]
    fn the_v_shift_translates_every_image_along_v_alone() {
        let period = core::f64::consts::TAU;
        let wall = Pcurve::Spiric {
            major: 0.09375,
            minor: 0.0703125,
            offset: 0.0078125,
            image: SpiricImage::Wall {
                u0: 0.25,
                v0: 0.5,
                sense: -1.0,
                torus: WallTorus::Mirror,
            },
        };
        let Pcurve::Spiric {
            major,
            minor,
            offset,
            image:
                SpiricImage::Wall {
                    u0,
                    v0,
                    sense,
                    torus,
                },
        } = shift_v_branch(&wall, 3, geom_brep::Period::new(period))
        else {
            panic!("the wall arm keeps its variant and its image kind");
        };
        assert_eq!(v0, 0.5 + 3.0 * period, "v0 takes the whole-period shift");
        assert_eq!(u0, 0.25, "the azimuth constant is the other door's");
        assert_eq!(sense, -1.0, "the sign is not a branch");
        assert_eq!(torus, WallTorus::Mirror, "nor is the torus");
        assert_eq!((major, minor, offset), (0.09375, 0.0703125, 0.0078125));

        // A cap lives on a plane chart, where no joint decides a
        // second-channel period; the map is the same translation there.
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
        let Pcurve::Spiric {
            image: SpiricImage::Cap { p0, pm, pa },
            ..
        } = shift_v_branch(&cap, 3, geom_brep::Period::new(period))
        else {
            panic!("the cap arm keeps its variant and its image kind");
        };
        assert_eq!(
            (p0.x, p0.y),
            (0.1, 0.2 + 3.0 * period),
            "p0.y takes the shift"
        );
        assert_eq!([pm.x, pm.y, pa.x, pa.y], [1.0, 0.0, 0.0, 0.0703125]);

        // Every coefficient live, on no chart: the row is of the map,
        // which takes a free channel's four coefficients alike on any
        // axis, so the shape is the point and no chart certifies it.
        let harmonic = Pcurve::free_harmonic(
            Point2::new(0.3, 0.4),
            Vec2::new(1.0, 0.25),
            Vec2::new(-0.5, 1.0),
            Vec2::new(0.125, -0.375),
        );
        let Some((p0, pa, pb, pl)) =
            shift_v_branch(&harmonic, 3, geom_brep::Period::new(period)).harmonic_form()
        else {
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
pub(crate) mod pole_slit_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use core::f64::consts::{FRAC_PI_2, PI};

    use geom::{Curve3, Surface};
    use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
    use geom_core::{Band, Point3, Tol, Vec3};

    use crate::{Body, FaceKey, LoopBoundary};

    use crate::construction::{FaceSurface, MefSite, MevSite};

    /// The unit sphere's point at chart `(u, v)`: axis `+z`, seam `+x`.
    fn at(u: f64, v: f64) -> Point3<f64> {
        Point3::new(v.cos() * u.cos(), v.cos() * u.sin(), v.sin())
    }

    /// **The snowman's cap, built to the shape the probe saw.** A cap of
    /// the unit sphere above the latitude `v = 0.5`, bounded by two rim
    /// arcs (`u ∈ [0, π]`, its carrier's parameter a period below, and
    /// `[π, 2π]`) and a meridian slit up `u = π`
    /// from the rim to the north pole and back: rim arc, slit up, slit
    /// down, rim arc. Returns the body, the cap and the cap's loop
    /// members.
    fn slit_cap() -> (Body<f64>, FaceKey, Vec<crate::HalfEdgeKey>) {
        slit_cap_below(1.0).expect("the slit cap mints")
    }

    /// [`slit_cap`] with its first rim arc's carrier parameter
    /// `periods` whole periods below the azimuth it covers, or the
    /// minting pass's refusal.
    pub(crate) fn slit_cap_below(
        periods: f64,
    ) -> Result<(Body<f64>, FaceKey, Vec<crate::HalfEdgeKey>), super::PcurveMintError> {
        let tol = Tol::witness();
        let v0 = 0.5;
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(at(0.0, v0), true).unwrap();
        let sphere = body
            .set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: Surface::Sphere {
                        center: Point3::new(0.0, 0.0, 0.0),
                        radius: 1.0,
                        axis: Vec3::unit_z(),
                        u_ref: Vec3::unit_x(),
                    },
                    sense: true,
                },
            )
            .unwrap();
        let rim_plane = body.add_surface(Surface::Plane {
            origin: Point3::new(0.0, 0.0, v0.sin()),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        });
        let rim = |t0: f64, t1: f64| EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(sphere, rim_plane),
                selector: geom_brep::CurveSelector::Witness(at(0.5 * (t0 + t1), v0)),
            },
            carrier: Curve3::Circle {
                center: Point3::new(0.0, 0.0, v0.sin()),
                axis: Vec3::unit_z(),
                radius: v0.cos(),
                u_ref: Vec3::unit_x(),
            },
            param_start: t0,
            param_end: t1,
        };
        let first_arc = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                at(PI, v0),
                // A period below the azimuth it covers, `[0, π]`: its
                // image's base sits a period below the slit's.
                rim(-2.0 * PI * periods, PI - 2.0 * PI * periods),
                tol,
            )
            .unwrap();
        let second_arc = body
            .mef(
                MefSite::Chords {
                    he1: first_arc.he_minus,
                    he2: first_arc.he_plus,
                },
                rim(PI, 2.0 * PI),
                FaceSurface::Shared {
                    key: sphere,
                    sense: true,
                },
                tol,
            )
            .unwrap();
        // The cap runs the rim with `u` increasing, seen from above.
        let cap_loop = body.get_half_edge(first_arc.he_plus).unwrap().parent_loop;
        assert_eq!(
            body.get_half_edge(second_arc.he_plus).unwrap().parent_loop,
            cap_loop,
            "both rim arcs run forward around the cap"
        );
        let cap = body.get_loop(cap_loop).unwrap().face;
        // The slit: up the meridian `u = π`, a great circle in the plane
        // `y = 0`, from the rim to the north pole.
        let meridian_plane = body.add_surface(Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::unit_y(),
            u_ref: Vec3::new(-1.0, 0.0, 0.0),
        });
        body.mev(
            MevSite::Fan {
                he1: second_arc.he_plus,
                he2: second_arc.he_plus,
            },
            at(0.0, FRAC_PI_2),
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(sphere, meridian_plane),
                    selector: geom_brep::CurveSelector::Witness(at(PI, 0.5 * (v0 + FRAC_PI_2))),
                },
                carrier: Curve3::Circle {
                    center: Point3::new(0.0, 0.0, 0.0),
                    axis: Vec3::unit_y(),
                    radius: 1.0,
                    u_ref: Vec3::new(-1.0, 0.0, 0.0),
                },
                param_start: v0,
                param_end: FRAC_PI_2,
            },
            tol,
        )
        .unwrap();
        crate::mint_pcurves(&mut body, tol)?;
        let LoopBoundary::Cycle { first } = body.get_loop(cap_loop).unwrap().boundary else {
            panic!("the cap's loop is a cycle")
        };
        let members = body.loop_cycle(first).unwrap();
        assert_eq!(members.len(), 4, "rim arc, slit up, slit down, rim arc");
        Ok((body, cap, members))
    }

    /// **A half-turn patch of the unit cylinder** (axis `+z`, seam `+x`)
    /// between heights 0 and 1: the bottom arc `u ∈ [0, π]`, its circle
    /// carrier's parameter `periods` whole periods below the azimuth it
    /// covers, a ruling up `u = π`, the top arc back to `u = 0` and a
    /// ruling down. Minted, or the minting pass's refusal.
    pub(crate) fn cylinder_patch_below(periods: f64) -> Result<Body<f64>, super::PcurveMintError> {
        cylinder_patch_turned(periods, 0.0)
    }

    /// [`cylinder_patch_below`] on a cylinder whose `u_ref` is turned by
    /// `turn` about its axis: the same patch in the world, its chart
    /// azimuth `turn` less, so its left meridian (the world's `x` axis)
    /// sits at chart `u = −turn`.
    pub(crate) fn cylinder_patch_turned(
        periods: f64,
        turn: f64,
    ) -> Result<Body<f64>, super::PcurveMintError> {
        let tol = Tol::witness();
        let p = |u: f64, z: f64| Point3::new(u.cos(), u.sin(), z);
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0, 0.0), true).unwrap();
        let cylinder = body
            .set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: Surface::Cylinder {
                        origin: Point3::new(0.0, 0.0, 0.0),
                        axis: Vec3::unit_z(),
                        radius: 1.0,
                        u_ref: Vec3::new(turn.cos(), turn.sin(), 0.0),
                    },
                    sense: true,
                },
            )
            .unwrap();
        let rim = |body: &mut Body<f64>, z: f64, t0: f64, t1: f64| {
            let plane = body.add_surface(Surface::Plane {
                origin: Point3::new(0.0, 0.0, z),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            });
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(cylinder, plane),
                    selector: geom_brep::CurveSelector::Witness(p(0.5 * (t0 + t1), z)),
                },
                carrier: Curve3::Circle {
                    center: Point3::new(0.0, 0.0, z),
                    axis: Vec3::unit_z(),
                    radius: 1.0,
                    u_ref: Vec3::unit_x(),
                },
                param_start: t0,
                param_end: t1,
            }
        };
        let shift = 2.0 * PI * periods;
        let bottom_spec = rim(&mut body, 0.0, -shift, PI - shift);
        let bottom = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(PI, 0.0),
                bottom_spec,
                tol,
            )
            .unwrap();
        let up = body
            .mev(
                MevSite::Fan {
                    he1: bottom.he_minus,
                    he2: bottom.he_minus,
                },
                p(PI, 1.0),
                EdgeCurveSpec::line_between(p(PI, 0.0), p(PI, 1.0)),
                tol,
            )
            .unwrap();
        // The top arc, its carrier run `[0, π]` like the bottom's, from
        // `u = π` back to `u = 0`.
        let mut top_spec = rim(&mut body, 1.0, 0.0, PI);
        top_spec.carrier = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 1.0),
            axis: Vec3::new(0.0, 0.0, -1.0),
            radius: 1.0,
            u_ref: Vec3::new(-1.0, 0.0, 0.0),
        };
        let top = body
            .mev(
                MevSite::Fan {
                    he1: up.he_minus,
                    he2: up.he_minus,
                },
                p(0.0, 1.0),
                top_spec,
                tol,
            )
            .unwrap();
        body.mef(
            MefSite::Chords {
                he1: top.he_minus,
                he2: bottom.he_plus,
            },
            EdgeCurveSpec::line_between(p(0.0, 1.0), p(0.0, 0.0)),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
        crate::mint_pcurves(&mut body, tol)?;
        Ok(body)
    }

    /// A half-turn patch of the cone of half-angle `0.4` about `z`, apex
    /// at the origin, between the rims at heights `1` and `2`, over the
    /// world azimuths `[0, π]`, on a chart whose `u_ref` is turned by
    /// `turn`: its left ruling (the world's `x` side) at chart `u = −turn`.
    pub(crate) fn cone_patch_turned(turn: f64) -> Result<Body<f64>, super::PcurveMintError> {
        let alpha: f64 = 0.4;
        let (z0, z1) = (1.0, 2.0);
        let tol = Tol::witness();
        let p =
            |u: f64, z: f64| Point3::new(z * alpha.tan() * u.cos(), z * alpha.tan() * u.sin(), z);
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0, z0), true).unwrap();
        let cone = body
            .set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: Surface::Cone {
                        apex: Point3::new(0.0, 0.0, 0.0),
                        axis: Vec3::unit_z(),
                        half_angle: alpha,
                        u_ref: Vec3::new(turn.cos(), turn.sin(), 0.0),
                    },
                    sense: true,
                },
            )
            .unwrap();
        let rim = |body: &mut Body<f64>, z: f64, t0: f64, t1: f64| {
            let plane = body.add_surface(Surface::Plane {
                origin: Point3::new(0.0, 0.0, z),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            });
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(cone, plane),
                    selector: geom_brep::CurveSelector::Witness(p(0.5 * (t0 + t1), z)),
                },
                carrier: Curve3::Circle {
                    center: Point3::new(0.0, 0.0, z),
                    axis: Vec3::unit_z(),
                    radius: z * alpha.tan(),
                    u_ref: Vec3::unit_x(),
                },
                param_start: t0,
                param_end: t1,
            }
        };
        let bottom_spec = rim(&mut body, z0, 0.0, PI);
        let bottom = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(PI, z0),
                bottom_spec,
                tol,
            )
            .unwrap();
        let up = body
            .mev(
                MevSite::Fan {
                    he1: bottom.he_minus,
                    he2: bottom.he_minus,
                },
                p(PI, z1),
                EdgeCurveSpec::line_between(p(PI, z0), p(PI, z1)),
                tol,
            )
            .unwrap();
        // The top arc, from `u = π` back to `u = 0`.
        let mut top_spec = rim(&mut body, z1, 0.0, PI);
        top_spec.carrier = Curve3::Circle {
            center: Point3::new(0.0, 0.0, z1),
            axis: Vec3::new(0.0, 0.0, -1.0),
            radius: z1 * alpha.tan(),
            u_ref: Vec3::new(-1.0, 0.0, 0.0),
        };
        let top = body
            .mev(
                MevSite::Fan {
                    he1: up.he_minus,
                    he2: up.he_minus,
                },
                p(0.0, z1),
                top_spec,
                tol,
            )
            .unwrap();
        body.mef(
            MefSite::Chords {
                he1: top.he_minus,
                he2: bottom.he_plus,
            },
            EdgeCurveSpec::line_between(p(0.0, z1), p(0.0, z0)),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
        crate::mint_pcurves(&mut body, tol)?;
        Ok(body)
    }

    /// The cap's stored rows — each half-edge's image and joint element —
    /// as text, by half-edge.
    fn rows(body: &Body<f64>, members: &[crate::HalfEdgeKey]) -> Vec<String> {
        members
            .iter()
            .map(|&he| format!("{he:?} {:?} {:?}", body.pcurve(he), body.joint(he)))
            .collect()
    }

    /// **A patch across the seam of a net closed in `u`.** A triangular
    /// prism stated as one degree-1 net, `u` knots at `0.3` and `0.7`,
    /// its first and last columns the same locus, so the chart closes in
    /// `u` with period `1`. The patch covers the two facets either side
    /// of the seam, `u ∈ [0.7, 1] ∪ [0, 0.3]`, `v ∈ [0, 1]`: a bottom
    /// split at the seam vertex, iso-line sides at `u = 0.3` and `0.7`,
    /// a top split the same way. Every edge is a straight line on a
    /// facet's edge or along a knot line, so each row is an iso line.
    pub(crate) fn prism_patch_across_the_seam()
    -> Result<(Body<f64>, FaceKey), super::PcurveMintError> {
        let tol = Tol::witness();
        let (c0, c1, c2) = (
            (1.0, 0.0),
            (-0.5, 0.75_f64.sqrt()),
            (-0.5, -(0.75_f64.sqrt())),
        );
        let at = |c: (f64, f64), z: f64| Point3::new(c.0, c.1, z);
        let ku =
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.3, 0.7, 1.0, 1.0], 1).unwrap();
        let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = [c0, c1, c2, c0]
            .into_iter()
            .flat_map(|c| [at(c, 0.0), at(c, 1.0)])
            .collect();
        let net = geom::NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap();
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(at(c2, 0.0), true).unwrap();
        let wall = body
            .set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: Surface::Nurbs(std::sync::Arc::new(net)),
                    sense: true,
                },
            )
            .unwrap();
        let column = |c: (f64, f64)| [at(c, 0.0), at(c, 1.0)];
        // A cap edge: the net's own row at height `v`, its carrier the
        // row on the net's basis and the chart's `u` as its parameter,
        // over `[u0, u1]`; its image the row `v`, stated, since the seam's
        // two sides `u = 1` and `u = 0` are one point to a projection.
        let row = |v: f64, (u0, u1): (f64, f64)| EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart_image(
                wall,
                geom_brep::Pcurve::IsoLine {
                    p0: geom_core::Point2::new(0.0, v),
                    pl: geom_core::Vec2::new(1.0, 0.0),
                },
            ),
            carrier: Curve3::Nurbs(std::sync::Arc::new(
                geom::NurbsCurve3::new(
                    geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.3, 0.7, 1.0, 1.0], 1)
                        .unwrap(),
                    vec![at(c0, v), at(c1, v), at(c2, v), at(c0, v)],
                    vec![1.0; 4],
                )
                .unwrap(),
            )),
            param_start: u0,
            param_end: u1,
        };
        // A side: the straight column at `u`, from height `v0` to `v1`.
        let side = |c: (f64, f64), u: f64, (v0, v1): (f64, f64)| {
            let [p, q] = column(c);
            let (p, q) = if v0 < v1 { (p, q) } else { (q, p) };
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::chart_image(
                    wall,
                    geom_brep::Pcurve::IsoLine {
                        p0: geom_core::Point2::new(u, v0),
                        pl: geom_core::Vec2::new(0.0, v1 - v0),
                    },
                ),
                carrier: Curve3::Line {
                    origin: p,
                    dir: q - p,
                },
                param_start: 0.0,
                param_end: 1.0,
            }
        };
        let fan = |he| MevSite::Fan { he1: he, he2: he };
        // The bottom, rising in `u` across the seam vertex.
        let b1 = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                at(c0, 0.0),
                row(0.0, (0.7, 1.0)),
                tol,
            )
            .unwrap();
        let b2 = body
            .mev(fan(b1.he_minus), at(c1, 0.0), row(0.0, (0.0, 0.3)), tol)
            .unwrap();
        // Up the side at `u = 0.7` from the start, and the top the same
        // way as the bottom.
        let up = body
            .mev(fan(b1.he_plus), at(c2, 1.0), side(c2, 0.7, (0.0, 1.0)), tol)
            .unwrap();
        let t1 = body
            .mev(fan(up.he_minus), at(c0, 1.0), row(1.0, (0.7, 1.0)), tol)
            .unwrap();
        let t2 = body
            .mev(fan(t1.he_minus), at(c1, 1.0), row(1.0, (0.0, 0.3)), tol)
            .unwrap();
        body.mef(
            MefSite::Chords {
                he1: t2.he_minus,
                he2: b2.he_minus,
            },
            side(c1, 0.3, (1.0, 0.0)),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
        crate::mint_pcurves(&mut body, tol)?;
        Ok((body, seed.face))
    }

    /// **A lifted copy is the row and its element, never certified
    /// again.** On the prism patch across the seam of a net closed in
    /// `u`, each face's walk decides a joint element of a whole chart
    /// period, `Deck.u = ±1`, and lifts iso-line rows by it: the patch's
    /// side at `u = 0.7` to `u = −0.3`, off the knot domain an iso line's
    /// hull bound holds on. `loop_windings` certifies each row as the
    /// row it is, before any lift, so both faces read and close. Certifying
    /// the lifted copy instead refuses: the bottom row lifted to
    /// `u ∈ [−0.3, 0]` reads its net off the domain, and its map
    /// residual is exceeded.
    #[test]
    fn a_lifted_iso_line_off_the_knot_domain_is_its_row_and_element() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (body, _) = prism_patch_across_the_seam().unwrap();
        for (f, face) in body.faces() {
            let s = body.get_surface(face.surface).unwrap().clone();
            let elements: Vec<_> = {
                let LoopBoundary::Cycle { first } = body.get_loop(face.outer).unwrap().boundary
                else {
                    panic!("the outer loop is a cycle")
                };
                body.loop_walk(first)
                    .closed("loop", first)
                    .into_iter()
                    .map(|he| body.joint(he))
                    .collect()
            };
            assert!(
                elements.iter().any(|e| matches!(
                    e,
                    Some(crate::joint::JointElement::Shift(d)) if d.u != 0
                )),
                "{f:?}: the walk crosses the seam: {elements:?}"
            );
            let read = super::loop_windings(&body, f, &s, band)
                .unwrap_or_else(|e| panic!("{f:?} reads: {e:?}"));
            assert!(
                read.iter().all(|w| w.zero && !w.singular),
                "{f:?}: every loop's lift closes: {read:?}"
            );
        }
    }

    /// **A patch across the seam of a net closed in `v`**: the prism of
    /// [`prism_patch_across_the_seam`] with its channels exchanged. One
    /// degree-1 net, `u` the height (knots `0, 1`), `v` around the prism
    /// with knots at `0.3` and `0.7`, its first and last rows the same
    /// locus, so the chart closes in `v` with period `1`. The patch
    /// covers the two facets either side of the seam, `u ∈ [0, 1]`,
    /// `v ∈ [0.7, 1] ∪ [0, 0.3]`: a bottom split at the seam vertex,
    /// iso-line sides at `v = 0.3` and `0.7`, a top split the same way.
    pub(crate) fn prism_patch_across_the_v_seam()
    -> Result<(Body<f64>, FaceKey), super::PcurveMintError> {
        let tol = Tol::witness();
        let (c0, c1, c2) = (
            (1.0, 0.0),
            (-0.5, 0.75_f64.sqrt()),
            (-0.5, -(0.75_f64.sqrt())),
        );
        let at = |c: (f64, f64), z: f64| Point3::new(c.0, c.1, z);
        let around = || {
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.3, 0.7, 1.0, 1.0], 1).unwrap()
        };
        let ku = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = [0.0, 1.0]
            .into_iter()
            .flat_map(|z| [c0, c1, c2, c0].map(|c| at(c, z)))
            .collect();
        let net = geom::NurbsSurface::new(ku, around(), control, vec![1.0; 8]).unwrap();
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(at(c2, 0.0), true).unwrap();
        let wall = body
            .set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: Surface::Nurbs(std::sync::Arc::new(net)),
                    sense: true,
                },
            )
            .unwrap();
        // A cap edge: the net's own column at height `u`, its carrier the
        // column on the net's basis and the chart's `v` as its parameter,
        // over `[v0, v1]`; its image the column `u`, stated, since the
        // seam's two sides `v = 1` and `v = 0` are one point to a
        // projection.
        let cap = |u: f64, (v0, v1): (f64, f64)| EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart_image(
                wall,
                geom_brep::Pcurve::IsoLine {
                    p0: geom_core::Point2::new(u, 0.0),
                    pl: geom_core::Vec2::new(0.0, 1.0),
                },
            ),
            carrier: Curve3::Nurbs(std::sync::Arc::new(
                geom::NurbsCurve3::new(
                    around(),
                    vec![at(c0, u), at(c1, u), at(c2, u), at(c0, u)],
                    vec![1.0; 4],
                )
                .unwrap(),
            )),
            param_start: v0,
            param_end: v1,
        };
        // A side: the straight row at `v`, from height `u0` to `u1`.
        let side = |c: (f64, f64), v: f64, (u0, u1): (f64, f64)| {
            let (p, q) = (at(c, u0), at(c, u1));
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::chart_image(
                    wall,
                    geom_brep::Pcurve::IsoLine {
                        p0: geom_core::Point2::new(u0, v),
                        pl: geom_core::Vec2::new(u1 - u0, 0.0),
                    },
                ),
                carrier: Curve3::Line {
                    origin: p,
                    dir: q - p,
                },
                param_start: 0.0,
                param_end: 1.0,
            }
        };
        let fan = |he| MevSite::Fan { he1: he, he2: he };
        // The bottom, rising in `v` across the seam vertex.
        let b1 = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                at(c0, 0.0),
                cap(0.0, (0.7, 1.0)),
                tol,
            )
            .unwrap();
        let b2 = body
            .mev(fan(b1.he_minus), at(c1, 0.0), cap(0.0, (0.0, 0.3)), tol)
            .unwrap();
        // Up the side at `v = 0.7` from the start, and the top the same
        // way as the bottom.
        let up = body
            .mev(fan(b1.he_plus), at(c2, 1.0), side(c2, 0.7, (0.0, 1.0)), tol)
            .unwrap();
        let t1 = body
            .mev(fan(up.he_minus), at(c0, 1.0), cap(1.0, (0.7, 1.0)), tol)
            .unwrap();
        let t2 = body
            .mev(fan(t1.he_minus), at(c1, 1.0), cap(1.0, (0.0, 0.3)), tol)
            .unwrap();
        body.mef(
            MefSite::Chords {
                he1: t2.he_minus,
                he2: b2.he_minus,
            },
            side(c1, 0.3, (1.0, 0.0)),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
        crate::mint_pcurves(&mut body, tol)?;
        Ok((body, seed.face))
    }

    /// **A net closed in `v` lifts its seam crossing by a `v` period**
    /// (C4: every closed channel has its own period, the knot domain's
    /// length on a net decided closed in that direction). On the prism
    /// patch across the `v` seam, each face's walk decides a joint
    /// element of a whole `v` period, `Deck.v = ±1`, and no `u` period;
    /// both faces' loops close (`loop_windings`); and the body validates. Without the `v` period the seam joint's gap is
    /// a whole knot length of `v`, and the mint refuses it as a
    /// discontinuity.
    #[test]
    fn a_net_closed_in_v_lifts_its_seam_crossing_by_a_v_period() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (body, _) = prism_patch_across_the_v_seam()
            .unwrap_or_else(|e| panic!("the prism closed in v mints: {e:?}"));
        for (f, face) in body.faces() {
            let s = body.get_surface(face.surface).unwrap().clone();
            assert!(
                super::chart_u_period(&s, band).is_none(),
                "{f:?}: the net is open in u"
            );
            assert_eq!(
                super::chart_v_period(&s, band).map(|p| p.period().length()),
                Some(1.0),
                "{f:?}: the net closes in v over its knot domain"
            );
            let elements: Vec<_> = {
                let LoopBoundary::Cycle { first } = body.get_loop(face.outer).unwrap().boundary
                else {
                    panic!("the outer loop is a cycle")
                };
                body.loop_walk(first)
                    .closed("loop", first)
                    .into_iter()
                    .map(|he| body.joint(he))
                    .collect()
            };
            assert!(
                elements.iter().any(|e| matches!(
                    e,
                    Some(crate::joint::JointElement::Shift(d)) if d.v.abs() == 1 && d.u == 0
                )),
                "{f:?}: the walk crosses the v seam by one period: {elements:?}"
            );
            let read = super::loop_windings(&body, f, &s, band)
                .unwrap_or_else(|e| panic!("{f:?} reads: {e:?}"));
            assert!(
                read.iter().all(|w| w.zero && !w.singular),
                "{f:?}: every loop's lift closes: {read:?}"
            );
        }
        let refusals = super::validate_pcurves(&body, band);
        assert!(refusals.is_empty(), "the body validates: {refusals:?}");
    }

    /// **The `v` period is the closed net's alone.** A joint whose two
    /// chart ends are the two sides of a net's `v` seam, `v = 1` onto
    /// `v = 0` at one point: on the net closed in `v` the walk decides
    /// `Deck.v = 1` there; on the same net with its last row moved off the
    /// first (open in `v`), `chart_v_period` answers none, no `v` element
    /// is decided, and the whole knot length of `v` is the gap, a
    /// discontinuity.
    #[test]
    fn only_a_net_closed_in_v_decides_a_v_element() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (c0, c1, c2) = (
            (1.0, 0.0),
            (-0.5, 0.75_f64.sqrt()),
            (-0.5, -(0.75_f64.sqrt())),
        );
        let at = |c: (f64, f64), z: f64| Point3::new(c.0, c.1, z);
        let net = |last: (f64, f64)| {
            let ku = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.3, 0.7, 1.0, 1.0], 1)
                .unwrap();
            let control = [0.0, 1.0]
                .into_iter()
                .flat_map(|z| [c0, c1, c2, last].map(|c| at(c, z)))
                .collect();
            Surface::Nurbs(std::sync::Arc::new(
                geom::NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap(),
            ))
        };
        // The row at `u = 0.5` entering at `v = 0`, its predecessor's exit
        // at `v = 1`: one point on the closed net.
        let image = geom_brep::Pcurve::IsoLine {
            p0: geom_core::Point2::new(0.5, 0.0),
            pl: geom_core::Vec2::new(0.0, 0.3),
        };
        let joint = |s: &Surface<f64>| {
            super::decide_joint(
                super::DescribedChart::of(s).unwrap(),
                &image,
                0.0,
                geom_core::Point2::new(0.5, 1.0),
                at(c0, 0.5),
                super::ChartPeriods::of(s, band),
                band,
            )
        };
        let read = |out: &Result<crate::joint::JointElement, super::PinMiss>| match out {
            Ok(element) => format!("{element:?}"),
            Err(super::PinMiss::Discontinuity) => "Discontinuity".to_owned(),
            Err(super::PinMiss::Escalated(cause)) => format!("Escalated({cause:?})"),
            Err(super::PinMiss::NetStretchUndecided(cause)) => {
                format!("NetStretchUndecided({cause:?})")
            }
        };
        let closed = net(c0);
        assert!(
            matches!(
                joint(&closed),
                Ok(crate::joint::JointElement::Shift(crate::joint::Deck {
                    u: 0,
                    v: 1,
                    twin: false
                }))
            ),
            "closed in v: {}",
            read(&joint(&closed))
        );
        let open = net((0.9, 0.1));
        assert!(super::chart_v_period(&open, band).is_none(), "open in v");
        assert!(
            matches!(joint(&open), Err(super::PinMiss::Discontinuity)),
            "open in v: {}",
            read(&joint(&open))
        );
    }

    /// **A meridian on the window's cut reads the same either side of
    /// it.** A generic cut makes an image on it rare, not impossible: a
    /// half-turn cylinder patch and a half-turn cone patch on charts
    /// turned so the left meridian sits at `u = −1`, the principal
    /// window's start, within an ulp either way and a hundred band
    /// widths either way (radius 1, so the band is in radians).
    /// Within an ulp the placement's mark is in band and its tie-break
    /// names one copy whichever side the rounding fell (the first
    /// premise); a hundred band widths off it is decided, and names `−1` on one side and
    /// `τ − 1` on the other (the second, both read off the stored rows).
    /// Nothing downstream tells the copies apart: each body validates,
    /// bounds its faces, reads the same polygon modulo a turn, and
    /// overlaps itself with the same answer.
    #[test]
    fn a_meridian_on_the_windows_cut_reads_the_same_either_side_of_it() {
        let band = Band::linear(Tol::witness()).unwrap();
        let tau = core::f64::consts::TAU;
        // The window's cut, pinned as the literal it is (`−1`), so a row
        // that moved the window would go red here rather than follow it.
        let turn = 1.0;
        assert_eq!(
            geom_brep::ANGLE_WINDOW_START,
            -turn,
            "the cut this row sits on"
        );
        // Far enough off the cut that the placement's mark is decided at
        // every ε row: past the escalation band, which is ten band zeros.
        let far = 100.0 * band.zero();
        let turns = [
            turn.next_down(),
            turn,
            turn.next_up(),
            turn - far,
            turn + far,
        ];
        type Build = fn(f64) -> Body<f64>;
        let builds: [(&str, Build); 2] = [
            ("cylinder", |t| cylinder_patch_turned(0.0, t).unwrap()),
            ("cone", |t| cone_patch_turned(t).unwrap()),
        ];
        for (name, build) in builds {
            let bodies: Vec<Body<f64>> = turns.iter().map(|&t| build(t)).collect();
            let mut copies: Vec<Vec<f64>> = Vec::new();
            for (t, body) in turns.iter().zip(&bodies) {
                let mut here = Vec::new();
                assert_eq!(
                    super::validate_pcurves(body, band),
                    vec![],
                    "{name}, turned {t}: validates"
                );
                for (f, face) in body.faces() {
                    let s = body.get_surface(face.surface).unwrap().clone();
                    let read = super::loop_windings(body, f, &s, band)
                        .unwrap_or_else(|e| panic!("{name}, turned {t}, {f:?}: reads: {e:?}"));
                    assert!(
                        read.iter().all(|w| w.zero),
                        "{name}, turned {t}, {f:?}: every loop closes: {read:?}"
                    );
                }
                // The left meridian's stored image: the constant azimuth
                // one copy of which is `−1`.
                for (he, _) in body.half_edges() {
                    if let Some(geom_brep::Pcurve::Harmonic {
                        u: geom_brep::Channel::Angle { phase, winding: 0 },
                        ..
                    }) = body.pcurve(he).map(|c| c.pcurve())
                    {
                        let off = (phase + 1.0).rem_euclid(tau);
                        if off < 2.0 * far || tau - off < 2.0 * far {
                            here.push(*phase);
                        }
                    }
                }
                assert!(
                    !here.is_empty(),
                    "{name}, turned {t}: the meridian is stored"
                );
                copies.push(here);
            }
            // The tie-break takes the lower branch, which names the
            // midpoint at the window's closed end, `τ − 1`: pinned as that
            // copy, so an upper tie-break goes red.
            let near: Vec<f64> = copies[..3].concat();
            assert!(
                near.iter().all(|u| (u - (tau - 1.0)).abs() < 1e-12),
                "{name}: the first premise, one copy, τ − 1, within an ulp of the cut: {near:?}"
            );
            let all = copies.concat();
            assert!(
                all.iter().any(|u| (u + 1.0).abs() < 2.0 * far)
                    && all.iter().any(|u| (u - (tau - 1.0)).abs() < 2.0 * far),
                "{name}: the second premise, the meridian named on both sides of the cut: {copies:?}"
            );
            let faces: Vec<FaceKey> = bodies[0].faces().map(|(f, _)| f).collect();
            let outcome = |a: usize, b: usize, f: FaceKey| {
                crate::glued_pair_overlap(
                    &bodies[a],
                    f,
                    &bodies[b],
                    f,
                    crate::CarrierPremise::Decided,
                    band,
                )
            };
            // Each body against itself: the copies the rounding named
            // are the only difference across the cut (bodies turned an
            // ulp apart differ in geometry too, which a near-flush seat
            // reads as such).
            for &f in &faces {
                let first = outcome(0, 0, f);
                for a in 1..turns.len() {
                    assert_eq!(
                        outcome(a, a, f),
                        first,
                        "{name}, {f:?}: turned {} against turned {}",
                        turns[a],
                        turns[0]
                    );
                }
            }
        }
    }

    /// **A sphere patch anywhere in latitude reads in the chart's
    /// region.** A patch between two meridians and two parallels, below
    /// latitude −1 rad, south and north of the equator and across it, its
    /// meridians' carriers started on the near half of their great circles
    /// and on the far half (the frame negated, the span moved by `π`: PR
    /// 5225's delta review, `d3`, where the far start stored the twin), is
    /// minted, validates, stores every latitude in `[−π/2, π/2]` (the
    /// chart's own latitudes, inside the latitude window `geom_brep`'s
    /// `Window::Latitude` places them in), and overlaps itself with
    /// positive area. Placed in the angular window instead, a
    /// meridian below latitude −1 was stored a turn up, which the chart
    /// region's sphere arm refuses (PR 5225's first review, P2).
    #[test]
    fn a_sphere_patch_anywhere_in_latitude_reads_in_the_charts_region() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let (o, z) = (Point3::new(0.0, 0.0, 0.0), Vec3::unit_z());
        let circle =
            |center: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>| Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            };
        let p = |u: f64, v: f64| Point3::new(v.cos() * u.cos(), v.cos() * u.sin(), v.sin());
        let rhat = |u: f64| Vec3::new(u.cos(), u.sin(), 0.0);
        let bands = [(-1.4_f64, -1.1_f64), (-0.9, -0.3), (0.3, 1.3), (-0.2, 0.4)];
        for (far, (va, vb)) in [false, true]
            .into_iter()
            .flat_map(|far| bands.map(|b| (far, b)))
        {
            let (flip, sh) = if far { (-1.0, PI) } else { (1.0, 0.0) };
            let (u0, u1) = (0.0_f64, 1.0_f64);
            let mut body = Body::<f64>::new();
            let seed = body.mvfs(p(u0, va), true).unwrap();
            body.set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: Surface::Sphere {
                        center: o,
                        radius: 1.0,
                        axis: z,
                        u_ref: Vec3::unit_x(),
                    },
                    sense: true,
                },
            )
            .unwrap();
            let bottom = EdgeCurveSpec::arc_of_circle(
                circle(Point3::new(0.0, 0.0, va.sin()), z, va.cos(), rhat(u0)),
                0.0,
                u1 - u0,
            )
            .unwrap();
            let b = body
                .mev(
                    MevSite::Lone {
                        r#loop: seed.r#loop,
                    },
                    p(u1, va),
                    bottom,
                    tol,
                )
                .unwrap();
            let right = EdgeCurveSpec::arc_of_circle(
                circle(o, rhat(u1).cross(z), 1.0, rhat(u1) * flip),
                va + sh,
                vb + sh,
            )
            .unwrap();
            let r = body
                .mev(
                    MevSite::Fan {
                        he1: b.he_minus,
                        he2: b.he_minus,
                    },
                    p(u1, vb),
                    right,
                    tol,
                )
                .unwrap();
            let top = EdgeCurveSpec::arc_of_circle(
                circle(Point3::new(0.0, 0.0, vb.sin()), -z, vb.cos(), rhat(u1)),
                0.0,
                u1 - u0,
            )
            .unwrap();
            let tp = body
                .mev(
                    MevSite::Fan {
                        he1: r.he_minus,
                        he2: r.he_minus,
                    },
                    p(u0, vb),
                    top,
                    tol,
                )
                .unwrap();
            let left = EdgeCurveSpec::arc_of_circle(
                circle(o, z.cross(rhat(u0)), 1.0, rhat(u0) * flip),
                -vb + sh,
                -va + sh,
            )
            .unwrap();
            body.mef(
                MefSite::Chords {
                    he1: tp.he_minus,
                    he2: b.he_plus,
                },
                left,
                FaceSurface::Inherit,
                tol,
            )
            .unwrap();
            super::mint_pcurves(&mut body, tol).unwrap();
            let label = format!("far {far} [{va}, {vb}]");
            assert_eq!(super::validate_pcurves(&body, band), vec![], "{label}");
            for (he, c) in body.pcurves() {
                let (a, b) = c.params();
                let mid = c.pcurve().eval(0.5 * (a + b));
                assert!(
                    mid.y.abs() <= core::f64::consts::FRAC_PI_2 + 1e-12,
                    "{label}, {he:?}: a latitude in the chart's region: {mid:?}"
                );
            }
            for (f, _) in body.faces() {
                let o = crate::glued_pair_overlap(
                    &body,
                    f,
                    &body,
                    f,
                    crate::CarrierPremise::Decided,
                    band,
                );
                assert!(
                    matches!(o, Ok(crate::ChartOverlap::PositiveArea)),
                    "{label}, {f:?}: {o:?}"
                );
            }
        }
    }

    /// **A one-edge loop's split closes on its own children.** A sheet
    /// bounded by one whole-turn circle (a cylinder's rim, a sphere's
    /// parallel, a torus's parallel) is split at `t ∈ {1, 3, 5, 5.5, 6}`.
    /// Past about `t = 4.28` the second child's midpoint is past the
    /// window's cut, so it is placed a turn below the first and the
    /// loop's closing joint, from the second child back into the first,
    /// changes; the edge is its own neighbour, so the split decides that
    /// joint between its children. Each split body validates and its rows
    /// and elements are byte for byte a fresh mint's (PR 5225's first
    /// review, P1).
    #[test]
    fn a_one_edge_loops_split_closes_on_its_children() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let (o, z, x) = (Point3::new(0.0, 0.0, 0.0), Vec3::unit_z(), Vec3::unit_x());
        let circle = |center: Point3<f64>, radius: f64| Curve3::Circle {
            center,
            axis: z,
            radius,
            u_ref: x,
        };
        let lat: f64 = 0.5;
        let cases = [
            (
                "cylinder rim",
                Surface::Cylinder {
                    origin: o,
                    axis: z,
                    radius: 1.0,
                    u_ref: x,
                },
                circle(Point3::new(0.0, 0.0, 0.5), 1.0),
            ),
            (
                "sphere parallel",
                Surface::Sphere {
                    center: o,
                    radius: 1.0,
                    axis: z,
                    u_ref: x,
                },
                circle(Point3::new(0.0, 0.0, lat.sin()), lat.cos()),
            ),
            (
                "torus parallel",
                Surface::Torus {
                    center: o,
                    axis: z,
                    major_radius: 2.0,
                    minor_radius: 0.5,
                    u_ref: x,
                },
                circle(Point3::new(0.0, 0.0, 0.5), 2.0),
            ),
        ];
        let rows = |body: &Body<f64>| {
            let mut out: Vec<String> = body
                .pcurves()
                .map(|(he, c)| {
                    format!(
                        "{he:?} {:?} {:?} {:?} {:?}",
                        c.params(),
                        c.pcurve(),
                        c.certificate(),
                        body.joint(he)
                    )
                })
                .collect();
            out.sort();
            out
        };
        for (name, surface, carrier) in cases {
            for t in [1.0, 3.0, 5.0, 5.5, 6.0] {
                let mut body = Body::<f64>::new();
                let seed = body.mvfs(carrier.eval(0.0), true).unwrap();
                body.set_face_surface(
                    seed.face,
                    FaceSurface::New {
                        surface: surface.clone(),
                        sense: true,
                    },
                )
                .unwrap();
                let spec = EdgeCurveSpec::arc_of_circle(carrier.clone(), 0.0, 2.0 * PI).unwrap();
                let made = body
                    .mef(
                        MefSite::Lone {
                            r#loop: seed.r#loop,
                        },
                        spec,
                        FaceSurface::Inherit,
                        tol,
                    )
                    .unwrap();
                super::mint_pcurves(&mut body, tol).unwrap();
                let edge = body.get_half_edge(made.he_plus).unwrap().edge;
                body.split_edge(edge, t, tol)
                    .unwrap_or_else(|e| panic!("{name} at {t}: splits: {e:?}"));
                assert_eq!(
                    super::validate_pcurves(&body, band),
                    vec![],
                    "{name} at {t}: validates"
                );
                let mut fresh = body.clone();
                super::mint_pcurves(&mut fresh, tol).unwrap();
                assert_eq!(rows(&body), rows(&fresh), "{name} at {t}: the mint's rows");
            }
        }
    }

    /// **A split's children are the mint's rows, and the loop still
    /// closes.** The cylinder patch, its bottom arc's carrier stated 0,
    /// ±1, 5 and 10 periods off, is split on that arc, and the split
    /// body's rows, their intervals, certificates and joint elements
    /// are byte for byte what a fresh `mint_pcurves` of the same body
    /// writes; it validates and bounds both faces. Two turns of the
    /// chart: one where both children sit inside the window, and one
    /// turned so the arc straddles the window's cut at `−1`, where the
    /// first child is placed a whole turn from the second and the joint
    /// between them is a decided `±1` (the premise, read off the stored
    /// element). Dropping the split's re-decision of the parent's end
    /// joints (`CarriedRows::entering`, `::leaving`) leaves the
    /// straddling patch's joints the parent's, and it no longer
    /// validates (`LoopDiscontinuity` on both halves the re-placed child
    /// meets); `sweep`'s `the_site_rows_are_the_passs_on_every_revolved_chart`
    /// goes red the same way on the full cone.
    #[test]
    fn a_split_of_an_edge_stated_periods_off_is_the_mints_and_closes() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let rows = |body: &Body<f64>| {
            let mut out: Vec<String> = body
                .pcurves()
                .map(|(he, c)| {
                    format!(
                        "{he:?} {:?} {:?} {:?} {:?}",
                        c.params(),
                        c.pcurve(),
                        c.certificate(),
                        body.joint(he)
                    )
                })
                .collect();
            out.sort();
            out
        };
        // Inside the window, and straddling its cut: the arc covers
        // chart `u ∈ [−turn, π − turn]`, split an eighth of a turn in.
        for (turn, straddles) in [(0.0, false), (1.0 + PI / 4.0, true)] {
            for periods in [0.0, 1.0, -1.0, 5.0, 10.0] {
                let label = format!("turned {turn}, {periods} periods off");
                let mut body = cylinder_patch_turned(periods, turn).unwrap();
                let bottom = body
                    .edges()
                    .map(|(e, _)| e)
                    .find(|&e| {
                        let he = body.get_edge(e).unwrap().he_plus;
                        matches!(
                            super::half_edge_carrier(&body, he),
                            Ok((Curve3::Circle { center, .. }, _, _)) if center.z == 0.0
                        )
                    })
                    .expect("the bottom arc");
                let he = body.get_edge(bottom).unwrap().he_plus;
                let (_, t0, t1) = super::half_edge_carrier(&body, he).unwrap();
                let t = t0.min(t1) + PI / 4.0;
                let made = body
                    .split_edge(bottom, t, tol)
                    .unwrap_or_else(|e| panic!("{label}: splits: {e:?}"));
                assert_eq!(
                    super::validate_pcurves(&body, band),
                    vec![],
                    "{label}: validates"
                );
                for (f, face) in body.faces() {
                    let s = body.get_surface(face.surface).unwrap().clone();
                    let read = super::loop_windings(&body, f, &s, band)
                        .unwrap_or_else(|e| panic!("{label}, {f:?}: reads: {e:?}"));
                    assert!(
                        read.iter().all(|w| w.zero),
                        "{label}, {f:?}: every loop closes: {read:?}"
                    );
                }
                let mut fresh = body.clone();
                super::mint_pcurves(&mut fresh, tol).unwrap();
                assert_eq!(
                    rows(&body),
                    rows(&fresh),
                    "{label}: the split's rows are the mint's"
                );
                let new = body.get_edge(made.new_edge).unwrap();
                let apart = [new.he_plus, new.he_minus, he].into_iter().any(
                    |h| matches!(body.joint(h), Some(crate::JointElement::Shift(d)) if d.u != 0),
                );
                assert_eq!(
                    apart, straddles,
                    "{label}: the premise, the children a whole turn apart where the arc \
                     straddles the cut"
                );
            }
        }
    }

    /// **A body stated periods off reads downstream as its principal
    /// twin.** The rows the walk mints far out reach the readers built
    /// on them, and each reads what the same body at zero periods reads:
    /// - `loop_windings` reads both faces of the cylinder patch 5, 6
    ///   and 10 periods below as it reads the twin's;
    /// - `chart_region_overlap` against the twin finds positive area on
    ///   either face at 5 periods and on the patch's own face at 10;
    /// - the slit cap's loop through the pole reads singular at 3 and 10
    ///   periods as at zero.
    ///
    /// The other face 6 periods below refuses `TouchingBoundary`, and
    /// that refusal is rounding, measured: the carrier's stated end
    /// `π − 6τ` rounds as it is written, so its seat's corner reads
    /// `π` plus 4 ulp, the two loops' walls are no longer exactly
    /// collinear, and the overlap refuses as it does any near-flush
    /// seat. No branch decision enters it; at 5 periods the end happens
    /// to round back to `π`.
    #[test]
    fn a_body_stated_periods_off_reads_downstream_as_its_principal_twin() {
        let band = Band::linear(Tol::witness()).unwrap();
        let twin = cylinder_patch_below(0.0).unwrap();
        let faces: Vec<FaceKey> = twin.faces().map(|(f, _)| f).collect();
        assert_eq!(faces.len(), 2);
        let bound = |body: &Body<f64>, f: FaceKey| {
            let s = body
                .get_surface(body.get_face(f).unwrap().surface)
                .unwrap()
                .clone();
            super::loop_windings(body, f, &s, band)
        };
        for periods in [5.0, 6.0, 10.0] {
            let body = cylinder_patch_below(periods).unwrap();
            for &f in &faces {
                assert_eq!(bound(&body, f), bound(&twin, f), "{periods} periods, {f:?}");
            }
            let overlaps: Vec<_> = faces
                .iter()
                .map(|&f| crate::chart_region_overlap(&body, f, &twin, f, band))
                .collect();
            let expected = if periods == 5.0 {
                [
                    Ok(crate::ChartOverlap::PositiveArea),
                    Ok(crate::ChartOverlap::PositiveArea),
                ]
            } else {
                [
                    Ok(crate::ChartOverlap::PositiveArea),
                    Err(crate::ChartRegionError::TouchingBoundary),
                ]
            };
            assert_eq!(overlaps, expected, "{periods} periods");
        }
        let cap_bound = |periods: f64| {
            let (body, cap, _) = slit_cap_below(periods).unwrap();
            bound(&body, cap).map(|read| read.iter().map(|w| w.singular).collect::<Vec<_>>())
        };
        let principal = cap_bound(0.0);
        assert!(
            matches!(&principal, Ok(singular) if singular.contains(&true)),
            "{principal:?}"
        );
        for periods in [3.0, 10.0] {
            assert_eq!(cap_bound(periods), principal, "{periods} periods");
        }
    }

    /// **A carrier stated whole periods off its azimuth mints.** Nothing
    /// bounds where a circle carrier's parameter runs against the azimuth
    /// it covers, so the loop walk's joint gap carries that offset, and
    /// the walk reads its marks from zero however far out it lies:
    /// - the slit cap with its first rim arc 3 and 10 periods below,
    ///   counted in the sphere's half steps, 6 and 20 of them;
    /// - a half-turn cylinder patch with its bottom arc 5 and 10 periods
    ///   below.
    ///
    /// Each mints, re-mints from every anchor where it is the slit cap,
    /// and reads clean. Check 4 reads no offset here: each stored image is
    /// its own placed re-derivation.
    #[test]
    fn a_carrier_stated_periods_off_its_azimuth_mints() {
        let band = Band::linear(Tol::witness()).unwrap();
        for periods in [3.0, 10.0] {
            let (mut body, cap, members) = slit_cap_below(periods)
                .unwrap_or_else(|e| panic!("slit cap {periods} periods below: {e:?}"));
            let cap_loop = body.get_half_edge(members[0]).unwrap().parent_loop;
            let minted = rows(&body, &members);
            for &anchor in &members {
                body.get_loop_mut(cap_loop).unwrap().boundary =
                    LoopBoundary::Cycle { first: anchor };
                crate::mint_pcurves_of(&mut body, &[cap], Tol::witness()).unwrap_or_else(|e| {
                    panic!("{periods} periods below, anchored at {anchor:?}: {e:?}")
                });
                assert_eq!(
                    rows(&body, &members),
                    minted,
                    "{periods}, anchored at {anchor:?}"
                );
                assert_eq!(super::validate_pcurves(&body, band), vec![], "{periods}");
            }
        }
        for periods in [5.0, 10.0] {
            let body = cylinder_patch_below(periods)
                .unwrap_or_else(|e| panic!("cylinder patch {periods} periods below: {e:?}"));
            assert_eq!(
                super::validate_pcurves(&body, band),
                vec![],
                "cylinder {periods}"
            );
        }
    }

    /// **The pass re-mints a pole-slit loop from every anchor, and reads
    /// what tier 3 reads.** The slit's two halves meet at the pole, a
    /// zero-lever joint whose element is the reset marker: the closure
    /// counts no azimuth across it, in the pass and at tier 3 alike. So
    /// with the loop anchored at each of its four half-edges in turn the
    /// pass mints the cap, writes the same images and elements, and
    /// tier 3 reads it clean (`work/topo/the-pass-refuses-a-tier-3-clean-
    /// loop-anchored-past-a-pole-slit`: the pass refused `LoopNotClosed`
    /// where the walk from the anchor carried the pole's free azimuth
    /// gap into its closure).
    #[test]
    fn a_pole_slit_loop_re_mints_from_every_anchor() {
        let (mut body, cap, members) = slit_cap();
        let band = Band::linear(Tol::witness()).unwrap();
        let cap_loop = body.get_half_edge(members[0]).unwrap().parent_loop;
        let pole_joints = members
            .iter()
            .filter(|&&he| body.joint(he).is_some_and(crate::JointElement::is_reset))
            .count();
        assert_eq!(
            pole_joints, 1,
            "the slit's turn at the pole is the one reset"
        );
        let minted = rows(&body, &members);
        for &anchor in &members {
            body.get_loop_mut(cap_loop).unwrap().boundary = LoopBoundary::Cycle { first: anchor };
            crate::mint_pcurves_of(&mut body, &[cap], Tol::witness())
                .unwrap_or_else(|e| panic!("anchored at {anchor:?}, the pass refuses: {e:?}"));
            assert_eq!(rows(&body, &members), minted, "anchored at {anchor:?}");
            assert_eq!(
                super::validate_pcurves(&body, band),
                vec![],
                "anchored at {anchor:?}"
            );
            assert!(
                body.loop_lift(cap_loop).is_ok(),
                "anchored at {anchor:?}, the loop lifts"
            );
        }
    }

    /// **A planted pole twin is loud.** At the slit's pole every azimuth
    /// names one point and the twin fixes the pole's latitude, so the
    /// reset with the twin composed in carries the slit's image onto the
    /// same pole point — and draws the slit's other half on the twin
    /// representation, off the cap's chart region. Tier 3 reads a stored
    /// element only as the one it decides there, so the planted twin is
    /// a discontinuity at that joint.
    #[test]
    fn a_planted_twin_on_the_pole_joint_is_loud() {
        let (mut body, _, members) = slit_cap();
        let band = Band::linear(Tol::witness()).unwrap();
        let pole = *members
            .iter()
            .find(|&&he| body.joint(he).is_some_and(crate::JointElement::is_reset))
            .expect("the slit's turn at the pole is a reset");
        let decided = body.joint(pole).unwrap().deck();
        let sigma = crate::Deck {
            u: 0,
            v: 0,
            twin: true,
        };
        let planted = crate::JointElement::Reset(decided.compose(sigma).without_u());
        assert_ne!(
            Some(planted),
            body.joint(pole),
            "the plant is another element"
        );
        body.attach_joint(pole, planted);
        let findings = super::validate_pcurves(&body, band);
        assert!(
            findings.contains(&super::PcurveMintError::LoopDiscontinuity { half_edge: pole }),
            "a twin planted on the pole joint is a discontinuity there: {findings:?}"
        );
    }

    /// **A joint's kind is part of its element at rest.** The same deck
    /// stored as the other kind — the pole's reset as an ordinary shift,
    /// and an ordinary joint's identity as a reset — is a discontinuity
    /// at that joint: tier 3 reads the stored element whole, not its deck
    /// alone. A shift at the pole would count the slit's azimuth winding
    /// across a point that has none, and a reset off it would drop a
    /// winding the loop has.
    #[test]
    fn a_joint_stored_as_the_other_kind_is_loud() {
        let (body, _, members) = slit_cap();
        let band = Band::linear(Tol::witness()).unwrap();
        let flip = |e: crate::JointElement| match e {
            crate::JointElement::Reset(deck) => crate::JointElement::Shift(deck),
            crate::JointElement::Shift(deck) => crate::JointElement::Reset(deck),
        };
        let reset = members
            .iter()
            .copied()
            .find(|&he| body.joint(he).is_some_and(crate::JointElement::is_reset))
            .expect("the slit's turn at the pole is a reset");
        let shift = members
            .iter()
            .copied()
            .find(|&he| body.joint(he).is_some_and(|e| !e.is_reset()))
            .expect("the cap's rim joints are ordinary");
        for he in [reset, shift] {
            let mut planted = body.clone();
            let swapped = flip(planted.joint(he).unwrap());
            planted.attach_joint(he, swapped);
            let findings = super::validate_pcurves(&planted, band);
            assert!(
                findings.contains(&super::PcurveMintError::LoopDiscontinuity { half_edge: he }),
                "{swapped:?} planted at {he:?} is a discontinuity there: {findings:?}"
            );
        }
    }
}

#[cfg(test)]
mod lift_tests {
    use super::lift_decks;
    use crate::joint::{Deck, JointElement, Winding};

    /// **The lift from every start is one chain.** A loop through both
    /// poles of a sphere, whose resets carry the twin and a
    /// second-channel period, closes on a whole number of azimuth
    /// periods. Rotated to start at each member — so the lift starts
    /// after one reset or the other — each member's lift differs from
    /// its lift in the original order by ONE deck transformation applied
    /// to the whole chain, read modulo the azimuth periods a reset
    /// restarts. A reset crossed without its twin or its second-channel
    /// period, or a start that dropped them, is a second offset.
    #[test]
    fn a_two_reset_loop_lifts_to_one_chain_from_every_start() {
        let elements = [
            JointElement::Reset(Deck {
                u: 0,
                v: 0,
                twin: true,
            }),
            JointElement::Shift(Deck {
                u: 1,
                v: 0,
                twin: false,
            }),
            JointElement::Reset(Deck {
                u: 0,
                v: 1,
                twin: true,
            }),
            JointElement::Shift(Deck {
                u: 0,
                v: 1,
                twin: false,
            }),
        ];
        let winding = Winding::of(elements);
        assert!(
            winding.deck.without_u() == Deck::IDENTITY && winding.closes(),
            "the premise: the loop closes on a whole number of azimuth periods: {winding:?}"
        );
        let n = elements.len();
        let base = lift_decks(&elements);
        for start in 1..n {
            let rotated: Vec<JointElement> = (0..n).map(|k| elements[(start + k) % n]).collect();
            let decks = lift_decks(&rotated);
            // `decks[k]` is member `start + k`'s lift from this start.
            let lift = |i: usize| decks[(i + n - start) % n];
            let offset = base[0].compose(lift(0).inverse());
            for i in 0..n {
                assert_eq!(
                    offset.compose(lift(i)).without_u(),
                    base[i].without_u(),
                    "start {start}, member {i}: {decks:?} against {base:?}"
                );
            }
        }
    }
}

#[cfg(test)]
mod kept_joint_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom_core::Tol;

    use super::{SiteCarriers, SiteHalf, SiteRows, apply_site_rows};
    use crate::test_support::arc_chain_over_the_jump;
    use crate::{Body, FaceKey, LoopBoundary};

    /// Every row and element `face`'s outer loop holds, in loop order.
    fn rows(body: &Body<f64>, face: FaceKey) -> Vec<String> {
        let outer = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("the wall's outer loop is a cycle")
        };
        let mut out: Vec<String> = body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .map(|he| {
                let c = body.pcurve(he).unwrap();
                format!(
                    "{he:?} {:?} {:?} {:?} {:?}",
                    c.params(),
                    c.pcurve(),
                    c.certificate(),
                    body.joint(he)
                )
            })
            .collect();
        out.sort();
        out
    }

    /// **A joint whose link the door changes is decided, not kept, even
    /// between two images that stand.** The strut kill of the arc
    /// chain's tip (`q → t` from `q`) re-links `s → q` onto `q → s`:
    /// both stand, and the element stored on `q → s` is the turn from
    /// `t → q`, a period, where the new joint's is the identity. The
    /// site mint planned over the kill's after-loop (the loop with the
    /// killed halves gone, as `kev_loops_after` hands it) decides that
    /// joint and nothing else, and its rows, written over the kill, are
    /// the pass's, elements included.
    #[test]
    fn a_re_linked_joint_between_standing_images_is_decided() {
        let tol = Tol::witness();
        let (body, face, s_q, q_t) = arc_chain_over_the_jump(tol);
        let outer = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("the wall's outer loop is a cycle")
        };
        let after: Vec<SiteHalf> = body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .filter(|he| !q_t.contains(he))
            .map(SiteHalf::Existing)
            .collect();
        let stale = body.joint(s_q[1]).unwrap();
        let plans = body
            .plan_site_mint(
                &[outer],
                |b, minted| {
                    Ok(minted
                        .iter()
                        .map(|(f, _)| b.site_face(*f, &[(outer, after.clone())], None))
                        .collect())
                },
                SiteCarriers::Existing,
                tol,
            )
            .unwrap();
        let [SiteRows::Mint { images, joints }] = plans.as_slice() else {
            panic!("the wall is minted, not left or cleared")
        };
        assert!(images.is_empty(), "the kill creates no image");
        let mut pass = body.clone();
        pass.kev(q_t[0]).unwrap();
        crate::mint_pcurves_of(&mut pass, &[face], tol).unwrap();
        let decided = pass.joint(s_q[1]).unwrap();
        assert_ne!(stale, decided, "the premise: the stored element is stale");
        assert_eq!(
            joints.as_slice(),
            [(SiteHalf::Existing(s_q[1]), decided)],
            "the re-linked joint, and only it, is decided"
        );
        let mut killed = body.clone();
        killed.kev(q_t[0]).unwrap();
        killed.write_joint(s_q[1], None);
        apply_site_rows(&mut killed, plans, None);
        assert_eq!(
            rows(&killed, face),
            rows(&pass, face),
            "the rows are the pass's"
        );
    }
}

#[cfg(test)]
mod lift_rows {
    //! [`super::decide_joint`]'s decisions, each on a joint built to sit
    //! where one of them is the only thing deciding. Every row runs at
    //! `f64` and at `Interval`, under a band pinned here (`zero = ε`),
    //! so the digits mean the same at every ε row.
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::{DescribedChart, PinMiss, Singular, decide_joint, singular_at};
    use crate::joint::{Deck, JointElement};
    use geom::Surface;
    use geom_brep::Pcurve;
    use geom_core::{Band, Decide, Interval, Point2, Point3, Vec3};

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

    /// A row that sits at one chart point of a sphere or torus chart,
    /// both of whose channels are angles: what a joint reads of it is
    /// its entry, and its twin.
    fn at<T: Decide>(u: f64, v: f64) -> Pcurve<T> {
        let f = T::from_f64;
        Pcurve::Harmonic {
            u: geom_brep::Channel::fixed_angle(f(u)),
            v: geom_brep::Channel::fixed_angle(f(v)),
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
    ) -> Result<JointElement, PinMiss> {
        let f = T::from_f64;
        decide_joint(
            DescribedChart::of(surface).unwrap(),
            &at(u, v),
            f(0.0),
            Point2::new(f(prev.0), f(prev.1)),
            vertex,
            super::ChartPeriods::given_u(Some(geom_brep::Period::new(T::tau())), surface, band(k)),
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
        // The twin either side: half a period over (`m = 1`, no period)
        // or half a period back (`m = −1`: the twin, one period back).
        let sides = [(core::f64::consts::PI, 0), (-core::f64::consts::PI, -1)];
        for k in [10.0, 3.0, 2.0] {
            for delta in [-3.5 * EPS, 3.5 * EPS] {
                for (side, ku) in sides {
                    let du = delta / v.cos();
                    let prev = (u + side + du, core::f64::consts::PI - v);
                    let Ok(element) = lift(&s, (u, v), prev, on_sphere(u, v), k) else {
                        panic!("{lane} K = {k} δ = {delta:e} side {side}: the joint lifts");
                    };
                    let want = Deck {
                        u: ku,
                        v: 0,
                        twin: true,
                    };
                    assert_eq!(
                        element,
                        JointElement::Shift(want),
                        "{lane} K = {k} δ = {delta:e} side {side}: the twin, {ku} periods"
                    );
                    // Where the element puts the entry, read on the f64
                    // lane: the element is integers, the same at both.
                    let lifted = want
                        .apply(
                            &at::<f64>(u, v),
                            &unit_sphere(),
                            super::ChartPeriods::stated(&unit_sphere()),
                        )
                        .unwrap();
                    assert!(
                        (lifted.eval(0.0).x - prev.0).abs() < 1e-6,
                        "{lane} K = {k} side {side}: the lifted entry lands on the predecessor"
                    );
                    assert_ne!(
                        element,
                        JointElement::IDENTITY,
                        "{lane} K = {k}: a twin is never the identity"
                    );
                }
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
    /// near-pole reading may take (the image, the twin either way, the
    /// image a period either way) is within ε of it at that lever. Taking the image there would be a
    /// branch no margin saw.
    fn near_pole_off_orbit_escalates<T: Decide>(lane: &str) {
        let s = unit_sphere::<T>();
        let theta: f64 = 5e-9;
        let v = core::f64::consts::FRAC_PI_2 - theta;
        let out = lift(&s, (0.0, v), (1.0, v), on_sphere(0.0, v), 10.0);
        assert!(
            matches!(out, Err(PinMiss::Escalated(_))),
            "{lane}: the incidence's escalation, not a lift: {:?}",
            out.as_ref().ok()
        );
    }

    /// **A joint whose vertex is on a pole is a reset**, whatever the
    /// azimuth gap: every azimuth names the point, so no period is
    /// decided, and the element says so rather than claiming the
    /// identity.
    fn on_the_pole_resets<T: Decide>(lane: &str) {
        let s = unit_sphere::<T>();
        let v = core::f64::consts::FRAC_PI_2;
        let f = T::from_f64;
        let pole = Point3::new(f(0.0), f(0.0), f(1.0));
        for gap in [0.0, 1.0, core::f64::consts::PI, core::f64::consts::TAU] {
            let out = lift(&s, (0.0, v), (gap, v), pole, 10.0);
            assert!(
                matches!(
                    out,
                    Ok(JointElement::Reset(Deck {
                        u: 0,
                        v: 0,
                        twin: false
                    }))
                ),
                "{lane} gap {gap}: a reset with no periods: {:?}",
                out.as_ref().ok()
            );
        }
    }

    #[test]
    fn a_joint_on_a_pole_is_a_reset() {
        on_the_pole_resets::<f64>("f64");
        on_the_pole_resets::<Interval>("Interval");
    }

    /// **Near a pole the fallback takes the orbit point the gap meets.**
    /// The vertex is `5ε` from the north pole (incidence undecided), so
    /// its lever is `5ε` and the half-period marks at it are undecided
    /// too. Against a predecessor on the image's own point the fallback
    /// takes `m = 0`, the identity; against one on the twin either way
    /// round it takes `m = ±1`, the twin. Each orbit point's gap at the
    /// vertex's lever decides Zero, and every other one's does not.
    fn near_pole_takes_the_orbit_point<T: Decide>(lane: &str) {
        let s = unit_sphere::<T>();
        let v = core::f64::consts::FRAC_PI_2 - 5e-9;
        let u = 0.3;
        let vertex = on_sphere(u, v);
        let twin_v = core::f64::consts::PI - v;
        let rows = [
            ((u, v), Deck::IDENTITY),
            (
                (u + core::f64::consts::PI, twin_v),
                Deck {
                    u: 0,
                    v: 0,
                    twin: true,
                },
            ),
            (
                (u - core::f64::consts::PI, twin_v),
                Deck {
                    u: -1,
                    v: 0,
                    twin: true,
                },
            ),
        ];
        for (prev, want) in rows {
            let out = lift(&s, (u, v), prev, vertex, 10.0);
            assert!(
                matches!(out, Ok(JointElement::Shift(deck)) if deck == want),
                "{lane} prev {prev:?}: the fallback takes {want:?}: {:?}",
                out.as_ref().ok()
            );
        }
    }

    #[test]
    fn near_a_pole_the_fallback_takes_the_orbit_point_the_gap_meets() {
        near_pole_takes_the_orbit_point::<f64>("f64");
        near_pole_takes_the_orbit_point::<Interval>("Interval");
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
        let Ok(element) = out else {
            panic!("{lane}: the marks decide")
        };
        let chart = DescribedChart::of(&s).unwrap();
        assert!(
            matches!(
                singular_at(chart, on_sphere(0.0, v), band(10.0)),
                Singular::Undecided(_)
            ),
            "{lane}"
        );
        assert_eq!(
            element,
            JointElement::Shift(Deck {
                u: 1,
                v: 0,
                twin: false
            }),
            "{lane}: one period over, not a reset"
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
        let Ok(element) = lift(&s, (u, v), (u, v + core::f64::consts::TAU), vertex, 10.0) else {
            panic!("{lane}: the joint lifts")
        };
        assert_eq!(
            element,
            JointElement::Shift(Deck {
                u: 0,
                v: 1,
                twin: false
            }),
            "{lane}: one meridian period"
        );
    }

    #[test]
    fn a_meridian_period_is_not_the_identity() {
        meridian_period::<f64>("f64");
        meridian_period::<Interval>("Interval");
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod turn_miss {
    use super::{Deck, JointElement, turn_element};
    use crate::construction::{MevSite, NewVertexSide};
    use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
    use crate::{Body, LoopBoundary};
    use geom::Surface;
    use geom_core::{Band, Tol};

    /// **A turn on a sub-band lever decides the lift its gap meets, in
    /// both orientations.** A null strut on a minted cylinder wall,
    /// described as the wall's own circle once round, is a closed
    /// carrier: its turn is a period, `−τ` read along `he_plus` and `+τ`
    /// along `he_minus`. On a cylinder whose radius `r` keeps the gap's
    /// nearest mark, `π·r` off, inside the band's escalate edge (under
    /// `escalate/π`, about `3.2ε` at this band), the marks read Zero or
    /// in band, and the near-pole read ([`super::lever_periods`]) takes
    /// the orbit point the gap meets within ε at that lever: the image
    /// one period over (`u = ∓1`), or under `ε/τ`, where every orbit
    /// point it tries is within ε, the image itself (`u = 0`). Past
    /// `escalate/π` the marks decide `u = ∓1` themselves, the control. Each is a lift of the one
    /// point, so the turn decides rather than missing as a
    /// discontinuity. Through the doors no body reaches either, since
    /// every joint at that vertex was decided at the same lever and
    /// margin when its rows were minted, so the row shrinks the
    /// carrier's surface under the stored rows.
    #[test]
    fn a_sub_band_turn_decides_the_lift_its_gap_meets() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut body = Body::<f64>::new();
        let frame = CylFrame::canonical(1.0);
        let face = cyl_wall_sheet(&mut body, frame, (0.2, 1.4), (0.0, 1.0), tol);
        let outer = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("the wall is bounded by a cycle")
        };
        let v = body.get_half_edge(first).unwrap().start;
        let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        let null = body
            .mev_null(
                MevSite::Fan {
                    he1: first,
                    he2: first,
                },
                NewVertexSide::Above,
            )
            .unwrap();
        let t0 = p.y.atan2(p.x);
        let circle = geom::Curve3::Circle {
            center: frame.origin + frame.axis * p.z,
            axis: frame.axis,
            radius: frame.radius,
            u_ref: frame.u_ref,
        };
        let spec = geom_brep::EdgeCurveSpec::arc_of_circle(circle, t0, t0 + core::f64::consts::TAU)
            .unwrap();
        body.set_edge_curve(null.edge, spec, tol).unwrap();
        let key = body.get_face(face).unwrap().surface;
        let he_minus = body.get_edge(null.edge).unwrap().he_minus;
        let at = |he, r: f64| {
            let mut shrunk = body.clone();
            let Some(Surface::Cylinder { radius, .. }) = shrunk.surfaces.get_mut(key) else {
                panic!("the wall is a cylinder")
            };
            *radius = r;
            turn_element(&shrunk, he, band)
        };
        let lift = |u| {
            Some(JointElement::Shift(Deck {
                u,
                v: 0,
                twin: false,
            }))
        };
        let read = |out: Result<Option<JointElement>, Option<geom_core::Indeterminate>>| {
            out.map_err(|cause| cause.is_some())
        };
        assert_eq!(
            read(at(null.he_plus, frame.radius)),
            Ok(lift(-1)),
            "the control: at the wall's own radius the turn is a period"
        );
        let eps = band.zero();
        for (he, turn, side) in [(null.he_plus, -1, "he_plus"), (he_minus, 1, "he_minus")] {
            for r in [0.01, 0.1, 0.15] {
                assert_eq!(
                    read(at(he, r * eps)),
                    Ok(lift(0)),
                    "{side}, r = {r}·ε: under ε/τ every orbit point is within ε, the image itself"
                );
            }
            for r in [0.17, 0.2, 0.25, 0.3, 0.5, 1.0, 3.0] {
                assert_eq!(
                    read(at(he, r * eps)),
                    Ok(lift(turn)),
                    "{side}, r = {r}·ε: the near-pole read takes the image a period over"
                );
            }
            assert_eq!(
                read(at(he, 10.0 * eps)),
                Ok(lift(turn)),
                "{side}, r = 10·ε: the control, where the marks decide the period"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod room_fence_tests {
    use super::{DescribedChart, Orbit, joint_has_room};
    use geom::Surface;
    use geom_core::{Band, Point3, Vec3};

    const EPS: f64 = 1e-9;

    /// The tightest band the kernel admits in practice: `K = 1.5`.
    fn tight() -> Band {
        Band::new(EPS, 1.5 * EPS).unwrap()
    }

    /// The fence at `vertex`, on the orbit a harmonic image on
    /// `surface` walks (a sphere's, half periods with its twin).
    fn room(surface: &Surface<f64>, vertex: Point3<f64>) -> bool {
        // A rim winding once: the second channel is an angle on a
        // sphere, a length on a cone.
        let v = match surface {
            Surface::Sphere { .. } | Surface::Torus { .. } => geom_brep::Channel::fixed_angle(0.0),
            _ => geom_brep::Channel::constant(0.0),
        };
        let image = geom_brep::Pcurve::Harmonic {
            u: geom_brep::Channel::Angle {
                phase: 0.0,
                winding: 1,
            },
            v,
        };
        joint_has_room(
            DescribedChart::of(surface).unwrap(),
            vertex,
            Some(geom_brep::Period::new(core::f64::consts::TAU)),
            Orbit::of(surface, &image),
            tight(),
        )
    }

    /// **The room fence holds at a small `K`.** A wrong orbit pick needs
    /// the joint's two chart ends half a step apart in azimuth, and
    /// those ends sit within `2ε` of the vertex, so the fence decides
    /// the chord they would span (`2·(d − 2ε)·sin(step/4)`) against the
    /// joint bound `4ε`. At `K = 1.5` a cone vertex `3ε` from the axis
    /// has an arc room `(π/2)·d` past four times the escalation, which
    /// the arc reading admitted, while a circle through it is under
    /// `4ε` across from the ends' least lever, so the integer pins
    /// nothing: it refuses. At `6ε` it has room.
    #[test]
    fn at_a_small_k_a_cone_joint_near_the_axis_has_no_room() {
        let alpha: f64 = 0.3;
        let cone = Surface::Cone {
            apex: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: alpha,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let at = |d: f64| Point3::new(d, 0.0, d / alpha.tan());
        assert!(!room(&cone, at(3.0 * EPS)), "3ε from the axis: no room");
        assert!(room(&cone, at(6.0 * EPS)), "6ε from the axis: room");
    }

    /// The sphere's step is half a period, so its chord is the
    /// `sin(π/4)` one: at `K = 1.5` a vertex `4.5ε` from the axis, which
    /// the arc reading admitted, refuses, and one `8ε` off has room.
    #[test]
    fn at_a_small_k_a_sphere_joint_near_a_pole_has_no_room() {
        let sphere = Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let at = |d: f64| Point3::new(d, 0.0, (1.0 - d * d).sqrt());
        assert!(!room(&sphere, at(4.5 * EPS)), "4.5ε from the axis: no room");
        assert!(room(&sphere, at(8.0 * EPS)), "8ε from the axis: room");
    }
}

/// A closed Villarceau edge's joint with itself, at the unit level, over
/// every pose and branch: the end-to-end row, a whole circle closing one
/// edge on a torus face through `mvfs` and `mef`, is
/// `tests/a_whole_villarceau_circle_bounds_a_torus_face.rs`.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod villarceau_joint_tests {
    use super::*;
    use core::f64::consts::TAU;
    use geom_core::{Point3, Tol, Vec3};

    /// **A whole turn of a Villarceau row is one period on each channel**
    /// (the reviewers' probe, PR 4227): on either family and traversal,
    /// from four centres and two starts, the joint carrying the row's
    /// exit onto its own entry decides
    /// `Shift(Deck { u: sense, v: vl, twin: false })` — the `(±1, ±1)`
    /// winding the loop invariant admits. The same holds on the row's
    /// lift by `k` azimuth periods and `−k` tube periods, which is the row
    /// with that deck element (`Deck::apply`), never a row of its own: as
    /// a row it refuses at its azimuth's fidelity term (C4).
    #[test]
    fn a_whole_turn_decides_one_period_on_each_channel() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (big, r) = (2.0_f64, 0.7_f64);
        let axis = Vec3::new(0.1, 0.2, 1.0).normalize();
        let u_ref = (Vec3::unit_x() - axis * axis.x).normalize();
        let center = Point3::new(0.3, -0.2, 0.5);
        let surface = Surface::Torus {
            center,
            axis,
            major_radius: big,
            minor_radius: r,
            u_ref,
        };
        let chart = DescribedChart::of(&surface).unwrap();
        let tilt = (r / big).asin();
        let mut cases = 0;
        for phi in [0.0_f64, 1.0, 2.5, -2.0] {
            let d = u_ref * phi.cos() + axis.cross(u_ref) * phi.sin();
            for family in [1.0_f64, -1.0] {
                let lean = d.cross(axis) * tilt.cos() + axis * (family * tilt.sin());
                for traversal in [1.0_f64, -1.0] {
                    for psi in [0.0_f64, 2.0] {
                        let carrier = geom::Curve3::Circle {
                            center: center + d * r,
                            axis: d.cross(lean) * traversal,
                            radius: big,
                            u_ref: d * psi.cos() + lean * psi.sin(),
                        };
                        let (t0, t1) = (0.3, 0.3 + TAU);
                        let row = geom_brep::PcurveCache::certify(
                            geom_brep::chart_pcurve_over(&carrier, t0, t1, &surface, band).unwrap(),
                            t0,
                            t1,
                            &carrier,
                            &surface,
                            band,
                        )
                        .unwrap();
                        let Pcurve::FocalSection(focal) = *row.pcurve() else {
                            panic!("a Villarceau row is a focal section")
                        };
                        for k in [-1, 0, 2] {
                            let lift = Deck {
                                u: k,
                                v: -k,
                                twin: false,
                            };
                            let lifted = lift
                                .apply(
                                    row.pcurve(),
                                    &surface,
                                    super::ChartPeriods::of(&surface, band),
                                )
                                .unwrap();
                            if k != 0 {
                                assert!(
                                    matches!(
                                        geom_brep::PcurveCache::certify(
                                            lifted.clone(),
                                            t0,
                                            t1,
                                            &carrier,
                                            &surface,
                                            band,
                                        ),
                                        Err(geom_brep::PcurveCertifyError::ResidualExceeded {
                                            check: geom_brep::PcurveCheck::EnvelopeTerm(
                                                geom_brep::EnvelopeTerm::FidelityU
                                            ),
                                            ..
                                        })
                                    ),
                                    "phi {phi}, family {family}, traversal {traversal}, \
                                     psi {psi}, k {k}: the lifted copy is no row"
                                );
                            }
                            let joint = decide_joint(
                                chart,
                                &lifted,
                                t0,
                                lifted.eval(t1),
                                carrier.eval(t0),
                                super::ChartPeriods::given_u(
                                    Some(geom_brep::Period::new(TAU)),
                                    chart.surface(),
                                    band,
                                ),
                                band,
                            );
                            let periods = (focal.sense, focal.vl);
                            assert!(
                                matches!(
                                    joint,
                                    Ok(JointElement::Shift(Deck { u, v, twin: false }))
                                        if (f64::from(u), f64::from(v)) == periods
                                ),
                                "phi {phi}, family {family}, traversal {traversal}, psi {psi}, \
                                 k {k}: {:?}, not a shift by {periods:?} periods",
                                joint.as_ref().ok()
                            );
                            cases += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(cases, 96);
    }
}

#[cfg(test)]
mod row_space_tests {
    #![allow(clippy::unwrap_used)]

    use super::RowSpace;
    use geom_core::spline::KnotVector;

    fn kv(knots: &[f64]) -> KnotVector {
        KnotVector::clamped(knots.to_vec(), 1).unwrap()
    }

    /// The row run back is any EXACT reflection of the row: on
    /// `[0.1, 0.3]` the knots `0.25` and `0.15` sum to `0.1 + 0.3` in ℝ
    /// while `fl(0.1 + 0.3 − 0.25)` is not `0.15`; on `[0, 1]`,
    /// `fl(1 − 0.1)` is `0.9` while `0.1 + 0.9` is not 1; the reflection
    /// through 0, `−k`, never rounds, while one about 0.7 rebuilt as
    /// `fl(0.7 − k)` does. Forward is the row's own knots and nothing
    /// else: a row symmetric only in decimal is not run back on them.
    #[test]
    fn each_direction_admits_its_own_relation() {
        let row = kv(&[0.1, 0.1, 0.25, 0.3, 0.3]);
        let only_back = RowSpace {
            forward: false,
            backward: true,
        };
        assert_eq!(
            RowSpace::of(&kv(&[0.1, 0.1, 0.15, 0.3, 0.3]), &row),
            only_back,
            "an exact mirror the rounded reflection misses runs the row back"
        );
        assert_eq!(
            RowSpace::of(&row.negated(), &row),
            only_back,
            "the row reflected through 0, which rounds nowhere, runs it back"
        );
        let about_0_7: Vec<f64> = row.knots().iter().rev().map(|k| 0.7 - k).collect();
        assert_eq!(
            RowSpace::of(&kv(&about_0_7), &row),
            RowSpace {
                forward: false,
                backward: false
            },
            "a reflection about 0.7 that holds only after rounding is refused"
        );
        assert_eq!(
            RowSpace::of(
                &kv(&[0.0, 0.0, 0.9, 1.0, 1.0]),
                &kv(&[0.0, 0.0, 0.1, 1.0, 1.0])
            ),
            RowSpace {
                forward: false,
                backward: false
            },
            "a vector the rounded reflection reproduces, a 2Sum residual off the exact \
             one, is in neither direction"
        );
        let decimal = kv(&[0.1, 0.1, 0.2, 0.3, 0.3]);
        assert_eq!(
            RowSpace::of(&decimal, &decimal),
            RowSpace {
                forward: true,
                backward: false
            },
            "a row symmetric only in decimal runs forward on its own knots, not back"
        );
        let dyadic = kv(&[0.0, 0.0, 0.5, 1.0, 1.0]);
        assert_eq!(
            RowSpace::of(&dyadic, &dyadic),
            RowSpace {
                forward: true,
                backward: true
            },
            "an exactly symmetric row runs either way on its own knots"
        );
    }
}

#[cfg(test)]
mod from_computed_rows {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use std::sync::Arc;

    use geom::{NurbsSurface, Surface};
    use geom_core::spline::KnotVector;
    use geom_core::{Point3, Real};

    use super::{computed_foot, derive_chart_foot};
    use crate::revalue_pin::{Revalued, S, revalued};

    /// A chart foot enters as computed values: the fitted lane's foot of
    /// a point the parameter moves, on a flat spline chart at
    /// `Sym<f64>`, re-values to neither coordinate `p0` measured.
    #[test]
    fn a_chart_foot_enters_as_computed() {
        let found = revalued([0.25, 0.25 + 1.0e-3], |_, x| {
            let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            let control = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
                .map(|(u, v)| Point3::new(u, v, 0.0).map(S::from_f64))
                .to_vec();
            let chart = Surface::Nurbs(Arc::new(
                NurbsSurface::new(
                    geom_core::spline::Knots::from(kv.clone()).lift(geom_core::Real::from_f64),
                    geom_core::spline::Knots::from(kv).lift(geom_core::Real::from_f64),
                    control,
                    vec![geom_core::Real::one(); 4],
                )
                .unwrap(),
            ));
            let point = Point3::new(x, S::from_f64(0.5) * x, S::zero());
            let foot = derive_chart_foot(point, &chart)
                .unwrap()
                .expect("a spline chart at a certifying scalar measures a foot");
            let foot = computed_foot::<S>(foot);
            vec![foot.x, foot.y]
        });
        assert_eq!(
            found,
            Revalued {
                kept: 0,
                opaque: 2,
                laundered: 0,
            }
        );
    }
}

/// **A net's own periods at its joints** (PCERT unit C's review rows):
/// the gates that read a net's periods, on a degree-1 triangular prism
/// net, `u` its height and `v` around it. The fixture and the first rows
/// are the reviewers' probes (PR 5444, review C r1 and r2), kept as rows.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod net_period_rows {
    use super::{ChartPeriods, DescribedChart, PinMiss, chart_v_period, decide_joint};
    use crate::joint::{Deck, JointElement};
    use geom::Surface;
    use geom_brep::Pcurve;
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Decide, Interval, Point2, Point3, Tol, Vec2};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn eps() -> f64 {
        Tol::witness().eps()
    }

    /// The prism's cross-section, a triangle of circumradius `size`.
    fn tri(size: f64) -> [(f64, f64); 3] {
        [
            (size, 0.0),
            (-0.5 * size, 0.75_f64.sqrt() * size),
            (-0.5 * size, -(0.75_f64.sqrt()) * size),
        ]
    }

    /// The prism as one net: `u` the height over `[0, 1]`, `v` around
    /// over `[a, a + s]` with interior knots at 0.3 and 0.7 of it, its
    /// last row the point `last` (the first row's for a net closed in
    /// `v`).
    fn net(size: f64, (a, s): (f64, f64), last: (f64, f64)) -> Surface<f64> {
        let [c0, c1, c2] = tri(size);
        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let kv =
            KnotVector::clamped(vec![a, a, a + 0.3 * s, a + 0.7 * s, a + s, a + s], 1).unwrap();
        let control = [0.0, 1.0]
            .into_iter()
            .flat_map(|z| [c0, c1, c2, last].map(|c| Point3::new(c.0, c.1, z)))
            .collect();
        Surface::Nurbs(std::sync::Arc::new(
            geom::NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap(),
        ))
    }

    fn closed(size: f64, domain: (f64, f64)) -> Surface<f64> {
        net(size, domain, tri(size)[0])
    }

    /// The joint into an iso line along `v` entering at `entry_v`, its
    /// predecessor's exit at `prev_v`, both at height 0.5, at `T`.
    fn joint<T: Decide>(
        s: &Surface<f64>,
        entry_v: f64,
        prev_v: f64,
    ) -> Result<JointElement, PinMiss> {
        let s: Surface<T> = s.map_scalar(T::from_f64);
        let f = T::from_f64;
        let image = Pcurve::IsoLine {
            p0: Point2::new(f(0.5), f(entry_v)),
            pl: Vec2::new(f(0.0), f(0.1)),
        };
        decide_joint(
            DescribedChart::of(&s).unwrap(),
            &image,
            f(0.0),
            Point2::new(f(0.5), f(prev_v)),
            Point3::new(f(1.0), f(0.0), f(0.5)),
            ChartPeriods::of(&s, band()),
            band(),
        )
    }

    fn shift(u: i32, v: i32) -> JointElement {
        JointElement::Shift(Deck { u, v, twin: false })
    }

    /// **`kv` reads the net's own domain**, offset and scaled, crossing
    /// the seam either way, at `f64` and at `Interval`: entering at `a`
    /// from `a + s` is one period up, the reverse one down, and a joint
    /// that crosses nothing is the identity. Two periods out the marks
    /// gallop to 2; exactly on a half-period mark no branch is named.
    #[test]
    fn kv_reads_an_offset_scaled_domain_both_ways_on_both_lanes() {
        for (a, s) in [(0.0, 1.0), (2.0, 3.0), (-7.5, 0.25), (100.0, 40.0)] {
            let n = closed(1.0, (a, s));
            assert_eq!(
                chart_v_period(&n, band()).map(|p| p.period().length()),
                Some(s),
                "({a}, {s})"
            );
            for (entry, prev, want) in [(a, a + s, 1), (a + s, a, -1), (a, a, 0), (a + s, a + s, 0)]
            {
                assert_eq!(
                    joint::<f64>(&n, entry, prev).ok(),
                    Some(shift(0, want)),
                    "f64 ({a}, {s}) {entry} <- {prev}"
                );
                assert_eq!(
                    joint::<Interval>(&n, entry, prev).ok(),
                    Some(shift(0, want)),
                    "Interval ({a}, {s}) {entry} <- {prev}"
                );
            }
            assert_eq!(
                joint::<f64>(&n, a, a + 2.0 * s).ok(),
                Some(shift(0, 2)),
                "two periods out"
            );
            assert!(
                joint::<f64>(&n, a, a + 0.5 * s).is_err(),
                "on a half-period mark"
            );
        }
    }

    /// **A net's `v` gate reads metres, not a rate.** A unit prism over a
    /// `v` knot domain 10¹⁰ long stretches under ε per parameter unit
    /// (`sup |S_v|` ≈ 6·10⁻¹⁰), yet a whole period moves a point metres
    /// (`net_reach`), so the seam crossing is one period, not a gap of
    /// metres refused as a discontinuity.
    #[test]
    fn a_long_v_domain_crosses_its_seam_by_one_period() {
        let n = closed(1.0, (0.0, 1e10));
        assert_eq!(joint::<f64>(&n, 0.0, 1e10).ok(), Some(shift(0, 1)));
        assert_eq!(joint::<Interval>(&n, 0.0, 1e10).ok(), Some(shift(0, 1)));
    }

    /// **The same of `u`** ([`super::singular_at`]'s net-level gate): the
    /// prism closed in `u` over a `u` knot domain 10¹⁰ long crosses its
    /// seam by one `u` period. Read as a rate, its stretch sits under
    /// the band and the gate wrote a reset with no `u` integer, its gap
    /// of metres a discontinuity.
    #[test]
    fn a_long_u_domain_crosses_its_seam_by_one_period() {
        let k = 1e10;
        let [c0, c1, c2] = tri(1.0);
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.3 * k, 0.7 * k, k, k], 1).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = [c0, c1, c2, c0]
            .into_iter()
            .flat_map(|c| [Point3::new(c.0, c.1, 0.0), Point3::new(c.0, c.1, 1.0)])
            .collect();
        let s = Surface::Nurbs(std::sync::Arc::new(
            geom::NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap(),
        ));
        let out = decide_joint(
            DescribedChart::of(&s).unwrap(),
            &Pcurve::IsoLine {
                p0: Point2::new(0.0, 0.5),
                pl: Vec2::new(0.3 * k, 0.0),
            },
            0.0,
            Point2::new(k, 0.5),
            Point3::new(1.0, 0.0, 0.5),
            ChartPeriods::of(&s, band()),
            band(),
        );
        assert_eq!(out.ok(), Some(shift(1, 0)));
    }

    /// **An undecided `v` gate refuses under its own name**
    /// (`PinMiss::NetStretchUndecided`): a net closed in `v` whose `v`
    /// knot domain moves a point within the band (`net_reach`) decides no
    /// `v` integer, and the joint writes no element, neither a reset nor
    /// an ordinary `v = 0`. Decided under the band, the same joint is an
    /// ordinary identity (`a_spline_gate_that_is_not_off_writes_a_reset`'s
    /// flat net); decided past it, the marks read.
    #[test]
    fn an_undecided_v_gate_refuses_under_its_own_name() {
        let mid = 0.5 * (1.0 + Tol::witness().k()) * eps();
        // The prism's `sup |S_v|` is its widest span: an edge `√3·size`
        // over 0.3 of the domain.
        let size = |reach: f64| reach * 0.3 / 3.0_f64.sqrt();
        let in_band = closed(size(mid), (0.0, 1.0));
        assert!(chart_v_period(&in_band, band()).is_some(), "closed in v");
        assert!(
            matches!(
                joint::<f64>(&in_band, 0.3, 0.3),
                Err(PinMiss::NetStretchUndecided(_))
            ),
            "f64"
        );
        assert!(
            matches!(
                joint::<Interval>(&in_band, 0.3, 0.3),
                Err(PinMiss::NetStretchUndecided(_))
            ),
            "Interval"
        );
        let under = closed(size(0.25 * eps()), (0.0, 1.0));
        assert_eq!(
            joint::<f64>(&under, 0.3, 0.3).ok(),
            Some(shift(0, 0)),
            "decided under"
        );
    }

    /// A net closed in BOTH channels: `u` around a unit triangle in the
    /// `xy` plane, `v` around a triangle of circumradius `minor` in each
    /// station's `(radial, z)` plane, both over `[0, 1]` with interior
    /// knots at 0.3 and 0.7 (review C-delta's probe fixture).
    fn tube(minor: f64) -> Surface<f64> {
        let ring = {
            let [a, b, c] = tri(1.0);
            [a, b, c, a]
        };
        let section = {
            let [a, b, c] = tri(minor);
            [a, b, c, a]
        };
        let k = || KnotVector::clamped(vec![0.0, 0.0, 0.3, 0.7, 1.0, 1.0], 1).unwrap();
        let mut control = Vec::new();
        for (x, y) in ring {
            let r = x.hypot(y);
            for (radial, z) in section {
                control.push(Point3::new(x + radial * x / r, y + radial * y / r, z));
            }
        }
        Surface::Nurbs(std::sync::Arc::new(
            geom::NurbsSurface::new(k(), k(), control, vec![1.0; 16]).unwrap(),
        ))
    }

    /// The joint into an iso line along `u` entering at `u = 0` from
    /// its predecessor's exit at `u = 1`: a crossing of the `u` seam, at
    /// `v = 0.5`, at `T`.
    fn u_crossing<T: Decide>(s: &Surface<f64>) -> Result<JointElement, PinMiss> {
        let s: Surface<T> = s.map_scalar(T::from_f64);
        let f = T::from_f64;
        decide_joint(
            DescribedChart::of(&s).unwrap(),
            &Pcurve::IsoLine {
                p0: Point2::new(f(0.0), f(0.5)),
                pl: Vec2::new(f(0.3), f(0.0)),
            },
            f(0.0),
            Point2::new(f(1.0), f(0.5)),
            Point3::new(f(1.0), f(0.0), f(0.0)),
            ChartPeriods::of(&s, band()),
            band(),
        )
    }

    /// **An undecided `v` gate refuses even where the `u` element is
    /// decided.** On a net closed in both channels whose `v` sweep sits
    /// in the band, a crossing of the `u` seam decides `u = 1`, but the
    /// joint refuses as `NetStretchUndecided`. A reset there would store
    /// the element modulo `T_u`, dropping the decided `u = 1` and keeping
    /// the undecided `v = 0`: the lift would restart `u`, the loop's
    /// winding would count no `u` wrap, and the chart boundary would
    /// refuse the loop under another name. With the `v` sweep decided
    /// past the band the same crossing is `Shift(u: 1)`.
    #[test]
    fn an_undecided_v_gate_refuses_a_decided_u_crossing() {
        let mid = 0.5 * (1.0 + Tol::witness().k()) * eps();
        let in_band = tube(mid * 0.3 / 3.0_f64.sqrt());
        assert!(
            super::chart_u_period(&in_band, band()).is_some(),
            "closed in u"
        );
        assert!(chart_v_period(&in_band, band()).is_some(), "closed in v");
        assert!(
            matches!(
                u_crossing::<f64>(&in_band),
                Err(PinMiss::NetStretchUndecided(_))
            ),
            "f64"
        );
        assert!(
            matches!(
                u_crossing::<Interval>(&in_band),
                Err(PinMiss::NetStretchUndecided(_))
            ),
            "Interval"
        );
        assert_eq!(
            u_crossing::<f64>(&tube(0.1)).ok(),
            Some(shift(1, 0)),
            "v decided"
        );
    }

    /// **A seam a near-band offset apart** (the net's last row moved off
    /// its first by `d`): under the band's zero the net is closed, and
    /// the crossing is one period; in the band, the closure folds to
    /// open (`work/pcert/chart-u-period-folds-an-in-band-net-closure-into-no-period.md`),
    /// no `v` period is offered, and the crossing's whole-domain gap
    /// refuses as a decided discontinuity where the honest answer would
    /// escalate. This row pins that fold.
    #[test]
    fn a_seam_a_near_band_offset_apart() {
        let [c0, ..] = tri(1.0);
        let off = |d: f64| net(1.0, (2.0, 3.0), (c0.0 + d, c0.1));
        for d in [0.0, 0.1 * eps(), 0.5 * eps()] {
            let n = off(d);
            assert!(chart_v_period(&n, band()).is_some(), "d = {d:e}: closed");
            assert_eq!(
                joint::<f64>(&n, 2.0, 5.0).ok(),
                Some(shift(0, 1)),
                "d = {d:e}"
            );
        }
        let d = 0.5 * (1.0 + Tol::witness().k()) * eps();
        let n = off(d);
        assert!(
            chart_v_period(&n, band()).is_none(),
            "in band: folded to open"
        );
        assert!(
            matches!(joint::<f64>(&n, 2.0, 5.0), Err(PinMiss::Discontinuity)),
            "the fold refuses as decided"
        );
    }
}
