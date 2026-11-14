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
//!   ([`PcurveCertifyError::Incidence`]) is refused. A circle on a
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
//! set — a pole, an apex, decided as 3-D incidence — the whole
//! latitude is the one point, and the element puts the image's entry
//! the face's turn past its predecessor's exit ([`singular_turn`]), so
//! the chain runs on through the point as the face's boundary. That
//! element is a function of the two edges, the chart and the face's
//! sense, with one exception stated where it is decided: two distinct
//! edges leaving the point along one direction (a slit's tip) turn by
//! nothing or by a whole turn, and which is read off the winding of the
//! rest of their loop. Every stored element records a decision: a
//! joint that decides none refuses. A spline chart's joint also states
//! its chart-space gap.
//!
//! A loop's **lift** — the chain of chart curves its readers draw — is
//! its images moved by the elements summed along it ([`Lift::read`]),
//! and its **winding** is the elements composed once around
//! ([`Winding`]). The loop must close: its winding a whole period of
//! the first channel at most, of a
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
//! rows where the two faces are on one CHART and one SIDE of it
//! ([`crate::Body::sides_agree`]: a turn on the chart's singular set is
//! the face's side's) and drops them where they are not
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
//! the loops [`crate::Body::half_edges_proven_whole`] proves), which is what makes its own
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
//!
//! [`JointElement::then`]: crate::joint::JointElement::then
//! [`PcurveCertifyError::Incidence`]: geom_brep::PcurveCertifyError::Incidence
//! [`Winding`]: crate::joint::Winding
//! [`Winding::closes`]: crate::joint::Winding::closes

mod decision;
mod lift;
mod mint;
mod site;
mod tier3;

pub use decision::*;
pub use lift::*;
pub use mint::*;
pub use site::*;
pub use tier3::*;

#[path = "pcurves_rings.rs"]
mod rings;
pub use rings::{FaceLift, PlacedRing, RingPlacementError, SplineTrimGap};
pub(crate) use rings::{
    HALVINGS, PIECE_DEPTH, PIECE_VISITS, RingMiss, SHIFTS, Shift, Side, WINDOW_SHARE, face_lift,
    place_ring,
};

#[cfg(test)]
pub(crate) mod staleness_posture;

/// The chart-stretch meter rows: the arms these loop-continuity
/// margins are metred by, and the direction each claim needs.
#[cfg(test)]
mod stretch_meter;

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod recourse_tests;

/// **The fitted refusal raised before any check.** The mint's
/// general-image derivation meets an absent door before an image
/// exists, so the refusal it carries must not claim a check ran.
///
/// Called directly, not through a body: the one arm that reaches it is
/// an `Intersection` seam on a spline chart, which the mint reaches at a
/// certifying scalar's door, so the row is written at `f64`.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod derive_without_a_door;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod odd_cone_circle;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::float_cmp, clippy::panic)]
mod v_shift_tests;

#[cfg(test)]
pub(crate) mod pole_slit_tests;

#[cfg(test)]
mod lift_agreement_rows;

#[cfg(test)]
mod lift_tests;

#[cfg(test)]
mod kept_joint_tests;

#[cfg(test)]
mod lift_rows;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod turn_miss;

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod room_fence_tests;

/// A closed Villarceau edge's joint with itself, at the unit level, over
/// every pose and branch: the end-to-end row, a whole circle closing one
/// edge on a torus face through `mvfs` and `mef`, is
/// `tests/a_whole_villarceau_circle_bounds_a_torus_face.rs`.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod villarceau_joint_tests;

#[cfg(test)]
mod row_space_tests;

#[cfg(test)]
mod from_computed_rows;

/// **A net's own periods at its joints** (PCERT unit C's review rows):
/// the gates that read a net's periods, on a degree-1 triangular prism
/// net, `u` its height and `v` around it. The fixture and the first rows
/// are the reviewers' probes (PR 5444, review C r1 and r2), kept as rows.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod net_period_rows;

#[cfg(test)]
mod singular_set_rows;
